//! JSF-EXT 1: JSON.stringify per-shape micro-benchmark.
//!
//! Decomposes the per-op cost across 5 stringify shape classes:
//!   A. small-object (10 keys, mixed types) — common case
//!   B. deeply-nested (5 levels) — recursion + allocation pressure
//!   C. array-of-objects (50 elements × 5 keys) — iteration heavy
//!   D. number-only (single Number) — exercises number_to_string
//!   E. string-only (single String, mixed ASCII + control) — exercises json_quote_string
//!
//! Per-iter cost decomposition supplies the JSF-EXT 2 design's hot-path
//! component target list.
//!
//! Builds + runs the cruft binary on a generated .mjs fixture rather than
//! benching Rust internals directly — keeps the bench within the same
//! dispatch path the realistic CRB workload uses.

use std::time::Instant;
use std::process::{Command, Stdio};

const FIXTURES_DIR: &str = "/tmp/jsf-bench";

fn write_fixture(name: &str, body: &str) -> String {
    let path = format!("{}/{}.mjs", FIXTURES_DIR, name);
    std::fs::create_dir_all(FIXTURES_DIR).expect("create dir");
    std::fs::write(&path, body).expect("write fixture");
    path
}

fn run_bench(name: &str, fixture_path: &str, runs: u32) -> u128 {
    let mut times = Vec::with_capacity(runs as usize);
    for _ in 0..runs {
        let t0 = Instant::now();
        let status = Command::new("/home/jaredef/bin/cruft")
            .arg(fixture_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status().expect("cruft run");
        if !status.success() {
            eprintln!("[{}] cruft exited non-zero", name);
            return u128::MAX;
        }
        times.push(t0.elapsed().as_micros());
    }
    times.sort();
    let median = times[times.len() / 2];
    println!("  {:<26} median: {:>7} us  (min={:>6} max={:>6})",
        name, median, times[0], times[times.len() - 1]);
    median
}

fn main() {
    // Bench A: small-object stringify
    let body_a = r#"
        const o = { a: 1, b: "hi", c: true, d: null, e: 3.14, f: 42, g: "world", h: false, i: 0, j: -7 };
        let n = 0;
        for (let i = 0; i < 100000; i++) n += JSON.stringify(o).length;
        console.log(n);
    "#;
    let path_a = write_fixture("bench_a_small_object", body_a);

    // Bench B: deeply-nested
    let body_b = r#"
        const o = { a: { b: { c: { d: { e: { f: 42, g: "deep" } } } } } };
        let n = 0;
        for (let i = 0; i < 100000; i++) n += JSON.stringify(o).length;
        console.log(n);
    "#;
    let path_b = write_fixture("bench_b_deep_nested", body_b);

    // Bench C: array-of-objects
    let body_c = r#"
        const arr = [];
        for (let i = 0; i < 50; i++) arr.push({ id: i, name: "name_" + i, score: i*13, active: (i%2)===0, tag: "x" });
        let n = 0;
        for (let i = 0; i < 5000; i++) n += JSON.stringify(arr).length;
        console.log(n);
    "#;
    let path_c = write_fixture("bench_c_array_of_obj", body_c);

    // Bench D: number-only (drives number_to_string)
    let body_d = r#"
        let n = 0;
        for (let i = 0; i < 1000000; i++) n += JSON.stringify(i * 13).length;
        console.log(n);
    "#;
    let path_d = write_fixture("bench_d_number_only", body_d);

    // Bench E: string-only with ASCII + escapes (drives json_quote_string)
    let body_e = r#"
        const s = "Hello, \"World\"\nThis is a test\twith various\\escapes";
        let n = 0;
        for (let i = 0; i < 1000000; i++) n += JSON.stringify(s).length;
        console.log(n);
    "#;
    let path_e = write_fixture("bench_e_string_only", body_e);

    println!("JSF-EXT 1 — JSON.stringify per-shape micro-bench");
    println!("================================================");
    println!("each fixture run 3 times; median reported");
    println!();

    println!("CRUFT:");
    let ma = run_bench("A: small-object 100k", &path_a, 3);
    let mb = run_bench("B: deep-nested 100k",  &path_b, 3);
    let mc = run_bench("C: array-of-obj 5k",   &path_c, 3);
    let md = run_bench("D: number-only 1M",    &path_d, 3);
    let me = run_bench("E: string-only 1M",    &path_e, 3);

    // For comparison, run node baseline.
    println!();
    println!("NODE baseline:");
    let node_run = |path: &str| {
        let mut times = Vec::with_capacity(3);
        for _ in 0..3 {
            let t0 = Instant::now();
            let _ = Command::new("/usr/bin/node").arg(path)
                .stdout(Stdio::null()).stderr(Stdio::null())
                .status().expect("node run");
            times.push(t0.elapsed().as_micros());
        }
        times.sort();
        times[times.len()/2]
    };
    let na = node_run(&path_a);
    let nb = node_run(&path_b);
    let nc = node_run(&path_c);
    let nd = node_run(&path_d);
    let ne = node_run(&path_e);
    println!("  A: small-object 100k       node median: {} us  (cruft/node = {:.2}x)", na, ma as f64 / na as f64);
    println!("  B: deep-nested 100k        node median: {} us  (cruft/node = {:.2}x)", nb, mb as f64 / nb as f64);
    println!("  C: array-of-obj 5k         node median: {} us  (cruft/node = {:.2}x)", nc, mc as f64 / nc as f64);
    println!("  D: number-only 1M          node median: {} us  (cruft/node = {:.2}x)", nd, md as f64 / nd as f64);
    println!("  E: string-only 1M          node median: {} us  (cruft/node = {:.2}x)", ne, me as f64 / ne as f64);
}
