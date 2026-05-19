//! ECMA-262 §23.1.3.{8, 9, 10, 11} — Array.prototype.{find, findIndex, findLast, findLastIndex}.
//!
//! find/findIndex retain their detailed spec-step IR shape (preamble + forward loop).
//! findLast/findLastIndex compile down to 1-step CallBuiltin to runtime helpers that
//! perform backward iteration directly (the Tier-1.7 forward-iterate-track-last
//! simplification was wired through E29 but diverged from cruftless's hand-written
//! impl on side-effecting predicates; lifting to a runtime helper restores parity).

use crate::ir::{ErrorClass, Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn b(e: Expr) -> Box<Expr> { Box::new(e) }
fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

fn preamble(method: &str) -> Vec<Step> {
    vec![
        Step { spec_step: "param.predicate".into(),
            node: IRNode::Let { name: "predicate".into(), value: Expr::Arg(0) }},
        Step { spec_step: "param.thisArg".into(),
            node: IRNode::Let { name: "this_arg".into(), value: Expr::Arg(1) }},
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "o".into(), value: Expr::ToObject(b(Expr::This)) }},
        Step { spec_step: "2".into(),
            node: IRNode::LetIndex { name: "len".into(),
                value: Expr::LengthOfArrayLike(b(v("o"))) }},
        Step { spec_step: "3".into(),
            node: IRNode::If {
                cond: Expr::Not(b(Expr::IsCallable(b(v("predicate"))))),
                then_body: vec![Step { spec_step: "3.throw".into(),
                    node: IRNode::Throw { class: ErrorClass::TypeError,
                        message: format!("Array.prototype.{}: predicate is not callable", method) }}],
                else_body: vec![],
            }},
    ]
}

/// Inner loop body — Get + Call + check. Return `return_what` on match.
fn loop_body(return_what: Expr) -> Vec<Step> {
    vec![
        Step { spec_step: "5.a".into(),
            node: IRNode::Let { name: "pk".into(), value: Expr::IndexAsKey(b(v("k"))) }},
        Step { spec_step: "5.b".into(),
            node: IRNode::Let { name: "k_value".into(),
                value: Expr::Get(b(v("o")), b(v("pk"))) }},
        Step { spec_step: "5.c".into(),
            node: IRNode::Let { name: "test_result".into(),
                value: Expr::ToBoolean(b(Expr::Call {
                    function: b(v("predicate")),
                    this: b(v("this_arg")),
                    args: vec![v("k_value"), Expr::IndexAsValue(b(v("k"))), v("o")],
                })) }},
        Step { spec_step: "5.d".into(),
            node: IRNode::If {
                cond: v("test_result"),
                then_body: vec![Step { spec_step: "5.d.i".into(),
                    node: IRNode::Return(return_what) }],
                else_body: vec![],
            }},
        Step { spec_step: "5.e".into(),
            node: IRNode::AssignIndex { name: "k".into(),
                value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))) }},
    ]
}

// ──────────────── §23.1.3.8 find ────────────────

pub fn build_find() -> IRFunction {
    let mut body = preamble("find");
    body.push(Step { spec_step: "4".into(),
        node: IRNode::LetIndex { name: "k".into(), value: Expr::IntConst(0) }});
    body.push(Step { spec_step: "5".into(),
        node: IRNode::While {
            cond: Expr::Lt(b(v("k")), b(v("len"))),
            body: loop_body(v("k_value")),
        }});
    body.push(Step { spec_step: "6".into(),
        node: IRNode::Return(Expr::Undefined) });
    IRFunction {
        spec_section: "23.1.3.8".into(),
        rust_name: "array_prototype_find".into(),
        title: "Array.prototype.find ( predicate [ , thisArg ] )".into(),
        body,
    }
}

// ──────────────── §23.1.3.9 findIndex ────────────────

