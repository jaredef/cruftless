//! Bytecode dispatch loop + Runtime + Frame management.
//! Per specs/rusty-js-runtime-design.md §III.

use crate::abstract_ops::*;
use crate::value::{new_upvalue_cell, InternalKind, Object, ObjectRef, UpvalueCell, Value};
use rusty_js_bytecode::{
    op::{decode_i32, decode_u16, op_from_byte, Op},
    CompiledModule,
};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    CompileError(String),
    TypeError(String),
    ReferenceError(String),
    RangeError(String),
    /// Ω.5.P62.E22: SyntaxError per ECMA — used by JSON.parse,
    /// RegExp ctor, Function ctor on malformed source, etc.
    SyntaxError(String),
    Unimplemented(String),
    /// Thrown JS value bubbling up the call stack.
    Thrown(Value),
}

pub struct Runtime {
    pub globals: HashMap<String, Value>,
    /// Ω.5.P55.E1 (Doc 729 §VII.B — engine-internal bilateral boundary).
    /// Compiler-emitted lowerings (`__await`, `__dynamic_import`, `__apply`,
    /// `__construct`, `__install_accessor__`, `__yield_push__`,
    /// `__yield_delegate__`, `__object_spread`, `__array_push_single`,
    /// `__array_extend`, `__destr_array_rest`, `__destr_object_rest`)
    /// resolve through this table on a LoadGlobal miss. They do not live
    /// in `globals`, so `Object.keys(globalThis)` does not enumerate them
    /// and `globalThis.__apply` reads as `undefined` from JS — closing the
    /// SERVER §4.1 engine-internal bilateral boundary per Doc 432.
    /// A JS-side `globalThis.X = ...` assignment writes to `globals` and
    /// thereby shadows the engine helper (standard global-resolution
    /// semantics); the fallback only fires on the unshadowed path.
    /// Diagnostic probes (`__resolution_trace`, `__post_eval_trace`,
    /// `__ns_synth_trace`, `__symbol_lookup_log`, `__host_stub_log`,
    /// `__operator_trace_size`) remain in `globals` — they are
    /// parity-script-callable by design (Doc 729 §XII probe surface).
    pub engine_helpers: HashMap<String, Value>,
    pub last_value: Value,
    pub host_hooks: crate::module::HostHooks,
    /// Tier-Ω.5.b: ESM module cache keyed by resolved URL
    /// (`file://...` for disk-backed modules, `node:foo` for built-ins).
    /// Interior mutability lets `evaluate_module` insert a Linking record
    /// before recursing into imports, so cyclic loads observe the partial
    /// namespace rather than re-entering parse/compile.
    pub modules: HashMap<String, std::rc::Rc<std::cell::RefCell<crate::module::ModuleRecord>>>,
    /// Tier-Ω.5.q: parsed package.json cache keyed by absolute package.json
    /// path. Bare-specifier resolution walks node_modules; without caching,
    /// a single import re-reads and re-parses package.json once per call.
    /// Inserted on first read; never invalidated (filesystem changes during
    /// runtime are out-of-scope for v1).
    pub pkg_json_cache: HashMap<std::path::PathBuf, std::rc::Rc<crate::module::ParsedPackageJson>>,
    /// Ω.5.P54.E1 (Axis-M probe — Doc 729 §XII): resolution-decision
    /// trace keyed by resolved URL. Populated by resolve_entry_point
    /// when a bare specifier maps to a file under a node_modules pkg.
    /// Trace string format: "spec='X' chose={url} via={rule}; alternatives={k=v,...}".
    /// Surfaced in error formatters whose receiver carries an attached
    /// source URL — turns Axis-M wrong-file picks (mri/heap-js/etc.)
    /// into self-naming failures rather than downstream callee_val-undefined.
    pub module_resolution_trace: HashMap<String, String>,
    /// Ω.5.P54.E2 (Axis-E probe — Doc 729 §XII): post-evaluation
    /// observations keyed by URL. Records (key_count, kind, last_throw_msg).
    /// Populated by evaluate_module / evaluate_cjs_module after namespace
    /// finalization. Empty-namespace results — whether from a swallowed
    /// throw, a kind-detection cut (heap-js .js-treated-as-CJS), or an
    /// otherwise-correct module that exports nothing — surface as a
    /// queryable record rather than projecting downstream onto
    /// "callee_val=undefined" type failures.
    pub module_post_eval_trace: HashMap<String, String>,
    /// Ω.5.P54.E3 (Axis-N probe — Doc 729 §XII): namespace synth-path
    /// tag keyed by URL. Records which composition path produced the
    /// namespace: "CJS-populate __esModule=B strip=B", "ESM-finalize
    /// Tuple-A-empty", "ESM-finalize Tuple-A-wide", "P53.E13 lift
    /// {gate=A/B/C}". Property-miss diagnostics append the tag so
    /// Axis-N walks can locate which synth branch produced the surface.
    pub module_ns_synth_trace: HashMap<String, String>,
    /// Ω.5.P54.E4 (Axis-S probe — Doc 729 §XII): well-known-symbol-key
    /// access misses keyed by the canonical name. Recorded by GetProp /
    /// GetIndex / CallMethod when the key matches a Symbol.X sentinel
    /// AND the lookup returned Undefined. Empty in the happy path; on
    /// failure surfaces "looked for Symbol.X on receiver-shape Y".
    pub symbol_lookup_miss_log: Vec<String>,
    /// Ω.5.P54.E5 (Axis-H probe — Doc 729 §XII): host-built-in surface
    /// gap log. Each entry records "{module_namespace}.{method}" when
    /// a CallMethod on a node:* namespace yielded method=Undefined.
    /// Distinct from symbol-miss; this targets the Bun-version-cadence
    /// catch-up problem (events, es-errors, etc).
    pub host_stub_miss_log: Vec<String>,
    /// Ω.5.P54.E6 (Axis-O probe — Doc 729 §XII): operator-lowering name
    /// trail keyed by source span (start,end). Each compile_logical_assign
    /// / compile_compound_member / compile_optional_chain emits its
    /// operator's canonical name; error formatter walks pc → span → name
    /// to surface "in compile_X(operator)" rather than bytecode-level
    /// stack diagnostics. Stub map; population threads through compiler.rs.
    pub operator_lowering_trace: HashMap<(usize, usize), String>,
    /// Managed heap. Wired but not yet authoritative for Value::Object;
    /// round 3.e.d migrates Value::Object from Rc<RefCell<Object>> to
    /// ObjectId, at which point this heap becomes the storage for every
    /// allocated Object.
    pub heap: rusty_js_gc::Heap<crate::value::Object>,
    /// Event-loop job queue per ECMA-262 §9.4 + WHATWG HTML §8.
    /// Engine-owned; replaces the pre-Ω cruftless-rquickjs's mio + JS-side
    /// __keepAlive + __tickKeepAlive split. Per Doc 714 §VI Consequence 5.
    pub job_queue: crate::job_queue::JobQueue,
    /// Promises that have been rejected with no reject handler attached.
    /// Per ECMA-262 §27.2.1.9 HostPromiseRejectionTracker: the host is
    /// notified at end-of-job for any rejection still without a handler.
    /// Drained by `drain_unhandled_rejections()` after run_to_completion.
    pub pending_unhandled: HashSet<rusty_js_gc::ObjectId>,
    /// `this` visible to a native function during its invocation. Set by
    /// call_function before dispatching into a NativeFn; native handlers
    /// read it via `rt.current_this()`. Tier-Ω.5.a: preserves the existing
    /// `Fn(&mut Runtime, &[Value])` NativeFn signature (no cascade through
    /// host-v2/* intrinsics) while still letting Function.prototype.call,
    /// Array.prototype.map's callback dispatch, and the like see a real
    /// receiver. Saved/restored across nested calls.
    pub current_this: Value,
    /// Tier-Ω.5.s: `new.target` slot pending injection into the next
    /// closure frame to be entered via `call_function`. Set by Op::New
    /// before dispatching, consumed (take()) at frame construction.
    /// Native frames don't read it directly; they call current_new_target()
    /// if they need the value. Mirrors current_this's save/restore shape
    /// for native dispatch.
    pub pending_new_target: Option<Value>,
    pub current_new_target: Option<Value>,
    // ─── Intrinsic prototypes (Tier-Ω.5.a) ───
    //
    // Stashed ObjectIds for the canonical prototype objects. Each
    // Object that ought to inherit from one of these has its `proto`
    // field set at allocation time:
    //   - Ordinary objects -> object_prototype
    //   - Array objects    -> array_prototype
    //   - Function/Closure/BoundFunction -> function_prototype
    //   - Promise          -> promise_prototype
    // Strings + Numbers + Booleans are primitives — their method dispatch
    // routes through these stashes via `Runtime::lookup_method_on_value`
    // without allocating a wrapper.
    pub object_prototype: Option<rusty_js_gc::ObjectId>,
    pub array_prototype: Option<rusty_js_gc::ObjectId>,
    pub function_prototype: Option<rusty_js_gc::ObjectId>,
    pub promise_prototype: Option<rusty_js_gc::ObjectId>,
    pub string_prototype: Option<rusty_js_gc::ObjectId>,
    pub number_prototype: Option<rusty_js_gc::ObjectId>,
    pub bigint_prototype: Option<rusty_js_gc::ObjectId>,
    /// Tier-Ω.5.i: %RegExp.prototype% — installed alongside other
    /// intrinsic prototypes; alloc_object auto-wires RegExp objects.
    pub regexp_prototype: Option<rusty_js_gc::ObjectId>,
    /// Tier-Ω.5.gggggg: stack of currently-running generator yields-arrays.
    /// Each generator function invocation pushes a fresh array on entry,
    /// pops it on completion. `__yield_push__(v)` appends to the top.
    /// Nested generators (yield inside a generator that yields a generator)
    /// stack correctly.
    pub gen_yields_stack: Vec<rusty_js_gc::ObjectId>,
    /// Tier-Ω.5.P23.E1.live-import-bindings: per-source-URL registry of
    /// import-bindings whose source module was still Linking at evaluate-
    /// time. When the source module's evaluation completes, the registry
    /// is drained and each cell receives the resolved binding value.
    /// Enables ECMA-262 §16.2.1.5 live binding semantics for the
    /// common circular-import case where module M imports a default/
    /// named export from module N while N is still loading M.
    /// Key: source-module URL. Value: list of (cell, binding-kind,
    /// optional-named-binding-name) tuples to update at drain time.
    pub pending_live_bindings: HashMap<String, Vec<crate::module::DeferredImportBinding>>,
    /// Tier-Ω.5.P34.E1.fd-table: host-side file-descriptor registry for
    /// node:fs ops that take an integer fd (openSync, closeSync,
    /// fsync/fdatasync/futimes/ftruncate, writeSync, readSync, etc.).
    /// Keyed by the fd integer the engine hands back to JS (starting at
    /// 3 to skip stdin/stdout/stderr). Insertion: openSync. Removal:
    /// closeSync. Lives on Runtime so it survives across native callbacks
    /// without per-call construction.
    pub fd_table: HashMap<i32, std::fs::File>,
    pub next_fd: i32,
    /// Ω.5.P46.E1.napi-v1: dlopen'd .node native modules. Held forever
    /// (until process exit) so their function pointers stay valid across
    /// later JS calls into the module.
    pub napi_libs: Vec<libloading::Library>,
    /// Ω.5.P46.E1.napi-v1: live NapiEnv boxes — one per loaded .node
    /// module. Roots their handle tables + ref tables so values survive
    /// across JS↔native boundaries.
    pub napi_envs: Vec<Box<crate::napi::NapiEnv>>,
    /// Ω.5.P46.E1.napi-v1: cache of `.node` module exports keyed by
    /// resolved URL. Separate from `modules` so we don't need to
    /// synthesize a CompiledModule for native libraries.
    pub napi_module_cache: HashMap<String, Value>,
    /// Ω.5.P46.E2.napi-async: thread-safe inbox for jobs enqueued from
    /// worker threads (async_work completion callbacks, threadsafe-
    /// function call requests). PollIo drains this between fs ops and
    /// the watcher poll. Boxed closures so each job carries its own
    /// captured state.
    pub napi_main_inbox: std::sync::Arc<std::sync::Mutex<std::collections::VecDeque<crate::napi::NapiMainJob>>>,
    /// Ω.5.P46.E3.napi-keepalive: count of napi resources currently
    /// holding the event loop alive (refd threadsafe functions, in-flight
    /// async work). PollIo's `has_pending` consults this; the loop won't
    /// exit while > 0.
    pub napi_keepalive: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    /// Tier-Ω.5.P45.E1.module-url-stack: stack of URLs of modules
    /// currently being evaluated. evaluate_module / evaluate_cjs_module
    /// push the URL before running the frame and pop after. `__dynamic_import`
    /// consults `.last()` to resolve relative specifiers against the
    /// actual calling module's URL (per ECMA-262 §16.2.1.8 step 5,
    /// which specifies the referencing script/module as the resolution
    /// origin). Pre-fix the cwd-fallback parent_url worked for bare/
    /// node: specifiers but broke relative imports from packages
    /// (e.g. nx's `import('../src/native/...')` resolving against the
    /// caller's cwd instead of the nx package's own location).
    pub current_module_url: Vec<String>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            engine_helpers: HashMap::new(),
            last_value: Value::Undefined,
            host_hooks: crate::module::HostHooks::default(),
            modules: HashMap::new(),
            pkg_json_cache: HashMap::new(),
            module_resolution_trace: HashMap::new(),
            module_post_eval_trace: HashMap::new(),
            module_ns_synth_trace: HashMap::new(),
            symbol_lookup_miss_log: Vec::new(),
            host_stub_miss_log: Vec::new(),
            operator_lowering_trace: HashMap::new(),
            heap: rusty_js_gc::Heap::new(),
            job_queue: crate::job_queue::JobQueue::new(),
            pending_unhandled: HashSet::new(),
            current_this: Value::Undefined,
            pending_new_target: None,
            current_new_target: None,
            object_prototype: None,
            array_prototype: None,
            function_prototype: None,
            promise_prototype: None,
            string_prototype: None,
            number_prototype: None,
            bigint_prototype: None,
            regexp_prototype: None,
            gen_yields_stack: Vec::new(),
            pending_live_bindings: HashMap::new(),
            fd_table: HashMap::new(),
            next_fd: 3,
            current_module_url: Vec::new(),
            napi_libs: Vec::new(),
            napi_envs: Vec::new(),
            napi_module_cache: HashMap::new(),
            napi_main_inbox: std::sync::Arc::new(std::sync::Mutex::new(std::collections::VecDeque::new())),
            napi_keepalive: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    /// `this` for the active native call. Returns Undefined outside one.
    pub fn current_this(&self) -> Value { self.current_this.clone() }

