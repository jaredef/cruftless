//! JIT vs interpreter speed test for the sum(N) function.
//!
//! Per pilots/rusty-js-jit/seed.md §I.1: first-cut JIT's success
//! criterion is "noticeably faster than interpreter on a function called
//! >1000 times." This bench measures the gap on the canonical numeric
//! hot-loop benchmark from the speed test stack (sum(1_000_000) =
//! 532ms in the cruftless interpreter).

use rusty_js_bytecode::compile_module;
use rusty_js_bytecode::Constant;
use rusty_js_jit::compile_function;
use std::time::Instant;

fn main() {
    let src = r#"function sum(n) { var s = 0; for (var i = 0; i < n; i++) s = s + i; return s; }"#;
    let m = compile_module(src).expect("compile module");
    let sum_proto = m.constants.entries().iter()
        .find_map(|c| match c { Constant::Function(p) if p.display_name == "sum" => Some((**p).clone()), _ => None })
        .expect("find sum proto");

    // JIT-compile.
    let t_jit_compile_start = Instant::now();
    let jit = compile_function(&sum_proto).expect("JIT compile failed");
    let jit_compile_ms = t_jit_compile_start.elapsed().as_secs_f64() * 1000.0;
    println!("JIT compile time: {:.3}ms", jit_compile_ms);

    // Warmup.
    for _ in 0..3 {
        let _ = jit.func.call1(10_000);
    }

    // Bench: sum(N) for N = 1_000_000.
    let n: i64 = 1_000_000;
    let iters = 10;
    let t_start = Instant::now();
    let mut total: i64 = 0;
    for _ in 0..iters {
        total = jit.func.call1(n);
    }
    let elapsed = t_start.elapsed().as_secs_f64() * 1000.0;
    let per_call = elapsed / iters as f64;
    println!("JIT sum({}) = {}", n, total);
    println!("JIT mean over {} iters: {:.4}ms/call (total {:.3}ms)", iters, per_call, elapsed);

    // Compare against the earlier interpreter baseline from the
    // session's speed test (cruftless interpreter sum(1M) ≈ 532ms).
    let interpreter_baseline_ms = 532.0;
    let speedup = interpreter_baseline_ms / per_call;
    println!();
    println!("Interpreter baseline (from session speed test): {:.1}ms", interpreter_baseline_ms);
    println!("JIT speedup over interpreter: {:.1}x", speedup);
    println!();
    println!("Bun (V8-class JIT) baseline from speed test: 3ms");
    println!("JIT vs Bun ratio: {:.2}x of Bun", per_call / 3.0);
}