pub fn build_find_index() -> IRFunction {
    let mut body = preamble("findIndex");
    body.push(Step { spec_step: "4".into(),
        node: IRNode::LetIndex { name: "k".into(), value: Expr::IntConst(0) }});
    body.push(Step { spec_step: "5".into(),
        node: IRNode::While {
            cond: Expr::Lt(b(v("k")), b(v("len"))),
            body: loop_body(Expr::IndexAsValue(b(v("k")))),
        }});
    body.push(Step { spec_step: "6".into(),
        node: IRNode::Return(Expr::Number(-1.0)) });
    IRFunction {
        spec_section: "23.1.3.9".into(),
        rust_name: "array_prototype_find_index".into(),
        title: "Array.prototype.findIndex ( predicate [ , thisArg ] )".into(),
        body,
    }
}

// ──────────────── §23.1.3.10 findLast (CallBuiltin to backward-iterating helper) ────────────────

pub fn build_find_last() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "array_proto_find_last_via", args: vec![Expr::AllArgs],
        })},
    ];
    IRFunction {
        spec_section: "23.1.3.10".into(),
        rust_name: "array_prototype_find_last".into(),
        title: "Array.prototype.findLast ( predicate [ , thisArg ] )".into(),
        body,
    }
}

// ──────────────── §23.1.3.11 findLastIndex (CallBuiltin to backward-iterating helper) ────────────────

pub fn build_find_last_index() -> IRFunction {
    let body = vec![
        Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "array_proto_find_last_index_via", args: vec![Expr::AllArgs],
        })},
    ];
    IRFunction {
        spec_section: "23.1.3.11".into(),
        rust_name: "array_prototype_find_last_index".into(),
        title: "Array.prototype.findLastIndex ( predicate [ , thisArg ] )".into(),
        body,
    }
}

// ──────────────── linter records ────────────────

pub fn spec_steps_find() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(),     abstract_ops: vec!["ToObject"],          throws: None, prose: "Let O be ? ToObject(this value)." },
        SpecStepRecord { step_id: "2".into(),     abstract_ops: vec!["LengthOfArrayLike"], throws: None, prose: "Let len be ? LengthOfArrayLike(O)." },
        SpecStepRecord { step_id: "3".into(),     abstract_ops: vec!["IsCallable"],        throws: None, prose: "If IsCallable(predicate) is false, throw TypeError." },
        SpecStepRecord { step_id: "3.throw".into(),abstract_ops: vec!["Throw"],            throws: Some("TypeError"), prose: "throw a TypeError exception." },
        SpecStepRecord { step_id: "4".into(),     abstract_ops: vec![],                    throws: None, prose: "Let k be 0." },
        SpecStepRecord { step_id: "5".into(),     abstract_ops: vec![],                    throws: None, prose: "Repeat, while k < len, …" },
        SpecStepRecord { step_id: "5.a".into(),   abstract_ops: vec!["ToString"],          throws: None, prose: "Let Pk be ! ToString(𝔽(k))." },
        SpecStepRecord { step_id: "5.b".into(),   abstract_ops: vec!["Get"],               throws: None, prose: "Let kValue be ? Get(O, Pk)." },
        SpecStepRecord { step_id: "5.c".into(),   abstract_ops: vec!["Call", "ToBoolean"], throws: None, prose: "Let testResult be ToBoolean(? Call(predicate, thisArg, « kValue, 𝔽(k), O »))." },
        SpecStepRecord { step_id: "5.d".into(),   abstract_ops: vec![],                    throws: None, prose: "If testResult is true, return kValue." },
        SpecStepRecord { step_id: "5.d.i".into(), abstract_ops: vec![],                    throws: None, prose: "Return kValue." },
        SpecStepRecord { step_id: "5.e".into(),   abstract_ops: vec![],                    throws: None, prose: "Set k to k + 1." },
        SpecStepRecord { step_id: "6".into(),     abstract_ops: vec![],                    throws: None, prose: "Return undefined." },
    ]
}

pub fn spec_steps_find_index() -> Vec<SpecStepRecord> {
    let mut r = spec_steps_find();
    r.last_mut().unwrap().prose = "Return -1.";
    r
}

pub fn spec_steps_find_last() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_find_last_via"], throws: None,
        prose: "Iterate the array-like backward; return the first kValue for which the predicate returns truthy, or undefined." }]
}

pub fn spec_steps_find_last_index() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["array_proto_find_last_index_via"], throws: None,
        prose: "Iterate the array-like backward; return the index k for which the predicate returns truthy, or -1." }]
}
