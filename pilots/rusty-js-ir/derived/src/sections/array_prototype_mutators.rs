//! ECMA-262 §23.1.3.{20, 19, 26, 32, 21} — Array.prototype.{push, pop, shift, unshift, reverse}.
//!
//! Mutators. Each is a 1-step CallBuiltin into a runtime helper that handles
//! the length-and-element rewiring directly.

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

pub fn build_to_reversed()      -> IRFunction { nullary_section ("23.1.3.33", "array_prototype_to_reversed",       "Array.prototype.toReversed ( )",                                "array_proto_to_reversed_via") }
pub fn build_to_sorted()        -> IRFunction { variadic_section("23.1.3.34", "array_prototype_to_sorted",         "Array.prototype.toSorted ( comparefn )",                         "array_proto_to_sorted_via") }
pub fn build_to_spliced()       -> IRFunction { variadic_section("23.1.3.35", "array_prototype_to_spliced",        "Array.prototype.toSpliced ( start, deleteCount, ...items )",      "array_proto_to_spliced_via") }
pub fn build_with()             -> IRFunction { variadic_section("23.1.3.39", "array_prototype_with",              "Array.prototype.with ( index, value )",                           "array_proto_with_via") }
pub fn build_to_locale_string() -> IRFunction { nullary_section ("23.1.3.30", "array_prototype_to_locale_string",  "Array.prototype.toLocaleString ( )",                              "array_proto_to_locale_string_via") }
pub fn build_to_string()        -> IRFunction { nullary_section ("23.1.3.36", "array_prototype_to_string",         "Array.prototype.toString ( )",                                    "array_proto_to_string_via") }

pub fn spec_steps_to_reversed()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_to_reversed_via"],      throws: None, prose: "Return a new array with the elements in reverse order; original unchanged." }] }
pub fn spec_steps_to_sorted()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_to_sorted_via"],        throws: None, prose: "Return a new sorted array; default lexicographic by ToString unless comparefn provided." }] }
pub fn spec_steps_to_spliced()       -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_to_spliced_via"],       throws: None, prose: "Return a new array with deleteCount elements removed at start and items inserted." }] }
pub fn spec_steps_with()             -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_with_via"],             throws: None, prose: "Return a new array where the element at relative index is replaced with value; throw on out-of-bounds." }] }
pub fn spec_steps_to_locale_string() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_to_locale_string_via"], throws: None, prose: "Coerce each element to string and join with comma (v1 locale-insensitive)." }] }
pub fn spec_steps_to_string()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_to_string_via"],        throws: None, prose: "Delegate to this.join() if callable; else return \"[object Array]\"." }] }

pub fn build_slice()        -> IRFunction { variadic_section("23.1.3.28", "array_prototype_slice",         "Array.prototype.slice ( start, end )",                              "array_proto_slice_via") }
pub fn build_splice()       -> IRFunction { variadic_section("23.1.3.31", "array_prototype_splice",        "Array.prototype.splice ( start, deleteCount, ...items )",            "array_proto_splice_via") }
pub fn build_concat()       -> IRFunction { variadic_section("23.1.3.2",  "array_prototype_concat",        "Array.prototype.concat ( ...items )",                                "array_proto_concat_via") }
pub fn build_join()         -> IRFunction { variadic_section("23.1.3.15", "array_prototype_join",          "Array.prototype.join ( separator )",                                 "array_proto_join_via") }
pub fn build_at()           -> IRFunction { variadic_section("23.1.3.1",  "array_prototype_at",            "Array.prototype.at ( index )",                                       "array_proto_at_via") }
pub fn build_fill()         -> IRFunction { variadic_section("23.1.3.7",  "array_prototype_fill",          "Array.prototype.fill ( value, start, end )",                         "array_proto_fill_via") }
pub fn build_last_index_of()-> IRFunction { variadic_section("23.1.3.18", "array_prototype_last_index_of", "Array.prototype.lastIndexOf ( searchElement, fromIndex )",           "array_proto_last_index_of_via") }
pub fn build_reduce_right() -> IRFunction { variadic_section("23.1.3.25", "array_prototype_reduce_right",  "Array.prototype.reduceRight ( callbackfn, initialValue )",           "array_proto_reduce_right_via") }
pub fn build_copy_within()  -> IRFunction { variadic_section("23.1.3.4",  "array_prototype_copy_within",   "Array.prototype.copyWithin ( target, start, end )",                  "array_proto_copy_within_via") }
pub fn build_flat()         -> IRFunction { variadic_section("23.1.3.10", "array_prototype_flat",          "Array.prototype.flat ( depth )",                                     "array_proto_flat_via") }
pub fn build_flat_map()     -> IRFunction { variadic_section("23.1.3.11", "array_prototype_flat_map",      "Array.prototype.flatMap ( callback, thisArg )",                      "array_proto_flat_map_via") }

