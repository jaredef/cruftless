//! ECMA-262 §22.1.3 — String.prototype.* methods.
//!
//! Each section is a 1-step CallBuiltin to a runtime helper that does
//! the §22.1.3 preamble (RequireObjectCoercible + ToStringStrict) +
//! arg coercion + the spec operation. cruftless's existing impls are
//! already P62.E13/E14/E15 spec-compliant; the IR adds spec-step
//! traceability + the §A8.30 brand-check discipline made explicit.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

pub fn build_char_at() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.pos".into(),
            node: IRNode::Let { name: "pos".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_char_at_via",
                args: vec![Expr::This, v("pos")],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.1".into(),
        rust_name: "string_prototype_char_at".into(),
        title: "String.prototype.charAt ( pos )".into(), body }
}

pub fn build_char_code_at() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.pos".into(),
            node: IRNode::Let { name: "pos".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_char_code_at_via",
                args: vec![Expr::This, v("pos")],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.2".into(),
        rust_name: "string_prototype_char_code_at".into(),
        title: "String.prototype.charCodeAt ( pos )".into(), body }
}

pub fn build_concat() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_concat_via",
                args: vec![Expr::This, Expr::AllArgs],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.3".into(),
        rust_name: "string_prototype_concat".into(),
        title: "String.prototype.concat ( ...args )".into(), body }
}

pub fn spec_steps_char_at() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_char_at_via"], throws: None,
        prose: "Let O be ? RequireObjectCoercible(this). Let S be ? ToString(O). Let position be ? ToIntegerOrInfinity(pos). Return single-character String at position, or empty String if out of range." }]
}
pub fn spec_steps_char_code_at() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_char_code_at_via"], throws: None,
        prose: "Same preamble as charAt. Return the Number value of the code unit at position, or NaN if out of range." }]
}
pub fn spec_steps_concat() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_concat_via"], throws: None,
        prose: "Let O be ? RequireObjectCoercible(this). Let S be ? ToString(O). For each arg, append ? ToString(arg). Return the joined String." }]
}

// ──────────────── §22.1.3.28 / §22.1.3.30 toLowerCase / toUpperCase ────────────────

pub fn build_to_lower_case() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_to_lower_case_via",
                args: vec![Expr::This],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.28".into(),
        rust_name: "string_prototype_to_lower_case".into(),
        title: "String.prototype.toLowerCase ( )".into(), body }
}

pub fn build_to_upper_case() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_to_upper_case_via",
                args: vec![Expr::This],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.30".into(),
        rust_name: "string_prototype_to_upper_case".into(),
        title: "String.prototype.toUpperCase ( )".into(), body }
}

// toLocale{Lower,Upper}Case share the same Tier-1.10 simplification
// (locale-insensitive; matches cruftless's pre-IR behavior).

pub fn build_to_locale_lower_case() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_to_lower_case_via",
                args: vec![Expr::This],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.26".into(),
        rust_name: "string_prototype_to_locale_lower_case".into(),
        title: "String.prototype.toLocaleLowerCase ( [ reserved1 [ , reserved2 ] ] )".into(), body }
}

pub fn build_to_locale_upper_case() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_to_upper_case_via",
                args: vec![Expr::This],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.27".into(),
        rust_name: "string_prototype_to_locale_upper_case".into(),
        title: "String.prototype.toLocaleUpperCase ( [ reserved1 [ , reserved2 ] ] )".into(), body }
}

pub fn spec_steps_to_lower_case() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_to_lower_case_via"], throws: None,
        prose: "Let O be ? RequireObjectCoercible(this). Let S be ? ToString(O). Return the Unicode-lowercased value of S." }]
}
pub fn spec_steps_to_upper_case() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_to_upper_case_via"], throws: None,
        prose: "Let O be ? RequireObjectCoercible(this). Let S be ? ToString(O). Return the Unicode-uppercased value of S." }]
}
pub fn spec_steps_to_locale_lower_case() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_to_lower_case_via"], throws: None,
        prose: "Tier-1.10 simplification: defers to toLowerCase (locale-insensitive)." }]
}
pub fn spec_steps_to_locale_upper_case() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_to_upper_case_via"], throws: None,
        prose: "Tier-1.10 simplification: defers to toUpperCase (locale-insensitive)." }]
}

// ──────────────── §22.1.3.{32, 33, 34} trim / trimStart / trimEnd ────────────────

fn trim_section(spec: &str, rust_name: &str, title: &str, builtin: &'static str) -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: builtin,
                args: vec![Expr::This],
            }) },
    ];
    IRFunction { spec_section: spec.into(),
        rust_name: rust_name.into(),
        title: title.into(), body }
}

pub fn build_trim() -> IRFunction       { trim_section("22.1.3.32", "string_prototype_trim",       "String.prototype.trim ( )",       "string_proto_trim_via") }
pub fn build_trim_start() -> IRFunction { trim_section("22.1.3.34", "string_prototype_trim_start", "String.prototype.trimStart ( )", "string_proto_trim_start_via") }
pub fn build_trim_end() -> IRFunction   { trim_section("22.1.3.33", "string_prototype_trim_end",   "String.prototype.trimEnd ( )",   "string_proto_trim_end_via") }
// trimLeft / trimRight are Annex B legacy aliases.
pub fn build_trim_left() -> IRFunction  { trim_section("B.2.2.1",   "string_prototype_trim_left",  "String.prototype.trimLeft ( )",  "string_proto_trim_start_via") }
pub fn build_trim_right() -> IRFunction { trim_section("B.2.2.2",   "string_prototype_trim_right", "String.prototype.trimRight ( )", "string_proto_trim_end_via") }

pub fn spec_steps_trim() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_trim_via"], throws: None,
        prose: "Let O be ? RequireObjectCoercible(this). Let S be ? ToString(O). Return S with leading + trailing WhiteSpace removed." }]
}
pub fn spec_steps_trim_start() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_trim_start_via"], throws: None,
        prose: "Like trim but only leading WhiteSpace removed." }]
}
pub fn spec_steps_trim_end() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_trim_end_via"], throws: None,
        prose: "Like trim but only trailing WhiteSpace removed." }]
}
pub fn spec_steps_trim_left() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_trim_start_via"], throws: None,
        prose: "Annex B alias for trimStart." }]
}
pub fn spec_steps_trim_right() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_trim_end_via"], throws: None,
        prose: "Annex B alias for trimEnd." }]
}
