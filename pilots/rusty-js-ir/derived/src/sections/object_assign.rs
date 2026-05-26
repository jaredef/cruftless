//! ECMA-262 §20.1.2.1 Object.assign — per-source copy step.
//!
//! IR-EXT 69: third higher-resolution-IR section. Encodes the inner
//! per-source loop of Object.assign(target, ...sources) — the work
//! done at §20.1.2.1 step 4 for each source. The outer source-list
//! iteration stays in Rust (object_assign_via) because the IR alphabet
//! doesn't yet model rest-arg iteration cleanly.
//!
//! Spec (per source, with target already ToObject'd):
//!   4.a. If source is undefined or null, return (skip).
//!   4.b. Let from be ToObject(source).
//!   4.c. Let keys be from.[[OwnPropertyKeys]].
//!   4.d. For each key in keys:
//!        i. Let desc be from.[[GetOwnProperty]](key).
//!        ii. If desc is not undefined and desc.[[Enumerable]] is true:
//!            - Let propValue be from.[[Get]](key, from).
//!            - Perform target.[[Set]](key, propValue, target).

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}
fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}

pub fn build_object_assign_source_into() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.source".into(),
            node: IRNode::Let {
                name: "source".into(),
                value: Expr::Arg(1),
            },
        },
        // §20.1.2.1 step 4.a: if source is null/undefined, return target.
        Step {
            spec_step: "4.a.null_check".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(b(v("source")), b(Expr::Null)),
                then_body: vec![Step {
                    spec_step: "4.a.null_return".into(),
                    node: IRNode::Return(v("target")),
                }],
                else_body: vec![],
            },
        },
        Step {
            spec_step: "4.a.undef_check".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(b(v("source")), b(Expr::Undefined)),
                then_body: vec![Step {
                    spec_step: "4.a.undef_return".into(),
                    node: IRNode::Return(v("target")),
                }],
                else_body: vec![],
            },
        },
        // §20.1.2.1 step 4.b: from = ToObject(source). For primitives this
        // boxes; for null/undefined it would throw but we already returned.
        Step {
            spec_step: "4.b.toobject".into(),
            node: IRNode::Let {
                name: "from".into(),
                value: Expr::CallBuiltin {
                    name: "to_object_strict_via",
                    args: vec![v("source")],
                },
            },
        },
        // §20.1.2.1 step 4.c: collect enumerable own string keys of from.
        // The runtime helper handles the [[OwnPropertyKeys]] + enumerable
        // filter + __primitive__/@@ exclusion in one shot.
        Step {
            spec_step: "4.c.keys".into(),
            node: IRNode::Let {
                name: "keys".into(),
                value: Expr::CallBuiltin {
                    name: "own_enumerable_string_keys_via",
                    args: vec![v("from")],
                },
            },
        },
        // §20.1.2.1 step 4.d: for each key, copy from→target via Get/Set.
        Step {
            spec_step: "4.d.len".into(),
            node: IRNode::LetIndex {
                name: "len".into(),
                value: Expr::LengthOfArrayLike(b(v("keys"))),
            },
        },
        Step {
            spec_step: "4.d.k".into(),
            node: IRNode::LetIndex {
                name: "k".into(),
                value: Expr::IntConst(0),
            },
        },
        Step {
            spec_step: "4.d.loop".into(),
            node: IRNode::While {
                cond: Expr::Lt(b(v("k")), b(v("len"))),
                body: vec![
                    // key = keys[k] — read the string at index k from the keys
                    // array. IndexAsKey(k) lowers to k.to_string() for the
                    // numeric-string property lookup.
                    Step {
                        spec_step: "4.d.i.key".into(),
                        node: IRNode::Let {
                            name: "key".into(),
                            value: Expr::Get(b(v("keys")), b(Expr::IndexAsKey(b(v("k"))))),
                        },
                    },
                    // propValue = Get(from, key) — accessor-getter aware.
                    Step {
                        spec_step: "4.d.ii.value".into(),
                        node: IRNode::Let {
                            name: "propValue".into(),
                            value: Expr::CallBuiltin {
                                name: "get_via",
                                args: vec![v("from"), v("key")],
                            },
                        },
                    },
                    // Set(target, key, propValue) — accessor-setter aware.
                    Step {
                        spec_step: "4.d.ii.set".into(),
                        node: IRNode::Expr(Expr::CallBuiltin {
                            name: "set_via",
                            args: vec![v("target"), v("key"), v("propValue")],
                        }),
                    },
                    // k++.
                    Step {
                        spec_step: "4.d.incr".into(),
                        node: IRNode::AssignIndex {
                            name: "k".into(),
                            value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))),
                        },
                    },
                ],
            },
        },
        // Return target.
        Step {
            spec_step: "5.return".into(),
            node: IRNode::Return(v("target")),
        },
    ];

    IRFunction {
        spec_section: "20.1.2.1".into(),
        rust_name: "object_assign_source_into".into(),
        title: "Object.assign per-source step (§20.1.2.1 step 4)".into(),
        body,
    }
}

pub fn spec_steps_object_assign_source_into() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord {
            step_id: "4.a.null_check".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Skip null source.",
        },
        SpecStepRecord {
            step_id: "4.a.null_return".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Return target on null source.",
        },
        SpecStepRecord {
            step_id: "4.a.undef_check".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Skip undefined source.",
        },
        SpecStepRecord {
            step_id: "4.a.undef_return".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Return target on undefined source.",
        },
        SpecStepRecord {
            step_id: "4.b.toobject".into(),
            abstract_ops: vec!["to_object_strict_via"],
            throws: None,
            prose: "from = ToObject(source).",
        },
        SpecStepRecord {
            step_id: "4.c.keys".into(),
            abstract_ops: vec!["own_enumerable_string_keys_via"],
            throws: None,
            prose: "Collect enumerable own string keys of from.",
        },
        SpecStepRecord {
            step_id: "4.d.len".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Length of keys array.",
        },
        SpecStepRecord {
            step_id: "4.d.k".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Initialize index k = 0.",
        },
        SpecStepRecord {
            step_id: "4.d.loop".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Loop while k < len.",
        },
        SpecStepRecord {
            step_id: "4.d.i.key".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Let key = keys[k].",
        },
        SpecStepRecord {
            step_id: "4.d.ii.value".into(),
            abstract_ops: vec!["get_via"],
            throws: None,
            prose: "propValue = Get(from, key).",
        },
        SpecStepRecord {
            step_id: "4.d.ii.set".into(),
            abstract_ops: vec!["set_via"],
            throws: None,
            prose: "Set(target, key, propValue).",
        },
        SpecStepRecord {
            step_id: "4.d.incr".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Increment k.",
        },
        SpecStepRecord {
            step_id: "5.return".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Return target.",
        },
    ]
}