    /// Ω.5.P60.E4 + Ω.5.P61.E16: full ECMA §7.1.17 ToString with
    /// §7.1.1.1 OrdinaryToPrimitive('string') for Object values:
    /// (1) if obj[@@toPrimitive] is callable, call with hint 'string';
    /// (2) else call obj.toString() if callable and primitive return;
    /// (3) else call obj.valueOf() if callable and primitive return;
    /// (4) all three returned Objects → TypeError per §7.1.1.1 step 6.
    /// Ω.5.P62.E21: ToPrimitive per ECMA §7.1.1 — when `v` is an Object,
    /// dispatch through @@toPrimitive(hint) → then either valueOf-then-
    /// toString (default/number hint) or toString-then-valueOf (string
    /// hint) per §7.1.1.1 OrdinaryToPrimitive. Returns the first
    /// primitive produced; throws TypeError if all dispatches return
    /// Objects. For non-Object input, returns v unchanged.
    pub fn to_primitive(&mut self, v: &Value, hint: &str) -> Result<Value, RuntimeError> {
        if let Value::Object(id) = v {
            let id = *id;
            // (1) @@toPrimitive (string or number hint).
            let tp = self.object_get(id, "@@toPrimitive");
            if matches!(tp, Value::Object(_)) {
                let r = self.call_function(tp, v.clone(), vec![
                    Value::String(Rc::new(hint.into())),
                ])?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(r);
                }
                return Err(RuntimeError::TypeError(
                    "Cannot convert object to primitive value".into()));
            }
            // (2) OrdinaryToPrimitive — order depends on hint.
            let methods: [&str; 2] = if hint == "string" {
                ["toString", "valueOf"]
            } else {
                ["valueOf", "toString"]
            };
            for m in methods {
                let f = self.object_get(id, m);
                if matches!(f, Value::Object(_)) {
                    let r = self.call_function(f, v.clone(), Vec::new())?;
                    if !matches!(r, Value::Object(_)) {
                        return Ok(r);
                    }
                }
            }
            return Err(RuntimeError::TypeError(
                "Cannot convert object to primitive value".into()));
        }
        Ok(v.clone())
    }

    /// Ω.5.P62.E21: op_add with Object→primitive dispatch per ECMA
    /// §13.15.4. If either operand is Object, ToPrimitive(default) on
    /// both; then if either resulting primitive is String, concatenate;
    /// else numeric add. Pure-primitive case delegates to
    /// abstract_ops::op_add for the common fast path.
    pub fn op_add_rt(&mut self, l: &Value, r: &Value) -> Result<Value, RuntimeError> {
        let lp = self.to_primitive(l, "default")?;
        let rp = self.to_primitive(r, "default")?;
        Ok(crate::abstract_ops::op_add(&lp, &rp))
    }

    /// Ω.5.P62.E21: loose-equality with Object→primitive dispatch per
    /// ECMA §7.2.13 step 12/13. When one side is Object and the other
    /// is a primitive, ToPrimitive(Object, default) and re-compare;
    /// throws TypeError up the chain if dispatch fails.
    pub fn is_loosely_equal_rt(&mut self, a: &Value, b: &Value) -> Result<bool, RuntimeError> {
        // Same-type or both-non-Object fast path.
        if !matches!(a, Value::Object(_)) && !matches!(b, Value::Object(_)) {
            return Ok(crate::abstract_ops::is_loosely_equal(a, b));
        }
        if matches!(a, Value::Object(_)) && matches!(b, Value::Object(_)) {
            // Both Objects: SameValue (reference equality).
            return Ok(crate::abstract_ops::is_strictly_equal(a, b));
        }
        // One Object, one primitive: ToPrimitive on the Object side.
        if matches!(a, Value::Object(_)) {
            let ap = self.to_primitive(a, "default")?;
            return Ok(crate::abstract_ops::is_loosely_equal(&ap, b));
        }
        let bp = self.to_primitive(b, "default")?;
        Ok(crate::abstract_ops::is_loosely_equal(a, &bp))
    }

    // ──────────────── IR lowering targets (rusty-js-ir Tier 1.5) ────────────────
    // These helpers exist specifically to be the Rust callsites that
    // lowered IR functions invoke. They wrap existing low-level
    // operations into &Value-taking, Result-returning forms suitable
    // for spec-faithful translation. See IR-DESIGN.md §3 and seed §A8.33.

    /// ToObject per ECMA §7.1.18 — coerces primitives to wrapper objects,
    /// throws TypeError on undefined/null.
    pub fn to_object(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        match v {
            Value::Undefined | Value::Null => Err(RuntimeError::TypeError(
                "Cannot convert undefined or null to object".into())),
            Value::Object(_) => Ok(v.clone()),
            Value::Boolean(b) => {
                let mut o = crate::value::Object::new_ordinary();
                o.set_own_internal("__primitive__".into(), Value::Boolean(*b));
                if let Some(Value::Object(bid)) = self.globals.get("Boolean").cloned() {
                    if let Value::Object(p) = self.object_get(bid, "prototype") {
                        o.proto = Some(p);
                    }
                }
                Ok(Value::Object(self.alloc_object(o)))
            }
            Value::Number(n) => {
                let mut o = crate::value::Object::new_ordinary();
                o.set_own_internal("__primitive__".into(), Value::Number(*n));
                if let Some(p) = self.number_prototype { o.proto = Some(p); }
                Ok(Value::Object(self.alloc_object(o)))
            }
            Value::String(s) => {
                let mut o = crate::value::Object::new_ordinary();
                o.set_own_internal("__primitive__".into(), Value::String(s.clone()));
                let n = s.chars().count();
                for (i, c) in s.chars().enumerate() {
                    o.set_own(i.to_string(), Value::String(std::rc::Rc::new(c.to_string())));
                }
                // Length on String exotic objects is non-enumerable per §22.1.4.
                o.set_own_internal("length".into(), Value::Number(n as f64));
                if let Some(p) = self.string_prototype { o.proto = Some(p); }
                Ok(Value::Object(self.alloc_object(o)))
            }
            Value::BigInt(_) | Value::Symbol(_) => Ok(v.clone()),
        }
    }

    /// LengthOfArrayLike per ECMA §7.3.20 — Get(O, "length") + ToLength.
    /// Returns usize; throws if the receiver is not an Object.
    pub fn length_of_array_like(&mut self, v: &Value) -> Result<usize, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError(
                "LengthOfArrayLike: receiver must be an Object".into())),
        };
        Ok(self.array_length(id))
    }

    /// HasProperty per ECMA §7.3.10 — walks the proto chain.
    pub fn has_property_via(&self, v: &Value, key: &str) -> bool {
        match v {
            Value::Object(id) => self.has_property(*id, key),
            _ => false,
        }
    }

    /// Get per ECMA §7.3.2 — dispatches accessor getters.
    pub fn read_property_via(&mut self, v: &Value, key: &str) -> Result<Value, RuntimeError> {
        match v {
            Value::Object(id) => self.read_property(*id, key),
            _ => Ok(Value::Undefined),
        }
    }

    /// CreateDataPropertyOrThrow per ECMA §7.3.6.
    pub fn create_data_property_or_throw(
        &mut self,
        v: &Value,
        key: &str,
        val: Value,
    ) -> Result<(), RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError(
                "CreateDataPropertyOrThrow: receiver must be an Object".into())),
        };
        self.object_set(id, key.to_string(), val);
        // On Array receivers, ensure length covers the new index per
        // §10.4.2.4 ArraySetLength.
        self.bump_array_length_if_needed(id, key);
        Ok(())
    }

    /// String.prototype.repeat(count) per ECMA §22.1.3.21.
    pub fn string_proto_repeat_via(&mut self, this: &Value, count: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let n_n = match count {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        if n_n.is_nan() || n_n < 0.0 || n_n == f64::INFINITY {
            return Err(RuntimeError::RangeError("Invalid count value".into()));
        }
        Ok(Value::String(std::rc::Rc::new(s.repeat(n_n as usize))))
    }

    /// String.prototype.padStart(targetLength, padString) per ECMA §22.1.3.17.
    pub fn string_proto_pad_start_via(&mut self, this: &Value, target: &Value, pad: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let target_n = match target { Value::Undefined => 0.0, v => self.coerce_to_number(v)? };
        let target_len = if target_n.is_nan() || target_n <= 0.0 { 0 } else { target_n as usize };
        let pad_s = match pad {
            Value::Undefined => " ".to_string(),
            v => self.to_string_strict(v)?,
        };
        if s.chars().count() >= target_len || pad_s.is_empty() {
            return Ok(Value::String(std::rc::Rc::new(s)));
        }
        let need = target_len - s.chars().count();
        let mut prefix = String::new();
        while prefix.chars().count() < need { prefix.push_str(&pad_s); }
        let prefix: String = prefix.chars().take(need).collect();
        Ok(Value::String(std::rc::Rc::new(prefix + &s)))
    }

    /// String.prototype.padEnd(targetLength, padString) per ECMA §22.1.3.16.
    pub fn string_proto_pad_end_via(&mut self, this: &Value, target: &Value, pad: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let target_n = match target { Value::Undefined => 0.0, v => self.coerce_to_number(v)? };
        let target_len = if target_n.is_nan() || target_n <= 0.0 { 0 } else { target_n as usize };
        let pad_s = match pad {
            Value::Undefined => " ".to_string(),
            v => self.to_string_strict(v)?,
        };
        if s.chars().count() >= target_len || pad_s.is_empty() {
            return Ok(Value::String(std::rc::Rc::new(s)));
        }
        let need = target_len - s.chars().count();
        let mut suffix = String::new();
        while suffix.chars().count() < need { suffix.push_str(&pad_s); }
        let suffix: String = suffix.chars().take(need).collect();
        Ok(Value::String(std::rc::Rc::new(s + &suffix)))
    }

    /// Number.prototype.toString(radix) per ECMA §21.1.3.6.
    pub fn number_proto_to_string_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        let unwrapped = self.unwrap_primitive(&this);
        let n = match unwrapped {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toString: this is not a Number".into())),
        };
        let radix = match args.first().cloned() {
            None | Some(Value::Undefined) => 10,
            Some(v) => {
                let n = self.coerce_to_number(&v)? as i32;
                if n < 2 || n > 36 {
                    return Err(RuntimeError::RangeError(
                        "toString() radix must be between 2 and 36".into()));
                }
                n
            }
        };
        if radix == 10 {
            Ok(Value::String(std::rc::Rc::new(crate::abstract_ops::number_to_string(n))))
        } else if (2..=36).contains(&radix) && n.is_finite() && n.fract() == 0.0 {
            let mut x = n as i64;
            if x == 0 { return Ok(Value::String(std::rc::Rc::new("0".into()))); }
            let neg = x < 0;
            if neg { x = -x; }
            let mut digits = Vec::new();
            while x > 0 {
                let d = (x % radix as i64) as u32;
                let c = if d < 10 { (b'0' + d as u8) as char } else { (b'a' + (d - 10) as u8) as char };
                digits.push(c);
                x /= radix as i64;
            }
            if neg { digits.push('-'); }
            digits.reverse();
            Ok(Value::String(std::rc::Rc::new(digits.into_iter().collect())))
        } else {
            Ok(Value::String(std::rc::Rc::new(crate::abstract_ops::number_to_string(n))))
        }
    }

    /// Number.prototype.toLocaleString() per ECMA §21.1.3.4 (v1: same as toString).
    pub fn number_proto_to_locale_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        let n = match self.unwrap_primitive(&this) {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toLocaleString: this is not a Number".into())),
        };
        Ok(Value::String(std::rc::Rc::new(crate::abstract_ops::number_to_string(n))))
    }

    /// String.fromCharCode(...codeUnits) per ECMA §22.1.2.1.
    pub fn string_from_char_code_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut s = String::new();
        for v in args {
            let n = self.coerce_to_number(v)? as u32 & 0xFFFF;
            if let Some(c) = char::from_u32(n) { s.push(c); }
        }
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    /// String.fromCodePoint(...codePoints) per ECMA §22.1.2.2.
    pub fn string_from_code_point_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut s = String::new();
        for v in args {
            let n = self.coerce_to_number(v)?;
            if !n.is_finite() || n.fract() != 0.0 || n < 0.0 || n > 0x10FFFF as f64 {
                return Err(RuntimeError::RangeError(format!("Invalid code point {}", n)));
            }
            if let Some(c) = char::from_u32(n as u32) { s.push(c); }
        }
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    /// Date.prototype.toString() (v1: ISO-like YYYY-MM-DDT00:00:00Z).
    pub fn date_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))) };
        let ms = match self.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))) };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        Ok(Value::String(std::rc::Rc::new(format!("{:04}-{:02}-{:02}T00:00:00Z", y, mo + 1, d))))
    }

    /// Date.prototype.toJSON() per ECMA §21.4.4.37 (v1: midnight ISO).
    pub fn date_proto_to_json_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(std::rc::Rc::new("".into()))) };
        let ms = match self.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(std::rc::Rc::new("".into()))) };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        Ok(Value::String(std::rc::Rc::new(format!("{:04}-{:02}-{:02}T00:00:00.000Z", y, mo + 1, d))))
    }

    fn date_ms_field(&mut self) -> Option<f64> {
        let id = match self.current_this() { Value::Object(id) => id, _ => return None };
        match self.object_get(id, "__date_ms") { Value::Number(n) => Some(n), _ => None }
    }

    /// Date.prototype.getFullYear() per ECMA §21.4.4.4.
    pub fn date_proto_get_full_year_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.date_ms_field().map(|ms| crate::intrinsics::date_components(ms).0 as f64).unwrap_or(f64::NAN)))
    }
    pub fn date_proto_get_month_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.date_ms_field().map(|ms| crate::intrinsics::date_components(ms).1 as f64).unwrap_or(f64::NAN)))
    }
    pub fn date_proto_get_date_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.date_ms_field().map(|ms| crate::intrinsics::date_components(ms).2 as f64).unwrap_or(f64::NAN)))
    }
    pub fn date_proto_get_day_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.date_ms_field().map(|ms| {
            let days = (ms / 86_400_000.0).floor() as i64;
            (((days % 7) + 7 + 4) % 7) as f64
        }).unwrap_or(f64::NAN)))
    }
    pub fn date_proto_get_hours_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.date_ms_field().map(|ms| ((ms / 3_600_000.0).floor() as i64 % 24) as f64).unwrap_or(f64::NAN)))
    }
    pub fn date_proto_get_minutes_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.date_ms_field().map(|ms| ((ms / 60_000.0).floor() as i64 % 60) as f64).unwrap_or(f64::NAN)))
    }
    pub fn date_proto_get_seconds_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.date_ms_field().map(|ms| ((ms / 1000.0).floor() as i64 % 60) as f64).unwrap_or(f64::NAN)))
    }
    pub fn date_proto_get_milliseconds_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.date_ms_field().map(|ms| (ms as i64 % 1000) as f64).unwrap_or(f64::NAN)))
    }

    /// Date.prototype.getTime() per ECMA §21.4.4.10.
    pub fn date_proto_get_time_via(&mut self) -> Result<Value, RuntimeError> {
        let this = match self.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(0.0)) };
        Ok(self.object_get(this, "__date_ms"))
    }

    /// Date.prototype.valueOf() per ECMA §21.4.4.44.
    pub fn date_proto_value_of_via(&mut self) -> Result<Value, RuntimeError> {
        let this = match self.current_this() { Value::Object(id) => id, _ => return Ok(Value::Number(0.0)) };
        Ok(self.object_get(this, "__date_ms"))
    }

    /// Date.prototype.toISOString() per ECMA §21.4.4.36.
    pub fn date_proto_to_iso_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(std::rc::Rc::new("".into()))) };
        let ms = match self.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(std::rc::Rc::new("".into()))) };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        let h = (ms / 3_600_000.0).floor() as i64 % 24;
        let mi = (ms / 60_000.0).floor() as i64 % 60;
        let se = (ms / 1000.0).floor() as i64 % 60;
        let mss = ms as i64 % 1000;
        Ok(Value::String(std::rc::Rc::new(format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            y, mo + 1, d, h, mi, se, mss))))
    }

    /// Date.prototype.toDateString() (v1: ISO-like YYYY-MM-DD).
    pub fn date_proto_to_date_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(std::rc::Rc::new(String::new()))) };
        let ms = match self.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))) };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        Ok(Value::String(std::rc::Rc::new(format!("{:04}-{:02}-{:02}", y, mo + 1, d))))
    }

    /// Date.prototype.toTimeString() (v1: HH:MM:SS).
    pub fn date_proto_to_time_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(std::rc::Rc::new(String::new()))) };
        let ms = match self.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))) };
        let h = (ms / 3_600_000.0).floor() as i64 % 24;
        let mi = (ms / 60_000.0).floor() as i64 % 60;
        let se = (ms / 1000.0).floor() as i64 % 60;
        Ok(Value::String(std::rc::Rc::new(format!("{:02}:{:02}:{:02}", h, mi, se))))
    }

    /// Date.prototype.toUTCString() (v1: YYYY-MM-DD HH:MM:SS GMT).
    pub fn date_proto_to_utc_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() { Value::Object(id) => id, _ => return Ok(Value::String(std::rc::Rc::new(String::new()))) };
        let ms = match self.object_get(this_id, "__date_ms") { Value::Number(n) => n, _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))) };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        let h = (ms / 3_600_000.0).floor() as i64 % 24;
        let mi = (ms / 60_000.0).floor() as i64 % 60;
        let se = (ms / 1000.0).floor() as i64 % 60;
        Ok(Value::String(std::rc::Rc::new(format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} GMT", y, mo + 1, d, h, mi, se))))
    }

    /// Symbol.prototype.toString() per ECMA §20.4.3.3.
    pub fn symbol_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        match self.current_this() {
            Value::Symbol(s) => {
                let body = s.strip_prefix("@@sym:").unwrap_or(&s);
                let desc = body.split_once(':').map(|(_, d)| d).unwrap_or(body);
                Ok(Value::String(std::rc::Rc::new(format!("Symbol({})", desc))))
            }
            v => Ok(Value::String(std::rc::Rc::new(crate::abstract_ops::to_string(&v).as_str().to_string()))),
        }
    }

    /// BigInt.prototype.toString(radix) per ECMA §21.2.3.4.
    pub fn bigint_proto_to_string_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let b = match self.current_this() {
            Value::BigInt(b) => b,
            _ => return Err(RuntimeError::TypeError("BigInt.prototype.toString: this is not a BigInt".into())),
        };
        let radix = match args.first() {
            Some(Value::Number(n)) if (2.0..=36.0).contains(n) => *n as u32,
            Some(Value::Undefined) | None => 10,
            _ => return Err(RuntimeError::TypeError("BigInt.prototype.toString radix out of range".into())),
        };
        Ok(Value::String(std::rc::Rc::new(b.to_radix(radix))))
    }

    /// Function.prototype.toString() per ECMA §20.2.3.5 (v1: native shape for all functions).
    pub fn function_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        let s = match &this {
            Value::Object(id) => {
                let name = match &self.obj(*id).internal_kind {
                    crate::value::InternalKind::Function(f) => f.name.clone(),
                    crate::value::InternalKind::Closure(_) => "anonymous".to_string(),
                    crate::value::InternalKind::BoundFunction(_) => "bound".to_string(),
                    _ => return Err(RuntimeError::TypeError("Function.prototype.toString: not a function".into())),
                };
                format!("function {}() {{ [native code] }}", name)
            }
            _ => return Err(RuntimeError::TypeError("Function.prototype.toString: not a function".into())),
        };
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    /// Error.prototype.toString() per ECMA §20.5.3.4.
    pub fn error_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        let (name, message) = match &this {
            Value::Object(id) => {
                let n = self.object_get(*id, "name");
                let m = self.object_get(*id, "message");
                (crate::abstract_ops::to_string(&n).as_str().to_string(),
                 crate::abstract_ops::to_string(&m).as_str().to_string())
            }
            _ => ("Error".into(), "".into()),
        };
        let out = if message.is_empty() { name } else { format!("{}: {}", name, message) };
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// Math.imul(a, b) per ECMA §21.3.2.19.
    pub fn math_imul_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let a = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64 as i32;
        let b = args.get(1).map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64 as i32;
        Ok(Value::Number(a.wrapping_mul(b) as f64))
    }

    /// Math.fround(x) per ECMA §21.3.2.16.
    pub fn math_fround_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let n = args.first().map(crate::abstract_ops::to_number).unwrap_or(f64::NAN);
        Ok(Value::Number(n as f32 as f64))
    }

    /// Math.clz32(x) per ECMA §21.3.2.11.
    pub fn math_clz32_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let n = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64 as u32;
        Ok(Value::Number(n.leading_zeros() as f64))
    }

    /// Array.isArray(arg) per ECMA §23.1.2.2.
    pub fn array_is_array_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        Ok(Value::Boolean(matches!(args.first(),
            Some(Value::Object(id)) if matches!(self.obj(*id).internal_kind, crate::value::InternalKind::Array))))
    }

    /// Array.of(...items) per ECMA §23.1.2.3.
    pub fn array_of_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let out = self.alloc_object(crate::value::Object::new_array());
        for (i, v) in args.iter().enumerate() {
            self.object_set(out, i.to_string(), v.clone());
        }
        self.object_set(out, "length".into(), Value::Number(args.len() as f64));
        Ok(Value::Object(out))
    }

    /// Object.prototype.toString() per ECMA §20.1.3.6 (with @@toStringTag).
    pub fn object_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        let s = match this {
            Value::Undefined => "[object Undefined]".to_string(),
            Value::Null => "[object Null]".to_string(),
            Value::Boolean(_) => "[object Boolean]".to_string(),
            Value::Number(_) => "[object Number]".to_string(),
            Value::String(_) => "[object String]".to_string(),
            Value::BigInt(_) => "[object BigInt]".to_string(),
            Value::Symbol(_) => "[object Symbol]".to_string(),
            Value::Object(id) => {
                let tag_val = self.object_get(id, "@@toStringTag");
                let tag = if let Value::String(s) = &tag_val {
                    s.as_str().to_string()
                } else {
                    match &self.obj(id).internal_kind {
                        crate::value::InternalKind::Array => "Array",
                        crate::value::InternalKind::Function(_)
                        | crate::value::InternalKind::Closure(_)
                        | crate::value::InternalKind::BoundFunction(_) => "Function",
                        crate::value::InternalKind::Promise(_) => "Promise",
                        crate::value::InternalKind::Error => "Error",
                        crate::value::InternalKind::RegExp(_) => "RegExp",
                        _ => "Object",
                    }.to_string()
                };
                format!("[object {}]", tag)
            }
        };
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    /// Object.prototype.hasOwnProperty(key) per ECMA §20.1.3.2.
    pub fn object_proto_has_own_property_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let key = crate::abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined)).as_str().to_string();
        let owns = match self.current_this() {
            Value::Object(id) => self.obj(id).properties.contains_key(&key),
            _ => false,
        };
        Ok(Value::Boolean(owns))
    }

    /// Object.prototype.valueOf() per ECMA §20.1.3.7.
    pub fn object_proto_value_of_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(self.current_this())
    }

    /// Object.prototype.propertyIsEnumerable(key) per ECMA §20.1.3.4.
    pub fn object_proto_property_is_enumerable_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let key = crate::abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
            .as_str().to_string();
        let owns = match self.current_this() {
            Value::Object(id) => self.obj(id).properties.contains_key(&key),
            _ => false,
        };
        Ok(Value::Boolean(owns))
    }

    /// Object.prototype.isPrototypeOf(target) per ECMA §20.1.3.3.
    pub fn object_proto_is_prototype_of_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let target = match args.first() {
            Some(Value::Object(id)) => *id,
            _ => return Ok(Value::Boolean(false)),
        };
        let this_id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::Boolean(false)),
        };
        let mut cur = self.obj(target).proto;
        while let Some(c) = cur {
            if c == this_id { return Ok(Value::Boolean(true)); }
            cur = self.obj(c).proto;
        }
        Ok(Value::Boolean(false))
    }

    /// Object.prototype.toLocaleString() per ECMA §20.1.3.5 — invoke this.toString().
    pub fn object_proto_to_locale_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        if let Value::Object(id) = &this {
            let to_str = self.object_get(*id, "toString");
            if matches!(to_str, Value::Object(_)) {
                return self.call_function(to_str, this.clone(), Vec::new());
            }
        }
        Ok(Value::String(std::rc::Rc::new(crate::abstract_ops::to_string(&this).as_str().to_string())))
    }

    /// Array.prototype.sort(comparefn) per ECMA §23.1.3.29.
    pub fn array_proto_sort_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let cmp_arg = args.first().cloned();
        let comparator = match cmp_arg {
            None | Some(Value::Undefined) => None,
            Some(v) => {
                if !self.is_callable(&v) {
                    return Err(RuntimeError::TypeError(
                        "Array.prototype.sort: comparefn must be callable".into()));
                }
                Some(v)
            }
        };
        let len = self.array_length(id);
        let mut items: Vec<Value> = (0..len).map(|i| self.object_get(id, &i.to_string())).collect();
        let mut err: Option<RuntimeError> = None;
        match comparator {
            None => {
                items.sort_by(|a, b| {
                    let sa = crate::abstract_ops::to_string(a);
                    let sb = crate::abstract_ops::to_string(b);
                    sa.as_str().cmp(sb.as_str())
                });
            }
            Some(cb) => {
                items.sort_by(|a, b| {
                    if err.is_some() { return std::cmp::Ordering::Equal; }
                    match self.call_function(cb.clone(), Value::Undefined, vec![a.clone(), b.clone()]) {
                        Ok(v) => {
                            let n = crate::abstract_ops::to_number(&v);
                            if n.is_nan() { std::cmp::Ordering::Equal }
                            else if n < 0.0 { std::cmp::Ordering::Less }
                            else if n > 0.0 { std::cmp::Ordering::Greater }
                            else { std::cmp::Ordering::Equal }
                        }
                        Err(e) => { err = Some(e); std::cmp::Ordering::Equal }
                    }
                });
            }
        }
        if let Some(e) = err { return Err(e); }
        for (i, v) in items.into_iter().enumerate() {
            self.object_set(id, i.to_string(), v);
        }
        self.object_set(id, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(id))
    }

    /// Array.prototype.entries() per ECMA §23.1.3.4.
    pub fn array_proto_entries_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id);
        let out = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            let v = self.object_get(id, &i.to_string());
            let pair = self.alloc_object(crate::value::Object::new_array());
            self.object_set(pair, "0".into(), Value::Number(i as f64));
            self.object_set(pair, "1".into(), v);
            self.object_set(pair, "length".into(), Value::Number(2.0));
            self.object_set(out, i.to_string(), Value::Object(pair));
        }
        self.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.keys() per ECMA §23.1.3.17.
    pub fn array_proto_keys_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id);
        let out = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            self.object_set(out, i.to_string(), Value::Number(i as f64));
        }
        self.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.values() per ECMA §23.1.3.38.
    pub fn array_proto_values_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id);
        let out = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            let v = self.object_get(id, &i.to_string());
            self.object_set(out, i.to_string(), v);
        }
        self.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.toReversed() per ECMA §23.1.3.33.
    pub fn array_proto_to_reversed_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id);
        let out = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            let v = self.object_get(id, &(len - 1 - i).to_string());
            self.object_set(out, i.to_string(), v);
        }
        self.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.toSorted(comparefn) per ECMA §23.1.3.34.
    pub fn array_proto_to_sorted_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id);
        let out = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            self.object_set(out, i.to_string(), self.object_get(id, &i.to_string()));
        }
        self.object_set(out, "length".into(), Value::Number(len as f64));
        let comparator = args.first().cloned().filter(|v| !matches!(v, Value::Undefined));
        let mut items: Vec<Value> = (0..len).map(|i| self.object_get(out, &i.to_string())).collect();
        let mut err: Option<RuntimeError> = None;
        match comparator {
            Some(cmp) => {
                items.sort_by(|a, b| {
                    if err.is_some() { return std::cmp::Ordering::Equal; }
                    match self.call_function(cmp.clone(), Value::Undefined, vec![a.clone(), b.clone()]) {
                        Ok(Value::Number(n)) => {
                            if n < 0.0 { std::cmp::Ordering::Less }
                            else if n > 0.0 { std::cmp::Ordering::Greater }
                            else { std::cmp::Ordering::Equal }
                        }
                        Ok(_) => std::cmp::Ordering::Equal,
                        Err(e) => { err = Some(e); std::cmp::Ordering::Equal }
                    }
                });
            }
            None => {
                items.sort_by(|a, b| {
                    let sa = crate::abstract_ops::to_string(a);
                    let sb = crate::abstract_ops::to_string(b);
                    sa.as_str().cmp(sb.as_str())
                });
            }
        }
        if let Some(e) = err { return Err(e); }
        for (i, v) in items.into_iter().enumerate() {
            self.object_set(out, i.to_string(), v);
        }
        Ok(Value::Object(out))
    }

    /// Array.prototype.toSpliced(start, deleteCount, ...items) per ECMA §23.1.3.35.
    pub fn array_proto_to_spliced_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id) as i64;
        let clamp = |i: i64, l: i64| if i < 0 { (l + i).max(0) } else { i.min(l) };
        let start = clamp(args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64, len);
        let del = match args.get(1) {
            Some(v) => {
                let n = crate::abstract_ops::to_number(v) as i64;
                n.max(0).min(len - start)
            }
            None => len - start,
        };
        let inserts: Vec<Value> = args.iter().skip(2).cloned().collect();
        let new_len = len - del + inserts.len() as i64;
        let out = self.alloc_object(crate::value::Object::new_array());
        let mut k = 0i64;
        for i in 0..start {
            self.object_set(out, k.to_string(), self.object_get(id, &i.to_string()));
            k += 1;
        }
        for v in inserts {
            self.object_set(out, k.to_string(), v);
            k += 1;
        }
        for i in (start + del)..len {
            self.object_set(out, k.to_string(), self.object_get(id, &i.to_string()));
            k += 1;
        }
        self.object_set(out, "length".into(), Value::Number(new_len as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.with(index, value) per ECMA §23.1.3.39.
    pub fn array_proto_with_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id) as i64;
        let idx = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
        let actual = if idx < 0 { len + idx } else { idx };
        if actual < 0 || actual >= len {
            return Err(RuntimeError::RangeError(format!("with: index {} out of bounds", idx)));
        }
        let val = args.get(1).cloned().unwrap_or(Value::Undefined);
        let out = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            let v = if i == actual { val.clone() } else { self.object_get(id, &i.to_string()) };
            self.object_set(out, i.to_string(), v);
        }
        self.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.toLocaleString() per ECMA §23.1.3.30 (v1: comma-join).
    pub fn array_proto_to_locale_string_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id);
        let mut out = String::new();
        for i in 0..len {
            if i > 0 { out.push(','); }
            let v = self.object_get(id, &i.to_string());
            out.push_str(crate::abstract_ops::to_string(&v).as_str());
        }
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// Array.prototype.toString() per ECMA §23.1.3.36 — delegate to this.join() or fall back.
    pub fn array_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        if let Value::Object(id) = this {
            let join = self.object_get(id, "join");
            if matches!(join, Value::Object(_)) {
                return self.call_function(join, Value::Object(id), Vec::new());
            }
        }
        Ok(Value::String(std::rc::Rc::new("[object Array]".into())))
    }

    /// Array.prototype.slice(start, end) per ECMA §23.1.3.28.
    pub fn array_proto_slice_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id) as i64;
        let start_arg = match args.first().cloned() {
            Some(Value::Undefined) | None => 0,
            Some(v) => self.coerce_to_number(&v)? as i64,
        };
        let end_arg = match args.get(1).cloned() {
            Some(Value::Undefined) | None => len,
            Some(v) => self.coerce_to_number(&v)? as i64,
        };
        let clamp = |i: i64, l: i64| if i < 0 { (l + i).max(0) } else { i.min(l) };
        let start = clamp(start_arg, len);
        let end = clamp(end_arg, len);
        let out = self.alloc_object(crate::value::Object::new_array());
        let mut j: i64 = 0;
        let mut i = start;
        while i < end {
            let v = self.object_get(id, &i.to_string());
            self.object_set(out, j.to_string(), v);
            j += 1;
            i += 1;
        }
        self.object_set(out, "length".into(), Value::Number(j as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.splice(start, deleteCount, ...items) per ECMA §23.1.3.31.
    pub fn array_proto_splice_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id) as i64;
        let start_arg = match args.first().cloned() {
            Some(Value::Undefined) | None => 0,
            Some(v) => self.coerce_to_number(&v)? as i64,
        };
        let clamp = |i: i64, l: i64| if i < 0 { (l + i).max(0) } else { i.min(l) };
        let start = clamp(start_arg, len);
        let delete_count = match args.get(1).cloned() {
            Some(Value::Undefined) | None => len - start,
            Some(v) => (self.coerce_to_number(&v)? as i64).max(0).min(len - start),
        };
        let items: Vec<Value> = args.iter().skip(2).cloned().collect();
        let removed = self.alloc_object(crate::value::Object::new_array());
        for j in 0..delete_count {
            let v = self.object_get(id, &(start + j).to_string());
            self.object_set(removed, j.to_string(), v);
        }
        self.object_set(removed, "length".into(), Value::Number(delete_count as f64));
        let new_len = len - delete_count + items.len() as i64;
        let shift = items.len() as i64 - delete_count;
        if shift > 0 {
            let mut i = len - 1;
            while i >= start + delete_count {
                let v = self.object_get(id, &i.to_string());
                self.object_set(id, (i + shift).to_string(), v);
                i -= 1;
            }
        } else if shift < 0 {
            let mut i = start + delete_count;
            while i < len {
                let v = self.object_get(id, &i.to_string());
                self.object_set(id, (i + shift).to_string(), v);
                i += 1;
            }
            for i in new_len..len {
                self.obj_mut(id).properties.shift_remove(&i.to_string());
            }
        }
        for (k, v) in items.into_iter().enumerate() {
            self.object_set(id, (start + k as i64).to_string(), v);
        }
        self.object_set(id, "length".into(), Value::Number(new_len as f64));
        Ok(Value::Object(removed))
    }

    /// Array.prototype.concat(...args) per ECMA §23.1.3.2.
    pub fn array_proto_concat_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        self.require_object_coercible(&this)?;
        let out = self.alloc_object(crate::value::Object::new_array());
        let mut j = 0usize;
        let mut items: Vec<Value> = Vec::with_capacity(args.len() + 1);
        items.push(this);
        items.extend(args.iter().cloned());
        for e in items {
            let spreadable = match &e {
                Value::Object(eid) => {
                    let flag = self.read_property(*eid, "@@isConcatSpreadable")?;
                    match flag {
                        Value::Undefined => matches!(self.obj(*eid).internal_kind, crate::value::InternalKind::Array),
                        v => crate::abstract_ops::to_boolean(&v),
                    }
                }
                _ => false,
            };
            if spreadable {
                if let Value::Object(eid) = e {
                    let el = self.array_length(eid);
                    for i in 0..el {
                        let key = i.to_string();
                        if self.has_property(eid, &key) {
                            let v = self.read_property(eid, &key)?;
                            self.object_set(out, j.to_string(), v);
                        }
                        j += 1;
                    }
                }
            } else {
                self.object_set(out, j.to_string(), e);
                j += 1;
            }
        }
        self.object_set(out, "length".into(), Value::Number(j as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.join(separator) per ECMA §23.1.3.15.
    pub fn array_proto_join_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let sep = match args.first() {
            Some(Value::Undefined) | None => ",".to_string(),
            Some(v) => crate::abstract_ops::to_string(v).as_str().to_string(),
        };
        let len = self.array_length(id);
        let mut parts = Vec::with_capacity(len);
        for i in 0..len {
            let v = self.object_get(id, &i.to_string());
            let s = match v {
                Value::Undefined | Value::Null => String::new(),
                other => crate::abstract_ops::to_string(&other).as_str().to_string(),
            };
            parts.push(s);
        }
        Ok(Value::String(std::rc::Rc::new(parts.join(&sep))))
    }

    /// Array.prototype.at(index) per ECMA §23.1.3.1.
    pub fn array_proto_at_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id) as i64;
        let i = args.first().map(crate::abstract_ops::to_number).unwrap_or(0.0) as i64;
        let idx = if i < 0 { len + i } else { i };
        if idx < 0 || idx >= len { return Ok(Value::Undefined); }
        Ok(self.object_get(id, &idx.to_string()))
    }

    /// Array.prototype.fill(value, start, end) per ECMA §23.1.3.7.
    pub fn array_proto_fill_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let value = args.first().cloned().unwrap_or(Value::Undefined);
        let len = self.array_length(id);
        let start = match args.get(1).cloned() {
            Some(Value::Undefined) | None => 0,
            Some(v) => {
                let n = self.coerce_to_number(&v)? as i64;
                if n < 0 { (len as i64 + n).max(0) as usize } else { (n as usize).min(len) }
            }
        };
        let end = match args.get(2).cloned() {
            Some(Value::Undefined) | None => len,
            Some(v) => {
                let n = self.coerce_to_number(&v)? as i64;
                if n < 0 { (len as i64 + n).max(0) as usize } else { (n as usize).min(len) }
            }
        };
        for i in start..end {
            self.object_set(id, i.to_string(), value.clone());
        }
        Ok(Value::Object(id))
    }

    /// Array.prototype.lastIndexOf(searchElement, fromIndex) per ECMA §23.1.3.18.
    pub fn array_proto_last_index_of_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let needle = args.first().cloned().unwrap_or(Value::Undefined);
        let len = self.array_length(id) as i64;
        let from = match args.get(1) {
            Some(v) if !matches!(v, Value::Undefined) => {
                let n = crate::abstract_ops::to_number(v) as i64;
                if n < 0 { (len + n).max(-1) } else { (n.min(len - 1)).max(-1) }
            }
            _ => (len - 1).max(-1),
        };
        let mut i = from;
        while i >= 0 {
            let key = i.to_string();
            if self.has_property(id, &key) {
                let v = self.read_property(id, &key)?;
                if crate::abstract_ops::is_strictly_equal(&v, &needle) {
                    return Ok(Value::Number(i as f64));
                }
            }
            i -= 1;
        }
        Ok(Value::Number(-1.0))
    }

    /// Array.prototype.reduceRight(callbackfn, initialValue) per ECMA §23.1.3.25.
    pub fn array_proto_reduce_right_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("reduceRight: callback required".into()))?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.reduceRight: callback is not callable".into()));
        }
        let len = self.array_length(id);
        let has_init = args.len() >= 2;
        let mut i: i64 = (len as i64) - 1;
        let mut acc = if has_init { args[1].clone() } else {
            let mut seed: Option<(i64, Value)> = None;
            while i >= 0 {
                let key = i.to_string();
                if self.has_property(id, &key) {
                    let v = self.read_property(id, &key)?;
                    seed = Some((i, v)); break;
                }
                i -= 1;
            }
            match seed {
                Some((start, v)) => { i = start - 1; v }
                None => return Err(RuntimeError::TypeError(
                    "reduce of empty array with no initial value".into())),
            }
        };
        while i >= 0 {
            let key = i.to_string();
            if self.has_property(id, &key) {
                let v = self.read_property(id, &key)?;
                acc = self.call_function(cb.clone(), Value::Undefined,
                    vec![acc, v, Value::Number(i as f64), Value::Object(id)])?;
            }
            i -= 1;
        }
        Ok(acc)
    }

    /// Array.prototype.copyWithin(target, start, end) per ECMA §23.1.3.4.
    pub fn array_proto_copy_within_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id) as i64;
        let clamp = |i: i64, l: i64| if i < 0 { (l + i).max(0) } else { i.min(l) };
        let arg_n = |slf: &mut Runtime, i: usize, default: i64| -> Result<i64, RuntimeError> {
            match args.get(i).cloned() {
                Some(Value::Undefined) | None => Ok(default),
                Some(v) => Ok(slf.coerce_to_number(&v)? as i64),
            }
        };
        let to = clamp(arg_n(self, 0, 0)?, len);
        let from = clamp(arg_n(self, 1, 0)?, len);
        let end = clamp(arg_n(self, 2, len)?, len);
        let count = (end - from).min(len - to).max(0);
        let buf: Vec<Value> = (0..count).map(|i|
            self.object_get(id, &(from + i).to_string())).collect();
        for (i, v) in buf.into_iter().enumerate() {
            self.object_set(id, (to + i as i64).to_string(), v);
        }
        Ok(Value::Object(id))
    }

    /// Array.prototype.flat(depth) per ECMA §23.1.3.10.
    pub fn array_proto_flat_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let depth = args.first().map(crate::abstract_ops::to_number).unwrap_or(1.0) as i64;
        let out = self.alloc_object(crate::value::Object::new_array());
        fn flat_into(rt: &mut Runtime, src: crate::value::ObjectRef, out: crate::value::ObjectRef, mut out_idx: usize, depth: i64) -> usize {
            let len = rt.array_length(src);
            for i in 0..len {
                let v = rt.object_get(src, &i.to_string());
                if depth > 0 {
                    if let Value::Object(nid) = &v {
                        if matches!(rt.obj(*nid).internal_kind, crate::value::InternalKind::Array) {
                            out_idx = flat_into(rt, *nid, out, out_idx, depth - 1);
                            continue;
                        }
                    }
                }
                rt.object_set(out, out_idx.to_string(), v);
                out_idx += 1;
            }
            out_idx
        }
        let final_len = flat_into(self, id, out, 0, depth);
        self.object_set(out, "length".into(), Value::Number(final_len as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.flatMap(callback, thisArg) per ECMA §23.1.3.11.
    pub fn array_proto_flat_map_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("Array.prototype.flatMap: callback required".into()))?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.flatMap: callback is not callable".into()));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        let len = self.array_length(id);
        let out = self.alloc_object(crate::value::Object::new_array());
        let mut out_idx = 0usize;
        for i in 0..len {
            let v = self.object_get(id, &i.to_string());
            let mapped = self.call_function(cb.clone(), this_arg.clone(),
                vec![v, Value::Number(i as f64), Value::Object(id)])?;
            if let Value::Object(nid) = &mapped {
                if matches!(self.obj(*nid).internal_kind, crate::value::InternalKind::Array) {
                    let n = self.array_length(*nid);
                    for j in 0..n {
                        let nv = self.object_get(*nid, &j.to_string());
                        self.object_set(out, out_idx.to_string(), nv);
                        out_idx += 1;
                    }
                    continue;
                }
            }
            self.object_set(out, out_idx.to_string(), mapped);
            out_idx += 1;
        }
        self.object_set(out, "length".into(), Value::Number(out_idx as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.push(...items) per ECMA §23.1.3.20.
    pub fn array_proto_push_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let mut len = self.array_length(id);
        for a in args {
            self.object_set(id, len.to_string(), a.clone());
            len += 1;
        }
        self.object_set(id, "length".into(), Value::Number(len as f64));
        Ok(Value::Number(len as f64))
    }

    /// Array.prototype.pop() per ECMA §23.1.3.19.
    pub fn array_proto_pop_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id);
        if len == 0 { return Ok(Value::Undefined); }
        let last_key = (len - 1).to_string();
        let v = self.object_get(id, &last_key);
        self.obj_mut(id).properties.shift_remove(&last_key);
        self.object_set(id, "length".into(), Value::Number((len - 1) as f64));
        Ok(v)
    }

    /// Array.prototype.shift() per ECMA §23.1.3.26.
    pub fn array_proto_shift_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id);
        if len == 0 { return Ok(Value::Undefined); }
        let first = self.object_get(id, "0");
        for i in 1..len {
            let v = self.object_get(id, &i.to_string());
            self.object_set(id, (i - 1).to_string(), v);
        }
        self.obj_mut(id).properties.shift_remove(&(len - 1).to_string());
        self.object_set(id, "length".into(), Value::Number((len - 1) as f64));
        Ok(first)
    }

    /// Array.prototype.unshift(...items) per ECMA §23.1.3.32.
    pub fn array_proto_unshift_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let n = args.len();
        let len = self.array_length(id);
        for i in (0..len).rev() {
            let v = self.object_get(id, &i.to_string());
            self.object_set(id, (i + n).to_string(), v);
        }
        for (i, a) in args.iter().enumerate() {
            self.object_set(id, i.to_string(), a.clone());
        }
        let new_len = len + n;
        self.object_set(id, "length".into(), Value::Number(new_len as f64));
        Ok(Value::Number(new_len as f64))
    }

    /// Array.prototype.reverse() per ECMA §23.1.3.21.
    pub fn array_proto_reverse_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.array_length(id) as i64;
        let mid = len / 2;
        for i in 0..mid {
            let j = len - 1 - i;
            let a = self.object_get(id, &i.to_string());
            let b = self.object_get(id, &j.to_string());
            self.object_set(id, i.to_string(), b);
            self.object_set(id, j.to_string(), a);
        }
        Ok(Value::Object(id))
    }

    /// Array.prototype.indexOf(searchElement, fromIndex) per ECMA §23.1.3.16.
    pub fn array_proto_index_of_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let needle = args.first().cloned().unwrap_or(Value::Undefined);
        let len = self.array_length(id) as i64;
        let from = match args.get(1) {
            Some(v) if !matches!(v, Value::Undefined) => {
                let n = crate::abstract_ops::to_number(v) as i64;
                if n < 0 { (len + n).max(0) } else { n.min(len) }
            }
            _ => 0,
        };
        let mut i = from;
        while i < len {
            let key = i.to_string();
            if self.has_property(id, &key) {
                let v = self.read_property(id, &key)?;
                if crate::abstract_ops::is_strictly_equal(&v, &needle) {
                    return Ok(Value::Number(i as f64));
                }
            }
            i += 1;
        }
        Ok(Value::Number(-1.0))
    }

    /// Array.prototype.includes(searchElement, fromIndex) per ECMA §23.1.3.14.
    pub fn array_proto_includes_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let needle = args.first().cloned().unwrap_or(Value::Undefined);
        let len = self.array_length(id);
        for i in 0..len {
            let key = i.to_string();
            let v = if self.has_property(id, &key) {
                self.read_property(id, &key)?
            } else {
                Value::Undefined
            };
            let eq = match (&v, &needle) {
                (Value::Number(a), Value::Number(b)) if a.is_nan() && b.is_nan() => true,
                _ => crate::abstract_ops::is_strictly_equal(&v, &needle),
            };
            if eq { return Ok(Value::Boolean(true)); }
        }
        Ok(Value::Boolean(false))
    }

    /// Array.prototype.findLast(predicate, thisArg) per ECMA §23.1.3.10.
    pub fn array_proto_find_last_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("findLast: callback required".into()))?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.findLast: callback is not callable".into()));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        let len = self.array_length(id);
        for i in (0..len).rev() {
            let v = self.object_get(id, &i.to_string());
            let r = self.call_function(cb.clone(), this_arg.clone(),
                vec![v.clone(), Value::Number(i as f64), Value::Object(id)])?;
            if crate::abstract_ops::to_boolean(&r) { return Ok(v); }
        }
        Ok(Value::Undefined)
    }

    /// Array.prototype.findLastIndex(predicate, thisArg) per ECMA §23.1.3.11.
    pub fn array_proto_find_last_index_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("findLastIndex: callback required".into()))?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.findLastIndex: callback is not callable".into()));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        let len = self.array_length(id);
        for i in (0..len).rev() {
            let v = self.object_get(id, &i.to_string());
            let r = self.call_function(cb.clone(), this_arg.clone(),
                vec![v, Value::Number(i as f64), Value::Object(id)])?;
            if crate::abstract_ops::to_boolean(&r) { return Ok(Value::Number(i as f64)); }
        }
        Ok(Value::Number(-1.0))
    }

    /// Array.prototype.reduce(callbackfn, initialValue) per ECMA §23.1.3.24.
    pub fn array_proto_reduce_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("Array.prototype.reduce: callback required".into()))?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.reduce: callback is not callable".into()));
        }
        let len = self.array_length(id);
        let has_init = args.len() >= 2;
        let mut i = 0usize;
        let mut acc = if has_init {
            args[1].clone()
        } else {
            let mut seed: Option<(usize, Value)> = None;
            while i < len {
                let key = i.to_string();
                if self.has_property(id, &key) {
                    let v = self.read_property(id, &key)?;
                    seed = Some((i, v));
                    break;
                }
                i += 1;
            }
            match seed {
                Some((start, v)) => { i = start + 1; v }
                None => return Err(RuntimeError::TypeError(
                    "reduce of empty array with no initial value".into())),
            }
        };
        while i < len {
            let key = i.to_string();
            if self.has_property(id, &key) {
                let v = self.read_property(id, &key)?;
                acc = self.call_function(cb.clone(), Value::Undefined,
                    vec![acc, v, Value::Number(i as f64), Value::Object(id)])?;
            }
            i += 1;
        }
        Ok(acc)
    }

    /// String.prototype.split(separator, limit) per ECMA §22.1.3.23.
    pub fn string_proto_split_via(&mut self, this: &Value, separator: &Value, limit: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        if let Value::Object(rx_id) = separator {
            let split_method = self.read_property(*rx_id, "@@split")?;
            if matches!(split_method, Value::Object(_)) {
                let s = self.to_string_strict(this)?;
                return self.call_function(split_method, separator.clone(),
                    vec![Value::String(std::rc::Rc::new(s)), limit.clone()]);
            }
        }
        let s = self.to_string_strict(this)?;
        let limit_n = match limit {
            Value::Undefined => u32::MAX,
            v => { let n = self.coerce_to_number(v)?; if n.is_nan() || n <= 0.0 { 0 } else { n as u32 } }
        };
        let out = self.alloc_object(crate::value::Object::new_array());
        if limit_n == 0 {
            self.object_set(out, "length".into(), Value::Number(0.0));
            return Ok(Value::Object(out));
        }
        let mut parts: Vec<String> = match separator {
            Value::Undefined => vec![s.clone()],
            _ => {
                let sep = self.to_string_strict(separator)?;
                if sep.is_empty() {
                    if s.is_empty() {
                        self.object_set(out, "length".into(), Value::Number(0.0));
                        return Ok(Value::Object(out));
                    }
                    s.chars().map(|c| c.to_string()).collect()
                } else if s.is_empty() {
                    vec![s.clone()]
                } else {
                    s.split(&sep).map(|s| s.to_string()).collect()
                }
            }
        };
        if (parts.len() as u32) > limit_n { parts.truncate(limit_n as usize); }
        for (i, p) in parts.iter().enumerate() {
            self.object_set(out, i.to_string(), Value::String(std::rc::Rc::new(p.clone())));
        }
        self.object_set(out, "length".into(), Value::Number(parts.len() as f64));
        Ok(Value::Object(out))
    }

    /// String.prototype.replace(searchValue, replaceValue) per ECMA §22.1.3.15.
    pub fn string_proto_replace_via(&mut self, this: &Value, search: &Value, replacement: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        if let Value::Object(rx_id) = search {
            let replace_method = self.read_property(*rx_id, "@@replace")?;
            if matches!(replace_method, Value::Object(_)) {
                return self.call_function(replace_method, search.clone(),
                    vec![Value::String(std::rc::Rc::new(s)), replacement.clone()]);
            }
        }
        let needle = self.to_string_strict(search)?;
        if self.is_callable(replacement) {
            return match s.find(&needle) {
                Some(byte_off) => {
                    let pos = s[..byte_off].chars().count() as f64;
                    let r = self.call_function(replacement.clone(), Value::Undefined, vec![
                        Value::String(std::rc::Rc::new(needle.clone())),
                        Value::Number(pos),
                        Value::String(std::rc::Rc::new(s.clone())),
                    ])?;
                    let repl_str = self.to_string_strict(&r)?;
                    let mut out = String::with_capacity(s.len());
                    out.push_str(&s[..byte_off]);
                    out.push_str(&repl_str);
                    out.push_str(&s[byte_off + needle.len()..]);
                    Ok(Value::String(std::rc::Rc::new(out)))
                }
                None => Ok(Value::String(std::rc::Rc::new(s))),
            };
        }
        let repl = self.to_string_strict(replacement)?;
        Ok(Value::String(std::rc::Rc::new(s.replacen(&needle, &repl, 1))))
    }

    /// String.prototype.replaceAll(searchValue, replaceValue) per ECMA §22.1.3.16.
    pub fn string_proto_replace_all_via(&mut self, this: &Value, search: &Value, replacement: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        if let Value::Object(rx_id) = search {
            let replace_method = self.read_property(*rx_id, "@@replace")?;
            if matches!(replace_method, Value::Object(_)) {
                return self.call_function(replace_method, search.clone(),
                    vec![Value::String(std::rc::Rc::new(s)), replacement.clone()]);
            }
        }
        let needle = self.to_string_strict(search)?;
        if self.is_callable(replacement) {
            let mut out = String::with_capacity(s.len());
            let mut cur = 0usize;
            if needle.is_empty() {
                return Ok(Value::String(std::rc::Rc::new(s)));
            }
            while let Some(rel) = s[cur..].find(&needle) {
                let byte_off = cur + rel;
                out.push_str(&s[cur..byte_off]);
                let pos = s[..byte_off].chars().count() as f64;
                let r = self.call_function(replacement.clone(), Value::Undefined, vec![
                    Value::String(std::rc::Rc::new(needle.clone())),
                    Value::Number(pos),
                    Value::String(std::rc::Rc::new(s.clone())),
                ])?;
                let repl_str = self.to_string_strict(&r)?;
                out.push_str(&repl_str);
                cur = byte_off + needle.len();
            }
            out.push_str(&s[cur..]);
            return Ok(Value::String(std::rc::Rc::new(out)));
        }
        let repl = self.to_string_strict(replacement)?;
        if needle.is_empty() {
            return Ok(Value::String(std::rc::Rc::new(s)));
        }
        Ok(Value::String(std::rc::Rc::new(s.replace(&needle, &repl))))
    }

    /// String.prototype.codePointAt(pos) per ECMA §22.1.3.4.
    pub fn string_proto_code_point_at_via(&mut self, this: &Value, pos: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let i_n = match pos { Value::Undefined => 0.0, v => self.coerce_to_number(v)? };
        if !i_n.is_finite() || i_n < 0.0 { return Ok(Value::Undefined); }
        let i = i_n as i64;
        let mut u16_idx: i64 = 0;
        for c in s.chars() {
            let units = c.len_utf16() as i64;
            if u16_idx == i { return Ok(Value::Number(c as u32 as f64)); }
            if u16_idx < i && i < u16_idx + units {
                let cp = c as u32;
                let low = 0xDC00 + ((cp - 0x10000) & 0x3FF);
                return Ok(Value::Number(low as f64));
            }
            u16_idx += units;
        }
        Ok(Value::Undefined)
    }

    /// String.prototype.at(index) per ECMA §22.1.3.2.
    pub fn string_proto_at_via(&mut self, this: &Value, index: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let i_n = match index { Value::Undefined => 0.0, v => self.coerce_to_number(v)? };
        let i = i_n as i64;
        let idx = if i < 0 { len + i } else { i };
        if idx < 0 || idx >= len { return Ok(Value::Undefined); }
        Ok(Value::String(std::rc::Rc::new(chars[idx as usize].to_string())))
    }

    /// String.prototype.normalize(form) per ECMA §22.1.3.13.
    /// v1 deviation: no actual NFC/NFD/NFKC/NFKD; coerces this and returns it.
    pub fn string_proto_normalize_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        Ok(Value::String(std::rc::Rc::new(self.to_string_strict(this)?)))
    }

    /// String.prototype.localeCompare(that) per ECMA §22.1.3.10.
    /// v1 deviation: locale-insensitive lexicographic compare.
    pub fn string_proto_locale_compare_via(&mut self, this: &Value, that: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let a = self.to_string_strict(this)?;
        let b = self.to_string_strict(that)?;
        Ok(Value::Number(match a.cmp(&b) {
            std::cmp::Ordering::Less => -1.0,
            std::cmp::Ordering::Equal => 0.0,
            std::cmp::Ordering::Greater => 1.0,
        }))
    }

    /// Helper: IsRegExp per §7.2.8 — checks @@match then InternalKind::RegExp.
    pub fn is_regexp_like_via(&mut self, v: &Value) -> Result<bool, RuntimeError> {
        let id = match v { Value::Object(id) => *id, _ => return Ok(false) };
        let matcher = self.read_property(id, "@@match")?;
        match matcher {
            Value::Undefined => Ok(matches!(self.obj(id).internal_kind, crate::value::InternalKind::RegExp(_))),
            _ => Ok(crate::abstract_ops::to_boolean(&matcher)),
        }
    }

    /// String.prototype.slice(start, end) per ECMA §22.1.3.22.
    pub fn string_proto_slice_via(&mut self, this: &Value, start: &Value, end: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let start_n = match start { Value::Undefined => 0.0, v => self.coerce_to_number(v)? };
        let end_n = match end { Value::Undefined => len as f64, v => self.coerce_to_number(v)? };
        let from = { let i = start_n as i64; if i < 0 { (len + i).max(0) } else { i.min(len) } };
        let to = { let i = end_n as i64; if i < 0 { (len + i).max(0) } else { i.min(len) } };
        if to <= from { return Ok(Value::String(std::rc::Rc::new(String::new()))); }
        let out: String = chars[from as usize..to as usize].iter().collect();
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// String.prototype.substring(start, end) per ECMA §22.1.3.24.
    pub fn string_proto_substring_via(&mut self, this: &Value, a: &Value, b: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let a_n = match a { Value::Undefined => 0.0, v => self.coerce_to_number(v)? };
        let b_n = match b { Value::Undefined => len as f64, v => self.coerce_to_number(v)? };
        let mut lo = (a_n as i64).clamp(0, len);
        let mut hi = (b_n as i64).clamp(0, len);
        if lo > hi { std::mem::swap(&mut lo, &mut hi); }
        let out: String = chars[lo as usize..hi as usize].iter().collect();
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// String.prototype.substr(start, length) per Annex B.2.2.1.
    pub fn string_proto_substr_via(&mut self, this: &Value, start: &Value, count: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let mut from = match start { Value::Undefined => 0, v => self.coerce_to_number(v)? as i64 };
        if from < 0 { from = (len + from).max(0); }
        let from = from.min(len) as usize;
        let count_n = match count { Value::Undefined => (len - from as i64) as f64, v => self.coerce_to_number(v)? };
        let n = (count_n as i64).max(0) as usize;
        let to = (from + n).min(chars.len());
        let out: String = chars[from..to].iter().collect();
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// String.prototype.indexOf(search, position) per ECMA §22.1.3.8.
    pub fn string_proto_index_of_via(&mut self, this: &Value, search: &Value, position: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) { let _ = self.coerce_to_number(position)?; }
        match s.find(&needle) {
            Some(byte_off) => Ok(Value::Number(s[..byte_off].chars().count() as f64)),
            None => Ok(Value::Number(-1.0)),
        }
    }

    /// String.prototype.lastIndexOf(search, position) per ECMA §22.1.3.10.
    pub fn string_proto_last_index_of_via(&mut self, this: &Value, search: &Value, position: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) { let _ = self.coerce_to_number(position)?; }
        match s.rfind(&needle) {
            Some(byte_off) => Ok(Value::Number(s[..byte_off].chars().count() as f64)),
            None => Ok(Value::Number(-1.0)),
        }
    }

    /// String.prototype.includes(search, position) per ECMA §22.1.3.7.
    pub fn string_proto_includes_via(&mut self, this: &Value, search: &Value, position: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        if self.is_regexp_like_via(search)? {
            return Err(RuntimeError::TypeError("String.prototype.includes: searchString cannot be a RegExp".into()));
        }
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) { let _ = self.coerce_to_number(position)?; }
        Ok(Value::Boolean(s.contains(&needle)))
    }

    /// String.prototype.startsWith(search, position) per ECMA §22.1.3.23.
    pub fn string_proto_starts_with_via(&mut self, this: &Value, search: &Value, position: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        if self.is_regexp_like_via(search)? {
            return Err(RuntimeError::TypeError("String.prototype.startsWith: searchString cannot be a RegExp".into()));
        }
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) { let _ = self.coerce_to_number(position)?; }
        Ok(Value::Boolean(s.starts_with(&needle)))
    }

    /// String.prototype.endsWith(search, position) per ECMA §22.1.3.6.
    pub fn string_proto_ends_with_via(&mut self, this: &Value, search: &Value, position: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        if self.is_regexp_like_via(search)? {
            return Err(RuntimeError::TypeError("String.prototype.endsWith: searchString cannot be a RegExp".into()));
        }
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) { let _ = self.coerce_to_number(position)?; }
        Ok(Value::Boolean(s.ends_with(&needle)))
    }

    /// String.prototype.trim() per ECMA §22.1.3.32.
    pub fn string_proto_trim_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        Ok(Value::String(std::rc::Rc::new(s.trim().to_string())))
    }

    /// String.prototype.trimStart() per ECMA §22.1.3.34.
    pub fn string_proto_trim_start_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        Ok(Value::String(std::rc::Rc::new(s.trim_start().to_string())))
    }

    /// String.prototype.trimEnd() per ECMA §22.1.3.33.
    pub fn string_proto_trim_end_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        Ok(Value::String(std::rc::Rc::new(s.trim_end().to_string())))
    }

    /// String.prototype.toLowerCase() per ECMA §22.1.3.28.
    pub fn string_proto_to_lower_case_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        Ok(Value::String(std::rc::Rc::new(s.to_lowercase())))
    }

    /// String.prototype.toUpperCase() per ECMA §22.1.3.30.
    pub fn string_proto_to_upper_case_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        Ok(Value::String(std::rc::Rc::new(s.to_uppercase())))
    }

    /// String.prototype.charAt(pos) per ECMA §22.1.3.1.
    pub fn string_proto_char_at_via(&mut self, this: &Value, pos: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let i_n = match pos {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        if !i_n.is_finite() || i_n < 0.0 {
            return Ok(Value::String(std::rc::Rc::new(String::new())));
        }
        let c = s.chars().nth(i_n as usize).map(|c| c.to_string()).unwrap_or_default();
        Ok(Value::String(std::rc::Rc::new(c)))
    }

    /// String.prototype.charCodeAt(pos) per ECMA §22.1.3.2.
    pub fn string_proto_char_code_at_via(&mut self, this: &Value, pos: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let i_n = match pos {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        if !i_n.is_finite() || i_n < 0.0 { return Ok(Value::Number(f64::NAN)); }
        match s.chars().nth(i_n as usize) {
            Some(c) => Ok(Value::Number(c as u32 as f64)),
            None => Ok(Value::Number(f64::NAN)),
        }
    }

    /// String.prototype.concat(...args) per ECMA §22.1.3.3.
    pub fn string_proto_concat_via(&mut self, this: &Value, args: &[Value]) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let mut s = self.to_string_strict(this)?;
        for a in args {
            s.push_str(&self.to_string_strict(a)?);
        }
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    /// Number.prototype.valueOf() per ECMA §21.1.3.7 — ThisNumberValue.
    pub fn number_proto_value_of_via(&self, this: &Value) -> Result<Value, RuntimeError> {
        match self.unwrap_primitive(this) {
            Value::Number(n) => Ok(Value::Number(n)),
            _ => Err(RuntimeError::TypeError(
                "Number.prototype.valueOf: this is not a Number".into())),
        }
    }

    /// Number.prototype.toExponential(digits) per ECMA §21.1.3.2.
    pub fn number_proto_to_exponential_via(&mut self, this: &Value, digits_arg: &Value) -> Result<Value, RuntimeError> {
        let n = match self.unwrap_primitive(this) {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toExponential: this is not a Number".into())),
        };
        let digits = match digits_arg {
            Value::Undefined => None,
            v => {
                let dn = self.coerce_to_number(v)?;
                if dn.is_nan() || dn < 0.0 || dn > 100.0 {
                    return Err(RuntimeError::RangeError(
                        "toExponential() digits argument must be between 0 and 100".into()));
                }
                Some(dn as usize)
            }
        };
        if n.is_nan() { return Ok(Value::String(std::rc::Rc::new("NaN".into()))); }
        if !n.is_finite() {
            return Ok(Value::String(std::rc::Rc::new(
                if n > 0.0 { "Infinity".into() } else { "-Infinity".into() })));
        }
        let s = match digits {
            Some(d) => format!("{:.*e}", d, n),
            None => format!("{:e}", n),
        };
        // Rust uses "1e0"; JS uses "1e+0" — patch.
        let mut out = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            out.push(c);
            if c == 'e' {
                if let Some(&next) = chars.peek() {
                    if next != '-' && next != '+' { out.push('+'); }
                }
            }
        }
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// Number.prototype.toPrecision(precision) per ECMA §21.1.3.5.
    pub fn number_proto_to_precision_via(&mut self, this: &Value, precision_arg: &Value) -> Result<Value, RuntimeError> {
        let n = match self.unwrap_primitive(this) {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toPrecision: this is not a Number".into())),
        };
        match precision_arg {
            Value::Undefined => Ok(Value::String(std::rc::Rc::new(
                crate::abstract_ops::number_to_string(n)))),
            v => {
                let pn = self.coerce_to_number(v)?;
                if pn.is_nan() || pn < 1.0 || pn > 100.0 {
                    return Err(RuntimeError::RangeError(
                        "toPrecision() argument must be between 1 and 100".into()));
                }
                let p = pn as usize;
                if n.is_nan() { return Ok(Value::String(std::rc::Rc::new("NaN".into()))); }
                if !n.is_finite() {
                    return Ok(Value::String(std::rc::Rc::new(
                        if n > 0.0 { "Infinity".into() } else { "-Infinity".into() })));
                }
                Ok(Value::String(std::rc::Rc::new(format!("{:.*}", p.saturating_sub(1), n))))
            }
        }
    }

    /// Boolean.prototype.valueOf() per ECMA §20.3.3.3 — ThisBooleanValue.
    pub fn boolean_proto_value_of_via(&self, this: &Value) -> Result<Value, RuntimeError> {
        match self.unwrap_primitive(this) {
            Value::Boolean(b) => Ok(Value::Boolean(b)),
            _ => Err(RuntimeError::TypeError(
                "Boolean.prototype.valueOf: this is not a Boolean".into())),
        }
    }

    /// Boolean.prototype.toString() per ECMA §20.3.3.2.
    pub fn boolean_proto_to_string_via(&self, this: &Value) -> Result<Value, RuntimeError> {
        match self.unwrap_primitive(this) {
            Value::Boolean(b) => Ok(Value::String(std::rc::Rc::new(b.to_string()))),
            _ => Err(RuntimeError::TypeError(
                "Boolean.prototype.toString: this is not a Boolean".into())),
        }
    }

    /// Number.prototype.toFixed(digits) per ECMA §21.1.3.3 — ThisNumberValue
    /// brand + RangeError on digits not in [0, 100] + NaN/Infinity
    /// short-circuit + Rust's f64 fixed-point formatting.
    pub fn number_proto_to_fixed_via(&mut self, this: &Value, digits_arg: &Value) -> Result<Value, RuntimeError> {
        let n = match self.unwrap_primitive(this) {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toFixed: this is not a Number".into())),
        };
        let digits_n = match digits_arg {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        if digits_n.is_nan() || digits_n < 0.0 || digits_n > 100.0 {
            return Err(RuntimeError::RangeError(
                "toFixed() digits argument must be between 0 and 100".into()));
        }
        let digits = digits_n as usize;
        if n.is_nan() { return Ok(Value::String(std::rc::Rc::new("NaN".into()))); }
        if !n.is_finite() {
            return Ok(Value::String(std::rc::Rc::new(
                if n > 0.0 { "Infinity".into() } else { "-Infinity".into() })));
        }
        Ok(Value::String(std::rc::Rc::new(format!("{:.*}", digits, n))))
    }

    /// Object.fromEntries(iter) per ECMA §20.1.2.7 — iterates the
    /// iterable and constructs an object from [key, value] pairs.
    /// Tier 1.10 simplification: uses cruftless's existing collect_iterable
    /// (which handles array-shape and @@iterator-shape iterables).
    pub fn object_from_entries_via(&mut self, iter: &Value) -> Result<Value, RuntimeError> {
        let out = self.alloc_object(crate::value::Object::new_ordinary());
        let entries = crate::intrinsics::collect_iterable(self, iter.clone())?;
        for e in entries {
            if let Value::Object(pair) = e {
                let k = self.object_get(pair, "0");
                let v = self.object_get(pair, "1");
                let key = crate::abstract_ops::to_string(&k).as_str().to_string();
                self.object_set(out, key, v);
            }
        }
        Ok(Value::Object(out))
    }

    /// Object.assign(target, ...sources) per ECMA §20.1.2.1 — copies
    /// enumerable own props from each source to target, dispatching
    /// accessor getters. Target must be coercible (throws otherwise).
    pub fn object_assign_via(&mut self, target: &Value, sources: &[Value]) -> Result<Value, RuntimeError> {
        let target_id = match target {
            Value::Object(id) => *id,
            Value::Undefined | Value::Null => return Err(RuntimeError::TypeError(
                "Object.assign: target cannot be undefined or null".into())),
            _ => {
                let boxed = self.to_object(target)?;
                if let Value::Object(id) = boxed { id }
                else { return Err(RuntimeError::TypeError(
                    "Object.assign: target must be coercible to Object".into())); }
            }
        };
        for src in sources {
            if let Value::Object(sid) = src {
                let entries: Vec<(String, Option<Value>, bool)> = self.obj(*sid).properties.iter()
                    .filter(|(_, d)| d.enumerable)
                    .map(|(k, d)| (k.clone(), d.getter.clone(), d.getter.is_none()))
                    .collect();
                for (k, getter_opt, is_data) in entries {
                    let v = if let Some(getter) = getter_opt {
                        self.call_function(getter, Value::Object(*sid), Vec::new())?
                    } else if is_data {
                        self.object_get(*sid, &k)
                    } else { continue };
                    self.object_set(target_id, k, v);
                }
            }
            // null/undefined sources are silently ignored per spec step 4.b.
        }
        Ok(Value::Object(target_id))
    }

    /// Object.getOwnPropertyNames(O) per ECMA §20.1.2.10 — returns Array
    /// of own string-keyed property names (excluding @@-prefixed symbols).
    /// Integer-indexed keys first in ascending order.
    pub fn own_property_names_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Object(self.alloc_object(crate::value::Object::new_array()))),
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        let keys: Vec<String> = {
            let o = self.obj(id);
            let is_array = matches!(o.internal_kind, crate::value::InternalKind::Array);
            if is_array {
                let mut ks: Vec<(u64, String)> = o.properties.iter()
                    .filter_map(|(k, _)| if k.starts_with("@@") { None } else {
                        k.parse::<u64>().ok().map(|n| (n, k.clone()))
                    })
                    .collect();
                ks.sort_by_key(|(n, _)| *n);
                let mut out: Vec<String> = ks.into_iter().map(|(_, k)| k).collect();
                // Arrays always have a "length" own property per §10.4.2.4;
                // unconditionally include it to match Bun + spec.
                out.push("length".into());
                out
            } else {
                o.properties.keys().filter(|k| !k.starts_with("@@")).cloned().collect()
            }
        };
        for (i, k) in keys.iter().enumerate() {
            self.object_set(arr, i.to_string(), Value::String(std::rc::Rc::new(k.clone())));
        }
        self.object_set(arr, "length".into(), Value::Number(keys.len() as f64));
        Ok(Value::Object(arr))
    }

    /// Object.getOwnPropertySymbols(O) per ECMA §20.1.2.11 — returns Array
    /// of user-created Symbol-keyed properties. Well-known slots
    /// (@@iterator etc.) are filtered out per cruftless convention.
    pub fn own_property_symbols_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Object(self.alloc_object(crate::value::Object::new_array()))),
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        let syms: Vec<String> = self.obj(id).properties.keys()
            .filter(|k| k.starts_with("@@sym:"))
            .cloned()
            .collect();
        for (i, s) in syms.iter().enumerate() {
            self.object_set(arr, i.to_string(), Value::Symbol(std::rc::Rc::new(s.clone())));
        }
        self.object_set(arr, "length".into(), Value::Number(syms.len() as f64));
        Ok(Value::Object(arr))
    }

    /// Math.* binary op dispatcher per ECMA §21.3.2.{27, 8} — pow, atan2.
    pub fn math_binary_op_via(&self, op: &Value, x: &Value, y: &Value) -> Result<Value, RuntimeError> {
        let op_name = match op {
            Value::String(s) => s.as_str().to_string(),
            _ => return Err(RuntimeError::TypeError("math_binary_op_via: op must be a string".into())),
        };
        let nx = crate::abstract_ops::to_number(x);
        let ny = crate::abstract_ops::to_number(y);
        let r = match op_name.as_str() {
            "pow" => nx.powf(ny),
            "atan2" => nx.atan2(ny),
            _ => return Err(RuntimeError::TypeError(format!(
                "math_binary_op_via: unknown op '{}'", op_name))),
        };
        Ok(Value::Number(r))
    }

    /// Math.max — variadic; NaN if any arg is NaN.
    pub fn math_max_via(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut m = f64::NEG_INFINITY;
        for v in args {
            let n = crate::abstract_ops::to_number(v);
            if n.is_nan() { return Ok(Value::Number(f64::NAN)); }
            if n > m { m = n; }
        }
        Ok(Value::Number(m))
    }

    /// Math.min — variadic; NaN if any arg is NaN.
    pub fn math_min_via(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut m = f64::INFINITY;
        for v in args {
            let n = crate::abstract_ops::to_number(v);
            if n.is_nan() { return Ok(Value::Number(f64::NAN)); }
            if n < m { m = n; }
        }
        Ok(Value::Number(m))
    }

    /// Math.hypot — variadic; sqrt(sum-of-squares).
    pub fn math_hypot_via(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut s = 0.0_f64;
        for v in args {
            let n = crate::abstract_ops::to_number(v);
            s += n * n;
        }
        Ok(Value::Number(s.sqrt()))
    }

    /// Reflect.getPrototypeOf(target) per ECMA §28.1.7 — like
    /// Object.getPrototypeOf but throws TypeError on non-Object target
    /// (Object.getPrototypeOf returns null).
    pub fn reflect_get_prototype_of_via(&self, target: &Value) -> Result<Value, RuntimeError> {
        match target {
            Value::Object(id) => match self.obj(*id).proto {
                Some(p) => Ok(Value::Object(p)),
                None => Ok(Value::Null),
            },
            _ => Err(RuntimeError::TypeError("Reflect.getPrototypeOf: target must be Object".into())),
        }
    }

    /// Reflect.setPrototypeOf(target, proto) per ECMA §28.1.14 — returns
    /// boolean (true on success) instead of the target.
    pub fn reflect_set_prototype_of_via(&mut self, target: &Value, proto: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError("Reflect.setPrototypeOf: target must be Object".into())),
        };
        let new_proto = match proto {
            Value::Object(p) => Some(*p),
            Value::Null => None,
            _ => return Err(RuntimeError::TypeError("Reflect.setPrototypeOf: prototype must be Object or null".into())),
        };
        self.obj_mut(id).proto = new_proto;
        Ok(Value::Boolean(true))
    }

    /// Reflect.isExtensible(target) per ECMA §28.1.10 — throws TypeError
    /// on non-Object (Object.isExtensible returns false).
    pub fn reflect_is_extensible_via(&self, target: &Value) -> Result<Value, RuntimeError> {
        match target {
            Value::Object(id) => Ok(Value::Boolean(self.obj(*id).extensible)),
            _ => Err(RuntimeError::TypeError("Reflect.isExtensible: target must be Object".into())),
        }
    }

    /// Reflect.preventExtensions(target) per ECMA §28.1.11 — returns
    /// boolean instead of the target.
    pub fn reflect_prevent_extensions_via(&mut self, target: &Value) -> Result<Value, RuntimeError> {
        match target {
            Value::Object(id) => { self.obj_mut(*id).extensible = false; Ok(Value::Boolean(true)) }
            _ => Err(RuntimeError::TypeError("Reflect.preventExtensions: target must be Object".into())),
        }
    }

    /// Reflect.has(target, key) per ECMA §28.1.9 — proto-chain HasProperty.
    pub fn reflect_has_via(&mut self, target: &Value, key: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError("Reflect.has: target must be Object".into())),
        };
        let key_s = self.coerce_to_string(key)?;
        Ok(Value::Boolean(self.has_property(id, &key_s)))
    }

    /// Reflect.get(target, key) per ECMA §28.1.8 — dispatches accessor getters.
    pub fn reflect_get_via(&mut self, target: &Value, key: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError("Reflect.get: target must be Object".into())),
        };
        let key_s = self.coerce_to_string(key)?;
        self.read_property(id, &key_s)
    }

    /// Reflect.set(target, key, value) per ECMA §28.1.13.
    pub fn reflect_set_via(&mut self, target: &Value, key: &Value, value: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError("Reflect.set: target must be Object".into())),
        };
        let key_s = self.coerce_to_string(key)?;
        self.object_set(id, key_s, value.clone());
        Ok(Value::Boolean(true))
    }

    /// Reflect.deleteProperty(target, key) per ECMA §28.1.4 — honors
    /// non-configurable per §10.1.10 (returns false instead of throwing).
    pub fn reflect_delete_property_via(&mut self, target: &Value, key: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError("Reflect.deleteProperty: target must be Object".into())),
        };
        let key_s = self.coerce_to_string(key)?;
        let configurable = self.obj(id).properties.get(&key_s).map(|d| d.configurable).unwrap_or(true);
        if !configurable { return Ok(Value::Boolean(false)); }
        self.obj_mut(id).properties.shift_remove(&key_s);
        Ok(Value::Boolean(true))
    }

    /// Reflect.ownKeys(target) per ECMA §28.1.12 — returns Array of own
    /// keys (Symbol-typed for @@sym: form, String-typed for everything else).
    pub fn reflect_own_keys_via(&mut self, target: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError("Reflect.ownKeys: target must be Object".into())),
        };
        let keys: Vec<String> = self.obj(id).properties.keys().cloned().collect();
        let arr = self.alloc_object(crate::value::Object::new_array());
        for (i, k) in keys.iter().enumerate() {
            let v = if k.starts_with("@@sym:") {
                Value::Symbol(std::rc::Rc::new(k.clone()))
            } else {
                Value::String(std::rc::Rc::new(k.clone()))
            };
            self.object_set(arr, i.to_string(), v);
        }
        self.object_set(arr, "length".into(), Value::Number(keys.len() as f64));
        Ok(Value::Object(arr))
    }

    /// Math.* unary op dispatcher per ECMA §21.3.2 — applies the named
    /// math operation to ToNumber(x). Used by Math.{abs, floor, ceil,
    /// round, trunc, sign, sqrt, cbrt}. The op name is passed as a
    /// Value::String (matching CallBuiltin's argument convention).
    pub fn math_unary_op_via(&self, op: &Value, x: &Value) -> Result<Value, RuntimeError> {
        let op_name = match op {
            Value::String(s) => s.as_str().to_string(),
            _ => return Err(RuntimeError::TypeError(
                "math_unary_op_via: op must be a string".into())),
        };
        let n = crate::abstract_ops::to_number(x);
        let r = match op_name.as_str() {
            "abs" => n.abs(),
            "floor" => n.floor(),
            "ceil" => n.ceil(),
            // JS Math.round rounds half-toward-positive-infinity.
            "round" => (n + 0.5).floor(),
            "trunc" => n.trunc(),
            "sqrt" => n.sqrt(),
            "cbrt" => n.cbrt(),
            // Exponential / logarithmic family (ECMA §21.3.2.14/.20-.24).
            "exp" => n.exp(),
            "expm1" => n.exp_m1(),
            "log" => n.ln(),
            "log1p" => n.ln_1p(),
            "log2" => n.log2(),
            "log10" => n.log10(),
            // Trigonometric family (ECMA §21.3.2.5-.7, .29, .31, .33).
            "sin" => n.sin(),
            "cos" => n.cos(),
            "tan" => n.tan(),
            "asin" => n.asin(),
            "acos" => n.acos(),
            "atan" => n.atan(),
            // Hyperbolic family (ECMA §21.3.2.34, .8, .12, .2, .4, .3).
            "sinh" => n.sinh(),
            "cosh" => n.cosh(),
            "tanh" => n.tanh(),
            "asinh" => n.asinh(),
            "acosh" => n.acosh(),
            "atanh" => n.atanh(),
            "sign" => {
                if n.is_nan() { f64::NAN }
                else if n > 0.0 { 1.0 }
                else if n < 0.0 { -1.0 }
                else { n }   // preserves +0 / -0
            }
            _ => return Err(RuntimeError::TypeError(format!(
                "math_unary_op_via: unknown op '{}'", op_name))),
        };
        Ok(Value::Number(r))
    }

    /// Global isNaN(v) per ECMA §19.2.3 — coerces via ToNumber unlike
    /// Number.isNaN which returns false on non-Number args.
    pub fn global_is_nan_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let n = self.coerce_to_number(v)?;
        Ok(Value::Boolean(n.is_nan()))
    }

    /// Global isFinite(v) per ECMA §19.2.2.
    pub fn global_is_finite_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let n = self.coerce_to_number(v)?;
        Ok(Value::Boolean(n.is_finite()))
    }

    /// Number.isInteger(v) per ECMA §21.1.2.3 — Number-typed + finite + integer.
    pub fn number_is_integer_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        let n = match v {
            Value::Number(n) => *n,
            _ => return Ok(Value::Boolean(false)),
        };
        Ok(Value::Boolean(n.is_finite() && n.floor() == n))
    }

    /// Number.isFinite(v) per ECMA §21.1.2.2 — Number-typed AND finite.
    pub fn number_is_finite_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        let n = match v {
            Value::Number(n) => *n,
            _ => return Ok(Value::Boolean(false)),
        };
        Ok(Value::Boolean(n.is_finite()))
    }

    /// Number.isNaN(v) per ECMA §21.1.2.4 — Number-typed AND NaN.
    pub fn number_is_nan_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        let n = match v {
            Value::Number(n) => *n,
            _ => return Ok(Value::Boolean(false)),
        };
        Ok(Value::Boolean(n.is_nan()))
    }

    /// Number.isSafeInteger(v) per ECMA §21.1.2.5.
    pub fn number_is_safe_integer_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        let n = match v {
            Value::Number(n) => *n,
            _ => return Ok(Value::Boolean(false)),
        };
        Ok(Value::Boolean(n.is_finite() && n.floor() == n && n.abs() <= 9007199254740991.0))
    }

    /// Object.freeze(O) per ECMA §20.1.2.7 — sets extensible:false and
    /// every own property to non-writable + non-configurable. Returns O.
    pub fn object_freeze_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        if let Value::Object(id) = v {
            let o = self.obj_mut(*id);
            o.extensible = false;
            for d in o.properties.values_mut() {
                d.writable = false;
                d.configurable = false;
            }
        }
        Ok(v.clone())
    }

    /// Object.seal(O) per ECMA §20.1.2.20.
    pub fn object_seal_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        if let Value::Object(id) = v {
            let o = self.obj_mut(*id);
            o.extensible = false;
            for d in o.properties.values_mut() {
                d.configurable = false;
            }
        }
        Ok(v.clone())
    }

    /// Object.preventExtensions(O) per ECMA §20.1.2.18.
    pub fn object_prevent_extensions_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        if let Value::Object(id) = v {
            self.obj_mut(*id).extensible = false;
        }
        Ok(v.clone())
    }

    /// Object.hasOwn(O, P) per ECMA §20.1.2.13.
    pub fn object_has_own_via(&mut self, v: &Value, key: &Value) -> Result<Value, RuntimeError> {
        let key_s = self.coerce_to_string(key)?;
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Boolean(false)),
        };
        Ok(Value::Boolean(self.obj(id).properties.contains_key(&key_s)))
    }

    /// Object.is(a, b) per ECMA §20.1.2.14 — SameValue.
    pub fn object_is_via(&self, a: &Value, b: &Value) -> Result<Value, RuntimeError> {
        Ok(Value::Boolean(crate::abstract_ops::same_value(a, b)))
    }

    /// Object.getPrototypeOf(O) per ECMA §20.1.2.12.
    pub fn get_prototype_of_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        match v {
            Value::Object(id) => match self.obj(*id).proto {
                Some(p) => Ok(Value::Object(p)),
                None => Ok(Value::Null),
            },
            _ => Ok(Value::Null),
        }
    }

    /// Object.setPrototypeOf(O, proto) per ECMA §20.1.2.21.
    pub fn set_prototype_of_via(&mut self, v: &Value, proto: &Value) -> Result<Value, RuntimeError> {
        let target = match v {
            Value::Object(id) => *id,
            _ => return Ok(v.clone()),
        };
        let new_proto = match proto {
            Value::Object(id) => Some(*id),
            Value::Null => None,
            _ => return Err(RuntimeError::TypeError(
                "Object.setPrototypeOf: prototype must be Object or null".into())),
        };
        self.obj_mut(target).proto = new_proto;
        Ok(Value::Object(target))
    }

    /// Object.isFrozen(O) per ECMA §20.1.2.16.
    pub fn is_frozen_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Boolean(true)),
        };
        let o = self.obj(id);
        let frozen = !o.extensible
            && o.properties.values().all(|d| !d.writable && !d.configurable);
        Ok(Value::Boolean(frozen))
    }

    /// Object.isSealed(O) per ECMA §20.1.2.17.
    pub fn is_sealed_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Boolean(true)),
        };
        let o = self.obj(id);
        let sealed = !o.extensible
            && o.properties.values().all(|d| !d.configurable);
        Ok(Value::Boolean(sealed))
    }

    /// Object.isExtensible(O) per ECMA §20.1.2.14.
    pub fn is_extensible_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Boolean(false)),
        };
        Ok(Value::Boolean(self.obj(id).extensible))
    }

    /// Promise.resolve(v) per ECMA §27.2.4.7 — IR-target for the
    /// "promise-wrap if not a thenable; otherwise return as-is"
    /// abstract op. Tier 1.10 simplification: always allocates a new
    /// promise and resolves it with v; spec-fast-path for "v is already
    /// a Promise of the same constructor" is deferred.
    pub fn promise_resolve_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let p = crate::promise::new_promise(self);
        crate::promise::resolve_promise(self, p, v.clone());
        Ok(Value::Object(p))
    }

    /// Promise.reject(r) per ECMA §27.2.4.5 — IR-target.
    pub fn promise_reject_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let p = crate::promise::new_promise(self);
        crate::promise::reject_promise(self, p, v.clone());
        Ok(Value::Object(p))
    }

    /// EnumerableOwnPropertyNames(O, "key") per ECMA §7.3.23 — returns
    /// the Array of own string keys of O, filtering @@-prefixed (Symbol)
    /// keys and Array's implicit `length`. Integer-index keys come first
    /// in ascending numeric order, then string keys in insertion order.
    /// IR-target for Object.keys per §20.1.2.18.
    pub fn enumerable_own_keys(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Object(self.alloc_object(crate::value::Object::new_array()))),
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        let keys: Vec<String> = {
            let o = self.obj(id);
            let is_array = matches!(o.internal_kind, crate::value::InternalKind::Array);
            if is_array {
                let mut ks: Vec<(u64, String)> = o.properties.iter()
                    .filter_map(|(k, d)| if d.enumerable && k != "length" && !k.starts_with("@@") {
                        k.parse::<u64>().ok().map(|n| (n, k.clone()))
                    } else { None })
                    .collect();
                ks.sort_by_key(|(n, _)| *n);
                ks.into_iter().map(|(_, k)| k).collect()
            } else {
                let all: Vec<(String, bool)> = o.properties.iter()
                    .filter(|(k, d)| d.enumerable && !k.starts_with("@@"))
                    .map(|(k, _)| (k.clone(), crate::intrinsics::is_integer_index(k)))
                    .collect();
                let mut numeric: Vec<(u64, String)> = all.iter()
                    .filter(|(_, idx)| *idx)
                    .filter_map(|(k, _)| k.parse::<u64>().ok().map(|n| (n, k.clone())))
                    .collect();
                numeric.sort_by_key(|(n, _)| *n);
                let strings: Vec<String> = all.into_iter()
                    .filter(|(_, idx)| !*idx)
                    .map(|(k, _)| k)
                    .collect();
                let mut out: Vec<String> = numeric.into_iter().map(|(_, k)| k).collect();
                out.extend(strings);
                out
            }
        };
        for (i, k) in keys.iter().enumerate() {
            self.object_set(arr, i.to_string(), Value::String(std::rc::Rc::new(k.clone())));
        }
        self.object_set(arr, "length".into(), Value::Number(keys.len() as f64));
        Ok(Value::Object(arr))
    }

    /// EnumerableOwnPropertyNames(O, "value") per ECMA §7.3.23. IR-target
    /// for Object.values per §20.1.2.23. Dispatches accessor getters.
    pub fn enumerable_own_values(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Object(self.alloc_object(crate::value::Object::new_array()))),
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        let entries: Vec<(String, Option<Value>)> = {
            let o = self.obj(id);
            let is_array = matches!(o.internal_kind, crate::value::InternalKind::Array);
            let mut es: Vec<(String, Option<Value>)> = o.properties.iter()
                .filter(|(k, d)| d.enumerable && !(is_array && *k == "length") && !k.starts_with("@@"))
                .map(|(k, d)| (k.clone(), d.getter.clone()))
                .collect();
            if is_array {
                es.sort_by_key(|(k, _)| k.parse::<u64>().unwrap_or(u64::MAX));
            }
            es
        };
        let mut kvs: Vec<Value> = Vec::with_capacity(entries.len());
        for (k, getter_opt) in entries {
            let val = if let Some(getter) = getter_opt {
                self.call_function(getter, Value::Object(id), Vec::new())?
            } else {
                self.object_get(id, &k)
            };
            kvs.push(val);
        }
        for (i, val) in kvs.iter().enumerate() {
            self.object_set(arr, i.to_string(), val.clone());
        }
        self.object_set(arr, "length".into(), Value::Number(kvs.len() as f64));
        Ok(Value::Object(arr))
    }

    /// EnumerableOwnPropertyNames(O, "key+value") per ECMA §7.3.23. IR-target
    /// for Object.entries per §20.1.2.5.
    pub fn enumerable_own_entries(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Object(self.alloc_object(crate::value::Object::new_array()))),
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        let entries: Vec<(String, Option<Value>)> = {
            let o = self.obj(id);
            let is_array = matches!(o.internal_kind, crate::value::InternalKind::Array);
            let mut es: Vec<(String, Option<Value>)> = o.properties.iter()
                .filter(|(k, d)| d.enumerable && !(is_array && *k == "length") && !k.starts_with("@@"))
                .map(|(k, d)| (k.clone(), d.getter.clone()))
                .collect();
            if is_array {
                es.sort_by_key(|(k, _)| k.parse::<u64>().unwrap_or(u64::MAX));
            }
            es
        };
        let mut kvs: Vec<(String, Value)> = Vec::with_capacity(entries.len());
        for (k, getter_opt) in entries {
            let val = if let Some(getter) = getter_opt {
                self.call_function(getter, Value::Object(id), Vec::new())?
            } else {
                self.object_get(id, &k)
            };
            kvs.push((k, val));
        }
        for (i, (k, val)) in kvs.iter().enumerate() {
            let pair = self.alloc_object(crate::value::Object::new_array());
            self.object_set(pair, "0".into(), Value::String(std::rc::Rc::new(k.clone())));
            self.object_set(pair, "1".into(), val.clone());
            self.object_set(pair, "length".into(), Value::Number(2.0));
            self.object_set(arr, i.to_string(), Value::Object(pair));
        }
        self.object_set(arr, "length".into(), Value::Number(kvs.len() as f64));
        Ok(Value::Object(arr))
    }

    /// ArraySpeciesCreate per ECMA §23.1.3.27 — Tier 1.5 simplification:
    /// always returns a fresh ordinary Array with [[Prototype]] set to
    /// %Array.prototype% and length pre-populated. Full @@species
    /// dispatch is queued for Tier 2.
    pub fn array_species_create(&mut self, _o: &Value, len: usize) -> Result<Value, RuntimeError> {
        let id = self.alloc_object(crate::value::Object::new_array());
        // Only set explicit length when len > 0; for len=0 (filter, etc.)
        // let the array length derive from max-index, so subsequent
        // CreateDataPropertyOrThrow calls grow the length naturally.
        if len > 0 {
            self.object_set(id, "length".into(), Value::Number(len as f64));
        }
        Ok(Value::Object(id))
    }

    /// CreateDataPropertyOrThrow on an Array receiver: ensure the receiver's
    /// length reflects the new max-index. Internal helper called from
    /// create_data_property_or_throw when applicable.
    fn bump_array_length_if_needed(&mut self, id: rusty_js_gc::ObjectId, key: &str) {
        let is_array = matches!(
            self.obj(id).internal_kind,
            crate::value::InternalKind::Array
        );
        if !is_array { return; }
        let Ok(i) = key.parse::<u32>() else { return; };
        let cur_len = self.array_length(id);
        if (i as usize) >= cur_len {
            self.object_set(
                id,
                "length".into(),
                Value::Number((i as usize + 1) as f64),
            );
        }
    }

    /// Ω.5.P62.E5: IsCallable per ECMA §7.2.4 — true iff `v` is an Object
    /// whose internal kind is one of the callable forms (Function,
    /// Closure, BoundFunction, Proxy with callable target). Used by
    /// Array.prototype.{map,forEach,filter,...} step 3 to throw
    /// TypeError before invoking a non-callable callback.
    pub fn is_callable(&self, v: &Value) -> bool {
        if let Value::Object(id) = v {
            return matches!(self.obj(*id).internal_kind,
                crate::value::InternalKind::Function(_)
                | crate::value::InternalKind::Closure(_)
                | crate::value::InternalKind::BoundFunction(_)
                | crate::value::InternalKind::Proxy(_));
        }
        false
    }

    /// Ω.5.P62.E1: unwrap a primitive-wrapper object's [[__primitive__]]
    /// slot. Returns the boxed primitive Value (String/Number/Boolean/
    /// BigInt/Symbol) if `v` is a wrapper, else returns `v` unchanged.
    /// Used by Number/String/Boolean.prototype.{toString,valueOf} so
    /// `(new Number(5)).toString()` and `(new String("hi")).valueOf()`
    /// resolve through the spec [[NumberData]]/[[StringData]] slots.
    pub fn unwrap_primitive(&self, v: &Value) -> Value {
        if let Value::Object(id) = v {
            if let Some(d) = self.obj(*id).properties.get("__primitive__") {
                return d.value.clone();
            }
        }
        v.clone()
    }

    /// Ω.5.P62.E9: ToNumber per ECMA §7.1.4 with Object→primitive dispatch.
    /// For Objects, dispatch through @@toPrimitive("number") → valueOf →
    /// toString (number-hint OrdinaryToPrimitive order, §7.1.1) and
    /// ToNumber the primitive result. Used by array_length to allow
    /// `length: {valueOf(){return 2}}` etc.
    pub fn coerce_to_number(&mut self, v: &Value) -> Result<f64, RuntimeError> {
        // Ω.5.P62.E17: ToNumber on Symbol throws TypeError per §7.1.4.
        if matches!(v, Value::Symbol(_)) {
            return Err(RuntimeError::TypeError(
                "Cannot convert a Symbol value to a number".into()));
        }
        if let Value::Object(id) = v {
            let id = *id;
            // (1) @@toPrimitive.
            let tp = self.object_get(id, "@@toPrimitive");
            if matches!(tp, Value::Object(_)) {
                let r = self.call_function(tp, v.clone(), vec![
                    Value::String(Rc::new("number".into())),
                ])?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(crate::abstract_ops::to_number(&r));
                }
            }
            // (2) valueOf (number hint prefers valueOf first per §7.1.1.1).
            let vo = self.object_get(id, "valueOf");
            if matches!(vo, Value::Object(_)) {
                let r = self.call_function(vo, v.clone(), Vec::new())?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(crate::abstract_ops::to_number(&r));
                }
            }
            // (3) toString.
            let ts = self.object_get(id, "toString");
            if matches!(ts, Value::Object(_)) {
                let r = self.call_function(ts, v.clone(), Vec::new())?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(crate::abstract_ops::to_number(&r));
                }
            }
            return Err(RuntimeError::TypeError(
                "Cannot convert object to primitive value".into()));
        }
        Ok(crate::abstract_ops::to_number(v))
    }

    /// Ω.5.P62.E13: strict ToString per ECMA §7.1.17 — throws TypeError
    /// on Symbol (and is the canonical path for built-in methods like
    /// String.prototype.includes/startsWith/endsWith/indexOf where the
    /// search-string-as-Symbol case is a test262 invariant). Also
    /// throws on null/undefined for RequireObjectCoercible-style use.
    /// Differs from coerce_to_string which preserves the @@sym: form
    /// (load-bearing for property-key dispatch in ToPropertyKey).
    pub fn to_string_strict(&mut self, v: &Value) -> Result<String, RuntimeError> {
        if matches!(v, Value::Symbol(_)) {
            return Err(RuntimeError::TypeError(
                "Cannot convert a Symbol value to a string".into()));
        }
        self.coerce_to_string(v)
    }

    /// Ω.5.P62.E13: RequireObjectCoercible per ECMA §7.2.1 — throws
    /// TypeError if value is null or undefined.
    pub fn require_object_coercible(&self, v: &Value) -> Result<(), RuntimeError> {
        if matches!(v, Value::Undefined | Value::Null) {
            return Err(RuntimeError::TypeError(
                "Cannot convert undefined or null to object".into()));
        }
        Ok(())
    }

    pub fn coerce_to_string(&mut self, v: &Value) -> Result<String, RuntimeError> {
        if let Value::Object(id) = v {
            let id = *id;
            let mut tried = false;
            // (1) @@toPrimitive.
            let tp = self.object_get(id, "@@toPrimitive");
            if matches!(tp, Value::Object(_)) {
                tried = true;
                let r = self.call_function(tp, v.clone(), vec![
                    Value::String(Rc::new("string".into())),
                ])?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(crate::abstract_ops::to_string(&r).as_str().to_string());
                }
            }
            // (2) toString.
            let ts = self.object_get(id, "toString");
            if matches!(ts, Value::Object(_)) {
                tried = true;
                let r = self.call_function(ts, v.clone(), Vec::new())?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(crate::abstract_ops::to_string(&r).as_str().to_string());
                }
            }
            // (3) valueOf.
            let vo = self.object_get(id, "valueOf");
            if matches!(vo, Value::Object(_)) {
                tried = true;
                let r = self.call_function(vo, v.clone(), Vec::new())?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(crate::abstract_ops::to_string(&r).as_str().to_string());
                }
            }
            // (4) All callable methods returned Objects (or none were
            // callable). Per spec, if any of them were called and all
            // returned Object, throw TypeError. If NONE were callable,
            // fall through to the static "[object Object]" form (which
            // is what abstract_ops::to_string yields).
            if tried {
                return Err(RuntimeError::TypeError(
                    "Cannot convert object to primitive value".into()));
            }
        }
        Ok(crate::abstract_ops::to_string(v).as_str().to_string())
    }

    /// Drain promises still rejected with no handler. Caller is the host;
    /// canonical action is print-to-stderr + exit nonzero. Idempotent.
    pub fn drain_unhandled_rejections(&mut self) -> Vec<(rusty_js_gc::ObjectId, Value)> {
        let ids: Vec<_> = self.pending_unhandled.drain().collect();
        ids.into_iter().filter_map(|id| {
            match &self.heap.get(id)?.internal_kind {
                InternalKind::Promise(ps) if matches!(ps.status, crate::value::PromiseStatus::Rejected) => {
                    Some((id, ps.value.clone()))
                }
                _ => None,
            }
        }).collect()
    }

    /// Run a full mark-sweep cycle on the heap with the runtime's
    /// current root set.
    pub fn collect(&mut self) -> usize {
        let roots = self.enumerate_roots();
        self.heap.collect(roots)
    }

    /// Enumerate every ObjectId reachable from the runtime's roots.
    ///
    /// Tracked roots:
    ///   - self.globals.values() — every Value::Object payload
    ///   - self.last_value — if Value::Object
    ///
    /// NOT tracked (3.e.d): the active call-stack frames' operand_stack /
    /// locals / try_stack. Frames are stack-allocated on the Rust call
    /// stack inside run_frame; their values are implicit roots while the
    /// frame is on the stack. This is safe because `collect()` is only
    /// invoked outside a frame's execution (e.g. by tests or external
    /// drivers between top-level run_module calls). When `collect()` is
    /// wired into the dispatch loop at safe points, frame walking will
    /// need to be added — there is no Runtime-side frame_stack field
    /// today (run_frame is called recursively via call_function with
    /// frames living on Rust's stack).
    pub fn enumerate_roots(&self) -> Vec<rusty_js_gc::ObjectId> {
        let mut roots: Vec<rusty_js_gc::ObjectId> = Vec::new();
        for v in self.globals.values() {
            if let Value::Object(id) = v { roots.push(*id); }
        }
        if let Value::Object(id) = &self.last_value { roots.push(*id); }
        roots
    }

    /// Allocate an Object via the managed heap. Returns the ObjectId
    /// handle. Tier-Ω.5.a: if the Object has no explicit proto and an
    /// intrinsic prototype matching its InternalKind has been installed,
    /// the proto is wired automatically. This is the seam through which
    /// prototype-chain method dispatch works without retrofitting every
    /// alloc call-site.
    pub fn alloc_object(&mut self, mut obj: crate::value::Object) -> rusty_js_gc::ObjectId {
        if obj.proto.is_none() {
            obj.proto = match &obj.internal_kind {
                crate::value::InternalKind::Ordinary => self.object_prototype,
                crate::value::InternalKind::Array => self.array_prototype,
                crate::value::InternalKind::Promise(_) => self.promise_prototype,
                crate::value::InternalKind::RegExp(_) => self.regexp_prototype,
                crate::value::InternalKind::Function(_)
                | crate::value::InternalKind::Closure(_)
                | crate::value::InternalKind::BoundFunction(_) => self.function_prototype,
                _ => None,
            };
        }
        self.heap.alloc(obj)
    }

    /// Ergonomic heap accessors. Panic on missing — the migration's
    /// invariant is that every ObjectId in a live Value points to a live
    /// slot. Stale handles after a sweep would be a GC-correctness bug
    /// surfaced loudly here.
    pub fn obj(&self, id: ObjectRef) -> &Object {
        self.heap.get(id).expect("ObjectId points to free/missing slot")
    }
    pub fn obj_mut(&mut self, id: ObjectRef) -> &mut Object {
        self.heap.get_mut(id).expect("ObjectId points to free/missing slot")
    }

    /// OrdinaryGet with prototype walk. Returns Undefined if neither the
    /// object nor any prototype owns the key.
    ///
    /// Tier-Ω.5.a: special-case Array.length — computed from the highest
    /// numeric-indexed own property + 1 (own-only, prototype walk skipped
    /// for this synthetic key). Matches the spec semantics close enough
    /// for the v1 surface without maintaining a separate length slot.
    /// Tier-Ω.5.nnn: walk the prototype chain looking for an accessor
    /// getter at `key`. Returns the getter function (Value::Object) if
    /// found, None for data properties or no-property.
    /// Tier-Ω.5.jjjjj: ToNumber with object valueOf dispatch.
    pub fn to_num_coerced(&mut self, v: &Value) -> Result<f64, RuntimeError> {
        match v {
            Value::Object(id) => {
                let vo = self.object_get(*id, "valueOf");
                if matches!(vo, Value::Object(_)) {
                    let r = self.call_function(vo, Value::Object(*id), Vec::new())?;
                    Ok(to_number(&r))
                } else { Ok(to_number(v)) }
            }
            _ => Ok(to_number(v)),
        }
    }
    pub fn find_getter(&self, id: ObjectRef, key: &str) -> Option<Value> {
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.properties.get(key) {
                return d.getter.clone();
            }
            cur = o.proto;
        }
        None
    }
    /// Tier-Ω.5.nnn: walk the prototype chain looking for an accessor
    /// setter at `key`. Returns the setter function if found.
    pub fn find_setter(&self, id: ObjectRef, key: &str) -> Option<Value> {
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.properties.get(key) {
                return d.setter.clone();
            }
            cur = o.proto;
        }
        None
    }

    /// Ω.5.P61.E13: getter-dispatching property read for Array.prototype
    /// iteration methods + spec sites that must invoke accessor getters
    /// per ECMA §10.1.8.1 [[Get]]. object_get returns the raw descriptor
    /// value (cheap, no getter dispatch); read_property invokes the
    /// getter if present. Array.prototype.reduce/forEach/map/filter use
    /// this so accessor-defined indices contribute their getter results
    /// rather than Undefined.
    pub fn read_property(&mut self, id: ObjectRef, key: &str) -> Result<Value, RuntimeError> {
        if let Some(getter) = self.find_getter(id, key) {
            return self.call_function(getter, Value::Object(id), Vec::new());
        }
        Ok(self.object_get(id, key))
    }

    /// Ω.5.P61.E13: HasProperty per ECMA §10.1.7.1 — walks own +
    /// prototype chain for the key. Used by Array.prototype iteration
    /// methods to skip sparse holes (a property present along the chain,
    /// even if its value is Undefined, is NOT a hole).
    pub fn has_property(&self, id: ObjectRef, key: &str) -> bool {
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if o.properties.contains_key(key) { return true; }
            cur = o.proto;
        }
        false
    }

    pub fn object_get(&self, id: ObjectRef, key: &str) -> Value {
        if key == "length" {
            let o = self.obj(id);
            if matches!(o.internal_kind, InternalKind::Array) {
                // If explicit "length" property is set, prefer it; otherwise
                // derive from max numeric index + 1.
                if let Some(d) = o.properties.get("length") {
                    return d.value.clone();
                }
                let mut max: i64 = -1;
                for k in o.properties.keys() {
                    if let Ok(i) = k.parse::<i64>() {
                        if i > max { max = i; }
                    }
                }
                return Value::Number((max + 1) as f64);
            }
        }
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.properties.get(key) {
                return d.value.clone();
            }
            cur = o.proto;
        }
        Value::Undefined
    }

    /// Array length helper used by Array.prototype.* methods.
    pub fn array_length(&mut self, id: ObjectRef) -> usize {
        // Ω.5.P62.E6: ToLength per ECMA §7.1.20.
        // Ω.5.P62.E8: read via read_property so accessor `length` getters
        // (test262 map-2-7/2-10: Object.defineProperty(obj, "length",
        // {get: ...})) dispatch their getter rather than returning the
        // raw undefined data slot.
        let v = self.read_property(id, "length").unwrap_or(Value::Undefined);
        let n = self.coerce_to_number(&v).unwrap_or(f64::NAN);
        if n.is_nan() || n <= 0.0 { return 0; }
        if !n.is_finite() { return usize::MAX; }
        let n = n.floor();
        let max_safe = 9007199254740991.0_f64; // 2^53 - 1
        let clamped = if n > max_safe { max_safe } else { n };
        clamped as usize
    }

    /// OrdinaryDefineOwnProperty — own-key set on the named object.
    pub fn object_set(&mut self, id: ObjectRef, key: String, value: Value) {
        // Ω.5.P61.E3: enforce non-writable descriptors per ECMA §10.1.9
        // OrdinarySet step 3 — assigning to a non-writable own data
        // property is a silent no-op (sloppy mode; strict mode throws,
        // but throwing is deferred since cruftless's strict-mode tracking
        // is incomplete). Object.freeze + the function-meta-props
        // (name/length descriptor non-writable) both depend on this.
        if let Some(d) = self.obj(id).properties.get(&key) {
            if !d.writable && d.getter.is_none() && d.setter.is_none() {
                return; // silent no-op for non-writable data property
            }
        }
        self.obj_mut(id).set_own(key, value);
    }

    /// Typeof with heap deref for Object/function discrimination.
    pub fn type_of_value(&self, v: &Value) -> &'static str {
        match v {
            Value::Object(id) => {
                let o = self.obj(*id);
                if matches!(o.internal_kind,
                    InternalKind::Function(_) | InternalKind::Closure(_) | InternalKind::BoundFunction(_))
                { "function" }
                else if let InternalKind::Proxy(p) = &o.internal_kind {
                    // Ω.5.P60.E3: Proxy's typeof reflects target's callability
                    // per ECMA §10.5 (proxy is callable iff target is callable).
                    self.type_of_value(&Value::Object(p.target))
                }
                else { "object" }
            }
            other => other.type_of(),
        }
    }

    /// Public wrapper: run a module-level Frame. Used by evaluate_module
    /// to drive bytecode execution while retaining access to the post-
    /// execution local-slot values.
    pub fn run_frame_module(&mut self, frame: &mut Frame) -> Result<Value, RuntimeError> {
        // Ω.5.P51.E1: propagate the URL onto module-level Frames so the
        // top-level run_module() entry can pass the URL through. Module
        // frames built via Frame::new_module have empty url by default;
        // evaluate_module sets it before invoking run_frame_module.
        self.run_frame(frame)
    }

    /// Execute a compiled module. Returns the terminal stack value (the
    /// last value on the operand stack at module exit) or Undefined.
    pub fn run_module(&mut self, m: &CompiledModule) -> Result<Value, RuntimeError> {
        let mut frame = Frame::new_module(m);
        self.run_frame(&mut frame)
    }

    fn run_frame(&mut self, frame: &mut Frame) -> Result<Value, RuntimeError> {
        // Outer driver: each iteration runs the inner dispatch; if a JS
        // throw bubbles up, walk the try_stack and either resume at a
        // catch handler or re-raise to the caller.
        loop {
            // Tier-Ω.5.mmmmmm: try/catch catches engine-side TypeError /
            // RangeError / ReferenceError, per ECMA-262 §13.15.
            // get-intrinsic (the es-shim ecosystem) intentionally throws
            // `null.error` to capture an Error instance — without this,
            // every es-abstract-using shim package fails at load.
            match self.run_frame_inner(frame) {
                Ok(v) => return Ok(v),
                Err(e) => {
                    // Ω.5.P51.E1: enrich runtime errors with file:line:col
                    // from the frame's source_map + line_starts. The faulting
                    // pc is just past the opcode that threw; the most recent
                    // source_map entry with offset <= pc names the span.
                    // We enrich once per error (idempotent via " at " marker)
                    // so re-throws through nested frames don't accumulate.
                    let e = enrich_with_source_pos(e, frame);
                    let (catchable_msg, catchable_name): (Option<String>, &str) = match &e {
                        RuntimeError::Thrown(_) => (None, ""),
                        RuntimeError::TypeError(m) => (Some(m.clone()), "TypeError"),
                        RuntimeError::RangeError(m) => (Some(m.clone()), "RangeError"),
                        RuntimeError::ReferenceError(m) => (Some(m.clone()), "ReferenceError"),
                        RuntimeError::SyntaxError(m) => (Some(m.clone()), "SyntaxError"),
                        _ => return Err(e),
                    };
                    if frame.try_stack.is_empty() { return Err(e); }
                    let v = if let RuntimeError::Thrown(v) = e { v } else {
                        let msg = catchable_msg.unwrap();
                        // Ω.5.P61.E4: route through make_error_instance so the
                        // thrown value's [[Prototype]] is the named ctor's
                        // .prototype. Without it, `e instanceof TypeError`
                        // returned false and test262's `assert.throws(TypeError,
                        // ...)` failed even when the engine threw TypeError.
                        let id_opt = crate::intrinsics::make_error_instance(
                            self, catchable_name, &msg);
                        match id_opt {
                            Some(id) => Value::Object(id),
                            None => {
                                // Bootstrap edge: ctor not yet installed. Fall
                                // back to bare ordinary object with name/message.
                                let mut o = crate::value::Object::new_ordinary();
                                if let Some(ep) = self.object_prototype { o.proto = Some(ep); }
                                o.set_own("message".into(), Value::String(std::rc::Rc::new(msg.clone())));
                                o.set_own("name".into(), Value::String(std::rc::Rc::new(catchable_name.into())));
                                o.set_own("stack".into(), Value::String(std::rc::Rc::new(format!("{}: {}", catchable_name, msg))));
                                Value::Object(self.alloc_object(o))
                            }
                        }
                    };
                    let t = frame.try_stack.pop().unwrap();
                    frame.operand_stack.truncate(t.sp_at_entry);
                    frame.operand_stack.push(v);
                    frame.pc = t.catch_offset;
                }
            }
        }
    }

    fn run_frame_inner(&mut self, frame: &mut Frame) -> Result<Value, RuntimeError> {
        loop {
            let pc = frame.pc;
            if pc >= frame.bytecode.len() {
                return Ok(self.last_value.clone());
            }
            let op_byte = frame.bytecode[pc];
            let op = op_from_byte(op_byte)
                .ok_or_else(|| RuntimeError::Unimplemented(format!("invalid opcode 0x{:02X} @{}", op_byte, pc)))?;
            frame.pc += 1;
            match op {
                // ─── Stack ops ───
                Op::PushNull => frame.push(Value::Null),
                Op::PushUndef => frame.push(Value::Undefined),
                Op::PushTrue => frame.push(Value::Boolean(true)),
                Op::PushFalse => frame.push(Value::Boolean(false)),
                Op::PushI32 => {
                    let v = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    frame.push(Value::Number(v as f64));
                }
                Op::PushConst => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let v = self.constant_to_value(frame, idx)?;
                    frame.push(v);
                }
                Op::Pop => { frame.pop()?; }
                Op::Dup => {
                    let top = frame.peek(0)?.clone();
                    frame.push(top);
                }
                Op::Swap => {
                    let len = frame.operand_stack.len();
                    if len < 2 { return Err(RuntimeError::TypeError("stack underflow on Swap".into())); }
                    frame.operand_stack.swap(len - 1, len - 2);
                }

                // ─── Variable / scope ───
                Op::LoadLocal => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let v = frame.read_local(slot);
                    // Tier-Ω.5.jj.diag: tag local-binding-name into the
                    // diagnostic stash so Op::Call's error includes which
                    // local was loaded. Compiler's local descriptor carries
                    // the source name; reuse it for error enrichment.
                    if slot < frame.bytecode.len() {
                        // frame.constants and frame.bytecode are slices; we
                        // need access to locals. The local name lives in
                        // CompiledModule.locals or FunctionProto.locals,
                        // both kept on the frame as &[LocalDescriptor] via
                        // the proto/module reference. Skip if unavailable.
                    }
                    // The frame.locals field is Vec<Value>, not descriptors.
                    // CompiledModule and FunctionProto carry the descriptors.
                    // Frame doesn't currently carry them; use the bytecode's
                    // owning structure via the constants pool name if needed
                    // — for now, just tag with the slot number.
                    let lname = frame.locals_names.get(slot).map(|d| d.name.clone()).unwrap_or_else(|| format!("<local${}>", slot));
                    frame.last_property_lookup = Some(lname);
                    frame.push(v);
                }
                Op::StoreLocal => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let v = frame.pop()?;
                    frame.write_local(slot, v);
                }
                Op::ResetLocalCell => {
                    // Detach any prior upvalue cell at this slot so the next
                    // CaptureLocal promotes to a fresh cell. Existing closures
                    // that already captured the previous cell retain their
                    // Rc<RefCell<Value>> handle — only the frame's binding to
                    // the cell is cleared. Tier-Ω.5.g.1 per-iteration binding.
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    if slot < frame.local_cells.len() {
                        frame.local_cells[slot] = None;
                    }
                }
                Op::LoadGlobal => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    // Ω.5.P55.E1: JS-visible globals first; engine_helpers
                    // is the unshadowed fallback for compiler-emitted
                    // lowerings (Doc 729 §VII.B).
                    let v = self.globals.get(&name).cloned()
                        .or_else(|| self.engine_helpers.get(&name).cloned())
                        .unwrap_or(Value::Undefined);
                    frame.last_property_lookup = Some(format!("<global>{}", name));
                    frame.push(v);
                }
                Op::StoreGlobal => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    let v = frame.pop()?;
                    self.globals.insert(name, v);
                }
                Op::LoadUpvalue => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let v = frame.upvalues.get(slot)
                        .map(|cell| cell.borrow().clone())
                        .unwrap_or(Value::Undefined);
                    // Tier-Ω.5.sssss: tag the upvalue name for callee
                    // diagnostics. Minified closures resolve callees via
                    // upvalues, so without this tag the trace lost the
                    // last identifier name before a failing call.
                    if let Some(desc) = frame.upvalue_names.get(slot) {
                        frame.last_property_lookup = Some(format!("^{}", desc.name));
                    }
                    frame.push(v);
                }
                Op::StoreUpvalue => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let v = frame.pop()?;
                    if let Some(cell) = frame.upvalues.get(slot) {
                        *cell.borrow_mut() = v;
                    } else {
                        // Out-of-range StoreUpvalue: shouldn't happen for
                        // well-formed bytecode. Extend with a fresh cell so
                        // a later LoadUpvalue at the same slot reads it back.
                        while frame.upvalues.len() <= slot { frame.upvalues.push(new_upvalue_cell(Value::Undefined)); }
                        *frame.upvalues[slot].borrow_mut() = v;
                    }
                }
                Op::CaptureLocal => {
                    // Promote outer-frame slot to a shared cell (idempotent),
                    // then push that cell's Rc into the closure's upvalues.
                    // Binding-shared semantics: outer-frame writes through
                    // the same cell, sibling closures share too.
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let cell = frame.promote_local(slot);
                    let top = match frame.peek(0)? {
                        Value::Object(id) => *id,
                        _ => return Err(RuntimeError::TypeError("CaptureLocal: top of stack is not a closure".into())),
                    };
                    if let InternalKind::Closure(c) = &mut self.obj_mut(top).internal_kind {
                        c.upvalues.push(cell);
                    } else {
                        return Err(RuntimeError::TypeError("CaptureLocal: top is not a closure".into()));
                    }
                }
                Op::CaptureUpvalue => {
                    // Transitive capture: share the Rc<RefCell<Value>> the
                    // enclosing closure already holds. Do NOT deep-copy the
                    // value out and re-wrap — that would break binding
                    // semantics across the three-deep nesting case.
                    let idx = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let cell = frame.upvalues.get(idx)
                        .cloned()
                        .unwrap_or_else(|| new_upvalue_cell(Value::Undefined));
                    let top = match frame.peek(0)? {
                        Value::Object(id) => *id,
                        _ => return Err(RuntimeError::TypeError("CaptureUpvalue: top is not a closure".into())),
                    };
                    if let InternalKind::Closure(c) = &mut self.obj_mut(top).internal_kind {
                        c.upvalues.push(cell);
                    } else {
                        return Err(RuntimeError::TypeError("CaptureUpvalue: top is not a closure".into()));
                    }
                }
                Op::DefineLocal => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    while frame.locals.len() <= slot { frame.locals.push(Value::Undefined); }
                }

                // ─── Arithmetic ───
                Op::Add => {
                    // Ω.5.P62.E21: route through op_add_rt so Object
                    // operands dispatch ToPrimitive per §13.15.4.
                    let r = frame.pop()?; let l = frame.pop()?;
                    let v = self.op_add_rt(&l, &r)?;
                    frame.push(v);
                }
                Op::Sub => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.sub(b))));
                    } else {
                        let r = self.to_num_coerced(&rv)?; let l = self.to_num_coerced(&lv)?;
                        frame.push(Value::Number(l - r));
                    }
                }
                Op::Mul => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.mul(b))));
                    } else {
                        let r = self.to_num_coerced(&rv)?; let l = self.to_num_coerced(&lv)?;
                        frame.push(Value::Number(l * r));
                    }
                }
                Op::Div => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.divmod(b) {
                            Some((q, _)) => frame.push(Value::BigInt(Rc::new(q))),
                            None => return Err(RuntimeError::TypeError("Division by zero".into())),
                        }
                    } else {
                        let r = self.to_num_coerced(&rv)?; let l = self.to_num_coerced(&lv)?;
                        frame.push(Value::Number(l / r));
                    }
                }
                Op::Mod => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.divmod(b) {
                            Some((_, r)) => frame.push(Value::BigInt(Rc::new(r))),
                            None => return Err(RuntimeError::TypeError("Division by zero".into())),
                        }
                    } else {
                        let r = self.to_num_coerced(&rv)?; let l = self.to_num_coerced(&lv)?;
                        frame.push(Value::Number(l % r));
                    }
                }
                Op::Pow => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.pow(b) {
                            Some(p) => frame.push(Value::BigInt(Rc::new(p))),
                            None => return Err(RuntimeError::TypeError("BigInt ** invalid exponent".into())),
                        }
                    } else {
                        let r = self.to_num_coerced(&rv)?; let l = self.to_num_coerced(&lv)?;
                        frame.push(Value::Number(l.powf(r)));
                    }
                }
                Op::Neg => {
                    let raw = frame.pop()?;
                    if let Value::BigInt(b) = &raw {
                        frame.push(Value::BigInt(Rc::new(b.neg())));
                    } else {
                        let v = self.to_num_coerced(&raw)?;
                        frame.push(Value::Number(-v));
                    }
                }
                Op::Pos => {
                    // Tier-Ω.5.jjjjj: unary `+` per ECMA-262 §13.5.4. When
                    // operand is an object, invoke its [Symbol.toPrimitive]
                    // / valueOf hook before to_number. Without this,
                    // `+new Date(...)` is NaN, which broke date-fns / luxon
                    // / dayjs and any lib that coerces a Date via +.
                    let raw = frame.pop()?;
                    let n = match raw {
                        Value::Object(id) => {
                            let v = self.object_get(id, "valueOf");
                            if matches!(v, Value::Object(_)) {
                                let r = self.call_function(v, Value::Object(id), Vec::new())?;
                                to_number(&r)
                            } else { to_number(&raw) }
                        }
                        _ => to_number(&raw),
                    };
                    frame.push(Value::Number(n));
                }
                Op::Inc => {
                    let raw = frame.pop()?;
                    if let Value::BigInt(b) = &raw {
                        frame.push(Value::BigInt(Rc::new(b.add(&crate::bigint::JsBigInt::one()))));
                    } else {
                        let v = to_number(&raw);
                        frame.push(Value::Number(v + 1.0));
                    }
                }
                Op::Dec => {
                    let raw = frame.pop()?;
                    if let Value::BigInt(b) = &raw {
                        frame.push(Value::BigInt(Rc::new(b.sub(&crate::bigint::JsBigInt::one()))));
                    } else {
                        let v = to_number(&raw);
                        frame.push(Value::Number(v - 1.0));
                    }
                }

                // ─── Comparison / equality ───
                Op::Eq => {
                    // Ω.5.P62.E21: route through is_loosely_equal_rt so
                    // Object/primitive comparison dispatches ToPrimitive
                    // per §7.2.13 step 12/13.
                    let r = frame.pop()?; let l = frame.pop()?;
                    let v = self.is_loosely_equal_rt(&l, &r)?;
                    frame.push(Value::Boolean(v));
                }
                Op::Ne => {
                    let r = frame.pop()?; let l = frame.pop()?;
                    let v = self.is_loosely_equal_rt(&l, &r)?;
                    frame.push(Value::Boolean(!v));
                }
                Op::StrictEq => {
                    let r = frame.pop()?; let l = frame.pop()?;
                    frame.push(Value::Boolean(is_strictly_equal(&l, &r)));
                }
                Op::StrictNe => {
                    let r = frame.pop()?; let l = frame.pop()?;
                    frame.push(Value::Boolean(!is_strictly_equal(&l, &r)));
                }
                Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                    let r = frame.pop()?; let l = frame.pop()?;
                    let ord = abstract_relational_compare(&l, &r);
                    let result = match op {
                        Op::Lt => matches!(ord, RelOrder::Less),
                        Op::Gt => matches!(ord, RelOrder::Greater),
                        Op::Le => matches!(ord, RelOrder::Less | RelOrder::Equal),
                        Op::Ge => matches!(ord, RelOrder::Greater | RelOrder::Equal),
                        _ => unreachable!(),
                    };
                    frame.push(Value::Boolean(result));
                }

                // ─── Bitwise / shift ───
                // Tier-Ω.5.JJJJJJJJ: all bitwise ops use ECMA-spec ToInt32 /
                // ToUint32. The Rust `n as i32` saturates for big f64 (e.g.
                // 7.2e16); spec-correct conversion is trunc-then-mod-2^32.
                // `n as i64 as i32` does exactly that (i64 holds the
                // truncated integer, i32 cast keeps lower 32 bits with
                // sign extension). bn.js's 26-bit limb arithmetic depended
                // on these being correct for big intermediate values.
                Op::BitAnd => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.bit_and(b))));
                    } else {
                        let r = to_number(&rv) as i64 as i32;
                        let l = to_number(&lv) as i64 as i32;
                        frame.push(Value::Number((l & r) as f64));
                    }
                }
                Op::BitOr => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.bit_or(b))));
                    } else {
                        let r = to_number(&rv) as i64 as i32;
                        let l = to_number(&lv) as i64 as i32;
                        frame.push(Value::Number((l | r) as f64));
                    }
                }
                Op::BitXor => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.bit_xor(b))));
                    } else {
                        let r = to_number(&rv) as i64 as i32;
                        let l = to_number(&lv) as i64 as i32;
                        frame.push(Value::Number((l ^ r) as f64));
                    }
                }
                Op::BitNot => {
                    let v = frame.pop()?;
                    if let Value::BigInt(b) = &v {
                        frame.push(Value::BigInt(Rc::new(b.bit_not())));
                    } else {
                        let n = to_number(&v) as i64 as i32;
                        frame.push(Value::Number((!n) as f64));
                    }
                }
                // Ω.5.P29.E1.bigint-shift-diag: tag the actual shift count
                // into the error so the fault is self-diagnosing per Doc 723
                // §IV.b. The pre-fix message "BigInt << invalid shift" gave
                // no signal about what `n` was.
                Op::Shl => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.shl(b) {
                            Some(p) => frame.push(Value::BigInt(Rc::new(p))),
                            None => return Err(RuntimeError::TypeError(format!(
                                "BigInt << invalid shift (lhs_bits={}, n={})",
                                a.mag_bit_len(), b.to_decimal()
                            ))),
                        }
                    } else {
                        let r = (to_number(&rv) as i64 as i32 as u32) & 0x1F;
                        let l = to_number(&lv) as i64 as i32;
                        frame.push(Value::Number((l.wrapping_shl(r)) as f64));
                    }
                }
                Op::Shr => {
                    let rv = frame.pop()?; let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.shr(b) {
                            Some(p) => frame.push(Value::BigInt(Rc::new(p))),
                            None => return Err(RuntimeError::TypeError(format!(
                                "BigInt >> invalid shift (lhs_bits={}, n={})",
                                a.mag_bit_len(), b.to_decimal()
                            ))),
                        }
                    } else {
                        let r = (to_number(&rv) as i64 as i32 as u32) & 0x1F;
                        let l = to_number(&lv) as i64 as i32;
                        frame.push(Value::Number((l >> r) as f64));
                    }
                }
                Op::UShr => {
                    // Tier-Ω.5.JJJJJJJJ: spec-correct ToUint32 per ECMA §7.1.7.
                    // Previously `n as u32` saturated for n >= 2^32, producing
                    // 0xFFFFFFFF for big f64s. Required: trunc-to-int64 then
                    // bit-cast to u32 (drops upper bits). bn.js's 26-bit limb
                    // arithmetic via `(x * y) >>> 0` was producing wrong limbs.
                    let r = (to_number(&frame.pop()?) as i64 as i32 as u32) & 0x1F;
                    let l = to_number(&frame.pop()?) as i64 as i32 as u32;
                    frame.push(Value::Number((l >> r) as f64));
                }

                // ─── Logical ───
                Op::Not => {
                    let v = to_boolean(&frame.pop()?);
                    frame.push(Value::Boolean(!v));
                }

                // ─── Unary type / void ───
                Op::Typeof => {
                    let v = frame.pop()?;
                    let t = self.type_of_value(&v);
                    frame.push(Value::String(Rc::new(t.to_string())));
                }
                Op::Void => {
                    let _ = frame.pop()?;
                    frame.push(Value::Undefined);
                }

                // ─── Control flow ───
                Op::Jump => {
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    frame.pc = (frame.pc as i32 + disp) as usize;
                }
                Op::JumpIfTrue => {
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    if to_boolean(&frame.pop()?) {
                        frame.pc = (frame.pc as i32 + disp) as usize;
                    }
                }
                Op::JumpIfFalse => {
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    if !to_boolean(&frame.pop()?) {
                        frame.pc = (frame.pc as i32 + disp) as usize;
                    }
                }
                Op::JumpIfTrueKeep => {
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    if to_boolean(frame.peek(0)?) {
                        frame.pc = (frame.pc as i32 + disp) as usize;
                    }
                }
                Op::JumpIfFalseKeep => {
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    if !to_boolean(frame.peek(0)?) {
                        frame.pc = (frame.pc as i32 + disp) as usize;
                    }
                }
                Op::JumpIfNullish => {
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    let v = frame.pop()?;
                    if matches!(v, Value::Undefined | Value::Null) {
                        frame.pc = (frame.pc as i32 + disp) as usize;
                    }
                }

                // ─── Exception handling (minimal in round 3.d.c) ───
                Op::Throw => {
                    let v = frame.pop()?;
                    return Err(RuntimeError::Thrown(v));
                }
                Op::TryEnter => {
                    // catch_offset is an absolute bytecode offset where
                    // the catch handler begins. Pushed onto frame.try_stack.
                    let catch_off = rusty_js_bytecode::op::decode_u32(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 4;
                    frame.try_stack.push(TryFrame {
                        catch_offset: catch_off,
                        sp_at_entry: frame.operand_stack.len(),
                    });
                }
                Op::TryExit => {
                    frame.try_stack.pop();
                }

                // ─── Returns ───
                Op::Return => {
                    let v = frame.pop()?;
                    self.last_value = v.clone();
                    return Ok(v);
                }
                Op::ReturnUndef => {
                    self.last_value = Value::Undefined;
                    return Ok(Value::Undefined);
                }

                // ─── Object / Array construction ───
                Op::NewObject => {
                    let id = self.alloc_object(Object::new_ordinary());
                    frame.push(Value::Object(id));
                }
                Op::NewArray => {
                    let _hint = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let id = self.alloc_object(Object::new_array());
                    frame.push(Value::Object(id));
                }
                Op::InitProp => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let key = self.constant_name(frame, idx)?;
                    let value = frame.pop()?;
                    let id = match frame.peek(0)? {
                        Value::Object(id) => *id,
                        _ => return Err(RuntimeError::TypeError("InitProp on non-object".into())),
                    };
                    self.object_set(id, key, value);
                }
                Op::InitIndex => {
                    let idx = rusty_js_bytecode::op::decode_u32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    let value = frame.pop()?;
                    let id = match frame.peek(0)? {
                        Value::Object(id) => *id,
                        _ => return Err(RuntimeError::TypeError("InitIndex on non-array".into())),
                    };
                    self.object_set(id, idx.to_string(), value);
                }

                // ─── Property access ───
                Op::GetProp => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let key = self.constant_name(frame, idx)?;
                    let obj_v = frame.pop()?;
                    // Tier-Ω.5.nnn: invoke accessor getter if present along
                    // the prototype chain. Captures the getter function +
                    // receiver before calling so re-entrant access works.
                    let v = match &obj_v {
                        Value::Object(id) => {
                            // Ω.5.P60.E1: Proxy get-trap dispatch. If obj
                            // is a Proxy and handler.get is callable, call
                            // handler.get(target, key, receiver) and use
                            // its return value. Missing trap falls through
                            // to target.
                            let proxy_dispatch = if let crate::value::InternalKind::Proxy(p) =
                                &self.obj(*id).internal_kind
                            {
                                Some((p.target, p.handler))
                            } else { None };
                            if let Some((target, handler)) = proxy_dispatch {
                                let trap = self.object_get(handler, "get");
                                if matches!(trap, Value::Object(_)) {
                                    let receiver = obj_v.clone();
                                    self.call_function(trap, Value::Object(handler), vec![
                                        Value::Object(target),
                                        Value::String(Rc::new(key.clone())),
                                        receiver,
                                    ])?
                                } else {
                                    self.object_get(target, &key)
                                }
                            } else {
                            // Tier-Ω.5.nnn: only check for accessor when the
                            // descriptor actually has one. Walking find_getter
                            // for every prop access has a cost; gate on
                            // direct-property existence first.
                            let has_accessor = {
                                let mut cur = Some(*id);
                                let mut found = false;
                                while let Some(c) = cur {
                                    if let Some(d) = self.obj(c).properties.get(&key) {
                                        if d.getter.is_some() { found = true; }
                                        break;
                                    }
                                    cur = self.obj(c).proto;
                                }
                                found
                            };
                            if has_accessor {
                                let getter = self.find_getter(*id, &key).unwrap();
                                self.call_function(getter, obj_v.clone(), Vec::new())?
                            } else {
                                self.object_get(*id, &key)
                            }
                            }
                        }
                        Value::String(s) if key == "length" => Value::Number(s.chars().count() as f64),
                        Value::String(_) => {
                            // Primitive string method auto-boxing: route to
                            // %String.prototype% if installed.
                            if let Some(proto) = self.string_prototype {
                                self.object_get(proto, &key)
                            } else { Value::Undefined }
                        }
                        Value::Number(_) => {
                            if let Some(proto) = self.number_prototype {
                                self.object_get(proto, &key)
                            } else { Value::Undefined }
                        }
                        Value::BigInt(_) => {
                            if let Some(proto) = self.bigint_prototype {
                                self.object_get(proto, &key)
                            } else { Value::Undefined }
                        }
                        Value::Undefined | Value::Null => {
                            // Tier-Ω.5.uuu: enrich the fault with the
                            // last LoadLocal/GetProp hint. Doc 723's
                            // threshold-of-diagnostic-semanticity finding
                            // (2026-05-15) named that single-tag faults
                            // are below-threshold for Layer-D bisect.
                            // Adding the source-side name of the value
                            // that resolved to undefined raises the
                            // signal level — `(receiver='X')` tags the
                            // local whose load preceded this access.
                            let receiver_hint = frame.last_property_lookup.clone()
                                .map(|s| format!(" (receiver='{}')", s))
                                .unwrap_or_default();
                            return Err(RuntimeError::TypeError(
                                format!("Cannot read property '{}' of {}{}", key,
                                    if matches!(obj_v, Value::Undefined) { "undefined" } else { "null" },
                                    receiver_hint)
                            ));
                        }
                        _ => Value::Undefined,
                    };
                    frame.last_property_lookup = Some(key.clone());
                    // Tier-Ω.5.yyyyy: pending_method survives arg loads.
                    frame.pending_method_name = Some(key);
                    frame.push(v);
                }
                Op::SetProp => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let key = self.constant_name(frame, idx)?;
                    let value = frame.pop()?;
                    let obj_v = frame.pop()?;
                    if let Value::Object(id) = &obj_v {
                        // Ω.5.P60.E2: Proxy set-trap dispatch.
                        let proxy_dispatch = if let crate::value::InternalKind::Proxy(p) =
                            &self.obj(*id).internal_kind
                        {
                            Some((p.target, p.handler))
                        } else { None };
                        if let Some((target, handler)) = proxy_dispatch {
                            let trap = self.object_get(handler, "set");
                            if matches!(trap, Value::Object(_)) {
                                self.call_function(trap, Value::Object(handler), vec![
                                    Value::Object(target),
                                    Value::String(Rc::new(key.clone())),
                                    value.clone(),
                                    Value::Object(*id),
                                ])?;
                            } else {
                                self.object_set(target, key, value.clone());
                            }
                        } else
                        // Tier-Ω.5.vvvv: same setter dispatch on identifier-
                        // keyed writes.
                        if let Some(setter) = self.find_setter(*id, &key) {
                            self.call_function(setter, Value::Object(*id), vec![value.clone()])?;
                        } else {
                            self.object_set(*id, key, value.clone());
                        }
                    } else {
                        // Tier-Ω.5.HHHHHHHH: enrich the non-object target tag
                        // with the last property-lookup hint per Doc 723 §IV.b
                        // route-(b). Bare `null.foo = bar` previously surfaced
                        // 'SetProp foo on non-object (null)'; with the
                        // receiver-hint the chain points at the upstream
                        // local that resolved to null.
                        let target_hint = frame.last_property_lookup.clone()
                            .map(|s| format!(" (target='{}')", s))
                            .unwrap_or_default();
                        return Err(RuntimeError::TypeError(
                            format!("SetProp '{}' on non-object ({}){}", key,
                                match &obj_v {
                                    Value::Undefined => "undefined",
                                    Value::Null => "null",
                                    Value::Boolean(_) => "boolean",
                                    Value::Number(_) => "number",
                                    Value::String(_) => "string",
                                    _ => "other",
                                },
                                target_hint,
                            )
                        ));
                    }
                    frame.push(value);
                }
                Op::GetIndex => {
                    let key_v = frame.pop()?;
                    let obj_v = frame.pop()?;
                    let key = property_key(&key_v);
                    let v = match obj_v {
                        // Tier-Ω.5.uuuu: dispatch accessor getters from
                        // computed-key reads. Op::GetProp already did this
                        // (Ω.5.nnn); Op::GetIndex did not, so non-identifier
                        // keys ("~standard", "with space") bypassed lazy
                        // accessors installed via Object.defineProperty.
                        // zod's defineLazy install on inst["~standard"] is
                        // the load-bearing case — without this dispatch,
                        // ZodType.init's Object.assign(inst["~standard"],
                        // {jsonSchema:...}) got undefined and bailed out
                        // of every schema-construction path.
                        Value::Object(id) => {
                            // Ω.5.P60.E1: Proxy get-trap dispatch at
                            // computed-key reads. Mirrors Op::GetProp.
                            let proxy_dispatch = if let crate::value::InternalKind::Proxy(p) =
                                &self.obj(id).internal_kind
                            {
                                Some((p.target, p.handler))
                            } else { None };
                            if let Some((target, handler)) = proxy_dispatch {
                                let trap = self.object_get(handler, "get");
                                if matches!(trap, Value::Object(_)) {
                                    self.call_function(trap, Value::Object(handler), vec![
                                        Value::Object(target),
                                        Value::String(Rc::new(key.clone())),
                                        Value::Object(id),
                                    ])?
                                } else {
                                    self.object_get(target, &key)
                                }
                            } else if let Some(getter) = self.find_getter(id, &key) {
                                self.call_function(getter, Value::Object(id), Vec::new())?
                            } else {
                                self.object_get(id, &key)
                            }
                        },
                        Value::String(s) => {
                            if let Ok(i) = key.parse::<usize>() {
                                s.chars().nth(i)
                                    .map(|c| Value::String(Rc::new(c.to_string())))
                                    .unwrap_or(Value::Undefined)
                            } else if key == "length" {
                                Value::Number(s.chars().count() as f64)
                            } else {
                                // Ω.5.P59.E5: primitive-string method auto-boxing
                                // via String.prototype for computed-key reads
                                // (mirrors Op::GetProp's branch). Pre-P59.E5
                                // `""[Symbol.iterator]` returned undefined
                                // because GetIndex didn't consult the
                                // prototype chain. After P59.E1 made well-
                                // known Symbols real Value::Symbol values,
                                // 8 packages started failing here at module-
                                // init (es-abstract, sinon, superagent,
                                // supertest, strapi, keystone, pug, express-
                                // promise-router) — they iterate over empty
                                // strings via the iterator protocol.
                                if let Some(proto) = self.string_prototype {
                                    self.object_get(proto, &key)
                                } else { Value::Undefined }
                            }
                        }
                        Value::Number(_) => {
                            // Mirror GetProp's number auto-box.
                            if let Some(proto) = self.number_prototype {
                                self.object_get(proto, &key)
                            } else { Value::Undefined }
                        }
                        Value::BigInt(_) => {
                            if let Some(proto) = self.bigint_prototype {
                                self.object_get(proto, &key)
                            } else { Value::Undefined }
                        }
                        Value::Undefined | Value::Null =>
                            return Err(RuntimeError::TypeError("Cannot index undefined/null".into())),
                        _ => Value::Undefined,
                    };
                    // Tier-Ω.5.yyyyy: tag the computed-key read so method
                    // diagnostics name the key. Mirrors GetProp's tagging.
                    frame.last_property_lookup = Some(key.clone());
                    frame.pending_method_name = Some(format!("[{}]", key));
                    frame.push(v);
                }
                Op::SetPrototype => {
                    // Pop [target, proto]; proto on top.
                    let proto_v = frame.pop()?;
                    let target_v = frame.pop()?;
                    let target_id = match target_v {
                        Value::Object(id) => id,
                        // Tier-Ω.5.ll: lenient — non-object target is a
                        // no-op rather than a throw. Packages doing
                        // duck-type-guarded setPrototypeOf rely on this.
                        _ => continue,
                    };
                    let new_proto = match proto_v {
                        Value::Object(id) => Some(id),
                        Value::Null => None,
                        // Tier-Ω.5.ll: lenient — undefined / primitive proto
                        // treated as "leave target's prototype unchanged"
                        // (matches the dominant package idiom where
                        // `class B extends X` with X undefined wants
                        // class-without-parent rather than crash).
                        _ => { let _ = target_id; continue; }
                    };
                    self.obj_mut(target_id).proto = new_proto;
                }
                Op::Delete => {
                    // `delete expr` per ECMA-262 §13.5.1. Pops the
                    // operand; v1 returns true (matches spec for any
                    // non-Reference operand). For `delete obj.prop` and
                    // `delete obj[key]` the compiler now emits DeleteProp /
                    // DeleteIndex instead — see Tier-Ω.5.BBBBBBBB.
                    let _ = frame.pop()?;
                    frame.push(Value::Boolean(true));
                }
                Op::DeleteProp => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let obj_v = frame.pop()?;
                    let key = match frame.constants.get(idx) {
                        Some(rusty_js_bytecode::Constant::String(s)) => s.clone(),
                        _ => return Err(RuntimeError::TypeError("Op::DeleteProp: key not String constant".into())),
                    };
                    let removed = match obj_v {
                        Value::Object(id) => {
                            // Ω.5.P60.E2: Proxy deleteProperty-trap dispatch.
                            let proxy_dispatch = if let crate::value::InternalKind::Proxy(p) =
                                &self.obj(id).internal_kind
                            {
                                Some((p.target, p.handler))
                            } else { None };
                            if let Some((target, handler)) = proxy_dispatch {
                                let trap = self.object_get(handler, "deleteProperty");
                                if matches!(trap, Value::Object(_)) {
                                    let r = self.call_function(trap, Value::Object(handler), vec![
                                        Value::Object(target),
                                        Value::String(Rc::new(key.clone())),
                                    ])?;
                                    crate::abstract_ops::to_boolean(&r)
                                } else {
                                    self.obj_mut(target).properties.shift_remove(&key).is_some()
                                }
                            } else {
                                // Ω.5.P62.E10: ECMA §10.1.10 OrdinaryDelete —
                                // own data property with configurable:false is
                                // not deletable. Return false (sloppy mode);
                                // strict mode throws but cruftless's strict
                                // tracking is incomplete (parity with sloppy
                                // delete semantics in P61.E3).
                                if let Some(d) = self.obj(id).properties.get(&key) {
                                    if !d.configurable {
                                        false
                                    } else {
                                        self.obj_mut(id).properties.shift_remove(&key).is_some()
                                    }
                                } else {
                                    true
                                }
                            }
                        }
                        _ => false,
                    };
                    frame.push(Value::Boolean(removed));
                }
                Op::DeleteIndex => {
                    let key_v = frame.pop()?;
                    let obj_v = frame.pop()?;
                    let key = crate::abstract_ops::to_string(&key_v).as_str().to_string();
                    let removed = match obj_v {
                        Value::Object(id) => {
                            // Ω.5.P60.E2: same Proxy dispatch as DeleteProp.
                            let proxy_dispatch = if let crate::value::InternalKind::Proxy(p) =
                                &self.obj(id).internal_kind
                            {
                                Some((p.target, p.handler))
                            } else { None };
                            if let Some((target, handler)) = proxy_dispatch {
                                let trap = self.object_get(handler, "deleteProperty");
                                if matches!(trap, Value::Object(_)) {
                                    let r = self.call_function(trap, Value::Object(handler), vec![
                                        Value::Object(target),
                                        Value::String(Rc::new(key.clone())),
                                    ])?;
                                    crate::abstract_ops::to_boolean(&r)
                                } else {
                                    self.obj_mut(target).properties.shift_remove(&key).is_some()
                                }
                            } else {
                                // Ω.5.P62.E10: §10.1.10 non-configurable guard.
                                if let Some(d) = self.obj(id).properties.get(&key) {
                                    if !d.configurable { false }
                                    else { self.obj_mut(id).properties.shift_remove(&key).is_some() }
                                } else { true }
                            }
                        }
                        _ => false,
                    };
                    frame.push(Value::Boolean(removed));
                }
                Op::In => {
                    // pops [key, obj]; obj on top per BinaryOp::In emit.
                    // `key in obj` per ECMA-262 §13.10: obj must be Object;
                    // otherwise TypeError. Returns true if the key (own or
                    // prototype-chain) exists; walks the prototype chain.
                    let obj_v = frame.pop()?;
                    let key_v = frame.pop()?;
                    let obj_id = match obj_v {
                        Value::Object(id) => id,
                        _ => {
                            // Tier-Ω.5.dddd: enrich with last-property-lookup
                            // hint per Doc 723's route-(b) discipline.
                            let hint = frame.last_property_lookup.clone()
                                .map(|s| format!(" (rhs='{}')", s)).unwrap_or_default();
                            let key_s = match &key_v {
                                Value::String(s) => format!("'{}'", s.as_str()),
                                _ => format!("{:?}", key_v),
                            };
                            return Err(RuntimeError::TypeError(
                                format!("Cannot use 'in' operator on non-object: {} in {:?}{}", key_s, obj_v, hint)));
                        }
                    };
                    let key = property_key(&key_v);
                    // Ω.5.P60.E2: Proxy has-trap dispatch.
                    let proxy_dispatch = if let crate::value::InternalKind::Proxy(p) =
                        &self.obj(obj_id).internal_kind
                    {
                        Some((p.target, p.handler))
                    } else { None };
                    let mut found = false;
                    if let Some((target, handler)) = proxy_dispatch {
                        let trap = self.object_get(handler, "has");
                        if matches!(trap, Value::Object(_)) {
                            let r = self.call_function(trap, Value::Object(handler), vec![
                                Value::Object(target),
                                Value::String(Rc::new(key.clone())),
                            ])?;
                            found = crate::abstract_ops::to_boolean(&r);
                        } else {
                            let mut cur = Some(target);
                            while let Some(c) = cur {
                                if self.obj(c).properties.contains_key(&key) { found = true; break; }
                                cur = self.obj(c).proto;
                            }
                        }
                    } else {
                    let mut cur = Some(obj_id);
                    while let Some(c) = cur {
                        if self.obj(c).properties.contains_key(&key) { found = true; break; }
                        cur = self.obj(c).proto;
                    }
                    }
                    frame.push(Value::Boolean(found));
                }
                Op::Instanceof => {
                    // pops [obj, ctor]; ctor on top.
                    let ctor_v = frame.pop()?;
                    let obj_v = frame.pop()?;
                    // Tier-Ω.5.hhhhhh: dispatch ctor[Symbol.hasInstance] when
                    // present per ECMA-262 §13.10.1 step 4. readable-stream's
                    // Writable customizes hasInstance for the Duplex inheritance
                    // shape; without dispatch, every `obj instanceof Writable`
                    // hits the fallback proto-chain check that always returns
                    // false for the userspace Writable.
                    let hi_result = if let Value::Object(ctor_id) = &ctor_v {
                        // Symbol.hasInstance is interned as "@@sym:0:hasInstance"
                        // in our engine. Try a few keys for compatibility.
                        let hi = self.object_get(*ctor_id, "Symbol(Symbol.hasInstance)");
                        let hi = if matches!(hi, Value::Undefined) {
                            self.object_get(*ctor_id, "@@hasInstance")
                        } else { hi };
                        let hi = if matches!(hi, Value::Undefined) {
                            // Try Symbol.hasInstance's string form.
                            let sym = self.globals.get("Symbol").cloned();
                            if let Some(Value::Object(sym_id)) = sym {
                                let hi_sym = self.object_get(sym_id, "hasInstance");
                                if let Value::String(s) = hi_sym {
                                    self.object_get(*ctor_id, &s)
                                } else { Value::Undefined }
                            } else { Value::Undefined }
                        } else { hi };
                        if matches!(hi, Value::Object(_)) {
                            let r = self.call_function(hi, ctor_v.clone(), vec![obj_v.clone()])?;
                            Some(matches!(r, Value::Boolean(true)) || (!matches!(r, Value::Boolean(false) | Value::Undefined | Value::Null) && match &r { Value::Number(n) => *n != 0.0, Value::String(s) => !s.is_empty(), _ => true }))
                        } else { None }
                    } else { None };
                    let result = if let Some(b) = hi_result { b } else {
                        match (&obj_v, &ctor_v) {
                            (Value::Object(obj_id), Value::Object(ctor_id)) => {
                                let proto_v = self.object_get(*ctor_id, "prototype");
                                match proto_v {
                                    Value::Object(target_proto) => {
                                        let mut cur = self.obj(*obj_id).proto;
                                        let mut found = false;
                                        while let Some(c) = cur {
                                            if c == target_proto { found = true; break; }
                                            cur = self.obj(c).proto;
                                        }
                                        found
                                    }
                                    _ => false,
                                }
                            }
                            _ => false,
                        }
                    };
                    frame.push(Value::Boolean(result));
                }
                Op::SetIndex => {
                    let value = frame.pop()?;
                    let key_v = frame.pop()?;
                    let obj_v = frame.pop()?;
                    let key = property_key(&key_v);
                    if let Value::Object(id) = &obj_v {
                        // Ω.5.P60.E2: Proxy set-trap dispatch at computed-key writes.
                        let proxy_dispatch = if let crate::value::InternalKind::Proxy(p) =
                            &self.obj(*id).internal_kind
                        {
                            Some((p.target, p.handler))
                        } else { None };
                        if let Some((target, handler)) = proxy_dispatch {
                            let trap = self.object_get(handler, "set");
                            if matches!(trap, Value::Object(_)) {
                                self.call_function(trap, Value::Object(handler), vec![
                                    Value::Object(target),
                                    Value::String(Rc::new(key.clone())),
                                    value.clone(),
                                    Value::Object(*id),
                                ])?;
                            } else {
                                self.object_set(target, key, value.clone());
                            }
                        } else
                        // Tier-Ω.5.vvvv: dispatch accessor setters, mirror of
                        // Ω.5.uuuu for GetIndex. Without this, writes through
                        // computed keys to lazy-defined properties silently
                        // overwrite the descriptor's getter with a data slot.
                        if let Some(setter) = self.find_setter(*id, &key) {
                            self.call_function(setter, Value::Object(*id), vec![value.clone()])?;
                        } else {
                            self.object_set(*id, key, value.clone());
                        }
                    } else {
                        // Tier-Ω.5.HHHHHHHH: route-(b) enrichment. mobx-state-tree
                        // and similar libs surfaced opaque 'SetIndex on non-object'
                        // — adding the key + target value-shape + last-property
                        // hint names the source-side gap.
                        let kind = match &obj_v {
                            Value::Undefined => "undefined",
                            Value::Null => "null",
                            Value::Boolean(_) => "boolean",
                            Value::Number(_) => "number",
                            Value::String(_) => "string",
                            _ => "other",
                        };
                        let target_hint = frame.last_property_lookup.clone()
                            .map(|s| format!(" (target='{}')", s))
                            .unwrap_or_default();
                        return Err(RuntimeError::TypeError(
                            format!("SetIndex '{}' on non-object ({}){}", key, kind, target_hint)
                        ));
                    }
                    frame.push(value);
                }

                // ─── Closure construction ───
                Op::MakeClosure | Op::MakeArrow => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let proto = match frame.constants.get(idx) {
                        Some(rusty_js_bytecode::Constant::Function(p)) => p.clone(),
                        _ => return Err(RuntimeError::TypeError("MakeClosure constant is not a function".into())),
                    };
                    let is_arrow = matches!(op, Op::MakeArrow);
                    let proto_rc = Rc::new(*proto);
                    // Tier-Ω.5.P15.E1: function .length per ECMA-262 §10.2.10
                    // is the count of params before the first rest/default;
                    // the compiler precomputes this as proto.function_length.
                    let fn_length = proto_rc.function_length;
                    let display_name = proto_rc.display_name.clone();
                    let is_async = proto_rc.is_async;
                    let is_gen = proto_rc.is_generator;
                    // Tier-Ω.5.sss: arrow inherits `this` from current
                    // frame. Capture at MakeArrow time so the arrow's
                    // call_function ignores its receiver argument and
                    // uses this captured value instead.
                    let bound_this = if is_arrow { Some(frame.this_value.clone()) } else { None };
                    let closure = Object {
                        proto: None,
                        extensible: true,
                        properties: indexmap::IndexMap::new(),
                        internal_kind: crate::value::InternalKind::Closure(crate::value::ClosureInternals {
                            proto: proto_rc,
                            upvalues: Vec::new(),
                            is_arrow,
                            bound_this,
                        }),
                    };
                    let id = self.alloc_object(closure);
                    // Tier-Ω.5.P15.E1: install spec-mandated .name + .length
                    // own properties per ECMA-262 §10.2.9 + §10.2.10. Both
                    // are {writable:false, enumerable:false, configurable:true}
                    // — non-enumerable so Object.keys filters them out (the
                    // ms-class default-fn-export probe was missing both),
                    // configurable so Object.defineProperty can rewrite them.
                    {
                        let props = &mut self.obj_mut(id).properties;
                        crate::value::install_function_meta_props(props, &display_name, fn_length as f64);
                    }
                    // Tier-Ω.5.ll: auto-create .prototype on non-arrow,
                    // non-async, non-generator functions per ECMA-262
                    // §10.2.5 (regular functions have [[ConstructorKind]]:
                    // Base). chalk + many other packages rely on
                    // `function F() {}; F.prototype.X = ...` — without auto-
                    // creation, F.prototype is undefined.
                    //
                    // Ω.5.P50.E1: async functions per §15.7.5 do NOT have a
                    // .prototype slot. Previously rusty-js gave them one,
                    // which leaked into the CJS-as-ESM namespace as a
                    // spurious `prototype` key (prompts, fast-glob, ioredis,
                    // proper-lockfile, @databases/sql, write-file-atomic
                    // pattern: `module.exports = async function name(){}`
                    // with named exports attached). Bun strips it because
                    // it isn't actually an own property of the function.
                    if !is_arrow && !is_async && !is_gen {
                        let mut proto_obj = Object::new_ordinary();
                        // Ω.5.P61.E19 (revised by E20): constructor backlink
                        // is { writable:true, enumerable:false, configurable:true } per §10.2.4.
                        proto_obj.set_own_internal("constructor".into(), Value::Object(id));
                        let proto_id = self.alloc_object(proto_obj);
                        // Ω.5.P62.E7: user-function .prototype is per ECMA §10.2.4
                        // { writable:true, enumerable:false, configurable:false }.
                        // Pre-E7 used set_own_frozen (P61.E20 collateral) which
                        // made .prototype non-writable, so `F.prototype = X`
                        // silently no-op'd and Con.prototype-style inheritance
                        // broke. Built-in ctor .prototype stays frozen via
                        // the intrinsics-side install (P61.E20).
                        self.obj_mut(id).properties.insert("prototype".into(),
                            crate::value::PropertyDescriptor {
                                value: Value::Object(proto_id),
                                writable: true, enumerable: false, configurable: false,
                                getter: None, setter: None,
                            });
                    }
                    frame.push(Value::Object(id));
                }

                // ─── Function call ───
                Op::Call => {
                    let n = frame.bytecode[frame.pc] as usize;
                    frame.pc += 1;
                    let mut args = Vec::with_capacity(n);
                    for _ in 0..n {
                        args.push(frame.pop()?);
                    }
                    args.reverse();
                    let callee = frame.pop()?;
                    let callee_hint = frame.last_property_lookup.clone();
                    frame.pending_method_name = None;
                    // Tier-Ω.5.CCCCCCCC: also capture the callee's *value shape*
                    // before invoking. The 'callee is not callable' tag previously
                    // named the upstream local but not what it resolved to.
                    // Per Doc 726 §III's probe-shape taxonomy, the residual long-
                    // tail decomposes by callee-value-shape (Object{keys=[...]}
                    // = bundle-internal namespace wrapper / String("...") =
                    // primitive-as-callee bug / Function = receiver-aware
                    // ctor case / etc.) — naming the shape at the engine site
                    // raises the signal level for every future failure here.
                    let callee_tag = describe_value_for_diag(self, &callee);
                    let result = self.call_function(callee, Value::Undefined, args).map_err(|e| match e {
                        RuntimeError::TypeError(msg) if msg.starts_with("callee is not callable") => {
                            RuntimeError::TypeError(format!(
                                "{} (callee='{}') (callee_val={})",
                                msg,
                                callee_hint.unwrap_or_else(|| "?".into()),
                                callee_tag,
                            ))
                        }
                        // Tier-Ω.5.ssss: route-(b) escalation per Doc 721 §VI.6.
                        // Native callees that throw TypeError without naming a
                        // call-site stay below Doc 723 Layer-B semanticity. Append
                        // the resolved LoadGlobal/LoadLocal hint so the chain
                        // points to the actual upstream undefined.
                        RuntimeError::TypeError(msg) if callee_hint.is_some() => {
                            RuntimeError::TypeError(format!("{} (in-call='{}')", msg, callee_hint.unwrap()))
                        }
                        other => other,
                    })?;
                    frame.push(result);
                }
                Op::CallMethod => {
                    let n = frame.bytecode[frame.pc] as usize;
                    frame.pc += 1;
                    let mut args = Vec::with_capacity(n);
                    for _ in 0..n {
                        args.push(frame.pop()?);
                    }
                    args.reverse();
                    let method = frame.pop()?;
                    let receiver = frame.pop()?;
                    // Tier-Ω.5.yyyyy: prefer pending_method_name (captured
                    // at GetProp time) over last_property_lookup (which may
                    // have been overwritten by arg-load).
                    let method_name = frame.pending_method_name.take()
                        .or_else(|| frame.last_property_lookup.clone());
                    // Ω.5.P54.E4/E5 (Axis-S + Axis-H probe population):
                    // when the resolved method is Undefined, record the
                    // miss against the appropriate trace. Symbol.X-keyed
                    // lookups go to symbol_lookup_miss_log; node:* host
                    // namespace lookups go to host_stub_miss_log. The
                    // discriminator is the method name shape.
                    if matches!(method, Value::Undefined) {
                        if let Some(name) = method_name.as_deref() {
                            if name.starts_with("@@sym:Symbol.") || name.starts_with("@@") {
                                let entry = format!("{} on receiver={}",
                                    name,
                                    describe_value_for_diag(self, &receiver));
                                if !self.symbol_lookup_miss_log.contains(&entry) {
                                    self.symbol_lookup_miss_log.push(entry);
                                }
                            } else if let Value::Object(r_id) = &receiver {
                                // Check if receiver is from a node:* namespace
                                // by inspecting whether it has the global-namespace
                                // shape we install for host stubs.
                                let is_likely_host_stub = {
                                    let o = self.obj(*r_id);
                                    matches!(o.internal_kind, crate::value::InternalKind::Ordinary | crate::value::InternalKind::ModuleNamespace) &&
                                        o.properties.keys().any(|k| k.starts_with("__"))
                                };
                                if is_likely_host_stub {
                                    let entry = format!("missing method '{}'", name);
                                    if !self.host_stub_miss_log.contains(&entry) {
                                        self.host_stub_miss_log.push(entry);
                                    }
                                }
                            }
                        }
                    }
                    // Tier-Ω.5.MMMMMMM: route-(b) escalation per Doc 723 §IV.b.
                    // When the method lookup yields undefined and the method
                    // name is itself uninformative (e.g. '@@iterator' — Symbol-
                    // keyed protocol probe, where the receiver matters more
                    // than the method name), the fault tag is below Doc 723's
                    // threshold of diagnostic semanticity. Pre-compute a
                    // receiver-shape tag at this engine site so the bisect
                    // has Layer-B context. Compounds across every CallMethod
                    // failure at this site.
                    let receiver_tag = describe_value_for_diag(self, &receiver);
                    // Tier-Ω.5.P24.E1.proto-chain-walk: when the method lookup
                    // yielded a non-callable value, pre-compute the prototype
                    // chain walk for the same key so the fault tag names
                    // exactly which prototype link is missing the slot. Doc 723
                    // route-(b) compounding — one engine-site enrichment that
                    // pays off across every "callee is not callable" with a
                    // method name. Cheap (only fires on the error path); names
                    // the missing intrinsic by structural walk rather than
                    // forcing a manual chain-walk per debug round.
                    let chain_tag = if matches!(&method, Value::Undefined | Value::Null) {
                        method_name.as_deref().map(|mn| describe_proto_chain_for_key(self, &receiver, mn))
                    } else { None };
                    let result = self.call_function(method, receiver, args).map_err(|e| match e {
                        RuntimeError::TypeError(msg) if msg.starts_with("callee is not callable") => {
                            let chain_suffix = chain_tag.as_ref()
                                .map(|c| format!(" (proto-chain='{}')", c))
                                .unwrap_or_default();
                            RuntimeError::TypeError(format!(
                                "{} (method='{}') (receiver={}){}",
                                msg,
                                method_name.unwrap_or_else(|| "?".into()),
                                receiver_tag,
                                chain_suffix,
                            ))
                        }
                        // Tier-Ω.5.ssss: same route-(b) escalation for method-
                        // dispatch. Native methods like Object.assign throw with
                        // no upstream context; tag with the resolved method name.
                        RuntimeError::TypeError(msg) if method_name.is_some() => {
                            RuntimeError::TypeError(format!("{} (in-method='{}')", msg, method_name.unwrap()))
                        }
                        other => other,
                    })?;
                    frame.push(result);
                }
                Op::PushThis => {
                    let t = frame.this_value.clone();
                    frame.push(t);
                }
                Op::PushImportMeta => {
                    // Tier-Ω.5.r: read the per-module synthetic object.
                    // Falls back to Undefined for frames the module loader
                    // didn't populate.
                    let v = match frame.import_meta {
                        Some(oid) => Value::Object(oid),
                        None => Value::Undefined,
                    };
                    frame.push(v);
                }
                Op::PushNewTarget => {
                    // Tier-Ω.5.s: read the per-frame new.target. Populated
                    // by Op::New before dispatching the constructor call;
                    // left None for plain Call frames (yields Undefined).
                    let v = frame.new_target.clone().unwrap_or(Value::Undefined);
                    frame.push(v);
                }
                Op::SetThis => {
                    // Tier-Ω.5.nnnnn: rebind this when super(...) returns
                    // an Object. Pops the top of stack; if Object, replaces
                    // this_value; otherwise leaves this_value unchanged.
                    let v = frame.pop()?;
                    if matches!(&v, Value::Object(_)) {
                        frame.this_value = v;
                    }
                }
                Op::New => {
                    let n = frame.bytecode[frame.pc] as usize;
                    frame.pc += 1;
                    let mut args = Vec::with_capacity(n);
                    for _ in 0..n {
                        args.push(frame.pop()?);
                    }
                    args.reverse();
                    let callee = frame.pop()?;
                    // Ω.5.P61.E4: enforce [[Construct]] per ECMA §10.3.3 +
                    // EvaluateNew step 7 (IsConstructor check). Native
                    // functions marked is_constructor=false (Math.abs,
                    // Object.keys, String.prototype.includes, etc.) throw
                    // TypeError on `new fn()`.
                    if let Value::Object(cid) = &callee {
                        if let crate::value::InternalKind::Function(fi) =
                            &self.obj(*cid).internal_kind
                        {
                            if !fi.is_constructor {
                                return Err(RuntimeError::TypeError(format!(
                                    "{} is not a constructor", fi.name
                                )));
                            }
                        }
                    }
                    // Tier-Ω.5.f: consult callee.prototype property to set
                    // the new instance's [[Prototype]]. This is the load-
                    // bearing engine change that makes user-defined classes
                    // (whose prototypes are ordinary objects with method
                    // properties, not intrinsic prototypes) work with `new`.
                    let proto_override = if let Value::Object(cid) = &callee {
                        match self.object_get(*cid, "prototype") {
                            Value::Object(pid) => Some(pid),
                            _ => None,
                        }
                    } else { None };
                    let mut ordinary = Object::new_ordinary();
                    if proto_override.is_some() {
                        ordinary.proto = proto_override;
                    }
                    let this_id = self.alloc_object(ordinary);
                    let this_obj = Value::Object(this_id);
                    // Tier-Ω.5.s: mark this dispatch as a `new` call. The
                    // pending slot is consumed by call_function when
                    // constructing the inner frame (or the native call's
                    // current_new_target).
                    self.pending_new_target = Some(callee.clone());
                    let callee_hint = frame.last_property_lookup.clone();
                    // Tier-Ω.5.CCCCCCCC: also capture the new-callee's value
                    // shape (mirrors Ω.5.hhhh's name-only tag with §III.d
                    // dispatch-fingerprint shape info per Doc 726).
                    let new_callee_tag = describe_value_for_diag(self, &callee);
                    let ret = self.call_function(callee, this_obj.clone(), args).map_err(|e| match e {
                        RuntimeError::TypeError(msg) if msg.starts_with("callee is not callable") => {
                            // Tier-Ω.5.hhhh: Op::New now appends the
                            // LoadGlobal/LoadLocal hint per Doc 723
                            // route-(b). Before, bare `new X()` with X
                            // undefined produced unannotated 'callee is
                            // not callable: undefined' (below threshold).
                            RuntimeError::TypeError(format!(
                                "{} (new-callee='{}') (new_val={})",
                                msg,
                                callee_hint.unwrap_or_else(|| "?".into()),
                                new_callee_tag,
                            ))
                        }
                        other => other,
                    })?;
                    let result = match ret {
                        Value::Object(_) => ret,
                        _ => this_obj,
                    };
                    frame.push(result);
                }

                // ─── Misc ───
                Op::Nop => {}
                Op::Debugger => {}

                _ => {
                    return Err(RuntimeError::Unimplemented(format!("opcode {:?} not yet handled @{}", op, pc)));
                }
            }
        }
    }

    /// Call a function value. Materializes a new Frame from the callee's
    /// FunctionProto, populates its locals slot 0..N with the arguments,
    /// runs the frame, returns the produced value (or Undefined on ReturnUndef).
    ///
    /// Tier-Ω.5.a: `this` is now threaded — stashed onto
    /// `Runtime::current_this` around NativeFn invocations (saved/restored
    /// across nesting), and set as `Frame::this_value` for closure frames.
    /// BoundFunction unwraps once, prepending bound args and overriding the
    /// caller's `this` with the bound this.
    pub fn call_function(&mut self, callee: Value, this: Value, args: Vec<Value>) -> Result<Value, RuntimeError> {
        let id = match callee {
            Value::Object(id) => id,
            // Tier-Ω.5.xxxxx: enrich callee-type tag with the actual
            // primitive type. Was: "callee is not callable: undefined".
            // Now: "callee is not callable: undefined" plus args-arity
            // so deeper bisects can see arg-count mismatches.
            other => {
                let tag = match &other {
                    Value::Undefined => "undefined",
                    Value::Null => "null",
                    Value::Boolean(_) => "boolean",
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::BigInt(_) => "bigint",
                    Value::Symbol(_) => "symbol",
                    Value::Object(_) => "object",
                };
                return Err(RuntimeError::TypeError(format!(
                    "callee is not callable: {} [argc={}]", tag, args.len()
                )));
            }
        };
        // Tier-Ω.5.s: claim the pending new.target slot for this invocation.
        // Op::New sets it just before dispatching; plain Call sites leave it
        // None. Taken (not cloned) so nested calls don't inherit it.
        let nt_for_this_call = self.pending_new_target.take();
        // Extract proto-or-native by inspecting the heap object once.
        // BoundFunction: rewrite to its target, prepending bound args.
        let (proto_opt, native_opt, effective_this, effective_args) = {
            let o = self.obj(id);
            match &o.internal_kind {
                crate::value::InternalKind::Closure(c) => {
                    // Tier-Ω.5.sss: arrow functions use their captured
                    // bound_this (set at MakeArrow time) regardless of
                    // the receiver argument. Regular closures use the
                    // passed receiver.
                    let actual_this = if c.is_arrow {
                        c.bound_this.clone().unwrap_or(Value::Undefined)
                    } else { this };
                    (Some(c.proto.clone()), None, actual_this, args)
                }
                crate::value::InternalKind::Function(f) => (None, Some(f.native.clone()), this, args),
                crate::value::InternalKind::Proxy(p) => {
                    // Ω.5.P60.E3: apply / construct trap dispatch. When the
                    // proxy is invoked as a callable (Op::Call) or as a ctor
                    // (Op::New), consult handler.apply / handler.construct
                    // respectively; missing trap delegates to the target.
                    let target = p.target;
                    let handler = p.handler;
                    drop(o);
                    let is_construct = nt_for_this_call.is_some();
                    let trap_name = if is_construct { "construct" } else { "apply" };
                    let trap = self.object_get(handler, trap_name);
                    if matches!(trap, Value::Object(_)) {
                        // Pack args into a real Array.
                        let arr = self.alloc_object(Object::new_array());
                        for (i, v) in args.iter().enumerate() {
                            self.object_set(arr, i.to_string(), v.clone());
                        }
                        self.object_set(arr, "length".into(), Value::Number(args.len() as f64));
                        if is_construct {
                            // handler.construct(target, argsArray, newTarget).
                            let nt = nt_for_this_call.clone().unwrap_or(Value::Object(target));
                            return self.call_function(trap, Value::Object(handler), vec![
                                Value::Object(target), Value::Object(arr), nt,
                            ]);
                        } else {
                            // handler.apply(target, thisArg, argsArray).
                            return self.call_function(trap, Value::Object(handler), vec![
                                Value::Object(target), this, Value::Object(arr),
                            ]);
                        }
                    }
                    // Missing trap: delegate to target.
                    self.pending_new_target = nt_for_this_call;
                    return self.call_function(Value::Object(target), this, args);
                }
                crate::value::InternalKind::BoundFunction(b) => {
                    // One level of unwrap is sufficient for v1; nested
                    // bindings recurse via tail-call into call_function.
                    let target = b.target;
                    let bound_this = b.this.clone();
                    let mut bound_args = b.args.clone();
                    bound_args.extend(args);
                    // Tier-Ω.5.s: propagate new.target through the bind shim.
                    self.pending_new_target = nt_for_this_call;
                    return self.call_function(Value::Object(target), bound_this, bound_args);
                }
                other => {
                    // Tier-Ω.5.xxxxx: enrich Object-callee tag with
                    // shape info — own-key count + first few keys so
                    // bisects can identify which object got mistakenly
                    // called. Also note presence of toString tag.
                    let kind = other.kind_name().to_string();
                    drop(o);
                    let keys: Vec<String> = self.obj(id).properties.keys().take(5).cloned().collect();
                    let nkeys = self.obj(id).properties.len();
                    let preview = if keys.is_empty() { String::new() } else { format!(" keys=[{}{}]", keys.join(","), if nkeys > 5 { ",…" } else { "" }) };
                    return Err(RuntimeError::TypeError(format!(
                        "callee is not callable: Object(kind={}{}) [argc={}]", kind, preview, args.len()
                    )));
                }
            }
        };
        if let Some(native) = native_opt {
            let saved = std::mem::replace(&mut self.current_this, effective_this);
            let saved_nt = std::mem::replace(&mut self.current_new_target, nt_for_this_call.clone());
            let result = native(self, &effective_args);
            self.current_this = saved;
            self.current_new_target = saved_nt;
            return result;
        }
        let proto = proto_opt.expect("closure branch implies proto");
        let is_generator = proto.is_generator;
        let gen_yields_id = if is_generator {
            // Tier-Ω.5.gggggg: push fresh yields-array on generator entry.
            let yields_arr = self.alloc_object(Object::new_array());
            self.object_set(yields_arr, "length".into(), Value::Number(0.0));
            self.gen_yields_stack.push(yields_arr);
            Some(yields_arr)
        } else { None };
        let args = effective_args;
        let this = effective_this;
        // Tier-Ω.5.e: binding-shared upvalues. Share the closure's
        // Rc<RefCell<Value>> handles with the inner frame; writes through
        // either side land in the same cell. The outer frame that created
        // the closure shares the cell too via its promoted local slot.
        let upvalues: Vec<UpvalueCell> = {
            let o = self.obj(id);
            match &o.internal_kind {
                crate::value::InternalKind::Closure(c) => c.upvalues.clone(),
                _ => Vec::new(),
            }
        };
        // Tier-Ω.5.l: rest parameter — collect args[rest_slot..] into an
        // Array bound to the rest slot. The Array carries InternalKind::Array
        // so alloc_object auto-wires %Array.prototype%.
        let mut locals: Vec<Value> = Vec::new();
        let rest_slot = proto.rest_param_slot;
        let args_slot = proto.arguments_slot;
        // Tier-Ω.5.zzz: allocate the `arguments` Array up-front so the
        // slot-population loop can store it at args_slot.
        let arguments_value: Option<Value> = if args_slot.is_some() {
            let mut arr = crate::value::Object::new_array();
            arr.set_own("length".into(), Value::Number(args.len() as f64));
            for (k, v) in args.iter().enumerate() {
                arr.set_own(k.to_string(), v.clone());
            }
            Some(Value::Object(self.alloc_object(arr)))
        } else { None };
        // Tier-Ω.5.kkkkk: self-binding for named function expr/decl.
        let self_slot = proto.self_name_slot;
        for (i, _) in proto.locals.iter().enumerate() {
            let slot = i as u16;
            if Some(slot) == args_slot {
                locals.push(arguments_value.clone().unwrap_or(Value::Undefined));
            } else if Some(slot) == self_slot {
                locals.push(Value::Object(id));
            } else if Some(slot) == rest_slot {
                let mut rest = crate::value::Object::new_array();
                let tail: Vec<Value> = if (i as usize) < args.len() {
                    args[i as usize..].to_vec()
                } else { Vec::new() };
                rest.set_own("length".into(), Value::Number(tail.len() as f64));
                for (k, v) in tail.into_iter().enumerate() {
                    rest.set_own(k.to_string(), v);
                }
                let id = self.alloc_object(rest);
                locals.push(Value::Object(id));
            } else if i < args.len() {
                locals.push(args[i].clone());
            } else {
                locals.push(Value::Undefined);
            }
        }
        let mut inner = Frame {
            bytecode: &proto.bytecode,
            constants: &proto.constants,
            source_map: &proto.source_map,
            line_starts: &proto.line_starts,
            source_url: &proto.source_url,
            construct_tags: &proto.construct_tags,
            locals_names: &proto.locals,
            upvalue_names: &proto.upvalues,
            locals,
            local_cells: Vec::new(),
            operand_stack: Vec::with_capacity(32),
            pc: 0,
            try_stack: Vec::new(),
            this_value: this,
            upvalues,
            last_property_lookup: None,
            pending_method_name: None,
            import_meta: None,
            new_target: nt_for_this_call.clone(),
        };
        let body_result = self.run_frame(&mut inner);
        if is_generator {
            // Tier-Ω.5.gggggg: pop yields-array on generator exit; build
            // an index-cursor iterator over the collected values. The
            // body's return value is discarded — generator return value
            // is exposed via the {value, done:true} terminal step in
            // proper coroutines; v1 sets done's value to undefined.
            let yields_id = self.gen_yields_stack.pop().expect("gen_yields_stack underflow");
            let _ = gen_yields_id;
            let _ = body_result; // even on Err, return an iterator that drains as if empty
            let iter = Object::new_ordinary();
            let it_id = self.alloc_object(iter);
            self.object_set(it_id, "__gen_arr__".into(), Value::Object(yields_id));
            self.object_set(it_id, "__gen_idx__".into(), Value::Number(0.0));
            let next_fn = crate::intrinsics::make_native("next", |rt, _args| {
                let this_id = match rt.current_this() { Value::Object(o) => o, _ => return Ok(Value::Undefined) };
                let arr = match rt.object_get(this_id, "__gen_arr__") { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
                let idx = match rt.object_get(this_id, "__gen_idx__") { Value::Number(n) => n as usize, _ => 0 };
                let len = rt.array_length(arr);
                let mut o = Object::new_ordinary();
                if idx >= len {
                    o.set_own("value".into(), Value::Undefined);
                    o.set_own("done".into(), Value::Boolean(true));
                } else {
                    let v = rt.object_get(arr, &idx.to_string());
                    rt.object_set(this_id, "__gen_idx__".into(), Value::Number((idx + 1) as f64));
                    o.set_own("value".into(), v);
                    o.set_own("done".into(), Value::Boolean(false));
                }
                Ok(Value::Object(rt.alloc_object(o)))
            });
            let next_id = self.alloc_object(next_fn);
            self.object_set(it_id, "next".into(), Value::Object(next_id));
            let return_fn = crate::intrinsics::make_native("return", |rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                let mut o = Object::new_ordinary();
                o.set_own("value".into(), v);
                o.set_own("done".into(), Value::Boolean(true));
                Ok(Value::Object(rt.alloc_object(o)))
            });
            let return_id = self.alloc_object(return_fn);
            self.object_set(it_id, "return".into(), Value::Object(return_id));
            let self_iter = it_id;
            let iter_fn = crate::intrinsics::make_native("@@iterator", move |_rt, _args| {
                Ok(Value::Object(self_iter))
            });
            let iter_fn_id = self.alloc_object(iter_fn);
            self.object_set(it_id, "@@iterator".into(), Value::Object(iter_fn_id));
            return Ok(Value::Object(it_id));
        }
        let body_result_v = body_result;
        // Ω.5.P58.E7: async function calls always return a Promise. Per
        // ECMA-262 §27.7 AsyncFunctionStart: the result is the
        // already-resolved Promise wrapping the body's return value
        // (resolve-with-throw if the body threw). Pre-P58.E7 cruftless
        // returned the body's value directly (or threw), so
        // (async () => 42)() returned 42 (number) rather than Promise.
        // execa, yeoman-environment, and many libs do
        // `(async () => {})().constructor.prototype` at module-init to
        // recover the native Promise prototype — that pattern requires
        // a real Promise to be returned.
        if proto.is_async {
            let p = crate::promise::new_promise(self);
            match body_result_v {
                Ok(v) => crate::promise::resolve_promise(self, p, v),
                Err(RuntimeError::Thrown(v)) => crate::promise::reject_promise(self, p, v),
                Err(e) => {
                    let msg = format!("{:?}", e);
                    let reason = Value::String(Rc::new(msg));
                    crate::promise::reject_promise(self, p, reason);
                }
            }
            return Ok(Value::Object(p));
        }
        let body_result = body_result_v?;
        // Tier-Ω.5.nnnnn: implicit-return-this for derived ctors.
        let result = if nt_for_this_call.is_some() && matches!(body_result, Value::Undefined) {
            inner.this_value.clone()
        } else { body_result };
        Ok(result)
    }

    fn constant_to_value(&self, frame: &Frame, idx: u16) -> Result<Value, RuntimeError> {
        match frame.constants.get(idx) {
            Some(rusty_js_bytecode::Constant::Number(n)) => Ok(Value::Number(*n)),
            Some(rusty_js_bytecode::Constant::String(s)) => Ok(Value::String(Rc::new(s.clone()))),
            Some(rusty_js_bytecode::Constant::BigInt(s)) => {
                match crate::bigint::JsBigInt::from_decimal(s) {
                    Some(b) => Ok(Value::BigInt(Rc::new(b))),
                    None => Err(RuntimeError::TypeError(format!("Invalid BigInt literal: {}", s))),
                }
            }
            Some(rusty_js_bytecode::Constant::Regex { .. }) => {
                Err(RuntimeError::Unimplemented("Regex literals not yet supported".into()))
            }
            Some(rusty_js_bytecode::Constant::Function(_)) => {
                // Function constants are not directly Pushable as values;
                // they're consumed by MakeClosure / MakeArrow. Reaching
                // here means the compiler emitted a PushConst on a
                // Function which would be a bug.
                Err(RuntimeError::TypeError("Function constant pushed as a value".into()))
            }
            None => Err(RuntimeError::TypeError(format!("invalid constant index {}", idx))),
        }
    }

    fn constant_name(&self, frame: &Frame, idx: u16) -> Result<String, RuntimeError> {
        match frame.constants.get(idx) {
            Some(rusty_js_bytecode::Constant::String(s)) => Ok(s.clone()),
            _ => Err(RuntimeError::TypeError(format!("constant {} is not a name string", idx))),
        }
    }
}

