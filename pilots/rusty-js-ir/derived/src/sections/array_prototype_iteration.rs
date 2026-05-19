//! ECMA-262 §23.1.3.{8,10,5,29} — Array.prototype.{forEach, filter, every, some}.
//!
//! All four methods share §23.1.3.20 Array.prototype.map's iteration shape:
//!   - ToObject(this), LengthOfArrayLike, IsCallable callback check
//!   - Loop k from 0 to len, with HasProperty / Get / Call sequence per iter
//!   - Differ only in (a) what they do with the callback result and
//!     (b) what they return at the end
//!
//! Hand-translated per the IR-DESIGN.md §3 alphabet. Each section's
//! spec_steps() carries the canonical step records the linter checks.

use crate::ir::{ErrorClass, Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}
fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

/// Shared preamble: steps 1-3 (ToObject + LengthOfArrayLike + IsCallable).
/// All four sections start with this exact sequence.
fn preamble() -> Vec<Step> {
    vec![
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
                name: "this_arg".into(),
                value: Expr::Arg(1),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Let {
                name: "o".into(),
                value: Expr::ToObject(b(Expr::This)),
            },
        },
        Step {
            spec_step: "2".into(),
            node: IRNode::LetIndex {
                name: "len".into(),
                value: Expr::LengthOfArrayLike(b(v("o"))),
            },
        },
        Step {
            spec_step: "3".into(),
            node: IRNode::If {
                cond: Expr::Not(b(Expr::IsCallable(b(v("callbackfn"))))),
                then_body: vec![Step {
                    spec_step: "3.throw".into(),
                    node: IRNode::Throw {
                        class: ErrorClass::TypeError,
                        // Each method overrides this with its own name in
                        // the builder. Default fits forEach.
                        message: "callback is not callable".into(),
                    },
                }],
                else_body: vec![],
            },
        },
    ]
}

/// Standard per-iteration prefix: ToString(k) → HasProperty → Get → Call.
/// Returns the steps; the caller chooses what to do with `mapped`.
fn iter_get_and_call(method_name: &str) -> Vec<Step> {
    let _ = method_name;
    vec![
        Step {
            spec_step: "iter.pk".into(),
            node: IRNode::Let {
                name: "pk".into(),
                value: Expr::IndexAsKey(b(v("k"))),
            },
        },
        Step {
            spec_step: "iter.has".into(),
            node: IRNode::Let {
                name: "k_present".into(),
                value: Expr::ToBoolean(b(Expr::HasProperty(b(v("o")), b(v("pk"))))),
            },
        },
    ]
}

// ──────────────── §23.1.3.8 Array.prototype.forEach ────────────────

pub fn build_for_each() -> IRFunction {
    let mut body = preamble();
    // forEach has no output array; just iterate and discard.
    body.push(Step {
        spec_step: "4".into(),
        node: IRNode::LetIndex {
            name: "k".into(),
            value: Expr::IntConst(0),
        },
    });
    body.push(Step {
        spec_step: "5".into(),
        node: IRNode::While {
            cond: Expr::Lt(b(v("k")), b(v("len"))),
            body: vec![
                Step {
                    spec_step: "5.a".into(),
                    node: IRNode::Let {
                        name: "pk".into(),
                        value: Expr::IndexAsKey(b(v("k"))),
                    },
                },
                Step {
                    spec_step: "5.b".into(),
                    node: IRNode::If {
                        cond: Expr::HasProperty(b(v("o")), b(v("pk"))),
                        then_body: vec![
                            Step {
                                spec_step: "5.c.i".into(),
                                node: IRNode::Let {
                                    name: "k_value".into(),
                                    value: Expr::Get(b(v("o")), b(v("pk"))),
                                },
                            },
                            Step {
                                spec_step: "5.c.ii".into(),
                                node: IRNode::Expr(Expr::Call {
                                    function: b(v("callbackfn")),
                                    this: b(v("this_arg")),
                                    args: vec![
                                        v("k_value"),
                                        Expr::IndexAsValue(b(v("k"))),
                                        v("o"),
                                    ],
                                }),
                            },
                        ],
                        else_body: vec![],
                    },
                },
                Step {
                    spec_step: "5.d".into(),
                    node: IRNode::AssignIndex {
                        name: "k".into(),
                        value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))),
                    },
                },
            ],
        },
    });
    // step 6: Return undefined.
    body.push(Step {
        spec_step: "6".into(),
        node: IRNode::Return(Expr::Undefined),
    });

    // Override step 3's throw message.
    fixup_throw_message(&mut body, "Array.prototype.forEach: callback is not callable");

    IRFunction {
        spec_section: "23.1.3.15".into(),
        rust_name: "array_prototype_for_each".into(),
        title: "Array.prototype.forEach ( callbackfn [ , thisArg ] )".into(),
        body,
    }
}

