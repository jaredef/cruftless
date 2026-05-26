//! LeJIT-Ψ VTI-EXT 1: pre-emission bench probe for the per-call
//! arg-coercion + dispatch overhead.
//!
//! Measures the baseline cost of calling a minimal JIT-compiled function
//! 1M times with a single Number arg. The function body is `function
//! id(x) { return x; }` — the only work per call is dispatcher entry,
//! arg coercion (unbox Value::Number → i64), JIT call, result rebox
//! (i64 → Value::Number). No body work; pure overhead measurement.
//!
//! Pred-vti.1 reads against this baseline: inline Number-tag check on
//! the arg-coercion path should reduce per-call cost by ≥20 ns. The
//! bench at VTI-EXT 6 re-measures with `CRUFTLESS_LEJIT_VTI=1`.
//!
//! Composition with LeJIT-Σ's bench_ic (271 ns pre-shape; 199 ns
//! post-shape): this bench isolates the dispatcher + arg-coercion cost
//! component without IC dispatch in the mix. Together the two benches
//! decompose the per-iter cost across the substrate axes.

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

/// Hand-build `function id(x) { return x; }` as bytecode.
fn build_id_proto() -> FunctionProto {
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::Return);

    FunctionProto {
        bytecode: bc,
        constants: ConstantsPool::new(),
        params: 1,
        display_name: "id".to_string(),
        function_length: 1,
        locals: vec![LocalDescriptor {
            name: "x".to_string(),
            kind: rusty_js_ast::VariableKind::Let,
            depth: 0,
        }],
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

fn main() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    rt.jit_threshold = 1;

    let proto = build_id_proto();
    let proto_rc = std::rc::Rc::new(proto);

    let closure_internals = ClosureInternals {
        proto: proto_rc.clone(),
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
    let closure_v = Value::Object(closure_id);
    let arg = Value::Number(42.0);

    // Warm-up: JIT compile happens on first call.
    for _ in 0..10 {
        let _ = rt.call_function(closure_v.clone(), Value::Undefined, vec![arg.clone()]);
    }

    // Bench: N = 1_000_000 call_function dispatches through the JIT
    // path for the trivial id(x) function. Per-iter cost = dispatcher
    // + arg coercion (Value::Number unbox) + JIT call + result rebox.
    const N: u64 = 1_000_000;
    let t0 = Instant::now();
    let mut last = Value::Undefined;
    for _ in 0..N {
        last = rt
            .call_function(closure_v.clone(), Value::Undefined, vec![arg.clone()])
            .expect("call_function should succeed");
    }
    let elapsed = t0.elapsed();
    let per_iter_ns = (elapsed.as_nanos() as f64) / N as f64;

    match last {
        Value::Number(n) => assert!((n - 42.0).abs() < 1e-9),
        other => panic!("expected Number(42); got {other:?}"),
    }

    println!("LeJIT-Ψ VTI-EXT 1 — pre-emission call-overhead bench baseline");
    println!("--------------------------------------------");
    println!("workload:    {} iterations of id(Number(42))", N);
    println!("dispatch:    call_function -> arg-coerce -> JIT-compiled id -> rebox");
    println!();
    println!("elapsed:     {:.3} ms", elapsed.as_secs_f64() * 1000.0);
    println!("per-iter:    {:.1} ns", per_iter_ns);
    println!();
    println!("Pred-vti.1 target: inline tag-check reduces per-iter by ≥20 ns post-VTI-EXT 4.");
    println!(
        "(For composition reading: bench_ic measures the same dispatcher + extra IC dispatch.)"
    );
}
