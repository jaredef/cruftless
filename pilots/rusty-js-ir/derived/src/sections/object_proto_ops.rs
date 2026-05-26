//! ECMA-262 §20.1.2.{12, 21, 14, 16, 17} — Object.{getPrototypeOf,
//! setPrototypeOf, isExtensible, isFrozen, isSealed}.
//!
//! All five are thin wrappers over their corresponding abstract op +
//! ToObject (or null/primitive short-circuit). Each becomes one or two
//! IR steps, all routing through CallBuiltin to the matching Runtime
//! helper extracted in interp.rs.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}
fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn build_get_prototype_of() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Let {
                name: "obj".into(),
                value: Expr::ToObject(b(v("target"))),
            },
        },
        Step {
            spec_step: "2".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "get_prototype_of_via",
                args: vec![v("obj")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.12".into(),
        rust_name: "object_get_prototype_of".into(),
        title: "Object.getPrototypeOf ( O )".into(),
        body,
    }
}

pub fn build_set_prototype_of() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.proto".into(),
            node: IRNode::Let {
                name: "proto".into(),
                value: Expr::Arg(1),
            },
        },
        // §20.1.2.21 step 1: Let O be ? RequireObjectCoercible(O).
        Step {
            spec_step: "1".into(),
            node: IRNode::Let {
                name: "o".into(),
                // ToObject covers RequireObjectCoercible + primitive box;
                // matches the Tier-1.5 convention used elsewhere in the IR.
                value: Expr::ToObject(b(v("target"))),
            },
        },
        Step {
            spec_step: "2".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "set_prototype_of_via",
                args: vec![v("o"), v("proto")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.21".into(),
        rust_name: "object_set_prototype_of".into(),
        title: "Object.setPrototypeOf ( O, proto )".into(),
        body,
    }
}

pub fn build_is_extensible() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "is_extensible_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.14".into(),
        rust_name: "object_is_extensible".into(),
        title: "Object.isExtensible ( O )".into(),
        body,
    }
}

pub fn build_is_frozen() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "is_frozen_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.16".into(),
        rust_name: "object_is_frozen".into(),
        title: "Object.isFrozen ( O )".into(),
        body,
    }
}

pub fn build_is_sealed() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "is_sealed_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.17".into(),
        rust_name: "object_is_sealed".into(),
        title: "Object.isSealed ( O )".into(),
        body,
    }
}

// ──────────────── linter records ────────────────

pub fn spec_steps_get_prototype_of() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord {
            step_id: "1".into(),
            abstract_ops: vec!["ToObject"],
            throws: None,
            prose: "Let obj be ? ToObject(O).",
        },
        SpecStepRecord {
            step_id: "2".into(),
            abstract_ops: vec!["get_prototype_of_via"],
            throws: None,
            prose: "Return ? obj.[[GetPrototypeOf]]().",
        },
    ]
}

pub fn spec_steps_set_prototype_of() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord {
            step_id: "1".into(),
            abstract_ops: vec!["ToObject"],
            throws: None,
            prose: "Let O be ? RequireObjectCoercible(O).",
        },
        SpecStepRecord {
            step_id: "2".into(),
            abstract_ops: vec!["set_prototype_of_via"],
            throws: None,
            prose: "Perform ? O.[[SetPrototypeOf]](proto). Return O.",
        },
    ]
}

pub fn spec_steps_is_extensible() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["is_extensible_via"],
        throws: None,
        prose: "If Type(O) is not Object, return false. Return ? IsExtensible(O).",
    }]
}

pub fn spec_steps_is_frozen() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["is_frozen_via"],
        throws: None,
        prose: "If Type(O) is not Object, return true. Return ? TestIntegrityLevel(O, frozen).",
    }]
}

pub fn spec_steps_is_sealed() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["is_sealed_via"],
        throws: None,
        prose: "If Type(O) is not Object, return true. Return ? TestIntegrityLevel(O, sealed).",
    }]
}
