//! ECMA-262 §21.1.2.{2,3,4,5} — Number.{isFinite, isInteger, isNaN, isSafeInteger}.
//!
//! All four are typeof-Number checks composed with a property test
//! (finite / integer / NaN / safe-integer). Unlike global isNaN, these
//! do NOT coerce — non-Number args return false.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

fn one_step_builtin(spec: &str, rust: &str, title: &str, builtin: &'static str) -> IRFunction {
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
                name: builtin,
                args: vec![v("number")],
            }),
        },
    ];
    IRFunction {
        spec_section: spec.into(),
        rust_name: rust.into(),
        title: title.into(),
        body,
    }
}

pub fn build_is_finite() -> IRFunction {
    one_step_builtin(
        "21.1.2.2",
        "number_is_finite",
        "Number.isFinite ( number )",
        "number_is_finite_via",
    )
}
pub fn build_is_integer() -> IRFunction {
    one_step_builtin(
        "21.1.2.3",
        "number_is_integer",
        "Number.isInteger ( number )",
        "number_is_integer_via",
    )
}
pub fn build_is_nan() -> IRFunction {
    one_step_builtin(
        "21.1.2.4",
        "number_is_nan",
        "Number.isNaN ( number )",
        "number_is_nan_via",
    )
}
pub fn build_is_safe_integer() -> IRFunction {
    one_step_builtin(
        "21.1.2.5",
        "number_is_safe_integer",
        "Number.isSafeInteger ( number )",
        "number_is_safe_integer_via",
    )
}

fn one_step_spec(rust_op: &'static str, prose: &'static str) -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec![rust_op],
        throws: None,
        prose,
    }]
}

pub fn spec_steps_is_finite() -> Vec<SpecStepRecord> {
    one_step_spec("number_is_finite_via",
        "If Type(number) is not Number, return false. If number is NaN or ±∞, return false. Otherwise return true.")
}
pub fn spec_steps_is_integer() -> Vec<SpecStepRecord> {
    one_step_spec("number_is_integer_via",
        "If Type(number) is not Number, return false. If number is not an integer, return false. Otherwise return true.")
}
pub fn spec_steps_is_nan() -> Vec<SpecStepRecord> {
    one_step_spec("number_is_nan_via",
        "If Type(number) is not Number, return false. If number is NaN, return true; otherwise return false.")
}
pub fn spec_steps_is_safe_integer() -> Vec<SpecStepRecord> {
    one_step_spec(
        "number_is_safe_integer_via",
        "If IsIntegralNumber(number) is true and abs(number) ≤ 2^53−1, return true; else false.",
    )
}
