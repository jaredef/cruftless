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

use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::types::{I64, I8};
use cranelift_codegen::ir::{AbiParam, Block, InstBuilder, Value as ClValue};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use rusty_js_bytecode::compiler::FunctionProto;
use rusty_js_bytecode::op::{op_from_byte, Op};
use std::collections::HashMap;

/// First-cut JIT signature: extern "C" fn(i64) -> i64 for a 1-arg fn,
/// extern "C" fn(i64, i64) -> i64 for a 2-arg fn. The translator emits
/// the variant matching proto.params.
pub type JitFn1 = extern "C" fn(i64) -> i64;
pub type JitFn2 = extern "C" fn(i64, i64) -> i64;

pub enum JitFn {
    Arity1(JitFn1),
    Arity2(JitFn2),
}

impl JitFn {
    pub fn call1(&self, a: i64) -> i64 {
        match self {
            JitFn::Arity1(f) => f(a),
            JitFn::Arity2(f) => f(a, 0),
        }
    }
    pub fn call2(&self, a: i64, b: i64) -> i64 {
        match self {
            JitFn::Arity1(f) => f(a),
            JitFn::Arity2(f) => f(a, b),
        }
    }
}

impl std::fmt::Debug for JitFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitFn::Arity1(p) => write!(f, "JitFn::Arity1(0x{:x})", *p as usize),
            JitFn::Arity2(p) => write!(f, "JitFn::Arity2(0x{:x})", *p as usize),
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
    // Auto-promote first; on success, compile the promoted variant.
    let owned;
    let working: &FunctionProto = match crate::promote::promote_to_typed_i64(proto) {
        Some(p) => { owned = p; &owned }
        None => proto,
    };
    compile_function_inner(working)
}

