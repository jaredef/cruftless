//! ECMA-262 §21.1.3 — Number.prototype.* methods.
//!
//! Tier 1.10: each section is a 1-step CallBuiltin to a runtime helper
//! that does ThisNumberValue brand-check + arg coercion + the spec
//! formatting. cruftless's existing impls are already P62.E19/E20
//! spec-compliant; the IR translation adds spec-step traceability +
//! the M6 discipline (one helper per spec method).

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn build_to_fixed() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.digits".into(),
            node: IRNode::Let {
                name: "digits".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "number_proto_to_fixed_via",
                args: vec![Expr::This, v("digits")],
            }),
        },
    ];
    IRFunction {
        spec_section: "21.1.3.3".into(),
        rust_name: "number_prototype_to_fixed".into(),
        title: "Number.prototype.toFixed ( fractionDigits )".into(),
        body,
    }
}

pub fn spec_steps_to_fixed() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["number_proto_to_fixed_via"], throws: None,
        prose: "Let x be ? ThisNumberValue(this). Let f be ? ToIntegerOrInfinity(fractionDigits). If f ∉ [0, 100], throw RangeError. Return the spec-prescribed Number-to-fixed-decimal string." }]
}

// ──────────────── §21.1.3.7 valueOf / §21.1.3.2 toExponential / §21.1.3.5 toPrecision ────────────────

pub fn build_value_of() -> IRFunction {
    let body = vec![Step {
        spec_step: "1".into(),
        node: IRNode::Return(Expr::CallBuiltin {
            name: "number_proto_value_of_via",
            args: vec![Expr::This],
        }),
    }];
    IRFunction {
        spec_section: "21.1.3.7".into(),
        rust_name: "number_prototype_value_of".into(),
        title: "Number.prototype.valueOf ( )".into(),
        body,
    }
}

pub fn build_to_exponential() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.digits".into(),
            node: IRNode::Let {
                name: "digits".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "number_proto_to_exponential_via",
                args: vec![Expr::This, v("digits")],
            }),
        },
    ];
    IRFunction {
        spec_section: "21.1.3.2".into(),
        rust_name: "number_prototype_to_exponential".into(),
        title: "Number.prototype.toExponential ( fractionDigits )".into(),
        body,
    }
}

pub fn build_to_precision() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.precision".into(),
            node: IRNode::Let {
                name: "precision".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "number_proto_to_precision_via",
                args: vec![Expr::This, v("precision")],
            }),
        },
    ];
    IRFunction {
        spec_section: "21.1.3.5".into(),
        rust_name: "number_prototype_to_precision".into(),
        title: "Number.prototype.toPrecision ( precision )".into(),
        body,
    }
}

pub fn spec_steps_value_of() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["number_proto_value_of_via"],
        throws: None,
        prose: "Return ? ThisNumberValue(this).",
    }]
}
pub fn spec_steps_to_exponential() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["number_proto_to_exponential_via"], throws: None,
        prose: "Let x be ? ThisNumberValue(this). Let f be ? ToIntegerOrInfinity(fractionDigits). If f ∉ [0, 100], throw RangeError. Return the spec-prescribed exponential string." }]
}
pub fn spec_steps_to_precision() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["number_proto_to_precision_via"], throws: None,
        prose: "Let x be ? ThisNumberValue(this). If precision is undefined, return ToString(x). Let p be ? ToIntegerOrInfinity(precision). If p ∉ [1, 100], throw RangeError. Return the spec-prescribed precision string." }]
}

// ──────────────── Boolean.prototype.{valueOf, toString} ────────────────
// Co-located here since they share the brand-checked proto-method pattern.

pub fn build_boolean_value_of() -> IRFunction {
    let body = vec![Step {
        spec_step: "1".into(),
        node: IRNode::Return(Expr::CallBuiltin {
            name: "boolean_proto_value_of_via",
            args: vec![Expr::This],
        }),
    }];
    IRFunction {
        spec_section: "20.3.3.3".into(),
        rust_name: "boolean_prototype_value_of".into(),
        title: "Boolean.prototype.valueOf ( )".into(),
        body,
    }
}

pub fn build_boolean_to_string() -> IRFunction {
    let body = vec![Step {
        spec_step: "1".into(),
        node: IRNode::Return(Expr::CallBuiltin {
            name: "boolean_proto_to_string_via",
            args: vec![Expr::This],
        }),
    }];
    IRFunction {
        spec_section: "20.3.3.2".into(),
        rust_name: "boolean_prototype_to_string".into(),
        title: "Boolean.prototype.toString ( )".into(),
        body,
    }
}

pub fn spec_steps_boolean_value_of() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["boolean_proto_value_of_via"],
        throws: None,
        prose: "Return ? ThisBooleanValue(this).",
    }]
}
pub fn spec_steps_boolean_to_string() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["boolean_proto_to_string_via"],
        throws: None,
        prose: "Let b be ? ThisBooleanValue(this). Return \"true\" or \"false\".",
    }]
}
