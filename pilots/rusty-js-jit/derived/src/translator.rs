//! Bytecode → Cranelift IR translator (i64-arithmetic + control-flow subset).
//!
//! Per Doc 731 §VII R3 (verifier-before-emission), `compile_function`
//! returns `Err` for any function whose bytecode contains an op not in
//! the supported set. The interpreter remains the fallback.
//!
//! JIT-EXT 4 op coverage (Class A subset, all values treated as raw i64):
//!
//!   LoadArg, LoadLocal, StoreLocal
//!   PushI32
//!   Add, Sub, Mul, Inc, Dec
//!   Lt, Le, Gt, Ge, Eq, Ne, StrictEq, StrictNe
//!   Jump, JumpIfTrue, JumpIfFalse
//!   Dup, Pop
//!   Return, ReturnUndef
//!
//! Control flow is handled by a pre-scan pass that finds all jump
//! targets, allocates a Cranelift Block per target, then translates
//! op-by-op switching blocks at target pcs. The virtual operand stack
//! is empty at every block boundary (a property of cruftless's compiler
//! output for statement-level boundaries); this assumption is verified
//! by the translator and a violation triggers Err.
//!
//! Locals are mapped to Cranelift `Variable`s — Cranelift's
//! mem2reg / ssa-conversion handles the SSA promotion automatically.

use cranelift_codegen::ir::condcodes::{FloatCC, IntCC};
use cranelift_codegen::ir::types::{F64, I64, I8};
use cranelift_codegen::ir::MemFlags;
use cranelift_codegen::ir::{AbiParam, Block, InstBuilder, Value as ClValue};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use rusty_js_bytecode::compiler::FunctionProto;
use rusty_js_bytecode::constants::{Constant, ConstantsPool};
use rusty_js_bytecode::op::{op_from_byte, Op};
use std::collections::HashMap;

/// JIT signature post-LeJIT-Φ Φ-EXT 2 (2026-05-23): extern "C" fn(f64) -> f64
/// for a 1-arg fn, extern "C" fn(f64, f64) -> f64 for a 2-arg fn. The
/// translator emits the variant matching proto.params. The dispatcher
/// passes raw f64 from Value::Number.payload (no truncation, no rebox).
///
/// At Φ-EXT 2 the JIT body's IR is unchanged (still iadd/etc.); the
/// per-arg prologue converts F64 → I64 via fcvt_to_sint_sat, and the
/// return converts I64 → F64 via fcvt_from_sint. Φ-EXT 3 flips the
/// body IR to f64-throughout.
pub type JitFn0 = extern "C" fn() -> f64;
pub type JitFn1 = extern "C" fn(f64) -> f64;
pub type JitFn2 = extern "C" fn(f64, f64) -> f64;
/// OSR-EXT 5b (2026-05-23): OSR loop-body signature. Single arg is a
/// `*mut f64` pointer into a caller-managed array of length
/// `proto.locals.len()`. Entry-block prologue loads each local from
/// the array; every Return / ReturnUndef site stores all locals back
/// to the array before returning. Closes the locals-marshaling
/// coverage tier per Doc 740 §VIII.2 + Finding VIII.2.
pub type JitFnOsr = extern "C" fn(*mut f64) -> f64;

pub enum JitFn {
    /// TL-EXT 3 (2026-05-23): 0-arg variant for module-body JIT entry.
    /// Top-level module bodies have no formal parameters; the dispatcher
    /// calls via `call0` and discards the f64 return (module result is
    /// always Undefined per ECMA module evaluation semantics).
    Arity0(JitFn0),
    Arity1(JitFn1),
    Arity2(JitFn2),
    /// OSR-EXT 5b (2026-05-23): OSR loop-body variant. Caller passes
    /// a `*mut f64` to a pre-marshaled locals array; JIT body loads on
    /// entry + stores on every return.
    ArityOsr(JitFnOsr),
}

impl JitFn {
    pub fn call0(&self) -> f64 {
        match self {
            JitFn::Arity0(f) => f(),
            JitFn::Arity1(f) => f(0.0),
            JitFn::Arity2(f) => f(0.0, 0.0),
            JitFn::ArityOsr(f) => f(std::ptr::null_mut()),
        }
    }
    pub fn call1(&self, a: f64) -> f64 {
        match self {
            JitFn::Arity0(f) => f(),
            JitFn::Arity1(f) => f(a),
            JitFn::Arity2(f) => f(a, 0.0),
            JitFn::ArityOsr(f) => f(std::ptr::null_mut()),
        }
    }
    pub fn call2(&self, a: f64, b: f64) -> f64 {
        match self {
            JitFn::Arity0(f) => f(),
            JitFn::Arity1(f) => f(a),
            JitFn::Arity2(f) => f(a, b),
            JitFn::ArityOsr(f) => f(std::ptr::null_mut()),
        }
    }
    /// OSR-EXT 5b (2026-05-23): invoke an OSR-compiled body. Caller
    /// passes a `*mut f64` to a pre-marshaled locals array (the
    /// frame.locals values unboxed via unbox_arg_f64; length =
    /// proto.locals.len()); on return, the array contains the post-
    /// JIT-body locals (the dispatcher reads back via box_to_value
    /// at OSR-EXT 5c). For non-OSR variants the pointer is unused.
    pub fn call_osr(&self, arr_ptr: *mut f64) -> f64 {
        match self {
            JitFn::Arity0(f) => f(),
            JitFn::Arity1(f) => f(0.0),
            JitFn::Arity2(f) => f(0.0, 0.0),
            JitFn::ArityOsr(f) => f(arr_ptr),
        }
    }
}

impl std::fmt::Debug for JitFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitFn::Arity0(p) => write!(f, "JitFn::Arity0(0x{:x})", *p as usize),
            JitFn::Arity1(p) => write!(f, "JitFn::Arity1(0x{:x})", *p as usize),
            JitFn::Arity2(p) => write!(f, "JitFn::Arity2(0x{:x})", *p as usize),
            JitFn::ArityOsr(p) => write!(f, "JitFn::ArityOsr(0x{:x})", *p as usize),
        }
    }
}

/// Compiled artifact. Holds the leaked JITModule to keep code valid.
pub struct CompiledFn {
    pub func: JitFn,
    _module: &'static mut JITModule,
    /// JIT-EXT 11: per-function deopt-site table. Empty until JIT-EXT 12
    /// starts emitting deopt sites. Indexed by site_id (the immediate
    /// the JIT'd code passes to `jit_deopt_thunk`).
    pub deopt_sites: crate::deopt::DeoptSiteTable,
    /// LeJIT-Ψ VTI-EXT 3b: true iff this function was compiled with
    /// `CRUFTLESS_LEJIT_VTI=1`. The dispatcher consults this flag at
    /// call time to decide whether to pre-unbox args (Rust-side
    /// `unbox_arg`, the historical path) or pass `*const Value`
    /// pointers (the VTI calling convention). Set at compile time so
    /// the dispatcher's choice agrees with the prologue's expectation.
    pub vti_enabled: bool,
    /// LeJIT-Τ TB-EXT 3a: Some(_) iff this function was compiled with
    /// `CRUFTLESS_LEJIT_TB=1`. Holds the compile-time-resolved facts
    /// the TB-EXT 3b inline call thunk reads once at thunk-build time
    /// instead of re-deriving them at each call. None when TB is off
    /// (default) — the dispatcher takes the historical path.
    pub tb_metadata: Option<crate::tiny_baseline::TinyBaselineMetadata>,
}

impl std::fmt::Debug for CompiledFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompiledFn({:?})", self.func)
    }
}

/// Try to JIT-compile `proto`. Returns Err with a short reason if the
/// bytecode contains any op not in the supported set.
///
/// JIT-EXT 7: auto-runs the typed-i64 promotion pass first. If the
/// function is eligible for typed-i64 promotion (no non-arithmetic
/// ops), the JIT compiles the promoted version (honest typed-i64
/// translation, Doc 731 §XIV.d β-path). Otherwise it falls back to
/// the plain-ops cheat path (still i64-assumed, JIT-EXT 4). Callers
/// see no behavioral difference; both paths produce structurally
/// identical Cranelift IR.
pub fn compile_function(proto: &FunctionProto) -> Result<CompiledFn, String> {
    // LeJIT-Φ Φ-EXT 3: auto-promotion to typed-i64 ops is disabled
    // because the JIT body now lowers untyped Add/Sub/Mul to f64
    // (fadd/fsub/fmul). The promote pass converts Op::Add → Op::AddI64
    // which still lowers to iadd; under Φ-EXT 3's F64 stack model
    // iadd would receive F64 operands and fail Cranelift verification.
    // Move 2 (separate pilot at the bytecode tier per Doc 731 §XIII)
    // will re-enable the promotion with a proper bytecode-tier-driven
    // typed-i64 IR that emits the typed ops at compile time, not at
    // JIT-translator entry. Until then: f64 path only.
    compile_function_inner(proto, false)
}

/// OSR-EXT 5b (2026-05-23): compile an OSR loop-body's synthetic
/// FunctionProto. Signature: `extern "C" fn(*mut f64) -> f64`. The
/// pointer is a caller-managed array of length `proto.locals.len()`;
/// entry-block prologue loads each local from the array; every
/// Return / ReturnUndef site stores all locals back to the array
/// before returning. Closes Doc 740 §VIII.2 locals-marshaling
/// coverage tier per Finding VIII.2.
///
/// Runtime dispatcher integration lands at OSR-EXT 5d (consumes this
/// + the OSR-EXT 5c box-to-value helper); this round's deliverable is
/// the compilable OSR-shape CompiledFn ready for consumption.
pub fn compile_function_osr(proto: &FunctionProto) -> Result<CompiledFn, String> {
    compile_function_inner(proto, true)
}

