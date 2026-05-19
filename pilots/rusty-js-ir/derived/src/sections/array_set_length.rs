//! ECMA-262 §10.4.2.1 ArraySetLength — the Array exotic
//! [[DefineOwnProperty]] dispatch for the "length" property.
//!
//! IR-EXT 66 (first higher-resolution-IR section per keeper's
//! conjecture): the spec algorithm here is intricate, with
//! interleaved RangeError + TypeError throws, ToUint32 round-trip
//! validation, descriptor flag preservation, and an element-deletion
//! truncation loop that stops on first non-configurable. Encoded as
//! IR steps that read 1:1 against §10.4.2.1; lowered deterministically
//! to Rust by the EXT 0c lowering compiler.

use crate::ir::{ErrorClass, Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }
fn b(e: Expr) -> Box<Expr> { Box::new(e) }

pub fn build_array_set_length() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.target".into(), node: IRNode::Let { name: "target".into(), value: Expr::Arg(0) } },
        Step { spec_step: "param.desc".into(),   node: IRNode::Let { name: "desc".into(),   value: Expr::Arg(1) } },

        // §10.4.2.1 step 1: descriptor configurable: true ⇒ TypeError.
        Step { spec_step: "1.config".into(), node: IRNode::If {
            cond: Expr::HasProperty(b(v("desc")), b(Expr::Str("configurable".into()))),
            then_body: vec![
                Step { spec_step: "1.config.check".into(), node: IRNode::If {
                    cond: Expr::ToBoolean(b(Expr::Get(b(v("desc")), b(Expr::Str("configurable".into()))))),
                    then_body: vec![
                        Step { spec_step: "1.config.throw".into(), node: IRNode::Throw {
                            class: ErrorClass::TypeError,
                            message: "Array length: configurable is false".into(),
                        }},
                    ],
                    else_body: vec![],
                }},
            ],
            else_body: vec![],
        }},

        // §10.4.2.1 step 2.a: descriptor enumerable: true ⇒ TypeError.
        Step { spec_step: "2.enum".into(), node: IRNode::If {
            cond: Expr::HasProperty(b(v("desc")), b(Expr::Str("enumerable".into()))),
            then_body: vec![
                Step { spec_step: "2.enum.check".into(), node: IRNode::If {
                    cond: Expr::ToBoolean(b(Expr::Get(b(v("desc")), b(Expr::Str("enumerable".into()))))),
                    then_body: vec![
                        Step { spec_step: "2.enum.throw".into(), node: IRNode::Throw {
                            class: ErrorClass::TypeError,
                            message: "Array length: enumerable is false".into(),
                        }},
                    ],
                    else_body: vec![],
                }},
            ],
            else_body: vec![],
        }},

        // §10.4.2.1 step 2.b: get on length ⇒ TypeError.
        Step { spec_step: "2.get_throw".into(), node: IRNode::If {
            cond: Expr::HasProperty(b(v("desc")), b(Expr::Str("get".into()))),
            then_body: vec![
                Step { spec_step: "2.get.throw".into(), node: IRNode::Throw {
                    class: ErrorClass::TypeError,
                    message: "Array length cannot be accessor (get)".into(),
                }},
            ],
            else_body: vec![],
        }},
        // §10.4.2.1 step 2.b': set on length ⇒ TypeError.
        Step { spec_step: "2.set_throw".into(), node: IRNode::If {
            cond: Expr::HasProperty(b(v("desc")), b(Expr::Str("set".into()))),
            then_body: vec![
                Step { spec_step: "2.set.throw".into(), node: IRNode::Throw {
                    class: ErrorClass::TypeError,
                    message: "Array length cannot be accessor (set)".into(),
                }},
            ],
            else_body: vec![],
        }},

        // §10.4.2.1 step 3: read current length + writable from the
        // synthesized Array length descriptor.
        Step { spec_step: "3.cur_writable".into(), node: IRNode::Let {
            name: "cur_writable".into(),
            value: Expr::CallBuiltin {
                name: "array_length_writable_via",
                args: vec![v("target")],
            },
        }},
        Step { spec_step: "3.old_len".into(), node: IRNode::Let {
            name: "old_len".into(),
            value: Expr::CallBuiltin {
                name: "array_length_value_via",
                args: vec![v("target")],
            },
        }},

        // §10.4.2.1 step 4: degenerate path — descriptor lacks [[Value]].
        Step { spec_step: "4.no_value".into(), node: IRNode::If {
            cond: Expr::Not(b(Expr::HasProperty(b(v("desc")), b(Expr::Str("value".into()))))),
            then_body: vec![
                // 4.a: if writable provided, apply it.
                Step { spec_step: "4.a.writable_provided".into(), node: IRNode::If {
                    cond: Expr::HasProperty(b(v("desc")), b(Expr::Str("writable".into()))),
                    then_body: vec![
                        Step { spec_step: "4.a.read".into(), node: IRNode::Let {
                            name: "new_w".into(),
                            value: Expr::Get(b(v("desc")), b(Expr::Str("writable".into()))),
                        }},
                        // Non-writable + want-writable ⇒ throw.
                        Step { spec_step: "4.a.promote_check".into(), node: IRNode::If {
                            cond: Expr::Not(b(Expr::ToBoolean(b(v("cur_writable"))))),
                            then_body: vec![
                                Step { spec_step: "4.a.promote_inner".into(), node: IRNode::If {
                                    cond: Expr::ToBoolean(b(v("new_w"))),
                                    then_body: vec![
                                        Step { spec_step: "4.a.promote.throw".into(), node: IRNode::Throw {
                                            class: ErrorClass::TypeError,
                                            message: "Cannot promote Array length to writable".into(),
                                        }},
                                    ],
                                    else_body: vec![],
                                }},
                            ],
                            else_body: vec![],
                        }},
                        Step { spec_step: "4.a.apply".into(), node: IRNode::Expr(Expr::CallBuiltin {
                            name: "array_length_set_internal_via",
                            args: vec![v("target"), v("old_len"), v("new_w")],
                        })},
                    ],
                    else_body: vec![],
                }},
                Step { spec_step: "4.return".into(), node: IRNode::Return(v("target")) },
            ],
            else_body: vec![],
        }},

        // §10.4.2.1 step 5-6: ToUint32 with round-trip validation —
        // throws RangeError on mismatch.
        Step { spec_step: "5.raw_value".into(), node: IRNode::Let {
            name: "raw_value".into(),
            value: Expr::Get(b(v("desc")), b(Expr::Str("value".into()))),
        }},
        Step { spec_step: "6.new_len".into(), node: IRNode::Let {
            name: "new_len".into(),
            value: Expr::CallBuiltin {
                name: "to_uint32_strict_via",
                args: vec![v("raw_value")],
            },
        }},

        // §10.4.2.1 step 10: non-writable + value change ⇒ TypeError.
        Step { spec_step: "10.writable_check".into(), node: IRNode::If {
            cond: Expr::Not(b(Expr::ToBoolean(b(v("cur_writable"))))),
            then_body: vec![
                Step { spec_step: "10.diff_check".into(), node: IRNode::If {
                    cond: Expr::Not(b(Expr::StrictEq(b(v("new_len")), b(v("old_len"))))),
                    then_body: vec![
                        Step { spec_step: "10.throw".into(), node: IRNode::Throw {
                            class: ErrorClass::TypeError,
                            message: "Cannot change non-writable Array length".into(),
                        }},
                    ],
                    else_body: vec![],
                }},
            ],
            else_body: vec![],
        }},

        // §10.4.2.1 step 11: compute new_writable. Initialize to cur_writable;
        // if descriptor has writable, override (via reassign within an If).
        Step { spec_step: "11.init_writable".into(), node: IRNode::Let {
            name: "new_writable".into(),
            value: v("cur_writable"),
        }},
        Step { spec_step: "11.override".into(), node: IRNode::If {
            cond: Expr::HasProperty(b(v("desc")), b(Expr::Str("writable".into()))),
            then_body: vec![
                Step { spec_step: "11.real".into(), node: IRNode::Assign {
                    name: "new_writable".into(),
                    // Read raw value (Value::Boolean expected; wrap if not).
                    value: Expr::Get(b(v("desc")), b(Expr::Str("writable".into()))),
                }},
            ],
            else_body: vec![],
        }},

        // §10.4.2.1 step 12-14: shrink trailing elements when new_len < old_len.
        // IR-EXT 67: Number arithmetic via Expr::NumberSub/NumberLt/NumberGe
        // (promoted alphabet — Value::Number-typed operators).
        Step { spec_step: "12.maybe_shrink".into(), node: IRNode::If {
            cond: Expr::NumberLt(b(v("new_len")), b(v("old_len"))),
            then_body: vec![
                Step { spec_step: "12.idx.init".into(), node: IRNode::Let {
                    name: "idx".into(),
                    value: Expr::NumberSub(b(v("old_len")), b(Expr::Number(1.0))),
                }},
                Step { spec_step: "12.loop".into(), node: IRNode::While {
                    cond: Expr::NumberGe(b(v("idx")), b(v("new_len"))),
                    body: vec![
                        Step { spec_step: "13.idx_key".into(), node: IRNode::Let {
                            name: "idx_key".into(),
                            value: Expr::CallBuiltin {
                                name: "number_to_string_key_via",
                                args: vec![v("idx")],
                            },
                        }},
                        Step { spec_step: "13.try_delete".into(), node: IRNode::Let {
                            name: "deleted".into(),
                            value: Expr::CallBuiltin {
                                name: "delete_own_via",
                                args: vec![v("target"), v("idx_key")],
                            },
                        }},
                        Step { spec_step: "13.delete_check".into(), node: IRNode::If {
                            cond: Expr::Not(b(Expr::ToBoolean(b(v("deleted"))))),
                            then_body: vec![
                                Step { spec_step: "13.stuck.idx_plus".into(), node: IRNode::Let {
                                    name: "stuck_len".into(),
                                    value: Expr::NumberAdd(b(v("idx")), b(Expr::Number(1.0))),
                                }},
                                Step { spec_step: "13.stuck.length".into(), node: IRNode::Expr(Expr::CallBuiltin {
                                    name: "array_length_set_internal_via",
                                    args: vec![v("target"), v("stuck_len"), v("new_writable")],
                                })},
                                Step { spec_step: "13.stuck.throw".into(), node: IRNode::Throw {
                                    class: ErrorClass::TypeError,
                                    message: "Cannot truncate Array: non-configurable element".into(),
                                }},
                            ],
                            else_body: vec![],
                        }},
                        Step { spec_step: "14.decrement".into(), node: IRNode::Assign {
                            name: "idx".into(),
                            value: Expr::NumberSub(b(v("idx")), b(Expr::Number(1.0))),
                        }},
                    ],
                }},
            ],
            else_body: vec![],
        }},

        // §10.4.2.1 step 15: install final length + writable.
        Step { spec_step: "15.install".into(), node: IRNode::Expr(Expr::CallBuiltin {
            name: "array_length_set_internal_via",
            args: vec![v("target"), v("new_len"), v("new_writable")],
        })},

        // Return target.
        Step { spec_step: "16.return".into(), node: IRNode::Return(v("target")) },
    ];

    IRFunction {
        spec_section: "10.4.2.1".into(),
        rust_name: "array_set_length".into(),
        title: "ArraySetLength ( A, Desc )".into(),
        body,
    }
}

