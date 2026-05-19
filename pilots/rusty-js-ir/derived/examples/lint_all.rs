//! Run the spec-vs-IR linter against every translated section.
//!
//! Run with: `cargo run --example lint_all -p rusty-js-ir`

use rusty_js_ir::lint::lint;
use rusty_js_ir::sections::{
    array_prototype_find as find, array_prototype_index_search as index_search,
    array_prototype_iteration as iter, array_prototype_map,
    array_prototype_reduce as reduce, global_predicates, math_binary_variadic,
    math_unary, number_prototype, number_static, object_integrity,
    object_proto_ops, object_static, promise_static, reflect_static,
    string_prototype,
};

fn main() {
    let sections = vec![
        ("Array.prototype.map", array_prototype_map::build(), array_prototype_map::spec_steps()),
        ("Array.prototype.forEach", iter::build_for_each(), iter::spec_steps_for_each()),
        ("Array.prototype.filter", iter::build_filter(), iter::spec_steps_filter()),
        ("Array.prototype.every", iter::build_every(), iter::spec_steps_every()),
        ("Array.prototype.some", iter::build_some(), iter::spec_steps_some()),
        ("Array.prototype.find", find::build_find(), find::spec_steps_find()),
        ("Array.prototype.findIndex", find::build_find_index(), find::spec_steps_find()),
        ("Array.prototype.findLast", find::build_find_last(), find::spec_steps_find()),
        ("Array.prototype.findLastIndex", find::build_find_last_index(), find::spec_steps_find()),
        ("Array.prototype.indexOf", index_search::build_index_of(), index_search::spec_steps_index_of()),
        ("Array.prototype.includes", index_search::build_includes(), index_search::spec_steps_includes()),
        ("Array.prototype.reduce", reduce::build_reduce(), reduce::spec_steps_reduce()),
        ("Object.keys", object_static::build_keys(), object_static::spec_steps_keys()),
        ("Object.values", object_static::build_values(), object_static::spec_steps_values()),
        ("Object.entries", object_static::build_entries(), object_static::spec_steps_entries()),
        ("Promise.resolve", promise_static::build_resolve(), promise_static::spec_steps_resolve()),
        ("Promise.reject", promise_static::build_reject(), promise_static::spec_steps_reject()),
        ("Object.getPrototypeOf", object_proto_ops::build_get_prototype_of(), object_proto_ops::spec_steps_get_prototype_of()),
        ("Object.setPrototypeOf", object_proto_ops::build_set_prototype_of(), object_proto_ops::spec_steps_set_prototype_of()),
        ("Object.isExtensible", object_proto_ops::build_is_extensible(), object_proto_ops::spec_steps_is_extensible()),
        ("Object.isFrozen", object_proto_ops::build_is_frozen(), object_proto_ops::spec_steps_is_frozen()),
        ("Object.isSealed", object_proto_ops::build_is_sealed(), object_proto_ops::spec_steps_is_sealed()),
        ("Object.freeze", object_integrity::build_freeze(), object_integrity::spec_steps_freeze()),
        ("Object.seal", object_integrity::build_seal(), object_integrity::spec_steps_seal()),
        ("Object.preventExtensions", object_integrity::build_prevent_extensions(), object_integrity::spec_steps_prevent_extensions()),
        ("Object.hasOwn", object_integrity::build_has_own(), object_integrity::spec_steps_has_own()),
        ("Object.is", object_integrity::build_is(), object_integrity::spec_steps_is()),
        ("Number.isFinite", number_static::build_is_finite(), number_static::spec_steps_is_finite()),
        ("Number.isInteger", number_static::build_is_integer(), number_static::spec_steps_is_integer()),
        ("Number.isNaN", number_static::build_is_nan(), number_static::spec_steps_is_nan()),
        ("Number.isSafeInteger", number_static::build_is_safe_integer(), number_static::spec_steps_is_safe_integer()),
        ("isNaN", global_predicates::build_is_nan(), global_predicates::spec_steps_is_nan()),
        ("isFinite", global_predicates::build_is_finite(), global_predicates::spec_steps_is_finite()),
        ("Math.abs",   math_unary::build_abs(),   math_unary::spec_steps_abs()),
        ("Math.floor", math_unary::build_floor(), math_unary::spec_steps_floor()),
        ("Math.ceil",  math_unary::build_ceil(),  math_unary::spec_steps_ceil()),
        ("Math.round", math_unary::build_round(), math_unary::spec_steps_round()),
        ("Math.trunc", math_unary::build_trunc(), math_unary::spec_steps_trunc()),
        ("Math.sqrt",  math_unary::build_sqrt(),  math_unary::spec_steps_sqrt()),
        ("Math.cbrt",  math_unary::build_cbrt(),  math_unary::spec_steps_cbrt()),
        ("Math.sign",  math_unary::build_sign(),  math_unary::spec_steps_sign()),
        ("Math.exp",   math_unary::build_exp(),   math_unary::spec_steps_exp()),
        ("Math.expm1", math_unary::build_expm1(), math_unary::spec_steps_expm1()),
        ("Math.log",   math_unary::build_log(),   math_unary::spec_steps_log()),
        ("Math.log1p", math_unary::build_log1p(), math_unary::spec_steps_log1p()),
        ("Math.log2",  math_unary::build_log2(),  math_unary::spec_steps_log2()),
        ("Math.log10", math_unary::build_log10(), math_unary::spec_steps_log10()),
        ("Math.sin",   math_unary::build_sin(),   math_unary::spec_steps_sin()),
        ("Math.cos",   math_unary::build_cos(),   math_unary::spec_steps_cos()),
        ("Math.tan",   math_unary::build_tan(),   math_unary::spec_steps_tan()),
        ("Math.asin",  math_unary::build_asin(),  math_unary::spec_steps_asin()),
        ("Math.acos",  math_unary::build_acos(),  math_unary::spec_steps_acos()),
        ("Math.atan",  math_unary::build_atan(),  math_unary::spec_steps_atan()),
        ("Math.sinh",  math_unary::build_sinh(),  math_unary::spec_steps_sinh()),
        ("Math.cosh",  math_unary::build_cosh(),  math_unary::spec_steps_cosh()),
        ("Math.tanh",  math_unary::build_tanh(),  math_unary::spec_steps_tanh()),
        ("Math.asinh", math_unary::build_asinh(), math_unary::spec_steps_asinh()),
        ("Math.acosh", math_unary::build_acosh(), math_unary::spec_steps_acosh()),
        ("Math.atanh", math_unary::build_atanh(), math_unary::spec_steps_atanh()),
        ("Reflect.has",            reflect_static::build_has(),             reflect_static::spec_steps_has()),
        ("Reflect.get",            reflect_static::build_get(),             reflect_static::spec_steps_get()),
        ("Reflect.set",            reflect_static::build_set(),             reflect_static::spec_steps_set()),
        ("Reflect.deleteProperty", reflect_static::build_delete_property(), reflect_static::spec_steps_delete_property()),
        ("Reflect.ownKeys",        reflect_static::build_own_keys(),        reflect_static::spec_steps_own_keys()),
        ("Reflect.getPrototypeOf", reflect_static::build_get_prototype_of(), reflect_static::spec_steps_get_prototype_of()),
        ("Reflect.setPrototypeOf", reflect_static::build_set_prototype_of(), reflect_static::spec_steps_set_prototype_of()),
        ("Reflect.isExtensible",   reflect_static::build_is_extensible(),   reflect_static::spec_steps_is_extensible()),
        ("Reflect.preventExtensions", reflect_static::build_prevent_extensions(), reflect_static::spec_steps_prevent_extensions()),
        ("Math.pow",   math_binary_variadic::build_pow(),   math_binary_variadic::spec_steps_pow()),
        ("Math.atan2", math_binary_variadic::build_atan2(), math_binary_variadic::spec_steps_atan2()),
        ("Math.max",   math_binary_variadic::build_max(),   math_binary_variadic::spec_steps_max()),
        ("Math.min",   math_binary_variadic::build_min(),   math_binary_variadic::spec_steps_min()),
        ("Math.hypot", math_binary_variadic::build_hypot(), math_binary_variadic::spec_steps_hypot()),
        ("Object.getOwnPropertyNames", object_static::build_get_own_property_names(), object_static::spec_steps_get_own_property_names()),
        ("Object.getOwnPropertySymbols", object_static::build_get_own_property_symbols(), object_static::spec_steps_get_own_property_symbols()),
        ("Object.assign", object_static::build_assign(), object_static::spec_steps_assign()),
        ("Object.fromEntries", object_static::build_from_entries(), object_static::spec_steps_from_entries()),
        ("Number.prototype.toFixed", number_prototype::build_to_fixed(), number_prototype::spec_steps_to_fixed()),
        ("Number.prototype.valueOf", number_prototype::build_value_of(), number_prototype::spec_steps_value_of()),
        ("Number.prototype.toExponential", number_prototype::build_to_exponential(), number_prototype::spec_steps_to_exponential()),
        ("Number.prototype.toPrecision", number_prototype::build_to_precision(), number_prototype::spec_steps_to_precision()),
        ("Boolean.prototype.valueOf", number_prototype::build_boolean_value_of(), number_prototype::spec_steps_boolean_value_of()),
        ("Boolean.prototype.toString", number_prototype::build_boolean_to_string(), number_prototype::spec_steps_boolean_to_string()),
        ("String.prototype.charAt",     string_prototype::build_char_at(),      string_prototype::spec_steps_char_at()),
        ("String.prototype.charCodeAt", string_prototype::build_char_code_at(), string_prototype::spec_steps_char_code_at()),
        ("String.prototype.concat",     string_prototype::build_concat(),       string_prototype::spec_steps_concat()),
    ];

    let mut total_unexpected = 0;
    for (name, f, spec) in &sections {
        // Filter the spec_steps list to drop synthetic-inline records
        // (matching the linter's collect_steps convention from
        // lint.rs). The hand-authored records still carry these for
        // documentation; the diff doesn't track them.
        let filtered_spec: Vec<_> = spec.iter()
            .filter(|r| !r.step_id.ends_with(".throw")
                && !r.step_id.ends_with(".guard")
                && !r.step_id.ends_with(".return")
                && !r.step_id.ends_with(".adj")
                && !r.step_id.ends_with(".seed"))
            .cloned()
            .collect();
        let report = lint(f, &filtered_spec);
        // Filter out known param.* binding-convention findings — those
        // are not in any spec section's algorithm step list.
        let unexpected: Vec<_> = report.findings.iter()
            .filter(|f| !f.spec_step.starts_with("param."))
            .collect();
        if unexpected.is_empty() {
            println!("✓ {} — OK", name);
        } else {
            println!("✗ {} — {} unexpected findings:", name, unexpected.len());
            for f in &unexpected {
                println!("    step {}: {}", f.spec_step, f.message);
            }
            total_unexpected += unexpected.len();
        }
    }

    println!();
    if total_unexpected == 0 {
        println!("All {} translated sections lint clean.", sections.len());
    } else {
        println!("{} total unexpected findings across {} sections.", total_unexpected, sections.len());
        std::process::exit(1);
    }
}
