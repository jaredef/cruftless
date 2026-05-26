//! ECMA-262 §20.1.2.{7, 20, 18, 13, 14} — Object integrity + comparison ops.
//!
//! Five sections sharing the CallBuiltin pattern:
//!   - Object.freeze(O)            §20.1.2.7
//!   - Object.seal(O)              §20.1.2.20
//!   - Object.preventExtensions(O) §20.1.2.18
//!   - Object.hasOwn(O, P)         §20.1.2.13
//!   - Object.is(a, b)             §20.1.2.14

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn build_freeze() -> IRFunction {
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
                name: "object_freeze_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.7".into(),
        rust_name: "object_freeze".into(),
        title: "Object.freeze ( O )".into(),
        body,
    }
}

pub fn build_seal() -> IRFunction {
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
                name: "object_seal_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.20".into(),
        rust_name: "object_seal".into(),
        title: "Object.seal ( O )".into(),
        body,
    }
}

pub fn build_prevent_extensions() -> IRFunction {
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
                name: "object_prevent_extensions_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.18".into(),
        rust_name: "object_prevent_extensions".into(),
        title: "Object.preventExtensions ( O )".into(),
        body,
    }
}

pub fn build_has_own() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.key".into(),
            node: IRNode::Let {
                name: "key".into(),
                value: Expr::Arg(1),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "object_has_own_via",
                args: vec![v("target"), v("key")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.13".into(),
        rust_name: "object_has_own".into(),
        title: "Object.hasOwn ( O, P )".into(),
        body,
    }
}

pub fn build_is() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.value1".into(),
            node: IRNode::Let {
                name: "value1".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.value2".into(),
            node: IRNode::Let {
                name: "value2".into(),
                value: Expr::Arg(1),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "object_is_via",
                args: vec![v("value1"), v("value2")],
            }),
        },
    ];
    IRFunction {
        spec_section: "20.1.2.14".into(),
        rust_name: "object_is".into(),
        title: "Object.is ( value1, value2 )".into(),
        body,
    }
}

// ──────────────── linter records ────────────────

pub fn spec_steps_freeze() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["object_freeze_via"],
        throws: None,
        prose: "Set integrity level frozen on O.",
    }]
}
pub fn spec_steps_seal() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["object_seal_via"],
        throws: None,
        prose: "Set integrity level sealed on O.",
    }]
}
pub fn spec_steps_prevent_extensions() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["object_prevent_extensions_via"],
        throws: None,
        prose: "Perform ? PreventExtensions(O). Return O.",
    }]
}
pub fn spec_steps_has_own() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["object_has_own_via"],
        throws: None,
        prose: "Return ? HasOwnProperty(? ToObject(O), ? ToPropertyKey(P)).",
    }]
}
pub fn spec_steps_is() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["object_is_via"],
        throws: None,
        prose: "Return SameValue(value1, value2).",
    }]
}
