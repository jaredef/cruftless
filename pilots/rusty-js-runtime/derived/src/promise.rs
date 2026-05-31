//! Promise intrinsic + reaction routing through the JobQueue per
//! ECMA-262 §27.2 + the Doc 714 §VI Consequence 5 architectural shift.
//!
//! Round 3.e.d migrates Promise objects onto the managed heap. Promise
//! state + reactions are accessed through `rt.obj(id)` / `rt.obj_mut(id)`.

use crate::interp::{Runtime, RuntimeError};
use crate::value::{
    FunctionInternals, InternalKind, NativeFn, Object, ObjectRef, PromiseReaction, PromiseState,
    PromiseStatus, Value,
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
                    "Promise constructor must be called with new".into(),
                ));
            }
            let executor = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&executor) {
                return Err(RuntimeError::TypeError(
                    "Promise constructor: executor must be callable".into(),
                ));
            }
            let p = new_promise(rt);
            // PSCV-EXT 1: per §27.2.3.1 step 3 OrdinaryCreateFromConstructor —
            // honor NewTarget so subclass derivation (`class SubP extends
            // Promise`) wires the result's [[Prototype]] to SubP.prototype.
            // Without this, `new SubP(...) instanceof SubP` is false and
            // Promise.all.call(SubP, ...) returns a default Promise.
            if let Some(nt) = rt.current_new_target.clone() {
                if let Value::Object(nt_id) = nt {
                    let proto = rt.object_get(nt_id, "prototype");
                    if let Value::Object(pid) = proto {
                        rt.obj_mut(p).proto = Some(pid);
                    }
                }
            }
            let p_for_resolve = p;
            let p_for_reject = p;
            // PEFM-EXT 1: per ECMA-262 §27.2.1.3.1/§27.2.1.3.2 the
            // Promise resolve/reject functions have name="" and length=1.
            // Previously named "<promise-resolve>"/"<promise-reject>" with
            // length=0 — visible to consumer code via fn.name/fn.length.
            let resolve_fn = crate::intrinsics::make_native_with_length("", 1, move |rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                resolve_promise(rt, p_for_resolve, v);
                Ok(Value::Undefined)
            });
            let reject_fn = crate::intrinsics::make_native_with_length("", 1, move |rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                reject_promise(rt, p_for_reject, v);
                Ok(Value::Undefined)
            });
            let resolve_id = rt.alloc_object(resolve_fn);
            let reject_id = rt.alloc_object(reject_fn);
            // Synchronously invoke executor(resolve, reject) per spec.
            let _ = rt.call_function(
                executor,
                Value::Undefined,
                vec![Value::Object(resolve_id), Value::Object(reject_id)],
            )?;
            Ok(Value::Object(p))
        });
        let promise_obj = self.alloc_object(promise_ctor);
        // Ω.5.P63.E5: Promise.resolve / Promise.reject routed through
        // IR-lowered generated::promise_resolve / promise_reject.
        // Underlying helpers (rt.promise_resolve_via / rt.promise_reject_via)
        // still call new_promise + resolve_promise / reject_promise; the
        // IR is a 2-step wrapper that exposes the spec's PromiseResolve
        // / PromiseReject step granularity to the linter.
        // PSCV-EXT 0 (Promise static-method ctx-validation): per ECMA-262
        // §27.2.4.4 + §27.2.4.7 step 1-2 "Let C be the this value. If
        // Type(C) is not Object, throw TypeError." Closure inlines the
        // Object check before delegating to the generated *_via path.
        crate::intrinsics::register_intrinsic_method(
            self,
            promise_obj,
            "resolve",
            1,
            move |rt, args| {
                let c = rt.current_this();
                if !matches!(c, Value::Object(_)) {
                    return Err(RuntimeError::TypeError(
                        "Promise.resolve: this is not an Object".into(),
                    ));
                }
                let x = args.first().cloned().unwrap_or(Value::Undefined);
                // PRESOLVE-EXT 1 §27.2.4.7 PromiseResolve step 1: if
                // IsPromise(x), let xConstructor be Get(x, "constructor"),
                // and if SameValue(xConstructor, C), return x.  Spec routes
                // ALL other cases through NewPromiseCapability(C) +
                // cap.[[Resolve]](x), so x.constructor=null or a different
                // ctor results in a fresh C-shaped capability promise.
                if let Value::Object(xid) = &x {
                    if matches!(rt.obj(*xid).internal_kind, InternalKind::Promise(_)) {
                        let xc = rt.spec_get(&x, "constructor")?;
                        if matches!((&xc, &c), (Value::Object(a), Value::Object(b)) if a == b) {
                            return Ok(x);
                        }
                    }
                }
                let (cap_promise, cap_resolve, _cap_reject) =
                    rt.new_promise_capability(&c)?;
                rt.call_function(cap_resolve, Value::Undefined, vec![x])?;
                Ok(cap_promise)
            },
        );
        crate::intrinsics::register_intrinsic_method(
            self,
            promise_obj,
            "reject",
            1,
            move |rt, args| {
                let c = rt.current_this();
                if !matches!(c, Value::Object(_)) {
                    return Err(RuntimeError::TypeError(
                        "Promise.reject: this is not an Object".into(),
                    ));
                }
                let r = args.first().cloned().unwrap_or(Value::Undefined);
                // §27.2.4.4 Promise.reject always uses NewPromiseCapability(C)
                // + cap.[[Reject]](r); there's no IsPromise short-circuit.
                let (cap_promise, _cap_resolve, cap_reject) = rt.new_promise_capability(&c)?;
                rt.call_function(cap_reject, Value::Undefined, vec![r])?;
                Ok(cap_promise)
            },
        );
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
        // PSCV-EXT 0: §27.2.4.1/2/3/5 step 1-2 "Let C be the this value.
        // If Type(C) is not Object, throw TypeError." Closure inlines
        // the Object check; the silent default-Promise fallback in the
        // *_via methods is preserved for the case where C is an Object
        // but not callable (NewPromiseCapability then throws inside).
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "all", 1, |rt, args| {
            let c = rt.current_this();
            if !matches!(c, Value::Object(_)) {
                return Err(RuntimeError::TypeError(
                    "Promise.all: this is not an Object".into(),
                ));
            }
            crate::generated::promise_all(rt, c, args)
        });
        crate::intrinsics::register_intrinsic_method(
            self,
            promise_obj,
            "allSettled",
            1,
            |rt, args| {
                let c = rt.current_this();
                if !matches!(c, Value::Object(_)) {
                    return Err(RuntimeError::TypeError(
                        "Promise.allSettled: this is not an Object".into(),
                    ));
                }
                crate::generated::promise_all_settled(rt, c, args)
            },
        );
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "any", 1, |rt, args| {
            let c = rt.current_this();
            if !matches!(c, Value::Object(_)) {
                return Err(RuntimeError::TypeError(
                    "Promise.any: this is not an Object".into(),
                ));
            }
            crate::generated::promise_any(rt, c, args)
        });
        crate::intrinsics::register_intrinsic_method(self, promise_obj, "race", 1, |rt, args| {
            let c = rt.current_this();
            if !matches!(c, Value::Object(_)) {
                return Err(RuntimeError::TypeError(
                    "Promise.race: this is not an Object".into(),
                ));
            }
            crate::generated::promise_race(rt, c, args)
        });
        // PAKD-EXT 0: Promise.allKeyed + Promise.allSettledKeyed per the
        // await-dictionary Stage 1 proposal. Input is an Object (own
        // enumerable string keys); output is a null-proto Object with
        // the same keys mapped to resolved values (allKeyed) or
        // {status, value/reason} entries (allSettledKeyed).
        crate::intrinsics::register_intrinsic_method(
            self,
            promise_obj,
            "allKeyed",
            1,
            |rt, args| {
                let c = rt.current_this();
                if !matches!(c, Value::Object(_)) {
                    return Err(RuntimeError::TypeError(
                        "Promise.allKeyed: this is not an Object".into(),
                    ));
                }
                rt.promise_all_keyed_via(args)
            },
        );
        crate::intrinsics::register_intrinsic_method(
            self,
            promise_obj,
            "allSettledKeyed",
            1,
            |rt, args| {
                let c = rt.current_this();
                if !matches!(c, Value::Object(_)) {
                    return Err(RuntimeError::TypeError(
                        "Promise.allSettledKeyed: this is not an Object".into(),
                    ));
                }
                rt.promise_all_settled_keyed_via(args)
            },
        );
        // Ω.5.P63.E55 Stage 2: routed through IR (uses Expr::Closure for the
        // resolve/reject functions; first IR section to demonstrate the
        // alphabet-closures primitive).
        crate::intrinsics::register_intrinsic_method(
            self,
            promise_obj,
            "withResolvers",
            0,
            |rt, args| crate::generated::promise_with_resolvers(rt, rt.current_this(), args),
        );
        if let Some(proto) = self.promise_prototype {
            self.obj_mut(promise_obj)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P63.E52: Promise.prototype[@@toStringTag] = "Promise" per §27.2.5.5.
            // {writable:false, enumerable:false, configurable:true} — set_own_frozen
            // gives configurable:false (frozen), but the spec is c:true. Use a
            // hand-set descriptor.
            self.obj_mut(proto).dict_mut().insert(
                "@@toStringTag".into(),
                crate::value::PropertyDescriptor {
                    value: Value::String(Rc::new("Promise".into())),
                    writable: false,
                    enumerable: false,
                    configurable: true,
                    getter: None,
                    setter: None,
                },
            );
            // Ω.5.P58.E7: Promise.prototype.constructor = Promise per ECMA §27.2.5.
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(promise_obj));
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
        self.define_global_property("Promise", Value::Object(promise_obj));
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
    if let Value::Object(value_id) = value.clone() {
        if value_id == promise {
            reject_promise(
                rt,
                promise,
                Value::String(std::rc::Rc::new(
                    "Promise cannot resolve to itself".to_string(),
                )),
            );
            return;
        }
        let maybe_promise = {
            let o = rt.obj(value_id);
            match &o.internal_kind {
                InternalKind::Promise(ps) => Some((ps.status, ps.value.clone())),
                _ => None,
            }
        };
        if let Some((status, settled_value)) = maybe_promise {
            match status {
                PromiseStatus::Pending => {
                    let src = rt.obj_mut(value_id);
                    if let InternalKind::Promise(ps) = &mut src.internal_kind {
                        ps.fulfill_reactions.push(PromiseReaction {
                            handler: None,
                            chain: promise,
                        });
                        ps.reject_reactions.push(PromiseReaction {
                            handler: None,
                            chain: promise,
                        });
                    }
                    return;
                }
                PromiseStatus::Fulfilled => {
                    resolve_promise(rt, promise, settled_value);
                    return;
                }
                PromiseStatus::Rejected => {
                    reject_promise(rt, promise, settled_value);
                    return;
                }
            }
        }
        // PRESOLVE-EXT 0: thenable resolution per §27.2.1.4. If value is an
        // Object whose .then property is callable, defer fulfillment to a
        // thenable-resolution job that calls value.then(resolveFn, rejectFn).
        // If accessing .then throws, reject the promise with the thrown error.
        // If .then is non-callable, fall through to ordinary fulfillment.
        let then_v = match rt.spec_get(&value, "then") {
            Ok(v) => v,
            Err(e) => {
                let reason = match e {
                    crate::interp::RuntimeError::Thrown(v) => v,
                    other => {
                        let msg = format!("{:?}", other);
                        Value::String(std::rc::Rc::new(msg))
                    }
                };
                reject_promise(rt, promise, reason);
                return;
            }
        };
        if rt.is_callable(&then_v) {
            // Synchronously invoke thenable.then(resolveFn, rejectFn) — our
            // promise model already settles synchronously elsewhere, so the
            // microtask-job deferral isn't load-bearing here.
            let p_resolve = promise;
            let resolve_fn = crate::intrinsics::make_native_with_length("", 1, move |rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                resolve_promise(rt, p_resolve, v);
                Ok(Value::Undefined)
            });
            let p_reject = promise;
            let reject_fn = crate::intrinsics::make_native_with_length("", 1, move |rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                reject_promise(rt, p_reject, v);
                Ok(Value::Undefined)
            });
            let resolve_id = rt.alloc_object(resolve_fn);
            let reject_id = rt.alloc_object(reject_fn);
            if let Err(e) = rt.call_function(
                then_v,
                value,
                vec![Value::Object(resolve_id), Value::Object(reject_id)],
            ) {
                let reason = match e {
                    crate::interp::RuntimeError::Thrown(v) => v,
                    other => Value::String(std::rc::Rc::new(format!("{:?}", other))),
                };
                reject_promise(rt, promise, reason);
            }
            return;
        }
    }
    let reactions = {
        let p = rt.obj_mut(promise);
        if let InternalKind::Promise(ps) = &mut p.internal_kind {
            if !matches!(ps.status, PromiseStatus::Pending) {
                return;
            }
            ps.status = PromiseStatus::Fulfilled;
            ps.value = value;
            std::mem::take(&mut ps.fulfill_reactions)
        } else {
            return;
        }
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
            if !matches!(ps.status, PromiseStatus::Pending) {
                return;
            }
            ps.status = PromiseStatus::Rejected;
            ps.value = reason;
            std::mem::take(&mut ps.reject_reactions)
        } else {
            return;
        }
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
            Some(h) => match rt.call_function(h, Value::Undefined, vec![value]) {
                Ok(result) => {
                    resolve_promise(rt, chain, result);
                }
                Err(e) => {
                    let thrown = match e {
                        RuntimeError::Thrown(v) => v,
                        other => Value::String(std::rc::Rc::new(format!("{:?}", other))),
                    };
                    reject_promise(rt, chain, thrown);
                }
            },
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
where
    F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
{
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, name, 0.0);
    let fn_obj = Object {
        proto: None,
        extensible: true,
        properties,
        // NACR-EXT 1: Promise.prototype.{then, catch, finally} are
        // non-constructors per ECMA-262 §27.2.5. Pre-fix is_constructor=true.
        internal_kind: InternalKind::Function(FunctionInternals {
            name: name.to_string(),
            length: 0,
            native,
            is_constructor: false,
        }),

        ..Default::default()
    };
    let fn_id = rt.alloc_object(fn_obj);
    rt.object_set(host, name.into(), Value::Object(fn_id));
}
