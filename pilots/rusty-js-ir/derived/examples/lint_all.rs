//! Run the spec-vs-IR linter against every translated section.
//!
//! Run with: `cargo run --example lint_all -p rusty-js-ir`

use rusty_js_ir::lint::lint;
use rusty_js_ir::sections::{
    array_prototype_find as find, array_prototype_index_search as index_search,
    array_prototype_iteration as iter, array_prototype_map,
    array_prototype_reduce as reduce, object_static,
};

fn main() {
    let sections = vec![
        ("Array.prototype.map", array_prototype_map::build(), array_prototype_map::spec_steps()),
        ("Array.prototype.forEach", iter::build_for_each(), iter::spec_steps_for_each()),
        ("Array.prototype.filter", iter::build_filter(), iter::spec_steps_filter()),
        ("Array.prototype.every", iter::build_every(), iter::spec_steps_every()),
        ("Array.prototype.some", iter::build_some(), iter::spec_steps_some()),
        ("Array.prototype.find", find::build_find(), find::spec_steps_find()),
        ("Array.prototype.findIndex", find::build_find_index(), find::spec_steps_find()),
        ("Array.prototype.findLast", find::build_find_last(), find::spec_steps_find()),
        ("Array.prototype.findLastIndex", find::build_find_last_index(), find::spec_steps_find()),
        ("Array.prototype.indexOf", index_search::build_index_of(), index_search::spec_steps_index_of()),
        ("Array.prototype.includes", index_search::build_includes(), index_search::spec_steps_includes()),
        ("Array.prototype.reduce", reduce::build_reduce(), reduce::spec_steps_reduce()),
        ("Object.keys", object_static::build_keys(), object_static::spec_steps_keys()),
        ("Object.values", object_static::build_values(), object_static::spec_steps_values()),
        ("Object.entries", object_static::build_entries(), object_static::spec_steps_entries()),
    ];

    let mut total_unexpected = 0;
    for (name, f, spec) in &sections {
        // Filter the spec_steps list to drop synthetic-inline records
        // (matching the linter's collect_steps convention from
        // lint.rs). The hand-authored records still carry these for
        // documentation; the diff doesn't track them.
        let filtered_spec: Vec<_> = spec.iter()
            .filter(|r| !r.step_id.ends_with(".throw")
                && !r.step_id.ends_with(".guard")
                && !r.step_id.ends_with(".return")
                && !r.step_id.ends_with(".adj")
                && !r.step_id.ends_with(".seed"))
            .cloned()
            .collect();
        let report = lint(f, &filtered_spec);
        // Filter out known param.* binding-convention findings — those
        // are not in any spec section's algorithm step list.
        let unexpected: Vec<_> = report.findings.iter()
            .filter(|f| !f.spec_step.starts_with("param."))
            .collect();
        if unexpected.is_empty() {
            println!("✓ {} — OK", name);
        } else {
            println!("✗ {} — {} unexpected findings:", name, unexpected.len());
            for f in &unexpected {
                println!("    step {}: {}", f.spec_step, f.message);
            }
            total_unexpected += unexpected.len();
        }
    }

    println!();
    if total_unexpected == 0 {
        println!("All {} translated sections lint clean.", sections.len());
    } else {
        println!("{} total unexpected findings across {} sections.", total_unexpected, sections.len());
        std::process::exit(1);
    }
}