pub fn spec_steps_slice()         -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_slice_via"],         throws: None, prose: "Return a new array containing the elements from clamped start to clamped end." }] }
pub fn spec_steps_splice()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_splice_via"],        throws: None, prose: "Remove deleteCount elements starting at start; insert items; return removed elements." }] }
pub fn spec_steps_concat()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_concat_via"],        throws: None, prose: "Return a new array with this + each arg, spreading per IsConcatSpreadable." }] }
pub fn spec_steps_join()          -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_join_via"],          throws: None, prose: "Coerce each element to string (treating null/undefined as empty) and join with separator." }] }
pub fn spec_steps_at()            -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_at_via"],            throws: None, prose: "Return the element at the relative index (negative counts from end), or undefined." }] }
pub fn spec_steps_fill()          -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_fill_via"],          throws: None, prose: "Fill indices [start, end) with value; clamp negative indices; return this." }] }
pub fn spec_steps_last_index_of() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_last_index_of_via"], throws: None, prose: "Search backward from fromIndex; return the last index whose element strictly-equals searchElement, or -1." }] }
pub fn spec_steps_reduce_right()  -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_reduce_right_via"], throws: None, prose: "Fold callback right-to-left over the array-like; seed from initialValue or last present element." }] }
pub fn spec_steps_copy_within()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_copy_within_via"],  throws: None, prose: "Copy a slice within the array-like, handling overlap via read-then-write buffer." }] }
pub fn spec_steps_flat()          -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_flat_via"],          throws: None, prose: "Return a new array with nested arrays flattened up to depth (default 1)." }] }
pub fn spec_steps_flat_map()      -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_flat_map_via"],      throws: None, prose: "Map then flatten by one level; non-array results are appended as-is." }] }

pub fn build_push()    -> IRFunction { variadic_section("23.1.3.20", "array_prototype_push",    "Array.prototype.push ( ...items )",    "array_proto_push_via") }
pub fn build_pop()     -> IRFunction { nullary_section ("23.1.3.19", "array_prototype_pop",     "Array.prototype.pop ( )",              "array_proto_pop_via") }
pub fn build_shift()   -> IRFunction { nullary_section ("23.1.3.26", "array_prototype_shift",   "Array.prototype.shift ( )",            "array_proto_shift_via") }
pub fn build_unshift() -> IRFunction { variadic_section("23.1.3.32", "array_prototype_unshift", "Array.prototype.unshift ( ...items )", "array_proto_unshift_via") }
pub fn build_reverse() -> IRFunction { nullary_section ("23.1.3.21", "array_prototype_reverse", "Array.prototype.reverse ( )",          "array_proto_reverse_via") }

pub fn spec_steps_push()    -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_push_via"],    throws: None, prose: "Append items to the array-like, update length, return new length." }] }
pub fn spec_steps_pop()     -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_pop_via"],     throws: None, prose: "Remove and return the last element of the array-like; return undefined when empty." }] }
pub fn spec_steps_shift()   -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_shift_via"],   throws: None, prose: "Remove and return the first element, shifting remaining indices left." }] }
pub fn spec_steps_unshift() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_unshift_via"], throws: None, prose: "Shift existing elements right by argCount; insert items at the front; return new length." }] }
pub fn spec_steps_reverse() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_reverse_via"], throws: None, prose: "Reverse the elements of the array-like in place; return the same object." }] }