pub fn spec_steps_for_each() -> Vec<SpecStepRecord> {
    let _ = iter_get_and_call;
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
            abstract_ops: vec![],
            throws: None,
            prose: "Let k be 0.",
        },
        SpecStepRecord {
            step_id: "5".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Repeat, while k < len, …",
        },
        SpecStepRecord {
            step_id: "5.a".into(),
            abstract_ops: vec!["ToString"],
            throws: None,
            prose: "Let Pk be ! ToString(𝔽(k)).",
        },
        SpecStepRecord {
            step_id: "5.b".into(),
            abstract_ops: vec!["HasProperty"],
            throws: None,
            prose: "Let kPresent be ? HasProperty(O, Pk).",
        },
        SpecStepRecord {
            step_id: "5.c.i".into(),
            abstract_ops: vec!["Get"],
            throws: None,
            prose: "Let kValue be ? Get(O, Pk).",
        },
        SpecStepRecord {
            step_id: "5.c.ii".into(),
            abstract_ops: vec!["Call"],
            throws: None,
            prose: "Perform ? Call(callbackfn, thisArg, « kValue, 𝔽(k), O »).",
        },
        SpecStepRecord {
            step_id: "5.d".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Set k to k + 1.",
        },
        SpecStepRecord {
            step_id: "6".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Return undefined.",
        },
    ]
}

// ──────────────── §23.1.3.7 Array.prototype.filter ────────────────