/// ToPropertyKey per ECMA-262 §7.1.19. v1 simplified: numbers stringify
/// to their canonical decimal form; other primitives ToString-coerce.
fn property_key(v: &Value) -> String {
    match v {
        Value::String(s) => s.as_str().to_string(),
        Value::Number(n) => crate::abstract_ops::number_to_string(*n),
        _ => crate::abstract_ops::to_string(v).as_str().to_string(),
    }
}

pub struct Frame<'a> {
    pub bytecode: &'a [u8],
    pub constants: &'a rusty_js_bytecode::ConstantsPool,
    /// Ω.5.P51.E1: pc → span map for this frame's bytecode. Lets runtime
    /// errors at fault time look up the source span (and via line_starts,
    /// the file:line:col) of the failing instruction. Empty for hand-built
    /// frames.
    pub source_map: &'a [(usize, rusty_js_ast::Span)],
    /// Ω.5.P51.E1: byte offsets of each line start in this frame's source.
    /// Used alongside source_map to derive line:col without re-scanning.
    pub line_starts: &'a [u32],
    /// Ω.5.P51.E1: URL of the source this frame was compiled from. Prepended
    /// to line:col in error messages. Empty when unknown.
    pub source_url: &'a str,
    /// Ω.5.P53.E2: AST-construct probe tags emitted by the compiler at
    /// known bug-prone sites (optional chains, try/catch, loops). Runtime
    /// error enrichment names the construct surrounding a fault. Empty
    /// for hand-built frames or modules compiled without probes.
    pub construct_tags: &'a [(usize, &'static str)],
    /// Tier-Ω.5.jj.diag: parallel to `locals`, carries the compiler's
    /// local-descriptor names so error messages can name which local
    /// resolved to an undefined callee. Empty when the frame doesn't
    /// carry descriptors (legacy paths or hand-built frames).
    pub locals_names: &'a [rusty_js_bytecode::LocalDescriptor],
    /// Tier-Ω.5.sssss: parallel to `upvalues`, carries the compiler's
    /// upvalue-descriptor names so error messages name the upvalue that
    /// resolved to undefined. Empty when the frame doesn't carry
    /// descriptors (hand-built frames).
    pub upvalue_names: &'a [rusty_js_bytecode::UpvalueDescriptor],
    pub locals: Vec<Value>,
    /// Parallel to `locals`. Tier-Ω.5.e: when a nested closure captures
    /// this frame's local slot `i`, `local_cells[i]` becomes
    /// `Some(Rc<RefCell<Value>>)` and authoritative; `locals[i]` is no
    /// longer read. Lazy in-place promotion (Approach A from the spec
    /// note) keeps unrelated frames on the fast path.
    pub local_cells: Vec<Option<UpvalueCell>>,
    pub operand_stack: Vec<Value>,
    pub pc: usize,
    pub try_stack: Vec<TryFrame>,
    /// `this` for the executing frame. Module frames default to Undefined;
    /// method-call frames receive the receiver. Tier-Ω.5.a.
    pub this_value: Value,
    /// Captured upvalues for this frame as shared binding cells. Closure
    /// frames receive Rc-clones of the closure's upvalue cells so writes
    /// propagate to the outer frame and to sibling closures. Tier-Ω.5.e.
    pub upvalues: Vec<UpvalueCell>,
    /// Diagnostic: name of the property most recently read by Op::GetProp.
    /// Used to enrich "callee is not callable" errors with the method name.
    pub last_property_lookup: Option<String>,
    /// Tier-Ω.5.yyyyy: method-name captured at GetProp time, only reset
    /// when consumed by Op::Call / Op::CallMethod / Op::New. Args loaded
    /// between GetProp and the call no longer overwrite it (the prior
    /// last_property_lookup was wrong for method-name diagnostics).
    pub pending_method_name: Option<String>,
    /// Tier-Ω.5.r: synthetic `import.meta` object for this module frame.
    /// Populated by `evaluate_module` (ESM path) with `{ url, dir }` keys.
    /// Frames that didn't enter through the module loader (raw run_module
    /// callers, function-call frames) leave this None; Op::PushImportMeta
    /// pushes Undefined in that case.
    pub import_meta: Option<crate::value::ObjectRef>,
    /// Tier-Ω.5.s: `new.target` slot. Populated by Op::New before
    /// dispatching the constructor call (set to the callee value). Plain
    /// Call frames leave this None; Op::PushNewTarget pushes Undefined
    /// in that case. Mirrors the import_meta threading shape.
    pub new_target: Option<Value>,
}

