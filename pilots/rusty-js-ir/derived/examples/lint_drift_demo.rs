//! Demonstrate that the linter catches drift between IR and spec.
//!
//! We start from the §23.1.3.20 Array.prototype.map IR, then induce
//! drift by removing step 3 (the IsCallable check) — exactly the kind
//! of missing-step bug surfaced ~30 times during the 2026-05-18/19
//! P62 substrate stretch. Re-run the linter; it should flag the
//! omission and reject the IR.
//!
//! Run with: `cargo run --example lint_drift_demo -p rusty-js-ir`

use rusty_js_ir::lint::lint;
use rusty_js_ir::sections::array_prototype_map;

fn main() {
    let mut f = array_prototype_map::build();
    let spec = array_prototype_map::spec_steps();

    // Induce drift: remove step 3 (the IsCallable check) entirely.
    f.body.retain(|s| s.spec_step != "3");

    let report = lint(&f, &spec);
    println!("After dropping IR step 3 (IsCallable check), linter findings:");
    for finding in &report.findings {
        println!("  step {}: {}", finding.spec_step, finding.message);
    }
    if report.findings.is_empty() {
        eprintln!("BUG: linter should have caught the missing step");
        std::process::exit(1);
    }
    println!("\n✓ Drift detected at resolver-instance #0b boundary, before reaching the runtime.");
    println!("  This is exactly the bug class that took ~30 commits to close in the P62 stretch.");
}
