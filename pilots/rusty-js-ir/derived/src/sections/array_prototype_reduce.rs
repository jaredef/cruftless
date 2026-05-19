//! ECMA-262 §23.1.3.24 — Array.prototype.reduce ( callbackfn [ , initialValue ] ).
//!
//! E30 refactor: lifted to 1-step CallBuiltin. The runtime helper handles
//! initialValue presence (via args.len()), find-first-present-index seeding,
//! sparse-hole skipping, and TypeError on empty-with-no-initial — paths the
//! prior detailed-step IR modeled but never wired (Tier-1.8 left this
//! section IR-only). The CallBuiltin lift preserves cruftless's hand-written
//! semantics exactly.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

pub fn build_reduce() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "array_proto_reduce_via", args: vec![Expr::AllArgs],
        })},
    ];
    IRFunction {
        spec_section: "23.1.3.24".into(),
        rust_name: "array_prototype_reduce".into(),
        title: "Array.prototype.reduce ( callbackfn [ , initialValue ] )".into(),
        body,
    }
}

pub fn spec_steps_reduce() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_reduce_via"], throws: None,
        prose: "Seed accumulator (from initialValue or first present element); fold callback over the array-like skipping holes." }]
}