pub fn spec_steps_array_set_length() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1.config".into(), abstract_ops: vec![], throws: None, prose: "If descriptor has configurable, prepare to check it." },
        SpecStepRecord { step_id: "1.config.check".into(), abstract_ops: vec![], throws: None, prose: "If descriptor.configurable is true, throw." },
        SpecStepRecord { step_id: "1.config.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "Array length is non-configurable; reject promotion." },
        SpecStepRecord { step_id: "2.enum".into(), abstract_ops: vec![], throws: None, prose: "If descriptor has enumerable, check it." },
        SpecStepRecord { step_id: "2.enum.check".into(), abstract_ops: vec![], throws: None, prose: "If enumerable is true, throw." },
        SpecStepRecord { step_id: "2.enum.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "Array length is non-enumerable; reject promotion." },
        SpecStepRecord { step_id: "2.get_throw".into(), abstract_ops: vec![], throws: None, prose: "If descriptor has get, throw." },
        SpecStepRecord { step_id: "2.get.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "Array length cannot be accessor (get)." },
        SpecStepRecord { step_id: "2.set_throw".into(), abstract_ops: vec![], throws: None, prose: "If descriptor has set, throw." },
        SpecStepRecord { step_id: "2.set.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "Array length cannot be accessor (set)." },
        SpecStepRecord { step_id: "3.cur_writable".into(), abstract_ops: vec!["array_length_writable_via"], throws: None, prose: "Read current length writable attribute." },
        SpecStepRecord { step_id: "3.old_len".into(), abstract_ops: vec!["array_length_value_via"], throws: None, prose: "Read current length value." },
        SpecStepRecord { step_id: "4.no_value".into(), abstract_ops: vec![], throws: None, prose: "If descriptor lacks [[Value]], degenerate path." },
        SpecStepRecord { step_id: "4.a.writable_provided".into(), abstract_ops: vec![], throws: None, prose: "If writable provided, apply it." },
        SpecStepRecord { step_id: "4.a.read".into(), abstract_ops: vec![], throws: None, prose: "Read new writable." },
        SpecStepRecord { step_id: "4.a.promote_check".into(), abstract_ops: vec![], throws: None, prose: "If current non-writable, prepare check." },
        SpecStepRecord { step_id: "4.a.promote_inner".into(), abstract_ops: vec![], throws: None, prose: "If want-writable, throw." },
        SpecStepRecord { step_id: "4.a.promote.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "Cannot promote Array length to writable." },
        SpecStepRecord { step_id: "4.a.apply".into(), abstract_ops: vec!["array_length_set_internal_via"], throws: None, prose: "Apply new writable while keeping current length." },
        SpecStepRecord { step_id: "4.return".into(), abstract_ops: vec![], throws: None, prose: "Return target after degenerate path." },
        SpecStepRecord { step_id: "5.raw_value".into(), abstract_ops: vec![], throws: None, prose: "Read descriptor's value." },
        SpecStepRecord { step_id: "6.new_len".into(), abstract_ops: vec!["to_uint32_strict_via"], throws: None, prose: "ToUint32 + round-trip validation (throws RangeError inside the builtin)." },
        SpecStepRecord { step_id: "10.writable_check".into(), abstract_ops: vec![], throws: None, prose: "If current non-writable, check value diff." },
        SpecStepRecord { step_id: "10.diff_check".into(), abstract_ops: vec![], throws: None, prose: "If new_len differs from old_len, throw." },
        SpecStepRecord { step_id: "10.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "Cannot change non-writable Array length." },
        SpecStepRecord { step_id: "11.init_writable".into(), abstract_ops: vec![], throws: None, prose: "Initialize new_writable to current." },
        SpecStepRecord { step_id: "11.override".into(), abstract_ops: vec![], throws: None, prose: "If descriptor.writable present, override." },
        SpecStepRecord { step_id: "11.real".into(), abstract_ops: vec![], throws: None, prose: "Assign new_writable from descriptor." },
        SpecStepRecord { step_id: "12.maybe_shrink".into(), abstract_ops: vec![], throws: None, prose: "If new_len < old_len, walk indices down deleting." },
        SpecStepRecord { step_id: "12.idx.init".into(), abstract_ops: vec![], throws: None, prose: "Start at old_len - 1." },
        SpecStepRecord { step_id: "12.loop".into(), abstract_ops: vec![], throws: None, prose: "Loop while idx >= new_len." },
        SpecStepRecord { step_id: "13.idx_key".into(), abstract_ops: vec!["number_to_string_key_via"], throws: None, prose: "Stringify idx to key form." },
        SpecStepRecord { step_id: "13.try_delete".into(), abstract_ops: vec!["delete_own_via"], throws: None, prose: "Attempt delete of arr[idx]." },
        SpecStepRecord { step_id: "13.delete_check".into(), abstract_ops: vec![], throws: None, prose: "If deletion failed, set stuck length and throw." },
        SpecStepRecord { step_id: "13.stuck.idx_plus".into(), abstract_ops: vec![], throws: None, prose: "Compute idx+1 for stuck length." },
        SpecStepRecord { step_id: "13.stuck.length".into(), abstract_ops: vec!["array_length_set_internal_via"], throws: None, prose: "Set length to idx+1 (stop point)." },
        SpecStepRecord { step_id: "13.stuck.throw".into(), abstract_ops: vec![], throws: Some("TypeError"), prose: "Cannot truncate Array: non-configurable element." },
        SpecStepRecord { step_id: "14.decrement".into(), abstract_ops: vec![], throws: None, prose: "Decrement idx." },
        SpecStepRecord { step_id: "15.install".into(), abstract_ops: vec!["array_length_set_internal_via"], throws: None, prose: "Install final new_len + new_writable." },
        SpecStepRecord { step_id: "16.return".into(), abstract_ops: vec![], throws: None, prose: "Return target." },
    ]
}