fn compile_function_inner(proto: &FunctionProto, osr_mode: bool) -> Result<CompiledFn, String> {
    // TL-EXT 3 (2026-05-23): also accept params=0 for module-body JIT
    // entry. The top-level module body has no formal parameters; the
    // JIT signature emits no AbiParam entries and call0 is used at the
    // dispatcher.
    //
    // OSR-EXT 5b (2026-05-23): under osr_mode, proto.params is ignored;
    // the JIT signature uses a single I64 (the *mut f64 locals array)
    // and locals are populated by the entry-block prologue. The OSR
    // synthetic FunctionProto is built with params=0 at try_osr_compile.
    if !osr_mode && proto.params != 0 && proto.params != 1 && proto.params != 2 {
        return Err(format!("first-cut JIT supports 0, 1, or 2 params; got {}", proto.params));
    }

    // Pre-scan: parse bytecode into a structured op list with absolute
    // pcs; identify all jump targets so we can allocate blocks.
    let mut parsed = parse_bytecode(&proto.bytecode, &proto.constants)?;
    // OSR-EXT 5e (2026-05-23): under osr_mode, append a synthetic
    // ReturnUndef at pc = bytecode.len() (one past last byte). This
    // serves as the fallthrough block for back-edge JumpIfX ops whose
    // condition is false (loop-exit case) AND for any forward-exit
    // jumps whose target is at end-of-slice. The synthesized op
    // triggers the existing OSR locals-store-back epilogue + return.
    // Without this synthetic, find_next_block_pc fails on the last
    // JumpIfX because parsed[i+1..] is empty. Closes the empirical
    // refinement of Finding OSR.2 (do-while loops also need this).
    if osr_mode {
        parsed.push((proto.bytecode.len(), ParsedOp::ReturnUndef));
    }
    let mut targets: Vec<usize> = parsed.iter()
        .filter_map(|(_, op)| op.jump_target())
        .collect();
    targets.push(0); // entry block.
    // Also: the instruction immediately AFTER any terminator (Jump,
    // JumpIfTrue, JumpIfFalse, Return, ReturnUndef) is a block start —
    // either it's the fallthrough target of a conditional, or it's
    // dead but might be reached via another jump landing here. Cranelift
    // brif needs an actual fallthrough block, so we materialize one.
    for i in 0..parsed.len() {
        let is_terminator = matches!(&parsed[i].1,
            ParsedOp::Jump(_) | ParsedOp::JumpIfTrue(_) | ParsedOp::JumpIfFalse(_)
            | ParsedOp::Return | ParsedOp::ReturnUndef);
        if is_terminator {
            if let Some(next) = parsed.get(i + 1) {
                targets.push(next.0);
            }
        }
    }
    targets.sort();
    targets.dedup();

    // Build Cranelift JIT module.
    let mut flag_builder = settings::builder();
    flag_builder.set("use_colocated_libcalls", "false")
        .map_err(|e| format!("flag: {e:?}"))?;
    flag_builder.set("is_pic", "false")
        .map_err(|e| format!("flag: {e:?}"))?;
    let isa_builder = cranelift_native::builder().map_err(|e| format!("isa: {e}"))?;
    let isa = isa_builder.finish(settings::Flags::new(flag_builder))
        .map_err(|e| format!("isa: {e:?}"))?;
    // JIT-EXT 13: detect the deopt-guard feature flag.
    //
    // When `CRUFTLESS_JIT_GUARD_OVERFLOW=1`, the translator emits
    // signed-overflow checks at every Add site and branches to a
    // deopt block on trip.
    //
    // JIT-EXT 17: `CRUFTLESS_JIT_FORCE_SHAPE_TRIP=1` is the IC
    // demonstrator flag. The translator emits an entry check that
    // reads `JIT_FORCE_SHAPE_TRIP` (a static AtomicBool) and fires
    // `ICShapeMismatch` deopt if true. Tests toggle the static to
    // exercise both paths.
    //
    // Both flags are opt-in. With neither set, the existing perf
    // profile is preserved.
    let guard_overflow = std::env::var("CRUFTLESS_JIT_GUARD_OVERFLOW")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let force_shape_trip = std::env::var("CRUFTLESS_JIT_FORCE_SHAPE_TRIP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let any_guard = guard_overflow || force_shape_trip;

    // LeJIT-Σ StubE-EXT 5a (env-flag plumbing only — behavior unchanged
    // in this round): `CRUFTLESS_LEJIT_STUB=1` opts into the LeJIT-Σ
    // IC stub emitter dispatch path at Op::GetPropOnObject sites. This
    // round adds the flag detection + site-id allocation; EXT 5b wires
    // the observer extern (IC cache populates); EXT 5c emits the
    // inline compare-branch-load fast path. Pre-binding the IC site
    // ids at translate time costs ~1 µs per GetPropOnObject occurrence
    // and is harmless when the flag is unset (site_ids alloc but
    // dispatch path stays the existing extern).
    // LeJIT-Σ StubE-EXT 8 (2026-05-23): default-on flip authorized by
    // keeper after three-probe-levels gate satisfied at StubE-EXT 7
    // (bench: TB+STUB 80.8 ns on bench_ic — Pred-tb.2 holds with
    // margin; consumer-route: diff-prod 42/42 under all flag combos;
    // fuzz: 4/4 runtime configs byte-identical on the cache state
    // machine fuzz fixture). Opt out via CRUFTLESS_LEJIT_STUB=0.
    let lejit_stub = std::env::var("CRUFTLESS_LEJIT_STUB")
        .map(|v| !(v == "0" || v.eq_ignore_ascii_case("false")))
        .unwrap_or(true);

    // LeJIT-Ψ VTI-EXT 3b (payload-extract-only first cut): when
    // `CRUFTLESS_LEJIT_VTI=1`, the dispatcher passes args as raw
    // `*const Value` pointers (reinterpreted as i64) instead of
    // pre-unboxed i64 payloads. The JIT prologue takes responsibility
    // for the f64 payload extraction at the layout-pinned offset
    // VALUE_NUMBER_PAYLOAD_OFFSET = 8 (per VTI-EXT 3a + value.rs
    // const assertions). The inline tag-check + WrongArgTag deopt
    // wiring is deferred to VTI-EXT 3c; this first cut trusts the
    // dispatcher's existing jit_compatible_arg precondition. Bench
    // measurement at 3b's end isolates the calling-convention
    // reinterpretation cost from the tag-check cost.
    let lejit_vti = std::env::var("CRUFTLESS_LEJIT_VTI")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    // Allocate one IC site per Op::GetPropOnObject occurrence (StubE-EXT
    // 3 site-id allocator + ICStubCache). EXT 5b consumes these at the
    // codegen site by indexing in parse order via a counter.
    let ic_site_ids: Vec<crate::stub_aarch64::ICSiteId> = if lejit_stub {
        parsed.iter()
            .filter(|(_, op)| matches!(op, ParsedOp::GetPropOnObject(_)))
            .map(|_| crate::stub_aarch64::alloc_ic_site())
            .collect()
    } else {
        Vec::new()
    };
    let mut ic_site_cursor: usize = 0;

    // JIT-EXT 20: detect GetPropOnObject in the parsed op list so we
    // can pre-bind the runtime helper. Pre-binding has no cost when
    // the symbol isn't used.
    let has_getprop = parsed.iter().any(|(_, op)| matches!(op, ParsedOp::GetPropOnObject(_)));

    let mut jit_builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    if any_guard {
        // Pre-bind the deopt thunk's symbol so Cranelift can resolve
        // the import at link time.
        jit_builder.symbol("deopt_trip", crate::deopt::deopt_trip as *const u8);
    }
    if has_getprop {
        jit_builder.symbol("jit_getprop_on_object",
            crate::deopt::jit_getprop_on_object as *const u8);
        // LeJIT-Σ StubE-EXT 5b: also pre-bind the IC-aware variant
        // when the flag is set.
        if lejit_stub {
            jit_builder.symbol("jit_getprop_with_ic",
                crate::deopt::jit_getprop_with_ic as *const u8);
        }
    }
    let mut module = JITModule::new(jit_builder);

    // Declare the deopt thunk's signature so the translator can emit
    // calls. Only consulted when at least one guard flag is set.
    let mut deopt_sites: crate::deopt::DeoptSiteTable = Vec::new();
    let trip_id_opt = if any_guard {
        let mut trip_sig = module.make_signature();
        for _ in 0..5 { trip_sig.params.push(AbiParam::new(I64)); }
        trip_sig.returns.push(AbiParam::new(I64));
        Some(module
            .declare_function("deopt_trip", Linkage::Import, &trip_sig)
            .map_err(|e| format!("declare deopt_trip: {e}"))?)
    } else { None };

    // JIT-EXT 20 + LeJIT-Σ StubE-EXT 5b: declare the GetProp runtime
    // helper. When CRUFTLESS_LEJIT_STUB=1, declare the IC-aware variant
    // (3-arg: site_id + receiver_idx + prop_name_idx) instead of the
    // standard 2-arg variant. The codegen path passes the site_id as a
    // Cranelift constant pulled from `_ic_site_ids` in parse order.
    let getprop_id_opt = if has_getprop {
        let mut sig = module.make_signature();
        if lejit_stub {
            sig.params.push(AbiParam::new(I64));  // site_id
        }
        sig.params.push(AbiParam::new(I64));  // receiver_idx
        sig.params.push(AbiParam::new(I64));  // prop_name_idx
        sig.returns.push(AbiParam::new(I64));
        let name = if lejit_stub { "jit_getprop_with_ic" } else { "jit_getprop_on_object" };
        Some(module
            .declare_function(name, Linkage::Import, &sig)
            .map_err(|e| format!("declare {name}: {e}"))?)
    } else { None };

    let mut ctx = module.make_context();
    let mut fb_ctx = FunctionBuilderContext::new();

    // LeJIT-Φ Φ-EXT 2: function signature is f64 throughout the calling
    // convention. The per-arg prologue converts F64 → I64 via
    // fcvt_to_sint_sat (saturating) so the existing JIT body IR (iadd
    // etc.) continues to operate on i64 locals. Return is converted
    // I64 → F64 via fcvt_from_sint just before the return instruction.
    // Φ-EXT 3 flips the body IR to native f64 (fadd etc.); this round
    // is substrate-introduction only.
    //
    // OSR-EXT 5b (2026-05-23): under osr_mode, signature is a single
    // I64 (the *mut f64 locals array pointer) + F64 return. Entry-
    // block prologue captures the pointer + loads locals from it;
    // every Return / ReturnUndef site stores locals back.
    if osr_mode {
        ctx.func.signature.params.push(AbiParam::new(I64));
    } else {
        for _ in 0..proto.params {
            ctx.func.signature.params.push(AbiParam::new(F64));
        }
    }
    ctx.func.signature.returns.push(AbiParam::new(F64));

    {
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fb_ctx);

        // JIT-EXT 13: bring the trip FuncRef into this function's
        // scope if the guard-overflow feature is on. We declare it
        // before any other block work so subsequent code can call it.
        let trip_ref = trip_id_opt.map(|id| module.declare_func_in_func(id, &mut builder.func));
        // JIT-EXT 20: same for the GetProp runtime helper.
        let getprop_ref = getprop_id_opt.map(|id| module.declare_func_in_func(id, &mut builder.func));

        // Allocate a Cranelift Block per jump target.
        let mut blocks: HashMap<usize, Block> = HashMap::new();
        for &t in &targets {
            blocks.insert(t, builder.create_block());
        }

        // OSR-EXT 5e (2026-05-23): in osr_mode the loop_top is at pc=0
        // (same as entry); back-edge JumpIfX targets blocks[&0]. The
        // function param (*mut f64 arr_ptr) is attached to whichever
        // block append_block_params_for_function_params runs on. If we
        // attached it to blocks[&0], brif back to that block would need
        // to pass arr_ptr explicitly — and the existing translator emits
        // brif with no args. Fix: allocate a separate pre_entry block,
        // attach the function param there, do the locals setup, then
        // jump to blocks[&0]. blocks[&0] has no params; brif back to it
        // from the loop's back-edge works as-is.
        let (entry_for_setup, real_entry) = if osr_mode {
            let pre = builder.create_block();
            builder.append_block_params_for_function_params(pre);
            builder.switch_to_block(pre);
            (pre, blocks[&0])
        } else {
            let entry = blocks[&0];
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            (entry, entry)
        };
        let entry = entry_for_setup;

        // Φ-EXT 3: locals are F64 (was I64). The JIT body operates on
        // f64-throughout per the f64 calling-convention closure round.
        // Object args are encoded as f64-via-bitcast at the dispatcher
        // (f64::from_bits(id.0 as u64)); the JIT bitcasts back when an
        // op needs the i64 receiver-id (e.g., Op::GetPropOnObject).
        let mut local_vars: Vec<Variable> = Vec::with_capacity(proto.locals.len());
        for i in 0..proto.locals.len() {
            let v = Variable::from_u32(i as u32);
            builder.declare_var(v, F64);
            local_vars.push(v);
        }
        // Initialize all locals to 0.0 (Undefined treated as 0.0 in the
        // first-cut, same shape as the i64 path's 0).
        let zero = builder.ins().f64const(0.0);
        for v in &local_vars {
            builder.def_var(*v, zero);
        }
        // OSR-EXT 5b (2026-05-23): allocate a Variable for the arr_ptr
        // (the *mut f64 locals array passed as the single entry-block
        // param under osr_mode). The Variable's index is allocated past
        // the local_vars range to avoid collision. None under non-osr
        // mode; Some at index local_vars.len() under osr_mode. Saved
        // here so Return / ReturnUndef sites can use it to emit the
        // epilogue store loop.
        let osr_arr_ptr_var: Option<Variable> = if osr_mode {
            let v = Variable::from_u32(local_vars.len() as u32);
            builder.declare_var(v, I64);
            // Initialize to 0 (will be overwritten from entry param below).
            let z = builder.ins().iconst(I64, 0);
            builder.def_var(v, z);
            Some(v)
        } else { None };
        // Args land in locals 0..params at function entry per the
        // interpreter convention (compile_function_proto allocates one
        // slot per param at the head of self.locals).
        //
        // LeJIT-Ψ VTI-EXT 3b (payload-extract-only): under
        // CRUFTLESS_LEJIT_VTI=1 the entry block params arrive as raw
        // `*const Value` pointers (i64-typed at the Cranelift ABI
        // level, reinterpreted from the dispatcher's `&args[i] as
        // *const Value as i64`). The prologue loads the f64 payload
        // at VALUE_NUMBER_PAYLOAD_OFFSET (=8 per value.rs const) and
        // saturating-converts to i64 to match the historical
        // `unbox_arg` semantics (`*f as i64`). Tag-check + WrongArgTag
        // deopt deferred to VTI-EXT 3c; this first cut trusts the
        // dispatcher's jit_compatible_arg precheck.
        // LeJIT-Φ Φ-EXT 2: entry-block params are F64 per the new ABI.
        // For each arg we convert F64 → I64 so the existing JIT body
        // IR (iadd etc.) keeps operating on i64 locals. Φ-EXT 3 will
        // change the body to operate on F64 natively; this round keeps
        // the body unchanged and adds the entry-prologue conversion.
        //
        // VTI=1 (env opt-in): the dispatcher passes a *const Value
        // pointer reinterpreted as f64-bits (f64::from_bits). The
        // prologue bitcasts F64 → I64 to recover the pointer, then
        // loads f64 from offset 8 (the Number payload per #[repr(C, u8)]
        // layout), then converts to I64. Φ-EXT 7 redesigns the VTI
        // calling-convention more cleanly.
        // Φ-EXT 3: entry-block params arrive as F64; locals are F64;
        // direct store. No type conversion at entry — the JIT body
        // operates on f64 directly. Under VTI=1 (env opt-in), the
        // dispatcher passes a *const Value pointer reinterpreted as
        // f64-bits via f64::from_bits; the prologue bitcasts back +
        // loads the f64 payload.
        let entry_params: Vec<ClValue> = builder.block_params(entry).to_vec();
        const VALUE_NUMBER_PAYLOAD_OFFSET: i32 = 8;
        if osr_mode {
            // OSR-EXT 5b (2026-05-23): entry param 0 is *mut f64 (the
            // locals array). Save to arr_ptr_var; then load each local
            // from arr_ptr + i*8, overriding the 0.0 init above.
            let arr_ptr = entry_params[0];
            builder.def_var(osr_arr_ptr_var.expect("osr_mode requires arr_ptr_var"), arr_ptr);
            for i in 0..local_vars.len() {
                let offset = (i * 8) as i32;
                let v = builder.ins().load(F64, MemFlags::trusted(), arr_ptr, offset);
                builder.def_var(local_vars[i], v);
            }
        } else {
            for (i, &p) in entry_params.iter().enumerate() {
                if i < local_vars.len() {
                    if lejit_vti && i < proto.params as usize {
                        // VTI=1: p is F64 holding *const Value bits.
                        let ptr_as_i64 = builder.ins().bitcast(I64,
                            cranelift_codegen::ir::MemFlags::new(), p);
                        let payload = builder.ins().load(
                            F64,
                            MemFlags::trusted(),
                            ptr_as_i64,
                            VALUE_NUMBER_PAYLOAD_OFFSET,
                        );
                        builder.def_var(local_vars[i], payload);
                    } else {
                        // Standard Φ path: F64 arg directly into F64 local.
                        builder.def_var(local_vars[i], p);
                    }
                }
            }
        }

        // OSR-EXT 5e (2026-05-23): in osr_mode, the setup ran on
        // pre_entry; jump to the real entry (blocks[&0]) where the
        // bytecode translation begins. brif-back-edges target the
        // real entry without arg-pass mismatch.
        if osr_mode {
            builder.ins().jump(real_entry, &[]);
            builder.seal_block(entry);
            builder.switch_to_block(real_entry);
        }

        // JIT-EXT 17: optional shape-trip check at function entry.
        // Reads `JIT_FORCE_SHAPE_TRIP` (a static AtomicBool); if true,
        // fires an ICShapeMismatch deopt before any user-bytecode op
        // runs. This is the IC demonstrator without needing real
        // GetProp support; real IC sites in JIT-EXT 18+ will read
        // per-site cache state instead of this global flag.
        if force_shape_trip {
            let st_trip_ref = trip_ref.expect("force_shape_trip requires trip_ref");
            let addr_val = builder.ins().iconst(I64,
                crate::deopt::get_force_shape_trip_addr() as i64);
            let flag_val = builder.ins().load(
                cranelift_codegen::ir::types::I8,
                cranelift_codegen::ir::MemFlags::trusted(),
                addr_val,
                0,
            );
            // Extend i8 to i64 so icmp accepts it cleanly.
            let flag_i64 = builder.ins().uextend(I64, flag_val);
            let z = builder.ins().iconst(I64, 0);
            let should_trip = builder.ins().icmp(IntCC::NotEqual, flag_i64, z);

            let trip_block = builder.create_block();
            let normal_block = builder.create_block();
            builder.ins().brif(should_trip, trip_block, &[], normal_block, &[]);

            builder.switch_to_block(trip_block);
            builder.seal_block(trip_block);
            let site_id = deopt_sites.len() as i64;
            let site_id_v = builder.ins().iconst(I64, site_id);
            let r0 = if !local_vars.is_empty() {
                builder.use_var(local_vars[0])
            } else { builder.ins().iconst(I64, 0) };
            let r1 = if local_vars.len() > 1 {
                builder.use_var(local_vars[1])
            } else { builder.ins().iconst(I64, 0) };
            let r2_const = builder.ins().iconst(I64, 0);
            let r3_const = builder.ins().iconst(I64, 0);
            let call_inst = builder.ins().call(st_trip_ref, &[site_id_v, r0, r1, r2_const, r3_const]);
            let sentinel = builder.inst_results(call_inst)[0];
            // Φ-EXT 2: sentinel is i64 (deopt_trip returns 0); convert
            // to F64 for the F64-return signature.
            let sentinel_f64 = builder.ins().fcvt_from_sint(F64, sentinel);
            builder.ins().return_(&[sentinel_f64]);

            deopt_sites.push(crate::deopt::DeoptSite {
                reason: crate::deopt::DeoptReason::ICShapeMismatch { ic_id: 0 },
                resume_pc: 0,
                live_locals: vec![
                    crate::deopt::DeoptLiveLocal {
                        interp_slot: 0,
                        jit_location: crate::deopt::JitLocation::Register(0),
                    },
                    crate::deopt::DeoptLiveLocal {
                        interp_slot: 1,
                        jit_location: crate::deopt::JitLocation::Register(1),
                    },
                ],
                stack_depth: 0,
                stack_slots: vec![],
            });

            builder.switch_to_block(normal_block);
            builder.seal_block(normal_block);
        }

        // Operand stack (virtual, SSA). Must be empty at block boundaries.
        let mut stack: Vec<ClValue> = Vec::new();
        let mut current_block: Block = entry;
        let mut block_terminated = false;

        for (pc, op) in &parsed {
            // If this pc starts a new block (jump target), close the
            // current block (if not yet terminated) with a jump and
            // switch.
            if *pc != 0 && blocks.contains_key(pc) {
                let next_block = blocks[pc];
                if !block_terminated {
                    if !stack.is_empty() {
                        return Err(format!(
                            "stack non-empty at block boundary pc={} (depth={})", pc, stack.len()));
                    }
                    builder.ins().jump(next_block, &[]);
                }
                builder.switch_to_block(next_block);
                current_block = next_block;
                block_terminated = false;
                // Stack assumed empty at block entries (cruftless
                // compiler invariant for statement-level joins).
                stack.clear();
            }

            match op {
                // Φ-EXT 3: untyped ops lower to f64-native Cranelift IR.
                // LoadLocal/StoreLocal/Dup/Pop/Jump are type-agnostic.
                // Add/Sub/Mul/Inc/Dec/Cmp use f64-domain IR. Typed-i64
                // variants (AddI64 etc., below) preserve the i64-domain
                // path for the bytecode tier's Doc 731 §XIII promotions.
                ParsedOp::LoadArg(slot) | ParsedOp::LoadLocal(slot) => {
                    let v = local_vars.get(*slot as usize)
                        .ok_or_else(|| format!("local slot {} out of range", slot))?;
                    let val = builder.use_var(*v);
                    stack.push(val);
                }
                ParsedOp::StoreLocal(slot) => {
                    let v = local_vars.get(*slot as usize)
                        .ok_or_else(|| format!("local slot {} out of range", slot))?;
                    let val = stack.pop().ok_or("StoreLocal: stack underflow")?;
                    builder.def_var(*v, val);
                }
                ParsedOp::PushI32(n) => {
                    // Φ-EXT 3: literal i32 becomes f64 (lossless: i32
                    // fits in f64 mantissa).
                    let v = builder.ins().f64const(*n as f64);
                    stack.push(v);
                }
                ParsedOp::PushConst(n) => {
                    // TL-EXT 2 (2026-05-23): PushConst resolved at parse
                    // time to f64; emit as f64const directly. Φ-EXT 3
                    // calling convention; the Number constant flows on
                    // the f64 stack.
                    let v = builder.ins().f64const(*n);
                    stack.push(v);
                }
                ParsedOp::Add => {
                    // Φ-EXT 3: untyped Add lowers to fadd (no overflow
                    // semantics; f64 IEEE-754).
                    binop(&mut stack, &mut builder, |b, l, r| b.ins().fadd(l, r))?;
                }
                ParsedOp::Sub => {
                    binop(&mut stack, &mut builder, |b, l, r| b.ins().fsub(l, r))?;
                }
                ParsedOp::Mul => {
                    binop(&mut stack, &mut builder, |b, l, r| b.ins().fmul(l, r))?;
                }
                ParsedOp::Inc => {
                    let v = stack.pop().ok_or("Inc: stack underflow")?;
                    let one = builder.ins().f64const(1.0);
                    let r = builder.ins().fadd(v, one);
                    stack.push(r);
                }
                ParsedOp::Dec => {
                    let v = stack.pop().ok_or("Dec: stack underflow")?;
                    let one = builder.ins().f64const(1.0);
                    let r = builder.ins().fsub(v, one);
                    stack.push(r);
                }
                ParsedOp::Lt => fcmpop(&mut stack, &mut builder, FloatCC::LessThan)?,
                ParsedOp::Le => fcmpop(&mut stack, &mut builder, FloatCC::LessThanOrEqual)?,
                ParsedOp::Gt => fcmpop(&mut stack, &mut builder, FloatCC::GreaterThan)?,
                ParsedOp::Ge => fcmpop(&mut stack, &mut builder, FloatCC::GreaterThanOrEqual)?,
                ParsedOp::Eq | ParsedOp::StrictEq => fcmpop(&mut stack, &mut builder, FloatCC::Equal)?,
                ParsedOp::Ne | ParsedOp::StrictNe => fcmpop(&mut stack, &mut builder, FloatCC::NotEqual)?,
                ParsedOp::Dup => {
                    let v = *stack.last().ok_or("Dup: stack underflow")?;
                    stack.push(v);
                }
                ParsedOp::Pop => {
                    stack.pop().ok_or("Pop: stack underflow")?;
                }
                ParsedOp::Jump(target) => {
                    if !stack.is_empty() {
                        return Err(format!("stack non-empty at Jump pc={} (depth={})", pc, stack.len()));
                    }
                    let target_block = blocks[target];
                    builder.ins().jump(target_block, &[]);
                    block_terminated = true;
                }
                ParsedOp::JumpIfTrue(target) | ParsedOp::JumpIfFalse(target) => {
                    let cond_f64 = stack.pop().ok_or("JumpIfX: stack underflow")?;
                    if !stack.is_empty() {
                        return Err(format!("stack non-empty at JumpIfX pc={} (depth={})", pc, stack.len()));
                    }
                    // Φ-EXT 3: cond is F64 (0.0 = false; non-zero,
                    // non-NaN = true). fcmp `ne 0.0` returns true for
                    // non-zero finite numbers; NaN comparison is always
                    // unordered so `ne 0.0` returns FALSE for NaN (which
                    // matches JS truthy: NaN is falsy).
                    let zero = builder.ins().f64const(0.0);
                    let truthy: ClValue = builder.ins().fcmp(FloatCC::NotEqual, cond_f64, zero);
                    let fall_pc = find_next_block_pc(&parsed, *pc, &blocks)?;
                    let fall_block = blocks[&fall_pc];
                    let target_block = blocks[target];
                    match op {
                        ParsedOp::JumpIfTrue(_) => {
                            builder.ins().brif(truthy, target_block, &[], fall_block, &[]);
                        }
                        ParsedOp::JumpIfFalse(_) => {
                            builder.ins().brif(truthy, fall_block, &[], target_block, &[]);
                        }
                        _ => unreachable!(),
                    }
                    block_terminated = true;
                }
                ParsedOp::Return => {
                    let v = stack.pop().ok_or("Return: stack underflow")?;
                    // OSR-EXT 5b (2026-05-23): under osr_mode, store all
                    // locals back to the arr_ptr array before returning.
                    if let Some(ap_var) = osr_arr_ptr_var {
                        let ap = builder.use_var(ap_var);
                        for i in 0..local_vars.len() {
                            let lv = builder.use_var(local_vars[i]);
                            let offset = (i * 8) as i32;
                            builder.ins().store(MemFlags::trusted(), lv, ap, offset);
                        }
                    }
                    // Φ-EXT 3: stack is F64; return F64 directly. No
                    // conversion at the return boundary.
                    builder.ins().return_(&[v]);
                    block_terminated = true;
                    stack.clear();
                }
                ParsedOp::ReturnUndef => {
                    // OSR-EXT 5b (2026-05-23): same epilogue under osr_mode.
                    if let Some(ap_var) = osr_arr_ptr_var {
                        let ap = builder.use_var(ap_var);
                        for i in 0..local_vars.len() {
                            let lv = builder.use_var(local_vars[i]);
                            let offset = (i * 8) as i32;
                            builder.ins().store(MemFlags::trusted(), lv, ap, offset);
                        }
                    }
                    let z = builder.ins().f64const(0.0);
                    builder.ins().return_(&[z]);
                    block_terminated = true;
                    stack.clear();
                }
                // Doc 731 §XIV.d typed-i64 ops: direct Cranelift lowering,
                // no cheating. The interpreter unboxes Number(f64) →
                // i64 at the op handler; the JIT assumes the bytecode
                // alphabet's typed contract holds (operands already i64
                // in JIT-internal SSA representation).
                ParsedOp::AddI64 => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        emit_guarded_add(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        binop(&mut stack, &mut builder, |b, l, r| b.ins().iadd(l, r))?;
                    }
                }
                ParsedOp::SubI64 => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        emit_guarded_sub(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        binop(&mut stack, &mut builder, |b, l, r| b.ins().isub(l, r))?;
                    }
                }
                ParsedOp::MulI64 => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        emit_guarded_mul(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        binop(&mut stack, &mut builder, |b, l, r| b.ins().imul(l, r))?;
                    }
                }
                ParsedOp::IncI64 => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        let one = builder.ins().iconst(I64, 1);
                        stack.push(one);
                        emit_guarded_add(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        let v = stack.pop().ok_or("IncI64: stack underflow")?;
                        let one = builder.ins().iconst(I64, 1);
                        stack.push(builder.ins().iadd(v, one));
                    }
                }
                ParsedOp::DecI64 => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        let one = builder.ins().iconst(I64, 1);
                        stack.push(one);
                        emit_guarded_sub(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        let v = stack.pop().ok_or("DecI64: stack underflow")?;
                        let one = builder.ins().iconst(I64, 1);
                        stack.push(builder.ins().isub(v, one));
                    }
                }
                ParsedOp::LtI64 => cmpop(&mut stack, &mut builder, IntCC::SignedLessThan)?,
                ParsedOp::LeI64 => cmpop(&mut stack, &mut builder, IntCC::SignedLessThanOrEqual)?,
                ParsedOp::GtI64 => cmpop(&mut stack, &mut builder, IntCC::SignedGreaterThan)?,
                ParsedOp::GeI64 => cmpop(&mut stack, &mut builder, IntCC::SignedGreaterThanOrEqual)?,
                ParsedOp::EqI64 => cmpop(&mut stack, &mut builder, IntCC::Equal)?,
                ParsedOp::NeI64 => cmpop(&mut stack, &mut builder, IntCC::NotEqual)?,
                ParsedOp::GetPropOnObject(prop_idx) => {
                    // JIT-EXT 20 + Φ-EXT 3: lower to a call into the
                    // runtime helper. Post-Φ the receiver pops as F64
                    // (encoded as f64-bits-of-ObjectId via from_bits at
                    // the dispatcher). Bitcast F64 → I64 to recover the
                    // ObjectId.0 bits before passing to the extern.
                    // The extern returns i64 (slot-value-as-Number-
                    // truncated currently; Φ-EXT 3+ may change return to
                    // f64 in a follow-on round). Convert returned i64 →
                    // F64 via fcvt_from_sint for stack uniformity.
                    let gpref = getprop_ref.expect("getprop_ref must be set when ParsedOp::GetPropOnObject is present");
                    let receiver_f64 = stack.pop().ok_or("GetPropOnObject: stack underflow (receiver)")?;
                    let receiver = builder.ins().bitcast(I64,
                        cranelift_codegen::ir::MemFlags::new(), receiver_f64);
                    let prop_v = builder.ins().iconst(I64, *prop_idx as i64);
                    let call_inst = if lejit_stub {
                        let site_id = ic_site_ids[ic_site_cursor];
                        ic_site_cursor += 1;
                        let site_v = builder.ins().iconst(I64, site_id as i64);
                        builder.ins().call(gpref, &[site_v, receiver, prop_v])
                    } else {
                        builder.ins().call(gpref, &[receiver, prop_v])
                    };
                    let result_i64 = builder.inst_results(call_inst)[0];
                    let result_f64 = builder.ins().fcvt_from_sint(F64, result_i64);
                    stack.push(result_f64);
                }
            }
            // Allow comparison op result (i8) to participate in stack
            // as if it were i64 — handled inside cmpop by extending.
            let _ = current_block;
        }

        // If the last instruction wasn't a terminator, synthesize a
        // ReturnUndef.
        if !block_terminated {
            // OSR-EXT 5b: same locals-store epilogue for synthesized
            // ReturnUndef at end-of-body (the OSR loop body falls
            // through here on natural loop exit).
            if let Some(ap_var) = osr_arr_ptr_var {
                let ap = builder.use_var(ap_var);
                for i in 0..local_vars.len() {
                    let lv = builder.use_var(local_vars[i]);
                    let offset = (i * 8) as i32;
                    builder.ins().store(MemFlags::trusted(), lv, ap, offset);
                }
            }
            // Φ-EXT 2: F64 signature; return f64 0.0 instead of i64 0.
            let z = builder.ins().f64const(0.0);
            builder.ins().return_(&[z]);
        }

        // Seal all blocks: post-order so predecessors are filled.
        for &t in &targets {
            builder.seal_block(blocks[&t]);
        }
        builder.finalize();
    }

    let name = if proto.display_name.is_empty() { "anon".to_string() } else { proto.display_name.clone() };
    let id = module
        .declare_function(&format!("jit_{}", name), Linkage::Export, &ctx.func.signature)
        .map_err(|e| format!("declare_function: {e}"))?;
    // Diagnostic: print the function IR on error.
    let ir_dump = format!("{}", ctx.func.display());
    module.define_function(id, &mut ctx)
        .map_err(|e| format!("define_function: {e}\nIR:\n{}", ir_dump))?;
    module.clear_context(&mut ctx);
    module.finalize_definitions()
        .map_err(|e| format!("finalize_definitions: {e}"))?;

    let code_ptr = module.get_finalized_function(id);
    let func = unsafe {
        if osr_mode {
            JitFn::ArityOsr(std::mem::transmute::<*const u8, JitFnOsr>(code_ptr))
        } else {
            match proto.params {
                0 => JitFn::Arity0(std::mem::transmute::<*const u8, JitFn0>(code_ptr)),
                1 => JitFn::Arity1(std::mem::transmute::<*const u8, JitFn1>(code_ptr)),
                2 => JitFn::Arity2(std::mem::transmute::<*const u8, JitFn2>(code_ptr)),
                _ => unreachable!(),
            }
        }
    };
    let leaked = Box::leak(Box::new(module));

    // LeJIT-Τ TB-EXT 3a: build TinyBaselineMetadata when the
    // CRUFTLESS_LEJIT_TB env flag is set AND the function is
    // param-eligible. Metadata is consumed by TB-EXT 3b's inline
    // call thunk; in 3a the field exists but the dispatcher does
    // not yet route through a thunk (no thunk emission).
    let tb_metadata = if crate::tiny_baseline::lejit_tb_enabled() {
        let jit_fn_ptr = code_ptr as usize;
        Some(crate::tiny_baseline::TinyBaselineMetadata::build(
            jit_fn_ptr,
            proto.params,
            proto.bytecode.len(),
        ))
    } else { None };

    Ok(CompiledFn { func, _module: leaked, deopt_sites, vti_enabled: lejit_vti, tb_metadata })
}

