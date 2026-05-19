//! ECMA-262 §23.1.3.{16, 19, 13} — Array.prototype.{indexOf, lastIndexOf, includes}.
//!
//! Comparison-based search; no callback. indexOf/lastIndexOf use
//! IsStrictlyEqual (NaN != NaN); includes uses SameValueZero (NaN == NaN).
//!
//! Tier-1.7 simplification: fromIndex defaults to 0 (or len-1 for
//! lastIndexOf), no negative-fromIndex normalization. Full normalization
//! per spec is queued for a follow-on round.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn b(e: Expr) -> Box<Expr> { Box::new(e) }
fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

// ──────────────── §23.1.3.16 indexOf ────────────────

pub fn build_index_of() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.searchElement".into(),
            node: IRNode::Let { name: "search_element".into(), value: Expr::Arg(0) }},
        // step 1: Let O be ? ToObject(this value).
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "o".into(), value: Expr::ToObject(b(Expr::This)) }},
        // step 2: Let len be ? LengthOfArrayLike(O).
        Step { spec_step: "2".into(),
            node: IRNode::LetIndex { name: "len".into(),
                value: Expr::LengthOfArrayLike(b(v("o"))) }},
        // step 3: If len = 0, return -1F.
        Step { spec_step: "3".into(),
            node: IRNode::If {
                cond: Expr::Not(b(Expr::Lt(b(Expr::IntConst(0)), b(v("len"))))),
                then_body: vec![Step { spec_step: "3.return".into(),
                    node: IRNode::Return(Expr::Number(-1.0)) }],
                else_body: vec![],
            }},
        // step 4-7 simplified: k starts at 0 (fromIndex normalization skipped).
        Step { spec_step: "8".into(),
            node: IRNode::LetIndex { name: "k".into(), value: Expr::IntConst(0) }},
        // step 9: Repeat, while k < len, …
        Step { spec_step: "9".into(),
            node: IRNode::While {
                cond: Expr::Lt(b(v("k")), b(v("len"))),
                body: vec![
                    Step { spec_step: "9.a".into(),
                        node: IRNode::Let { name: "pk".into(),
                            value: Expr::IndexAsKey(b(v("k"))) }},
                    Step { spec_step: "9.b".into(),
                        node: IRNode::Let { name: "k_present".into(),
                            value: Expr::HasProperty(b(v("o")), b(v("pk"))) }},
                    Step { spec_step: "9.c".into(),
                        node: IRNode::If {
                            cond: v("k_present"),
                            then_body: vec![
                                Step { spec_step: "9.c.i".into(),
                                    node: IRNode::Let { name: "elem".into(),
                                        value: Expr::Get(b(v("o")), b(v("pk"))) }},
                                Step { spec_step: "9.c.ii".into(),
                                    node: IRNode::If {
                                        cond: Expr::StrictEq(b(v("search_element")), b(v("elem"))),
                                        then_body: vec![Step { spec_step: "9.c.ii.1".into(),
                                            node: IRNode::Return(Expr::IndexAsValue(b(v("k")))) }],
                                        else_body: vec![],
                                    }},
                            ],
                            else_body: vec![],
                        }},
                    Step { spec_step: "9.d".into(),
                        node: IRNode::AssignIndex { name: "k".into(),
                            value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))) }},
                ],
            }},
        Step { spec_step: "10".into(),
            node: IRNode::Return(Expr::Number(-1.0)) },
    ];
    IRFunction {
        spec_section: "23.1.3.16".into(),
        rust_name: "array_prototype_index_of".into(),
        title: "Array.prototype.indexOf ( searchElement [ , fromIndex ] )".into(),
        body,
    }
}

// ──────────────── §23.1.3.13 includes ────────────────

pub fn build_includes() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.searchElement".into(),
            node: IRNode::Let { name: "search_element".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "o".into(), value: Expr::ToObject(b(Expr::This)) }},
        Step { spec_step: "2".into(),
            node: IRNode::LetIndex { name: "len".into(),
                value: Expr::LengthOfArrayLike(b(v("o"))) }},
        Step { spec_step: "3".into(),
            node: IRNode::If {
                cond: Expr::Not(b(Expr::Lt(b(Expr::IntConst(0)), b(v("len"))))),
                then_body: vec![Step { spec_step: "3.return".into(),
                    node: IRNode::Return(Expr::Bool(false)) }],
                else_body: vec![],
            }},
        Step { spec_step: "8".into(),
            node: IRNode::LetIndex { name: "k".into(), value: Expr::IntConst(0) }},
        Step { spec_step: "9".into(),
            node: IRNode::While {
                cond: Expr::Lt(b(v("k")), b(v("len"))),
                body: vec![
                    // includes uses Get (not HasProperty-gated) — every
                    // index is read per §23.1.3.13.
                    Step { spec_step: "9.a".into(),
                        node: IRNode::Let { name: "pk".into(),
                            value: Expr::IndexAsKey(b(v("k"))) }},
                    Step { spec_step: "9.b".into(),
                        node: IRNode::Let { name: "elem".into(),
                            value: Expr::Get(b(v("o")), b(v("pk"))) }},
                    Step { spec_step: "9.c".into(),
                        node: IRNode::If {
                            cond: Expr::SameValueZero(b(v("search_element")), b(v("elem"))),
                            then_body: vec![Step { spec_step: "9.c.i".into(),
                                node: IRNode::Return(Expr::Bool(true)) }],
                            else_body: vec![],
                        }},
                    Step { spec_step: "9.d".into(),
                        node: IRNode::AssignIndex { name: "k".into(),
                            value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))) }},
                ],
            }},
        Step { spec_step: "10".into(),
            node: IRNode::Return(Expr::Bool(false)) },
    ];
    IRFunction {
        spec_section: "23.1.3.13".into(),
        rust_name: "array_prototype_includes".into(),
        title: "Array.prototype.includes ( searchElement [ , fromIndex ] )".into(),
        body,
    }
}

