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
use crate::value::{FunctionInternals, InternalKind, NativeFn, Object, ObjectRef, PropertyDescriptor, Value};
use std::collections::HashMap;
use std::rc::Rc;

impl Runtime {
    pub fn install_intrinsics(&mut self) {
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
        self.install_console();
        self.install_promise();
        self.install_regexp();
        self.install_test_record();
        self.install_destructure_helpers();
        self.install_spread_helpers();
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
            let spec = args.first()
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
            let resolved = match rt.resolve_module_full(&parent, &spec, crate::module::ModuleKind::ESM) {
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
                    let message = format!(
                        "dynamic import('{}') load failed: {}",
                        spec, detail
                    );
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
                    (false, crate::value::PromiseStatus::Pending, Value::Undefined)
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
                    Err(RuntimeError::TypeError(
                        "await on pending Promise not yet supported (Tier-Ω.5.P17.E1 stub)".into()
                    ))
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
            crate::promise::reject_promise(rt, p, Value::String(Rc::new(
                "TypeError: WebAssembly.compile not implemented (Tier-Ω.5.P26.E1 stub)".into()
            )));
            Ok(Value::Object(p))
        });
        register_method(self, wasm, "instantiate", move |rt, _args| {
            let p = crate::promise::new_promise(rt);
            crate::promise::reject_promise(rt, p, Value::String(Rc::new(
                "TypeError: WebAssembly.instantiate not implemented (Tier-Ω.5.P26.E1 stub)".into()
            )));
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
        register_method(self, wasm, "validate", |_rt, _args| Ok(Value::Boolean(false)));
        // Constructor stubs — packages probe `typeof WebAssembly.Module` etc.
        // to decide on a code path; returning a callable that throws on
        // construction is more disciplined than leaving them undefined.
        for ctor_name in &["Module", "Instance", "Memory", "Table", "Global", "Tag", "Function"] {
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
                let msg = args.first()
                    .map(|v| crate::abstract_ops::to_string(v).as_str().to_string())
                    .unwrap_or_default();
                _rt.object_set(id, "message".into(), Value::String(Rc::new(msg)));
                Ok(Value::Object(id))
            });
            let stub_id = self.alloc_object(stub);
            self.object_set(wasm, name, Value::Object(stub_id));
        }
        self.globals.insert("WebAssembly".into(), Value::Object(wasm));

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
    pub fn install_global_this_refresh(&mut self) { self.install_global_this(); }