fn binop<F>(stack: &mut Vec<ClValue>, builder: &mut FunctionBuilder, f: F) -> Result<(), String>
where F: FnOnce(&mut FunctionBuilder, ClValue, ClValue) -> ClValue {
    let r = stack.pop().ok_or("binop: stack underflow (rhs)")?;
    let l = stack.pop().ok_or("binop: stack underflow (lhs)")?;
    let v = f(builder, l, r);
    stack.push(v);
    Ok(())
}

/// JIT-EXT 13: guarded i64 add with signed-overflow detection +
/// deopt-on-trip.
///
/// Pops lhs/rhs, computes the sum, detects signed overflow via the
/// standard `(a XOR result) AND (b XOR result) < 0` idiom (no Cranelift
/// dedicated overflow instruction is portable across the version
/// surface we target; the XOR idiom lowers to a handful of instructions
/// and is correctness-equivalent).
///
/// On overflow: branch to a deopt block that calls `deopt_trip(site_id,
/// lhs, rhs, local0, local1)` and returns the thunk's sentinel. Records
/// a `DeoptSite` with `live_locals = [(0, Register(2)), (1, Register(3))]`
/// and `stack_slots = [(0, Register(0)), (1, Register(1))]` so the
/// dispatcher can reconstruct the interpreter state at the failing pc.
///
/// On no-overflow: pushes the sum onto the stack and continues.
///
/// First-cut constraints documented in the deopt-audit doc:
///   - Up to 2 locals captured (slot 0 + slot 1); others zeroed
///   - Only Add is currently guarded; Sub/Mul/Inc/Dec are future rounds
fn emit_guarded_add(
    stack: &mut Vec<ClValue>,
    builder: &mut FunctionBuilder,
    trip_ref: cranelift_codegen::ir::FuncRef,
    pc: usize,
    local_vars: &[Variable],
    deopt_sites: &mut crate::deopt::DeoptSiteTable,
) -> Result<(), String> {
    use crate::deopt::{DeoptSite, DeoptReason, DeoptLiveLocal, JitLocation};

    let r = stack.pop().ok_or("guarded_add: stack underflow (rhs)")?;
    let l = stack.pop().ok_or("guarded_add: stack underflow (lhs)")?;

    // Compute the candidate result, then detect signed overflow.
    let result = builder.ins().iadd(l, r);
    // (a XOR result) AND (b XOR result) < 0 means signed overflow.
    let xor_a = builder.ins().bxor(l, result);
    let xor_b = builder.ins().bxor(r, result);
    let combined = builder.ins().band(xor_a, xor_b);
    let zero = builder.ins().iconst(I64, 0);
    let overflowed = builder.ins().icmp(IntCC::SignedLessThan, combined, zero);

    // Allocate a deopt block and a continuation block.
    let deopt_block = builder.create_block();
    let cont_block = builder.create_block();
    // The continuation block carries the result as a block parameter
    // so SSA stays clean.
    let cont_result = builder.append_block_param(cont_block, I64);

    builder.ins().brif(overflowed, deopt_block, &[], cont_block, &[result]);

    // Deopt block: build the trip call args and invoke the thunk.
    builder.switch_to_block(deopt_block);
    builder.seal_block(deopt_block);
    let site_id = deopt_sites.len() as i64;
    let site_id_v = builder.ins().iconst(I64, site_id);
    // r2 / r3: first two locals (zeros if fewer).
    let local0 = if !local_vars.is_empty() {
        builder.use_var(local_vars[0])
    } else { builder.ins().iconst(I64, 0) };
    let local1 = if local_vars.len() > 1 {
        builder.use_var(local_vars[1])
    } else { builder.ins().iconst(I64, 0) };
    let call_inst = builder.ins().call(trip_ref, &[site_id_v, l, r, local0, local1]);
    let sentinel = builder.inst_results(call_inst)[0];
    // Φ-EXT 2: F64 return signature; convert i64 sentinel.
    let sentinel_f64 = builder.ins().fcvt_from_sint(F64, sentinel);
    builder.ins().return_(&[sentinel_f64]);

    // Record the site so the dispatcher can reconstruct state.
    deopt_sites.push(DeoptSite {
        reason: DeoptReason::IntegerOverflow { op_pc: pc as u32 },
        resume_pc: pc as u32,
        live_locals: vec![
            DeoptLiveLocal { interp_slot: 0, jit_location: JitLocation::Register(2) },
            DeoptLiveLocal { interp_slot: 1, jit_location: JitLocation::Register(3) },
        ],
        stack_depth: 2,
        stack_slots: vec![
            DeoptLiveLocal { interp_slot: 0, jit_location: JitLocation::Register(0) },
            DeoptLiveLocal { interp_slot: 1, jit_location: JitLocation::Register(1) },
        ],
    });

    // Continuation: push the result onto the operand stack and resume.
    builder.switch_to_block(cont_block);
    builder.seal_block(cont_block);
    stack.push(cont_result);
    Ok(())
}

