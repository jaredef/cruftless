//! ECMA-262 §25.5.2.4 SerializeJSONProperty — the per-value JSON
//! serializer dispatched by JSON.stringify.
//!
//! IR-EXT 68: second higher-resolution-IR spec-step section per the
//! keeper's "higher-resolution IR for spec edge cases" conjecture.
//! Encodes the intricate dispatch (toJSON method invocation, primitive
//! wrapper unwrap, primitive serialization branches, BigInt TypeError,
//! object/array recursion, elision sentinel) as IR steps reading 1:1
//! against §25.5.2.4.
//!
//! Returns: Value::String("serialized") for serializable values, or
//! Value::Undefined for top-level undefined/function/symbol (the
//! elision sentinel that propagates through SerializeJSONObject).

use crate::ir::{ErrorClass, Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}
fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}

pub fn build_json_serialize_property() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.value".into(),
            node: IRNode::Let {
                name: "value".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.key".into(),
            node: IRNode::Let {
                name: "key".into(),
                value: Expr::Arg(1),
            },
        },
        // §25.5.2.4 step 2: if value is Object or BigInt, dispatch toJSON.
        Step {
            spec_step: "2.tojson".into(),
            node: IRNode::Assign {
                name: "value".into(),
                value: Expr::CallBuiltin {
                    name: "json_apply_to_json_via",
                    args: vec![v("value"), v("key")],
                },
            },
        },
        // §25.5.2.4 step 4: unwrap primitive wrapper Object.
        Step {
            spec_step: "4.unwrap".into(),
            node: IRNode::Assign {
                name: "value".into(),
                value: Expr::CallBuiltin {
                    name: "json_unwrap_wrapper_via",
                    args: vec![v("value")],
                },
            },
        },
        // §25.5.2.4 step 5: value is null → "null".
        Step {
            spec_step: "5.null".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(b(v("value")), b(Expr::Null)),
                then_body: vec![Step {
                    spec_step: "5.return".into(),
                    node: IRNode::Return(Expr::Str("null".into())),
                }],
                else_body: vec![],
            },
        },
        // §25.5.2.4 step 6: value is true → "true".
        Step {
            spec_step: "6.true".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(b(v("value")), b(Expr::Bool(true))),
                then_body: vec![Step {
                    spec_step: "6.return".into(),
                    node: IRNode::Return(Expr::Str("true".into())),
                }],
                else_body: vec![],
            },
        },
        // §25.5.2.4 step 7: value is false → "false".
        Step {
            spec_step: "7.false".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(b(v("value")), b(Expr::Bool(false))),
                then_body: vec![Step {
                    spec_step: "7.return".into(),
                    node: IRNode::Return(Expr::Str("false".into())),
                }],
                else_body: vec![],
            },
        },
        // §25.5.2.4 step 8: value is a String → QuoteJSONString.
        Step {
            spec_step: "8.string".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(
                    b(Expr::TypeOf(b(v("value")))),
                    b(Expr::Str("string".into())),
                ),
                then_body: vec![Step {
                    spec_step: "8.return".into(),
                    node: IRNode::Return(Expr::CallBuiltin {
                        name: "json_quote_string_via",
                        args: vec![v("value")],
                    }),
                }],
                else_body: vec![],
            },
        },
        // §25.5.2.4 step 9: value is a Number → ToString if finite, else "null".
        Step {
            spec_step: "9.number".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(
                    b(Expr::TypeOf(b(v("value")))),
                    b(Expr::Str("number".into())),
                ),
                then_body: vec![Step {
                    spec_step: "9.return".into(),
                    node: IRNode::Return(Expr::CallBuiltin {
                        name: "json_format_number_via",
                        args: vec![v("value")],
                    }),
                }],
                else_body: vec![],
            },
        },
        // §25.5.2.4 step 10: value is a BigInt → throw TypeError.
        Step {
            spec_step: "10.bigint".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(
                    b(Expr::TypeOf(b(v("value")))),
                    b(Expr::Str("bigint".into())),
                ),
                then_body: vec![Step {
                    spec_step: "10.throw".into(),
                    node: IRNode::Throw {
                        class: ErrorClass::TypeError,
                        message: "Do not know how to serialize a BigInt".into(),
                    },
                }],
                else_body: vec![],
            },
        },
        // §25.5.2.4 step 11: value is an Object and not callable →
        // recurse via SerializeJSONObject / SerializeJSONArray.
        Step {
            spec_step: "11.object".into(),
            node: IRNode::If {
                cond: Expr::StrictEq(
                    b(Expr::TypeOf(b(v("value")))),
                    b(Expr::Str("object".into())),
                ),
                then_body: vec![Step {
                    spec_step: "11.return".into(),
                    node: IRNode::Return(Expr::CallBuiltin {
                        name: "json_serialize_compound_via",
                        args: vec![v("value")],
                    }),
                }],
                else_body: vec![],
            },
        },
        // §25.5.2.4 step 12: fallthrough (undefined, function, symbol) →
        // return undefined as the elision sentinel.
        Step {
            spec_step: "12.return".into(),
            node: IRNode::Return(Expr::Undefined),
        },
    ];

    IRFunction {
        spec_section: "25.5.2.4".into(),
        rust_name: "json_serialize_property".into(),
        title: "SerializeJSONProperty ( state, key, holder )".into(),
        body,
    }
}

