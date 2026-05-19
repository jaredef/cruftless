//! ECMA-262 §23.1.3.24 — Array.prototype.reduce ( callbackfn [ , initialValue ] ).
//!
//! Iteration with accumulator. Spec distinguishes "initialValue present"
//! from "not present"; the latter must seed from the first present index
//! (and throws TypeError if no present indices exist).
//!
//! Tier 1.8 added Expr::HasArg(i) so the IR can faithfully model the
//! "initialValue is present" check at step 4 + step 6/7 fork.

use crate::ir::{ErrorClass, Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn b(e: Expr) -> Box<Expr> { Box::new(e) }
fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

/// Construct §23.1.3.24 reduce.
pub fn build_reduce() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.callbackfn".into(),
            node: IRNode::Let { name: "callbackfn".into(), value: Expr::Arg(0) }},
        // step 1
        Step { spec_step: "1".into(),
            node: IRNode::Let { name: "o".into(), value: Expr::ToObject(b(Expr::This)) }},
        // step 2
        Step { spec_step: "2".into(),
            node: IRNode::LetIndex { name: "len".into(),
                value: Expr::LengthOfArrayLike(b(v("o"))) }},
        // step 3: callable check
        Step { spec_step: "3".into(),
            node: IRNode::If {
                cond: Expr::Not(b(Expr::IsCallable(b(v("callbackfn"))))),
                then_body: vec![Step { spec_step: "3.throw".into(),
                    node: IRNode::Throw { class: ErrorClass::TypeError,
                        message: "Array.prototype.reduce: callback is not callable".into() }}],
                else_body: vec![],
            }},
        // step 4: If len = 0 and initialValue is not present, throw TypeError.
        Step { spec_step: "4".into(),
            node: IRNode::If {
                // (len == 0) && !HasArg(1)
                cond: Expr::Not(b(Expr::Lt(b(Expr::IntConst(0)), b(v("len"))))),
                then_body: vec![Step { spec_step: "4.guard".into(),
                    node: IRNode::If {
                        cond: Expr::Not(b(Expr::HasArg(1))),
                        then_body: vec![Step { spec_step: "4.throw".into(),
                            node: IRNode::Throw { class: ErrorClass::TypeError,
                                message: "Reduce of empty array with no initial value".into() }}],
                        else_body: vec![],
                    }}],
                else_body: vec![],
            }},
        // step 5: Let k be 0.
        Step { spec_step: "5".into(),
            node: IRNode::LetIndex { name: "k".into(), value: Expr::IntConst(0) }},
        // step 6/7 fork.
        Step { spec_step: "6".into(),
            node: IRNode::Let { name: "mut accumulator".into(),
                value: Expr::Undefined }},
        Step { spec_step: "7".into(),
            node: IRNode::If {
                cond: Expr::HasArg(1),
                then_body: vec![Step { spec_step: "7.a".into(),
                    node: IRNode::Assign { name: "accumulator".into(),
                        value: Expr::Arg(1) }}],
                else_body: vec![Step { spec_step: "7.b".into(),
                    // Find first present index; seed accumulator; advance k.
                    // Tier-1.8: cruftless arrays are dense at indices 0..len in
                    // practice, so a single Get at k=0 is sufficient for the
                    // common case. A faithful version would loop until
                    // HasProperty returns true.
                    node: IRNode::If {
                        cond: Expr::Lt(b(v("k")), b(v("len"))),
                        then_body: vec![
                            Step { spec_step: "7.b.iv".into(),
                                node: IRNode::Assign { name: "accumulator".into(),
                                    value: Expr::Get(b(v("o")),
                                        b(Expr::IndexAsKey(b(v("k"))))) }},
                            Step { spec_step: "7.b.v".into(),
                                node: IRNode::AssignIndex { name: "k".into(),
                                    value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))) }},
                        ],
                        else_body: vec![],
                    }}],
            }},
        // step 8: Repeat, while k < len, …
        Step { spec_step: "8".into(),
            node: IRNode::While {
                cond: Expr::Lt(b(v("k")), b(v("len"))),
                body: vec![
                    Step { spec_step: "8.a".into(),
                        node: IRNode::Let { name: "pk".into(),
                            value: Expr::IndexAsKey(b(v("k"))) }},
                    Step { spec_step: "8.b".into(),
                        node: IRNode::If {
                            cond: Expr::HasProperty(b(v("o")), b(v("pk"))),
                            then_body: vec![
                                Step { spec_step: "8.c.i".into(),
                                    node: IRNode::Let { name: "k_value".into(),
                                        value: Expr::Get(b(v("o")), b(v("pk"))) }},
                                Step { spec_step: "8.c.ii".into(),
                                    node: IRNode::Assign { name: "accumulator".into(),
                                        value: Expr::Call {
                                            function: b(v("callbackfn")),
                                            this: b(Expr::Undefined),
                                            args: vec![
                                                v("accumulator"),
                                                v("k_value"),
                                                Expr::IndexAsValue(b(v("k"))),
                                                v("o"),
                                            ],
                                        } }},
                            ],
                            else_body: vec![],
                        }},
                    Step { spec_step: "8.d".into(),
                        node: IRNode::AssignIndex { name: "k".into(),
                            value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))) }},
                ],
            }},
        // step 9
        Step { spec_step: "9".into(),
            node: IRNode::Return(v("accumulator")) },
    ];
    IRFunction {
        spec_section: "23.1.3.24".into(),
        rust_name: "array_prototype_reduce".into(),
        title: "Array.prototype.reduce ( callbackfn [ , initialValue ] )".into(),
        body,
    }
}

