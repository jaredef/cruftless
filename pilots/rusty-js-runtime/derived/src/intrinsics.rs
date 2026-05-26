//! Built-in intrinsics — minimal v1 surface for the parity-119 corpus.
//! Per specs/rusty-js-runtime-design.md §V.
//!
//! Round 3.d.e scope:
//! - Global functions: parseInt, parseFloat, isNaN, isFinite
//! - Math intrinsic: abs, floor, ceil, round, trunc, sqrt, pow, max, min,
//!   sign, exp, log, sin, cos, tan, random, PI, E, LN2, LN10
//! - JSON intrinsic: stringify (limited), parse (limited)
//! - Number static: parseInt, parseFloat, isNaN, isFinite, isInteger,
//!   isSafeInteger, MAX_SAFE_INTEGER, MAX_VALUE, etc.
//! - Console.log

use crate::abstract_ops;
use crate::interp::{Runtime, RuntimeError};
use crate::value::{
    FunctionInternals, InternalKind, NativeFn, Object, ObjectRef, PropertyDescriptor, Value,
};
use std::collections::HashMap;
use std::rc::Rc;

/// CAPS-EXT 10: gate a stdio operation through the capability dispatcher.
/// Same shape as host-v2's check_fs / check_process / check_env helpers.
/// Lives in the runtime crate because the console intrinsic is installed
/// here (rather than in host-v2).
fn check_stdio(rt: &Runtime, op: crate::caps::StdioOp) -> Result<(), RuntimeError> {
    let url = rt.current_module_url.last().cloned().unwrap_or_default();
    let provenance = if url.contains("/node_modules/") {
        crate::caps::ModuleProvenance::Dependency
    } else if url.starts_with("node:") {
        crate::caps::ModuleProvenance::Builtin
    } else {
        crate::caps::ModuleProvenance::Application
    };
    let caller = crate::caps::ModuleId { url, provenance };
    rt.caps
        .require_stdio(&crate::caps::Stdio::none(), op, &caller)
        .map_err(|e| RuntimeError::TypeError(e.to_string()))
}

impl Runtime {
    pub fn install_intrinsics(&mut self) {
        // JIT-EXT 22: register the runtime-side GetPropOnObject helper
        // with the JIT crate's function-pointer indirection. Idempotent
        // (setting the same fn twice is a no-op), so calling it from
        // install_intrinsics — which runs once per Runtime — is correct.
        Self::install_jit_getprop_helper();

        // Prototype intrinsics must install first so subsequent alloc_object
        // calls (Math/JSON/console hosts, Promise) inherit from
        // Object.prototype. Tier-Ω.5.a.
        self.install_prototypes();
        self.install_globals();
        self.install_object_static();
        self.install_array_static();
        self.install_symbol_static();
        self.install_number_static();
        self.install_math();
        self.install_json();
        // TF-EXT 1: Temporal foundation. Registers the Temporal namespace
        // and stub class identifiers per pilots/temporal-implementation/
        // temporal-foundation/. No operative methods yet; per-class
        // implementation lands in sub-locales (temporal-now, etc.).
        self.install_temporal();
        self.install_console();
        self.install_promise();
        // diff-prod Rung-19 continuation: Iterator helpers + ES2024–26 batch.
        // Must run after install_promise (Promise.try needs the global) and
        // after install_map_set_globals + install_error_globals (Map.groupBy
        // + Error.isError need theirs). install_intrinsics enforces those
        // orderings already; the call lands after them.
        // (The actual call is moved below to land after all dependencies.)
        self.install_regexp();
        self.install_test_record();
        self.install_destructure_helpers();
        self.install_destructure_iter_helpers();
        self.install_spread_helpers();
        self.install_compartment();
        // Tier-Ω.5.P17.E2: dynamic import() walks the real module resolver
        // (was: returned an unconditionally-rejected Promise per Ω.5.CCCCCCC
        // stub). Routes through the same `resolve_module_full` + `load_module`
        // / `resolve_builtin_namespace` pipeline that static `import` uses.
        // The loader is synchronous, so the returned Promise is synchronously
        // settled — fulfilled with the module namespace on success, rejected
        // with a string reason on failure. The compiler's `__await` lowering
        // (Ω.5.P17.E1) then unwraps it on the same tick.
        //
        // Parent URL is synthetic — bare and `node:` specifiers don't consult
        // it. Relative specifiers in dynamic imports would need real caller-
        // frame plumbing; deferred until a consumer needs it.
        register_engine_helper(self, "__dynamic_import", |rt, args| {
            let spec = args
                .first()
                .map(|v| crate::abstract_ops::to_string(v).as_str().to_string())
                .unwrap_or_else(|| "<unknown>".into());
            let p = crate::promise::new_promise(rt);
            // Ω.5.P45.E1: parent URL is the URL of the calling module if
            // we're inside one (via `current_module_url` stack pushed by
            // evaluate_module/evaluate_cjs_module). Falls back to the
            // process cwd for the top-level case (script run directly,
            // not from inside a loaded module body). Closes nx and similar
            // packages whose internal `import('../src/native/X.js')` needs
            // to resolve relative to the importing file, not the script's
            // cwd.
            let parent = if let Some(url) = rt.current_module_url.last() {
                url.clone()
            } else {
                let cwd = std::env::current_dir()
                    .ok()
                    .and_then(|p| p.to_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "/".to_string());
                format!("file://{}/__dynamic_import__", cwd)
            };
            let resolved =
                match rt.resolve_module_full(&parent, &spec, crate::module::ModuleKind::ESM) {
                    Ok(r) => r,
                    Err(e) => {
                        // Ω.5.P58.E5: same Error-instance reject as the load-failed
                        // branch below.
                        let message = format!("dynamic import('{}') resolve failed: {:?}", spec, e);
                        let err_id = make_error_instance(rt, "TypeError", &message);
                        let reason = match err_id {
                            Some(id) => Value::Object(id),
                            None => Value::String(Rc::new(format!("TypeError: {}", message))),
                        };
                        crate::promise::reject_promise(rt, p, reason);
                        return Ok(Value::Object(p));
                    }
                };
            let ns_result = if resolved.starts_with("node:") {
                rt.resolve_builtin_namespace(&resolved)
            } else {
                rt.load_module(&resolved)
            };
            match ns_result {
                Ok(ns) => crate::promise::resolve_promise(rt, p, Value::Object(ns)),
                Err(e) => {
                    // Ω.5.P51.E5: extract a readable message for Thrown(Object)
                    // values. Got and other libraries throw Error instances at
                    // module-init whose useful info lives on the .message and
                    // .name properties; rb's Debug format printed Object IDs
                    // like '[Object #4144]', erasing the diagnostic content.
                    let detail = describe_thrown_for_diag(rt, &e);
                    // Ω.5.P58.E5: reject with a real TypeError-instance, not a
                    // Value::String. Bun rejects dynamic-import failures with
                    // Error instances; consumer catch handlers do
                    // `e instanceof Error`, read `e.message`, dispatch on
                    // `e.constructor.name`. Pre-P58.E5 cruftless rejected with
                    // a string, breaking those patterns and projecting onto
                    // the parity probe as `error:"String"` (cf. ast-types,
                    // many others). Construct the instance by looking up the
                    // global TypeError ctor's prototype and assembling an
                    // ordinary object with the spec-mandated {name, message,
                    // stack} surface.
                    let message = format!("dynamic import('{}') load failed: {}", spec, detail);
                    let err_id = make_error_instance(rt, "TypeError", &message);
                    let reason = match err_id {
                        Some(id) => Value::Object(id),
                        None => Value::String(Rc::new(format!("TypeError: {}", message))),
                    };
                    crate::promise::reject_promise(rt, p, reason);
                }
            }
            Ok(Value::Object(p))
        });
        // Tier-Ω.5.P17.E1: synchronous unwrap of already-settled Promises.
        // Paired with the compiler's `await` → `__await(expr)` lowering.
        // - Non-Promise value: returned unchanged (spec: `await v` on a
        //   non-thenable yields v).
        // - Fulfilled Promise: returns the resolved value; clears any
        //   pending-unhandled bookkeeping.
        // - Rejected Promise: throws the rejection reason via RuntimeError::
        //   Thrown so the surrounding try/catch behaves as ECMA-262 requires.
        // - Pending Promise: errors with TypeError. Real suspension would
        //   require frame park/resume; deferred. The dynamic-import path
        //   synthesizes synchronously-settled Promises, so the probe never
        //   hits this branch.
        // Ω.5.P54.E1 (Axis-M probe — Doc 729 §XII surface):
        // __resolution_trace(spec_or_url) returns the captured entry-point
        // decision string. Walks the trace map by exact URL key first,
        // then by substring match against the spec the trace recorded.
        // Diagnostic-only; no behavior change. Lets parity probes ask the
        // engine "which file did you actually pick?" so Axis-M wrong-file
        // picks (heap-js .umd over .es5, mri-class divergences) become
        // observable from JS-side test scripts rather than requiring
        // engine recompilation with a debug print.
        register_global_fn(self, "__resolution_trace", |rt, args| {
            let q = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => return Ok(Value::Undefined),
            };
            if let Some(t) = rt.module_resolution_trace.get(&q) {
                return Ok(Value::String(std::rc::Rc::new(t.clone())));
            }
            for (url, t) in rt.module_resolution_trace.iter() {
                if t.contains(&format!("spec='{}'", q)) || url.contains(&q) {
                    return Ok(Value::String(std::rc::Rc::new(t.clone())));
                }
            }
            Ok(Value::Undefined)
        });
        // Ω.5.P54.E2 (Axis-E probe surface): __post_eval_trace(spec_or_url)
        // returns the post-evaluation observation for a module:
        // "kind=ESM|CJS key_count=N status=... exports_reassigned=...".
        // Empty-namespace results are the predicate Axis-E catches; this
        // surface lets parity probes query them.
        register_global_fn(self, "__post_eval_trace", |rt, args| {
            let q = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => return Ok(Value::Undefined),
            };
            if let Some(t) = rt.module_post_eval_trace.get(&q) {
                return Ok(Value::String(std::rc::Rc::new(t.clone())));
            }
            for (url, t) in rt.module_post_eval_trace.iter() {
                if url.contains(&q) {
                    return Ok(Value::String(std::rc::Rc::new(t.clone())));
                }
            }
            Ok(Value::Undefined)
        });
        // Ω.5.P54.E3 (Axis-N probe surface): __ns_synth_trace(spec_or_url)
        // returns the namespace-synthesis-path tag recorded by the ESM
        // FinalizeModuleNamespace hook (and, when threaded, the CJS
        // populator). Names which branch composed the surface.
        register_global_fn(self, "__ns_synth_trace", |rt, args| {
            let q = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => return Ok(Value::Undefined),
            };
            if let Some(t) = rt.module_ns_synth_trace.get(&q) {
                return Ok(Value::String(std::rc::Rc::new(t.clone())));
            }
            for (url, t) in rt.module_ns_synth_trace.iter() {
                if url.contains(&q) {
                    return Ok(Value::String(std::rc::Rc::new(t.clone())));
                }
            }
            Ok(Value::Undefined)
        });
        // Ω.5.P54.E4/E5/E6 (Axis-S / Axis-H / Axis-O probe surfaces).
        // Each returns the accumulated miss list (S, H) or trace map (O).
        register_global_fn(self, "__symbol_lookup_log", |rt, _args| {
            let s = rt.symbol_lookup_miss_log.join(" | ");
            Ok(Value::String(std::rc::Rc::new(s)))
        });
        register_global_fn(self, "__host_stub_log", |rt, _args| {
            let s = rt.host_stub_miss_log.join(" | ");
            Ok(Value::String(std::rc::Rc::new(s)))
        });
        register_global_fn(self, "__operator_trace_size", |rt, _args| {
            Ok(Value::Number(rt.operator_lowering_trace.len() as f64))
        });
        // RS-EXT 2e: realm-isolated eval. Allocates a fresh realm,
        // enters it, evaluates the source, exits, returns the result.
        // The fresh realm has its own cloned Array.prototype / Object.
        // prototype / Function.prototype, so mutations inside the source
        // don't leak to the primordial realm. Used by the realm-substrate
        // prototype-pollution probe per Pred-rs.2.
        register_engine_helper(self, "__cruftless_eval_realm", |rt, args| {
            let source = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => return Ok(Value::Undefined),
            };
            let new_realm = rt.allocate_realm();
            let prior = rt.enter_realm(new_realm);
            let url = format!("file://<realm-eval:{}>", new_realm);
            let result = rt.evaluate_module(&source, &url);
            rt.exit_realm(prior);
            match result {
                Ok(_) => Ok(Value::Undefined),
                Err(RuntimeError::CompileError(msg)) => Err(RuntimeError::SyntaxError(msg)),
                Err(e) => Err(e),
            }
        });
        register_engine_helper(self, "__await", |rt, args| {
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            let id = match v {
                Value::Object(id) => id,
                other => return Ok(other),
            };
            let (is_promise, status, value) = {
                let o = rt.obj(id);
                if let InternalKind::Promise(ps) = &o.internal_kind {
                    (true, ps.status, ps.value.clone())
                } else {
                    (
                        false,
                        crate::value::PromiseStatus::Pending,
                        Value::Undefined,
                    )
                }
            };
            if !is_promise {
                return Ok(Value::Object(id));
            }
            match status {
                crate::value::PromiseStatus::Fulfilled => {
                    rt.pending_unhandled.remove(&id);
                    Ok(value)
                }
                crate::value::PromiseStatus::Rejected => {
                    rt.pending_unhandled.remove(&id);
                    Err(RuntimeError::Thrown(value))
                }
                crate::value::PromiseStatus::Pending => {
                    // v1 stand-in for proper frame park/resume: pump the
                    // event loop synchronously until the awaited Promise
                    // settles. Real suspension is queued as its own rung;
                    // this unblocks any program whose await target is
                    // settleable by draining queues (Promise.allSettled,
                    // Promise.race against resolved, await setTimeout).
                    let max_pumps = 100_000usize;
                    let mut pumps = 0usize;
                    loop {
                        let did_work = crate::job_queue::pump_one_tick(rt)?;
                        // Re-check promise status.
                        let (status, value) = {
                            let o = rt.obj(id);
                            if let InternalKind::Promise(ps) = &o.internal_kind {
                                (ps.status, ps.value.clone())
                            } else {
                                return Err(RuntimeError::TypeError(
                                    "await: lost-track on Promise during pump".into(),
                                ));
                            }
                        };
                        match status {
                            crate::value::PromiseStatus::Fulfilled => {
                                rt.pending_unhandled.remove(&id);
                                return Ok(value);
                            }
                            crate::value::PromiseStatus::Rejected => {
                                rt.pending_unhandled.remove(&id);
                                return Err(RuntimeError::Thrown(value));
                            }
                            crate::value::PromiseStatus::Pending => {}
                        }
                        if !did_work {
                            // Try poll_io once before declaring idle.
                            let progressed = if let Some(poll) = rt.host_hooks.poll_io.take() {
                                let p = poll(rt)?;
                                rt.host_hooks.poll_io = Some(poll);
                                p
                            } else {
                                false
                            };
                            if !progressed {
                                return Err(RuntimeError::TypeError(
                                    "await: Promise never settled (event loop idle)".into(),
                                ));
                            }
                        }
                        pumps += 1;
                        if pumps > max_pumps {
                            return Err(RuntimeError::TypeError(
                                "await: max-pump bound exceeded (likely self-pending promise cycle)".into()));
                        }
                    }
                }
            }
        });
        // Tier-Ω.5.P26.E1.webassembly-stub: minimum-viable WebAssembly
        // global so packages that capture WebAssembly.compile / .instantiate
        // / .Module at module init don't crash on `undefined.compile`.
        // Surfaced through Ω.5.P24.E1 proto-chain probe walking
        // @actions/http-client (whose `lazyllhttp` shim calls
        // WebAssembly.compile during require). All methods return rejected
        // Promises or throw; consumers that actually run wasm fail later
        // with a clear "WebAssembly not implemented" error, but the
        // module-load gate is closed.
        let wasm = self.alloc_object(Object::new_ordinary());
        let unsupported = || -> RuntimeError {
            RuntimeError::TypeError("WebAssembly not implemented (Tier-Ω.5.P26.E1 stub)".into())
        };
        register_method(self, wasm, "compile", move |rt, _args| {
            let p = crate::promise::new_promise(rt);
            crate::promise::reject_promise(
                rt,
                p,
                Value::String(Rc::new(
                    "TypeError: WebAssembly.compile not implemented (Tier-Ω.5.P26.E1 stub)".into(),
                )),
            );
            Ok(Value::Object(p))
        });
        register_method(self, wasm, "instantiate", move |rt, _args| {
            let p = crate::promise::new_promise(rt);
            crate::promise::reject_promise(
                rt,
                p,
                Value::String(Rc::new(
                    "TypeError: WebAssembly.instantiate not implemented (Tier-Ω.5.P26.E1 stub)"
                        .into(),
                )),
            );
            Ok(Value::Object(p))
        });
        register_method(self, wasm, "compileStreaming", move |rt, _args| {
            let p = crate::promise::new_promise(rt);
            crate::promise::reject_promise(rt, p, Value::String(Rc::new(
                "TypeError: WebAssembly.compileStreaming not implemented (Tier-Ω.5.P26.E1 stub)".into()
            )));
            Ok(Value::Object(p))
        });
        register_method(self, wasm, "instantiateStreaming", move |rt, _args| {
            let p = crate::promise::new_promise(rt);
            crate::promise::reject_promise(rt, p, Value::String(Rc::new(
                "TypeError: WebAssembly.instantiateStreaming not implemented (Tier-Ω.5.P26.E1 stub)".into()
            )));
            Ok(Value::Object(p))
        });
        register_method(self, wasm, "validate", |_rt, _args| {
            Ok(Value::Boolean(false))
        });
        // Constructor stubs — packages probe `typeof WebAssembly.Module` etc.
        // to decide on a code path; returning a callable that throws on
        // construction is more disciplined than leaving them undefined.
        for ctor_name in &[
            "Module", "Instance", "Memory", "Table", "Global", "Tag", "Function",
        ] {
            let name = (*ctor_name).to_string();
            let stub = make_native(&name, move |_rt, _args| Err(unsupported()));
            let stub_id = self.alloc_object(stub);
            self.object_set(wasm, name, Value::Object(stub_id));
        }
        // Error-class stubs — packages do `instanceof WebAssembly.CompileError`
        // / `RuntimeError` / `LinkError` after their try/catch.
        for err_name in &["CompileError", "LinkError", "RuntimeError"] {
            let name = (*err_name).to_string();
            let stub = make_native(&name, move |_rt, args| {
                let o = Object::new_ordinary();
                let id = _rt.alloc_object(o);
                let msg = args
                    .first()
                    .map(|v| crate::abstract_ops::to_string(v).as_str().to_string())
                    .unwrap_or_default();
                _rt.object_set(id, "message".into(), Value::String(Rc::new(msg)));
                Ok(Value::Object(id))
            });
            let stub_id = self.alloc_object(stub);
            self.object_set(wasm, name, Value::Object(stub_id));
        }
        self.globals
            .insert("WebAssembly".into(), Value::Object(wasm));

        self.install_iterator_helpers_and_recent_methods();
        self.install_global_this();
    }

    /// Tier-Ω.5.t: install `globalThis` as a synthetic object mirroring
    /// the current globals map. Self-references via `globalThis.globalThis`.
    /// Read-only snapshot at install time — subsequent writes to globals
    /// do NOT propagate. Acceptable v1 deviation: real spec has globalThis
    /// be the *actual* global object, but our globals are a HashMap, not
    /// an Object. Most consumer code reads from globalThis rather than
    /// writes; the snapshot is sufficient for shape probes.
    ///
    /// Hosts that add globals after install_intrinsics should call
    /// `install_global_this_refresh` once their wiring is complete so the
    /// snapshot picks up host-added bindings.
    pub fn install_global_this_refresh(&mut self) {
        self.install_global_this();
    }

    fn install_global_this(&mut self) {
        let gt = self.alloc_object(Object::new_ordinary());
        let entries: Vec<(String, Value)> = self
            .globals
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        // GBNE-EXT 1: ECMA-262 §17 baseline — standard built-in properties
        // on the global object are {w:t, e:f, c:t}. Use dict_mut().insert
        // with explicit PropertyDescriptor instead of self.object_set
        // (which silently installs enumerable). User-installed globals
        // via Op::SetProperty continue to land enumerable per default
        // CreateDataPropertyOrThrow.
        for (k, v) in entries {
            self.obj_mut(gt).dict_mut().insert(
                crate::value::PropertyKey::String(k),
                crate::value::PropertyDescriptor {
                    value: v,
                    writable: true,
                    enumerable: false,
                    configurable: true,
                    getter: None,
                    setter: None,
                },
            );
        }
        // globalThis self-reference: §19.1.1 — {w:t, e:f, c:t}.
        // `global` (Node alias): same convention.
        for k in &["globalThis", "global"] {
            self.obj_mut(gt).dict_mut().insert(
                crate::value::PropertyKey::String((*k).to_string()),
                crate::value::PropertyDescriptor {
                    value: Value::Object(gt),
                    writable: true,
                    enumerable: false,
                    configurable: true,
                    getter: None,
                    setter: None,
                },
            );
        }
        self.globals.insert("globalThis".into(), Value::Object(gt));
        self.globals.insert("global".into(), Value::Object(gt));
        // Tier-Ω.5.bbbb: Intl namespace with stub constructors. Real
        // locale-aware behavior is deferred; the stubs return objects
        // that survive shape probes and method existence checks. Lifts
        // packages that gate on `typeof Intl.X === 'function'`.
        let intl = self.alloc_object(Object::new_ordinary());
        for ctor_name in &[
            "DateTimeFormat",
            "NumberFormat",
            "Collator",
            "PluralRules",
            "RelativeTimeFormat",
            "ListFormat",
            "Segmenter",
            "DisplayNames",
            "Locale",
        ] {
            let name = (*ctor_name).to_string();
            // Ω.5.P52.E2: Intl-instance constructor now captures locale + options
            // on the instance and exposes resolvedOptions() returning the merged
            // shape (input options + sensible defaults). temporal-polyfill probes
            // `new Intl.DateTimeFormat(undefined, {calendar: 'iso8601'}).resolvedOptions().calendar === 'iso8601'`
            // at module-init to detect bug-resilient implementations; the prior
            // stub returned an empty Object instance with no methods, hard-failing
            // the .resolvedOptions() call.
            // Ω.5.P52.E2: install a populated .prototype on the Intl ctor stub.
            // temporal-polyfill iterates Object.getOwnPropertyDescriptors(en.prototype)
            // and inspects each entry's .value to wrap callable members. The prior
            // empty prototype caused the iteration to see only `constructor`, which
            // bypassed the consumer's wrap logic. Real spec exposes format /
            // formatToParts / resolvedOptions as prototype methods that read
            // instance state (the captured locale + options).
            let proto = self.alloc_object(Object::new_ordinary());
            let proto_for_closure = proto;
            let stub = make_native(&name, move |rt, args| {
                let mut o = Object::new_ordinary();
                o.proto = Some(proto_for_closure);
                let id = rt.alloc_object(o);
                let locale = args.first().cloned().unwrap_or(Value::Undefined);
                let opts = args.get(1).cloned().unwrap_or(Value::Undefined);
                rt.object_set(id, "__locale".into(), locale);
                rt.object_set(id, "__opts".into(), opts);
                Ok(Value::Object(id))
            });
            let stub_id = self.alloc_object(stub);
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(stub_id));
            register_intrinsic_method(self, proto, "format", 1, |_rt, args| {
                Ok(Value::String(std::rc::Rc::new(
                    crate::abstract_ops::to_string(
                        &args.first().cloned().unwrap_or(Value::Undefined),
                    )
                    .as_str()
                    .to_string(),
                )))
            });
            register_intrinsic_method(self, proto, "formatToParts", 1, |rt, args| {
                let arr = Object::new_array();
                let aid = rt.alloc_object(arr);
                let part = rt.alloc_object(Object::new_ordinary());
                rt.object_set(
                    part,
                    "type".into(),
                    Value::String(std::rc::Rc::new("literal".into())),
                );
                rt.object_set(
                    part,
                    "value".into(),
                    Value::String(std::rc::Rc::new(
                        crate::abstract_ops::to_string(
                            &args.first().cloned().unwrap_or(Value::Undefined),
                        )
                        .as_str()
                        .to_string(),
                    )),
                );
                rt.object_set(aid, "0".into(), Value::Object(part));
                rt.object_set(aid, "length".into(), Value::Number(1.0));
                Ok(Value::Object(aid))
            });
            register_intrinsic_method(self, proto, "resolvedOptions", 1, |rt, _args| {
                let this_id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Ok(Value::Undefined),
                };
                let opts = rt.object_get(this_id, "__opts");
                let locale_v = rt.object_get(this_id, "__locale");
                let res = rt.alloc_object(Object::new_ordinary());
                let locale_str = match &locale_v {
                    Value::String(s) => (**s).clone(),
                    _ => "en-US".to_string(),
                };
                rt.object_set(
                    res,
                    "locale".into(),
                    Value::String(std::rc::Rc::new(locale_str)),
                );
                rt.object_set(
                    res,
                    "calendar".into(),
                    Value::String(std::rc::Rc::new("iso8601".into())),
                );
                rt.object_set(
                    res,
                    "numberingSystem".into(),
                    Value::String(std::rc::Rc::new("latn".into())),
                );
                rt.object_set(
                    res,
                    "timeZone".into(),
                    Value::String(std::rc::Rc::new("UTC".into())),
                );
                if let Value::Object(opts_id) = opts {
                    let pairs: Vec<(String, Value)> = rt
                        .obj(opts_id)
                        .properties
                        .iter()
                        .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                        .collect();
                    for (k, v) in pairs {
                        rt.object_set(res, k, v);
                    }
                }
                Ok(Value::Object(res))
            });
            self.obj_mut(stub_id)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            // Static method on the ctor itself.
            register_intrinsic_method(self, stub_id, "supportedLocalesOf", 1, |_rt, _args| {
                let o = Object::new_array();
                let id = _rt.alloc_object(o);
                _rt.object_set(id, "length".into(), Value::Number(0.0));
                Ok(Value::Object(id))
            });
            self.object_set(intl, ctor_name.to_string(), Value::Object(stub_id));
        }
        // getCanonicalLocales(locales) → array of canonical locale tags.
        register_intrinsic_method(self, intl, "getCanonicalLocales", 1, |rt, _args| {
            let arr = Object::new_array();
            let id = rt.alloc_object(arr);
            rt.object_set(id, "length".into(), Value::Number(0.0));
            Ok(Value::Object(id))
        });
        self.globals.insert("Intl".into(), Value::Object(intl));
        // Tier-Ω.5.iiii: TextEncoder / TextDecoder per WHATWG Encoding
        // spec. v1 deviation: only UTF-8 supported; encode returns a
        // Uint8Array-shaped object (length + indexed bytes); decode
        // reads bytes back as JS string. Sufficient for jose / ky /
        // get-stream / many crypto + stream-using packages.
        let te = make_native("TextEncoder", |rt, _args| {
            let mut o = Object::new_ordinary();
            o.set_own(
                "encoding".into(),
                Value::String(Rc::new("utf-8".to_string())),
            );
            let id = rt.alloc_object(o);
            register_intrinsic_method(rt, id, "encode", 1, |rt, args| {
                let s = match args.first() {
                    Some(Value::String(s)) => s.as_str().to_string(),
                    None => String::new(),
                    Some(v) => crate::abstract_ops::to_string(v).as_str().to_string(),
                };
                let bytes: Vec<u8> = s.into_bytes();
                let mut out = Object::new_array();
                out.set_own("length".into(), Value::Number(bytes.len() as f64));
                for (i, b) in bytes.iter().enumerate() {
                    out.set_own(i.to_string(), Value::Number(*b as f64));
                }
                Ok(Value::Object(rt.alloc_object(out)))
            });
            Ok(Value::Object(id))
        });
        let te_id = self.alloc_object(te);
        // Tier-Ω.5.qqqq: TextEncoder.prototype.encode for pako and any lib
        // that reaches the encode method via the prototype rather than via
        // an instance.
        let te_proto = self.alloc_object(Object::new_ordinary());
        register_method(self, te_proto, "encode", |rt, args| {
            let s = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                None => String::new(),
                Some(v) => crate::abstract_ops::to_string(v).as_str().to_string(),
            };
            let bytes: Vec<u8> = s.into_bytes();
            let mut out = Object::new_array();
            out.set_own("length".into(), Value::Number(bytes.len() as f64));
            for (i, b) in bytes.iter().enumerate() {
                out.set_own(i.to_string(), Value::Number(*b as f64));
            }
            Ok(Value::Object(rt.alloc_object(out)))
        });
        self.obj_mut(te_id)
            .set_own_frozen("prototype".into(), Value::Object(te_proto));
        self.globals
            .insert("TextEncoder".into(), Value::Object(te_id));
        let td = make_native("TextDecoder", |rt, args| {
            let encoding = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => "utf-8".to_string(),
            };
            let mut o = Object::new_ordinary();
            o.set_own("encoding".into(), Value::String(Rc::new(encoding)));
            let id = rt.alloc_object(o);
            register_intrinsic_method(rt, id, "decode", 1, |rt, args| {
                let bytes_id = match args.first() {
                    Some(Value::Object(id)) => *id,
                    _ => return Ok(Value::String(Rc::new(String::new()))),
                };
                let len = rt.array_length(bytes_id);
                let mut bytes: Vec<u8> = Vec::with_capacity(len);
                for i in 0..len {
                    if let Value::Number(n) = rt.object_get(bytes_id, &i.to_string()) {
                        bytes.push(n as u8);
                    }
                }
                let s = String::from_utf8_lossy(&bytes).to_string();
                Ok(Value::String(Rc::new(s)))
            });
            Ok(Value::Object(id))
        });
        let td_id = self.alloc_object(td);
        let td_proto = self.alloc_object(Object::new_ordinary());
        register_method(self, td_proto, "decode", |rt, args| {
            let bytes_id = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Ok(Value::String(Rc::new(String::new()))),
            };
            let len = rt.array_length(bytes_id);
            let mut bytes: Vec<u8> = Vec::with_capacity(len);
            for i in 0..len {
                if let Value::Number(n) = rt.object_get(bytes_id, &i.to_string()) {
                    bytes.push(n as u8);
                }
            }
            let s = String::from_utf8_lossy(&bytes).to_string();
            Ok(Value::String(Rc::new(s)))
        });
        self.obj_mut(td_id)
            .set_own_frozen("prototype".into(), Value::Object(td_proto));
        self.globals
            .insert("TextDecoder".into(), Value::Object(td_id));
    }

    /// Tier-Ω.5.k: helpers the compiler emits LoadGlobal+Call into for
    /// object-literal spread and spread arguments. All return the target
    /// (array or object) so they compose without extra stack juggling.
    fn install_spread_helpers(&mut self) {
        // __object_spread(target, src) → target. Copies own enumerable
        // string-keyed properties from src to target, left-to-right.
        // Tier-Ω.5.gggggg: yield helpers. The compiler lowers `yield expr`
        // to `__yield_push__(expr)` and `yield* iter` to
        // `__yield_delegate__(iter)`. The runtime maintains a stack of
        // yields-arrays — generator calls push on entry, pop on exit;
        // these helpers append to the top.
        // Tier-Ω.5.kkkkkk: __install_accessor__(target, key, "get"|"set", fn).
        // Installs an accessor property descriptor on target. Class
        // getters / setters lower to this call.
        register_engine_helper(self, "__install_accessor__", |rt, args| {
            let target = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Ok(Value::Undefined),
            };
            let key: String = match args.get(1) {
                Some(Value::String(s)) => (**s).clone(),
                _ => return Ok(Value::Undefined),
            };
            let kind: String = match args.get(2) {
                Some(Value::String(s)) => (**s).clone(),
                _ => return Ok(Value::Undefined),
            };
            let fn_v = args.get(3).cloned().unwrap_or(Value::Undefined);
            let o = rt.obj_mut(target);
            // Class accessors install as enumerable:false per ECMA-262 sec
            // 15.7 MethodDefinitionEvaluation. Object-literal accessors
            // use a separate helper (__install_accessor_obj__) below to
            // get enumerable:true per sec 13.2.5.5 PropertyDefinitionEvaluation.
            let desc = o
                .properties
                .entry(crate::value::PropertyKey::String(key))
                .or_insert_with(|| crate::value::PropertyDescriptor {
                    value: Value::Undefined,
                    writable: false,
                    enumerable: false,
                    configurable: true,
                    getter: None,
                    setter: None,
                });
            if kind == "get" {
                desc.getter = Some(fn_v);
            } else if kind == "set" {
                desc.setter = Some(fn_v);
            }
            Ok(Value::Undefined)
        });
        // Object-literal accessors variant. ECMA-262 sec 13.2.5.5
        // PropertyDefinitionEvaluation step 8 makes object-literal
        // accessors {writable:false, enumerable:true, configurable:true}.
        // Sharing one helper with class-side install hid this defect
        // behind enumerable:false, so {get v(){}} -wrapped objects had
        // their getters excluded from Object.keys / for-in / object-rest
        // spread - the latter surfaced as the dstr obj-ptrn-rest-getter
        // cluster (4 tests in the sample).
        register_engine_helper(self, "__install_accessor_obj__", |rt, args| {
            let target = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Ok(Value::Undefined),
            };
            let key: String = match args.get(1) {
                Some(Value::String(s)) => (**s).clone(),
                _ => return Ok(Value::Undefined),
            };
            let kind: String = match args.get(2) {
                Some(Value::String(s)) => (**s).clone(),
                _ => return Ok(Value::Undefined),
            };
            let fn_v = args.get(3).cloned().unwrap_or(Value::Undefined);
            let o = rt.obj_mut(target);
            let desc = o
                .properties
                .entry(crate::value::PropertyKey::String(key))
                .or_insert_with(|| crate::value::PropertyDescriptor {
                    value: Value::Undefined,
                    writable: false,
                    enumerable: true,
                    configurable: true,
                    getter: None,
                    setter: None,
                });
            // If the property already existed (e.g. installed by a
            // sibling getter/setter half of the pair), force enumerable
            // back to true in case the prior install used the class form.
            desc.enumerable = true;
            if kind == "get" {
                desc.getter = Some(fn_v);
            } else if kind == "set" {
                desc.setter = Some(fn_v);
            }
            Ok(Value::Undefined)
        });
        // Ω.5.P03.E2.class-method-non-enumerable: __install_method__(
        //   target, key, fn). Installs fn at target[key] with the
        //   spec-mandated method descriptor: {writable: true,
        //   enumerable: false, configurable: true} per ECMA-262 §15.7
        //   ClassDefinitionEvaluation + §15.7.10. Pre-substrate, class
        //   methods were SetProp'd which produces enumerable: true; the
        //   resulting Object.keys(Class.prototype) returned method names
        //   instead of [], and any code iterating prototypes via Object.
        //   values / Object.entries / for-in picked up methods that the
        //   spec says it should not.
        register_engine_helper(self, "__install_method__", |rt, args| {
            let target = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Ok(Value::Undefined),
            };
            // Ω.5.P03.E2.class-method-non-enumerable post-fix: accept
            // Symbol-keyed method names too. Computed class members
            // can use Symbol values as keys (`class C { [Symbol.X]() {} }`);
            // cruftless stringifies Symbols to their internal "@@sym:N:NAME"
            // form. Pre-fix, the helper dropped Symbol-keyed methods on the
            // floor, surfacing as a top500 regression on sharp-cli
            // (yargs-uses-Symbol-keyed-helper pattern).
            let key: String = match args.get(1) {
                Some(Value::String(s)) => (**s).clone(),
                Some(Value::Symbol(s)) => (**s).clone(),
                Some(Value::Number(n)) => {
                    if n.fract() == 0.0 {
                        format!("{}", *n as i64)
                    } else {
                        format!("{}", n)
                    }
                }
                _ => return Ok(Value::Undefined),
            };
            let fn_v = args.get(2).cloned().unwrap_or(Value::Undefined);
            rt.obj_mut(target).set_own_internal(key, fn_v);
            Ok(Value::Undefined)
        });
        // Ω.5.P03.E2.super-get-this: __super_get(this_val, super_base, key)
        // implements ECMA-262 §13.3.7.3 MakeSuperPropertyReference +
        // §10.4.4 GetSuperBase + §10.1.7.2 OrdinaryGet — super.X reads
        // walk the [[HomeObject]]'s [[Prototype]] chain to find the
        // property, but if the property is an accessor, the getter is
        // invoked with `this = the calling method's this binding`, NOT
        // the super-base. Pre-substrate, cruftless compiled super.X
        // reads as `LoadIdent <super.proto>; GetProp X` — and Op::GetProp
        // uses the popped object value as the receiver for accessor
        // invocation. So a `get foo() { return super.foo; }` pattern
        // produced this = the super-base prototype inside the inherited
        // getter, instead of this = the original instance. arktype's
        // BaseRoot has `get rawIn() { return super.rawIn; }` (root.js:21),
        // and the BaseNode getter does cacheGetter("rawIn", ...) which
        // wrote the result onto the super-base prototype itself. The
        // cached value then leaked through every subsequent
        // branch.rawIn access on instances — wall 4 of the arktype
        // deep-trace localized this exact path.
        register_engine_helper(self, "__super_get", |rt, args| {
            let this_val = args.first().cloned().unwrap_or(Value::Undefined);
            let super_base = args.get(1).cloned().unwrap_or(Value::Undefined);
            let key: String = match args.get(2) {
                Some(Value::String(s)) => (**s).clone(),
                Some(Value::Number(n)) => {
                    if n.fract() == 0.0 {
                        format!("{}", *n as i64)
                    } else {
                        format!("{}", n)
                    }
                }
                _ => return Ok(Value::Undefined),
            };
            let base_id = match super_base {
                Value::Object(id) => id,
                _ => return Ok(Value::Undefined),
            };
            // Walk super_base.[[Prototype]] chain (i.e. start at super_base
            // itself, since super.X looks up X on super_base which is the
            // parent prototype). Find the property descriptor.
            let mut cur: Option<rusty_js_gc::ObjectId> = Some(base_id);
            while let Some(c) = cur {
                let o = rt.obj(c);
                if let Some(desc) = o.get_own(&key) {
                    if let Some(getter) = desc.getter.clone() {
                        // Accessor with getter — invoke with this = original this_val.
                        return rt.call_function(getter, this_val, vec![]);
                    }
                    // Data property — return value directly.
                    return Ok(desc.value.clone());
                }
                cur = o.proto;
            }
            Ok(Value::Undefined)
        });
        // Ω.5.P04.E1.for-in-nullish-skip: __for_in_keys(obj) returns
        // Object.keys(obj) for object/function receivers, but [] for
        // undefined and null receivers per ECMA-262 §14.7.5.6
        // ForIn/OfHeadEvaluation step 6 (if exprValue is undefined or
        // null, return Completion(undefined) — which causes the for-in
        // loop body to be skipped). Pre-substrate, cruftless compiled
        // for-in to Object.keys(right) directly; Object.keys on
        // undefined/null throws "Cannot convert undefined or null to
        // object", masking the spec-mandated short-circuit.
        // Manifested across the joi cluster (14 packages on the
        // post-substrate top500 sweep) as the cluster's leading error
        // signature: pattern `for (const k in this.$_super)` where
        // this.$_super is undefined on some object configurations.
        register_engine_helper(self, "__for_in_keys", |rt, args| {
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            if matches!(v, Value::Undefined | Value::Null) {
                let arr = rt.alloc_object(crate::value::Object::new_array());
                return Ok(Value::Object(arr));
            }
            // FIPC-EXT 1: ECMA-262 §14.7.5.6 EnumerateObjectProperties —
            // walk the prototype chain. At each level, yield own enumerable
            // string keys (in §7.3.22 canonical order). Shadowing: a name
            // first seen as an own property at a closer level (enumerable or
            // not) is excluded from later levels.
            let id = match &v {
                Value::Object(id) => *id,
                _ => match rt.to_object(&v)? {
                    Value::Object(id) => id,
                    _ => {
                        let arr = rt.alloc_object(crate::value::Object::new_array());
                        return Ok(Value::Object(arr));
                    }
                },
            };
            let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
            let mut visible: Vec<String> = Vec::new();
            let mut cur = Some(id);
            while let Some(c) = cur {
                let own_enum = rt.ordinary_own_enumerable_string_keys(c);
                for k in &own_enum {
                    if seen.insert(k.clone()) {
                        visible.push(k.clone());
                    }
                }
                // Shadow record: add own non-enumerable (and own enumerable)
                // names to seen so deeper-proto entries with same name are
                // skipped. own_enum is already added above; add non-enumerable
                // own names from the full property bag.
                let o = rt.obj(c);
                let extra_keys: Vec<String> = o
                    .properties
                    .iter()
                    .filter(|(k, d)| {
                        !d.enumerable
                            && k.is_string()
                            && k.as_str() != "__primitive__"
                            && !k.as_str().starts_with("@@")
                    })
                    .map(|(k, _)| k.as_str().to_string())
                    .collect();
                for k in extra_keys {
                    seen.insert(k);
                }
                cur = o.proto;
            }
            let arr = rt.alloc_object(crate::value::Object::new_array());
            for (i, k) in visible.iter().enumerate() {
                rt.object_set(
                    arr,
                    i.to_string(),
                    Value::String(std::rc::Rc::new(k.clone())),
                );
            }
            rt.object_set(arr, "length".into(), Value::Number(visible.len() as f64));
            Ok(Value::Object(arr))
        });
        register_engine_helper(self, "__yield_push__", |rt, args| {
            if let Some(&arr) = rt.gen_yields_stack.last() {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                let len = rt.array_length(arr);
                rt.object_set(arr, len.to_string(), v);
                rt.object_set(arr, "length".into(), Value::Number((len + 1) as f64));
            }
            Ok(Value::Undefined)
        });
        register_engine_helper(self, "__yield_delegate__", |rt, args| {
            let target_arr = match rt.gen_yields_stack.last().copied() {
                Some(a) => a,
                None => return Ok(Value::Undefined),
            };
            let it_arg = args.first().cloned().unwrap_or(Value::Undefined);
            // Iterate via Symbol.iterator / @@iterator / array length.
            let it_obj = match &it_arg {
                Value::Object(id) => *id,
                _ => return Ok(Value::Undefined),
            };
            // If the iterable is itself an array-like with length, walk indices.
            // Otherwise, try @@iterator and .next() repeatedly.
            let try_iter = rt.object_get(it_obj, "@@iterator");
            let iter_obj = if matches!(try_iter, Value::Object(_)) {
                match rt.call_function(try_iter, Value::Object(it_obj), Vec::new()) {
                    Ok(Value::Object(id)) => Some(id),
                    _ => None,
                }
            } else {
                None
            };
            if let Some(iter_id) = iter_obj {
                let next = rt.object_get(iter_id, "next");
                if matches!(next, Value::Object(_)) {
                    loop {
                        let step = match rt.call_function(
                            next.clone(),
                            Value::Object(iter_id),
                            Vec::new(),
                        ) {
                            Ok(v) => v,
                            Err(_) => break,
                        };
                        let step_id = match step {
                            Value::Object(id) => id,
                            _ => break,
                        };
                        if matches!(rt.object_get(step_id, "done"), Value::Boolean(true)) {
                            break;
                        }
                        let v = rt.object_get(step_id, "value");
                        let len = rt.array_length(target_arr);
                        rt.object_set(target_arr, len.to_string(), v);
                        rt.object_set(target_arr, "length".into(), Value::Number((len + 1) as f64));
                    }
                    return Ok(Value::Undefined);
                }
            }
            // Fallback: array-like.
            let len = rt.array_length(it_obj);
            for i in 0..len {
                let v = rt.object_get(it_obj, &i.to_string());
                let tl = rt.array_length(target_arr);
                rt.object_set(target_arr, tl.to_string(), v);
                rt.object_set(target_arr, "length".into(), Value::Number((tl + 1) as f64));
            }
            Ok(Value::Undefined)
        });
        register_engine_helper(self, "__object_spread", |rt, args| {
            let target = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "__object_spread: target must be an object".into(),
                    ))
                }
            };
            if let Some(Value::Object(sid)) = args.get(1) {
                // Tier-Ω.5.bbbbb: dispatch accessor getters during spread.
                // CMig-EXT 15 (2026-05-23): shape-aware iteration. Shape-
                // enrolled sources keep values in shape_values with empty
                // .properties; pre-fix iteration here returned {} silently
                // for any Shaped source. Per shapes seed §IV carve-out,
                // shape-stored entries are all enumerable plain-data
                // descriptors with no accessor — emit them without the
                // getter dispatch path. Dictionary-stored entries follow
                // the original property-map iteration with accessor handling.
                let mut shape_entries: Vec<String> = Vec::new();
                let mut dict_entries: Vec<(String, Option<Value>)> = Vec::new();
                {
                    let o = rt.obj(*sid);
                    if let Some(shape) = o.shape.as_ref() {
                        for (name, _) in shape.iter_slots() {
                            shape_entries.push(name.to_string());
                        }
                    }
                    for (k, d) in o.properties.iter().filter(|(_, d)| d.enumerable) {
                        dict_entries.push((k.to_string_content(), d.getter.clone()));
                    }
                }
                for k in shape_entries {
                    let v = rt.object_get(*sid, &k);
                    rt.object_set(target, k, v);
                }
                for (k, getter_opt) in dict_entries {
                    let v = if let Some(getter) = getter_opt {
                        rt.call_function(getter, Value::Object(*sid), Vec::new())?
                    } else {
                        rt.object_get(*sid, &k)
                    };
                    rt.object_set(target, k, v);
                }
            }
            // Non-object sources (null/undefined) are a no-op per ECMA-262.
            Ok(Value::Object(target))
        });
        // __array_push_single(arr, v) → arr. Appends one value.
        register_engine_helper(self, "__array_push_single", |rt, args| {
            let arr = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "__array_push_single: target must be an array".into(),
                    ))
                }
            };
            let v = args.get(1).cloned().unwrap_or(Value::Undefined);
            let len = rt.array_length(arr);
            rt.object_set(arr, len.to_string(), v);
            rt.object_set(arr, "length".into(), Value::Number((len + 1) as f64));
            Ok(Value::Object(arr))
        });
        // __array_extend(arr, iter) → arr. Iterates iter via @@iterator
        // protocol and appends each yielded value.
        register_engine_helper(self, "__array_extend", |rt, args| {
            let arr = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "__array_extend: target must be an array".into(),
                    ))
                }
            };
            let src = args.get(1).cloned().unwrap_or(Value::Undefined);
            let values = collect_iterable(rt, src)?;
            let mut len = rt.array_length(arr);
            for v in values {
                rt.object_set(arr, len.to_string(), v);
                len += 1;
            }
            rt.object_set(arr, "length".into(), Value::Number(len as f64));
            Ok(Value::Object(arr))
        });
        // __apply(callee, thisArg, argsArray) → callee.apply(thisArg, argsArray).
        // Used by the compiler to lower spread-argument calls.
        register_engine_helper(self, "__apply", |rt, args| {
            let callee = args.first().cloned().unwrap_or(Value::Undefined);
            let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
            let arr = args.get(2).cloned().unwrap_or(Value::Undefined);
            let collected = match arr {
                Value::Object(id) => {
                    let n = rt.array_length(id);
                    (0..n).map(|i| rt.object_get(id, &i.to_string())).collect()
                }
                _ => Vec::new(),
            };
            rt.call_function(callee, this_arg, collected)
        });
        // Ω.5.P03.E2.super-new-target: __super_apply(callee, thisArg,
        // argsArray) is __apply that ALSO forwards the active new.target
        // into the inner call so the parent constructor invocation has
        // construct semantics. Used by compile_super_call's spread
        // branch (super(...args) inside a derived ctor). The active
        // new.target is current_new_target at __super_apply's entry,
        // which is the derived ctor's new.target propagated by the
        // PropagateNewTarget op emitted just before this helper's call.
        register_engine_helper(self, "__super_apply", |rt, args| {
            let callee = args.first().cloned().unwrap_or(Value::Undefined);
            let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
            let arr = args.get(2).cloned().unwrap_or(Value::Undefined);
            let collected = match arr {
                Value::Object(id) => {
                    let n = rt.array_length(id);
                    (0..n).map(|i| rt.object_get(id, &i.to_string())).collect()
                }
                _ => Vec::new(),
            };
            // Forward our current new.target into the inner dispatch.
            if let Some(nt) = rt.current_new_target.clone() {
                rt.pending_new_target = Some(nt);
            }
            rt.call_function(callee, this_arg, collected)
        });
        // __construct(callee, argsArray) → new callee(...argsArray).
        // Mirrors the Op::New handler: consults callee.prototype for the
        // new instance's [[Prototype]] and discards non-object returns.
        register_engine_helper(self, "__construct", |rt, args| {
            let callee = args.first().cloned().unwrap_or(Value::Undefined);
            let arr = args.get(1).cloned().unwrap_or(Value::Undefined);
            let collected: Vec<Value> = match arr {
                Value::Object(id) => {
                    let n = rt.array_length(id);
                    (0..n).map(|i| rt.object_get(id, &i.to_string())).collect()
                }
                _ => Vec::new(),
            };
            let proto_override = if let Value::Object(cid) = &callee {
                match rt.object_get(*cid, "prototype") {
                    Value::Object(pid) => Some(pid),
                    _ => None,
                }
            } else {
                None
            };
            let mut ordinary = Object::new_ordinary();
            if proto_override.is_some() {
                ordinary.proto = proto_override;
            }
            let this_id = rt.alloc_object(ordinary);
            let this_obj = Value::Object(this_id);
            // Tier-Ω.5.s: __construct mirrors Op::New — mark new.target.
            rt.pending_new_target = Some(callee.clone());
            let ret = rt.call_function(callee, this_obj.clone(), collected)?;
            Ok(match ret {
                Value::Object(_) => ret,
                _ => this_obj,
            })
        });
    }

    /// IPEP-EXT 1: ECMA-262 §13.15.5.3 RS:DestructuringAssignmentEvaluation
    /// and §14.4.2.4 IteratorBindingInitialization require Array-pattern
    /// destructure to operate on an IteratorRecord opened from the source
    /// via GetIterator(value). Three engine helpers cooperate with
    /// inline-emitted iterator-protocol bytecode:
    ///   - __destr_iter_open(value) → iterator (throws if @@iterator getter
    ///     throws; throws TypeError if value is null/undefined or @@iterator
    ///     returns a non-object).
    ///   - __destr_iter_step(iter) → IteratorResult `{value, done}`.
    ///   - __destr_iter_rest(iter) → array of remaining values.
    /// Compiler emits these into both emit_destructure (binding) and
    /// emit_destructure_assign (assignment) Array paths.
    /// CP-EXT 1+2+3: install the JS-visible `Compartment` class per the
    /// TC39 Compartments proposal (Stage 1, frozen-snapshot 2025-12-01).
    ///
    /// `new Compartment({globals, modules})` allocates a fresh realm with
    /// cloned intrinsics (per RS-EXT 2 minimum-realm). The `globals`
    /// entries become the only non-intrinsic ambient bindings inside the
    /// compartment. `modules` is parsed but deferred to CP-EXT 4.
    /// `c.evaluate(source)` runs the source under the compartment's
    /// realm; `c.globalThis` returns the compartment's globalThis object.
    ///
    /// The hooks (importHook/loadHook/resolveHook) and Module Source
    /// records remain deferred per the locale's CP-EXT 6/7 prospective.
    fn install_compartment(&mut self) {
        // Build Compartment.prototype with evaluate + globalThis accessor.
        let proto = self.alloc_object(Object::new_ordinary());

        // Compartment.prototype.evaluate(source) — eval source in the
        // compartment's realm. Reads `this.__compartment_realm` (slot
        // populated by the ctor below) and `this.__compartment_globalthis`
        // for the per-compartment globalThis identity.
        register_intrinsic_method(self, proto, "evaluate", 1, |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(id) => id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Compartment.prototype.evaluate: this is not a Compartment".into(),
                    ))
                }
            };
            let realm_idx = match rt.object_get(this_id, "__compartment_realm") {
                Value::Number(n) => n as usize,
                _ => return Err(RuntimeError::TypeError("Compartment.prototype.evaluate: this is not a Compartment (missing realm slot)".into())),
            };
            let source = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Compartment.prototype.evaluate: source must be a string".into(),
                    ))
                }
            };
            // Wrap in indirect-eval form to capture the last expression's
            // value into a stash global, mirroring eval()'s pattern.
            use std::sync::atomic::{AtomicUsize, Ordering};
            static EVAL_COUNTER: AtomicUsize = AtomicUsize::new(0);
            let n = EVAL_COUNTER.fetch_add(1, Ordering::Relaxed);
            let url = format!("file://<compartment:{}:eval:{}>", realm_idx, n);
            let stash_key = format!("__cp_out_{}", n);
            let expr_source = format!("{} = ({});", stash_key, source);
            let prior = rt.enter_realm(realm_idx);
            let expr_ok = rt.evaluate_module(&expr_source, &url).is_ok();
            let result = if expr_ok {
                let r = rt
                    .globals
                    .get(&stash_key)
                    .cloned()
                    .unwrap_or(Value::Undefined);
                rt.globals.remove(&stash_key);
                r
            } else {
                let stmt_url = format!("file://<compartment:{}:stmt:{}>", realm_idx, n);
                match rt.evaluate_module(&source, &stmt_url) {
                    Ok(_) => Value::Undefined,
                    Err(RuntimeError::CompileError(msg)) => {
                        rt.exit_realm(prior);
                        return Err(RuntimeError::SyntaxError(msg));
                    }
                    Err(e) => {
                        rt.exit_realm(prior);
                        return Err(e);
                    }
                }
            };
            rt.exit_realm(prior);
            Ok(result)
        });

        // CP-EXT 4: Compartment.prototype.import(specifier) — Promise of
        // module namespace. Looks up the specifier in the compartment's
        // modules map (populated at ctor time); evaluates the source as
        // an ESM module under the compartment's realm; returns a
        // Promise.resolved with the namespace. Specifiers absent from
        // the map return a Promise.rejected with TypeError per the
        // proposal's closed-import-graph semantics.
        register_intrinsic_method(self, proto, "import", 1, |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(id) => id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Compartment.prototype.import: this is not a Compartment".into(),
                    ))
                }
            };
            let realm_idx = match rt.object_get(this_id, "__compartment_realm") {
                Value::Number(n) => n as usize,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Compartment.prototype.import: missing realm slot".into(),
                    ))
                }
            };
            let modules_id = match rt.object_get(this_id, "__compartment_modules") {
                Value::Object(id) => id,
                _ => {
                    let p = crate::promise::new_promise(rt);
                    crate::promise::reject_promise(
                        rt,
                        p,
                        Value::String(Rc::new("Compartment has no modules map".into())),
                    );
                    return Ok(Value::Object(p));
                }
            };
            let spec = match args.first() {
                Some(Value::String(s)) => (**s).clone(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Compartment.prototype.import: specifier must be a string".into(),
                    ))
                }
            };
            let source_v = rt.object_get(modules_id, &spec);
            let source = match source_v {
                Value::String(s) => s.as_str().to_string(),
                _ => {
                    // Specifier not in map — per proposal, rejected Promise.
                    let p = crate::promise::new_promise(rt);
                    let err = rt.alloc_object(Object::new_ordinary());
                    rt.object_set(
                        err,
                        "message".into(),
                        Value::String(Rc::new(format!(
                            "Module '{}' not found in compartment",
                            spec
                        ))),
                    );
                    rt.object_set(
                        err,
                        "name".into(),
                        Value::String(Rc::new("TypeError".into())),
                    );
                    crate::promise::reject_promise(rt, p, Value::Object(err));
                    return Ok(Value::Object(p));
                }
            };
            let url = format!("file://<compartment:{}:module:{}>", realm_idx, spec);
            let prior = rt.enter_realm(realm_idx);
            let result = rt.evaluate_module(&source, &url);
            rt.exit_realm(prior);
            let p = crate::promise::new_promise(rt);
            match result {
                Ok(ns) => crate::promise::resolve_promise(rt, p, Value::Object(ns)),
                Err(RuntimeError::CompileError(msg)) => {
                    let err = rt.alloc_object(Object::new_ordinary());
                    rt.object_set(err, "message".into(), Value::String(Rc::new(msg)));
                    rt.object_set(
                        err,
                        "name".into(),
                        Value::String(Rc::new("SyntaxError".into())),
                    );
                    crate::promise::reject_promise(rt, p, Value::Object(err));
                }
                Err(e) => {
                    let msg = format!("{:?}", e);
                    let err = rt.alloc_object(Object::new_ordinary());
                    rt.object_set(err, "message".into(), Value::String(Rc::new(msg)));
                    crate::promise::reject_promise(rt, p, Value::Object(err));
                }
            }
            Ok(Value::Object(p))
        });

        // Compartment.prototype.globalThis as a data property reader.
        // Returns the compartment's globalThis object slot. (Spec
        // mandates a getter; the simpler data-property reader is
        // observationally equivalent for the probe surface and avoids
        // accessor-property substrate cost.)
        register_intrinsic_method(self, proto, "globalThis", 0, |rt, _args| {
            let this_id = match rt.current_this() {
                Value::Object(id) => id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Compartment.prototype.globalThis: this is not a Compartment".into(),
                    ))
                }
            };
            Ok(rt.object_get(this_id, "__compartment_globalthis"))
        });

        // Save proto for the ctor closure.
        let proto_for_ctor = proto;
        let ctor_obj = make_native_with_length("Compartment", 1, move |rt, args| {
            // Parse options: { globals?: Object, modules?: Object }.
            let opts = args.first().cloned().unwrap_or(Value::Undefined);
            let mut endowments: std::collections::HashMap<String, Value> =
                std::collections::HashMap::new();
            if let Value::Object(opts_id) = &opts {
                let globals_v = rt.object_get(*opts_id, "globals");
                if let Value::Object(globals_id) = globals_v {
                    // Iterate own enumerable keys per ECMA OrdinaryOwnPropertyKeys.
                    let keys = rt.ordinary_own_enumerable_string_keys(globals_id);
                    for k in keys {
                        let v = rt.object_get(globals_id, &k);
                        endowments.insert(k, v);
                    }
                }
                // `modules` field stored as a slot for CP-EXT 4 import().
            }
            // Compartment-instance modules slot: clone {spec: source} pairs
            // into a fresh internal object; import(spec) looks up there.
            let modules_slot = rt.alloc_object(Object::new_ordinary());
            if let Value::Object(opts_id) = &opts {
                let mods_v = rt.object_get(*opts_id, "modules");
                if let Value::Object(mods_id) = mods_v {
                    let keys = rt.ordinary_own_enumerable_string_keys(mods_id);
                    for k in keys {
                        let v = rt.object_get(mods_id, &k);
                        if let Value::String(_) = v {
                            rt.object_set(modules_slot, k, v);
                        }
                        // Non-string entries silently skipped at this round;
                        // Module Source records would be the typed alternative.
                    }
                }
            }
            let realm_idx = rt.allocate_compartment_realm(endowments.clone());
            // Per-compartment globalThis: a fresh object pre-populated with
            // the endowments (so `globalThis.x === x` inside the compartment).
            let gt = rt.alloc_object(Object::new_ordinary());
            for (k, v) in &endowments {
                rt.object_set(gt, k.clone(), v.clone());
            }
            // The Compartment instance.
            let inst_obj = Object::new_ordinary();
            let inst = rt.alloc_object(inst_obj);
            rt.obj_mut(inst).proto = Some(proto_for_ctor);
            rt.object_set(
                inst,
                "__compartment_realm".into(),
                Value::Number(realm_idx as f64),
            );
            rt.object_set(inst, "__compartment_globalthis".into(), Value::Object(gt));
            rt.object_set(
                inst,
                "__compartment_modules".into(),
                Value::Object(modules_slot),
            );
            Ok(Value::Object(inst))
        });
        let ctor = self.alloc_object(ctor_obj);
        self.obj_mut(ctor)
            .set_own_frozen("prototype".into(), Value::Object(proto));
        self.obj_mut(proto)
            .set_own_internal("constructor".into(), Value::Object(ctor));
        self.globals
            .insert("Compartment".to_string(), Value::Object(ctor));
    }

    fn install_destructure_iter_helpers(&mut self) {
        register_engine_helper(self, "__destr_iter_open", |rt, args| {
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            if matches!(v, Value::Null | Value::Undefined) {
                return Err(RuntimeError::TypeError(format!(
                    "cannot destructure {}",
                    if matches!(v, Value::Null) {
                        "null"
                    } else {
                        "undefined"
                    }
                )));
            }
            // For string sources, build the string-iterator object the
            // existing helper supplies (matches what `for (c of "abc")` does).
            if let Value::String(s) = &v {
                let iter = crate::iterator::make_string_iterator(rt, (**s).clone());
                return Ok(Value::Object(iter));
            }
            let v_obj = match &v {
                Value::Object(id) => *id,
                _ => return Err(RuntimeError::TypeError("value is not iterable".into())),
            };
            let iter_fn = rt.object_get(v_obj, "@@iterator");
            if !matches!(iter_fn, Value::Object(_)) {
                return Err(RuntimeError::TypeError(
                    "value is not iterable (no @@iterator)".into(),
                ));
            }
            let iter = rt.call_function(iter_fn, Value::Object(v_obj), Vec::new())?;
            match iter {
                Value::Object(_) => Ok(iter),
                _ => Err(RuntimeError::TypeError(
                    "[Symbol.iterator]() returned non-object".into(),
                )),
            }
        });
        register_engine_helper(self, "__destr_iter_step", |rt, args| {
            let iter = args.first().cloned().unwrap_or(Value::Undefined);
            let iter_obj = match iter {
                Value::Object(id) => id,
                _ => return Err(RuntimeError::TypeError("iterator is not an object".into())),
            };
            let next_fn = rt.object_get(iter_obj, "next");
            rt.call_function(next_fn, Value::Object(iter_obj), Vec::new())
        });
        register_engine_helper(self, "__destr_iter_close", |rt, args| {
            // ECMA-262 §7.4.9 IteratorClose. Called when the destructure
            // exits without exhausting the iterator (per §13.15.5.3 step 5).
            // If iter.return is undefined, NormalCompletion(); if it's a
            // function, call it with this=iter, args=[]. Throw if the call
            // throws (propagates per IteratorClose step 7).
            let iter = args.first().cloned().unwrap_or(Value::Undefined);
            let iter_obj = match iter {
                Value::Object(id) => id,
                _ => return Ok(Value::Undefined),
            };
            let ret = rt.object_get(iter_obj, "return");
            if matches!(ret, Value::Undefined | Value::Null) {
                return Ok(Value::Undefined);
            }
            if !matches!(ret, Value::Object(_)) {
                // Per §7.4.9 step 4-5: GetMethod returns undefined for non-callable;
                // callable check is implicit via Value::Object above.
                return Ok(Value::Undefined);
            }
            rt.call_function(ret, Value::Object(iter_obj), Vec::new())
        });
        register_engine_helper(self, "__destr_iter_rest", |rt, args| {
            let iter = args.first().cloned().unwrap_or(Value::Undefined);
            let iter_obj = match iter {
                Value::Object(id) => id,
                _ => return Err(RuntimeError::TypeError("iterator is not an object".into())),
            };
            let out_id = rt.alloc_object(Object::new_array());
            let next_fn = rt.object_get(iter_obj, "next");
            let mut write_idx: usize = 0;
            loop {
                let result =
                    rt.call_function(next_fn.clone(), Value::Object(iter_obj), Vec::new())?;
                let r_obj = match result {
                    Value::Object(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "iter.next() returned non-object".into(),
                        ))
                    }
                };
                let done = rt.object_get(r_obj, "done");
                if abstract_ops::to_boolean(&done) {
                    break;
                }
                let v = rt.object_get(r_obj, "value");
                rt.object_set(out_id, write_idx.to_string(), v);
                rt.object_set(
                    out_id,
                    "length".into(),
                    Value::Number((write_idx + 1) as f64),
                );
                write_idx += 1;
            }
            Ok(Value::Object(out_id))
        });
    }

    /// Tier-Ω.5.g.3: helpers the compiler emits LoadGlobal+Call into for
    /// rest-collection during destructure. Installed as plain globals
    /// under `__`-prefixed names so user JS sees them.
    fn install_destructure_helpers(&mut self) {
        register_engine_helper(self, "__destr_array_rest", |rt, args| {
            let src = args.first().cloned().unwrap_or(Value::Undefined);
            let start = abstract_ops::to_number(args.get(1).unwrap_or(&Value::Undefined)) as usize;
            let out_id = rt.alloc_object(Object::new_array());
            let src_id = match src {
                Value::Object(id) => id,
                _ => return Ok(Value::Object(out_id)),
            };
            let len = rt.array_length(src_id);
            let mut write_idx: usize = 0;
            for i in start..len {
                let v = rt.object_get(src_id, &i.to_string());
                rt.object_set(out_id, write_idx.to_string(), v);
                write_idx += 1;
            }
            Ok(Value::Object(out_id))
        });
        register_engine_helper(self, "__destr_object_rest", |rt, args| {
            let src = args.first().cloned().unwrap_or(Value::Undefined);
            let excluded = args.get(1).cloned().unwrap_or(Value::Undefined);
            let out_id = rt.alloc_object(Object::new_ordinary());
            let src_id = match src {
                Value::Object(id) => id,
                _ => return Ok(Value::Object(out_id)),
            };
            // Build excluded-set from the array-arg.
            let mut excluded_keys: Vec<String> = Vec::new();
            if let Value::Object(ex_id) = excluded {
                let n = rt.array_length(ex_id);
                for i in 0..n {
                    let v = rt.object_get(ex_id, &i.to_string());
                    excluded_keys.push(abstract_ops::to_string(&v).as_str().to_string());
                }
            }
            // Lift (rung-14): canonical OrdinaryOwnEnumerableStringKeys.
            // Spec sec 14.3.1 (rest-binding) uses CopyDataProperties =
            // [[OwnPropertyKeys]] + [[Get]] per key. The [[Get]] dispatches
            // accessor getters - simply cloning the descriptor's value
            // field skipped them, so a rest pattern over { get v(){...} }
            // copied undefined. Routed through the canonical helper so
            // ordering and the @@/__primitive__/Array-length filters
            // match every other own-enumerable-keys site.
            let keys = rt.ordinary_own_enumerable_string_keys(src_id);
            for k in keys {
                if excluded_keys.iter().any(|e| e == &k) {
                    continue;
                }
                let v = rt.read_property(src_id, &k)?;
                rt.object_set(out_id, k, v);
            }
            Ok(Value::Object(out_id))
        });
    }

    fn install_globals(&mut self) {
        // Tier-Ω.5.P27.E1.global-hasOwnProperty: webpack-bundled CJS
        // packages reach for `hasOwnProperty` as a global identifier
        // (`hasOwnProperty.call(obj, key)`) rather than going through
        // `Object.prototype.hasOwnProperty.call`. Per ECMA-262 this
        // resolution falls through globalThis → Object.prototype, which
        // works in a real sloppy-mode global env but not in our snapshot-
        // shaped globals map. Install a direct global wrapper that
        // forwards to the spec implementation. Surfaced via Ω.5.P24.E1
        // proto-chain probe walking @jest/expect.
        register_global_fn(self, "hasOwnProperty", |rt, args| {
            let target = args.first().cloned().unwrap_or(Value::Undefined);
            let key = abstract_ops::to_string(&args.get(1).cloned().unwrap_or(Value::Undefined));
            match target {
                Value::Object(id) => Ok(Value::Boolean(rt.obj(id).has_own_str(key.as_str()))),
                _ => Ok(Value::Boolean(false)),
            }
        });
        // Tier-Ω.5.eee: atob / btoa base64 globals (HTML living standard,
        // also exposed by Node 16+). entities + parse5 depend on atob to
        // decode their packed trie data at module load.
        register_global_fn(self, "atob", |_rt, args| {
            let s = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => return Err(RuntimeError::TypeError("atob: expected a string".into())),
            };
            // Standard base64 with padding tolerance.
            let cleaned: String = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();
            let decoded = base64_decode(&cleaned).map_err(|e| {
                RuntimeError::Thrown(Value::String(Rc::new(format!(
                    "InvalidCharacterError: {}",
                    e
                ))))
            })?;
            // Per spec atob returns a binary string (one byte per char).
            let out: String = decoded.iter().map(|&b| b as char).collect();
            Ok(Value::String(Rc::new(out)))
        });
        register_global_fn(self, "btoa", |_rt, args| {
            let s = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => return Err(RuntimeError::TypeError("btoa: expected a string".into())),
            };
            let bytes: Vec<u8> = s.chars().map(|c| c as u8).collect();
            Ok(Value::String(Rc::new(base64_encode(&bytes))))
        });
        register_global_fn(self, "parseInt", |rt, args| {
            crate::generated::parse_int(rt, rt.current_this(), args)
        });
        register_global_fn(self, "parseFloat", |rt, args| {
            crate::generated::parse_float(rt, rt.current_this(), args)
        });
        // ECMA-262 §19.2.6 URI handling. v1 uses Rust's percent-encoding
        // standard library mappings; the unreserved-character sets match
        // RFC 3986. encodeURI keeps reserved chars (`/`, `?`, `:`, `@`,
        // `&`, `=`, `+`, `$`, `,`, `#`, `;`); encodeURIComponent encodes
        // all reserved chars.
        register_global_fn(self, "encodeURIComponent", |_rt, args| {
            let s = match args.first() {
                Some(v) => crate::abstract_ops::to_string(v).as_str().to_string(),
                None => "undefined".to_string(),
            };
            Ok(Value::String(Rc::new(uri_percent_encode(&s, false))))
        });
        register_global_fn(self, "encodeURI", |_rt, args| {
            let s = match args.first() {
                Some(v) => crate::abstract_ops::to_string(v).as_str().to_string(),
                None => "undefined".to_string(),
            };
            Ok(Value::String(Rc::new(uri_percent_encode(&s, true))))
        });
        register_global_fn(self, "decodeURIComponent", |_rt, args| {
            let s = match args.first() {
                Some(v) => crate::abstract_ops::to_string(v).as_str().to_string(),
                None => "undefined".to_string(),
            };
            uri_percent_decode(&s)
                .map(|d| Value::String(Rc::new(d)))
                .ok_or_else(|| RuntimeError::TypeError("decodeURIComponent: malformed URI".into()))
        });
        register_global_fn(self, "decodeURI", |_rt, args| {
            let s = match args.first() {
                Some(v) => crate::abstract_ops::to_string(v).as_str().to_string(),
                None => "undefined".to_string(),
            };
            uri_percent_decode(&s)
                .map(|d| Value::String(Rc::new(d)))
                .ok_or_else(|| RuntimeError::TypeError("decodeURI: malformed URI".into()))
        });
        // Ω.5.P63.E9: global isNaN / isFinite routed through IR-lowered
        // generated::global_is_*. Differ from Number.isNaN / Number.isFinite
        // by coercing the arg via ToNumber.
        register_global_fn(self, "isNaN", |rt, args| {
            crate::generated::global_is_nan(rt, Value::Undefined, args)
        });
        register_global_fn(self, "isFinite", |rt, args| {
            crate::generated::global_is_finite(rt, Value::Undefined, args)
        });
        // Tier-Ω.5.j.proto: Function global as a non-constructible stub.
        // Full eval-via-Function would need parser+compiler dependency
        // injection and a Closure-from-FunctionExpression path; deferred.
        // Stub throws a clearer error than "callee is not callable".
        // Tier-Ω.5.ccc: Function constructor v1 stub. The single
        // overwhelmingly-common pattern in real code is the
        // global-detection idiom `Function('return this')()` (lodash,
        // many polyfills). Recognize that exact body and return a
        // closure that yields globalThis. Everything else still
        // throws — full eval-via-Function needs a parser+compiler
        // dependency and is deferred.
        // Ω.5.P59.E3: real Function constructor per ECMA §20.2.1. Up
        // through P58 this was a stub recognizing only `Function('return
        // this')`. exceljs, express-promise-router, gulp-uglify, keystone,
        // metro, pug all use `new Function('p1', 'p2', 'body')` at
        // module-init to compile templates / pre-allocate hot paths.
        //
        // Implementation: assemble `globalThis.__fc_out = function
        // anonymous(p1, p2, ...) { body }; ` as a synthetic module
        // source, evaluate it through evaluate_module under a synthetic
        // URL, then read globalThis.__fc_out as the resulting closure.
        // The closure has NO upvalue capture from the caller (per ECMA
        // §20.2.1.1.1 CreateDynamicFunction step 4: the [[Environment]]
        // is the realm's global environment, not the caller's). All
        // free identifiers in the body resolve through globalThis.
        //
        // Special fast-path for `Function('return this')` retained for
        // identity stability — the eager lookup of globalThis at create
        // time is preserved.
        // ECMA-262 §20.2.1: Function is a constructor. `new Function(...)`
        // and `Function(...)` should both yield a function compiled from
        // the source body. depd / ejs / bluebird / uglify-js / metro and
        // many top-500 packages use `new Function(...)`.
        register_global_ctor(self, "Function", |rt, args| {
            let body = match args.last() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => String::new(),
            };
            let body_trim = body.trim();
            if body_trim == "return this" || body_trim == "return this;" {
                let global_obj = rt
                    .globals
                    .get("globalThis")
                    .cloned()
                    .unwrap_or(Value::Undefined);
                let f_obj = make_native("<Function('return this')>", move |_rt, _args| {
                    Ok(global_obj.clone())
                });
                return Ok(Value::Object(rt.alloc_object(f_obj)));
            }
            // Param list: all args except the last (which is the body).
            let params: Vec<String> = if args.len() > 1 {
                args[..args.len() - 1]
                    .iter()
                    .map(|v| crate::abstract_ops::to_string(v).as_str().to_string())
                    .collect()
            } else {
                Vec::new()
            };
            // Pick a per-call URL so the source map / line:col diagnostics
            // are distinct across multiple Function() calls.
            use std::sync::atomic::{AtomicUsize, Ordering};
            static FC_COUNTER: AtomicUsize = AtomicUsize::new(0);
            let n = FC_COUNTER.fetch_add(1, Ordering::Relaxed);
            let url = format!("file://<Function:{}>", n);
            let stash_key = format!("__fc_out_{}", n);
            // Use bare assignment so the StoreGlobal opcode writes
            // directly to runtime.globals (where we read it back).
            // `globalThis.X = ...` would SetProp the globalThis Object
            // instead of touching the globals map.
            // HLCL-EXT 1: per ECMA-262 §20.2.1.1.1 CreateDynamicFunction
            // step 13, the synthesized source places `\n` between params
            // and the closing `)`, and between `)` and the opening `{`.
            // This newline placement is what allows Annex B B.1.3
            // SingleLineHTMLCloseComment `-->` in params not to swallow
            // the `)`. Putting `({params}) {` on one line breaks the
            // HTML-comment / dynamic-function interaction; the spec
            // structure is `function anonymous(<params>\n) {\n<body>\n}`.
            let source = format!(
                "{} = function anonymous({}\n) {{\n{}\n}};",
                stash_key,
                params.join(","),
                body
            );
            match rt.evaluate_module(&source, &url) {
                Ok(_ns) => {
                    let result = rt
                        .globals
                        .get(&stash_key)
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    // Clean up the stash key — it was a side-channel,
                    // not a JS-visible global.
                    rt.globals.remove(&stash_key);
                    Ok(result)
                }
                Err(e) => Err(e),
            }
        });
        // WHATWG structuredClone — deep clone with identity preservation,
        // honoring Date / RegExp / Map / Set special cases. Functions and
        // Symbols throw DataCloneError (per spec, surfaced as TypeError
        // here for the catchable-error shape; bun uses DOMException but
        // the F-fixture's probe checks threw-true rather than ctor).
        register_global_fn(self, "structuredClone", |rt, args| {
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            let mut seen: std::collections::HashMap<u32, ObjectRef> =
                std::collections::HashMap::new();
            structured_clone_walk(rt, &v, &mut seen)
        });
        // Ω.5.P59.E4: indirect eval per ECMA §19.2.1.2 PerformEval (case
        // strictCaller=false, direct=false). Source is parsed + compiled
        // as a Script, evaluated in the global Lexical Environment. Free
        // identifiers in the source resolve through globalThis, NOT
        // through the caller's lexical scope.
        //
        // ECMA's spec-correct direct-eval — where eval is invoked by the
        // literal name `eval` at the call site and the source DOES see
        // the caller's lexical scope — requires runtime frame-walking to
        // snapshot/restore caller locals into a synthetic scope. The
        // Runtime today has no frame-stack field (cf. interp.rs:286 —
        // frames live on Rust's call stack via recursive call_function).
        // Direct-eval-with-closure-capture is therefore deferred as a
        // separate engine investment. Indirect eval covers cases like:
        //   eval('1 + 2')                                     // → 3
        //   eval('(function () { return 42; })')()             // → 42
        //   eval('({ a: 1 })')                                 // → {a:1}
        //   bundler-emitted eval('module.exports = ...')      // top-level
        // depd's eval('(function (...) { ... })') wraps in a function
        // expression whose body references outer-scope locals (log,
        // deprecate, ...); the eval'd function compiles but those refs
        // resolve via globalThis at runtime. Module-init usually doesn't
        // invoke the deprecation wrapper, so the package loads — the
        // wrapper would only throw at the deprecation site itself.
        // EXT 90 / Doc 730 §XIV + EXT 91 / Doc 730 §XV:
        // __cruftless_tolerate(name) opts into the named deviation at the
        // deviation-tier alphabet — strict-by-default is preserved;
        // consumer code (or a host wrapper script) calls this once to
        // relax a specific spec-correct rejection that the consumer's
        // dependency tree depends on Bun absorbing.
        //
        // Per §XV's constraint-comprehension contract, each deviation
        // primitive carries a 5-field shape:
        //   (name, pattern, strict_rejection, tolerant_lowering, diagnostic)
        // plus a protected_invariants list — each invariant either
        // Comprehended (the strict_rejection's spec purpose has been
        // typed as a §XIII primitive) or Waived (the engagement has
        // explicitly accepted enabling the deviation without typing
        // the invariant, with a reference to the trajectory entry that
        // records the consumer-impact analysis).
        //
        // The known-deviations registry below carries the contract
        // inline. Adding a new deviation requires either lifting its
        // protected invariants to §XIII primitives or recording the
        // Waived entry against a trajectory commit.
        register_global_fn(self, "__cruftless_tolerate", |rt, args| {
            let name = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "__cruftless_tolerate: expected string deviation name".into(),
                    ))
                }
            };
            // Deviation registry. Each entry: (name, [protected_invariants]).
            // Each protected_invariant is "C:<spec_primitive>" (Comprehended)
            // or "W:<waiver_ref>" (Waived per §XV.c).
            let known: Option<(&'static str, &[&'static str])> = match name.as_str() {
                "to-object-coerce-nullish" => Some((
                    "to-object-coerce-nullish",
                    &[
                        // Waiver #1: ECMA §7.1.18 ToObject's TypeError on
                        // null/undefined is a defensive precondition for
                        // every spec-op that requires-object-coercible
                        // (Object.keys, Object.assign, Object.setPrototypeOf,
                        // Object.entries, Object.values, spread targets,
                        // etc.). Skipping it means each downstream op now
                        // sees a fresh empty Object where it would have
                        // received a TypeError-throwing nullish. The
                        // downstream ops are themselves defensive against
                        // empty Objects, so the substitution preserves
                        // most observable behaviors — but library code
                        // depending on the TypeError as a runtime check
                        // for "did I pass undefined?" loses that signal.
                        // Waived for v1: 14-package recovery in the
                        // EXT 84-89 top500 set; trajectory record EXT 93.
                        "W:EXT-93:to-object-typeerror-as-runtime-nullcheck",
                        // Waiver #2: the @sec-ant/readable-stream module
                        // (transitive dep of got/get-stream/clipboardy/
                        // execa/got-fetch) uses Object.setPrototypeOf
                        // patterns whose target arg is computed from a
                        // chain that may be undefined under cruftless's
                        // current intrinsic install order. The deviation
                        // hides this gap rather than fixing it — could
                        // surface as observable divergence in any package
                        // whose init reads back the unset prototype.
                        "W:EXT-93:set-prototype-of-nullish-target-silent-noop",
                    ],
                )),
                "function-not-constructor-relax" => Some((
                    "function-not-constructor-relax",
                    &[
                        // Waiver #1: the spec rule (§10.3.3 + EvaluateNew step 7)
                        // is placed to protect callers whose non-constructor
                        // function bodies make this-write assumptions that
                        // assume `this` is the caller-supplied receiver, not a
                        // freshly allocated ordinary Object. Under the deviation
                        // those writes silently land in the discarded fresh
                        // Object. Waived for v1: engagement decision to accept
                        // the tradeoff for the 8-of-11 EXT-90 parity recovery;
                        // recorded against trajectory entry EXT 90 (commit
                        // 9520f504) + Doc 730 §XV.c paragraph naming this
                        // specific waiver as the worked example.
                        "W:EXT-90:non-constructor-this-write-assumption",
                        // Waiver #2: callers using `new fn()` as a runtime
                        // type-check (expecting TypeError on non-constructor)
                        // lose that signal under the deviation. Same trajectory
                        // reference; same engagement-decision rationale.
                        "W:EXT-90:typeerror-as-runtime-type-check",
                    ],
                )),
                _ => None,
            };
            let (canon, protected): (&'static str, &[&'static str]) = match known {
                Some(p) => p,
                None => {
                    return Err(RuntimeError::RangeError(format!(
                        "__cruftless_tolerate: unknown deviation '{}'",
                        name
                    )))
                }
            };
            // §XV.c: refuse to opt in if any protected invariant carries
            // an Unknown marker ("U:..."). Comprehended (C:) and Waived
            // (W:) entries pass.
            for inv in protected {
                if inv.starts_with("U:") {
                    return Err(RuntimeError::TypeError(format!(
                        "__cruftless_tolerate('{}'): refused — protected_invariant '{}' is Unknown (§XV.c contract violation; lift to §XIII typed primitive or convert to Waived entry first)", canon, inv)));
                }
            }
            rt.tolerated_deviations.insert(canon);
            Ok(Value::Undefined)
        });
        register_global_fn(self, "eval", |rt, args| {
            let source = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                Some(v) => return Ok(v.clone()), // eval(non-string) returns the arg unchanged per §19.2.1.1
                None => return Ok(Value::Undefined),
            };
            use std::sync::atomic::{AtomicUsize, Ordering};
            static EVAL_COUNTER: AtomicUsize = AtomicUsize::new(0);
            let n = EVAL_COUNTER.fetch_add(1, Ordering::Relaxed);
            let url = format!("file://<eval:{}>", n);
            // Try expression form first: wrap as assignment so the value
            // is captured in a stash global. If parse fails, fall through
            // to raw-statements form (no return value).
            let stash_key = format!("__eval_out_{}", n);
            let expr_source = format!("{} = ({});", stash_key, source);
            // EXT 74: ECMA-262 §19.2.1.1 PerformEval. Indirect eval runs the
            // source as a Script in the global Lexical Environment with
            // `this` bound to globalThis (not the caller's `this`, which
            // is the spec direct-eval shape). This matches Script semantics
            // — `this` at the top level of a Script *is* globalThis —
            // which a number of test262 fixtures (S15.3.4.3_A3_T1.js et al.)
            // depend on when they read `this[\"field\"]` after an apply()
            // assigned to globalThis inside a sloppy function.
            let saved_this = std::mem::replace(
                &mut rt.current_this,
                rt.globals
                    .get("globalThis")
                    .cloned()
                    .unwrap_or(Value::Undefined),
            );
            let expr_ok = rt.evaluate_module(&expr_source, &url).is_ok();
            if expr_ok {
                rt.current_this = saved_this;
                let result = rt
                    .globals
                    .get(&stash_key)
                    .cloned()
                    .unwrap_or(Value::Undefined);
                rt.globals.remove(&stash_key);
                return Ok(result);
            }
            // Statement form: run as-is, no captured result.
            let stmt_url = format!("file://<eval:{}:stmt>", n);
            let r = rt.evaluate_module(&source, &stmt_url);
            rt.current_this = saved_this;
            match r {
                Ok(_) => Ok(Value::Undefined),
                // §19.2.1.1 PerformEval step 5: if Script parsing fails,
                // throw a SyntaxError. Surface parse-tier CompileError as
                // a JS-catchable SyntaxError so test262 negative-phase-parse
                // tests can observe the throw.
                Err(RuntimeError::CompileError(msg)) => Err(RuntimeError::SyntaxError(msg)),
                Err(e) => Err(e),
            }
        });

        // Tier-Ω.5.yyy: expose Function.prototype on the Function
        // global. The intrinsic %Function.prototype% is the same
        // function_prototype that backs all callable instances. Adding
        // it here lets `Function.prototype.toString.call(f)` (object-
        // hash, immer-style native-function detection) resolve.
        if let Some(fp) = self.function_prototype {
            if let Some(Value::Object(fn_global)) = self.globals.get("Function").cloned() {
                self.obj_mut(fn_global)
                    .set_own_frozen("prototype".into(), Value::Object(fp));
                self.obj_mut(fp)
                    .set_own_internal("constructor".into(), Value::Object(fn_global));
            }
        }
    }

    fn install_math(&mut self) {
        let math = self.alloc_object(Object::new_ordinary());
        // Ω.5.P63.E10: Math unary one-liners routed through IR.
        register_intrinsic_method(self, math, "abs", 1, |rt, args| {
            crate::generated::math_abs(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "floor", 1, |rt, args| {
            crate::generated::math_floor(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "ceil", 1, |rt, args| {
            crate::generated::math_ceil(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "round", 1, |rt, args| {
            crate::generated::math_round(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "trunc", 1, |rt, args| {
            crate::generated::math_trunc(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "sqrt", 1, |rt, args| {
            crate::generated::math_sqrt(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "cbrt", 1, |rt, args| {
            crate::generated::math_cbrt(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E14: pow / max / min routed through IR.
        register_intrinsic_method(self, math, "pow", 2, |rt, args| {
            crate::generated::math_pow(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "max", 2, |rt, args| {
            crate::generated::math_max(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "min", 2, |rt, args| {
            crate::generated::math_min(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E10: Math.sign routed through IR. (Duplicate
        // installation at line ~1094 below is harmless: register order
        // overwrites and both paths produce identical results.)
        register_intrinsic_method(self, math, "sign", 1, |rt, args| {
            crate::generated::math_sign(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E11: Math exp/log/trig family routed through IR.
        register_intrinsic_method(self, math, "exp", 1, |rt, args| {
            crate::generated::math_exp(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "log", 1, |rt, args| {
            crate::generated::math_log(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "log2", 1, |rt, args| {
            crate::generated::math_log2(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "log10", 1, |rt, args| {
            crate::generated::math_log10(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "sin", 1, |rt, args| {
            crate::generated::math_sin(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "cos", 1, |rt, args| {
            crate::generated::math_cos(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "tan", 1, |rt, args| {
            crate::generated::math_tan(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "atan", 1, |rt, args| {
            crate::generated::math_atan(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E11: asin / acos newly installed via IR (were missing from cruftless).
        register_intrinsic_method(self, math, "asin", 1, |rt, args| {
            crate::generated::math_asin(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "acos", 1, |rt, args| {
            crate::generated::math_acos(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E14: atan2 routed through IR.
        register_intrinsic_method(self, math, "atan2", 2, |rt, args| {
            crate::generated::math_atan2(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "random", 0, |rt, args| {
            crate::generated::math_random(rt, rt.current_this(), args)
        });
        // Ω.5.P62.E3: Math constants per ECMA §21.3.1 — all
        // { writable:false, enumerable:false, configurable:false }.
        self.obj_mut(math)
            .set_own_frozen("PI".into(), Value::Number(std::f64::consts::PI));
        self.obj_mut(math)
            .set_own_frozen("E".into(), Value::Number(std::f64::consts::E));
        self.obj_mut(math)
            .set_own_frozen("LN2".into(), Value::Number(std::f64::consts::LN_2));
        self.obj_mut(math)
            .set_own_frozen("LN10".into(), Value::Number(std::f64::consts::LN_10));
        self.obj_mut(math)
            .set_own_frozen("LOG2E".into(), Value::Number(std::f64::consts::LOG2_E));
        self.obj_mut(math)
            .set_own_frozen("LOG10E".into(), Value::Number(std::f64::consts::LOG10_E));
        self.obj_mut(math)
            .set_own_frozen("SQRT2".into(), Value::Number(std::f64::consts::SQRT_2));
        // SQRT1_2 absent pre-E3.
        self.obj_mut(math).set_own_frozen(
            "SQRT1_2".into(),
            Value::Number(std::f64::consts::FRAC_1_SQRT_2),
        );

        // Tier-Ω.5.JJJJJJJJ: Math.imul / Math.fround / Math.clz32 / Math.sign /
        // Math.expm1 / Math.log1p / Math.log2 / Math.log10 / Math.cbrt /
        // Math.hypot / Math.sinh / Math.cosh / Math.tanh / Math.asinh /
        // Math.acosh / Math.atanh per ECMA-262 §21.3.
        //
        // The load-bearing one is Math.imul: bn.js's 26-bit limb arithmetic
        // depends on it for safe 32-bit integer multiplication. Without it,
        // bn.js's modular reduction produces wrong results, and elliptic's
        // secp256k1 generator-point validation fails with 'Invalid curve'
        // (4-package cluster: ethereumjs-tx / ethereumjs-util /
        // ethereumjs-wallet / secp256k1).
        // E36: Math.{imul, fround, clz32} routed through IR.
        register_intrinsic_method(self, math, "imul", 2, |rt, args| {
            crate::generated::math_imul(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, math, "fround", 1, |rt, args| {
            crate::generated::math_fround(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, math, "clz32", 1, |rt, args| {
            crate::generated::math_clz32(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, math, "sign", 1, |_rt, args| {
            let n = args
                .first()
                .map(abstract_ops::to_number)
                .unwrap_or(f64::NAN);
            if n.is_nan() {
                Ok(Value::Number(f64::NAN))
            } else if n > 0.0 {
                Ok(Value::Number(1.0))
            } else if n < 0.0 {
                Ok(Value::Number(-1.0))
            } else {
                Ok(Value::Number(n))
            } // preserves +0/-0
        });
        // Ω.5.P63.E11: expm1/log1p routed through IR.
        // (log2/log10 already routed above; this block previously
        // installed duplicates — preserve only the unique ones here.)
        register_intrinsic_method(self, math, "expm1", 1, |rt, args| {
            crate::generated::math_expm1(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "log1p", 1, |rt, args| {
            crate::generated::math_log1p(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "cbrt", 1, |_rt, args| {
            let n = args
                .first()
                .map(abstract_ops::to_number)
                .unwrap_or(f64::NAN);
            Ok(Value::Number(n.cbrt()))
        });
        // Ω.5.P63.E14: hypot routed through IR (variadic via Expr::AllArgs).
        register_intrinsic_method(self, math, "hypot", 2, |rt, args| {
            crate::generated::math_hypot(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E11: hyperbolic family routed through IR.
        register_intrinsic_method(self, math, "sinh", 1, |rt, args| {
            crate::generated::math_sinh(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "cosh", 1, |rt, args| {
            crate::generated::math_cosh(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "tanh", 1, |rt, args| {
            crate::generated::math_tanh(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "asinh", 1, |rt, args| {
            crate::generated::math_asinh(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "acosh", 1, |rt, args| {
            crate::generated::math_acosh(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, math, "atanh", 1, |rt, args| {
            crate::generated::math_atanh(rt, Value::Undefined, args)
        });

        // Ω.5.P62.E4: Math[Symbol.toStringTag] === "Math" per ECMA §21.3.1.9.
        // Drives Object.prototype.toString.call(Math) → "[object Math]"
        // (test262 Array.prototype.map-1-10 + many ducktyping libs rely
        // on this).
        self.obj_mut(math).set_own_frozen(
            "@@toStringTag".into(),
            Value::String(Rc::new("Math".into())),
        );
        self.globals.insert("Math".into(), Value::Object(math));
    }

    fn install_temporal(&mut self) {
        // TF-EXT 1 (temporal-foundation): register the Temporal namespace
        // as a frozen global with empty stub objects for Now and each
        // class (Instant, PlainDate, PlainTime, PlainDateTime,
        // PlainMonthDay, PlainYearMonth, Duration, ZonedDateTime). No
        // operative methods at this rung — sub-locales implement them.
        // Goal: `typeof Temporal === "object"` + `Temporal.Now`,
        // `Temporal.PlainDate` etc. exist as objects so instanceof checks
        // and namespace traversal work. Methods that the 3 Now tests call
        // (`plainDateTimeISO`, `zonedDateTimeISO`, `instant`) throw a
        // TypeError 'Temporal.Now.X not implemented (Tier-L stub)' until
        // temporal-now lands.
        let temporal = self.alloc_object(Object::new_ordinary());
        // Temporal.Now sub-namespace.
        let now = self.alloc_object(Object::new_ordinary());
        for method in &["plainDateTimeISO", "zonedDateTimeISO", "instant",
                        "plainDateISO", "plainTimeISO", "timeZoneId"] {
            let m = (*method).to_string();
            register_intrinsic_method(self, now, method, 0, move |_rt, _args| {
                Err(RuntimeError::TypeError(format!(
                    "Temporal.Now.{} not implemented (Tier-L stub)", m
                )))
            });
        }
        self.obj_mut(now).set_own_frozen(
            "@@toStringTag".into(),
            Value::String(Rc::new("Temporal.Now".into())),
        );
        self.obj_mut(temporal).set_own_internal("Now".into(), Value::Object(now));
        // Temporal class stubs — each is a constructor-shaped function that
        // throws "not implemented" when invoked, but exists as an object
        // so `Temporal.PlainDate` etc. is defined and `instanceof` checks
        // do not crash. Real prototype + ctor land in per-class sub-locales.
        // Stubs for classes whose per-class rung hasn't landed yet.
        // Duration + Instant are REAL (installed below); others remain stubs.
        for class_name in &["PlainDate", "PlainTime", "PlainDateTime",
                            "PlainMonthDay", "PlainYearMonth",
                            "ZonedDateTime"] {
            let stub = self.alloc_object(Object::new_ordinary());
            let cn = (*class_name).to_string();
            self.obj_mut(stub).set_own_frozen(
                "@@toStringTag".into(),
                Value::String(Rc::new(format!("Temporal.{}", cn))),
            );
            self.obj_mut(temporal).set_own_internal(
                (*class_name).into(),
                Value::Object(stub),
            );
        }
        self.obj_mut(temporal).set_own_frozen(
            "@@toStringTag".into(),
            Value::String(Rc::new("Temporal".into())),
        );
        // TDur-EXT 1 (duration-ctor-fields): install Temporal.Duration as
        // a real constructor with prototype + 10 unit getters + valueOf-
        // throws-TypeError + @@toStringTag. Per ECMA-262 §11.1 Temporal.Duration:
        //   new Temporal.Duration(years, months, weeks, days, hours,
        //                         minutes, seconds, milliseconds,
        //                         microseconds, nanoseconds)
        // Each arg is ToIntegerIfIntegral (default 0). Sign-uniformity
        // and RangeError validation deferred to duration-arithmetic rung.
        let dur_proto = self.alloc_object(Object::new_ordinary());
        // 10 unit field getters. Each reads the __td_<unit> sentinel.
        const UNITS: &[&str] = &[
            "years", "months", "weeks", "days", "hours",
            "minutes", "seconds", "milliseconds", "microseconds", "nanoseconds",
        ];
        for unit in UNITS {
            let unit_name: &'static str = unit;
            let key = format!("__td_{}", unit);
            // Accessor getter pattern per regexp.rs::install_regexp_proto_accessor.
            // `d.years` invokes the getter; tests probing
            // Object.getOwnPropertyDescriptor(proto, 'years').get also see it.
            let k = key.clone();
            let getter_obj = make_native_non_ctor(
                &format!("get {}", unit_name),
                0,
                move |rt, _args| {
                    let id = match rt.current_this() {
                        Value::Object(o) => o,
                        _ => return Err(RuntimeError::TypeError(format!(
                            "Temporal.Duration.prototype.{}: this is not an object",
                            unit_name
                        ))),
                    };
                    // Brand-check: this must be a Temporal.Duration instance.
                    // Use sentinel presence as the brand.
                    match rt.object_get(id, &k) {
                        Value::Undefined => Err(RuntimeError::TypeError(format!(
                            "Temporal.Duration.prototype.{}: this is not a Temporal.Duration",
                            unit_name
                        ))),
                        v => Ok(v),
                    }
                },
            );
            let getter_id = self.alloc_object(getter_obj);
            self.obj_mut(dur_proto).dict_mut().insert(
                unit_name.into(),
                PropertyDescriptor {
                    value: Value::Undefined,
                    writable: false,
                    enumerable: false,
                    configurable: true,
                    getter: Some(Value::Object(getter_id)),
                    setter: None,
                },
            );
        }
        // valueOf throws TypeError per §11.4.18 (Temporal.Duration is
        // not orderable / comparable via valueOf coercion).
        register_intrinsic_method(self, dur_proto, "valueOf", 0, |_rt, _args| {
            Err(RuntimeError::TypeError(
                "Temporal.Duration valueOf cannot be used; use compare()".into()
            ))
        });
        // DDP-EXT 1 (duration-derived-properties): sign + blank accessors,
        // abs + negated methods. Brand-check via __td_years sentinel
        // presence (all Duration instances carry it).
        let proto_for_derived = dur_proto;
        // DSV-EXT 1 (duration-sign-validation): per spec §11.4.2.1
        // ToTemporalDuration step "validate uniform sign", all non-zero
        // units must share the same sign. Returns RangeError if mixed.
        fn validate_uniform_sign(units: &[f64; 10]) -> Result<(), RuntimeError> {
            let mut sign: f64 = 0.0;
            for &u in units {
                if u == 0.0 { continue; }
                let s = u.signum();
                if sign == 0.0 { sign = s; }
                else if sign != s {
                    return Err(RuntimeError::RangeError(
                        "Temporal.Duration: all non-zero unit fields must share sign".into()
                    ));
                }
            }
            Ok(())
        }
        // Helper: read all 10 unit sentinels from `this`. Returns
        // [years..nanoseconds] or TypeError if `this` isn't a Duration.
        fn read_units(rt: &mut Runtime, this_id: ObjectRef) -> Result<[f64; 10], RuntimeError> {
            let units = ["years", "months", "weeks", "days", "hours",
                         "minutes", "seconds", "milliseconds",
                         "microseconds", "nanoseconds"];
            // Brand-check: first sentinel must be present.
            if matches!(rt.object_get(this_id, "__td_years"), Value::Undefined) {
                return Err(RuntimeError::TypeError(
                    "this is not a Temporal.Duration".into()
                ));
            }
            let mut out = [0.0f64; 10];
            for (i, u) in units.iter().enumerate() {
                let key = format!("__td_{}", u);
                if let Value::Number(n) = rt.object_get(this_id, &key) {
                    out[i] = n;
                }
            }
            Ok(out)
        }
        // Helper: allocate a new Duration instance with given units.
        fn make_duration(rt: &mut Runtime, proto: ObjectRef, units: [f64; 10]) -> Value {
            let units_names = ["years", "months", "weeks", "days", "hours",
                               "minutes", "seconds", "milliseconds",
                               "microseconds", "nanoseconds"];
            let mut o = Object::new_ordinary();
            o.proto = Some(proto);
            let id = rt.alloc_object(o);
            for (i, u) in units_names.iter().enumerate() {
                let key = format!("__td_{}", u);
                let v = if units[i] == 0.0 { 0.0 } else { units[i] };
                rt.set_engine_sentinel(id, &key, Value::Number(v));
            }
            Value::Object(id)
        }
        // sign accessor: -1 / 0 / 1 based on first non-zero unit's sign
        // (per spec uniform-sign invariant; if any non-zero, all non-zeros
        // share sign).
        {
            let getter = make_native_non_ctor("get sign", 0, move |rt, _args| {
                let id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Err(RuntimeError::TypeError(
                        "Temporal.Duration.prototype.sign: this is not an object".into()
                    )),
                };
                let units = read_units(rt, id)?;
                let s = units.iter().find(|&&u| u != 0.0).map_or(0.0, |&u| u.signum());
                Ok(Value::Number(s))
            });
            let getter_id = self.alloc_object(getter);
            self.obj_mut(dur_proto).dict_mut().insert(
                "sign".into(),
                PropertyDescriptor {
                    value: Value::Undefined, writable: false,
                    enumerable: false, configurable: true,
                    getter: Some(Value::Object(getter_id)), setter: None,
                },
            );
        }
        // blank accessor: true iff all units are 0.
        {
            let getter = make_native_non_ctor("get blank", 0, move |rt, _args| {
                let id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Err(RuntimeError::TypeError(
                        "Temporal.Duration.prototype.blank: this is not an object".into()
                    )),
                };
                let units = read_units(rt, id)?;
                Ok(Value::Boolean(units.iter().all(|&u| u == 0.0)))
            });
            let getter_id = self.alloc_object(getter);
            self.obj_mut(dur_proto).dict_mut().insert(
                "blank".into(),
                PropertyDescriptor {
                    value: Value::Undefined, writable: false,
                    enumerable: false, configurable: true,
                    getter: Some(Value::Object(getter_id)), setter: None,
                },
            );
        }
        // abs() method: new Duration with abs(unit) for each.
        register_intrinsic_method(self, dur_proto, "abs", 0, move |rt, _args| {
            let id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError(
                    "Temporal.Duration.prototype.abs: this is not an object".into()
                )),
            };
            let mut units = read_units(rt, id)?;
            for u in units.iter_mut() { *u = u.abs(); }
            Ok(make_duration(rt, proto_for_derived, units))
        });
        // DWith-EXT 1 (duration-with): with(durationLike) returns a new
        // Duration where unit-name keys in durationLike OVERRIDE the
        // existing units. Primitives / non-objects throw TypeError; an
        // object with no recognized unit-name keys throws TypeError.
        register_intrinsic_method(self, dur_proto, "with", 1, move |rt, args| {
            let id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError(
                    "Temporal.Duration.prototype.with: this is not an object".into()
                )),
            };
            let mut units = read_units(rt, id)?;
            // Argument must be an Object (not undefined / null / primitive).
            let arg = args.first().cloned().unwrap_or(Value::Undefined);
            let arg_id = match arg {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError(
                    "Temporal.Duration.prototype.with: argument must be an object".into()
                )),
            };
            let units_names = ["years", "months", "weeks", "days", "hours",
                               "minutes", "seconds", "milliseconds",
                               "microseconds", "nanoseconds"];
            let mut has_any = false;
            for (i, u) in units_names.iter().enumerate() {
                let v = rt.object_get(arg_id, u);
                if matches!(v, Value::Undefined) { continue; }
                has_any = true;
                let n = crate::abstract_ops::to_number(&v);
                if !n.is_finite() || n != n.trunc() {
                    return Err(RuntimeError::RangeError(format!(
                        "Temporal.Duration.prototype.with: {} must be a finite integer", u
                    )));
                }
                units[i] = if n == 0.0 { 0.0 } else { n };
            }
            if !has_any {
                return Err(RuntimeError::TypeError(
                    "Temporal.Duration.prototype.with: argument must have at least one unit property".into()
                ));
            }
            // DSV-EXT 1: uniform-sign after the merge (the merged-in fields
            // might conflict with retained ones).
            validate_uniform_sign(&units)?;
            Ok(make_duration(rt, proto_for_derived, units))
        });
        // negated() method: new Duration with -unit for each.
        register_intrinsic_method(self, dur_proto, "negated", 0, move |rt, _args| {
            let id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError(
                    "Temporal.Duration.prototype.negated: this is not an object".into()
                )),
            };
            let mut units = read_units(rt, id)?;
            for u in units.iter_mut() { *u = if *u == 0.0 { 0.0 } else { -*u }; }
            Ok(make_duration(rt, proto_for_derived, units))
        });
        self.obj_mut(dur_proto).set_own_frozen(
            "@@toStringTag".into(),
            Value::String(Rc::new("Temporal.Duration".into())),
        );
        let proto_for_ctor = dur_proto;
        let dur_ctor_obj = make_native_with_length("Duration", 0, move |rt, args| {
            // Per §11.1.1 step 1: if NewTarget is undefined, throw TypeError.
            if rt.current_new_target.is_none() {
                return Err(RuntimeError::TypeError(
                    "Temporal.Duration constructor cannot be called as a function".into()
                ));
            }
            // ToIntegerIfIntegral for each of 10 args; undefined -> 0.
            // Negative zero -> 0 per spec ToIntegerIfIntegral.
            let mut units = [0.0f64; 10];
            for (i, slot) in units.iter_mut().enumerate() {
                let v = args.get(i).cloned().unwrap_or(Value::Undefined);
                let n = match v {
                    Value::Undefined => 0.0,
                    _ => crate::abstract_ops::to_number(&v),
                };
                // ToIntegerIfIntegral: must be integer or RangeError.
                // Sign-uniformity check deferred to duration-arithmetic.
                if !n.is_finite() {
                    return Err(RuntimeError::RangeError(
                        "Temporal.Duration: arguments must be finite integers".into()
                    ));
                }
                if n != n.trunc() {
                    return Err(RuntimeError::RangeError(
                        "Temporal.Duration: arguments must be integers".into()
                    ));
                }
                // Normalize -0 to 0.
                *slot = if n == 0.0 { 0.0 } else { n };
            }
            // DSV-EXT 1: uniform-sign invariant.
            validate_uniform_sign(&units)?;
            let mut o = Object::new_ordinary();
            o.proto = Some(proto_for_ctor);
            let id = rt.alloc_object(o);
            for (i, unit) in UNITS.iter().enumerate() {
                let key = format!("__td_{}", unit);
                rt.set_engine_sentinel(id, &key, Value::Number(units[i]));
            }
            Ok(Value::Object(id))
        });
        let dur_ctor = self.alloc_object(dur_ctor_obj);
        self.obj_mut(dur_proto)
            .set_own_internal("constructor".into(), Value::Object(dur_ctor));
        // Install ctor.prototype so `instanceof Temporal.Duration` works
        // and `Temporal.Duration.prototype` is the dur_proto object.
        self.obj_mut(dur_ctor)
            .set_own_frozen("prototype".into(), Value::Object(dur_proto));
        // DStat-EXT 1 (duration-static): Temporal.Duration.from + compare.
        // IDP-EXT 1 (iso-duration-parse): parse ISO 8601 duration string
        // into a 10-unit array per ECMA-262 §11.8.1. Grammar:
        //   ('+' | '-')? 'P' (n 'Y')? (n 'M')? (n 'W')? (n 'D')?
        //     ('T' (n 'H')? (n 'M')? (n 'S')?)?
        // - At least one designator must follow P.
        // - The smallest non-zero unit may carry a fractional part.
        // - The fractional part must be on the smallest specified unit.
        // - T must be followed by at least one time designator.
        // - Sign prefix applies to all units.
        fn parse_iso_duration(s: &str) -> Option<[f64; 10]> {
            // Returns None on parse failure; spec mandates SyntaxError.
            let bytes = s.as_bytes();
            let mut i = 0;
            // Optional sign.
            let sign: f64 = match bytes.get(i) {
                Some(b'+') => { i += 1; 1.0 }
                Some(b'-') => { i += 1; -1.0 }
                Some(0xE2) if bytes.get(i+1) == Some(&0x88) && bytes.get(i+2) == Some(&0x92) => {
                    // U+2212 MINUS SIGN per spec also accepted.
                    i += 3; -1.0
                }
                _ => 1.0,
            };
            // P (case-insensitive per spec).
            match bytes.get(i) {
                Some(b'P') | Some(b'p') => i += 1,
                _ => return None,
            }
            let mut units = [0.0f64; 10];
            // Designator order: Y, M, W, D (date part); then T, H, M, S (time part).
            // Date-part Ms are months; time-part Ms are minutes.
            let date_designators: [(u8, usize); 4] = [
                (b'Y', 0), (b'M', 1), (b'W', 2), (b'D', 3),
            ];
            let time_designators: [(u8, usize); 3] = [
                (b'H', 4), (b'M', 5), (b'S', 6),
            ];
            let mut any_designator = false;
            // Helper: parse a number (possibly fractional). Returns
            // (integer_part, fractional_part, new_index, has_fractional).
            fn parse_number(b: &[u8], mut j: usize) -> Option<(f64, f64, usize, bool)> {
                let int_start = j;
                while j < b.len() && b[j].is_ascii_digit() { j += 1; }
                if j == int_start { return None; }
                let int_str = std::str::from_utf8(&b[int_start..j]).ok()?;
                let int_val: f64 = int_str.parse().ok()?;
                let mut frac_val: f64 = 0.0;
                let mut has_frac = false;
                if matches!(b.get(j), Some(b'.') | Some(b',')) {
                    has_frac = true;
                    j += 1;
                    let frac_start = j;
                    while j < b.len() && b[j].is_ascii_digit() { j += 1; }
                    if j == frac_start { return None; } // dot without digits
                    let frac_str = std::str::from_utf8(&b[frac_start..j]).ok()?;
                    let frac_int: f64 = frac_str.parse().ok()?;
                    let divisor = 10f64.powi((j - frac_start) as i32);
                    frac_val = frac_int / divisor;
                }
                Some((int_val, frac_val, j, has_frac))
            }
            // Helper: consume designators in fixed order. Returns the index
            // of the LAST consumed designator (for "fractional only on
            // smallest" enforcement), or None if invalid.
            fn consume_part(
                b: &[u8],
                mut i: usize,
                designators: &[(u8, usize)],
                units: &mut [f64; 10],
                sign: f64,
                fractional_taken: &mut Option<usize>,
            ) -> Option<(usize, bool)> {
                let mut consumed_any = false;
                let mut next_d = 0;
                while i < b.len() && next_d < designators.len() {
                    if fractional_taken.is_some() {
                        // Once fractional is taken, no more designators allowed.
                        return None;
                    }
                    // Try parsing a number starting at i.
                    let (int_val, frac_val, new_i, has_frac) = match parse_number(b, i) {
                        Some(t) => t,
                        None => break,
                    };
                    if new_i >= b.len() { return None; }
                    let designator_byte = b[new_i].to_ascii_uppercase();
                    // Skip designators until we find this one.
                    while next_d < designators.len() && designators[next_d].0 != designator_byte {
                        next_d += 1;
                    }
                    if next_d >= designators.len() { return None; }
                    let slot = designators[next_d].1;
                    units[slot] = sign * (int_val + frac_val);
                    if has_frac { *fractional_taken = Some(slot); }
                    consumed_any = true;
                    next_d += 1; // can only use each designator once and in order
                    i = new_i + 1; // consume the designator byte
                }
                Some((i, consumed_any))
            }
            // Date part: indices 0..3 in `units` are years/months/weeks/days.
            let mut fractional_taken: Option<usize> = None;
            let (mut i, consumed_date) = consume_part(
                bytes, i, &date_designators, &mut units, sign, &mut fractional_taken
            )?;
            if consumed_date { any_designator = true; }
            // Optional T section.
            if i < bytes.len() && (bytes[i] == b'T' || bytes[i] == b't') {
                i += 1;
                // Time-part Ms are minutes (slot 5), not months (slot 1).
                // The shared designators table uses slot indices into units.
                let (new_i, consumed_time) = consume_part(
                    bytes, i, &time_designators, &mut units, sign, &mut fractional_taken
                )?;
                if !consumed_time { return None; } // T with no time units
                any_designator = true;
                i = new_i;
            }
            // Must have consumed at least one designator AND reached end.
            if !any_designator { return None; }
            if i != bytes.len() { return None; }
            // Fractional on smallest enforcement: parse_number set
            // fractional_taken to the unit's slot; verify no subsequent
            // designator could have followed. consume_part already
            // ensures no more designators after fractional, so this is OK.
            // Fractional must round to integer when stored in non-second
            // units — for now we accept the fraction in units array; the
            // caller's integer-validation will reject.
            // EXCEPT: spec actually allows fractional in HOUR / MINUTE
            // to be propagated to seconds; we don't do that here; tests
            // that use fractional H/M will fail with integer-validation.
            let _ = (sign, consumed_date);
            Some(units)
        }
        let proto_for_from = dur_proto;
        register_intrinsic_method(self, dur_ctor, "from", 1, move |rt, args| {
            let item = args.first().cloned().unwrap_or(Value::Undefined);
            // IDP-EXT 1: parse ISO 8601 duration string per §11.8.1.
            if let Value::String(s) = &item {
                let units = parse_iso_duration(s).ok_or_else(|| RuntimeError::RangeError(format!(
                    "Temporal.Duration.from(string): invalid ISO 8601 duration: {:?}", s
                )))?;
                // Integer-validate (spec: each unit must be integer).
                for u in &units {
                    if !u.is_finite() || *u != u.trunc() {
                        return Err(RuntimeError::RangeError(
                            "Temporal.Duration.from(string): fractional unit out of position".into()
                        ));
                    }
                }
                validate_uniform_sign(&units)?;
                return Ok(make_duration(rt, proto_for_from, units));
            }
            let id = match item {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError(
                    "Temporal.Duration.from: argument must be a Duration, object, or string".into()
                )),
            };
            let units_names = ["years", "months", "weeks", "days", "hours",
                               "minutes", "seconds", "milliseconds",
                               "microseconds", "nanoseconds"];
            let mut units = [0.0f64; 10];
            // If the object is already a Temporal.Duration, clone its
            // internal slots. Brand-check via __td_years sentinel.
            let is_duration = !matches!(rt.object_get(id, "__td_years"), Value::Undefined);
            if is_duration {
                for (i, u) in units_names.iter().enumerate() {
                    let key = format!("__td_{}", u);
                    if let Value::Number(n) = rt.object_get(id, &key) {
                        units[i] = n;
                    }
                }
            } else {
                // Property bag: read each unit name; undefined / missing -> 0.
                // Per spec, an empty property bag throws TypeError; tracked
                // by has_any_unit below.
                let mut has_any_unit = false;
                for (i, u) in units_names.iter().enumerate() {
                    let v = rt.object_get(id, u);
                    if matches!(v, Value::Undefined) { continue; }
                    has_any_unit = true;
                    let n = crate::abstract_ops::to_number(&v);
                    if !n.is_finite() || n != n.trunc() {
                        return Err(RuntimeError::RangeError(format!(
                            "Temporal.Duration.from: {} must be a finite integer", u
                        )));
                    }
                    units[i] = if n == 0.0 { 0.0 } else { n };
                }
                if !has_any_unit {
                    return Err(RuntimeError::TypeError(
                        "Temporal.Duration.from: object must have at least one duration unit property".into()
                    ));
                }
            }
            // DSV-EXT 1: uniform-sign invariant on the property-bag path.
            // (The Duration-clone path inherits the source's validation.)
            validate_uniform_sign(&units)?;
            Ok(make_duration(rt, proto_for_from, units))
        });
        register_intrinsic_method(self, dur_ctor, "compare", 2, move |rt, args| {
            let a = args.first().cloned().unwrap_or(Value::Undefined);
            let b = args.get(1).cloned().unwrap_or(Value::Undefined);
            // Both args must be coercible to Duration. Reuse from() logic
            // inline: if it's a string, defer; if object, brand-or-bag.
            fn coerce(rt: &mut Runtime, v: Value) -> Result<[f64; 10], RuntimeError> {
                if let Value::String(s) = &v {
                    // IDP-EXT 1: parse ISO duration string for compare.
                    let units = parse_iso_duration(s).ok_or_else(|| RuntimeError::RangeError(format!(
                        "Temporal.Duration.compare(string): invalid ISO 8601 duration: {:?}", s
                    )))?;
                    for u in &units {
                        if !u.is_finite() || *u != u.trunc() {
                            return Err(RuntimeError::RangeError(
                                "Temporal.Duration.compare(string): fractional unit out of position".into()
                            ));
                        }
                    }
                    return Ok(units);
                }
                let id = match v {
                    Value::Object(o) => o,
                    _ => return Err(RuntimeError::TypeError(
                        "Temporal.Duration.compare: argument must be Duration, object, or string".into()
                    )),
                };
                let units_names = ["years", "months", "weeks", "days", "hours",
                                   "minutes", "seconds", "milliseconds",
                                   "microseconds", "nanoseconds"];
                let mut out = [0.0f64; 10];
                let is_dur = !matches!(rt.object_get(id, "__td_years"), Value::Undefined);
                for (i, u) in units_names.iter().enumerate() {
                    let key = if is_dur { format!("__td_{}", u) } else { (*u).to_string() };
                    if let Value::Number(n) = rt.object_get(id, &key) {
                        out[i] = n;
                    }
                }
                Ok(out)
            }
            let ua = coerce(rt, a)?;
            let ub = coerce(rt, b)?;
            // If any year/month/week present in either, relativeTo is
            // required. Without it (and without our temporal-relative-to
            // substrate), throw RangeError per spec.
            let needs_relative = ua[0] != 0.0 || ua[1] != 0.0 || ua[2] != 0.0
                              || ub[0] != 0.0 || ub[1] != 0.0 || ub[2] != 0.0;
            if needs_relative {
                // Check options.relativeTo; if present, defer (not implemented).
                let opts = args.get(2).cloned().unwrap_or(Value::Undefined);
                let has_rel = match opts {
                    Value::Object(o) => !matches!(rt.object_get(o, "relativeTo"), Value::Undefined),
                    _ => false,
                };
                if has_rel {
                    return Err(RuntimeError::TypeError(
                        "Temporal.Duration.compare with relativeTo not yet implemented (Tier-L stub)".into()
                    ));
                }
                return Err(RuntimeError::RangeError(
                    "Temporal.Duration.compare: a starting point (relativeTo) is required for years/months/weeks".into()
                ));
            }
            // Below years/months/weeks: convert to approximate nanoseconds
            // (1 day = 86400e9 ns; this is exact in the absence of DST,
            // which is fine here because there's no calendar/TZ context
            // and the spec defines this path as the no-relativeTo case).
            fn to_ns(u: [f64; 10]) -> f64 {
                u[3] * 86_400e9
                + u[4] * 3600e9
                + u[5] * 60e9
                + u[6] * 1e9
                + u[7] * 1e6
                + u[8] * 1e3
                + u[9]
            }
            let na = to_ns(ua);
            let nb = to_ns(ub);
            Ok(Value::Number(if na < nb { -1.0 } else if na > nb { 1.0 } else { 0.0 }))
        });
        // Overwrite the Duration stub on the Temporal namespace with the real ctor.
        self.obj_mut(temporal)
            .set_own_internal("Duration".into(), Value::Object(dur_ctor));
        // TInst-EXT 1 (instant-ctor-fields): Temporal.Instant class.
        // Stores epochNanoseconds as a BigInt sentinel __ti_ns.
        // Spec range: |ns| <= 8.64e21 (about ±271,821 years from epoch).
        let inst_proto = self.alloc_object(Object::new_ordinary());
        // epochNanoseconds accessor — returns the BigInt sentinel directly.
        {
            let getter = make_native_non_ctor("get epochNanoseconds", 0, |rt, _args| {
                let id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Err(RuntimeError::TypeError(
                        "Temporal.Instant.prototype.epochNanoseconds: this is not an object".into()
                    )),
                };
                match rt.object_get(id, "__ti_ns") {
                    Value::Undefined => Err(RuntimeError::TypeError(
                        "Temporal.Instant.prototype.epochNanoseconds: this is not a Temporal.Instant".into()
                    )),
                    v => Ok(v),
                }
            });
            let getter_id = self.alloc_object(getter);
            self.obj_mut(inst_proto).dict_mut().insert(
                "epochNanoseconds".into(),
                PropertyDescriptor {
                    value: Value::Undefined, writable: false,
                    enumerable: false, configurable: true,
                    getter: Some(Value::Object(getter_id)), setter: None,
                },
            );
        }
        // epochMilliseconds accessor — derives from __ti_ns by floor-div 1_000_000.
        {
            let getter = make_native_non_ctor("get epochMilliseconds", 0, |rt, _args| {
                let id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Err(RuntimeError::TypeError(
                        "Temporal.Instant.prototype.epochMilliseconds: this is not an object".into()
                    )),
                };
                let ns = match rt.object_get(id, "__ti_ns") {
                    Value::BigInt(b) => b,
                    _ => return Err(RuntimeError::TypeError(
                        "Temporal.Instant.prototype.epochMilliseconds: this is not a Temporal.Instant".into()
                    )),
                };
                // Per spec: floor(ns / 10^6). Use the BigInt's to_decimal +
                // f64 conversion: divide by 1e6 then floor.
                // Simpler: convert BigInt to f64 (lossy for very large but
                // adequate for v1; spec also has fixed range).
                let ns_f = ns.to_f64();
                Ok(Value::Number((ns_f / 1_000_000.0).floor()))
            });
            let getter_id = self.alloc_object(getter);
            self.obj_mut(inst_proto).dict_mut().insert(
                "epochMilliseconds".into(),
                PropertyDescriptor {
                    value: Value::Undefined, writable: false,
                    enumerable: false, configurable: true,
                    getter: Some(Value::Object(getter_id)), setter: None,
                },
            );
        }
        // valueOf throws TypeError per spec (Instant is not orderable via valueOf).
        register_intrinsic_method(self, inst_proto, "valueOf", 0, |_rt, _args| {
            Err(RuntimeError::TypeError(
                "Temporal.Instant valueOf cannot be used; use compare() or equals()".into()
            ))
        });
        self.obj_mut(inst_proto).set_own_frozen(
            "@@toStringTag".into(),
            Value::String(Rc::new("Temporal.Instant".into())),
        );
        let inst_proto_for_ctor = inst_proto;
        // Spec range as a decimal-string for comparison via BigInt.
        let inst_ctor_obj = make_native_with_length("Instant", 1, move |rt, args| {
            if rt.current_new_target.is_none() {
                return Err(RuntimeError::TypeError(
                    "Temporal.Instant constructor cannot be called as a function".into()
                ));
            }
            let arg = args.first().cloned().unwrap_or(Value::Undefined);
            // ToBigInt — handles BigInt, bool, string (SyntaxError on bad string).
            let ns = match crate::abstract_ops::to_bigint(rt, &arg)? {
                Value::BigInt(b) => b,
                _ => return Err(RuntimeError::TypeError(
                    "Temporal.Instant: argument must be a BigInt".into()
                )),
            };
            // Range check: |ns| <= 8.64e21. Use the BigInt's f64 conversion
            // for the bounds test; range is well within f64 precision.
            let ns_f = ns.to_f64();
            if !ns_f.is_finite() || ns_f.abs() > 8.64e21 {
                return Err(RuntimeError::RangeError(
                    "Temporal.Instant: epochNanoseconds out of range (|ns| > 8.64e21)".into()
                ));
            }
            let mut o = Object::new_ordinary();
            o.proto = Some(inst_proto_for_ctor);
            let id = rt.alloc_object(o);
            rt.set_engine_sentinel(id, "__ti_ns", Value::BigInt(ns));
            Ok(Value::Object(id))
        });
        let inst_ctor = self.alloc_object(inst_ctor_obj);
        self.obj_mut(inst_proto)
            .set_own_internal("constructor".into(), Value::Object(inst_ctor));
        self.obj_mut(inst_ctor)
            .set_own_frozen("prototype".into(), Value::Object(inst_proto));
        // TIS-EXT 1 (instant-static): from / fromEpochMilliseconds /
        // fromEpochNanoseconds / compare on Temporal.Instant ctor.
        fn make_instant(rt: &mut Runtime, proto: ObjectRef, ns: crate::value::Value) -> Result<Value, RuntimeError> {
            // ns must be BigInt; range-checked.
            let big = match ns {
                Value::BigInt(b) => b,
                _ => return Err(RuntimeError::TypeError(
                    "Temporal.Instant: epochNanoseconds must be a BigInt".into()
                )),
            };
            let f = big.to_f64();
            if !f.is_finite() || f.abs() > 8.64e21 {
                return Err(RuntimeError::RangeError(
                    "Temporal.Instant: epochNanoseconds out of range (|ns| > 8.64e21)".into()
                ));
            }
            let mut o = Object::new_ordinary();
            o.proto = Some(proto);
            let id = rt.alloc_object(o);
            rt.set_engine_sentinel(id, "__ti_ns", Value::BigInt(big));
            Ok(Value::Object(id))
        }
        let proto_for_static = inst_proto;
        register_intrinsic_method(self, inst_ctor, "from", 1, move |rt, args| {
            let item = args.first().cloned().unwrap_or(Value::Undefined);
            // String form: defer ISO 8601 datetime parsing to
            // temporal-iso-string-parse shared substrate.
            if let Value::String(s) = &item {
                return Err(RuntimeError::TypeError(format!(
                    "Temporal.Instant.from(string): ISO 8601 datetime string parsing not yet implemented (Tier-L stub); got {:?}",
                    s
                )));
            }
            let id = match item {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError(
                    "Temporal.Instant.from: argument must be an Instant or string".into()
                )),
            };
            // Brand-check via __ti_ns presence; if present, clone.
            match rt.object_get(id, "__ti_ns") {
                Value::BigInt(b) => make_instant(rt, proto_for_static, Value::BigInt(b)),
                _ => Err(RuntimeError::TypeError(
                    "Temporal.Instant.from: argument is not a Temporal.Instant".into()
                )),
            }
        });
        register_intrinsic_method(self, inst_ctor, "fromEpochMilliseconds", 1, move |rt, args| {
            // Per spec: ToNumber then convert to BigInt nanoseconds.
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            let ms = crate::abstract_ops::to_number(&v);
            if !ms.is_finite() {
                return Err(RuntimeError::RangeError(
                    "Temporal.Instant.fromEpochMilliseconds: argument must be finite".into()
                ));
            }
            // Truncate then convert to ns BigInt (× 1_000_000).
            let ms_int = ms.trunc() as i64;
            let ns_str = format!("{}000000", ms_int);
            let ns = crate::bigint::JsBigInt::from_decimal(&ns_str)
                .ok_or_else(|| RuntimeError::RangeError(
                    "Temporal.Instant.fromEpochMilliseconds: cannot convert to BigInt".into()
                ))?;
            make_instant(rt, proto_for_static, Value::BigInt(std::rc::Rc::new(ns)))
        });
        register_intrinsic_method(self, inst_ctor, "fromEpochNanoseconds", 1, move |rt, args| {
            // Per spec: argument must already be BigInt (no Number coercion).
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            match v {
                Value::BigInt(_) => make_instant(rt, proto_for_static, v),
                _ => Err(RuntimeError::TypeError(
                    "Temporal.Instant.fromEpochNanoseconds: argument must be a BigInt".into()
                )),
            }
        });
        register_intrinsic_method(self, inst_ctor, "compare", 2, move |rt, args| {
            fn extract_ns(rt: &mut Runtime, v: Value) -> Result<f64, RuntimeError> {
                if let Value::String(_) = &v {
                    return Err(RuntimeError::TypeError(
                        "Temporal.Instant.compare: ISO string parsing not yet implemented (Tier-L stub)".into()
                    ));
                }
                let id = match v {
                    Value::Object(o) => o,
                    _ => return Err(RuntimeError::TypeError(
                        "Temporal.Instant.compare: argument must be Instant or string".into()
                    )),
                };
                match rt.object_get(id, "__ti_ns") {
                    Value::BigInt(b) => Ok(b.to_f64()),
                    _ => Err(RuntimeError::TypeError(
                        "Temporal.Instant.compare: argument is not a Temporal.Instant".into()
                    )),
                }
            }
            let a = extract_ns(rt, args.first().cloned().unwrap_or(Value::Undefined))?;
            let b = extract_ns(rt, args.get(1).cloned().unwrap_or(Value::Undefined))?;
            Ok(Value::Number(if a < b { -1.0 } else if a > b { 1.0 } else { 0.0 }))
        });
        self.obj_mut(temporal)
            .set_own_internal("Instant".into(), Value::Object(inst_ctor));
        self.globals.insert("Temporal".into(), Value::Object(temporal));
    }

    fn install_json(&mut self) {
        let json = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, json, "stringify", 3, |rt, args| {
            crate::generated::json_stringify(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, json, "parse", 2, |rt, args| {
            crate::generated::json_parse(rt, rt.current_this(), args)
        });
        // Ω.5.P62.E4: JSON[Symbol.toStringTag] === "JSON" per §25.5.1.5.
        self.obj_mut(json).set_own_frozen(
            "@@toStringTag".into(),
            Value::String(Rc::new("JSON".into())),
        );
        self.globals.insert("JSON".into(), Value::Object(json));
    }

    fn install_test_record(&mut self) {
        // __record(value) - testing-only intrinsic that stores its
        // argument into runtime.globals["__last_recorded"]. Used by the
        // test harness to verify side effects from microtask reactions.
        register_global_fn(self, "__record", |rt, args| {
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            rt.globals.insert("__last_recorded".into(), v);
            Ok(Value::Undefined)
        });
    }

    fn install_object_static(&mut self) {
        // Tier-Ω.5.uuuuuu: Object is a real Function (callable + constructible)
        // per ECMA-262 §20.1.1. `Object(value)` returns ToObject(value);
        // when value is undefined/null/missing, returns a fresh ordinary
        // object. `new Object(value)` behaves the same. csso / joi /
        // object.getownpropertydescriptors / power-assert / single-line-log
        // all invoke `Object(x)` or `new Object()` at module-init.
        let obj_ctor_native = make_native("Object", |rt, args| {
            // EXT 83: ECMA §20.1.1.1 Object(value).
            // - undefined / null / no arg → fresh ordinary Object.
            // - Object → pass through.
            // - primitive (Number / String / Boolean / BigInt / Symbol)
            //   → box via ToObject so the result carries the spec
            //   [[NumberData]] / [[StringData]] / [[BooleanData]] /
            //   [[BigIntData]] internal slot and Object.prototype.toString
            //   reports "[object Number]" et al. Previously every primitive
            //   path returned a fresh ordinary Object, defeating the brand.
            match args.first() {
                None | Some(Value::Undefined) | Some(Value::Null) => {
                    Ok(Value::Object(rt.alloc_object(Object::new_ordinary())))
                }
                Some(v @ Value::Object(_)) => Ok(v.clone()),
                Some(v) => rt.to_object(v),
            }
        });
        let obj_ctor = self.alloc_object(obj_ctor_native);
        // Ω.5.P63.E4: Object.keys routed through IR-lowered generated::object_keys.
        // The previous hand-written impl (with integer-index-first sort
        // + enumerable filter + @@-prefix filter) lives now in
        // rt.enumerable_own_keys, which generated::object_keys invokes
        // via CallBuiltin.
        // EXT 86: Object.keys/values/entries dispatch Proxy.ownKeys
        // when target is a Proxy. Object.keys uses EnumerableOwnProperties
        // ("key" kind) — calls trap, validates invariants, filters to
        // enumerable string-keyed properties via target's [[GetOwnProperty]].
        // Pragmatic v1 shape: filter to string keys + collect via spec_get
        // on each. Symbol keys are excluded per Object.keys spec.
        register_intrinsic_method(self, obj_ctor, "keys", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "ownKeys");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'ownKeys' trap is not callable".into(),
                            ));
                        }
                        let result = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        let trap_keys = rt.apply_proxy_own_keys_invariants(&result, tgt)?;
                        let out = rt.alloc_object(Object::new_array());
                        let mut j = 0;
                        for k in trap_keys {
                            if let Value::String(_) = &k {
                                rt.object_set(out, j.to_string(), k);
                                j += 1;
                            }
                        }
                        rt.object_set(out, "length".into(), Value::Number(j as f64));
                        return Ok(Value::Object(out));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_keys(rt, Value::Undefined, &new_args);
                }
            }
            crate::generated::object_keys(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E4: Object.values/entries routed through IR-lowered
        // generated::object_{values,entries}. Existing impl extracted to
        // rt.enumerable_own_{values,entries}.
        register_intrinsic_method(self, obj_ctor, "values", 1, |rt, args| {
            crate::generated::object_values(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "entries", 1, |rt, args| {
            crate::generated::object_entries(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E16: Object.assign routed through IR.
        register_intrinsic_method(self, obj_ctor, "assign", 2, |rt, args| {
            crate::generated::object_assign(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E7: freeze routed through IR.
        register_intrinsic_method(self, obj_ctor, "freeze", 1, |rt, args| {
            crate::generated::object_freeze(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E6: isFrozen routed through IR.
        register_intrinsic_method(self, obj_ctor, "isFrozen", 1, |rt, args| {
            crate::generated::object_is_frozen(rt, Value::Undefined, args)
        });
        // Ω.5.P61.E10: Object.seal / isSealed / preventExtensions /
        // isExtensible per ECMA §20.1.2. seal makes properties non-
        // configurable but leaves writable. preventExtensions blocks new
        // properties without touching existing.
        // Ω.5.P63.E7: seal routed through IR.
        register_intrinsic_method(self, obj_ctor, "seal", 1, |rt, args| {
            crate::generated::object_seal(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E6: isSealed routed through IR.
        register_intrinsic_method(self, obj_ctor, "isSealed", 1, |rt, args| {
            crate::generated::object_is_sealed(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E7: preventExtensions routed through IR.
        // EXT 84e: Object.preventExtensions / isExtensible dispatch Proxy
        // traps with trap-callable + boolean-coerce per §10.5.{3,4}.
        register_intrinsic_method(self, obj_ctor, "preventExtensions", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "preventExtensions");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'preventExtensions' trap is not callable".into(),
                            ));
                        }
                        let r2 = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        if !crate::abstract_ops::to_boolean(&r2) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'preventExtensions' trap returned falsy".into(),
                            ));
                        }
                        // EXT 87 / Pass C: §10.5.4 step 7 — if trap
                        // returned true but target is still extensible,
                        // throw TypeError. Otherwise the Proxy could
                        // report itself non-extensible while the
                        // underlying target remained mutable.
                        if rt.obj(tgt).extensible {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'preventExtensions' trap returned true but target is still extensible".into()));
                        }
                        return Ok(Value::Object(*id));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_prevent_extensions(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::object_prevent_extensions(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "isExtensible", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "isExtensible");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'isExtensible' trap is not callable".into(),
                            ));
                        }
                        let r2 = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        let trap_ext = crate::abstract_ops::to_boolean(&r2);
                        // EXT 87 / Pass C: §10.5.3 step 8 — trap result
                        // must SameValue(target.[[IsExtensible]]).
                        if trap_ext != rt.obj(tgt).extensible {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'isExtensible' trap result does not match target's extensibility".into()));
                        }
                        return Ok(Value::Boolean(trap_ext));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_is_extensible(rt, Value::Undefined, &new_args);
                }
            }
            crate::generated::object_is_extensible(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "groupBy", 2, |rt, args| {
            crate::generated::object_group_by(rt, rt.current_this(), args)
        });
        // Ω.5.P63.E17: Object.fromEntries routed through IR.
        register_intrinsic_method(self, obj_ctor, "fromEntries", 1, |rt, args| {
            crate::generated::object_from_entries(rt, Value::Undefined, args)
        });
        // Tier-Ω.5.j.proto: Object.defineProperty / defineProperties /
        // getOwnPropertyDescriptor / getOwnPropertyNames.
        // v1 reads only `value` from the descriptor; writable/enumerable/
        // configurable are tracked as defaults via existing object_set.
        // Accessor descriptors (get/set) are not yet honored.
        // IR-EXT 56: descriptor surface lifted into rusty-js-ir.
        // EXT 84c: Object.defineProperty / getOwnPropertyDescriptor dispatch
        // through Proxy traps when the target is a Proxy. Trap-is-not-
        // callable / trap-is-null tests gate on this — the spec routes
        // every property-descriptor mutation through [[DefineOwnProperty]]
        // / [[GetOwnProperty]], which on a Proxy is the trap. v1 went
        // straight to the IR-routed direct-target impl, silently
        // delegating to a property the Proxy doesn't own. The trap-
        // callable check follows the Reflect.defineProperty pattern.
        register_intrinsic_method(self, obj_ctor, "defineProperty", 3, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "defineProperty");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'defineProperty' trap is not callable".into(),
                            ));
                        }
                        let key = args.get(1).cloned().unwrap_or(Value::Undefined);
                        let desc = args.get(2).cloned().unwrap_or(Value::Undefined);
                        let r2 = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt), key.clone(), desc.clone()],
                        )?;
                        if !crate::abstract_ops::to_boolean(&r2) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'defineProperty' trap returned falsy".into(),
                            ));
                        }
                        // EXT 89 / Pass C: §10.5.6 invariants.
                        let key_str = crate::abstract_ops::to_string(&key).as_str().to_string();
                        rt.apply_proxy_define_property_invariant(tgt, &key_str, &desc)?;
                        return Ok(Value::Object(*id));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_define_property(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::object_define_property(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "defineProperties", 2, |rt, args| {
            crate::generated::object_define_properties(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "getOwnPropertyDescriptor", 2, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "getOwnPropertyDescriptor");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'getOwnPropertyDescriptor' trap is not callable".into(),
                            ));
                        }
                        let key = args.get(1).cloned().unwrap_or(Value::Undefined);
                        let trap_result = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt), key.clone()],
                        )?;
                        // EXT 89 / Pass C: §10.5.5 invariants (undefined-leg + non-Object check).
                        let key_str = crate::abstract_ops::to_string(&key).as_str().to_string();
                        rt.apply_proxy_get_own_property_descriptor_invariant(
                            tgt,
                            &key_str,
                            &trap_result,
                        )?;
                        return Ok(trap_result);
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_get_own_property_descriptor(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::object_get_own_property_descriptor(rt, Value::Undefined, args)
        });
        // Tier-Ω.5.rrrrrr: Object.getOwnPropertyDescriptors per §20.1.2.10.
        register_intrinsic_method(
            self,
            obj_ctor,
            "getOwnPropertyDescriptors",
            1,
            |rt, args| {
                crate::generated::object_get_own_property_descriptors(rt, Value::Undefined, args)
            },
        );
        // Ω.5.P63.E15: getOwnPropertyNames routed through IR.
        // EXT 84d / EXT 86: Object.getOwnPropertyNames dispatches
        // Proxy.ownKeys trap and applies §10.5.11 invariants
        // (apply_proxy_own_keys_invariants) before filtering the result
        // to string-keyed entries per §20.1.2.10.
        register_intrinsic_method(self, obj_ctor, "getOwnPropertyNames", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "ownKeys");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'ownKeys' trap is not callable".into(),
                            ));
                        }
                        let result = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        let trap_keys = rt.apply_proxy_own_keys_invariants(&result, tgt)?;
                        let out = rt.alloc_object(Object::new_array());
                        let mut j = 0;
                        for k in trap_keys {
                            if let Value::String(_) = &k {
                                rt.object_set(out, j.to_string(), k);
                                j += 1;
                            }
                        }
                        rt.object_set(out, "length".into(), Value::Number(j as f64));
                        return Ok(Value::Object(out));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_get_own_property_names(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::object_get_own_property_names(rt, Value::Undefined, args)
        });
        // Tier-Ω.5.LLLLLLLL: Object.getOwnPropertySymbols per ECMA-262 §20.1.2.11.
        // V1 representation: symbols are strings prefixed '@@'; return only the
        // own '@@' keys as String values (consumers that compare via Symbol.X
        // get the same string). Sufficient for define-properties-checks
        // (es-define-property / set-function-length / onetime) which probe
        // for Symbol.toStringTag / iterator placement.
        // Ω.5.P63.E15: getOwnPropertySymbols routed through IR.
        // EXT 84d / EXT 86: Object.getOwnPropertySymbols same shape,
        // filtering to Symbol-keyed entries after invariant validation.
        register_intrinsic_method(self, obj_ctor, "getOwnPropertySymbols", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "ownKeys");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'ownKeys' trap is not callable".into(),
                            ));
                        }
                        let result = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        let trap_keys = rt.apply_proxy_own_keys_invariants(&result, tgt)?;
                        let out = rt.alloc_object(Object::new_array());
                        let mut j = 0;
                        for k in trap_keys {
                            if let Value::Symbol(_) = &k {
                                rt.object_set(out, j.to_string(), k);
                                j += 1;
                            }
                        }
                        rt.object_set(out, "length".into(), Value::Number(j as f64));
                        return Ok(Value::Object(out));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_get_own_property_symbols(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::object_get_own_property_symbols(rt, Value::Undefined, args)
        });
        // Object.hasOwn per ECMA 2022 §20.1.2.13 — static convenience for
        // Object.prototype.hasOwnProperty.call. Many modern packages prefer it.
        // Ω.5.P63.E7: hasOwn routed through IR.
        register_intrinsic_method(self, obj_ctor, "hasOwn", 2, |rt, args| {
            crate::generated::object_has_own(rt, Value::Undefined, args)
        });
        // Tier-Ω.5.v: Object.create(proto, propertiesObject?). Per
        // ECMA-262 §20.1.2.2: proto must be Object or null; otherwise
        // throw TypeError. Subset: properties handled via the `value`
        // field of each descriptor (matches our defineProperty subset).
        // Tier-Ω.5.nn: Object.getPrototypeOf + Object.setPrototypeOf.
        // axios + many others destructure `const { getPrototypeOf } = Object;`
        // at module top level. Without these statics, getPrototypeOf is
        // undefined and `getPrototypeOf(Uint8Array)` errors. The Reflect
        // variant existed (Ω.5.cc) but consumer code uses Object.X.
        // Ω.5.P63.E6: getPrototypeOf / setPrototypeOf routed through IR.
        // EXT 84e: Object.getPrototypeOf / setPrototypeOf dispatch Proxy
        // traps per §10.5.{1,2}.
        register_intrinsic_method(self, obj_ctor, "getPrototypeOf", 1, |rt, args| {
            // EXT 94b / Doc 730 §XV: the 'to-object-coerce-nullish'
            // deviation (EXT 93) generated a fresh Object on null/undefined
            // input. For Object.getPrototypeOf that introduces an infinite
            // prototype-walk loop — the fresh Object's [[Prototype]] is
            // Object.prototype rather than null, so `while (p) p =
            // getPrototypeOf(p)` never terminates. Scope the deviation
            // here: nullish input under the deviation returns Null
            // directly, matching V8/Bun's behavior and preserving
            // prototype-walk termination as a protected invariant.
            // (Strict-default still throws TypeError per spec via the
            // to_object call in generated::object_get_prototype_of.)
            if matches!(args.first(), Some(Value::Undefined) | Some(Value::Null))
                && rt.tolerated_deviations.contains("to-object-coerce-nullish")
            {
                return Ok(Value::Null);
            }
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "getPrototypeOf");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'getPrototypeOf' trap is not callable".into(),
                            ));
                        }
                        let handler_proto = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        // EXT 87 / Pass C: §10.5.1 step 8 — trap return
                        // must be Object or Null. step 9 — if target is
                        // non-extensible, trap return must SameValue
                        // target.[[GetPrototypeOf]]().
                        if !matches!(handler_proto, Value::Object(_) | Value::Null) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'getPrototypeOf' trap returned non-Object non-Null".into(),
                            ));
                        }
                        if !rt.obj(tgt).extensible {
                            let target_proto = match rt.obj(tgt).proto {
                                Some(p) => Value::Object(p),
                                None => Value::Null,
                            };
                            if !crate::abstract_ops::is_strictly_equal(
                                &handler_proto,
                                &target_proto,
                            ) {
                                return Err(RuntimeError::TypeError(
                                    "Proxy 'getPrototypeOf' trap returned proto inconsistent with non-extensible target".into()));
                            }
                        }
                        return Ok(handler_proto);
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_get_prototype_of(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::object_get_prototype_of(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "setPrototypeOf", 2, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "setPrototypeOf");
                    if !matches!(trap, Value::Undefined) {
                        if !rt.is_callable(&trap) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'setPrototypeOf' trap is not callable".into(),
                            ));
                        }
                        let proto = args.get(1).cloned().unwrap_or(Value::Undefined);
                        let r2 = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt), proto.clone()],
                        )?;
                        if !crate::abstract_ops::to_boolean(&r2) {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'setPrototypeOf' trap returned falsy".into(),
                            ));
                        }
                        // EXT 87 / Pass C: §10.5.2 step 9-10 — if target
                        // is non-extensible and trap returned true, V must
                        // SameValue target.[[GetPrototypeOf]]().
                        if !rt.obj(tgt).extensible {
                            let target_proto = match rt.obj(tgt).proto {
                                Some(p) => Value::Object(p),
                                None => Value::Null,
                            };
                            if !crate::abstract_ops::is_strictly_equal(&proto, &target_proto) {
                                return Err(RuntimeError::TypeError(
                                    "Proxy 'setPrototypeOf' trap returned true but V is inconsistent with non-extensible target's prototype".into()));
                            }
                        }
                        return Ok(Value::Object(*id));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_set_prototype_of(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::object_set_prototype_of(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "create", 2, |rt, args| {
            crate::generated::object_create(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E7: Object.is routed through IR.
        register_intrinsic_method(self, obj_ctor, "is", 2, |rt, args| {
            crate::generated::object_is(rt, Value::Undefined, args)
        });
        // Tier-Ω.5.t: wire `Object.prototype` to the intrinsic %Object.prototype%
        // so consumers can read `Object.prototype.hasOwnProperty` etc.
        // Without this, `var has = Object.prototype.hasOwnProperty` (a dense
        // dequal/acorn/fast-equals idiom) errors "Cannot read property
        // 'hasOwnProperty' of undefined".
        if let Some(proto) = self.object_prototype {
            self.obj_mut(obj_ctor)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            // Tier-Ω.5.lll: Object.prototype.constructor = Object. Per
            // ECMA-262 §20.1.3.1. Without this, plain-object `.constructor`
            // returns undefined, breaking type-tag idioms like dequal's
            // `(ctor=foo.constructor) === bar.constructor` followed by
            // `ctor === Date` / `ctor === RegExp` / `ctor === Array`
            // dispatch.
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(obj_ctor));
        }
        self.globals
            .insert("Object".into(), Value::Object(obj_ctor));
    }

    fn install_array_static(&mut self) {
        // Tier-Ω.5.ttt: Array is a real Function (callable) per ECMA-262
        // §23.1. `new Array(n)` produces an array of length n;
        // `new Array(v0, v1, ...)` or `Array(v0, ...)` produces an
        // array of those values. rfdc's `new Array(keys.length)` and
        // many polyfill patterns depend on this.
        let arr_proto_ref = self.array_prototype;
        let arr_ctor_native = make_native("Array", move |rt, args| {
            // Tier-Ω.5.DDDDDDD: receiver-aware Array constructor for
            // `class Z extends Array { constructor(n) { super(n); ... } }`
            // patterns (lru-cache's ZeroArray, glob's bundled copy).
            // Op::New for the derived class synthesizes `this` with proto
            // wired to the derived class's prototype (whose own proto is
            // Array.prototype). When super(...) calls into here, the
            // existing receiver is the right object to mutate — allocating
            // a sibling array discards the derived-class proto wiring,
            // leaving the resulting instance with `this.fill` undefined.
            // Mirrors the Ω.5.ffff fix for Error.
            let receiver_id = match rt.current_this() {
                Value::Object(id) if matches!(rt.obj(id).internal_kind, InternalKind::Array) => {
                    Some(id)
                }
                _ => None,
            };
            let id = match receiver_id {
                Some(id) => id,
                None => rt.alloc_object(Object::new_array()),
            };
            if args.len() == 1 {
                if let Value::Number(n) = &args[0] {
                    // ECMA-262 §22.1.1.2 step 5: if argument is a Number
                    // and ToUint32(argument) != argument (or < 0 or
                    // non-integer or ≥ 2^32), throw RangeError. Without
                    // this, `new Array(-1)` silently constructed an array
                    // of length 0 (lossy usize cast) instead of throwing.
                    if !n.is_finite() || *n < 0.0 || *n > 4294967295.0 || n.fract() != 0.0 {
                        return Err(RuntimeError::RangeError("Invalid array length".into()));
                    }
                    let len = *n as usize;
                    rt.object_set(id, "length".into(), Value::Number(len as f64));
                    let _ = arr_proto_ref;
                    return Ok(Value::Object(id));
                }
            }
            // Variadic form: each arg becomes an element.
            for (i, v) in args.iter().enumerate() {
                rt.object_set(id, i.to_string(), v.clone());
            }
            rt.object_set(id, "length".into(), Value::Number(args.len() as f64));
            let _ = arr_proto_ref;
            Ok(Value::Object(id))
        });
        let arr_ctor = self.alloc_object(arr_ctor_native);
        register_intrinsic_method(self, arr_ctor, "isArray", 1, |rt, args| {
            crate::generated::array_is_array(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, arr_ctor, "of", 0, |rt, args| {
            crate::generated::array_of(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, arr_ctor, "from", 1, |rt, args| {
            crate::generated::array_from(rt, rt.current_this(), args)
        });
        if let Some(proto) = self.array_prototype {
            self.obj_mut(arr_ctor)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P58.E4: Array.prototype.constructor = Array per ECMA §10.2.12.
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(arr_ctor));
        }
        // ECMA-262 sec 23.1.5.2 get %Array%[@@species]: accessor whose
        // getter returns `this`. ArraySpeciesCreate (sec 23.1.3.1) reads
        // this property; when not installed, subclass-extends-Array
        // patterns degrade to plain Array (their .map / .filter / .slice
        // lose the subclass type). Pre-rung-15 cruftless's array_species_
        // create had a hand-rolled subclass branch; rung-15 routes it
        // through the spec @@species path which requires this getter.
        let species_getter = make_native("[Symbol.species]", |rt, _args| Ok(rt.current_this()));
        let species_getter_id = self.alloc_object(species_getter);
        let species_desc = crate::value::PropertyDescriptor {
            value: Value::Undefined,
            writable: false,
            enumerable: false,
            configurable: true,
            getter: Some(Value::Object(species_getter_id)),
            setter: None,
        };
        self.obj_mut(arr_ctor).dict_mut().insert(
            crate::value::PropertyKey::String("@@species".into()),
            species_desc,
        );
        self.globals.insert("Array".into(), Value::Object(arr_ctor));
    }

    /// Tier-Ω.5.s: Number static surface — constants + numeric predicates.
    /// The comment at the top of this file promised this surface; the
    /// install function was never wired. semver and friends read
    /// `Number.MAX_SAFE_INTEGER` / `Number.isInteger`, so this closure
    /// is load-bearing for the parity corpus.
    fn install_number_static(&mut self) {
        // Tier-Ω.5.z: Number is also callable: `Number("3") === 3`.
        let num_obj = make_native("Number", |rt, args| {
            // Ω.5.P62.E1: `new Number(v)` per ECMA §21.1.1 produces a
            // Number-exotic object with [[NumberData]]. We model
            // [[NumberData]] via the non-enumerable __primitive__ slot,
            // which Number.prototype.{valueOf,toString} unwrap.
            // Ω.5.P62.E19: route through coerce_to_number so Object → @@toPrimitive/valueOf/
            // toString dispatch + Symbol → TypeError + Object-with-Object-returning-coercers
            // throws TypeError per §7.1.4.
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            let n = if args.is_empty() {
                0.0
            } else {
                rt.coerce_to_number(&v)?
            };
            if rt.current_new_target.is_some() {
                let mut obj = crate::value::Object::new_ordinary();
                obj.set_own_internal("__primitive__".into(), Value::Number(n));
                // EXT 83: tag [[NumberData]] internal slot so
                // Object.prototype.toString reports "[object Number]".
                obj.internal_kind = crate::value::InternalKind::NumberWrapper(Value::Number(n));
                let proto = match rt.globals.get("Number").cloned() {
                    Some(Value::Object(id)) => match rt.object_get(id, "prototype") {
                        Value::Object(p) => Some(p),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(p) = proto {
                    obj.proto = Some(p);
                }
                let id = rt.alloc_object(obj);
                return Ok(Value::Object(id));
            }
            Ok(Value::Number(n))
        });
        let num = self.alloc_object(num_obj);
        // Constants per ECMA-262 §21.1.2.
        // Ω.5.P62.E3: Number namespace constants per ECMA §21.1.2 — all
        // { writable:false, enumerable:false, configurable:false }.
        self.obj_mut(num)
            .set_own_frozen("MAX_SAFE_INTEGER".into(), Value::Number(9007199254740991.0));
        self.obj_mut(num).set_own_frozen(
            "MIN_SAFE_INTEGER".into(),
            Value::Number(-9007199254740991.0),
        );
        self.obj_mut(num)
            .set_own_frozen("MAX_VALUE".into(), Value::Number(f64::MAX));
        self.obj_mut(num)
            .set_own_frozen("MIN_VALUE".into(), Value::Number(5e-324));
        self.obj_mut(num)
            .set_own_frozen("EPSILON".into(), Value::Number(f64::EPSILON));
        self.obj_mut(num)
            .set_own_frozen("POSITIVE_INFINITY".into(), Value::Number(f64::INFINITY));
        self.obj_mut(num)
            .set_own_frozen("NEGATIVE_INFINITY".into(), Value::Number(f64::NEG_INFINITY));
        self.obj_mut(num)
            .set_own_frozen("NaN".into(), Value::Number(f64::NAN));
        // Tier-Ω.5.ggggg: global Infinity / NaN / undefined per ECMA-262
        // §19.1. acorn's tokenizer uses `Infinity` as a sentinel in
        // `for (var i=0, e=Infinity; i<e; ...)`; without the global,
        // i<undefined is false, the loop never runs, every numeric literal
        // fails to tokenize.
        self.globals
            .insert("Infinity".into(), Value::Number(f64::INFINITY));
        self.globals.insert("NaN".into(), Value::Number(f64::NAN));
        self.globals.insert("undefined".into(), Value::Undefined);
        // Predicates. Note: Number.isX (capital-N) differs from global
        // isX in NOT coercing — typeof check first, false otherwise.
        // Ω.5.P63.E8: Number.{isInteger, isFinite, isNaN, isSafeInteger}
        // routed through IR-lowered generated::number_is_*.
        register_intrinsic_method(self, num, "isInteger", 1, |rt, args| {
            crate::generated::number_is_integer(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, num, "isFinite", 1, |rt, args| {
            crate::generated::number_is_finite(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, num, "isNaN", 1, |rt, args| {
            crate::generated::number_is_nan(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, num, "isSafeInteger", 1, |rt, args| {
            crate::generated::number_is_safe_integer(rt, Value::Undefined, args)
        });
        // Alias the global parseInt / parseFloat onto Number.
        if let Some(pi) = self.globals.get("parseInt").cloned() {
            self.object_set(num, "parseInt".into(), pi);
        }
        if let Some(pf) = self.globals.get("parseFloat").cloned() {
            self.object_set(num, "parseFloat".into(), pf);
        }
        if let Some(proto) = self.number_prototype {
            self.obj_mut(num)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P58.E4: Number.prototype.constructor = Number per ECMA §10.2.12.
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(num));
            // Ω.5.P62.E19: Number.prototype is a Number exotic with
            // [[NumberData]] = +0 per §21.1.4. Brand-checked methods
            // (toString/toFixed/valueOf) must accept Number.prototype
            // directly (Number.prototype.toString() returns "0").
            self.obj_mut(proto)
                .set_own_internal("__primitive__".into(), Value::Number(0.0));
        }
        self.globals.insert("Number".into(), Value::Object(num));
        self.install_string_global();
        self.install_boolean_global();
    }

    /// Tier-Ω.5.z: `String(x)` callable — coerces to string per ToString.
    /// `new String(x)` (wrapper object) deferred; v1 returns the primitive.
    /// Carries `String.prototype` for the dense `String.prototype.X`
    /// access idiom (axios, etc.) used by polyfills + duck-type checks.
    fn install_string_global(&mut self) {
        let str_obj = make_native("String", |rt, args| {
            // Ω.5.P61.E21: String(v) — coerce per ECMA §22.1.1.1.
            // Ω.5.P62.E1: `new String(v)` per §22.1.1 produces a
            // String-exotic object with [[StringData]] = s. Modeled via
            // non-enumerable __primitive__ slot.
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            let s_rc: Rc<String> = if args.is_empty() {
                Rc::new(String::new())
            } else if let Value::Symbol(_) = &v {
                if rt.current_new_target.is_some() {
                    return Err(RuntimeError::TypeError(
                        "Cannot convert a Symbol value to a string".into(),
                    ));
                }
                Rc::new(abstract_ops::to_string(&v).as_str().to_string())
            } else {
                Rc::new(rt.coerce_to_string(&v)?)
            };
            if rt.current_new_target.is_some() {
                let mut obj = crate::value::Object::new_ordinary();
                obj.set_own_internal("__primitive__".into(), Value::String(s_rc.clone()));
                // EXT 83: tag [[StringData]] for Object.prototype.toString brand.
                obj.internal_kind =
                    crate::value::InternalKind::StringWrapper(Value::String(s_rc.clone()));
                // Index-access compatibility: install per-char own props +
                // length so `new String("ab")[0]` reads "a" and "length"
                // is the codepoint count. Spec models these as exotic
                // own properties on the String object.
                for (i, ch) in s_rc.chars().enumerate() {
                    obj.set_own(i.to_string(), Value::String(Rc::new(ch.to_string())));
                }
                obj.set_own_frozen("length".into(), Value::Number(s_rc.chars().count() as f64));
                let proto = match rt.globals.get("String").cloned() {
                    Some(Value::Object(id)) => match rt.object_get(id, "prototype") {
                        Value::Object(p) => Some(p),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(p) = proto {
                    obj.proto = Some(p);
                }
                let id = rt.alloc_object(obj);
                return Ok(Value::Object(id));
            }
            Ok(Value::String(s_rc))
        });
        let str_id = self.alloc_object(str_obj);
        register_intrinsic_method(self, str_id, "fromCharCode", 1, |rt, args| {
            crate::generated::string_from_char_code(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, str_id, "fromCodePoint", 1, |rt, args| {
            crate::generated::string_from_code_point(rt, rt.current_this(), args)
        });
        // Tier-Ω.5.ww.b: String.raw(template, ...subs). Spec uses
        // template.raw; v1 falls back to indexed cooked values from the
        // strings array (Tier-Ω.5.ww doesn't populate .raw yet). Sufficient
        // for the camelcase / consola / styled-components patterns where
        // .raw vs cooked agree (no escape sequences requiring raw).
        register_intrinsic_method(self, str_id, "raw", 1, |rt, args| {
            crate::generated::string_raw(rt, rt.current_this(), args)
        });
        if let Some(proto) = self.string_prototype {
            self.obj_mut(str_id)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P58.E4: Constructor.prototype.constructor === Constructor
            // per ECMA-262 §10.2.12. ast-types' Type.from uses indexOf on
            // builtInCtorFns (which holds `"x".constructor`, `(123).constructor`,
            // etc.) to recognize built-in types. Pre-P58.E4 cruftless's
            // String.prototype.constructor was a separate Object (named
            // "Object"), so `"x".constructor === String` returned false and
            // the ast-types lookup fell through to the `missing name` throw.
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(str_id));
            // Ω.5.P62.E19: String.prototype is a String exotic with
            // [[StringData]] = "" per §22.1.4.
            self.obj_mut(proto).set_own_internal(
                "__primitive__".into(),
                Value::String(Rc::new(String::new())),
            );
        }
        self.globals.insert("String".into(), Value::Object(str_id));
    }

    /// Tier-Ω.5.z: `Boolean(x)` callable — coerces to boolean per ToBoolean.
    fn install_boolean_global(&mut self) {
        let b_obj = make_native("Boolean", |_rt, args| {
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            Ok(Value::Boolean(abstract_ops::to_boolean(&v)))
        });
        let b_id = self.alloc_object(b_obj);
        self.globals.insert("Boolean".into(), Value::Object(b_id));
        // Tier-Ω.5.pp: Proxy as a stub constructor. v1 deviation: the
        // proxy doesn't actually intercept operations; it's a transparent
        // pass-through that returns the target as-is. This lets `new
        // Proxy(target, handler)` not crash; access still goes through
        // the underlying target. Many packages create proxies for
        // deprecation guards or namespace shims where the trap-handling
        // isn't actually exercised during shape probe.
        let proxy_obj = make_native("Proxy", |rt, args| {
            let target = args.first().cloned().unwrap_or(Value::Undefined);
            // Return target directly; trap-handling deferred.
            let _ = (rt, args);
            Ok(target)
        });
        let proxy_id = self.alloc_object(proxy_obj);
        // Tier-Ω.5.zzzzz: Proxy.revocable(target, handler) → { proxy, revoke }.
        // immer reaches for revocable at every produce() to enforce
        // post-draft-finalization invariants. v1 deviation: proxy is the
        // target (no trap dispatch); revoke is a no-op.
        register_method(self, proxy_id, "revocable", |rt, args| {
            let target = args.first().cloned().unwrap_or(Value::Undefined);
            let revoke = make_native("revoke", |_rt, _args| Ok(Value::Undefined));
            let revoke_id = rt.alloc_object(revoke);
            let mut o = Object::new_ordinary();
            o.set_own("proxy".into(), target);
            o.set_own("revoke".into(), Value::Object(revoke_id));
            Ok(Value::Object(rt.alloc_object(o)))
        });
        self.globals.insert("Proxy".into(), Value::Object(proxy_id));

        // Tier-Ω.5.ccccc: minimal WHATWG URL global. Parses
        // scheme://[user:pass@]host[:port]/path?query#fragment and exposes
        // the standard read-only properties. Real spec parsing is intricate
        // (punycode, percent-encoding canonicalization, IDN); v1 covers
        // the URL shapes the corpus actually constructs.
        let url_ctor = make_native("URL", |rt, args| {
            let input = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                Some(v) => crate::abstract_ops::to_string(v).as_str().to_string(),
                None => return Err(RuntimeError::TypeError("URL: invalid URL".into())),
            };
            let base = match args.get(1) {
                Some(Value::String(s)) => Some(s.as_str().to_string()),
                _ => None,
            };
            // Resolve against base if provided and input is relative.
            let full = match base {
                Some(b) if !input.contains("://") && !input.starts_with("//") => {
                    // Strip filename from base path, append input.
                    let cut = b.rfind('/').map(|i| &b[..=i]).unwrap_or(&b);
                    format!("{}{}", cut, input)
                }
                _ => input.clone(),
            };
            let mut rest: &str = &full;
            let (protocol, after_scheme) = if let Some(i) = rest.find("://") {
                let p = format!("{}:", &rest[..i]);
                rest = &rest[i + 3..];
                (p, true)
            } else if let Some(i) = rest.find(':') {
                let p = format!("{}:", &rest[..i]);
                rest = &rest[i + 1..];
                (p, false)
            } else {
                ("".to_string(), false)
            };
            let (hash, rest2) = match rest.find('#') {
                Some(i) => (rest[i..].to_string(), &rest[..i]),
                None => ("".to_string(), rest),
            };
            let (search, rest3) = match rest2.find('?') {
                Some(i) => (rest2[i..].to_string(), &rest2[..i]),
                None => ("".to_string(), rest2),
            };
            let (authority, path) = if after_scheme {
                match rest3.find('/') {
                    Some(i) => (&rest3[..i], &rest3[i..]),
                    None => (rest3, ""),
                }
            } else {
                ("", rest3)
            };
            let path_s = if path.is_empty() && after_scheme {
                "/".to_string()
            } else {
                path.to_string()
            };
            let (userinfo, hostport) = match authority.rfind('@') {
                Some(i) => (&authority[..i], &authority[i + 1..]),
                None => ("", authority),
            };
            let (username, password) = match userinfo.find(':') {
                Some(i) => (&userinfo[..i], &userinfo[i + 1..]),
                None => (userinfo, ""),
            };
            let (hostname, port) = if hostport.starts_with('[') {
                // IPv6 literal.
                match hostport.find("]:") {
                    Some(i) => (&hostport[..=i], &hostport[i + 2..]),
                    None => (hostport, ""),
                }
            } else {
                match hostport.rfind(':') {
                    Some(i) => (&hostport[..i], &hostport[i + 1..]),
                    None => (hostport, ""),
                }
            };
            let origin = if protocol.is_empty() {
                "null".to_string()
            } else {
                format!("{}//{}", protocol, hostport)
            };
            let href = full.clone();

            let url_obj = match rt.current_this() {
                Value::Object(id) => id,
                _ => rt.alloc_object(Object::new_ordinary()),
            };
            // ESNE-EXT 4: URL's 11 fields are accessors on URL.prototype per
            // WHATWG URL §4.4. Min-scope: hide as non-enumerable via
            // set_engine_sentinel. Subsequent setter writes (e.g.
            // url.protocol = 'https:') preserve attrs through object_set's
            // update path. Spec-strict accessor-with-reparse semantics are
            // a separate locale candidate.
            rt.set_engine_sentinel(url_obj, "href", Value::String(Rc::new(href)));
            rt.set_engine_sentinel(url_obj, "protocol", Value::String(Rc::new(protocol)));
            rt.set_engine_sentinel(url_obj, "username", Value::String(Rc::new(username.into())));
            rt.set_engine_sentinel(url_obj, "password", Value::String(Rc::new(password.into())));
            rt.set_engine_sentinel(url_obj, "host", Value::String(Rc::new(hostport.into())));
            rt.set_engine_sentinel(url_obj, "hostname", Value::String(Rc::new(hostname.into())));
            rt.set_engine_sentinel(url_obj, "port", Value::String(Rc::new(port.into())));
            rt.set_engine_sentinel(url_obj, "pathname", Value::String(Rc::new(path_s)));
            rt.set_engine_sentinel(url_obj, "search", Value::String(Rc::new(search)));
            rt.set_engine_sentinel(url_obj, "hash", Value::String(Rc::new(hash)));
            rt.set_engine_sentinel(url_obj, "origin", Value::String(Rc::new(origin)));
            register_method(rt, url_obj, "toString", |rt, _args| {
                Ok(rt.object_get(
                    match rt.current_this() {
                        Value::Object(id) => id,
                        _ => return Ok(Value::String(Rc::new(String::new()))),
                    },
                    "href",
                ))
            });
            register_method(rt, url_obj, "toJSON", |rt, _args| {
                Ok(rt.object_get(
                    match rt.current_this() {
                        Value::Object(id) => id,
                        _ => return Ok(Value::String(Rc::new(String::new()))),
                    },
                    "href",
                ))
            });
            Ok(Value::Object(url_obj))
        });
        let url_id = self.alloc_object(url_ctor);
        let url_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(url_id)
            .set_own_frozen("prototype".into(), Value::Object(url_proto));
        register_method(self, url_id, "canParse", |_rt, args| {
            let s = match args.first() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => return Ok(Value::Boolean(false)),
            };
            Ok(Value::Boolean(
                s.contains("://") || s.starts_with("file:") || s.starts_with("data:"),
            ))
        });
        self.globals.insert("URL".into(), Value::Object(url_id));

        // Tier-Ω.5.AAAAAAA + diff-prod Rung-19: AbortController + AbortSignal
        // globals per WHATWG DOM §3.1. Signal instances carry an internal
        // listener list (__ac_listeners__) and synchronously dispatch on
        // abort. abort() is idempotent. Signal instances chain to
        // AbortSignal.prototype so `instanceof AbortSignal` resolves.
        //
        // Scope-limit deferred to a future rung: AbortSignal.timeout(ms)
        // requires routing through the host-tier timer queue (cruftless/
        // src/timer.rs), which the runtime layer cannot reach. The factory
        // is preserved as a present-but-non-firing stub; consumers that
        // depend on real timeout behavior get a non-aborting signal.
        let abort_signal_proto = self.alloc_object(Object::new_ordinary());
        let abort_signal_ctor = make_native("AbortSignal", |_rt, _args| {
            Err(RuntimeError::TypeError(
                "AbortSignal constructor not directly callable (use AbortController.prototype.abort, AbortSignal.abort, or AbortSignal.any)".into()
            ))
        });
        let abort_signal_id = self.alloc_object(abort_signal_ctor);
        self.obj_mut(abort_signal_id)
            .set_own_frozen("prototype".into(), Value::Object(abort_signal_proto));
        self.obj_mut(abort_signal_proto)
            .set_own_internal("constructor".into(), Value::Object(abort_signal_id));

        // Helper: build a fresh signal instance proto-chained to AbortSignal.prototype.
        fn alloc_abort_signal(
            rt: &mut Runtime,
            proto: ObjectRef,
            aborted: bool,
            reason: Value,
        ) -> ObjectRef {
            let mut o = Object::new_ordinary();
            o.proto = Some(proto);
            let sig = rt.alloc_object(o);
            // ESNE-EXT 4: hide as non-enumerable. Spec (WHATWG DOM §3.1) puts
            // aborted/reason as accessors on AbortSignal.prototype; onabort is
            // an event-handler attribute (also accessor). Min-scope mirrors
            // ESNE-EXT 2 (size on Map/Set): close Object.keys leak via
            // set_engine_sentinel; subsequent fire_abort updates preserve
            // attrs through the object_set update path.
            rt.set_engine_sentinel(sig, "aborted", Value::Boolean(aborted));
            rt.set_engine_sentinel(sig, "reason", reason);
            rt.set_engine_sentinel(sig, "onabort", Value::Null);
            let listeners = rt.alloc_object(Object::new_dictionary());
            rt.obj_mut(listeners)
                .set_own_internal("__count".into(), Value::Number(0.0));
            rt.obj_mut(sig)
                .set_own_internal("__ac_listeners__".into(), Value::Object(listeners));
            sig
        }

        // Helper: default reason when abort() is called with no argument is an
        // Error with name='AbortError', per DOM §3.1.4. cruftless lacks
        // DOMException; an Error with the right name is what every consumer
        // pattern-matches on (`e && e.name === 'AbortError'`).
        fn default_abort_reason(rt: &mut Runtime) -> Value {
            let e = rt.alloc_object(Object::new_ordinary());
            rt.object_set(
                e,
                "name".into(),
                Value::String(Rc::new("AbortError".into())),
            );
            rt.object_set(
                e,
                "message".into(),
                Value::String(Rc::new("The operation was aborted.".into())),
            );
            Value::Object(e)
        }

        // Helper: fire abort on a signal. Idempotent; second call is a no-op.
        fn fire_abort(rt: &mut Runtime, sig: ObjectRef, reason: Value) {
            if let Value::Boolean(true) = rt.object_get(sig, "aborted") {
                return;
            }
            rt.object_set(sig, "aborted".into(), Value::Boolean(true));
            rt.object_set(sig, "reason".into(), reason);
            // Drain listeners. Snapshot first so a listener that mutates the
            // list doesn't break iteration.
            let listeners_v = rt.object_get(sig, "__ac_listeners__");
            if let Value::Object(listeners) = listeners_v {
                let count = match rt.object_get(listeners, "__count") {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };
                let mut callbacks: Vec<Value> = Vec::with_capacity(count);
                for i in 0..count {
                    let key = format!("__l{}", i);
                    let v = rt.object_get(listeners, &key);
                    if !matches!(v, Value::Undefined) {
                        callbacks.push(v);
                    }
                }
                for cb in callbacks {
                    let _ = rt.call_function(cb, Value::Object(sig), Vec::new());
                }
            }
            // onabort sole-handler convention.
            let onabort = rt.object_get(sig, "onabort");
            if !matches!(onabort, Value::Null | Value::Undefined) {
                let _ = rt.call_function(onabort, Value::Object(sig), Vec::new());
            }
        }

        // AbortSignal.prototype.throwIfAborted — §3.1.5.
        register_method(self, abort_signal_proto, "throwIfAborted", |rt, _args| {
            let this = rt.current_this();
            if let Value::Object(sig) = this {
                if let Value::Boolean(true) = rt.object_get(sig, "aborted") {
                    let r = rt.object_get(sig, "reason");
                    return Err(RuntimeError::Thrown(r));
                }
            }
            Ok(Value::Undefined)
        });

        // AbortSignal.prototype.addEventListener — narrow shape: type='abort',
        // callback appended to __ac_listeners__. Other event types are accepted
        // but never fire (no real EventTarget). removeEventListener is a no-op
        // stub for surface presence.
        register_method(self, abort_signal_proto, "addEventListener", |rt, args| {
            let ty = match args.first() {
                Some(Value::String(s)) => (**s).clone(),
                _ => String::new(),
            };
            let cb = args.get(1).cloned().unwrap_or(Value::Undefined);
            if ty != "abort" {
                return Ok(Value::Undefined);
            }
            let this = rt.current_this();
            if let Value::Object(sig) = this {
                let listeners_v = rt.object_get(sig, "__ac_listeners__");
                if let Value::Object(listeners) = listeners_v {
                    let count = match rt.object_get(listeners, "__count") {
                        Value::Number(n) => n as usize,
                        _ => 0,
                    };
                    let key = format!("__l{}", count);
                    rt.obj_mut(listeners).set_own_internal(key.into(), cb);
                    rt.obj_mut(listeners)
                        .set_own_internal("__count".into(), Value::Number((count + 1) as f64));
                }
            }
            Ok(Value::Undefined)
        });
        register_method(
            self,
            abort_signal_proto,
            "removeEventListener",
            |_rt, _args| Ok(Value::Undefined),
        );
        register_method(self, abort_signal_proto, "dispatchEvent", |_rt, _args| {
            Ok(Value::Boolean(false))
        });

        // AbortSignal.abort(reason) — §3.1.3.1. Returns a pre-aborted signal.
        let asp_for_static = abort_signal_proto;
        register_method(self, abort_signal_id, "abort", move |rt, args| {
            let reason = match args.first() {
                Some(v) => v.clone(),
                None => default_abort_reason(rt),
            };
            let sig = alloc_abort_signal(rt, asp_for_static, true, reason);
            Ok(Value::Object(sig))
        });
        // AbortSignal.timeout(ms) — present surface; firing requires host-tier
        // timer routing (deferred). Returns a non-aborting signal so consumers
        // that defensively register listeners don't crash at install time.
        register_method(self, abort_signal_id, "timeout", move |rt, _args| {
            let sig = alloc_abort_signal(rt, asp_for_static, false, Value::Undefined);
            Ok(Value::Object(sig))
        });
        // AbortSignal.any([s1, s2, ...]) — §3.1.3.2. Composite signal that
        // aborts when any input aborts. If any input is already aborted, the
        // returned signal is pre-aborted with that input's reason.
        register_method(self, abort_signal_id, "any", move |rt, args| {
            let arr = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => {
                    return Ok(Value::Object(alloc_abort_signal(
                        rt,
                        asp_for_static,
                        false,
                        Value::Undefined,
                    )))
                }
            };
            // Iterate array-like.
            let len = match rt.object_get(arr, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            // First, check for an already-aborted input.
            for i in 0..len {
                let v = rt.object_get(arr, &i.to_string());
                if let Value::Object(s) = v {
                    if let Value::Boolean(true) = rt.object_get(s, "aborted") {
                        let r = rt.object_get(s, "reason");
                        let sig = alloc_abort_signal(rt, asp_for_static, true, r);
                        return Ok(Value::Object(sig));
                    }
                }
            }
            // Otherwise, build a composite that attaches to each input.
            let composite = alloc_abort_signal(rt, asp_for_static, false, Value::Undefined);
            for i in 0..len {
                let v = rt.object_get(arr, &i.to_string());
                if let Value::Object(s) = v {
                    let listeners_v = rt.object_get(s, "__ac_listeners__");
                    if let Value::Object(listeners) = listeners_v {
                        // Synthesize a forwarder closure that fires the composite when this input fires.
                        // We approximate the closure by storing the composite-id alongside; fire_abort
                        // doesn't know about composites, so we add a parallel __ac_forwards__ list.
                        let fwds_v = rt.object_get(s, "__ac_forwards__");
                        let fwds = if let Value::Object(id) = fwds_v {
                            id
                        } else {
                            let new_fwds = rt.alloc_object(Object::new_dictionary());
                            rt.obj_mut(new_fwds)
                                .set_own_internal("__count".into(), Value::Number(0.0));
                            rt.obj_mut(s).set_own_internal(
                                "__ac_forwards__".into(),
                                Value::Object(new_fwds),
                            );
                            new_fwds
                        };
                        let count = match rt.object_get(fwds, "__count") {
                            Value::Number(n) => n as usize,
                            _ => 0,
                        };
                        let key = format!("__f{}", count);
                        rt.obj_mut(fwds)
                            .set_own_internal(key.into(), Value::Object(composite));
                        rt.obj_mut(fwds)
                            .set_own_internal("__count".into(), Value::Number((count + 1) as f64));
                        let _ = listeners; // listeners list already exists; forwarders are a parallel channel
                    }
                }
            }
            Ok(Value::Object(composite))
        });

        self.globals
            .insert("AbortSignal".into(), Value::Object(abort_signal_id));

        let abort_controller_proto = self.alloc_object(Object::new_ordinary());
        let asp_for_ctor = abort_signal_proto;
        let acp_for_ctor = abort_controller_proto;
        let abort_controller_ctor = make_native("AbortController", move |rt, _args| {
            let mut o = Object::new_ordinary();
            o.proto = Some(acp_for_ctor);
            let inst = rt.alloc_object(o);
            let sig = alloc_abort_signal(rt, asp_for_ctor, false, Value::Undefined);
            // ESNE-EXT 4: AbortController.signal is an accessor on the proto
            // per spec; minimum-scope hide via set_engine_sentinel.
            rt.set_engine_sentinel(inst, "signal", Value::Object(sig));
            Ok(Value::Object(inst))
        });
        let abort_controller_id = self.alloc_object(abort_controller_ctor);
        self.obj_mut(abort_controller_id)
            .set_own_frozen("prototype".into(), Value::Object(abort_controller_proto));
        self.obj_mut(abort_controller_proto)
            .set_own_internal("constructor".into(), Value::Object(abort_controller_id));

        // AbortController.prototype.abort(reason) — §3.2.4.1. Fires abort on
        // this.signal, idempotent. Also drains any composite forwarders so
        // AbortSignal.any() targets fire transitively.
        register_method(self, abort_controller_proto, "abort", |rt, args| {
            let this = rt.current_this();
            if let Value::Object(inst) = this {
                let sig_v = rt.object_get(inst, "signal");
                if let Value::Object(sig) = sig_v {
                    let reason = match args.first() {
                        Some(v) => v.clone(),
                        None => default_abort_reason(rt),
                    };
                    // Snapshot composite forwarders before mutating state (the
                    // forwarders are signals whose abort() must be triggered).
                    let fwds_v = rt.object_get(sig, "__ac_forwards__");
                    let mut fwd_composites: Vec<ObjectRef> = Vec::new();
                    if let Value::Object(fwds) = fwds_v {
                        let count = match rt.object_get(fwds, "__count") {
                            Value::Number(n) => n as usize,
                            _ => 0,
                        };
                        for i in 0..count {
                            let key = format!("__f{}", i);
                            if let Value::Object(c) = rt.object_get(fwds, &key) {
                                fwd_composites.push(c);
                            }
                        }
                    }
                    fire_abort(rt, sig, reason.clone());
                    for c in fwd_composites {
                        fire_abort(rt, c, reason.clone());
                    }
                }
            }
            Ok(Value::Undefined)
        });
        self.globals
            .insert("AbortController".into(), Value::Object(abort_controller_id));

        // Tier-Ω.5.xxxxxx: URLSearchParams as a callable global Function with
        // .prototype. node-fetch's headers.js does `class Headers extends
        // URLSearchParams`; the class compile reads `URLSearchParams.prototype`
        // for [[Prototype]] wiring. A constructor stub plus an ordinary
        // .prototype object is sufficient for the inheritance chain to
        // resolve at module-init. Method bodies on the prototype remain
        // queued (get/set/has/delete/append/keys/values/entries/forEach/
        // toString) — consumers that hit them get a TypeError naming the stub.
        let usp_ctor = make_native("URLSearchParams", |_rt, _args| {
            Err(RuntimeError::TypeError(
                "URLSearchParams constructor not yet implemented (Tier-Ω.5.xxxxxx stub)".into(),
            ))
        });
        let usp_id = self.alloc_object(usp_ctor);
        let usp_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(usp_id)
            .set_own_frozen("prototype".into(), Value::Object(usp_proto));
        self.obj_mut(usp_proto)
            .set_own_internal("constructor".into(), Value::Object(usp_id));
        self.globals
            .insert("URLSearchParams".into(), Value::Object(usp_id));

        // Ω.5.P49.E3: Fetch-API constructor stubs as callable globals.
        // playwright-core's coreBundle aliases the global `Request` as
        // `GlobalRequest` and writes `class APIRequest extends GlobalRequest`,
        // which compiles down to a read of `GlobalRequest.prototype`. Each
        // stub below is a callable global with a `.prototype` carrying a
        // `.constructor` backref — sufficient for the [[Prototype]] wiring
        // at class-init, and for util.inherits(X, Request) which reads
        // super_.prototype. Real implementations are deferred.
        // Bulk-install: WHATWG stream ctors + the fetch-API ctors that
        // don't need post-construction state (Response/Blob/File/FormData
        // + the stream sub-types). These return an empty-prototype'd
        // instance; method calls on the returned value still fail
        // downstream — only the construction gate is open.
        for name in &[
            "Response",
            "FormData",
            "Blob",
            "File",
            "ReadableStream",
            "WritableStream",
            "TransformStream",
            "ReadableStreamDefaultReader",
            "ReadableStreamBYOBReader",
            "ReadableStreamDefaultController",
            "ReadableByteStreamController",
            "WritableStreamDefaultWriter",
            "WritableStreamDefaultController",
            "TransformStreamDefaultController",
            "ByteLengthQueuingStrategy",
            "CountQueuingStrategy",
            "TextEncoderStream",
            "TextDecoderStream",
        ] {
            let proto = self.alloc_object(Object::new_ordinary());
            let proto_for_closure = proto;
            let ctor = make_native(name, move |rt, _args| {
                let mut inst = Object::new_ordinary();
                inst.proto = Some(proto_for_closure);
                let id = rt.alloc_object(inst);
                Ok(Value::Object(id))
            });
            let id = self.alloc_object(ctor);
            self.obj_mut(id)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(id));
            self.globals.insert((*name).into(), Value::Object(id));
        }

        // Ω.5.P53.E4: Headers ctor with populated prototype. ky and many
        // other consumers do `new Request(url, opts).headers.has(...)` at
        // module-init; the prior empty-prototype Headers instances tripped
        // every method access. Implement the spec surface that consumers
        // touch at module-init: has/get/set/append/delete, entries/keys/
        // values, forEach. Instance state: a __headers Object keyed by
        // lowercased name → string value.
        let headers_proto = self.alloc_object(Object::new_ordinary());
        let headers_proto_for_closure = headers_proto;
        let headers_ctor_fn = make_native("Headers", move |rt, args| {
            let mut inst = Object::new_ordinary();
            inst.proto = Some(headers_proto_for_closure);
            let id = rt.alloc_object(inst);
            let bag = rt.alloc_object(Object::new_dictionary());
            rt.object_set(id, "__headers".into(), Value::Object(bag));
            // Init from arg 0: undefined / Object / Array / Headers-instance.
            if let Some(init) = args.first() {
                if let Value::Object(src) = init {
                    // Try as plain object: copy own enumerable string keys.
                    // CMig-EXT 9 Family B: shape entries first (insertion
                    // order; all enumerable + non-__headers by carve-out),
                    // then non-__headers IndexMap entries.
                    let pairs: Vec<(String, Value)> = {
                        let s = rt.obj(*src);
                        let mut out: Vec<(String, Value)> = Vec::new();
                        if let Some(shape) = s.shape.as_ref() {
                            for (name, slot) in shape.iter_slots() {
                                if name == "__headers" {
                                    continue;
                                }
                                let idx = slot as usize;
                                if let Some(v) = s.shape_values.get(idx) {
                                    out.push((name.to_string(), v.clone()));
                                }
                            }
                        }
                        out.extend(
                            s.properties
                                .iter()
                                .filter(|(k, d)| d.enumerable && k.as_str() != "__headers")
                                .map(|(k, d)| (k.to_string_content(), d.value.clone())),
                        );
                        out
                    };
                    for (k, v) in pairs {
                        let lk = k.to_ascii_lowercase();
                        let s = abstract_ops::to_string(&v).as_str().to_string();
                        rt.object_set(bag, lk, Value::String(Rc::new(s)));
                    }
                    // If the src is itself a Headers instance, fold in its __headers too.
                    if let Value::Object(src_bag) = rt.object_get(*src, "__headers") {
                        let inner: Vec<(String, Value)> = rt
                            .obj(src_bag)
                            .properties
                            .iter()
                            .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                            .collect();
                        for (k, v) in inner {
                            rt.object_set(bag, k, v);
                        }
                    }
                }
            }
            Ok(Value::Object(id))
        });
        let headers_ctor_id = self.alloc_object(headers_ctor_fn);
        self.obj_mut(headers_ctor_id)
            .set_own_frozen("prototype".into(), Value::Object(headers_proto));
        self.obj_mut(headers_proto)
            .set_own_internal("constructor".into(), Value::Object(headers_ctor_id));
        register_method(self, headers_proto, "has", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Boolean(false)),
            };
            let bag = match rt.object_get(this_id, "__headers") {
                Value::Object(b) => b,
                _ => return Ok(Value::Boolean(false)),
            };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str()
                .to_ascii_lowercase();
            Ok(Value::Boolean(!matches!(
                rt.object_get(bag, &name),
                Value::Undefined
            )))
        });
        register_method(self, headers_proto, "get", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Null),
            };
            let bag = match rt.object_get(this_id, "__headers") {
                Value::Object(b) => b,
                _ => return Ok(Value::Null),
            };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str()
                .to_ascii_lowercase();
            match rt.object_get(bag, &name) {
                Value::Undefined => Ok(Value::Null),
                v => Ok(v),
            }
        });
        register_method(self, headers_proto, "set", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let bag = match rt.object_get(this_id, "__headers") {
                Value::Object(b) => b,
                _ => return Ok(Value::Undefined),
            };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str()
                .to_ascii_lowercase();
            let value = abstract_ops::to_string(&args.get(1).cloned().unwrap_or(Value::Undefined))
                .as_str()
                .to_string();
            rt.object_set(bag, name, Value::String(Rc::new(value)));
            Ok(Value::Undefined)
        });
        register_method(self, headers_proto, "append", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let bag = match rt.object_get(this_id, "__headers") {
                Value::Object(b) => b,
                _ => return Ok(Value::Undefined),
            };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str()
                .to_ascii_lowercase();
            let value = abstract_ops::to_string(&args.get(1).cloned().unwrap_or(Value::Undefined))
                .as_str()
                .to_string();
            let existing = rt.object_get(bag, &name);
            let combined = match existing {
                Value::String(s) => format!("{}, {}", s, value),
                _ => value,
            };
            rt.object_set(bag, name, Value::String(Rc::new(combined)));
            Ok(Value::Undefined)
        });
        register_method(self, headers_proto, "delete", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let bag = match rt.object_get(this_id, "__headers") {
                Value::Object(b) => b,
                _ => return Ok(Value::Undefined),
            };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str()
                .to_ascii_lowercase();
            rt.object_set(bag, name, Value::Undefined);
            Ok(Value::Undefined)
        });
        register_method(self, headers_proto, "forEach", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let bag = match rt.object_get(this_id, "__headers") {
                Value::Object(b) => b,
                _ => return Ok(Value::Undefined),
            };
            let cb = args.first().cloned().unwrap_or(Value::Undefined);
            let pairs: Vec<(String, Value)> = rt
                .obj(bag)
                .properties
                .iter()
                .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                .collect();
            for (k, v) in pairs {
                rt.call_function(
                    cb.clone(),
                    Value::Undefined,
                    vec![v, Value::String(Rc::new(k)), Value::Object(this_id)],
                )?;
            }
            Ok(Value::Undefined)
        });
        self.globals
            .insert("Headers".into(), Value::Object(headers_ctor_id));

        // Ω.5.P53.E4: Request ctor populates .headers from opts.headers, plus
        // .url, .method, .body. Empty-instance pre-fix tripped consumers that
        // chained off .headers immediately at module-init (ky's
        // constants.js:12 supportsRequestStreams probe).
        let request_proto = self.alloc_object(Object::new_ordinary());
        let request_proto_for_closure = request_proto;
        let request_ctor_fn = make_native("Request", move |rt, args| {
            let mut inst = Object::new_ordinary();
            inst.proto = Some(request_proto_for_closure);
            let id = rt.alloc_object(inst);
            let url = args
                .first()
                .cloned()
                .unwrap_or(Value::String(Rc::new(String::new())));
            rt.object_set(id, "url".into(), url);
            let opts = args.get(1).cloned().unwrap_or(Value::Undefined);
            let (method, body, headers_init) = if let Value::Object(opts_id) = &opts {
                let m = rt.object_get(*opts_id, "method");
                let b = rt.object_get(*opts_id, "body");
                let h = rt.object_get(*opts_id, "headers");
                (m, b, h)
            } else {
                (Value::Undefined, Value::Undefined, Value::Undefined)
            };
            let method_s = match method {
                Value::String(s) => (*s).clone(),
                _ => "GET".to_string(),
            };
            rt.object_set(id, "method".into(), Value::String(Rc::new(method_s)));
            rt.object_set(id, "body".into(), body);
            // Synthesize a Headers via the global Headers ctor.
            let h_inst = match rt.globals.get("Headers").cloned() {
                Some(Value::Object(_)) => {
                    // Inline: build a fresh Headers, fold headers_init.
                    let mut h_obj = Object::new_ordinary();
                    h_obj.proto = Some(headers_proto_for_closure);
                    let h_id = rt.alloc_object(h_obj);
                    let bag = rt.alloc_object(Object::new_dictionary());
                    rt.object_set(h_id, "__headers".into(), Value::Object(bag));
                    if let Value::Object(src) = headers_init {
                        let pairs: Vec<(String, Value)> = rt
                            .obj(src)
                            .properties
                            .iter()
                            .filter(|(k, d)| d.enumerable && k.as_str() != "__headers")
                            .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                            .collect();
                        for (k, v) in pairs {
                            let lk = k.to_ascii_lowercase();
                            let s = abstract_ops::to_string(&v).as_str().to_string();
                            rt.object_set(bag, lk, Value::String(Rc::new(s)));
                        }
                        if let Value::Object(src_bag) = rt.object_get(src, "__headers") {
                            let inner: Vec<(String, Value)> = rt
                                .obj(src_bag)
                                .properties
                                .iter()
                                .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                                .collect();
                            for (k, v) in inner {
                                rt.object_set(bag, k, v);
                            }
                        }
                    }
                    Value::Object(h_id)
                }
                _ => Value::Undefined,
            };
            rt.object_set(id, "headers".into(), h_inst);
            Ok(Value::Object(id))
        });
        let request_ctor_id = self.alloc_object(request_ctor_fn);
        self.obj_mut(request_ctor_id)
            .set_own_frozen("prototype".into(), Value::Object(request_proto));
        self.obj_mut(request_proto)
            .set_own_internal("constructor".into(), Value::Object(request_ctor_id));
        self.globals
            .insert("Request".into(), Value::Object(request_ctor_id));
        // fetch() as a callable global that returns a rejected-Promise-shaped
        // value (host-v2 lacks real Promise scheduling for fetch; the call
        // surface exists for module-init read-shape probes).
        let fetch_obj = make_native("fetch", |_rt, _args| {
            Err(RuntimeError::TypeError(
                "fetch not yet implemented (Tier-Ω.5.P49.E3 stub)".into(),
            ))
        });
        let fetch_id = self.alloc_object(fetch_obj);
        self.globals.insert("fetch".into(), Value::Object(fetch_id));

        // Tier-Ω.5.ll: BigInt as callable global. zod uses `BigInt(x)`.
        // Tier-Ω.5.CCCCCCCC: backed by real JsBigInt arithmetic substrate.
        let bi_obj = make_native("BigInt", |rt, args| {
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            crate::abstract_ops::to_bigint(rt, &v)
        });
        let bi_id = self.alloc_object(bi_obj);
        // EXT 78: BigInt.asIntN / asUintN dispatch ToBigInt on their second
        // argument per §21.2.2.1 / §21.2.2.2 step 2. v1's clamp/mask
        // shape is a passthrough (deferred), but the coercion + error
        // propagation now match spec.
        register_intrinsic_method(self, bi_id, "asIntN", 2, |rt, args| {
            let v = args.get(1).cloned().unwrap_or(Value::Undefined);
            crate::abstract_ops::to_bigint(rt, &v)
        });
        register_intrinsic_method(self, bi_id, "asUintN", 2, |rt, args| {
            let v = args.get(1).cloned().unwrap_or(Value::Undefined);
            crate::abstract_ops::to_bigint(rt, &v)
        });
        // Tier-Ω.5.oooooo: BigInt.prototype with valueOf + toString. unbox-
        // primitive / is-bigint reach for `BigInt.prototype.valueOf`.
        let bi_proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, bi_proto, "valueOf", 0, |rt, _args| {
            // EXT 83: ThisBigIntValue per §21.2.3 — unwraps a BigInt
            // wrapper object via its [[BigIntData]] internal slot in
            // addition to the bare BigInt case.
            match rt.current_this() {
                Value::BigInt(b) => Ok(Value::BigInt(b)),
                Value::Object(id) => {
                    if let crate::value::InternalKind::BigIntWrapper(v) = &rt.obj(id).internal_kind
                    {
                        return Ok(v.clone());
                    }
                    Err(RuntimeError::TypeError(
                        "BigInt.prototype.valueOf: this is not a BigInt".into(),
                    ))
                }
                _ => Err(RuntimeError::TypeError(
                    "BigInt.prototype.valueOf: this is not a BigInt".into(),
                )),
            }
        });
        register_intrinsic_method(self, bi_proto, "toString", 0, |rt, args| {
            crate::generated::bigint_prototype_to_string(rt, rt.current_this(), args)
        });
        self.obj_mut(bi_id)
            .set_own_frozen("prototype".into(), Value::Object(bi_proto));
        self.bigint_prototype = Some(bi_proto);
        self.globals.insert("BigInt".into(), Value::Object(bi_id));
        // Boolean ctor with prototype.valueOf.
        let bool_obj = make_native("Boolean", |rt, args| {
            // Ω.5.P62.E1: `new Boolean(v)` per ECMA §20.3.1 produces a
            // Boolean-exotic object with [[BooleanData]]. Modeled via
            // non-enumerable __primitive__ slot.
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            let b = crate::abstract_ops::to_boolean(&v);
            if rt.current_new_target.is_some() {
                let mut obj = crate::value::Object::new_ordinary();
                obj.set_own_internal("__primitive__".into(), Value::Boolean(b));
                // EXT 83: tag [[BooleanData]] for Object.prototype.toString brand.
                obj.internal_kind = crate::value::InternalKind::BooleanWrapper(Value::Boolean(b));
                let proto = match rt.globals.get("Boolean").cloned() {
                    Some(Value::Object(id)) => match rt.object_get(id, "prototype") {
                        Value::Object(p) => Some(p),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(p) = proto {
                    obj.proto = Some(p);
                }
                let id = rt.alloc_object(obj);
                return Ok(Value::Object(id));
            }
            Ok(Value::Boolean(b))
        });
        let bool_id = self.alloc_object(bool_obj);
        let bool_proto = self.alloc_object(Object::new_ordinary());
        // Ω.5.P63.E19: Boolean.prototype.{valueOf, toString} routed through IR.
        register_intrinsic_method(self, bool_proto, "valueOf", 0, |rt, _args| {
            let this = rt.current_this();
            crate::generated::boolean_prototype_value_of(rt, this, &[])
        });
        register_intrinsic_method(self, bool_proto, "toString", 0, |rt, _args| {
            let this = rt.current_this();
            crate::generated::boolean_prototype_to_string(rt, this, &[])
        });
        self.obj_mut(bool_id)
            .set_own_frozen("prototype".into(), Value::Object(bool_proto));
        // Ω.5.P58.E4: Boolean.prototype.constructor = Boolean per ECMA §10.2.12.
        self.obj_mut(bool_proto)
            .set_own_internal("constructor".into(), Value::Object(bool_id));
        // Ω.5.P62.E19: Boolean.prototype is a Boolean exotic with
        // [[BooleanData]] = false per §20.3.4.
        self.obj_mut(bool_proto)
            .set_own_internal("__primitive__".into(), Value::Boolean(false));
        self.globals
            .insert("Boolean".into(), Value::Object(bool_id));
        // Tier-Ω.5.tttttt: EventTarget + Event + CustomEvent global stubs
        // (chai / web-platform-ish libs). v1: ordinary objects with the
        // standard surface; no actual dispatch.
        let et = make_native("EventTarget", |rt, _args| {
            let mut o = Object::new_ordinary();
            o.set_own_internal(
                "__listeners__".into(),
                Value::Object(rt.alloc_object(Object::new_ordinary())),
            );
            Ok(Value::Object(rt.alloc_object(o)))
        });
        let et_id = self.alloc_object(et);
        let et_proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, et_proto, "addEventListener", 1, |rt, _args| {
            let _ = rt;
            Ok(Value::Undefined)
        });
        register_intrinsic_method(self, et_proto, "removeEventListener", 1, |rt, _args| {
            let _ = rt;
            Ok(Value::Undefined)
        });
        register_intrinsic_method(self, et_proto, "dispatchEvent", 1, |_rt, _args| {
            Ok(Value::Boolean(false))
        });
        self.obj_mut(et_id)
            .set_own_frozen("prototype".into(), Value::Object(et_proto));
        self.globals
            .insert("EventTarget".into(), Value::Object(et_id));
        let ev = make_native("Event", |rt, args| {
            let mut o = Object::new_ordinary();
            let ty = match args.first() {
                Some(Value::String(s)) => (**s).clone(),
                _ => String::new(),
            };
            o.set_own("type".into(), Value::String(Rc::new(ty)));
            o.set_own("bubbles".into(), Value::Boolean(false));
            o.set_own("cancelable".into(), Value::Boolean(false));
            o.set_own("defaultPrevented".into(), Value::Boolean(false));
            Ok(Value::Object(rt.alloc_object(o)))
        });
        let ev_id = self.alloc_object(ev);
        let ev_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(ev_id)
            .set_own_frozen("prototype".into(), Value::Object(ev_proto));
        self.globals.insert("Event".into(), Value::Object(ev_id));
        let ce = make_native("CustomEvent", |rt, args| {
            let mut o = Object::new_ordinary();
            let ty = match args.first() {
                Some(Value::String(s)) => (**s).clone(),
                _ => String::new(),
            };
            o.set_own("type".into(), Value::String(Rc::new(ty)));
            let detail = match args.get(1) {
                Some(Value::Object(id)) => rt.object_get(*id, "detail"),
                _ => Value::Undefined,
            };
            o.set_own("detail".into(), detail);
            Ok(Value::Object(rt.alloc_object(o)))
        });
        let ce_id = self.alloc_object(ce);
        let ce_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(ce_id)
            .set_own_frozen("prototype".into(), Value::Object(ce_proto));
        self.globals
            .insert("CustomEvent".into(), Value::Object(ce_id));
        // Ω.5.P58.E8: MessageEvent, ErrorEvent, CloseEvent, ProgressEvent,
        // BeforeUnloadEvent stubs. @mswjs/data does
        // `class X extends MessageEvent` at module-init; many web-ish
        // consumers extend Event subclasses. Each is a callable that
        // returns an ordinary object; .prototype set so class-extends
        // can read it.
        // BroadcastChannel stub: same pattern but exposes .postMessage,
        // .close, .onmessage stubs since consumers may attach handlers
        // at module-init (msw / @mswjs/data instance pattern).
        let bc = make_native("BroadcastChannel", |rt, args| {
            let mut o = Object::new_ordinary();
            let name = match args.first() {
                Some(Value::String(s)) => (**s).clone(),
                _ => String::new(),
            };
            o.set_own("name".into(), Value::String(Rc::new(name)));
            let id = rt.alloc_object(o);
            // Install no-op methods on the instance.
            let postm = make_native("postMessage", |_rt, _a| Ok(Value::Undefined));
            let postm_id = rt.alloc_object(postm);
            rt.object_set(id, "postMessage".into(), Value::Object(postm_id));
            let close = make_native("close", |_rt, _a| Ok(Value::Undefined));
            let close_id = rt.alloc_object(close);
            rt.object_set(id, "close".into(), Value::Object(close_id));
            let addel = make_native("addEventListener", |_rt, _a| Ok(Value::Undefined));
            let addel_id = rt.alloc_object(addel);
            rt.object_set(id, "addEventListener".into(), Value::Object(addel_id));
            Ok(Value::Object(id))
        });
        let bc_id = self.alloc_object(bc);
        let bc_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(bc_id)
            .set_own_frozen("prototype".into(), Value::Object(bc_proto));
        self.obj_mut(bc_proto)
            .set_own_internal("constructor".into(), Value::Object(bc_id));
        self.globals
            .insert("BroadcastChannel".into(), Value::Object(bc_id));
        for name in &[
            "MessageEvent",
            "ErrorEvent",
            "CloseEvent",
            "ProgressEvent",
            "BeforeUnloadEvent",
            "FocusEvent",
        ] {
            let ctor_name = *name;
            let nm = make_native(ctor_name, move |rt, args| {
                let mut o = Object::new_ordinary();
                let ty = match args.first() {
                    Some(Value::String(s)) => (**s).clone(),
                    _ => String::new(),
                };
                o.set_own("type".into(), Value::String(Rc::new(ty)));
                if let Some(Value::Object(init_id)) = args.get(1) {
                    let data = rt.object_get(*init_id, "data");
                    o.set_own("data".into(), data);
                }
                Ok(Value::Object(rt.alloc_object(o)))
            });
            let nm_id = self.alloc_object(nm);
            let nm_proto = self.alloc_object(Object::new_ordinary());
            self.obj_mut(nm_id)
                .set_own_frozen("prototype".into(), Value::Object(nm_proto));
            self.obj_mut(nm_proto)
                .set_own_internal("constructor".into(), Value::Object(nm_id));
            self.globals.insert((*name).into(), Value::Object(nm_id));
        }
        self.install_error_globals();
        self.install_reflect();
        self.install_map_set_globals();
        self.install_date_global();
        self.install_typed_array_stubs();
        self.install_weak_ref_globals();
        self.install_proxy();
    }

    /// Ω.5.P60.E1: Proxy(target, handler) per ECMA-262 §28.2 + §10.5.
    /// Creates a Proxy exotic object that delegates property access through
    /// the handler's traps (get/set/has/deleteProperty/ownKeys/...) when
    /// present; missing-trap path delegates to the target.
    ///
    /// v1 implementation scope: Op::GetProp / Op::GetIndex consult the
    /// handler's `get` trap if present. Other traps (set/has/deleteProperty/
    /// apply/construct/ownKeys/getOwnPropertyDescriptor/getPrototypeOf/
    /// setPrototypeOf/isExtensible/preventExtensions/defineProperty) are
    /// not yet dispatched — those reads fall through to the target. The
    /// `get` trap is the load-bearing path for module-init parity (lazy
    /// property loading, defineLazy patterns, ESM-namespace proxies).
    fn install_proxy(&mut self) {
        let proxy_obj = make_native("Proxy", |rt, args| {
            let target = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Proxy: target must be an object".into(),
                    ))
                }
            };
            let handler = match args.get(1) {
                Some(Value::Object(id)) => *id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Proxy: handler must be an object".into(),
                    ))
                }
            };
            let mut o = Object::new_ordinary();
            o.internal_kind = InternalKind::Proxy(crate::value::ProxyInternals {
                revoked: false,
                target,
                handler,
            });
            // Proxy's [[Prototype]] is the target's prototype so that
            // `instanceof` and prototype-chain walks see the same chain.
            o.proto = rt.obj(target).proto;
            Ok(Value::Object(rt.alloc_object(o)))
        });
        let pid = self.alloc_object(proxy_obj);
        // Proxy.revocable(target, handler) — for revocable proxies.
        register_intrinsic_method(self, pid, "revocable", 1, |rt, args| {
            let target = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Proxy.revocable: target must be an object".into(),
                    ))
                }
            };
            let handler = match args.get(1) {
                Some(Value::Object(id)) => *id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Proxy.revocable: handler must be an object".into(),
                    ))
                }
            };
            let mut o = Object::new_ordinary();
            o.internal_kind = InternalKind::Proxy(crate::value::ProxyInternals {
                revoked: false,
                target,
                handler,
            });
            o.proto = rt.obj(target).proto;
            let proxy_id = rt.alloc_object(o);
            let mut result = Object::new_ordinary();
            result.set_own("proxy".into(), Value::Object(proxy_id));
            // EXT 84: revoke closure captures proxy_id and flips the
            // ProxyInternals.revoked flag on first call. Subsequent
            // operations on the proxy throw TypeError per spec.
            let revoke = make_native("revoke", move |rt, _args| {
                if let crate::value::InternalKind::Proxy(p) =
                    &mut rt.obj_mut(proxy_id).internal_kind
                {
                    p.revoked = true;
                }
                Ok(Value::Undefined)
            });
            let revoke_id = rt.alloc_object(revoke);
            result.set_own("revoke".into(), Value::Object(revoke_id));
            Ok(Value::Object(rt.alloc_object(result)))
        });
        self.globals.insert("Proxy".into(), Value::Object(pid));
    }

    /// Tier-Ω.5.dd: Map / Set / WeakMap / WeakSet as real implementations.
    /// Storage uses the underlying Object's properties map for v1 — keys
    /// are stringified via ToString. This is a v1 deviation: real Map keys
    /// are by SameValueZero, so object keys would each be distinct identity-
    /// wise. Our string-keyed storage collides object keys via their
    /// stringified form. Most parity packages don't depend on object-keyed
    /// Maps; documented for future substrate.
    fn install_map_set_globals(&mut self) {
        for collection in &["Map", "WeakMap"] {
            let proto = self.alloc_object(Object::new_ordinary());
            let is_weak_proto = *collection == "WeakMap";
            // SPBC-EXT 3 / MPBC sibling: brand-check wrappers per registered
            // proto. Map.prototype.{get,set,has,delete} must reject WeakMap
            // receivers (cross-proto call); WeakMap.prototype.{get,set,has,
            // delete} must reject non-WeakMap receivers per spec
            // RequireInternalSlot([[MapData]] / [[WeakMapData]]).
            let brand_chk = move |rt: &mut Runtime, method: &str| -> Result<(), RuntimeError> {
                let this_v = rt.current_this();
                let this_id = match &this_v {
                    Value::Object(id) => *id,
                    _ => return Ok(()),
                };
                let receiver_is_weak =
                    matches!(rt.object_get(this_id, "__is_weakmap"), Value::Boolean(true));
                if is_weak_proto && !receiver_is_weak {
                    return Err(RuntimeError::TypeError(format!(
                        "WeakMap.prototype.{}: this is not a WeakMap",
                        method
                    )));
                }
                if !is_weak_proto && receiver_is_weak {
                    return Err(RuntimeError::TypeError(format!(
                        "Map.prototype.{}: this is a WeakMap, not a Map",
                        method
                    )));
                }
                Ok(())
            };
            // §24.1.3 / §24.3.3 spec arities (.length values).
            register_intrinsic_method(self, proto, "get", 1, move |rt, args| {
                brand_chk(rt, "get")?;
                crate::generated::map_prototype_get(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "set", 2, move |rt, args| {
                brand_chk(rt, "set")?;
                crate::generated::map_prototype_set(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "has", 1, move |rt, args| {
                brand_chk(rt, "has")?;
                crate::generated::map_prototype_has(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "delete", 1, move |rt, args| {
                brand_chk(rt, "delete")?;
                crate::generated::map_prototype_delete(rt, rt.current_this(), args)
            });
            // MGOI-EXT 1: Map.prototype.getOrInsert / getOrInsertComputed
            // per TC39 upsert proposal. Same brand-check discipline as
            // the basic methods. Registered on Map proto only (WeakMap
            // gets its own variant when needed; WeakMap version handles
            // weak-ref keys differently).
            // MGOI / WMGOI: TC39 upsert proposal — getOrInsert + getOrInsertComputed
            // exist on both Map.prototype and WeakMap.prototype. The underlying
            // impl (map_proto_get_or_insert_via) works on the shared __map_data
            // storage regardless of __is_weakmap flag; brand_chk in the install
            // loop is per-proto so the WeakMap installation rejects Map receivers
            // and vice versa.
            register_intrinsic_method(self, proto, "getOrInsert", 2, move |rt, args| {
                brand_chk(rt, "getOrInsert")?;
                rt.map_proto_get_or_insert_via(args)
            });
            register_intrinsic_method(self, proto, "getOrInsertComputed", 2, move |rt, args| {
                brand_chk(rt, "getOrInsertComputed")?;
                rt.map_proto_get_or_insert_computed_via(args)
            });
            // ECMA-262 sec 24.1.3.10 get Map.prototype.size: accessor on
            // the prototype, not a data property on instances. Pre-fix
            // cruftless stored a per-instance data 'size' that shadowed
            // the (missing) accessor. Install the accessor here so
            // Object.getOwnPropertyDescriptor(Map.prototype, 'size')
            // returns the accessor descriptor with .get/.set per spec.
            // The instance data property is preserved for compatibility
            // with internal incrementers; instance lookup of m.size
            // finds the own data property first, so the existing
            // increment/decrement code keeps working unchanged.
            if !is_weak_proto {
                let size_getter = make_native("get size", |rt, _args| {
                    let this = match rt.current_this() {
                        Value::Object(id) => id,
                        _ => {
                            return Err(RuntimeError::TypeError(
                                "Map.prototype.size: this is not a Map".into(),
                            ))
                        }
                    };
                    // If the instance carries an own 'size' data property
                    // (initialized by the Map constructor), return it.
                    // Otherwise compute from __map_data storage.
                    if let Some(d) = rt.obj(this).get_own("size") {
                        return Ok(d.value.clone());
                    }
                    match rt.object_get(this, "__map_data") {
                        Value::Object(storage) => {
                            let n = rt.obj(storage).properties.len();
                            Ok(Value::Number(n as f64))
                        }
                        _ => Err(RuntimeError::TypeError(
                            "Map.prototype.size: this is not a Map (no __map_data)".into(),
                        )),
                    }
                });
                let size_getter_id = self.alloc_object(size_getter);
                let size_desc = crate::value::PropertyDescriptor {
                    value: Value::Undefined,
                    writable: false,
                    enumerable: false,
                    configurable: true,
                    getter: Some(Value::Object(size_getter_id)),
                    setter: None,
                };
                self.obj_mut(proto)
                    .dict_mut()
                    .insert(crate::value::PropertyKey::String("size".into()), size_desc);
            }
            // EXT 81: per ECMA §24.3.3, WeakMap.prototype has only
            // {get, set, has, delete} — not clear / forEach / entries /
            // keys / values / @@iterator. The Map-only methods below are
            // skipped on the WeakMap proto so tests that call
            // Map.prototype.clear.call(wm) hit the __is_weakmap brand
            // check in map_this_and_storage and throw TypeError.
            if !is_weak_proto {
                register_intrinsic_method(self, proto, "clear", 0, |rt, args| {
                    crate::generated::map_prototype_clear(rt, rt.current_this(), args)
                });
                register_intrinsic_method(self, proto, "forEach", 1, |rt, args| {
                    crate::generated::map_prototype_for_each(rt, rt.current_this(), args)
                });
                // Tier-Ω.5.KKKKKKK: Map.prototype.values / keys / entries per ECMA
                // §24.1.3.3 / .4 / .5. Returns an array (eager-collect — full
                // iterator-protocol support is queued downstream). wrap-ansi /
                // log-update / mime / many spread the map's values into a Set
                // via `new Set(m.values())` which exercises Symbol.iterator on
                // the returned object; an Array satisfies both the iterator
                // (via @@iterator on Array.prototype) and the spread protocol.
                register_intrinsic_method(self, proto, "values", 0, |rt, args| {
                    crate::generated::map_prototype_values(rt, rt.current_this(), args)
                });
                register_intrinsic_method(self, proto, "keys", 0, |rt, args| {
                    crate::generated::map_prototype_keys(rt, rt.current_this(), args)
                });
                register_intrinsic_method(self, proto, "entries", 0, |rt, args| {
                    crate::generated::map_prototype_entries(rt, rt.current_this(), args)
                });
                // Tier-Ω.5.MMMMMMM: Map.prototype[@@iterator] aliases entries
                // per ECMA §24.1.3.12. Surfaced by Step-6 route-(b) escalation:
                // adding receiver-shape tags to the CallMethod undef-fault
                // surfaced 'receiver=Object keys=[__map_data,size]' on the
                // cli-truncate/fast-xml-parser/log-update cluster, naming Map
                // as the iterated receiver. for-of and spread reach for
                // [Symbol.iterator], which on Map is Map.prototype.entries.
                register_intrinsic_method(self, proto, "@@iterator", 1, |rt, _args| {
                    let this = match rt.current_this() {
                        Value::Object(id) => id,
                        _ => {
                            return Err(RuntimeError::TypeError(
                                "Map.prototype method: this is not a Map object".into(),
                            ))
                        }
                    };
                    let storage = match rt.object_get(this, "__map_data") {
                        Value::Object(id) => id,
                        _ => return Ok(Value::Object(rt.alloc_object(Object::new_array()))),
                    };
                    // Same key-decoding discipline as map_proto_entries_via:
                    // route storage-key strings through the __map_orig_keys
                    // side channel so non-string original keys (Number, Object,
                    // Symbol, Boolean, null, undefined) round-trip as their
                    // original type. Without this, `for (const [k,v] of map)`
                    // yielded string keys for what was set via Number.
                    let pairs: Vec<(String, Value)> = rt
                        .obj(storage)
                        .properties
                        .iter()
                        .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                        .collect();
                    let arr = rt.alloc_object(Object::new_array());
                    for (i, (k, v)) in pairs.into_iter().enumerate() {
                        let key_v = {
                            // Inline of map_decode_key — that helper is private
                            // to interp::Runtime; replicate the lookup here.
                            let orig = rt.object_get(this, "__map_orig_keys");
                            if let Value::Object(orig_id) = orig {
                                let candidate = rt.object_get(orig_id, &k);
                                if !matches!(candidate, Value::Undefined) {
                                    candidate
                                } else {
                                    Value::String(Rc::new(k.clone()))
                                }
                            } else {
                                Value::String(Rc::new(k.clone()))
                            }
                        };
                        let pair = rt.alloc_object(Object::new_array());
                        rt.object_set(pair, "0".into(), key_v);
                        rt.object_set(pair, "1".into(), v);
                        rt.object_set(pair, "length".into(), Value::Number(2.0));
                        rt.object_set(arr, i.to_string(), Value::Object(pair));
                    }
                    let len = rt.array_length(arr);
                    rt.object_set(arr, "length".into(), Value::Number(len as f64));
                    Ok(Value::Object(crate::iterator::make_array_iterator(rt, arr)))
                });
            } // end !is_weak_proto guard for Map-only methods
            let proto_for_ctor = proto;
            let name = (*collection).to_string();
            // EXT 81: mark WeakMap instances with __is_weakmap=true so
            // Map.prototype.* brand checks (map_this_and_storage) can
            // reject them with TypeError per §24.1.3 [[MapData]] check.
            // Real Map/WeakMap discrimination would need separate proto
            // chains; v1 ships shared methods + a marker.
            let is_weak = name == "WeakMap";
            let ctor_obj = make_native(&name, move |rt, args| {
                let mut o = Object::new_ordinary();
                o.proto = Some(proto_for_ctor);
                let id = rt.alloc_object(o);
                let storage = rt.alloc_object(Object::new_dictionary());
                // ESNE-EXT 1: install engine sentinels non-enumerable.
                // ESNE-EXT 2: size on Map/WeakMap installed hidden too;
                // subsequent updates via object_set preserve attrs.
                rt.set_engine_sentinel(id, "__map_data", Value::Object(storage));
                rt.set_engine_sentinel(id, "size", Value::Number(0.0));
                if is_weak {
                    rt.set_engine_sentinel(id, "__is_weakmap", Value::Boolean(true));
                }
                // Tier-Ω.5.LLLLLLL: iterable-arg processing per ECMA §24.1.1.1.
                // `new Map(iterable)` iterates each entry (array-like with [k,v])
                // and inserts. Common patterns: new Map([['a',1]]), new Map(other),
                // new Map(otherArray.map(x => [x.key, x.value])).
                // Eager-collect: if arg is array-shape, walk indices 0..length;
                // for each entry that's also array-shape, read [0] and [1] as
                // (key, value) and store. Real iterator-protocol with next()/done
                // is deferred — array-shape covers the dense majority.
                if let Some(init) = args.first().cloned() {
                    if let Value::Object(arr_id) = init {
                        // `new Map(otherMap)` — if the arg is itself a Map
                        // instance, iterate its __map_data storage rather
                        // than treating it as a length-keyed array (which
                        // returns zero entries since Map has no length).
                        let src_storage = match rt.object_get(arr_id, "__map_data") {
                            Value::Object(sid) => Some(sid),
                            _ => None,
                        };
                        if let Some(sid) = src_storage {
                            let pairs: Vec<(String, Value)> = rt
                                .obj(sid)
                                .properties
                                .iter()
                                .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                                .collect();
                            for (k, v) in pairs {
                                rt.object_set(storage, k, v);
                            }
                        } else {
                            let len = rt.array_length(arr_id);
                            for i in 0..len {
                                let entry = rt.object_get(arr_id, &i.to_string());
                                if let Value::Object(eid) = entry {
                                    let k = rt.object_get(eid, "0");
                                    let v = rt.object_get(eid, "1");
                                    let key_s = abstract_ops::to_string(&k).as_str().to_string();
                                    rt.object_set(storage, key_s, v);
                                }
                            }
                        }
                        let cnt = rt.obj(storage).properties.len() as f64;
                        rt.object_set(id, "size".into(), Value::Number(cnt));
                    }
                }
                Ok(Value::Object(id))
            });
            let ctor = self.alloc_object(ctor_obj);
            self.obj_mut(ctor)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(ctor));
            self.globals
                .insert((*collection).to_string(), Value::Object(ctor));
        }
        for collection in &["Set", "WeakSet"] {
            let proto = self.alloc_object(Object::new_ordinary());
            let is_weak_proto = *collection == "WeakSet";
            // SPBC-EXT 3 / MPBC sibling: per-proto brand check at the
            // registration closure. Set.prototype.add must reject WeakSet
            // receivers (cross-proto call) and vice versa. Cruft's
            // set_proto_add_via is shared between both protos, so the
            // brand discrimination must live at registration.
            let set_brand_chk = move |rt: &mut Runtime, method: &str| -> Result<(), RuntimeError> {
                let this_v = rt.current_this();
                let this_id = match &this_v {
                    Value::Object(id) => *id,
                    _ => return Ok(()),
                };
                let receiver_is_weak =
                    matches!(rt.object_get(this_id, "__is_weakset"), Value::Boolean(true));
                if is_weak_proto && !receiver_is_weak {
                    return Err(RuntimeError::TypeError(format!(
                        "WeakSet.prototype.{}: this is not a WeakSet",
                        method
                    )));
                }
                if !is_weak_proto && receiver_is_weak {
                    return Err(RuntimeError::TypeError(format!(
                        "Set.prototype.{}: this is a WeakSet (does not have [[SetData]])",
                        method
                    )));
                }
                Ok(())
            };
            // §24.2.3 spec arities.
            // SPBC-EXT 4: WeakSet.prototype.add additionally checks the
            // value arg can be held weakly (Object or Symbol per
            // CanBeHeldWeakly). Primitives (string/number/bigint/bool/
            // null/undef) throw TypeError per 24.4.3.1 step 4.
            register_intrinsic_method(self, proto, "add", 1, move |rt, args| {
                set_brand_chk(rt, "add")?;
                if is_weak_proto {
                    let v = args.first().cloned().unwrap_or(Value::Undefined);
                    if !matches!(v, Value::Object(_) | Value::Symbol(_)) {
                        return Err(RuntimeError::TypeError(
                            "WeakSet.prototype.add: value cannot be held weakly".into(),
                        ));
                    }
                }
                crate::generated::set_prototype_add(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "has", 1, move |rt, args| {
                set_brand_chk(rt, "has")?;
                crate::generated::set_prototype_has(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "delete", 1, move |rt, args| {
                set_brand_chk(rt, "delete")?;
                crate::generated::set_prototype_delete(rt, rt.current_this(), args)
            });
            // ECMA-262 sec 24.2.3.10 get Set.prototype.size: accessor on
            // the prototype, parallel to Map.prototype.size.
            if !is_weak_proto {
                let size_getter = make_native("get size", |rt, _args| {
                    let this = match rt.current_this() {
                        Value::Object(id) => id,
                        _ => {
                            return Err(RuntimeError::TypeError(
                                "Set.prototype.size: this is not a Set".into(),
                            ))
                        }
                    };
                    if let Some(d) = rt.obj(this).get_own("size") {
                        return Ok(d.value.clone());
                    }
                    match rt.object_get(this, "__set_data") {
                        Value::Object(storage) => {
                            let n = rt.obj(storage).properties.len();
                            Ok(Value::Number(n as f64))
                        }
                        _ => Err(RuntimeError::TypeError(
                            "Set.prototype.size: this is not a Set (no __set_data)".into(),
                        )),
                    }
                });
                let size_getter_id = self.alloc_object(size_getter);
                let size_desc = crate::value::PropertyDescriptor {
                    value: Value::Undefined,
                    writable: false,
                    enumerable: false,
                    configurable: true,
                    getter: Some(Value::Object(size_getter_id)),
                    setter: None,
                };
                self.obj_mut(proto)
                    .dict_mut()
                    .insert(crate::value::PropertyKey::String("size".into()), size_desc);
            }
            // SPBC-EXT 4: wrap clear/forEach with set_brand_chk too (per
            // sweep regression where Set.prototype.clear.call(weakset)
            // and Set.prototype.forEach.call(weakset) failed to throw).
            register_intrinsic_method(self, proto, "clear", 0, move |rt, args| {
                set_brand_chk(rt, "clear")?;
                crate::generated::set_prototype_clear(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "forEach", 1, move |rt, args| {
                set_brand_chk(rt, "forEach")?;
                crate::generated::set_prototype_for_each(rt, rt.current_this(), args)
            });
            // Tier-Ω.5.rrr: @@iterator returns a values-iterator. Per
            // spec Set.prototype[Symbol.iterator] === Set.prototype.values.
            // Required for `[...new Set(arr)]` to spread.
            register_intrinsic_method(self, proto, "@@iterator", 1, |rt, _args| {
                let this = match rt.current_this() {
                    Value::Object(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Set.prototype method: this is not a Set object".into(),
                        ))
                    }
                };
                make_set_values_iterator(rt, this)
            });
            register_intrinsic_method(self, proto, "values", 1, |rt, _args| {
                let this = match rt.current_this() {
                    Value::Object(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Set.prototype method: this is not a Set object".into(),
                        ))
                    }
                };
                make_set_values_iterator(rt, this)
            });
            // Ω.5.P61.E11: Set.prototype.keys is alias for values per ECMA §24.2.4.
            register_intrinsic_method(self, proto, "keys", 0, |rt, _args| {
                let this = match rt.current_this() {
                    Value::Object(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Set.prototype method: this is not a Set object".into(),
                        ))
                    }
                };
                make_set_values_iterator(rt, this)
            });
            // Set.prototype.entries returns iterator of [v, v] pairs.
            register_intrinsic_method(self, proto, "entries", 0, |rt, _args| {
                let this = match rt.current_this() {
                    Value::Object(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Set.prototype method: this is not a Set object".into(),
                        ))
                    }
                };
                let storage = match rt.object_get(this, "__set_data") {
                    Value::Object(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Set.prototype method: this is not a Set object".into(),
                        ))
                    }
                };
                let vals: Vec<Value> = rt
                    .obj(storage)
                    .properties
                    .values()
                    .map(|d| d.value.clone())
                    .collect();
                let arr = rt.alloc_object(Object::new_array());
                for (i, v) in vals.iter().enumerate() {
                    let pair = rt.alloc_object(Object::new_array());
                    rt.object_set(pair, "0".into(), v.clone());
                    rt.object_set(pair, "1".into(), v.clone());
                    rt.object_set(pair, "length".into(), Value::Number(2.0));
                    rt.object_set(arr, i.to_string(), Value::Object(pair));
                }
                rt.object_set(arr, "length".into(), Value::Number(vals.len() as f64));
                // Return an iterator over the pairs.
                Ok(Value::Object(crate::iterator::make_array_iterator(rt, arr)))
            });
            register_intrinsic_method(self, proto, "union", 1, |rt, args| {
                crate::generated::set_prototype_union(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "intersection", 1, |rt, args| {
                crate::generated::set_prototype_intersection(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "difference", 1, |rt, args| {
                crate::generated::set_prototype_difference(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "symmetricDifference", 1, |rt, args| {
                crate::generated::set_prototype_symmetric_difference(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "isSubsetOf", 1, |rt, args| {
                crate::generated::set_prototype_is_subset_of(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "isSupersetOf", 1, |rt, args| {
                crate::generated::set_prototype_is_superset_of(rt, rt.current_this(), args)
            });
            register_intrinsic_method(self, proto, "isDisjointFrom", 1, |rt, args| {
                crate::generated::set_prototype_is_disjoint_from(rt, rt.current_this(), args)
            });
            // (legacy hand-written set-op implementations removed; all routed through IR above.)
            let proto_for_ctor = proto;
            let name = (*collection).to_string();
            let is_weak_ctor = is_weak_proto;
            let ctor_obj = make_native(&name, move |rt, args| {
                let mut o = Object::new_ordinary();
                o.proto = Some(proto_for_ctor);
                let id = rt.alloc_object(o);
                let storage = rt.alloc_object(Object::new_dictionary());
                // ESNE-EXT 1: install engine sentinels non-enumerable.
                // ESNE-EXT 2: size on Set/WeakSet installed hidden too.
                rt.set_engine_sentinel(id, "__set_data", Value::Object(storage));
                // SPBC-EXT 2: brand mark for WeakSet (parallel to __is_weakmap).
                if is_weak_ctor {
                    rt.set_engine_sentinel(id, "__is_weakset", Value::Boolean(true));
                }
                rt.set_engine_sentinel(id, "size", Value::Number(0.0));
                // Tier-Ω.5.rrr: populate from iterable arg. Per spec
                // `new Set(iterable)` calls .add for each yielded value.
                if let Some(arg) = args.first() {
                    if let Ok(values) = collect_iterable(rt, arg.clone()) {
                        let mut size = 0.0_f64;
                        for v in values {
                            let key_s = abstract_ops::to_string(&v).as_str().to_string();
                            if matches!(rt.object_get(storage, &key_s), Value::Undefined) {
                                rt.object_set(storage, key_s, v);
                                size += 1.0;
                            }
                        }
                        rt.object_set(id, "size".into(), Value::Number(size));
                    }
                }
                Ok(Value::Object(id))
            });
            let ctor = self.alloc_object(ctor_obj);
            self.obj_mut(ctor)
                .set_own_frozen("prototype".into(), Value::Object(proto));
            self.obj_mut(proto)
                .set_own_internal("constructor".into(), Value::Object(ctor));
            self.globals
                .insert((*collection).to_string(), Value::Object(ctor));
        }
    }

    /// Tier-Ω.5.aaaa: Date global. Real Gregorian arithmetic for year/
    /// month/day extraction; ISO-string parsing in the constructor;
    /// per-spec getter methods.
    fn install_date_global(&mut self) {
        let proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, proto, "getTime", 1, |rt, args| {
            crate::generated::date_prototype_get_time(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "valueOf", 0, |rt, args| {
            crate::generated::date_prototype_value_of(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getFullYear", 1, |rt, args| {
            crate::generated::date_prototype_get_full_year(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getMonth", 1, |rt, args| {
            crate::generated::date_prototype_get_month(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getDate", 1, |rt, args| {
            crate::generated::date_prototype_get_date(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getDay", 1, |rt, args| {
            crate::generated::date_prototype_get_day(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getHours", 1, |rt, args| {
            crate::generated::date_prototype_get_hours(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getMinutes", 1, |rt, args| {
            crate::generated::date_prototype_get_minutes(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getSeconds", 1, |rt, args| {
            crate::generated::date_prototype_get_seconds(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getMilliseconds", 1, |rt, args| {
            crate::generated::date_prototype_get_milliseconds(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getTimezoneOffset", 1, |rt, args| {
            crate::generated::date_prototype_get_timezone_offset(rt, rt.current_this(), args)
        });
        // Tier-Ω.5.P31.E1.date-utc-getters-setters: getUTC* mirror the
        // non-UTC getters (we treat __date_ms as UTC throughout — no
        // local-time conversion). setUTC* mutate the date by replacing
        // the corresponding component. Surfaced by Ω.5.P24.E1 probe
        // walking temporal-polyfill (whose `setUTCHours` call landed on
        // a fake-Date-shaped object with no Date.prototype in its chain).
        // E42: UTC getters route to the same IR helpers as the non-UTC variants
        // (cruftless treats __date_ms as UTC throughout, so the values are identical).
        register_intrinsic_method(self, proto, "getUTCFullYear", 1, |rt, args| {
            crate::generated::date_prototype_get_full_year(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getUTCMonth", 1, |rt, args| {
            crate::generated::date_prototype_get_month(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getUTCDate", 1, |rt, args| {
            crate::generated::date_prototype_get_date(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getUTCDay", 1, |rt, args| {
            crate::generated::date_prototype_get_day(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getUTCHours", 1, |rt, args| {
            crate::generated::date_prototype_get_hours(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getUTCMinutes", 1, |rt, args| {
            crate::generated::date_prototype_get_minutes(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getUTCSeconds", 1, |rt, args| {
            crate::generated::date_prototype_get_seconds(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "getUTCMilliseconds", 1, |rt, args| {
            crate::generated::date_prototype_get_milliseconds(rt, rt.current_this(), args)
        });
        // setUTC* family. Each replaces the named component(s) in the
        // current ms and returns the new ms per ECMA §21.4.4.x.
        // E43: setUTC* + set* family routed through IR (cruftless treats __date_ms as UTC).
        register_intrinsic_method(self, proto, "setTime", 1, |rt, args| {
            crate::generated::date_prototype_set_time(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setUTCHours", 1, |rt, args| {
            crate::generated::date_prototype_set_hours(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setUTCMinutes", 1, |rt, args| {
            crate::generated::date_prototype_set_minutes(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setUTCSeconds", 1, |rt, args| {
            crate::generated::date_prototype_set_seconds(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setUTCMilliseconds", 1, |rt, args| {
            crate::generated::date_prototype_set_milliseconds(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setUTCDate", 1, |rt, args| {
            crate::generated::date_prototype_set_date(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setUTCMonth", 1, |rt, args| {
            crate::generated::date_prototype_set_month(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setUTCFullYear", 1, |rt, args| {
            crate::generated::date_prototype_set_full_year(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setHours", 1, |rt, args| {
            crate::generated::date_prototype_set_hours(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setMinutes", 1, |rt, args| {
            crate::generated::date_prototype_set_minutes(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setSeconds", 1, |rt, args| {
            crate::generated::date_prototype_set_seconds(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setMilliseconds", 1, |rt, args| {
            crate::generated::date_prototype_set_milliseconds(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setDate", 1, |rt, args| {
            crate::generated::date_prototype_set_date(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setMonth", 1, |rt, args| {
            crate::generated::date_prototype_set_month(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setFullYear", 1, |rt, args| {
            crate::generated::date_prototype_set_full_year(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "toISOString", 1, |rt, args| {
            crate::generated::date_prototype_to_iso_string(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "toJSON", 1, |rt, args| {
            crate::generated::date_prototype_to_json(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "toString", 0, |rt, args| {
            crate::generated::date_prototype_to_string(rt, rt.current_this(), args)
        });
        // Ω.5.P61.E12: Date.prototype additional format + legacy methods
        // per ECMA §21.4.4. v1 deviates from locale-sensitive output;
        // returns the ISO-like form (sufficient for module-init presence
        // probes; consumer-locale-display gaps not yet surfaced).
        let date_fmt_date = |rt: &mut Runtime, _args: &[Value]| -> Result<Value, RuntimeError> {
            let this_id = match rt.current_this() {
                Value::Object(id) => id,
                _ => return Ok(Value::String(Rc::new(String::new()))),
            };
            let ms = match rt.object_get(this_id, "__date_ms") {
                Value::Number(n) => n,
                _ => return Ok(Value::String(Rc::new("Invalid Date".into()))),
            };
            let (y, mo, d) = date_components(ms);
            Ok(Value::String(Rc::new(format!(
                "{:04}-{:02}-{:02}",
                y,
                mo + 1,
                d
            ))))
        };
        let date_fmt_time = |rt: &mut Runtime, _args: &[Value]| -> Result<Value, RuntimeError> {
            let this_id = match rt.current_this() {
                Value::Object(id) => id,
                _ => return Ok(Value::String(Rc::new(String::new()))),
            };
            let ms = match rt.object_get(this_id, "__date_ms") {
                Value::Number(n) => n,
                _ => return Ok(Value::String(Rc::new("Invalid Date".into()))),
            };
            let h = (ms / 3_600_000.0).floor() as i64 % 24;
            let mi = (ms / 60_000.0).floor() as i64 % 60;
            let se = (ms / 1000.0).floor() as i64 % 60;
            Ok(Value::String(Rc::new(format!(
                "{:02}:{:02}:{:02}",
                h, mi, se
            ))))
        };
        let date_fmt_utc = |rt: &mut Runtime, _args: &[Value]| -> Result<Value, RuntimeError> {
            let this_id = match rt.current_this() {
                Value::Object(id) => id,
                _ => return Ok(Value::String(Rc::new(String::new()))),
            };
            let ms = match rt.object_get(this_id, "__date_ms") {
                Value::Number(n) => n,
                _ => return Ok(Value::String(Rc::new("Invalid Date".into()))),
            };
            let (y, mo, d) = date_components(ms);
            let h = (ms / 3_600_000.0).floor() as i64 % 24;
            let mi = (ms / 60_000.0).floor() as i64 % 60;
            let se = (ms / 1000.0).floor() as i64 % 60;
            Ok(Value::String(Rc::new(format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} GMT",
                y,
                mo + 1,
                d,
                h,
                mi,
                se
            ))))
        };
        let _ = (date_fmt_date, date_fmt_time, date_fmt_utc);
        register_intrinsic_method(self, proto, "toDateString", 0, |rt, args| {
            crate::generated::date_prototype_to_date_string(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "toLocaleDateString", 0, |rt, args| {
            crate::generated::date_prototype_to_date_string(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "toTimeString", 0, |rt, args| {
            crate::generated::date_prototype_to_time_string(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "toLocaleTimeString", 0, |rt, args| {
            crate::generated::date_prototype_to_time_string(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "toUTCString", 0, |rt, args| {
            crate::generated::date_prototype_to_utc_string(rt, rt.current_this(), args)
        });
        // getYear / setYear per Annex B.2.4 (legacy). getYear returns
        // year - 1900; setYear sets full year, with two-digit values
        // mapped to 1900s for 0-99.
        register_intrinsic_method(self, proto, "getYear", 0, |rt, args| {
            crate::generated::date_prototype_get_year(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, proto, "setYear", 1, |rt, args| {
            crate::generated::date_prototype_set_year(rt, rt.current_this(), args)
        });
        let proto_for_ctor = proto;
        let ctor_obj = make_native("Date", move |rt, args| {
            // Tier-Ω.5.iiiii: Date(y, mo, d, h, m, s, ms) multi-arg ctor
            // must be checked FIRST per ECMA-262 §21.4.2.1 step 2 — when
            // NewTarget supplies ≥ 2 args, treat them as date components.
            // The prior order let Date(2026,4,15) fall through to the
            // single-Number arm and treat 2026 as a unix-ms timestamp.
            // Tier-Ω.5.qqqqq: when single arg is a Date / object, coerce
            // via valueOf per ECMA-262 §21.4.2.1. `new Date(otherDate)`
            // should copy the time, not yield epoch zero.
            let ms = if args.len() == 1 {
                if let Some(Value::Object(id)) = args.first() {
                    let v = rt.object_get(*id, "valueOf");
                    if matches!(v, Value::Object(_)) {
                        let r = rt.call_function(v, Value::Object(*id), Vec::new())?;
                        if let Value::Number(n) = r {
                            let mut o = Object::new_ordinary();
                            o.proto = Some(proto_for_ctor);
                            let new_id = rt.alloc_object(o);
                            rt.set_engine_sentinel(new_id, "__date_ms", Value::Number(n));
                            return Ok(Value::Object(new_id));
                        }
                    }
                }
                match args.first() {
                    Some(Value::Number(n)) => *n,
                    Some(Value::String(s)) => parse_date_string(s.as_str()),
                    _ => 0.0,
                }
            } else if args.len() >= 2 {
                // Tier-Ω.5.dddddd: ToNumber coercion on each component per
                // ECMA-262 §21.4.2.1 step 3. dayjs passes regex-match strings
                // like new Date("2026", 4, 15); previously we treated string
                // args as 0, yielding year 0000.
                let y = crate::abstract_ops::to_number(&args[0]) as i64;
                let mo = crate::abstract_ops::to_number(&args[1]) as i64;
                let d = args
                    .get(2)
                    .map(crate::abstract_ops::to_number)
                    .unwrap_or(1.0) as i64;
                let h = args
                    .get(3)
                    .map(crate::abstract_ops::to_number)
                    .unwrap_or(0.0) as i64;
                let mi = args
                    .get(4)
                    .map(crate::abstract_ops::to_number)
                    .unwrap_or(0.0) as i64;
                let se = args
                    .get(5)
                    .map(crate::abstract_ops::to_number)
                    .unwrap_or(0.0) as i64;
                let mss = args
                    .get(6)
                    .map(crate::abstract_ops::to_number)
                    .unwrap_or(0.0) as i64;
                (ymd_to_ms(y, mo, d) + h * 3_600_000 + mi * 60_000 + se * 1000 + mss) as f64
            } else {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as f64)
                    .unwrap_or(0.0)
            };
            let mut o = Object::new_ordinary();
            o.proto = Some(proto_for_ctor);
            let id = rt.alloc_object(o);
            rt.set_engine_sentinel(id, "__date_ms", Value::Number(ms));
            Ok(Value::Object(id))
        });
        let ctor = self.alloc_object(ctor_obj);
        register_intrinsic_method(self, ctor, "now", 0, |rt, args| {
            crate::generated::date_now(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, ctor, "parse", 2, |rt, args| {
            crate::generated::date_parse(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, ctor, "UTC", 1, |rt, args| {
            crate::generated::date_utc(rt, rt.current_this(), args)
        });
        self.obj_mut(ctor)
            .set_own_frozen("prototype".into(), Value::Object(proto));
        self.obj_mut(proto)
            .set_own_internal("constructor".into(), Value::Object(ctor));
        self.globals.insert("Date".into(), Value::Object(ctor));
    }

    /// Tier-Ω.5.dd: Uint8Array / ArrayBuffer / DataView / Int8Array etc.
    /// All as minimal stub constructors that succeed with `new X(n)` and
    /// expose `.length` / `.byteLength` / `.buffer`. Real binary semantics
    /// deferred to a substrate round.
    fn install_typed_array_stubs(&mut self) {
        // Tier-Ω.5.xxxx: shared TypedArray prototype with subarray / set /
        // slice / fill. tweetnacl, hash libs, and the crypto cluster reach
        // these methods at every step. Prior stub instances had no .subarray
        // so `keyPair()` failed at first byte op.
        let ta_proto = self.alloc_object(Object::new_ordinary());
        register_method(self, ta_proto, "subarray", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "subarray: this must be a TypedArray".into(),
                    ))
                }
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            let start = args
                .first()
                .and_then(|v| {
                    if let Value::Number(n) = v {
                        Some(*n as i64)
                    } else {
                        None
                    }
                })
                .unwrap_or(0);
            let end = args
                .get(1)
                .and_then(|v| {
                    if let Value::Number(n) = v {
                        Some(*n as i64)
                    } else {
                        None
                    }
                })
                .unwrap_or(len as i64);
            let start = (if start < 0 {
                (len as i64 + start).max(0)
            } else {
                start
            })
            .min(len as i64) as usize;
            let end = (if end < 0 {
                (len as i64 + end).max(0)
            } else {
                end
            })
            .min(len as i64) as usize;
            let slice_len = end.saturating_sub(start);
            let kind = match rt.object_get(this_id, "__kind") {
                Value::String(s) => (*s).clone(),
                _ => "Uint8Array".into(),
            };
            let mut o = Object::new_ordinary();
            o.set_own("length".into(), Value::Number(slice_len as f64));
            o.set_own_internal("__kind".into(), Value::String(Rc::new(kind)));
            let new_id = rt.alloc_object(o);
            for i in 0..slice_len {
                let v = rt.object_get(this_id, &(start + i).to_string());
                rt.object_set(new_id, i.to_string(), v);
            }
            // Inherit prototype from the source so subarray methods chain.
            let src_proto = rt.obj(this_id).proto;
            rt.obj_mut(new_id).proto = src_proto;
            Ok(Value::Object(new_id))
        });
        register_method(self, ta_proto, "set", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "set: this must be a TypedArray".into(),
                    ))
                }
            };
            let src = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Ok(Value::Undefined),
            };
            let offset = args
                .get(1)
                .and_then(|v| {
                    if let Value::Number(n) = v {
                        Some(*n as usize)
                    } else {
                        None
                    }
                })
                .unwrap_or(0);
            let src_len = match rt.object_get(src, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..src_len {
                let v = rt.object_get(src, &i.to_string());
                rt.object_set(this_id, (offset + i).to_string(), v);
            }
            Ok(Value::Undefined)
        });
        register_method(self, ta_proto, "fill", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "fill: this must be a TypedArray".into(),
                    ))
                }
            };
            let v = args.first().cloned().unwrap_or(Value::Number(0.0));
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..len {
                rt.object_set(this_id, i.to_string(), v.clone());
            }
            Ok(Value::Object(this_id))
        });
        register_method(self, ta_proto, "slice", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "slice: this must be a TypedArray".into(),
                    ))
                }
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            let start = args
                .first()
                .and_then(|v| {
                    if let Value::Number(n) = v {
                        Some(*n as i64)
                    } else {
                        None
                    }
                })
                .unwrap_or(0);
            let end = args
                .get(1)
                .and_then(|v| {
                    if let Value::Number(n) = v {
                        Some(*n as i64)
                    } else {
                        None
                    }
                })
                .unwrap_or(len as i64);
            let start = (if start < 0 {
                (len as i64 + start).max(0)
            } else {
                start
            })
            .min(len as i64) as usize;
            let end = (if end < 0 {
                (len as i64 + end).max(0)
            } else {
                end
            })
            .min(len as i64) as usize;
            let slice_len = end.saturating_sub(start);
            let mut o = Object::new_ordinary();
            o.set_own("length".into(), Value::Number(slice_len as f64));
            let new_id = rt.alloc_object(o);
            for i in 0..slice_len {
                let v = rt.object_get(this_id, &(start + i).to_string());
                rt.object_set(new_id, i.to_string(), v);
            }
            let src_proto = rt.obj(this_id).proto;
            rt.obj_mut(new_id).proto = src_proto;
            Ok(Value::Object(new_id))
        });
        // Tier-Ω.5.jjjjjj: TypedArray + Array @@iterator. for-of over
        // a Uint8Array currently fails with "@@iterator undefined" — add
        // index-cursor iterator on the prototype.
        // TypedArray.prototype iterator triplet (values/keys/entries).
        // Spec: values === [Symbol.iterator]. @noble/hashes returns
        // Uint8Array from sha3; cuid2 does `for (const i of buf.values())`;
        // superagent depends on cuid2. Without these the iteration path
        // throws "callee not callable: undefined (method='values')".
        register_method(self, ta_proto, "values", |rt, _args| {
            let src_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "values: this must be TypedArray".into(),
                    ))
                }
            };
            let mut o = Object::new_ordinary();
            o.set_own_internal("__it_src__".into(), Value::Object(src_id));
            o.set_own_internal("__it_idx__".into(), Value::Number(0.0));
            o.set_own_internal("__it_mode__".into(), Value::String(Rc::new("value".into())));
            let it_id = rt.alloc_object(o);
            register_intrinsic_method(rt, it_id, "next", 1, |rt, _args| ta_iter_next(rt));
            register_intrinsic_method(
                rt,
                it_id,
                "@@iterator",
                0,
                |rt, _args| Ok(rt.current_this()),
            );
            Ok(Value::Object(it_id))
        });
        register_method(self, ta_proto, "keys", |rt, _args| {
            let src_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "keys: this must be TypedArray".into(),
                    ))
                }
            };
            let mut o = Object::new_ordinary();
            o.set_own_internal("__it_src__".into(), Value::Object(src_id));
            o.set_own_internal("__it_idx__".into(), Value::Number(0.0));
            o.set_own_internal("__it_mode__".into(), Value::String(Rc::new("key".into())));
            let it_id = rt.alloc_object(o);
            register_intrinsic_method(rt, it_id, "next", 1, |rt, _args| ta_iter_next(rt));
            register_intrinsic_method(
                rt,
                it_id,
                "@@iterator",
                0,
                |rt, _args| Ok(rt.current_this()),
            );
            Ok(Value::Object(it_id))
        });
        register_method(self, ta_proto, "entries", |rt, _args| {
            let src_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "entries: this must be TypedArray".into(),
                    ))
                }
            };
            let mut o = Object::new_ordinary();
            o.set_own_internal("__it_src__".into(), Value::Object(src_id));
            o.set_own_internal("__it_idx__".into(), Value::Number(0.0));
            o.set_own_internal("__it_mode__".into(), Value::String(Rc::new("entry".into())));
            let it_id = rt.alloc_object(o);
            register_intrinsic_method(rt, it_id, "next", 1, |rt, _args| ta_iter_next(rt));
            register_intrinsic_method(
                rt,
                it_id,
                "@@iterator",
                0,
                |rt, _args| Ok(rt.current_this()),
            );
            Ok(Value::Object(it_id))
        });
        register_method(self, ta_proto, "@@iterator", |rt, _args| {
            let src_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "@@iterator: this must be TypedArray".into(),
                    ))
                }
            };
            let mut o = Object::new_ordinary();
            o.set_own_internal("__it_src__".into(), Value::Object(src_id));
            o.set_own_internal("__it_idx__".into(), Value::Number(0.0));
            let it_id = rt.alloc_object(o);
            register_intrinsic_method(rt, it_id, "next", 1, |rt, _args| {
                let this_id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Ok(Value::Undefined),
                };
                let src = match rt.object_get(this_id, "__it_src__") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Undefined),
                };
                let idx = match rt.object_get(this_id, "__it_idx__") {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };
                if rt.typed_array_view_out_of_bounds(src) {
                    return Err(RuntimeError::TypeError(
                        "TypedArray iterator receiver is out of bounds".into(),
                    ));
                }
                let len = match rt.object_get(src, "length") {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };
                let mut o = Object::new_ordinary();
                if idx >= len {
                    o.set_own("value".into(), Value::Undefined);
                    o.set_own("done".into(), Value::Boolean(true));
                } else {
                    let v = rt.object_get(src, &idx.to_string());
                    rt.object_set(
                        this_id,
                        "__it_idx__".into(),
                        Value::Number((idx + 1) as f64),
                    );
                    o.set_own("value".into(), v);
                    o.set_own("done".into(), Value::Boolean(false));
                }
                Ok(Value::Object(rt.alloc_object(o)))
            });
            Ok(Value::Object(it_id))
        });

        // Tier-Ω.5.P28.E1.typedarray-iter-methods: common Array-shaped methods
        // missing from the TypedArray prototype. Surfaced via Ω.5.P24.E1
        // proto-chain probe walking @dotenvx/dotenvx (Uint8Array.reverse
        // missing → proto-chain reported `Object→Object.prototype` since
        // typed-arrays are Object-backed and don't inherit from
        // Array.prototype). Cover the high-fanout set: reverse, indexOf,
        // includes, forEach, find, findIndex, every, some, join.
        register_method(self, ta_proto, "reverse", |rt, _args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "reverse: this must be a TypedArray".into(),
                    ))
                }
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            let mid = len / 2;
            for i in 0..mid {
                let j = len - 1 - i;
                let a = rt.object_get(this_id, &i.to_string());
                let b = rt.object_get(this_id, &j.to_string());
                rt.object_set(this_id, i.to_string(), b);
                rt.object_set(this_id, j.to_string(), a);
            }
            Ok(Value::Object(this_id))
        });
        register_method(self, ta_proto, "indexOf", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Number(-1.0)),
            };
            let needle = args.first().cloned().unwrap_or(Value::Undefined);
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                if crate::abstract_ops::is_strictly_equal(&v, &needle) {
                    return Ok(Value::Number(i as f64));
                }
            }
            Ok(Value::Number(-1.0))
        });
        register_method(self, ta_proto, "includes", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Boolean(false)),
            };
            let needle = args.first().cloned().unwrap_or(Value::Undefined);
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                if crate::abstract_ops::is_strictly_equal(&v, &needle) {
                    return Ok(Value::Boolean(true));
                }
            }
            Ok(Value::Boolean(false))
        });
        register_method(self, ta_proto, "forEach", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let cb = args
                .first()
                .cloned()
                .ok_or_else(|| RuntimeError::TypeError("forEach: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                rt.call_function(
                    cb.clone(),
                    Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)],
                )?;
            }
            Ok(Value::Undefined)
        });
        register_method(self, ta_proto, "find", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let cb = args
                .first()
                .cloned()
                .ok_or_else(|| RuntimeError::TypeError("find: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(
                    cb.clone(),
                    Value::Undefined,
                    vec![v.clone(), Value::Number(i as f64), Value::Object(this_id)],
                )?;
                if abstract_ops::to_boolean(&r) {
                    return Ok(v);
                }
            }
            Ok(Value::Undefined)
        });
        register_method(self, ta_proto, "findIndex", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Number(-1.0)),
            };
            let cb = args
                .first()
                .cloned()
                .ok_or_else(|| RuntimeError::TypeError("findIndex: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(
                    cb.clone(),
                    Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)],
                )?;
                if abstract_ops::to_boolean(&r) {
                    return Ok(Value::Number(i as f64));
                }
            }
            Ok(Value::Number(-1.0))
        });
        register_method(self, ta_proto, "every", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Boolean(true)),
            };
            let cb = args
                .first()
                .cloned()
                .ok_or_else(|| RuntimeError::TypeError("every: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(
                    cb.clone(),
                    Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)],
                )?;
                if !abstract_ops::to_boolean(&r) {
                    return Ok(Value::Boolean(false));
                }
            }
            Ok(Value::Boolean(true))
        });
        register_method(self, ta_proto, "some", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Boolean(false)),
            };
            let cb = args
                .first()
                .cloned()
                .ok_or_else(|| RuntimeError::TypeError("some: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(
                    cb.clone(),
                    Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)],
                )?;
                if abstract_ops::to_boolean(&r) {
                    return Ok(Value::Boolean(true));
                }
            }
            Ok(Value::Boolean(false))
        });
        register_method(self, ta_proto, "join", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::String(Rc::new(String::new()))),
            };
            let sep = match args.first() {
                Some(v) => abstract_ops::to_string(v).as_str().to_string(),
                None => ",".into(),
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            let mut out = String::new();
            for i in 0..len {
                if i > 0 {
                    out.push_str(&sep);
                }
                let v = rt.object_get(this_id, &i.to_string());
                let s = abstract_ops::to_string(&v);
                out.push_str(s.as_str());
            }
            Ok(Value::String(Rc::new(out)))
        });

        // Ω.5.P58.E9: TypedArray.prototype.{map, filter, reduce, reduceRight,
        // sort, copyWithin, toString} per ECMA §23.2.3.
        // Ω.5.P59.E6: results of .map/.filter are same-kind TypedArrays
        // per §23.2.3.21 (TypedArraySpeciesCreate). Pre-P59.E6 result was
        // a plain Array, which JSON.stringify serialized as `[...]`
        // (vs Bun's `{0:...}` object shape) — visible byte-shape
        // divergence in any consumer that probed map/filter outputs.
        // The shape: an ordinary Object with the source's proto (ta_proto
        // via the type-specific subtype chain), length, byteLength,
        // __kind sentinel (non-enumerable per P58.E1).
        register_method(self, ta_proto, "map", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "TypedArray.prototype.map: this must be a TypedArray".into(),
                    ))
                }
            };
            let f = match args.first() {
                Some(v @ Value::Object(_)) => v.clone(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "TypedArray.prototype.map: callback must be a function".into(),
                    ))
                }
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            let out = make_typed_array_like(rt, this_id, len);
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(
                    f.clone(),
                    Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)],
                )?;
                rt.object_set(out, i.to_string(), r);
            }
            Ok(Value::Object(out))
        });
        register_method(self, ta_proto, "filter", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "TypedArray.prototype.filter: this must be a TypedArray".into(),
                    ))
                }
            };
            let f = match args.first() {
                Some(v @ Value::Object(_)) => v.clone(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "TypedArray.prototype.filter: callback must be a function".into(),
                    ))
                }
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            // Two-pass: first collect matches, then alloc with right length.
            let mut keeps: Vec<Value> = Vec::with_capacity(len);
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let pred = rt.call_function(
                    f.clone(),
                    Value::Undefined,
                    vec![v.clone(), Value::Number(i as f64), Value::Object(this_id)],
                )?;
                if abstract_ops::to_boolean(&pred) {
                    keeps.push(v);
                }
            }
            let out = make_typed_array_like(rt, this_id, keeps.len());
            for (i, v) in keeps.into_iter().enumerate() {
                rt.object_set(out, i.to_string(), v);
            }
            Ok(Value::Object(out))
        });
        register_method(self, ta_proto, "reduce", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "TypedArray.prototype.reduce: this must be a TypedArray".into(),
                    ))
                }
            };
            let f = match args.first() {
                Some(v @ Value::Object(_)) => v.clone(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "TypedArray.prototype.reduce: callback must be a function".into(),
                    ))
                }
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            let (mut acc, start) = match args.get(1) {
                Some(v) => (v.clone(), 0),
                None => {
                    if len == 0 {
                        return Err(RuntimeError::TypeError(
                            "TypedArray.prototype.reduce: empty with no initial".into(),
                        ));
                    }
                    (rt.object_get(this_id, "0"), 1)
                }
            };
            for i in start..len {
                let v = rt.object_get(this_id, &i.to_string());
                acc = rt.call_function(
                    f.clone(),
                    Value::Undefined,
                    vec![acc, v, Value::Number(i as f64), Value::Object(this_id)],
                )?;
            }
            Ok(acc)
        });
        register_method(self, ta_proto, "reduceRight", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "TypedArray.prototype.reduceRight: this must be a TypedArray".into(),
                    ))
                }
            };
            let f = match args.first() {
                Some(v @ Value::Object(_)) => v.clone(),
                _ => {
                    return Err(RuntimeError::TypeError(
                        "TypedArray.prototype.reduceRight: callback must be a function".into(),
                    ))
                }
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            let (mut acc, start_back) = match args.get(1) {
                Some(v) => (v.clone(), len as i64 - 1),
                None => {
                    if len == 0 {
                        return Err(RuntimeError::TypeError(
                            "TypedArray.prototype.reduceRight: empty with no initial".into(),
                        ));
                    }
                    (
                        rt.object_get(this_id, &(len - 1).to_string()),
                        len as i64 - 2,
                    )
                }
            };
            let mut i = start_back;
            while i >= 0 {
                let v = rt.object_get(this_id, &i.to_string());
                acc = rt.call_function(
                    f.clone(),
                    Value::Undefined,
                    vec![acc, v, Value::Number(i as f64), Value::Object(this_id)],
                )?;
                i -= 1;
            }
            Ok(acc)
        });
        register_method(self, ta_proto, "toString", |rt, _args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::String(Rc::new(String::new()))),
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize,
                _ => 0,
            };
            let mut out = String::new();
            for i in 0..len {
                if i > 0 {
                    out.push(',');
                }
                let v = rt.object_get(this_id, &i.to_string());
                let s = abstract_ops::to_string(&v);
                out.push_str(s.as_str());
            }
            Ok(Value::String(Rc::new(out)))
        });

        // Tier-Ω.5.ZZZZZZZ: install @@toStringTag accessor at the spec
        // location — on %TypedArray%.prototype, which sits ONE LEVEL ABOVE
        // each per-element-type prototype (Int8Array.prototype etc.).
        // safe-stable-stringify (under roarr / slonik / mongoose) walks
        //   Object.getPrototypeOf(Object.getPrototypeOf(new Int8Array()))
        // (i.e. two levels) and reads
        //   Object.getOwnPropertyDescriptor(__, Symbol.toStringTag).get
        // V1 layout had a single shared ta_proto; this commit splits it into
        // a per-instance level (ta_proto, still holding subarray/set/fill/
        // slice/@@iterator) whose [[Prototype]] is a fresh %TypedArray%
        // prototype-stub that carries the toStringTag accessor. Both walks
        // (1 or 2 levels) now reach an object with the accessor at level 2.
        let tag_getter = make_native("get @@toStringTag", |rt, _args| match rt.current_this() {
            Value::Object(id) => match rt.object_get(id, "__ta_kind") {
                v @ Value::String(_) => Ok(v),
                _ => Ok(Value::Undefined),
            },
            _ => Ok(Value::Undefined),
        });
        let tag_getter_id = self.alloc_object(tag_getter);
        let ta_proto_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(ta_proto_proto).dict_mut().insert(
            "@@toStringTag".into(),
            crate::value::PropertyDescriptor {
                value: Value::Undefined,
                writable: false,
                enumerable: false,
                configurable: true,
                getter: Some(Value::Object(tag_getter_id)),
                setter: None,
            },
        );
        self.obj_mut(ta_proto).proto = Some(ta_proto_proto);

        let array_buffer_proto = self.alloc_object(Object::new_ordinary());
        register_method(self, array_buffer_proto, "resize", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "ArrayBuffer.prototype.resize: this must be an ArrayBuffer".into(),
                    ))
                }
            };
            let new_len = match args.first() {
                Some(Value::Number(n)) if *n >= 0.0 => *n as usize,
                Some(v) => rt.coerce_to_number(v)? as usize,
                None => 0,
            };
            rt.resize_array_buffer(this_id, new_len)?;
            Ok(Value::Undefined)
        });
        let ab_proto_for_ctor = array_buffer_proto;
        let array_buffer_ctor = make_native("ArrayBuffer", move |rt, args| {
            let byte_length = match args.first() {
                Some(Value::Number(n)) if *n >= 0.0 => *n as usize,
                Some(v) => rt.coerce_to_number(v)? as usize,
                None => 0,
            };
            let mut max_byte_length = byte_length;
            if let Some(Value::Object(opts)) = args.get(1) {
                if let Value::Number(n) = rt.object_get(*opts, "maxByteLength") {
                    if n >= 0.0 {
                        max_byte_length = n as usize;
                    }
                }
            }
            let mut o = Object::new_ordinary();
            o.set_own_internal(
                "__kind".into(),
                Value::String(Rc::new("ArrayBuffer".into())),
            );
            o.proto = Some(ab_proto_for_ctor);
            let id = rt.alloc_object(o);
            rt.array_buffers.insert(
                id,
                crate::interp::ArrayBufferRecord {
                    byte_length,
                    max_byte_length,
                    data: vec![Value::Number(0.0); byte_length],
                },
            );
            Ok(Value::Object(id))
        });
        let ab_id = self.alloc_object(array_buffer_ctor);
        self.obj_mut(ab_id)
            .set_own_frozen("prototype".into(), Value::Object(array_buffer_proto));
        self.globals
            .insert("ArrayBuffer".into(), Value::Object(ab_id));

        for name in &[
            "SharedArrayBuffer",
            "DataView",
            "Uint8Array",
            "Uint8ClampedArray",
            "Int8Array",
            "Uint16Array",
            "Int16Array",
            "Uint32Array",
            "Int32Array",
            "Float32Array",
            "Float64Array",
            "BigInt64Array",
            "BigUint64Array",
        ] {
            let n = (*name).to_string();
            let proto_id = ta_proto;
            let ctor_obj = make_native(name, move |rt, args| {
                // Ω.5.P59.E6 byteLength correctness: bytes-per-element
                // per typed-array kind. Pre-P59.E6 cruftless hardcoded
                // `len * 4.0` which was wrong for every element type
                // except 32-bit ones. Bun's Uint8Array(4).byteLength === 4.
                let bpe: usize = match n.as_str() {
                    "Int8Array" | "Uint8Array" | "Uint8ClampedArray" => 1,
                    "Int16Array" | "Uint16Array" => 2,
                    "Int32Array" | "Uint32Array" | "Float32Array" => 4,
                    "Float64Array" | "BigInt64Array" | "BigUint64Array" => 8,
                    _ => 4,
                };
                if let Some(Value::Object(buf)) = args.first() {
                    if rt.array_buffers.contains_key(buf)
                        && n != "DataView"
                        && n != "SharedArrayBuffer"
                    {
                        let byte_offset = match args.get(1) {
                            Some(Value::Number(n)) if *n >= 0.0 => *n as usize,
                            Some(v) => rt.coerce_to_number(v)? as usize,
                            None => 0,
                        };
                        let fixed_length = match args.get(2) {
                            Some(Value::Number(n)) if *n >= 0.0 => Some(*n as usize),
                            Some(v) => Some(rt.coerce_to_number(v)? as usize),
                            None => None,
                        };
                        let mut o = Object::new_ordinary();
                        o.set_own_internal("__kind".into(), Value::String(Rc::new(n.clone())));
                        o.set_own_internal("__ta_kind".into(), Value::String(Rc::new(n.clone())));
                        o.proto = Some(proto_id);
                        let id = rt.alloc_object(o);
                        rt.typed_array_views.insert(
                            id,
                            crate::interp::TypedArrayViewRecord {
                                buffer: *buf,
                                byte_offset,
                                fixed_length,
                                bytes_per_element: bpe,
                            },
                        );
                        return Ok(Value::Object(id));
                    }
                }
                let len = match args.first() {
                    Some(Value::Number(n)) => *n,
                    Some(Value::Object(arr)) => {
                        // new Uint8Array(arrayLike) — copy length+contents.
                        match rt.object_get(*arr, "length") {
                            Value::Number(n) => n,
                            _ => 0.0,
                        }
                    }
                    _ => 0.0,
                };
                let mut o = Object::new_ordinary();
                o.set_own("length".into(), Value::Number(len));
                o.set_own("byteLength".into(), Value::Number(len * bpe as f64));
                o.set_own_internal("__kind".into(), Value::String(Rc::new(n.clone())));
                o.set_own_internal("__ta_kind".into(), Value::String(Rc::new(n.clone())));
                o.proto = Some(proto_id);
                let id = rt.alloc_object(o);
                // Copy from source if first arg was an object.
                if let Some(Value::Object(src)) = args.first() {
                    let src_len = len as usize;
                    for i in 0..src_len {
                        let v = rt.object_get(*src, &i.to_string());
                        rt.object_set(id, i.to_string(), v);
                    }
                } else {
                    // Zero-initialize for new Uint8Array(N).
                    let cap = (len as usize).min(65536);
                    for i in 0..cap {
                        rt.object_set(id, i.to_string(), Value::Number(0.0));
                    }
                }
                Ok(Value::Object(id))
            });
            let id = self.alloc_object(ctor_obj);
            let bpe = match *name {
                "Int8Array" | "Uint8Array" | "Uint8ClampedArray" => 1.0,
                "Int16Array" | "Uint16Array" => 2.0,
                "Int32Array" | "Uint32Array" | "Float32Array" => 4.0,
                "Float64Array" | "BigInt64Array" | "BigUint64Array" => 8.0,
                _ => 1.0,
            };
            self.obj_mut(id)
                .set_own_frozen("BYTES_PER_ELEMENT".into(), Value::Number(bpe));
            register_intrinsic_method(self, id, "isView", 1, |_rt, _args| {
                Ok(Value::Boolean(false))
            });
            let from_proto = ta_proto;
            let of_proto = ta_proto;
            register_intrinsic_method(self, id, "of", 0, move |rt, args| {
                // TypedArray.of(...items) per ECMA §23.2.2.2 — pack args.
                let len = args.len();
                let mut o = Object::new_ordinary();
                o.set_own("length".into(), Value::Number(len as f64));
                o.proto = Some(of_proto);
                let new_id = rt.alloc_object(o);
                for (i, v) in args.iter().enumerate() {
                    rt.object_set(new_id, i.to_string(), v.clone());
                }
                Ok(Value::Object(new_id))
            });
            register_intrinsic_method(self, id, "from", 1, move |rt, args| {
                let src = args.first().cloned().unwrap_or(Value::Undefined);
                let len: usize = match &src {
                    Value::Object(id) => rt.array_length(*id) as usize,
                    Value::String(s) => s.chars().count(),
                    _ => 0,
                };
                let mut o = Object::new_ordinary();
                o.set_own("length".into(), Value::Number(len as f64));
                o.proto = Some(from_proto);
                let new_id = rt.alloc_object(o);
                if let Value::Object(sid) = &src {
                    for i in 0..len {
                        let v = rt.object_get(*sid, &i.to_string());
                        rt.object_set(new_id, i.to_string(), v);
                    }
                }
                Ok(Value::Object(new_id))
            });
            self.obj_mut(id)
                .set_own_frozen("prototype".into(), Value::Object(ta_proto));
            self.globals.insert((*name).to_string(), Value::Object(id));
        }
    }

    /// Tier-Ω.5.dd: WeakRef + FinalizationRegistry minimal stubs. Real
    /// weak-reference semantics need GC integration (deferred). Stubs hold
    /// strong references for v1; `.deref()` always returns the held value.
    fn install_weak_ref_globals(&mut self) {
        let weakref_proto = self.alloc_object(Object::new_ordinary());
        register_method(self, weakref_proto, "deref", |rt, _args| {
            let this = match rt.current_this() {
                Value::Object(id) => id,
                _ => return Ok(Value::Undefined),
            };
            Ok(rt.object_get(this, "__ref"))
        });
        let proto_for_ctor = weakref_proto;
        let weakref_ctor = make_native("WeakRef", move |rt, args| {
            let target = args.first().cloned().unwrap_or(Value::Undefined);
            let mut o = Object::new_ordinary();
            o.proto = Some(proto_for_ctor);
            let id = rt.alloc_object(o);
            rt.object_set(id, "__ref".into(), target);
            Ok(Value::Object(id))
        });
        let wr = self.alloc_object(weakref_ctor);
        self.obj_mut(wr)
            .set_own_frozen("prototype".into(), Value::Object(weakref_proto));
        self.obj_mut(weakref_proto)
            .set_own_internal("constructor".into(), Value::Object(wr));
        self.globals.insert("WeakRef".into(), Value::Object(wr));

        let fr_proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, fr_proto, "register", 1, |_rt, _args| {
            Ok(Value::Undefined)
        });
        register_intrinsic_method(self, fr_proto, "unregister", 1, |_rt, _args| {
            Ok(Value::Boolean(true))
        });
        let fr_proto_for_ctor = fr_proto;
        let fr_ctor = make_native("FinalizationRegistry", move |rt, _args| {
            let mut o = Object::new_ordinary();
            o.proto = Some(fr_proto_for_ctor);
            Ok(Value::Object(rt.alloc_object(o)))
        });
        let fr = self.alloc_object(fr_ctor);
        self.obj_mut(fr)
            .set_own_frozen("prototype".into(), Value::Object(fr_proto));
        self.obj_mut(fr_proto)
            .set_own_internal("constructor".into(), Value::Object(fr));
        self.globals
            .insert("FinalizationRegistry".into(), Value::Object(fr));
    }

    /// Tier-Ω.5.cc: Reflect global — most methods route to existing Object
    /// statics. has/get/set/deleteProperty/ownKeys/getPrototypeOf used by
    /// many packages doing duck-type checks.
    fn install_reflect(&mut self) {
        let r = self.alloc_object(Object::new_ordinary());
        // Ω.5.P63.E12: Reflect.has/get/set/deleteProperty routed through IR.
        register_intrinsic_method(self, r, "has", 2, |rt, args| {
            crate::generated::reflect_has(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "get", 2, |rt, args| {
            crate::generated::reflect_get(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "set", 3, |rt, args| {
            crate::generated::reflect_set(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "deleteProperty", 2, |rt, args| {
            crate::generated::reflect_delete_property(rt, Value::Undefined, args)
        });
        // EXT 79d: Reflect.{ownKeys, getPrototypeOf, setPrototypeOf,
        // defineProperty, getOwnPropertyDescriptor, isExtensible,
        // preventExtensions} all route through their Proxy handler trap
        // when the target is a Proxy with a callable [trap] method.
        // Missing trap → fall through to the IR-routed direct-target
        // implementation. Trap signatures match spec (§28.1.*).
        register_intrinsic_method(self, r, "ownKeys", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "ownKeys");
                    if matches!(trap, Value::Object(_)) {
                        // EXT 86: validate trap result against §10.5.11
                        // invariants, then re-pack the validated key list
                        // into a fresh Array (preserves trap order, drops
                        // any non-key entries the invariants caught).
                        let result = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        let trap_keys = rt.apply_proxy_own_keys_invariants(&result, tgt)?;
                        let out = rt.alloc_object(Object::new_array());
                        for (i, k) in trap_keys.iter().enumerate() {
                            rt.object_set(out, i.to_string(), k.clone());
                        }
                        rt.object_set(out, "length".into(), Value::Number(trap_keys.len() as f64));
                        return Ok(Value::Object(out));
                    }
                    return crate::generated::reflect_own_keys(
                        rt,
                        Value::Undefined,
                        &[Value::Object(tgt)],
                    );
                }
            }
            crate::generated::reflect_own_keys(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "getPrototypeOf", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "getPrototypeOf");
                    if matches!(trap, Value::Object(_)) {
                        return rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        );
                    }
                    return crate::generated::reflect_get_prototype_of(
                        rt,
                        Value::Undefined,
                        &[Value::Object(tgt)],
                    );
                }
            }
            crate::generated::reflect_get_prototype_of(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "defineProperty", 3, |rt, args| {
            // Spec sec 28.1.3 Reflect.defineProperty: dispatch to
            // OrdinaryDefineOwnProperty (NOT DefinePropertyOrThrow), then
            // return the Boolean result. Validation failures return false;
            // only abrupt completions from getter/setter side effects
            // propagate. The shim here invokes Object.defineProperty's
            // helper (which throws on validation failure) and catches the
            // validation-shaped TypeErrors back into Boolean(false).
            // Abrupt completions from descriptor getters / accessor calls
            // continue to propagate.
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "defineProperty");
                    if matches!(trap, Value::Object(_)) {
                        let key = args.get(1).cloned().unwrap_or(Value::Undefined);
                        let desc = args.get(2).cloned().unwrap_or(Value::Undefined);
                        let r2 = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt), key, desc],
                        )?;
                        return Ok(Value::Boolean(crate::abstract_ops::to_boolean(&r2)));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return match crate::generated::object_define_property(
                        rt,
                        Value::Undefined,
                        &new_args,
                    ) {
                        Ok(_) => Ok(Value::Boolean(true)),
                        Err(RuntimeError::TypeError(msg))
                            if msg.contains("Cannot redefine")
                                || msg.contains("Cannot add property")
                                || msg.contains("not extensible") =>
                        {
                            Ok(Value::Boolean(false))
                        }
                        Err(e) => Err(e),
                    };
                }
            }
            match crate::generated::object_define_property(rt, Value::Undefined, args) {
                Ok(_) => Ok(Value::Boolean(true)),
                Err(RuntimeError::TypeError(msg))
                    if msg.contains("Cannot redefine")
                        || msg.contains("Cannot add property")
                        || msg.contains("not extensible") =>
                {
                    Ok(Value::Boolean(false))
                }
                Err(e) => Err(e),
            }
        });
        register_intrinsic_method(self, r, "getOwnPropertyDescriptor", 2, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "getOwnPropertyDescriptor");
                    if matches!(trap, Value::Object(_)) {
                        let key = args.get(1).cloned().unwrap_or(Value::Undefined);
                        let trap_result = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt), key.clone()],
                        )?;
                        // EXT 89 / Pass C: §10.5.5 invariants (undefined-leg + non-Object check).
                        let key_str = crate::abstract_ops::to_string(&key).as_str().to_string();
                        rt.apply_proxy_get_own_property_descriptor_invariant(
                            tgt,
                            &key_str,
                            &trap_result,
                        )?;
                        return Ok(trap_result);
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::object_get_own_property_descriptor(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::object_get_own_property_descriptor(rt, Value::Undefined, args)
        });
        // Tier-Ω.5.rrrrr: Reflect.setPrototypeOf / apply / construct.
        register_intrinsic_method(self, r, "setPrototypeOf", 2, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "setPrototypeOf");
                    if matches!(trap, Value::Object(_)) {
                        let proto = args.get(1).cloned().unwrap_or(Value::Undefined);
                        let r2 = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt), proto],
                        )?;
                        return Ok(Value::Boolean(crate::abstract_ops::to_boolean(&r2)));
                    }
                    let mut new_args = args.to_vec();
                    new_args[0] = Value::Object(tgt);
                    return crate::generated::reflect_set_prototype_of(
                        rt,
                        Value::Undefined,
                        &new_args,
                    );
                }
            }
            crate::generated::reflect_set_prototype_of(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "apply", 3, |rt, args| {
            let target = args.first().cloned().unwrap_or(Value::Undefined);
            let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
            // EXT 79c: ECMA §7.3.18 CreateListFromArrayLike. Read length
            // via the property-access path (invokes inherited getters,
            // dispatches Proxy.get traps, propagates throws), and read
            // each element index the same way. The prior path used
            // array_length / object_get which bypassed user accessors,
            // so a length-getter throw never surfaced.
            // EXT 79c: ECMA §7.3.18 CreateListFromArrayLike. Non-Object
            // argumentsList (including undefined / null) throws TypeError.
            let arg_list: Vec<Value> = match args.get(2) {
                Some(Value::Object(arr)) => {
                    let arr_v = Value::Object(*arr);
                    let len_v = rt.spec_get(&arr_v, "length")?;
                    let len = crate::abstract_ops::to_number(&len_v) as usize;
                    let mut v = Vec::with_capacity(len);
                    for i in 0..len {
                        v.push(rt.spec_get(&arr_v, &i.to_string())?);
                    }
                    v
                }
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Reflect.apply: argumentsList must be an Object".into(),
                    ))
                }
            };
            rt.call_function(target, this_arg, arg_list)
        });
        register_intrinsic_method(self, r, "construct", 2, |rt, args| {
            let target = args.first().cloned().unwrap_or(Value::Undefined);
            // Ω.5.P61.E4: IsConstructor check per ECMA §10.5.13. The
            // new-target (3rd arg, falls back to target if missing) is
            // what must satisfy IsConstructor — test262's isConstructor
            // helper passes the candidate as newTarget. Both target and
            // newTarget must be constructors per §28.1.5.
            let new_target = args.get(2).cloned().unwrap_or(target.clone());
            for v in [&target, &new_target] {
                if let Value::Object(id) = v {
                    if let crate::value::InternalKind::Function(fi) = &rt.obj(*id).internal_kind {
                        if !fi.is_constructor {
                            return Err(RuntimeError::TypeError(format!(
                                "Reflect.construct: {} is not a constructor",
                                fi.name
                            )));
                        }
                    }
                } else {
                    return Err(RuntimeError::TypeError(
                        "Reflect.construct: target/newTarget must be a constructor".into(),
                    ));
                }
            }
            // EXT 79c: Reflect.construct's argumentsList uses the same
            // CreateListFromArrayLike path as Reflect.apply above; non-
            // Object argumentsList throws TypeError per §7.3.18.
            let arg_list: Vec<Value> = match args.get(1) {
                Some(Value::Object(arr)) => {
                    let arr_v = Value::Object(*arr);
                    let len_v = rt.spec_get(&arr_v, "length")?;
                    let len = crate::abstract_ops::to_number(&len_v) as usize;
                    let mut v = Vec::with_capacity(len);
                    for i in 0..len {
                        v.push(rt.spec_get(&arr_v, &i.to_string())?);
                    }
                    v
                }
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Reflect.construct: argumentsList must be an Object".into(),
                    ))
                }
            };
            // Use Op::New-equivalent via call_function with a fresh this.
            let proto_id = match &target {
                Value::Object(tid) => match rt.object_get(*tid, "prototype") {
                    Value::Object(pid) => Some(pid),
                    _ => None,
                },
                _ => None,
            };
            let mut o = Object::new_ordinary();
            o.proto = proto_id;
            let this_id = rt.alloc_object(o);
            let this_obj = Value::Object(this_id);
            rt.pending_new_target = Some(new_target);
            let ret = rt.call_function(target, this_obj.clone(), arg_list)?;
            Ok(match ret {
                Value::Object(_) => ret,
                _ => this_obj,
            })
        });
        // EXT 79d (cont.): isExtensible / preventExtensions Proxy traps.
        register_intrinsic_method(self, r, "isExtensible", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "isExtensible");
                    if matches!(trap, Value::Object(_)) {
                        let r2 = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        return Ok(Value::Boolean(crate::abstract_ops::to_boolean(&r2)));
                    }
                    return crate::generated::reflect_is_extensible(
                        rt,
                        Value::Undefined,
                        &[Value::Object(tgt)],
                    );
                }
            }
            crate::generated::reflect_is_extensible(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "preventExtensions", 1, |rt, args| {
            if let Some(Value::Object(id)) = args.first() {
                if let Some((tgt, handler)) = rt.proxy_target_handler_checked(*id)? {
                    let trap = rt.object_get(handler, "preventExtensions");
                    if matches!(trap, Value::Object(_)) {
                        let r2 = rt.call_function(
                            trap,
                            Value::Object(handler),
                            vec![Value::Object(tgt)],
                        )?;
                        return Ok(Value::Boolean(crate::abstract_ops::to_boolean(&r2)));
                    }
                    return crate::generated::reflect_prevent_extensions(
                        rt,
                        Value::Undefined,
                        &[Value::Object(tgt)],
                    );
                }
            }
            crate::generated::reflect_prevent_extensions(rt, Value::Undefined, args)
        });
        self.globals.insert("Reflect".into(), Value::Object(r));
    }

    /// Tier-Ω.5.z: Error + TypeError + RangeError + SyntaxError + ReferenceError
    /// + URIError + EvalError constructors. Each is callable; carrying a
    /// .prototype so `class X extends Error {}` works (the dense pattern
    /// in real packages: ulid, joi, commander, luxon all use it).
    /// The Error.prototype object exposes .name and .message so duck-type
    /// checks pass; instance shape is `{name, message, stack:""}`.
    fn install_error_globals(&mut self) {
        for (name, default_name) in &[
            ("Error", "Error"),
            ("TypeError", "TypeError"),
            ("RangeError", "RangeError"),
            ("SyntaxError", "SyntaxError"),
            ("ReferenceError", "ReferenceError"),
            ("URIError", "URIError"),
            ("EvalError", "EvalError"),
            ("AggregateError", "AggregateError"),
        ] {
            let proto_id = self.alloc_object(Object::new_ordinary());
            // §20.5.6.{1,2}: Error.prototype.{name, message} are non-enumerable.
            self.obj_mut(proto_id).set_own_internal(
                "name".into(),
                Value::String(Rc::new((*default_name).to_string())),
            );
            self.obj_mut(proto_id)
                .set_own_internal("message".into(), Value::String(Rc::new("".to_string())));
            register_intrinsic_method(self, proto_id, "toString", 0, |rt, args| {
                crate::generated::error_prototype_to_string(rt, rt.current_this(), args)
            });

            let default_name = (*default_name).to_string();
            let proto_for_ctor = proto_id;
            // §20.5.7.1: Error.length === 1 (single 'message' parameter).
            // AggregateError takes (errors, message) but spec is .length === 2.
            let ctor_arity: u32 = if *name == "AggregateError" { 2 } else { 1 };
            let ctor_obj = make_native_with_length(name, ctor_arity, move |rt, args| {
                // Tier-Ω.5.ffff: when invoked via super(...) from a
                // derived class, the receiver is the already-allocated
                // derived-instance. Mutate it in place rather than
                // allocating a fresh one — otherwise `class E extends
                // Error { constructor(m) { super(m); } }; new E('hi')`
                // produces an E with empty .message because the Error
                // native allocates a sibling Object and discards it
                // (Op::CallMethod takes call_function's return Object
                // as the result, overwriting the synthesized this).
                let receiver_id = match rt.current_this() {
                    Value::Object(id) => {
                        // Use receiver iff it's an ordinary (not
                        // already an Error-shaped) object. The derived
                        // class's Op::New synthesized this with proto
                        // wired to the derived ctor's prototype, which
                        // already inherits from Error.prototype.
                        Some(id)
                    }
                    _ => None,
                };
                let id = match receiver_id {
                    Some(id) => id,
                    None => {
                        let mut o = Object::new_ordinary();
                        o.proto = Some(proto_for_ctor);
                        rt.alloc_object(o)
                    }
                };
                // EIPD-EXT 1: §20.5.7.1 ErrorConstructor + §20.5.6.1
                // InstallErrorCause. message / cause / stack are installed
                // via CreateNonEnumerableDataPropertyOrThrow → {w:t, e:f,
                // c:t}. `name` lives on Error.prototype via set_own_internal
                // (already non-enumerable); do NOT install per-instance —
                // the prototype entry serves all instances.
                let install_non_enum =
                    |rt: &mut Runtime, id: crate::value::ObjectRef, k: &str, v: Value| {
                        rt.obj_mut(id).dict_mut().insert(
                            crate::value::PropertyKey::String(k.to_string()),
                            crate::value::PropertyDescriptor {
                                value: v,
                                writable: true,
                                enumerable: false,
                                configurable: true,
                                getter: None,
                                setter: None,
                            },
                        );
                    };
                if let Some(msg) = args.first() {
                    if !matches!(msg, Value::Undefined) {
                        let m = rt.coerce_to_string(msg)?;
                        install_non_enum(rt, id, "message", Value::String(Rc::new(m)));
                    }
                }
                install_non_enum(rt, id, "stack", Value::String(Rc::new("".into())));
                // ES2022 (§20.5.7.1 step 4) — InstallErrorCause: if the
                // second argument is an Object with a `cause` own key,
                // install error.cause as a non-enumerable property.
                if let Some(Value::Object(opts_id)) = args.get(1) {
                    let has_cause = rt.obj(*opts_id).has_own_str("cause");
                    if has_cause {
                        let cause = rt.object_get(*opts_id, "cause");
                        install_non_enum(rt, id, "cause", cause);
                    }
                }
                Ok(Value::Object(id))
            });
            let ctor_id = self.alloc_object(ctor_obj);
            self.obj_mut(ctor_id)
                .set_own_frozen("prototype".into(), Value::Object(proto_id));
            // proto.constructor = ctor (per spec).
            self.obj_mut(proto_id)
                .set_own_internal("constructor".into(), Value::Object(ctor_id));
            // Tier-Ω.5.JJJJJJJ: Error.captureStackTrace(target, ctorOpt) per V8
            // convention. http-errors / koa / serve-static (via depd) call it
            // at module-init to attach a `stack` string to a fresh error-like
            // object. Spec is V8-extension, not ECMA; implementation sets
            // target.stack = "" (no real trace yet — engine doesn't capture
            // frame data) so callers' presence-and-shape checks pass.
            // Installed on every Error-family constructor (TypeError /
            // RangeError / etc.) since real Node attaches it to all of them.
            register_intrinsic_method(self, ctor_id, "captureStackTrace", 1, |rt, args| {
                if let Some(Value::Object(target)) = args.first() {
                    // Per V8 convention, if Error.prepareStackTrace is set, it
                    // is invoked with (target, framesArray) and its return
                    // value becomes target.stack. depd does this to capture
                    // file/line info for deprecation warnings:
                    //     Error.prepareStackTrace = (err, frames) => frames;
                    //     Error.captureStackTrace(obj);
                    //     obj.stack[0].getFileName();
                    // Build a 1-element framesArray with a stub CallSite that
                    // answers getFileName/getLineNumber/etc with placeholders.
                    let prepare = rt
                        .globals
                        .get("Error")
                        .and_then(|v| {
                            if let Value::Object(eid) = v {
                                Some(*eid)
                            } else {
                                None
                            }
                        })
                        .map(|eid| rt.object_get(eid, "prepareStackTrace"));
                    if let Some(Value::Object(_)) = &prepare {
                        let call_site = rt.alloc_object(crate::value::Object::new_ordinary());
                        register_method(rt, call_site, "getFileName", |_rt, _a| {
                            Ok(Value::String(Rc::new("<native>".into())))
                        });
                        register_method(rt, call_site, "getLineNumber", |_rt, _a| {
                            Ok(Value::Number(0.0))
                        });
                        register_method(rt, call_site, "getColumnNumber", |_rt, _a| {
                            Ok(Value::Number(0.0))
                        });
                        register_method(rt, call_site, "getFunctionName", |_rt, _a| {
                            Ok(Value::String(Rc::new("<anonymous>".into())))
                        });
                        register_method(rt, call_site, "getMethodName", |_rt, _a| Ok(Value::Null));
                        register_method(rt, call_site, "getTypeName", |_rt, _a| {
                            Ok(Value::String(Rc::new("<anonymous>".into())))
                        });
                        register_method(rt, call_site, "isNative", |_rt, _a| {
                            Ok(Value::Boolean(true))
                        });
                        register_method(rt, call_site, "isConstructor", |_rt, _a| {
                            Ok(Value::Boolean(false))
                        });
                        register_method(rt, call_site, "isToplevel", |_rt, _a| {
                            Ok(Value::Boolean(true))
                        });
                        register_method(rt, call_site, "isEval", |_rt, _a| {
                            Ok(Value::Boolean(false))
                        });
                        // Build a small stack of stub frames so consumers
                        // doing `callSites.slice(1)[0]` (depd / err-stack)
                        // still find a defined CallSite.
                        let frames = rt.alloc_object(crate::value::Object::new_array());
                        for i in 0..6 {
                            rt.object_set(frames, i.to_string(), Value::Object(call_site));
                        }
                        rt.object_set(frames, "length".into(), Value::Number(6.0));
                        let result = rt.call_function(
                            prepare.unwrap(),
                            Value::Undefined,
                            vec![Value::Object(*target), Value::Object(frames)],
                        )?;
                        rt.object_set(*target, "stack".into(), result);
                    } else {
                        rt.object_set(*target, "stack".into(), Value::String(Rc::new("".into())));
                    }
                }
                Ok(Value::Undefined)
            });
            // Error.stackTraceLimit — Node default is 10; consumers occasionally
            // probe `Error.stackTraceLimit = Infinity` then set back.
            self.object_set(ctor_id, "stackTraceLimit".into(), Value::Number(10.0));
            self.globals
                .insert((*name).to_string(), Value::Object(ctor_id));
        }
        // Chain Error-subclass prototypes through Error.prototype per
        // ECMA-262 §20.5.6 (each NativeError.prototype's [[Prototype]]
        // is %Error.prototype%). Without this, `e instanceof Error` is
        // false even when e is a TypeError / RangeError / etc.
        let err_proto_id = match self.globals.get("Error").cloned() {
            Some(Value::Object(eid)) => match self.object_get(eid, "prototype") {
                Value::Object(pid) => Some(pid),
                _ => None,
            },
            _ => None,
        };
        if let Some(epid) = err_proto_id {
            for sub_name in &[
                "TypeError",
                "RangeError",
                "SyntaxError",
                "ReferenceError",
                "URIError",
                "EvalError",
                "AggregateError",
            ] {
                if let Some(Value::Object(sid)) = self.globals.get(*sub_name).cloned() {
                    if let Value::Object(spid) = self.object_get(sid, "prototype") {
                        self.obj_mut(spid).proto = Some(epid);
                    }
                }
            }
        }
    }

    fn install_symbol_static(&mut self) {
        // Tier-Ω.5.w: Symbol is now callable as `Symbol(desc?)`. Returns a
        // fresh Value::String of the form "@@sym:<counter>:<desc>" — the
        // counter is appended via a thread_local AtomicUsize so two calls
        // with the same description produce distinct strings (sufficient
        // for the spec's identity-distinct expectation under v1's
        // string-shaped Symbol representation).
        // Ω.5.P63.E51: Symbol ctor — invoked-with-new TypeError per §20.4.1.1
        // step 1; description coercion via OrdinaryToPrimitive (string hint)
        // so Symbol(symbol_val) throws and Symbol(obj_with_throwing_toString)
        // propagates correctly. undefined description → undefined (not empty
        // string) so that .description observation returns undefined.
        let sym_obj = make_native("Symbol", |rt, args| {
            if rt.current_new_target.is_some() {
                return Err(RuntimeError::TypeError(
                    "Symbol is not a constructor".into(),
                ));
            }
            use std::sync::atomic::{AtomicUsize, Ordering};
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let (desc_part, has_desc) = match args.first() {
                None | Some(Value::Undefined) => (String::new(), false),
                Some(v) => (rt.to_string_strict(v)?, true),
            };
            // Encode description presence into the symbol identifier: with-desc
            // uses `@@sym:<n>:<desc>`, without-desc uses `@@sym:<n>` so the
            // .description getter and to_string_via can distinguish.
            let s = if has_desc {
                format!("@@sym:{}:{}", n, desc_part)
            } else {
                format!("@@sym:{}", n)
            };
            Ok(Value::Symbol(Rc::new(s)))
        });
        let sym = self.alloc_object(sym_obj);
        // Ω.5.P59.E1: well-known symbols are real Value::Symbol values now
        // per ECMA §6.1.5 + §20.4.2. Pre-P59.E1 they were Value::String
        // sentinels — typeof Symbol.iterator returned "string" not
        // "symbol", and Symbol === checks against globals failed.
        // The string content ("@@iterator" etc.) is preserved so that
        // `obj[Symbol.iterator]` continues to resolve to the same string
        // key — property_key (interp.rs:1967) coerces Value::Symbol via
        // abstract_ops::to_string, which returns the inner string. Every
        // existing iterator-protocol callsite that registers
        // `obj["@@iterator"]` as a method continues to work unchanged.
        // The visible behavior change: typeof Symbol.X === "symbol",
        // `Symbol.iterator === Symbol.iterator` (Rc::ptr_eq-based when
        // the same Rc is reused; canonicalize_well_known_symbol below
        // pre-allocates the Rc per global so identity is stable).
        // Closes Doc 729 §XII Axis-S residuals: async-iterator-to-stream
        // (sole surviving Symbol-typeof case at canonical scale), zod
        // $brand pattern at deeper scope, has-tostringtag dispatch.
        let well_known: &[(&str, &str)] = &[
            ("iterator", "@@iterator"),
            ("asyncIterator", "@@asyncIterator"),
            ("hasInstance", "@@hasInstance"),
            ("toPrimitive", "@@toPrimitive"),
            ("toStringTag", "@@toStringTag"),
            ("isConcatSpreadable", "@@isConcatSpreadable"),
            ("species", "@@species"),
            ("match", "@@match"),
            ("matchAll", "@@matchAll"),
            ("replace", "@@replace"),
            ("search", "@@search"),
            ("split", "@@split"),
            ("unscopables", "@@unscopables"),
            ("dispose", "@@dispose"),
            ("asyncDispose", "@@asyncDispose"),
        ];
        // Ω.5.P63.E51: well-known symbols are frozen ({w:false, e:false,
        // c:false}) per §20.4.2 — closes 15-test prop-desc cluster.
        for &(name, sym_str) in well_known {
            self.obj_mut(sym)
                .set_own_frozen(name.into(), Value::Symbol(Rc::new(sym_str.to_string())));
        }
        register_intrinsic_method(self, sym, "for", 1, |rt, args| {
            crate::generated::symbol_for(rt, rt.current_this(), args)
        });
        register_intrinsic_method(self, sym, "keyFor", 1, |rt, args| {
            crate::generated::symbol_key_for(rt, rt.current_this(), args)
        });
        // Tier-Ω.5.wwww: Symbol.prototype with a toString that returns the
        // description. yup captures Symbol.prototype.toString at module init.
        let sym_proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, sym_proto, "toString", 0, |rt, args| {
            crate::generated::symbol_prototype_to_string(rt, rt.current_this(), args)
        });
        // Symbol.prototype.valueOf per §20.4.3.4 — returns the symbol primitive.
        register_intrinsic_method(self, sym_proto, "valueOf", 0, |rt, _args| {
            let this = rt.current_this();
            let t = rt.unwrap_primitive(&this);
            match t {
                Value::Symbol(s) => Ok(Value::Symbol(s)),
                _ => Err(RuntimeError::TypeError(
                    "Symbol.prototype.valueOf: this is not a Symbol".into(),
                )),
            }
        });
        // Symbol.prototype.description getter (data property in v1 — most
        // consumers read it as a plain prop).
        let desc_fn = make_native("get description", |rt, _args| {
            let t = rt.unwrap_primitive(&rt.current_this());
            let s = match t {
                Value::Symbol(s) => s,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Symbol.prototype.description: this is not a Symbol".into(),
                    ))
                }
            };
            // Encoded forms:
            //   "@@sym:<n>"          → no description (returns undefined)
            //   "@@sym:<n>:<desc>"   → description = <desc>
            //   "@@sym:<key>"        → registry symbol (Symbol.for); description = <key>
            let body = s.strip_prefix("@@sym:").unwrap_or(&s);
            let starts_with_digit = body
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false);
            if starts_with_digit {
                match body.split_once(':') {
                    Some((_, d)) => Ok(Value::String(Rc::new(d.to_string()))),
                    None => Ok(Value::Undefined),
                }
            } else {
                Ok(Value::String(Rc::new(body.to_string())))
            }
        });
        let desc_id = self.alloc_object(desc_fn);
        self.obj_mut(sym_proto).dict_mut().insert(
            "description".into(),
            crate::value::PropertyDescriptor {
                value: Value::Undefined,
                writable: false,
                enumerable: false,
                configurable: true,
                getter: Some(Value::Object(desc_id)),
                setter: None,
            },
        );
        // Symbol.prototype[@@toPrimitive] per §20.4.3.5 — ignore hint, return
        // [[SymbolData]] (unwrap primitive). Installed under the well-known
        // string key "@@toPrimitive"; brand-check rejects non-Symbol receivers.
        register_intrinsic_method(self, sym_proto, "@@toPrimitive", 0, |rt, _args| {
            let t = rt.unwrap_primitive(&rt.current_this());
            match t {
                Value::Symbol(s) => Ok(Value::Symbol(s)),
                _ => Err(RuntimeError::TypeError(
                    "Symbol.prototype[@@toPrimitive]: this is not a Symbol".into(),
                )),
            }
        });
        // Symbol.prototype[@@toStringTag] = "Symbol" per §20.4.3.6.
        self.obj_mut(sym_proto).set_own_frozen(
            "@@toStringTag".into(),
            Value::String(Rc::new("Symbol".into())),
        );
        self.obj_mut(sym)
            .set_own_frozen("prototype".into(), Value::Object(sym_proto));
        // Symbol.prototype.constructor = Symbol.
        self.obj_mut(sym_proto)
            .set_own_internal("constructor".into(), Value::Object(sym));
        self.globals.insert("Symbol".into(), Value::Object(sym));
        self.symbol_prototype = Some(sym_proto);
    }

    fn install_console(&mut self) {
        let console = self.alloc_object(Object::new_ordinary());
        register_method(self, console, "log", |rt, args| {
            let out = console_format(rt, args);
            check_stdio(rt, crate::caps::StdioOp::Stdout(out.as_bytes().to_vec()))?;
            println!("{}", out);
            Ok(Value::Undefined)
        });
        // CAPS-EXT 10: console.error and console.warn write to stderr,
        // which remains ungated this round. stderr is the probe-harness
        // escape valve for LOSES sentinels under --sealed; gating it
        // here would block the harness from observing capability errors.
        register_method(self, console, "error", |rt, args| {
            let out = console_format(rt, args);
            eprintln!("{}", out);
            Ok(Value::Undefined)
        });
        register_method(self, console, "warn", |rt, args| {
            let out = console_format(rt, args);
            eprintln!("{}", out);
            Ok(Value::Undefined)
        });
        self.globals
            .insert("console".into(), Value::Object(console));
    }

    // diff-prod Rung-19 continuation: Iterator Helpers (ES2025) +
    // Map.groupBy / Promise.try / Error.isError surface (ES2023–26).
    //
    // Iterator helpers are eager-consuming over finite iterators. Lazy
    // iterators (infinite generators threaded through .map/.take) require
    // lazy generator semantics (frame park/resume) deferred per Rung-9.
    // Each non-terminal helper drains the underlying iterator via .next()
    // calls into a Vec<Value>, then returns a fresh array-iterator chained
    // to iterator_prototype so further helpers compose. Terminal helpers
    // (reduce/forEach/some/every/find/toArray) consume directly.
    fn install_iterator_helpers_and_recent_methods(&mut self) {
        // ── Iterator Helpers ──────────────────────────────────────────────
        let iter_proto = match self.iterator_prototype {
            Some(id) => id,
            None => return, // install_prototypes didn't run; shouldn't happen
        };

        fn drain_iterator(rt: &mut Runtime, this: Value) -> Result<Vec<Value>, RuntimeError> {
            let it = match this {
                Value::Object(id) => id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Iterator helper: receiver is not an iterator".into(),
                    ))
                }
            };
            let next = rt.object_get(it, "next");
            if !rt.is_callable(&next) {
                return Err(RuntimeError::TypeError(
                    "Iterator helper: receiver has no callable 'next' method".into(),
                ));
            }
            let mut out = Vec::new();
            loop {
                let step = rt.call_function(next.clone(), Value::Object(it), Vec::new())?;
                let step_id = match step {
                    Value::Object(id) => id,
                    _ => break,
                };
                if matches!(rt.object_get(step_id, "done"), Value::Boolean(true)) {
                    break;
                }
                out.push(rt.object_get(step_id, "value"));
                if out.len() > 10_000_000 {
                    return Err(RuntimeError::RangeError(
                        "Iterator helper: result exceeds 10M elements".into(),
                    ));
                }
            }
            Ok(out)
        }

        fn make_array_iterator(
            rt: &mut Runtime,
            iter_proto: ObjectRef,
            items: Vec<Value>,
        ) -> ObjectRef {
            let mut o = Object::new_ordinary();
            o.proto = Some(iter_proto);
            let it = rt.alloc_object(o);
            // Store backing array under __ai_data with explicit length.
            let arr = rt.alloc_object(Object::new_ordinary());
            let n = items.len();
            for (i, v) in items.into_iter().enumerate() {
                rt.object_set(arr, i.to_string(), v);
            }
            rt.object_set(arr, "length".into(), Value::Number(n as f64));
            rt.obj_mut(it)
                .set_own_internal("__ai_data".into(), Value::Object(arr));
            rt.obj_mut(it)
                .set_own_internal("__ai_idx".into(), Value::Number(0.0));
            // Install own `next` method dispatching against __ai_data/__ai_idx.
            let next_fn = make_native("next", |rt, _args| {
                let this_id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Ok(Value::Undefined),
                };
                let arr = match rt.object_get(this_id, "__ai_data") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Undefined),
                };
                let idx = match rt.object_get(this_id, "__ai_idx") {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };
                let len = match rt.object_get(arr, "length") {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };
                let mut o = Object::new_ordinary();
                if idx >= len {
                    o.set_own("value".into(), Value::Undefined);
                    o.set_own("done".into(), Value::Boolean(true));
                } else {
                    let v = rt.object_get(arr, &idx.to_string());
                    rt.obj_mut(this_id)
                        .set_own_internal("__ai_idx".into(), Value::Number((idx + 1) as f64));
                    o.set_own("value".into(), v);
                    o.set_own("done".into(), Value::Boolean(false));
                }
                Ok(Value::Object(rt.alloc_object(o)))
            });
            let next_id = rt.alloc_object(next_fn);
            rt.object_set(it, "next".into(), Value::Object(next_id));
            it
        }

        let ip_for_helpers = iter_proto;

        register_method(self, iter_proto, "map", move |rt, args| {
            let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&fn_v) {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.map: callback is not callable".into(),
                ));
            }
            let items = drain_iterator(rt, rt.current_this())?;
            let mut out = Vec::with_capacity(items.len());
            for v in items {
                out.push(rt.call_function(fn_v.clone(), Value::Undefined, vec![v])?);
            }
            Ok(Value::Object(make_array_iterator(rt, ip_for_helpers, out)))
        });
        register_method(self, iter_proto, "filter", move |rt, args| {
            let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&fn_v) {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.filter: callback is not callable".into(),
                ));
            }
            let items = drain_iterator(rt, rt.current_this())?;
            let mut out = Vec::new();
            for v in items {
                let keep = rt.call_function(fn_v.clone(), Value::Undefined, vec![v.clone()])?;
                if abstract_ops::to_boolean(&keep) {
                    out.push(v);
                }
            }
            Ok(Value::Object(make_array_iterator(rt, ip_for_helpers, out)))
        });
        register_method(self, iter_proto, "take", move |rt, args| {
            let n = match args.first() {
                Some(Value::Number(n)) if *n >= 0.0 => *n as usize,
                _ => 0,
            };
            let items = drain_iterator(rt, rt.current_this())?;
            let out: Vec<Value> = items.into_iter().take(n).collect();
            Ok(Value::Object(make_array_iterator(rt, ip_for_helpers, out)))
        });
        register_method(self, iter_proto, "drop", move |rt, args| {
            let n = match args.first() {
                Some(Value::Number(n)) if *n >= 0.0 => *n as usize,
                _ => 0,
            };
            let items = drain_iterator(rt, rt.current_this())?;
            let out: Vec<Value> = items.into_iter().skip(n).collect();
            Ok(Value::Object(make_array_iterator(rt, ip_for_helpers, out)))
        });
        register_method(self, iter_proto, "flatMap", move |rt, args| {
            let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&fn_v) {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.flatMap: callback is not callable".into(),
                ));
            }
            let items = drain_iterator(rt, rt.current_this())?;
            let mut out = Vec::new();
            for v in items {
                let mapped = rt.call_function(fn_v.clone(), Value::Undefined, vec![v])?;
                // mapped is expected to be iterable. Handle Array-shape + Iterator-shape.
                if let Value::Object(id) = mapped {
                    // Array-shape: walk length.
                    let len = rt.array_length(id);
                    if len > 0 || matches!(rt.object_get(id, "length"), Value::Number(_)) {
                        for i in 0..len {
                            out.push(rt.object_get(id, &i.to_string()));
                        }
                    } else {
                        // Iterator-shape: drain via .next().
                        out.extend(drain_iterator(rt, Value::Object(id))?);
                    }
                }
            }
            Ok(Value::Object(make_array_iterator(rt, ip_for_helpers, out)))
        });

        // Terminal helpers — consume only.
        register_method(self, iter_proto, "toArray", |rt, _args| {
            let items = drain_iterator(rt, rt.current_this())?;
            let arr = rt.alloc_object(crate::value::Object::new_array());
            for (i, v) in items.iter().enumerate() {
                rt.object_set(arr, i.to_string(), v.clone());
            }
            rt.object_set(arr, "length".into(), Value::Number(items.len() as f64));
            Ok(Value::Object(arr))
        });
        register_method(self, iter_proto, "reduce", |rt, args| {
            let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&fn_v) {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.reduce: callback is not callable".into(),
                ));
            }
            let items = drain_iterator(rt, rt.current_this())?;
            let (start_idx, mut acc) = if args.len() >= 2 {
                (0usize, args[1].clone())
            } else if !items.is_empty() {
                (1usize, items[0].clone())
            } else {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.reduce: empty iterator with no initial value".into(),
                ));
            };
            for v in items.into_iter().skip(start_idx) {
                acc = rt.call_function(fn_v.clone(), Value::Undefined, vec![acc, v])?;
            }
            Ok(acc)
        });
        register_method(self, iter_proto, "forEach", |rt, args| {
            let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&fn_v) {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.forEach: callback is not callable".into(),
                ));
            }
            let items = drain_iterator(rt, rt.current_this())?;
            for v in items {
                let _ = rt.call_function(fn_v.clone(), Value::Undefined, vec![v])?;
            }
            Ok(Value::Undefined)
        });
        register_method(self, iter_proto, "some", |rt, args| {
            let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&fn_v) {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.some: callback is not callable".into(),
                ));
            }
            let items = drain_iterator(rt, rt.current_this())?;
            for v in items {
                let r = rt.call_function(fn_v.clone(), Value::Undefined, vec![v])?;
                if abstract_ops::to_boolean(&r) {
                    return Ok(Value::Boolean(true));
                }
            }
            Ok(Value::Boolean(false))
        });
        register_method(self, iter_proto, "every", |rt, args| {
            let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&fn_v) {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.every: callback is not callable".into(),
                ));
            }
            let items = drain_iterator(rt, rt.current_this())?;
            for v in items {
                let r = rt.call_function(fn_v.clone(), Value::Undefined, vec![v])?;
                if !abstract_ops::to_boolean(&r) {
                    return Ok(Value::Boolean(false));
                }
            }
            Ok(Value::Boolean(true))
        });
        register_method(self, iter_proto, "find", |rt, args| {
            let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
            if !rt.is_callable(&fn_v) {
                return Err(RuntimeError::TypeError(
                    "Iterator.prototype.find: callback is not callable".into(),
                ));
            }
            let items = drain_iterator(rt, rt.current_this())?;
            for v in items {
                let r = rt.call_function(fn_v.clone(), Value::Undefined, vec![v.clone()])?;
                if abstract_ops::to_boolean(&r) {
                    return Ok(v);
                }
            }
            Ok(Value::Undefined)
        });

        // Iterator global with .from static.
        let iter_ctor = make_native("Iterator", |_rt, _args| {
            Err(RuntimeError::TypeError(
                "Iterator constructor is abstract; use Iterator.from(iterable)".into(),
            ))
        });
        let iter_id = self.alloc_object(iter_ctor);
        self.obj_mut(iter_id)
            .set_own_frozen("prototype".into(), Value::Object(iter_proto));
        self.obj_mut(iter_proto)
            .set_own_internal("constructor".into(), Value::Object(iter_id));
        let ip_for_from = iter_proto;
        register_method(self, iter_id, "from", move |rt, args| {
            let arg = args.first().cloned().unwrap_or(Value::Undefined);
            let obj = match arg {
                Value::Object(id) => id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Iterator.from: argument is not iterable".into(),
                    ))
                }
            };
            // If obj is itself an iterator (has callable .next), drain directly.
            let next_v = rt.object_get(obj, "next");
            let inner = if rt.is_callable(&next_v) {
                obj
            } else {
                // Look up @@iterator and call it to get an iterator.
                let it_method = rt.object_get(obj, "@@iterator");
                if !rt.is_callable(&it_method) {
                    return Err(RuntimeError::TypeError(
                        "Iterator.from: argument is not iterable (no @@iterator)".into(),
                    ));
                }
                match rt.call_function(it_method, Value::Object(obj), Vec::new())? {
                    Value::Object(id) => id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Iterator.from: @@iterator did not return an object".into(),
                        ))
                    }
                }
            };
            let items = drain_iterator(rt, Value::Object(inner))?;
            Ok(Value::Object(make_array_iterator(rt, ip_for_from, items)))
        });
        self.globals
            .insert("Iterator".into(), Value::Object(iter_id));

        // ── Map.groupBy ───────────────────────────────────────────────────
        // §24.1.2.2: like Object.groupBy but returns a Map. Iterate items,
        // call callback for each, accumulate into a Map keyed by callback
        // return (SameValueZero — but our Map uses ToString-keys, so v1
        // matches Object.groupBy's string-key behavior with the same caveat).
        if let Some(Value::Object(map_ctor)) = self.globals.get("Map").cloned() {
            let mc = map_ctor;
            register_intrinsic_method(self, map_ctor, "groupBy", 2, move |rt, args| {
                let map_ctor = mc;
                let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                let cb = args.get(1).cloned().unwrap_or(Value::Undefined);
                if !rt.is_callable(&cb) {
                    return Err(RuntimeError::TypeError(
                        "Map.groupBy: callback is not callable".into(),
                    ));
                }
                // Drain the iterable. Array-shape primary, iterator-shape fallback.
                let items: Vec<Value> = match iterable {
                    Value::Object(id) => {
                        let len = rt.array_length(id);
                        if len > 0 || matches!(rt.object_get(id, "length"), Value::Number(_)) {
                            (0..len)
                                .map(|i| rt.object_get(id, &i.to_string()))
                                .collect()
                        } else {
                            let it_m = rt.object_get(id, "@@iterator");
                            if rt.is_callable(&it_m) {
                                let it = rt.call_function(it_m, Value::Object(id), Vec::new())?;
                                drain_iterator(rt, it)?
                            } else {
                                return Err(RuntimeError::TypeError(
                                    "Map.groupBy: argument is not iterable".into(),
                                ));
                            }
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Map.groupBy: argument is not iterable".into(),
                        ))
                    }
                };
                // Construct a new Map via the ctor so it's properly wired.
                let new_map =
                    rt.call_function(Value::Object(map_ctor), Value::Undefined, Vec::new())?;
                let map_id = match new_map {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Undefined),
                };
                let storage = match rt.object_get(map_id, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Object(map_id)),
                };
                // Group items by ToString(cb(item)). Within each bucket, append.
                for v in items {
                    let key_v = rt.call_function(cb.clone(), Value::Undefined, vec![v.clone()])?;
                    let key_s = abstract_ops::to_string(&key_v).as_str().to_string();
                    // Get or create the bucket array.
                    let bucket_id = match rt.object_get(storage, &key_s) {
                        Value::Object(id) => id,
                        _ => {
                            let arr = rt.alloc_object(crate::value::Object::new_array());
                            rt.object_set(arr, "length".into(), Value::Number(0.0));
                            rt.object_set(storage, key_s.clone(), Value::Object(arr));
                            arr
                        }
                    };
                    let len = rt.array_length(bucket_id);
                    rt.object_set(bucket_id, len.to_string(), v);
                    rt.object_set(bucket_id, "length".into(), Value::Number((len + 1) as f64));
                }
                // Refresh size.
                let count = rt.obj(storage).properties.len() as f64;
                rt.object_set(map_id, "size".into(), Value::Number(count));
                Ok(Value::Object(map_id))
            });
        }

        // ── Promise.try ────────────────────────────────────────────────────
        // ES2026 stage 4 §27.2.4.x: Promise.try(fn, ...args). Sync-invokes fn,
        // wraps return value in a resolved promise; catches sync throws into
        // a rejected promise. Async returns flow through promise_resolve_via
        // which handles thenable unwrapping.
        if let Some(Value::Object(promise_ctor)) = self.globals.get("Promise").cloned() {
            register_intrinsic_method(self, promise_ctor, "try", 1, |rt, args| {
                let fn_v = args.first().cloned().unwrap_or(Value::Undefined);
                if !rt.is_callable(&fn_v) {
                    return Err(RuntimeError::TypeError(
                        "Promise.try: callback is not callable".into(),
                    ));
                }
                let rest: Vec<Value> = args.iter().skip(1).cloned().collect();
                match rt.call_function(fn_v, Value::Undefined, rest) {
                    Ok(v) => rt.promise_resolve_via(&v),
                    Err(RuntimeError::Thrown(v)) => rt.promise_reject_via(&v),
                    Err(e) => Err(e),
                }
            });
        }

        // ── Error.isError ─────────────────────────────────────────────────
        // ES2025 §20.5.x: returns true iff argument has an [[ErrorData]]
        // internal slot. cruftless marks Error instances via the
        // %Error.prototype% chain; the proto-chain walk is the durable
        // discriminator. Plain {message: "x"} objects without the chain
        // return false per spec.
        if let Some(Value::Object(error_ctor)) = self.globals.get("Error").cloned() {
            let err_proto_v = self.object_get(error_ctor, "prototype");
            let err_proto = if let Value::Object(id) = err_proto_v {
                Some(id)
            } else {
                None
            };
            register_intrinsic_method(self, error_ctor, "isError", 1, move |rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                let id = match v {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Boolean(false)),
                };
                // Walk the proto chain looking for Error.prototype.
                let target = match err_proto {
                    Some(p) => p,
                    None => return Ok(Value::Boolean(false)),
                };
                let mut cur = rt.obj(id).proto;
                while let Some(p) = cur {
                    if p == target {
                        return Ok(Value::Boolean(true));
                    }
                    cur = rt.obj(p).proto;
                }
                Ok(Value::Boolean(false))
            });
        }
    }
}

/// Drain an iterable's @@iterator into a Vec<Value>. Used by
/// Object.fromEntries / Array.from.
/// Tier-Ω.5.rrr: build a values-iterator for a Set. The iterator object
/// snapshots the Set's current values into a private array and exposes a
/// next() that yields each in turn. Sufficient for `[...new Set(arr)]`
/// spread.
pub(crate) fn make_set_values_iterator(
    rt: &mut Runtime,
    set_id: crate::value::ObjectRef,
) -> Result<Value, RuntimeError> {
    let values: Vec<Value> = match rt.object_get(set_id, "__set_data") {
        Value::Object(storage) => rt
            .obj(storage)
            .properties
            .values()
            .map(|d| d.value.clone())
            .collect(),
        _ => {
            return Err(RuntimeError::TypeError(
                "Set.prototype method: this is not a Set object".into(),
            ))
        }
    };
    // Build an iterator object: { __idx: 0, __vals: [v0,v1,...], next() }
    let iter = rt.alloc_object(Object::new_ordinary());
    let vals_arr = rt.alloc_object(Object::new_array());
    for (i, v) in values.iter().enumerate() {
        rt.object_set(vals_arr, i.to_string(), v.clone());
    }
    rt.object_set(
        vals_arr,
        "length".into(),
        Value::Number(values.len() as f64),
    );
    // ESNE-EXT 3: hide engine sentinels per CLAUDE.md __X convention.
    rt.set_engine_sentinel(iter, "__vals", Value::Object(vals_arr));
    rt.set_engine_sentinel(iter, "__idx", Value::Number(0.0));
    register_intrinsic_method(rt, iter, "next", 1, |rt, _args| {
        let this = match rt.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::Undefined),
        };
        let idx = match rt.object_get(this, "__idx") {
            Value::Number(n) => n as usize,
            _ => 0,
        };
        let vals = match rt.object_get(this, "__vals") {
            Value::Object(id) => id,
            _ => return Ok(Value::Undefined),
        };
        let len = rt.array_length(vals);
        let result = rt.alloc_object(Object::new_ordinary());
        if idx >= len {
            rt.object_set(result, "done".into(), Value::Boolean(true));
            rt.object_set(result, "value".into(), Value::Undefined);
        } else {
            let v = rt.object_get(vals, &idx.to_string());
            rt.object_set(result, "done".into(), Value::Boolean(false));
            rt.object_set(result, "value".into(), v);
            rt.object_set(this, "__idx".into(), Value::Number((idx + 1) as f64));
        }
        Ok(Value::Object(result))
    });
    Ok(Value::Object(iter))
}

pub(crate) fn collect_iterable(rt: &mut Runtime, src: Value) -> Result<Vec<Value>, RuntimeError> {
    // IPTO-EXT 1: ECMA-262 §7.3.20 GetIterator(obj). Property access
    // `obj[Symbol.iterator]` ToObject-wraps primitives implicitly; cruft's
    // pre-fix non-Object short-circuit returned an empty Vec, silently
    // dropping iteration on strings (e.g. [..."abc"] gave []). undefined
    // and null still error per spec. Other primitives go through to_object
    // (String -> StringWrapper which has @@iterator on String.prototype;
    // Number/Boolean/BigInt/Symbol wrap to objects with no @@iterator and
    // hit the existing "iterator is not an object" TypeError downstream).
    let id = match src {
        Value::Object(id) => id,
        Value::Undefined | Value::Null => {
            return Err(RuntimeError::TypeError(
                "iterable: cannot iterate undefined or null".into(),
            ));
        }
        ref other => match rt.to_object(other)? {
            Value::Object(id) => id,
            _ => return Ok(Vec::new()),
        },
    };
    let method = rt.object_get(id, "@@iterator");
    let iter = rt.call_function(method, Value::Object(id), Vec::new())?;
    let iter_id = match iter {
        Value::Object(id) => id,
        _ => return Err(RuntimeError::TypeError("iterator is not an object".into())),
    };
    let next = rt.object_get(iter_id, "next");
    let mut out = Vec::new();
    loop {
        let result = rt.call_function(next.clone(), Value::Object(iter_id), Vec::new())?;
        let rid = match result {
            Value::Object(id) => id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "iterator next did not return an object".into(),
                ))
            }
        };
        let done = abstract_ops::to_boolean(&rt.object_get(rid, "done"));
        if done {
            break;
        }
        out.push(rt.object_get(rid, "value"));
    }
    Ok(out)
}

fn ta_iter_next(rt: &mut Runtime) -> Result<Value, RuntimeError> {
    let this_id = match rt.current_this() {
        Value::Object(o) => o,
        _ => return Ok(Value::Undefined),
    };
    let src = match rt.object_get(this_id, "__it_src__") {
        Value::Object(id) => id,
        _ => return Ok(Value::Undefined),
    };
    let idx = match rt.object_get(this_id, "__it_idx__") {
        Value::Number(n) => n as usize,
        _ => 0,
    };
    let mode = match rt.object_get(this_id, "__it_mode__") {
        Value::String(s) => s.as_str().to_string(),
        _ => "value".to_string(),
    };
    if rt.typed_array_view_out_of_bounds(src) {
        return Err(RuntimeError::TypeError(
            "TypedArray iterator receiver is out of bounds".into(),
        ));
    }
    let len = match rt.object_get(src, "length") {
        Value::Number(n) => n as usize,
        _ => 0,
    };
    let mut o = Object::new_ordinary();
    if idx >= len {
        o.set_own("value".into(), Value::Undefined);
        o.set_own("done".into(), Value::Boolean(true));
    } else {
        let v = rt.object_get(src, &idx.to_string());
        let yielded = match mode.as_str() {
            "key" => Value::Number(idx as f64),
            "entry" => {
                let pair = rt.alloc_object(Object::new_array());
                rt.object_set(pair, "0".into(), Value::Number(idx as f64));
                rt.object_set(pair, "1".into(), v);
                rt.object_set(pair, "length".into(), Value::Number(2.0));
                Value::Object(pair)
            }
            _ => v,
        };
        rt.object_set(
            this_id,
            "__it_idx__".into(),
            Value::Number((idx + 1) as f64),
        );
        o.set_own("value".into(), yielded);
        o.set_own("done".into(), Value::Boolean(false));
    }
    Ok(Value::Object(rt.alloc_object(o)))
}

fn num_arg(args: &[Value], i: usize) -> f64 {
    args.get(i).map(abstract_ops::to_number).unwrap_or(f64::NAN)
}

/// Ω.5.P51.E5: render a RuntimeError for diagnostic display when an Error
/// thrown at module-init bubbles out of dynamic import. Thrown(Object) values
/// — typically Error instances — get their .name + .message extracted so the
/// dynamic-import wrapper's diagnostic carries the original cause text. Other
/// thrown shapes (primitives, non-Error objects) fall back to Debug format.
/// Ω.5.P58.E5: construct a {name, message, stack} ordinary object whose
/// [[Prototype]] is `globalThis[ctor_name].prototype`. Returns None if
/// the named constructor isn't installed yet (early-bootstrap edge).
/// Used by the dynamic-import reject path so promise rejections carry
/// real Error-instance shape rather than a raw string.
pub(crate) fn make_error_instance(
    rt: &mut Runtime,
    ctor_name: &str,
    message: &str,
) -> Option<rusty_js_gc::ObjectId> {
    let ctor_id = match rt.globals.get(ctor_name).cloned() {
        Some(Value::Object(id)) => id,
        _ => return None,
    };
    let proto = match rt.object_get(ctor_id, "prototype") {
        Value::Object(id) => Some(id),
        _ => None,
    };
    let mut o = Object::new_ordinary();
    o.proto = proto;
    o.set_own("name".into(), Value::String(Rc::new(ctor_name.to_string())));
    o.set_own(
        "message".into(),
        Value::String(Rc::new(message.to_string())),
    );
    o.set_own("stack".into(), Value::String(Rc::new(String::new())));
    Some(rt.alloc_object(o))
}

/// Ω.5.P59.E6: allocate a same-kind TypedArray-like instance from a
/// source TypedArray, used by .map / .filter to satisfy ECMA §23.2.3.21
/// TypedArraySpeciesCreate semantics at the shape level (length +
/// byteLength + __kind sentinel + proto inheritance from source).
fn make_typed_array_like(
    rt: &mut Runtime,
    src: rusty_js_gc::ObjectId,
    len: usize,
) -> rusty_js_gc::ObjectId {
    let src_kind = match rt.object_get(src, "__kind") {
        Value::String(s) | Value::Symbol(s) => (*s).clone(),
        _ => "Uint8Array".into(),
    };
    let src_proto = rt.obj(src).proto;
    let mut o = Object::new_ordinary();
    o.proto = src_proto;
    o.set_own("length".into(), Value::Number(len as f64));
    // byteLength approximation: same per-element width as source.
    let src_byte_len = match rt.object_get(src, "byteLength") {
        Value::Number(n) => n,
        _ => 0.0,
    };
    let src_len = match rt.object_get(src, "length") {
        Value::Number(n) => n,
        _ => 1.0,
    };
    let bpe = if src_len > 0.0 {
        src_byte_len / src_len
    } else {
        1.0
    };
    o.set_own("byteLength".into(), Value::Number(len as f64 * bpe));
    o.set_own_internal("__kind".into(), Value::String(Rc::new(src_kind)));
    rt.alloc_object(o)
}

fn describe_thrown_for_diag(rt: &Runtime, e: &RuntimeError) -> String {
    match e {
        RuntimeError::Thrown(v) => match v {
            Value::Object(id) => {
                let name = rt.object_get(*id, "name");
                let msg = rt.object_get(*id, "message");
                let stack = rt.object_get(*id, "stack");
                match (name, msg, stack) {
                    (Value::String(n), Value::String(m), _) => format!("{}: {}", n, m),
                    (_, Value::String(m), _) => (*m).to_string(),
                    (_, _, Value::String(s)) => (*s).to_string(),
                    _ => format!("{:?}", e),
                }
            }
            Value::String(s) => (*s).to_string(),
            other => format!("{:?}", other),
        },
        RuntimeError::TypeError(m) => format!("TypeError({:?})", m),
        RuntimeError::RangeError(m) => format!("RangeError({:?})", m),
        RuntimeError::ReferenceError(m) => format!("ReferenceError({:?})", m),
        other => format!("{:?}", other),
    }
}

/// WHATWG structuredClone walker. Deep-copies the input, preserving
/// shared-reference identity via a seen-table keyed on source ObjectId.
/// Honors Date / RegExp / Map / Set as special cases; throws on
/// Functions and Symbols (uncloneable per spec).
fn structured_clone_walk(
    rt: &mut Runtime,
    v: &Value,
    seen: &mut std::collections::HashMap<u32, ObjectRef>,
) -> Result<Value, RuntimeError> {
    match v {
        Value::Undefined
        | Value::Null
        | Value::Boolean(_)
        | Value::Number(_)
        | Value::String(_)
        | Value::BigInt(_) => Ok(v.clone()),
        Value::Symbol(_) => Err(RuntimeError::TypeError(
            "structuredClone: Symbol values are not cloneable".into(),
        )),
        Value::Object(oid) => {
            if let Some(dst) = seen.get(&oid.0) {
                return Ok(Value::Object(*dst));
            }
            // Function check.
            if matches!(
                rt.obj(*oid).internal_kind,
                InternalKind::Function(_)
                    | InternalKind::Closure(_)
                    | InternalKind::BoundFunction(_)
            ) {
                return Err(RuntimeError::TypeError(
                    "structuredClone: function values are not cloneable".into(),
                ));
            }
            // Special-case Map.
            if !matches!(rt.object_get(*oid, "__map_data"), Value::Undefined) {
                let dst_id = if let Some(Value::Object(ctor)) = rt.globals.get("Map").cloned() {
                    let proto = match rt.object_get(ctor, "prototype") {
                        Value::Object(pid) => Some(pid),
                        _ => None,
                    };
                    let mut o = Object::new_ordinary();
                    o.proto = proto;
                    let id = rt.alloc_object(o);
                    let storage = rt.alloc_object(Object::new_dictionary());
                    rt.set_engine_sentinel(id, "__map_data", Value::Object(storage));
                    rt.set_engine_sentinel(id, "size", Value::Number(0.0));
                    id
                } else {
                    rt.alloc_object(Object::new_ordinary())
                };
                seen.insert(oid.0, dst_id);
                let src_storage = match rt.object_get(*oid, "__map_data") {
                    Value::Object(s) => s,
                    _ => return Ok(Value::Object(dst_id)),
                };
                let pairs: Vec<(String, Value)> = rt
                    .obj(src_storage)
                    .properties
                    .iter()
                    .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                    .collect();
                let dst_storage = match rt.object_get(dst_id, "__map_data") {
                    Value::Object(s) => s,
                    _ => return Ok(Value::Object(dst_id)),
                };
                let mut size = 0;
                for (k, v) in pairs {
                    let new_v = structured_clone_walk(rt, &v, seen)?;
                    rt.object_set(dst_storage, k, new_v);
                    size += 1;
                }
                rt.object_set(dst_id, "size".into(), Value::Number(size as f64));
                return Ok(Value::Object(dst_id));
            }
            // Special-case Set.
            if !matches!(rt.object_get(*oid, "__set_data"), Value::Undefined) {
                let dst_id = if let Some(Value::Object(ctor)) = rt.globals.get("Set").cloned() {
                    let proto = match rt.object_get(ctor, "prototype") {
                        Value::Object(pid) => Some(pid),
                        _ => None,
                    };
                    let mut o = Object::new_ordinary();
                    o.proto = proto;
                    let id = rt.alloc_object(o);
                    let storage = rt.alloc_object(Object::new_dictionary());
                    rt.set_engine_sentinel(id, "__set_data", Value::Object(storage));
                    rt.set_engine_sentinel(id, "size", Value::Number(0.0));
                    id
                } else {
                    rt.alloc_object(Object::new_ordinary())
                };
                seen.insert(oid.0, dst_id);
                let src_storage = match rt.object_get(*oid, "__set_data") {
                    Value::Object(s) => s,
                    _ => return Ok(Value::Object(dst_id)),
                };
                let entries: Vec<(String, Value)> = rt
                    .obj(src_storage)
                    .properties
                    .iter()
                    .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                    .collect();
                let dst_storage = match rt.object_get(dst_id, "__set_data") {
                    Value::Object(s) => s,
                    _ => return Ok(Value::Object(dst_id)),
                };
                let mut size = 0;
                for (k, v) in entries {
                    let new_v = structured_clone_walk(rt, &v, seen)?;
                    rt.object_set(dst_storage, k, new_v);
                    size += 1;
                }
                rt.object_set(dst_id, "size".into(), Value::Number(size as f64));
                return Ok(Value::Object(dst_id));
            }
            // Date: clone via internal-time slot if recognizable.
            if !matches!(rt.object_get(*oid, "__date_time"), Value::Undefined) {
                let time = rt.object_get(*oid, "__date_time");
                let mut o = Object::new_ordinary();
                o.proto = rt.obj(*oid).proto;
                o.set_own("__date_time".into(), time);
                let dst_id = rt.alloc_object(o);
                seen.insert(oid.0, dst_id);
                return Ok(Value::Object(dst_id));
            }
            // RegExp: clone via source/flags. RIAS-EXT 1 follow-up — detect
            // via internal_kind, not by source/flags property probes. Post-
            // shadow-removal, source/flags live behind prototype accessors;
            // probing returns Undefined and fails the branch. Read from
            // InternalKind::RegExp directly.
            if let InternalKind::RegExp(re) = &rt.obj(*oid).internal_kind {
                let src = Value::String(re.source.clone());
                let flags = Value::String(re.flags.clone());
                if let Some(Value::Object(ctor)) = rt.globals.get("RegExp").cloned() {
                    let prev = rt.pending_new_target.take();
                    rt.pending_new_target = Some(Value::Object(ctor));
                    let r =
                        rt.call_function(Value::Object(ctor), Value::Undefined, vec![src, flags]);
                    rt.pending_new_target = prev;
                    if let Ok(v) = r {
                        if let Value::Object(dst_id) = &v {
                            seen.insert(oid.0, *dst_id);
                        }
                        return Ok(v);
                    }
                }
            }
            // Array.
            let is_arr = matches!(rt.obj(*oid).internal_kind, InternalKind::Array);
            let dst_id = if is_arr {
                rt.alloc_object(Object::new_array())
            } else {
                let mut o = Object::new_ordinary();
                o.proto = rt.obj(*oid).proto;
                rt.alloc_object(o)
            };
            seen.insert(oid.0, dst_id);
            // CMig-EXT 9 Family B: shape entries first (insertion order),
            // then non-@@ string-keyed properties entries.
            let pairs: Vec<(String, Value)> = {
                let src = rt.obj(*oid);
                let mut out: Vec<(String, Value)> = Vec::new();
                if let Some(shape) = src.shape.as_ref() {
                    for (name, slot) in shape.iter_slots() {
                        let idx = slot as usize;
                        if let Some(v) = src.shape_values.get(idx) {
                            out.push((name.to_string(), v.clone()));
                        }
                    }
                }
                out.extend(
                    src.properties
                        .iter()
                        .filter(|(k, _)| !k.as_str().starts_with("@@"))
                        .map(|(k, d)| (k.to_string_content(), d.value.clone())),
                );
                out
            };
            for (k, v) in pairs {
                let new_v = structured_clone_walk(rt, &v, seen)?;
                rt.object_set(dst_id, k, new_v);
            }
            Ok(Value::Object(dst_id))
        }
    }
}

/// RFC 3986 percent-encoding. When keep_reserved=true (encodeURI), the
/// reserved set `; , / ? : @ & = + $` plus mark chars + alphanumerics
/// pass through unchanged; otherwise (encodeURIComponent) only
/// alphanumerics and the mark set `- _ . ! ~ * ' ( )` pass through.
fn uri_percent_encode(s: &str, keep_reserved: bool) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        let keep = (byte as char).is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
            )
            || (keep_reserved
                && matches!(
                    byte,
                    b';' | b',' | b'/' | b'?' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b'#'
                ));
        if keep {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{:02X}", byte));
        }
    }
    out
}

fn uri_percent_decode(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if i + 2 >= bytes.len() {
                return None;
            }
            let h1 = (bytes[i + 1] as char).to_digit(16)?;
            let h2 = (bytes[i + 2] as char).to_digit(16)?;
            out.push(((h1 << 4) | h2) as u8);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

// ──────────────── console.log inspect formatter (CLIF-EXT 1) ────────────────
//
// Mirrors Node's util.inspect closely enough that console.log(arr) prints
// `[ 1, 2, 3 ]` instead of `[object Object]`. Top-level strings are
// unquoted (Node behavior); nested strings are quoted. Recursion is capped
// at INSPECT_MAX_DEPTH; cycles are short-circuited via a visited set.

const INSPECT_MAX_DEPTH: u32 = 2;

/// Format a list of console.log arguments per Node semantics: space-joined,
/// top-level strings unquoted, everything else through inspect_value.
fn console_format(rt: &Runtime, args: &[Value]) -> String {
    let mut out = String::new();
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        match a {
            Value::String(s) => out.push_str(s.as_str()),
            _ => out.push_str(&inspect_value(rt, a)),
        }
    }
    out
}

/// Format a Value for console.log inspection. Top-level entry; allocates
/// the visited-set used to break cycles.
pub(crate) fn inspect_value(rt: &Runtime, v: &Value) -> String {
    let mut visited = std::collections::HashSet::new();
    inspect_inner(rt, v, 0, &mut visited, false)
}

fn inspect_inner(
    rt: &Runtime,
    v: &Value,
    depth: u32,
    visited: &mut std::collections::HashSet<u32>,
    in_container: bool,
) -> String {
    match v {
        Value::Undefined => "undefined".into(),
        Value::Null => "null".into(),
        Value::Boolean(b) => b.to_string(),
        Value::Number(n) => format_number(*n),
        Value::BigInt(b) => format!("{}n", b.to_decimal()),
        Value::Symbol(s) => format!("Symbol({})", s.as_str().trim_start_matches("@@sym:")),
        Value::String(s) => {
            if in_container {
                format!(
                    "'{}'",
                    s.as_str().replace('\\', "\\\\").replace('\'', "\\'")
                )
            } else {
                s.as_str().to_string()
            }
        }
        Value::Object(id) => inspect_object(rt, *id, depth, visited),
    }
}

fn format_number(n: f64) -> String {
    if n.is_nan() {
        return "NaN".into();
    }
    if n.is_infinite() {
        return if n > 0.0 {
            "Infinity".into()
        } else {
            "-Infinity".into()
        };
    }
    if n == 0.0 && n.is_sign_negative() {
        return "-0".into();
    }
    crate::abstract_ops::number_to_string(n)
}

fn inspect_object(
    rt: &Runtime,
    id: crate::value::ObjectRef,
    depth: u32,
    visited: &mut std::collections::HashSet<u32>,
) -> String {
    use crate::value::InternalKind;
    let key = id.0;
    if !visited.insert(key) {
        // Cycle: short-circuit per Node's '<ref *1>' behavior, simplified.
        return "[Circular]".into();
    }
    let result = match &rt.obj(id).internal_kind {
        InternalKind::Function(fi) => {
            visited.remove(&key);
            if fi.name.is_empty() {
                "[Function (anonymous)]".into()
            } else {
                format!("[Function: {}]", fi.name)
            }
        }
        InternalKind::Closure(ci) => {
            visited.remove(&key);
            let n = ci.proto.display_name.as_str();
            if n.is_empty() {
                "[Function (anonymous)]".into()
            } else {
                format!("[Function: {}]", n)
            }
        }
        InternalKind::BoundFunction(_) => {
            visited.remove(&key);
            "[Function]".into()
        }
        InternalKind::RegExp(r) => {
            visited.remove(&key);
            format!("/{}/{}", r.source, r.flags)
        }
        InternalKind::Error => {
            let msg = match rt.object_get(id, "message") {
                Value::String(s) => s.as_str().to_string(),
                _ => String::new(),
            };
            let name = match rt.object_get(id, "name") {
                Value::String(s) => s.as_str().to_string(),
                _ => "Error".into(),
            };
            visited.remove(&key);
            if msg.is_empty() {
                name
            } else {
                format!("{}: {}", name, msg)
            }
        }
        InternalKind::Array => {
            let r = inspect_array(rt, id, depth, visited);
            visited.remove(&key);
            r
        }
        _ => {
            let r = inspect_plain_object(rt, id, depth, visited);
            visited.remove(&key);
            r
        }
    };
    result
}

fn inspect_array(
    rt: &Runtime,
    id: crate::value::ObjectRef,
    depth: u32,
    visited: &mut std::collections::HashSet<u32>,
) -> String {
    // object_get(id, "length") handles Arrays via &self (its special branch
    // synthesizes length from max numeric index when no own length is set).
    let len = match rt.object_get(id, "length") {
        Value::Number(n) if n >= 0.0 && n.is_finite() => n as usize,
        _ => 0,
    };
    if len == 0 {
        return "[]".into();
    }
    if depth >= INSPECT_MAX_DEPTH {
        return "[Array]".into();
    }
    let mut parts: Vec<String> = Vec::with_capacity(len);
    for i in 0..len {
        let v = rt.object_get(id, &i.to_string());
        parts.push(inspect_inner(rt, &v, depth + 1, visited, true));
    }
    format!("[ {} ]", parts.join(", "))
}

fn inspect_plain_object(
    rt: &Runtime,
    id: crate::value::ObjectRef,
    depth: u32,
    visited: &mut std::collections::HashSet<u32>,
) -> String {
    // CLIF-EXT 1: detect Set/Map/Error via cruft's sentinel + proto conventions
    // (cruft stores these as Ordinary objects with engine-internal sentinels
    // plus wired prototypes, not as dedicated InternalKind variants).
    let has_set_data = rt.obj(id).has_own_str("__set_data");
    let has_map_data = rt.obj(id).has_own_str("__map_data");
    let has_weak = matches!(rt.object_get(id, "__is_weakmap"), Value::Boolean(true))
        || matches!(rt.object_get(id, "__is_weakset"), Value::Boolean(true));
    if has_set_data {
        let storage = match rt.object_get(id, "__set_data") {
            Value::Object(sid) => Some(sid),
            _ => None,
        };
        return inspect_set_like(
            rt,
            id,
            storage,
            depth,
            visited,
            if has_weak { "WeakSet" } else { "Set" },
        );
    }
    if has_map_data {
        let storage = match rt.object_get(id, "__map_data") {
            Value::Object(sid) => Some(sid),
            _ => None,
        };
        return inspect_map_like(
            rt,
            id,
            storage,
            depth,
            visited,
            if has_weak { "WeakMap" } else { "Map" },
        );
    }
    if let Some(err_name) = detect_error_class(rt, id) {
        let msg = match rt.object_get(id, "message") {
            Value::String(s) => s.as_str().to_string(),
            _ => String::new(),
        };
        return if msg.is_empty() {
            err_name
        } else {
            format!("{}: {}", err_name, msg)
        };
    }
    let keys: Vec<String> = rt
        .ordinary_own_enumerable_string_keys(id)
        .into_iter()
        // Filter engine-internal sentinels (__-prefixed and @@-prefixed); they
        // should never reach observable output. EIPD/GBNE work made many of
        // these non-enumerable, but a residual layer still leaks.
        .filter(|k| !k.starts_with("__") && !k.starts_with("@@"))
        .collect();
    if keys.is_empty() {
        return "{}".into();
    }
    if depth >= INSPECT_MAX_DEPTH {
        return "[Object]".into();
    }
    let mut parts: Vec<String> = Vec::with_capacity(keys.len());
    for k in &keys {
        let v = rt.object_get(id, k);
        let key_str = if is_valid_identifier(k) {
            k.clone()
        } else {
            format!("'{}'", k)
        };
        parts.push(format!(
            "{}: {}",
            key_str,
            inspect_inner(rt, &v, depth + 1, visited, true)
        ));
    }
    format!("{{ {} }}", parts.join(", "))
}

fn inspect_set_like(
    rt: &Runtime,
    instance: crate::value::ObjectRef,
    storage: Option<crate::value::ObjectRef>,
    depth: u32,
    visited: &mut std::collections::HashSet<u32>,
    label: &str,
) -> String {
    let size = match rt.object_get(instance, "size") {
        Value::Number(n) if n >= 0.0 => n as usize,
        _ => 0,
    };
    if size == 0 {
        return format!("{}(0) {{}}", label);
    }
    if depth >= INSPECT_MAX_DEPTH {
        return format!("[{}]", label);
    }
    let mut parts = Vec::new();
    if let Some(sid) = storage {
        for (_, d) in rt.obj(sid).properties.iter() {
            parts.push(inspect_inner(rt, &d.value, depth + 1, visited, true));
        }
    }
    format!("{}({}) {{ {} }}", label, size, parts.join(", "))
}

fn inspect_map_like(
    rt: &Runtime,
    instance: crate::value::ObjectRef,
    storage: Option<crate::value::ObjectRef>,
    depth: u32,
    visited: &mut std::collections::HashSet<u32>,
    label: &str,
) -> String {
    let size = match rt.object_get(instance, "size") {
        Value::Number(n) if n >= 0.0 => n as usize,
        _ => 0,
    };
    if size == 0 {
        return format!("{}(0) {{}}", label);
    }
    if depth >= INSPECT_MAX_DEPTH {
        return format!("[{}]", label);
    }
    let mut parts = Vec::new();
    if let Some(sid) = storage {
        for (k, d) in rt.obj(sid).properties.iter() {
            let k_str = k.as_str().to_string();
            let k_render = if is_valid_identifier(&k_str) {
                k_str
            } else {
                format!("'{}'", k_str)
            };
            parts.push(format!(
                "{} => {}",
                k_render,
                inspect_inner(rt, &d.value, depth + 1, visited, true)
            ));
        }
    }
    format!("{}({}) {{ {} }}", label, size, parts.join(", "))
}

/// Walk the proto chain looking for an Error.prototype-shaped object
/// (carries "name" set to "Error"/"TypeError"/etc. per EIPD-EXT 1's
/// per-prototype set_own_internal). Returns the class name when found.
fn detect_error_class(rt: &Runtime, id: crate::value::ObjectRef) -> Option<String> {
    const NAMES: &[&str] = &[
        "Error",
        "TypeError",
        "RangeError",
        "SyntaxError",
        "ReferenceError",
        "URIError",
        "EvalError",
        "AggregateError",
    ];
    let mut cur = rt.obj(id).proto;
    let mut hops = 0;
    while let Some(c) = cur {
        if hops > 5 {
            break;
        }
        if let Some(d) = rt.obj(c).get_own("name") {
            if let Value::String(s) = &d.value {
                if NAMES.iter().any(|n| n == &s.as_str()) {
                    return Some(s.as_str().to_string());
                }
            }
        }
        cur = rt.obj(c).proto;
        hops += 1;
    }
    None
}

fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        None => return false,
        Some(c) if !c.is_ascii_alphabetic() && c != '_' && c != '$' => return false,
        _ => {}
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

pub(crate) fn make_native(
    name: &str,
    f: impl Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
) -> Object {
    make_native_with_length(name, 0, f)
}

/// Tier-Ω.5.P15.E1: intrinsic constructor with explicit ECMA-262 §10.2.10
/// arity. Use this at sites where the spec mandates a specific .length
/// (e.g. Math.min = 2, Object.keys = 1); the zero-default of `make_native`
/// is observable through `fn.length` reads in consumer code.
pub(crate) fn make_native_with_length(
    name: &str,
    length: u32,
    f: impl Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
) -> Object {
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, name, length as f64);
    Object {
        proto: None,
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals {
            name: name.to_string(),
            length,
            native,
            is_constructor: true,
        }),

        ..Default::default()
    }
}

/// Ω.5.P61.E4: build a non-constructor native (Math.abs, Object.keys,
/// String.prototype.includes, ...). Mirrors make_native_with_length but
/// sets FunctionInternals.is_constructor = false; Op::New and
/// Reflect.construct check the flag and throw TypeError per ECMA §21.3.
pub(crate) fn make_native_non_ctor(
    name: &str,
    length: u32,
    f: impl Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
) -> Object {
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, name, length as f64);
    Object {
        proto: None,
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals {
            name: name.to_string(),
            length,
            native,
            is_constructor: false,
        }),

        ..Default::default()
    }
}

fn register_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, f: F)
where
    F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
{
    // Ω.5.P62.E2: built-in methods installed via register_method are
    // intrinsics per ECMA §10.2.x — non-enumerable + non-constructor.
    // Only register_method's length/arity stays at 0 (callers that need
    // spec-correct arity reach for register_intrinsic_method directly).
    // User-code property assignment goes through Op::SetProperty, never
    // this path, so making the default non-enumerable closes the
    // Date.prototype.getUTC* enumerability hole + the symmetric cluster
    // across most built-in protos exposed by Object.gOPD test262 slice.
    let fn_obj = make_native_non_ctor(name, 0, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.obj_mut(host)
        .set_own_internal(name.into(), Value::Object(fn_id));
}

/// Ω.5.P61.E3: install an intrinsic method (Math.abs, Object.keys, etc.)
/// with ECMA-correct descriptor + arity per §10.2.9/§10.2.10 + §6.2.5.4:
/// length set to `arity`; the property on `host` is
/// {writable: true, enumerable: false, configurable: true} — non-enum
/// is the ECMA invariant for built-ins (Object.keys(Math) returns only
/// numeric constants, not method names).
///
/// Use at intrinsic-install sites; user-code property assignment
/// continues to use `register_method` (enumerable per spec for
/// CreateDataPropertyOrThrow defaults).
pub(crate) fn register_intrinsic_method<F>(
    rt: &mut Runtime,
    host: ObjectRef,
    name: &str,
    length: u32,
    f: F,
) where
    F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
{
    // Ω.5.P61.E4: intrinsic methods are non-constructors per ECMA §21.3
    // (and the same applies to every built-in not identified as a
    // constructor — Object.keys, String.prototype.includes, Array.
    // prototype.map, etc.). make_native_non_ctor sets the flag so
    // Op::New + Reflect.construct throw TypeError on `new Math.abs()`.
    let fn_obj = make_native_non_ctor(name, length, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.obj_mut(host).dict_mut().insert(
        crate::value::PropertyKey::String(name.to_string()),
        crate::value::PropertyDescriptor {
            value: Value::Object(fn_id),
            writable: true,
            enumerable: false,
            configurable: true,
            getter: None,
            setter: None,
        },
    );
}

/// Register a global as a constructor-callable native. Use for §20.2.1
/// Function and any other intrinsic that the spec marks `[[Construct]]`.
fn register_global_ctor<F>(rt: &mut Runtime, name: &str, f: F)
where
    F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
{
    let fn_obj = make_native(name, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.globals.insert(name.into(), Value::Object(fn_id));
}

fn register_global_fn<F>(rt: &mut Runtime, name: &str, f: F)
where
    F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
{
    // §19.2.{1..6} parseInt, parseFloat, isNaN, isFinite, decodeURI,
    // decodeURIComponent, encodeURI, encodeURIComponent — all are functions,
    // not constructors. Use make_native_non_ctor so `new parseInt(...)`
    // throws TypeError per spec.
    let fn_obj = make_native_non_ctor(name, 1, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.globals.insert(name.into(), Value::Object(fn_id));
}

/// Ω.5.P55.E1 (Doc 729 §VII.B): register a compiler-emitted lowering
/// behind the engine-internal bilateral boundary. The helper resolves
/// through `Op::LoadGlobal`'s fallback path (interp.rs) but does not
/// appear in `globals`, so `globalThis.__X` reads as `undefined` and
/// `Object.keys(globalThis)` does not enumerate it.
fn register_engine_helper<F>(rt: &mut Runtime, name: &str, f: F)
where
    F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
{
    let fn_obj = make_native(name, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.engine_helpers.insert(name.into(), Value::Object(fn_id));
}

// ──────────────── JSON.stringify (limited) ────────────────

pub(crate) fn json_stringify(rt: &Runtime, v: &Value) -> String {
    // JSF-EXT 3 (2026-05-23): thin wrapper around the buffer-threaded
    // json_stringify_into. The 256-byte initial capacity is the default
    // tuning per JSF-EXT 2 design §R2; revisit at JSF-EXT 5 measurement.
    let mut out = String::with_capacity(256);
    json_stringify_into(rt, v, &mut out);
    out
}

pub(crate) fn json_stringify_into(rt: &Runtime, v: &Value, out: &mut String) {
    match v {
        Value::Undefined => out.push_str("undefined"),
        Value::Null => out.push_str("null"),
        Value::Boolean(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => {
            if n.is_finite() {
                // JSF-EXT 5 (Move 3 — cascade-revival pilot #2 per Doc
                // 739): integer fast-path writes digits directly into
                // the buffer; f64-fractional falls back to
                // number_to_string. Integer detection via the bit-exact
                // fract()==0 + range check that JS itself uses for
                // ECMA number-to-string integer branch.
                let n = *n;
                if n == 0.0 {
                    out.push('0');
                } else if n.is_finite()
                    && n.fract() == 0.0
                    && n >= i64::MIN as f64
                    && n <= i64::MAX as f64
                {
                    write_i64_into(n as i64, out);
                } else {
                    out.push_str(&abstract_ops::number_to_string(n));
                }
            } else {
                out.push_str("null");
            }
        }
        Value::String(s) => json_quote_string_into(s.as_str(), out),
        Value::BigInt(_) => out.push_str("null"),
        // ECMA §25.5.2.4 SerializeJSONProperty: Symbol values serialize to
        // undefined and the enclosing object omits the key. We surface
        // "undefined" here; the caller's per-property filter at the object
        // branch elides keys whose serialized form is "undefined".
        Value::Symbol(_) => out.push_str("undefined"),
        Value::Object(id) => {
            // §25.5.2.2 SerializeJSONProperty: if the value is a Number,
            // String, or Boolean Object wrapper, unwrap to its primitive
            // before serializing. cruftless stores the primitive in the
            // non-enumerable __primitive__ slot at construction time.
            if let Some(d) = rt.obj(*id).get_own("__primitive__") {
                match &d.value {
                    Value::Number(_) | Value::String(_) | Value::Boolean(_) => {
                        let unwrapped = d.value.clone();
                        json_stringify_into(rt, &unwrapped, out);
                        return;
                    }
                    _ => {}
                }
            }
            // CMig-EXT 16.bis (2026-05-23): shape-aware. Per shapes seed
            // §IV carve-out, shape-stored entries are plain-data
            // descriptors with user-default {w:t, e:t, c:t}; emit them
            // as if they had a PropertyDescriptor. Dictionary entries
            // follow with their original descriptors.
            //
            // JSF-EXT 6 (Move 4): iterate via reference; no per-property
            // PropertyDescriptor.clone() or Value.clone(). The obj
            // borrow + the recursive json_stringify_into's rt borrow
            // are both shared (json_stringify_into takes &Runtime), so
            // they coexist via NLL.
            let obj = rt.obj(*id);
            let is_array = matches!(obj.internal_kind, InternalKind::Array);
            if is_array {
                // Two-pass: gather (index, &Value) then sort numerically.
                let mut entries: Vec<(usize, &Value)> = Vec::new();
                if let Some(shape) = obj.shape.as_ref() {
                    for (name, slot) in shape.iter_slots() {
                        if let Ok(i) = name.parse::<usize>() {
                            if let Some(val) = obj.shape_values.get(slot as usize) {
                                entries.push((i, val));
                            }
                        }
                    }
                }
                for (k, d) in &obj.properties {
                    if let Ok(i) = k.to_string_content().parse::<usize>() {
                        entries.push((i, &d.value));
                    }
                }
                entries.sort_by_key(|(i, _)| *i);
                out.push('[');
                let mut first = true;
                for (_, v) in &entries {
                    if !first {
                        out.push(',');
                    }
                    first = false;
                    json_stringify_into(rt, v, out);
                }
                out.push(']');
            } else {
                // Ω.5.P19.E1: JSON.stringify ignores Symbol-keyed properties
                // per ECMA §25.5.2.4 (the `@@` prefix on both user symbols
                // and well-known-symbol slots). Also skip values whose
                // serialized form is `"undefined"`.
                out.push('{');
                let mut first = true;
                if let Some(shape) = obj.shape.as_ref() {
                    for (name, slot) in shape.iter_slots() {
                        if name.starts_with("@@") {
                            continue;
                        }
                        if let Some(val) = obj.shape_values.get(slot as usize) {
                            if matches!(val, Value::Undefined | Value::Symbol(_)) {
                                continue;
                            }
                            if !first {
                                out.push(',');
                            }
                            first = false;
                            json_quote_string_into(name, out);
                            out.push(':');
                            json_stringify_into(rt, val, out);
                        }
                    }
                }
                for (k, d) in &obj.properties {
                    if !d.enumerable {
                        continue;
                    }
                    if matches!(d.value, Value::Undefined | Value::Symbol(_)) {
                        continue;
                    }
                    let ks = k.to_string_content();
                    if ks.starts_with("@@") {
                        continue;
                    }
                    if !first {
                        out.push(',');
                    }
                    first = false;
                    json_quote_string_into(&ks, out);
                    out.push(':');
                    json_stringify_into(rt, &d.value, out);
                }
                out.push('}');
            }
        }
    }
}

/// JSF-EXT 5 (Move 3): write a signed 64-bit integer's decimal form
/// directly into the buffer with no allocation. Reverse-emit digits
/// then in-place reverse the appended ASCII slice. i64::MIN handled
/// by emitting its known string ("-9223372036854775808") directly
/// to avoid the negate overflow.
fn write_i64_into(n: i64, out: &mut String) {
    if n == i64::MIN {
        out.push_str("-9223372036854775808");
        return;
    }
    let neg = n < 0;
    let mut m: u64 = if neg { (-n) as u64 } else { n as u64 };
    if neg {
        out.push('-');
    }
    let start = out.len();
    while m > 0 {
        // SAFETY: digit byte 0x30..=0x39 is valid UTF-8 (ASCII).
        unsafe {
            out.as_mut_vec().push(b'0' + (m % 10) as u8);
        }
        m /= 10;
    }
    // SAFETY: only ASCII digits were pushed at [start..]; byte-level
    // reverse preserves UTF-8 validity.
    unsafe {
        out.as_mut_vec()[start..].reverse();
    }
}

pub(crate) fn json_quote_string_pub(s: &str) -> String {
    json_quote_string(s)
}

fn json_quote_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    json_quote_string_into(s, &mut out);
    out
}

/// JSF-EXT 4 (2026-05-23, Move 2 — cascade-revival pilot per Doc 739):
/// branchless ASCII fast-path. Stage 1 scans bytes forward to the next
/// byte requiring escape (special ASCII or control char); stage 1
/// bulk-copies the run via push_str. Stage 2 emits the escape and
/// advances. Multibyte UTF-8 continuation bytes (>= 0x80) are
/// non-special and stay in the fast scan. The format!("\\u{:04x}")
/// allocation per control char is replaced by a direct 6-byte emit.
fn json_quote_string_into(s: &str, out: &mut String) {
    out.push('"');
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'"' || b == b'\\' || b < 0x20 {
                break;
            }
            i += 1;
        }
        if i > start {
            // SAFETY: bytes[start..i] is a valid UTF-8 prefix of s; we
            // only advanced past ASCII non-special bytes and through
            // multibyte continuations as opaque bytes, and stopped
            // before any byte that could start a fresh ASCII special.
            out.push_str(unsafe { std::str::from_utf8_unchecked(&bytes[start..i]) });
        }
        if i < bytes.len() {
            let b = bytes[i];
            match b {
                b'"' => out.push_str("\\\""),
                b'\\' => out.push_str("\\\\"),
                b'\n' => out.push_str("\\n"),
                b'\r' => out.push_str("\\r"),
                b'\t' => out.push_str("\\t"),
                b'\x08' => out.push_str("\\b"),
                b'\x0c' => out.push_str("\\f"),
                c => {
                    let hi = (c >> 4) & 0xF;
                    let lo = c & 0xF;
                    out.push_str("\\u00");
                    out.push(if hi < 10 {
                        (b'0' + hi) as char
                    } else {
                        (b'a' + hi - 10) as char
                    });
                    out.push(if lo < 10 {
                        (b'0' + lo) as char
                    } else {
                        (b'a' + lo - 10) as char
                    });
                }
            }
            i += 1;
        }
    }
    out.push('"');
}

// ──────────────── JSON.parse (limited recursive-descent) ────────────────

pub fn json_parse(rt: &mut Runtime, s: &str) -> Result<Value, RuntimeError> {
    let bytes = s.as_bytes();
    let mut p = 0;
    skip_ws(bytes, &mut p);
    let v = json_parse_value(rt, bytes, &mut p)?;
    skip_ws(bytes, &mut p);
    if p != bytes.len() {
        return Err(RuntimeError::SyntaxError(
            "JSON.parse: trailing characters".into(),
        ));
    }
    Ok(v)
}

fn skip_ws(b: &[u8], p: &mut usize) {
    while *p < b.len() && matches!(b[*p], b' ' | b'\t' | b'\n' | b'\r') {
        *p += 1;
    }
}

fn json_parse_value(rt: &mut Runtime, b: &[u8], p: &mut usize) -> Result<Value, RuntimeError> {
    skip_ws(b, p);
    if *p >= b.len() {
        return Err(RuntimeError::SyntaxError(
            "JSON.parse: unexpected end".into(),
        ));
    }
    match b[*p] {
        b'{' => json_parse_object(rt, b, p),
        b'[' => json_parse_array(rt, b, p),
        b'"' => json_parse_string(b, p).map(|s| Value::String(Rc::new(s))),
        b't' if b[*p..].starts_with(b"true") => {
            *p += 4;
            Ok(Value::Boolean(true))
        }
        b'f' if b[*p..].starts_with(b"false") => {
            *p += 5;
            Ok(Value::Boolean(false))
        }
        b'n' if b[*p..].starts_with(b"null") => {
            *p += 4;
            Ok(Value::Null)
        }
        b'-' | b'0'..=b'9' => json_parse_number(b, p),
        _ => Err(RuntimeError::SyntaxError(format!(
            "JSON.parse: unexpected character at offset {}",
            p
        ))),
    }
}

fn json_parse_object(rt: &mut Runtime, b: &[u8], p: &mut usize) -> Result<Value, RuntimeError> {
    *p += 1; // consume '{'
    let obj = rt.alloc_object(Object::new_ordinary());
    skip_ws(b, p);
    if *p < b.len() && b[*p] == b'}' {
        *p += 1;
        return Ok(Value::Object(obj));
    }
    loop {
        skip_ws(b, p);
        let key = json_parse_string(b, p)?;
        skip_ws(b, p);
        if *p >= b.len() || b[*p] != b':' {
            return Err(RuntimeError::SyntaxError("JSON.parse: expected ':'".into()));
        }
        *p += 1;
        let value = json_parse_value(rt, b, p)?;
        rt.object_set(obj, key, value);
        skip_ws(b, p);
        match b.get(*p) {
            Some(&b',') => {
                *p += 1;
                continue;
            }
            Some(&b'}') => {
                *p += 1;
                return Ok(Value::Object(obj));
            }
            _ => {
                return Err(RuntimeError::SyntaxError(
                    "JSON.parse: expected ',' or '}'".into(),
                ))
            }
        }
    }
}

fn json_parse_array(rt: &mut Runtime, b: &[u8], p: &mut usize) -> Result<Value, RuntimeError> {
    *p += 1; // consume '['
    let arr = rt.alloc_object(Object::new_array());
    skip_ws(b, p);
    if *p < b.len() && b[*p] == b']' {
        *p += 1;
        return Ok(Value::Object(arr));
    }
    let mut i = 0u32;
    loop {
        let value = json_parse_value(rt, b, p)?;
        rt.object_set(arr, i.to_string(), value);
        i += 1;
        skip_ws(b, p);
        match b.get(*p) {
            Some(&b',') => {
                *p += 1;
                continue;
            }
            Some(&b']') => {
                *p += 1;
                return Ok(Value::Object(arr));
            }
            _ => {
                return Err(RuntimeError::SyntaxError(
                    "JSON.parse: expected ',' or ']'".into(),
                ))
            }
        }
    }
}

fn json_parse_string(b: &[u8], p: &mut usize) -> Result<String, RuntimeError> {
    if *p >= b.len() || b[*p] != b'"' {
        return Err(RuntimeError::SyntaxError(
            "JSON.parse: expected string".into(),
        ));
    }
    *p += 1;
    // Collect bytes (not chars). Non-ASCII UTF-8 byte sequences pass
    // through verbatim and decode correctly at from_utf8_lossy time at
    // the end. Pre-fix, `out.push(c as char)` decoded each byte as a
    // Latin-1 codepoint, mangling multi-byte sequences like "中"
    // (0xE4 0xB8 0xAD → "ä¸­" instead of one Unicode codepoint).
    let mut bytes: Vec<u8> = Vec::new();
    while *p < b.len() {
        let c = b[*p];
        if c == b'"' {
            *p += 1;
            return Ok(String::from_utf8_lossy(&bytes).to_string());
        }
        if c == b'\\' {
            *p += 1;
            if *p >= b.len() {
                return Err(RuntimeError::SyntaxError("JSON.parse: dangling \\".into()));
            }
            match b[*p] {
                b'"' => bytes.push(b'"'),
                b'\\' => bytes.push(b'\\'),
                b'/' => bytes.push(b'/'),
                b'n' => bytes.push(b'\n'),
                b'r' => bytes.push(b'\r'),
                b't' => bytes.push(b'\t'),
                b'b' => bytes.push(0x08),
                b'f' => bytes.push(0x0C),
                b'u' if *p + 4 < b.len() => {
                    let hex = std::str::from_utf8(&b[*p + 1..*p + 5])
                        .map_err(|_| RuntimeError::SyntaxError("JSON.parse: bad \\u".into()))?;
                    let cp = u32::from_str_radix(hex, 16)
                        .map_err(|_| RuntimeError::SyntaxError("JSON.parse: bad \\u".into()))?;
                    if let Some(ch) = char::from_u32(cp) {
                        let mut buf = [0u8; 4];
                        let s = ch.encode_utf8(&mut buf);
                        bytes.extend_from_slice(s.as_bytes());
                    }
                    *p += 4;
                }
                _ => return Err(RuntimeError::SyntaxError("JSON.parse: bad escape".into())),
            }
            *p += 1;
        } else {
            // Ω.5.P62.E22: ECMA §25.5.1 JSONStringCharacter excludes
            // U+0000 through U+001F; control chars must be escaped.
            if c < 0x20 {
                return Err(RuntimeError::SyntaxError(
                    "JSON.parse: invalid control character in string".into(),
                ));
            }
            bytes.push(c);
            *p += 1;
        }
    }
    Err(RuntimeError::SyntaxError(
        "JSON.parse: unterminated string".into(),
    ))
}

fn json_parse_number(b: &[u8], p: &mut usize) -> Result<Value, RuntimeError> {
    let start = *p;
    if b[*p] == b'-' {
        *p += 1;
    }
    while *p < b.len() && b[*p].is_ascii_digit() {
        *p += 1;
    }
    if *p < b.len() && b[*p] == b'.' {
        *p += 1;
        while *p < b.len() && b[*p].is_ascii_digit() {
            *p += 1;
        }
    }
    if *p < b.len() && (b[*p] == b'e' || b[*p] == b'E') {
        *p += 1;
        if *p < b.len() && (b[*p] == b'+' || b[*p] == b'-') {
            *p += 1;
        }
        while *p < b.len() && b[*p].is_ascii_digit() {
            *p += 1;
        }
    }
    let s = std::str::from_utf8(&b[start..*p])
        .map_err(|_| RuntimeError::SyntaxError("JSON.parse: bad number".into()))?;
    let n = s
        .parse::<f64>()
        .map_err(|_| RuntimeError::SyntaxError("JSON.parse: bad number".into()))?;
    Ok(Value::Number(n))
}

// Tier-Ω.5.eee: minimal base64 codec for atob/btoa. Standard alphabet,
// padding required on decode (entities-generated data is well-formed).
const B64_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
/// Ω.5.P44.E1: ECMA §6.1.7 IsIntegerIndex predicate. A property key is
/// an integer index iff its ToString form is identical to ToString of
/// its ToUint32. Practically: a non-empty all-digit string with no
/// leading zeros (except "0" itself) and value ≤ 2^32-2.
pub(crate) fn is_integer_index(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    if s == "0" {
        return true;
    }
    if s.starts_with('0') {
        return false;
    }
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    match s.parse::<u64>() {
        Ok(n) if n < ((1u64 << 32) - 1) => true,
        _ => false,
    }
}

fn base64_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    let mut i = 0;
    while i + 3 <= input.len() {
        let n = ((input[i] as u32) << 16) | ((input[i + 1] as u32) << 8) | (input[i + 2] as u32);
        out.push(B64_ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[((n >> 6) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[(n & 0x3F) as usize] as char);
        i += 3;
    }
    let rem = input.len() - i;
    if rem == 1 {
        let n = (input[i] as u32) << 16;
        out.push(B64_ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        out.push('=');
        out.push('=');
    } else if rem == 2 {
        let n = ((input[i] as u32) << 16) | ((input[i + 1] as u32) << 8);
        out.push(B64_ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[((n >> 6) & 0x3F) as usize] as char);
        out.push('=');
    }
    out
}
fn base64_decode(s: &str) -> Result<Vec<u8>, &'static str> {
    let mut lut = [255u8; 256];
    for (i, &c) in B64_ALPHABET.iter().enumerate() {
        lut[c as usize] = i as u8;
    }
    let bytes: Vec<u8> = s.bytes().filter(|&b| b != b'=').collect();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    let mut i = 0;
    while i + 4 <= bytes.len() {
        let (a, b, c, d) = (
            lut[bytes[i] as usize],
            lut[bytes[i + 1] as usize],
            lut[bytes[i + 2] as usize],
            lut[bytes[i + 3] as usize],
        );
        if (a | b | c | d) == 255 {
            return Err("invalid base64 character");
        }
        let n = ((a as u32) << 18) | ((b as u32) << 12) | ((c as u32) << 6) | (d as u32);
        out.push(((n >> 16) & 0xFF) as u8);
        out.push(((n >> 8) & 0xFF) as u8);
        out.push((n & 0xFF) as u8);
        i += 4;
    }
    let rem = bytes.len() - i;
    if rem == 2 {
        let (a, b) = (lut[bytes[i] as usize], lut[bytes[i + 1] as usize]);
        if (a | b) == 255 {
            return Err("invalid base64 character");
        }
        let n = ((a as u32) << 18) | ((b as u32) << 12);
        out.push(((n >> 16) & 0xFF) as u8);
    } else if rem == 3 {
        let (a, b, c) = (
            lut[bytes[i] as usize],
            lut[bytes[i + 1] as usize],
            lut[bytes[i + 2] as usize],
        );
        if (a | b | c) == 255 {
            return Err("invalid base64 character");
        }
        let n = ((a as u32) << 18) | ((b as u32) << 12) | ((c as u32) << 6);
        out.push(((n >> 16) & 0xFF) as u8);
        out.push(((n >> 8) & 0xFF) as u8);
    } else if rem == 1 {
        return Err("invalid base64 length");
    }
    Ok(out)
}

// Tier-Ω.5.aaaa: Gregorian date arithmetic helpers for Date intrinsics.
//
// All functions operate on milliseconds since Unix epoch (UTC, no
// timezone). Sufficient for moment / dayjs / date-fns module-load and
// basic API exercise; not full IANA-timezone-aware.

/// Compute (year, month-0-based, day-1-based) from epoch-ms.
pub(crate) fn date_components(ms: f64) -> (i64, i64, i64) {
    let days = (ms / 86_400_000.0).floor() as i64;
    // Days since 1970-01-01.
    // Convert to year, month, day via Gregorian algorithm.
    let mut z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    z = m - 1; // month 0-based
    let _ = z;
    (year, m - 1, d)
}

/// Build epoch-ms from (year, month-0-based, day-1-based).
pub(crate) fn ymd_to_ms(year: i64, month: i64, day: i64) -> i64 {
    let y = if month < 2 { year - 1 } else { year };
    let m = if month < 2 {
        (month + 9) as i64
    } else {
        (month - 2) as i64
    };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * m + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days_since_epoch = era * 146097 + doe - 719468;
    days_since_epoch * 86_400_000
}

/// Parse a Date string. Delegates to interp's parse_iso8601_to_epoch_ms
/// for consistent behavior with Date.parse(). Returns NaN on failure.
fn parse_date_string(s: &str) -> f64 {
    if let Some(v) = crate::interp::parse_iso8601_to_epoch_ms_public(s) {
        return v;
    }
    // Fall through to the legacy hand-rolled parser for shapes the new
    // parser doesn't recognize.
    parse_date_string_legacy(s)
}

fn parse_date_string_legacy(s: &str) -> f64 {
    let s = s.trim();
    if s.len() < 10 {
        return f64::NAN;
    }
    let y: i64 = match s[0..4].parse() {
        Ok(v) => v,
        Err(_) => return f64::NAN,
    };
    if s.as_bytes()[4] != b'-' {
        return f64::NAN;
    }
    let mo: i64 = match s[5..7].parse() {
        Ok(v) => v,
        Err(_) => return f64::NAN,
    };
    if s.as_bytes()[7] != b'-' {
        return f64::NAN;
    }
    let d: i64 = match s[8..10].parse() {
        Ok(v) => v,
        Err(_) => return f64::NAN,
    };
    let mut ms = ymd_to_ms(y, mo - 1, d);
    if s.len() >= 19 && s.as_bytes()[10] == b'T' {
        let h: i64 = s[11..13].parse().unwrap_or(0);
        let mi: i64 = s[14..16].parse().unwrap_or(0);
        let se: i64 = s[17..19].parse().unwrap_or(0);
        ms += h * 3_600_000 + mi * 60_000 + se * 1000;
        if s.len() >= 23 && s.as_bytes()[19] == b'.' {
            let mss: i64 = s[20..23].parse().unwrap_or(0);
            ms += mss;
        }
    }
    ms as f64
}