pub fn spec_steps_json_serialize_property() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord {
            step_id: "2.tojson".into(),
            abstract_ops: vec!["json_apply_to_json_via"],
            throws: None,
            prose: "If value is Object|BigInt, invoke toJSON method.",
        },
        SpecStepRecord {
            step_id: "4.unwrap".into(),
            abstract_ops: vec!["json_unwrap_wrapper_via"],
            throws: None,
            prose: "Unwrap Number/String/Boolean/BigInt wrapper Object.",
        },
        SpecStepRecord {
            step_id: "5.null".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "If value is null, prepare to return 'null'.",
        },
        SpecStepRecord {
            step_id: "5.return".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Return 'null'.",
        },
        SpecStepRecord {
            step_id: "6.true".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "If value is true, prepare to return 'true'.",
        },
        SpecStepRecord {
            step_id: "6.return".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Return 'true'.",
        },
        SpecStepRecord {
            step_id: "7.false".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "If value is false, prepare to return 'false'.",
        },
        SpecStepRecord {
            step_id: "7.return".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Return 'false'.",
        },
        SpecStepRecord {
            step_id: "8.string".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "If typeof value is string, quote it.",
        },
        SpecStepRecord {
            step_id: "8.return".into(),
            abstract_ops: vec!["json_quote_string_via"],
            throws: None,
            prose: "Return QuoteJSONString(value).",
        },
        SpecStepRecord {
            step_id: "9.number".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "If typeof value is number, format it.",
        },
        SpecStepRecord {
            step_id: "9.return".into(),
            abstract_ops: vec!["json_format_number_via"],
            throws: None,
            prose: "Return ToString(n) when finite, 'null' otherwise.",
        },
        SpecStepRecord {
            step_id: "10.bigint".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "If typeof value is bigint, throw.",
        },
        SpecStepRecord {
            step_id: "10.throw".into(),
            abstract_ops: vec![],
            throws: Some("TypeError"),
            prose: "BigInt cannot be JSON-serialized.",
        },
        SpecStepRecord {
            step_id: "11.object".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "If typeof value is object, recurse.",
        },
        SpecStepRecord {
            step_id: "11.return".into(),
            abstract_ops: vec!["json_serialize_compound_via"],
            throws: None,
            prose: "Return SerializeJSONObject/Array.",
        },
        SpecStepRecord {
            step_id: "12.return".into(),
            abstract_ops: vec![],
            throws: None,
            prose: "Elide undefined/function/symbol values.",
        },
    ]
}
