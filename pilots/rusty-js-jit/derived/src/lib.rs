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
pub use translator::{compile_function, CompiledFn, JitFn};

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
}