#[derive(Debug)]
pub struct TryFrame {
    pub catch_offset: usize,
    pub sp_at_entry: usize,
}

impl<'a> Frame<'a> {
    pub fn new_module(m: &'a CompiledModule) -> Self {
        let mut locals = Vec::new();
        for _ in &m.locals { locals.push(Value::Undefined); }
        Self {
            bytecode: &m.bytecode,
            constants: &m.constants,
            source_map: &m.source_map,
            line_starts: &m.line_starts,
            source_url: "",
            construct_tags: &m.construct_tags,
            locals_names: &m.locals,
            upvalue_names: &[],
            locals,
            local_cells: Vec::new(),
            operand_stack: Vec::with_capacity(32),
            pc: 0,
            try_stack: Vec::new(),
            this_value: Value::Undefined,
            upvalues: Vec::new(),
            last_property_lookup: None,
            pending_method_name: None,
            import_meta: None,
            new_target: None,
        }
    }

    /// Read local `slot`. If promoted (a closure captured it), read
    /// through the shared cell; else read the value slot directly.
    pub fn read_local(&self, slot: usize) -> Value {
        if let Some(Some(cell)) = self.local_cells.get(slot) {
            return cell.borrow().clone();
        }
        self.locals.get(slot).cloned().unwrap_or(Value::Undefined)
    }

