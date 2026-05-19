//! Lint §23.1.3.20 Array.prototype.map IR against the canonical step
//! records. Tier 1 — the records are hand-authored; Tier 2 will parse
//! them from ECMA-262 XML.
//!
//! Run with: `cargo run --example lint_array_map -p rusty-js-ir`

use rusty_js_ir::lint::lint;
use rusty_js_ir::sections::array_prototype_map;

fn main() {
    let f = array_prototype_map::build();
    let spec = array_prototype_map::spec_steps();
    let report = lint(&f, &spec);
    if report.ok() {
        println!("§{} — OK (0 findings; {} steps verified)", f.spec_section, spec.len());
    } else {
        eprintln!("§{} — {} findings:", f.spec_section, report.findings.len());
        for finding in &report.findings {
            eprintln!("  step {}: {}", finding.spec_step, finding.message);
        }
        std::process::exit(1);
    }
}
