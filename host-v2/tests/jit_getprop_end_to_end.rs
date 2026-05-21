//! JIT-EXT 23 end-to-end test: a JIT-compiled function with
//! GetPropOnObject runs through the dispatcher, the real runtime
//! helper performs the property lookup, and the correct value is
//! returned.
//!
//! Bypasses the upstream JS parser (which doesn't yet emit
//! GetPropOnObject) by hand-building a FunctionProto + the Closure
//! wrapping it. This is a Rust-level integration test; the
//! end-to-end "JS source → parser → bytecode → JIT → runtime helper
//! → result" flow lands when the bytecode compiler's typed-promotion
//! pass extends to emit GetPropOnObject (queued as a separate
//! workstream).

use rusty_js_bytecode::compiler::{FunctionProto, LocalDescriptor, UpvalueDescriptor};
use rusty_js_bytecode::op::Op;
use rusty_js_bytecode::constants::{Constant, ConstantsPool};
use rusty_js_runtime::{Runtime, Value};
use rusty_js_runtime::value::Object;

fn encode_op(bc: &mut Vec<u8>, op: Op) { bc.push(op as u8); }
fn encode_u16(bc: &mut Vec<u8>, v: u16) { bc.extend_from_slice(&v.to_le_bytes()); }

/// Hand-build `function getx(obj) { return obj.x; }` with GetPropOnObject.
fn build_getx_proto(prop_name: &str) -> FunctionProto {
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::LoadArg); encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::GetPropOnObject); encode_u16(&mut bc, 0);  // constant idx 0
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

#[test]
fn jit_compiled_getprop_returns_object_property_value() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    // Bring the JIT threshold to 1 so first call compiles.
    rt.jit_threshold = 1;

    // Allocate an object with .x = 42.
    let obj_id = rt.alloc_object(Object::new_ordinary());
    rt.object_set(obj_id, "x".into(), Value::Number(42.0));

    // Hand-build the proto + Closure.
    let proto = build_getx_proto("x");
    let proto_rc = std::rc::Rc::new(proto);

    // The Closure is what call_function dispatches on. Use the
    // runtime's MakeClosure pipeline via a low-level constructor.
    use rusty_js_runtime::value::{ClosureInternals, InternalKind};
    let closure_internals = ClosureInternals {
        proto: proto_rc.clone(),
        upvalues: Vec::new(),
        bound_this: None,
        is_arrow: false,
        call_count: std::cell::Cell::new(0),
        jit_disabled: std::cell::Cell::new(false),
    };
    let closure_obj = Object {
        proto: None,
        extensible: true,
        properties: indexmap::IndexMap::new(),
        internal_kind: InternalKind::Closure(closure_internals),
    };
    let closure_id = rt.alloc_object(closure_obj);

    // Invoke twice: first call JIT-compiles (threshold=1); second
    // exercises the cached JIT path through the runtime helper.
    for trial in 0..2 {
        let result = rt.call_function(
            Value::Object(closure_id),
            Value::Undefined,
            vec![Value::Object(obj_id)],
        ).expect("call_function should succeed");

        match result {
            Value::Number(n) => assert_eq!(n, 42.0,
                "getx(obj) where obj.x=42 should return 42; trial={trial}, got {n}"),
            other => panic!("trial={trial}: expected Number(42); got {other:?}"),
        }
    }
}
