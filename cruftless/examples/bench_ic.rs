//! LeJIT-Σ StubE-EXT 1: pre-stub bench probe for the IC dispatch fast path.
//!
//! Establishes the baseline measurement of the CURRENT extern-call IC
//! dispatch — `Op::GetPropOnObject` lowered to a Cranelift `call` to
//! the `runtime_getprop_on_object` extern (per JIT-EXT 22-24). The
//! stub-emitter pilot (LeJIT-Σ; see `pilots/rusty-js-jit/stub-emitter/`)
//! targets ≥3× per-hit speedup over this baseline per Pred-stub.1 +
//! Doc 735 §X.h.b.
//!
//! The bench hand-builds `function getx(obj) { return obj.x; }` as a
//! FunctionProto with `Op::GetPropOnObject` (the bytecode compiler's
//! upstream typed-promotion pass doesn't emit this op yet; the
//! synthetic proto bypasses that gap). The receiver is an Object with
//! `obj.x = 42.0` so the IC succeeds (Number result; no deopt).
//!
//! Calls `getx(obj)` N = 1,000,000 times in a tight loop, measures
//! wall-clock time, computes per-iter ns. Runs the same loop a second
//! time post-JIT-warmup to amortize compile cost.
//!
//! Per Doc 735 §X.h.c three-probe-levels: this is the bench probe
//! (one of three) for the Pred-stub.1 claim. Consumer-route probe
//! lands at StubE-EXT 5 (diff-prod 42/42 + a JIT-on hot-loop fixture);
//! fuzz probe lands at StubE-EXT 7 (shape-transition-history fuzz over
//! the IC dispatch space).
//!
//! Pi target measurement is the load-bearing reading; this example
//! also runs on any aarch64-or-x86_64 host but the comparison point is
//! the Pi number (the engagement's reference hardware per the seed).

use rusty_js_bytecode::compiler::{FunctionProto, LocalDescriptor, UpvalueDescriptor};
use rusty_js_bytecode::constants::ConstantsPool;
use rusty_js_bytecode::op::Op;
use rusty_js_bytecode::Constant;
use rusty_js_runtime::{Runtime, Value};
use rusty_js_runtime::value::{ClosureInternals, InternalKind, Object};
use std::time::Instant;

fn encode_op(bc: &mut Vec<u8>, op: Op) { bc.push(op as u8); }
fn encode_u16(bc: &mut Vec<u8>, v: u16) { bc.extend_from_slice(&v.to_le_bytes()); }

fn build_getx_proto(prop_name: &str) -> FunctionProto {
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::LoadLocal); encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::GetPropOnObject); encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::Return);

    let mut constants = ConstantsPool::new();
    let idx = constants.intern(Constant::String(prop_name.to_string()));
    assert_eq!(idx, 0, "expected constant idx 0 for the prop name");

    FunctionProto {
        bytecode: bc,
        constants,
        params: 1,
        display_name: "getx".to_string(),
        function_length: 1,
        locals: vec![LocalDescriptor {
            name: "obj".to_string(),
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

    let obj_id = rt.alloc_object(Object::new_ordinary());
    rt.object_set(obj_id, "x".into(), Value::Number(42.0));

    let proto = build_getx_proto("x");
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
    let obj_v = Value::Object(obj_id);

    // Warm-up: JIT compile happens on first call (jit_threshold=1).
    for _ in 0..10 {
        let _ = rt.call_function(closure_v.clone(), Value::Undefined,
            vec![obj_v.clone()]);
    }

    // Bench: N = 1_000_000 IC dispatches through the current extern-call path.
    const N: u64 = 1_000_000;
    let t0 = Instant::now();
    let mut last = Value::Undefined;
    for _ in 0..N {
        last = rt.call_function(closure_v.clone(), Value::Undefined,
            vec![obj_v.clone()]).expect("call_function should succeed");
    }
    let elapsed = t0.elapsed();
    let total_ns = elapsed.as_nanos() as f64;
    let per_iter_ns = total_ns / N as f64;

    match last {
        Value::Number(n) => assert!((n - 42.0).abs() < 1e-9, "last result should be 42.0"),
        other => panic!("expected Number(42); got {other:?}"),
    }

    println!("LeJIT-Σ StubE-EXT 1 — pre-stub IC bench baseline");
    println!("--------------------------------------------");
    println!("workload:    {} iterations of getx(obj) where obj.x = 42", N);
    println!("dispatch:    Op::GetPropOnObject -> Cranelift call -> extern jit_getprop_on_object -> runtime_getprop_on_object (object_get)");
    println!("ic-form:     monomorphic, single receiver shape (Shape::None pre-CMig-EXT 8)");
    println!();
    println!("elapsed:     {:.3} ms", elapsed.as_secs_f64() * 1000.0);
    println!("per-iter:    {:.1} ns", per_iter_ns);
    println!();
    println!("Pred-stub.1 target: stub-emitted dispatch ≥3× faster (≤{:.1} ns/iter).", per_iter_ns / 3.0);
    println!("Re-measure post-StubE-EXT 5 to corroborate or falsify.");
}