    /// Write local `slot`. If promoted, write through the shared cell so
    /// nested closures see the update.
    pub fn write_local(&mut self, slot: usize, v: Value) {
        if let Some(Some(cell)) = self.local_cells.get(slot) {
            *cell.borrow_mut() = v;
            return;
        }
        while self.locals.len() <= slot { self.locals.push(Value::Undefined); }
        self.locals[slot] = v;
    }

    /// Promote local `slot` to a shared cell (idempotent). Used when a
    /// nested closure captures the slot — the cell becomes authoritative
    /// for both this frame's reads/writes and the closure's upvalue.
    pub fn promote_local(&mut self, slot: usize) -> UpvalueCell {
        while self.locals.len() <= slot { self.locals.push(Value::Undefined); }
        while self.local_cells.len() <= slot { self.local_cells.push(None); }
        if let Some(cell) = &self.local_cells[slot] {
            return cell.clone();
        }
        let v = std::mem::replace(&mut self.locals[slot], Value::Undefined);
        let cell = new_upvalue_cell(v);
        self.local_cells[slot] = Some(cell.clone());
        cell
    }

    pub fn push(&mut self, v: Value) { self.operand_stack.push(v); }

    pub fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.operand_stack.pop()
            .ok_or_else(|| RuntimeError::TypeError("operand stack underflow".into()))
    }

    pub fn peek(&self, depth: usize) -> Result<&Value, RuntimeError> {
        let len = self.operand_stack.len();
        if depth >= len {
            return Err(RuntimeError::TypeError("operand stack peek underflow".into()));
        }
        Ok(&self.operand_stack[len - 1 - depth])
    }
}

