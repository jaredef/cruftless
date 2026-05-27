//! JIT vs interpreter speed test for the sum(N) function.
//!
//! Per pilots/rusty-js-jit/seed.md §I.1: first-cut JIT's success
//! criterion is "noticeably faster than interpreter on a function called
//! >1000 times." This bench measures the gap on the canonical numeric
//! hot-loop benchmark from the speed test stack (sum(1_000_000) =
//! 532ms in the cruftless interpreter).
//!
//! Per JIT-EXT 5: also benches a hand-built typed-i64 version of sum()
//! that uses the Doc 731 §XIV.d alphabet promotion (AddI64, LtI64,
//! IncI64) to demonstrate the β path end-to-end. The typed version is
//! the same algorithm, but the bytecode encodes the operand-type
//! discrimination, so the JIT has no cheating to do.

use rusty_js_bytecode::compile_module;
use rusty_js_bytecode::compiler::{FunctionProto, LocalDescriptor, UpvalueDescriptor};
use rusty_js_bytecode::constants::ConstantsPool;
use rusty_js_bytecode::op::{encode_i32, encode_op, encode_u16, Op};
use rusty_js_bytecode::Constant;
use rusty_js_jit::compile_function;
use std::time::Instant;

fn main() {
    let src = r#"function sum(n) { var s = 0; for (var i = 0; i < n; i++) s = s + i; return s; }"#;
    let m = compile_module(src).expect("compile module");
    let sum_proto = m
        .constants
        .entries()
        .iter()
        .find_map(|c| match c {
            Constant::Function(p) if p.display_name == "sum" => Some((**p).clone()),
            _ => None,
        })
        .expect("find sum proto");

    // JIT-compile.
    let t_jit_compile_start = Instant::now();
    let jit = compile_function(&sum_proto).expect("JIT compile failed");
    let jit_compile_ms = t_jit_compile_start.elapsed().as_secs_f64() * 1000.0;
    println!("JIT compile time: {:.3}ms", jit_compile_ms);

    // Warmup.
    for _ in 0..3 {
        let _ = jit.func.call1(10_000.0);
    }

    // Bench: sum(N) for N = 1_000_000.
    let n: f64 = 1_000_000.0;
    let iters = 10;
    let t_start = Instant::now();
    let mut total: f64 = 0.0;
    for _ in 0..iters {
        total = jit.func.call1(n);
    }
    let elapsed = t_start.elapsed().as_secs_f64() * 1000.0;
    let per_call = elapsed / iters as f64;
    println!("JIT sum({}) = {}", n, total);
    println!(
        "JIT mean over {} iters: {:.4}ms/call (total {:.3}ms)",
        iters, per_call, elapsed
    );

    // Compare against the earlier interpreter baseline from the
    // session's speed test (cruftless interpreter sum(1M) ≈ 532ms).
    let interpreter_baseline_ms = 532.0;
    let speedup = interpreter_baseline_ms / per_call;
    println!();
    println!(
        "Interpreter baseline (from session speed test): {:.1}ms",
        interpreter_baseline_ms
    );
    println!("JIT speedup over interpreter: {:.1}x", speedup);
    println!();
    println!("Bun (V8-class JIT) baseline from speed test: 3ms");
    println!("JIT vs Bun ratio: {:.2}x of Bun", per_call / 3.0);

    println!();
    println!("=== JIT-EXT 5 typed-i64 bench (β-path alphabet promotion) ===");
    // Hand-built FunctionProto using AddI64/LtI64/IncI64 — same as
    // the jit_typed_i64_sum test. The bytecode encodes the operand-
    // type discrimination at the alphabet level; the JIT translates
    // each typed op directly to its Cranelift instruction with no
    // type assumption needed at the JIT tier.
    let typed_proto = build_typed_i64_sum_proto();
    let t_jit_compile_start2 = Instant::now();
    let jit2 = compile_function(&typed_proto).expect("typed-i64 JIT compile failed");
    let jit_compile_ms2 = t_jit_compile_start2.elapsed().as_secs_f64() * 1000.0;
    println!("typed-i64 JIT compile time: {:.3}ms", jit_compile_ms2);
    for _ in 0..3 {
        let _ = jit2.func.call1(10_000.0);
    }
    let t_start2 = Instant::now();
    let mut total2: f64 = 0.0;
    for _ in 0..iters {
        total2 = jit2.func.call1(n);
    }
    let elapsed2 = t_start2.elapsed().as_secs_f64() * 1000.0;
    let per_call2 = elapsed2 / iters as f64;
    println!("typed-i64 JIT sum({}) = {}", n, total2);
    println!(
        "typed-i64 JIT mean over {} iters: {:.4}ms/call (total {:.3}ms)",
        iters, per_call2, elapsed2
    );
    println!();
    println!("=== summary ===");
    println!(
        "interpreter:                  {:.0}ms",
        interpreter_baseline_ms
    );
    println!(
        "JIT (plain ops, i64-cheat):   {:.3}ms  ({:.1}x speedup)",
        per_call,
        interpreter_baseline_ms / per_call
    );
    println!(
        "JIT (typed-i64 ops):          {:.3}ms  ({:.1}x speedup)",
        per_call2,
        interpreter_baseline_ms / per_call2
    );
    println!("Bun (V8 JIT):                 3.000ms");
}