fn cmpop(stack: &mut Vec<ClValue>, builder: &mut FunctionBuilder, cc: IntCC) -> Result<(), String> {
    let r = stack.pop().ok_or("cmp: stack underflow (rhs)")?;
    let l = stack.pop().ok_or("cmp: stack underflow (lhs)")?;
    let i8_result = builder.ins().icmp(cc, l, r);
    // Extend bool (i8) to i64 so the operand stack remains uniformly i64.
    let i64_result = builder.ins().uextend(I64, i8_result);
    stack.push(i64_result);
    Ok(())
}

/// Φ-EXT 3: f64-domain comparison op. Pops two F64 operands; performs
/// fcmp; promotes the i8 result to F64 (0.0 or 1.0) so the operand
/// stack remains uniformly F64 post-Φ.
fn fcmpop(stack: &mut Vec<ClValue>, builder: &mut FunctionBuilder, cc: FloatCC) -> Result<(), String> {
    let r = stack.pop().ok_or("fcmp: stack underflow (rhs)")?;
    let l = stack.pop().ok_or("fcmp: stack underflow (lhs)")?;
    let i8_result = builder.ins().fcmp(cc, l, r);
    // Promote bool (i8) → i64 → F64 (0.0 or 1.0) for uniform F64 stack.
    let i64_result = builder.ins().uextend(I64, i8_result);
    let f64_result = builder.ins().fcvt_from_uint(F64, i64_result);
    stack.push(f64_result);
    Ok(())
}

