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

pub fn build_date_to_string()        -> IRFunction { nullary_section("21.4.4.41a", "date_prototype_to_string",        "Date.prototype.toString ( )",        "date_proto_to_string_via") }
pub fn build_date_to_json()          -> IRFunction { nullary_section("21.4.4.37",  "date_prototype_to_json",          "Date.prototype.toJSON ( )",          "date_proto_to_json_via") }
pub fn build_date_get_full_year()    -> IRFunction { nullary_section("21.4.4.4",   "date_prototype_get_full_year",    "Date.prototype.getFullYear ( )",     "date_proto_get_full_year_via") }
pub fn build_date_get_month()        -> IRFunction { nullary_section("21.4.4.8",   "date_prototype_get_month",        "Date.prototype.getMonth ( )",        "date_proto_get_month_via") }
pub fn build_date_get_date()         -> IRFunction { nullary_section("21.4.4.2",   "date_prototype_get_date",         "Date.prototype.getDate ( )",         "date_proto_get_date_via") }
pub fn build_date_get_day()          -> IRFunction { nullary_section("21.4.4.3",   "date_prototype_get_day",          "Date.prototype.getDay ( )",          "date_proto_get_day_via") }
pub fn build_date_get_hours()        -> IRFunction { nullary_section("21.4.4.5",   "date_prototype_get_hours",        "Date.prototype.getHours ( )",        "date_proto_get_hours_via") }
pub fn build_date_get_minutes()      -> IRFunction { nullary_section("21.4.4.7",   "date_prototype_get_minutes",      "Date.prototype.getMinutes ( )",      "date_proto_get_minutes_via") }
pub fn build_date_get_seconds()      -> IRFunction { nullary_section("21.4.4.9",   "date_prototype_get_seconds",      "Date.prototype.getSeconds ( )",      "date_proto_get_seconds_via") }
pub fn build_date_get_milliseconds() -> IRFunction { nullary_section("21.4.4.6",   "date_prototype_get_milliseconds", "Date.prototype.getMilliseconds ( )", "date_proto_get_milliseconds_via") }

pub fn spec_steps_date_to_string()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_to_string_via"],        throws: None, prose: "Format [[DateValue]] as YYYY-MM-DDT00:00:00Z (v1 deviation; spec uses RFC 7231 local-time form)." }] }
pub fn spec_steps_date_to_json()          -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_to_json_via"],          throws: None, prose: "Format [[DateValue]] as midnight ISO YYYY-MM-DDT00:00:00.000Z (v1)." }] }
pub fn spec_steps_date_get_full_year()    -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_full_year_via"],    throws: None, prose: "Return the year component of [[DateValue]]." }] }
pub fn spec_steps_date_get_month()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_month_via"],        throws: None, prose: "Return the month component (0-based) of [[DateValue]]." }] }
pub fn spec_steps_date_get_date()         -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_date_via"],         throws: None, prose: "Return the day-of-month (1-based) of [[DateValue]]." }] }
pub fn spec_steps_date_get_day()          -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_day_via"],          throws: None, prose: "Return the day-of-week (0=Sunday) of [[DateValue]]." }] }
pub fn spec_steps_date_get_hours()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_hours_via"],        throws: None, prose: "Return the hours component of [[DateValue]]." }] }
pub fn spec_steps_date_get_minutes()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_minutes_via"],      throws: None, prose: "Return the minutes component of [[DateValue]]." }] }
pub fn spec_steps_date_get_seconds()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_seconds_via"],      throws: None, prose: "Return the seconds component of [[DateValue]]." }] }
pub fn spec_steps_date_get_milliseconds() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_milliseconds_via"], throws: None, prose: "Return the milliseconds component of [[DateValue]]." }] }

