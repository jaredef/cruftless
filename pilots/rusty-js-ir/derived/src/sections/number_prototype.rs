//! ECMA-262 §21.1.3 — Number.prototype.* methods.
//!
//! Tier 1.10: each section is a 1-step CallBuiltin to a runtime helper
//! that does ThisNumberValue brand-check + arg coercion + the spec
//! formatting. cruftless's existing impls are already P62.E19/E20
//! spec-compliant; the IR translation adds spec-step traceability +
//! the M6 discipline (one helper per spec method).

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

pub fn build_to_fixed() -> IRFunction {
    let body = vec![
        Step { spec_step: "param.digits".into(),
            node: IRNode::Let { name: "digits".into(), value: Expr::Arg(0) }},
        Step { spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "number_proto_to_fixed_via",
                args: vec![Expr::This, v("digits")],
            }) },
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
