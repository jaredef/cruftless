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

pub fn build_set_add()      -> IRFunction { variadic_section("24.2.3.1", "set_prototype_add",      "Set.prototype.add ( value )",    "set_proto_add_via") }
pub fn build_set_has()      -> IRFunction { variadic_section("24.2.3.6", "set_prototype_has",      "Set.prototype.has ( value )",    "set_proto_has_via") }
pub fn build_set_delete()   -> IRFunction { variadic_section("24.2.3.5", "set_prototype_delete",   "Set.prototype.delete ( value )", "set_proto_delete_via") }
pub fn build_set_clear()    -> IRFunction { nullary_section ("24.2.3.4", "set_prototype_clear",    "Set.prototype.clear ( )",        "set_proto_clear_via") }
pub fn build_set_for_each() -> IRFunction { variadic_section("24.2.3.7", "set_prototype_for_each", "Set.prototype.forEach ( cb )",   "set_proto_for_each_via") }

pub fn spec_steps_set_add()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["set_proto_add_via"],      throws: None, prose: "Brand-check Set; add value (no duplicates); bump size; return this." }] }
pub fn spec_steps_set_has()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["set_proto_has_via"],      throws: None, prose: "Brand-check Set; return whether the value is present." }] }
pub fn spec_steps_set_delete()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["set_proto_delete_via"],   throws: None, prose: "Brand-check Set; remove; decrement size; return whether removal occurred." }] }
pub fn spec_steps_set_clear()    -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["set_proto_clear_via"],    throws: None, prose: "Brand-check Set; reset storage and size; return undefined." }] }
pub fn spec_steps_set_for_each() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["set_proto_for_each_via"], throws: None, prose: "Brand-check Set; iterate (value, value, set) over entries calling cb." }] }

pub fn build_map_get()      -> IRFunction { variadic_section("24.1.3.5",  "map_prototype_get",      "Map.prototype.get ( key )",            "map_proto_get_via") }
pub fn build_map_set()      -> IRFunction { variadic_section("24.1.3.9",  "map_prototype_set",      "Map.prototype.set ( key, value )",     "map_proto_set_via") }
pub fn build_map_has()      -> IRFunction { variadic_section("24.1.3.6",  "map_prototype_has",      "Map.prototype.has ( key )",            "map_proto_has_via") }
pub fn build_map_delete()   -> IRFunction { variadic_section("24.1.3.3",  "map_prototype_delete",   "Map.prototype.delete ( key )",         "map_proto_delete_via") }
pub fn build_map_clear()    -> IRFunction { nullary_section ("24.1.3.2",  "map_prototype_clear",    "Map.prototype.clear ( )",              "map_proto_clear_via") }
pub fn build_map_for_each() -> IRFunction { variadic_section("24.1.3.4",  "map_prototype_for_each", "Map.prototype.forEach ( cb )",         "map_proto_for_each_via") }
pub fn build_map_values()   -> IRFunction { nullary_section ("24.1.3.10", "map_prototype_values",   "Map.prototype.values ( )",             "map_proto_values_via") }
pub fn build_map_keys()     -> IRFunction { nullary_section ("24.1.3.7",  "map_prototype_keys",     "Map.prototype.keys ( )",               "map_proto_keys_via") }
pub fn build_map_entries()  -> IRFunction { nullary_section ("24.1.3.4a", "map_prototype_entries",  "Map.prototype.entries ( )",            "map_proto_entries_via") }

pub fn spec_steps_map_get()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_get_via"],      throws: None, prose: "Brand-check Map; return the value for the stringified key (undefined if absent)." }] }
pub fn spec_steps_map_set()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_set_via"],      throws: None, prose: "Brand-check Map; insert/update; bump size on new key; return this." }] }
pub fn spec_steps_map_has()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_has_via"],      throws: None, prose: "Brand-check Map; return whether the key is present." }] }
pub fn spec_steps_map_delete()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_delete_via"],   throws: None, prose: "Brand-check Map; remove the key; decrement size; return whether removal occurred." }] }
pub fn spec_steps_map_clear()    -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_clear_via"],    throws: None, prose: "Brand-check Map; reset storage and size; return undefined." }] }
pub fn spec_steps_map_for_each() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_for_each_via"], throws: None, prose: "Brand-check Map; iterate (value, key, map) over entries calling cb." }] }
pub fn spec_steps_map_values()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_values_via"],   throws: None, prose: "Brand-check Map; return eager array of values (v1 deviation)." }] }
pub fn spec_steps_map_keys()     -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_keys_via"],     throws: None, prose: "Brand-check Map; return eager array of keys." }] }
pub fn spec_steps_map_entries()  -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["map_proto_entries_via"],  throws: None, prose: "Brand-check Map; return eager array of [k, v] pairs." }] }

