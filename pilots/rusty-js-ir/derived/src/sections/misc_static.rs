//! Stragglers: Math.{imul, fround, clz32} + Array.{isArray, of}.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn variadic_section(spec: &str, rust_name: &str, title: &str, via: &'static str) -> IRFunction {
    IRFunction {
        spec_section: spec.into(), rust_name: rust_name.into(), title: title.into(),
        body: vec![Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: via, args: vec![Expr::AllArgs],
        })}],
    }
}

fn nullary_section(spec: &str, rust_name: &str, title: &str, via: &'static str) -> IRFunction {
    IRFunction {
        spec_section: spec.into(), rust_name: rust_name.into(), title: title.into(),
        body: vec![Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: via, args: vec![],
        })}],
    }
}

pub fn build_number_proto_to_string()        -> IRFunction { variadic_section("21.1.3.6", "number_prototype_to_string",        "Number.prototype.toString ( [ radix ] )", "number_proto_to_string_via") }
pub fn build_number_proto_to_locale_string() -> IRFunction { nullary_section ("21.1.3.4", "number_prototype_to_locale_string", "Number.prototype.toLocaleString ( )",     "number_proto_to_locale_string_via") }
pub fn build_string_from_char_code()         -> IRFunction { variadic_section("22.1.2.1", "string_from_char_code",             "String.fromCharCode ( ...codeUnits )",    "string_from_char_code_via") }
pub fn build_string_from_code_point()        -> IRFunction { variadic_section("22.1.2.2", "string_from_code_point",            "String.fromCodePoint ( ...codePoints )",  "string_from_code_point_via") }

pub fn spec_steps_number_proto_to_string()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["number_proto_to_string_via"],        throws: None, prose: "Brand-check Number; coerce radix (default 10, range 2..=36, else RangeError); format n in radix." }] }
pub fn spec_steps_number_proto_to_locale_string() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["number_proto_to_locale_string_via"], throws: None, prose: "Brand-check Number; return locale-formatted string (v1: same as toString)." }] }
pub fn spec_steps_string_from_char_code()         -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_from_char_code_via"],         throws: None, prose: "Coerce each arg to uint16; concatenate as UTF-16 code units." }] }
pub fn spec_steps_string_from_code_point()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_from_code_point_via"],        throws: None, prose: "Validate each arg as a Unicode code point [0, 0x10FFFF]; throw RangeError otherwise; concatenate." }] }

pub fn build_math_imul()   -> IRFunction { variadic_section("21.3.2.19", "math_imul",   "Math.imul ( x, y )",   "math_imul_via") }
pub fn build_math_fround() -> IRFunction { variadic_section("21.3.2.16", "math_fround", "Math.fround ( x )",    "math_fround_via") }
pub fn build_math_clz32()  -> IRFunction { variadic_section("21.3.2.11", "math_clz32",  "Math.clz32 ( x )",     "math_clz32_via") }
pub fn build_is_array()    -> IRFunction { variadic_section("23.1.2.2",  "array_is_array", "Array.isArray ( arg )", "array_is_array_via") }
pub fn build_array_of()    -> IRFunction { variadic_section("23.1.2.3",  "array_of",       "Array.of ( ...items )", "array_of_via") }

pub fn spec_steps_math_imul()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["math_imul_via"],   throws: None, prose: "Coerce both args to int32 and return their 32-bit wrapping product." }] }
pub fn spec_steps_math_fround() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["math_fround_via"], throws: None, prose: "Coerce arg to single-precision float and return as f64." }] }
pub fn spec_steps_math_clz32()  -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["math_clz32_via"],  throws: None, prose: "Coerce arg to uint32 and return its leading-zero count." }] }
pub fn spec_steps_is_array()    -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_is_array_via"], throws: None, prose: "Return whether arg is an exotic Array object." }] }
pub fn spec_steps_array_of()    -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_of_via"],       throws: None, prose: "Return a new array containing the given items as elements." }] }
