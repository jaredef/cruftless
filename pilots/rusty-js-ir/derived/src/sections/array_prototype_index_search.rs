//! ECMA-262 §23.1.3.{16, 13} — Array.prototype.{indexOf, includes}.
//!
//! Comparison-based search; no callback. indexOf uses IsStrictlyEqual
//! (NaN != NaN); includes uses SameValueZero (NaN == NaN).
//!
//! E30 refactor: lifted to 1-step CallBuiltin so the runtime helper handles
//! fromIndex normalization (negative offset, ToNumber coercion) — paths the
//! prior detailed-step IR couldn't reach without a signed-Int alphabet
//! extension. Trajectory anchor: the alphabet-extension queue (signed-Int +
//! IndexSub) is still queued; this round chose the CallBuiltin lift instead.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

// ──────────────── §23.1.3.16 indexOf ────────────────

pub fn build_index_of() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "array_proto_index_of_via", args: vec![Expr::AllArgs],
        })},
    ];
    IRFunction {
        spec_section: "23.1.3.16".into(),
        rust_name: "array_prototype_index_of".into(),
        title: "Array.prototype.indexOf ( searchElement [ , fromIndex ] )".into(),
        body,
    }
}

// ──────────────── §23.1.3.14 includes ────────────────

pub fn build_includes() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "array_proto_includes_via", args: vec![Expr::AllArgs],
        })},
    ];
    IRFunction {
        spec_section: "23.1.3.14".into(),
        rust_name: "array_prototype_includes".into(),
        title: "Array.prototype.includes ( searchElement [ , fromIndex ] )".into(),
        body,
    }
}

// ──────────────── linter records ────────────────

pub fn spec_steps_index_of() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_index_of_via"], throws: None,
        prose: "Search the array-like forward from fromIndex; return the first index whose element is strictly equal to searchElement, or -1." }]
}

pub fn spec_steps_includes() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_includes_via"], throws: None,
        prose: "Return whether the array-like contains an element SameValueZero-equal to searchElement, treating holes as undefined." }]
}