pub fn build_object_group_by() -> IRFunction { variadic_section("20.1.2.10", "object_group_by", "Object.groupBy ( items, callbackFn )", "object_group_by_via") }
pub fn spec_steps_object_group_by() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_group_by_via"], throws: None, prose: "Iterate items; bucket each by ToString(callbackFn(item, i)) into an Object-of-arrays." }] }

pub fn build_json_stringify()      -> IRFunction { variadic_section("24.5.2", "json_stringify",                "JSON.stringify ( value, replacer, space )", "json_stringify_via") }
pub fn build_json_parse()          -> IRFunction { variadic_section("24.5.1", "json_parse",                    "JSON.parse ( text, reviver )",              "json_parse_via") }
pub fn build_symbol_for()          -> IRFunction { variadic_section("20.4.2.6", "symbol_for",                  "Symbol.for ( key )",                         "symbol_for_via") }
pub fn build_symbol_key_for()      -> IRFunction { variadic_section("20.4.2.7", "symbol_key_for",              "Symbol.keyFor ( sym )",                      "symbol_key_for_via") }
pub fn build_date_get_year()       -> IRFunction { nullary_section ("B.2.4.1",  "date_prototype_get_year",     "Date.prototype.getYear ( )",                 "date_proto_get_year_via") }
pub fn build_date_set_year()       -> IRFunction { variadic_section("B.2.4.2",  "date_prototype_set_year",     "Date.prototype.setYear ( y )",               "date_proto_set_year_via") }

pub fn spec_steps_json_stringify() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["json_stringify_via"], throws: None, prose: "Serialize value to JSON text (v1: replacer + space args ignored)." }] }
pub fn spec_steps_json_parse()     -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["json_parse_via"],     throws: None, prose: "Parse JSON text to a Value tree; SyntaxError on malformed input (v1: reviver ignored)." }] }
pub fn spec_steps_symbol_for()     -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["symbol_for_via"],     throws: None, prose: "Return a registry-interned Symbol value with the given key (v1: prefix-encoded as @@sym:KEY)." }] }
pub fn spec_steps_symbol_key_for() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["symbol_key_for_via"], throws: None, prose: "Return the registry key for a Symbol, or undefined for non-registry / non-Symbol values." }] }
pub fn spec_steps_date_get_year()  -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_year_via"], throws: None, prose: "Return year - 1900 (Annex B legacy)." }] }
pub fn spec_steps_date_set_year()  -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_year_via"], throws: None, prose: "Set year (0-99 maps to 1900+y; full years pass through)." }] }

pub fn build_parse_int()   -> IRFunction { variadic_section("19.2.5", "parse_int",   "parseInt ( string, radix )",   "parse_int_via") }
pub fn build_parse_float() -> IRFunction { variadic_section("19.2.4", "parse_float", "parseFloat ( string )",        "parse_float_via") }

pub fn spec_steps_parse_int()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["parse_int_via"],   throws: None, prose: "Coerce string; honor optional sign; parse digits in radix (default 10); return NaN on no-digits." }] }
pub fn spec_steps_parse_float() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["parse_float_via"], throws: None, prose: "Coerce string; find longest valid numeric prefix (sign, digits, dot, exponent); parse as f64." }] }

pub fn build_math_random()              -> IRFunction { nullary_section("21.3.2.27", "math_random",                        "Math.random ( )",                       "math_random_via") }
pub fn build_date_get_timezone_offset() -> IRFunction { nullary_section("21.4.4.12", "date_prototype_get_timezone_offset", "Date.prototype.getTimezoneOffset ( )",  "date_proto_get_timezone_offset_via") }

pub fn spec_steps_math_random()              -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["math_random_via"],              throws: None, prose: "Return a pseudo-random number in [0,1) (v1: LCG seeded from system clock; not cryptographic)." }] }
pub fn spec_steps_date_get_timezone_offset() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_get_timezone_offset_via"], throws: None, prose: "Return the local-vs-UTC offset in minutes (v1: 0 since cruftless treats __date_ms as UTC)." }] }

pub fn build_date_now()   -> IRFunction { nullary_section ("21.4.3.1", "date_now",   "Date.now ( )",                   "date_now_via") }
pub fn build_date_parse() -> IRFunction { variadic_section("21.4.3.2", "date_parse", "Date.parse ( string )",          "date_parse_via") }
pub fn build_date_utc()   -> IRFunction { variadic_section("21.4.3.4", "date_utc",   "Date.UTC ( y, mo, d, h, mi, s )","date_utc_via") }

