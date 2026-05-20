//! ECMA-262 §7.1.1 ToPrimitive(input, preferredType) + §7.1.1.1
//! OrdinaryToPrimitive — the receiver-coercion dispatcher at the
//! center of stringification, numification, and loose-equality.
//!
//! IR-EXT 72 (resolver-instance lift, per keeper conjecture msg 8556):
//! lifting this into IR makes the dispatch sequence (@@toPrimitive →
//! toString → valueOf, or → valueOf → toString depending on hint)
//! legible at the IR-pinning tier. Any future divergence at adjacent
//! coercion steps becomes traceable through the spec-step trace
//! rather than buried in Rust control flow.

use crate::ir::{ErrorClass, Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }
fn b(e: Expr) -> Box<Expr> { Box::new(e) }

pub fn build_to_primitive() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.value".into(), node: IRNode::Let { name: "value".into(), value: Expr::Arg(0) } },
        Step { spec_step: "param.hint".into(),  node: IRNode::Let { name: "hint".into(),  value: Expr::Arg(1) } },

        // §7.1.1 step 1: if Type(input) is not Object, return input.
        // IR-EXT 95 (Doc 730 §XIII alphabet promotion): the prior
        // implementation discriminated via Expr::TypeOf(value) compared
        // against "object" and "function" string literals. That pair
        // collapsed spec-Null (typeof "object", spec Type Null) into the
        // spec-Object branch, so ToPrimitive(null) failed to short-circuit
        // here and walked the @@toPrimitive / toString / valueOf chain
        // (none defined on null), reaching step 6's throw on `${null}`
        // template coercion. IsSpecObject is the Tier-1.5 typed primitive
        // that names spec Type(V) === Object directly, excluding Null /
        // Undefined / all primitives by construction. The collapse is
        // resolved at the alphabet boundary instead of by ad-hoc
        // additional string comparisons.
        Step { spec_step: "1.fast".into(), node: IRNode::If {
            cond: Expr::Not(b(Expr::IsSpecObject(b(v("value"))))),
            then_body: vec![
                Step { spec_step: "1.return".into(), node: IRNode::Return(v("value")) },
            ],
            else_body: vec![],
        }},

        // §7.1.1 step 2.a: exoticToPrim = ? GetMethod(input, @@toPrimitive).
        Step { spec_step: "2.a.lookup".into(), node: IRNode::Let {
            name: "exotic".into(),
            // EXT 85 / Tier-1.5: spec step 2.a is literally `GetMethod`.
            // The IR primitive Expr::GetMethod (EXT 85) handles the
            // §7.3.10 post-condition (callable-or-undefined-or-throw)
            // explicitly — including the case where @@toPrimitive is
            // defined-but-non-callable (which the IsCallable check
            // below silently treats as "no exotic" instead of throwing
            // TypeError per spec).
            value: Expr::GetMethod(b(v("value")), b(Expr::Str("@@toPrimitive".into()))),
        }},
        // §7.1.1 step 2.b: if exoticToPrim is not undefined.
        Step { spec_step: "2.b.has_exotic".into(), node: IRNode::If {
            cond: Expr::IsCallable(b(v("exotic"))),
            then_body: vec![
                // §7.1.1 step 2.b.i: result = ? Call(exoticToPrim, input, [hint]).
                Step { spec_step: "2.b.i.call".into(), node: IRNode::Let {
                    name: "result".into(),
                    value: Expr::Call {
                        function: b(v("exotic")),
                        this: b(v("value")),
                        args: vec![v("hint")],
                    },
                }},
                // §7.1.1 step 2.b.ii: if Type(result) is not Object, return result.
                // IR-EXT 95: spec Type discrimination via IsSpecObject —
                // typeof(null) === "object" was previously misclassifying a
                // @@toPrimitive return of null as "result is an object",
                // triggering the spec step 2.b.iii TypeError on a spec-valid
                // primitive return.
                Step { spec_step: "2.b.ii.check".into(), node: IRNode::If {
                    cond: Expr::Not(b(Expr::IsSpecObject(b(v("result"))))),
                    then_body: vec![
                        Step { spec_step: "2.b.ii.return".into(), node: IRNode::Return(v("result")) },
                    ],
                    else_body: vec![],
                }},
                // §7.1.1 step 2.b.iii: throw TypeError.
                Step { spec_step: "2.b.iii.throw".into(), node: IRNode::Throw {
                    class: ErrorClass::TypeError,
                    message: "@@toPrimitive returned an object".into(),
                }},
            ],
            else_body: vec![],
        }},

        // §7.1.1.1 OrdinaryToPrimitive — method order from hint.
        Step { spec_step: "3.order".into(), node: IRNode::Let {
            name: "method1".into(),
            value: Expr::Str("valueOf".into()),
        }},
        Step { spec_step: "3.order.alt".into(), node: IRNode::Let {
            name: "method2".into(),
            value: Expr::Str("toString".into()),
        }},
        // If hint === "string", swap.
        Step { spec_step: "3.hint_check".into(), node: IRNode::If {
            cond: Expr::StrictEq(b(v("hint")), b(Expr::Str("string".into()))),
            then_body: vec![
                Step { spec_step: "3.swap.1".into(), node: IRNode::Assign {
                    name: "method1".into(),
                    value: Expr::Str("toString".into()),
                }},
                Step { spec_step: "3.swap.2".into(), node: IRNode::Assign {
                    name: "method2".into(),
                    value: Expr::Str("valueOf".into()),
                }},
            ],
            else_body: vec![],
        }},

        // §7.1.1.1 try method1.
        Step { spec_step: "4.m1.lookup".into(), node: IRNode::Let {
            name: "m1".into(),
            value: Expr::CallBuiltin {
                name: "get_via",
                args: vec![v("value"), v("method1")],
            },
        }},
        Step { spec_step: "4.m1.callable".into(), node: IRNode::If {
            cond: Expr::IsCallable(b(v("m1"))),
            then_body: vec![
                Step { spec_step: "4.m1.call".into(), node: IRNode::Let {
                    name: "r1".into(),
                    value: Expr::Call {
                        function: b(v("m1")),
                        this: b(v("value")),
                        args: vec![],
                    },
                }},
                // IR-EXT 95: §7.1.1.1 OrdinaryToPrimitive step "if
                // Type(r1) is not Object, return r1". IsSpecObject
                // closes the typeof-null collapse identically here.
                Step { spec_step: "4.m1.check".into(), node: IRNode::If {
                    cond: Expr::Not(b(Expr::IsSpecObject(b(v("r1"))))),
                    then_body: vec![
                        Step { spec_step: "4.m1.return".into(), node: IRNode::Return(v("r1")) },
                    ],
                    else_body: vec![],
                }},
            ],
            else_body: vec![],
        }},

        // §7.1.1.1 try method2.
        Step { spec_step: "5.m2.lookup".into(), node: IRNode::Let {
            name: "m2".into(),
            value: Expr::CallBuiltin {
                name: "get_via",
                args: vec![v("value"), v("method2")],
            },
        }},
        Step { spec_step: "5.m2.callable".into(), node: IRNode::If {
            cond: Expr::IsCallable(b(v("m2"))),
            then_body: vec![
                Step { spec_step: "5.m2.call".into(), node: IRNode::Let {
                    name: "r2".into(),
                    value: Expr::Call {
                        function: b(v("m2")),
                        this: b(v("value")),
                        args: vec![],
                    },
                }},
                // IR-EXT 95: same Type(r2) === Object discrimination via
                // IsSpecObject — closes the null case at the second method.
                Step { spec_step: "5.m2.check".into(), node: IRNode::If {
                    cond: Expr::Not(b(Expr::IsSpecObject(b(v("r2"))))),
                    then_body: vec![
                        Step { spec_step: "5.m2.return".into(), node: IRNode::Return(v("r2")) },
                    ],
                    else_body: vec![],
                }},
            ],
            else_body: vec![],
        }},

        // §7.1.1.1 step 4 (fallthrough): throw TypeError.
        Step { spec_step: "6.throw".into(), node: IRNode::Throw {
            class: ErrorClass::TypeError,
            message: "Cannot convert object to primitive value".into(),
        }},
    ];

    IRFunction {
        spec_section: "7.1.1".into(),
        rust_name: "to_primitive".into(),
        title: "ToPrimitive ( input, preferredType ) — resolver-instance".into(),
        body,
    }
}