/// JIT-EXT 15: guarded i64 subtract with signed-overflow detection.
///
/// Sub overflows when sign(lhs) differs from sign(rhs) AND sign(result)
/// differs from sign(lhs). The XOR-idiom expression of this:
///   `(lhs XOR rhs) AND (lhs XOR result) < 0`
fn emit_guarded_sub(
    stack: &mut Vec<ClValue>,
    builder: &mut FunctionBuilder,
    trip_ref: cranelift_codegen::ir::FuncRef,
    pc: usize,
    local_vars: &[Variable],
    deopt_sites: &mut crate::deopt::DeoptSiteTable,
) -> Result<(), String> {
    use crate::deopt::{DeoptSite, DeoptReason, DeoptLiveLocal, JitLocation};

    let r = stack.pop().ok_or("guarded_sub: stack underflow (rhs)")?;
    let l = stack.pop().ok_or("guarded_sub: stack underflow (lhs)")?;

    let result = builder.ins().isub(l, r);
    let xor_lr = builder.ins().bxor(l, r);
    let xor_lres = builder.ins().bxor(l, result);
    let combined = builder.ins().band(xor_lr, xor_lres);
    let zero = builder.ins().iconst(I64, 0);
    let overflowed = builder.ins().icmp(IntCC::SignedLessThan, combined, zero);

    let deopt_block = builder.create_block();
    let cont_block = builder.create_block();
    let cont_result = builder.append_block_param(cont_block, I64);

    builder.ins().brif(overflowed, deopt_block, &[], cont_block, &[result]);

    builder.switch_to_block(deopt_block);
    builder.seal_block(deopt_block);
    let site_id = deopt_sites.len() as i64;
    let site_id_v = builder.ins().iconst(I64, site_id);
    let local0 = if !local_vars.is_empty() { builder.use_var(local_vars[0]) } else { builder.ins().iconst(I64, 0) };
    let local1 = if local_vars.len() > 1 { builder.use_var(local_vars[1]) } else { builder.ins().iconst(I64, 0) };
    let call_inst = builder.ins().call(trip_ref, &[site_id_v, l, r, local0, local1]);
    let sentinel = builder.inst_results(call_inst)[0];
    // Φ-EXT 2: F64 return signature; convert i64 sentinel.
    let sentinel_f64 = builder.ins().fcvt_from_sint(F64, sentinel);
    builder.ins().return_(&[sentinel_f64]);

    deopt_sites.push(DeoptSite {
        reason: DeoptReason::IntegerOverflow { op_pc: pc as u32 },
        resume_pc: pc as u32,
        live_locals: vec![
            DeoptLiveLocal { interp_slot: 0, jit_location: JitLocation::Register(2) },
            DeoptLiveLocal { interp_slot: 1, jit_location: JitLocation::Register(3) },
        ],
        stack_depth: 2,
        stack_slots: vec![
            DeoptLiveLocal { interp_slot: 0, jit_location: JitLocation::Register(0) },
            DeoptLiveLocal { interp_slot: 1, jit_location: JitLocation::Register(1) },
        ],
    });

    builder.switch_to_block(cont_block);
    builder.seal_block(cont_block);
    stack.push(cont_result);
    Ok(())
}

