//! ECMA-262 §20.1.3 — Object.prototype.{toString, hasOwnProperty, valueOf, propertyIsEnumerable, isPrototypeOf, toLocaleString}.

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

pub fn build_to_string()              -> IRFunction { nullary_section ("20.1.3.6", "object_prototype_to_string",              "Object.prototype.toString ( )",                       "object_proto_to_string_via") }
pub fn build_has_own_property()       -> IRFunction { variadic_section("20.1.3.2", "object_prototype_has_own_property",        "Object.prototype.hasOwnProperty ( V )",               "object_proto_has_own_property_via") }
pub fn build_value_of()               -> IRFunction { nullary_section ("20.1.3.7", "object_prototype_value_of",                "Object.prototype.valueOf ( )",                        "object_proto_value_of_via") }
pub fn build_property_is_enumerable() -> IRFunction { variadic_section("20.1.3.4", "object_prototype_property_is_enumerable",  "Object.prototype.propertyIsEnumerable ( V )",         "object_proto_property_is_enumerable_via") }
pub fn build_is_prototype_of()        -> IRFunction { variadic_section("20.1.3.3", "object_prototype_is_prototype_of",         "Object.prototype.isPrototypeOf ( V )",                "object_proto_is_prototype_of_via") }
pub fn build_to_locale_string()       -> IRFunction { nullary_section ("20.1.3.5", "object_prototype_to_locale_string",        "Object.prototype.toLocaleString ( )",                 "object_proto_to_locale_string_via") }

pub fn spec_steps_to_string()              -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_to_string_via"],              throws: None, prose: "Return [object Tag] where Tag is @@toStringTag (if string) or the internal-slot tag (Array/Function/Promise/Error/RegExp/Object)." }] }
pub fn spec_steps_has_own_property()       -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_has_own_property_via"],       throws: None, prose: "Return whether this has an own property with the given key (no proto-chain lookup)." }] }
pub fn spec_steps_value_of()               -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_value_of_via"],               throws: None, prose: "Return ? ToObject(this value) — for Object.prototype, this is the identity." }] }
pub fn spec_steps_property_is_enumerable() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_property_is_enumerable_via"], throws: None, prose: "Return whether this has an own enumerable property at the given key (v1: treats all own properties as enumerable)." }] }
pub fn spec_steps_is_prototype_of()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_is_prototype_of_via"],        throws: None, prose: "Walk target's proto chain; return true if this is encountered." }] }
pub fn spec_steps_to_locale_string()       -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_to_locale_string_via"],       throws: None, prose: "Default to invoking this.toString(); subclass prototypes (Number/Date/Array) override." }] }