/// Tier-Ω.5.MMMMMMM: diagnostic helper. Render a Value into a compact tag
/// for fault-message enrichment. Primitives report their type name + a
/// short value preview; Objects report kind + up to 3 own-key names so the
/// bisect can identify the receiver shape without an interactive trace.
/// Tier-Ω.5.P24.E1.proto-chain-walk: walk the receiver's prototype chain
/// and produce a tag naming each link's internal-kind plus whether the
/// requested key was found there. When a method dispatch ends in
/// "callee is not callable: undefined", appending this tag tells the
/// caller exactly which prototype is missing the intrinsic — `Array→
/// Array.prototype→Object.prototype: no 'entries' slot` immediately
/// names Array.prototype.entries as the missing slot. Compounds across
/// every CallMethod error at the engine site that wires it in.
/// Ω.5.P51.E1: enrich a runtime error with file:line:col derived from the
/// frame's pc + source_map + line_starts. Idempotent — re-throws through
/// nested frames will see the marker " @" and skip re-enrichment. Empty
/// source_map / line_starts (hand-built frames) leave the error untouched.
fn enrich_with_source_pos(e: RuntimeError, frame: &Frame) -> RuntimeError {
    fn enrich_msg(msg: String, frame: &Frame) -> String {
        if msg.contains(" @") || frame.source_map.is_empty() || frame.line_starts.is_empty() {
            return msg;
        }
        // pc at fault is past the opcode byte; the span we want is the
        // largest source_map entry whose offset <= pc.
        let pc = frame.pc.saturating_sub(1);
        let span = match frame.source_map.iter().rposition(|&(off, _)| off <= pc) {
            Some(idx) => frame.source_map[idx].1,
            None => return msg,
        };
        let (line, col) = rusty_js_bytecode::byte_offset_to_line_col(
            frame.line_starts, span.start as u32,
        );
        // Ω.5.P53.E2: look up the most-recent construct tag at this pc.
        // If present, prepend the construct name so the diagnostic says
        // e.g. 'TypeError: operand stack underflow [optional-chain call] @file:line:col'.
        let construct = frame
            .construct_tags
            .iter()
            .rposition(|&(off, _)| off <= pc)
            .map(|i| frame.construct_tags[i].1);
        let pos = if frame.source_url.is_empty() {
            format!("@{}:{}", line, col)
        } else {
            format!("@{}:{}:{}", frame.source_url, line, col)
        };
        match construct {
            Some(c) => format!("{} [in {}] {}", msg, c, pos),
            None => format!("{} {}", msg, pos),
        }
    }
    match e {
        RuntimeError::TypeError(m) => RuntimeError::TypeError(enrich_msg(m, frame)),
        RuntimeError::RangeError(m) => RuntimeError::RangeError(enrich_msg(m, frame)),
        RuntimeError::ReferenceError(m) => RuntimeError::ReferenceError(enrich_msg(m, frame)),
        other => other,
    }
}

