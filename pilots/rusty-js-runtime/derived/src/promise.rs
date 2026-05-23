//! Promise intrinsic + reaction routing through the JobQueue per
//! ECMA-262 §27.2 + the Doc 714 §VI Consequence 5 architectural shift.
//!
//! Round 3.e.d migrates Promise objects onto the managed heap. Promise
//! state + reactions are accessed through `rt.obj(id)` / `rt.obj_mut(id)`.

use crate::interp::{Runtime, RuntimeError};
use crate::value::{
    FunctionInternals, InternalKind, NativeFn, Object, ObjectRef, PromiseReaction,
    PromiseState, PromiseStatus, Value,
};
use std::collections::HashMap;
use std::rc::Rc;

impl Runtime {
    pub fn install_promise(&mut self) {
        // Tier-Ω.5.kkk: Promise installed as a real Function (InternalKind::Function),
        // not an ordinary object. Without this, `new Promise((r, j) => {...})`
        // hit "callee is not callable: Object(kind=ordinary)" — load-bearing
        // for p-defer, ky, jose, got, get-stream, and any package whose
        // module-init constructs a Promise (~5 packages in route-2 ERR).
        // The constructor takes the executor function, builds a fresh
        // pending Promise, invokes the executor with resolve/reject
        // callbacks, and returns the promise.
        let promise_ctor = crate::intrinsics::make_native("Promise", |rt, args| {
            // Ω.5.P62.E24: §27.2.3.1 — Promise must be called with `new`;
            // executor must be callable.
            if rt.current_new_target.is_none() {
                return Err(RuntimeError::TypeError(
                    "Promise constructor must be called with new".into()));
            }
            let executor = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&executor) {
                return Err(RuntimeError::TypeError(
                    "Promise constructor: executor must be callable".into()));
            }
            let p = new_promise(rt);
            let p_for_resolve = p;
            let p_for_reject = p;
            let resolve_fn = crate::intrinsics::make_native("<promise-resolve>", move |rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                resolve_promise(rt, p_for_resolve, v);
                Ok(Value::Undefined)
            });
            let reject_fn = crate::intrinsics::make_native("<promise-reject>", move |rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                reject_promise(rt, p_for_reject, v);
                Ok(Value::Undefined)
            });
            let resolve_id = rt.alloc_object(resolve_fn);
            let reject_id = rt.alloc_object(reject_fn);
            // Synchronously invoke executor(resolve, reject) per spec.
            let _ = rt.call_function(executor, Value::Undefined, vec![
                Value::Object(resolve_id),
                Value::Object(reject_id),
            ])?;
            Ok(Value::Object(p))
        });
        let promise_obj = self.alloc_object(promise_ctor);
        // Ω.5.P63.E5: Promise.resolve / Promise.reject routed through
        // IR-lowered generated::promise_resolve / promise_reject.
        // Underlying helpers (rt.promise_resolve_via / rt.promise_reject_via)
        // still call new_promise + resolve_promise / reject_promise; the
        // IR is a 2-step wrapper that exposes the spec's PromiseResolve
        // / PromiseReject step granularity to the linter.
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "resolve", 1, |rt, args| {
            crate::generated::promise_resolve(rt, Value::Undefined, args)
        });
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "reject", 1, |rt, args| {
            crate::generated::promise_reject(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E52: Promise.then / catch_ routed through IR.
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "then", 3, |rt, args| {
            crate::generated::promise_prototype_then(rt, rt.current_this(), args)
        });
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "catch_", 1, |rt, args| {
            crate::generated::promise_prototype_catch(rt, rt.current_this(), args)
        });
        // Ω.5.P61.E11: Promise.all / allSettled / any / race / withResolvers
        // per ECMA §27.2.4. v1 implementations leverage cruftless's
        // synchronously-settled promise machinery — since most consumer
        // module-init paths use already-settled promises (Promise.resolve/
        // reject wrappers), the synchronous-iteration approximation closes
        // a large fraction of test262 cases without requiring full async
        // event-loop choreography.
        // Ω.5.P63.E52: Promise.all / allSettled / any / race routed through IR.
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "all", 1, |rt, args| {
            crate::generated::promise_all(rt, rt.current_this(), args)
        });
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "allSettled", 1, |rt, args| {
            crate::generated::promise_all_settled(rt, rt.current_this(), args)
        });
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "any", 1, |rt, args| {
            crate::generated::promise_any(rt, rt.current_this(), args)
        });
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "race", 1, |rt, args| {
            crate::generated::promise_race(rt, rt.current_this(), args)
        });
        // Ω.5.P63.E55 Stage 2: routed through IR (uses Expr::Closure for the
        // resolve/reject functions; first IR section to demonstrate the
        // alphabet-closures primitive).
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "withResolvers", 0, |rt, args| {
            crate::generated::promise_with_resolvers(rt, rt.current_this(), args)
        });
        if let Some(proto) = self.promise_prototype {
            self.obj_mut(promise_obj).set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P63.E52: Promise.prototype[@@toStringTag] = "Promise" per §27.2.5.5.
            // {writable:false, enumerable:false, configurable:true} — set_own_frozen
            // gives configurable:false (frozen), but the spec is c:true. Use a
            // hand-set descriptor.
            self.obj_mut(proto).dict_mut().insert("@@toStringTag".into(),
                crate::value::PropertyDescriptor {
                    value: Value::String(Rc::new("Promise".into())),
                    writable: false, enumerable: false, configurable: true,
                    getter: None, setter: None,
                });
            // Ω.5.P58.E7: Promise.prototype.constructor = Promise per ECMA §27.2.5.
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(promise_obj));
            // Ω.5.P61.E11: Promise.prototype.finally per ECMA §27.2.5.3.
            // Ω.5.P63.E52: Promise.prototype.finally routed through IR.
            // The via-helper expects args[0]=source (current_this), args[1..]=user args;
            // wire by prepending current_this then forwarding the user args.
            crate::intrinsics::register_intrinsic_method(self, proto, "finally", 1, |rt, args| {
                let mut a: Vec<Value> = Vec::with_capacity(args.len() + 1);
                a.push(rt.current_this());
                a.extend(args.iter().cloned());
                crate::generated::promise_prototype_finally(rt, rt.current_this(), &a)
            });
        }
        self.globals.insert("Promise".into(), Value::Object(promise_obj));
    }
}

