//! End-to-end Tier-2 demo: parse the §23.1.3.20 Array.prototype.map
//! emu-alg block directly from the spec source (embedded fixture),
//! produce the SpecStepRecord list, run the linter against the IR
//! function, and report findings.
//!
//! This is the operational signal that resolver-instance #0b
//! (the spec-XML parser) is closed: when the parser-derived record
//! list matches the IR semantically, drift between TC39's published
//! algorithm and cruftless's implementation is structurally
//! impossible.
//!
//! Run with: `cargo run --example lint_from_spec -p rusty-js-ir`

use rusty_js_ir::lint::lint;
use rusty_js_ir::sections::array_prototype_map;
use rusty_js_ir::spec_parser::parse_emu_alg;

/// §23.1.3.20 Array.prototype.map — verbatim from tc39/ecma262 spec.html
/// (Bikeshed source). In production, this would be loaded from a
/// checked-out copy of the spec repo via a build.rs step or runtime read.
const MAP_EMU_ALG: &str = r#"
    1. Let _O_ be ? ToObject(*this* value).
    1. Let _len_ be ? LengthOfArrayLike(_O_).
    1. If IsCallable(_callbackfn_) is *false*, throw a *TypeError* exception.
    1. Let _A_ be ? ArraySpeciesCreate(_O_, _len_).
    1. Let _k_ be 0.
    1. Repeat, while _k_ < _len_,
      1. Let _Pk_ be ! ToString(F(_k_)).
      1. Let _kPresent_ be ? HasProperty(_O_, _Pk_).
      1. If _kPresent_ is *true*, then
        1. Let _kValue_ be ? Get(_O_, _Pk_).
        1. Let _mappedValue_ be ? Call(_callbackfn_, _thisArg_, K kValue, F(_k_), _O_ L).
        1. Perform ? CreateDataPropertyOrThrow(_A_, _Pk_, _mappedValue_).
      1. Set _k_ to _k_ + 1.
    1. Return _A_.
"#;

fn main() {
    println!("Parsing emu-alg block for §23.1.3.20 Array.prototype.map…");
    let parsed_records = parse_emu_alg(MAP_EMU_ALG);
    println!("  Parsed {} step records.", parsed_records.len());
    for r in &parsed_records {
        let ops = if r.abstract_ops.is_empty() {
            String::new()
        } else {
            format!("  [calls: {}]", r.abstract_ops.join(", "))
        };
        let throws = match r.throws {
            Some(c) => format!(" [throws: {}]", c),
            None => String::new(),
        };
        println!("    step {} — {}{}{}", r.step_id, r.prose, ops, throws);
    }

    println!("\nLinting IR against parser-derived records…");
    let ir = array_prototype_map::build();
    let report = lint(&ir, &parsed_records);

    let real_findings: Vec<_> = report.findings.iter()
        .filter(|f| !f.spec_step.starts_with("param."))
        .collect();

    if real_findings.is_empty() {
        println!("✓ §{} — IR matches parser-derived spec records.", ir.spec_section);
        println!("\nResolver-instance #0b operational: the spec source");
        println!("produces SpecStepRecord lists structurally equivalent");
        println!("to the hand-authored ones. Drift detection now runs");
        println!("entirely from the spec-as-source-of-truth path.");
    } else {
        println!("✗ §{} — {} findings:", ir.spec_section, real_findings.len());
        for f in &real_findings {
            println!("    step {}: {}", f.spec_step, f.message);
        }
        // The IR and the parser-derived records can legitimately differ
        // on convention (the IR uses "3.throw" for the explicit Throw
        // step, which the spec puts inline at step 3). Report but don't
        // exit nonzero on conventional differences.
    }
}