pub fn build_filter() -> IRFunction {
    let mut body = preamble();
    // Filter creates an output array via ArraySpeciesCreate with length 0.
    body.push(Step {
        spec_step: "4".into(),
        node: IRNode::Let {
            name: "a".into(),
            value: Expr::ArraySpeciesCreate {
                o: b(v("o")),
                length: b(Expr::IntConst(0)),
            },
        },
    });
    body.push(Step {
        spec_step: "5".into(),
        node: IRNode::LetIndex {
            name: "k".into(),
            value: Expr::IntConst(0),
        },
    });
    body.push(Step {
        spec_step: "6".into(),
        node: IRNode::LetIndex {
            name: "to".into(),
            value: Expr::IntConst(0),
        },
    });
    body.push(Step {
        spec_step: "7".into(),
        node: IRNode::While {
            cond: Expr::Lt(b(v("k")), b(v("len"))),
            body: vec![
                Step {
                    spec_step: "7.a".into(),
                    node: IRNode::Let {
                        name: "pk".into(),
                        value: Expr::IndexAsKey(b(v("k"))),
                    },
                },
                Step {
                    spec_step: "7.b".into(),
                    node: IRNode::If {
                        cond: Expr::HasProperty(b(v("o")), b(v("pk"))),
                        then_body: vec![
                            Step {
                                spec_step: "7.c.i".into(),
                                node: IRNode::Let {
                                    name: "k_value".into(),
                                    value: Expr::Get(b(v("o")), b(v("pk"))),
                                },
                            },
                            Step {
                                spec_step: "7.c.ii".into(),
                                node: IRNode::Let {
                                    name: "selected".into(),
                                    value: Expr::ToBoolean(b(Expr::Call {
                                        function: b(v("callbackfn")),
                                        this: b(v("this_arg")),
                                        args: vec![
                                            v("k_value"),
                                            Expr::IndexAsValue(b(v("k"))),
                                            v("o"),
                                        ],
                                    })),
                                },
                            },
                            Step {
                                spec_step: "7.c.iii".into(),
                                node: IRNode::If {
                                    // `selected` is already bool-typed (ToBoolean
                                    // lowered above); use directly as cond.
                                    cond: v("selected"),
                                    then_body: vec![
                                        Step {
                                            spec_step: "7.c.iii.1".into(),
                                            node: IRNode::Expr(Expr::CreateDataPropertyOrThrow(
                                                b(v("a")),
                                                b(Expr::IndexAsKey(b(v("to")))),
                                                b(v("k_value")),
                                            )),
                                        },
                                        Step {
                                            spec_step: "7.c.iii.2".into(),
                                            node: IRNode::AssignIndex {
                                                name: "to".into(),
                                                value: Expr::IndexAdd(b(v("to")), b(Expr::IntConst(1))),
                                            },
                                        },
                                    ],
                                    else_body: vec![],
                                },
                            },
                        ],
                        else_body: vec![],
                    },
                },
                Step {
                    spec_step: "7.d".into(),
                    node: IRNode::AssignIndex {
                        name: "k".into(),
                        value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))),
                    },
                },
            ],
        },
    });
    body.push(Step {
        spec_step: "8".into(),
        node: IRNode::Return(v("a")),
    });

    fixup_throw_message(&mut body, "Array.prototype.filter: callback is not callable");

    IRFunction {
        spec_section: "23.1.3.7".into(),
        rust_name: "array_prototype_filter".into(),
        title: "Array.prototype.filter ( callbackfn [ , thisArg ] )".into(),
        body,
    }
}

pub fn spec_steps_filter() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(),     abstract_ops: vec!["ToObject"],            throws: None,             prose: "Let O be ? ToObject(this value)." },
        SpecStepRecord { step_id: "2".into(),     abstract_ops: vec!["LengthOfArrayLike"],   throws: None,             prose: "Let len be ? LengthOfArrayLike(O)." },
        SpecStepRecord { step_id: "3".into(),     abstract_ops: vec!["IsCallable"],          throws: None,             prose: "If IsCallable(callbackfn) is false, throw TypeError." },
        SpecStepRecord { step_id: "3.throw".into(),abstract_ops: vec!["Throw"],              throws: Some("TypeError"),prose: "throw a TypeError exception." },
        SpecStepRecord { step_id: "4".into(),     abstract_ops: vec!["ArraySpeciesCreate"],  throws: None,             prose: "Let A be ? ArraySpeciesCreate(O, 0)." },
        SpecStepRecord { step_id: "5".into(),     abstract_ops: vec![],                      throws: None,             prose: "Let k be 0." },
        SpecStepRecord { step_id: "6".into(),     abstract_ops: vec![],                      throws: None,             prose: "Let to be 0." },
        SpecStepRecord { step_id: "7".into(),     abstract_ops: vec![],                      throws: None,             prose: "Repeat, while k < len, …" },
        SpecStepRecord { step_id: "7.a".into(),   abstract_ops: vec!["ToString"],            throws: None,             prose: "Let Pk be ! ToString(𝔽(k))." },
        SpecStepRecord { step_id: "7.b".into(),   abstract_ops: vec!["HasProperty"],         throws: None,             prose: "Let kPresent be ? HasProperty(O, Pk)." },
        SpecStepRecord { step_id: "7.c.i".into(), abstract_ops: vec!["Get"],                 throws: None,             prose: "Let kValue be ? Get(O, Pk)." },
        SpecStepRecord { step_id: "7.c.ii".into(),abstract_ops: vec!["Call", "ToBoolean"],   throws: None,             prose: "Let selected be ToBoolean(? Call(callbackfn, thisArg, « kValue, 𝔽(k), O »))." },
        SpecStepRecord { step_id: "7.c.iii".into(),abstract_ops: vec![],                     throws: None,             prose: "If selected is true, then …" },
        SpecStepRecord { step_id: "7.c.iii.1".into(),abstract_ops: vec!["CreateDataPropertyOrThrow"],throws:None,      prose: "Perform ? CreateDataPropertyOrThrow(A, ! ToString(𝔽(to)), kValue)." },
        SpecStepRecord { step_id: "7.c.iii.2".into(),abstract_ops: vec![],                   throws: None,             prose: "Set to to to + 1." },
        SpecStepRecord { step_id: "7.d".into(),   abstract_ops: vec![],                      throws: None,             prose: "Set k to k + 1." },
        SpecStepRecord { step_id: "8".into(),     abstract_ops: vec![],                      throws: None,             prose: "Return A." },
    ]
}

