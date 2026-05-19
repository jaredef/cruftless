//! ECMA-262 §20.1.2.{18, 23, 5} — Object.{keys, values, entries}.
//!
//! Per spec, each is a thin wrapper around EnumerableOwnPropertyNames(O, kind)
//! per §7.3.23. cruftless exposes this via three Runtime helpers:
//!   - rt.enumerable_own_keys(v)
//!   - rt.enumerable_own_values(v)
//!   - rt.enumerable_own_entries(v)
//!
//! Each helper handles ToObject coercion + accessor-getter dispatch + the
//! integer-index-first ordering required by §10.1.11 OrdinaryOwnPropertyKeys.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn b(e: Expr) -> Box<Expr> { Box::new(e) }
fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

pub fn build_keys() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "obj".into(),
                value: Expr::ToObject(b(v("target"))) }},
        Step { spec_step: "2".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "enumerable_own_keys",
                args: vec![v("obj")],
            }) },
    ];
    IRFunction {
        spec_section: "20.1.2.18".into(),
        rust_name: "object_keys".into(),
        title: "Object.keys ( O )".into(),
        body,
    }
}

pub fn build_values() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "obj".into(),
                value: Expr::ToObject(b(v("target"))) }},
        Step { spec_step: "2".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "enumerable_own_values",
                args: vec![v("obj")],
            }) },
    ];
    IRFunction {
        spec_section: "20.1.2.23".into(),
        rust_name: "object_values".into(),
        title: "Object.values ( O )".into(),
        body,
    }
}

pub fn build_entries() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "obj".into(),
                value: Expr::ToObject(b(v("target"))) }},
        Step { spec_step: "2".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "enumerable_own_entries",
                args: vec![v("obj")],
            }) },
    ];
    IRFunction {
        spec_section: "20.1.2.5".into(),
        rust_name: "object_entries".into(),
        title: "Object.entries ( O )".into(),
        body,
    }
}

pub fn spec_steps_keys() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["ToObject"], throws: None, prose: "Let obj be ? ToObject(O)." },
        SpecStepRecord { step_id: "2".into(), abstract_ops: vec!["enumerable_own_keys"], throws: None, prose: "Return ? EnumerableOwnPropertyNames(obj, key)." },
    ]
}

pub fn spec_steps_values() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["ToObject"], throws: None, prose: "Let obj be ? ToObject(O)." },
        SpecStepRecord { step_id: "2".into(), abstract_ops: vec!["enumerable_own_values"], throws: None, prose: "Return ? EnumerableOwnPropertyNames(obj, value)." },
    ]
}

pub fn spec_steps_entries() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["ToObject"], throws: None, prose: "Let obj be ? ToObject(O)." },
        SpecStepRecord { step_id: "2".into(), abstract_ops: vec!["enumerable_own_entries"], throws: None, prose: "Return ? EnumerableOwnPropertyNames(obj, key+value)." },
    ]
}