// ──────────────── linter records ────────────────

pub fn spec_steps_index_of() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(),   abstract_ops: vec!["ToObject"],          throws: None, prose: "Let O be ? ToObject(this value)." },
        SpecStepRecord { step_id: "2".into(),   abstract_ops: vec!["LengthOfArrayLike"], throws: None, prose: "Let len be ? LengthOfArrayLike(O)." },
        SpecStepRecord { step_id: "3".into(),   abstract_ops: vec![],                    throws: None, prose: "If len = 0, return -1𝔽." },
        SpecStepRecord { step_id: "3.return".into(), abstract_ops: vec![],               throws: None, prose: "Return -1𝔽." },
        SpecStepRecord { step_id: "8".into(),   abstract_ops: vec![],                    throws: None, prose: "Let k be 0." },
        SpecStepRecord { step_id: "9".into(),   abstract_ops: vec![],                    throws: None, prose: "Repeat, while k < len, …" },
        SpecStepRecord { step_id: "9.a".into(), abstract_ops: vec!["ToString"],          throws: None, prose: "Let Pk be ! ToString(𝔽(k))." },
        SpecStepRecord { step_id: "9.b".into(), abstract_ops: vec!["HasProperty"],       throws: None, prose: "Let kPresent be ? HasProperty(O, Pk)." },
        SpecStepRecord { step_id: "9.c".into(), abstract_ops: vec![],                    throws: None, prose: "If kPresent is true, then …" },
        SpecStepRecord { step_id: "9.c.i".into(), abstract_ops: vec!["Get"],             throws: None, prose: "Let elementK be ? Get(O, Pk)." },
        SpecStepRecord { step_id: "9.c.ii".into(),abstract_ops: vec![],                  throws: None, prose: "If IsStrictlyEqual(searchElement, elementK) is true, return 𝔽(k)." },
        SpecStepRecord { step_id: "9.c.ii.1".into(),abstract_ops: vec![],                throws: None, prose: "Return 𝔽(k)." },
        SpecStepRecord { step_id: "9.d".into(), abstract_ops: vec![],                    throws: None, prose: "Set k to k + 1." },
        SpecStepRecord { step_id: "10".into(),  abstract_ops: vec![],                    throws: None, prose: "Return -1𝔽." },
    ]
}

pub fn spec_steps_includes() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(),   abstract_ops: vec!["ToObject"],          throws: None, prose: "Let O be ? ToObject(this value)." },
        SpecStepRecord { step_id: "2".into(),   abstract_ops: vec!["LengthOfArrayLike"], throws: None, prose: "Let len be ? LengthOfArrayLike(O)." },
        SpecStepRecord { step_id: "3".into(),   abstract_ops: vec![],                    throws: None, prose: "If len = 0, return false." },
        SpecStepRecord { step_id: "3.return".into(), abstract_ops: vec![],               throws: None, prose: "Return false." },
        SpecStepRecord { step_id: "8".into(),   abstract_ops: vec![],                    throws: None, prose: "Let k be 0." },
        SpecStepRecord { step_id: "9".into(),   abstract_ops: vec![],                    throws: None, prose: "Repeat, while k < len, …" },
        SpecStepRecord { step_id: "9.a".into(), abstract_ops: vec!["ToString"],          throws: None, prose: "Let Pk be ! ToString(𝔽(k))." },
        SpecStepRecord { step_id: "9.b".into(), abstract_ops: vec!["Get"],               throws: None, prose: "Let elementK be ? Get(O, Pk)." },
        SpecStepRecord { step_id: "9.c".into(), abstract_ops: vec!["SameValueZero"],     throws: None, prose: "If SameValueZero(searchElement, elementK) is true, return true." },
        SpecStepRecord { step_id: "9.c.i".into(), abstract_ops: vec![],                  throws: None, prose: "Return true." },
        SpecStepRecord { step_id: "9.d".into(), abstract_ops: vec![],                    throws: None, prose: "Set k to k + 1." },
        SpecStepRecord { step_id: "10".into(),  abstract_ops: vec![],                    throws: None, prose: "Return false." },
    ]
}