fn compile_function_inner(proto: &FunctionProto) -> Result<CompiledFn, String> {
    if proto.params != 1 && proto.params != 2 {
        return Err(format!("first-cut JIT supports 1 or 2 params; got {}", proto.params));
    }

    // Pre-scan: parse bytecode into a structured op list with absolute
    // pcs; identify all jump targets so we can allocate blocks.
    let parsed = parse_bytecode(&proto.bytecode)?;
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
    let lejit_stub = std::env::var("CRUFTLESS_LEJIT_STUB")
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

    for _ in 0..proto.params {
        ctx.func.signature.params.push(AbiParam::new(I64));
    }
    ctx.func.signature.returns.push(AbiParam::new(I64));

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

        // Entry block: append params, declare local variables, store args.
        let entry = blocks[&0];
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);

        // Declare a Variable per LocalDescriptor. Locals are typed I64 in
        // this first cut. Cranelift handles SSA conversion via mem2reg.
        let mut local_vars: Vec<Variable> = Vec::with_capacity(proto.locals.len());
        for i in 0..proto.locals.len() {
            let v = Variable::from_u32(i as u32);
            builder.declare_var(v, I64);
            local_vars.push(v);
        }
        // Initialize all locals to 0 (matches interpreter's PushUndefined →
        // we treat undef as 0 in the i64-only first cut).
        let zero = builder.ins().iconst(I64, 0);
        for v in &local_vars {
            builder.def_var(*v, zero);
        }
        // Args land in locals 0..params at function entry per the
        // interpreter convention (compile_function_proto allocates one
        // slot per param at the head of self.locals).
        let entry_params: Vec<ClValue> = builder.block_params(entry).to_vec();
        for (i, &p) in entry_params.iter().enumerate() {
            if i < local_vars.len() {
                builder.def_var(local_vars[i], p);
            }
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
            builder.ins().return_(&[sentinel]);

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
                    let v = builder.ins().iconst(I64, *n as i64);
                    stack.push(v);
                }
                ParsedOp::Add => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        emit_guarded_add(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        binop(&mut stack, &mut builder, |b, l, r| b.ins().iadd(l, r))?;
                    }
                }
                ParsedOp::Sub => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        emit_guarded_sub(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        binop(&mut stack, &mut builder, |b, l, r| b.ins().isub(l, r))?;
                    }
                }
                ParsedOp::Mul => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        emit_guarded_mul(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        binop(&mut stack, &mut builder, |b, l, r| b.ins().imul(l, r))?;
                    }
                }
                ParsedOp::Inc => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        // Inc(v) = Add(v, 1). Synthesize rhs=1 onto the
                        // stack and reuse emit_guarded_add. The stack
                        // had [v]; after push, [v, 1]; emit_guarded_add
                        // pops r=1, l=v, pushes v+1.
                        let one = builder.ins().iconst(I64, 1);
                        stack.push(one);
                        emit_guarded_add(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        let v = stack.pop().ok_or("Inc: stack underflow")?;
                        let one = builder.ins().iconst(I64, 1);
                        let r = builder.ins().iadd(v, one);
                        stack.push(r);
                    }
                }
                ParsedOp::Dec => {
                    if guard_overflow { let tr = trip_ref.expect("guard_overflow requires trip_ref");
                        // Dec(v) = Sub(v, 1). Synthesize rhs=1.
                        let one = builder.ins().iconst(I64, 1);
                        stack.push(one);
                        emit_guarded_sub(&mut stack, &mut builder, tr, *pc, &local_vars, &mut deopt_sites)?;
                    } else {
                        let v = stack.pop().ok_or("Dec: stack underflow")?;
                        let one = builder.ins().iconst(I64, 1);
                        let r = builder.ins().isub(v, one);
                        stack.push(r);
                    }
                }
                ParsedOp::Lt => cmpop(&mut stack, &mut builder, IntCC::SignedLessThan)?,
                ParsedOp::Le => cmpop(&mut stack, &mut builder, IntCC::SignedLessThanOrEqual)?,
                ParsedOp::Gt => cmpop(&mut stack, &mut builder, IntCC::SignedGreaterThan)?,
                ParsedOp::Ge => cmpop(&mut stack, &mut builder, IntCC::SignedGreaterThanOrEqual)?,
                ParsedOp::Eq | ParsedOp::StrictEq => cmpop(&mut stack, &mut builder, IntCC::Equal)?,
                ParsedOp::Ne | ParsedOp::StrictNe => cmpop(&mut stack, &mut builder, IntCC::NotEqual)?,
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
                    let cond_i64 = stack.pop().ok_or("JumpIfX: stack underflow")?;
                    if !stack.is_empty() {
                        return Err(format!("stack non-empty at JumpIfX pc={} (depth={})", pc, stack.len()));
                    }
                    // Reduce i64 cond to i8 truthy flag via icmp NE 0.
                    let zero = builder.ins().iconst(I64, 0);
                    let truthy: ClValue = builder.ins().icmp(IntCC::NotEqual, cond_i64, zero);
                    // Find fallthrough block (next pc's block).
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
                    builder.ins().return_(&[v]);
                    block_terminated = true;
                    stack.clear();
                }
                ParsedOp::ReturnUndef => {
                    let z = builder.ins().iconst(I64, 0);
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
                    // JIT-EXT 20: lower to a call into the runtime helper.
                    // The receiver i64 (typed as ObjectRef index by
                    // upstream) is popped from the stack; the
                    // prop_name_idx is emitted as a Cranelift constant.
                    //
                    // LeJIT-Σ StubE-EXT 5b: when lejit_stub is set, the
                    // helper is jit_getprop_with_ic (3-arg: site_id +
                    // receiver_idx + prop_name_idx). The IC cache
                    // populates as a side effect via the runtime-
                    // registered observer.
                    let gpref = getprop_ref.expect("getprop_ref must be set when ParsedOp::GetPropOnObject is present");
                    let receiver = stack.pop().ok_or("GetPropOnObject: stack underflow (receiver)")?;
                    let prop_v = builder.ins().iconst(I64, *prop_idx as i64);
                    let call_inst = if lejit_stub {
                        let site_id = ic_site_ids[ic_site_cursor];
                        ic_site_cursor += 1;
                        let site_v = builder.ins().iconst(I64, site_id as i64);
                        builder.ins().call(gpref, &[site_v, receiver, prop_v])
                    } else {
                        builder.ins().call(gpref, &[receiver, prop_v])
                    };
                    let result = builder.inst_results(call_inst)[0];
                    stack.push(result);
                }
            }
            // Allow comparison op result (i8) to participate in stack
            // as if it were i64 — handled inside cmpop by extending.
            let _ = current_block;
        }

        // If the last instruction wasn't a terminator, synthesize a
        // ReturnUndef.
        if !block_terminated {
            let z = builder.ins().iconst(I64, 0);
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
        match proto.params {
            1 => JitFn::Arity1(std::mem::transmute::<*const u8, JitFn1>(code_ptr)),
            2 => JitFn::Arity2(std::mem::transmute::<*const u8, JitFn2>(code_ptr)),
            _ => unreachable!(),
        }
    };
    let leaked = Box::leak(Box::new(module));
    Ok(CompiledFn { func, _module: leaked, deopt_sites })
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
    builder.ins().return_(&[sentinel]);

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
    builder.ins().return_(&[sentinel]);

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
    builder.ins().return_(&[sentinel]);

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