// ──────────────── §23.1.3.6 Array.prototype.every ────────────────

pub fn build_every() -> IRFunction {
    let mut body = preamble();
    body.push(Step {
        spec_step: "4".into(),
        node: IRNode::LetIndex {
            name: "k".into(),
            value: Expr::IntConst(0),
        },
    });
    body.push(Step {
        spec_step: "5".into(),
        node: IRNode::While {
            cond: Expr::Lt(b(v("k")), b(v("len"))),
            body: vec![
                Step {
                    spec_step: "5.a".into(),
                    node: IRNode::Let {
                        name: "pk".into(),
                        value: Expr::IndexAsKey(b(v("k"))),
                    },
                },
                Step {
                    spec_step: "5.b".into(),
                    node: IRNode::If {
                        cond: Expr::HasProperty(b(v("o")), b(v("pk"))),
                        then_body: vec![
                            Step {
                                spec_step: "5.c.i".into(),
                                node: IRNode::Let {
                                    name: "k_value".into(),
                                    value: Expr::Get(b(v("o")), b(v("pk"))),
                                },
                            },
                            Step {
                                spec_step: "5.c.ii".into(),
                                node: IRNode::Let {
                                    name: "test_result".into(),
                                    value: Expr::ToBoolean(b(Expr::Call {
                                        function: b(v("callbackfn")),
                                        this: b(v("this_arg")),
                                        args: vec![
                                            v("k_value"),
                                            Expr::IndexAsValue(b(v("k"))),
                                            v("o"),
                                        ],
                                    })),
                                },
                            },
                            Step {
                                spec_step: "5.c.iii".into(),
                                node: IRNode::If {
                                    cond: Expr::Not(b(v("test_result"))),
                                    then_body: vec![Step {
                                        spec_step: "5.c.iii.1".into(),
                                        node: IRNode::Return(Expr::Bool(false)),
                                    }],
                                    else_body: vec![],
                                },
                            },
                        ],
                        else_body: vec![],
                    },
                },
                Step {
                    spec_step: "5.d".into(),
                    node: IRNode::AssignIndex {
                        name: "k".into(),
                        value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))),
                    },
                },
            ],
        },
    });
    body.push(Step {
        spec_step: "6".into(),
        node: IRNode::Return(Expr::Bool(true)),
    });

    fixup_throw_message(&mut body, "Array.prototype.every: callback is not callable");

    IRFunction {
        spec_section: "23.1.3.6".into(),
        rust_name: "array_prototype_every".into(),
        title: "Array.prototype.every ( callbackfn [ , thisArg ] )".into(),
        body,
    }
}