pub fn build_date_get_time()      -> IRFunction { nullary_section("21.4.4.10", "date_prototype_get_time",       "Date.prototype.getTime ( )",       "date_proto_get_time_via") }
pub fn build_date_value_of()      -> IRFunction { nullary_section("21.4.4.44", "date_prototype_value_of",       "Date.prototype.valueOf ( )",       "date_proto_value_of_via") }
pub fn build_date_to_iso_string() -> IRFunction { nullary_section("21.4.4.36", "date_prototype_to_iso_string",  "Date.prototype.toISOString ( )",   "date_proto_to_iso_string_via") }
pub fn build_date_to_date_string()-> IRFunction { nullary_section("21.4.4.35", "date_prototype_to_date_string", "Date.prototype.toDateString ( )",  "date_proto_to_date_string_via") }
pub fn build_date_to_time_string()-> IRFunction { nullary_section("21.4.4.41", "date_prototype_to_time_string", "Date.prototype.toTimeString ( )",  "date_proto_to_time_string_via") }
pub fn build_date_to_utc_string() -> IRFunction { nullary_section("21.4.4.42", "date_prototype_to_utc_string",  "Date.prototype.toUTCString ( )",   "date_proto_to_utc_string_via") }

pub fn spec_steps_date_get_time()       -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_time_via"],       throws: None, prose: "Return the [[DateValue]] internal slot (stored as __date_ms)." }] }
pub fn spec_steps_date_value_of()       -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_value_of_via"],       throws: None, prose: "Return the [[DateValue]] internal slot." }] }
pub fn spec_steps_date_to_iso_string()  -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_to_iso_string_via"],  throws: None, prose: "Format [[DateValue]] as YYYY-MM-DDTHH:MM:SS.sssZ." }] }
pub fn spec_steps_date_to_date_string() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_to_date_string_via"], throws: None, prose: "Format [[DateValue]] as YYYY-MM-DD (v1 locale-insensitive)." }] }
pub fn spec_steps_date_to_time_string() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_to_time_string_via"], throws: None, prose: "Format [[DateValue]] as HH:MM:SS (v1 locale-insensitive)." }] }
pub fn spec_steps_date_to_utc_string()  -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_to_utc_string_via"],  throws: None, prose: "Format [[DateValue]] as YYYY-MM-DD HH:MM:SS GMT (v1 locale-insensitive)." }] }

pub fn build_symbol_proto_to_string()   -> IRFunction { nullary_section ("20.4.3.3", "symbol_prototype_to_string",   "Symbol.prototype.toString ( )",   "symbol_proto_to_string_via") }
pub fn build_bigint_proto_to_string()   -> IRFunction { variadic_section("21.2.3.4", "bigint_prototype_to_string",   "BigInt.prototype.toString ( [ radix ] )", "bigint_proto_to_string_via") }
pub fn build_function_proto_to_string() -> IRFunction { nullary_section ("20.2.3.5", "function_prototype_to_string", "Function.prototype.toString ( )", "function_proto_to_string_via") }

pub fn spec_steps_symbol_proto_to_string()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["symbol_proto_to_string_via"],   throws: None, prose: "Return \"Symbol(<description>)\" for a Symbol receiver; fall through to ToString for non-Symbol (per v1 deviation)." }] }
pub fn spec_steps_bigint_proto_to_string()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["bigint_proto_to_string_via"],   throws: None, prose: "Brand-check BigInt; radix default 10, range 2..=36; format in radix." }] }
pub fn spec_steps_function_proto_to_string() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["function_proto_to_string_via"], throws: None, prose: "Brand-check Function/Closure/BoundFunction; return native-shape \"function NAME() { [native code] }\"." }] }

pub fn build_error_proto_to_string() -> IRFunction { nullary_section("20.5.3.4", "error_prototype_to_string", "Error.prototype.toString ( )", "error_proto_to_string_via") }
pub fn spec_steps_error_proto_to_string() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["error_proto_to_string_via"], throws: None, prose: "Return name (default \"Error\") and message (default \"\") concatenated as \"name: message\" when message is non-empty, else just name." }] }

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