pub fn spec_steps_reduce() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(),     abstract_ops: vec!["ToObject"],          throws: None, prose: "Let O be ? ToObject(this value)." },
        SpecStepRecord { step_id: "2".into(),     abstract_ops: vec!["LengthOfArrayLike"], throws: None, prose: "Let len be ? LengthOfArrayLike(O)." },
        SpecStepRecord { step_id: "3".into(),     abstract_ops: vec!["IsCallable"],        throws: None, prose: "If IsCallable(callbackfn) is false, throw TypeError." },
        SpecStepRecord { step_id: "3.throw".into(),abstract_ops: vec!["Throw"],            throws: Some("TypeError"), prose: "throw a TypeError exception." },
        SpecStepRecord { step_id: "4".into(),     abstract_ops: vec![],                    throws: None, prose: "If len = 0 and initialValue is not present, throw TypeError." },
        SpecStepRecord { step_id: "4.guard".into(),abstract_ops: vec![],                   throws: None, prose: "Inner guard on initialValue absence." },
        SpecStepRecord { step_id: "4.throw".into(),abstract_ops: vec!["Throw"],            throws: Some("TypeError"), prose: "throw a TypeError exception." },
        SpecStepRecord { step_id: "5".into(),     abstract_ops: vec![],                    throws: None, prose: "Let k be 0." },
        SpecStepRecord { step_id: "6".into(),     abstract_ops: vec![],                    throws: None, prose: "Let accumulator be undefined." },
        SpecStepRecord { step_id: "7".into(),     abstract_ops: vec![],                    throws: None, prose: "If initialValue is present, then ... else ..." },
        SpecStepRecord { step_id: "7.a".into(),   abstract_ops: vec![],                    throws: None, prose: "Set accumulator to initialValue." },
        SpecStepRecord { step_id: "7.b".into(),   abstract_ops: vec![],                    throws: None, prose: "Else seed from O[k] and advance k." },
        SpecStepRecord { step_id: "7.b.iv".into(),abstract_ops: vec!["Get"],               throws: None, prose: "Set accumulator to ? Get(O, Pk)." },
        SpecStepRecord { step_id: "7.b.v".into(), abstract_ops: vec![],                    throws: None, prose: "Set k to k + 1." },
        SpecStepRecord { step_id: "8".into(),     abstract_ops: vec![],                    throws: None, prose: "Repeat, while k < len, …" },
        SpecStepRecord { step_id: "8.a".into(),   abstract_ops: vec!["ToString"],          throws: None, prose: "Let Pk be ! ToString(𝔽(k))." },
        SpecStepRecord { step_id: "8.b".into(),   abstract_ops: vec!["HasProperty"],       throws: None, prose: "Let kPresent be ? HasProperty(O, Pk)." },
        SpecStepRecord { step_id: "8.c.i".into(), abstract_ops: vec!["Get"],               throws: None, prose: "Let kValue be ? Get(O, Pk)." },
        SpecStepRecord { step_id: "8.c.ii".into(),abstract_ops: vec!["Call"],              throws: None, prose: "Set accumulator to ? Call(callbackfn, undefined, « accumulator, kValue, 𝔽(k), O »)." },
        SpecStepRecord { step_id: "8.d".into(),   abstract_ops: vec![],                    throws: None, prose: "Set k to k + 1." },
        SpecStepRecord { step_id: "9".into(),     abstract_ops: vec![],                    throws: None, prose: "Return accumulator." },
    ]
}