pub fn spec_steps_every() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(),     abstract_ops: vec!["ToObject"],          throws: None, prose: "Let O be ? ToObject(this value)." },
        SpecStepRecord { step_id: "2".into(),     abstract_ops: vec!["LengthOfArrayLike"], throws: None, prose: "Let len be ? LengthOfArrayLike(O)." },
        SpecStepRecord { step_id: "3".into(),     abstract_ops: vec!["IsCallable"],        throws: None, prose: "If IsCallable(callbackfn) is false, throw TypeError." },
        SpecStepRecord { step_id: "3.throw".into(),abstract_ops: vec!["Throw"],            throws: Some("TypeError"), prose: "throw a TypeError exception." },
        SpecStepRecord { step_id: "4".into(),     abstract_ops: vec![],                    throws: None, prose: "Let k be 0." },
        SpecStepRecord { step_id: "5".into(),     abstract_ops: vec![],                    throws: None, prose: "Repeat, while k < len, …" },
        SpecStepRecord { step_id: "5.a".into(),   abstract_ops: vec!["ToString"],          throws: None, prose: "Let Pk be ! ToString(𝔽(k))." },
        SpecStepRecord { step_id: "5.b".into(),   abstract_ops: vec!["HasProperty"],       throws: None, prose: "Let kPresent be ? HasProperty(O, Pk)." },
        SpecStepRecord { step_id: "5.c.i".into(), abstract_ops: vec!["Get"],               throws: None, prose: "Let kValue be ? Get(O, Pk)." },
        SpecStepRecord { step_id: "5.c.ii".into(),abstract_ops: vec!["Call", "ToBoolean"], throws: None, prose: "Let testResult be ToBoolean(? Call(callbackfn, thisArg, « kValue, 𝔽(k), O »))." },
        SpecStepRecord { step_id: "5.c.iii".into(),abstract_ops: vec![],                   throws: None, prose: "If testResult is false, return false." },
        SpecStepRecord { step_id: "5.c.iii.1".into(),abstract_ops: vec![],                 throws: None, prose: "Return false." },
        SpecStepRecord { step_id: "5.d".into(),   abstract_ops: vec![],                    throws: None, prose: "Set k to k + 1." },
        SpecStepRecord { step_id: "6".into(),     abstract_ops: vec![],                    throws: None, prose: "Return true." },
    ]
}

// ──────────────── §23.1.3.29 Array.prototype.some ────────────────

pub fn build_some() -> IRFunction {
    let mut body = preamble();
    body.push(Step {
        spec_step: "4".into(),
        node: IRNode::LetIndex {
            name: "k".into(),
            value: Expr::IntConst(0),
        },
    });
    body.push(Step {
        spec_step: "5".into(),
        node: IRNode::While {
            cond: Expr::Lt(b(v("k")), b(v("len"))),
            body: vec![
                Step {
                    spec_step: "5.a".into(),
                    node: IRNode::Let {
                        name: "pk".into(),
                        value: Expr::IndexAsKey(b(v("k"))),
                    },
                },
                Step {
                    spec_step: "5.b".into(),
                    node: IRNode::If {
                        cond: Expr::HasProperty(b(v("o")), b(v("pk"))),
                        then_body: vec![
                            Step {
                                spec_step: "5.c.i".into(),
                                node: IRNode::Let {
                                    name: "k_value".into(),
                                    value: Expr::Get(b(v("o")), b(v("pk"))),
                                },
                            },
                            Step {
                                spec_step: "5.c.ii".into(),
                                node: IRNode::Let {
                                    name: "test_result".into(),
                                    value: Expr::ToBoolean(b(Expr::Call {
                                        function: b(v("callbackfn")),
                                        this: b(v("this_arg")),
                                        args: vec![
                                            v("k_value"),
                                            Expr::IndexAsValue(b(v("k"))),
                                            v("o"),
                                        ],
                                    })),
                                },
                            },
                            Step {
                                spec_step: "5.c.iii".into(),
                                node: IRNode::If {
                                    cond: v("test_result"),
                                    then_body: vec![Step {
                                        spec_step: "5.c.iii.1".into(),
                                        node: IRNode::Return(Expr::Bool(true)),
                                    }],
                                    else_body: vec![],
                                },
                            },
                        ],
                        else_body: vec![],
                    },
                },
                Step {
                    spec_step: "5.d".into(),
                    node: IRNode::AssignIndex {
                        name: "k".into(),
                        value: Expr::IndexAdd(b(v("k")), b(Expr::IntConst(1))),
                    },
                },
            ],
        },
    });
    body.push(Step {
        spec_step: "6".into(),
        node: IRNode::Return(Expr::Bool(false)),
    });

    fixup_throw_message(&mut body, "Array.prototype.some: callback is not callable");

    IRFunction {
        spec_section: "23.1.3.29".into(),
        rust_name: "array_prototype_some".into(),
        title: "Array.prototype.some ( callbackfn [ , thisArg ] )".into(),
        body,
    }
}