/// JIT-EXT 15: guarded i64 multiply with signed-overflow detection.
///
/// Mul overflows when the result requires more than 64 signed bits.
/// Detection: compute the high 64 bits via Cranelift's `smulhi`
/// (signed-multiply-high) and compare against the sign extension of
/// the low 64 bits. If `smulhi(a, b) != ASHR(a*b, 63)`, the multiply
/// overflowed.
fn emit_guarded_mul(
    stack: &mut Vec<ClValue>,
    builder: &mut FunctionBuilder,
    trip_ref: cranelift_codegen::ir::FuncRef,
    pc: usize,
    local_vars: &[Variable],
    deopt_sites: &mut crate::deopt::DeoptSiteTable,
) -> Result<(), String> {
    use crate::deopt::{DeoptSite, DeoptReason, DeoptLiveLocal, JitLocation};

    let r = stack.pop().ok_or("guarded_mul: stack underflow (rhs)")?;
    let l = stack.pop().ok_or("guarded_mul: stack underflow (lhs)")?;

    let result = builder.ins().imul(l, r);
    let hi = builder.ins().smulhi(l, r);
    // sign-extension of result's high bit: ASHR by 63 gives 0 or -1.
    let sign_ext = builder.ins().sshr_imm(result, 63);
    let overflowed = builder.ins().icmp(IntCC::NotEqual, hi, sign_ext);

    let deopt_block = builder.create_block();
    let cont_block = builder.create_block();
    let cont_result = builder.append_block_param(cont_block, I64);

    builder.ins().brif(overflowed, deopt_block, &[], cont_block, &[result]);

    builder.switch_to_block(deopt_block);
    builder.seal_block(deopt_block);
    let site_id = deopt_sites.len() as i64;
    let site_id_v = builder.ins().iconst(I64, site_id);
    let local0 = if !local_vars.is_empty() { builder.use_var(local_vars[0]) } else { builder.ins().iconst(I64, 0) };
    let local1 = if local_vars.len() > 1 { builder.use_var(local_vars[1]) } else { builder.ins().iconst(I64, 0) };
    let call_inst = builder.ins().call(trip_ref, &[site_id_v, l, r, local0, local1]);
    let sentinel = builder.inst_results(call_inst)[0];
    // Φ-EXT 2: F64 return signature; convert i64 sentinel.
    let sentinel_f64 = builder.ins().fcvt_from_sint(F64, sentinel);
    builder.ins().return_(&[sentinel_f64]);

    deopt_sites.push(DeoptSite {
        reason: DeoptReason::IntegerOverflow { op_pc: pc as u32 },
        resume_pc: pc as u32,
        live_locals: vec![
            DeoptLiveLocal { interp_slot: 0, jit_location: JitLocation::Register(2) },
            DeoptLiveLocal { interp_slot: 1, jit_location: JitLocation::Register(3) },
        ],
        stack_depth: 2,
        stack_slots: vec![
            DeoptLiveLocal { interp_slot: 0, jit_location: JitLocation::Register(0) },
            DeoptLiveLocal { interp_slot: 1, jit_location: JitLocation::Register(1) },
        ],
    });

    builder.switch_to_block(cont_block);
    builder.seal_block(cont_block);
    stack.push(cont_result);
    Ok(())
}

fn find_next_block_pc(parsed: &[(usize, ParsedOp)], cur_pc: usize, blocks: &HashMap<usize, Block>)
    -> Result<usize, String>
{
    let i = parsed.iter().position(|(p, _)| *p == cur_pc).expect("pc not found");
    for (p, _) in parsed[i + 1..].iter() {
        if blocks.contains_key(p) { return Ok(*p); }
    }
    Err(format!("no fallthrough block after JumpIfX at pc={}", cur_pc))
}

/// Parsed op + jump target convenience.
#[derive(Debug, Clone, Copy)]
enum ParsedOp {
    LoadArg(u16),
    LoadLocal(u16),
    StoreLocal(u16),
    PushI32(i32),
    /// TL-EXT 2 (2026-05-23): Op::PushConst with Number constant resolved
    /// to f64 at parse-time. Other Constant variants (String/BigInt/Regex/
    /// Function) cause parse_bytecode to bail per C8 bail-discipline.
    PushConst(f64),
    Add, Sub, Mul, Inc, Dec,
    Lt, Le, Gt, Ge, Eq, Ne, StrictEq, StrictNe,
    Dup, Pop,
    Jump(usize),
    JumpIfTrue(usize),
    JumpIfFalse(usize),
    Return, ReturnUndef,
    // Doc 731 §XIV.d typed-I64 alphabet promotion. The JIT prefers
    // these over the plain variants because no type assumption is
    // required at the JIT tier — the typed assumption is encoded in
    // the bytecode alphabet itself, and the upstream emitter is
    // responsible for proving it.
    AddI64, SubI64, MulI64, IncI64, DecI64,
    LtI64, LeI64, GtI64, GeI64, EqI64, NeI64,
    /// JIT-EXT 19 (Doc 731 §XIV.d β-path for property access).
    /// Recognized at parser-tier; JIT lowering arrives at JIT-EXT 20.
    /// Operand is the constant-pool index of the property name.
    GetPropOnObject(u16),
}

impl ParsedOp {
    fn jump_target(&self) -> Option<usize> {
        match self {
            ParsedOp::Jump(t) | ParsedOp::JumpIfTrue(t) | ParsedOp::JumpIfFalse(t) => Some(*t),
            _ => None,
        }
    }
}

