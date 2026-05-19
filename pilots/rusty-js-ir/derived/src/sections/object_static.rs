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

// ──────────────── §20.1.2.10/.11 getOwnPropertyNames / Symbols ────────────────

pub fn build_get_own_property_names() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "obj".into(),
                value: Expr::ToObject(b(v("target"))) }},
        Step { spec_step: "2".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "own_property_names_via",
                args: vec![v("obj")],
            }) },
    ];
    IRFunction {
        spec_section: "20.1.2.10".into(),
        rust_name: "object_get_own_property_names".into(),
        title: "Object.getOwnPropertyNames ( O )".into(),
        body,
    }
}

pub fn build_get_own_property_symbols() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "obj".into(),
                value: Expr::ToObject(b(v("target"))) }},
        Step { spec_step: "2".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "own_property_symbols_via",
                args: vec![v("obj")],
            }) },
    ];
    IRFunction {
        spec_section: "20.1.2.11".into(),
        rust_name: "object_get_own_property_symbols".into(),
        title: "Object.getOwnPropertySymbols ( O )".into(),
        body,
    }
}

pub fn spec_steps_get_own_property_names() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["ToObject"], throws: None, prose: "Let obj be ? ToObject(O)." },
        SpecStepRecord { step_id: "2".into(), abstract_ops: vec!["own_property_names_via"], throws: None, prose: "Return CreateArrayFromList(? GetOwnPropertyKeys(obj, string))." },
    ]
}

pub fn spec_steps_get_own_property_symbols() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["ToObject"], throws: None, prose: "Let obj be ? ToObject(O)." },
        SpecStepRecord { step_id: "2".into(), abstract_ops: vec!["own_property_symbols_via"], throws: None, prose: "Return CreateArrayFromList(? GetOwnPropertyKeys(obj, symbol))." },
    ]
}

// ──────────────── §20.1.2.1 Object.assign ────────────────

pub fn build_assign() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(),
            node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "object_assign_via",
                args: vec![v("target"), Expr::ArgsRest(1)],
            }) },
    ];
    IRFunction {
        spec_section: "20.1.2.1".into(),
        rust_name: "object_assign".into(),
        title: "Object.assign ( target, ...sources )".into(),
        body,
    }
}

pub fn spec_steps_assign() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_assign_via"], throws: None,
            prose: "Let to be ? ToObject(target). For each source S, for each own enumerable key K of S, set to[K] = ? Get(S, K). Return to." },
    ]
}

// ──────────────── §20.1.2.7 Object.fromEntries ────────────────

pub fn build_from_entries() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.iter".into(),
            node: IRNode::Let { name: "iter".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "object_from_entries_via",
                args: vec![v("iter")],
            }) },
    ];
    IRFunction {
        spec_section: "20.1.2.7".into(),
        rust_name: "object_from_entries".into(),
        title: "Object.fromEntries ( iterable )".into(),
        body,
    }
}

pub fn spec_steps_from_entries() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["object_from_entries_via"], throws: None,
        prose: "RequireObjectCoercible(iterable). Iterate via @@iterator; for each [k, v] pair, set obj[ToPropertyKey(k)] = v. Return obj." }]
}
