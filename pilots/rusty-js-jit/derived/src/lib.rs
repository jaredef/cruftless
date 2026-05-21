//! rusty-js-jit — baseline JIT compiler for cruftless bytecode.
//!
//! Doc 731 §VII (R1–R8) is the design target. This crate owns:
//!
//! - The bytecode-to-Cranelift-IR translation table (see
//!   `docs/op-p4-classification.md` for the per-Op classification that
//!   drives this table).
//! - The function-table linkage that lets the runtime dispatch a JIT-
//!   compiled function instead of running the bytecode interpreter.
//! - The compilation threshold counter that decides when a function is
//!   hot enough to compile.
//! - The bytecode well-typedness verifier (R3) that runs before any
//!   Cranelift IR is emitted.
//!
//! What this crate does NOT own (per R2 + R7 + R8):
//!
//! - Instruction selection / register allocation / scheduling / peephole
//!   optimization / machine-code emission → Cranelift.
//! - Internal optimization passes (constant-folding, DCE, CSE) → Cranelift,
//!   or, more often, the upstream bytecode compiler.
//! - Stack-map / GC root tracking → Cranelift's framework.
//!
//! JIT-EXT 2 scope: crate scaffold + Cranelift smoke test. No bytecode
//! translation yet. The smoke test verifies that Cranelift builds on
//! the engagement's target platform (aarch64-linux on Pi) and that
//! a hand-built `fn add(a: i64, b: i64) -> i64` round-trips through
//! Cranelift's JIT module path.

pub mod translator;
pub mod promote;
pub mod deopt;
pub use translator::{compile_function, CompiledFn, JitFn};
pub use promote::promote_to_typed_i64;
pub use deopt::{
    DeoptReason, DeoptSite, DeoptLiveLocal, JitLocation,
    DeoptCallFrame, DeoptRecoveredState, JitCallOutcome,
    DeoptSiteTable, jit_deopt_thunk, reconstruct_state,
    deopt_trip, set_current_deopt_sites, clear_current_deopt_sites, take_last_deopt,
    set_force_shape_trip, get_force_shape_trip_addr,
    jit_getprop_on_object, GetPropFn,
    set_active_getprop_fn, clear_active_getprop_fn,
    set_current_runtime, clear_current_runtime, get_current_runtime,
    set_current_proto, clear_current_proto, get_current_proto,
};

// JIT-EXT 12: synthetic-trip smoke test. Builds a hand-rolled
// JIT'd function that unconditionally calls `deopt_trip(site_id=0, 42, 0, 0, 0)`
// and returns its sentinel. Verifies the extern-symbol wiring +
// dispatcher TLS round-trip end-to-end through Cranelift.
//
// Lives at the lib.rs tier (not translator.rs) because it shares
// the JITBuilder/JITModule plumbing with `smoke_test_add` and the
// test acts as a Cranelift integration probe rather than a
// translator-feature probe.
#[cfg(test)]
pub fn synthetic_trip_smoke() -> Result<extern "C" fn() -> i64, String> {
    let mut flag_builder = settings::builder();
    flag_builder.set("use_colocated_libcalls", "false")
        .map_err(|e| format!("flag use_colocated_libcalls: {e:?}"))?;
    flag_builder.set("is_pic", "false")
        .map_err(|e| format!("flag is_pic: {e:?}"))?;
    let isa_builder = cranelift_native::builder()
        .map_err(|e| format!("isa builder: {e}"))?;
    let isa = isa_builder.finish(settings::Flags::new(flag_builder))
        .map_err(|e| format!("isa finish: {e:?}"))?;

    let mut jit_builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    // Pre-bind the deopt trip symbol so Cranelift's linker can find
    // it. The address is the address of our Rust extern "C" function.
    jit_builder.symbol("deopt_trip", deopt::deopt_trip as *const u8);
    let mut module = JITModule::new(jit_builder);

    // Declare the trip function's signature in the JIT module so we
    // can refer to it from emitted code.
    let mut trip_sig = module.make_signature();
    for _ in 0..5 { trip_sig.params.push(AbiParam::new(I64)); }
    trip_sig.returns.push(AbiParam::new(I64));
    let trip_id = module
        .declare_function("deopt_trip", Linkage::Import, &trip_sig)
        .map_err(|e| format!("declare trip: {e}"))?;

    let mut ctx = module.make_context();
    let mut fb_ctx = FunctionBuilderContext::new();

    // Signature for our smoke function: () -> i64
    ctx.func.signature.returns.push(AbiParam::new(I64));

    {
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fb_ctx);
        let entry = builder.create_block();
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        // Bring `deopt_trip` into this function's scope.
        let trip_ref = module.declare_func_in_func(trip_id, &mut builder.func);

        // Build the call args: site_id=0, r0=42, r1=0, r2=0, r3=0.
        let site_id = builder.ins().iconst(I64, 0);
        let r0 = builder.ins().iconst(I64, 42);
        let r1 = builder.ins().iconst(I64, 0);
        let r2 = builder.ins().iconst(I64, 0);
        let r3 = builder.ins().iconst(I64, 0);
        let call_inst = builder.ins().call(trip_ref, &[site_id, r0, r1, r2, r3]);
        let ret = builder.inst_results(call_inst)[0];
        builder.ins().return_(&[ret]);
        builder.finalize();
    }

    let id = module
        .declare_function("synthetic_trip", Linkage::Export, &ctx.func.signature)
        .map_err(|e| format!("declare synthetic: {e}"))?;
    module.define_function(id, &mut ctx)
        .map_err(|e| format!("define synthetic: {e}"))?;
    module.clear_context(&mut ctx);
    module.finalize_definitions()
        .map_err(|e| format!("finalize: {e:?}"))?;

    let code_ptr = module.get_finalized_function(id);
    let f: extern "C" fn() -> i64 = unsafe { std::mem::transmute(code_ptr) };
    Box::leak(Box::new(module));
    Ok(f)
}