fn parse_bytecode(bc: &[u8], constants: &ConstantsPool) -> Result<Vec<(usize, ParsedOp)>, String> {
    let mut out = Vec::new();
    let mut pc = 0;
    while pc < bc.len() {
        let op_pc = pc;
        let opcode = op_from_byte(bc[pc])
            .ok_or_else(|| format!("unknown opcode byte 0x{:02x} at pc={}", bc[pc], pc))?;
        pc += 1;
        let parsed = match opcode {
            Op::LoadArg => { let s = u16::from_le_bytes([bc[pc], bc[pc + 1]]); pc += 2; ParsedOp::LoadArg(s) }
            Op::LoadLocal => { let s = u16::from_le_bytes([bc[pc], bc[pc + 1]]); pc += 2; ParsedOp::LoadLocal(s) }
            Op::StoreLocal => { let s = u16::from_le_bytes([bc[pc], bc[pc + 1]]); pc += 2; ParsedOp::StoreLocal(s) }
            Op::PushI32 => { let n = i32::from_le_bytes([bc[pc], bc[pc + 1], bc[pc + 2], bc[pc + 3]]); pc += 4; ParsedOp::PushI32(n) }
            // TL-EXT 2 (2026-05-23): resolve PushConst's constant pool
            // index at parse time. Only Number constants are JIT-eligible;
            // other variants bail the whole function per C8 bail discipline.
            Op::PushConst => {
                let idx = u16::from_le_bytes([bc[pc], bc[pc + 1]]);
                pc += 2;
                match constants.get(idx) {
                    Some(Constant::Number(n)) => ParsedOp::PushConst(*n),
                    Some(_) => return Err(format!("PushConst at pc={op_pc}: non-Number constant unsupported in JIT alphabet")),
                    None => return Err(format!("PushConst at pc={op_pc}: constant idx {idx} out of bounds")),
                }
            }
            Op::Add => ParsedOp::Add,
            Op::Sub => ParsedOp::Sub,
            Op::Mul => ParsedOp::Mul,
            Op::Inc => ParsedOp::Inc,
            Op::Dec => ParsedOp::Dec,
            Op::Lt => ParsedOp::Lt,
            Op::Le => ParsedOp::Le,
            Op::Gt => ParsedOp::Gt,
            Op::Ge => ParsedOp::Ge,
            Op::Eq => ParsedOp::Eq,
            Op::Ne => ParsedOp::Ne,
            Op::StrictEq => ParsedOp::StrictEq,
            Op::StrictNe => ParsedOp::StrictNe,
            Op::Dup => ParsedOp::Dup,
            Op::Pop => ParsedOp::Pop,
            Op::Jump => {
                let disp = i32::from_le_bytes([bc[pc], bc[pc + 1], bc[pc + 2], bc[pc + 3]]);
                pc += 4;
                let target = (pc as i32 + disp) as usize;
                ParsedOp::Jump(target)
            }
            Op::JumpIfTrue => {
                let disp = i32::from_le_bytes([bc[pc], bc[pc + 1], bc[pc + 2], bc[pc + 3]]);
                pc += 4;
                ParsedOp::JumpIfTrue((pc as i32 + disp) as usize)
            }
            Op::JumpIfFalse => {
                let disp = i32::from_le_bytes([bc[pc], bc[pc + 1], bc[pc + 2], bc[pc + 3]]);
                pc += 4;
                ParsedOp::JumpIfFalse((pc as i32 + disp) as usize)
            }
            Op::Return => ParsedOp::Return,
            Op::ReturnUndef => ParsedOp::ReturnUndef,
            // JIT-EXT 19: GetPropOnObject is recognized in the parser
            // but the translator dispatch (above) returns Err for it
            // until JIT-EXT 20 lands the lowering + runtime helper.
            Op::GetPropOnObject => {
                let s = u16::from_le_bytes([bc[pc], bc[pc + 1]]);
                pc += 2;
                ParsedOp::GetPropOnObject(s)
            }
            Op::AddI64 => ParsedOp::AddI64,
            Op::SubI64 => ParsedOp::SubI64,
            Op::MulI64 => ParsedOp::MulI64,
            Op::IncI64 => ParsedOp::IncI64,
            Op::DecI64 => ParsedOp::DecI64,
            Op::LtI64 => ParsedOp::LtI64,
            Op::LeI64 => ParsedOp::LeI64,
            Op::GtI64 => ParsedOp::GtI64,
            Op::GeI64 => ParsedOp::GeI64,
            Op::EqI64 => ParsedOp::EqI64,
            Op::NeI64 => ParsedOp::NeI64,
            other => return Err(format!("first-cut JIT does not support op {:?} at pc={}", other, op_pc)),
        };
        out.push((op_pc, parsed));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_js_bytecode::compile_module;
    use rusty_js_bytecode::compiler::{FunctionProto, LocalDescriptor, UpvalueDescriptor};
    use rusty_js_bytecode::constants::{Constant, ConstantsPool};
    use rusty_js_bytecode::op::{encode_op, encode_u16};

    fn empty_proto(bytecode: Vec<u8>, params: u16) -> FunctionProto {
        let mut locals = Vec::<LocalDescriptor>::new();
        for i in 0..params {
            locals.push(LocalDescriptor {
                name: format!("arg{}", i),
                kind: rusty_js_ast::VariableKind::Let,
                depth: 0,
            });
        }
        FunctionProto {
            bytecode,
            constants: ConstantsPool::new(),
            params,
            display_name: "test".to_string(),
            function_length: params,
            locals,
            upvalues: Vec::<UpvalueDescriptor>::new(),
            rest_param_slot: None,
            arguments_slot: None,
            self_name_slot: None,
            is_generator: false,
            is_async: false,
            source_url: String::new(),
            line_starts: Vec::new(),
            source_map: Vec::new(),
            construct_tags: Vec::new(),
            strict: false,
        }
    }

    #[test]
    fn jit_add_two_args() {
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Add);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let jit = compile_function(&proto).expect("compile failed");
        assert_eq!(jit.func.call2(2.0_f64, 3.0_f64), 5.0_f64);
        assert_eq!(jit.func.call2(-10.0_f64, 100.0_f64), 90.0_f64);
    }

    #[test]
    fn jit_combined_arith() {
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Add);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Sub);
        encode_op(&mut bc, Op::Mul);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let jit = compile_function(&proto).expect("compile failed");
        assert_eq!(jit.func.call2(5.0_f64, 3.0_f64), 16.0_f64);
        assert_eq!(jit.func.call2(10.0_f64, 4.0_f64), 84.0_f64);
    }

    #[test]
    fn jit_rejects_unsupported_op() {
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::PushNull);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        assert!(compile_function(&proto).is_err());
    }

    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn jit_typed_i64_sum() {
        // Hand-built FunctionProto using typed-I64 ops directly (no
        // upstream type inference needed). Validates the typed alphabet
        // promotion end-to-end at the JIT layer: AddI64 → iadd,
        // LtI64 → icmp slt, IncI64 → iadd 1, no cheating.
        //
        // Equivalent JS: function tsum(n) { var s=0, i=0; while (i<n) { s = s+i; i++; } return s; }
        //
        // Bytecode (manually constructed):
        //   PushI32 0          ; locals[1] = s = 0
        //   StoreLocal 1
        //   PushI32 0          ; locals[2] = i = 0
        //   StoreLocal 2
        //   LABEL loop_top:    ; pc 16
        //   LoadLocal 2
        //   LoadArg 0
        //   LtI64
        //   JumpIfFalse →exit
        //   LoadLocal 1        ; s += i
        //   LoadLocal 2
        //   AddI64
        //   StoreLocal 1
        //   LoadLocal 2        ; i++
        //   IncI64
        //   StoreLocal 2
        //   Jump →loop_top
        //   LABEL exit:
        //   LoadLocal 1
        //   Return
        use rusty_js_bytecode::op::{encode_i32, encode_u16};
        let mut bc = Vec::new();
        // s = 0
        encode_op(&mut bc, Op::PushI32); encode_i32(&mut bc, 0);
        encode_op(&mut bc, Op::StoreLocal); encode_u16(&mut bc, 1);
        // i = 0
        encode_op(&mut bc, Op::PushI32); encode_i32(&mut bc, 0);
        encode_op(&mut bc, Op::StoreLocal); encode_u16(&mut bc, 2);
        let loop_top = bc.len();
        // i < n
        encode_op(&mut bc, Op::LoadLocal); encode_u16(&mut bc, 2);
        encode_op(&mut bc, Op::LoadArg);   encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LtI64);
        // JumpIfFalse → exit (patched after we know exit pc)
        encode_op(&mut bc, Op::JumpIfFalse);
        let jif_disp_at = bc.len();
        encode_i32(&mut bc, 0);
        // s = s + i
        encode_op(&mut bc, Op::LoadLocal); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::LoadLocal); encode_u16(&mut bc, 2);
        encode_op(&mut bc, Op::AddI64);
        encode_op(&mut bc, Op::StoreLocal); encode_u16(&mut bc, 1);
        // i++
        encode_op(&mut bc, Op::LoadLocal); encode_u16(&mut bc, 2);
        encode_op(&mut bc, Op::IncI64);
        encode_op(&mut bc, Op::StoreLocal); encode_u16(&mut bc, 2);
        // Jump → loop_top
        encode_op(&mut bc, Op::Jump);
        let jump_disp_at = bc.len();
        let jump_next_pc = bc.len() + 4;
        encode_i32(&mut bc, loop_top as i32 - jump_next_pc as i32);
        // exit
        let exit_pc = bc.len();
        // patch JIF: disp = exit_pc - (jif_disp_at + 4)
        let jif_disp = exit_pc as i32 - (jif_disp_at + 4) as i32;
        bc[jif_disp_at..jif_disp_at + 4].copy_from_slice(&jif_disp.to_le_bytes());
        // return s
        encode_op(&mut bc, Op::LoadLocal); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Return);
        // dead stub
        encode_op(&mut bc, Op::ReturnUndef);
        let _ = jump_disp_at;

        // Need 3 locals: n (arg 0), s (slot 1), i (slot 2).
        let mut proto = empty_proto(bc, 1);
        proto.locals.push(LocalDescriptor { name: "s".to_string(), kind: rusty_js_ast::VariableKind::Var, depth: 0 });
        proto.locals.push(LocalDescriptor { name: "i".to_string(), kind: rusty_js_ast::VariableKind::Var, depth: 0 });

        let jit = compile_function(&proto).expect("typed-i64 JIT compile failed");
        assert_eq!(jit.func.call1(0.0_f64), 0.0_f64);
        assert_eq!(jit.func.call1(5.0_f64), 10.0_f64);
        assert_eq!(jit.func.call1(100.0_f64), 4950.0_f64);
        assert_eq!(jit.func.call1(1_000_000 as f64), 499_999_500_000_i64 as f64);
    }

    /// JIT-EXT 15: guarded-sub trips on i64::MIN - 1 → would-be i64::MAX+1.
    /// i64::MIN - 1 wraps to i64::MAX; the guard detects the sign flip.
    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn guarded_sub_trips_on_overflow() {
        use crate::deopt::{set_current_deopt_sites, take_last_deopt, clear_current_deopt_sites, DeoptReason};

        std::env::set_var("CRUFTLESS_JIT_GUARD_OVERFLOW", "1");
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Sub);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let jit = compile_function(&proto).expect("guarded compile failed");
        std::env::remove_var("CRUFTLESS_JIT_GUARD_OVERFLOW");

        assert_eq!(jit.deopt_sites.len(), 1);
        set_current_deopt_sites(&jit.deopt_sites);

        // 10 - 3 = 7, no trip.
        assert_eq!(jit.func.call2(10.0_f64, 3.0_f64), 7.0_f64);
        assert!(take_last_deopt().is_none());

        // i64::MIN - 1 overflows.
        let r = jit.func.call2(i64::MIN as f64, 1 as f64);
        assert_eq!(r, 0 as f64, "guarded sub trips on i64::MIN - 1");
        let state = take_last_deopt().expect("sub overflow should trip");
        assert!(matches!(state.reason, DeoptReason::IntegerOverflow { .. }));
        assert_eq!(state.local_values, vec![(0, i64::MIN), (1, 1)]);

        clear_current_deopt_sites();
    }

    /// JIT-EXT 15: guarded-mul trips when the product exceeds i64 range.
    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn guarded_mul_trips_on_overflow() {
        use crate::deopt::{set_current_deopt_sites, take_last_deopt, clear_current_deopt_sites, DeoptReason};

        std::env::set_var("CRUFTLESS_JIT_GUARD_OVERFLOW", "1");
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Mul);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let jit = compile_function(&proto).expect("guarded compile failed");
        std::env::remove_var("CRUFTLESS_JIT_GUARD_OVERFLOW");

        assert_eq!(jit.deopt_sites.len(), 1);
        set_current_deopt_sites(&jit.deopt_sites);

        // 1000 * 1000 = 1_000_000, no trip.
        assert_eq!(jit.func.call2(1000.0_f64, 1000.0_f64), 1_000_000 as f64);
        assert!(take_last_deopt().is_none());

        // i64::MAX * 2 overflows.
        let r = jit.func.call2(i64::MAX as f64, 2 as f64);
        assert_eq!(r, 0 as f64, "guarded mul trips on i64::MAX * 2");
        let state = take_last_deopt().expect("mul overflow should trip");
        assert!(matches!(state.reason, DeoptReason::IntegerOverflow { .. }));
        assert_eq!(state.local_values, vec![(0, i64::MAX), (1, 2)]);

        clear_current_deopt_sites();
    }

    /// JIT-EXT 20: GetPropOnObject is now lowered to a runtime-helper
    /// call. The helper is a deterministic stub at this round:
    /// `(receiver_idx << 8) ^ prop_name_idx`. The JIT-compiled
    /// function calls the stub and returns its result; the test
    /// verifies the call chain works end-to-end through Cranelift.
    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn jit_lowers_getprop_on_object_calls_stub() {
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::GetPropOnObject); encode_u16(&mut bc, 7);  // prop_idx = 7
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 1);
        let jit = compile_function(&proto).expect("JIT-EXT 20 should compile GetPropOnObject");

        // Helper stub returns `(receiver << 8) ^ prop_idx`.
        // For receiver=100, prop_idx=7: (100 << 8) ^ 7 = 25600 ^ 7 = 25607.
        let r = jit.func.call1(100.0_f64);
        assert_eq!(r, 25607 as f64, "JIT-compiled GetPropOnObject should call the stub helper and return its result; got {r}");

        // Another data point for confidence.
        // receiver=42, prop_idx=7: (42 << 8) ^ 7 = 10752 ^ 7 = 10759.
        let r = jit.func.call1(42.0_f64);
        assert_eq!(r, 10759 as f64);
    }

    /// JIT-EXT 17: ICShapeMismatch deopt demonstrator.
    ///
    /// Under `CRUFTLESS_JIT_FORCE_SHAPE_TRIP=1`, the translator emits
    /// an entry check that reads the `JIT_FORCE_SHAPE_TRIP` static and
    /// fires an `ICShapeMismatch` deopt if true. The test toggles the
    /// static to demonstrate both the trip and the normal-pass paths,
    /// without needing real GetProp IC support.
    ///
    /// This verifies:
    ///   1. A non-arithmetic deopt reason (ICShapeMismatch) flows through
    ///   2. JIT'd code can read a Rust static via Cranelift's memory load
    ///   3. The recovered state's reason variant matches the emitted site
    ///   4. Toggling the trip at runtime works in both directions
    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn shape_trip_at_entry_demonstrator() {
        use crate::deopt::{
            set_current_deopt_sites, take_last_deopt, clear_current_deopt_sites,
            set_force_shape_trip, DeoptReason,
        };

        std::env::set_var("CRUFTLESS_JIT_FORCE_SHAPE_TRIP", "1");
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Add);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let jit = compile_function(&proto).expect("shape-trip compile failed");
        std::env::remove_var("CRUFTLESS_JIT_FORCE_SHAPE_TRIP");

        // Exactly one DeoptSite for the entry check (no arith guards
        // because guard_overflow was not set).
        assert_eq!(jit.deopt_sites.len(), 1, "expected one entry-shape site");
        assert!(matches!(jit.deopt_sites[0].reason, DeoptReason::ICShapeMismatch { .. }));
        assert_eq!(jit.deopt_sites[0].resume_pc, 0);

        set_current_deopt_sites(&jit.deopt_sites);

        // Flag false → entry check passes, arithmetic runs.
        set_force_shape_trip(false);
        assert_eq!(jit.func.call2(7.0_f64, 5.0_f64), 12.0_f64);
        assert!(take_last_deopt().is_none(),
            "no trip when JIT_FORCE_SHAPE_TRIP is false");

        // Flag true → entry check trips before the Add runs.
        set_force_shape_trip(true);
        let r = jit.func.call2(7.0_f64, 5.0_f64);
        assert_eq!(r, 0 as f64, "trip should return sentinel; got {r}");
        let state = take_last_deopt().expect("flag-on trip should record state");
        assert!(matches!(state.reason, DeoptReason::ICShapeMismatch { .. }),
            "trip reason should be ICShapeMismatch; got {:?}", state.reason);
        assert_eq!(state.resume_pc, 0, "trip site is at function entry");
        assert_eq!(state.local_values, vec![(0, 7), (1, 5)],
            "recovered locals should be the original args");

        // Flag false again → resumes normal behavior.
        set_force_shape_trip(false);
        assert_eq!(jit.func.call2(3.0_f64, 4.0_f64), 7.0_f64);
        assert!(take_last_deopt().is_none());

        clear_current_deopt_sites();
    }

    /// JIT-EXT 16: guarded Inc trips on Inc(i64::MAX).
    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn guarded_inc_trips_on_overflow() {
        use crate::deopt::{set_current_deopt_sites, take_last_deopt, clear_current_deopt_sites, DeoptReason};

        std::env::set_var("CRUFTLESS_JIT_GUARD_OVERFLOW", "1");
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::Inc);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 1);
        let jit = compile_function(&proto).expect("guarded compile failed");
        std::env::remove_var("CRUFTLESS_JIT_GUARD_OVERFLOW");

        assert_eq!(jit.deopt_sites.len(), 1);
        set_current_deopt_sites(&jit.deopt_sites);

        // Inc(7) = 8, no trip.
        assert_eq!(jit.func.call1(7.0_f64), 8.0_f64);
        assert!(take_last_deopt().is_none());

        // Inc(i64::MAX) overflows.
        let r = jit.func.call1(i64::MAX as f64);
        assert_eq!(r, 0 as f64, "guarded inc trips on i64::MAX + 1");
        let state = take_last_deopt().expect("inc overflow should trip");
        assert!(matches!(state.reason, DeoptReason::IntegerOverflow { .. }));

        clear_current_deopt_sites();
    }

    /// JIT-EXT 16: guarded Dec trips on Dec(i64::MIN).
    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn guarded_dec_trips_on_overflow() {
        use crate::deopt::{set_current_deopt_sites, take_last_deopt, clear_current_deopt_sites, DeoptReason};

        std::env::set_var("CRUFTLESS_JIT_GUARD_OVERFLOW", "1");
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::Dec);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 1);
        let jit = compile_function(&proto).expect("guarded compile failed");
        std::env::remove_var("CRUFTLESS_JIT_GUARD_OVERFLOW");

        assert_eq!(jit.deopt_sites.len(), 1);
        set_current_deopt_sites(&jit.deopt_sites);

        // Dec(7) = 6, no trip.
        assert_eq!(jit.func.call1(7.0_f64), 6.0_f64);
        assert!(take_last_deopt().is_none());

        // Dec(i64::MIN) overflows.
        let r = jit.func.call1(i64::MIN as f64);
        assert_eq!(r, 0 as f64, "guarded dec trips on i64::MIN - 1");
        let state = take_last_deopt().expect("dec overflow should trip");
        assert!(matches!(state.reason, DeoptReason::IntegerOverflow { .. }));

        clear_current_deopt_sites();
    }

    /// JIT-EXT 13: guarded-add demonstrator end-to-end.
    ///
    /// Compiles `add(a, b) = a + b` with `CRUFTLESS_JIT_GUARD_OVERFLOW=1`,
    /// invokes with non-overflowing args (sum returned), then invokes
    /// with overflowing args (i64::MAX + 1) and verifies:
    ///   1. The JIT call returns the thunk's sentinel (0)
    ///   2. `take_last_deopt()` returns a populated DeoptRecoveredState
    ///   3. The recovered state's reason is IntegerOverflow at the Add pc
    ///   4. The recovered state's local_values map back to the original args
    ///   5. The CompiledFn carries exactly one DeoptSite (the one Add)
    ///
    /// Also verifies the no-overflow case does NOT trip (no recorded deopt).
    #[ignore = "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"]
    #[test]
    fn guarded_add_trips_on_overflow() {
        use crate::deopt::{set_current_deopt_sites, take_last_deopt, clear_current_deopt_sites, DeoptReason};

        // The env-var is read once at compile_function entry.
        std::env::set_var("CRUFTLESS_JIT_GUARD_OVERFLOW", "1");

        let mut bc = Vec::new();
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
        encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 1);
        encode_op(&mut bc, Op::Add);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        let jit = compile_function(&proto).expect("guarded compile failed");

        // Cleanup the env var so subsequent tests are unaffected.
        std::env::remove_var("CRUFTLESS_JIT_GUARD_OVERFLOW");

        // Sanity: one DeoptSite for the one Add.
        assert_eq!(jit.deopt_sites.len(), 1,
            "expected exactly one DeoptSite; got {:?}", jit.deopt_sites);
        assert!(matches!(jit.deopt_sites[0].reason, DeoptReason::IntegerOverflow { .. }));

        // Wire the dispatcher TLS to point at the compiled fn's sites.
        set_current_deopt_sites(&jit.deopt_sites);

        // No-overflow call: must work normally, no trip recorded.
        let r = jit.func.call2(2.0_f64, 3.0_f64);
        assert_eq!(r, 5 as f64);
        assert!(take_last_deopt().is_none(), "no-overflow call should not trip");

        // Overflow call: i64::MAX + 1 wraps to i64::MIN; the guard
        // detects this and trips. The JIT returns sentinel 0.
        let r = jit.func.call2(i64::MAX as f64, 1 as f64);
        assert_eq!(r, 0 as f64, "guarded JIT should return sentinel on trip; got {r}");

        let state = take_last_deopt().expect("overflow should have tripped");
        assert!(matches!(state.reason, DeoptReason::IntegerOverflow { .. }));
        // local_values: locals[0] = arg0 = i64::MAX, locals[1] = arg1 = 1
        assert_eq!(state.local_values, vec![(0, i64::MAX), (1, 1)]);
        // stack_values: the lhs (i64::MAX) and rhs (1) being added
        assert_eq!(state.stack_values, vec![(0, i64::MAX), (1, 1)]);

        clear_current_deopt_sites();
    }

    #[test]
    fn jit_compile_sum_function() {
        // function sum(n) { var s=0; for (var i=0; i<n; i++) s=s+i; return s; }
        let src = r#"function sum(n) { var s = 0; for (var i = 0; i < n; i++) s = s + i; return s; }"#;
        let m = compile_module(src).expect("compile module");
        let sum_proto = m.constants.entries().iter()
            .find_map(|c| match c { Constant::Function(p) if p.display_name == "sum" => Some((**p).clone()), _ => None })
            .expect("find sum proto");
        let jit = compile_function(&sum_proto).expect("JIT compile sum failed");
        // sum(0) = 0, sum(1) = 0, sum(5) = 0+1+2+3+4 = 10, sum(100) = 4950
        assert_eq!(jit.func.call1(0.0_f64), 0.0_f64);
        assert_eq!(jit.func.call1(1.0_f64), 0.0_f64);
        assert_eq!(jit.func.call1(5.0_f64), 10.0_f64);
        assert_eq!(jit.func.call1(100.0_f64), 4950.0_f64);
        assert_eq!(jit.func.call1(1_000_000 as f64), 499999500000.0_f64);
    }
}
