//! JIT-EXT 21: Runtime::resume_from_deopt_state end-to-end smoke test.
//!
//! Hand-build a FunctionProto for `add(a, b) { return a + b; }`, hand-
//! craft a DeoptRecoveredState that names locals + stack + resume_pc,
//! call resume_from_deopt_state, and verify the interpreter resumes
//! correctly and returns the right Number value.
//!
//! Lives at the host-v2 tier because constructing a FunctionProto + the
//! bytecode the interpreter accepts requires the bytecode encoder
//! helpers, which are not all re-exported from the JIT crate. The test
//! exercises the resume_from_deopt_state API directly without
//! involving the dispatcher (which still falls through to re-execute-
//! from-pc-0).

use rusty_js_bytecode::compiler::{FunctionProto, LocalDescriptor, UpvalueDescriptor};
use rusty_js_bytecode::constants::ConstantsPool;
use rusty_js_bytecode::op::Op;
use rusty_js_jit::{DeoptReason, DeoptRecoveredState};
use rusty_js_runtime::{Runtime, Value};

fn encode_op(bc: &mut Vec<u8>, op: Op) {
    bc.push(op as u8);
}
fn encode_u16(bc: &mut Vec<u8>, v: u16) {
    bc.extend_from_slice(&v.to_le_bytes());
}

fn make_proto(bytecode: Vec<u8>, params: u16, name: &str) -> FunctionProto {
    let mut locals = Vec::<LocalDescriptor>::new();
    for i in 0..params {
        locals.push(LocalDescriptor {
            name: format!("arg{i}"),
            kind: rusty_js_ast::VariableKind::Let,
            depth: 0,
        });
    }
    FunctionProto {
        bytecode,
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

fn add_proto() -> FunctionProto {
    // function add(a, b) { return a + b; }
    //   pc=0 LoadArg 0  (3 bytes)
    //   pc=3 LoadArg 1  (3 bytes)
    //   pc=6 Add        (1 byte)
    //   pc=7 Return     (1 byte)
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::LoadArg);
    encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::LoadArg);
    encode_u16(&mut bc, 1);
    encode_op(&mut bc, Op::Add);
    encode_op(&mut bc, Op::Return);
    make_proto(bc, 2, "add")
}

#[test]
fn resume_from_deopt_state_runs_remaining_bytecode() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();

    let proto = add_proto();

    // Construct a recovered state that simulates a deopt at the Add op
    // (pc=6). The interpreter, resumed here, should execute the Add
    // (using the stack-recovered lhs=10 + rhs=32) and then Return,
    // yielding 42.
    let state = DeoptRecoveredState {
        reason: DeoptReason::IntegerOverflow { op_pc: 6 },
        resume_pc: 6,
        local_values: vec![(0, 10), (1, 32)],
        stack_values: vec![(0, 10), (1, 32)],
    };

    let result = rt
        .resume_from_deopt_state(
            &proto,
            Value::Undefined,
            vec![Value::Number(10.0), Value::Number(32.0)],
            &state,
        )
        .expect("resume_from_deopt_state");

    match result {
        Value::Number(n) => assert_eq!(
            n, 42.0,
            "Add of 10 + 32 from recovered stack should yield 42; got {n}"
        ),
        other => panic!("expected Number(42); got {other:?}"),
    }
}

#[test]
fn resume_from_deopt_state_widens_i64_to_f64() {
    // Recovered state carries i64; resume widens to Number(f64).
    // Synthetic: a function that just returns its first local.
    //   pc=0 LoadArg 0
    //   pc=3 Return
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::LoadArg);
    encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::Return);
    let proto = make_proto(bc, 1, "identity");

    let mut rt = Runtime::new();
    rt.install_intrinsics();

    // Resume at pc=3 (Return) with the operand stack carrying the
    // recovered value already; the interpreter just returns it.
    let state = DeoptRecoveredState {
        reason: DeoptReason::IntegerOverflow { op_pc: 3 },
        resume_pc: 3,
        local_values: vec![(0, 12345)],
        // Return pops the operand stack; recovered stack carries 12345.
        stack_values: vec![(0, 12345)],
    };

    let result = rt
        .resume_from_deopt_state(
            &proto,
            Value::Undefined,
            vec![Value::Number(0.0)], // discarded; recovered state wins
            &state,
        )
        .expect("resume_from_deopt_state");

    match result {
        Value::Number(n) => assert_eq!(
            n, 12345.0,
            "recovered i64 should widen to Number(12345.0); got {n}"
        ),
        other => panic!("expected Number(12345); got {other:?}"),
    }
}