use cranelift_codegen::ir::types::I64;
use cranelift_codegen::ir::{AbiParam, InstBuilder};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

/// Compile a smoke-test `fn add(a: i64, b: i64) -> i64` through
/// Cranelift's JIT module path and return a callable function pointer.
/// Used to verify the Cranelift toolchain is wired correctly on the
/// engagement's target platform before any bytecode translation is
/// attempted.
pub fn smoke_test_add() -> Result<extern "C" fn(i64, i64) -> i64, String> {
    let mut flag_builder = settings::builder();
    flag_builder.set("use_colocated_libcalls", "false")
        .map_err(|e| format!("flag use_colocated_libcalls: {e:?}"))?;
    flag_builder.set("is_pic", "false")
        .map_err(|e| format!("flag is_pic: {e:?}"))?;
    let isa_builder = cranelift_native::builder()
        .map_err(|e| format!("isa builder: {e}"))?;
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .map_err(|e| format!("isa finish: {e:?}"))?;
    let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    let mut module = JITModule::new(builder);

    let mut ctx = module.make_context();
    let mut fb_ctx = FunctionBuilderContext::new();

    // Signature: (i64, i64) -> i64
    ctx.func.signature.params.push(AbiParam::new(I64));
    ctx.func.signature.params.push(AbiParam::new(I64));
    ctx.func.signature.returns.push(AbiParam::new(I64));

    {
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fb_ctx);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        builder.seal_block(entry);
        let a = builder.block_params(entry)[0];
        let b = builder.block_params(entry)[1];
        let sum = builder.ins().iadd(a, b);
        builder.ins().return_(&[sum]);
        builder.finalize();
    }

    let id = module
        .declare_function("smoke_add", Linkage::Export, &ctx.func.signature)
        .map_err(|e| format!("declare_function: {e}"))?;
    module.define_function(id, &mut ctx)
        .map_err(|e| format!("define_function: {e}"))?;
    module.clear_context(&mut ctx);
    module.finalize_definitions()
        .map_err(|e| format!("finalize_definitions: {e}"))?;

    let code_ptr = module.get_finalized_function(id);
    // Safety: the function we just compiled has signature
    // (i64, i64) -> i64 matching extern "C" fn(i64, i64) -> i64.
    // The pointer is valid for the lifetime of `module`; this smoke
    // test intentionally leaks `module` so the pointer stays valid
    // through the test's return.
    let f: extern "C" fn(i64, i64) -> i64 = unsafe { std::mem::transmute(code_ptr) };
    Box::leak(Box::new(module));
    Ok(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_add_works() {
        let add = smoke_test_add().expect("Cranelift smoke test failed");
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-10, 100), 90);
        assert_eq!(add(i64::MAX - 1, 1), i64::MAX);
    }

    /// JIT-EXT 12: a JIT'd function calls `deopt_trip` via Cranelift's
    /// extern-call mechanism. Confirms:
    ///   1. Cranelift's JITBuilder can pre-bind a Rust extern symbol
    ///   2. JIT'd code can call it with the correct calling convention
    ///   3. Our thread-local plumbing routes the trip back to the caller
    ///   4. `take_last_deopt()` returns the reconstructed state
    #[test]
    fn synthetic_trip_calls_thunk_end_to_end() {
        // Set up the active site table BEFORE invoking the JIT'd code.
        let sites = vec![DeoptSite {
            reason: DeoptReason::IntegerOverflow { op_pc: 100 },
            resume_pc: 200,
            live_locals: vec![DeoptLiveLocal {
                interp_slot: 0,
                jit_location: JitLocation::Register(0),
            }],
            stack_depth: 0,
            stack_slots: vec![],
        }];
        set_current_deopt_sites(&sites);

        let trip_fn = synthetic_trip_smoke().expect("Cranelift extern wiring failed");
        let ret = trip_fn();
        assert_eq!(ret, 0, "thunk's sentinel propagates through Cranelift return");

        let recovered = take_last_deopt().expect("trip should have recorded state");
        assert_eq!(recovered.reason, DeoptReason::IntegerOverflow { op_pc: 100 });
        assert_eq!(recovered.resume_pc, 200);
        // The synthetic_trip passes r0=42, which maps to interp_slot 0
        // per the site's live_locals.
        assert_eq!(recovered.local_values, vec![(0, 42)]);

        clear_current_deopt_sites();
    }
}
