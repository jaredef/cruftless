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