/// Create a new Pending Promise object on the managed heap.
pub fn new_promise(rt: &mut Runtime) -> ObjectRef {
    // Ω.5.P58.E7: set [[Prototype]] to Promise.prototype so
    // `promise.constructor === Promise` per ECMA §27.2.3.1. execa /
    // yeoman / many libs walk `p.constructor.prototype` at module-init
    // to recover the native Promise prototype.
    let proto = rt.promise_prototype;
    rt.alloc_object(Object {
        proto,
        extensible: true,
        properties: indexmap::IndexMap::new(),
        internal_kind: InternalKind::Promise(PromiseState {
            status: PromiseStatus::Pending,
            value: Value::Undefined,
            fulfill_reactions: Vec::new(),
            reject_reactions: Vec::new(),
        }),
    
        ..Default::default()
    })
}

pub fn resolve_promise(rt: &mut Runtime, promise: ObjectRef, value: Value) {
    let reactions = {
        let p = rt.obj_mut(promise);
        if let InternalKind::Promise(ps) = &mut p.internal_kind {
            if !matches!(ps.status, PromiseStatus::Pending) { return; }
            ps.status = PromiseStatus::Fulfilled;
            ps.value = value;
            std::mem::take(&mut ps.fulfill_reactions)
        } else { return; }
    };
    let value = match &rt.obj(promise).internal_kind {
        InternalKind::Promise(ps) => ps.value.clone(),
        _ => Value::Undefined,
    };
    for reaction in reactions {
        enqueue_reaction(rt, reaction.handler, value.clone(), reaction.chain, false);
    }
}

pub fn reject_promise(rt: &mut Runtime, promise: ObjectRef, reason: Value) {
    let reactions = {
        let p = rt.obj_mut(promise);
        if let InternalKind::Promise(ps) = &mut p.internal_kind {
            if !matches!(ps.status, PromiseStatus::Pending) { return; }
            ps.status = PromiseStatus::Rejected;
            ps.value = reason;
            std::mem::take(&mut ps.reject_reactions)
        } else { return; }
    };
    // Per §27.2.1.9 HostPromiseRejectionTracker: a rejection landing with
    // no reject reaction attached is a candidate unhandled rejection.
    // .then / .catch_ removes the entry if a handler attaches later (still
    // valid only because the source promise is already Rejected at that
    // point, so the spec-side "unhandledrejection" event timing collapses).
    if reactions.is_empty() {
        rt.pending_unhandled.insert(promise);
    }
    let value = match &rt.obj(promise).internal_kind {
        InternalKind::Promise(ps) => ps.value.clone(),
        _ => Value::Undefined,
    };
    for reaction in reactions {
        enqueue_reaction(rt, reaction.handler, value.clone(), reaction.chain, true);
    }
}

pub(crate) fn enqueue_reaction(
    rt: &mut Runtime,
    handler: Option<Value>,
    value: Value,
    chain: ObjectRef,
    is_rejected: bool,
) {
    rt.enqueue_microtask("PromiseReactionJob", move |rt| {
        match handler {
            Some(h) => {
                match rt.call_function(h, Value::Undefined, vec![value]) {
                    Ok(result) => { resolve_promise(rt, chain, result); }
                    Err(e) => {
                        let thrown = match e {
                            RuntimeError::Thrown(v) => v,
                            other => Value::String(std::rc::Rc::new(format!("{:?}", other))),
                        };
                        reject_promise(rt, chain, thrown);
                    }
                }
            }
            None => {
                if is_rejected {
                    reject_promise(rt, chain, value);
                } else {
                    resolve_promise(rt, chain, value);
                }
            }
        }
        Ok(())
    });
}

fn register_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, name, 0.0);
    let fn_obj = Object {
        proto: None,
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals { name: name.to_string(), length: 0, native, is_constructor: true }),
    
        ..Default::default()
    };
    let fn_id = rt.alloc_object(fn_obj);
    rt.object_set(host, name.into(), Value::Object(fn_id));
}
