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

// ──────────────── §22.1.3.{16, 17, 21} padEnd / padStart / repeat ────────────────

pub fn build_repeat() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.count".into(),
            node: IRNode::Let { name: "count".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_repeat_via",
                args: vec![Expr::This, v("count")],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.21".into(),
        rust_name: "string_prototype_repeat".into(),
        title: "String.prototype.repeat ( count )".into(), body }
}

pub fn build_pad_start() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "param.pad".into(),
            node: IRNode::Let { name: "pad".into(), value: Expr::Arg(1) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_pad_start_via",
                args: vec![Expr::This, v("target"), v("pad")],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.17".into(),
        rust_name: "string_prototype_pad_start".into(),
        title: "String.prototype.padStart ( maxLength [ , fillString ] )".into(), body }
}

pub fn build_pad_end() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "param.pad".into(),
            node: IRNode::Let { name: "pad".into(), value: Expr::Arg(1) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "string_proto_pad_end_via",
                args: vec![Expr::This, v("target"), v("pad")],
            }) },
    ];
    IRFunction { spec_section: "22.1.3.16".into(),
        rust_name: "string_prototype_pad_end".into(),
        title: "String.prototype.padEnd ( maxLength [ , fillString ] )".into(), body }
}

fn two_arg_section(spec: &str, rust_name: &str, title: &str, via: &'static str, p0: &str, p1: &str) -> IRFunction {
    let body = vec![
        Step { spec_step: format!("param.{}", p0), node: IRNode::Let { name: p0.into(), value: Expr::Arg(0) }},
        Step { spec_step: format!("param.{}", p1), node: IRNode::Let { name: p1.into(), value: Expr::Arg(1) }},
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: via, args: vec![Expr::This, v(p0), v(p1)],
        })},
    ];
    IRFunction { spec_section: spec.into(), rust_name: rust_name.into(), title: title.into(), body }
}

fn one_arg_section(spec: &str, rust_name: &str, title: &str, via: &'static str, p0: &str) -> IRFunction {
    let body = vec![
        Step { spec_step: format!("param.{}", p0), node: IRNode::Let { name: p0.into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: via, args: vec![Expr::This, v(p0)],
        })},
    ];
    IRFunction { spec_section: spec.into(), rust_name: rust_name.into(), title: title.into(), body }
}

fn zero_arg_section(spec: &str, rust_name: &str, title: &str, via: &'static str) -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: via, args: vec![Expr::This],
        })},
    ];
    IRFunction { spec_section: spec.into(), rust_name: rust_name.into(), title: title.into(), body }
}

pub fn build_code_point_at()  -> IRFunction { one_arg_section("22.1.3.4",  "string_prototype_code_point_at",  "String.prototype.codePointAt ( pos )",       "string_proto_code_point_at_via",  "pos") }
pub fn build_at()             -> IRFunction { one_arg_section("22.1.3.2",  "string_prototype_at",             "String.prototype.at ( index )",              "string_proto_at_via",             "index") }
pub fn build_normalize()      -> IRFunction { zero_arg_section("22.1.3.13", "string_prototype_normalize",     "String.prototype.normalize ( [ form ] )",    "string_proto_normalize_via") }
pub fn build_locale_compare() -> IRFunction { one_arg_section("22.1.3.10", "string_prototype_locale_compare", "String.prototype.localeCompare ( that )",    "string_proto_locale_compare_via", "that") }

pub fn spec_steps_code_point_at()  -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_code_point_at_via"],  throws: None, prose: "Return the code point at the given UTF-16 index, or undefined if out of range." }] }
pub fn spec_steps_at()             -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_at_via"],             throws: None, prose: "Return the character at the relative index (negative counts from end), or undefined." }] }
pub fn spec_steps_normalize()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_normalize_via"],      throws: None, prose: "Return ? ToString(? RequireObjectCoercible(this))." }] }
pub fn spec_steps_locale_compare() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_locale_compare_via"], throws: None, prose: "Return -1/0/1 comparing this and that as strings." }] }

pub fn build_slice()         -> IRFunction { two_arg_section("22.1.3.22", "string_prototype_slice",         "String.prototype.slice ( start, end )",                     "string_proto_slice_via",         "start", "end") }
pub fn build_substring()     -> IRFunction { two_arg_section("22.1.3.24", "string_prototype_substring",     "String.prototype.substring ( start, end )",                  "string_proto_substring_via",     "start", "end") }
pub fn build_substr()        -> IRFunction { two_arg_section("B.2.2.2",   "string_prototype_substr",        "String.prototype.substr ( start, length )",                  "string_proto_substr_via",        "start", "length") }
pub fn build_index_of()      -> IRFunction { two_arg_section("22.1.3.8",  "string_prototype_index_of",      "String.prototype.indexOf ( searchString, position )",        "string_proto_index_of_via",      "search", "position") }
pub fn build_last_index_of() -> IRFunction { two_arg_section("22.1.3.10", "string_prototype_last_index_of", "String.prototype.lastIndexOf ( searchString, position )",    "string_proto_last_index_of_via", "search", "position") }
pub fn build_includes()      -> IRFunction { two_arg_section("22.1.3.7",  "string_prototype_includes",      "String.prototype.includes ( searchString, position )",       "string_proto_includes_via",      "search", "position") }
pub fn build_starts_with()   -> IRFunction { two_arg_section("22.1.3.23", "string_prototype_starts_with",   "String.prototype.startsWith ( searchString, position )",     "string_proto_starts_with_via",   "search", "position") }
pub fn build_ends_with()     -> IRFunction { two_arg_section("22.1.3.6",  "string_prototype_ends_with",     "String.prototype.endsWith ( searchString, endPosition )",    "string_proto_ends_with_via",     "search", "position") }

pub fn spec_steps_slice()         -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_slice_via"],         throws: None, prose: "Return the substring of S between clamped start and end indices." }] }
pub fn spec_steps_substring()     -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_substring_via"],     throws: None, prose: "Return the substring of S between clamped, ordered start and end indices." }] }
pub fn spec_steps_substr()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_substr_via"],        throws: None, prose: "Return a length-bounded substring of S starting at clamped start." }] }
pub fn spec_steps_index_of()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_index_of_via"],      throws: None, prose: "Return the index of the first occurrence of searchString in S, or -1." }] }
pub fn spec_steps_last_index_of() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_last_index_of_via"], throws: None, prose: "Return the index of the last occurrence of searchString in S, or -1." }] }
pub fn spec_steps_includes()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_includes_via"],      throws: None, prose: "Throw if searchString is a RegExp; otherwise return whether S contains it." }] }
pub fn spec_steps_starts_with()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_starts_with_via"],   throws: None, prose: "Throw if searchString is a RegExp; otherwise return whether S starts with it." }] }
pub fn spec_steps_ends_with()     -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_ends_with_via"],     throws: None, prose: "Throw if searchString is a RegExp; otherwise return whether S ends with it." }] }

pub fn spec_steps_repeat() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_repeat_via"], throws: None,
        prose: "Let O be ? RequireObjectCoercible(this). Let S be ? ToString(O). Let n be ? ToIntegerOrInfinity(count). If n < 0 or n is +∞, throw RangeError. Return S repeated n times." }]
}
pub fn spec_steps_pad_start() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_pad_start_via"], throws: None,
        prose: "Return the spec-prescribed StringPad(O, maxLength, fillString, start)." }]
}
pub fn spec_steps_pad_end() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_proto_pad_end_via"], throws: None,
        prose: "Return the spec-prescribed StringPad(O, maxLength, fillString, end)." }]
}