fn parse_bytecode(bc: &[u8]) -> Result<Vec<(usize, ParsedOp)>, String> {
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
        assert_eq!(jit.func.call2(2, 3), 5);
        assert_eq!(jit.func.call2(-10, 100), 90);
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
        assert_eq!(jit.func.call2(5, 3), 16);
        assert_eq!(jit.func.call2(10, 4), 84);
    }

    #[test]
    fn jit_rejects_unsupported_op() {
        let mut bc = Vec::new();
        encode_op(&mut bc, Op::PushNull);
        encode_op(&mut bc, Op::Return);
        let proto = empty_proto(bc, 2);
        assert!(compile_function(&proto).is_err());
    }

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
        assert_eq!(jit.func.call1(0), 0);
        assert_eq!(jit.func.call1(5), 10);
        assert_eq!(jit.func.call1(100), 4950);
        assert_eq!(jit.func.call1(1_000_000), 499_999_500_000);
    }

    /// JIT-EXT 15: guarded-sub trips on i64::MIN - 1 → would-be i64::MAX+1.
    /// i64::MIN - 1 wraps to i64::MAX; the guard detects the sign flip.
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
        assert_eq!(jit.func.call2(10, 3), 7);
        assert!(take_last_deopt().is_none());

        // i64::MIN - 1 overflows.
        let r = jit.func.call2(i64::MIN, 1);
        assert_eq!(r, 0, "guarded sub trips on i64::MIN - 1");
        let state = take_last_deopt().expect("sub overflow should trip");
        assert!(matches!(state.reason, DeoptReason::IntegerOverflow { .. }));
        assert_eq!(state.local_values, vec![(0, i64::MIN), (1, 1)]);

        clear_current_deopt_sites();
    }

    /// JIT-EXT 15: guarded-mul trips when the product exceeds i64 range.
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
        assert_eq!(jit.func.call2(1000, 1000), 1_000_000);
        assert!(take_last_deopt().is_none());

        // i64::MAX * 2 overflows.
        let r = jit.func.call2(i64::MAX, 2);
        assert_eq!(r, 0, "guarded mul trips on i64::MAX * 2");
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
        let r = jit.func.call1(100);
        assert_eq!(r, 25607,
            "JIT-compiled GetPropOnObject should call the stub helper and return its result; got {r}");

        // Another data point for confidence.
        // receiver=42, prop_idx=7: (42 << 8) ^ 7 = 10752 ^ 7 = 10759.
        let r = jit.func.call1(42);
        assert_eq!(r, 10759);
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
        assert_eq!(jit.func.call2(7, 5), 12);
        assert!(take_last_deopt().is_none(),
            "no trip when JIT_FORCE_SHAPE_TRIP is false");

        // Flag true → entry check trips before the Add runs.
        set_force_shape_trip(true);
        let r = jit.func.call2(7, 5);
        assert_eq!(r, 0, "trip should return sentinel; got {r}");
        let state = take_last_deopt().expect("flag-on trip should record state");
        assert!(matches!(state.reason, DeoptReason::ICShapeMismatch { .. }),
            "trip reason should be ICShapeMismatch; got {:?}", state.reason);
        assert_eq!(state.resume_pc, 0, "trip site is at function entry");
        assert_eq!(state.local_values, vec![(0, 7), (1, 5)],
            "recovered locals should be the original args");

        // Flag false again → resumes normal behavior.
        set_force_shape_trip(false);
        assert_eq!(jit.func.call2(3, 4), 7);
        assert!(take_last_deopt().is_none());

        clear_current_deopt_sites();
    }

    /// JIT-EXT 16: guarded Inc trips on Inc(i64::MAX).
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
        assert_eq!(jit.func.call1(7), 8);
        assert!(take_last_deopt().is_none());

        // Inc(i64::MAX) overflows.
        let r = jit.func.call1(i64::MAX);
        assert_eq!(r, 0, "guarded inc trips on i64::MAX + 1");
        let state = take_last_deopt().expect("inc overflow should trip");
        assert!(matches!(state.reason, DeoptReason::IntegerOverflow { .. }));

        clear_current_deopt_sites();
    }

    /// JIT-EXT 16: guarded Dec trips on Dec(i64::MIN).
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
        assert_eq!(jit.func.call1(7), 6);
        assert!(take_last_deopt().is_none());

        // Dec(i64::MIN) overflows.
        let r = jit.func.call1(i64::MIN);
        assert_eq!(r, 0, "guarded dec trips on i64::MIN - 1");
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
        let r = jit.func.call2(2, 3);
        assert_eq!(r, 5);
        assert!(take_last_deopt().is_none(), "no-overflow call should not trip");

        // Overflow call: i64::MAX + 1 wraps to i64::MIN; the guard
        // detects this and trips. The JIT returns sentinel 0.
        let r = jit.func.call2(i64::MAX, 1);
        assert_eq!(r, 0, "guarded JIT should return sentinel on trip; got {r}");

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
        assert_eq!(jit.func.call1(0), 0);
        assert_eq!(jit.func.call1(1), 0);
        assert_eq!(jit.func.call1(5), 10);
        assert_eq!(jit.func.call1(100), 4950);
        assert_eq!(jit.func.call1(1_000_000), 499999500000);
    }
}
