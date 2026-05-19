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

fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

fn build_unary(spec: &str, rust_name: &str, title: &str, op: &'static str) -> IRFunction {
    let body = vec![
        Step { spec_step: "param.x".into(),
            node: IRNode::Let { name: "x".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "math_unary_op_via",
                args: vec![Expr::Str(op.into()), v("x")],
            })},
    ];
    IRFunction {
        spec_section: spec.into(),
        rust_name: rust_name.into(),
        title: title.into(),
        body,
    }
}

pub fn build_abs() -> IRFunction   { build_unary("21.3.2.1",  "math_abs",   "Math.abs ( x )",   "abs") }
pub fn build_floor() -> IRFunction { build_unary("21.3.2.16", "math_floor", "Math.floor ( x )", "floor") }
pub fn build_ceil() -> IRFunction  { build_unary("21.3.2.10", "math_ceil",  "Math.ceil ( x )",  "ceil") }
pub fn build_round() -> IRFunction { build_unary("21.3.2.28", "math_round", "Math.round ( x )", "round") }
pub fn build_trunc() -> IRFunction { build_unary("21.3.2.35", "math_trunc", "Math.trunc ( x )", "trunc") }
pub fn build_sqrt() -> IRFunction  { build_unary("21.3.2.32", "math_sqrt",  "Math.sqrt ( x )",  "sqrt") }
pub fn build_cbrt() -> IRFunction  { build_unary("21.3.2.9",  "math_cbrt",  "Math.cbrt ( x )",  "cbrt") }
pub fn build_sign() -> IRFunction  { build_unary("21.3.2.30", "math_sign",  "Math.sign ( x )",  "sign") }

fn one_step_spec(prose: &'static str) -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["math_unary_op_via"],
        throws: None,
        prose,
    }]
}

pub fn spec_steps_abs() -> Vec<SpecStepRecord>   { one_step_spec("Let n be ? ToNumber(x). Return abs(n).") }
pub fn spec_steps_floor() -> Vec<SpecStepRecord> { one_step_spec("Let n be ? ToNumber(x). Return floor(n).") }
pub fn spec_steps_ceil() -> Vec<SpecStepRecord>  { one_step_spec("Let n be ? ToNumber(x). Return ceil(n).") }
pub fn spec_steps_round() -> Vec<SpecStepRecord> { one_step_spec("Let n be ? ToNumber(x). Return round-half-toward-+Infinity(n).") }
pub fn spec_steps_trunc() -> Vec<SpecStepRecord> { one_step_spec("Let n be ? ToNumber(x). Return trunc(n).") }
pub fn spec_steps_sqrt() -> Vec<SpecStepRecord>  { one_step_spec("Let n be ? ToNumber(x). Return sqrt(n).") }
pub fn spec_steps_cbrt() -> Vec<SpecStepRecord>  { one_step_spec("Let n be ? ToNumber(x). Return cbrt(n).") }
pub fn spec_steps_sign() -> Vec<SpecStepRecord>  { one_step_spec("Let n be ? ToNumber(x). Return sign(n), preserving +0/-0/NaN.") }
