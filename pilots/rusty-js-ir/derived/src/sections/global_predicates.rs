//! ECMA-262 §19.2.{2, 3} — Global isFinite / isNaN.
//!
//! Unlike Number.isFinite / Number.isNaN, these *coerce* via ToNumber.
//! `isNaN("abc")` returns true (since ToNumber("abc") is NaN); `Number.isNaN("abc")`
//! returns false (typeof check rejects non-Number).

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn build_is_nan() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.number".into(),
            node: IRNode::Let {
                name: "number".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "global_is_nan_via",
                args: vec![v("number")],
            }),
        },
    ];
    IRFunction {
        spec_section: "19.2.3".into(),
        rust_name: "global_is_nan".into(),
        title: "isNaN ( number )".into(),
        body,
    }
}

pub fn build_is_finite() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.number".into(),
            node: IRNode::Let {
                name: "number".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "global_is_finite_via",
                args: vec![v("number")],
            }),
        },
    ];
    IRFunction {
        spec_section: "19.2.2".into(),
        rust_name: "global_is_finite".into(),
        title: "isFinite ( number )".into(),
        body,
    }
}

pub fn spec_steps_is_nan() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["global_is_nan_via"],
        throws: None,
        prose: "Let num be ? ToNumber(number). Return true if num is NaN, else false.",
    }]
}

pub fn spec_steps_is_finite() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["global_is_finite_via"],
        throws: None,
        prose: "Let num be ? ToNumber(number). Return true if num is finite, else false.",
    }]
}