fn describe_proto_chain_for_key(rt: &Runtime, receiver: &Value, key: &str) -> String {
    let mut links: Vec<String> = Vec::new();
    let start_id = match receiver {
        Value::Object(id) => Some(*id),
        Value::String(_) => rt.string_prototype,
        Value::Number(_) => rt.number_prototype,
        Value::BigInt(_) => rt.bigint_prototype,
        _ => None,
    };
    let receiver_kind = match receiver {
        Value::Object(id) => kind_tag(rt, *id),
        Value::String(_) => "String".into(),
        Value::Number(_) => "Number".into(),
        Value::BigInt(_) => "BigInt".into(),
        Value::Symbol(_) => "Symbol".into(),
        Value::Boolean(_) => "Boolean".into(),
        Value::Undefined => return "undefined".into(),
        Value::Null => return "null".into(),
    };
    links.push(receiver_kind);
    let mut cur = start_id;
    let mut depth = 0;
    let mut found_link: Option<usize> = None;
    while let Some(c) = cur {
        depth += 1;
        if depth > 16 { links.push("…(deep)".into()); break; }
        let o = rt.obj(c);
        let kind = kind_tag(rt, c);
        if !links.last().map(|s| s.as_str() == kind.as_str()).unwrap_or(false) {
            links.push(kind);
        }
        if o.properties.contains_key(key) {
            // Found the slot, but its value resolved to non-callable.
            // The descriptor itself is present at this link.
            found_link = Some(links.len() - 1);
            break;
        }
        cur = o.proto;
    }
    match found_link {
        Some(i) => format!("{}: '{}' slot present at link {} but value not callable", links.join("→"), key, i),
        None => format!("{}: no '{}' slot on chain", links.join("→"), key),
    }
}

