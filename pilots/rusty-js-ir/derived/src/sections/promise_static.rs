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

// ──────────────── Ω.5.P63.E55 Stage 2: Promise.withResolvers via alphabet closures ────────────────
//
// First IR section that uses Expr::Closure end-to-end. Models §27.2.4.4:
//
//   1. Let C be the this value.
//   2. Let promiseCapability be ? NewPromiseCapability(C).
//   3. Let obj be OrdinaryObjectCreate(%Object.prototype%).
//   4. Perform ! CreateDataProperty(obj, "promise", promiseCapability.[[Promise]]).
//   5. Perform ! CreateDataProperty(obj, "resolve", promiseCapability.[[Resolve]]).
//   6. Perform ! CreateDataProperty(obj, "reject", promiseCapability.[[Reject]]).
//   7. Return obj.
//
// We inline NewPromiseCapability for the built-in Promise constructor case
// (the spec's most common path): allocate a fresh pending promise, then
// construct two resolve/reject closures that capture the promise handle and
// settle it on invocation. Each closure body is a single CallBuiltin to the
// settling helper.

pub fn build_with_resolvers() -> IRFunction {
    let body = vec![
        // step 1 alt: allocate the fresh pending Promise.
        Step { spec_step: "1".into(), node: IRNode::Let {
            name: "p".into(),
            value: Expr::CallBuiltin { name: "new_promise_value_via", args: vec![] },
        }},
        // steps 2/5: resolve function. Captures `p`; the body settles `p`
        // with the first arg.
        Step { spec_step: "2".into(), node: IRNode::Let {
            name: "resolve_fn".into(),
            value: Expr::Closure {
                label: "<Promise.withResolvers resolve>",
                params: vec!["v".into()],
                captures: vec!["p".into()],
                body: vec![
                    Step { spec_step: "2.a".into(), node: IRNode::Expr(Expr::CallBuiltin {
                        name: "promise_settle_fulfilled_via",
                        args: vec![v("p"), v("v")],
                    })},
                ],
            },
        }},
        // steps 3/6: reject function. Symmetric to resolve.
        Step { spec_step: "3".into(), node: IRNode::Let {
            name: "reject_fn".into(),
            value: Expr::Closure {
                label: "<Promise.withResolvers reject>",
                params: vec!["v".into()],
                captures: vec!["p".into()],
                body: vec![
                    Step { spec_step: "3.a".into(), node: IRNode::Expr(Expr::CallBuiltin {
                        name: "promise_settle_rejected_via",
                        args: vec![v("p"), v("v")],
                    })},
                ],
            },
        }},
        // step 7 (combined with 4/5/6): assemble the {promise, resolve, reject} object.
        Step { spec_step: "7".into(), node: IRNode::Return(Expr::CallBuiltin {
            name: "promise_with_resolvers_assemble_via",
            args: vec![v("p"), v("resolve_fn"), v("reject_fn")],
        })},
    ];
    IRFunction {
        spec_section: "27.2.4.4".into(),
        rust_name: "promise_with_resolvers".into(),
        title: "Promise.withResolvers ( )".into(),
        body,
    }
}

pub fn spec_steps_with_resolvers() -> Vec<SpecStepRecord> {
    vec![
        SpecStepRecord { step_id: "1".into(),     abstract_ops: vec!["new_promise_value_via"],            throws: None,
            prose: "Let promiseCapability.[[Promise]] be a new pending Promise." },
        SpecStepRecord { step_id: "2".into(),     abstract_ops: vec![],                                    throws: None,
            prose: "Let resolveFn be a new built-in function whose internal slot captures the promise." },
        SpecStepRecord { step_id: "2.a".into(),   abstract_ops: vec!["promise_settle_fulfilled_via"],     throws: None,
            prose: "When called with v, fulfill the captured promise with v." },
        SpecStepRecord { step_id: "3".into(),     abstract_ops: vec![],                                    throws: None,
            prose: "Let rejectFn be a new built-in function whose internal slot captures the promise." },
        SpecStepRecord { step_id: "3.a".into(),   abstract_ops: vec!["promise_settle_rejected_via"],      throws: None,
            prose: "When called with v, reject the captured promise with v." },
        SpecStepRecord { step_id: "7".into(),     abstract_ops: vec!["promise_with_resolvers_assemble_via"], throws: None,
            prose: "Return { promise, resolve: resolveFn, reject: rejectFn }." },
    ]
}

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
