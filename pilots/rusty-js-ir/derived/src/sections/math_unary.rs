//! ECMA-262 §21.3.2.{1, 9, 5, 28, 35, 30, 9, 10} — Math unary numeric ops.
//!
//! All eight share the shape "Return Math op of ToNumber(x)". Each
//! lowers via the shared `rt.math_unary_op_via(op_name, x)` helper —
//! a single Rust function with a string-dispatch table.
//!
//! Tier 1.10 pattern: one Runtime helper, eight IR sections, each
//! ~12 LOC. Demonstrates the "shared dispatcher + per-section thin
//! wrapper" idiom for the cluster pattern where the spec sections
//! are structurally identical modulo a single op-name string.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

fn build_unary(spec: &str, rust_name: &str, title: &str, op: &'static str) -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.x".into(),
            node: IRNode::Let {
                name: "x".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "math_unary_op_via",
                args: vec![Expr::Str(op.into()), v("x")],
            }),
        },
    ];
    IRFunction {
        spec_section: spec.into(),
        rust_name: rust_name.into(),
        title: title.into(),
        body,
    }
}

pub fn build_abs() -> IRFunction {
    build_unary("21.3.2.1", "math_abs", "Math.abs ( x )", "abs")
}
pub fn build_floor() -> IRFunction {
    build_unary("21.3.2.16", "math_floor", "Math.floor ( x )", "floor")
}
pub fn build_ceil() -> IRFunction {
    build_unary("21.3.2.10", "math_ceil", "Math.ceil ( x )", "ceil")
}
pub fn build_round() -> IRFunction {
    build_unary("21.3.2.28", "math_round", "Math.round ( x )", "round")
}
pub fn build_trunc() -> IRFunction {
    build_unary("21.3.2.35", "math_trunc", "Math.trunc ( x )", "trunc")
}
pub fn build_sqrt() -> IRFunction {
    build_unary("21.3.2.32", "math_sqrt", "Math.sqrt ( x )", "sqrt")
}
pub fn build_cbrt() -> IRFunction {
    build_unary("21.3.2.9", "math_cbrt", "Math.cbrt ( x )", "cbrt")
}
pub fn build_sign() -> IRFunction {
    build_unary("21.3.2.30", "math_sign", "Math.sign ( x )", "sign")
}
// Exponential / logarithmic family.
pub fn build_exp() -> IRFunction {
    build_unary("21.3.2.14", "math_exp", "Math.exp ( x )", "exp")
}
pub fn build_expm1() -> IRFunction {
    build_unary("21.3.2.15", "math_expm1", "Math.expm1 ( x )", "expm1")
}
pub fn build_log() -> IRFunction {
    build_unary("21.3.2.20", "math_log", "Math.log ( x )", "log")
}
pub fn build_log1p() -> IRFunction {
    build_unary("21.3.2.21", "math_log1p", "Math.log1p ( x )", "log1p")
}
pub fn build_log2() -> IRFunction {
    build_unary("21.3.2.22", "math_log2", "Math.log2 ( x )", "log2")
}
pub fn build_log10() -> IRFunction {
    build_unary("21.3.2.23", "math_log10", "Math.log10 ( x )", "log10")
}
// Trigonometric family.
pub fn build_sin() -> IRFunction {
    build_unary("21.3.2.29", "math_sin", "Math.sin ( x )", "sin")
}
pub fn build_cos() -> IRFunction {
    build_unary("21.3.2.12", "math_cos", "Math.cos ( x )", "cos")
}
pub fn build_tan() -> IRFunction {
    build_unary("21.3.2.33", "math_tan", "Math.tan ( x )", "tan")
}
pub fn build_asin() -> IRFunction {
    build_unary("21.3.2.3", "math_asin", "Math.asin ( x )", "asin")
}
pub fn build_acos() -> IRFunction {
    build_unary("21.3.2.2", "math_acos", "Math.acos ( x )", "acos")
}
pub fn build_atan() -> IRFunction {
    build_unary("21.3.2.6", "math_atan", "Math.atan ( x )", "atan")
}
// Hyperbolic family.
pub fn build_sinh() -> IRFunction {
    build_unary("21.3.2.31", "math_sinh", "Math.sinh ( x )", "sinh")
}
pub fn build_cosh() -> IRFunction {
    build_unary("21.3.2.13", "math_cosh", "Math.cosh ( x )", "cosh")
}
pub fn build_tanh() -> IRFunction {
    build_unary("21.3.2.34", "math_tanh", "Math.tanh ( x )", "tanh")
}
pub fn build_asinh() -> IRFunction {
    build_unary("21.3.2.5", "math_asinh", "Math.asinh ( x )", "asinh")
}
pub fn build_acosh() -> IRFunction {
    build_unary("21.3.2.4", "math_acosh", "Math.acosh ( x )", "acosh")
}
pub fn build_atanh() -> IRFunction {
    build_unary("21.3.2.7", "math_atanh", "Math.atanh ( x )", "atanh")
}

fn one_step_spec(prose: &'static str) -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["math_unary_op_via"],
        throws: None,
        prose,
    }]
}

pub fn spec_steps_abs() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return abs(n).")
}
pub fn spec_steps_floor() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return floor(n).")
}
pub fn spec_steps_ceil() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return ceil(n).")
}
pub fn spec_steps_round() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return round-half-toward-+Infinity(n).")
}
pub fn spec_steps_trunc() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return trunc(n).")
}
pub fn spec_steps_sqrt() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return sqrt(n).")
}
pub fn spec_steps_cbrt() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return cbrt(n).")
}
pub fn spec_steps_sign() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return sign(n), preserving +0/-0/NaN.")
}

pub fn spec_steps_exp() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return exp(n).")
}
pub fn spec_steps_expm1() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return exp(n) − 1.")
}
pub fn spec_steps_log() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return ln(n).")
}
pub fn spec_steps_log1p() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return ln(1 + n).")
}
pub fn spec_steps_log2() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return log2(n).")
}
pub fn spec_steps_log10() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return log10(n).")
}

pub fn spec_steps_sin() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return sin(n).")
}
pub fn spec_steps_cos() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return cos(n).")
}
pub fn spec_steps_tan() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return tan(n).")
}
pub fn spec_steps_asin() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return asin(n).")
}
pub fn spec_steps_acos() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return acos(n).")
}
pub fn spec_steps_atan() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return atan(n).")
}

pub fn spec_steps_sinh() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return sinh(n).")
}
pub fn spec_steps_cosh() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return cosh(n).")
}
pub fn spec_steps_tanh() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return tanh(n).")
}
pub fn spec_steps_asinh() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return asinh(n).")
}
pub fn spec_steps_acosh() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return acosh(n).")
}
pub fn spec_steps_atanh() -> Vec<SpecStepRecord> {
    one_step_spec("Let n be ? ToNumber(x). Return atanh(n).")
}