    fn install_global_this(&mut self) {
        let gt = self.alloc_object(Object::new_ordinary());
        let entries: Vec<(String, Value)> = self.globals.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (k, v) in entries {
            self.object_set(gt, k, v);
        }
        self.object_set(gt, "globalThis".into(), Value::Object(gt));
        // Tier-Ω.5.bbbb: `global` is a Node-side alias for globalThis;
        // many CJS packages do `global.foo = ...` or `global.process`.
        self.object_set(gt, "global".into(), Value::Object(gt));
        self.globals.insert("globalThis".into(), Value::Object(gt));
        self.globals.insert("global".into(), Value::Object(gt));
        // Tier-Ω.5.bbbb: Intl namespace with stub constructors. Real
        // locale-aware behavior is deferred; the stubs return objects
        // that survive shape probes and method existence checks. Lifts
        // packages that gate on `typeof Intl.X === 'function'`.
        let intl = self.alloc_object(Object::new_ordinary());
        for ctor_name in &["DateTimeFormat", "NumberFormat", "Collator", "PluralRules", "RelativeTimeFormat", "ListFormat", "Segmenter", "DisplayNames", "Locale"] {
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
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(stub_id));
            register_intrinsic_method(self, proto, "format", 1, |_rt, args| {
                Ok(Value::String(std::rc::Rc::new(
                    crate::abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined)).as_str().to_string()
                )))
            });
            register_intrinsic_method(self, proto, "formatToParts", 1, |rt, args| {
                let arr = Object::new_array();
                let aid = rt.alloc_object(arr);
                let part = rt.alloc_object(Object::new_ordinary());
                rt.object_set(part, "type".into(), Value::String(std::rc::Rc::new("literal".into())));
                rt.object_set(part, "value".into(), Value::String(std::rc::Rc::new(
                    crate::abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined)).as_str().to_string()
                )));
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
                rt.object_set(res, "locale".into(), Value::String(std::rc::Rc::new(locale_str)));
                rt.object_set(res, "calendar".into(), Value::String(std::rc::Rc::new("iso8601".into())));
                rt.object_set(res, "numberingSystem".into(), Value::String(std::rc::Rc::new("latn".into())));
                rt.object_set(res, "timeZone".into(), Value::String(std::rc::Rc::new("UTC".into())));
                if let Value::Object(opts_id) = opts {
                    let pairs: Vec<(String, Value)> = rt.obj(opts_id).properties
                        .iter().map(|(k, d)| (k.clone(), d.value.clone())).collect();
                    for (k, v) in pairs {
                        rt.object_set(res, k, v);
                    }
                }
                Ok(Value::Object(res))
            });
            self.obj_mut(stub_id).set_own_frozen("prototype".into(), Value::Object(proto));
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
            o.set_own("encoding".into(), Value::String(Rc::new("utf-8".to_string())));
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
        self.obj_mut(te_id).set_own_frozen("prototype".into(), Value::Object(te_proto));
        self.globals.insert("TextEncoder".into(), Value::Object(te_id));
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
        self.obj_mut(td_id).set_own_frozen("prototype".into(), Value::Object(td_proto));
        self.globals.insert("TextDecoder".into(), Value::Object(td_id));
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
            let target = match args.first() { Some(Value::Object(id)) => *id, _ => return Ok(Value::Undefined) };
            let key: String = match args.get(1) { Some(Value::String(s)) => (**s).clone(), _ => return Ok(Value::Undefined) };
            let kind: String = match args.get(2) { Some(Value::String(s)) => (**s).clone(), _ => return Ok(Value::Undefined) };
            let fn_v = args.get(3).cloned().unwrap_or(Value::Undefined);
            let o = rt.obj_mut(target);
            let desc = o.properties.entry(key).or_insert_with(|| crate::value::PropertyDescriptor {
                value: Value::Undefined,
                writable: true, enumerable: true, configurable: true,
                getter: None, setter: None,
            });
            if kind == "get" { desc.getter = Some(fn_v); }
            else if kind == "set" { desc.setter = Some(fn_v); }
            Ok(Value::Undefined)
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
            let target_arr = match rt.gen_yields_stack.last().copied() { Some(a) => a, None => return Ok(Value::Undefined) };
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
            } else { None };
            if let Some(iter_id) = iter_obj {
                let next = rt.object_get(iter_id, "next");
                if matches!(next, Value::Object(_)) {
                    loop {
                        let step = match rt.call_function(next.clone(), Value::Object(iter_id), Vec::new()) {
                            Ok(v) => v, Err(_) => break,
                        };
                        let step_id = match step { Value::Object(id) => id, _ => break };
                        if matches!(rt.object_get(step_id, "done"), Value::Boolean(true)) { break; }
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
                _ => return Err(RuntimeError::TypeError(
                    "__object_spread: target must be an object".into())),
            };
            if let Some(Value::Object(sid)) = args.get(1) {
                // Tier-Ω.5.bbbbb: dispatch accessor getters during spread.
                let entries: Vec<(String, Option<Value>)> = rt.obj(*sid).properties.iter()
                    .filter(|(_, d)| d.enumerable)
                    .map(|(k, d)| (k.clone(), d.getter.clone()))
                    .collect();
                for (k, getter_opt) in entries {
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
                _ => return Err(RuntimeError::TypeError(
                    "__array_push_single: target must be an array".into())),
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
                _ => return Err(RuntimeError::TypeError(
                    "__array_extend: target must be an array".into())),
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
            } else { None };
            let mut ordinary = Object::new_ordinary();
            if proto_override.is_some() { ordinary.proto = proto_override; }
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
            // Snapshot own enumerable property keys from src.
            let entries: Vec<(String, Value)> = {
                let o = rt.obj(src_id);
                o.properties.iter()
                    .filter(|(_, d)| d.enumerable)
                    .map(|(k, d)| (k.clone(), d.value.clone()))
                    .collect()
            };
            for (k, v) in entries {
                if excluded_keys.iter().any(|e| e == &k) { continue; }
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
                Value::Object(id) => Ok(Value::Boolean(rt.obj(id).properties.contains_key(key.as_str()))),
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
            let decoded = base64_decode(&cleaned).map_err(|e| RuntimeError::Thrown(
                Value::String(Rc::new(format!("InvalidCharacterError: {}", e)))
            ))?;
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
        register_global_fn(self, "parseInt", |_rt, args|{
            let s = if args.is_empty() { return Ok(Value::Number(f64::NAN)); } else { abstract_ops::to_string(&args[0]) };
            let radix = args.get(1).map(|v| abstract_ops::to_number(v) as i32).unwrap_or(10);
            let radix = if radix == 0 { 10 } else { radix };
            let trimmed = s.trim_start();
            let (sign, body) = if let Some(rest) = trimmed.strip_prefix('-') { (-1.0, rest) }
                else if let Some(rest) = trimmed.strip_prefix('+') { (1.0, rest) }
                else { (1.0, trimmed) };
            let mut acc: u64 = 0;
            let mut any = false;
            for c in body.chars() {
                let d = match c {
                    '0'..='9' => c as u32 - '0' as u32,
                    'a'..='z' => c as u32 - 'a' as u32 + 10,
                    'A'..='Z' => c as u32 - 'A' as u32 + 10,
                    _ => break,
                };
                if (d as i32) >= radix { break; }
                acc = acc.saturating_mul(radix as u64).saturating_add(d as u64);
                any = true;
            }
            if !any { return Ok(Value::Number(f64::NAN)); }
            Ok(Value::Number(sign * acc as f64))
        });
        register_global_fn(self, "parseFloat", |_rt, args|{
            if args.is_empty() { return Ok(Value::Number(f64::NAN)); }
            let s = abstract_ops::to_string(&args[0]);
            let trimmed = s.trim_start();
            // Find longest numeric prefix
            let mut end = 0;
            let mut saw_digit = false;
            let mut saw_dot = false;
            let mut saw_e = false;
            for (i, c) in trimmed.char_indices() {
                if i == 0 && (c == '+' || c == '-') { end = i + 1; continue; }
                match c {
                    '0'..='9' => { saw_digit = true; end = i + 1; }
                    '.' if !saw_dot && !saw_e => { saw_dot = true; end = i + 1; }
                    'e' | 'E' if saw_digit && !saw_e => { saw_e = true; end = i + 1; }
                    '+' | '-' if saw_e && trimmed[..i].chars().last() == Some('e' as char) => { end = i + 1; }
                    _ => break,
                }
            }
            if end == 0 { return Ok(Value::Number(f64::NAN)); }
            Ok(Value::Number(trimmed[..end].parse().unwrap_or(f64::NAN)))
        });
        // Ω.5.P63.E9: global isNaN / isFinite routed through IR-lowered
        // generated::global_is_*. Differ from Number.isNaN / Number.isFinite
        // by coercing the arg via ToNumber.
        register_global_fn(self, "isNaN", |rt, args|{
            crate::generated::global_is_nan(rt, Value::Undefined, args)
        });
        register_global_fn(self, "isFinite", |rt, args|{
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
        register_global_fn(self, "Function", |rt, args| {
            let body = match args.last() {
                Some(Value::String(s)) => s.as_str().to_string(),
                _ => String::new(),
            };
            let body_trim = body.trim();
            if body_trim == "return this" || body_trim == "return this;" {
                let global_obj = rt.globals.get("globalThis").cloned().unwrap_or(Value::Undefined);
                let f_obj = make_native("<Function('return this')>", move |_rt, _args| Ok(global_obj.clone()));
                return Ok(Value::Object(rt.alloc_object(f_obj)));
            }
            // Param list: all args except the last (which is the body).
            let params: Vec<String> = if args.len() > 1 {
                args[..args.len() - 1].iter()
                    .map(|v| crate::abstract_ops::to_string(v).as_str().to_string())
                    .collect()
            } else { Vec::new() };
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
            let source = format!(
                "{} = function anonymous({}) {{\n{}\n}};",
                stash_key, params.join(","), body
            );
            match rt.evaluate_module(&source, &url) {
                Ok(_ns) => {
                    let result = rt.globals.get(&stash_key).cloned().unwrap_or(Value::Undefined);
                    // Clean up the stash key — it was a side-channel,
                    // not a JS-visible global.
                    rt.globals.remove(&stash_key);
                    Ok(result)
                }
                Err(e) => Err(e),
            }
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
            if rt.evaluate_module(&expr_source, &url).is_ok() {
                let result = rt.globals.get(&stash_key).cloned().unwrap_or(Value::Undefined);
                rt.globals.remove(&stash_key);
                return Ok(result);
            }
            // Statement form: run as-is, no captured result.
            let stmt_url = format!("file://<eval:{}:stmt>", n);
            match rt.evaluate_module(&source, &stmt_url) {
                Ok(_) => Ok(Value::Undefined),
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
                self.obj_mut(fn_global).set_own_frozen("prototype".into(), Value::Object(fp));
                self.obj_mut(fp).set_own_internal("constructor".into(), Value::Object(fn_global));
            }
        }
    }

    fn install_math(&mut self) {
        let math = self.alloc_object(Object::new_ordinary());
        // Ω.5.P63.E10: Math unary one-liners routed through IR.
        register_intrinsic_method(self, math, "abs", 1, |rt, args| crate::generated::math_abs(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "floor", 1, |rt, args| crate::generated::math_floor(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "ceil", 1, |rt, args| crate::generated::math_ceil(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "round", 1, |rt, args| crate::generated::math_round(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "trunc", 1, |rt, args| crate::generated::math_trunc(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "sqrt", 1, |rt, args| crate::generated::math_sqrt(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "cbrt", 1, |rt, args| crate::generated::math_cbrt(rt, Value::Undefined, args));
        // Ω.5.P63.E14: pow / max / min routed through IR.
        register_intrinsic_method(self, math, "pow", 2, |rt, args| crate::generated::math_pow(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "max", 2, |rt, args| crate::generated::math_max(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "min", 2, |rt, args| crate::generated::math_min(rt, Value::Undefined, args));
        // Ω.5.P63.E10: Math.sign routed through IR. (Duplicate
        // installation at line ~1094 below is harmless: register order
        // overwrites and both paths produce identical results.)
        register_intrinsic_method(self, math, "sign", 1, |rt, args| crate::generated::math_sign(rt, Value::Undefined, args));
        // Ω.5.P63.E11: Math exp/log/trig family routed through IR.
        register_intrinsic_method(self, math, "exp", 1, |rt, args| crate::generated::math_exp(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "log", 1, |rt, args| crate::generated::math_log(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "log2", 1, |rt, args| crate::generated::math_log2(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "log10", 1, |rt, args| crate::generated::math_log10(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "sin", 1, |rt, args| crate::generated::math_sin(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "cos", 1, |rt, args| crate::generated::math_cos(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "tan", 1, |rt, args| crate::generated::math_tan(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "atan", 1, |rt, args| crate::generated::math_atan(rt, Value::Undefined, args));
        // Ω.5.P63.E11: asin / acos newly installed via IR (were missing from cruftless).
        register_intrinsic_method(self, math, "asin", 1, |rt, args| crate::generated::math_asin(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "acos", 1, |rt, args| crate::generated::math_acos(rt, Value::Undefined, args));
        // Ω.5.P63.E14: atan2 routed through IR.
        register_intrinsic_method(self, math, "atan2", 2, |rt, args| crate::generated::math_atan2(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "random", 0, |_rt, _|{
            // v1: simple LCG-style PRNG seeded from time. Not crypto-grade.
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(0);
            let pseudo = ((nanos as u64).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)) as f64;
            Ok(Value::Number((pseudo / u64::MAX as f64).abs().fract()))
        });
        // Ω.5.P62.E3: Math constants per ECMA §21.3.1 — all
        // { writable:false, enumerable:false, configurable:false }.
        self.obj_mut(math).set_own_frozen("PI".into(), Value::Number(std::f64::consts::PI));
        self.obj_mut(math).set_own_frozen("E".into(), Value::Number(std::f64::consts::E));
        self.obj_mut(math).set_own_frozen("LN2".into(), Value::Number(std::f64::consts::LN_2));
        self.obj_mut(math).set_own_frozen("LN10".into(), Value::Number(std::f64::consts::LN_10));
        self.obj_mut(math).set_own_frozen("LOG2E".into(), Value::Number(std::f64::consts::LOG2_E));
        self.obj_mut(math).set_own_frozen("LOG10E".into(), Value::Number(std::f64::consts::LOG10_E));
        self.obj_mut(math).set_own_frozen("SQRT2".into(), Value::Number(std::f64::consts::SQRT_2));
        // SQRT1_2 absent pre-E3.
        self.obj_mut(math).set_own_frozen("SQRT1_2".into(), Value::Number(std::f64::consts::FRAC_1_SQRT_2));

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
            let n = args.first().map(abstract_ops::to_number).unwrap_or(f64::NAN);
            if n.is_nan() { Ok(Value::Number(f64::NAN)) }
            else if n > 0.0 { Ok(Value::Number(1.0)) }
            else if n < 0.0 { Ok(Value::Number(-1.0)) }
            else { Ok(Value::Number(n)) } // preserves +0/-0
        });
        // Ω.5.P63.E11: expm1/log1p routed through IR.
        // (log2/log10 already routed above; this block previously
        // installed duplicates — preserve only the unique ones here.)
        register_intrinsic_method(self, math, "expm1", 1, |rt, args| crate::generated::math_expm1(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "log1p", 1, |rt, args| crate::generated::math_log1p(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "cbrt", 1, |_rt, args| {
            let n = args.first().map(abstract_ops::to_number).unwrap_or(f64::NAN);
            Ok(Value::Number(n.cbrt()))
        });
        // Ω.5.P63.E14: hypot routed through IR (variadic via Expr::AllArgs).
        register_intrinsic_method(self, math, "hypot", 2, |rt, args| crate::generated::math_hypot(rt, Value::Undefined, args));
        // Ω.5.P63.E11: hyperbolic family routed through IR.
        register_intrinsic_method(self, math, "sinh", 1, |rt, args| crate::generated::math_sinh(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "cosh", 1, |rt, args| crate::generated::math_cosh(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "tanh", 1, |rt, args| crate::generated::math_tanh(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "asinh", 1, |rt, args| crate::generated::math_asinh(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "acosh", 1, |rt, args| crate::generated::math_acosh(rt, Value::Undefined, args));
        register_intrinsic_method(self, math, "atanh", 1, |rt, args| crate::generated::math_atanh(rt, Value::Undefined, args));

        // Ω.5.P62.E4: Math[Symbol.toStringTag] === "Math" per ECMA §21.3.1.9.
        // Drives Object.prototype.toString.call(Math) → "[object Math]"
        // (test262 Array.prototype.map-1-10 + many ducktyping libs rely
        // on this).
        self.obj_mut(math).set_own_frozen("@@toStringTag".into(),
            Value::String(Rc::new("Math".into())));
        self.globals.insert("Math".into(), Value::Object(math));
    }

    fn install_json(&mut self) {
        let json = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, json, "stringify", 3, |rt, args|{
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            Ok(Value::String(Rc::new(json_stringify(rt, &v))))
        });
        register_intrinsic_method(self, json, "parse", 2, |rt, args|{
            let s = if let Some(v) = args.first() { abstract_ops::to_string(v) } else {
                return Err(RuntimeError::SyntaxError("JSON.parse requires a string".into()));
            };
            json_parse(rt, s.as_str())
        });
        // Ω.5.P62.E4: JSON[Symbol.toStringTag] === "JSON" per §25.5.1.5.
        self.obj_mut(json).set_own_frozen("@@toStringTag".into(),
            Value::String(Rc::new("JSON".into())));
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
            match args.first() {
                None | Some(Value::Undefined) | Some(Value::Null) => {
                    Ok(Value::Object(rt.alloc_object(Object::new_ordinary())))
                }
                Some(v @ Value::Object(_)) => Ok(v.clone()),
                Some(_) => Ok(Value::Object(rt.alloc_object(Object::new_ordinary()))),
            }
        });
        let obj_ctor = self.alloc_object(obj_ctor_native);
        // Ω.5.P63.E4: Object.keys routed through IR-lowered generated::object_keys.
        // The previous hand-written impl (with integer-index-first sort
        // + enumerable filter + @@-prefix filter) lives now in
        // rt.enumerable_own_keys, which generated::object_keys invokes
        // via CallBuiltin.
        register_intrinsic_method(self, obj_ctor, "keys", 1, |rt, args| {
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
        register_intrinsic_method(self, obj_ctor, "preventExtensions", 1, |rt, args| {
            crate::generated::object_prevent_extensions(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E6: isExtensible routed through IR.
        register_intrinsic_method(self, obj_ctor, "isExtensible", 1, |rt, args| {
            crate::generated::object_is_extensible(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "groupBy", 2, |rt, args| {
            // Object.groupBy(items, callbackFn) per ECMA §20.1.2.10.
            // Returns an Object whose keys are callback results and
            // values are arrays of items mapped to that key.
            let items = args.first().cloned().unwrap_or(Value::Undefined);
            let cb = args.get(1).cloned().ok_or_else(||
                RuntimeError::TypeError("Object.groupBy: callbackFn required".into()))?;
            let entries = collect_iterable(rt, items)?;
            let out = rt.alloc_object(Object::new_ordinary());
            for (i, v) in entries.into_iter().enumerate() {
                let key_v = rt.call_function(cb.clone(), Value::Undefined,
                    vec![v.clone(), Value::Number(i as f64)])?;
                let key = crate::abstract_ops::to_string(&key_v).as_str().to_string();
                // Append v to out[key] (array). Allocate array if missing.
                let arr_id = match rt.object_get(out, &key) {
                    Value::Object(id) => id,
                    _ => {
                        let a = rt.alloc_object(Object::new_array());
                        rt.object_set(out, key.clone(), Value::Object(a));
                        rt.object_set(a, "length".into(), Value::Number(0.0));
                        a
                    }
                };
                let n = rt.array_length(arr_id);
                rt.object_set(arr_id, n.to_string(), v);
                rt.object_set(arr_id, "length".into(), Value::Number((n + 1) as f64));
            }
            Ok(Value::Object(out))
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
        register_intrinsic_method(self, obj_ctor, "defineProperty", 3, |rt, args| {
            let target = match args.first() {
                Some(Value::Object(id)) => *id,
                other => return Err(RuntimeError::TypeError(format!(
                    "Object.defineProperty: target must be an object (got {})",
                    other.map(|v| match v {
                        Value::Undefined => "undefined".to_string(),
                        Value::Null => "null".to_string(),
                        Value::Boolean(_) => "boolean".to_string(),
                        Value::Number(_) => "number".to_string(),
                        Value::String(_) => "string".to_string(),
                        _ => "other".to_string(),
                    }).unwrap_or_else(|| "missing".into())
                ))),
            };
            // Ω.5.P61.E16: ToPropertyKey on the key argument per
            // ECMA §7.1.19. For Object values, dispatches @@toPrimitive /
            // toString / valueOf; throws TypeError if all return Objects.
            let key_v = args.get(1).cloned().unwrap_or(Value::Undefined);
            let key = rt.coerce_to_string(&key_v)?;
            let desc = args.get(2).cloned().unwrap_or(Value::Undefined);
            let desc_id = match desc {
                Value::Object(id) => id,
                _ => return Err(RuntimeError::TypeError("Object.defineProperty: descriptor must be an object".into())),
            };
            // Tier-Ω.5.nnn: accessor-descriptor support. If the descriptor
            // has a `get` or `set` function, store as accessor; else
            // treat as data descriptor (existing semantics).
            // Ω.5.P62.E12: ToPropertyDescriptor validation per §6.2.5.5.
            // Steps 5/7: if has "get"/"set" and it's not undefined+not
            // callable, throw TypeError. (Number/Boolean/String/etc. as
            // set/get triggers this.)
            let has_get_key = rt.has_property(desc_id, "get");
            let has_set_key = rt.has_property(desc_id, "set");
            let getter = if has_get_key { rt.read_property(desc_id, "get")? } else { Value::Undefined };
            let setter = if has_set_key { rt.read_property(desc_id, "set")? } else { Value::Undefined };
            if has_get_key && !matches!(getter, Value::Undefined) && !rt.is_callable(&getter) {
                return Err(RuntimeError::TypeError(
                    "Invalid property descriptor: getter must be callable".into()));
            }
            if has_set_key && !matches!(setter, Value::Undefined) && !rt.is_callable(&setter) {
                return Err(RuntimeError::TypeError(
                    "Invalid property descriptor: setter must be callable".into()));
            }
            let has_getter = matches!(&getter, Value::Object(_));
            let has_setter = matches!(&setter, Value::Object(_));
            // §6.2.5.5 step 10 — cannot mix accessor + data attrs.
            let has_value_key = rt.obj(desc_id).properties.contains_key("value")
                || rt.obj(desc_id).properties.contains_key("writable");
            let has_accessor_key = has_get_key || has_set_key;
            if has_value_key && has_accessor_key {
                return Err(RuntimeError::TypeError(
                    "Invalid property descriptor: cannot both specify accessors and a value or writable attribute".into()));
            }
            if has_getter || has_setter {
                rt.obj_mut(target).properties.insert(key, crate::value::PropertyDescriptor {
                    value: Value::Undefined,
                    writable: false, enumerable: true, configurable: true,
                    getter: if has_getter { Some(getter) } else { None },
                    setter: if has_setter { Some(setter) } else { None },
                });
            } else {
                // Tier-Ω.5.yyyyyy: per ECMA-262 §9.1.6.3 (ValidateAndApply
                // PropertyDescriptor), a *generic* data descriptor (no `value`
                // key on the descriptor object) does NOT erase the existing
                // property's [[Value]] — only writable/enumerable/configurable
                // attributes are updated. Without this preservation, Babel's
                // _createClass emits
                //   Object.defineProperty(e, "prototype", { writable: !1 })
                // and Foo.prototype gets clobbered to undefined; downstream
                // `Object.create(e && e.prototype, ...)` throws.
                // abortcontroller-polyfill / postcss-selector-parser /
                // protobufjs all hit this exact transpile pattern.
                // Tier-Ω.5.BBBBBBBB: honor the spec defaults for absent
                // attribute keys on a *new* data descriptor per ECMA §6.2.5.4
                // (CompletePropertyDescriptor): when the descriptor object
                // lacks writable/enumerable/configurable, they default to
                // false. Previously the engine used object_set, which
                // installed every property as enumerable=true; consumers
                // like color-convert's conversions.js then saw 'channels'
                // and 'labels' in Object.keys() despite being explicitly
                // defined as non-enumerable.
                let has_value = rt.obj(desc_id).properties.contains_key("value");
                let read_attr = |rt: &Runtime, name: &str| -> Option<bool> {
                    if !rt.obj(desc_id).properties.contains_key(name) { return None; }
                    match rt.object_get(desc_id, name) {
                        Value::Boolean(b) => Some(b),
                        Value::Undefined => Some(false),
                        Value::Null => Some(false),
                        Value::Number(n) => Some(n != 0.0),
                        Value::String(s) => Some(!s.as_str().is_empty()),
                        _ => Some(true),
                    }
                };
                let writable = read_attr(rt, "writable");
                let enumerable = read_attr(rt, "enumerable");
                let configurable = read_attr(rt, "configurable");
                let value = if has_value {
                    rt.object_get(desc_id, "value")
                } else {
                    // Preserve existing value when descriptor lacks `value`.
                    match rt.obj(target).properties.get(&key) {
                        Some(d) => d.value.clone(),
                        None => Value::Undefined,
                    }
                };
                // Existing flags act as the starting point when descriptor
                // is partial (matches CompletePropertyDescriptor for
                // already-present properties); for a new property, absent
                // attributes default to false.
                let exists = rt.obj(target).properties.contains_key(&key);
                let (default_w, default_e, default_c, existing_value) = if exists {
                    let d = &rt.obj(target).properties[&key];
                    (d.writable, d.enumerable, d.configurable, d.value.clone())
                } else {
                    (false, false, false, Value::Undefined)
                };
                let new_w = writable.unwrap_or(default_w);
                let new_e = enumerable.unwrap_or(default_e);
                let new_c = configurable.unwrap_or(default_c);
                // Ω.5.P61.E9: ValidateAndApplyPropertyDescriptor per ECMA
                // §10.1.6.2. When existing is non-configurable, redefining
                // it to differ in attributes/value (beyond writable:
                // true→false, which is permitted) throws TypeError.
                if exists && !default_c {
                    let value_changed = has_value && !crate::abstract_ops::is_strictly_equal(&value, &existing_value);
                    let attrs_changed = new_e != default_e || new_c != default_c
                        || (default_w == false && new_w == true); // writable false → true not allowed
                    // Going from writable:true → writable:false IS allowed
                    // per §10.1.6.3 step 4.a even if non-configurable.
                    // Value change is allowed only if existing is writable.
                    let value_change_allowed = if value_changed { default_w } else { true };
                    if attrs_changed || !value_change_allowed {
                        return Err(RuntimeError::TypeError(format!(
                            "Cannot redefine non-configurable property '{}'", key
                        )));
                    }
                }
                rt.obj_mut(target).properties.insert(key, crate::value::PropertyDescriptor {
                    value,
                    writable: new_w,
                    enumerable: new_e,
                    configurable: new_c,
                    getter: None,
                    setter: None,
                });
            }
            Ok(Value::Object(target))
        });
        register_intrinsic_method(self, obj_ctor, "defineProperties", 2, |rt, args| {
            let target = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Err(RuntimeError::TypeError("Object.defineProperties: target must be an object".into())),
            };
            let props = match args.get(1) {
                Some(Value::Object(id)) => *id,
                _ => return Err(RuntimeError::TypeError("Object.defineProperties: props must be an object".into())),
            };
            // Snapshot own enumerable keys + descriptor objects.
            let entries: Vec<(String, Value)> = {
                let o = rt.obj(props);
                o.properties.iter()
                    .filter(|(_, d)| d.enumerable)
                    .map(|(k, d)| (k.clone(), d.value.clone()))
                    .collect()
            };
            for (k, dv) in entries {
                if let Value::Object(did) = dv {
                    // Ω.5.P61.E15: full descriptor semantics per ECMA
                    // §20.1.2.4 + §6.2.5.4. Pre-E15 the data path called
                    // object_set which defaulted everything to enumerable;
                    // accessor path forced enumerable:true. Now reads the
                    // descriptor's own writable/enumerable/configurable
                    // (default false when absent) per CompletePropertyDescriptor.
                    let read_bool = |rt: &Runtime, name: &str| -> Option<bool> {
                        if !rt.obj(did).properties.contains_key(name) { return None; }
                        match rt.object_get(did, name) {
                            Value::Boolean(b) => Some(b),
                            Value::Undefined | Value::Null => Some(false),
                            _ => Some(true),
                        }
                    };
                    let getter = rt.object_get(did, "get");
                    let setter = rt.object_get(did, "set");
                    let has_getter = matches!(&getter, Value::Object(_));
                    let has_setter = matches!(&setter, Value::Object(_));
                    let writable = read_bool(rt, "writable").unwrap_or(false);
                    let enumerable = read_bool(rt, "enumerable").unwrap_or(false);
                    let configurable = read_bool(rt, "configurable").unwrap_or(false);
                    let value = if rt.obj(did).properties.contains_key("value") {
                        rt.object_get(did, "value")
                    } else { Value::Undefined };
                    rt.obj_mut(target).properties.insert(k, crate::value::PropertyDescriptor {
                        value,
                        writable, enumerable, configurable,
                        getter: if has_getter { Some(getter) } else { None },
                        setter: if has_setter { Some(setter) } else { None },
                    });
                }
            }
            Ok(Value::Object(target))
        });
        register_intrinsic_method(self, obj_ctor, "getOwnPropertyDescriptor", 2, |rt, args| {
            let id = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Ok(Value::Undefined),
            };
            // Ω.5.P61.E18: ToPropertyKey per ECMA §7.1.19 — dispatch through
            // @@toPrimitive/toString/valueOf so non-string keys (Array via
            // toString-join, Object via @@toPrimitive, etc.) coerce per spec.
            // Plain ToString gave "[object Object]" for arrays, breaking
            // gOPD-2-{38..47} which pass [1]/[true]/etc. as the key.
            let key = rt.coerce_to_string(&args.get(1).cloned().unwrap_or(Value::Undefined))?;
            // Tier-Ω.5.ZZZZZZZ: distinguish accessor descriptors (return {get,
            // set, enumerable, configurable}) from data descriptors (return
            // {value, writable, enumerable, configurable}). Previously
            // accessor properties had their getter/setter dropped, returning
            // a data-shape descriptor whose .value was undefined and whose
            // consumers read .get and got undefined.
            // safe-stable-stringify (under roarr / slonik / mongoose) does:
            //   Object.getOwnPropertyDescriptor(TypedArrayProto, @@toStringTag).get
            let (has, value, writable, enumerable, configurable, getter, setter) = {
                let o = rt.obj(id);
                match o.properties.get(&key) {
                    Some(d) => (true, d.value.clone(), d.writable, d.enumerable, d.configurable, d.getter.clone(), d.setter.clone()),
                    None => (false, Value::Undefined, false, false, false, None, None),
                }
            };
            if !has { return Ok(Value::Undefined); }
            let out = rt.alloc_object(Object::new_ordinary());
            if getter.is_some() || setter.is_some() {
                rt.object_set(out, "get".into(), getter.unwrap_or(Value::Undefined));
                rt.object_set(out, "set".into(), setter.unwrap_or(Value::Undefined));
            } else {
                rt.object_set(out, "value".into(), value);
                rt.object_set(out, "writable".into(), Value::Boolean(writable));
            }
            rt.object_set(out, "enumerable".into(), Value::Boolean(enumerable));
            rt.object_set(out, "configurable".into(), Value::Boolean(configurable));
            Ok(Value::Object(out))
        });
        // Tier-Ω.5.rrrrrr: Object.getOwnPropertyDescriptors per §20.1.2.10.
        register_intrinsic_method(self, obj_ctor, "getOwnPropertyDescriptors", 1, |rt, args| {
            let id = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Ok(Value::Object(rt.alloc_object(Object::new_ordinary()))),
            };
            let entries: Vec<(String, Value, bool, bool, bool, Option<Value>, Option<Value>)> = {
                let o = rt.obj(id);
                o.properties.iter().map(|(k, d)| (k.clone(), d.value.clone(), d.writable, d.enumerable, d.configurable, d.getter.clone(), d.setter.clone())).collect()
            };
            let out = rt.alloc_object(Object::new_ordinary());
            for (k, v, w, e, c, getter, setter) in entries {
                let desc = rt.alloc_object(Object::new_ordinary());
                if let Some(g) = getter { rt.object_set(desc, "get".into(), g); }
                if let Some(s) = setter { rt.object_set(desc, "set".into(), s); }
                if !matches!(v, Value::Undefined) { rt.object_set(desc, "value".into(), v); }
                rt.object_set(desc, "writable".into(), Value::Boolean(w));
                rt.object_set(desc, "enumerable".into(), Value::Boolean(e));
                rt.object_set(desc, "configurable".into(), Value::Boolean(c));
                rt.object_set(out, k, Value::Object(desc));
            }
            Ok(Value::Object(out))
        });
        // Ω.5.P63.E15: getOwnPropertyNames routed through IR.
        register_intrinsic_method(self, obj_ctor, "getOwnPropertyNames", 1, |rt, args| {
            crate::generated::object_get_own_property_names(rt, Value::Undefined, args)
        });
        // Tier-Ω.5.LLLLLLLL: Object.getOwnPropertySymbols per ECMA-262 §20.1.2.11.
        // V1 representation: symbols are strings prefixed '@@'; return only the
        // own '@@' keys as String values (consumers that compare via Symbol.X
        // get the same string). Sufficient for define-properties-checks
        // (es-define-property / set-function-length / onetime) which probe
        // for Symbol.toStringTag / iterator placement.
        // Ω.5.P63.E15: getOwnPropertySymbols routed through IR.
        register_intrinsic_method(self, obj_ctor, "getOwnPropertySymbols", 1, |rt, args| {
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
        register_intrinsic_method(self, obj_ctor, "create", 2, |rt, args| {
            let proto_arg = args.first().cloned().unwrap_or(Value::Undefined);
            let proto_id = match proto_arg {
                Value::Null => None,
                Value::Object(id) => Some(id),
                _ => return Err(RuntimeError::TypeError(
                    "Object.create: prototype must be Object or null".into())),
            };
            let mut obj = Object::new_ordinary();
            obj.proto = proto_id;
            let id = rt.alloc_object(obj);
            if let Some(Value::Object(props_id)) = args.get(1) {
                let entries: Vec<(String, Value)> = {
                    let o = rt.obj(*props_id);
                    o.properties.iter()
                        .filter(|(_, d)| d.enumerable)
                        .map(|(k, d)| (k.clone(), d.value.clone()))
                        .collect()
                };
                for (k, dv) in entries {
                    let v = match dv {
                        Value::Object(did) => rt.object_get(did, "value"),
                        _ => Value::Undefined,
                    };
                    rt.object_set(id, k, v);
                }
            }
            Ok(Value::Object(id))
        });
        // Tier-Ω.5.nn: Object.getPrototypeOf + Object.setPrototypeOf.
        // axios + many others destructure `const { getPrototypeOf } = Object;`
        // at module top level. Without these statics, getPrototypeOf is
        // undefined and `getPrototypeOf(Uint8Array)` errors. The Reflect
        // variant existed (Ω.5.cc) but consumer code uses Object.X.
        // Ω.5.P63.E6: getPrototypeOf / setPrototypeOf routed through IR.
        register_intrinsic_method(self, obj_ctor, "getPrototypeOf", 1, |rt, args| {
            crate::generated::object_get_prototype_of(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "setPrototypeOf", 2, |rt, args| {
            crate::generated::object_set_prototype_of(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, obj_ctor, "create", 2, |rt, args| {
            // Ω.5.P61.E15: apply full descriptor semantics per ECMA
            // §20.1.2.2 (Object.create) + §10.1.6.1 OrdinaryDefineOwnProperty.
            // Pre-E15 the impl wrote values via object_set which defaulted
            // writable/enumerable/configurable to true. Per §6.2.5.4
            // CompletePropertyDescriptor the absent attributes default to
            // false on a new property.
            let proto_arg = args.first().cloned().unwrap_or(Value::Undefined);
            let mut obj = Object::new_ordinary();
            obj.proto = match proto_arg {
                Value::Object(id) => Some(id),
                Value::Null => None,
                _ => return Err(RuntimeError::TypeError("Object.create: prototype must be object or null".into())),
            };
            let id = rt.alloc_object(obj);
            if let Some(Value::Object(props_id)) = args.get(1) {
                let props_id = *props_id;
                let keys: Vec<String> = rt.obj(props_id).properties.iter()
                    .filter(|(_, d)| d.enumerable)
                    .map(|(k, _)| k.clone())
                    .collect();
                for k in keys {
                    // §20.1.2.2 step 5 + §10.1.6.1: Get(props, key) dispatches
                    // accessor getters per §10.1.8.1.
                    let dv = rt.read_property(props_id, &k)?;
                    let did = match dv {
                        Value::Object(d) => d,
                        _ => return Err(RuntimeError::TypeError(
                            "Property description must be an object".into())),
                    };
                    // Ω.5.P62.E11: ToPropertyDescriptor per ECMA §6.2.5.5.
                    let has = |rt: &mut Runtime, name: &str| -> bool {
                        rt.has_property(did, name)
                    };
                    let has_value = has(rt, "value");
                    let has_writable = has(rt, "writable");
                    let has_get = has(rt, "get");
                    let has_set = has(rt, "set");
                    // Step 5/7: get + set must be callable or undefined.
                    let getter_v = if has_get { rt.read_property(did, "get")? } else { Value::Undefined };
                    let setter_v = if has_set { rt.read_property(did, "set")? } else { Value::Undefined };
                    if has_get && !matches!(getter_v, Value::Undefined) && !rt.is_callable(&getter_v) {
                        return Err(RuntimeError::TypeError(
                            "Object.create: getter must be callable".into()));
                    }
                    if has_set && !matches!(setter_v, Value::Undefined) && !rt.is_callable(&setter_v) {
                        return Err(RuntimeError::TypeError(
                            "Object.create: setter must be callable".into()));
                    }
                    // Step 9: cannot mix data + accessor.
                    if (has_value || has_writable) && (has_get || has_set) {
                        return Err(RuntimeError::TypeError(
                            "Object.create: cannot both specify accessors and a value or writable".into()));
                    }
                    let read_bool = |rt: &mut Runtime, name: &str| -> Result<Option<bool>, RuntimeError> {
                        if !rt.has_property(did, name) { return Ok(None); }
                        let v = rt.read_property(did, name)?;
                        Ok(Some(crate::abstract_ops::to_boolean(&v)))
                    };
                    let writable = read_bool(rt, "writable")?.unwrap_or(false);
                    let enumerable = read_bool(rt, "enumerable")?.unwrap_or(false);
                    let configurable = read_bool(rt, "configurable")?.unwrap_or(false);
                    let value = if has_value { rt.read_property(did, "value")? } else { Value::Undefined };
                    let has_getter = matches!(getter_v, Value::Object(_));
                    let has_setter = matches!(setter_v, Value::Object(_));
                    rt.obj_mut(id).properties.insert(k, crate::value::PropertyDescriptor {
                        value,
                        writable, enumerable, configurable,
                        getter: if has_getter { Some(getter_v) } else { None },
                        setter: if has_setter { Some(setter_v) } else { None },
                    });
                }
            }
            Ok(Value::Object(id))
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
            self.obj_mut(obj_ctor).set_own_frozen("prototype".into(), Value::Object(proto));
            // Tier-Ω.5.lll: Object.prototype.constructor = Object. Per
            // ECMA-262 §20.1.3.1. Without this, plain-object `.constructor`
            // returns undefined, breaking type-tag idioms like dequal's
            // `(ctor=foo.constructor) === bar.constructor` followed by
            // `ctor === Date` / `ctor === RegExp` / `ctor === Array`
            // dispatch.
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(obj_ctor));
        }
        self.globals.insert("Object".into(), Value::Object(obj_ctor));
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
                Value::Object(id) if matches!(rt.obj(id).internal_kind, InternalKind::Array) => Some(id),
                _ => None,
            };
            let id = match receiver_id {
                Some(id) => id,
                None => rt.alloc_object(Object::new_array()),
            };
            if args.len() == 1 {
                if let Value::Number(n) = &args[0] {
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
            let src = args.first().cloned().unwrap_or(Value::Undefined);
            let map_fn = args.get(1).cloned();
            let out = rt.alloc_object(Object::new_array());
            // Two shapes: iterable (has @@iterator) or array-like (has length).
            let items: Vec<Value> = match &src {
                Value::Object(id) => {
                    // Try iterator first.
                    let has_iter = !matches!(rt.object_get(*id, "@@iterator"), Value::Undefined);
                    if has_iter {
                        collect_iterable(rt, src.clone())?
                    } else {
                        let len = rt.array_length(*id);
                        (0..len).map(|i| rt.object_get(*id, &i.to_string())).collect()
                    }
                }
                Value::String(s) => s.chars().map(|c| Value::String(Rc::new(c.to_string()))).collect(),
                _ => Vec::new(),
            };
            for (i, v) in items.into_iter().enumerate() {
                let mapped = if let Some(f) = &map_fn {
                    rt.call_function(f.clone(), Value::Undefined, vec![v, Value::Number(i as f64)])?
                } else { v };
                rt.object_set(out, i.to_string(), mapped);
            }
            let len = rt.array_length(out);
            rt.object_set(out, "length".into(), Value::Number(len as f64));
            Ok(Value::Object(out))
        });
        if let Some(proto) = self.array_prototype {
            self.obj_mut(arr_ctor).set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P58.E4: Array.prototype.constructor = Array per ECMA §10.2.12.
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(arr_ctor));
        }
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
            let n = if args.is_empty() { 0.0 } else { rt.coerce_to_number(&v)? };
            if rt.current_new_target.is_some() {
                let mut obj = crate::value::Object::new_ordinary();
                obj.set_own_internal("__primitive__".into(), Value::Number(n));
                let proto = match rt.globals.get("Number").cloned() {
                    Some(Value::Object(id)) => match rt.object_get(id, "prototype") {
                        Value::Object(p) => Some(p), _ => None,
                    },
                    _ => None,
                };
                if let Some(p) = proto { obj.proto = Some(p); }
                let id = rt.alloc_object(obj);
                return Ok(Value::Object(id));
            }
            Ok(Value::Number(n))
        });
        let num = self.alloc_object(num_obj);
        // Constants per ECMA-262 §21.1.2.
        // Ω.5.P62.E3: Number namespace constants per ECMA §21.1.2 — all
        // { writable:false, enumerable:false, configurable:false }.
        self.obj_mut(num).set_own_frozen("MAX_SAFE_INTEGER".into(), Value::Number(9007199254740991.0));
        self.obj_mut(num).set_own_frozen("MIN_SAFE_INTEGER".into(), Value::Number(-9007199254740991.0));
        self.obj_mut(num).set_own_frozen("MAX_VALUE".into(), Value::Number(f64::MAX));
        self.obj_mut(num).set_own_frozen("MIN_VALUE".into(), Value::Number(5e-324));
        self.obj_mut(num).set_own_frozen("EPSILON".into(), Value::Number(f64::EPSILON));
        self.obj_mut(num).set_own_frozen("POSITIVE_INFINITY".into(), Value::Number(f64::INFINITY));
        self.obj_mut(num).set_own_frozen("NEGATIVE_INFINITY".into(), Value::Number(f64::NEG_INFINITY));
        self.obj_mut(num).set_own_frozen("NaN".into(), Value::Number(f64::NAN));
        // Tier-Ω.5.ggggg: global Infinity / NaN / undefined per ECMA-262
        // §19.1. acorn's tokenizer uses `Infinity` as a sentinel in
        // `for (var i=0, e=Infinity; i<e; ...)`; without the global,
        // i<undefined is false, the loop never runs, every numeric literal
        // fails to tokenize.
        self.globals.insert("Infinity".into(), Value::Number(f64::INFINITY));
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
            self.obj_mut(num).set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P58.E4: Number.prototype.constructor = Number per ECMA §10.2.12.
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(num));
            // Ω.5.P62.E19: Number.prototype is a Number exotic with
            // [[NumberData]] = +0 per §21.1.4. Brand-checked methods
            // (toString/toFixed/valueOf) must accept Number.prototype
            // directly (Number.prototype.toString() returns "0").
            self.obj_mut(proto).set_own_internal("__primitive__".into(), Value::Number(0.0));
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
                        "Cannot convert a Symbol value to a string".into()));
                }
                Rc::new(abstract_ops::to_string(&v).as_str().to_string())
            } else {
                Rc::new(rt.coerce_to_string(&v)?)
            };
            if rt.current_new_target.is_some() {
                let mut obj = crate::value::Object::new_ordinary();
                obj.set_own_internal("__primitive__".into(), Value::String(s_rc.clone()));
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
                        Value::Object(p) => Some(p), _ => None,
                    },
                    _ => None,
                };
                if let Some(p) = proto { obj.proto = Some(p); }
                let id = rt.alloc_object(obj);
                return Ok(Value::Object(id));
            }
            Ok(Value::String(s_rc))
        });
        let str_id = self.alloc_object(str_obj);
        register_intrinsic_method(self, str_id, "fromCharCode", 1, |_rt, args| {
            let mut s = String::new();
            for a in args {
                let n = abstract_ops::to_number(a);
                if let Some(c) = char::from_u32(n as u32) { s.push(c); }
            }
            Ok(Value::String(Rc::new(s)))
        });
        register_intrinsic_method(self, str_id, "fromCodePoint", 1, |_rt, args| {
            let mut s = String::new();
            for a in args {
                let n = abstract_ops::to_number(a);
                if let Some(c) = char::from_u32(n as u32) { s.push(c); }
            }
            Ok(Value::String(Rc::new(s)))
        });
        // Tier-Ω.5.ww.b: String.raw(template, ...subs). Spec uses
        // template.raw; v1 falls back to indexed cooked values from the
        // strings array (Tier-Ω.5.ww doesn't populate .raw yet). Sufficient
        // for the camelcase / consola / styled-components patterns where
        // .raw vs cooked agree (no escape sequences requiring raw).
        register_intrinsic_method(self, str_id, "raw", 1, |rt, args| {
            let template = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Err(RuntimeError::TypeError("String.raw: first argument must be an object".into())),
            };
            let raw = match rt.object_get(template, &"raw".to_string()) {
                Value::Undefined => Value::Object(template),
                v => v,
            };
            let raw_id = match raw {
                Value::Object(id) => id,
                _ => return Err(RuntimeError::TypeError("String.raw: raw must be an object".into())),
            };
            let length = match rt.object_get(raw_id, &"length".to_string()) {
                Value::Number(n) => n as i64,
                _ => {
                    let mut n: i64 = 0;
                    while !matches!(rt.object_get(raw_id, &n.to_string()), Value::Undefined) {
                        n += 1;
                    }
                    n
                }
            };
            let mut out = String::new();
            for i in 0..length {
                let seg = rt.object_get(raw_id, &i.to_string());
                out.push_str(&abstract_ops::to_string(&seg));
                if i + 1 < length {
                    if let Some(sub) = args.get((i as usize) + 1) {
                        out.push_str(&abstract_ops::to_string(sub));
                    }
                }
            }
            Ok(Value::String(Rc::new(out)))
        });
        if let Some(proto) = self.string_prototype {
            self.obj_mut(str_id).set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P58.E4: Constructor.prototype.constructor === Constructor
            // per ECMA-262 §10.2.12. ast-types' Type.from uses indexOf on
            // builtInCtorFns (which holds `"x".constructor`, `(123).constructor`,
            // etc.) to recognize built-in types. Pre-P58.E4 cruftless's
            // String.prototype.constructor was a separate Object (named
            // "Object"), so `"x".constructor === String` returned false and
            // the ast-types lookup fell through to the `missing name` throw.
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(str_id));
            // Ω.5.P62.E19: String.prototype is a String exotic with
            // [[StringData]] = "" per §22.1.4.
            self.obj_mut(proto).set_own_internal("__primitive__".into(),
                Value::String(Rc::new(String::new())));
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
                rest = &rest[i+3..];
                (p, true)
            } else if let Some(i) = rest.find(':') {
                let p = format!("{}:", &rest[..i]);
                rest = &rest[i+1..];
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
            let path_s = if path.is_empty() && after_scheme { "/".to_string() } else { path.to_string() };
            let (userinfo, hostport) = match authority.rfind('@') {
                Some(i) => (&authority[..i], &authority[i+1..]),
                None => ("", authority),
            };
            let (username, password) = match userinfo.find(':') {
                Some(i) => (&userinfo[..i], &userinfo[i+1..]),
                None => (userinfo, ""),
            };
            let (hostname, port) = if hostport.starts_with('[') {
                // IPv6 literal.
                match hostport.find("]:") {
                    Some(i) => (&hostport[..=i], &hostport[i+2..]),
                    None => (hostport, ""),
                }
            } else {
                match hostport.rfind(':') {
                    Some(i) => (&hostport[..i], &hostport[i+1..]),
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
            rt.object_set(url_obj, "href".into(), Value::String(Rc::new(href)));
            rt.object_set(url_obj, "protocol".into(), Value::String(Rc::new(protocol)));
            rt.object_set(url_obj, "username".into(), Value::String(Rc::new(username.into())));
            rt.object_set(url_obj, "password".into(), Value::String(Rc::new(password.into())));
            rt.object_set(url_obj, "host".into(), Value::String(Rc::new(hostport.into())));
            rt.object_set(url_obj, "hostname".into(), Value::String(Rc::new(hostname.into())));
            rt.object_set(url_obj, "port".into(), Value::String(Rc::new(port.into())));
            rt.object_set(url_obj, "pathname".into(), Value::String(Rc::new(path_s)));
            rt.object_set(url_obj, "search".into(), Value::String(Rc::new(search)));
            rt.object_set(url_obj, "hash".into(), Value::String(Rc::new(hash)));
            rt.object_set(url_obj, "origin".into(), Value::String(Rc::new(origin)));
            register_method(rt, url_obj, "toString", |rt, _args| {
                Ok(rt.object_get(match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(Rc::new(String::new()))) }, "href"))
            });
            register_method(rt, url_obj, "toJSON", |rt, _args| {
                Ok(rt.object_get(match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(Rc::new(String::new()))) }, "href"))
            });
            Ok(Value::Object(url_obj))
        });
        let url_id = self.alloc_object(url_ctor);
        let url_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(url_id).set_own_frozen("prototype".into(), Value::Object(url_proto));
        register_method(self, url_id, "canParse", |_rt, args| {
            let s = match args.first() { Some(Value::String(s)) => s.as_str().to_string(), _ => return Ok(Value::Boolean(false)) };
            Ok(Value::Boolean(s.contains("://") || s.starts_with("file:") || s.starts_with("data:")))
        });
        self.globals.insert("URL".into(), Value::Object(url_id));

        // Tier-Ω.5.AAAAAAA: AbortController + AbortSignal globals per WHATWG DOM
        // AbortController interface. execa / node-fetch / undici-style HTTP
        // consumers do `new AbortController()` and reference `.signal` at
        // module-init or in the closure that defines a request helper. Class
        // shape needs to exist for the class compile / instance construction
        // to resolve; the signal's `aborted`/`reason`/`onabort` slots are
        // present on the prototype (false / undefined respectively); abort()
        // flips `signal.aborted` to true. Event-dispatch to listeners is
        // deferred — sufficient for load-time presence and the most common
        // sync-check pattern (`if (signal.aborted) { ... }`).
        let abort_signal_proto = self.alloc_object(Object::new_ordinary());
        let abort_signal_ctor = make_native("AbortSignal", |_rt, _args| {
            Err(RuntimeError::TypeError(
                "AbortSignal constructor not directly callable (use AbortController) — Tier-Ω.5.AAAAAAA stub".into()
            ))
        });
        let abort_signal_id = self.alloc_object(abort_signal_ctor);
        self.obj_mut(abort_signal_id).set_own_frozen("prototype".into(), Value::Object(abort_signal_proto));
        self.obj_mut(abort_signal_proto).set_own_internal("constructor".into(), Value::Object(abort_signal_id));
        // Static factories per spec §3.1.3.
        register_method(self, abort_signal_id, "abort", |rt, args| {
            let sig = rt.alloc_object(Object::new_ordinary());
            rt.object_set(sig, "aborted".into(), Value::Boolean(true));
            rt.object_set(sig, "reason".into(), args.first().cloned().unwrap_or(Value::Undefined));
            Ok(Value::Object(sig))
        });
        register_method(self, abort_signal_id, "timeout", |rt, _args| {
            let sig = rt.alloc_object(Object::new_ordinary());
            rt.object_set(sig, "aborted".into(), Value::Boolean(false));
            rt.object_set(sig, "reason".into(), Value::Undefined);
            Ok(Value::Object(sig))
        });
        self.globals.insert("AbortSignal".into(), Value::Object(abort_signal_id));

        let abort_controller_proto = self.alloc_object(Object::new_ordinary());
        let abort_controller_ctor = make_native("AbortController", |rt, _args| {
            let inst = rt.alloc_object(Object::new_ordinary());
            let sig = rt.alloc_object(Object::new_ordinary());
            rt.object_set(sig, "aborted".into(), Value::Boolean(false));
            rt.object_set(sig, "reason".into(), Value::Undefined);
            rt.object_set(sig, "onabort".into(), Value::Null);
            rt.object_set(inst, "signal".into(), Value::Object(sig));
            Ok(Value::Object(inst))
        });
        let abort_controller_id = self.alloc_object(abort_controller_ctor);
        self.obj_mut(abort_controller_id).set_own_frozen("prototype".into(), Value::Object(abort_controller_proto));
        self.obj_mut(abort_controller_proto).set_own_internal("constructor".into(), Value::Object(abort_controller_id));
        self.globals.insert("AbortController".into(), Value::Object(abort_controller_id));

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
        self.obj_mut(usp_id).set_own_frozen("prototype".into(), Value::Object(usp_proto));
        self.obj_mut(usp_proto).set_own_internal("constructor".into(), Value::Object(usp_id));
        self.globals.insert("URLSearchParams".into(), Value::Object(usp_id));

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
            "Response", "FormData", "Blob", "File",
            "ReadableStream", "WritableStream", "TransformStream",
            "ReadableStreamDefaultReader", "ReadableStreamBYOBReader",
            "ReadableStreamDefaultController", "ReadableByteStreamController",
            "WritableStreamDefaultWriter", "WritableStreamDefaultController",
            "TransformStreamDefaultController",
            "ByteLengthQueuingStrategy", "CountQueuingStrategy",
            "TextEncoderStream", "TextDecoderStream",
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
            self.obj_mut(id).set_own_frozen("prototype".into(), Value::Object(proto));
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(id));
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
            let bag = rt.alloc_object(Object::new_ordinary());
            rt.object_set(id, "__headers".into(), Value::Object(bag));
            // Init from arg 0: undefined / Object / Array / Headers-instance.
            if let Some(init) = args.first() {
                if let Value::Object(src) = init {
                    // Try as plain object: copy own enumerable string keys.
                    let pairs: Vec<(String, Value)> = rt.obj(*src).properties
                        .iter()
                        .filter(|(k, d)| d.enumerable && k.as_str() != "__headers")
                        .map(|(k, d)| (k.clone(), d.value.clone()))
                        .collect();
                    for (k, v) in pairs {
                        let lk = k.to_ascii_lowercase();
                        let s = abstract_ops::to_string(&v).as_str().to_string();
                        rt.object_set(bag, lk, Value::String(Rc::new(s)));
                    }
                    // If the src is itself a Headers instance, fold in its __headers too.
                    if let Value::Object(src_bag) = rt.object_get(*src, "__headers") {
                        let inner: Vec<(String, Value)> = rt.obj(src_bag).properties
                            .iter().map(|(k, d)| (k.clone(), d.value.clone())).collect();
                        for (k, v) in inner {
                            rt.object_set(bag, k, v);
                        }
                    }
                }
            }
            Ok(Value::Object(id))
        });
        let headers_ctor_id = self.alloc_object(headers_ctor_fn);
        self.obj_mut(headers_ctor_id).set_own_frozen("prototype".into(), Value::Object(headers_proto));
        self.obj_mut(headers_proto).set_own_internal("constructor".into(), Value::Object(headers_ctor_id));
        register_method(self, headers_proto, "has", |rt, args| {
            let this_id = match rt.current_this() { Value::Object(o) => o, _ => return Ok(Value::Boolean(false)) };
            let bag = match rt.object_get(this_id, "__headers") { Value::Object(b) => b, _ => return Ok(Value::Boolean(false)) };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str().to_ascii_lowercase();
            Ok(Value::Boolean(!matches!(rt.object_get(bag, &name), Value::Undefined)))
        });
        register_method(self, headers_proto, "get", |rt, args| {
            let this_id = match rt.current_this() { Value::Object(o) => o, _ => return Ok(Value::Null) };
            let bag = match rt.object_get(this_id, "__headers") { Value::Object(b) => b, _ => return Ok(Value::Null) };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str().to_ascii_lowercase();
            match rt.object_get(bag, &name) {
                Value::Undefined => Ok(Value::Null),
                v => Ok(v),
            }
        });
        register_method(self, headers_proto, "set", |rt, args| {
            let this_id = match rt.current_this() { Value::Object(o) => o, _ => return Ok(Value::Undefined) };
            let bag = match rt.object_get(this_id, "__headers") { Value::Object(b) => b, _ => return Ok(Value::Undefined) };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str().to_ascii_lowercase();
            let value = abstract_ops::to_string(&args.get(1).cloned().unwrap_or(Value::Undefined))
                .as_str().to_string();
            rt.object_set(bag, name, Value::String(Rc::new(value)));
            Ok(Value::Undefined)
        });
        register_method(self, headers_proto, "append", |rt, args| {
            let this_id = match rt.current_this() { Value::Object(o) => o, _ => return Ok(Value::Undefined) };
            let bag = match rt.object_get(this_id, "__headers") { Value::Object(b) => b, _ => return Ok(Value::Undefined) };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str().to_ascii_lowercase();
            let value = abstract_ops::to_string(&args.get(1).cloned().unwrap_or(Value::Undefined))
                .as_str().to_string();
            let existing = rt.object_get(bag, &name);
            let combined = match existing {
                Value::String(s) => format!("{}, {}", s, value),
                _ => value,
            };
            rt.object_set(bag, name, Value::String(Rc::new(combined)));
            Ok(Value::Undefined)
        });
        register_method(self, headers_proto, "delete", |rt, args| {
            let this_id = match rt.current_this() { Value::Object(o) => o, _ => return Ok(Value::Undefined) };
            let bag = match rt.object_get(this_id, "__headers") { Value::Object(b) => b, _ => return Ok(Value::Undefined) };
            let name = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
                .as_str().to_ascii_lowercase();
            rt.object_set(bag, name, Value::Undefined);
            Ok(Value::Undefined)
        });
        register_method(self, headers_proto, "forEach", |rt, args| {
            let this_id = match rt.current_this() { Value::Object(o) => o, _ => return Ok(Value::Undefined) };
            let bag = match rt.object_get(this_id, "__headers") { Value::Object(b) => b, _ => return Ok(Value::Undefined) };
            let cb = args.first().cloned().unwrap_or(Value::Undefined);
            let pairs: Vec<(String, Value)> = rt.obj(bag).properties
                .iter().map(|(k, d)| (k.clone(), d.value.clone())).collect();
            for (k, v) in pairs {
                rt.call_function(cb.clone(), Value::Undefined,
                    vec![v, Value::String(Rc::new(k)), Value::Object(this_id)])?;
            }
            Ok(Value::Undefined)
        });
        self.globals.insert("Headers".into(), Value::Object(headers_ctor_id));

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
            let url = args.first().cloned().unwrap_or(Value::String(Rc::new(String::new())));
            rt.object_set(id, "url".into(), url);
            let opts = args.get(1).cloned().unwrap_or(Value::Undefined);
            let (method, body, headers_init) = if let Value::Object(opts_id) = &opts {
                let m = rt.object_get(*opts_id, "method");
                let b = rt.object_get(*opts_id, "body");
                let h = rt.object_get(*opts_id, "headers");
                (m, b, h)
            } else { (Value::Undefined, Value::Undefined, Value::Undefined) };
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
                    let bag = rt.alloc_object(Object::new_ordinary());
                    rt.object_set(h_id, "__headers".into(), Value::Object(bag));
                    if let Value::Object(src) = headers_init {
                        let pairs: Vec<(String, Value)> = rt.obj(src).properties
                            .iter()
                            .filter(|(k, d)| d.enumerable && k.as_str() != "__headers")
                            .map(|(k, d)| (k.clone(), d.value.clone()))
                            .collect();
                        for (k, v) in pairs {
                            let lk = k.to_ascii_lowercase();
                            let s = abstract_ops::to_string(&v).as_str().to_string();
                            rt.object_set(bag, lk, Value::String(Rc::new(s)));
                        }
                        if let Value::Object(src_bag) = rt.object_get(src, "__headers") {
                            let inner: Vec<(String, Value)> = rt.obj(src_bag).properties
                                .iter().map(|(k, d)| (k.clone(), d.value.clone())).collect();
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
        self.obj_mut(request_ctor_id).set_own_frozen("prototype".into(), Value::Object(request_proto));
        self.obj_mut(request_proto).set_own_internal("constructor".into(), Value::Object(request_ctor_id));
        self.globals.insert("Request".into(), Value::Object(request_ctor_id));
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
        let bi_obj = make_native("BigInt", |_rt, args| {
            use crate::bigint::JsBigInt;
            let v = args.first().cloned().unwrap_or(Value::Undefined);
            match v {
                Value::BigInt(b) => Ok(Value::BigInt(b)),
                Value::Number(n) => {
                    if !n.is_finite() || n.fract() != 0.0 {
                        return Err(RuntimeError::TypeError(format!(
                            "Cannot convert non-integer Number {} to BigInt", n)));
                    }
                    Ok(Value::BigInt(std::rc::Rc::new(JsBigInt::from_i64(n as i64))))
                }
                Value::String(s) => {
                    match JsBigInt::from_decimal(s.trim()) {
                        Some(b) => Ok(Value::BigInt(std::rc::Rc::new(b))),
                        None => Err(RuntimeError::TypeError(format!(
                            "Cannot convert {:?} to BigInt", s.as_str()))),
                    }
                }
                Value::Boolean(b) => Ok(Value::BigInt(std::rc::Rc::new(
                    if b { JsBigInt::one() } else { JsBigInt::zero() }))),
                _ => Err(RuntimeError::TypeError("Cannot convert to BigInt".into())),
            }
        });
        let bi_id = self.alloc_object(bi_obj);
        register_intrinsic_method(self, bi_id, "asIntN", 2, |_rt, args| Ok(args.get(1).cloned().unwrap_or(Value::Undefined)));
        register_intrinsic_method(self, bi_id, "asUintN", 2, |_rt, args| Ok(args.get(1).cloned().unwrap_or(Value::Undefined)));
        // Tier-Ω.5.oooooo: BigInt.prototype with valueOf + toString. unbox-
        // primitive / is-bigint reach for `BigInt.prototype.valueOf`.
        let bi_proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, bi_proto, "valueOf", 0, |rt, _args| {
            match rt.current_this() {
                Value::BigInt(b) => Ok(Value::BigInt(b)),
                _ => Err(RuntimeError::TypeError("BigInt.prototype.valueOf: this is not a BigInt".into())),
            }
        });
        register_intrinsic_method(self, bi_proto, "toString", 0, |rt, args| {
            let b = match rt.current_this() {
                Value::BigInt(b) => b,
                _ => return Err(RuntimeError::TypeError("BigInt.prototype.toString: this is not a BigInt".into())),
            };
            let radix = match args.first() {
                Some(Value::Number(n)) if (2.0..=36.0).contains(n) => *n as u32,
                Some(Value::Undefined) | None => 10,
                _ => return Err(RuntimeError::TypeError("BigInt.prototype.toString radix out of range".into())),
            };
            Ok(Value::String(std::rc::Rc::new(b.to_radix(radix))))
        });
        self.obj_mut(bi_id).set_own_frozen("prototype".into(), Value::Object(bi_proto));
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
                let proto = match rt.globals.get("Boolean").cloned() {
                    Some(Value::Object(id)) => match rt.object_get(id, "prototype") {
                        Value::Object(p) => Some(p), _ => None,
                    },
                    _ => None,
                };
                if let Some(p) = proto { obj.proto = Some(p); }
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
        self.obj_mut(bool_id).set_own_frozen("prototype".into(), Value::Object(bool_proto));
        // Ω.5.P58.E4: Boolean.prototype.constructor = Boolean per ECMA §10.2.12.
        self.obj_mut(bool_proto).set_own_internal("constructor".into(), Value::Object(bool_id));
        // Ω.5.P62.E19: Boolean.prototype is a Boolean exotic with
        // [[BooleanData]] = false per §20.3.4.
        self.obj_mut(bool_proto).set_own_internal("__primitive__".into(), Value::Boolean(false));
        self.globals.insert("Boolean".into(), Value::Object(bool_id));
        // Tier-Ω.5.tttttt: EventTarget + Event + CustomEvent global stubs
        // (chai / web-platform-ish libs). v1: ordinary objects with the
        // standard surface; no actual dispatch.
        let et = make_native("EventTarget", |rt, _args| {
            let mut o = Object::new_ordinary();
            o.set_own_internal("__listeners__".into(), Value::Object(rt.alloc_object(Object::new_ordinary())));
            Ok(Value::Object(rt.alloc_object(o)))
        });
        let et_id = self.alloc_object(et);
        let et_proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, et_proto, "addEventListener", 1, |rt, _args| { let _=rt; Ok(Value::Undefined) });
        register_intrinsic_method(self, et_proto, "removeEventListener", 1, |rt, _args| { let _=rt; Ok(Value::Undefined) });
        register_intrinsic_method(self, et_proto, "dispatchEvent", 1, |_rt, _args| Ok(Value::Boolean(false)));
        self.obj_mut(et_id).set_own_frozen("prototype".into(), Value::Object(et_proto));
        self.globals.insert("EventTarget".into(), Value::Object(et_id));
        let ev = make_native("Event", |rt, args| {
            let mut o = Object::new_ordinary();
            let ty = match args.first() { Some(Value::String(s)) => (**s).clone(), _ => String::new() };
            o.set_own("type".into(), Value::String(Rc::new(ty)));
            o.set_own("bubbles".into(), Value::Boolean(false));
            o.set_own("cancelable".into(), Value::Boolean(false));
            o.set_own("defaultPrevented".into(), Value::Boolean(false));
            Ok(Value::Object(rt.alloc_object(o)))
        });
        let ev_id = self.alloc_object(ev);
        let ev_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(ev_id).set_own_frozen("prototype".into(), Value::Object(ev_proto));
        self.globals.insert("Event".into(), Value::Object(ev_id));
        let ce = make_native("CustomEvent", |rt, args| {
            let mut o = Object::new_ordinary();
            let ty = match args.first() { Some(Value::String(s)) => (**s).clone(), _ => String::new() };
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
        self.obj_mut(ce_id).set_own_frozen("prototype".into(), Value::Object(ce_proto));
        self.globals.insert("CustomEvent".into(), Value::Object(ce_id));
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
            let name = match args.first() { Some(Value::String(s)) => (**s).clone(), _ => String::new() };
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
        self.obj_mut(bc_id).set_own_frozen("prototype".into(), Value::Object(bc_proto));
        self.obj_mut(bc_proto).set_own_internal("constructor".into(), Value::Object(bc_id));
        self.globals.insert("BroadcastChannel".into(), Value::Object(bc_id));
        for name in &["MessageEvent", "ErrorEvent", "CloseEvent", "ProgressEvent", "BeforeUnloadEvent", "FocusEvent"] {
            let ctor_name = *name;
            let nm = make_native(ctor_name, move |rt, args| {
                let mut o = Object::new_ordinary();
                let ty = match args.first() { Some(Value::String(s)) => (**s).clone(), _ => String::new() };
                o.set_own("type".into(), Value::String(Rc::new(ty)));
                if let Some(Value::Object(init_id)) = args.get(1) {
                    let data = rt.object_get(*init_id, "data");
                    o.set_own("data".into(), data);
                }
                Ok(Value::Object(rt.alloc_object(o)))
            });
            let nm_id = self.alloc_object(nm);
            let nm_proto = self.alloc_object(Object::new_ordinary());
            self.obj_mut(nm_id).set_own_frozen("prototype".into(), Value::Object(nm_proto));
            self.obj_mut(nm_proto).set_own_internal("constructor".into(), Value::Object(nm_id));
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
                _ => return Err(RuntimeError::TypeError("Proxy: target must be an object".into())),
            };
            let handler = match args.get(1) {
                Some(Value::Object(id)) => *id,
                _ => return Err(RuntimeError::TypeError("Proxy: handler must be an object".into())),
            };
            let mut o = Object::new_ordinary();
            o.internal_kind = InternalKind::Proxy(crate::value::ProxyInternals {
                target, handler,
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
                _ => return Err(RuntimeError::TypeError("Proxy.revocable: target must be an object".into())),
            };
            let handler = match args.get(1) {
                Some(Value::Object(id)) => *id,
                _ => return Err(RuntimeError::TypeError("Proxy.revocable: handler must be an object".into())),
            };
            let mut o = Object::new_ordinary();
            o.internal_kind = InternalKind::Proxy(crate::value::ProxyInternals {
                target, handler,
            });
            o.proto = rt.obj(target).proto;
            let proxy_id = rt.alloc_object(o);
            let mut result = Object::new_ordinary();
            result.set_own("proxy".into(), Value::Object(proxy_id));
            let revoke = make_native("revoke", move |_rt, _args| Ok(Value::Undefined));
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
            register_intrinsic_method(self, proto, "get", 2, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let key = args.first().cloned().unwrap_or(Value::Undefined);
                let key_s = abstract_ops::to_string(&key).as_str().to_string();
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())),
                };
                Ok(rt.object_get(storage, &key_s))
            });
            register_intrinsic_method(self, proto, "set", 3, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let key = args.first().cloned().unwrap_or(Value::Undefined);
                let val = args.get(1).cloned().unwrap_or(Value::Undefined);
                let key_s = abstract_ops::to_string(&key).as_str().to_string();
                // Ω.5.P62.E23: [[MapData]] brand — no auto-create.
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Map.prototype.set: this is not a Map object".into())),
                };
                let existed = !matches!(rt.object_get(storage, &key_s), Value::Undefined);
                rt.object_set(storage, key_s, val);
                if !existed {
                    let prev = match rt.object_get(this, "size") {
                        Value::Number(n) => n,
                        _ => 0.0,
                    };
                    rt.object_set(this, "size".into(), Value::Number(prev + 1.0));
                }
                Ok(Value::Object(this))
            });
            register_intrinsic_method(self, proto, "has", 2, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let key = args.first().cloned().unwrap_or(Value::Undefined);
                let key_s = abstract_ops::to_string(&key).as_str().to_string();
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())),
                };
                Ok(Value::Boolean(rt.obj(storage).properties.contains_key(&key_s)))
            });
            register_intrinsic_method(self, proto, "delete", 1, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let key = args.first().cloned().unwrap_or(Value::Undefined);
                let key_s = abstract_ops::to_string(&key).as_str().to_string();
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())),
                };
                let existed = rt.obj_mut(storage).properties.shift_remove(&key_s).is_some();
                if existed {
                    let prev = match rt.object_get(this, "size") {
                        Value::Number(n) => n,
                        _ => 0.0,
                    };
                    rt.object_set(this, "size".into(), Value::Number((prev - 1.0).max(0.0)));
                }
                Ok(Value::Boolean(existed))
            });
            register_intrinsic_method(self, proto, "clear", 1, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let fresh = rt.alloc_object(Object::new_ordinary());
                rt.object_set(this, "__map_data".into(), Value::Object(fresh));
                rt.object_set(this, "size".into(), Value::Number(0.0));
                Ok(Value::Undefined)
            });
            register_intrinsic_method(self, proto, "forEach", 1, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let cb = args.first().cloned().unwrap_or(Value::Undefined);
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())),
                };
                let pairs: Vec<(String, Value)> = rt.obj(storage).properties.iter()
                    .map(|(k, d)| (k.clone(), d.value.clone()))
                    .collect();
                for (k, v) in pairs {
                    let key_v = Value::String(Rc::new(k));
                    rt.call_function(cb.clone(), Value::Undefined, vec![v, key_v, Value::Object(this)])?;
                }
                Ok(Value::Undefined)
            });
            // Tier-Ω.5.KKKKKKK: Map.prototype.values / keys / entries per ECMA
            // §24.1.3.3 / .4 / .5. Returns an array (eager-collect — full
            // iterator-protocol support is queued downstream). wrap-ansi /
            // log-update / mime / many spread the map's values into a Set
            // via `new Set(m.values())` which exercises Symbol.iterator on
            // the returned object; an Array satisfies both the iterator
            // (via @@iterator on Array.prototype) and the spread protocol.
            register_intrinsic_method(self, proto, "values", 1, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Object(rt.alloc_object(Object::new_array()))),
                };
                let vs: Vec<Value> = rt.obj(storage).properties.iter()
                    .map(|(_k, d)| d.value.clone()).collect();
                let arr = rt.alloc_object(Object::new_array());
                for (i, v) in vs.into_iter().enumerate() {
                    rt.object_set(arr, i.to_string(), v);
                }
                let len = rt.array_length(arr);
                rt.object_set(arr, "length".into(), Value::Number(len as f64));
                Ok(Value::Object(arr))
            });
            register_intrinsic_method(self, proto, "keys", 1, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Object(rt.alloc_object(Object::new_array()))),
                };
                let ks: Vec<String> = rt.obj(storage).properties.keys().cloned().collect();
                let arr = rt.alloc_object(Object::new_array());
                for (i, k) in ks.into_iter().enumerate() {
                    rt.object_set(arr, i.to_string(), Value::String(Rc::new(k)));
                }
                let len = rt.array_length(arr);
                rt.object_set(arr, "length".into(), Value::Number(len as f64));
                Ok(Value::Object(arr))
            });
            register_intrinsic_method(self, proto, "entries", 1, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Object(rt.alloc_object(Object::new_array()))),
                };
                let pairs: Vec<(String, Value)> = rt.obj(storage).properties.iter()
                    .map(|(k, d)| (k.clone(), d.value.clone())).collect();
                let arr = rt.alloc_object(Object::new_array());
                for (i, (k, v)) in pairs.into_iter().enumerate() {
                    let pair = rt.alloc_object(Object::new_array());
                    rt.object_set(pair, "0".into(), Value::String(Rc::new(k)));
                    rt.object_set(pair, "1".into(), v);
                    rt.object_set(pair, "length".into(), Value::Number(2.0));
                    rt.object_set(arr, i.to_string(), Value::Object(pair));
                }
                let len = rt.array_length(arr);
                rt.object_set(arr, "length".into(), Value::Number(len as f64));
                Ok(Value::Object(arr))
            });
            // Tier-Ω.5.MMMMMMM: Map.prototype[@@iterator] aliases entries
            // per ECMA §24.1.3.12. Surfaced by Step-6 route-(b) escalation:
            // adding receiver-shape tags to the CallMethod undef-fault
            // surfaced 'receiver=Object keys=[__map_data,size]' on the
            // cli-truncate/fast-xml-parser/log-update cluster, naming Map
            // as the iterated receiver. for-of and spread reach for
            // [Symbol.iterator], which on Map is Map.prototype.entries.
            register_intrinsic_method(self, proto, "@@iterator", 1, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Map.prototype method: this is not a Map object".into())) };
                let storage = match rt.object_get(this, "__map_data") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Object(rt.alloc_object(Object::new_array()))),
                };
                let pairs: Vec<(String, Value)> = rt.obj(storage).properties.iter()
                    .map(|(k, d)| (k.clone(), d.value.clone())).collect();
                let arr = rt.alloc_object(Object::new_array());
                for (i, (k, v)) in pairs.into_iter().enumerate() {
                    let pair = rt.alloc_object(Object::new_array());
                    rt.object_set(pair, "0".into(), Value::String(Rc::new(k)));
                    rt.object_set(pair, "1".into(), v);
                    rt.object_set(pair, "length".into(), Value::Number(2.0));
                    rt.object_set(arr, i.to_string(), Value::Object(pair));
                }
                let len = rt.array_length(arr);
                rt.object_set(arr, "length".into(), Value::Number(len as f64));
                Ok(Value::Object(crate::iterator::make_array_iterator(rt, arr)))
            });
            let proto_for_ctor = proto;
            let name = (*collection).to_string();
            let ctor_obj = make_native(&name, move |rt, args| {
                let mut o = Object::new_ordinary();
                o.proto = Some(proto_for_ctor);
                let id = rt.alloc_object(o);
                let storage = rt.alloc_object(Object::new_ordinary());
                rt.object_set(id, "__map_data".into(), Value::Object(storage));
                rt.object_set(id, "size".into(), Value::Number(0.0));
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
                        let cnt = rt.obj(storage).properties.len() as f64;
                        rt.object_set(id, "size".into(), Value::Number(cnt));
                    }
                }
                Ok(Value::Object(id))
            });
            let ctor = self.alloc_object(ctor_obj);
            self.obj_mut(ctor).set_own_frozen("prototype".into(), Value::Object(proto));
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(ctor));
            self.globals.insert((*collection).to_string(), Value::Object(ctor));
        }
        for collection in &["Set", "WeakSet"] {
            let proto = self.alloc_object(Object::new_ordinary());
            register_intrinsic_method(self, proto, "add", 1, |rt, args| {
                // Ω.5.P62.E23: §24.2.3.1 step 3 — brand-check [[SetData]];
                // throw TypeError when receiver lacks the internal slot.
                let this = match rt.current_this() {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Set.prototype.add: this is not a Set object".into())),
                };
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                let key_s = abstract_ops::to_string(&v).as_str().to_string();
                let storage = match rt.object_get(this, "__set_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Set.prototype.add: this is not a Set object".into())),
                };
                let existed = !matches!(rt.object_get(storage, &key_s), Value::Undefined);
                rt.object_set(storage, key_s, v);
                if !existed {
                    let prev = match rt.object_get(this, "size") {
                        Value::Number(n) => n,
                        _ => 0.0,
                    };
                    rt.object_set(this, "size".into(), Value::Number(prev + 1.0));
                }
                Ok(Value::Object(this))
            });
            register_intrinsic_method(self, proto, "has", 2, |rt, args| {
                // Ω.5.P62.E23: [[SetData]] brand check.
                let this = match rt.current_this() {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Set.prototype.has: this is not a Set object".into())),
                };
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                let key_s = abstract_ops::to_string(&v).as_str().to_string();
                let storage = match rt.object_get(this, "__set_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Set.prototype.has: this is not a Set object".into())),
                };
                Ok(Value::Boolean(rt.obj(storage).properties.contains_key(&key_s)))
            });
            register_intrinsic_method(self, proto, "delete", 1, |rt, args| {
                let this = match rt.current_this() {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Set.prototype.delete: this is not a Set object".into())),
                };
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                let key_s = abstract_ops::to_string(&v).as_str().to_string();
                let storage = match rt.object_get(this, "__set_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Set.prototype.delete: this is not a Set object".into())),
                };
                let existed = rt.obj_mut(storage).properties.shift_remove(&key_s).is_some();
                if existed {
                    let prev = match rt.object_get(this, "size") {
                        Value::Number(n) => n,
                        _ => 0.0,
                    };
                    rt.object_set(this, "size".into(), Value::Number((prev - 1.0).max(0.0)));
                }
                Ok(Value::Boolean(existed))
            });
            register_intrinsic_method(self, proto, "clear", 1, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Set.prototype method: this is not a Set object".into())) };
                let fresh = rt.alloc_object(Object::new_ordinary());
                rt.object_set(this, "__set_data".into(), Value::Object(fresh));
                rt.object_set(this, "size".into(), Value::Number(0.0));
                Ok(Value::Undefined)
            });
            register_intrinsic_method(self, proto, "forEach", 1, |rt, args| {
                let this = match rt.current_this() {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Set.prototype.forEach: this is not a Set object".into())),
                };
                let cb = args.first().cloned().unwrap_or(Value::Undefined);
                if !rt.is_callable(&cb) {
                    return Err(RuntimeError::TypeError(
                        "Set.prototype.forEach: callback is not callable".into()));
                }
                let storage = match rt.object_get(this, "__set_data") {
                    Value::Object(id) => id,
                    _ => return Err(RuntimeError::TypeError(
                        "Set.prototype.forEach: this is not a Set object".into())),
                };
                let vals: Vec<Value> = rt.obj(storage).properties.values()
                    .map(|d| d.value.clone())
                    .collect();
                for v in vals {
                    rt.call_function(cb.clone(), Value::Undefined, vec![v.clone(), v, Value::Object(this)])?;
                }
                Ok(Value::Undefined)
            });
            // Tier-Ω.5.rrr: @@iterator returns a values-iterator. Per
            // spec Set.prototype[Symbol.iterator] === Set.prototype.values.
            // Required for `[...new Set(arr)]` to spread.
            register_intrinsic_method(self, proto, "@@iterator", 1, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Set.prototype method: this is not a Set object".into())) };
                make_set_values_iterator(rt, this)
            });
            register_intrinsic_method(self, proto, "values", 1, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Set.prototype method: this is not a Set object".into())) };
                make_set_values_iterator(rt, this)
            });
            // Ω.5.P61.E11: Set.prototype.keys is alias for values per ECMA §24.2.4.
            register_intrinsic_method(self, proto, "keys", 0, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Set.prototype method: this is not a Set object".into())) };
                make_set_values_iterator(rt, this)
            });
            // Set.prototype.entries returns iterator of [v, v] pairs.
            register_intrinsic_method(self, proto, "entries", 0, |rt, _args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Set.prototype method: this is not a Set object".into())) };
                let storage = match rt.object_get(this, "__set_data") {
                    Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Set.prototype method: this is not a Set object".into())),
                };
                let vals: Vec<Value> = rt.obj(storage).properties.values()
                    .map(|d| d.value.clone()).collect();
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
            // Set-theoretic operations per ECMA §24.2.4 (ES2024).
            let set_op_union = |rt: &mut Runtime, args: &[Value]| -> Result<Value, RuntimeError> {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("union: this not Set".into())) };
                let other = args.first().cloned().unwrap_or(Value::Undefined);
                let other_vals = collect_iterable(rt, other)?;
                // Build new Set
                let out_proto = match rt.globals.get("Set").cloned() {
                    Some(Value::Object(cid)) => match rt.object_get(cid, "prototype") { Value::Object(p) => Some(p), _ => None },
                    _ => None,
                };
                let mut o = Object::new_ordinary();
                o.proto = out_proto;
                let new_set = rt.alloc_object(o);
                let storage = rt.alloc_object(Object::new_ordinary());
                rt.object_set(new_set, "__set_data".into(), Value::Object(storage));
                let mut size = 0.0;
                let this_storage = match rt.object_get(this, "__set_data") { Value::Object(id) => Some(id), _ => None };
                if let Some(s) = this_storage {
                    let kvs: Vec<(String, Value)> = rt.obj(s).properties.iter().map(|(k,d)| (k.clone(), d.value.clone())).collect();
                    for (k, v) in kvs { rt.object_set(storage, k, v); size += 1.0; }
                }
                for v in other_vals {
                    let k = abstract_ops::to_string(&v).as_str().to_string();
                    if !rt.obj(storage).properties.contains_key(&k) {
                        rt.object_set(storage, k, v); size += 1.0;
                    }
                }
                rt.object_set(new_set, "size".into(), Value::Number(size));
                Ok(Value::Object(new_set))
            };
            register_intrinsic_method(self, proto, "union", 1, set_op_union);
            register_intrinsic_method(self, proto, "intersection", 1, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("intersection: this not Set".into())) };
                let other = args.first().cloned().unwrap_or(Value::Undefined);
                let other_vals = collect_iterable(rt, other)?;
                let other_keys: std::collections::HashSet<String> = other_vals.iter()
                    .map(|v| abstract_ops::to_string(v).as_str().to_string()).collect();
                let out_proto = match rt.globals.get("Set").cloned() {
                    Some(Value::Object(cid)) => match rt.object_get(cid, "prototype") { Value::Object(p) => Some(p), _ => None },
                    _ => None,
                };
                let mut o = Object::new_ordinary(); o.proto = out_proto;
                let new_set = rt.alloc_object(o);
                let storage = rt.alloc_object(Object::new_ordinary());
                rt.object_set(new_set, "__set_data".into(), Value::Object(storage));
                let mut size = 0.0;
                if let Value::Object(s) = rt.object_get(this, "__set_data") {
                    let kvs: Vec<(String, Value)> = rt.obj(s).properties.iter().map(|(k,d)| (k.clone(), d.value.clone())).collect();
                    for (k, v) in kvs {
                        if other_keys.contains(&k) { rt.object_set(storage, k, v); size += 1.0; }
                    }
                }
                rt.object_set(new_set, "size".into(), Value::Number(size));
                Ok(Value::Object(new_set))
            });
            register_intrinsic_method(self, proto, "difference", 1, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("difference: this not Set".into())) };
                let other = args.first().cloned().unwrap_or(Value::Undefined);
                let other_vals = collect_iterable(rt, other)?;
                let other_keys: std::collections::HashSet<String> = other_vals.iter()
                    .map(|v| abstract_ops::to_string(v).as_str().to_string()).collect();
                let out_proto = match rt.globals.get("Set").cloned() {
                    Some(Value::Object(cid)) => match rt.object_get(cid, "prototype") { Value::Object(p) => Some(p), _ => None },
                    _ => None,
                };
                let mut o = Object::new_ordinary(); o.proto = out_proto;
                let new_set = rt.alloc_object(o);
                let storage = rt.alloc_object(Object::new_ordinary());
                rt.object_set(new_set, "__set_data".into(), Value::Object(storage));
                let mut size = 0.0;
                if let Value::Object(s) = rt.object_get(this, "__set_data") {
                    let kvs: Vec<(String, Value)> = rt.obj(s).properties.iter().map(|(k,d)| (k.clone(), d.value.clone())).collect();
                    for (k, v) in kvs {
                        if !other_keys.contains(&k) { rt.object_set(storage, k, v); size += 1.0; }
                    }
                }
                rt.object_set(new_set, "size".into(), Value::Number(size));
                Ok(Value::Object(new_set))
            });
            register_intrinsic_method(self, proto, "symmetricDifference", 1, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("symmetricDifference: this not Set".into())) };
                let other = args.first().cloned().unwrap_or(Value::Undefined);
                let other_vals = collect_iterable(rt, other)?;
                let other_keys: std::collections::HashSet<String> = other_vals.iter()
                    .map(|v| abstract_ops::to_string(v).as_str().to_string()).collect();
                let out_proto = match rt.globals.get("Set").cloned() {
                    Some(Value::Object(cid)) => match rt.object_get(cid, "prototype") { Value::Object(p) => Some(p), _ => None },
                    _ => None,
                };
                let mut o = Object::new_ordinary(); o.proto = out_proto;
                let new_set = rt.alloc_object(o);
                let storage = rt.alloc_object(Object::new_ordinary());
                rt.object_set(new_set, "__set_data".into(), Value::Object(storage));
                let mut size = 0.0;
                // In this but not other.
                if let Value::Object(s) = rt.object_get(this, "__set_data") {
                    let kvs: Vec<(String, Value)> = rt.obj(s).properties.iter().map(|(k,d)| (k.clone(), d.value.clone())).collect();
                    for (k, v) in kvs {
                        if !other_keys.contains(&k) { rt.object_set(storage, k, v); size += 1.0; }
                    }
                }
                // In other but not this.
                let this_storage = match rt.object_get(this, "__set_data") { Value::Object(id) => Some(id), _ => None };
                for v in other_vals {
                    let k = abstract_ops::to_string(&v).as_str().to_string();
                    let in_this = this_storage.map(|s| rt.obj(s).properties.contains_key(&k)).unwrap_or(false);
                    if !in_this { rt.object_set(storage, k, v); size += 1.0; }
                }
                rt.object_set(new_set, "size".into(), Value::Number(size));
                Ok(Value::Object(new_set))
            });
            register_intrinsic_method(self, proto, "isSubsetOf", 1, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Set.prototype method: this is not a Set object".into())) };
                let other = args.first().cloned().unwrap_or(Value::Undefined);
                let other_vals = collect_iterable(rt, other)?;
                let other_keys: std::collections::HashSet<String> = other_vals.iter()
                    .map(|v| abstract_ops::to_string(v).as_str().to_string()).collect();
                if let Value::Object(s) = rt.object_get(this, "__set_data") {
                    for k in rt.obj(s).properties.keys() {
                        if !other_keys.contains(k) { return Ok(Value::Boolean(false)); }
                    }
                }
                Ok(Value::Boolean(true))
            });
            register_intrinsic_method(self, proto, "isSupersetOf", 1, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Err(RuntimeError::TypeError("Set.prototype method: this is not a Set object".into())) };
                let other = args.first().cloned().unwrap_or(Value::Undefined);
                let other_vals = collect_iterable(rt, other)?;
                let this_storage = match rt.object_get(this, "__set_data") { Value::Object(id) => Some(id), _ => None };
                for v in other_vals {
                    let k = abstract_ops::to_string(&v).as_str().to_string();
                    let in_this = this_storage.map(|s| rt.obj(s).properties.contains_key(&k)).unwrap_or(false);
                    if !in_this { return Ok(Value::Boolean(false)); }
                }
                Ok(Value::Boolean(true))
            });
            register_intrinsic_method(self, proto, "isDisjointFrom", 1, |rt, args| {
                let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Boolean(true)) };
                let other = args.first().cloned().unwrap_or(Value::Undefined);
                let other_vals = collect_iterable(rt, other)?;
                let this_storage = match rt.object_get(this, "__set_data") { Value::Object(id) => Some(id), _ => None };
                for v in other_vals {
                    let k = abstract_ops::to_string(&v).as_str().to_string();
                    let in_this = this_storage.map(|s| rt.obj(s).properties.contains_key(&k)).unwrap_or(false);
                    if in_this { return Ok(Value::Boolean(false)); }
                }
                Ok(Value::Boolean(true))
            });
            let proto_for_ctor = proto;
            let name = (*collection).to_string();
            let ctor_obj = make_native(&name, move |rt, args| {
                let mut o = Object::new_ordinary();
                o.proto = Some(proto_for_ctor);
                let id = rt.alloc_object(o);
                let storage = rt.alloc_object(Object::new_ordinary());
                rt.object_set(id, "__set_data".into(), Value::Object(storage));
                rt.object_set(id, "size".into(), Value::Number(0.0));
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
            self.obj_mut(ctor).set_own_frozen("prototype".into(), Value::Object(proto));
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(ctor));
            self.globals.insert((*collection).to_string(), Value::Object(ctor));
        }
    }

    /// Tier-Ω.5.aaaa: Date global. Real Gregorian arithmetic for year/
    /// month/day extraction; ISO-string parsing in the constructor;
    /// per-spec getter methods.
    fn install_date_global(&mut self) {
        let proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, proto, "getTime", 1, |rt, _args| {
            let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(0.0)) };
            Ok(rt.object_get(this, "__date_ms"))
        });
        register_intrinsic_method(self, proto, "valueOf", 0, |rt, _args| {
            let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(0.0)) };
            Ok(rt.object_get(this, "__date_ms"))
        });
        register_intrinsic_method(self, proto, "getFullYear", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            Ok(Value::Number(date_components(ms).0 as f64))
        });
        register_intrinsic_method(self, proto, "getMonth", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            Ok(Value::Number(date_components(ms).1 as f64))
        });
        register_intrinsic_method(self, proto, "getDate", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            Ok(Value::Number(date_components(ms).2 as f64))
        });
        register_intrinsic_method(self, proto, "getDay", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            // Jan 1 1970 was a Thursday (day 4).
            let days = (ms / 86_400_000.0).floor() as i64;
            let dow = ((days % 7) + 7 + 4) % 7;
            Ok(Value::Number(dow as f64))
        });
        register_intrinsic_method(self, proto, "getHours", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            Ok(Value::Number(((ms / 3_600_000.0).floor() as i64 % 24) as f64))
        });
        register_intrinsic_method(self, proto, "getMinutes", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            Ok(Value::Number(((ms / 60_000.0).floor() as i64 % 60) as f64))
        });
        register_intrinsic_method(self, proto, "getSeconds", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            Ok(Value::Number(((ms / 1000.0).floor() as i64 % 60) as f64))
        });
        register_intrinsic_method(self, proto, "getMilliseconds", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            Ok(Value::Number((ms as i64 % 1000) as f64))
        });
        register_intrinsic_method(self, proto, "getTimezoneOffset", 1, |_rt, _args| Ok(Value::Number(0.0)));
        // Tier-Ω.5.P31.E1.date-utc-getters-setters: getUTC* mirror the
        // non-UTC getters (we treat __date_ms as UTC throughout — no
        // local-time conversion). setUTC* mutate the date by replacing
        // the corresponding component. Surfaced by Ω.5.P24.E1 probe
        // walking temporal-polyfill (whose `setUTCHours` call landed on
        // a fake-Date-shaped object with no Date.prototype in its chain).
        for (name, key) in [
            ("getUTCFullYear", "getFullYear"),
            ("getUTCMonth", "getMonth"),
            ("getUTCDate", "getDate"),
            ("getUTCDay", "getDay"),
            ("getUTCHours", "getHours"),
            ("getUTCMinutes", "getMinutes"),
            ("getUTCSeconds", "getSeconds"),
            ("getUTCMilliseconds", "getMilliseconds"),
        ] {
            let _ = key; // alias only — both compute the same value
            let n = name.to_string();
            let n_for_closure = n.clone();
            register_method(self, proto, &n, move |rt, _args| {
                let n = &n_for_closure;
                let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
                let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
                let (y, mo, d) = date_components(ms);
                let h = (ms / 3_600_000.0).floor() as i64 % 24;
                let mi = (ms / 60_000.0).floor() as i64 % 60;
                let se = (ms / 1000.0).floor() as i64 % 60;
                let mss = ms as i64 % 1000;
                let days = (ms / 86_400_000.0).floor() as i64;
                let dow = ((days % 7) + 7 + 4) % 7;
                let v = match n.as_str() {
                    "getUTCFullYear" => y as f64,
                    "getUTCMonth" => mo as f64,
                    "getUTCDate" => d as f64,
                    "getUTCDay" => dow as f64,
                    "getUTCHours" => h as f64,
                    "getUTCMinutes" => mi as f64,
                    "getUTCSeconds" => se as f64,
                    "getUTCMilliseconds" => mss as f64,
                    _ => f64::NAN,
                };
                Ok(Value::Number(v))
            });
        }
        // setUTC* family. Each replaces the named component(s) in the
        // current ms and returns the new ms per ECMA §21.4.4.x.
        register_intrinsic_method(self, proto, "setUTCHours", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            let (y, mo, d) = date_components(ms);
            let cur_mi = (ms / 60_000.0).floor() as i64 % 60;
            let cur_se = (ms / 1000.0).floor() as i64 % 60;
            let cur_mss = ms as i64 % 1000;
            let h = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
            let mi = args.get(1).map(crate::abstract_ops::to_number).unwrap_or(cur_mi as f64) as i64;
            let se = args.get(2).map(crate::abstract_ops::to_number).unwrap_or(cur_se as f64) as i64;
            let mss = args.get(3).map(crate::abstract_ops::to_number).unwrap_or(cur_mss as f64) as i64;
            let new_ms = (ymd_to_ms(y, mo, d) + h * 3_600_000 + mi * 60_000 + se * 1000 + mss) as f64;
            rt.object_set(this_id, "__date_ms".into(), Value::Number(new_ms));
            Ok(Value::Number(new_ms))
        });
        register_intrinsic_method(self, proto, "setUTCMinutes", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            let (y, mo, d) = date_components(ms);
            let cur_h = (ms / 3_600_000.0).floor() as i64 % 24;
            let cur_se = (ms / 1000.0).floor() as i64 % 60;
            let cur_mss = ms as i64 % 1000;
            let mi = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
            let se = args.get(1).map(crate::abstract_ops::to_number).unwrap_or(cur_se as f64) as i64;
            let mss = args.get(2).map(crate::abstract_ops::to_number).unwrap_or(cur_mss as f64) as i64;
            let new_ms = (ymd_to_ms(y, mo, d) + cur_h * 3_600_000 + mi * 60_000 + se * 1000 + mss) as f64;
            rt.object_set(this_id, "__date_ms".into(), Value::Number(new_ms));
            Ok(Value::Number(new_ms))
        });
        register_intrinsic_method(self, proto, "setUTCSeconds", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            let (y, mo, d) = date_components(ms);
            let cur_h = (ms / 3_600_000.0).floor() as i64 % 24;
            let cur_mi = (ms / 60_000.0).floor() as i64 % 60;
            let cur_mss = ms as i64 % 1000;
            let se = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
            let mss = args.get(1).map(crate::abstract_ops::to_number).unwrap_or(cur_mss as f64) as i64;
            let new_ms = (ymd_to_ms(y, mo, d) + cur_h * 3_600_000 + cur_mi * 60_000 + se * 1000 + mss) as f64;
            rt.object_set(this_id, "__date_ms".into(), Value::Number(new_ms));
            Ok(Value::Number(new_ms))
        });
        register_intrinsic_method(self, proto, "setUTCMilliseconds", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            let mss = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
            let base = (ms as i64 / 1000) * 1000;
            let new_ms = (base + mss) as f64;
            rt.object_set(this_id, "__date_ms".into(), Value::Number(new_ms));
            Ok(Value::Number(new_ms))
        });
        register_intrinsic_method(self, proto, "setUTCDate", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            let (y, mo, _d) = date_components(ms);
            let d = args.first().map(crate::abstract_ops::to_number).unwrap_or(1.0) as i64;
            let tod = ms as i64 - (ms as i64 / 86_400_000) * 86_400_000;
            let new_ms = (ymd_to_ms(y, mo, d) + tod) as f64;
            rt.object_set(this_id, "__date_ms".into(), Value::Number(new_ms));
            Ok(Value::Number(new_ms))
        });
        register_intrinsic_method(self, proto, "setUTCMonth", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            let (y, _mo, d) = date_components(ms);
            let mo = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
            let tod = ms as i64 - (ms as i64 / 86_400_000) * 86_400_000;
            let new_ms = (ymd_to_ms(y, mo, d) + tod) as f64;
            rt.object_set(this_id, "__date_ms".into(), Value::Number(new_ms));
            Ok(Value::Number(new_ms))
        });
        register_intrinsic_method(self, proto, "setUTCFullYear", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            let (_y, mo, d) = date_components(ms);
            let y = args.first().map(crate::abstract_ops::to_number).unwrap_or(1970.0) as i64;
            let mo2 = args.get(1).map(crate::abstract_ops::to_number).unwrap_or(mo as f64) as i64;
            let d2 = args.get(2).map(crate::abstract_ops::to_number).unwrap_or(d as f64) as i64;
            let tod = ms as i64 - (ms as i64 / 86_400_000) * 86_400_000;
            let new_ms = (ymd_to_ms(y, mo2, d2) + tod) as f64;
            rt.object_set(this_id, "__date_ms".into(), Value::Number(new_ms));
            Ok(Value::Number(new_ms))
        });
        register_intrinsic_method(self, proto, "setTime", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let v = args.first().map(crate::abstract_ops::to_number).unwrap_or(f64::NAN);
            rt.object_set(this_id, "__date_ms".into(), Value::Number(v));
            Ok(Value::Number(v))
        });
        // Local-time set* aliases (local == UTC since getTimezoneOffset == 0).
        for (name, alias) in [
            ("setHours", "setUTCHours"),
            ("setMinutes", "setUTCMinutes"),
            ("setSeconds", "setUTCSeconds"),
            ("setMilliseconds", "setUTCMilliseconds"),
            ("setDate", "setUTCDate"),
            ("setMonth", "setUTCMonth"),
            ("setFullYear", "setUTCFullYear"),
        ] {
            let target_key = alias.to_string();
            let n = name.to_string();
            register_method(self, proto, &n, move |rt, args| {
                let this = rt.current_this();
                let this_id = match this { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
                let f = rt.object_get(this_id, &target_key);
                rt.call_function(f, Value::Object(this_id), args.to_vec())
            });
        }
        register_intrinsic_method(self, proto, "toISOString", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(Rc::new("".into()))) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(Rc::new("".into()))) };
            let (y, mo, d) = date_components(ms);
            let h = (ms / 3_600_000.0).floor() as i64 % 24;
            let mi = (ms / 60_000.0).floor() as i64 % 60;
            let se = (ms / 1000.0).floor() as i64 % 60;
            let mss = ms as i64 % 1000;
            Ok(Value::String(Rc::new(format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
                y, mo + 1, d, h, mi, se, mss))))
        });
        register_intrinsic_method(self, proto, "toJSON", 1, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(Rc::new("".into()))) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(Rc::new("".into()))) };
            let (y, mo, d) = date_components(ms);
            Ok(Value::String(Rc::new(format!("{:04}-{:02}-{:02}T00:00:00.000Z", y, mo + 1, d))))
        });
        register_intrinsic_method(self, proto, "toString", 0, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(Rc::new("Invalid Date".into()))) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(Rc::new("Invalid Date".into()))) };
            let (y, mo, d) = date_components(ms);
            Ok(Value::String(Rc::new(format!("{:04}-{:02}-{:02}T00:00:00Z", y, mo + 1, d))))
        });
        // Ω.5.P61.E12: Date.prototype additional format + legacy methods
        // per ECMA §21.4.4. v1 deviates from locale-sensitive output;
        // returns the ISO-like form (sufficient for module-init presence
        // probes; consumer-locale-display gaps not yet surfaced).
        let date_fmt_date = |rt: &mut Runtime, _args: &[Value]| -> Result<Value, RuntimeError> {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(Rc::new(String::new()))) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(Rc::new("Invalid Date".into()))) };
            let (y, mo, d) = date_components(ms);
            Ok(Value::String(Rc::new(format!("{:04}-{:02}-{:02}", y, mo + 1, d))))
        };
        let date_fmt_time = |rt: &mut Runtime, _args: &[Value]| -> Result<Value, RuntimeError> {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(Rc::new(String::new()))) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(Rc::new("Invalid Date".into()))) };
            let h = (ms / 3_600_000.0).floor() as i64 % 24;
            let mi = (ms / 60_000.0).floor() as i64 % 60;
            let se = (ms / 1000.0).floor() as i64 % 60;
            Ok(Value::String(Rc::new(format!("{:02}:{:02}:{:02}", h, mi, se))))
        };
        let date_fmt_utc = |rt: &mut Runtime, _args: &[Value]| -> Result<Value, RuntimeError> {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(Rc::new(String::new()))) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(Rc::new("Invalid Date".into()))) };
            let (y, mo, d) = date_components(ms);
            let h = (ms / 3_600_000.0).floor() as i64 % 24;
            let mi = (ms / 60_000.0).floor() as i64 % 60;
            let se = (ms / 1000.0).floor() as i64 % 60;
            Ok(Value::String(Rc::new(format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} GMT", y, mo + 1, d, h, mi, se))))
        };
        register_intrinsic_method(self, proto, "toDateString", 0, date_fmt_date);
        register_intrinsic_method(self, proto, "toLocaleDateString", 0, date_fmt_date);
        register_intrinsic_method(self, proto, "toTimeString", 0, date_fmt_time);
        register_intrinsic_method(self, proto, "toLocaleTimeString", 0, date_fmt_time);
        register_intrinsic_method(self, proto, "toUTCString", 0, date_fmt_utc);
        // getYear / setYear per Annex B.2.4 (legacy). getYear returns
        // year - 1900; setYear sets full year, with two-digit values
        // mapped to 1900s for 0-99.
        register_intrinsic_method(self, proto, "getYear", 0, |rt, _args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::Number(f64::NAN)) };
            let (y, _, _) = date_components(ms);
            Ok(Value::Number((y - 1900) as f64))
        });
        register_intrinsic_method(self, proto, "setYear", 1, |rt, args| {
            let this_id = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(f64::NAN)) };
            let y_raw = args.first().map(crate::abstract_ops::to_number).unwrap_or(f64::NAN);
            let full_year = if y_raw >= 0.0 && y_raw <= 99.0 { y_raw + 1900.0 } else { y_raw };
            // Crude: re-encode the current ms with adjusted year. Use the
            // existing components and convert back.
            let ms = match rt.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => 0.0 };
            let (_, mo, d) = date_components(ms);
            // Approximate ms-for-year (ignores leap-second / DST nuance).
            let days_per_year = 365.25;
            let new_ms = ((full_year - 1970.0) * days_per_year * 86_400_000.0)
                + ((mo as f64) * 30.0 * 86_400_000.0)
                + ((d as f64 - 1.0) * 86_400_000.0);
            rt.object_set(this_id, "__date_ms".into(), Value::Number(new_ms));
            Ok(Value::Number(new_ms))
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
                            rt.object_set(new_id, "__date_ms".into(), Value::Number(n));
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
                let d = args.get(2).map(crate::abstract_ops::to_number).unwrap_or(1.0) as i64;
                let h = args.get(3).map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
                let mi = args.get(4).map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
                let se = args.get(5).map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
                let mss = args.get(6).map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
                (ymd_to_ms(y, mo, d) + h * 3_600_000 + mi * 60_000 + se * 1000 + mss) as f64
            } else {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as f64).unwrap_or(0.0)
            };
            let mut o = Object::new_ordinary();
            o.proto = Some(proto_for_ctor);
            let id = rt.alloc_object(o);
            rt.object_set(id, "__date_ms".into(), Value::Number(ms));
            Ok(Value::Object(id))
        });
        let ctor = self.alloc_object(ctor_obj);
        register_intrinsic_method(self, ctor, "now", 0, |_rt, _args| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let ms = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as f64).unwrap_or(0.0);
            Ok(Value::Number(ms))
        });
        register_intrinsic_method(self, ctor, "parse", 2, |_rt, _args| Ok(Value::Number(0.0)));
        register_intrinsic_method(self, ctor, "UTC", 1, |_rt, _args| Ok(Value::Number(0.0)));
        self.obj_mut(ctor).set_own_frozen("prototype".into(), Value::Object(proto));
        self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(ctor));
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
                _ => return Err(RuntimeError::TypeError("subarray: this must be a TypedArray".into())),
            };
            let len = match rt.object_get(this_id, "length") {
                Value::Number(n) => n as usize, _ => 0,
            };
            let start = args.first().and_then(|v| if let Value::Number(n) = v { Some(*n as i64) } else { None }).unwrap_or(0);
            let end = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as i64) } else { None }).unwrap_or(len as i64);
            let start = (if start < 0 { (len as i64 + start).max(0) } else { start }).min(len as i64) as usize;
            let end = (if end < 0 { (len as i64 + end).max(0) } else { end }).min(len as i64) as usize;
            let slice_len = end.saturating_sub(start);
            let kind = match rt.object_get(this_id, "__kind") { Value::String(s) => (*s).clone(), _ => "Uint8Array".into() };
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
                _ => return Err(RuntimeError::TypeError("set: this must be a TypedArray".into())),
            };
            let src = match args.first() {
                Some(Value::Object(id)) => *id,
                _ => return Ok(Value::Undefined),
            };
            let offset = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(0);
            let src_len = match rt.object_get(src, "length") { Value::Number(n) => n as usize, _ => 0 };
            for i in 0..src_len {
                let v = rt.object_get(src, &i.to_string());
                rt.object_set(this_id, (offset + i).to_string(), v);
            }
            Ok(Value::Undefined)
        });
        register_method(self, ta_proto, "fill", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError("fill: this must be a TypedArray".into())),
            };
            let v = args.first().cloned().unwrap_or(Value::Number(0.0));
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            for i in 0..len { rt.object_set(this_id, i.to_string(), v.clone()); }
            Ok(Value::Object(this_id))
        });
        register_method(self, ta_proto, "slice", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError("slice: this must be a TypedArray".into())),
            };
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            let start = args.first().and_then(|v| if let Value::Number(n) = v { Some(*n as i64) } else { None }).unwrap_or(0);
            let end = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as i64) } else { None }).unwrap_or(len as i64);
            let start = (if start < 0 { (len as i64 + start).max(0) } else { start }).min(len as i64) as usize;
            let end = (if end < 0 { (len as i64 + end).max(0) } else { end }).min(len as i64) as usize;
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
        register_method(self, ta_proto, "@@iterator", |rt, _args| {
            let src_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError("@@iterator: this must be TypedArray".into())),
            };
            let mut o = Object::new_ordinary();
            o.set_own_internal("__it_src__".into(), Value::Object(src_id));
            o.set_own_internal("__it_idx__".into(), Value::Number(0.0));
            let it_id = rt.alloc_object(o);
            register_intrinsic_method(rt, it_id, "next", 1, |rt, _args| {
                let this_id = match rt.current_this() { Value::Object(o) => o, _ => return Ok(Value::Undefined) };
                let src = match rt.object_get(this_id, "__it_src__") { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
                let idx = match rt.object_get(this_id, "__it_idx__") { Value::Number(n) => n as usize, _ => 0 };
                let len = match rt.object_get(src, "length") { Value::Number(n) => n as usize, _ => 0 };
                let mut o = Object::new_ordinary();
                if idx >= len {
                    o.set_own("value".into(), Value::Undefined);
                    o.set_own("done".into(), Value::Boolean(true));
                } else {
                    let v = rt.object_get(src, &idx.to_string());
                    rt.object_set(this_id, "__it_idx__".into(), Value::Number((idx + 1) as f64));
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
                _ => return Err(RuntimeError::TypeError("reverse: this must be a TypedArray".into())),
            };
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
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
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
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
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
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
            let cb = args.first().cloned().ok_or_else(||
                RuntimeError::TypeError("forEach: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                rt.call_function(cb.clone(), Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)])?;
            }
            Ok(Value::Undefined)
        });
        register_method(self, ta_proto, "find", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let cb = args.first().cloned().ok_or_else(||
                RuntimeError::TypeError("find: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(cb.clone(), Value::Undefined,
                    vec![v.clone(), Value::Number(i as f64), Value::Object(this_id)])?;
                if abstract_ops::to_boolean(&r) { return Ok(v); }
            }
            Ok(Value::Undefined)
        });
        register_method(self, ta_proto, "findIndex", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Number(-1.0)),
            };
            let cb = args.first().cloned().ok_or_else(||
                RuntimeError::TypeError("findIndex: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(cb.clone(), Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)])?;
                if abstract_ops::to_boolean(&r) { return Ok(Value::Number(i as f64)); }
            }
            Ok(Value::Number(-1.0))
        });
        register_method(self, ta_proto, "every", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Boolean(true)),
            };
            let cb = args.first().cloned().ok_or_else(||
                RuntimeError::TypeError("every: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(cb.clone(), Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)])?;
                if !abstract_ops::to_boolean(&r) { return Ok(Value::Boolean(false)); }
            }
            Ok(Value::Boolean(true))
        });
        register_method(self, ta_proto, "some", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Boolean(false)),
            };
            let cb = args.first().cloned().ok_or_else(||
                RuntimeError::TypeError("some: callback required".into()))?;
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(cb.clone(), Value::Undefined,
                    vec![v, Value::Number(i as f64), Value::Object(this_id)])?;
                if abstract_ops::to_boolean(&r) { return Ok(Value::Boolean(true)); }
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
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            let mut out = String::new();
            for i in 0..len {
                if i > 0 { out.push_str(&sep); }
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
                _ => return Err(RuntimeError::TypeError("TypedArray.prototype.map: this must be a TypedArray".into())),
            };
            let f = match args.first() {
                Some(v @ Value::Object(_)) => v.clone(),
                _ => return Err(RuntimeError::TypeError("TypedArray.prototype.map: callback must be a function".into())),
            };
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            let out = make_typed_array_like(rt, this_id, len);
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let r = rt.call_function(f.clone(), Value::Undefined, vec![v, Value::Number(i as f64), Value::Object(this_id)])?;
                rt.object_set(out, i.to_string(), r);
            }
            Ok(Value::Object(out))
        });
        register_method(self, ta_proto, "filter", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError("TypedArray.prototype.filter: this must be a TypedArray".into())),
            };
            let f = match args.first() {
                Some(v @ Value::Object(_)) => v.clone(),
                _ => return Err(RuntimeError::TypeError("TypedArray.prototype.filter: callback must be a function".into())),
            };
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            // Two-pass: first collect matches, then alloc with right length.
            let mut keeps: Vec<Value> = Vec::with_capacity(len);
            for i in 0..len {
                let v = rt.object_get(this_id, &i.to_string());
                let pred = rt.call_function(f.clone(), Value::Undefined, vec![v.clone(), Value::Number(i as f64), Value::Object(this_id)])?;
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
                _ => return Err(RuntimeError::TypeError("TypedArray.prototype.reduce: this must be a TypedArray".into())),
            };
            let f = match args.first() {
                Some(v @ Value::Object(_)) => v.clone(),
                _ => return Err(RuntimeError::TypeError("TypedArray.prototype.reduce: callback must be a function".into())),
            };
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            let (mut acc, start) = match args.get(1) {
                Some(v) => (v.clone(), 0),
                None => {
                    if len == 0 {
                        return Err(RuntimeError::TypeError("TypedArray.prototype.reduce: empty with no initial".into()));
                    }
                    (rt.object_get(this_id, "0"), 1)
                }
            };
            for i in start..len {
                let v = rt.object_get(this_id, &i.to_string());
                acc = rt.call_function(f.clone(), Value::Undefined, vec![acc, v, Value::Number(i as f64), Value::Object(this_id)])?;
            }
            Ok(acc)
        });
        register_method(self, ta_proto, "reduceRight", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Err(RuntimeError::TypeError("TypedArray.prototype.reduceRight: this must be a TypedArray".into())),
            };
            let f = match args.first() {
                Some(v @ Value::Object(_)) => v.clone(),
                _ => return Err(RuntimeError::TypeError("TypedArray.prototype.reduceRight: callback must be a function".into())),
            };
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            let (mut acc, start_back) = match args.get(1) {
                Some(v) => (v.clone(), len as i64 - 1),
                None => {
                    if len == 0 {
                        return Err(RuntimeError::TypeError("TypedArray.prototype.reduceRight: empty with no initial".into()));
                    }
                    (rt.object_get(this_id, &(len - 1).to_string()), len as i64 - 2)
                }
            };
            let mut i = start_back;
            while i >= 0 {
                let v = rt.object_get(this_id, &i.to_string());
                acc = rt.call_function(f.clone(), Value::Undefined, vec![acc, v, Value::Number(i as f64), Value::Object(this_id)])?;
                i -= 1;
            }
            Ok(acc)
        });
        register_method(self, ta_proto, "toString", |rt, _args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::String(Rc::new(String::new()))),
            };
            let len = match rt.object_get(this_id, "length") { Value::Number(n) => n as usize, _ => 0 };
            let mut out = String::new();
            for i in 0..len {
                if i > 0 { out.push(','); }
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
        let tag_getter = make_native("get @@toStringTag", |rt, _args| {
            match rt.current_this() {
                Value::Object(id) => match rt.object_get(id, "__ta_kind") {
                    v @ Value::String(_) => Ok(v),
                    _ => Ok(Value::Undefined),
                },
                _ => Ok(Value::Undefined),
            }
        });
        let tag_getter_id = self.alloc_object(tag_getter);
        let ta_proto_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(ta_proto_proto).properties.insert(
            "@@toStringTag".into(),
            crate::value::PropertyDescriptor {
                value: Value::Undefined,
                writable: false, enumerable: false, configurable: true,
                getter: Some(Value::Object(tag_getter_id)),
                setter: None,
            },
        );
        self.obj_mut(ta_proto).proto = Some(ta_proto_proto);

        for name in &[
            "ArrayBuffer", "SharedArrayBuffer", "DataView",
            "Uint8Array", "Uint8ClampedArray", "Int8Array",
            "Uint16Array", "Int16Array", "Uint32Array", "Int32Array",
            "Float32Array", "Float64Array", "BigInt64Array", "BigUint64Array",
        ] {
            let n = (*name).to_string();
            let proto_id = ta_proto;
            let ctor_obj = make_native(name, move |rt, args| {
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
                // Ω.5.P59.E6 byteLength correctness: bytes-per-element
                // per typed-array kind. Pre-P59.E6 cruftless hardcoded
                // `len * 4.0` which was wrong for every element type
                // except 32-bit ones. Bun's Uint8Array(4).byteLength === 4.
                let bpe: f64 = match n.as_str() {
                    "Int8Array" | "Uint8Array" | "Uint8ClampedArray" => 1.0,
                    "Int16Array" | "Uint16Array" => 2.0,
                    "Int32Array" | "Uint32Array" | "Float32Array" => 4.0,
                    "Float64Array" | "BigInt64Array" | "BigUint64Array" => 8.0,
                    _ => 4.0,
                };
                let mut o = Object::new_ordinary();
                o.set_own("length".into(), Value::Number(len));
                o.set_own("byteLength".into(), Value::Number(len * bpe));
                o.set_own_internal("__kind".into(), Value::String(Rc::new(n.clone())));
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
            register_intrinsic_method(self, id, "isView", 1, |_rt, _args| Ok(Value::Boolean(false)));
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
            self.obj_mut(id).set_own_frozen("prototype".into(), Value::Object(ta_proto));
            self.globals.insert((*name).to_string(), Value::Object(id));
        }
    }

    /// Tier-Ω.5.dd: WeakRef + FinalizationRegistry minimal stubs. Real
    /// weak-reference semantics need GC integration (deferred). Stubs hold
    /// strong references for v1; `.deref()` always returns the held value.
    fn install_weak_ref_globals(&mut self) {
        let weakref_proto = self.alloc_object(Object::new_ordinary());
        register_method(self, weakref_proto, "deref", |rt, _args| {
            let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
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
        self.obj_mut(wr).set_own_frozen("prototype".into(), Value::Object(weakref_proto));
        self.obj_mut(weakref_proto).set_own_internal("constructor".into(), Value::Object(wr));
        self.globals.insert("WeakRef".into(), Value::Object(wr));

        let fr_proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, fr_proto, "register", 1, |_rt, _args| Ok(Value::Undefined));
        register_intrinsic_method(self, fr_proto, "unregister", 1, |_rt, _args| Ok(Value::Boolean(true)));
        let fr_proto_for_ctor = fr_proto;
        let fr_ctor = make_native("FinalizationRegistry", move |rt, _args| {
            let mut o = Object::new_ordinary();
            o.proto = Some(fr_proto_for_ctor);
            Ok(Value::Object(rt.alloc_object(o)))
        });
        let fr = self.alloc_object(fr_ctor);
        self.obj_mut(fr).set_own_frozen("prototype".into(), Value::Object(fr_proto));
        self.obj_mut(fr_proto).set_own_internal("constructor".into(), Value::Object(fr));
        self.globals.insert("FinalizationRegistry".into(), Value::Object(fr));
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
        // Ω.5.P63.E12: Reflect.ownKeys routed through IR.
        register_intrinsic_method(self, r, "ownKeys", 1, |rt, args| {
            crate::generated::reflect_own_keys(rt, Value::Undefined, args)
        });
        // Ω.5.P63.E13: Reflect.getPrototypeOf routed through IR.
        register_intrinsic_method(self, r, "getPrototypeOf", 1, |rt, args| {
            crate::generated::reflect_get_prototype_of(rt, Value::Undefined, args)
        });
        // defineProperty / construct / apply — alias existing logic.
        if let Some(v) = self.globals.get("Object").cloned() {
            if let Value::Object(oid) = v {
                let dp = self.object_get(oid, "defineProperty");
                if !matches!(dp, Value::Undefined) { self.object_set(r, "defineProperty".into(), dp); }
                let gopd = self.object_get(oid, "getOwnPropertyDescriptor");
                if !matches!(gopd, Value::Undefined) { self.object_set(r, "getOwnPropertyDescriptor".into(), gopd); }
            }
        }
        // Tier-Ω.5.rrrrr: Reflect.setPrototypeOf / apply / construct.
        // ansi-colors uses Reflect.setPrototypeOf at module-init time;
        // without it, the import of `ansi-colors` (which calls
        // create() at the bottom) failed before module.exports was set.
        // Ω.5.P63.E13: Reflect.setPrototypeOf routed through IR.
        register_intrinsic_method(self, r, "setPrototypeOf", 2, |rt, args| {
            crate::generated::reflect_set_prototype_of(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "apply", 3, |rt, args| {
            let target = args.first().cloned().unwrap_or(Value::Undefined);
            let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
            let arg_list: Vec<Value> = match args.get(2) {
                Some(Value::Object(arr)) => {
                    let len = rt.array_length(*arr);
                    (0..len).map(|i| rt.object_get(*arr, &i.to_string())).collect()
                }
                _ => Vec::new(),
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
                    if let crate::value::InternalKind::Function(fi) =
                        &rt.obj(*id).internal_kind
                    {
                        if !fi.is_constructor {
                            return Err(RuntimeError::TypeError(format!(
                                "Reflect.construct: {} is not a constructor", fi.name
                            )));
                        }
                    }
                } else {
                    return Err(RuntimeError::TypeError(
                        "Reflect.construct: target/newTarget must be a constructor".into()));
                }
            }
            let arg_list: Vec<Value> = match args.get(1) {
                Some(Value::Object(arr)) => {
                    let len = rt.array_length(*arr);
                    (0..len).map(|i| rt.object_get(*arr, &i.to_string())).collect()
                }
                _ => Vec::new(),
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
            Ok(match ret { Value::Object(_) => ret, _ => this_obj })
        });
        // Ω.5.P63.E13: Reflect.isExtensible / preventExtensions routed through IR.
        register_intrinsic_method(self, r, "isExtensible", 1, |rt, args| {
            crate::generated::reflect_is_extensible(rt, Value::Undefined, args)
        });
        register_intrinsic_method(self, r, "preventExtensions", 1, |rt, args| {
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
            self.object_set(proto_id, "name".into(), Value::String(Rc::new((*default_name).to_string())));
            self.object_set(proto_id, "message".into(), Value::String(Rc::new("".to_string())));
            register_intrinsic_method(self, proto_id, "toString", 0, |rt, _args| {
                let this = rt.current_this();
                let (name, message) = match &this {
                    Value::Object(id) => {
                        let n = rt.object_get(*id, "name");
                        let m = rt.object_get(*id, "message");
                        (abstract_ops::to_string(&n).as_str().to_string(),
                         abstract_ops::to_string(&m).as_str().to_string())
                    }
                    _ => ("Error".into(), "".into()),
                };
                let out = if message.is_empty() { name } else { format!("{}: {}", name, message) };
                Ok(Value::String(Rc::new(out)))
            });

            let default_name = (*default_name).to_string();
            let proto_for_ctor = proto_id;
            let ctor_obj = make_native(name, move |rt, args| {
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
                if let Some(msg) = args.first() {
                    let m = abstract_ops::to_string(msg).as_str().to_string();
                    rt.object_set(id, "message".into(), Value::String(Rc::new(m)));
                }
                rt.object_set(id, "name".into(), Value::String(Rc::new(default_name.clone())));
                rt.object_set(id, "stack".into(), Value::String(Rc::new("".into())));
                Ok(Value::Object(id))
            });
            let ctor_id = self.alloc_object(ctor_obj);
            self.obj_mut(ctor_id).set_own_frozen("prototype".into(), Value::Object(proto_id));
            // proto.constructor = ctor (per spec).
            self.obj_mut(proto_id).set_own_internal("constructor".into(), Value::Object(ctor_id));
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
                        .and_then(|v| if let Value::Object(eid) = v { Some(*eid) } else { None })
                        .map(|eid| rt.object_get(eid, "prepareStackTrace"));
                    if let Some(Value::Object(_)) = &prepare {
                        let call_site = rt.alloc_object(crate::value::Object::new_ordinary());
                        register_method(rt, call_site, "getFileName", |_rt, _a| {
                            Ok(Value::String(Rc::new("<native>".into())))
                        });
                        register_method(rt, call_site, "getLineNumber", |_rt, _a| Ok(Value::Number(0.0)));
                        register_method(rt, call_site, "getColumnNumber", |_rt, _a| Ok(Value::Number(0.0)));
                        register_method(rt, call_site, "getFunctionName", |_rt, _a| {
                            Ok(Value::String(Rc::new("<anonymous>".into())))
                        });
                        register_method(rt, call_site, "getMethodName", |_rt, _a| Ok(Value::Null));
                        register_method(rt, call_site, "getTypeName", |_rt, _a| {
                            Ok(Value::String(Rc::new("<anonymous>".into())))
                        });
                        register_method(rt, call_site, "isNative", |_rt, _a| Ok(Value::Boolean(true)));
                        register_method(rt, call_site, "isConstructor", |_rt, _a| Ok(Value::Boolean(false)));
                        register_method(rt, call_site, "isToplevel", |_rt, _a| Ok(Value::Boolean(true)));
                        register_method(rt, call_site, "isEval", |_rt, _a| Ok(Value::Boolean(false)));
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
            self.globals.insert((*name).to_string(), Value::Object(ctor_id));
        }
    }

    fn install_symbol_static(&mut self) {
        // Tier-Ω.5.w: Symbol is now callable as `Symbol(desc?)`. Returns a
        // fresh Value::String of the form "@@sym:<counter>:<desc>" — the
        // counter is appended via a thread_local AtomicUsize so two calls
        // with the same description produce distinct strings (sufficient
        // for the spec's identity-distinct expectation under v1's
        // string-shaped Symbol representation).
        let sym_obj = make_native("Symbol", |_rt, args| {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let desc = args.first()
                .map(|v| crate::abstract_ops::to_string(v).as_str().to_string())
                .unwrap_or_default();
            Ok(Value::Symbol(Rc::new(format!("@@sym:{}:{}", n, desc))))
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
        for &(name, sym_str) in well_known {
            self.object_set(sym, name.into(), Value::Symbol(Rc::new(sym_str.to_string())));
        }
        register_intrinsic_method(self, sym, "for", 1, |_rt, args| {
            let s = args.first().map(|v| crate::abstract_ops::to_string(v).as_str().to_string()).unwrap_or_default();
            Ok(Value::Symbol(Rc::new(format!("@@sym:{}", s))))
        });
        register_intrinsic_method(self, sym, "keyFor", 1, |_rt, args| {
            // Ω.5.P19.E1: Symbol value type. Accept Value::Symbol only
            // (spec § 20.4.2.7 throws TypeError on non-Symbol; we return
            // undefined to match prior pragmatic laxity). Recover the key
            // from the canonical `@@sym:<key>` form Symbol.for produces.
            let s = args.first().and_then(|v| if let Value::Symbol(s) = v { Some(s.clone()) } else { None });
            match s {
                Some(s) if s.starts_with("@@sym:") && !s.contains(':') => Ok(Value::Undefined),
                Some(s) => {
                    let body = s.strip_prefix("@@sym:").unwrap_or(&s);
                    Ok(Value::String(Rc::new(body.split_once(':').map(|(_, d)| d.to_string()).unwrap_or_else(|| body.to_string()))))
                }
                _ => Ok(Value::Undefined),
            }
        });
        // Tier-Ω.5.wwww: Symbol.prototype with a toString that returns the
        // description. yup captures Symbol.prototype.toString at module init.
        let sym_proto = self.alloc_object(Object::new_ordinary());
        register_intrinsic_method(self, sym_proto, "toString", 0, |rt, _args| {
            match rt.current_this() {
                Value::Symbol(s) => {
                    let body = s.strip_prefix("@@sym:").unwrap_or(&s);
                    let desc = body.split_once(':').map(|(_, d)| d).unwrap_or(body);
                    Ok(Value::String(Rc::new(format!("Symbol({})", desc))))
                }
                v => Ok(Value::String(Rc::new(crate::abstract_ops::to_string(&v).as_str().to_string()))),
            }
        });
        self.obj_mut(sym).set_own_frozen("prototype".into(), Value::Object(sym_proto));
        self.globals.insert("Symbol".into(), Value::Object(sym));
    }

    fn install_console(&mut self) {
        let console = self.alloc_object(Object::new_ordinary());
        register_method(self, console, "log", |_rt, args|{
            let mut out = String::new();
            for (i, a) in args.iter().enumerate() {
                if i > 0 { out.push(' '); }
                out.push_str(&abstract_ops::to_string(a));
            }
            println!("{}", out);
            Ok(Value::Undefined)
        });
        register_method(self, console,"error", |_rt, args|{
            let mut out = String::new();
            for (i, a) in args.iter().enumerate() {
                if i > 0 { out.push(' '); }
                out.push_str(&abstract_ops::to_string(a));
            }
            eprintln!("{}", out);
            Ok(Value::Undefined)
        });
        register_method(self, console,"warn", |_rt, args|{
            let mut out = String::new();
            for (i, a) in args.iter().enumerate() {
                if i > 0 { out.push(' '); }
                out.push_str(&abstract_ops::to_string(a));
            }
            eprintln!("{}", out);
            Ok(Value::Undefined)
        });
        self.globals.insert("console".into(), Value::Object(console));
    }
}

/// Drain an iterable's @@iterator into a Vec<Value>. Used by
/// Object.fromEntries / Array.from.
/// Tier-Ω.5.rrr: build a values-iterator for a Set. The iterator object
/// snapshots the Set's current values into a private array and exposes a
/// next() that yields each in turn. Sufficient for `[...new Set(arr)]`
/// spread.
pub(crate) fn make_set_values_iterator(rt: &mut Runtime, set_id: crate::value::ObjectRef) -> Result<Value, RuntimeError> {
    let values: Vec<Value> = match rt.object_get(set_id, "__set_data") {
        Value::Object(storage) => {
            rt.obj(storage).properties.values().map(|d| d.value.clone()).collect()
        }
        _ => Vec::new(),
    };
    // Build an iterator object: { __idx: 0, __vals: [v0,v1,...], next() }
    let iter = rt.alloc_object(Object::new_ordinary());
    let vals_arr = rt.alloc_object(Object::new_array());
    for (i, v) in values.iter().enumerate() {
        rt.object_set(vals_arr, i.to_string(), v.clone());
    }
    rt.object_set(vals_arr, "length".into(), Value::Number(values.len() as f64));
    rt.object_set(iter, "__vals".into(), Value::Object(vals_arr));
    rt.object_set(iter, "__idx".into(), Value::Number(0.0));
    register_intrinsic_method(rt, iter, "next", 1, |rt, _args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
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
    let id = match src {
        Value::Object(id) => id,
        _ => return Ok(Vec::new()),
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
            _ => return Err(RuntimeError::TypeError("iterator next did not return an object".into())),
        };
        let done = abstract_ops::to_boolean(&rt.object_get(rid, "done"));
        if done { break; }
        out.push(rt.object_get(rid, "value"));
    }
    Ok(out)
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
pub(crate) fn make_error_instance(rt: &mut Runtime, ctor_name: &str, message: &str) -> Option<rusty_js_gc::ObjectId> {
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
    o.set_own("message".into(), Value::String(Rc::new(message.to_string())));
    o.set_own("stack".into(), Value::String(Rc::new(String::new())));
    Some(rt.alloc_object(o))
}

/// Ω.5.P59.E6: allocate a same-kind TypedArray-like instance from a
/// source TypedArray, used by .map / .filter to satisfy ECMA §23.2.3.21
/// TypedArraySpeciesCreate semantics at the shape level (length +
/// byteLength + __kind sentinel + proto inheritance from source).
fn make_typed_array_like(rt: &mut Runtime, src: rusty_js_gc::ObjectId, len: usize) -> rusty_js_gc::ObjectId {
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
    let src_len = match rt.object_get(src, "length") { Value::Number(n) => n, _ => 1.0 };
    let bpe = if src_len > 0.0 { src_byte_len / src_len } else { 1.0 };
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

pub(crate) fn make_native(name: &str, f: impl Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static) -> Object {
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
    }
}

fn register_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
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
    rt.obj_mut(host).set_own_internal(name.into(), Value::Object(fn_id));
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
pub(crate) fn register_intrinsic_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, length: u32, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    // Ω.5.P61.E4: intrinsic methods are non-constructors per ECMA §21.3
    // (and the same applies to every built-in not identified as a
    // constructor — Object.keys, String.prototype.includes, Array.
    // prototype.map, etc.). make_native_non_ctor sets the flag so
    // Op::New + Reflect.construct throw TypeError on `new Math.abs()`.
    let fn_obj = make_native_non_ctor(name, length, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.obj_mut(host).properties.insert(name.to_string(), crate::value::PropertyDescriptor {
        value: Value::Object(fn_id),
        writable: true, enumerable: false, configurable: true,
        getter: None, setter: None,
    });
}

fn register_global_fn<F>(rt: &mut Runtime, name: &str, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    let fn_obj = make_native(name, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.globals.insert(name.into(), Value::Object(fn_id));
}

/// Ω.5.P55.E1 (Doc 729 §VII.B): register a compiler-emitted lowering
/// behind the engine-internal bilateral boundary. The helper resolves
/// through `Op::LoadGlobal`'s fallback path (interp.rs) but does not
/// appear in `globals`, so `globalThis.__X` reads as `undefined` and
/// `Object.keys(globalThis)` does not enumerate it.
fn register_engine_helper<F>(rt: &mut Runtime, name: &str, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    let fn_obj = make_native(name, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.engine_helpers.insert(name.into(), Value::Object(fn_id));
}

// ──────────────── JSON.stringify (limited) ────────────────

fn json_stringify(rt: &Runtime, v: &Value) -> String {
    match v {
        Value::Undefined => "undefined".into(),
        Value::Null => "null".into(),
        Value::Boolean(b) => b.to_string(),
        Value::Number(n) => {
            if n.is_finite() { abstract_ops::number_to_string(*n) } else { "null".into() }
        }
        Value::String(s) => json_quote_string(s.as_str()),
        Value::BigInt(_) => "null".into(),
        // ECMA §25.5.2.4 SerializeJSONProperty: Symbol values serialize to
        // undefined and the enclosing object omits the key. We surface
        // "undefined" here; the caller's per-property filter at the object
        // branch elides keys whose serialized form is "undefined".
        Value::Symbol(_) => "undefined".into(),
        Value::Object(id) => {
            // Snapshot the props (clones Value) to avoid recursive borrow.
            let (is_array, props): (bool, Vec<(String, PropertyDescriptor)>) = {
                let obj = rt.obj(*id);
                let is_array = matches!(obj.internal_kind, InternalKind::Array);
                let v: Vec<_> = obj.properties.iter()
                    .map(|(k, d)| (k.clone(), d.clone())).collect();
                (is_array, v)
            };
            if is_array {
                let mut entries: Vec<(usize, String)> = props.iter()
                    .filter_map(|(k, d)| k.parse::<usize>().ok().map(|i| (i, json_stringify(rt, &d.value))))
                    .collect();
                entries.sort_by_key(|(i, _)| *i);
                let body: Vec<String> = entries.into_iter().map(|(_, s)| s).collect();
                format!("[{}]", body.join(","))
            } else {
                // Ω.5.P19.E1: JSON.stringify ignores Symbol-keyed properties
                // per ECMA §25.5.2.4 (the `@@` prefix on both user symbols
                // and well-known-symbol slots). Also skip values whose
                // serialized form is `"undefined"` (covers Symbol values
                // too — the upper-level Symbol match returns "undefined").
                let entries: Vec<String> = props.iter()
                    .filter(|(k, d)| d.enumerable && !k.starts_with("@@") && !matches!(d.value, Value::Undefined | Value::Symbol(_)))
                    .map(|(k, d)| format!("{}:{}", json_quote_string(k), json_stringify(rt, &d.value)))
                    .collect();
                format!("{{{}}}", entries.join(","))
            }
        }
    }
}

fn json_quote_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// ──────────────── JSON.parse (limited recursive-descent) ────────────────

pub fn json_parse(rt: &mut Runtime, s: &str) -> Result<Value, RuntimeError> {
    let bytes = s.as_bytes();
    let mut p = 0;
    skip_ws(bytes, &mut p);
    let v = json_parse_value(rt, bytes, &mut p)?;
    skip_ws(bytes, &mut p);
    if p != bytes.len() {
        return Err(RuntimeError::SyntaxError("JSON.parse: trailing characters".into()));
    }
    Ok(v)
}

fn skip_ws(b: &[u8], p: &mut usize) {
    while *p < b.len() && matches!(b[*p], b' ' | b'\t' | b'\n' | b'\r') { *p += 1; }
}

fn json_parse_value(rt: &mut Runtime, b: &[u8], p: &mut usize) -> Result<Value, RuntimeError> {
    skip_ws(b, p);
    if *p >= b.len() { return Err(RuntimeError::SyntaxError("JSON.parse: unexpected end".into())); }
    match b[*p] {
        b'{' => json_parse_object(rt, b, p),
        b'[' => json_parse_array(rt, b, p),
        b'"' => json_parse_string(b, p).map(|s| Value::String(Rc::new(s))),
        b't' if b[*p..].starts_with(b"true") => { *p += 4; Ok(Value::Boolean(true)) }
        b'f' if b[*p..].starts_with(b"false") => { *p += 5; Ok(Value::Boolean(false)) }
        b'n' if b[*p..].starts_with(b"null") => { *p += 4; Ok(Value::Null) }
        b'-' | b'0'..=b'9' => json_parse_number(b, p),
        _ => Err(RuntimeError::SyntaxError(format!("JSON.parse: unexpected character at offset {}", p))),
    }
}

fn json_parse_object(rt: &mut Runtime, b: &[u8], p: &mut usize) -> Result<Value, RuntimeError> {
    *p += 1; // consume '{'
    let obj = rt.alloc_object(Object::new_ordinary());
    skip_ws(b, p);
    if *p < b.len() && b[*p] == b'}' { *p += 1; return Ok(Value::Object(obj)); }
    loop {
        skip_ws(b, p);
        let key = json_parse_string(b, p)?;
        skip_ws(b, p);
        if *p >= b.len() || b[*p] != b':' { return Err(RuntimeError::SyntaxError("JSON.parse: expected ':'".into())); }
        *p += 1;
        let value = json_parse_value(rt, b, p)?;
        rt.object_set(obj, key, value);
        skip_ws(b, p);
        match b.get(*p) {
            Some(&b',') => { *p += 1; continue; }
            Some(&b'}') => { *p += 1; return Ok(Value::Object(obj)); }
            _ => return Err(RuntimeError::SyntaxError("JSON.parse: expected ',' or '}'".into())),
        }
    }
}

fn json_parse_array(rt: &mut Runtime, b: &[u8], p: &mut usize) -> Result<Value, RuntimeError> {
    *p += 1; // consume '['
    let arr = rt.alloc_object(Object::new_array());
    skip_ws(b, p);
    if *p < b.len() && b[*p] == b']' { *p += 1; return Ok(Value::Object(arr)); }
    let mut i = 0u32;
    loop {
        let value = json_parse_value(rt, b, p)?;
        rt.object_set(arr, i.to_string(), value);
        i += 1;
        skip_ws(b, p);
        match b.get(*p) {
            Some(&b',') => { *p += 1; continue; }
            Some(&b']') => { *p += 1; return Ok(Value::Object(arr)); }
            _ => return Err(RuntimeError::SyntaxError("JSON.parse: expected ',' or ']'".into())),
        }
    }
}

fn json_parse_string(b: &[u8], p: &mut usize) -> Result<String, RuntimeError> {
    if *p >= b.len() || b[*p] != b'"' {
        return Err(RuntimeError::SyntaxError("JSON.parse: expected string".into()));
    }
    *p += 1;
    let mut out = String::new();
    while *p < b.len() {
        let c = b[*p];
        if c == b'"' { *p += 1; return Ok(out); }
        if c == b'\\' {
            *p += 1;
            if *p >= b.len() { return Err(RuntimeError::SyntaxError("JSON.parse: dangling \\".into())); }
            match b[*p] {
                b'"' => out.push('"'),
                b'\\' => out.push('\\'),
                b'/' => out.push('/'),
                b'n' => out.push('\n'),
                b'r' => out.push('\r'),
                b't' => out.push('\t'),
                b'b' => out.push('\u{0008}'),
                b'f' => out.push('\u{000C}'),
                b'u' if *p + 4 < b.len() => {
                    let hex = std::str::from_utf8(&b[*p+1..*p+5]).map_err(|_|RuntimeError::SyntaxError("JSON.parse: bad \\u".into()))?;
                    let cp = u32::from_str_radix(hex, 16).map_err(|_|RuntimeError::SyntaxError("JSON.parse: bad \\u".into()))?;
                    if let Some(ch) = char::from_u32(cp) { out.push(ch); }
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
                    "JSON.parse: invalid control character in string".into()));
            }
            out.push(c as char);
            *p += 1;
        }
    }
    Err(RuntimeError::SyntaxError("JSON.parse: unterminated string".into()))
}

fn json_parse_number(b: &[u8], p: &mut usize) -> Result<Value, RuntimeError> {
    let start = *p;
    if b[*p] == b'-' { *p += 1; }
    while *p < b.len() && b[*p].is_ascii_digit() { *p += 1; }
    if *p < b.len() && b[*p] == b'.' {
        *p += 1;
        while *p < b.len() && b[*p].is_ascii_digit() { *p += 1; }
    }
    if *p < b.len() && (b[*p] == b'e' || b[*p] == b'E') {
        *p += 1;
        if *p < b.len() && (b[*p] == b'+' || b[*p] == b'-') { *p += 1; }
        while *p < b.len() && b[*p].is_ascii_digit() { *p += 1; }
    }
    let s = std::str::from_utf8(&b[start..*p]).map_err(|_|RuntimeError::SyntaxError("JSON.parse: bad number".into()))?;
    let n = s.parse::<f64>().map_err(|_|RuntimeError::SyntaxError("JSON.parse: bad number".into()))?;
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
    if s.is_empty() { return false; }
    if s == "0" { return true; }
    if s.starts_with('0') { return false; }
    if !s.chars().all(|c| c.is_ascii_digit()) { return false; }
    match s.parse::<u64>() {
        Ok(n) if n < ((1u64 << 32) - 1) => true,
        _ => false,
    }
}

fn base64_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    let mut i = 0;
    while i + 3 <= input.len() {
        let n = ((input[i] as u32) << 16) | ((input[i+1] as u32) << 8) | (input[i+2] as u32);
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
        let n = ((input[i] as u32) << 16) | ((input[i+1] as u32) << 8);
        out.push(B64_ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[((n >> 6) & 0x3F) as usize] as char);
        out.push('=');
    }
    out
}
fn base64_decode(s: &str) -> Result<Vec<u8>, &'static str> {
    let mut lut = [255u8; 256];
    for (i, &c) in B64_ALPHABET.iter().enumerate() { lut[c as usize] = i as u8; }
    let bytes: Vec<u8> = s.bytes().filter(|&b| b != b'=').collect();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    let mut i = 0;
    while i + 4 <= bytes.len() {
        let (a, b, c, d) = (lut[bytes[i] as usize], lut[bytes[i+1] as usize], lut[bytes[i+2] as usize], lut[bytes[i+3] as usize]);
        if (a | b | c | d) == 255 { return Err("invalid base64 character"); }
        let n = ((a as u32) << 18) | ((b as u32) << 12) | ((c as u32) << 6) | (d as u32);
        out.push(((n >> 16) & 0xFF) as u8);
        out.push(((n >> 8) & 0xFF) as u8);
        out.push((n & 0xFF) as u8);
        i += 4;
    }
    let rem = bytes.len() - i;
    if rem == 2 {
        let (a, b) = (lut[bytes[i] as usize], lut[bytes[i+1] as usize]);
        if (a | b) == 255 { return Err("invalid base64 character"); }
        let n = ((a as u32) << 18) | ((b as u32) << 12);
        out.push(((n >> 16) & 0xFF) as u8);
    } else if rem == 3 {
        let (a, b, c) = (lut[bytes[i] as usize], lut[bytes[i+1] as usize], lut[bytes[i+2] as usize]);
        if (a | b | c) == 255 { return Err("invalid base64 character"); }
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
fn date_components(ms: f64) -> (i64, i64, i64) {
    let days = (ms / 86_400_000.0).floor() as i64;
    // Days since 1970-01-01.
    // Convert to year, month, day via Gregorian algorithm.
    let mut z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe/4 - yoe/100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    z = m - 1; // month 0-based
    let _ = z;
    (year, m - 1, d)
}

/// Build epoch-ms from (year, month-0-based, day-1-based).
fn ymd_to_ms(year: i64, month: i64, day: i64) -> i64 {
    let y = if month < 2 { year - 1 } else { year };
    let m = if month < 2 { (month + 9) as i64 } else { (month - 2) as i64 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * m + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days_since_epoch = era * 146097 + doe - 719468;
    days_since_epoch * 86_400_000
}

/// Parse a Date string. Supports:
/// - "YYYY-MM-DD"
/// - "YYYY-MM-DDTHH:MM:SS"
/// - "YYYY-MM-DDTHH:MM:SS.sssZ"
/// Returns f64 ms-since-epoch, or NaN on parse failure.
fn parse_date_string(s: &str) -> f64 {
    let s = s.trim();
    if s.len() < 10 { return f64::NAN; }
    let y: i64 = match s[0..4].parse() { Ok(v) => v, Err(_) => return f64::NAN };
    if s.as_bytes()[4] != b'-' { return f64::NAN; }
    let mo: i64 = match s[5..7].parse() { Ok(v) => v, Err(_) => return f64::NAN };
    if s.as_bytes()[7] != b'-' { return f64::NAN; }
    let d: i64 = match s[8..10].parse() { Ok(v) => v, Err(_) => return f64::NAN };
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
