//! ECMA-262 §20.1.2 — Object.{defineProperty, defineProperties,
//! getOwnPropertyDescriptor, getOwnPropertyDescriptors, create} and
//! Annex B.2.2.{2,3,4,5} — Object.prototype.{__defineGetter__,
//! __defineSetter__, __lookupGetter__, __lookupSetter__}.
//!
//! Each section is a 1-step CallBuiltin to a runtime helper that holds
//! the §10.1.6 ValidateAndApplyPropertyDescriptor + §6.2.5.5
//! ToPropertyDescriptor logic in Rust. Per the IR-EXT 56 recognition:
//! since a property descriptor is just a JS Object in v1, the existing
//! CallBuiltin + via-helper pattern handles the entire family without
//! needing new IR alphabet primitives — the "descriptor builder"
//! extension queued at IR-EXT 52 reduces to a runtime extension.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

pub fn build_define_property() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(), node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) } },
        Step { spec_step: "param.key".into(),    node: IRNode::Let { name: "key".into(),    value: Expr::Arg(1) } },
        Step { spec_step: "param.desc".into(),   node: IRNode::Let { name: "desc".into(),   value: Expr::Arg(2) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_define_property_via",
            args: vec![v("target"), v("key"), v("desc")],
        })},
    ];
    IRFunction {
        spec_section: "20.1.2.4".into(),
        rust_name: "object_define_property".into(),
        title: "Object.defineProperty ( O, P, Attributes )".into(),
        body,
    }
}

pub fn spec_steps_define_property() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_define_property_via"], throws: None,
            prose: "Coerce P via ToPropertyKey, validate Attributes via ToPropertyDescriptor (§6.2.5.5), then perform OrdinaryDefineOwnProperty (§10.1.6). Return O." },
    ]
}

pub fn build_define_properties() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(), node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) } },
        Step { spec_step: "param.props".into(),  node: IRNode::Let { name: "props".into(),  value: Expr::Arg(1) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_define_properties_via",
            args: vec![v("target"), v("props")],
        })},
    ];
    IRFunction {
        spec_section: "20.1.2.5".into(),
        rust_name: "object_define_properties".into(),
        title: "Object.defineProperties ( O, Properties )".into(),
        body,
    }
}

pub fn spec_steps_define_properties() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_define_properties_via"], throws: None,
            prose: "For each enumerable own key of Properties, perform DefinePropertyOrThrow on the descriptor value. Return O." },
    ]
}

pub fn build_get_own_property_descriptor() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.obj".into(), node: IRNode::Let { name: "obj".into(), value: Expr::Arg(0) } },
        Step { spec_step: "param.key".into(), node: IRNode::Let { name: "key".into(), value: Expr::Arg(1) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_get_own_property_descriptor_via",
            args: vec![v("obj"), v("key")],
        })},
    ];
    IRFunction {
        spec_section: "20.1.2.9".into(),
        rust_name: "object_get_own_property_descriptor".into(),
        title: "Object.getOwnPropertyDescriptor ( O, P )".into(),
        body,
    }
}

pub fn spec_steps_get_own_property_descriptor() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_get_own_property_descriptor_via"], throws: None,
            prose: "Coerce O via ToObject, P via ToPropertyKey. Return FromPropertyDescriptor(OrdinaryGetOwnProperty(O, P)) — accessor shape {get,set,enumerable,configurable} for accessor, data shape {value,writable,enumerable,configurable} otherwise; undefined if absent." },
    ]
}

pub fn build_get_own_property_descriptors() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.obj".into(), node: IRNode::Let { name: "obj".into(), value: Expr::Arg(0) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_get_own_property_descriptors_via",
            args: vec![v("obj")],
        })},
    ];
    IRFunction {
        spec_section: "20.1.2.10".into(),
        rust_name: "object_get_own_property_descriptors".into(),
        title: "Object.getOwnPropertyDescriptors ( O )".into(),
        body,
    }
}

pub fn spec_steps_get_own_property_descriptors() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_get_own_property_descriptors_via"], throws: None,
            prose: "For each own key of O, build FromPropertyDescriptor and assemble into a new object keyed by the original key." },
    ]
}

