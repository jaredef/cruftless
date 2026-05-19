//! ECMA-262 §28.1.{4, 8, 9, 12, 13} — Reflect.{has, get, set, deleteProperty, ownKeys}.
//!
//! Five sections mirroring the existing Object.* patterns plus the
//! Reflect-specific "return boolean instead of throw" semantics for
//! the mutation ops.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

pub fn build_has() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "param.key".into(),
            node: IRNode::Let { name: "key".into(), value: Expr::Arg(1) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_has_via", args: vec![v("target"), v("key")],
            })},
    ];
    IRFunction { spec_section: "28.1.9".into(),
        rust_name: "reflect_has".into(),
        title: "Reflect.has ( target, propertyKey )".into(), body }
}

pub fn build_get() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "param.key".into(),
            node: IRNode::Let { name: "key".into(), value: Expr::Arg(1) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_get_via", args: vec![v("target"), v("key")],
            })},
    ];
    IRFunction { spec_section: "28.1.8".into(),
        rust_name: "reflect_get".into(),
        title: "Reflect.get ( target, propertyKey [ , receiver ] )".into(), body }
}

pub fn build_set() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "param.key".into(),
            node: IRNode::Let { name: "key".into(), value: Expr::Arg(1) }},
        Step { spec_step: "param.value".into(),
            node: IRNode::Let { name: "value".into(), value: Expr::Arg(2) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_set_via", args: vec![v("target"), v("key"), v("value")],
            })},
    ];
    IRFunction { spec_section: "28.1.13".into(),
        rust_name: "reflect_set".into(),
        title: "Reflect.set ( target, propertyKey, V [ , receiver ] )".into(), body }
}

pub fn build_delete_property() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "param.key".into(),
            node: IRNode::Let { name: "key".into(), value: Expr::Arg(1) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_delete_property_via", args: vec![v("target"), v("key")],
            })},
    ];
    IRFunction { spec_section: "28.1.4".into(),
        rust_name: "reflect_delete_property".into(),
        title: "Reflect.deleteProperty ( target, propertyKey )".into(), body }
}

pub fn build_own_keys() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_own_keys_via", args: vec![v("target")],
            })},
    ];
    IRFunction { spec_section: "28.1.12".into(),
        rust_name: "reflect_own_keys".into(),
        title: "Reflect.ownKeys ( target )".into(), body }
}

// ──────────────── linter records ────────────────

pub fn spec_steps_has() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["reflect_has_via"], throws: None,
        prose: "Return ? HasProperty(target, ToPropertyKey(propertyKey))." }]
}
pub fn spec_steps_get() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["reflect_get_via"], throws: None,
        prose: "Return ? target.[[Get]](ToPropertyKey(propertyKey), receiver)." }]
}
pub fn spec_steps_set() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["reflect_set_via"], throws: None,
        prose: "Return ? target.[[Set]](ToPropertyKey(propertyKey), V, receiver)." }]
}
pub fn spec_steps_delete_property() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["reflect_delete_property_via"], throws: None,
        prose: "Return ? target.[[Delete]](ToPropertyKey(propertyKey))." }]
}
pub fn spec_steps_own_keys() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["reflect_own_keys_via"], throws: None,
        prose: "Let keys be ? target.[[OwnPropertyKeys]](). Return CreateArrayFromList(keys)." }]
}