fn kind_tag(rt: &Runtime, id: rusty_js_gc::ObjectId) -> String {
    let o = rt.obj(id);
    match &o.internal_kind {
        crate::value::InternalKind::Array => "Array".into(),
        crate::value::InternalKind::Function(fi) => {
            if fi.name.is_empty() { "Function".into() } else { format!("Function({})", fi.name) }
        }
        crate::value::InternalKind::Closure(_) => "Closure".into(),
        crate::value::InternalKind::BoundFunction(_) => "BoundFunction".into(),
        crate::value::InternalKind::Promise(_) => "Promise".into(),
        crate::value::InternalKind::Error => "Error".into(),
        crate::value::InternalKind::RegExp(_) => "RegExp".into(),
        _ => {
            // Try matching against known intrinsic prototypes for clarity.
            if rt.object_prototype == Some(id) { "Object.prototype".into() }
            else if rt.array_prototype == Some(id) { "Array.prototype".into() }
            else if rt.function_prototype == Some(id) { "Function.prototype".into() }
            else if rt.promise_prototype == Some(id) { "Promise.prototype".into() }
            else if rt.string_prototype == Some(id) { "String.prototype".into() }
            else if rt.number_prototype == Some(id) { "Number.prototype".into() }
            else if rt.bigint_prototype == Some(id) { "BigInt.prototype".into() }
            else if rt.regexp_prototype == Some(id) { "RegExp.prototype".into() }
            else { "Object".into() }
        }
    }
}

fn describe_value_for_diag(rt: &Runtime, v: &Value) -> String {
    match v {
        Value::Undefined => "undefined".into(),
        Value::Null => "null".into(),
        Value::Boolean(b) => format!("Boolean({})", b),
        Value::Number(n) => format!("Number({})", n),
        Value::BigInt(b) => format!("BigInt({})", b.to_decimal()),
        Value::Symbol(s) => format!("Symbol({:?})", s.as_str()),
        Value::String(s) => {
            let t = s.as_str();
            if t.len() <= 24 { format!("String({:?})", t) }
            else {
                // Truncate at char boundary, not byte index.
                let mut end = 24;
                while end > 0 && !t.is_char_boundary(end) { end -= 1; }
                format!("String({:?}…)", &t[..end])
            }
        }
        Value::Object(id) => {
            let o = rt.obj(*id);
            // Tier-Ω.5.HHHHHHHH: richer per-kind preview.
            match &o.internal_kind {
                crate::value::InternalKind::Function(fi) => {
                    // Include the function's [[Name]] when present.
                    if fi.name.is_empty() {
                        "Function".to_string()
                    } else {
                        format!("Function({})", fi.name)
                    }
                }
                crate::value::InternalKind::Array => {
                    let len = match o.properties.get("length") {
                        Some(d) => match &d.value {
                            Value::Number(n) => *n as usize,
                            _ => 0,
                        },
                        None => 0,
                    };
                    // First few elements' shape (recursion-safe — only one level).
                    let mut elems = Vec::new();
                    for i in 0..len.min(2) {
                        match o.properties.get(&i.to_string()).map(|d| &d.value) {
                            Some(Value::Number(n)) => elems.push(format!("{}", n)),
                            Some(Value::String(s)) => {
                                let t = s.as_str();
                                if t.len() <= 12 { elems.push(format!("{:?}", t)); }
                                else {
                                    let mut end = 12;
                                    while end > 0 && !t.is_char_boundary(end) { end -= 1; }
                                    elems.push(format!("{:?}…", &t[..end]));
                                }
                            }
                            Some(Value::Boolean(b)) => elems.push(format!("{}", b)),
                            Some(Value::Null) => elems.push("null".into()),
                            Some(Value::Undefined) => elems.push("undefined".into()),
                            Some(_) => elems.push("...".into()),
                            None => elems.push("hole".into()),
                        }
                    }
                    let preview = if elems.is_empty() { String::new() }
                                  else { format!(" [{}{}]", elems.join(","),
                                                if len > 2 { ",…" } else { "" }) };
                    format!("Array(len={}){}", len, preview)
                }
                _ => {
                    let kind = match &o.internal_kind {
                        crate::value::InternalKind::Ordinary => "Object",
                        _ => "Object",
                    };
                    let keys: Vec<String> = o.properties.keys().take(3).cloned().collect();
                    let preview = if keys.is_empty() {
                        String::new()
                    } else {
                        format!(" keys=[{}{}]", keys.join(","),
                            if o.properties.len() > 3 { ",…" } else { "" })
                    };
                    format!("{}{}", kind, preview)
                }
            }
        }
    }
}
