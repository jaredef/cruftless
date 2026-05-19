//! ECMA-262 §21.3.2.{27, 8, 17, 25, 26} — Math.{pow, atan2, hypot, max, min}.
//!
//! pow / atan2: binary (two positional args).
//! max / min / hypot: variadic (all args, dispatched via Expr::AllArgs).

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

// ──────────────── binary ────────────────

fn build_binary(spec: &str, rust_name: &str, title: &str, op: &'static str) -> IRFunction {
    let body = vec![
        Step { spec_step: "param.x".into(),
            node: IRNode::Let { name: "x".into(), value: Expr::Arg(0) }},
        Step { spec_step: "param.y".into(),
            node: IRNode::Let { name: "y".into(), value: Expr::Arg(1) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "math_binary_op_via",
                args: vec![Expr::Str(op.into()), v("x"), v("y")],
            })},
    ];
    IRFunction { spec_section: spec.into(),
        rust_name: rust_name.into(),
        title: title.into(), body }
}

pub fn build_pow() -> IRFunction   { build_binary("21.3.2.27", "math_pow",   "Math.pow ( base, exponent )", "pow") }
pub fn build_atan2() -> IRFunction { build_binary("21.3.2.8",  "math_atan2", "Math.atan2 ( y, x )",         "atan2") }

// ──────────────── variadic ────────────────

fn build_variadic(spec: &str, rust_name: &str, title: &str, builtin: &'static str) -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: builtin,
                args: vec![Expr::AllArgs],
            })},
    ];
    IRFunction { spec_section: spec.into(),
        rust_name: rust_name.into(),
        title: title.into(), body }
}

pub fn build_max() -> IRFunction   { build_variadic("21.3.2.25", "math_max",   "Math.max ( ...values )", "math_max_via") }
pub fn build_min() -> IRFunction   { build_variadic("21.3.2.26", "math_min",   "Math.min ( ...values )", "math_min_via") }
pub fn build_hypot() -> IRFunction { build_variadic("21.3.2.17", "math_hypot", "Math.hypot ( ...values )", "math_hypot_via") }

// ──────────────── linter records ────────────────

fn one_step_spec(rust_op: &'static str, prose: &'static str) -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec![rust_op], throws: None, prose }]
}

pub fn spec_steps_pow() -> Vec<SpecStepRecord>   { one_step_spec("math_binary_op_via", "Let base = ToNumber(x). Let exp = ToNumber(y). Return base^exp.") }
pub fn spec_steps_atan2() -> Vec<SpecStepRecord> { one_step_spec("math_binary_op_via", "Let ny = ToNumber(y). Let nx = ToNumber(x). Return atan2(ny, nx).") }
pub fn spec_steps_max() -> Vec<SpecStepRecord>   { one_step_spec("math_max_via", "Return max of ToNumber-ed args; NaN if any arg is NaN.") }
pub fn spec_steps_min() -> Vec<SpecStepRecord>   { one_step_spec("math_min_via", "Return min of ToNumber-ed args; NaN if any arg is NaN.") }
pub fn spec_steps_hypot() -> Vec<SpecStepRecord> { one_step_spec("math_hypot_via", "Return sqrt(sum of squares of ToNumber-ed args).") }