pub fn spec_steps_date_now()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_now_via"],   throws: None, prose: "Return the current epoch milliseconds." }] }
pub fn spec_steps_date_parse() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_parse_via"], throws: None, prose: "Parse a Date string (v1 deviation: returns 0 — full ISO/RFC parser deferred)." }] }
pub fn spec_steps_date_utc()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_utc_via"],   throws: None, prose: "Compute epoch ms from year/month/day/... (v1 stub returns 0)." }] }

pub fn build_string_raw() -> IRFunction { variadic_section("22.1.2.4", "string_raw", "String.raw ( template, ...subs )", "string_raw_via") }
pub fn build_array_from() -> IRFunction { variadic_section("23.1.2.1", "array_from", "Array.from ( arrayLike, mapfn, thisArg )", "array_from_via") }

pub fn spec_steps_string_raw() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["string_raw_via"], throws: None, prose: "Concatenate template.raw segments with stringified subs between them (TypeError if first arg or raw is not an object)." }] }
pub fn spec_steps_array_from() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_from_via"], throws: None, prose: "Convert string or array-like source to a new array, optionally mapping each (i, v) through mapfn with thisArg. v1: iterable protocol deferred." }] }

pub fn build_date_set_time()         -> IRFunction { variadic_section("21.4.4.27", "date_prototype_set_time",         "Date.prototype.setTime ( v )",                  "date_proto_set_time_via") }
pub fn build_date_set_hours()        -> IRFunction { variadic_section("21.4.4.34", "date_prototype_set_hours",        "Date.prototype.setHours ( h, mi, se, mss )",     "date_proto_set_hours_via") }
pub fn build_date_set_minutes()      -> IRFunction { variadic_section("21.4.4.24", "date_prototype_set_minutes",      "Date.prototype.setMinutes ( mi, se, mss )",      "date_proto_set_minutes_via") }
pub fn build_date_set_seconds()      -> IRFunction { variadic_section("21.4.4.26", "date_prototype_set_seconds",      "Date.prototype.setSeconds ( se, mss )",          "date_proto_set_seconds_via") }
pub fn build_date_set_milliseconds() -> IRFunction { variadic_section("21.4.4.23", "date_prototype_set_milliseconds", "Date.prototype.setMilliseconds ( mss )",         "date_proto_set_milliseconds_via") }
pub fn build_date_set_date()         -> IRFunction { variadic_section("21.4.4.20", "date_prototype_set_date",         "Date.prototype.setDate ( d )",                   "date_proto_set_date_via") }
pub fn build_date_set_month()        -> IRFunction { variadic_section("21.4.4.25", "date_prototype_set_month",        "Date.prototype.setMonth ( mo, d )",              "date_proto_set_month_via") }
pub fn build_date_set_full_year()    -> IRFunction { variadic_section("21.4.4.21", "date_prototype_set_full_year",    "Date.prototype.setFullYear ( y, mo, d )",        "date_proto_set_full_year_via") }

pub fn spec_steps_date_set_time()         -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_time_via"],         throws: None, prose: "Replace [[DateValue]] with the coerced argument and return it." }] }
pub fn spec_steps_date_set_hours()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_hours_via"],        throws: None, prose: "Recompute [[DateValue]] from current year/month/day plus the given hour/min/sec/ms (omitted components retained)." }] }
pub fn spec_steps_date_set_minutes()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_minutes_via"],      throws: None, prose: "Recompute [[DateValue]] keeping current hour/day, replacing min/sec/ms." }] }
pub fn spec_steps_date_set_seconds()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_seconds_via"],      throws: None, prose: "Recompute [[DateValue]] keeping current hour/min, replacing sec/ms." }] }
pub fn spec_steps_date_set_milliseconds() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_milliseconds_via"], throws: None, prose: "Replace only the ms component of [[DateValue]]." }] }
pub fn spec_steps_date_set_date()         -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_date_via"],         throws: None, prose: "Recompute [[DateValue]] keeping current year/month/time-of-day, replacing day." }] }
pub fn spec_steps_date_set_month()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_month_via"],        throws: None, prose: "Recompute [[DateValue]] keeping current year/day/time-of-day, replacing month." }] }
pub fn spec_steps_date_set_full_year()    -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["date_proto_set_full_year_via"],    throws: None, prose: "Recompute [[DateValue]] replacing year (and optionally month/day)." }] }

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