fn build_typed_i64_sum_proto() -> FunctionProto {
    // function tsum(n) { var s=0, i=0; while (i<n) { s = s+i; i++; } return s; }
    // using AddI64 / LtI64 / IncI64 directly.
    let mut bc = Vec::new();
    encode_op(&mut bc, Op::PushI32);
    encode_i32(&mut bc, 0);
    encode_op(&mut bc, Op::StoreLocal);
    encode_u16(&mut bc, 1);
    encode_op(&mut bc, Op::PushI32);
    encode_i32(&mut bc, 0);
    encode_op(&mut bc, Op::StoreLocal);
    encode_u16(&mut bc, 2);
    let loop_top = bc.len();
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 2);
    encode_op(&mut bc, Op::LoadArg);
    encode_u16(&mut bc, 0);
    encode_op(&mut bc, Op::LtI64);
    encode_op(&mut bc, Op::JumpIfFalse);
    let jif_disp_at = bc.len();
    encode_i32(&mut bc, 0);
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 1);
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 2);
    encode_op(&mut bc, Op::AddI64);
    encode_op(&mut bc, Op::StoreLocal);
    encode_u16(&mut bc, 1);
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 2);
    encode_op(&mut bc, Op::IncI64);
    encode_op(&mut bc, Op::StoreLocal);
    encode_u16(&mut bc, 2);
    encode_op(&mut bc, Op::Jump);
    let jump_next_pc = bc.len() + 4;
    encode_i32(&mut bc, loop_top as i32 - jump_next_pc as i32);
    let exit_pc = bc.len();
    let jif_disp = exit_pc as i32 - (jif_disp_at + 4) as i32;
    bc[jif_disp_at..jif_disp_at + 4].copy_from_slice(&jif_disp.to_le_bytes());
    encode_op(&mut bc, Op::LoadLocal);
    encode_u16(&mut bc, 1);
    encode_op(&mut bc, Op::Return);
    encode_op(&mut bc, Op::ReturnUndef);
    FunctionProto {
        bytecode: bc,
        constants: ConstantsPool::new(),
        params: 1,
        display_name: "tsum".to_string(),
        function_length: 1,
        locals: vec![
            LocalDescriptor {
                name: "n".to_string(),
                kind: rusty_js_ast::VariableKind::Let,
                depth: 0,
            },
            LocalDescriptor {
                name: "s".to_string(),
                kind: rusty_js_ast::VariableKind::Var,
                depth: 0,
            },
            LocalDescriptor {
                name: "i".to_string(),
                kind: rusty_js_ast::VariableKind::Var,
                depth: 0,
            },
        ],
        upvalues: Vec::<UpvalueDescriptor>::new(),
        rest_param_slot: None,
        arguments_slot: None,
        self_name_slot: None,
        param_prologue_end: 0,
        is_generator: false,
        is_async: false,
        source_url: String::new(),
        line_starts: Vec::new(),
        source_map: Vec::new(),
        construct_tags: Vec::new(),
        strict: false,
    }
}