pub fn spec_steps_to_primitive() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1.fast".into(), abstract_ops: vec![], throws: None, prose: "Non-Object: return as-is." },
        SpecStepRecord { step_id: "1.return".into(), abstract_ops: vec![], throws: None, prose: "Return primitive." },
        SpecStepRecord { step_id: "2.a.lookup".into(), abstract_ops: vec![], throws: None, prose: "exotic = Get(value, @@toPrimitive)." },
        SpecStepRecord { step_id: "2.b.has_exotic".into(), abstract_ops: vec![], throws: None, prose: "If exotic is callable." },
        SpecStepRecord { step_id: "2.b.i.call".into(), abstract_ops: vec![], throws: None, prose: "result = Call(exotic, value, [hint])." },
        SpecStepRecord { step_id: "2.b.ii.check".into(), abstract_ops: vec![], throws: None, prose: "If result is not Object." },
        SpecStepRecord { step_id: "2.b.ii.return".into(), abstract_ops: vec![], throws: None, prose: "Return result." },
        SpecStepRecord { step_id: "2.b.iii.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "@@toPrimitive returned Object." },
        SpecStepRecord { step_id: "3.order".into(), abstract_ops: vec![], throws: None, prose: "Initialize method order." },
        SpecStepRecord { step_id: "3.order.alt".into(), abstract_ops: vec![], throws: None, prose: "Second method initial." },
        SpecStepRecord { step_id: "3.hint_check".into(), abstract_ops: vec![], throws: None, prose: "If hint is 'string', swap." },
        SpecStepRecord { step_id: "3.swap.1".into(), abstract_ops: vec![], throws: None, prose: "method1 = toString." },
        SpecStepRecord { step_id: "3.swap.2".into(), abstract_ops: vec![], throws: None, prose: "method2 = valueOf." },
        SpecStepRecord { step_id: "4.m1.lookup".into(), abstract_ops: vec!["get_via"], throws: None, prose: "m1 = Get(value, method1)." },
        SpecStepRecord { step_id: "4.m1.callable".into(), abstract_ops: vec![], throws: None, prose: "If m1 is callable." },
        SpecStepRecord { step_id: "4.m1.call".into(), abstract_ops: vec![], throws: None, prose: "r1 = Call(m1, value)." },
        SpecStepRecord { step_id: "4.m1.check".into(), abstract_ops: vec![], throws: None, prose: "If r1 is not Object." },
        SpecStepRecord { step_id: "4.m1.return".into(), abstract_ops: vec![], throws: None, prose: "Return r1." },
        SpecStepRecord { step_id: "5.m2.lookup".into(), abstract_ops: vec!["get_via"], throws: None, prose: "m2 = Get(value, method2)." },
        SpecStepRecord { step_id: "5.m2.callable".into(), abstract_ops: vec![], throws: None, prose: "If m2 is callable." },
        SpecStepRecord { step_id: "5.m2.call".into(), abstract_ops: vec![], throws: None, prose: "r2 = Call(m2, value)." },
        SpecStepRecord { step_id: "5.m2.check".into(), abstract_ops: vec![], throws: None, prose: "If r2 is not Object." },
        SpecStepRecord { step_id: "5.m2.return".into(), abstract_ops: vec![], throws: None, prose: "Return r2." },
        SpecStepRecord { step_id: "6.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "All methods returned Object." },
    ]
}
