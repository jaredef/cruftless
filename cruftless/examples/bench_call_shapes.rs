//! LeJIT-Τ TB-EXT 1: multi-shape call-overhead bench probe.
//!
//! Extends VTI-EXT 1's `bench_call_overhead` (single id(x) shape) to
//! cover three function shapes that exercise different cost components
//! of the Rust `call_function` dispatcher:
//!
//!   - id1:       1-arg, no body work, no locals beyond the arg
//!   - id2:       2-arg, one Add op, two args -> exercises 2-arg gate
//!   - id_locals: 1-arg, one extra local + StoreLocal -> exercises local
//!                management at JIT-entry (locals[1] init + StoreLocal)
//!
//! Per-shape baselines feed TB-EXT 2's dispatcher-decomposition audit
//! by isolating which cost components scale with arity vs locals vs
//! body-work. TB-EXT 4's post-implementation measurement will use the
//! same shape set so the comparison is shape-controlled.

use rusty_js_bytecode::compiler::{FunctionProto, LocalDescriptor, UpvalueDescriptor};
use rusty_js_bytecode::constants::ConstantsPool;
use rusty_js_bytecode::op::Op;
use rusty_js_runtime::value::{ClosureInternals, InternalKind, Object};
use rusty_js_runtime::{Runtime, Value};
use std::time::Instant;

fn encode_op(bc: &mut Vec<u8>, op: Op) {
    bc.push(op as u8);
}
fn encode_u16(bc: &mut Vec<u8>, v: u16) {
    bc.extend_from_slice(&v.to_le_bytes());
}

fn local(name: &str) -> LocalDescriptor {
    LocalDescriptor {
        name: name.to_string(),
        kind: rusty_js_ast::VariableKind::Let,
        depth: 0,
    }
}

fn make_proto(name: &str, bc: Vec<u8>, params: u16, locals: Vec<LocalDescriptor>) -> FunctionProto {
    FunctionProto {
        bytecode: bc,
        constants: ConstantsPool::new(),
        params,
        display_name: name.to_string(),
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

/// function id1(x) { return x; }
fn build_id1() -> FunctionProto {
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::Return);
    make_proto("id1", bc, 1, vec![local("x")])
}

/// function id2(x, y) { return x + y; }
fn build_id2() -> FunctionProto {
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 1);
    encode_op(&mut bc, Op::Add);
    encode_op(&mut bc, Op::Return);
    make_proto("id2", bc, 2, vec![local("x"), local("y")])
}

/// function id_locals(x) { let y = x; return y; }
fn build_id_locals() -> FunctionProto {
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::StoreLocal);
    encode_u16(&mut bc, 1);
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 1);
    encode_op(&mut bc, Op::Return);
    make_proto("id_locals", bc, 1, vec![local("x"), local("y")])
}

fn install_closure(rt: &mut Runtime, proto: FunctionProto) -> Value {
    let proto_rc = std::rc::Rc::new(proto);
    let closure_internals = ClosureInternals {
        proto: proto_rc,
        upvalues: Vec::new(),
        bound_this: None,
        bound_this_cell: None,
        is_arrow: false,
        call_count: std::cell::Cell::new(0),
        jit_disabled: std::cell::Cell::new(false),
        tb_metadata_ptr: std::cell::Cell::new(None),
    };
    let closure_obj = Object {
        proto: None,
        extensible: true,
        properties: indexmap::IndexMap::new(),
        internal_kind: InternalKind::Closure(closure_internals),
        ..Default::default()
    };
    let closure_id = rt.alloc_object(closure_obj);
    Value::Object(closure_id)
}

fn bench(name: &str, rt: &mut Runtime, closure_v: Value, args: Vec<Value>, expected: f64) -> f64 {
    // Warm-up: JIT compile happens on first call.
    for _ in 0..10 {
        let _ = rt.call_function(closure_v.clone(), Value::Undefined, args.clone());
    }
    const N: u64 = 1_000_000;
    let t0 = Instant::now();
    let mut last = Value::Undefined;
    for _ in 0..N {
        last = rt
            .call_function(closure_v.clone(), Value::Undefined, args.clone())
            .expect("call_function should succeed");
    }
    let elapsed = t0.elapsed();
    let per_iter_ns = (elapsed.as_nanos() as f64) / N as f64;
    match last {
        Value::Number(n) => assert!(
            (n - expected).abs() < 1e-9,
            "{name}: expected {expected}, got {n}"
        ),
        other => panic!("{name}: expected Number({expected}); got {other:?}"),
    }
    println!(
        "  {name:<12}  elapsed: {:>7.2} ms   per-iter: {:>6.1} ns",
        elapsed.as_secs_f64() * 1000.0,
        per_iter_ns
    );
    per_iter_ns
}

fn main() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    rt.jit_threshold = 1;

    let id1 = install_closure(&mut rt, build_id1());
    let id2 = install_closure(&mut rt, build_id2());
    let id_locals = install_closure(&mut rt, build_id_locals());

    let arg42 = Value::Number(42.0);
    let arg17 = Value::Number(17.0);

    println!("LeJIT-Τ TB-EXT 1 — multi-shape call-overhead bench baseline");
    println!("============================================================");
    println!("workload:    1,000,000 iterations per shape");
    println!("dispatch:    call_function -> arg-coerce -> JIT -> rebox");
    println!();

    let p1 = bench("id1", &mut rt, id1, vec![arg42.clone()], 42.0);
    let p2 = bench(
        "id2",
        &mut rt,
        id2,
        vec![arg42.clone(), arg17.clone()],
        59.0,
    );
    let p3 = bench("id_locals", &mut rt, id_locals, vec![arg42.clone()], 42.0);

    println!();
    println!("Decomposition reading:");
    println!(
        "  id2 - id1       = {:>5.1} ns  (2nd arg coerce + Add op)",
        p2 - p1
    );
    println!(
        "  id_locals - id1 = {:>5.1} ns  (local-init + StoreLocal)",
        p3 - p1
    );
    println!();
    println!("Pred-tb.1 target: TB-EXT 3b call-thunk reduces id1 by >=40 ns post-3b.");
}
