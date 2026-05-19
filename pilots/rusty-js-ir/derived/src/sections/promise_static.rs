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

// ──────────────── Ω.5.P63.E52: Promise prototype + variadic statics ────────────────
//
// then / catch / finally / all / allSettled / any / race all reduce to a
// 1-step CallBuiltin over Expr::AllArgs. The runtime helpers carry the
// full PromiseStatus dispatch + reaction-queue plumbing; the IR is the
// spec-traceable shim.

fn variadic_section(spec: &str, rust_name: &str, title: &str, via: &'static str) -> IRFunction {
    IRFunction {
        spec_section: spec.into(), rust_name: rust_name.into(), title: title.into(),
        body: vec![Step { spec_step: "1".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: via, args: vec![Expr::AllArgs],
        })}],
    }
}

pub fn build_then()         -> IRFunction { variadic_section("27.2.5.4", "promise_prototype_then",     "Promise.prototype.then ( onFulfilled, onRejected )", "promise_then_via") }
pub fn build_catch()        -> IRFunction { variadic_section("27.2.5.1", "promise_prototype_catch",    "Promise.prototype.catch ( onRejected )",              "promise_catch_via") }
pub fn build_finally()      -> IRFunction { variadic_section("27.2.5.3", "promise_prototype_finally",  "Promise.prototype.finally ( onFinally )",             "promise_finally_via") }
pub fn build_all()          -> IRFunction { variadic_section("27.2.4.1", "promise_all",                "Promise.all ( iterable )",                            "promise_all_via") }
pub fn build_all_settled()  -> IRFunction { variadic_section("27.2.4.2", "promise_all_settled",        "Promise.allSettled ( iterable )",                     "promise_all_settled_via") }
pub fn build_any()          -> IRFunction { variadic_section("27.2.4.3", "promise_any",                "Promise.any ( iterable )",                            "promise_any_via") }
pub fn build_race()         -> IRFunction { variadic_section("27.2.4.5", "promise_race",               "Promise.race ( iterable )",                           "promise_race_via") }

pub fn spec_steps_then()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["promise_then_via"],        throws: None, prose: "Chain on a source Promise; queue or enqueue reactions based on settlement state." }] }
pub fn spec_steps_catch()       -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["promise_catch_via"],       throws: None, prose: "Chain rejected-only handler; equivalent to then(undefined, onRejected)." }] }
pub fn spec_steps_finally()     -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["promise_finally_via"],     throws: None, prose: "Run onFinally side-effect callback, then propagate source settlement to the chain." }] }
pub fn spec_steps_all()         -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["promise_all_via"],         throws: None, prose: "Aggregate iterable to a single Promise resolved with the array of values, or rejected with the first rejection (v1 sync-only)." }] }
pub fn spec_steps_all_settled() -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["promise_all_settled_via"], throws: None, prose: "Return a Promise resolved with {status, value/reason} entries per iteration item." }] }
pub fn spec_steps_any()         -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["promise_any_via"],         throws: None, prose: "Resolve with the first fulfilled entry; reject with AggregateError if all rejected." }] }
pub fn spec_steps_race()        -> Vec<SpecStepRecord> { vec![SpecStepRecord { step_id: "1".into(), abstract_ops: vec!["promise_race_via"],        throws: None, prose: "Settle with the first-settled entry of the iterable." }] }

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
