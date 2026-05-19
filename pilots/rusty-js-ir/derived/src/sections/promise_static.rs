//! ECMA-262 §27.2.4.{7, 5} — Promise.resolve / Promise.reject.
//!
//! Both are thin wrappers over NewPromiseCapability + Resolve/Reject:
//!   Promise.resolve(x):
//!     1. If C is not an Object, throw TypeError.
//!     2. If x is an Object with constructor C, return x.
//!     3. Let promiseCapability be ? NewPromiseCapability(C).
//!     4. Perform ? Call(promiseCapability.[[Resolve]], undefined, « x »).
//!     5. Return promiseCapability.[[Promise]].
//!
//! Tier 1.10 simplification: the Tier-1 IR doesn't yet model
//! IsConstructor + NewPromiseCapability over an arbitrary `this`. We
//! always use cruftless's built-in Promise constructor (matching
//! cruftless's pre-IR Promise.resolve / Promise.reject impls).
//!
//! When the IR alphabet gains IsConstructor + the NewPromiseCapability
//! builtin, this section can be reauthored to faithfully model
//! `Promise.resolve.call(SubPromise, x)` returning a SubPromise.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn b(e: Expr) -> Box<Expr> { Box::new(e) }
fn v(name: &str) -> Expr { Expr::Var(name.to_string()) }

pub fn build_resolve() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.x".into(),
            node: IRNode::Let { name: "x".into(), value: Expr::Arg(0) },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "promise_resolve_via",
                args: vec![v("x")],
            }),
        },
    ];
    IRFunction {
        spec_section: "27.2.4.7".into(),
        rust_name: "promise_resolve".into(),
        title: "Promise.resolve ( x )".into(),
        body,
    }
}

pub fn build_reject() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.r".into(),
            node: IRNode::Let { name: "reason".into(), value: Expr::Arg(0) },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "promise_reject_via",
                args: vec![v("reason")],
            }),
        },
    ];
    IRFunction {
        spec_section: "27.2.4.5".into(),
        rust_name: "promise_reject".into(),
        title: "Promise.reject ( r )".into(),
        body,
    }
}

pub fn spec_steps_resolve() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord {
            step_id: "1".into(),
            abstract_ops: vec!["promise_resolve_via"],
            throws: None,
            prose: "Return PromiseResolve(C, x) — Tier-1.10 simplified to use the built-in Promise constructor.",
        },
    ]
}

pub fn spec_steps_reject() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord {
            step_id: "1".into(),
            abstract_ops: vec!["promise_reject_via"],
            throws: None,
            prose: "Return a new Promise rejected with r — Tier-1.10 simplified to use the built-in Promise constructor.",
        },
    ]
}