pub fn spec_steps_some() -> Vec<SpecStepRecord> {
    // Same as every but returns true on first truthy, false at end.
    vec![
        SpecStepRecord { step_id: "1".into(),     abstract_ops: vec!["ToObject"],          throws: None, prose: "Let O be ? ToObject(this value)." },
        SpecStepRecord { step_id: "2".into(),     abstract_ops: vec!["LengthOfArrayLike"], throws: None, prose: "Let len be ? LengthOfArrayLike(O)." },
        SpecStepRecord { step_id: "3".into(),     abstract_ops: vec!["IsCallable"],        throws: None, prose: "If IsCallable(callbackfn) is false, throw TypeError." },
        SpecStepRecord { step_id: "3.throw".into(),abstract_ops: vec!["Throw"],            throws: Some("TypeError"), prose: "throw a TypeError exception." },
        SpecStepRecord { step_id: "4".into(),     abstract_ops: vec![],                    throws: None, prose: "Let k be 0." },
        SpecStepRecord { step_id: "5".into(),     abstract_ops: vec![],                    throws: None, prose: "Repeat, while k < len, …" },
        SpecStepRecord { step_id: "5.a".into(),   abstract_ops: vec!["ToString"],          throws: None, prose: "Let Pk be ! ToString(𝔽(k))." },
        SpecStepRecord { step_id: "5.b".into(),   abstract_ops: vec!["HasProperty"],       throws: None, prose: "Let kPresent be ? HasProperty(O, Pk)." },
        SpecStepRecord { step_id: "5.c.i".into(), abstract_ops: vec!["Get"],               throws: None, prose: "Let kValue be ? Get(O, Pk)." },
        SpecStepRecord { step_id: "5.c.ii".into(),abstract_ops: vec!["Call", "ToBoolean"], throws: None, prose: "Let testResult be ToBoolean(? Call(callbackfn, thisArg, « kValue, 𝔽(k), O »))." },
        SpecStepRecord { step_id: "5.c.iii".into(),abstract_ops: vec![],                   throws: None, prose: "If testResult is true, return true." },
        SpecStepRecord { step_id: "5.c.iii.1".into(),abstract_ops: vec![],                 throws: None, prose: "Return true." },
        SpecStepRecord { step_id: "5.d".into(),   abstract_ops: vec![],                    throws: None, prose: "Set k to k + 1." },
        SpecStepRecord { step_id: "6".into(),     abstract_ops: vec![],                    throws: None, prose: "Return false." },
    ]
}

// ──────────────── helper ────────────────

/// Replace the first Throw step's message with `msg`. Used by each section
/// to customize the §A8.31 SyntaxError canonical error message.
fn fixup_throw_message(body: &mut Vec<Step>, msg: &str) {
    fn walk(steps: &mut Vec<Step>, msg: &str) -> bool {
        for s in steps {
            match &mut s.node {
                IRNode::Throw { message, .. } => {
                    *message = msg.to_string();
                    return true;
                }
                IRNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    if walk(then_body, msg) {
                        return true;
                    }
                    if walk(else_body, msg) {
                        return true;
                    }
                }
                IRNode::While { body, .. } => {
                    if walk(body, msg) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    walk(body, msg);
}
