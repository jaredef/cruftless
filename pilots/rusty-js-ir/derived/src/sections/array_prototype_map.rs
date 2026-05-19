//! ECMA-262 §23.1.3.20 — Array.prototype.map ( callbackfn [ , thisArg ] )
//!
//! Hand-translated per IR-DESIGN.md §4. Each spec step has exactly one IR
//! statement; spec-step IDs are preserved verbatim for the linter.

use crate::ir::{ErrorClass, Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

/// Construct the IR function for §23.1.3.20.
pub fn build() -> IRFunction {
    let body = vec![
        // Parameter binding (spec convention).
        Step {
            spec_step: "param.callbackfn".into(),
            node: IRNode::Let {
                name: "callbackfn".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.thisArg".into(),
            node: IRNode::Let {
                name: "thisArg".into(),
                value: Expr::Arg(1),
            },
        },
        // step 1: Let O be ? ToObject(this value).
        Step {
            spec_step: "1".into(),
            node: IRNode::Let {
                name: "o".into(),
                value: Expr::ToObject(b(v("this"))),
            },
        },
        // step 2: Let len be ? LengthOfArrayLike(O).
        Step {
            spec_step: "2".into(),
            node: IRNode::Let {
                name: "len".into(),
                value: Expr::LengthOfArrayLike(b(v("o"))),
            },
        },
        // step 3: If IsCallable(callbackfn) is false, throw a TypeError exception.
        Step {
            spec_step: "3".into(),
            node: IRNode::If {
                cond: Expr::Not(b(Expr::IsCallable(b(v("callbackfn"))))),
                then_body: vec![Step {
                    spec_step: "3.throw".into(),
                    node: IRNode::Throw {
                        class: ErrorClass::TypeError,
                        message: "Array.prototype.map: callback is not callable".into(),
                    },
                }],
                else_body: vec![],
            },
        },
        // step 4: Let A be ? ArraySpeciesCreate(O, len).
        Step {
            spec_step: "4".into(),
            node: IRNode::Let {
                name: "a".into(),
                value: Expr::ArraySpeciesCreate {
                    o: b(v("o")),
                    length: b(v("len")),
                },
            },
        },
        // step 5: Let k be 0.
        Step {
            spec_step: "5".into(),
            node: IRNode::Let {
                name: "mut k".into(),
                value: Expr::Number(0.0),
            },
        },
        // step 6: Repeat, while k < len, …
        Step {
            spec_step: "6".into(),
            node: IRNode::While {
                cond: Expr::Lt(b(v("k")), b(v("len"))),
                body: vec![
                    // step 6.a: Let Pk be ! ToString(𝔽(k)).
                    Step {
                        spec_step: "6.a".into(),
                        node: IRNode::Let {
                            name: "pk".into(),
                            value: Expr::ToString(b(v("k"))),
                        },
                    },
                    // step 6.b/6.c: If HasProperty(O, Pk) is true, then …
                    Step {
                        spec_step: "6.b".into(),
                        node: IRNode::If {
                            cond: Expr::HasProperty(b(v("o")), b(v("pk"))),
                            then_body: vec![
                                // step 6.c.i: Let kValue be ? Get(O, Pk).
                                Step {
                                    spec_step: "6.c.i".into(),
                                    node: IRNode::Let {
                                        name: "k_value".into(),
                                        value: Expr::Get(b(v("o")), b(v("pk"))),
                                    },
                                },
                                // step 6.c.ii: Let mappedValue be ? Call(callbackfn, thisArg, « kValue, 𝔽(k), O »).
                                Step {
                                    spec_step: "6.c.ii".into(),
                                    node: IRNode::Let {
                                        name: "mapped".into(),
                                        value: Expr::Call {
                                            function: b(v("callbackfn")),
                                            this: b(v("thisArg")),
                                            args: vec![
                                                v("k_value"),
                                                v("k"),
                                                v("o"),
                                            ],
                                        },
                                    },
                                },
                                // step 6.c.iii: Perform ? CreateDataPropertyOrThrow(A, Pk, mappedValue).
                                Step {
                                    spec_step: "6.c.iii".into(),
                                    node: IRNode::Expr(Expr::CreateDataPropertyOrThrow(
                                        b(v("a")),
                                        b(v("pk")),
                                        b(v("mapped")),
                                    )),
                                },
                            ],
                            else_body: vec![],
                        },
                    },
                    // step 6.d: Set k to k + 1.
                    Step {
                        spec_step: "6.d".into(),
                        node: IRNode::Assign {
                            name: "k".into(),
                            value: Expr::OpAdd(b(v("k")), b(Expr::Number(1.0))),
                        },
                    },
                ],
            },
        },
        // step 7: Return A.
        Step {
            spec_step: "7".into(),
            node: IRNode::Return(v("a")),
        },
    ];

    IRFunction {
        spec_section: "23.1.3.20".into(),
        rust_name: "array_prototype_map".into(),
        title: "Array.prototype.map ( callbackfn [ , thisArg ] )".into(),
        body,
    }
}

/// ECMA-262 §23.1.3.20 algorithm steps as canonical records — the linter
/// input. In Tier 2 this will be auto-derived from `<emu-alg>` XML.
pub fn spec_steps() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord {
            step_id: "1".into(),
            abstract_ops: vec!["ToObject"],
            throws: None,
            prose: "Let O be ? ToObject(this value).",
        },
        SpecStepRecord {
            step_id: "2".into(),
            abstract_ops: vec!["LengthOfArrayLike"],
            throws: None,
            prose: "Let len be ? LengthOfArrayLike(O).",
        },
        SpecStepRecord {
            step_id: "3".into(),
            abstract_ops: vec!["IsCallable"],
            throws: None,
            prose: "If IsCallable(callbackfn) is false, throw a TypeError exception.",
        },
        SpecStepRecord {
            step_id: "3.throw".into(),
            abstract_ops: vec!["Throw"],
            throws: Some("TypeError"),
            prose: "throw a TypeError exception (step 3 consequent).",
        },
        SpecStepRecord {
            step_id: "4".into(),
            abstract_ops: vec!["ArraySpeciesCreate"],
            throws: None,
            prose: "Let A be ? ArraySpeciesCreate(O, len).",
        },
        SpecStepRecord {
            step_id: "5".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Let k be 0.",
        },
        SpecStepRecord {
            step_id: "6".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Repeat, while k < len, …",
        },
        SpecStepRecord {
            step_id: "6.a".into(),
            abstract_ops: vec!["ToString"],
            throws: None,
            prose: "Let Pk be ! ToString(𝔽(k)).",
        },
        SpecStepRecord {
            step_id: "6.b".into(),
            abstract_ops: vec!["HasProperty"],
            throws: None,
            prose: "Let kPresent be ? HasProperty(O, Pk). If kPresent is true, then …",
        },
        SpecStepRecord {
            step_id: "6.c.i".into(),
            abstract_ops: vec!["Get"],
            throws: None,
            prose: "Let kValue be ? Get(O, Pk).",
        },
        SpecStepRecord {
            step_id: "6.c.ii".into(),
            abstract_ops: vec!["Call"],
            throws: None,
            prose: "Let mappedValue be ? Call(callbackfn, thisArg, « kValue, 𝔽(k), O »).",
        },
        SpecStepRecord {
            step_id: "6.c.iii".into(),
            abstract_ops: vec!["CreateDataPropertyOrThrow"],
            throws: None,
            prose: "Perform ? CreateDataPropertyOrThrow(A, Pk, mappedValue).",
        },
        SpecStepRecord {
            step_id: "6.d".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Set k to k + 1.",
        },
        SpecStepRecord {
            step_id: "7".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Return A.",
        },
    ]
}