pub fn build_create() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.proto".into(), node: IRNode::Let { name: "proto".into(), value: Expr::Arg(0) } },
        Step { spec_step: "param.props".into(), node: IRNode::Let { name: "props".into(), value: Expr::Arg(1) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_create_via",
            args: vec![v("proto"), v("props")],
        })},
    ];
    IRFunction {
        spec_section: "20.1.2.2".into(),
        rust_name: "object_create".into(),
        title: "Object.create ( O, Properties )".into(),
        body,
    }
}

pub fn spec_steps_create() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_create_via"], throws: None,
            prose: "If O is neither Object nor Null, throw TypeError. Allocate an OrdinaryObject with [[Prototype]] = O. If Properties is not undefined, perform ObjectDefineProperties(obj, Properties)." },
    ]
}

pub fn build_proto_define_getter() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.this".into(), node: IRNode::Let { name: "this_".into(), value: Expr::This } },
        Step { spec_step: "param.key".into(),  node: IRNode::Let { name: "key".into(),   value: Expr::Arg(0) } },
        Step { spec_step: "param.fn".into(),   node: IRNode::Let { name: "fn_".into(),   value: Expr::Arg(1) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_proto_define_getter_via",
            args: vec![v("this_"), v("key"), v("fn_")],
        })},
    ];
    IRFunction {
        spec_section: "B.2.2.2".into(),
        rust_name: "object_proto_define_getter".into(),
        title: "Object.prototype.__defineGetter__ ( P, getter )".into(),
        body,
    }
}

pub fn spec_steps_proto_define_getter() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_define_getter_via"], throws: None,
            prose: "If getter is not callable, throw TypeError. Otherwise, install an accessor descriptor with the supplied getter, preserving any existing setter; enumerable:true, configurable:true." },
    ]
}

pub fn build_proto_define_setter() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.this".into(), node: IRNode::Let { name: "this_".into(), value: Expr::This } },
        Step { spec_step: "param.key".into(),  node: IRNode::Let { name: "key".into(),   value: Expr::Arg(0) } },
        Step { spec_step: "param.fn".into(),   node: IRNode::Let { name: "fn_".into(),   value: Expr::Arg(1) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_proto_define_setter_via",
            args: vec![v("this_"), v("key"), v("fn_")],
        })},
    ];
    IRFunction {
        spec_section: "B.2.2.3".into(),
        rust_name: "object_proto_define_setter".into(),
        title: "Object.prototype.__defineSetter__ ( P, setter )".into(),
        body,
    }
}

pub fn spec_steps_proto_define_setter() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_define_setter_via"], throws: None,
            prose: "If setter is not callable, throw TypeError. Otherwise, install an accessor descriptor with the supplied setter, preserving any existing getter; enumerable:true, configurable:true." },
    ]
}

pub fn build_proto_lookup_getter() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.this".into(), node: IRNode::Let { name: "this_".into(), value: Expr::This } },
        Step { spec_step: "param.key".into(),  node: IRNode::Let { name: "key".into(),   value: Expr::Arg(0) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_proto_lookup_getter_via",
            args: vec![v("this_"), v("key")],
        })},
    ];
    IRFunction {
        spec_section: "B.2.2.4".into(),
        rust_name: "object_proto_lookup_getter".into(),
        title: "Object.prototype.__lookupGetter__ ( P )".into(),
        body,
    }
}

pub fn spec_steps_proto_lookup_getter() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_lookup_getter_via"], throws: None,
            prose: "Walk own property at P (no proto-chain in v1) and return its getter; undefined if absent or data descriptor." },
    ]
}

pub fn build_proto_lookup_setter() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.this".into(), node: IRNode::Let { name: "this_".into(), value: Expr::This } },
        Step { spec_step: "param.key".into(),  node: IRNode::Let { name: "key".into(),   value: Expr::Arg(0) } },
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "object_proto_lookup_setter_via",
            args: vec![v("this_"), v("key")],
        })},
    ];
    IRFunction {
        spec_section: "B.2.2.5".into(),
        rust_name: "object_proto_lookup_setter".into(),
        title: "Object.prototype.__lookupSetter__ ( P )".into(),
        body,
    }
}

pub fn spec_steps_proto_lookup_setter() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_proto_lookup_setter_via"], throws: None,
            prose: "Walk own property at P (no proto-chain in v1) and return its setter; undefined if absent or data descriptor." },
    ]
}
