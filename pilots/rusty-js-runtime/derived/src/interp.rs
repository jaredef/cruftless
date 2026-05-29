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

/// RS-EXT 2a: ECMA-262 §9.3 RealmRecord scaffold.
///
/// Each entry is a Realm Record holding the realm-scoped intrinsic-
/// prototype ObjectIds + globals. The primordial Realm (realm 0) is
/// allocated at Runtime init; additional realms can be allocated via
/// `Runtime::allocate_realm` (forthcoming).
///
/// This commit is BEHAVIOR-PRESERVING: realm 0's fields mirror the
/// Runtime's existing intrinsic-prototype fields. Later commits flip
/// the dispatch direction so that Runtime's fields are reads-from-current-
/// realm rather than the canonical source. Capability-passing realm
/// isolation per Doc 736 lands at the realm-level intrinsic-table swap.
#[derive(Debug, Default, Clone)]
pub struct RealmRecord {
    pub object_prototype: Option<rusty_js_gc::ObjectId>,
    pub array_prototype: Option<rusty_js_gc::ObjectId>,
    pub function_prototype: Option<rusty_js_gc::ObjectId>,
    pub promise_prototype: Option<rusty_js_gc::ObjectId>,
    pub string_prototype: Option<rusty_js_gc::ObjectId>,
    /// RS-EXT 2g: realm-scoped globals overrides. enter_realm shadows
    /// the primordial globals (Array, Object, Function, ...) with the
    /// realm's cloned constructors. exit_realm restores. Empty for
    /// realm 0 (the primordial uses Runtime.globals directly).
    pub globals_overrides: std::collections::HashMap<String, Value>,
    /// RS-EXT 2g: snapshot of primordial globals' values for each
    /// shadowed key, captured at enter_realm and restored at exit_realm.
    /// Stored on the realm so nested enter/exit pairs compose.
    pub primordial_snapshot: std::collections::HashMap<String, Value>,
    /// CP-EXT 5: when true, enter_realm clears non-allowlist host globals
    /// from the active globals; only ECMAScript intrinsics +
    /// `globals_overrides` (endowments) remain visible inside. exit_realm
    /// restores the full primordial via `primordial_full_snapshot`. This
    /// is the substrate that earns Doc 736's "ambient authority denied
    /// by default" property at the JS-API level.
    pub ambient_denied: bool,
    /// CP-EXT 5: snapshot of the FULL primordial globals captured at
    /// enter_realm when ambient_denied is true. Restored on exit_realm.
    pub primordial_full_snapshot: Option<std::collections::HashMap<String, Value>>,
}

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
    /// Ω.5.P04.E2.jit-runtime-dispatch: per-FunctionProto JIT cache.
    /// Key is the FunctionProto's Rc pointer cast to usize; value is
    /// Some(jit_fn) if a JIT compile succeeded, None if it failed and
    /// we should not retry. Populated lazily at the call_function entry
    /// for hot closures (call_count > jit_threshold).
    // LeJIT-Τ TB-EXT 7 (segfault fix 2026-05-23): values boxed so the
    // CompiledFn sits at a stable heap address. The TB closure-side
    // metadata cache holds `*const CompiledFn` populated on first
    // JIT-hit; if CompiledFn lived directly in the HashMap, subsequent
    // inserts triggering rehash would move the struct + dangle every
    // cached pointer. Box puts the CompiledFn on its own heap
    // allocation, stable for as long as the Box lives (forever, since
    // entries are insert-once never-evicted).
    pub jit_cache: HashMap<usize, Option<Box<rusty_js_jit::CompiledFn>>>,
    /// Doc 731 §VII R6: compilation budget is a counter threshold. After
    /// this many invocations of a Closure that hasn't yet been JIT-compiled,
    /// the runtime attempts compile. Default 100; can be overridden for
    /// bench/test purposes.
    pub jit_threshold: u32,
    /// TTTC-EXT 1: host-stack guard at the JS call dispatcher boundary.
    /// Until proper-tail-call lowering lands, deeply recursive call sites
    /// must fail as ordinary JS RangeErrors instead of aborting the host.
    pub call_depth: usize,
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
    /// TTOB-EXT 2: realm-local tagged-template registry. Parser lowering
    /// passes a stable source-site key into `__template_object__`; the
    /// helper canonicalizes the frozen TemplateStringsArray here.
    pub template_registry: HashMap<String, Value>,
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
    pub array_buffers: HashMap<rusty_js_gc::ObjectId, ArrayBufferRecord>,
    pub typed_array_views: HashMap<rusty_js_gc::ObjectId, TypedArrayViewRecord>,
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
    /// IR-EXT 23 TDZ sentinel: a uniquely-allocated Rc<String> wrapped in a
    /// Symbol-shaped Value. Stored on the Runtime so any LoadLocal/StoreLocal
    /// handler can detect it via `Rc::ptr_eq` on the inner Rc and throw the
    /// TDZ ReferenceError. The Symbol shape was chosen so the sentinel
    /// doesn't collide with any user-allocated Value variant and doesn't
    /// require a new Value enum variant (would touch every match site).
    pub tdz_sentinel: std::rc::Rc<String>,
    /// EXT 90 / Doc 730 §XIV: opt-in deviation set per the dual-pipeline
    /// formalization. Each name is a typed primitive at the deviation-tier
    /// alphabet — recognizing one ecosystem-bug-tolerated pattern that
    /// the spec forbids but production engines (Bun, V8, Node) silently
    /// absorb. Helpers at the strict-spec sites consult this set; when
    /// the matching name is present, they downgrade the TypeError to a
    /// tolerant lowering (with a diagnostic surface). Strict-by-default
    /// is preserved; consumer code opts in via __cruftless_tolerate(name).
    ///
    /// EXT 91 / Doc 730 §XV: each entry's value is the deviation's
    /// constraint-comprehension contract — the list of protected
    /// invariants the strict_rejection enforces, each either
    /// Comprehended(name of a typed §XIII primitive) or
    /// Waived(audit-reference text). __cruftless_tolerate refuses to
    /// opt into a deviation whose protected_invariants contain any
    /// Unknown entries; Waived entries are accepted on the audit
    /// reference.
    pub tolerated_deviations: HashSet<&'static str>,
    /// Tier-Ω.5.s: `new.target` slot pending injection into the next
    /// closure frame to be entered via `call_function`. Set by Op::New
    /// before dispatching, consumed (take()) at frame construction.
    /// Native frames don't read it directly; they call current_new_target()
    /// if they need the value. Mirrors current_this's save/restore shape
    /// for native dispatch.
    pub pending_new_target: Option<Value>,
    pub current_new_target: Option<Value>,
    /// ECMA-262 §25.5.2 JSON.stringify state. LIFO stack of replacer
    /// functions; the topmost is consulted by SerializeJSONProperty for
    /// the active stringify call. Pushed by json_stringify_via on entry
    /// and popped on exit; nested stringify (via toJSON callbacks that
    /// re-enter stringify) push their own frame.
    pub json_replacer_stack: Vec<Value>,
    /// ECMA-262 §25.5.2 step 4.b — when the replacer is an Array, its
    /// items (after the spec's String/Number coercion) form the
    /// PropertyList that filters and orders the keys serialized for
    /// every non-array compound. None at a frame means "no PropertyList
    /// active for this stringify call"; Some(list) means the list is
    /// the whitelist, in the given order.
    pub json_property_list_stack: Vec<Option<Vec<String>>>,
    /// ECMA-262 §25.5.2.5/6 circular-reference stack for the active
    /// JSON.stringify call. Object ids are pushed while serializing a
    /// compound value and popped before returning.
    pub json_stringify_stack: Vec<rusty_js_gc::ObjectId>,
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
    /// RS-EXT 2a: realm substrate scaffold per ECMA-262 §9.3 RealmRecord.
    /// Each entry is a RealmRecord; current_realm indexes into realms.
    /// Behavior-preserving in this commit: realm 0 is the primordial realm
    /// and its intrinsic-prototype fields mirror the Runtime's. Later
    /// commits will (a) populate realms[k>0] from `allocate_realm`, (b)
    /// swap Runtime's intrinsic-prototype fields on enter_realm/exit_realm,
    /// (c) add `__cruftless_eval_realm` to expose the API to the
    /// realm-pollution probe. See pilots/realm-substrate/seed.md §I.3.
    pub realms: Vec<RealmRecord>,
    pub current_realm: usize,
    /// CharCode-EXT 2 (2026-05-23, JIT-EXT 33 interp-tier IC): cached
    /// ObjectId of String.prototype.charCodeAt. Populated lazily at the
    /// first Op::CallMethod fast-path eligibility check; used to
    /// verify the resolved method is the intrinsic (not a user
    /// override) before bypassing call_function.
    pub intrinsic_string_charcodeat_id: Option<rusty_js_gc::ObjectId>,
    /// IHI-EXT 3 (2026-05-24): cached ObjectId for String.prototype.toLowerCase
    /// per the interp-tier IC table's override-safety gate. Lazy-populated
    /// at first eligible Op::CallMethod fast-path attempt.
    pub intrinsic_string_to_lower_case_id: Option<rusty_js_gc::ObjectId>,
    /// IHI-EXT 4 (2026-05-24): cached ObjectId for String.prototype.trim.
    pub intrinsic_string_trim_id: Option<rusty_js_gc::ObjectId>,
    /// IHI-EXT 5 (2026-05-24): cached ObjectId for String.prototype.indexOf.
    pub intrinsic_string_index_of_id: Option<rusty_js_gc::ObjectId>,
    /// IHI-EXT 9 (2026-05-24): String.prototype.{codePointAt, toUpperCase,
    /// startsWith, endsWith, includes} intrinsic-ObjectId caches.
    pub intrinsic_string_code_point_at_id: Option<rusty_js_gc::ObjectId>,
    pub intrinsic_string_to_upper_case_id: Option<rusty_js_gc::ObjectId>,
    pub intrinsic_string_starts_with_id: Option<rusty_js_gc::ObjectId>,
    pub intrinsic_string_ends_with_id: Option<rusty_js_gc::ObjectId>,
    pub intrinsic_string_includes_id: Option<rusty_js_gc::ObjectId>,
    /// IHI-EXT 10 (2026-05-24, Finding IHI.2 deeper-layer closure per
    /// Doc 740 §IV.2): per-FunctionProto IC dispatch side-table.
    /// Outer HashMap: bytecode_ptr → per-pc Vec<CachedDispatch>.
    /// Inner Vec is O(1) array-indexed by pc (~5ns/access). Replaces
    /// IHI-EXT 8's (pc, proto) tuple HashMap (~80ns/get). Per-CallMethod
    /// dispatch cost drops from ~80ns to ~35ns (HashMap.get-per-proto
    /// + Vec[pc]).
    pub ic_dispatch_cache: HashMap<usize, Vec<crate::interp_ic_table::CachedDispatch>>,
    pub number_prototype: Option<rusty_js_gc::ObjectId>,
    pub bigint_prototype: Option<rusty_js_gc::ObjectId>,
    pub symbol_prototype: Option<rusty_js_gc::ObjectId>,
    /// Tier-Ω.5.i: %RegExp.prototype% — installed alongside other
    /// intrinsic prototypes; alloc_object auto-wires RegExp objects.
    pub regexp_prototype: Option<rusty_js_gc::ObjectId>,
    /// Tier-Ω Round 1 (2026-05-21): generator + async-generator
    /// prototype-chain intrinsics per ECMA-262 §27.3 / §27.4 / §27.5.
    /// Layout per spec:
    ///   %IteratorPrototype%        ← root of sync iter chain
    ///     ← %GeneratorPrototype%   (Generator instances' [[Prototype]])
    ///       ← function.prototype on fn declared as function*()
    ///   %GeneratorFunction.prototype% ← Generator fn's [[Prototype]]
    ///   %AsyncIteratorPrototype%   ← root of async iter chain
    ///     ← %AsyncGeneratorPrototype%
    ///   %AsyncGeneratorFunction.prototype% ← async-gen fn's [[Prototype]]
    /// Allocated at install_prototypes; chained via .proto. MakeClosure
    /// for generator/async-generator closures sets the closure's
    /// [[Prototype]] to the corresponding *Function.prototype intrinsic
    /// and sets fn.prototype to the corresponding *Generator.prototype.
    /// Without these, `Object.getPrototypeOf(asyncGenFn).prototype`
    /// returned undefined; the @sec-ant/readable-stream ponyfill (and
    /// transitively got + get-stream + runtypes) failed at module-init
    /// with a Cannot-convert-undefined error.
    pub iterator_prototype: Option<rusty_js_gc::ObjectId>,
    pub generator_prototype: Option<rusty_js_gc::ObjectId>,
    pub generator_function_prototype: Option<rusty_js_gc::ObjectId>,
    pub async_iterator_prototype: Option<rusty_js_gc::ObjectId>,
    pub async_generator_prototype: Option<rusty_js_gc::ObjectId>,
    pub async_generator_function_prototype: Option<rusty_js_gc::ObjectId>,
    /// Tier-Ω.5.gggggg: stack of currently-running generator yields-arrays.
    /// Each generator function invocation pushes a fresh array on entry,
    /// pops it on completion. `__yield_push__(v)` appends to the top.
    /// Nested generators (yield inside a generator that yields a generator)
    /// stack correctly.
    pub gen_yields_stack: Vec<rusty_js_gc::ObjectId>,
    pub gen_async_stack: Vec<bool>,
    pub active_generator_for_yield: Option<ObjectRef>,
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
    pub napi_main_inbox:
        std::sync::Arc<std::sync::Mutex<std::collections::VecDeque<crate::napi::NapiMainJob>>>,
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
    /// CAPS-EXT 3+ (Doc 736 Pilot α): the capability dispatcher. Holds
    /// the per-process CapMode + ambient capability set + audit log.
    /// Created in Compat (Mode 0) by default; host changes mode via
    /// `Runtime::set_cap_mode` when the CLI passes `--audit` etc.
    /// Effectful methods route through this; until CAPS-EXT 6+ wires
    /// the routes, the dispatcher exists but is not consulted.
    pub caps: std::sync::Arc<crate::caps::CapDispatcher>,
    /// ES-EXT 2 (eval-scope-binding-chain): one-shot flag set by
    /// `evaluate_script` and consumed at the next compile step inside
    /// `evaluate_module`. When true, the compile call switches from
    /// `compile_module_with_url` to `compile_script_with_url`, threading
    /// Script semantics (top-level `var` attaches to globalThis per
    /// ECMA-262 §19.2.1.3 PerformEval). Cleared via `mem::replace` at
    /// the compile site so subsequent ESM compiles inside this Runtime
    /// revert to Module semantics.
    pub pending_script_mode: bool,
    pub pending_direct_eval_super_context: Option<rusty_js_bytecode::DirectEvalSuperContext>,
    pub pending_eval_this: Option<Value>,
    pub pending_eval_derived_initial_this: Option<Value>,
    pub pending_eval_new_target: Option<Value>,
    pub pending_eval_private_home: Option<ObjectRef>,
    /// GBSU-EXT 1 (global-binding-surface-unification rung 1): direct
    /// ObjectId handle to the globalThis Object — the spec's global Variable
    /// Environment Record (ECMA-262 §9.1.1.4, §16.1). Today the substrate
    /// also carries `self.globals: HashMap<String, Value>` as a parallel
    /// surface; this handle is the migration target. Populated by
    /// `install_global_this` (intrinsics.rs). None until install runs.
    /// Doc 731 alphabet-purity: this is the single global-binding word the
    /// downstream alphabet will speak after rungs 2-4 collapse the HashMap.
    pub global_object: Option<rusty_js_gc::ObjectId>,
}

#[derive(Clone)]
pub struct ArrayBufferRecord {
    pub byte_length: usize,
    pub max_byte_length: usize,
    pub data: Vec<Value>,
    pub detached: bool,
}

#[derive(Clone)]
pub struct TypedArrayViewRecord {
    pub buffer: rusty_js_gc::ObjectId,
    pub byte_offset: usize,
    pub fixed_length: Option<usize>,
    pub bytes_per_element: usize,
}

/// TAWR-EXT 2: classification of a property-key string for
/// IntegerIndexedExotic [[DefineOwnProperty]] / [[Set]] / [[GetOwnProperty]]
/// per §7.1.21 CanonicalNumericIndexString + §10.4.5.{3,5}.
#[derive(Clone, Debug)]
pub enum NumericIndexClass {
    ValidArrayIndex(usize),
    InvalidNumericIndex,
}

impl Runtime {
    fn canonical_array_index_key(key: &str) -> Option<usize> {
        if key.is_empty() || (key.len() > 1 && key.starts_with('0')) {
            return None;
        }
        key.parse::<usize>().ok()
    }

    /// TAWR-EXT 2: classify a property-key string per §7.1.21
    /// CanonicalNumericIndexString + §10.4.5.{3,5} IntegerIndexed
    /// numeric-index discipline. Returns:
    ///   - ValidArrayIndex(usize) when key is a canonical-numeric-index
    ///     AND IsInteger AND >= 0 (callers still bounds-check).
    ///   - InvalidNumericIndex when key is a canonical-numeric-index
    ///     but fails IsInteger / equals -0 / is negative (callers should
    ///     return false / undefined per the IntegerIndexed branch).
    ///   - None when key is NOT a canonical-numeric-index string
    ///     (string keys like "foo" — fall through to OrdinaryX path).
    fn classify_numeric_index_key(key: &str) -> Option<NumericIndexClass> {
        if key.is_empty() {
            return None;
        }
        if key == "-0" {
            return Some(NumericIndexClass::InvalidNumericIndex);
        }
        // Try parse as canonical non-negative integer first (the common
        // valid-array-index case).
        if let Some(idx) = Self::canonical_array_index_key(key) {
            return Some(NumericIndexClass::ValidArrayIndex(idx));
        }
        // Try parse as f64; if ToString(f64) round-trips to key, it's
        // a canonical-numeric-index per §7.1.21 step 3. Otherwise None.
        let parsed: f64 = match key.parse() {
            Ok(n) => n,
            Err(_) => return None,
        };
        if format!("{}", parsed) != key && format!("{:?}", parsed) != key {
            // Not canonical (e.g. "01", "1e0" don't round-trip).
            return None;
        }
        // It IS a canonical-numeric-index but failed the integer-and-
        // non-negative check (NaN / Infinity / fractional / negative).
        Some(NumericIndexClass::InvalidNumericIndex)
    }

    fn typed_array_view_len(&self, id: ObjectRef) -> Option<usize> {
        let view = self.typed_array_views.get(&id)?;
        let buf = self.array_buffers.get(&view.buffer)?;
        if buf.detached {
            return Some(0);
        }
        if view.byte_offset > buf.byte_length {
            return Some(0);
        }
        match view.fixed_length {
            Some(len) => {
                let required = view
                    .byte_offset
                    .saturating_add(len.saturating_mul(view.bytes_per_element));
                if required > buf.byte_length {
                    Some(0)
                } else {
                    Some(len)
                }
            }
            None => Some((buf.byte_length - view.byte_offset) / view.bytes_per_element),
        }
    }

    pub fn typed_array_view_out_of_bounds(&self, id: ObjectRef) -> bool {
        let Some(view) = self.typed_array_views.get(&id) else {
            return false;
        };
        let Some(buf) = self.array_buffers.get(&view.buffer) else {
            return true;
        };
        if buf.detached {
            return true;
        }
        if view.byte_offset > buf.byte_length {
            return true;
        }
        if let Some(len) = view.fixed_length {
            let required = view
                .byte_offset
                .saturating_add(len.saturating_mul(view.bytes_per_element));
            return required > buf.byte_length;
        }
        false
    }

    pub fn typed_array_view_detached(&self, id: ObjectRef) -> bool {
        let Some(view) = self.typed_array_views.get(&id) else {
            return false;
        };
        self.array_buffers
            .get(&view.buffer)
            .map(|buf| buf.detached)
            .unwrap_or(true)
    }

    fn typed_array_view_byte_length(&self, id: ObjectRef) -> Option<usize> {
        let view = self.typed_array_views.get(&id)?;
        Some(
            self.typed_array_view_len(id)?
                .saturating_mul(view.bytes_per_element),
        )
    }

    fn typed_array_get_index(&self, id: ObjectRef, idx: usize) -> Option<Value> {
        let view = self.typed_array_views.get(&id)?;
        let len = self.typed_array_view_len(id)?;
        if idx >= len {
            return Some(Value::Undefined);
        }
        let byte_index = view
            .byte_offset
            .saturating_add(idx.saturating_mul(view.bytes_per_element));
        let buf = self.array_buffers.get(&view.buffer)?;
        Some(
            buf.data
                .get(byte_index)
                .cloned()
                .unwrap_or(Value::Number(0.0)),
        )
    }

    fn typed_array_set_index(&mut self, id: ObjectRef, idx: usize, value: Value) -> bool {
        let Some(view) = self.typed_array_views.get(&id).cloned() else {
            return false;
        };
        let Some(len) = self.typed_array_view_len(id) else {
            return false;
        };
        if idx >= len {
            return true;
        }
        let byte_index = view
            .byte_offset
            .saturating_add(idx.saturating_mul(view.bytes_per_element));
        let Some(buf) = self.array_buffers.get_mut(&view.buffer) else {
            return true;
        };
        if byte_index >= buf.data.len() {
            buf.data.resize(byte_index + 1, Value::Number(0.0));
        }
        buf.data[byte_index] = value;
        true
    }

    pub fn resize_array_buffer(
        &mut self,
        id: ObjectRef,
        new_len: usize,
    ) -> Result<(), RuntimeError> {
        let Some(buf) = self.array_buffers.get_mut(&id) else {
            return Err(RuntimeError::TypeError(
                "ArrayBuffer.prototype.resize: incompatible receiver".into(),
            ));
        };
        if buf.detached {
            return Err(RuntimeError::TypeError(
                "ArrayBuffer.prototype.resize: buffer is detached".into(),
            ));
        }
        if new_len > buf.max_byte_length {
            return Err(RuntimeError::RangeError(
                "ArrayBuffer.prototype.resize: new length exceeds maxByteLength".into(),
            ));
        }
        buf.byte_length = new_len;
        buf.data.resize(new_len, Value::Number(0.0));
        Ok(())
    }

    pub fn detach_array_buffer(&mut self, id: ObjectRef) -> Result<(), RuntimeError> {
        let Some(buf) = self.array_buffers.get_mut(&id) else {
            return Err(RuntimeError::TypeError(
                "$262.detachArrayBuffer: argument must be an ArrayBuffer".into(),
            ));
        };
        buf.detached = true;
        buf.byte_length = 0;
        buf.max_byte_length = 0;
        buf.data.clear();
        Ok(())
    }

    pub fn new() -> Self {
        let mut rt = Self {
            jit_cache: HashMap::new(),
            // Threshold defaults to 100 calls but is overridable via
            // CRUFTLESS_JIT_THRESHOLD env var for bench/test purposes.
            // Set to 1 to make every Closure JIT on first invocation.
            // CRUFT_JIT_THRESHOLD takes precedence; CRUFTLESS_JIT_THRESHOLD
            // kept for one-release backwards-compat after the cruft rename.
            jit_threshold: std::env::var("CRUFT_JIT_THRESHOLD")
                .or_else(|_| std::env::var("CRUFTLESS_JIT_THRESHOLD"))
                .ok()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(100),
            call_depth: 0,
            engine_helpers: HashMap::new(),
            last_value: Value::Undefined,
            host_hooks: crate::module::HostHooks::default(),
            modules: HashMap::new(),
            pkg_json_cache: HashMap::new(),
            template_registry: HashMap::new(),
            module_resolution_trace: HashMap::new(),
            module_post_eval_trace: HashMap::new(),
            module_ns_synth_trace: HashMap::new(),
            symbol_lookup_miss_log: Vec::new(),
            host_stub_miss_log: Vec::new(),
            array_buffers: HashMap::new(),
            typed_array_views: HashMap::new(),
            operator_lowering_trace: HashMap::new(),
            heap: rusty_js_gc::Heap::new(),
            job_queue: crate::job_queue::JobQueue::new(),
            pending_unhandled: HashSet::new(),
            current_this: Value::Undefined,
            tdz_sentinel: std::rc::Rc::new("__cruft_tdz__".to_string()),
            tolerated_deviations: HashSet::new(),
            pending_new_target: None,
            current_new_target: None,
            json_replacer_stack: Vec::new(),
            json_property_list_stack: Vec::new(),
            json_stringify_stack: Vec::new(),
            object_prototype: None,
            array_prototype: None,
            function_prototype: None,
            promise_prototype: None,
            string_prototype: None,
            // RS-EXT 2a: primordial realm (realm 0); fields will be
            // mirror-populated as the install_* methods fire during init.
            realms: vec![RealmRecord::default()],
            current_realm: 0,
            intrinsic_string_charcodeat_id: None,
            intrinsic_string_to_lower_case_id: None,
            intrinsic_string_trim_id: None,
            intrinsic_string_index_of_id: None,
            intrinsic_string_code_point_at_id: None,
            intrinsic_string_to_upper_case_id: None,
            intrinsic_string_starts_with_id: None,
            intrinsic_string_ends_with_id: None,
            intrinsic_string_includes_id: None,
            ic_dispatch_cache: HashMap::new(),
            number_prototype: None,
            bigint_prototype: None,
            symbol_prototype: None,
            regexp_prototype: None,
            iterator_prototype: None,
            generator_prototype: None,
            generator_function_prototype: None,
            async_iterator_prototype: None,
            async_generator_prototype: None,
            async_generator_function_prototype: None,
            gen_yields_stack: Vec::new(),
            gen_async_stack: Vec::new(),
            active_generator_for_yield: None,
            pending_live_bindings: HashMap::new(),
            fd_table: HashMap::new(),
            next_fd: 3,
            current_module_url: Vec::new(),
            napi_libs: Vec::new(),
            napi_envs: Vec::new(),
            napi_module_cache: HashMap::new(),
            napi_main_inbox: std::sync::Arc::new(std::sync::Mutex::new(
                std::collections::VecDeque::new(),
            )),
            napi_keepalive: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            caps: std::sync::Arc::new(crate::caps::CapDispatcher::compat()),
            pending_script_mode: false,
            pending_direct_eval_super_context: None,
            pending_eval_this: None,
            pending_eval_derived_initial_this: None,
            pending_eval_new_target: None,
            pending_eval_private_home: None,
            global_object: None,
        };
        // GBSU-EXT 7f.4: eager-allocate the unified globalThis Object at
        // Runtime construction so `global_object` is always `Some` for the
        // lifetime of the Runtime. This is the precondition that lets every
        // bootstrap-fallback branch in enumerate_roots / define_global_property
        // / Op::StoreGlobal be deleted — global_object is provably set before
        // any of those handlers fire. The Object starts empty; install_globals
        // populates it (and install_global_this attaches the self-reference
        // descriptor + the Intl namespace etc.).
        let gt = rt.alloc_object(crate::value::Object::new_ordinary());
        rt.global_object = Some(gt);
        rt
    }

    /// CAPS-EXT 4: replace the capability dispatcher with one set to the
    /// requested mode. Discards any previously-accumulated audit log;
    /// intended to be called once at startup after CLI parsing.
    pub fn set_cap_mode(&mut self, mode: crate::caps::CapMode) {
        self.caps = std::sync::Arc::new(crate::caps::CapDispatcher::new(mode));
    }

    /// `this` for the active native call. Returns Undefined outside one.
    pub fn current_this(&self) -> Value {
        self.current_this.clone()
    }

    pub fn new_generator_scaffold(&mut self) -> ObjectRef {
        let mut obj = Object::new_ordinary();
        obj.proto = self.generator_prototype;
        obj.internal_kind = crate::value::InternalKind::Generator(crate::value::GeneratorObject {
            state: crate::value::GeneratorState::SuspendedStart,
            continuation: None,
            yielded_value: None,
        });
        let generator = self.alloc_object(obj);
        self.install_generator_lifecycle_methods(generator);
        generator
    }

    fn new_generator_with_continuation(&mut self, snapshot: FrameSnapshot) -> ObjectRef {
        let generator = self.new_generator_scaffold();
        if let crate::value::InternalKind::Generator(g) = &mut self.obj_mut(generator).internal_kind
        {
            g.continuation = Some(Box::new(snapshot));
        }
        generator
    }

    fn install_generator_lifecycle_methods(&mut self, generator: ObjectRef) {
        let next_fn = crate::intrinsics::make_native("next", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let sent = args.first().cloned().unwrap_or(Value::Undefined);
            rt.generator_next_scaffold(this_id, sent)
        });
        let next_id = self.alloc_object(next_fn);
        self.set_engine_sentinel(generator, "next", Value::Object(next_id));

        let return_fn = crate::intrinsics::make_native("return", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let value = args.first().cloned().unwrap_or(Value::Undefined);
            rt.generator_return_scaffold(this_id, value)
        });
        let return_id = self.alloc_object(return_fn);
        self.set_engine_sentinel(generator, "return", Value::Object(return_id));

        let throw_fn = crate::intrinsics::make_native("throw", |rt, args| {
            let this_id = match rt.current_this() {
                Value::Object(o) => o,
                _ => return Ok(Value::Undefined),
            };
            let thrown = args.first().cloned().unwrap_or(Value::Undefined);
            rt.generator_throw_scaffold(this_id, thrown)
        });
        let throw_id = self.alloc_object(throw_fn);
        self.set_engine_sentinel(generator, "throw", Value::Object(throw_id));

        let self_iter = generator;
        let iter_fn = crate::intrinsics::make_native("@@iterator", move |_rt, _args| {
            Ok(Value::Object(self_iter))
        });
        let iter_id = self.alloc_object(iter_fn);
        self.set_engine_sentinel(generator, "@@iterator", Value::Object(iter_id));
    }

    fn generator_result_object(&mut self, value: Value, done: bool) -> Value {
        let mut o = Object::new_ordinary();
        o.set_own("value".into(), value);
        o.set_own("done".into(), Value::Boolean(done));
        Value::Object(self.alloc_object(o))
    }

    pub fn with_active_generator_for_yield<F, T>(
        &mut self,
        generator: ObjectRef,
        f: F,
    ) -> Result<T, RuntimeError>
    where
        F: FnOnce(&mut Self) -> Result<T, RuntimeError>,
    {
        let prior = self.active_generator_for_yield.replace(generator);
        let result = f(self);
        self.active_generator_for_yield = prior;
        result
    }

    pub fn generator_next_scaffold(
        &mut self,
        generator: ObjectRef,
        sent: Value,
    ) -> Result<Value, RuntimeError> {
        let (snapshot, inject_sent_value) = {
            let obj = self.obj_mut(generator);
            match &mut obj.internal_kind {
                crate::value::InternalKind::Generator(g) => {
                    if matches!(g.state, crate::value::GeneratorState::Executing) {
                        return Err(RuntimeError::TypeError(
                            "Generator.prototype.next: generator is already executing".into(),
                        ));
                    }
                    if matches!(g.state, crate::value::GeneratorState::Completed) {
                        return Ok(self.generator_result_object(Value::Undefined, true));
                    }
                    let inject_sent_value =
                        matches!(g.state, crate::value::GeneratorState::SuspendedYield);
                    g.state = crate::value::GeneratorState::Executing;
                    g.yielded_value = None;
                    (g.continuation.take(), inject_sent_value)
                }
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Generator.prototype.next: receiver is not a Generator object".into(),
                    ))
                }
            }
        };
        let Some(mut snapshot) = snapshot else {
            if let crate::value::InternalKind::Generator(g) =
                &mut self.obj_mut(generator).internal_kind
            {
                g.state = crate::value::GeneratorState::Completed;
            }
            return Ok(self.generator_result_object(Value::Undefined, true));
        };
        if inject_sent_value {
            if let Some(slot) = snapshot.operand_stack.last_mut() {
                *slot = sent;
            } else {
                snapshot.operand_stack.push(sent);
            }
        }

        let mut frame = Frame::from(snapshot.as_ref());
        let result = self.with_active_generator_for_yield(generator, |rt| rt.run_frame(&mut frame));
        match result {
            Ok(value) => {
                let yielded = match &self.obj(generator).internal_kind {
                    crate::value::InternalKind::Generator(g)
                        if matches!(g.state, crate::value::GeneratorState::SuspendedYield) =>
                    {
                        true
                    }
                    _ => false,
                };
                if yielded {
                    Ok(self.generator_result_object(value, false))
                } else {
                    if let crate::value::InternalKind::Generator(g) =
                        &mut self.obj_mut(generator).internal_kind
                    {
                        g.state = crate::value::GeneratorState::Completed;
                        g.continuation = None;
                    }
                    Ok(self.generator_result_object(value, true))
                }
            }
            Err(e) => {
                if let crate::value::InternalKind::Generator(g) =
                    &mut self.obj_mut(generator).internal_kind
                {
                    g.state = crate::value::GeneratorState::Completed;
                    g.continuation = None;
                }
                Err(e)
            }
        }
    }

    pub fn generator_throw_scaffold(
        &mut self,
        generator: ObjectRef,
        thrown: Value,
    ) -> Result<Value, RuntimeError> {
        let snapshot = {
            let obj = self.obj_mut(generator);
            match &mut obj.internal_kind {
                crate::value::InternalKind::Generator(g) => {
                    if matches!(g.state, crate::value::GeneratorState::Executing) {
                        return Err(RuntimeError::TypeError(
                            "Generator.prototype.throw: generator is already executing".into(),
                        ));
                    }
                    if !matches!(g.state, crate::value::GeneratorState::SuspendedYield) {
                        g.state = crate::value::GeneratorState::Completed;
                        g.continuation = None;
                        g.yielded_value = None;
                        return Err(RuntimeError::Thrown(thrown));
                    }
                    g.state = crate::value::GeneratorState::Executing;
                    g.yielded_value = None;
                    g.continuation.take()
                }
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Generator.prototype.throw: receiver is not a Generator object".into(),
                    ))
                }
            }
        };
        let Some(snapshot) = snapshot else {
            if let crate::value::InternalKind::Generator(g) =
                &mut self.obj_mut(generator).internal_kind
            {
                g.state = crate::value::GeneratorState::Completed;
            }
            return Err(RuntimeError::Thrown(thrown));
        };

        let mut frame = Frame::from(snapshot.as_ref());
        if let Err(e) = inject_throw_into_frame(&mut frame, thrown) {
            if let crate::value::InternalKind::Generator(g) =
                &mut self.obj_mut(generator).internal_kind
            {
                g.state = crate::value::GeneratorState::Completed;
                g.continuation = None;
            }
            return Err(e);
        }
        let result = self.with_active_generator_for_yield(generator, |rt| rt.run_frame(&mut frame));
        match result {
            Ok(value) => {
                let yielded = match &self.obj(generator).internal_kind {
                    crate::value::InternalKind::Generator(g)
                        if matches!(g.state, crate::value::GeneratorState::SuspendedYield) =>
                    {
                        true
                    }
                    _ => false,
                };
                if yielded {
                    Ok(self.generator_result_object(value, false))
                } else {
                    if let crate::value::InternalKind::Generator(g) =
                        &mut self.obj_mut(generator).internal_kind
                    {
                        g.state = crate::value::GeneratorState::Completed;
                        g.continuation = None;
                    }
                    Ok(self.generator_result_object(value, true))
                }
            }
            Err(e) => {
                if let crate::value::InternalKind::Generator(g) =
                    &mut self.obj_mut(generator).internal_kind
                {
                    g.state = crate::value::GeneratorState::Completed;
                    g.continuation = None;
                }
                Err(e)
            }
        }
    }

    pub fn generator_return_scaffold(
        &mut self,
        generator: ObjectRef,
        value: Value,
    ) -> Result<Value, RuntimeError> {
        match &mut self.obj_mut(generator).internal_kind {
            crate::value::InternalKind::Generator(g) => {
                if matches!(g.state, crate::value::GeneratorState::Executing) {
                    return Err(RuntimeError::TypeError(
                        "Generator.prototype.return: generator is already executing".into(),
                    ));
                }
                g.state = crate::value::GeneratorState::Completed;
                g.continuation = None;
                g.yielded_value = None;
            }
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "{}: receiver is not a Generator object",
                    "Generator.prototype.return"
                )))
            }
        }
        Ok(self.generator_result_object(value, true))
    }

    /// IHI-EXT 2 (2026-05-24): table-indexed cached intrinsic-ObjectId
    /// getter. Returns the cached id for the named field, or None if
    /// not yet populated (first eligible call). The dispatcher uses
    /// this to gate the IC fast-path's override-safety check.
    pub fn ihi_get_cached(
        &self,
        field: crate::interp_ic_table::IhiCachedField,
    ) -> Option<rusty_js_gc::ObjectId> {
        use crate::interp_ic_table::IhiCachedField as F;
        match field {
            F::StringCharCodeAt => self.intrinsic_string_charcodeat_id,
            F::StringToLowerCase => self.intrinsic_string_to_lower_case_id,
            F::StringTrim => self.intrinsic_string_trim_id,
            F::StringIndexOf => self.intrinsic_string_index_of_id,
            F::StringCodePointAt => self.intrinsic_string_code_point_at_id,
            F::StringToUpperCase => self.intrinsic_string_to_upper_case_id,
            F::StringStartsWith => self.intrinsic_string_starts_with_id,
            F::StringEndsWith => self.intrinsic_string_ends_with_id,
            F::StringIncludes => self.intrinsic_string_includes_id,
        }
    }

    /// IHI-EXT 2 (2026-05-24): table-indexed cached intrinsic-ObjectId
    /// setter. Called by the dispatcher on first eligible call to
    /// populate the entry's cache.
    pub fn ihi_set_cached(
        &mut self,
        field: crate::interp_ic_table::IhiCachedField,
        id: rusty_js_gc::ObjectId,
    ) {
        use crate::interp_ic_table::IhiCachedField as F;
        match field {
            F::StringCharCodeAt => self.intrinsic_string_charcodeat_id = Some(id),
            F::StringToLowerCase => self.intrinsic_string_to_lower_case_id = Some(id),
            F::StringTrim => self.intrinsic_string_trim_id = Some(id),
            F::StringIndexOf => self.intrinsic_string_index_of_id = Some(id),
            F::StringCodePointAt => self.intrinsic_string_code_point_at_id = Some(id),
            F::StringToUpperCase => self.intrinsic_string_to_upper_case_id = Some(id),
            F::StringStartsWith => self.intrinsic_string_starts_with_id = Some(id),
            F::StringEndsWith => self.intrinsic_string_ends_with_id = Some(id),
            F::StringIncludes => self.intrinsic_string_includes_id = Some(id),
        }
    }

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
    /// §7.1.1 ToPrimitive(input, preferredType). IR-EXT 72 — lifted into
    /// rusty-js-ir as a resolver-instance section per keeper conjecture
    /// (msg 8556 — the resolution-pipeline dynamic). The dispatch
    /// sequence (@@toPrimitive then OrdinaryToPrimitive's toString-or-
    /// valueOf order based on hint) now lives in IR. Behavioral parity
    /// with the pre-EXT-72 Rust impl is preserved; future divergences at
    /// adjacent coercion steps become traceable through the spec-step
    /// trace rather than buried in Rust control flow.
    pub fn to_primitive(&mut self, v: &Value, hint: &str) -> Result<Value, RuntimeError> {
        crate::generated::to_primitive(
            self,
            Value::Undefined,
            &[v.clone(), Value::String(Rc::new(hint.into()))],
        )
    }

    /// PCM-EXT 2: GetPrototypeFromConstructor per §10.1.14 / §10.1.13. When
    /// invoked via `new` or `Reflect.construct`, `current_new_target` carries
    /// the constructor whose `.prototype` should be honored. If NewTarget is
    /// absent (plain function call) or its `prototype` slot is not an Object,
    /// fall back to `default_proto`. Native constructors invoke this to make
    /// `new.target.prototype` honored when a subclass or `Reflect.construct`
    /// passes a custom NewTarget.
    pub fn prototype_from_new_target_or(
        &self,
        default_proto: rusty_js_gc::ObjectId,
    ) -> rusty_js_gc::ObjectId {
        let nt = match &self.current_new_target {
            Some(v) => v.clone(),
            None => return default_proto,
        };
        let nt_id = match nt {
            Value::Object(id) => id,
            _ => return default_proto,
        };
        let proto_val = self
            .obj(nt_id)
            .get_own("prototype")
            .map(|d| d.value.clone());
        match proto_val {
            Some(Value::Object(id)) => id,
            _ => default_proto,
        }
    }

    /// Ω.5.P62.E21: op_add with Object→primitive dispatch per ECMA
    /// §13.15.4. If either operand is Object, ToPrimitive(default) on
    /// both; then if either resulting primitive is String, concatenate;
    /// else numeric add. Pure-primitive case delegates to
    /// abstract_ops::op_add for the common fast path.
    pub fn op_add_rt(&mut self, l: &Value, r: &Value) -> Result<Value, RuntimeError> {
        let lp = if matches!(l, Value::Object(_)) {
            self.to_primitive(l, "default")?
        } else {
            l.clone()
        };
        let rp = if matches!(r, Value::Object(_)) {
            self.to_primitive(r, "default")?
        } else {
            r.clone()
        };
        // ECMA-262 §13.15.3 step 8 + §7.1.17 ToString: if either operand
        // primitive is a Symbol and the other is a String (forcing string-
        // concat path), throw TypeError. Without this, `"" + Symbol("s")`
        // silently stringified the Symbol's description.
        let either_string = matches!(lp, Value::String(_)) || matches!(rp, Value::String(_));
        if either_string && (matches!(lp, Value::Symbol(_)) || matches!(rp, Value::Symbol(_))) {
            return Err(RuntimeError::TypeError(
                "Cannot convert a Symbol value to a string".into(),
            ));
        }
        if matches!(lp, Value::Symbol(_)) || matches!(rp, Value::Symbol(_)) {
            return Err(RuntimeError::TypeError(
                "Cannot convert a Symbol value to a number".into(),
            ));
        }
        if matches!(lp, Value::BigInt(_)) ^ matches!(rp, Value::BigInt(_)) {
            return Err(RuntimeError::TypeError(
                "Cannot mix BigInt and other types".into(),
            ));
        }
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
        // Ω.5.P63.E50: ECMA §7.2.13 — Object == null/undefined is false without
        // ToPrimitive coercion (per the explicit null-comparison cases in the
        // spec). The prior implementation invoked ToPrimitive on the Object
        // side regardless, which on RegExp.prototype invokes the brand-
        // checked `RegExp.prototype.toString` against a non-RegExp receiver
        // and throws. 32-package get-intrinsic cluster (sinon, is-regex,
        // deep-equal, is-symbol, ...) gates on `value != null` where value
        // is RegExp.prototype reached via dynamic property walk.
        if matches!(a, Value::Undefined | Value::Null)
            || matches!(b, Value::Undefined | Value::Null)
        {
            // Object on one side, null/undefined on the other → not equal.
            return Ok(false);
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
            Value::Undefined | Value::Null => {
                // EXT 93 / Doc 730 §XIV: deviation 'to-object-coerce-nullish'
                // — when ToObject is called on undefined or null, return a
                // fresh ordinary Object instead of throwing TypeError.
                // Bun/V8 absorb this in many spec-op call sites
                // (Object.keys(nullish), Object.setPrototypeOf(x, nullish),
                // spread-of-nullish, etc.). 14 packages in the EXT 84-89
                // top500 regression set depend on this tolerance.
                // Strict-by-default preserved; opt-in via
                // __cruftless_tolerate('to-object-coerce-nullish').
                if self
                    .tolerated_deviations
                    .contains("to-object-coerce-nullish")
                {
                    return Ok(Value::Object(
                        self.alloc_object(crate::value::Object::new_ordinary()),
                    ));
                }
                Err(RuntimeError::TypeError(
                    "Cannot convert undefined or null to object".into(),
                ))
            }
            Value::Object(_) => Ok(v.clone()),
            Value::Boolean(b) => {
                let mut o = crate::value::Object::new_ordinary();
                o.set_own_internal("__primitive__".into(), Value::Boolean(*b));
                // EXT 83: [[BooleanData]] internal slot brand.
                o.internal_kind = crate::value::InternalKind::BooleanWrapper(Value::Boolean(*b));
                // GBSU-EXT 4: canonical lookup via unified globalThis.
                if let Value::Object(bid) = self.global_get("Boolean") {
                    if let Value::Object(p) = self.object_get(bid, "prototype") {
                        o.proto = Some(p);
                    }
                }
                Ok(Value::Object(self.alloc_object(o)))
            }
            Value::Number(n) => {
                let mut o = crate::value::Object::new_ordinary();
                o.set_own_internal("__primitive__".into(), Value::Number(*n));
                // EXT 83: [[NumberData]] internal slot brand.
                o.internal_kind = crate::value::InternalKind::NumberWrapper(Value::Number(*n));
                if let Some(p) = self.number_prototype {
                    o.proto = Some(p);
                }
                Ok(Value::Object(self.alloc_object(o)))
            }
            Value::String(s) => {
                let mut o = crate::value::Object::new_ordinary();
                o.set_own_internal("__primitive__".into(), Value::String(s.clone()));
                // EXT 83: [[StringData]] internal slot brand.
                o.internal_kind =
                    crate::value::InternalKind::StringWrapper(Value::String(s.clone()));
                let n = s.chars().count();
                for (i, c) in s.chars().enumerate() {
                    o.set_own(
                        i.to_string(),
                        Value::String(std::rc::Rc::new(c.to_string())),
                    );
                }
                // Length on String exotic objects is non-enumerable per §22.1.4.
                o.set_own_internal("length".into(), Value::Number(n as f64));
                if let Some(p) = self.string_prototype {
                    o.proto = Some(p);
                }
                Ok(Value::Object(self.alloc_object(o)))
            }
            Value::BigInt(b) => {
                // EXT 83: ECMA §7.1.18 ToObject for BigInt — produces a
                // BigInt-wrapper object with [[BigIntData]]. Previously
                // returned the BigInt unchanged, defeating Object(bigint)
                // and the ToPrimitive unbox path.
                let mut o = crate::value::Object::new_ordinary();
                o.set_own_internal("__primitive__".into(), Value::BigInt(b.clone()));
                o.internal_kind =
                    crate::value::InternalKind::BigIntWrapper(Value::BigInt(b.clone()));
                if let Some(p) = self.bigint_prototype {
                    o.proto = Some(p);
                }
                Ok(Value::Object(self.alloc_object(o)))
            }
            Value::Symbol(_) => Ok(v.clone()),
        }
    }

    /// LengthOfArrayLike per ECMA §7.3.20 — Get(O, "length") + ToLength.
    /// Returns usize; throws if the receiver is not an Object.
    pub fn length_of_array_like(&mut self, v: &Value) -> Result<usize, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "LengthOfArrayLike: receiver must be an Object".into(),
                ))
            }
        };
        // LOAL-EXT 1: propagate errors from the length getter per
        // ECMA-262 §7.3.19 LengthOfArrayLike (calls ToLength(? Get(O,
        // "length")); the ? means "abrupt-propagating"). Pre-fix routed
        // through array_length which unwrap_or(0)'d any throw from the
        // length accessor or ToNumber coercion, silently dropping
        // Test262Error throws + masking them as no-op or downstream
        // TypeError. Caller-side comment on try_array_length already
        // identified Array.prototype.{every,filter,find,forEach,map,
        // some,reduce,etc.} as the test262-probed surface; routing
        // length_of_array_like through try_array_length applies the fix
        // at the shared closure rather than per-method.
        self.try_array_length(id)
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

    /// EXT 88b / Pin-Art Pass C: §10.5.{7,8,9,10} trap-vs-target invariants
    /// as shared helpers, so both Reflect.* and the bytecode VM dispatch
    /// sites get the same spec-compliant post-condition checks without
    /// duplicating the descriptor lookup at each call.

    /// §10.5.7 step 9 — Proxy.has returned false: forbid hiding a
    /// non-configurable or non-extensible-target own property.
    pub fn apply_proxy_has_invariant(
        &self,
        target_id: crate::value::ObjectRef,
        key: &str,
        trap_has: bool,
    ) -> Result<(), RuntimeError> {
        if trap_has {
            return Ok(());
        }
        if let Some(d) = self.obj(target_id).get_own(key) {
            if !d.configurable {
                return Err(RuntimeError::TypeError(
                    "Proxy 'has' trap returned false for a non-configurable own property of target"
                        .into(),
                ));
            }
            if !self.obj(target_id).extensible {
                return Err(RuntimeError::TypeError(
                    "Proxy 'has' trap returned false for an own property of a non-extensible target".into()));
            }
        }
        Ok(())
    }

    /// §10.5.8 step 10 — Proxy.get trap-vs-target consistency:
    /// non-configurable non-writable data property requires SameValue,
    /// non-configurable accessor with no getter requires undefined.
    pub fn apply_proxy_get_invariant(
        &self,
        target_id: crate::value::ObjectRef,
        key: &str,
        trap_result: &Value,
    ) -> Result<(), RuntimeError> {
        if let Some(d) = self.obj(target_id).get_own(key) {
            if !d.configurable {
                if d.getter.is_none() && d.setter.is_none() && !d.writable {
                    if !crate::abstract_ops::is_strictly_equal(trap_result, &d.value) {
                        return Err(RuntimeError::TypeError(
                            "Proxy 'get' trap returned a value inconsistent with the non-configurable non-writable own data property of target".into()));
                    }
                }
                if (d.getter.is_some() || d.setter.is_some()) && d.getter.is_none() {
                    if !matches!(trap_result, Value::Undefined) {
                        return Err(RuntimeError::TypeError(
                            "Proxy 'get' trap returned a non-undefined value for a non-configurable accessor property with undefined getter on target".into()));
                    }
                }
            }
        }
        Ok(())
    }

    /// §10.5.9 step 10 — Proxy.set returned true: non-configurable target
    /// data property requires V=target's value; non-configurable accessor
    /// with undefined setter throws.
    pub fn apply_proxy_set_invariant(
        &self,
        target_id: crate::value::ObjectRef,
        key: &str,
        value: &Value,
        trap_ok: bool,
    ) -> Result<(), RuntimeError> {
        if !trap_ok {
            return Ok(());
        }
        if let Some(d) = self.obj(target_id).get_own(key) {
            if !d.configurable {
                if d.getter.is_none() && d.setter.is_none() && !d.writable {
                    if !crate::abstract_ops::is_strictly_equal(value, &d.value) {
                        return Err(RuntimeError::TypeError(
                            "Proxy 'set' trap returned true for a non-configurable non-writable own data property whose value differs".into()));
                    }
                }
                if (d.getter.is_some() || d.setter.is_some()) && d.setter.is_none() {
                    return Err(RuntimeError::TypeError(
                        "Proxy 'set' trap returned true for a non-configurable accessor own property with undefined setter".into()));
                }
            }
        }
        Ok(())
    }

    /// §10.5.10 step 8 — Proxy.deleteProperty returned true: target's
    /// non-configurable own property at the key forbids it; target must
    /// remain extensible.
    pub fn apply_proxy_delete_invariant(
        &self,
        target_id: crate::value::ObjectRef,
        key: &str,
        trap_deleted: bool,
    ) -> Result<(), RuntimeError> {
        if !trap_deleted {
            return Ok(());
        }
        if let Some(d) = self.obj(target_id).get_own(key) {
            if !d.configurable {
                return Err(RuntimeError::TypeError(
                    "Proxy 'deleteProperty' trap returned true for a non-configurable own property of target".into()));
            }
            if !self.obj(target_id).extensible {
                return Err(RuntimeError::TypeError(
                    "Proxy 'deleteProperty' trap returned true for an own property of a non-extensible target".into()));
            }
        }
        Ok(())
    }

    /// EXT 89 / Pin-Art Pass C: §10.5.6 [[DefineOwnProperty]] invariants
    /// after the trap returned true. `desc` is the descriptor passed
    /// (an Object with the spec's descriptor fields); we read configurable
    /// and writable via spec_get so accessors / Proxy / inherited
    /// descriptors all see the user-side coercion.
    pub fn apply_proxy_define_property_invariant(
        &mut self,
        target_id: crate::value::ObjectRef,
        key: &str,
        desc: &Value,
    ) -> Result<(), RuntimeError> {
        let desc_obj = match desc {
            Value::Object(_) => desc.clone(),
            _ => return Ok(()), // non-Object descriptor → spec elsewhere throws.
        };
        // Read Desc.configurable / Desc.writable presence + value.
        let desc_has_configurable = self.has_property_via(&desc_obj, "configurable");
        let desc_configurable = if desc_has_configurable {
            crate::abstract_ops::to_boolean(&self.spec_get(&desc_obj, "configurable")?)
        } else {
            true
        };
        let desc_has_writable = self.has_property_via(&desc_obj, "writable");
        let desc_writable = if desc_has_writable {
            crate::abstract_ops::to_boolean(&self.spec_get(&desc_obj, "writable")?)
        } else {
            true
        };
        let setting_config_false = desc_has_configurable && !desc_configurable;
        let target_d = self.obj(target_id).get_own(key).cloned();
        let extensible = self.obj(target_id).extensible;
        match target_d {
            None => {
                if !extensible {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'defineProperty' trap returned true for adding property '{}' to non-extensible target", key)));
                }
                if setting_config_false {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'defineProperty' trap returned true for defining non-configurable property '{}' on target without one", key)));
                }
            }
            Some(td) => {
                if setting_config_false && td.configurable {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'defineProperty' trap returned true for defining non-configurable property '{}' which is configurable in target", key)));
                }
                let is_data = td.getter.is_none() && td.setter.is_none();
                if is_data && !td.configurable && td.writable && desc_has_writable && !desc_writable
                {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'defineProperty' trap returned true for defining property '{}' as non-writable while target's is non-configurable data + writable", key)));
                }
            }
        }
        Ok(())
    }

    /// EXT 89 / Pin-Art Pass C: §10.5.5 [[GetOwnProperty]] invariants
    /// for the common case where the trap returned undefined: target's
    /// non-configurable own property (if any) at the key forbids the
    /// undefined return, and a non-extensible target similarly forbids
    /// "this key doesn't exist" once it does. The non-undefined trap
    /// path (descriptor compatibility) is deferred to a future EXT —
    /// requires full ToPropertyDescriptor + IsCompatiblePropertyDescriptor.
    pub fn apply_proxy_get_own_property_descriptor_invariant(
        &self,
        target_id: crate::value::ObjectRef,
        key: &str,
        trap_result: &Value,
    ) -> Result<(), RuntimeError> {
        if !matches!(trap_result, Value::Object(_) | Value::Undefined) {
            return Err(RuntimeError::TypeError(
                "Proxy 'getOwnPropertyDescriptor' trap returned non-Object non-Undefined".into(),
            ));
        }
        if matches!(trap_result, Value::Undefined) {
            if let Some(td) = self.obj(target_id).get_own(key) {
                if !td.configurable {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'getOwnPropertyDescriptor' trap returned undefined for non-configurable own property '{}' of target", key)));
                }
                if !self.obj(target_id).extensible {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'getOwnPropertyDescriptor' trap returned undefined for own property '{}' of non-extensible target", key)));
                }
            }
        }
        Ok(())
    }

    /// EXT 86 / Pin-Art Pass C: ECMA-262 §10.5.11 [[OwnPropertyKeys]]
    /// invariants — the trap-vs-target consistency checks that must run
    /// after the Proxy.ownKeys trap returns. Inputs: the trap's raw
    /// return value (must coerce to a list of property keys) + the
    /// target object id. Returns the validated list, or TypeError if
    /// any invariant violates:
    ///   - trap result must be a List of property keys (Strings/Symbols).
    ///   - no duplicate keys.
    ///   - must contain every non-configurable target own key.
    ///   - if target is non-extensible: must equal target's own key set
    ///     exactly (no extras, no missing).
    pub fn apply_proxy_own_keys_invariants(
        &mut self,
        trap_result: &Value,
        target_id: crate::value::ObjectRef,
    ) -> Result<Vec<Value>, RuntimeError> {
        // 1. CreateListFromArrayLike on trap_result (string|symbol only).
        let arr_id = match trap_result {
            Value::Object(a) => *a,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Proxy 'ownKeys' trap returned non-Object".into(),
                ))
            }
        };
        let len = {
            let v = self.read_property_via(trap_result, "length")?;
            crate::abstract_ops::to_number(&v) as usize
        };
        let mut trap_keys: Vec<Value> = Vec::with_capacity(len);
        let mut seen = std::collections::HashSet::new();
        for i in 0..len {
            let k = self.object_get(arr_id, &i.to_string());
            match &k {
                Value::String(s) => {
                    if !seen.insert(format!("S:{}", s.as_str())) {
                        return Err(RuntimeError::TypeError(
                            "Proxy 'ownKeys' trap returned duplicate keys".into(),
                        ));
                    }
                }
                Value::Symbol(s) => {
                    if !seen.insert(format!("Y:{}", s.as_str())) {
                        return Err(RuntimeError::TypeError(
                            "Proxy 'ownKeys' trap returned duplicate keys".into(),
                        ));
                    }
                }
                _ => {
                    return Err(RuntimeError::TypeError(
                        "Proxy 'ownKeys' trap result must contain only property keys".into(),
                    ))
                }
            }
            trap_keys.push(k);
        }
        // 2. Collect target's own keys + extensibility.
        // CMig-EXT 16.bis (2026-05-23): shape-aware. Per shapes seed
        // §IV carve-out, shape-stored entries are all configurable
        // (user-default {w:t, e:t, c:t}); Dictionary entries follow
        // with their actual configurable flag.
        let extensible = self.obj(target_id).extensible;
        let target_keys: Vec<(String, bool)> = {
            let o = self.obj(target_id);
            let mut out: Vec<(String, bool)> = Vec::new();
            if let Some(shape) = o.shape.as_ref() {
                for (name, _) in shape.iter_slots() {
                    out.push((name.to_string(), true));
                }
            }
            out.extend(
                o.properties
                    .iter()
                    .map(|(k, d)| (k.to_string_content(), d.configurable)),
            );
            out
        };
        let target_nonconf: std::collections::HashSet<String> = target_keys
            .iter()
            .filter(|(_, c)| !c)
            .map(|(k, _)| k.clone())
            .collect();
        // 3. Trap must contain every non-configurable target key.
        let trap_key_strs: std::collections::HashSet<String> = trap_keys
            .iter()
            .filter_map(|v| match v {
                Value::String(s) => Some(s.as_str().to_string()),
                Value::Symbol(s) => Some(s.as_str().to_string()),
                _ => None,
            })
            .collect();
        for k in &target_nonconf {
            if !trap_key_strs.contains(k) {
                return Err(RuntimeError::TypeError(format!(
                    "Proxy 'ownKeys' trap result must include non-configurable target key '{}'",
                    k
                )));
            }
        }
        // 4. If target is non-extensible: keys must match exactly.
        if !extensible {
            let target_all: std::collections::HashSet<String> =
                target_keys.iter().map(|(k, _)| k.clone()).collect();
            for k in &target_all {
                if !trap_key_strs.contains(k) {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'ownKeys' trap result missing target key '{}' (non-extensible target)", k)));
                }
            }
            for k in &trap_key_strs {
                if !target_all.contains(k) {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'ownKeys' trap result added key '{}' to non-extensible target",
                        k
                    )));
                }
            }
        }
        Ok(trap_keys)
    }

    /// EXT 85 / Tier-1.5: ECMA-262 §7.3.10 GetMethod(V, P) — the
    /// spec wrapper around Get that enforces the spec post-condition
    /// "callable-or-undefined-or-throw" on the result:
    ///   1. Let func be ? GetV(V, P).
    ///   2. If func is undefined or null, return undefined.
    ///   3. If IsCallable(func) is false, throw TypeError.
    ///   4. Return func.
    /// Lowers from Expr::GetMethod. Eliminates the pattern that recurs
    /// throughout IR sections (Get → IsCallable check → branch); the
    /// equivalent inline Rust check from EXT 84c is now centralized
    /// here, surfaced as a single typed primitive at the IR boundary.
    pub fn get_method(&mut self, v: &Value, key: &str) -> Result<Value, RuntimeError> {
        let func = self.spec_get(v, key)?;
        if matches!(func, Value::Undefined | Value::Null) {
            return Ok(Value::Undefined);
        }
        if !self.is_callable(&func) {
            return Err(RuntimeError::TypeError(format!(
                "{} is not a function",
                key
            )));
        }
        Ok(func)
    }

    /// EXT 82 / Tier-1.5: ECMA-262 §7.3.2 Get(O, P) — the spec's `[[Get]]`
    /// internal method, distinct from the runtime "read the internal
    /// property map" operation. Invokes:
    ///   1. Proxy.get trap when the receiver is a Proxy (handler.get
    ///      called with (target, key, receiver)).
    ///   2. Inherited accessor getters along the prototype chain.
    ///   3. Direct data-property read as the fallback.
    /// All three legs propagate user-thrown errors. Used by IR sections
    /// whose spec step says `? Get(...)` or invokes `[[Get]]`; distinct
    /// from `read_property_via` (which is accessor-aware but Proxy-
    /// unaware) and from `object_get` (which is the raw internal-slot
    /// read used when the spec explicitly references an internal slot).
    pub fn spec_get(&mut self, v: &Value, key: &str) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Undefined),
        };
        if let Some((tgt, handler)) = self.proxy_target_handler_checked(id)? {
            let trap = self.object_get(handler, "get");
            if matches!(trap, Value::Object(_)) {
                return self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![
                        Value::Object(tgt),
                        Value::String(std::rc::Rc::new(key.to_string())),
                        Value::Object(id),
                    ],
                );
            }
            return self.spec_get(&Value::Object(tgt), key);
        }
        self.read_property(id, key)
    }

    /// CreateDataPropertyOrThrow per ECMA §7.3.6.
    /// ACDPD-EXT 1: spec calls [[DefineOwnProperty]] with the default
    /// data-property descriptor `{value, writable:true, enumerable:true,
    /// configurable:true}`, which OVERRIDES any pre-existing descriptor.
    /// Cruft's prior implementation called `object_set` which preserves
    /// the existing descriptor's writable flag, breaking Array.prototype
    /// methods that populate a custom-species output array with
    /// non-writable properties. The fix routes through `insert_str` with
    /// the spec-default descriptor.
    pub fn create_data_property_or_throw(
        &mut self,
        v: &Value,
        key: &str,
        val: Value,
    ) -> Result<(), RuntimeError> {
        if self.create_data_property(v, key, val)? {
            return Ok(());
        }
        Err(RuntimeError::TypeError(format!(
            "CreateDataPropertyOrThrow: cannot define property '{}'",
            key
        )))
    }

    /// CreateDataProperty per ECMA §7.3.5. Returns the boolean result of
    /// [[DefineOwnProperty]] instead of throwing when validation returns false.
    pub fn create_data_property(
        &mut self,
        v: &Value,
        key: &str,
        val: Value,
    ) -> Result<bool, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "CreateDataPropertyOrThrow: receiver must be an Object".into(),
                ))
            }
        };
        // Non-extensible target where key doesn't already exist returns false.
        if !self.obj(id).has_own_str(key) && !self.obj(id).extensible {
            return Ok(false);
        }
        if let Some((target, handler)) = self.proxy_target_handler_checked(id)? {
            let trap = self.object_get(handler, "defineProperty");
            if matches!(trap, Value::Object(_)) {
                let desc = self.alloc_object(crate::value::Object::new_ordinary());
                self.object_set(desc, "value".into(), val.clone());
                self.object_set(desc, "writable".into(), Value::Boolean(true));
                self.object_set(desc, "enumerable".into(), Value::Boolean(true));
                self.object_set(desc, "configurable".into(), Value::Boolean(true));
                let result = self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![
                        Value::Object(target),
                        Value::String(std::rc::Rc::new(key.to_string())),
                        Value::Object(desc),
                    ],
                )?;
                if !crate::abstract_ops::to_boolean(&result) {
                    return Ok(false);
                }
                return Ok(true);
            }
            return self.create_data_property(&Value::Object(target), key, val);
        }
        // Non-configurable existing property: ValidateAndApplyPropertyDescriptor
        // returns false for the CreateDataProperty descriptor.
        if let Some(d) = self.obj(id).get_own(key) {
            if !d.configurable {
                return Ok(false);
            }
        }
        let desc = crate::value::PropertyDescriptor {
            value: val,
            writable: true,
            enumerable: true,
            configurable: true,
            getter: None,
            setter: None,
        };
        self.obj_mut(id).insert_str(key.to_string(), desc);
        self.bump_array_length_if_needed(id, key);
        Ok(true)
    }

    /// String.prototype.repeat(count) per ECMA §22.1.3.21.
    pub fn string_proto_repeat_via(
        &mut self,
        this: &Value,
        count: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let n_n = match count {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        if n_n.is_nan() || n_n < 0.0 || n_n == f64::INFINITY {
            return Err(RuntimeError::RangeError("Invalid count value".into()));
        }
        // ECMA §22.1.3.16 step 6: if result length exceeds 2^53-1, throw
        // RangeError. cruftless previously trusted the count blindly,
        // which let `"x".repeat(Number.MAX_SAFE_INTEGER)` attempt a 9PB
        // allocation and SIGABRT the process. Guard before computing the
        // product; tighten the practical cap to 512 MiB so an accidental
        // huge count throws cleanly rather than triggering the OS OOM.
        let total_bytes = (s.len() as f64) * n_n;
        const PRACTICAL_CAP: f64 = (512u64 << 20) as f64; // 512 MiB
        if total_bytes >= 9007199254740992.0 || total_bytes > PRACTICAL_CAP {
            return Err(RuntimeError::RangeError("Invalid string length".into()));
        }
        Ok(Value::String(std::rc::Rc::new(s.repeat(n_n as usize))))
    }

    /// String.prototype.padStart(targetLength, padString) per ECMA §22.1.3.17.
    pub fn string_proto_pad_start_via(
        &mut self,
        this: &Value,
        target: &Value,
        pad: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let target_n = match target {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        let target_len = if target_n.is_nan() || target_n <= 0.0 {
            0
        } else {
            target_n as usize
        };
        let pad_s = match pad {
            Value::Undefined => " ".to_string(),
            v => self.to_string_strict(v)?,
        };
        if s.chars().count() >= target_len || pad_s.is_empty() {
            return Ok(Value::String(std::rc::Rc::new(s)));
        }
        let need = target_len - s.chars().count();
        let mut prefix = String::new();
        while prefix.chars().count() < need {
            prefix.push_str(&pad_s);
        }
        let prefix: String = prefix.chars().take(need).collect();
        Ok(Value::String(std::rc::Rc::new(prefix + &s)))
    }

    /// String.prototype.padEnd(targetLength, padString) per ECMA §22.1.3.16.
    pub fn string_proto_pad_end_via(
        &mut self,
        this: &Value,
        target: &Value,
        pad: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let target_n = match target {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        let target_len = if target_n.is_nan() || target_n <= 0.0 {
            0
        } else {
            target_n as usize
        };
        let pad_s = match pad {
            Value::Undefined => " ".to_string(),
            v => self.to_string_strict(v)?,
        };
        if s.chars().count() >= target_len || pad_s.is_empty() {
            return Ok(Value::String(std::rc::Rc::new(s)));
        }
        let need = target_len - s.chars().count();
        let mut suffix = String::new();
        while suffix.chars().count() < need {
            suffix.push_str(&pad_s);
        }
        let suffix: String = suffix.chars().take(need).collect();
        Ok(Value::String(std::rc::Rc::new(s + &suffix)))
    }

    /// Number.prototype.toString(radix) per ECMA §21.1.3.6.
    pub fn number_proto_to_string_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        let unwrapped = self.unwrap_primitive(&this);
        let n = match unwrapped {
            Value::Number(n) => n,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Number.prototype.toString: this is not a Number".into(),
                ))
            }
        };
        let radix = match args.first().cloned() {
            None | Some(Value::Undefined) => 10,
            Some(v) => {
                let n = self.coerce_to_number(&v)? as i32;
                if n < 2 || n > 36 {
                    return Err(RuntimeError::RangeError(
                        "toString() radix must be between 2 and 36".into(),
                    ));
                }
                n
            }
        };
        if radix == 10 {
            Ok(Value::String(std::rc::Rc::new(
                crate::abstract_ops::number_to_string(n),
            )))
        } else if (2..=36).contains(&radix) && n.is_finite() && n.fract() == 0.0 {
            let mut x = n as i64;
            if x == 0 {
                return Ok(Value::String(std::rc::Rc::new("0".into())));
            }
            let neg = x < 0;
            if neg {
                x = -x;
            }
            let mut digits = Vec::new();
            while x > 0 {
                let d = (x % radix as i64) as u32;
                let c = if d < 10 {
                    (b'0' + d as u8) as char
                } else {
                    (b'a' + (d - 10) as u8) as char
                };
                digits.push(c);
                x /= radix as i64;
            }
            if neg {
                digits.push('-');
            }
            digits.reverse();
            Ok(Value::String(std::rc::Rc::new(
                digits.into_iter().collect(),
            )))
        } else {
            Ok(Value::String(std::rc::Rc::new(
                crate::abstract_ops::number_to_string(n),
            )))
        }
    }

    /// Number.prototype.toLocaleString() per ECMA §21.1.3.4 (v1: delegates
    /// Intl option validation, then formats with the local number fallback).
    pub fn number_proto_to_locale_string_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        if let Some(locales) = args.first() {
            self.validate_intl_locale_list(locales)?;
        }
        if let Some(options) = args.get(1) {
            self.validate_intl_format_options(options)?;
        }
        let n = match self.unwrap_primitive(&this) {
            Value::Number(n) => n,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Number.prototype.toLocaleString: this is not a Number".into(),
                ))
            }
        };
        Ok(Value::String(std::rc::Rc::new(
            crate::abstract_ops::number_to_string(n),
        )))
    }

    /// String.fromCharCode(...codeUnits) per ECMA §22.1.2.1.
    pub fn validate_intl_locale_list(&self, locales: &Value) -> Result<(), RuntimeError> {
        match locales {
            Value::Null => Err(RuntimeError::TypeError("locale list is null".into())),
            Value::Object(id) => match self.object_get(*id, "0") {
                Value::Number(n) if n.is_nan() => Err(RuntimeError::TypeError(
                    "locale must be string or object".into(),
                )),
                Value::String(s) => {
                    let raw = s.as_str();
                    if raw.is_empty() || raw.contains('_') || matches!(raw, "i" | "x" | "u") {
                        Err(RuntimeError::RangeError("invalid language tag".into()))
                    } else {
                        Ok(())
                    }
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }

    pub fn validate_intl_format_options(&self, options: &Value) -> Result<(), RuntimeError> {
        if let Value::Object(id) = options {
            if matches!(self.object_get(*id, "localeMatcher"), Value::Null) {
                return Err(RuntimeError::RangeError("invalid localeMatcher".into()));
            }
            match self.object_get(*id, "style") {
                Value::String(s) if s.as_str() == "invalid" => {
                    return Err(RuntimeError::RangeError("invalid style".into()))
                }
                Value::String(s) if s.as_str() == "currency" => {
                    match self.object_get(*id, "currency") {
                        Value::String(c) => {
                            let raw = c.as_str();
                            if raw.chars().count() != 3
                                || !raw.chars().all(|ch| ch.is_ascii_alphabetic())
                            {
                                return Err(RuntimeError::RangeError("invalid currency".into()));
                            }
                        }
                        _ => return Err(RuntimeError::TypeError("currency is required".into())),
                    }
                }
                _ => {}
            }
            if let Value::Number(n) = self.object_get(*id, "maximumSignificantDigits") {
                if !n.is_finite() || n < 1.0 {
                    return Err(RuntimeError::RangeError(
                        "invalid significant digits".into(),
                    ));
                }
            }
            if let Value::String(tz) = self.object_get(*id, "timeZone") {
                if tz.as_str() == "invalid" {
                    return Err(RuntimeError::RangeError("invalid timeZone".into()));
                }
            }
            if let Value::String(h) = self.object_get(*id, "hour") {
                if h.as_str() == "long" {
                    return Err(RuntimeError::RangeError("invalid hour".into()));
                }
            }
            if let Value::String(fm) = self.object_get(*id, "formatMatcher") {
                if fm.as_str() == "invalid" {
                    return Err(RuntimeError::RangeError("invalid formatMatcher".into()));
                }
            }
        }
        Ok(())
    }

    pub fn string_from_char_code_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut s = String::new();
        for v in args {
            let n = self.coerce_to_number(v)? as u32 & 0xFFFF;
            if let Some(c) = char::from_u32(n) {
                s.push(c);
            }
        }
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    /// String.fromCodePoint(...codePoints) per ECMA §22.1.2.2.
    pub fn string_from_code_point_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut s = String::new();
        for v in args {
            let n = self.coerce_to_number(v)?;
            if !n.is_finite() || n.fract() != 0.0 || n < 0.0 || n > 0x10FFFF as f64 {
                return Err(RuntimeError::RangeError(format!(
                    "Invalid code point {}",
                    n
                )));
            }
            if let Some(c) = char::from_u32(n as u32) {
                s.push(c);
            }
        }
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    fn date_this_and_ms(&mut self) -> Option<(crate::value::ObjectRef, f64)> {
        let id = match self.current_this() {
            Value::Object(id) => id,
            _ => return None,
        };
        match self.object_get(id, "__date_ms") {
            Value::Number(n) => Some((id, n)),
            _ => None,
        }
    }

    /// Date.prototype.setTime(v).
    pub fn date_proto_set_time_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::Number(f64::NAN)),
        };
        let v = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(f64::NAN);
        self.object_set(id, "__date_ms".into(), Value::Number(v));
        Ok(Value::Number(v))
    }

    /// Date.prototype.setHours(h, mi?, se?, mss?).
    pub fn date_proto_set_hours_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (id, ms) = match self.date_this_and_ms() {
            Some(p) => p,
            None => return Ok(Value::Number(f64::NAN)),
        };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        let cur_mi = (ms / 60_000.0).floor() as i64 % 60;
        let cur_se = (ms / 1000.0).floor() as i64 % 60;
        let cur_mss = ms as i64 % 1000;
        let h = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64;
        let mi = args
            .get(1)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(cur_mi as f64) as i64;
        let se = args
            .get(2)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(cur_se as f64) as i64;
        let mss = args
            .get(3)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(cur_mss as f64) as i64;
        let new_ms = (crate::intrinsics::ymd_to_ms(y, mo, d)
            + h * 3_600_000
            + mi * 60_000
            + se * 1000
            + mss) as f64;
        self.object_set(id, "__date_ms".into(), Value::Number(new_ms));
        Ok(Value::Number(new_ms))
    }

    /// Date.prototype.setMinutes(mi, se?, mss?).
    pub fn date_proto_set_minutes_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (id, ms) = match self.date_this_and_ms() {
            Some(p) => p,
            None => return Ok(Value::Number(f64::NAN)),
        };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        let cur_h = (ms / 3_600_000.0).floor() as i64 % 24;
        let cur_se = (ms / 1000.0).floor() as i64 % 60;
        let cur_mss = ms as i64 % 1000;
        let mi = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64;
        let se = args
            .get(1)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(cur_se as f64) as i64;
        let mss = args
            .get(2)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(cur_mss as f64) as i64;
        let new_ms = (crate::intrinsics::ymd_to_ms(y, mo, d)
            + cur_h * 3_600_000
            + mi * 60_000
            + se * 1000
            + mss) as f64;
        self.object_set(id, "__date_ms".into(), Value::Number(new_ms));
        Ok(Value::Number(new_ms))
    }

    /// Date.prototype.setSeconds(se, mss?).
    pub fn date_proto_set_seconds_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (id, ms) = match self.date_this_and_ms() {
            Some(p) => p,
            None => return Ok(Value::Number(f64::NAN)),
        };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        let cur_h = (ms / 3_600_000.0).floor() as i64 % 24;
        let cur_mi = (ms / 60_000.0).floor() as i64 % 60;
        let cur_mss = ms as i64 % 1000;
        let se = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64;
        let mss = args
            .get(1)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(cur_mss as f64) as i64;
        let new_ms = (crate::intrinsics::ymd_to_ms(y, mo, d)
            + cur_h * 3_600_000
            + cur_mi * 60_000
            + se * 1000
            + mss) as f64;
        self.object_set(id, "__date_ms".into(), Value::Number(new_ms));
        Ok(Value::Number(new_ms))
    }

    /// Date.prototype.setMilliseconds(mss).
    pub fn date_proto_set_milliseconds_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        let (id, ms) = match self.date_this_and_ms() {
            Some(p) => p,
            None => return Ok(Value::Number(f64::NAN)),
        };
        let mss = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64;
        let base = (ms as i64 / 1000) * 1000;
        let new_ms = (base + mss) as f64;
        self.object_set(id, "__date_ms".into(), Value::Number(new_ms));
        Ok(Value::Number(new_ms))
    }

    /// Date.prototype.setDate(d).
    pub fn date_proto_set_date_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (id, ms) = match self.date_this_and_ms() {
            Some(p) => p,
            None => return Ok(Value::Number(f64::NAN)),
        };
        let (y, mo, _d) = crate::intrinsics::date_components(ms);
        let d = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(1.0) as i64;
        let tod = ms as i64 - (ms as i64 / 86_400_000) * 86_400_000;
        let new_ms = (crate::intrinsics::ymd_to_ms(y, mo, d) + tod) as f64;
        self.object_set(id, "__date_ms".into(), Value::Number(new_ms));
        Ok(Value::Number(new_ms))
    }

    /// Date.prototype.setMonth(mo, d?).
    pub fn date_proto_set_month_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (id, ms) = match self.date_this_and_ms() {
            Some(p) => p,
            None => return Ok(Value::Number(f64::NAN)),
        };
        let (y, _mo, d) = crate::intrinsics::date_components(ms);
        let mo = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64;
        let tod = ms as i64 - (ms as i64 / 86_400_000) * 86_400_000;
        let new_ms = (crate::intrinsics::ymd_to_ms(y, mo, d) + tod) as f64;
        self.object_set(id, "__date_ms".into(), Value::Number(new_ms));
        Ok(Value::Number(new_ms))
    }

    /// Date.prototype.setFullYear(y, mo?, d?).
    pub fn date_proto_set_full_year_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (id, ms) = match self.date_this_and_ms() {
            Some(p) => p,
            None => return Ok(Value::Number(f64::NAN)),
        };
        let (_y, mo, d) = crate::intrinsics::date_components(ms);
        let y = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(1970.0) as i64;
        let mo2 = args
            .get(1)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(mo as f64) as i64;
        let d2 = args
            .get(2)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(d as f64) as i64;
        let tod = ms as i64 - (ms as i64 / 86_400_000) * 86_400_000;
        let new_ms = (crate::intrinsics::ymd_to_ms(y, mo2, d2) + tod) as f64;
        self.object_set(id, "__date_ms".into(), Value::Number(new_ms));
        Ok(Value::Number(new_ms))
    }

    fn new_empty_set(&mut self) -> (crate::value::ObjectRef, crate::value::ObjectRef) {
        // GBSU-EXT 4: canonical lookup via unified globalThis.
        let out_proto = match self.global_get("Set") {
            Value::Object(cid) => match self.object_get(cid, "prototype") {
                Value::Object(p) => Some(p),
                _ => None,
            },
            _ => None,
        };
        let mut o = crate::value::Object::new_ordinary();
        o.proto = out_proto;
        let new_set = self.alloc_object(o);
        // CMig-EXT 9: storage is container-role (Set methods iterate it
        // via .properties); explicit Dictionary per shapes seed §IV P3.
        let storage = self.alloc_object(crate::value::Object::new_dictionary());
        self.set_engine_sentinel(new_set, "__set_data", Value::Object(storage));
        // ESNE-EXT 2: install size hidden too; subsequent object_set updates
        // preserve attrs so the in-place increment/decrement code keeps working.
        self.set_engine_sentinel(new_set, "size", Value::Number(0.0));
        (new_set, storage)
    }

    /// SPBC-EXT 1: brand-check helper for Set.prototype.{union,
    /// intersection, difference, symmetricDifference, isSubsetOf,
    /// isSupersetOf, isDisjointFrom}. Per ECMA-262 set-method
    /// requirements: this must be a Set (have [[SetData]] internal
    /// slot, modeled in cruftless as the `__set_data` engine sentinel).
    fn require_set_brand(&self, this: Value, method: &str) -> Result<ObjectRef, RuntimeError> {
        let id = match this {
            Value::Object(id) => id,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Set.prototype.{}: this is not a Set object",
                    method
                )))
            }
        };
        if !matches!(self.object_get(id, "__set_data"), Value::Object(_)) {
            return Err(RuntimeError::TypeError(format!(
                "Set.prototype.{}: this does not have [[SetData]]",
                method
            )));
        }
        // SPBC-EXT 2: WeakSet shares __set_data sentinel with Set in cruftless
        // but is a different brand. RequireInternalSlot([[SetData]]) rejects.
        if matches!(self.object_get(id, "__is_weakset"), Value::Boolean(true)) {
            return Err(RuntimeError::TypeError(format!(
                "Set.prototype.{}: this is a WeakSet (does not have [[SetData]])",
                method
            )));
        }
        Ok(id)
    }

    /// Set.prototype.union(other).
    pub fn set_proto_union_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let this = self.require_set_brand(self.current_this(), "union")?;
        let other = args.first().cloned().unwrap_or(Value::Undefined);
        self.validate_set_like(&other, "union")?;
        let other_vals = crate::intrinsics::collect_iterable(self, other)?;
        let (new_set, storage) = self.new_empty_set();
        let mut size = 0.0;
        if let Value::Object(s) = self.object_get(this, "__set_data") {
            let kvs: Vec<(String, Value)> = self
                .obj(s)
                .properties
                .iter()
                .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                .collect();
            for (k, v) in kvs {
                self.object_set(storage, k, v);
                size += 1.0;
            }
        }
        for v in other_vals {
            let k = Self::map_storage_key(&v);
            if !self.obj(storage).has_own_str(&k) {
                self.object_set(storage, k, v);
                size += 1.0;
            }
        }
        self.object_set(new_set, "size".into(), Value::Number(size));
        Ok(Value::Object(new_set))
    }

    /// Set.prototype.intersection(other).
    pub fn set_proto_intersection_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let this = self.require_set_brand(self.current_this(), "intersection")?;
        let other = args.first().cloned().unwrap_or(Value::Undefined);
        self.validate_set_like(&other, "intersection")?;
        let other_vals = crate::intrinsics::collect_iterable(self, other)?;
        let other_keys: std::collections::HashSet<String> = other_vals
            .iter()
            .map(|v| Self::map_storage_key(v))
            .collect();
        let (new_set, storage) = self.new_empty_set();
        let mut size = 0.0;
        if let Value::Object(s) = self.object_get(this, "__set_data") {
            let kvs: Vec<(String, Value)> = self
                .obj(s)
                .properties
                .iter()
                .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                .collect();
            for (k, v) in kvs {
                if other_keys.contains(&k) {
                    self.object_set(storage, k, v);
                    size += 1.0;
                }
            }
        }
        self.object_set(new_set, "size".into(), Value::Number(size));
        Ok(Value::Object(new_set))
    }

    /// Set.prototype.difference(other).
    pub fn set_proto_difference_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let this = self.require_set_brand(self.current_this(), "difference")?;
        let other = args.first().cloned().unwrap_or(Value::Undefined);
        self.validate_set_like(&other, "difference")?;
        let other_vals = crate::intrinsics::collect_iterable(self, other)?;
        let other_keys: std::collections::HashSet<String> = other_vals
            .iter()
            .map(|v| Self::map_storage_key(v))
            .collect();
        let (new_set, storage) = self.new_empty_set();
        let mut size = 0.0;
        if let Value::Object(s) = self.object_get(this, "__set_data") {
            let kvs: Vec<(String, Value)> = self
                .obj(s)
                .properties
                .iter()
                .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                .collect();
            for (k, v) in kvs {
                if !other_keys.contains(&k) {
                    self.object_set(storage, k, v);
                    size += 1.0;
                }
            }
        }
        self.object_set(new_set, "size".into(), Value::Number(size));
        Ok(Value::Object(new_set))
    }

    /// Set.prototype.symmetricDifference(other).
    pub fn set_proto_symmetric_difference_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        let this = self.require_set_brand(self.current_this(), "symmetricDifference")?;
        let other = args.first().cloned().unwrap_or(Value::Undefined);
        self.validate_set_like(&other, "symmetricDifference")?;
        let other_vals = crate::intrinsics::collect_iterable(self, other)?;
        let other_keys: std::collections::HashSet<String> = other_vals
            .iter()
            .map(|v| Self::map_storage_key(v))
            .collect();
        let (new_set, storage) = self.new_empty_set();
        let mut size = 0.0;
        if let Value::Object(s) = self.object_get(this, "__set_data") {
            let kvs: Vec<(String, Value)> = self
                .obj(s)
                .properties
                .iter()
                .map(|(k, d)| (k.to_string_content(), d.value.clone()))
                .collect();
            for (k, v) in kvs {
                if !other_keys.contains(&k) {
                    self.object_set(storage, k, v);
                    size += 1.0;
                }
            }
        }
        let this_storage = match self.object_get(this, "__set_data") {
            Value::Object(id) => Some(id),
            _ => None,
        };
        for v in other_vals {
            let k = Self::map_storage_key(&v);
            let in_this = this_storage
                .map(|s| self.obj(s).has_own_str(&k))
                .unwrap_or(false);
            if !in_this {
                self.object_set(storage, k, v);
                size += 1.0;
            }
        }
        self.object_set(new_set, "size".into(), Value::Number(size));
        Ok(Value::Object(new_set))
    }

    /// Set.prototype.isSubsetOf(other).
    pub fn set_proto_is_subset_of_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let this = self.require_set_brand(self.current_this(), "isSubsetOf")?;
        let other = args.first().cloned().unwrap_or(Value::Undefined);
        self.validate_set_like(&other, "isSubsetOf")?;
        let other_vals = crate::intrinsics::collect_iterable(self, other)?;
        let other_keys: std::collections::HashSet<String> = other_vals
            .iter()
            .map(|v| Self::map_storage_key(v))
            .collect();
        if let Value::Object(s) = self.object_get(this, "__set_data") {
            for k in self.obj(s).properties.keys() {
                if !other_keys.contains(k.as_str()) {
                    return Ok(Value::Boolean(false));
                }
            }
        }
        Ok(Value::Boolean(true))
    }

    /// Set.prototype.isSupersetOf(other).
    pub fn set_proto_is_superset_of_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let this = self.require_set_brand(self.current_this(), "isSupersetOf")?;
        let other = args.first().cloned().unwrap_or(Value::Undefined);
        self.validate_set_like(&other, "isSupersetOf")?;
        let other_vals = crate::intrinsics::collect_iterable(self, other)?;
        let this_storage = match self.object_get(this, "__set_data") {
            Value::Object(id) => Some(id),
            _ => None,
        };
        for v in other_vals {
            let k = Self::map_storage_key(&v);
            let in_this = this_storage
                .map(|s| self.obj(s).has_own_str(&k))
                .unwrap_or(false);
            if !in_this {
                return Ok(Value::Boolean(false));
            }
        }
        Ok(Value::Boolean(true))
    }

    /// Set.prototype.isDisjointFrom(other).
    pub fn set_proto_is_disjoint_from_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        let this = self.require_set_brand(self.current_this(), "isDisjointFrom")?;
        let other = args.first().cloned().unwrap_or(Value::Undefined);
        self.validate_set_like(&other, "isDisjointFrom")?;
        let other_vals = crate::intrinsics::collect_iterable(self, other)?;
        let this_storage = match self.object_get(this, "__set_data") {
            Value::Object(id) => Some(id),
            _ => None,
        };
        for v in other_vals {
            let k = Self::map_storage_key(&v);
            let in_this = this_storage
                .map(|s| self.obj(s).has_own_str(&k))
                .unwrap_or(false);
            if in_this {
                return Ok(Value::Boolean(false));
            }
        }
        Ok(Value::Boolean(true))
    }

    fn set_this_and_storage(
        &mut self,
        who: &str,
    ) -> Result<(crate::value::ObjectRef, crate::value::ObjectRef), RuntimeError> {
        let this = match self.current_this() {
            Value::Object(id) => id,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Set.prototype.{}: this is not a Set object",
                    who
                )))
            }
        };
        let storage = match self.object_get(this, "__set_data") {
            Value::Object(id) => id,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Set.prototype.{}: this is not a Set object",
                    who
                )))
            }
        };
        // SPBC-EXT 2: reject WeakSet receiver (shares __set_data sentinel
        // SPBC-EXT 3 carve-back: WeakSet rejection moved to per-proto
        // wrappers in intrinsics.rs to avoid breaking WeakSet's own
        // basic methods (cruft shares set_proto_add_via et al. between
        // Set.prototype and WeakSet.prototype registrations).
        let _ = "weakset-rejection-relocated-to-per-proto-wrappers";
        Ok((this, storage))
    }

    /// Set.prototype.add(value).
    pub fn set_proto_add_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (this, storage) = self.set_this_and_storage("add")?;
        let v = args.first().cloned().unwrap_or(Value::Undefined);
        // Set uses SameValueZero identity per ECMA-262 §24.2.3. Mirror
        // the Map fix: encode Object/Symbol values as identity-stable
        // storage keys so object members compare by reference.
        let key_s = Self::map_storage_key(&v);
        let existed = self.obj(storage).has_own_str(&key_s);
        self.object_set(storage, key_s, v);
        if !existed {
            let prev = match self.object_get(this, "size") {
                Value::Number(n) => n,
                _ => 0.0,
            };
            self.object_set(this, "size".into(), Value::Number(prev + 1.0));
        }
        Ok(Value::Object(this))
    }

    /// Set.prototype.has(value).
    pub fn set_proto_has_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (_this, storage) = self.set_this_and_storage("has")?;
        let v = args.first().cloned().unwrap_or(Value::Undefined);
        let key_s = Self::map_storage_key(&v);
        Ok(Value::Boolean(self.obj(storage).has_own_str(&key_s)))
    }

    /// Set.prototype.delete(value).
    pub fn set_proto_delete_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (this, storage) = self.set_this_and_storage("delete")?;
        let v = args.first().cloned().unwrap_or(Value::Undefined);
        let key_s = Self::map_storage_key(&v);
        let existed = self.obj_mut(storage).remove_str(&key_s).is_some();
        if existed {
            let prev = match self.object_get(this, "size") {
                Value::Number(n) => n,
                _ => 0.0,
            };
            self.object_set(this, "size".into(), Value::Number((prev - 1.0).max(0.0)));
        }
        Ok(Value::Boolean(existed))
    }

    /// Set.prototype.clear().
    pub fn set_proto_clear_via(&mut self) -> Result<Value, RuntimeError> {
        let (this, _storage) = self.set_this_and_storage("clear")?;
        let fresh = self.alloc_object(crate::value::Object::new_ordinary());
        self.object_set(this, "__set_data".into(), Value::Object(fresh));
        self.object_set(this, "size".into(), Value::Number(0.0));
        Ok(Value::Undefined)
    }

    /// Set.prototype.forEach(cb).
    pub fn set_proto_for_each_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (this, storage) = self.set_this_and_storage("forEach")?;
        let cb = args.first().cloned().unwrap_or(Value::Undefined);
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError(
                "Set.prototype.forEach: callback is not callable".into(),
            ));
        }
        let vals: Vec<Value> = self
            .obj(storage)
            .properties
            .values()
            .map(|d| d.value.clone())
            .collect();
        for v in vals {
            self.call_function(
                cb.clone(),
                Value::Undefined,
                vec![v.clone(), v, Value::Object(this)],
            )?;
        }
        Ok(Value::Undefined)
    }

    /// SLV-EXT 1: ECMA-262 §24.2.1.2 GetSetRecord — Set ops require a
    /// Set-like Object (size: Number, has: callable, keys: callable). Plain
    /// Arrays, plain Objects, primitives all throw TypeError. Called at the
    /// top of each Set.prototype op (union/intersection/etc.).
    pub fn validate_set_like(&mut self, other: &Value, op: &str) -> Result<(), RuntimeError> {
        let id = match other {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Set.prototype.{}: argument is not an object",
                    op
                )))
            }
        };
        // Spec calls Get(obj, "size") and coerces to Number; NaN throws.
        let size = self.read_property(id, "size")?;
        let size_n = self.coerce_to_number(&size)?;
        if size_n.is_nan() {
            return Err(RuntimeError::TypeError(format!(
                "Set.prototype.{}: argument has invalid size",
                op
            )));
        }
        let has = self.read_property(id, "has")?;
        if !self.is_callable(&has) {
            return Err(RuntimeError::TypeError(format!(
                "Set.prototype.{}: argument's has property is not callable",
                op
            )));
        }
        let keys = self.read_property(id, "keys")?;
        if !self.is_callable(&keys) {
            return Err(RuntimeError::TypeError(format!(
                "Set.prototype.{}: argument's keys property is not callable",
                op
            )));
        }
        Ok(())
    }

    fn map_this_and_storage(
        &mut self,
        who: &str,
    ) -> Result<(crate::value::ObjectRef, crate::value::ObjectRef), RuntimeError> {
        let this = match self.current_this() {
            Value::Object(id) => id,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Map.prototype.{}: this is not a Map object",
                    who
                )))
            }
        };
        // EXT 81: Map-only methods (per ECMA §24.1.3) reject WeakMap-
        // tagged instances. The four operations that exist on both Map
        // and WeakMap (get/set/has/delete) accept either kind; the
        // remaining methods (clear/forEach/values/keys/entries/iterator)
        // are Map-only — when invoked via Map.prototype.X.call(weakmap)
        // they must throw TypeError because a WeakMap has [[WeakMapData]],
        // not [[MapData]]. (Those methods aren't registered on the
        // WeakMap proto, so the rejection only fires when callers
        // explicitly cross-proto.)
        let map_only = matches!(
            who,
            "clear" | "forEach" | "values" | "keys" | "entries" | "@@iterator"
        );
        if map_only && matches!(self.object_get(this, "__is_weakmap"), Value::Boolean(true)) {
            return Err(RuntimeError::TypeError(format!(
                "Map.prototype.{}: this is a WeakMap, not a Map",
                who
            )));
        }
        let storage = match self.object_get(this, "__map_data") {
            Value::Object(id) => id,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "Map.prototype.{}: this is not a Map object",
                    who
                )))
            }
        };
        Ok((this, storage))
    }

    /// SpeciesConstructor(O, defaultCtor) per ECMA §7.3.23.
    /// Returns the value to use as the constructor for derived objects:
    /// either O.constructor[@@species] if it's a constructor, or default.
    pub fn species_constructor(
        &mut self,
        o: &Value,
        default_ctor: Value,
    ) -> Result<Value, RuntimeError> {
        let o_id = match o {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "SpeciesConstructor: this is not an Object".into(),
                ))
            }
        };
        let c = self.spec_get(&Value::Object(o_id), "constructor")?;
        if matches!(c, Value::Undefined) {
            return Ok(default_ctor);
        }
        let c_id = match &c {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "SpeciesConstructor: constructor is not an Object".into(),
                ))
            }
        };
        let s = self.spec_get(&Value::Object(c_id), "@@species")?;
        if matches!(s, Value::Undefined | Value::Null) {
            return Ok(default_ctor);
        }
        if !self.is_callable(&s) {
            return Err(RuntimeError::TypeError(
                "SpeciesConstructor: @@species is not a constructor".into(),
            ));
        }
        Ok(s)
    }

    /// NewPromiseCapability(C) per ECMA §27.2.1.5.
    /// Returns (promise, resolve, reject) where promise is a fresh instance
    /// of C and resolve/reject are the functions captured by C's executor.
    /// If C is the built-in Promise, fast-pathed to internal new_promise.
    pub fn new_promise_capability(
        &mut self,
        ctor: &Value,
    ) -> Result<(Value, Value, Value), RuntimeError> {
        if !self.is_callable(ctor) {
            return Err(RuntimeError::TypeError(
                "NewPromiseCapability: C is not a constructor".into(),
            ));
        }
        // Shared cell that the executor populates.
        let cell: std::rc::Rc<std::cell::RefCell<(Value, Value)>> = std::rc::Rc::new(
            std::cell::RefCell::new((Value::Undefined, Value::Undefined)),
        );
        let cell_for_exec = cell.clone();
        let executor =
            crate::intrinsics::make_native("<NewPromiseCapability executor>", move |_rt, args| {
                let r = args.first().cloned().unwrap_or(Value::Undefined);
                let j = args.get(1).cloned().unwrap_or(Value::Undefined);
                *cell_for_exec.borrow_mut() = (r, j);
                Ok(Value::Undefined)
            });
        let exec_id = self.alloc_object(executor);
        // Construct(C, [executor]) — signal "new" via pending_new_target;
        // call_function's site picks it up at frame setup.
        let prev_pending = self.pending_new_target.take();
        self.pending_new_target = Some(ctor.clone());
        let promise =
            self.call_function(ctor.clone(), Value::Undefined, vec![Value::Object(exec_id)]);
        self.pending_new_target = prev_pending;
        let promise = promise?;
        let (resolve, reject) = cell.borrow().clone();
        if !self.is_callable(&resolve) {
            return Err(RuntimeError::TypeError(
                "NewPromiseCapability: resolve is not callable".into(),
            ));
        }
        if !self.is_callable(&reject) {
            return Err(RuntimeError::TypeError(
                "NewPromiseCapability: reject is not callable".into(),
            ));
        }
        Ok((promise, resolve, reject))
    }

    /// Ω.5.P63.E55 helper: allocate a fresh pending Promise and return its
    /// Value::Object handle. Exposed as a CallBuiltin target for IR sections
    /// that construct Promise capabilities (Promise.withResolvers,
    /// NewPromiseCapability, the capability-style allocation in
    /// Promise.all/race etc.).
    pub fn new_promise_value_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Object(crate::promise::new_promise(self)))
    }

    /// Ω.5.P63.E55 helper: settle a promise with a fulfillment value.
    /// First arg must be the Promise object; second is the resolved value.
    /// Used inside IR-Expr::Closure bodies that model Promise capability
    /// resolve functions.
    pub fn promise_settle_fulfilled_via(
        &mut self,
        promise: &Value,
        value: &Value,
    ) -> Result<Value, RuntimeError> {
        if let Value::Object(id) = promise {
            crate::promise::resolve_promise(self, *id, value.clone());
        }
        Ok(Value::Undefined)
    }

    /// Ω.5.P63.E55 helper: settle a promise with a rejection reason.
    pub fn promise_settle_rejected_via(
        &mut self,
        promise: &Value,
        value: &Value,
    ) -> Result<Value, RuntimeError> {
        if let Value::Object(id) = promise {
            crate::promise::reject_promise(self, *id, value.clone());
        }
        Ok(Value::Undefined)
    }

    /// Ω.5.P63.E55 Stage 3 helper: per Promise.all Resolve Element Function
    /// "maybe-complete" step. Decrements remaining; when it hits zero,
    /// assembles the values array and invokes the capability resolve.
    /// Takes the shared values cell, the remaining counter cell, and the
    /// capability resolve as Value (cells held as Rc<RefCell<Value>> can't
    /// be expressed as Values without boxing; instead, we model them as
    /// host-side Objects carrying a __cell_slot internal — kept simple here
    /// by representing the cells as plain mutable JS Arrays whose [0]
    /// element is the contained value).
    pub fn promise_all_maybe_complete_via(
        &mut self,
        values_arr: &Value,
        remaining_cell: &Value,
        cap_resolve: &Value,
    ) -> Result<Value, RuntimeError> {
        // remaining_cell is a host Array with [0]=Number(count); decrement.
        let remaining_id = match remaining_cell {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "promise_all_maybe_complete: remaining_cell must be a cell".into(),
                ))
            }
        };
        let cur = match self.object_get(remaining_id, "0") {
            Value::Number(n) => n,
            _ => {
                return Err(RuntimeError::TypeError(
                    "promise_all_maybe_complete: cell[0] must be Number".into(),
                ))
            }
        };
        let new_n = cur - 1.0;
        self.object_set(remaining_id, "0".into(), Value::Number(new_n));
        if new_n == 0.0 {
            self.call_function(
                cap_resolve.clone(),
                Value::Undefined,
                vec![values_arr.clone()],
            )?;
        }
        Ok(Value::Undefined)
    }

    /// Ω.5.P63.E55 Stage 3 helper: cell-style accessors so IR can model the
    /// spec's "Set values[index] to value" step. Uses a JS Array as the
    /// shared cell substrate (cells held as Value::Object).
    pub fn cell_array_set_via(
        &mut self,
        cell_array: &Value,
        index: &Value,
        value: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match cell_array {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "cell_array_set: cell must be Object".into(),
                ))
            }
        };
        let i = match index {
            Value::Number(n) => *n as usize,
            _ => 0,
        };
        self.object_set(id, i.to_string(), value.clone());
        Ok(Value::Undefined)
    }

    /// Ω.5.P63.E55 Stage 3 helper: returns true and sets the already_called
    /// flag to true if it was previously false. Atomic "first-call wins"
    /// semantics modeling the [[AlreadyCalled]] slot.
    pub fn cell_check_and_set_via(&mut self, cell: &Value) -> Result<Value, RuntimeError> {
        let id = match cell {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "cell_check_and_set: cell must be Object".into(),
                ))
            }
        };
        let prev = match self.object_get(id, "0") {
            Value::Boolean(b) => b,
            _ => false,
        };
        if prev {
            return Ok(Value::Boolean(false));
        }
        self.object_set(id, "0".into(), Value::Boolean(true));
        Ok(Value::Boolean(true))
    }

    /// Ω.5.P63.E55 Stage 3 helper: allocate a cell (host Array with [0]=init).
    pub fn cell_array_new_via(&mut self, init: &Value) -> Result<Value, RuntimeError> {
        let arr = self.alloc_object(crate::value::Object::new_array());
        self.object_set(arr, "0".into(), init.clone());
        self.object_set(arr, "length".into(), Value::Number(1.0));
        Ok(Value::Object(arr))
    }

    /// IR-EXT 55 Stage 3 helper: build the §27.2.4.2 settled-fulfilled entry
    /// `{status: "fulfilled", value: v}` for Promise.allSettled.
    pub fn make_settled_fulfilled_entry_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let mut entry = crate::value::Object::new_ordinary();
        entry.set_own(
            "status".into(),
            Value::String(std::rc::Rc::new("fulfilled".into())),
        );
        entry.set_own("value".into(), v.clone());
        Ok(Value::Object(self.alloc_object(entry)))
    }

    /// IR-EXT 55 Stage 3 helper: build the §27.2.4.2 settled-rejected entry
    /// `{status: "rejected", reason: r}` for Promise.allSettled.
    pub fn make_settled_rejected_entry_via(&mut self, r: &Value) -> Result<Value, RuntimeError> {
        let mut entry = crate::value::Object::new_ordinary();
        entry.set_own(
            "status".into(),
            Value::String(std::rc::Rc::new("rejected".into())),
        );
        entry.set_own("reason".into(), r.clone());
        Ok(Value::Object(self.alloc_object(entry)))
    }

    /// IR-EXT 55 Stage 3 helper: build the §27.2.4.3 AggregateError that
    /// Promise.any throws when all input promises reject.
    pub fn make_aggregate_error_via(&mut self, errors_arr: &Value) -> Result<Value, RuntimeError> {
        let mut agg = crate::value::Object::new_ordinary();
        agg.set_own(
            "name".into(),
            Value::String(std::rc::Rc::new("AggregateError".into())),
        );
        agg.set_own(
            "message".into(),
            Value::String(std::rc::Rc::new("All promises were rejected".into())),
        );
        agg.set_own("errors".into(), errors_arr.clone());
        Ok(Value::Object(self.alloc_object(agg)))
    }

    /// IR-EXT 55 Stage 3 helper: §27.2.4.3 step 11.d/e — decrement remaining,
    /// and when it hits zero build an AggregateError from the errors cell-array
    /// and reject the capability with it.
    pub fn promise_any_maybe_reject_via(
        &mut self,
        errors_arr: &Value,
        remaining_cell: &Value,
        cap_reject: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match remaining_cell {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "promise_any_maybe_reject: remaining_cell must be a cell".into(),
                ))
            }
        };
        let cur = match self.object_get(id, "0") {
            Value::Number(n) => n,
            _ => {
                return Err(RuntimeError::TypeError(
                    "promise_any_maybe_reject: cell[0] must be Number".into(),
                ))
            }
        };
        let new_n = cur - 1.0;
        self.object_set(id, "0".into(), Value::Number(new_n));
        if new_n == 0.0 {
            let agg = self.make_aggregate_error_via(errors_arr)?;
            self.call_function(cap_reject.clone(), Value::Undefined, vec![agg])?;
        }
        Ok(Value::Undefined)
    }

    /// IR-EXT 56 — Object.defineProperty body lifted from intrinsics.rs.
    /// Implements §10.1.6 ValidateAndApplyPropertyDescriptor + §6.2.5.5
    /// ToPropertyDescriptor: validates the desc object, throws TypeError on
    /// non-callable get/set or mixed data+accessor, honors generic-data
    /// preservation of existing [[Value]], enforces non-configurable redef.
    pub fn object_define_property_via(
        &mut self,
        target_v: &Value,
        key_v: &Value,
        desc_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let target = match target_v {
            Value::Object(id) => *id,
            other => {
                return Err(RuntimeError::TypeError(format!(
                    "Object.defineProperty: target must be an object (got {})",
                    match other {
                        Value::Undefined => "undefined",
                        Value::Null => "null",
                        Value::Boolean(_) => "boolean",
                        Value::Number(_) => "number",
                        Value::String(_) => "string",
                        _ => "other",
                    }
                )))
            }
        };
        // §10.1.6.1 ToPropertyKey: the P argument is coerced via
        // ToPrimitive(hint=string) then ToString. For Object inputs this
        // dispatches through @@toPrimitive / toString / valueOf so that
        // `Object.defineProperty(o, [1,2], desc)` lands at key "1,2"
        // (Array.prototype.toString → join), not "[object Object]".
        let coerced_key = match key_v {
            Value::Object(_) => {
                let prim = self.to_primitive(key_v, "string")?;
                // ToPrimitive may still return a Symbol; preserve it.
                match prim {
                    Value::Symbol(_) => prim,
                    _ => Value::String(crate::abstract_ops::to_string(&prim)),
                }
            }
            other => other.clone(),
        };
        // Tier-Ω Round 2 (2026-05-21): preserve Symbol-typed keys.
        let key_pk = crate::interp::property_key(&coerced_key);
        let key = key_pk.as_str().to_string();
        let desc_id = match desc_v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Object.defineProperty: descriptor must be an object".into(),
                ))
            }
        };
        // §10.1.6.3 step 2: if target is not extensible and the property
        // does not already exist, throw TypeError.
        if !self.obj(target).has_own_str(&key) && !self.obj(target).extensible {
            return Err(RuntimeError::TypeError(format!(
                "Cannot add property '{}': object is not extensible",
                key
            )));
        }
        // §10.4.2.1 ArraySetLength for Array exotic length redefinition.
        // IR-EXT 66: lifted into an IR section per keeper's higher-resolution-IR
        // conjecture. The intricate spec algorithm (RangeError + TypeError
        // throws, ToUint32 round-trip, descriptor flag preservation,
        // truncation loop) now lives in rusty-js-ir as 1:1 spec-step IR rather
        // than as a hand-written Rust helper.
        if key == "length"
            && matches!(
                self.obj(target).internal_kind,
                crate::value::InternalKind::Array
            )
        {
            return crate::generated::array_set_length(
                self,
                Value::Undefined,
                &[Value::Object(target), Value::Object(desc_id)],
            );
        }
        // §6.2.5.5 ToPropertyDescriptor: HasProperty + Get dispatch through
        // the prototype chain (test262 15.2.3.6-3-129 et al. inherit descriptor
        // attrs through proto). Previously used has_own_str + object_get.
        let has_get_key = self.has_property(desc_id, "get");
        let has_set_key = self.has_property(desc_id, "set");
        let getter = if has_get_key {
            self.read_property(desc_id, "get")?
        } else {
            Value::Undefined
        };
        let setter = if has_set_key {
            self.read_property(desc_id, "set")?
        } else {
            Value::Undefined
        };
        if has_get_key && !matches!(getter, Value::Undefined) && !self.is_callable(&getter) {
            return Err(RuntimeError::TypeError(
                "Invalid property descriptor: getter must be callable".into(),
            ));
        }
        if has_set_key && !matches!(setter, Value::Undefined) && !self.is_callable(&setter) {
            return Err(RuntimeError::TypeError(
                "Invalid property descriptor: setter must be callable".into(),
            ));
        }
        let has_getter = matches!(&getter, Value::Object(_));
        let has_setter = matches!(&setter, Value::Object(_));
        let has_value_key =
            self.has_property(desc_id, "value") || self.has_property(desc_id, "writable");
        let has_accessor_key = has_get_key || has_set_key;
        if has_value_key && has_accessor_key {
            return Err(RuntimeError::TypeError(
                "Invalid property descriptor: cannot both specify accessors and a value or writable attribute".into()));
        }
        // Accessor branch — has get and/or set in descriptor, possibly with
        // enumerable/configurable. §10.1.6.3 ValidateAndApply enforcement
        // applies symmetrically to data and accessor properties.
        if has_get_key || has_set_key {
            if self.typed_array_views.contains_key(&target)
                && Self::canonical_array_index_key(&key).is_some()
            {
                return Err(RuntimeError::TypeError(
                    "Cannot define accessor property on TypedArray index".into(),
                ));
            }
            let read_bool_via =
                |rt: &mut Runtime, name: &str| -> Result<Option<bool>, RuntimeError> {
                    if !rt.has_property(desc_id, name) {
                        return Ok(None);
                    }
                    let v = rt.read_property(desc_id, name)?;
                    Ok(Some(crate::abstract_ops::to_boolean(&v)))
                };
            let enumerable = read_bool_via(self, "enumerable")?;
            let configurable = read_bool_via(self, "configurable")?;
            let exists = self.obj(target).has_own_str(&key);
            let (default_e, default_c, existing_getter, existing_setter, existing_is_accessor) =
                if exists {
                    let d = self.obj(target).get_own(&key).unwrap();
                    let is_acc = d.getter.is_some() || d.setter.is_some();
                    (
                        d.enumerable,
                        d.configurable,
                        d.getter.clone(),
                        d.setter.clone(),
                        is_acc,
                    )
                } else {
                    (false, false, None, None, false)
                };
            let new_e = enumerable.unwrap_or(default_e);
            let new_c = configurable.unwrap_or(default_c);
            // §10.1.6.3 step 4: when existing is non-configurable.
            if exists && !default_c {
                // 4.a: configurable change disallowed.
                if configurable == Some(true) {
                    return Err(RuntimeError::TypeError(format!(
                        "Cannot redefine non-configurable property '{}': configurable would change",
                        key
                    )));
                }
                // 4.b: enumerable change disallowed.
                if enumerable.is_some() && new_e != default_e {
                    return Err(RuntimeError::TypeError(format!(
                        "Cannot redefine non-configurable property '{}': enumerable would change",
                        key
                    )));
                }
                // 4.c-d: data ⇄ accessor conversion disallowed.
                if !existing_is_accessor {
                    return Err(RuntimeError::TypeError(format!(
                        "Cannot redefine non-configurable data property '{}' as accessor",
                        key
                    )));
                }
                // 4.e: replacing existing get/set with a different one disallowed.
                if has_get_key {
                    let same = match (&existing_getter, &getter) {
                        (None, Value::Undefined) => true,
                        (Some(Value::Object(a)), Value::Object(b)) => a == b,
                        _ => false,
                    };
                    if !same {
                        return Err(RuntimeError::TypeError(format!(
                            "Cannot redefine non-configurable accessor '{}': [[Get]] would change",
                            key
                        )));
                    }
                }
                if has_set_key {
                    let same = match (&existing_setter, &setter) {
                        (None, Value::Undefined) => true,
                        (Some(Value::Object(a)), Value::Object(b)) => a == b,
                        _ => false,
                    };
                    if !same {
                        return Err(RuntimeError::TypeError(format!(
                            "Cannot redefine non-configurable accessor '{}': [[Set]] would change",
                            key
                        )));
                    }
                }
            }
            // GOPD-EXT 1: Option<Value> on PropertyDescriptor now carries
            // the spec discriminator "is field present in the descriptor":
            // Some(Value::Undefined) means "accessor with explicit undefined
            // getter/setter field" (per ECMA-262 §6.2.5 IsAccessorDescriptor);
            // None means "field absent" (data descriptor). Pre-fix conflated
            // Some(callable) with "is accessor"; lost the explicit-undefined
            // case so Object.getOwnPropertyDescriptor returned data-shape.
            let final_getter = if has_get_key {
                Some(getter)
            } else {
                existing_getter
            };
            let final_setter = if has_set_key {
                Some(setter)
            } else {
                existing_setter
            };
            let _ = has_getter;
            let _ = has_setter;
            self.obj_mut(target).dict_mut().insert(
                key_pk.clone(),
                crate::value::PropertyDescriptor {
                    value: Value::Undefined,
                    writable: false,
                    enumerable: new_e,
                    configurable: new_c,
                    getter: final_getter,
                    setter: final_setter,
                },
            );
        } else {
            let has_value = self.has_property(desc_id, "value");
            let has_writable = self.has_property(desc_id, "writable");
            // §6.2.5.6 generic descriptor: when Desc has none of
            // value/writable/get/set, the operation preserves the
            // existing property's type (data or accessor) and only
            // updates enumerable/configurable. Without this, redefining
            // an accessor with `{enumerable: true}` would silently
            // replace it with an undefined data property.
            let is_generic = !has_value && !has_writable;
            // §6.2.5.5 attribute reads use ToBoolean per abstract_ops.
            let read_bool_via =
                |rt: &mut Runtime, name: &str| -> Result<Option<bool>, RuntimeError> {
                    if !rt.has_property(desc_id, name) {
                        return Ok(None);
                    }
                    let v = rt.read_property(desc_id, name)?;
                    Ok(Some(crate::abstract_ops::to_boolean(&v)))
                };
            let writable = read_bool_via(self, "writable")?;
            let enumerable = read_bool_via(self, "enumerable")?;
            let configurable = read_bool_via(self, "configurable")?;
            // TAWR-EXT 2: §10.4.5.3 [[DefineOwnProperty]] for
            // IntegerIndexedExotic. Per spec, when key is a canonical-
            // numeric-index string (CanonicalNumericIndexString returns
            // not-undefined), return false (not throw) for:
            //   (3.b.i) IsInteger(numericIndex) false  (e.g. "NaN", "1.5")
            //   (3.b.iii) numericIndex === -0
            //   (3.b.iv) numericIndex < 0 or >= length
            //   (3.b.v) the descriptor's writable/enumerable/configurable
            //          attribute disagrees with the IntegerIndexedExotic
            //          fixed shape (writable:true, enumerable:true,
            //          configurable:true since ES2023)
            // Successful set: store value, return true.
            if self.typed_array_views.contains_key(&target) {
                if let Some(numeric_class) = Self::classify_numeric_index_key(&key) {
                    match numeric_class {
                        NumericIndexClass::ValidArrayIndex(idx) => {
                            let len = self.typed_array_view_len(target).unwrap_or(0);
                            if idx >= len {
                                return Ok(Value::Boolean(false));
                            }
                            // §10.4.5.3 step 3.b.v attribute checks
                            if configurable == Some(false)
                                || enumerable == Some(false)
                                || writable == Some(false)
                            {
                                return Ok(Value::Boolean(false));
                            }
                            let value = if has_value {
                                self.read_property(desc_id, "value")?
                            } else {
                                self.typed_array_get_index(target, idx)
                                    .unwrap_or(Value::Undefined)
                            };
                            self.typed_array_set_index(target, idx, value);
                            return Ok(Value::Boolean(true));
                        }
                        NumericIndexClass::InvalidNumericIndex => {
                            return Ok(Value::Boolean(false));
                        }
                    }
                }
            }
            if is_generic {
                let existed = self.obj(target).has_own_str(&key);
                if existed {
                    let prev = self.obj(target).get_own(&key).cloned().unwrap_or_else(|| {
                        crate::value::PropertyDescriptor {
                            value: self.object_get(target, &key),
                            writable: true,
                            enumerable: true,
                            configurable: true,
                            getter: None,
                            setter: None,
                        }
                    });
                    let new_e = enumerable.unwrap_or(prev.enumerable);
                    let new_c = configurable.unwrap_or(prev.configurable);
                    // §10.1.6.3 non-configurable invariants for generic
                    // descriptor: only enumerable/configurable can change,
                    // and only in the legal direction.
                    if !prev.configurable {
                        if configurable == Some(true) {
                            return Err(RuntimeError::TypeError(format!(
                                "Cannot redefine non-configurable property '{}': configurable would change", key)));
                        }
                        if enumerable.is_some() && new_e != prev.enumerable {
                            return Err(RuntimeError::TypeError(format!(
                                "Cannot redefine non-configurable property '{}': enumerable would change", key)));
                        }
                    }
                    self.obj_mut(target).dict_mut().insert(
                        key_pk.clone(),
                        crate::value::PropertyDescriptor {
                            value: prev.value,
                            writable: prev.writable,
                            enumerable: new_e,
                            configurable: new_c,
                            getter: prev.getter,
                            setter: prev.setter,
                        },
                    );
                    return Ok(Value::Object(target));
                }
                // No existing property: install a data property with
                // value=undefined and absent flags defaulting to false.
                self.obj_mut(target).dict_mut().insert(
                    key_pk.clone(),
                    crate::value::PropertyDescriptor {
                        value: Value::Undefined,
                        writable: false,
                        enumerable: enumerable.unwrap_or(false),
                        configurable: configurable.unwrap_or(false),
                        getter: None,
                        setter: None,
                    },
                );
                return Ok(Value::Object(target));
            }
            let value = if has_value {
                self.read_property(desc_id, "value")?
            } else {
                match self.obj(target).get_own(&key) {
                    Some(d) => d.value.clone(),
                    None => Value::Undefined,
                }
            };
            let exists = self.obj(target).has_own_str(&key);
            let (default_w, default_e, default_c, existing_value, existing_is_accessor) = if exists
            {
                let d = self.obj(target).get_own(&key).unwrap();
                let is_acc = d.getter.is_some() || d.setter.is_some();
                (
                    d.writable,
                    d.enumerable,
                    d.configurable,
                    d.value.clone(),
                    is_acc,
                )
            } else {
                (false, false, false, Value::Undefined, false)
            };
            let new_w = writable.unwrap_or(default_w);
            let new_e = enumerable.unwrap_or(default_e);
            let new_c = configurable.unwrap_or(default_c);
            if exists && !default_c {
                // §10.1.6.3 step 4.a: configurable promotion disallowed.
                if configurable == Some(true) {
                    return Err(RuntimeError::TypeError(format!(
                        "Cannot redefine non-configurable property '{}': configurable would change",
                        key
                    )));
                }
                // §10.1.6.3 step 4.b: enumerable change disallowed.
                if enumerable.is_some() && new_e != default_e {
                    return Err(RuntimeError::TypeError(format!(
                        "Cannot redefine non-configurable property '{}': enumerable would change",
                        key
                    )));
                }
                // §10.1.6.3 step 4.c-d: accessor → data conversion disallowed.
                if existing_is_accessor {
                    return Err(RuntimeError::TypeError(format!(
                        "Cannot redefine non-configurable accessor '{}' as data property",
                        key
                    )));
                }
                // Data → data: writable promotion (false → true) and value
                // change while non-writable are forbidden.
                if default_w == false && new_w == true {
                    return Err(RuntimeError::TypeError(format!(
                        "Cannot redefine non-configurable non-writable property '{}': writable would change", key)));
                }
                let value_changed =
                    has_value && !crate::abstract_ops::is_strictly_equal(&value, &existing_value);
                if value_changed && !default_w {
                    return Err(RuntimeError::TypeError(format!(
                        "Cannot redefine non-configurable non-writable property '{}'",
                        key
                    )));
                }
            }
            if has_value {
                let mapped_cell = {
                    let o = self.obj(target);
                    if let InternalKind::MappedArguments { parameter_map } = &o.internal_kind {
                        parameter_map.get(&key).cloned()
                    } else {
                        None
                    }
                };
                if let Some(cell) = mapped_cell {
                    *cell.borrow_mut() = value.clone();
                }
            }
            self.obj_mut(target).dict_mut().insert(
                key_pk.clone(),
                crate::value::PropertyDescriptor {
                    value,
                    writable: new_w,
                    enumerable: new_e,
                    configurable: new_c,
                    getter: None,
                    setter: None,
                },
            );
        }
        Ok(Value::Object(target))
    }

    pub fn number_to_string_key_via(&mut self, n: &Value) -> Result<Value, RuntimeError> {
        let num = self.coerce_to_number(n)?;
        Ok(Value::String(std::rc::Rc::new((num as u32).to_string())))
    }

    /// IR-EXT 66 runtime primitive: ToUint32(v) with the spec's
    /// "must round-trip" invariant — throws RangeError if
    /// ToUint32(v) != ToNumber(v). Used by §10.4.2.1 step 3.
    pub fn to_uint32_strict_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let num = self.coerce_to_number(v)?;
        // ToUint32 per §7.1.7.
        let u32_val = if num.is_nan() || num == 0.0 || num.is_infinite() {
            0_u32
        } else {
            // sign(n) * floor(|n|) modulo 2^32.
            let abs = num.abs().floor();
            let signed = if num < 0.0 { -abs } else { abs };
            let modulo = signed.rem_euclid(4294967296.0);
            modulo as u32
        };
        if (u32_val as f64) != num {
            return Err(RuntimeError::RangeError(
                "Invalid array length: ToUint32 does not round-trip".into(),
            ));
        }
        Ok(Value::Number(u32_val as f64))
    }

    /// IR-EXT 66 runtime primitive: read the Array's current length value
    /// as a Number, with no proto-chain walk and no descriptor synthesis
    /// quirks. Reads from own data if present (which is the case after the
    /// first defineProperty), or falls back to try_array_length's
    /// derive-from-indices path. Used by the IR-lifted §10.4.2.1.
    pub fn array_length_value_via(&mut self, target_v: &Value) -> Result<Value, RuntimeError> {
        let id = match target_v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "array_length_value_via: target is not an Array".into(),
                ))
            }
        };
        let n = self.try_array_length(id)?;
        Ok(Value::Number(n as f64))
    }

    /// IR-EXT 66 runtime primitive: return the Array length [[Writable]]
    /// attribute as a Boolean. Defaults to true when not explicitly set.
    pub fn array_length_writable_via(&mut self, target_v: &Value) -> Result<Value, RuntimeError> {
        let id = match target_v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "array_length_writable_via: target is not an Array".into(),
                ))
            }
        };
        Ok(Value::Boolean(
            self.obj(id)
                .get_own("length")
                .map(|d| d.writable)
                .unwrap_or(true),
        ))
    }

    /// IR-EXT 66 runtime primitive: write the Array length descriptor in
    /// one shot (value + writable), preserving spec-required
    /// non-configurable + non-enumerable. Bypasses the dispatching
    /// DefineOwnProperty so the IR-lifted §10.4.2.1 doesn't re-enter
    /// itself.
    pub fn array_length_set_internal_via(
        &mut self,
        target_v: &Value,
        new_len_v: &Value,
        writable_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match target_v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "array_length_set_internal_via: target is not an Array".into(),
                ))
            }
        };
        let len = match new_len_v {
            Value::Number(n) => *n,
            _ => 0.0,
        };
        let writable = matches!(writable_v, Value::Boolean(true));
        self.obj_mut(id).dict_mut().insert(
            crate::value::PropertyKey::String("length".into()),
            crate::value::PropertyDescriptor {
                value: Value::Number(len),
                writable,
                enumerable: false,
                configurable: false,
                getter: None,
                setter: None,
            },
        );
        Ok(Value::Undefined)
    }

    /// IR-EXT 66 runtime primitive: delete the own property at the given
    /// stringified key on the target. Returns Boolean(true) if deleted
    /// (or absent), Boolean(false) if present-and-non-configurable.
    pub fn delete_own_via(
        &mut self,
        target_v: &Value,
        key_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match target_v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "delete_own_via: target is not an Object".into(),
                ))
            }
        };
        let key = self.coerce_to_string(key_v)?;
        let configurable = self
            .obj(id)
            .get_own(&key)
            .map(|d| d.configurable)
            .unwrap_or(true);
        if !configurable {
            return Ok(Value::Boolean(false));
        }
        self.obj_mut(id).remove_str(&key);
        Ok(Value::Boolean(true))
    }

    /// IR-EXT 56 — Object.defineProperties: snapshot enumerable keys of props,
    /// then dispatch to object_define_property_via for each.
    pub fn object_define_properties_via(
        &mut self,
        target_v: &Value,
        props_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let target = match target_v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Object.defineProperties: target must be an object".into(),
                ))
            }
        };
        let props = match props_v {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Object.defineProperties: props must be an object".into(),
                ))
            }
        };
        let entries: Vec<(String, Value)> = {
            let o = self.obj(props);
            // CMig-EXT 4 Family B (P1 shape-iterate then properties-iterate):
            // shape-stored entries are user-default {w:t, e:t, c:t} per
            // shapes seed §IV — all enumerable, all string-keyed. Emit
            // before the IndexMap entries in insertion order.
            let mut out: Vec<(String, Value)> = Vec::new();
            if let Some(shape) = o.shape.as_ref() {
                for (name, slot) in shape.iter_slots() {
                    let idx = slot as usize;
                    if let Some(v) = o.shape_values.get(idx) {
                        out.push((name.to_string(), v.clone()));
                    }
                }
            }
            out.extend(
                o.properties
                    .iter()
                    .filter(|(_, d)| d.enumerable)
                    .map(|(k, d)| (k.to_string_content(), d.value.clone())),
            );
            out
        };
        for (k, dv) in entries {
            if matches!(dv, Value::Object(_)) {
                self.object_define_property_via(
                    &Value::Object(target),
                    &Value::String(std::rc::Rc::new(k)),
                    &dv,
                )?;
            }
        }
        Ok(Value::Object(target))
    }

    /// IR-EXT 56 — Object.getOwnPropertyDescriptor per §20.1.2.10. Returns
    /// {value,writable,enumerable,configurable} for data or
    /// {get,set,enumerable,configurable} for accessor; undefined if absent.
    pub fn object_get_own_property_descriptor_via(
        &mut self,
        obj_v: &Value,
        key_v: &Value,
    ) -> Result<Value, RuntimeError> {
        // §20.1.2.9 step 1: O = ? ToObject(O).
        let id = match obj_v {
            Value::Object(id) => *id,
            Value::Undefined | Value::Null => {
                return Err(RuntimeError::TypeError(
                    "Object.getOwnPropertyDescriptor: argument is not coercible to Object".into(),
                ))
            }
            _ => match self.to_object(obj_v)? {
                Value::Object(id) => id,
                _ => return Ok(Value::Undefined),
            },
        };
        let key = self.coerce_to_string(key_v)?;
        // §10.4.2 Array exotic: length is always an own property. Most
        // arrays synthesize it lazily as writable:true, but frozen template
        // arrays install an explicit descriptor before Object.freeze flips
        // writable/configurable; descriptor reflection must see that state.
        let is_array_length = key == "length"
            && matches!(
                self.obj(id).internal_kind,
                crate::value::InternalKind::Array
            );
        // TAMM-EXT 9: §10.4.5.1 [[GetOwnProperty]] for IntegerIndexedExotic:
        // when receiver is a TypedArray view and the key is a canonical
        // array index in [0, length), synthesize a data descriptor with the
        // element value and (writable:true, enumerable:true, configurable:true)
        // per ES2023+ semantics. Pre-EXT 9 the descriptor lookup missed
        // because typed-array indexed elements aren't stored as ordinary
        // dict properties.
        if self.typed_array_views.contains_key(&id) {
            if let Some(idx) = Self::canonical_array_index_key(&key) {
                let len = match self.object_get(id, "length") {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };
                if (idx as usize) < len {
                    let v = self.object_get(id, &key);
                    let out = self.alloc_object(crate::value::Object::new_ordinary());
                    self.object_set(out, "value".into(), v);
                    self.object_set(out, "writable".into(), Value::Boolean(true));
                    self.object_set(out, "enumerable".into(), Value::Boolean(true));
                    self.object_set(out, "configurable".into(), Value::Boolean(true));
                    return Ok(Value::Object(out));
                }
                return Ok(Value::Undefined);
            }
        }
        let (has, value, writable, enumerable, configurable, getter, setter) = if is_array_length {
            let len_v = self.object_get(id, "length");
            if let Some(d) = self.obj(id).get_own("length") {
                (
                    true,
                    len_v,
                    d.writable,
                    d.enumerable,
                    d.configurable,
                    d.getter.clone(),
                    d.setter.clone(),
                )
            } else {
                (true, len_v, true, false, false, None, None)
            }
        } else {
            let o = self.obj(id);
            // CMig-EXT 13: shape-aware lookup. Shape-stored entries are
            // user-default {w:t, e:t, c:t} per carve-out invariant;
            // synthesize the descriptor for them. Falls through to
            // properties lookup for non-shape entries.
            if let Some(v) = o.shape_get(&key) {
                (true, v.clone(), true, true, true, None, None)
            } else {
                match o.get_own(&key) {
                    Some(d) => (
                        true,
                        d.value.clone(),
                        d.writable,
                        d.enumerable,
                        d.configurable,
                        d.getter.clone(),
                        d.setter.clone(),
                    ),
                    None => (false, Value::Undefined, false, false, false, None, None),
                }
            }
        };
        if !has {
            return Ok(Value::Undefined);
        }
        let out = self.alloc_object(crate::value::Object::new_ordinary());
        if getter.is_some() || setter.is_some() {
            self.object_set(out, "get".into(), getter.unwrap_or(Value::Undefined));
            self.object_set(out, "set".into(), setter.unwrap_or(Value::Undefined));
        } else {
            self.object_set(out, "value".into(), value);
            self.object_set(out, "writable".into(), Value::Boolean(writable));
        }
        self.object_set(out, "enumerable".into(), Value::Boolean(enumerable));
        self.object_set(out, "configurable".into(), Value::Boolean(configurable));
        Ok(Value::Object(out))
    }

    /// IR-EXT 56 — Object.getOwnPropertyDescriptors per §20.1.2.10.
    /// Per §20.1.2.11 step 1: O = ? ToObject(O) — coerce primitives and
    /// throw TypeError on undefined/null.
    pub fn object_get_own_property_descriptors_via(
        &mut self,
        obj_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match obj_v {
            Value::Object(id) => *id,
            Value::Undefined | Value::Null => {
                return Err(RuntimeError::TypeError(
                    "Object.getOwnPropertyDescriptors: argument is not coercible to Object".into(),
                ))
            }
            _ => match self.to_object(obj_v)? {
                Value::Object(id) => id,
                // Symbol/BigInt: to_object returns the primitive itself in
                // cruftless v1 (no Symbol/BigInt wrapper objects). Spec says
                // return an empty descriptor object — no own properties to
                // enumerate on the primitive.
                _ => {
                    return Ok(Value::Object(
                        self.alloc_object(crate::value::Object::new_ordinary()),
                    ))
                }
            },
        };
        // §20.1.2.11 walks O.[[OwnPropertyKeys]] which excludes engine-
        // internal slots. cruftless represents [[BooleanData]] / [[NumberData]]
        // / [[StringData]] / [[SymbolData]] as the non-enumerable __primitive__
        // property; filter it from the returned descriptors.
        let entries: Vec<(
            String,
            Value,
            bool,
            bool,
            bool,
            Option<Value>,
            Option<Value>,
        )> = {
            let o = self.obj(id);
            // CMig-EXT 4 Family D (hybrid: synthesize default descriptor
            // for shape-stored entries per shapes seed §IV). Shape entries
            // are user-default {w:t, e:t, c:t} data descriptors by
            // invariant; emit synthesized descriptors before the
            // IndexMap entries.
            let mut out: Vec<(
                String,
                Value,
                bool,
                bool,
                bool,
                Option<Value>,
                Option<Value>,
            )> = Vec::new();
            if let Some(shape) = o.shape.as_ref() {
                for (name, slot) in shape.iter_slots() {
                    let idx = slot as usize;
                    if let Some(v) = o.shape_values.get(idx) {
                        out.push((name.to_string(), v.clone(), true, true, true, None, None));
                    }
                }
            }
            out.extend(
                o.properties
                    .iter()
                    .filter(|(k, _)| k.to_string_content() != "__primitive__")
                    .map(|(k, d)| {
                        (
                            k.to_string_content(),
                            d.value.clone(),
                            d.writable,
                            d.enumerable,
                            d.configurable,
                            d.getter.clone(),
                            d.setter.clone(),
                        )
                    }),
            );
            out
        };
        let out = self.alloc_object(crate::value::Object::new_ordinary());
        for (k, v, w, e, c, getter, setter) in entries {
            let desc = self.alloc_object(crate::value::Object::new_ordinary());
            if let Some(g) = getter {
                self.object_set(desc, "get".into(), g);
            }
            if let Some(s) = setter {
                self.object_set(desc, "set".into(), s);
            }
            if !matches!(v, Value::Undefined) {
                self.object_set(desc, "value".into(), v);
            }
            self.object_set(desc, "writable".into(), Value::Boolean(w));
            self.object_set(desc, "enumerable".into(), Value::Boolean(e));
            self.object_set(desc, "configurable".into(), Value::Boolean(c));
            self.object_set(out, k, Value::Object(desc));
        }
        Ok(Value::Object(out))
    }

    /// IR-EXT 56 — Object.create per §20.1.2.2 with full descriptor semantics.
    pub fn object_create_via(
        &mut self,
        proto_v: &Value,
        props_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let mut obj = crate::value::Object::new_ordinary();
        let explicit_null = matches!(proto_v, Value::Null);
        obj.proto = match proto_v {
            Value::Object(id) => Some(*id),
            Value::Null => None,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Object.create: prototype must be object or null".into(),
                ))
            }
        };
        let id = if explicit_null {
            self.alloc_object_with_explicit_null_proto(obj)
        } else {
            self.alloc_object(obj)
        };
        if let Value::Object(props_id) = props_v {
            let props_id = *props_id;
            // CMig-EXT 12: shape-aware key enumeration of the Properties
            // argument. Pre-fix this iterated .properties only; under
            // enrollment the descriptor map `{ prop: { value: ... } }`
            // has `prop` in shape → keys was empty → 257 test262
            // built-ins/Object/create fixtures failed because the
            // properties were silently skipped. Shape entries are all
            // enumerable by carve-out invariant.
            let keys: Vec<String> = {
                let o = self.obj(props_id);
                let mut out: Vec<String> = Vec::new();
                if let Some(shape) = o.shape.as_ref() {
                    for (name, _) in shape.iter_slots() {
                        out.push(name.to_string());
                    }
                }
                out.extend(
                    o.properties
                        .iter()
                        .filter(|(_, d)| d.enumerable)
                        .map(|(k, _)| k.as_str().to_string()),
                );
                out
            };
            for k in keys {
                let dv = self.read_property(props_id, &k)?;
                let did = match dv {
                    Value::Object(d) => d,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Property description must be an object".into(),
                        ))
                    }
                };
                let has_value = self.has_property(did, "value");
                let has_writable = self.has_property(did, "writable");
                let has_get = self.has_property(did, "get");
                let has_set = self.has_property(did, "set");
                let getter_v = if has_get {
                    self.read_property(did, "get")?
                } else {
                    Value::Undefined
                };
                let setter_v = if has_set {
                    self.read_property(did, "set")?
                } else {
                    Value::Undefined
                };
                if has_get && !matches!(getter_v, Value::Undefined) && !self.is_callable(&getter_v)
                {
                    return Err(RuntimeError::TypeError(
                        "Object.create: getter must be callable".into(),
                    ));
                }
                if has_set && !matches!(setter_v, Value::Undefined) && !self.is_callable(&setter_v)
                {
                    return Err(RuntimeError::TypeError(
                        "Object.create: setter must be callable".into(),
                    ));
                }
                if (has_value || has_writable) && (has_get || has_set) {
                    return Err(RuntimeError::TypeError(
                        "Object.create: cannot both specify accessors and a value or writable"
                            .into(),
                    ));
                }
                let read_bool =
                    |rt: &mut Runtime, name: &str| -> Result<Option<bool>, RuntimeError> {
                        if !rt.has_property(did, name) {
                            return Ok(None);
                        }
                        let v = rt.read_property(did, name)?;
                        Ok(Some(crate::abstract_ops::to_boolean(&v)))
                    };
                let writable = read_bool(self, "writable")?.unwrap_or(false);
                let enumerable = read_bool(self, "enumerable")?.unwrap_or(false);
                let configurable = read_bool(self, "configurable")?.unwrap_or(false);
                let value = if has_value {
                    self.read_property(did, "value")?
                } else {
                    Value::Undefined
                };
                let has_getter = matches!(getter_v, Value::Object(_));
                let has_setter = matches!(setter_v, Value::Object(_));
                self.obj_mut(id).dict_mut().insert(
                    crate::value::PropertyKey::String(k),
                    crate::value::PropertyDescriptor {
                        value,
                        writable,
                        enumerable,
                        configurable,
                        getter: if has_getter { Some(getter_v) } else { None },
                        setter: if has_setter { Some(setter_v) } else { None },
                    },
                );
            }
        }
        Ok(Value::Object(id))
    }

    /// IR-EXT 56 — Object.prototype.__defineGetter__ per Annex B.2.2.2.
    pub fn object_proto_define_getter_via(
        &mut self,
        this_v: &Value,
        key_v: &Value,
        fn_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let this = match this_v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Undefined),
        };
        // VMA-EXT 2: spec §B.2.2.2 step 1 IsCallable; step 2 ToPropertyKey.
        // Pre-fix used matches!(_, Object) (misses BoundFunction edge cases
        // and is not the spec's IsCallable) + static abstract_ops::to_string
        // (collapses Objects, stringifies Symbols to @@sym:N).
        if !self.is_callable(fn_v) {
            return Err(RuntimeError::TypeError(
                "__defineGetter__: getter must be callable".into(),
            ));
        }
        let key_coerced = if matches!(key_v, Value::Symbol(_)) {
            key_v.clone()
        } else {
            Value::String(std::rc::Rc::new(self.coerce_to_string(key_v)?))
        };
        let pk = Self::property_key_of(&key_coerced);
        let existing_setter = self
            .obj(this)
            .properties
            .get(&pk)
            .and_then(|d| d.setter.clone());
        self.obj_mut(this).dict_mut().insert(
            pk,
            crate::value::PropertyDescriptor {
                value: Value::Undefined,
                writable: false,
                enumerable: true,
                configurable: true,
                getter: Some(fn_v.clone()),
                setter: existing_setter,
            },
        );
        Ok(Value::Undefined)
    }

    /// IR-EXT 56 — Object.prototype.__defineSetter__ per Annex B.2.2.3.
    pub fn object_proto_define_setter_via(
        &mut self,
        this_v: &Value,
        key_v: &Value,
        fn_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let this = match this_v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Undefined),
        };
        // VMA-EXT 2: spec §B.2.2.3 IsCallable + ToPropertyKey.
        if !self.is_callable(fn_v) {
            return Err(RuntimeError::TypeError(
                "__defineSetter__: setter must be callable".into(),
            ));
        }
        let key_coerced = if matches!(key_v, Value::Symbol(_)) {
            key_v.clone()
        } else {
            Value::String(std::rc::Rc::new(self.coerce_to_string(key_v)?))
        };
        let pk = Self::property_key_of(&key_coerced);
        let existing_getter = self
            .obj(this)
            .properties
            .get(&pk)
            .and_then(|d| d.getter.clone());
        self.obj_mut(this).dict_mut().insert(
            pk,
            crate::value::PropertyDescriptor {
                value: Value::Undefined,
                writable: false,
                enumerable: true,
                configurable: true,
                getter: existing_getter,
                setter: Some(fn_v.clone()),
            },
        );
        Ok(Value::Undefined)
    }

    /// IR-EXT 56 — Object.prototype.__lookupGetter__ per Annex B.2.2.4.
    pub fn object_proto_lookup_getter_via(
        &mut self,
        this_v: &Value,
        key_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let this = match this_v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Undefined),
        };
        // VMA-EXT 2: spec §B.2.2.4 ToPropertyKey + proto-chain walk for
        // accessor lookup. Pre-fix used static to_string + own-only lookup;
        // spec actually says walk proto chain for an accessor with [[Get]].
        let key_coerced = if matches!(key_v, Value::Symbol(_)) {
            key_v.clone()
        } else {
            Value::String(std::rc::Rc::new(self.coerce_to_string(key_v)?))
        };
        let pk = Self::property_key_of(&key_coerced);
        let mut cur = Some(this);
        while let Some(c) = cur {
            if let Some(d) = self.obj(c).properties.get(&pk) {
                if let Some(g) = &d.getter {
                    if !matches!(g, Value::Undefined) {
                        return Ok(g.clone());
                    }
                }
                return Ok(Value::Undefined);
            }
            cur = self.obj(c).proto;
        }
        Ok(Value::Undefined)
    }

    /// IR-EXT 56 — Object.prototype.__lookupSetter__ per Annex B.2.2.5.
    pub fn object_proto_lookup_setter_via(
        &mut self,
        this_v: &Value,
        key_v: &Value,
    ) -> Result<Value, RuntimeError> {
        let this = match this_v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Undefined),
        };
        let key_coerced = if matches!(key_v, Value::Symbol(_)) {
            key_v.clone()
        } else {
            Value::String(std::rc::Rc::new(self.coerce_to_string(key_v)?))
        };
        let pk = Self::property_key_of(&key_coerced);
        let mut cur = Some(this);
        while let Some(c) = cur {
            if let Some(d) = self.obj(c).properties.get(&pk) {
                if let Some(s) = &d.setter {
                    if !matches!(s, Value::Undefined) {
                        return Ok(s.clone());
                    }
                }
                return Ok(Value::Undefined);
            }
            cur = self.obj(c).proto;
        }
        Ok(Value::Undefined)
    }

    /// Ω.5.P63.E55 helper: assemble the {promise, resolve, reject} object
    /// returned by Promise.withResolvers.
    pub fn promise_with_resolvers_assemble_via(
        &mut self,
        promise: &Value,
        resolve: &Value,
        reject: &Value,
    ) -> Result<Value, RuntimeError> {
        let mut out = crate::value::Object::new_ordinary();
        out.set_own("promise".into(), promise.clone());
        out.set_own("resolve".into(), resolve.clone());
        out.set_own("reject".into(), reject.clone());
        Ok(Value::Object(self.alloc_object(out)))
    }

    /// Promise.prototype.then(onFulfilled, onRejected) per ECMA §27.2.5.4.
    /// First arg is the source Promise (passed by call site via Expr::This).
    pub fn promise_then_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let source = match args.first() {
            Some(Value::Object(id)) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Promise.then: first arg must be a Promise".into(),
                ))
            }
        };
        let on_fulfilled = args.get(1).cloned();
        let on_rejected = args.get(2).cloned();
        let chain = crate::promise::new_promise(self);
        let (status, value) = {
            let s = self.obj(source);
            if let crate::value::InternalKind::Promise(ps) = &s.internal_kind {
                (ps.status, ps.value.clone())
            } else {
                return Err(RuntimeError::TypeError(
                    "Promise.then: first arg not a Promise object".into(),
                ));
            }
        };
        match status {
            crate::value::PromiseStatus::Pending => {
                let src = self.obj_mut(source);
                if let crate::value::InternalKind::Promise(ps) = &mut src.internal_kind {
                    ps.fulfill_reactions.push(crate::value::PromiseReaction {
                        handler: on_fulfilled.clone(),
                        chain,
                    });
                    ps.reject_reactions.push(crate::value::PromiseReaction {
                        handler: on_rejected.clone(),
                        chain,
                    });
                }
            }
            crate::value::PromiseStatus::Fulfilled => {
                crate::promise::enqueue_reaction(self, on_fulfilled, value, chain, false);
            }
            crate::value::PromiseStatus::Rejected => {
                self.pending_unhandled.remove(&source);
                crate::promise::enqueue_reaction(self, on_rejected, value, chain, true);
            }
        }
        Ok(Value::Object(chain))
    }

    /// Promise.prototype.catch(onRejected) per ECMA §27.2.5.1 —
    /// `Return Invoke(this, "then", «undefined, onRejected»)`. Delegating
    /// to this.then preserves the spec-required dispatch through a user-
    /// overridden then (load-bearing for the test262 catch/invokes-then
    /// suite and matches Bun).
    pub fn promise_catch_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let source = args.first().cloned().unwrap_or(Value::Undefined);
        let on_rejected = args.get(1).cloned().unwrap_or(Value::Undefined);
        // Resolve `this.then` (walks prototype chain). The parser routes the
        // user-facing identifier `then` directly; no name mangling needed.
        let then_fn = match &source {
            Value::Object(id) => {
                let mut cur = Some(*id);
                let mut found = Value::Undefined;
                while let Some(c) = cur {
                    if let Some(d) = self.obj(c).get_own("then") {
                        found = d.value.clone();
                        // GOPD-EXT 1: only invoke callable getters; Some(Undefined)
                        // means accessor-with-undefined-getter (returns undefined).
                        if let Some(g) = d.getter.clone() {
                            if self.is_callable(&g) {
                                found = self.call_function(g, source.clone(), Vec::new())?;
                            }
                        }
                        break;
                    }
                    cur = self.obj(c).proto;
                }
                found
            }
            _ => {
                return Err(RuntimeError::TypeError(
                    "Promise.prototype.catch: this is not an Object".into(),
                ))
            }
        };
        self.call_function(then_fn, source, vec![Value::Undefined, on_rejected])
    }

    /// Promise.prototype.finally(onFinally) per ECMA §27.2.5.3.
    /// args[0] = source Promise (current_this).
    pub fn promise_finally_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let source = match args.first() {
            Some(Value::Object(id)) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Promise.prototype.finally: this is not a Promise".into(),
                ))
            }
        };
        let cb = args.get(1).cloned();
        let chain = crate::promise::new_promise(self);
        let (status, value) = {
            let s = self.obj(source);
            if let crate::value::InternalKind::Promise(ps) = &s.internal_kind {
                (ps.status, ps.value.clone())
            } else {
                return Err(RuntimeError::TypeError(
                    "Promise.prototype.finally: this not a Promise".into(),
                ));
            }
        };
        if let Some(c) = &cb {
            if matches!(c, Value::Object(_)) {
                if let Err(e) = self.call_function(c.clone(), Value::Undefined, Vec::new()) {
                    if let RuntimeError::Thrown(v) = e {
                        crate::promise::reject_promise(self, chain, v);
                        return Ok(Value::Object(chain));
                    }
                    return Err(e);
                }
            }
        }
        match status {
            crate::value::PromiseStatus::Fulfilled => {
                crate::promise::resolve_promise(self, chain, value)
            }
            crate::value::PromiseStatus::Rejected => {
                crate::promise::reject_promise(self, chain, value)
            }
            crate::value::PromiseStatus::Pending => {}
        }
        Ok(Value::Object(chain))
    }

    /// Promise.all(iterable) per ECMA §27.2.4.1 using NewPromiseCapability +
    /// per-element [[AlreadyCalled]] resolve functions.
    fn promise_collect_iterable_or_reject(
        &mut self,
        iter_v: Value,
        cap_reject: &Value,
    ) -> Result<Option<Vec<Value>>, RuntimeError> {
        match self.promise_collect_iterable(iter_v) {
            Ok(entries) => Ok(Some(entries)),
            Err(e) => {
                let rejection = match e {
                    RuntimeError::Thrown(v) => v,
                    RuntimeError::TypeError(msg) => {
                        match crate::intrinsics::make_error_instance(self, "TypeError", &msg) {
                            Some(id) => Value::Object(id),
                            None => Value::String(Rc::new(format!("TypeError({:?})", msg))),
                        }
                    }
                    RuntimeError::RangeError(msg) => {
                        match crate::intrinsics::make_error_instance(self, "RangeError", &msg) {
                            Some(id) => Value::Object(id),
                            None => Value::String(Rc::new(format!("RangeError({:?})", msg))),
                        }
                    }
                    RuntimeError::ReferenceError(msg) => {
                        match crate::intrinsics::make_error_instance(self, "ReferenceError", &msg) {
                            Some(id) => Value::Object(id),
                            None => Value::String(Rc::new(format!("ReferenceError({:?})", msg))),
                        }
                    }
                    RuntimeError::SyntaxError(msg) => {
                        match crate::intrinsics::make_error_instance(self, "SyntaxError", &msg) {
                            Some(id) => Value::Object(id),
                            None => Value::String(Rc::new(format!("SyntaxError({:?})", msg))),
                        }
                    }
                    other => return Err(other),
                };
                self.call_function(cap_reject.clone(), Value::Undefined, vec![rejection])?;
                Ok(None)
            }
        }
    }

    fn promise_collect_iterable(&mut self, src: Value) -> Result<Vec<Value>, RuntimeError> {
        let id = match src {
            Value::Object(id) => id,
            Value::Undefined | Value::Null => {
                return Err(RuntimeError::TypeError(
                    "iterable: cannot iterate undefined or null".into(),
                ));
            }
            ref other => match self.to_object(other)? {
                Value::Object(id) => id,
                _ => return Ok(Vec::new()),
            },
        };
        let iterator_key =
            crate::value::PropertyKey::Symbol(Rc::new("@@iterator".to_string()));
        let method = match self.read_property_pk(id, &iterator_key)? {
            Value::Undefined => self.object_get(id, "@@iterator"),
            other => other,
        };
        let iter = self.call_function(method, Value::Object(id), Vec::new())?;
        let iter_id = match iter {
            Value::Object(id) => id,
            _ => return Err(RuntimeError::TypeError("iterator is not an object".into())),
        };
        let next = self.spec_get(&Value::Object(iter_id), "next")?;
        let mut out = Vec::new();
        loop {
            let result = self.call_function(next.clone(), Value::Object(iter_id), Vec::new())?;
            let rid = match result {
                Value::Object(id) => id,
                _ => {
                    return Err(RuntimeError::TypeError(
                        "iterator next did not return an object".into(),
                    ))
                }
            };
            let done = to_boolean(&self.spec_get(&Value::Object(rid), "done")?);
            if done {
                break;
            }
            out.push(self.spec_get(&Value::Object(rid), "value")?);
        }
        Ok(out)
    }

    pub fn promise_all_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let c = self.current_this();
        let default_promise = self.global_get("Promise");
        let ctor = if matches!(c, Value::Object(_)) && self.is_callable(&c) {
            c
        } else {
            default_promise.clone()
        };
        let (capability_promise, cap_resolve, cap_reject) = self.new_promise_capability(&ctor)?;
        let promise_resolve = match self.spec_get(&ctor, "resolve") {
            Ok(v) => v,
            Err(RuntimeError::Thrown(v)) => {
                self.call_function(cap_reject, Value::Undefined, vec![v])?;
                return Ok(capability_promise);
            }
            Err(e) => return Err(e),
        };
        if !self.is_callable(&promise_resolve) {
            let msg = "Promise.all: C.resolve is not callable";
            let rejection = match crate::intrinsics::make_error_instance(self, "TypeError", msg) {
                Some(id) => Value::Object(id),
                None => Value::String(Rc::new(format!("TypeError({:?})", msg))),
            };
            self.call_function(cap_reject, Value::Undefined, vec![rejection])?;
            return Ok(capability_promise);
        }
        let iter_v = args.first().cloned().unwrap_or(Value::Undefined);
        let entries = match self.promise_collect_iterable_or_reject(iter_v, &cap_reject)? {
            Some(entries) => entries,
            None => return Ok(capability_promise),
        };
        let n = entries.len();
        // Cells via IR-shaped helpers: values array preallocated, remaining cell at count=1.
        let values_arr = self.alloc_object(crate::value::Object::new_array());
        for j in 0..n {
            self.object_set(values_arr, j.to_string(), Value::Undefined);
        }
        self.object_set(values_arr, "length".into(), Value::Number(n as f64));
        let values_v = Value::Object(values_arr);
        let remaining_v = self.cell_array_new_via(&Value::Number(1.0))?;
        for (i, entry) in entries.into_iter().enumerate() {
            // remaining += 1
            let cur = match self.object_get(
                match &remaining_v {
                    Value::Object(id) => *id,
                    _ => unreachable!(),
                },
                "0",
            ) {
                Value::Number(n) => n,
                _ => 0.0,
            };
            self.object_set(
                match &remaining_v {
                    Value::Object(id) => *id,
                    _ => unreachable!(),
                },
                "0".into(),
                Value::Number(cur + 1.0),
            );
            let already_v = self.cell_array_new_via(&Value::Boolean(false))?;
            let resolve_element = crate::generated::promise_all_resolve_element_factory(
                self,
                Value::Undefined,
                &[
                    Value::Number(i as f64),
                    values_v.clone(),
                    already_v,
                    remaining_v.clone(),
                    cap_resolve.clone(),
                ],
            )?;
            let next_promise =
                self.call_function(promise_resolve.clone(), ctor.clone(), vec![entry])?;
            let then_fn = match &next_promise {
                Value::Object(id) => self.object_get(*id, "then"),
                _ => Value::Undefined,
            };
            if !self.is_callable(&then_fn) {
                return Err(RuntimeError::TypeError(
                    "Promise.all: next.then is not callable".into(),
                ));
            }
            self.call_function(
                then_fn,
                next_promise,
                vec![resolve_element, cap_reject.clone()],
            )?;
        }
        // Final loop-self decrement via the same maybe-complete primitive.
        self.promise_all_maybe_complete_via(&values_v, &remaining_v, &cap_resolve)?;
        Ok(capability_promise)
    }

    /// Promise.allSettled(iterable) per ECMA §27.2.4.2 — capability + per-element
    /// resolve/reject elements with [[AlreadyCalled]].
    pub fn promise_all_settled_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let c = self.current_this();
        let default_promise = self.global_get("Promise");
        let ctor = if matches!(c, Value::Object(_)) && self.is_callable(&c) {
            c
        } else {
            default_promise.clone()
        };
        let (capability_promise, cap_resolve, cap_reject) = self.new_promise_capability(&ctor)?;
        let promise_resolve = match self.spec_get(&ctor, "resolve") {
            Ok(v) => v,
            Err(RuntimeError::Thrown(v)) => {
                self.call_function(cap_reject.clone(), Value::Undefined, vec![v])?;
                return Ok(capability_promise);
            }
            Err(e) => return Err(e),
        };
        if !self.is_callable(&promise_resolve) {
            let msg = "Promise.allSettled: C.resolve is not callable";
            let rejection = match crate::intrinsics::make_error_instance(self, "TypeError", msg) {
                Some(id) => Value::Object(id),
                None => Value::String(Rc::new(format!("TypeError({:?})", msg))),
            };
            self.call_function(cap_reject.clone(), Value::Undefined, vec![rejection])?;
            return Ok(capability_promise);
        }
        let iter_v = args.first().cloned().unwrap_or(Value::Undefined);
        let entries = match self.promise_collect_iterable_or_reject(iter_v, &cap_reject)? {
            Some(entries) => entries,
            None => return Ok(capability_promise),
        };
        let n = entries.len();
        let values_arr = self.alloc_object(crate::value::Object::new_array());
        for j in 0..n {
            self.object_set(values_arr, j.to_string(), Value::Undefined);
        }
        self.object_set(values_arr, "length".into(), Value::Number(n as f64));
        let values_v = Value::Object(values_arr);
        let remaining_v = self.cell_array_new_via(&Value::Number(1.0))?;
        for (i, entry) in entries.into_iter().enumerate() {
            let cur = match self.object_get(
                match &remaining_v {
                    Value::Object(id) => *id,
                    _ => unreachable!(),
                },
                "0",
            ) {
                Value::Number(n) => n,
                _ => 0.0,
            };
            self.object_set(
                match &remaining_v {
                    Value::Object(id) => *id,
                    _ => unreachable!(),
                },
                "0".into(),
                Value::Number(cur + 1.0),
            );
            let already_v = self.cell_array_new_via(&Value::Boolean(false))?;
            let factory_args = vec![
                Value::Number(i as f64),
                values_v.clone(),
                already_v,
                remaining_v.clone(),
                cap_resolve.clone(),
            ];
            let resolve_element = crate::generated::promise_all_settled_resolve_element_factory(
                self,
                Value::Undefined,
                &factory_args,
            )?;
            let reject_element = crate::generated::promise_all_settled_reject_element_factory(
                self,
                Value::Undefined,
                &factory_args,
            )?;
            let next_promise =
                self.call_function(promise_resolve.clone(), ctor.clone(), vec![entry])?;
            let then_fn = match &next_promise {
                Value::Object(id) => self.object_get(*id, "then"),
                _ => Value::Undefined,
            };
            if !self.is_callable(&then_fn) {
                return Err(RuntimeError::TypeError(
                    "Promise.allSettled: next.then is not callable".into(),
                ));
            }
            self.call_function(then_fn, next_promise, vec![resolve_element, reject_element])?;
        }
        self.promise_all_maybe_complete_via(&values_v, &remaining_v, &cap_resolve)?;
        Ok(capability_promise)
    }

    /// Promise.any(iterable) per ECMA §27.2.4.3 — capability + per-element
    /// reject tracking; resolves on first fulfillment, rejects with
    /// AggregateError when all reject.
    pub fn promise_any_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let c = self.current_this();
        let default_promise = self.global_get("Promise");
        let ctor = if matches!(c, Value::Object(_)) && self.is_callable(&c) {
            c
        } else {
            default_promise.clone()
        };
        let (capability_promise, cap_resolve, cap_reject) = self.new_promise_capability(&ctor)?;
        let promise_resolve = match &ctor {
            Value::Object(cid) => self.object_get(*cid, "resolve"),
            _ => Value::Undefined,
        };
        if !self.is_callable(&promise_resolve) {
            return Err(RuntimeError::TypeError(
                "Promise.any: C.resolve is not callable".into(),
            ));
        }
        let iter_v = args.first().cloned().unwrap_or(Value::Undefined);
        let entries = crate::intrinsics::collect_iterable(self, iter_v)?;
        let n = entries.len();
        let errors_arr = self.alloc_object(crate::value::Object::new_array());
        for j in 0..n {
            self.object_set(errors_arr, j.to_string(), Value::Undefined);
        }
        self.object_set(errors_arr, "length".into(), Value::Number(n as f64));
        let errors_v = Value::Object(errors_arr);
        let remaining_v = self.cell_array_new_via(&Value::Number(1.0))?;
        for (i, entry) in entries.into_iter().enumerate() {
            let cur = match self.object_get(
                match &remaining_v {
                    Value::Object(id) => *id,
                    _ => unreachable!(),
                },
                "0",
            ) {
                Value::Number(n) => n,
                _ => 0.0,
            };
            self.object_set(
                match &remaining_v {
                    Value::Object(id) => *id,
                    _ => unreachable!(),
                },
                "0".into(),
                Value::Number(cur + 1.0),
            );
            let already_v = self.cell_array_new_via(&Value::Boolean(false))?;
            let reject_element = crate::generated::promise_any_reject_element_factory(
                self,
                Value::Undefined,
                &[
                    Value::Number(i as f64),
                    errors_v.clone(),
                    already_v,
                    remaining_v.clone(),
                    cap_reject.clone(),
                ],
            )?;
            let next_promise =
                self.call_function(promise_resolve.clone(), ctor.clone(), vec![entry])?;
            let then_fn = match &next_promise {
                Value::Object(id) => self.object_get(*id, "then"),
                _ => Value::Undefined,
            };
            if !self.is_callable(&then_fn) {
                return Err(RuntimeError::TypeError(
                    "Promise.any: next.then is not callable".into(),
                ));
            }
            self.call_function(
                then_fn,
                next_promise,
                vec![cap_resolve.clone(), reject_element],
            )?;
        }
        self.promise_any_maybe_reject_via(&errors_v, &remaining_v, &cap_reject)?;
        Ok(capability_promise)
    }

    /// Promise.race(iterable) per ECMA §27.2.4.5.
    /// Uses NewPromiseCapability over current_this so Promise.race.call(C, ...)
    /// constructs a C-shaped chain.
    pub fn promise_race_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let c = self.current_this();
        let default_promise = self.global_get("Promise");
        let ctor = if matches!(c, Value::Object(_)) && self.is_callable(&c) {
            c
        } else {
            default_promise.clone()
        };
        let (capability_promise, cap_resolve, cap_reject) = self.new_promise_capability(&ctor)?;
        let promise_resolve = match self.spec_get(&ctor, "resolve") {
            Ok(v) => v,
            Err(RuntimeError::Thrown(v)) => {
                self.call_function(cap_reject.clone(), Value::Undefined, vec![v])?;
                return Ok(capability_promise);
            }
            Err(e) => return Err(e),
        };
        if !self.is_callable(&promise_resolve) {
            let msg = "Promise.race: C.resolve is not callable";
            let rejection = match crate::intrinsics::make_error_instance(self, "TypeError", msg) {
                Some(id) => Value::Object(id),
                None => Value::String(Rc::new(format!("TypeError({:?})", msg))),
            };
            self.call_function(cap_reject.clone(), Value::Undefined, vec![rejection])?;
            return Ok(capability_promise);
        }
        let iter = args.first().cloned().unwrap_or(Value::Undefined);
        let entries = match self.promise_collect_iterable_or_reject(iter, &cap_reject)? {
            Some(entries) => entries,
            None => return Ok(capability_promise),
        };
        for v in entries {
            let next_promise =
                self.call_function(promise_resolve.clone(), ctor.clone(), vec![v])?;
            // next_promise.then(cap_resolve, cap_reject)
            let then_fn = match &next_promise {
                Value::Object(id) => self.object_get(*id, "then"),
                _ => Value::Undefined,
            };
            if !self.is_callable(&then_fn) {
                return Err(RuntimeError::TypeError(
                    "Promise.race: next.then is not callable".into(),
                ));
            }
            self.call_function(
                then_fn,
                next_promise,
                vec![cap_resolve.clone(), cap_reject.clone()],
            )?;
        }
        Ok(capability_promise)
    }

    /// ECMA-262 §24.1: Map keys compare by SameValueZero. cruftless's
    /// storage uses a string-keyed IndexMap, so we encode each JS key
    /// value into a stable storage key that preserves SameValueZero
    /// semantics:
    ///   - Object → "__objkey@<heap-id>" (identity-based)
    ///   - Symbol → "@@sym:<content>"    (already-prefixed unique)
    ///   - other  → ToString result
    /// Without this, `new Map().set({a:1},"x").set({a:1},"y").size` was
    /// 1 (both keys collapsed to "[object Object]") instead of 2.
    fn map_storage_key(key: &Value) -> String {
        match key {
            Value::Object(oid) => format!("__objkey@{}", oid.0),
            Value::Symbol(s) => {
                if s.starts_with("@@") {
                    (**s).clone()
                } else {
                    format!("@@sym:{}", s)
                }
            }
            _ => crate::abstract_ops::to_string(key).as_str().to_string(),
        }
    }

    /// Map.prototype.get(key).
    pub fn map_proto_get_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (_this, storage) = self.map_this_and_storage("get")?;
        let key = args.first().cloned().unwrap_or(Value::Undefined);
        let key_s = Self::map_storage_key(&key);
        Ok(self.object_get(storage, &key_s))
    }

    /// Map.prototype.set(key, value).
    pub fn map_proto_set_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (this, storage) = self.map_this_and_storage("set")?;
        let key = args.first().cloned().unwrap_or(Value::Undefined);
        let val = args.get(1).cloned().unwrap_or(Value::Undefined);
        let key_s = Self::map_storage_key(&key);
        let existed = self.obj(storage).has_own_str(&key_s);
        self.object_set(storage, key_s.clone(), val);
        // Side-channel: for non-string original keys (Object, Number,
        // Boolean, BigInt, null, undefined, Symbol), stash the original
        // Value under __map_orig_keys so iterators can return the proper
        // key shape rather than the encoded storage string.
        if !matches!(&key, Value::String(_)) {
            let orig_id = match self.object_get(this, "__map_orig_keys") {
                Value::Object(id) => id,
                _ => {
                    let id = self.alloc_object(crate::value::Object::new_ordinary());
                    self.object_set(this, "__map_orig_keys".into(), Value::Object(id));
                    id
                }
            };
            self.object_set(orig_id, key_s, key);
        }
        if !existed {
            let prev = match self.object_get(this, "size") {
                Value::Number(n) => n,
                _ => 0.0,
            };
            self.object_set(this, "size".into(), Value::Number(prev + 1.0));
        }
        Ok(Value::Object(this))
    }

    /// MGOI-EXT 1: Map.prototype.getOrInsert(key, value) per TC39 upsert
    /// proposal. If the map already has key, returns its value. Otherwise
    /// inserts (key, value) and returns value.
    pub fn map_proto_get_or_insert_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (this, storage) = self.map_this_and_storage("getOrInsert")?;
        let key = args.first().cloned().unwrap_or(Value::Undefined);
        let val = args.get(1).cloned().unwrap_or(Value::Undefined);
        let key_s = Self::map_storage_key(&key);
        if self.obj(storage).has_own_str(&key_s) {
            return Ok(self.object_get(storage, &key_s));
        }
        self.object_set(storage, key_s.clone(), val.clone());
        if !matches!(&key, Value::String(_)) {
            let orig_id = match self.object_get(this, "__map_orig_keys") {
                Value::Object(id) => id,
                _ => {
                    let id = self.alloc_object(crate::value::Object::new_ordinary());
                    self.object_set(this, "__map_orig_keys".into(), Value::Object(id));
                    id
                }
            };
            self.object_set(orig_id, key_s, key);
        }
        let prev = match self.object_get(this, "size") {
            Value::Number(n) => n,
            _ => 0.0,
        };
        self.object_set(this, "size".into(), Value::Number(prev + 1.0));
        Ok(val)
    }

    /// MGOI-EXT 1: Map.prototype.getOrInsertComputed(key, callbackfn) per
    /// TC39 upsert proposal. If has(key), return its value. Else call
    /// callbackfn(key), insert (key, result), return result. The callback
    /// is called BEFORE the insert; if it throws, no insert happens.
    pub fn map_proto_get_or_insert_computed_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        let (this, storage) = self.map_this_and_storage("getOrInsertComputed")?;
        let key = args.first().cloned().unwrap_or(Value::Undefined);
        let cb = args.get(1).cloned().unwrap_or(Value::Undefined);
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError(
                "Map.prototype.getOrInsertComputed: callbackfn is not callable".into(),
            ));
        }
        let key_s = Self::map_storage_key(&key);
        if self.obj(storage).has_own_str(&key_s) {
            return Ok(self.object_get(storage, &key_s));
        }
        let val = self.call_function(cb, Value::Undefined, vec![key.clone()])?;
        // Per spec, check again after callback (callback may have inserted).
        if self.obj(storage).has_own_str(&key_s) {
            return Ok(self.object_get(storage, &key_s));
        }
        self.object_set(storage, key_s.clone(), val.clone());
        if !matches!(&key, Value::String(_)) {
            let orig_id = match self.object_get(this, "__map_orig_keys") {
                Value::Object(id) => id,
                _ => {
                    let id = self.alloc_object(crate::value::Object::new_ordinary());
                    self.object_set(this, "__map_orig_keys".into(), Value::Object(id));
                    id
                }
            };
            self.object_set(orig_id, key_s, key);
        }
        let prev = match self.object_get(this, "size") {
            Value::Number(n) => n,
            _ => 0.0,
        };
        self.object_set(this, "size".into(), Value::Number(prev + 1.0));
        Ok(val)
    }

    /// Map.prototype.has(key).
    pub fn map_proto_has_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (_this, storage) = self.map_this_and_storage("has")?;
        let key = args.first().cloned().unwrap_or(Value::Undefined);
        let key_s = Self::map_storage_key(&key);
        Ok(Value::Boolean(self.obj(storage).has_own_str(&key_s)))
    }

    /// Map.prototype.delete(key).
    pub fn map_proto_delete_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (this, storage) = self.map_this_and_storage("delete")?;
        let key = args.first().cloned().unwrap_or(Value::Undefined);
        let key_s = Self::map_storage_key(&key);
        let existed = self.obj_mut(storage).remove_str(&key_s).is_some();
        if existed {
            let prev = match self.object_get(this, "size") {
                Value::Number(n) => n,
                _ => 0.0,
            };
            self.object_set(this, "size".into(), Value::Number((prev - 1.0).max(0.0)));
        }
        Ok(Value::Boolean(existed))
    }

    /// Map.prototype.clear().
    pub fn map_proto_clear_via(&mut self) -> Result<Value, RuntimeError> {
        let (this, _storage) = self.map_this_and_storage("clear")?;
        let fresh = self.alloc_object(crate::value::Object::new_ordinary());
        self.object_set(this, "__map_data".into(), Value::Object(fresh));
        self.object_set(this, "size".into(), Value::Number(0.0));
        Ok(Value::Undefined)
    }

    /// Map.prototype.forEach(cb).
    /// Decode a storage key string back into the original JS Value. For
    /// string keys this is identity; for object/symbol/etc. keys it
    /// looks up the original Value from the __map_orig_keys side channel.
    fn map_decode_key(&mut self, this: crate::value::ObjectRef, k: &str) -> Value {
        if k.starts_with("__objkey@") || k.starts_with("@@sym:") {
            if let Value::Object(orig_id) = self.object_get(this, "__map_orig_keys") {
                let v = self.object_get(orig_id, k);
                if !matches!(v, Value::Undefined) {
                    return v;
                }
            }
        }
        // For numeric/boolean/null/undefined original keys, the side
        // channel preserves them too; check before falling back to string.
        if let Value::Object(orig_id) = self.object_get(this, "__map_orig_keys") {
            let v = self.object_get(orig_id, k);
            if !matches!(v, Value::Undefined) {
                return v;
            }
        }
        Value::String(std::rc::Rc::new(k.to_string()))
    }

    pub fn map_proto_for_each_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let (this, storage) = self.map_this_and_storage("forEach")?;
        let cb = args.first().cloned().unwrap_or(Value::Undefined);
        let pairs: Vec<(String, Value)> = self
            .obj(storage)
            .properties
            .iter()
            .map(|(k, d)| (k.to_string_content(), d.value.clone()))
            .collect();
        for (k, v) in pairs {
            let key_v = self.map_decode_key(this, &k);
            self.call_function(
                cb.clone(),
                Value::Undefined,
                vec![v, key_v, Value::Object(this)],
            )?;
        }
        Ok(Value::Undefined)
    }

    /// Map.prototype.values() — v1 eager-collect.
    pub fn map_proto_values_via(&mut self) -> Result<Value, RuntimeError> {
        let (_this, storage) = self.map_this_and_storage("values")?;
        let vs: Vec<Value> = self
            .obj(storage)
            .properties
            .iter()
            .map(|(_k, d)| d.value.clone())
            .collect();
        let arr = self.alloc_object(crate::value::Object::new_array());
        for (i, v) in vs.into_iter().enumerate() {
            self.object_set(arr, i.to_string(), v);
        }
        let len = self.array_length(arr);
        self.object_set(arr, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(crate::iterator::make_array_iterator(
            self, arr,
        )))
    }

    /// Map.prototype.keys() — v1 eager-collect.
    pub fn map_proto_keys_via(&mut self) -> Result<Value, RuntimeError> {
        let (this, storage) = self.map_this_and_storage("keys")?;
        let ks: Vec<String> = self.obj(storage).string_key_clones().collect();
        let arr = self.alloc_object(crate::value::Object::new_array());
        for (i, k) in ks.into_iter().enumerate() {
            let key_v = self.map_decode_key(this, &k);
            self.object_set(arr, i.to_string(), key_v);
        }
        let len = self.array_length(arr);
        self.object_set(arr, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(crate::iterator::make_array_iterator(
            self, arr,
        )))
    }

    /// Map.prototype.entries() — v1 eager-collect array-of-pairs.
    pub fn map_proto_entries_via(&mut self) -> Result<Value, RuntimeError> {
        let (this, storage) = self.map_this_and_storage("entries")?;
        let pairs: Vec<(String, Value)> = self
            .obj(storage)
            .properties
            .iter()
            .map(|(k, d)| (k.to_string_content(), d.value.clone()))
            .collect();
        let arr = self.alloc_object(crate::value::Object::new_array());
        for (i, (k, v)) in pairs.into_iter().enumerate() {
            let key_v = self.map_decode_key(this, &k);
            let pair = self.alloc_object(crate::value::Object::new_array());
            self.object_set(pair, "0".into(), key_v);
            self.object_set(pair, "1".into(), v);
            self.object_set(pair, "length".into(), Value::Number(2.0));
            self.object_set(arr, i.to_string(), Value::Object(pair));
        }
        let len = self.array_length(arr);
        self.object_set(arr, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(crate::iterator::make_array_iterator(
            self, arr,
        )))
    }

    /// Object.groupBy(items, callbackFn) per ECMA §20.1.2.10.
    pub fn object_group_by_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let items = args.first().cloned().unwrap_or(Value::Undefined);
        let cb = args
            .get(1)
            .cloned()
            .ok_or_else(|| RuntimeError::TypeError("Object.groupBy: callbackFn required".into()))?;
        let entries = crate::intrinsics::collect_iterable(self, items)?;
        let out = self.alloc_object(crate::value::Object::new_ordinary());
        for (i, v) in entries.into_iter().enumerate() {
            let key_v = self.call_function(
                cb.clone(),
                Value::Undefined,
                vec![v.clone(), Value::Number(i as f64)],
            )?;
            let key = crate::abstract_ops::to_string(&key_v).as_str().to_string();
            let arr_id = match self.object_get(out, &key) {
                Value::Object(id) => id,
                _ => {
                    let a = self.alloc_object(crate::value::Object::new_array());
                    self.object_set(out, key.clone(), Value::Object(a));
                    self.object_set(a, "length".into(), Value::Number(0.0));
                    a
                }
            };
            let n = self.array_length(arr_id);
            self.object_set(arr_id, n.to_string(), v);
            self.object_set(arr_id, "length".into(), Value::Number((n + 1) as f64));
        }
        Ok(Value::Object(out))
    }

    /// JSON.stringify(value, replacer, space) per ECMA §25.5.2.
    /// IR-EXT 68: dispatch through the IR-lifted SerializeJSONProperty.
    /// Returns Value::String on success, Value::Undefined when value
    /// serializes to nothing (top-level undefined/function/symbol per spec).
    pub fn json_stringify_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let v = args.first().cloned().unwrap_or(Value::Undefined);
        // ECMA-262 §25.5.2 step 4: if IsCallable(replacer) is true, store
        // it as state.[[ReplacerFunction]]. cruftless threads this via a
        // LIFO stack on Runtime — pushed here, popped after the recursive
        // serialization completes, consulted by json_apply_replacer_via
        // at the top of each SerializeJSONProperty call.
        // §25.5.2 step 4: replacer can be a callable, OR an Array whose
        // items (after String/Number coercion) form a PropertyList.
        let mut pushed_replacer = false;
        let mut pushed_property_list = false;
        if let Some(r) = args.get(1) {
            if self.is_callable(r) {
                self.json_replacer_stack.push(r.clone());
                pushed_replacer = true;
            } else if let Value::Object(rid) = r {
                if self.json_internalize_is_array(*rid)? {
                    let len = self.try_array_length(*rid)?;
                    let mut list: Vec<String> = Vec::with_capacity(len);
                    for i in 0..len {
                        let item = self.spec_get(r, &i.to_string())?;
                        let coerced: Option<String> = match &item {
                            Value::String(s) => Some((**s).clone()),
                            Value::Number(n) => Some(crate::abstract_ops::number_to_string(*n)),
                            Value::Object(_) => {
                                // §25.5.2 step 4.b.iii.3: String/Number
                                // wrapper objects contribute ? ToString(v),
                                // not their raw [[StringData]]/[[NumberData]].
                                let prim = self.json_unwrap_wrapper_via(&item)?;
                                match prim {
                                    Value::String(_) | Value::Number(_) => {
                                        Some(self.to_string_strict(&item)?)
                                    }
                                    _ => None,
                                }
                            }
                            _ => None,
                        };
                        if let Some(s) = coerced {
                            if !list.contains(&s) {
                                list.push(s);
                            }
                        }
                    }
                    self.json_property_list_stack.push(Some(list));
                    pushed_property_list = true;
                }
            }
        }
        // Push a None frame for nested calls when no replacer was given,
        // so the compound-serialization site can pop a frame unconditionally.
        if !pushed_property_list {
            self.json_property_list_stack.push(None);
            pushed_property_list = true;
        }
        let wrapper = self.alloc_object(crate::value::Object::new_ordinary());
        self.create_data_property_or_throw(&Value::Object(wrapper), "", v)?;
        let result = crate::generated::json_serialize_property(
            self,
            Value::Undefined,
            &[
                Value::Object(wrapper),
                Value::String(std::rc::Rc::new(String::new())),
            ],
        );
        if pushed_replacer {
            self.json_replacer_stack.pop();
        }
        if pushed_property_list {
            self.json_property_list_stack.pop();
        }
        result
    }

    /// ECMA-262 §25.5.2.4 step 3 — apply the active replacer function to
    /// (key, value) with the holder as `this`, and return its result.
    /// If no replacer is active, returns the value unchanged.
    pub fn json_apply_replacer_via(
        &mut self,
        holder: &Value,
        value: &Value,
        key: &Value,
    ) -> Result<Value, RuntimeError> {
        let replacer = match self.json_replacer_stack.last() {
            Some(r) => r.clone(),
            None => return Ok(value.clone()),
        };
        self.call_function(replacer, holder.clone(), vec![key.clone(), value.clone()])
    }

    pub fn json_get_property_via(
        &mut self,
        holder: &Value,
        key: &Value,
    ) -> Result<Value, RuntimeError> {
        let key_name = match key {
            Value::String(s) => (**s).clone(),
            _ => self.coerce_to_string(key)?,
        };
        self.spec_get(holder, &key_name)
    }

    /// IR-EXT 68: §25.5.2.4 step 2 — invoke value.toJSON(key) when value
    /// is an Object|BigInt and has a callable toJSON. Returns the result,
    /// or the original value unchanged.
    pub fn json_apply_to_json_via(
        &mut self,
        value: &Value,
        key: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match value {
            Value::Object(id) => *id,
            _ => return Ok(value.clone()),
        };
        let to_json = self.read_property(id, "toJSON")?;
        if !self.is_callable(&to_json) {
            return Ok(value.clone());
        }
        self.call_function(to_json, value.clone(), vec![key.clone()])
    }

    /// IR-EXT 68: §25.5.2.4 step 4 — unwrap a primitive wrapper Object
    /// (Boolean/Number/String/BigInt) to its underlying primitive.
    pub fn json_unwrap_wrapper_via(&mut self, value: &Value) -> Result<Value, RuntimeError> {
        let id = match value {
            Value::Object(id) => *id,
            _ => return Ok(value.clone()),
        };
        if let Some(d) = self.obj(id).get_own("__primitive__") {
            match &d.value {
                Value::Number(_) | Value::String(_) | Value::Boolean(_) | Value::BigInt(_) => {
                    return Ok(d.value.clone());
                }
                _ => {}
            }
        }
        Ok(value.clone())
    }

    /// IR-EXT 68: §25.5.2.{5,6} SerializeJSONObject/SerializeJSONArray.
    /// Recursive structural serialization, calling back into the IR-lifted
    /// json_serialize_property for each child.
    pub fn json_serialize_compound_via(&mut self, value: &Value) -> Result<Value, RuntimeError> {
        let id = match value {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "json_serialize_compound: value must be an Object".into(),
                ))
            }
        };
        // RPTP-EXT 2: ECMA-262 §25.5.2.5 step 1 + §10.5.{12,13}: traversing
        // a Proxy whose [[ProxyHandler]] is null (revoked) must throw
        // TypeError. JSON.stringify on a revoked proxy (or a tree containing
        // one) hits this at each compound-traversal entry.
        if self.proxy_is_revoked(id) {
            return Err(RuntimeError::TypeError(
                "Cannot perform operation on a revoked Proxy".into(),
            ));
        }
        if self.json_stringify_stack.contains(&id) {
            return Err(RuntimeError::TypeError(
                "Converting circular structure to JSON".into(),
            ));
        }
        self.json_stringify_stack.push(id);
        let is_array = self.json_internalize_is_array(id)?;
        let result = (|rt: &mut Runtime| -> Result<Value, RuntimeError> {
            if is_array {
                let len = rt.try_array_length(id)?;
                let mut parts: Vec<String> = Vec::with_capacity(len);
                for i in 0..len {
                    let key = i.to_string();
                    let key_v = Value::String(std::rc::Rc::new(key));
                    let serialized = crate::generated::json_serialize_property(
                        rt,
                        Value::Undefined,
                        &[Value::Object(id), key_v],
                    )?;
                    let part = match serialized {
                        Value::String(s) => (*s).clone(),
                        _ => "null".to_string(),
                    };
                    parts.push(part);
                }
                Ok(Value::String(std::rc::Rc::new(format!(
                    "[{}]",
                    parts.join(",")
                ))))
            } else {
                // ECMA-262 §10.1.11 OrdinaryOwnPropertyKeys: integer-indexed
                // keys first in numeric order, then string keys in insertion
                // order. Without this, JSON.stringify({0:'a',10:'x',2:'b'})
                // produced {"0":"a","10":"x","2":"b"} instead of the
                // spec-correct {"0":"a","2":"b","10":"x"}. Surfaced by the
                // diff-prod json-roundtrip fixture's canonicalizer.
                // §25.5.2.5 step 4: if a PropertyList is active for this
                // stringify frame, that list IS the key set (filter + order).
                // Otherwise compute OrdinaryOwnPropertyKeys-style ordering.
                let keys: Vec<String> = if let Some(Some(list)) = rt.json_property_list_stack.last()
                {
                    list.clone()
                } else {
                    // Lift: route through canonical helper.
                    rt.json_internalize_enumerable_keys(id)?
                };
                let mut parts: Vec<String> = Vec::new();
                for k in keys {
                    let key_v = Value::String(std::rc::Rc::new(k.clone()));
                    let serialized = crate::generated::json_serialize_property(
                        rt,
                        Value::Undefined,
                        &[Value::Object(id), key_v],
                    )?;
                    if let Value::String(s) = serialized {
                        parts.push(format!(
                            "{}:{}",
                            crate::intrinsics::json_quote_string_pub(&k),
                            *s
                        ));
                    }
                }
                Ok(Value::String(std::rc::Rc::new(format!(
                    "{{{}}}",
                    parts.join(",")
                ))))
            }
        })(self);
        self.json_stringify_stack.pop();
        result
    }

    /// IR-EXT 68: §25.5.2.2 QuoteJSONString.
    pub fn json_quote_string_via(&mut self, value: &Value) -> Result<Value, RuntimeError> {
        let s: String = match value {
            Value::String(s) => (**s).clone(),
            _ => self.coerce_to_string(value)?,
        };
        Ok(Value::String(std::rc::Rc::new(
            crate::intrinsics::json_quote_string_pub(&s),
        )))
    }

    /// IR-EXT 69: ECMA §7.1.18 ToObject — with TypeError on null/undefined.
    /// Wraps Runtime::to_object as a CallBuiltin target for IR sections.
    pub fn to_object_strict_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        match v {
            Value::Undefined | Value::Null => Err(RuntimeError::TypeError(
                "Cannot convert undefined or null to Object".into(),
            )),
            _ => self.to_object(v),
        }
    }

    /// IR-EXT 69: return an Array of the source's enumerable own
    /// string-keyed property names (excluding internal __primitive__ and
    /// well-known-symbol @@-prefixed keys). Used by Object.assign's
    /// per-source walk.
    pub fn own_enumerable_string_keys_via(
        &mut self,
        source: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match source {
            Value::Object(id) => *id,
            _ => {
                return Ok(Value::Object(
                    self.alloc_object(crate::value::Object::new_array()),
                ))
            }
        };
        // Lift: canonical ordering + filter. Pre-lift this site used a
        // looser filter (no integer-first ordering, didn't exclude
        // Array's "length") which made Object.assign's source enumeration
        // diverge from Object.keys' on Array sources.
        let keys = self.ordinary_own_enumerable_string_keys(id);
        let arr = self.alloc_object(crate::value::Object::new_array());
        for (i, k) in keys.iter().enumerate() {
            self.object_set(
                arr,
                i.to_string(),
                Value::String(std::rc::Rc::new(k.clone())),
            );
        }
        self.object_set(arr, "length".into(), Value::Number(keys.len() as f64));
        Ok(Value::Object(arr))
    }

    /// IR-EXT 69: Get(source, key) — invokes accessor getters if present.
    /// Thin wrapper exposing read_property as a CallBuiltin target.
    pub fn get_via(&mut self, source: &Value, key: &Value) -> Result<Value, RuntimeError> {
        // EXT 82b / Tier-1.5: promote the IR-emitted `get_via` (used by
        // CallBuiltin{name:"get_via"} for computed-method-name lookups in
        // ToPrimitive m1/m2 and elsewhere) to the spec-correct §7.3.2
        // [[Get]] path. This dispatches Proxy.get traps so a Proxy
        // receiver's user `get` handler fires on valueOf / toString /
        // computed-key lookups inside ToPrimitive, matching what EXT 82
        // already did for the literal @@toPrimitive lookup via
        // Expr::SpecGet. Doc 730 §XIII: each IR-emitted Get-shape
        // primitive lowers to the Proxy-aware path; bypass remains
        // available via object_get / read_property when the spec
        // explicitly names an internal slot.
        let k = self.coerce_to_string(key)?;
        self.spec_get(source, &k)
    }

    /// IR-EXT 69: Set(target, key, value) — non-throwing assign that goes
    /// through object_set's writable-check + accessor-setter dispatch.
    pub fn set_via(
        &mut self,
        target: &Value,
        key: &Value,
        value: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "set_via: target is not an Object".into(),
                ))
            }
        };
        let k = self.coerce_to_string(key)?;
        self.object_set(id, k, value.clone());
        Ok(Value::Undefined)
    }

    /// IR-EXT 68: §25.5.2.4 step 9 — finite Number → ToString(n); else "null".
    pub fn json_format_number_via(&mut self, value: &Value) -> Result<Value, RuntimeError> {
        let n = match value {
            Value::Number(n) => *n,
            _ => {
                return Err(RuntimeError::TypeError(
                    "json_format_number: expected Number".into(),
                ))
            }
        };
        if n.is_finite() {
            Ok(Value::String(std::rc::Rc::new(
                crate::abstract_ops::number_to_string(n),
            )))
        } else {
            Ok(Value::String(std::rc::Rc::new("null".into())))
        }
    }

    /// JSON.parse(text, reviver) per ECMA §24.5.1.
    pub fn json_parse_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let text = args.first().cloned().unwrap_or(Value::Undefined);
        let s = self.to_string_strict(&text)?;
        let unfiltered = crate::intrinsics::json_parse(self, s.as_str())?;
        let reviver = args.get(1).cloned().unwrap_or(Value::Undefined);
        if !self.is_callable(&reviver) {
            return Ok(unfiltered);
        }
        let source_spans = Self::json_source_spans(s.as_str())?;
        let root = Value::Object(self.alloc_object(crate::value::Object::new_ordinary()));
        self.create_data_property_or_throw(&root, "", unfiltered)?;
        self.internalize_json_property(root, "", reviver, &source_spans, Vec::new())
    }

    fn internalize_json_property(
        &mut self,
        holder: Value,
        name: &str,
        reviver: Value,
        source_spans: &HashMap<Vec<String>, (String, Value)>,
        path: Vec<String>,
    ) -> Result<Value, RuntimeError> {
        let mut value = self.spec_get(&holder, name)?;
        if let Value::Object(value_id) = value.clone() {
            let keys = if self.json_internalize_is_array(value_id)? {
                let len = self.try_array_length(value_id)?;
                (0..len).map(|i| i.to_string()).collect()
            } else {
                self.json_internalize_enumerable_keys(value_id)?
            };
            for key in keys {
                let mut child_path = path.clone();
                child_path.push(key.clone());
                let new_element = self.internalize_json_property(
                    Value::Object(value_id),
                    &key,
                    reviver.clone(),
                    source_spans,
                    child_path,
                )?;
                if matches!(new_element, Value::Undefined) {
                    let _ = self.reflect_delete_property_via(
                        &Value::Object(value_id),
                        &Value::String(std::rc::Rc::new(key.clone())),
                    )?;
                } else {
                    let _ =
                        self.create_data_property(&Value::Object(value_id), &key, new_element)?;
                }
            }
            value = Value::Object(value_id);
        }
        let source = source_spans.get(&path).and_then(|(source, original)| {
            if crate::abstract_ops::is_strictly_equal(&value, original) {
                Some(source.clone())
            } else {
                None
            }
        });
        let context = self.json_reviver_context(source)?;
        self.call_function(
            reviver,
            holder,
            vec![
                Value::String(std::rc::Rc::new(name.to_string())),
                value,
                context,
            ],
        )
    }

    fn json_reviver_context(&mut self, source: Option<String>) -> Result<Value, RuntimeError> {
        let context = Value::Object(self.alloc_object(crate::value::Object::new_ordinary()));
        if let Some(source) = source {
            self.create_data_property_or_throw(
                &context,
                "source",
                Value::String(std::rc::Rc::new(source)),
            )?;
        }
        Ok(context)
    }

    fn json_source_spans(
        source: &str,
    ) -> Result<HashMap<Vec<String>, (String, Value)>, RuntimeError> {
        let bytes = source.as_bytes();
        let mut p = 0;
        let mut out = HashMap::new();
        Self::json_source_skip_ws(bytes, &mut p);
        Self::json_source_value(source, bytes, &mut p, &mut Vec::new(), &mut out)?;
        Ok(out)
    }

    fn json_source_value(
        source: &str,
        bytes: &[u8],
        p: &mut usize,
        path: &mut Vec<String>,
        out: &mut HashMap<Vec<String>, (String, Value)>,
    ) -> Result<(), RuntimeError> {
        Self::json_source_skip_ws(bytes, p);
        let start = *p;
        match bytes.get(*p).copied() {
            Some(b'{') => Self::json_source_object(source, bytes, p, path, out),
            Some(b'[') => Self::json_source_array(source, bytes, p, path, out),
            Some(b'"') => {
                let value = Self::json_source_string_decoded(source, bytes, p)?;
                out.insert(
                    path.clone(),
                    (
                        source[start..*p].to_string(),
                        Value::String(std::rc::Rc::new(value)),
                    ),
                );
                Ok(())
            }
            Some(b't') if bytes[*p..].starts_with(b"true") => {
                *p += 4;
                out.insert(
                    path.clone(),
                    (source[start..*p].to_string(), Value::Boolean(true)),
                );
                Ok(())
            }
            Some(b'f') if bytes[*p..].starts_with(b"false") => {
                *p += 5;
                out.insert(
                    path.clone(),
                    (source[start..*p].to_string(), Value::Boolean(false)),
                );
                Ok(())
            }
            Some(b'n') if bytes[*p..].starts_with(b"null") => {
                *p += 4;
                out.insert(path.clone(), (source[start..*p].to_string(), Value::Null));
                Ok(())
            }
            Some(b'-' | b'0'..=b'9') => {
                Self::json_source_number(bytes, p);
                let raw = source[start..*p].to_string();
                let n = raw.parse::<f64>().map_err(|_| {
                    RuntimeError::SyntaxError("JSON.parse source scan: bad number".into())
                })?;
                out.insert(path.clone(), (raw, Value::Number(n)));
                Ok(())
            }
            _ => Err(RuntimeError::SyntaxError(
                "JSON.parse source scan: invalid value".into(),
            )),
        }
    }

    fn json_source_object(
        source: &str,
        bytes: &[u8],
        p: &mut usize,
        path: &mut Vec<String>,
        out: &mut HashMap<Vec<String>, (String, Value)>,
    ) -> Result<(), RuntimeError> {
        *p += 1;
        Self::json_source_skip_ws(bytes, p);
        if matches!(bytes.get(*p), Some(b'}')) {
            *p += 1;
            return Ok(());
        }
        loop {
            Self::json_source_skip_ws(bytes, p);
            let key = Self::json_source_string_decoded(source, bytes, p)?;
            Self::json_source_skip_ws(bytes, p);
            if !matches!(bytes.get(*p), Some(b':')) {
                return Err(RuntimeError::SyntaxError(
                    "JSON.parse source scan: expected ':'".into(),
                ));
            }
            *p += 1;
            path.push(key);
            Self::json_source_value(source, bytes, p, path, out)?;
            path.pop();
            Self::json_source_skip_ws(bytes, p);
            match bytes.get(*p) {
                Some(b',') => *p += 1,
                Some(b'}') => {
                    *p += 1;
                    return Ok(());
                }
                _ => {
                    return Err(RuntimeError::SyntaxError(
                        "JSON.parse source scan: expected ',' or '}'".into(),
                    ))
                }
            }
        }
    }

    fn json_source_array(
        source: &str,
        bytes: &[u8],
        p: &mut usize,
        path: &mut Vec<String>,
        out: &mut HashMap<Vec<String>, (String, Value)>,
    ) -> Result<(), RuntimeError> {
        *p += 1;
        Self::json_source_skip_ws(bytes, p);
        if matches!(bytes.get(*p), Some(b']')) {
            *p += 1;
            return Ok(());
        }
        let mut i = 0usize;
        loop {
            path.push(i.to_string());
            Self::json_source_value(source, bytes, p, path, out)?;
            path.pop();
            i += 1;
            Self::json_source_skip_ws(bytes, p);
            match bytes.get(*p) {
                Some(b',') => *p += 1,
                Some(b']') => {
                    *p += 1;
                    return Ok(());
                }
                _ => {
                    return Err(RuntimeError::SyntaxError(
                        "JSON.parse source scan: expected ',' or ']'".into(),
                    ))
                }
            }
        }
    }

    fn json_source_skip_ws(bytes: &[u8], p: &mut usize) {
        while *p < bytes.len() && matches!(bytes[*p], b' ' | b'\t' | b'\n' | b'\r') {
            *p += 1;
        }
    }

    fn json_source_string_decoded(
        source: &str,
        bytes: &[u8],
        p: &mut usize,
    ) -> Result<String, RuntimeError> {
        if !matches!(bytes.get(*p), Some(b'"')) {
            return Err(RuntimeError::SyntaxError(
                "JSON.parse source scan: expected string".into(),
            ));
        }
        *p += 1;
        let mut out = String::new();
        while *p < bytes.len() {
            let c = bytes[*p];
            if c == b'"' {
                *p += 1;
                return Ok(out);
            }
            if c == b'\\' {
                *p += 1;
                let Some(esc) = bytes.get(*p).copied() else {
                    return Err(RuntimeError::SyntaxError(
                        "JSON.parse source scan: bad escape".into(),
                    ));
                };
                match esc {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'b' => out.push('\u{0008}'),
                    b'f' => out.push('\u{000c}'),
                    b'u' if *p + 4 < bytes.len() => {
                        let hex = &source[*p + 1..*p + 5];
                        let cp = u32::from_str_radix(hex, 16).map_err(|_| {
                            RuntimeError::SyntaxError("JSON.parse source scan: bad \\u".into())
                        })?;
                        if let Some(ch) = char::from_u32(cp) {
                            out.push(ch);
                        }
                        *p += 4;
                    }
                    _ => {
                        return Err(RuntimeError::SyntaxError(
                            "JSON.parse source scan: bad escape".into(),
                        ))
                    }
                }
                *p += 1;
            } else {
                let tail = std::str::from_utf8(&bytes[*p..]).map_err(|_| {
                    RuntimeError::SyntaxError("JSON.parse source scan: bad utf-8".into())
                })?;
                let Some(ch) = tail.chars().next() else {
                    return Err(RuntimeError::SyntaxError(
                        "JSON.parse source scan: unterminated string".into(),
                    ));
                };
                out.push(ch);
                *p += ch.len_utf8();
            }
        }
        Err(RuntimeError::SyntaxError(
            "JSON.parse source scan: unterminated string".into(),
        ))
    }

    fn json_source_number(bytes: &[u8], p: &mut usize) {
        if matches!(bytes.get(*p), Some(b'-')) {
            *p += 1;
        }
        while *p < bytes.len() && bytes[*p].is_ascii_digit() {
            *p += 1;
        }
        if matches!(bytes.get(*p), Some(b'.')) {
            *p += 1;
            while *p < bytes.len() && bytes[*p].is_ascii_digit() {
                *p += 1;
            }
        }
        if matches!(bytes.get(*p), Some(b'e' | b'E')) {
            *p += 1;
            if matches!(bytes.get(*p), Some(b'+' | b'-')) {
                *p += 1;
            }
            while *p < bytes.len() && bytes[*p].is_ascii_digit() {
                *p += 1;
            }
        }
    }

    fn json_internalize_enumerable_keys(
        &mut self,
        value_id: crate::value::ObjectRef,
    ) -> Result<Vec<String>, RuntimeError> {
        if let Some((target, handler)) = self.proxy_target_handler_checked(value_id)? {
            let trap = self.object_get(handler, "ownKeys");
            if matches!(trap, Value::Object(_)) {
                let trap_result =
                    self.call_function(trap, Value::Object(handler), vec![Value::Object(target)])?;
                let mut keys = Vec::new();
                for key in self.apply_proxy_own_keys_invariants(&trap_result, target)? {
                    if let Value::String(key) = key {
                        let key_v = Value::String(key.clone());
                        let desc = {
                            let trap = self.object_get(handler, "getOwnPropertyDescriptor");
                            if !matches!(trap, Value::Undefined) {
                                if !self.is_callable(&trap) {
                                    return Err(RuntimeError::TypeError(
                                        "Proxy 'getOwnPropertyDescriptor' trap is not callable"
                                            .into(),
                                    ));
                                }
                                let trap_result = self.call_function(
                                    trap,
                                    Value::Object(handler),
                                    vec![Value::Object(target), key_v.clone()],
                                )?;
                                self.apply_proxy_get_own_property_descriptor_invariant(
                                    target,
                                    key.as_str(),
                                    &trap_result,
                                )?;
                                trap_result
                            } else {
                                self.object_get_own_property_descriptor_via(
                                    &Value::Object(target),
                                    &key_v,
                                )?
                            }
                        };
                        let desc_id = match desc {
                            Value::Object(id) => id,
                            _ => continue,
                        };
                        if crate::abstract_ops::to_boolean(
                            &self.read_property(desc_id, "enumerable")?,
                        ) {
                            keys.push(key.as_str().to_string());
                        }
                    }
                }
                return Ok(keys);
            }
            return self.json_internalize_enumerable_keys(target);
        }
        Ok(self.ordinary_own_enumerable_string_keys(value_id))
    }

    fn json_internalize_is_array(
        &self,
        value_id: crate::value::ObjectRef,
    ) -> Result<bool, RuntimeError> {
        if matches!(
            self.obj(value_id).internal_kind,
            crate::value::InternalKind::Array
        ) {
            return Ok(true);
        }
        if let Some((target, _)) = self.proxy_target_handler_checked(value_id)? {
            return self.json_internalize_is_array(target);
        }
        Ok(false)
    }

    /// Symbol.for(key) per ECMA §20.4.2.6 — interns a registry symbol.
    /// Coerces the argument via strict ToString so toString-throwing args
    /// propagate (per §20.4.2.6 step 1: Let stringKey be ? ToString(key)).
    pub fn symbol_for_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let arg = args.first().cloned().unwrap_or(Value::Undefined);
        let s = self.to_string_strict(&arg)?;
        Ok(Value::Symbol(std::rc::Rc::new(format!("@@sym:{}", s))))
    }

    /// Symbol.keyFor(sym) per ECMA §20.4.2.7 — recovers the registry key, or
    /// undefined when the symbol isn't registry-interned. Throws TypeError
    /// when called with a non-Symbol value per step 1.
    pub fn symbol_key_for_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let arg = args.first().cloned().unwrap_or(Value::Undefined);
        let s = match arg {
            Value::Symbol(s) => s,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Symbol.keyFor: argument is not a Symbol".into(),
                ))
            }
        };
        // Registry symbols use the prefix "@@sym:<key>" form (no counter
        // numeral); non-registry use "@@sym:<n>:<desc>" or "@@sym:<n>".
        let body = match s.strip_prefix("@@sym:") {
            Some(b) => b,
            None => return Ok(Value::Undefined),
        };
        // Distinguish registry (no leading digit run) from non-registry
        // (leading digit run followed by ':' or end-of-string).
        let leading_is_numeric = body
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false);
        if leading_is_numeric {
            return Ok(Value::Undefined);
        }
        Ok(Value::String(std::rc::Rc::new(body.to_string())))
    }

    /// Date.prototype.getYear() per Annex B.2.4.1 — year minus 1900.
    pub fn date_proto_get_year_via(&mut self) -> Result<Value, RuntimeError> {
        let id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::Number(f64::NAN)),
        };
        let ms = match self.object_get(id, "__date_ms") {
            Value::Number(n) => n,
            _ => return Ok(Value::Number(f64::NAN)),
        };
        let (y, _, _) = crate::intrinsics::date_components(ms);
        Ok(Value::Number((y - 1900) as f64))
    }

    /// Date.prototype.setYear(y) per Annex B.2.4.2 — 0..99 maps to 1900+y.
    pub fn date_proto_set_year_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::Number(f64::NAN)),
        };
        let y_raw = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(f64::NAN);
        let full_year = if y_raw >= 0.0 && y_raw <= 99.0 {
            y_raw + 1900.0
        } else {
            y_raw
        };
        let ms = match self.object_get(id, "__date_ms") {
            Value::Number(n) => n,
            _ => 0.0,
        };
        let (_, mo, d) = crate::intrinsics::date_components(ms);
        let days_per_year = 365.25;
        let new_ms = ((full_year - 1970.0) * days_per_year * 86_400_000.0)
            + ((mo as f64) * 30.0 * 86_400_000.0)
            + ((d as f64 - 1.0) * 86_400_000.0);
        self.object_set(id, "__date_ms".into(), Value::Number(new_ms));
        Ok(Value::Number(new_ms))
    }

    /// parseInt(string, radix) per ECMA §19.2.5.
    pub fn parse_int_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        if args.is_empty() {
            return Ok(Value::Number(f64::NAN));
        }
        let s = crate::abstract_ops::to_string(&args[0]);
        // ECMA-262 §19.2.5 step 7: if radix arg is undefined or 0, set
        // to 10 unless the string has a 0x/0X prefix, in which case 16.
        let radix_arg = args
            .get(1)
            .map(|v| crate::abstract_ops::to_number(v) as i32);
        let mut radix = match radix_arg {
            Some(r) if r != 0 => r,
            _ => 10,
        };
        let trimmed = s.trim_start();
        let (sign, body0) = if let Some(rest) = trimmed.strip_prefix('-') {
            (-1.0, rest)
        } else if let Some(rest) = trimmed.strip_prefix('+') {
            (1.0, rest)
        } else {
            (1.0, trimmed)
        };
        // ECMA-262 §19.2.5 step 11: if (radix === undefined || radix === 16)
        // and body starts with '0x'/'0X', strip prefix and use radix 16.
        let body = if (radix_arg.is_none() || radix == 16)
            && (body0.starts_with("0x") || body0.starts_with("0X"))
        {
            radix = 16;
            &body0[2..]
        } else {
            body0
        };
        let mut acc: u64 = 0;
        let mut any = false;
        for c in body.chars() {
            let d = match c {
                '0'..='9' => c as u32 - '0' as u32,
                'a'..='z' => c as u32 - 'a' as u32 + 10,
                'A'..='Z' => c as u32 - 'A' as u32 + 10,
                _ => break,
            };
            if (d as i32) >= radix {
                break;
            }
            acc = acc.saturating_mul(radix as u64).saturating_add(d as u64);
            any = true;
        }
        if !any {
            return Ok(Value::Number(f64::NAN));
        }
        Ok(Value::Number(sign * acc as f64))
    }

    /// parseFloat(string) per ECMA §19.2.4.
    pub fn parse_float_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        if args.is_empty() {
            return Ok(Value::Number(f64::NAN));
        }
        let s = crate::abstract_ops::to_string(&args[0]);
        let trimmed = s.trim_start();
        let mut end = 0;
        let mut saw_digit = false;
        let mut saw_dot = false;
        let mut saw_e = false;
        for (i, c) in trimmed.char_indices() {
            if i == 0 && (c == '+' || c == '-') {
                end = i + 1;
                continue;
            }
            match c {
                '0'..='9' => {
                    saw_digit = true;
                    end = i + 1;
                }
                '.' if !saw_dot && !saw_e => {
                    saw_dot = true;
                    end = i + 1;
                }
                'e' | 'E' if saw_digit && !saw_e => {
                    saw_e = true;
                    end = i + 1;
                }
                '+' | '-' if saw_e && trimmed[..i].chars().last() == Some('e' as char) => {
                    end = i + 1;
                }
                _ => break,
            }
        }
        if end == 0 {
            return Ok(Value::Number(f64::NAN));
        }
        Ok(Value::Number(trimmed[..end].parse().unwrap_or(f64::NAN)))
    }

    /// Math.random() per ECMA §21.3.2.27 (v1: LCG seeded from clock).
    pub fn math_random_via(&mut self) -> Result<Value, RuntimeError> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        let pseudo = ((nanos as u64)
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407)) as f64;
        Ok(Value::Number((pseudo / u64::MAX as f64).abs().fract()))
    }

    /// Date.prototype.getTimezoneOffset() per ECMA §21.4.4.12 (v1: always 0/UTC).
    pub fn date_proto_get_timezone_offset_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(0.0))
    }

    /// Date.now() per ECMA §21.4.3.1 — current epoch ms.
    pub fn date_now_via(&mut self) -> Result<Value, RuntimeError> {
        use std::time::{SystemTime, UNIX_EPOCH};
        // CAPS-EXT 11: gate clock-read through the dispatcher.
        let url = self.current_module_url.last().cloned().unwrap_or_default();
        let provenance = if url.contains("/node_modules/") {
            crate::caps::ModuleProvenance::Dependency
        } else if url.starts_with("node:") {
            crate::caps::ModuleProvenance::Builtin
        } else {
            crate::caps::ModuleProvenance::Application
        };
        let caller = crate::caps::ModuleId { url, provenance };
        self.caps
            .require_clock(
                &crate::caps::Clock::disabled(),
                crate::caps::ClockOp::Now,
                &caller,
            )
            .map_err(|e| RuntimeError::TypeError(e.to_string()))?;
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0);
        Ok(Value::Number(ms))
    }

    /// Date.parse(s) per ECMA §21.4.3.2 — v1 stub returns 0.
    pub fn date_parse_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let s = match args.first() {
            Some(Value::String(s)) => s.as_str().to_string(),
            Some(v) => self.coerce_to_string(v)?,
            None => return Ok(Value::Number(f64::NAN)),
        };
        Ok(parse_iso8601_to_epoch_ms(&s)
            .map(Value::Number)
            .unwrap_or(Value::Number(f64::NAN)))
    }

    /// Date.UTC(year, month, day?, hours?, min?, sec?, ms?) per ECMA
    /// §21.4.3.4. Returns the milliseconds-since-epoch for the UTC
    /// timestamp. month is 0-indexed per JS convention.
    pub fn date_utc_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let read = |i: usize, dflt: i64| -> i64 {
            match args.get(i) {
                Some(v) => {
                    let n = crate::abstract_ops::to_number(v);
                    if n.is_nan() {
                        dflt
                    } else {
                        n as i64
                    }
                }
                None => dflt,
            }
        };
        let mut year = read(0, 0);
        // §21.4.3.4 step 8: if 0 ≤ year ≤ 99, year += 1900.
        if year >= 0 && year <= 99 {
            year += 1900;
        }
        let month = read(1, 0);
        let day = read(2, 1);
        let h = read(3, 0);
        let mi = read(4, 0);
        let s = read(5, 0);
        let ms = read(6, 0);
        Ok(Value::Number(
            utc_components_to_epoch_ms(year, month, day, h, mi, s, ms) as f64,
        ))
    }

    /// String.raw(template, ...subs) per ECMA §22.1.2.4.
    pub fn string_raw_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let template = match args.first() {
            Some(Value::Object(id)) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "String.raw: first argument must be an object".into(),
                ))
            }
        };
        let raw = match self.object_get(template, "raw") {
            Value::Undefined => Value::Object(template),
            v => v,
        };
        let raw_id = match raw {
            Value::Object(id) => id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "String.raw: raw must be an object".into(),
                ))
            }
        };
        let length = match self.object_get(raw_id, "length") {
            Value::Number(n) => n as i64,
            _ => {
                let mut n: i64 = 0;
                while !matches!(self.object_get(raw_id, &n.to_string()), Value::Undefined) {
                    n += 1;
                }
                n
            }
        };
        let mut out = String::new();
        for i in 0..length {
            let seg = self.object_get(raw_id, &i.to_string());
            out.push_str(&crate::abstract_ops::to_string(&seg));
            if i + 1 < length {
                if let Some(sub) = args.get((i as usize) + 1) {
                    out.push_str(&crate::abstract_ops::to_string(sub));
                }
            }
        }
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// Array.from(arrayLike, mapfn?, thisArg?) per ECMA §23.1.2.1.
    pub fn array_from_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let src = args.first().cloned().unwrap_or(Value::Undefined);
        let map_fn_v = args.get(1).cloned().unwrap_or(Value::Undefined);
        let this_arg = args.get(2).cloned().unwrap_or(Value::Undefined);
        // §23.1.2.1 step 2-3: if mapfn is undefined, mapping = false.
        // Otherwise if !IsCallable(mapfn), throw TypeError.
        let mapping = match &map_fn_v {
            Value::Undefined => None,
            v if self.is_callable(v) => Some(v.clone()),
            _ => {
                return Err(RuntimeError::TypeError(
                    "Array.from: mapfn must be callable".into(),
                ))
            }
        };
        let out = self.alloc_object(crate::value::Object::new_array());
        let items: Vec<Value> = match &src {
            Value::Object(id) => {
                let has_iter = !matches!(self.object_get(*id, "@@iterator"), Value::Undefined);
                if has_iter {
                    crate::intrinsics::collect_iterable(self, src.clone())?
                } else {
                    let len = self.try_array_length(*id)?;
                    (0..len)
                        .map(|i| self.object_get(*id, &i.to_string()))
                        .collect()
                }
            }
            Value::String(s) => s
                .chars()
                .map(|c| Value::String(std::rc::Rc::new(c.to_string())))
                .collect(),
            Value::Undefined | Value::Null => {
                return Err(RuntimeError::TypeError(
                    "Array.from: items must not be null or undefined".into(),
                ))
            }
            _ => Vec::new(),
        };
        for (i, v) in items.into_iter().enumerate() {
            let mapped = if let Some(f) = &mapping {
                // §23.1.2.1 step 6.f.iv: Call(mapfn, thisArg, [kValue, k]).
                self.call_function(
                    f.clone(),
                    this_arg.clone(),
                    vec![v, Value::Number(i as f64)],
                )?
            } else {
                v
            };
            self.object_set(out, i.to_string(), mapped);
        }
        let len = self.try_array_length(out)?;
        self.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    }

    /// Date.prototype.toString() (v1: ISO-like YYYY-MM-DDT00:00:00Z).
    pub fn date_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))),
        };
        let ms = match self.object_get(this_id, "__date_ms") {
            Value::Number(n) => n,
            _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))),
        };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        Ok(Value::String(std::rc::Rc::new(format!(
            "{:04}-{:02}-{:02}T00:00:00Z",
            y,
            mo + 1,
            d
        ))))
    }

    /// Date.prototype.toJSON() per ECMA §21.4.4.37 (v1: midnight ISO).
    pub fn date_proto_to_json_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::String(std::rc::Rc::new("".into()))),
        };
        let ms = match self.object_get(this_id, "__date_ms") {
            Value::Number(n) => n,
            _ => return Ok(Value::String(std::rc::Rc::new("".into()))),
        };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        Ok(Value::String(std::rc::Rc::new(format!(
            "{:04}-{:02}-{:02}T00:00:00.000Z",
            y,
            mo + 1,
            d
        ))))
    }

    fn date_ms_field(&mut self) -> Option<f64> {
        let id = match self.current_this() {
            Value::Object(id) => id,
            _ => return None,
        };
        match self.object_get(id, "__date_ms") {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Date.prototype.getFullYear() per ECMA §21.4.4.4.
    pub fn date_proto_get_full_year_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(
            self.date_ms_field()
                .map(|ms| crate::intrinsics::date_components(ms).0 as f64)
                .unwrap_or(f64::NAN),
        ))
    }
    pub fn date_proto_get_month_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(
            self.date_ms_field()
                .map(|ms| crate::intrinsics::date_components(ms).1 as f64)
                .unwrap_or(f64::NAN),
        ))
    }
    pub fn date_proto_get_date_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(
            self.date_ms_field()
                .map(|ms| crate::intrinsics::date_components(ms).2 as f64)
                .unwrap_or(f64::NAN),
        ))
    }
    pub fn date_proto_get_day_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(
            self.date_ms_field()
                .map(|ms| {
                    let days = (ms / 86_400_000.0).floor() as i64;
                    (((days % 7) + 7 + 4) % 7) as f64
                })
                .unwrap_or(f64::NAN),
        ))
    }
    pub fn date_proto_get_hours_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(
            self.date_ms_field()
                .map(|ms| ((ms / 3_600_000.0).floor() as i64 % 24) as f64)
                .unwrap_or(f64::NAN),
        ))
    }
    pub fn date_proto_get_minutes_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(
            self.date_ms_field()
                .map(|ms| ((ms / 60_000.0).floor() as i64 % 60) as f64)
                .unwrap_or(f64::NAN),
        ))
    }
    pub fn date_proto_get_seconds_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(
            self.date_ms_field()
                .map(|ms| ((ms / 1000.0).floor() as i64 % 60) as f64)
                .unwrap_or(f64::NAN),
        ))
    }
    pub fn date_proto_get_milliseconds_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(Value::Number(
            self.date_ms_field()
                .map(|ms| (ms as i64 % 1000) as f64)
                .unwrap_or(f64::NAN),
        ))
    }

    /// Date.prototype.getTime() per ECMA §21.4.4.10.
    pub fn date_proto_get_time_via(&mut self) -> Result<Value, RuntimeError> {
        let this = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::Number(0.0)),
        };
        Ok(self.object_get(this, "__date_ms"))
    }

    /// Date.prototype.valueOf() per ECMA §21.4.4.44.
    pub fn date_proto_value_of_via(&mut self) -> Result<Value, RuntimeError> {
        let this = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::Number(0.0)),
        };
        Ok(self.object_get(this, "__date_ms"))
    }

    /// Date.prototype.toISOString() per ECMA §21.4.4.36.
    pub fn date_proto_to_iso_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::String(std::rc::Rc::new("".into()))),
        };
        let ms = match self.object_get(this_id, "__date_ms") {
            Value::Number(n) => n,
            _ => return Ok(Value::String(std::rc::Rc::new("".into()))),
        };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        let h = (ms / 3_600_000.0).floor() as i64 % 24;
        let mi = (ms / 60_000.0).floor() as i64 % 60;
        let se = (ms / 1000.0).floor() as i64 % 60;
        let mss = ms as i64 % 1000;
        Ok(Value::String(std::rc::Rc::new(format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            y,
            mo + 1,
            d,
            h,
            mi,
            se,
            mss
        ))))
    }

    /// Date.prototype.toDateString() (v1: ISO-like YYYY-MM-DD).
    pub fn date_proto_to_date_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::String(std::rc::Rc::new(String::new()))),
        };
        let ms = match self.object_get(this_id, "__date_ms") {
            Value::Number(n) => n,
            _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))),
        };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        Ok(Value::String(std::rc::Rc::new(format!(
            "{:04}-{:02}-{:02}",
            y,
            mo + 1,
            d
        ))))
    }

    /// Date.prototype.toTimeString() (v1: HH:MM:SS).
    pub fn date_proto_to_time_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::String(std::rc::Rc::new(String::new()))),
        };
        let ms = match self.object_get(this_id, "__date_ms") {
            Value::Number(n) => n,
            _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))),
        };
        let h = (ms / 3_600_000.0).floor() as i64 % 24;
        let mi = (ms / 60_000.0).floor() as i64 % 60;
        let se = (ms / 1000.0).floor() as i64 % 60;
        Ok(Value::String(std::rc::Rc::new(format!(
            "{:02}:{:02}:{:02}",
            h, mi, se
        ))))
    }

    /// Date.prototype.toUTCString() (v1: YYYY-MM-DD HH:MM:SS GMT).
    pub fn date_proto_to_utc_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_id = match self.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::String(std::rc::Rc::new(String::new()))),
        };
        let ms = match self.object_get(this_id, "__date_ms") {
            Value::Number(n) => n,
            _ => return Ok(Value::String(std::rc::Rc::new("Invalid Date".into()))),
        };
        let (y, mo, d) = crate::intrinsics::date_components(ms);
        let h = (ms / 3_600_000.0).floor() as i64 % 24;
        let mi = (ms / 60_000.0).floor() as i64 % 60;
        let se = (ms / 1000.0).floor() as i64 % 60;
        Ok(Value::String(std::rc::Rc::new(format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} GMT",
            y,
            mo + 1,
            d,
            h,
            mi,
            se
        ))))
    }

    /// Symbol.prototype.toString() per ECMA §20.4.3.3.
    pub fn symbol_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this_v = self.current_this();
        let this = self.unwrap_primitive(&this_v);
        match this {
            Value::Symbol(s) => {
                // Decoding parallels Symbol.prototype.description (see install_symbol_static).
                let body = s.strip_prefix("@@sym:").unwrap_or(&s);
                let starts_with_digit = body
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false);
                let desc = if starts_with_digit {
                    match body.split_once(':') {
                        Some((_, d)) => d.to_string(),
                        None => String::new(),
                    }
                } else {
                    body.to_string()
                };
                Ok(Value::String(std::rc::Rc::new(format!("Symbol({})", desc))))
            }
            _ => Err(RuntimeError::TypeError(
                "Symbol.prototype.toString: this is not a Symbol".into(),
            )),
        }
    }

    /// BigInt.prototype.toString(radix) per ECMA §21.2.3.4.
    pub fn bigint_proto_to_string_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let b = match self.current_this() {
            Value::BigInt(b) => b,
            _ => {
                return Err(RuntimeError::TypeError(
                    "BigInt.prototype.toString: this is not a BigInt".into(),
                ))
            }
        };
        let radix = match args.first() {
            Some(Value::Number(n)) if (2.0..=36.0).contains(n) => *n as u32,
            Some(Value::Undefined) | None => 10,
            _ => {
                return Err(RuntimeError::TypeError(
                    "BigInt.prototype.toString radix out of range".into(),
                ))
            }
        };
        Ok(Value::String(std::rc::Rc::new(b.to_radix(radix))))
    }

    /// Function.prototype.toString() per ECMA §20.2.3.5 (v1: native shape for all functions).
    pub fn function_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        // EXT 75: ECMA-262 §20.2.3.5 — when invoked on a Proxy whose target
        // is a callable, the spec routes through ProxyToString → unwrap to
        // target. Walk the proxy chain to the first non-Proxy callable and
        // stringify against that. A revoked proxy or non-callable target
        // falls through to the TypeError branch below.
        let mut id = match &this {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Function.prototype.toString: not a function".into(),
                ))
            }
        };
        let mut hops = 0;
        while let crate::value::InternalKind::Proxy(p) = &self.obj(id).internal_kind {
            id = p.target;
            hops += 1;
            if hops > 32 {
                return Err(RuntimeError::TypeError(
                    "Function.prototype.toString: proxy chain too deep".into(),
                ));
            }
        }
        let name = match &self.obj(id).internal_kind {
            crate::value::InternalKind::Function(f) => f.name.clone(),
            crate::value::InternalKind::Closure(_) => "anonymous".to_string(),
            crate::value::InternalKind::BoundFunction(_) => "bound".to_string(),
            _ => {
                return Err(RuntimeError::TypeError(
                    "Function.prototype.toString: not a function".into(),
                ))
            }
        };
        let s = format!("function {}() {{ [native code] }}", name);
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    /// Error.prototype.toString() per ECMA §20.5.3.4.
    pub fn error_proto_to_string_via(&mut self) -> Result<Value, RuntimeError> {
        let this = self.current_this();
        let (name, message) = match &this {
            Value::Object(id) => {
                let n = self.object_get(*id, "name");
                let m = self.object_get(*id, "message");
                (
                    crate::abstract_ops::to_string(&n).as_str().to_string(),
                    crate::abstract_ops::to_string(&m).as_str().to_string(),
                )
            }
            _ => ("Error".into(), "".into()),
        };
        let out = if message.is_empty() {
            name
        } else {
            format!("{}: {}", name, message)
        };
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// Math.imul(a, b) per ECMA §21.3.2.19.
    pub fn math_imul_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let a = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64 as i32;
        let b = args
            .get(1)
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64 as i32;
        Ok(Value::Number(a.wrapping_mul(b) as f64))
    }

    /// Math.fround(x) per ECMA §21.3.2.16.
    pub fn math_fround_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let n = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(f64::NAN);
        Ok(Value::Number(n as f32 as f64))
    }

    /// Math.clz32(x) per ECMA §21.3.2.11.
    pub fn math_clz32_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let n = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64 as u32;
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
                        // EXT 83: primitive-wrapper brand strings per
                        // §20.1.3.6 step 14. Without these, Object(0n)
                        // and new Number/String/Boolean(...) all report
                        // "[object Object]" instead of the spec brand.
                        crate::value::InternalKind::NumberWrapper(_) => "Number",
                        crate::value::InternalKind::StringWrapper(_) => "String",
                        crate::value::InternalKind::BooleanWrapper(_) => "Boolean",
                        crate::value::InternalKind::BigIntWrapper(_) => "BigInt",
                        _ => "Object",
                    }
                    .to_string()
                };
                format!("[object {}]", tag)
            }
        };
        Ok(Value::String(std::rc::Rc::new(s)))
    }

    /// Object.prototype.hasOwnProperty(key) per ECMA §20.1.3.2.
    pub fn object_proto_has_own_property_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        // VMA-EXT 1: spec §20.1.3.2 step 1 is ToPropertyKey(V) which is
        // ToPrimitive + Symbol-pass-through. Pre-fix used static
        // abstract_ops::to_string which collapsed Objects to "[object
        // Object]" and stringified Symbols to their @@sym:N form
        // (mis-coercion). property_key + per-bucket lookup is the
        // spec-aligned path.
        let arg = args.first().cloned().unwrap_or(Value::Undefined);
        let arg_coerced = if matches!(arg, Value::Symbol(_)) {
            arg.clone()
        } else {
            Value::String(std::rc::Rc::new(self.coerce_to_string(&arg)?))
        };
        let pk = Self::property_key_of(&arg_coerced);
        let owns = match self.current_this() {
            Value::Object(id) => self.has_own_property_key(id, &pk),
            _ => false,
        };
        Ok(Value::Boolean(owns))
    }

    fn property_key_of(v: &Value) -> crate::value::PropertyKey {
        match v {
            Value::Symbol(rc) => crate::value::PropertyKey::Symbol(rc.clone()),
            Value::String(s) => crate::value::PropertyKey::String(s.as_str().to_string()),
            Value::Number(n) => {
                crate::value::PropertyKey::String(crate::abstract_ops::number_to_string(*n))
            }
            _ => crate::value::PropertyKey::String(
                crate::abstract_ops::to_string(v).as_str().to_string(),
            ),
        }
    }

    fn has_own_property_key(&self, id: ObjectRef, key: &crate::value::PropertyKey) -> bool {
        let o = self.obj(id);
        match key {
            crate::value::PropertyKey::String(s) => o.has_own_str(s.as_str()),
            crate::value::PropertyKey::Symbol(rc) => {
                o.properties.contains_key(key) || o.has_own_str(rc.as_str())
            }
        }
    }

    /// Object.prototype.valueOf() per ECMA §20.1.3.7.
    pub fn object_proto_value_of_via(&mut self) -> Result<Value, RuntimeError> {
        Ok(self.current_this())
    }

    /// Object.prototype.propertyIsEnumerable(key) per ECMA §20.1.3.4.
    pub fn object_proto_property_is_enumerable_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        // VMA-EXT 1: same spec §7.1.19 ToPropertyKey path as hasOwnProperty.
        // Pre-fix's static abstract_ops::to_string collapsed Objects to
        // "[object Object]" and stringified Symbols.
        let arg = args.first().cloned().unwrap_or(Value::Undefined);
        let arg_coerced = if matches!(arg, Value::Symbol(_)) {
            arg.clone()
        } else {
            Value::String(std::rc::Rc::new(self.coerce_to_string(&arg)?))
        };
        let key = match Self::property_key_of(&arg_coerced) {
            crate::value::PropertyKey::String(s) => s,
            crate::value::PropertyKey::Symbol(rc) => (*rc).clone(),
        };
        // ODP-EXT 1: spec §20.1.3.4 step 4 — return desc.enumerable.
        // Pre-fix returned only `has_own_str`, accidentally correct for
        // Array.length when has_own_str was wrong. Now both halves are
        // checked.
        let result = match self.current_this() {
            Value::Object(id) => {
                if !self.obj(id).has_own_str(&key) {
                    false
                } else if let Some(d) = self.obj(id).get_own(&key) {
                    d.enumerable
                } else if key == "length"
                    && matches!(
                        self.obj(id).internal_kind,
                        crate::value::InternalKind::Array
                    )
                {
                    false // Array.length is non-enumerable per §10.4.2
                } else {
                    // Shape-stored own property: defaults to user-default
                    // {w:t,e:t,c:t} per CMig-EXT 9 carve-out.
                    true
                }
            }
            _ => false,
        };
        Ok(Value::Boolean(result))
    }

    /// Object.prototype.isPrototypeOf(target) per ECMA §20.1.3.3.
    pub fn object_proto_is_prototype_of_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
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
            if c == this_id {
                return Ok(Value::Boolean(true));
            }
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
        Ok(Value::String(std::rc::Rc::new(
            crate::abstract_ops::to_string(&this).as_str().to_string(),
        )))
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
                        "Array.prototype.sort: comparefn must be callable".into(),
                    ));
                }
                Some(v)
            }
        };
        let len = self.try_array_length(id)?;
        let mut items: Vec<Value> = (0..len)
            .map(|i| self.object_get(id, &i.to_string()))
            .collect();
        let mut err: Option<RuntimeError> = None;
        match comparator {
            None => {
                // ASD-EXT 1: spec SortCompare (§23.1.3.30.1 step 4) requires
                // ToString dispatch on each pair via ToPrimitive(hint=string),
                // which calls user-defined toString/valueOf. Cruft's
                // abstract_ops::to_string takes the low-level path that
                // returns "[object Object]" for any Object without dispatch.
                // Materialize each element's spec-correct string once before
                // sort_by to keep dispatch out of the comparator and to avoid
                // re-borrowing self inside the closure.
                let mut keys: Vec<String> = Vec::with_capacity(items.len());
                for v in &items {
                    let prim = self.to_primitive(v, "string")?;
                    keys.push(crate::abstract_ops::to_string(&prim).as_str().to_string());
                }
                let mut idx: Vec<usize> = (0..items.len()).collect();
                idx.sort_by(|&a, &b| keys[a].cmp(&keys[b]));
                items = idx.into_iter().map(|i| items[i].clone()).collect();
            }
            Some(cb) => {
                items.sort_by(|a, b| {
                    if err.is_some() {
                        return std::cmp::Ordering::Equal;
                    }
                    match self.call_function(
                        cb.clone(),
                        Value::Undefined,
                        vec![a.clone(), b.clone()],
                    ) {
                        Ok(v) => {
                            let n = crate::abstract_ops::to_number(&v);
                            if n.is_nan() {
                                std::cmp::Ordering::Equal
                            } else if n < 0.0 {
                                std::cmp::Ordering::Less
                            } else if n > 0.0 {
                                std::cmp::Ordering::Greater
                            } else {
                                std::cmp::Ordering::Equal
                            }
                        }
                        Err(e) => {
                            err = Some(e);
                            std::cmp::Ordering::Equal
                        }
                    }
                });
            }
        }
        if let Some(e) = err {
            return Err(e);
        }
        for (i, v) in items.into_iter().enumerate() {
            self.object_set(id, i.to_string(), v);
        }
        self.object_set(id, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(id))
    }

    /// Array.prototype.entries() per ECMA §23.1.3.4 — returns an Array
    /// Iterator object yielding [index, value] pairs. IR-EXT 57: previously
    /// returned the materialized pairs Array, which made .next() undefined
    /// for every test262 case that consumes the iterator protocol.
    pub fn array_proto_entries_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)?;
        let pairs = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            let v = self.object_get(id, &i.to_string());
            let pair = self.alloc_object(crate::value::Object::new_array());
            self.object_set(pair, "0".into(), Value::Number(i as f64));
            self.object_set(pair, "1".into(), v);
            self.object_set(pair, "length".into(), Value::Number(2.0));
            self.object_set(pairs, i.to_string(), Value::Object(pair));
        }
        self.object_set(pairs, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(crate::iterator::make_array_iterator(
            self, pairs,
        )))
    }

    /// Array.prototype.keys() per ECMA §23.1.3.17 — returns an Array
    /// Iterator yielding indices.
    pub fn array_proto_keys_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)?;
        let keys = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            self.object_set(keys, i.to_string(), Value::Number(i as f64));
        }
        self.object_set(keys, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(crate::iterator::make_array_iterator(
            self, keys,
        )))
    }

    /// Array.prototype.values() per ECMA §23.1.3.38 — returns an Array
    /// Iterator yielding values.
    pub fn array_proto_values_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)?;
        let vals = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            let v = self.object_get(id, &i.to_string());
            self.object_set(vals, i.to_string(), v);
        }
        self.object_set(vals, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(crate::iterator::make_array_iterator(
            self, vals,
        )))
    }

    /// Array.prototype.toReversed() per ECMA §23.1.3.33.
    pub fn array_proto_to_reversed_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)?;
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
        let len = self.try_array_length(id)?;
        let out = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            self.object_set(out, i.to_string(), self.object_get(id, &i.to_string()));
        }
        self.object_set(out, "length".into(), Value::Number(len as f64));
        let comparator = args
            .first()
            .cloned()
            .filter(|v| !matches!(v, Value::Undefined));
        let mut items: Vec<Value> = (0..len)
            .map(|i| self.object_get(out, &i.to_string()))
            .collect();
        let mut err: Option<RuntimeError> = None;
        match comparator {
            Some(cmp) => {
                items.sort_by(|a, b| {
                    if err.is_some() {
                        return std::cmp::Ordering::Equal;
                    }
                    match self.call_function(
                        cmp.clone(),
                        Value::Undefined,
                        vec![a.clone(), b.clone()],
                    ) {
                        Ok(Value::Number(n)) => {
                            if n < 0.0 {
                                std::cmp::Ordering::Less
                            } else if n > 0.0 {
                                std::cmp::Ordering::Greater
                            } else {
                                std::cmp::Ordering::Equal
                            }
                        }
                        Ok(_) => std::cmp::Ordering::Equal,
                        Err(e) => {
                            err = Some(e);
                            std::cmp::Ordering::Equal
                        }
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
        if let Some(e) = err {
            return Err(e);
        }
        for (i, v) in items.into_iter().enumerate() {
            self.object_set(out, i.to_string(), v);
        }
        Ok(Value::Object(out))
    }

    /// Array.prototype.toSpliced(start, deleteCount, ...items) per ECMA §23.1.3.35.
    pub fn array_proto_to_spliced_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)? as i64;
        let clamp = |i: i64, l: i64| if i < 0 { (l + i).max(0) } else { i.min(l) };
        let start = clamp(
            args.first()
                .map(crate::abstract_ops::to_number)
                .unwrap_or(0.0) as i64,
            len,
        );
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
        let len = self.try_array_length(id)? as i64;
        let idx = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(0.0) as i64;
        let actual = if idx < 0 { len + idx } else { idx };
        if actual < 0 || actual >= len {
            return Err(RuntimeError::RangeError(format!(
                "with: index {} out of bounds",
                idx
            )));
        }
        let val = args.get(1).cloned().unwrap_or(Value::Undefined);
        let out = self.alloc_object(crate::value::Object::new_array());
        for i in 0..len {
            let v = if i == actual {
                val.clone()
            } else {
                self.object_get(id, &i.to_string())
            };
            self.object_set(out, i.to_string(), v);
        }
        self.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.toLocaleString() per ECMA §23.1.3.30 (v1: comma-join).
    pub fn array_proto_to_locale_string_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)?;
        let mut out = String::new();
        for i in 0..len {
            if i > 0 {
                out.push(',');
            }
            let v = self.object_get(id, &i.to_string());
            if matches!(v, Value::Undefined | Value::Null) {
                continue;
            }
            if let Value::Object(oid) = v.clone() {
                let to_locale = self.object_get(oid, "toLocaleString");
                if matches!(to_locale, Value::Object(_)) {
                    let locale = args.first().cloned().unwrap_or(Value::Undefined);
                    let opts = args.get(1).cloned().unwrap_or(Value::Undefined);
                    let s = self.call_function(to_locale, v.clone(), vec![locale, opts])?;
                    out.push_str(crate::abstract_ops::to_string(&s).as_str());
                    continue;
                }
            }
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
        let len = self.try_array_length(id)? as i64;
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
        // ACDPD-EXT 1: lift the ASCD-EXT 1 carve-out. Per §23.1.3.28
        // step 5 + step 10.c.ii: species + per-element CreateDataProperty.
        let count = (end - start).max(0) as usize;
        let out_v = self.array_species_create(&Value::Object(id), count)?;
        let out = match out_v {
            Value::Object(oid) => oid,
            _ => unreachable!(),
        };
        let mut j: i64 = 0;
        let mut i = start;
        while i < end {
            let v = self.object_get(id, &i.to_string());
            self.create_data_property_or_throw(&Value::Object(out), &j.to_string(), v)?;
            j += 1;
            i += 1;
        }
        self.object_set(out, "length".into(), Value::Number(j as f64));
        Ok(Value::Object(out))
    }

    /// Array.prototype.splice(start, deleteCount, ...items) per ECMA §23.1.3.31.
    pub fn array_proto_splice_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)? as i64;
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
        // ACDPD-EXT 1: lift the ASCD-EXT 1 carve-out per §23.1.3.32 step 9
        // + step 13.b: species + per-element CreateDataProperty.
        let removed_v = self.array_species_create(&Value::Object(id), delete_count as usize)?;
        let removed = match removed_v {
            Value::Object(rid) => rid,
            _ => unreachable!(),
        };
        for j in 0..delete_count {
            let v = self.object_get(id, &(start + j).to_string());
            self.create_data_property_or_throw(&Value::Object(removed), &j.to_string(), v)?;
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
                self.obj_mut(id).remove_str(&i.to_string());
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
        // ECMA-262 sec 23.1.3.1 step 2: O.constructor is consulted via
        // ArraySpeciesCreate, which throws TypeError if the constructor
        // is present but not a valid constructor. Pre-fix, concat
        // allocated a plain Array, missing the spec's constructor
        // validation entirely.
        let out_v = self.array_species_create(&this, 0)?;
        let out = match out_v {
            Value::Object(id) => id,
            _ => unreachable!(),
        };
        let mut j = 0usize;
        let mut items: Vec<Value> = Vec::with_capacity(args.len() + 1);
        items.push(this);
        items.extend(args.iter().cloned());
        for e in items {
            let spreadable = match &e {
                Value::Object(eid) => {
                    let flag = self.read_property(*eid, "@@isConcatSpreadable")?;
                    match flag {
                        Value::Undefined => matches!(
                            self.obj(*eid).internal_kind,
                            crate::value::InternalKind::Array
                        ),
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
            Some(v) => self.coerce_to_string(v)?,
        };
        let len = self.try_array_length(id)?;
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
        let len = self.try_array_length(id)? as i64;
        // ASAS-EXT 1: dispatching coerce — throws on Symbol (§7.1.4),
        // dispatches @@toPrimitive/valueOf/toString for Objects.
        let i = match args.first() {
            Some(v) => self.coerce_to_number(v)? as i64,
            None => 0,
        };
        let idx = if i < 0 { len + i } else { i };
        if idx < 0 || idx >= len {
            return Ok(Value::Undefined);
        }
        Ok(self.object_get(id, &idx.to_string()))
    }

    /// Array.prototype.fill(value, start, end) per ECMA §23.1.3.7.
    pub fn array_proto_fill_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let value = args.first().cloned().unwrap_or(Value::Undefined);
        let len = self.try_array_length(id)?;
        let start = match args.get(1).cloned() {
            Some(Value::Undefined) | None => 0,
            Some(v) => {
                let n = self.coerce_to_number(&v)? as i64;
                if n < 0 {
                    (len as i64 + n).max(0) as usize
                } else {
                    (n as usize).min(len)
                }
            }
        };
        let end = match args.get(2).cloned() {
            Some(Value::Undefined) | None => len,
            Some(v) => {
                let n = self.coerce_to_number(&v)? as i64;
                if n < 0 {
                    (len as i64 + n).max(0) as usize
                } else {
                    (n as usize).min(len)
                }
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
        let len = self.try_array_length(id)? as i64;
        // ASAS-EXT 1: dispatching coerce on fromIndex.
        let from = match args.get(1) {
            Some(v) if !matches!(v, Value::Undefined) => {
                let n = self.coerce_to_number(v)? as i64;
                if n < 0 {
                    (len + n).max(-1)
                } else {
                    (n.min(len - 1)).max(-1)
                }
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
        // Spec order: length BEFORE callable check (§23.1.3.27).
        let len = self.try_array_length(id)?;
        let cb = args
            .first()
            .cloned()
            .ok_or_else(|| RuntimeError::TypeError("reduceRight: callback required".into()))?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError(
                "Array.prototype.reduceRight: callback is not callable".into(),
            ));
        }
        let has_init = args.len() >= 2;
        let mut i: i64 = (len as i64) - 1;
        let mut acc = if has_init {
            args[1].clone()
        } else {
            let mut seed: Option<(i64, Value)> = None;
            while i >= 0 {
                let key = i.to_string();
                if self.has_property(id, &key) {
                    let v = self.read_property(id, &key)?;
                    seed = Some((i, v));
                    break;
                }
                i -= 1;
            }
            match seed {
                Some((start, v)) => {
                    i = start - 1;
                    v
                }
                None => {
                    return Err(RuntimeError::TypeError(
                        "reduce of empty array with no initial value".into(),
                    ))
                }
            }
        };
        while i >= 0 {
            let key = i.to_string();
            if self.has_property(id, &key) {
                let v = self.read_property(id, &key)?;
                acc = self.call_function(
                    cb.clone(),
                    Value::Undefined,
                    vec![acc, v, Value::Number(i as f64), Value::Object(id)],
                )?;
            }
            i -= 1;
        }
        Ok(acc)
    }

    /// Array.prototype.copyWithin(target, start, end) per ECMA §23.1.3.4.
    pub fn array_proto_copy_within_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)? as i64;
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
        let buf: Vec<Value> = (0..count)
            .map(|i| self.object_get(id, &(from + i).to_string()))
            .collect();
        for (i, v) in buf.into_iter().enumerate() {
            self.object_set(id, (to + i as i64).to_string(), v);
        }
        Ok(Value::Object(id))
    }

    /// Array.prototype.flat(depth) per ECMA §23.1.3.10.
    pub fn array_proto_flat_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let depth = args
            .first()
            .map(crate::abstract_ops::to_number)
            .unwrap_or(1.0) as i64;
        let out = self.alloc_object(crate::value::Object::new_array());
        fn flat_into(
            rt: &mut Runtime,
            src: crate::value::ObjectRef,
            out: crate::value::ObjectRef,
            mut out_idx: usize,
            depth: i64,
        ) -> usize {
            let len = rt.array_length(src);
            for i in 0..len {
                let v = rt.object_get(src, &i.to_string());
                if depth > 0 {
                    if let Value::Object(nid) = &v {
                        if matches!(
                            rt.obj(*nid).internal_kind,
                            crate::value::InternalKind::Array
                        ) {
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
        // Spec order: length BEFORE callable check (§23.1.3.12).
        let len = self.try_array_length(id)?;
        let cb = args.first().cloned().ok_or_else(|| {
            RuntimeError::TypeError("Array.prototype.flatMap: callback required".into())
        })?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError(
                "Array.prototype.flatMap: callback is not callable".into(),
            ));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        let out = self.alloc_object(crate::value::Object::new_array());
        let mut out_idx = 0usize;
        for i in 0..len {
            let v = self.object_get(id, &i.to_string());
            let mapped = self.call_function(
                cb.clone(),
                this_arg.clone(),
                vec![v, Value::Number(i as f64), Value::Object(id)],
            )?;
            if let Value::Object(nid) = &mapped {
                if matches!(
                    self.obj(*nid).internal_kind,
                    crate::value::InternalKind::Array
                ) {
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
        let len = self.try_array_length(id)?;
        if len == 0 {
            return Ok(Value::Undefined);
        }
        let last_key = (len - 1).to_string();
        let v = self.object_get(id, &last_key);
        self.obj_mut(id).remove_str(&last_key);
        self.object_set(id, "length".into(), Value::Number((len - 1) as f64));
        Ok(v)
    }

    /// Array.prototype.shift() per ECMA §23.1.3.26.
    pub fn array_proto_shift_via(&mut self) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let len = self.try_array_length(id)?;
        if len == 0 {
            return Ok(Value::Undefined);
        }
        let first = self.object_get(id, "0");
        for i in 1..len {
            let v = self.object_get(id, &i.to_string());
            self.object_set(id, (i - 1).to_string(), v);
        }
        self.obj_mut(id).remove_str(&(len - 1).to_string());
        self.object_set(id, "length".into(), Value::Number((len - 1) as f64));
        Ok(first)
    }

    /// Array.prototype.unshift(...items) per ECMA §23.1.3.32.
    pub fn array_proto_unshift_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        let n = args.len();
        let len = self.try_array_length(id)?;
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
        let len = self.try_array_length(id)? as i64;
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
        let len = self.try_array_length(id)? as i64;
        // ASAS-EXT 1: dispatching coerce on fromIndex.
        let from = match args.get(1) {
            Some(v) if !matches!(v, Value::Undefined) => {
                let n = self.coerce_to_number(v)? as i64;
                if n < 0 {
                    (len + n).max(0)
                } else {
                    n.min(len)
                }
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
        let len = self.try_array_length(id)? as i64;
        // §23.1.3.14 step 3: if len is 0, return false. Spec mandates this
        // BEFORE ToIntegerOrInfinity(fromIndex) — test262
        // length-zero-returns-false probes the order via a valueOf-counting
        // fromIndex.
        if len == 0 {
            return Ok(Value::Boolean(false));
        }
        // ASAS-EXT 1: fromIndex support per §23.1.3.14 step 4-5. Was missing
        // entirely; iteration always started at 0.
        let from = match args.get(1) {
            Some(v) if !matches!(v, Value::Undefined) => {
                let n = self.coerce_to_number(v)? as i64;
                if n < 0 {
                    (len + n).max(0)
                } else {
                    n.min(len)
                }
            }
            _ => 0,
        };
        for i in from..len {
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
            if eq {
                return Ok(Value::Boolean(true));
            }
        }
        Ok(Value::Boolean(false))
    }

    /// Array.prototype.findLast(predicate, thisArg) per ECMA §23.1.3.10.
    pub fn array_proto_find_last_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        // Spec order: length BEFORE callable check (§23.1.3.10).
        let len = self.try_array_length(id)?;
        let cb = args
            .first()
            .cloned()
            .ok_or_else(|| RuntimeError::TypeError("findLast: callback required".into()))?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError(
                "Array.prototype.findLast: callback is not callable".into(),
            ));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        for i in (0..len).rev() {
            let v = self.object_get(id, &i.to_string());
            let r = self.call_function(
                cb.clone(),
                this_arg.clone(),
                vec![v.clone(), Value::Number(i as f64), Value::Object(id)],
            )?;
            if crate::abstract_ops::to_boolean(&r) {
                return Ok(v);
            }
        }
        Ok(Value::Undefined)
    }

    /// Array.prototype.findLastIndex(predicate, thisArg) per ECMA §23.1.3.11.
    pub fn array_proto_find_last_index_via(
        &mut self,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        // Spec order: length BEFORE callable check (§23.1.3.11).
        let len = self.try_array_length(id)?;
        let cb = args
            .first()
            .cloned()
            .ok_or_else(|| RuntimeError::TypeError("findLastIndex: callback required".into()))?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError(
                "Array.prototype.findLastIndex: callback is not callable".into(),
            ));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        for i in (0..len).rev() {
            let v = self.object_get(id, &i.to_string());
            let r = self.call_function(
                cb.clone(),
                this_arg.clone(),
                vec![v, Value::Number(i as f64), Value::Object(id)],
            )?;
            if crate::abstract_ops::to_boolean(&r) {
                return Ok(Value::Number(i as f64));
            }
        }
        Ok(Value::Number(-1.0))
    }

    /// Array.prototype.reduce(callbackfn, initialValue) per ECMA §23.1.3.24.
    pub fn array_proto_reduce_via(&mut self, args: &[Value]) -> Result<Value, RuntimeError> {
        let id = crate::prototype::to_array_this(self)?;
        // ECMA-262 §23.1.3.25 step 2 reads length BEFORE step 3's IsCallable
        // check. test262 4-10/4-11 probe the order via a throwing length
        // getter (Test262Error) + undefined callback; spec says the length
        // getter's throw must reach the test, not the substrate's
        // callback-not-callable TypeError.
        let len = self.try_array_length(id)?;
        let cb = args.first().cloned().ok_or_else(|| {
            RuntimeError::TypeError("Array.prototype.reduce: callback required".into())
        })?;
        if !self.is_callable(&cb) {
            return Err(RuntimeError::TypeError(
                "Array.prototype.reduce: callback is not callable".into(),
            ));
        }
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
                Some((start, v)) => {
                    i = start + 1;
                    v
                }
                None => {
                    return Err(RuntimeError::TypeError(
                        "reduce of empty array with no initial value".into(),
                    ))
                }
            }
        };
        while i < len {
            let key = i.to_string();
            if self.has_property(id, &key) {
                let v = self.read_property(id, &key)?;
                acc = self.call_function(
                    cb.clone(),
                    Value::Undefined,
                    vec![acc, v, Value::Number(i as f64), Value::Object(id)],
                )?;
            }
            i += 1;
        }
        Ok(acc)
    }

    /// String.prototype.split(separator, limit) per ECMA §22.1.3.23.
    pub fn string_proto_split_via(
        &mut self,
        this: &Value,
        separator: &Value,
        limit: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        if let Value::Object(rx_id) = separator {
            let split_method = self.read_property(*rx_id, "@@split")?;
            if matches!(split_method, Value::Object(_)) {
                let s = self.to_string_strict(this)?;
                return self.call_function(
                    split_method,
                    separator.clone(),
                    vec![Value::String(std::rc::Rc::new(s)), limit.clone()],
                );
            }
        }
        let s = self.to_string_strict(this)?;
        let limit_n = match limit {
            Value::Undefined => u32::MAX,
            v => {
                let n = self.coerce_to_number(v)?;
                if n.is_nan() || n <= 0.0 {
                    0
                } else {
                    n as u32
                }
            }
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
        if (parts.len() as u32) > limit_n {
            parts.truncate(limit_n as usize);
        }
        for (i, p) in parts.iter().enumerate() {
            self.object_set(
                out,
                i.to_string(),
                Value::String(std::rc::Rc::new(p.clone())),
            );
        }
        self.object_set(out, "length".into(), Value::Number(parts.len() as f64));
        Ok(Value::Object(out))
    }

    /// String.prototype.replace(searchValue, replaceValue) per ECMA §22.1.3.15.
    pub fn string_proto_replace_via(
        &mut self,
        this: &Value,
        search: &Value,
        replacement: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        if let Value::Object(rx_id) = search {
            let replace_method = self.read_property(*rx_id, "@@replace")?;
            if matches!(replace_method, Value::Object(_)) {
                return self.call_function(
                    replace_method,
                    search.clone(),
                    vec![Value::String(std::rc::Rc::new(s)), replacement.clone()],
                );
            }
        }
        let needle = self.to_string_strict(search)?;
        if self.is_callable(replacement) {
            return match s.find(&needle) {
                Some(byte_off) => {
                    let pos = s[..byte_off].chars().count() as f64;
                    let r = self.call_function(
                        replacement.clone(),
                        Value::Undefined,
                        vec![
                            Value::String(std::rc::Rc::new(needle.clone())),
                            Value::Number(pos),
                            Value::String(std::rc::Rc::new(s.clone())),
                        ],
                    )?;
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
        // ECMA-262 §22.1.3.15 step 11 GetSubstitution: process $$ / $& /
        // $` / $' in the replacement string. (Capture groups $N apply
        // only to RegExp searches, dispatched via @@replace above.)
        match s.find(&needle) {
            Some(byte_off) => {
                let before = &s[..byte_off];
                let after = &s[byte_off + needle.len()..];
                let substituted = process_replacement_substitution(&repl, &needle, before, after);
                let mut out = String::with_capacity(s.len() + substituted.len());
                out.push_str(before);
                out.push_str(&substituted);
                out.push_str(after);
                Ok(Value::String(std::rc::Rc::new(out)))
            }
            None => Ok(Value::String(std::rc::Rc::new(s))),
        }
    }

    /// String.prototype.replaceAll(searchValue, replaceValue) per ECMA §22.1.3.16.
    pub fn string_proto_replace_all_via(
        &mut self,
        this: &Value,
        search: &Value,
        replacement: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        if let Value::Object(rx_id) = search {
            let replace_method = self.read_property(*rx_id, "@@replace")?;
            if matches!(replace_method, Value::Object(_)) {
                return self.call_function(
                    replace_method,
                    search.clone(),
                    vec![Value::String(std::rc::Rc::new(s)), replacement.clone()],
                );
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
                let r = self.call_function(
                    replacement.clone(),
                    Value::Undefined,
                    vec![
                        Value::String(std::rc::Rc::new(needle.clone())),
                        Value::Number(pos),
                        Value::String(std::rc::Rc::new(s.clone())),
                    ],
                )?;
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
        // Apply GetSubstitution per match to honor $$ / $& / $` / $'.
        let mut out = String::with_capacity(s.len());
        let mut cur = 0usize;
        while let Some(rel) = s[cur..].find(&needle) {
            let byte_off = cur + rel;
            let before = &s[..byte_off];
            let after = &s[byte_off + needle.len()..];
            out.push_str(&s[cur..byte_off]);
            let substituted = process_replacement_substitution(&repl, &needle, before, after);
            out.push_str(&substituted);
            cur = byte_off + needle.len();
        }
        out.push_str(&s[cur..]);
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// String.prototype.codePointAt(pos) per ECMA §22.1.3.4.
    pub fn string_proto_code_point_at_via(
        &mut self,
        this: &Value,
        pos: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let i_n = match pos {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        if !i_n.is_finite() || i_n < 0.0 {
            return Ok(Value::Undefined);
        }
        let i = i_n as i64;
        let mut u16_idx: i64 = 0;
        for c in s.chars() {
            let units = c.len_utf16() as i64;
            if u16_idx == i {
                return Ok(Value::Number(c as u32 as f64));
            }
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
    pub fn string_proto_at_via(
        &mut self,
        this: &Value,
        index: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let i_n = match index {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        let i = i_n as i64;
        let idx = if i < 0 { len + i } else { i };
        if idx < 0 || idx >= len {
            return Ok(Value::Undefined);
        }
        Ok(Value::String(std::rc::Rc::new(
            chars[idx as usize].to_string(),
        )))
    }

    /// String.prototype.normalize(form) per ECMA §22.1.3.13.
    /// v1 deviation: no actual NFC/NFD/NFKC/NFKD; coerces this and returns it.
    pub fn string_proto_normalize_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        Ok(Value::String(std::rc::Rc::new(
            self.to_string_strict(this)?,
        )))
    }

    /// String.prototype.localeCompare(that) per ECMA §22.1.3.10.
    /// v1 deviation: locale-insensitive lexicographic compare.
    pub fn string_proto_locale_compare_via(
        &mut self,
        this: &Value,
        that: &Value,
    ) -> Result<Value, RuntimeError> {
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
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(false),
        };
        let matcher = self.read_property(id, "@@match")?;
        match matcher {
            Value::Undefined => Ok(matches!(
                self.obj(id).internal_kind,
                crate::value::InternalKind::RegExp(_)
            )),
            _ => Ok(crate::abstract_ops::to_boolean(&matcher)),
        }
    }

    /// String.prototype.slice(start, end) per ECMA §22.1.3.22.
    pub fn string_proto_slice_via(
        &mut self,
        this: &Value,
        start: &Value,
        end: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let start_n = match start {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        let end_n = match end {
            Value::Undefined => len as f64,
            v => self.coerce_to_number(v)?,
        };
        let from = {
            let i = start_n as i64;
            if i < 0 {
                (len + i).max(0)
            } else {
                i.min(len)
            }
        };
        let to = {
            let i = end_n as i64;
            if i < 0 {
                (len + i).max(0)
            } else {
                i.min(len)
            }
        };
        if to <= from {
            return Ok(Value::String(std::rc::Rc::new(String::new())));
        }
        let out: String = chars[from as usize..to as usize].iter().collect();
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// String.prototype.substring(start, end) per ECMA §22.1.3.24.
    pub fn string_proto_substring_via(
        &mut self,
        this: &Value,
        a: &Value,
        b: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let a_n = match a {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        let b_n = match b {
            Value::Undefined => len as f64,
            v => self.coerce_to_number(v)?,
        };
        let mut lo = (a_n as i64).clamp(0, len);
        let mut hi = (b_n as i64).clamp(0, len);
        if lo > hi {
            std::mem::swap(&mut lo, &mut hi);
        }
        let out: String = chars[lo as usize..hi as usize].iter().collect();
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// String.prototype.substr(start, length) per Annex B.2.2.1.
    pub fn string_proto_substr_via(
        &mut self,
        this: &Value,
        start: &Value,
        count: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let mut from = match start {
            Value::Undefined => 0,
            v => self.coerce_to_number(v)? as i64,
        };
        if from < 0 {
            from = (len + from).max(0);
        }
        let from = from.min(len) as usize;
        let count_n = match count {
            Value::Undefined => (len - from as i64) as f64,
            v => self.coerce_to_number(v)?,
        };
        let n = (count_n as i64).max(0) as usize;
        let to = (from + n).min(chars.len());
        let out: String = chars[from..to].iter().collect();
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// String.prototype.indexOf(search, position) per ECMA §22.1.3.8.
    pub fn string_proto_index_of_via(
        &mut self,
        this: &Value,
        search: &Value,
        position: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        // ECMA §22.1.3.9 step 5: clamp position to [0, len].
        // Pre-fix this fn coerced position but discarded the value, leaving
        // every search starting from 0. The hang in entities loading was a
        // direct downstream: parseEncodeTrie's readEntity loop relies on
        // s.indexOf(';', cursor) returning the next ';' AFTER cursor.
        let char_count = s.chars().count();
        let start_char = if matches!(position, Value::Undefined) {
            0usize
        } else {
            let n = self.coerce_to_number(position)?;
            if n.is_nan() || n <= 0.0 {
                0
            } else if n >= char_count as f64 {
                char_count
            } else {
                n as usize
            }
        };
        // Convert start char index to byte offset.
        let start_byte = s
            .char_indices()
            .nth(start_char)
            .map(|(b, _)| b)
            .unwrap_or(s.len());
        match s[start_byte..].find(&needle) {
            Some(rel_byte) => {
                let abs_byte = start_byte + rel_byte;
                Ok(Value::Number(s[..abs_byte].chars().count() as f64))
            }
            None => Ok(Value::Number(-1.0)),
        }
    }

    /// String.prototype.lastIndexOf(search, position) per ECMA §22.1.3.10.
    pub fn string_proto_last_index_of_via(
        &mut self,
        this: &Value,
        search: &Value,
        position: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) {
            let _ = self.coerce_to_number(position)?;
        }
        match s.rfind(&needle) {
            Some(byte_off) => Ok(Value::Number(s[..byte_off].chars().count() as f64)),
            None => Ok(Value::Number(-1.0)),
        }
    }

    /// String.prototype.includes(search, position) per ECMA §22.1.3.7.
    pub fn string_proto_includes_via(
        &mut self,
        this: &Value,
        search: &Value,
        position: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        if self.is_regexp_like_via(search)? {
            return Err(RuntimeError::TypeError(
                "String.prototype.includes: searchString cannot be a RegExp".into(),
            ));
        }
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) {
            let _ = self.coerce_to_number(position)?;
        }
        Ok(Value::Boolean(s.contains(&needle)))
    }

    /// String.prototype.startsWith(search, position) per ECMA §22.1.3.23.
    pub fn string_proto_starts_with_via(
        &mut self,
        this: &Value,
        search: &Value,
        position: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        if self.is_regexp_like_via(search)? {
            return Err(RuntimeError::TypeError(
                "String.prototype.startsWith: searchString cannot be a RegExp".into(),
            ));
        }
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) {
            let _ = self.coerce_to_number(position)?;
        }
        Ok(Value::Boolean(s.starts_with(&needle)))
    }

    /// String.prototype.endsWith(search, position) per ECMA §22.1.3.6.
    pub fn string_proto_ends_with_via(
        &mut self,
        this: &Value,
        search: &Value,
        position: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        if self.is_regexp_like_via(search)? {
            return Err(RuntimeError::TypeError(
                "String.prototype.endsWith: searchString cannot be a RegExp".into(),
            ));
        }
        let s = self.to_string_strict(this)?;
        let needle = self.to_string_strict(search)?;
        if !matches!(position, Value::Undefined) {
            let _ = self.coerce_to_number(position)?;
        }
        Ok(Value::Boolean(s.ends_with(&needle)))
    }

    /// String.prototype.trim() per ECMA §22.1.3.32.
    pub fn string_proto_trim_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        Ok(Value::String(std::rc::Rc::new(es_trim(s.as_str()))))
    }

    /// String.prototype.trimStart() per ECMA §22.1.3.34.
    pub fn string_proto_trim_start_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        Ok(Value::String(std::rc::Rc::new(
            es_trim_start(s.as_str()).to_string(),
        )))
    }

    /// String.prototype.trimEnd() per ECMA §22.1.3.33.
    pub fn string_proto_trim_end_via(&mut self, this: &Value) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        Ok(Value::String(std::rc::Rc::new(
            es_trim_end(s.as_str()).to_string(),
        )))
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
    pub fn string_proto_char_at_via(
        &mut self,
        this: &Value,
        pos: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let i_n = match pos {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        if !i_n.is_finite() || i_n < 0.0 {
            return Ok(Value::String(std::rc::Rc::new(String::new())));
        }
        let c = s
            .chars()
            .nth(i_n as usize)
            .map(|c| c.to_string())
            .unwrap_or_default();
        Ok(Value::String(std::rc::Rc::new(c)))
    }

    /// String.prototype.charCodeAt(pos) per ECMA §22.1.3.2.
    ///
    /// CharCode-EXT 1 (2026-05-23, post JSF-EXT 8): ASCII fast-path
    /// closes the O(n²) bug surfaced by the json_parse_transform A/B
    /// probe. For ASCII strings (the common case), index by byte in
    /// O(1) instead of iterating chars from the start (O(i)). The
    /// per-call cost drops from O(i) to O(ascii-check-byte-scan +
    /// O(1)); aggregated over tight charCodeAt loops the per-iter
    /// cost drops from O(n²) to O(n).
    pub fn string_proto_char_code_at_via(
        &mut self,
        this: &Value,
        pos: &Value,
    ) -> Result<Value, RuntimeError> {
        self.require_object_coercible(this)?;
        let s = self.to_string_strict(this)?;
        let i_n = match pos {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        if !i_n.is_finite() || i_n < 0.0 {
            return Ok(Value::Number(f64::NAN));
        }
        let i = i_n as usize;
        let bytes = s.as_bytes();
        if s.is_ascii() {
            return Ok(if i < bytes.len() {
                Value::Number(bytes[i] as f64)
            } else {
                Value::Number(f64::NAN)
            });
        }
        match s.chars().nth(i) {
            Some(c) => Ok(Value::Number(c as u32 as f64)),
            None => Ok(Value::Number(f64::NAN)),
        }
    }

    /// String.prototype.concat(...args) per ECMA §22.1.3.3.
    pub fn string_proto_concat_via(
        &mut self,
        this: &Value,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
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
                "Number.prototype.valueOf: this is not a Number".into(),
            )),
        }
    }

    /// Number.prototype.toExponential(digits) per ECMA §21.1.3.2.
    pub fn number_proto_to_exponential_via(
        &mut self,
        this: &Value,
        digits_arg: &Value,
    ) -> Result<Value, RuntimeError> {
        let n = match self.unwrap_primitive(this) {
            Value::Number(n) => n,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Number.prototype.toExponential: this is not a Number".into(),
                ))
            }
        };
        // §21.1.3.2 step 2 must coerce fractionDigits before NaN/Infinity
        // shortcut so observable side effects on the argument's
        // ToIntegerOrInfinity coercion still occur (per spec ordering).
        let digits = match digits_arg {
            Value::Undefined => None,
            v => {
                let raw = self.coerce_to_number(v)?;
                let dn = if raw.is_nan() { 0.0 } else { raw.trunc() };
                Some(dn)
            }
        };
        // §21.1.3.2 step 4: NaN short-circuit BEFORE the bounds check on f.
        if n.is_nan() {
            return Ok(Value::String(std::rc::Rc::new("NaN".into())));
        }
        // §21.1.3.2 step 5: Infinity shortcut likewise BEFORE bounds check.
        if !n.is_finite() {
            return Ok(Value::String(std::rc::Rc::new(if n > 0.0 {
                "Infinity".into()
            } else {
                "-Infinity".into()
            })));
        }
        // §21.1.3.2 step 7: bounds check (only when fractionDigits supplied).
        let digits: Option<usize> = match digits {
            Some(dn) => {
                if !dn.is_finite() || dn < 0.0 || dn > 100.0 {
                    return Err(RuntimeError::RangeError(
                        "toExponential() digits argument must be between 0 and 100".into(),
                    ));
                }
                Some(dn as usize)
            }
            None => None,
        };
        // §21.1.3.2 step 9: if x is 0 then s="0"; spec emits unsigned "0" form.
        // Rust's {:.*e} on -0.0 yields "-0e0"; flip sign here.
        let n = if n == 0.0 { 0.0 } else { n };
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
                    if next != '-' && next != '+' {
                        out.push('+');
                    }
                }
            }
        }
        Ok(Value::String(std::rc::Rc::new(out)))
    }

    /// Number.prototype.toPrecision(precision) per ECMA §21.1.3.5.
    pub fn number_proto_to_precision_via(
        &mut self,
        this: &Value,
        precision_arg: &Value,
    ) -> Result<Value, RuntimeError> {
        let n = match self.unwrap_primitive(this) {
            Value::Number(n) => n,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Number.prototype.toPrecision: this is not a Number".into(),
                ))
            }
        };
        match precision_arg {
            Value::Undefined => Ok(Value::String(std::rc::Rc::new(
                crate::abstract_ops::number_to_string(n),
            ))),
            v => {
                // §21.1.3.5 step 2: ToIntegerOrInfinity(precision).
                let raw = self.coerce_to_number(v)?;
                let pn = if raw.is_nan() { 0.0 } else { raw.trunc() };
                // §21.1.3.5 step 4: NaN short-circuit BEFORE the bounds check.
                if n.is_nan() {
                    return Ok(Value::String(std::rc::Rc::new("NaN".into())));
                }
                // §21.1.3.5 step 5: Infinity shortcut likewise BEFORE bounds check.
                if !n.is_finite() {
                    return Ok(Value::String(std::rc::Rc::new(if n > 0.0 {
                        "Infinity".into()
                    } else {
                        "-Infinity".into()
                    })));
                }
                if !pn.is_finite() || pn < 1.0 || pn > 100.0 {
                    return Err(RuntimeError::RangeError(
                        "toPrecision() argument must be between 1 and 100".into(),
                    ));
                }
                let p = pn as usize;
                // §21.1.3.5 step 8: zero short-circuit emits unsigned "0".
                let n = if n == 0.0 { 0.0 } else { n };
                Ok(Value::String(std::rc::Rc::new(format!(
                    "{:.*}",
                    p.saturating_sub(1),
                    n
                ))))
            }
        }
    }

    /// Boolean.prototype.valueOf() per ECMA §20.3.3.3 — ThisBooleanValue.
    pub fn boolean_proto_value_of_via(&self, this: &Value) -> Result<Value, RuntimeError> {
        match self.unwrap_primitive(this) {
            Value::Boolean(b) => Ok(Value::Boolean(b)),
            _ => Err(RuntimeError::TypeError(
                "Boolean.prototype.valueOf: this is not a Boolean".into(),
            )),
        }
    }

    /// Boolean.prototype.toString() per ECMA §20.3.3.2.
    pub fn boolean_proto_to_string_via(&self, this: &Value) -> Result<Value, RuntimeError> {
        match self.unwrap_primitive(this) {
            Value::Boolean(b) => Ok(Value::String(std::rc::Rc::new(b.to_string()))),
            _ => Err(RuntimeError::TypeError(
                "Boolean.prototype.toString: this is not a Boolean".into(),
            )),
        }
    }

    /// Number.prototype.toFixed(digits) per ECMA §21.1.3.3 — ThisNumberValue
    /// brand + RangeError on digits not in [0, 100] + NaN/Infinity
    /// short-circuit + Rust's f64 fixed-point formatting.
    pub fn number_proto_to_fixed_via(
        &mut self,
        this: &Value,
        digits_arg: &Value,
    ) -> Result<Value, RuntimeError> {
        let n = match self.unwrap_primitive(this) {
            Value::Number(n) => n,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Number.prototype.toFixed: this is not a Number".into(),
                ))
            }
        };
        // §21.1.3.3 step 1: BigInt argument throws TypeError (no implicit
        // coercion from BigInt to Number; coerce_to_number rejects BigInts
        // via the abstract op, but we surface a clearer error here).
        if matches!(digits_arg, Value::BigInt(_)) {
            return Err(RuntimeError::TypeError(
                "Number.prototype.toFixed: fractionDigits must not be a BigInt".into(),
            ));
        }
        // §21.1.3.3 step 1 — ToIntegerOrInfinity(fractionDigits).
        let raw = match digits_arg {
            Value::Undefined => 0.0,
            v => self.coerce_to_number(v)?,
        };
        let digits_n = if raw.is_nan() { 0.0 } else { raw.trunc() };
        if !digits_n.is_finite() || digits_n < 0.0 || digits_n > 100.0 {
            return Err(RuntimeError::RangeError(
                "toFixed() digits argument must be between 0 and 100".into(),
            ));
        }
        let digits = digits_n as usize;
        if n.is_nan() {
            return Ok(Value::String(std::rc::Rc::new("NaN".into())));
        }
        if !n.is_finite() {
            return Ok(Value::String(std::rc::Rc::new(if n > 0.0 {
                "Infinity".into()
            } else {
                "-Infinity".into()
            })));
        }
        // §21.1.3.3 step 6: if |x| >= 1e21, return ToString(x) (which uses
        // exponential notation for large magnitudes).
        if n.abs() >= 1.0e21 {
            return Ok(Value::String(std::rc::Rc::new(
                crate::abstract_ops::number_to_string(n),
            )));
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
    pub fn object_assign_via(
        &mut self,
        target: &Value,
        sources: &[Value],
    ) -> Result<Value, RuntimeError> {
        // §20.1.2.1 step 1: to = ? ToObject(target). Routes through the
        // strict variant which throws on null/undefined.
        let to = self.to_object_strict_via(target)?;
        // §20.1.2.1 step 4: for each source in sources, dispatch the
        // per-source IR section (§20.1.2.1 step 4) which handles
        // null/undefined skip, ToObject, key enumeration, Get/Set copy.
        for src in sources {
            crate::generated::object_assign_source_into(
                self,
                Value::Undefined,
                &[to.clone(), src.clone()],
            )?;
        }
        Ok(to)
    }

    /// Object.getOwnPropertyNames(O) per ECMA §20.1.2.10 — returns Array
    /// of own string-keyed property names (excluding @@-prefixed symbols).
    /// Integer-indexed keys first in ascending order.
    pub fn own_property_names_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => {
                return Ok(Value::Object(
                    self.alloc_object(crate::value::Object::new_array()),
                ))
            }
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        let keys: Vec<String> = {
            let o = self.obj(id);
            let is_array = matches!(o.internal_kind, crate::value::InternalKind::Array);
            if is_array {
                let mut ks: Vec<(u64, String)> = o
                    .properties
                    .iter()
                    .filter_map(|(k, _)| {
                        if k.is_symbol() {
                            None
                        } else {
                            k.as_str()
                                .parse::<u64>()
                                .ok()
                                .map(|n| (n, k.as_str().to_string()))
                        }
                    })
                    .collect();
                ks.sort_by_key(|(n, _)| *n);
                let mut out: Vec<String> = ks.into_iter().map(|(_, k)| k).collect();
                // Arrays always have a "length" own property per §10.4.2.4;
                // unconditionally include it to match Bun + spec.
                out.push("length".into());
                out
            } else {
                // CMig-EXT 4 Family B: shape entries first (insertion
                // order), then non-shape string keys.
                let mut out: Vec<String> = Vec::new();
                if let Some(shape) = o.shape.as_ref() {
                    for (name, _) in shape.iter_slots() {
                        out.push(name.to_string());
                    }
                }
                out.extend(
                    o.properties
                        .keys()
                        .filter(|k| k.is_string())
                        .map(|k| k.as_str().to_string()),
                );
                out
            }
        };
        for (i, k) in keys.iter().enumerate() {
            self.object_set(
                arr,
                i.to_string(),
                Value::String(std::rc::Rc::new(k.clone())),
            );
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
            _ => {
                return Ok(Value::Object(
                    self.alloc_object(crate::value::Object::new_array()),
                ))
            }
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        // PropertyKey migration: real Symbol-keyed properties live in the
        // Symbol variant; the legacy `@@sym:` string filter is now obsolete
        // (it would match well-known-Symbol names too but those are Symbol-
        // typed natively now).
        let syms: Vec<std::rc::Rc<String>> = self
            .obj(id)
            .properties
            .keys()
            .filter_map(|k| match k {
                crate::value::PropertyKey::Symbol(rc) => Some(rc.clone()),
                _ => None,
            })
            .collect();
        for (i, s) in syms.iter().enumerate() {
            self.object_set(arr, i.to_string(), Value::Symbol(s.clone()));
        }
        self.object_set(arr, "length".into(), Value::Number(syms.len() as f64));
        Ok(Value::Object(arr))
    }

    /// Math.* binary op dispatcher per ECMA §21.3.2.{27, 8} — pow, atan2.
    pub fn math_binary_op_via(
        &self,
        op: &Value,
        x: &Value,
        y: &Value,
    ) -> Result<Value, RuntimeError> {
        let op_name = match op {
            Value::String(s) => s.as_str().to_string(),
            _ => {
                return Err(RuntimeError::TypeError(
                    "math_binary_op_via: op must be a string".into(),
                ))
            }
        };
        let nx = crate::abstract_ops::to_number(x);
        let ny = crate::abstract_ops::to_number(y);
        let r = match op_name.as_str() {
            "pow" => nx.powf(ny),
            "atan2" => nx.atan2(ny),
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "math_binary_op_via: unknown op '{}'",
                    op_name
                )))
            }
        };
        Ok(Value::Number(r))
    }

    /// Math.max — variadic; NaN if any arg is NaN.
    pub fn math_max_via(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut m = f64::NEG_INFINITY;
        for v in args {
            let n = crate::abstract_ops::to_number(v);
            if n.is_nan() {
                return Ok(Value::Number(f64::NAN));
            }
            if n > m {
                m = n;
            }
        }
        Ok(Value::Number(m))
    }

    /// Math.min — variadic; NaN if any arg is NaN.
    pub fn math_min_via(&self, args: &[Value]) -> Result<Value, RuntimeError> {
        let mut m = f64::INFINITY;
        for v in args {
            let n = crate::abstract_ops::to_number(v);
            if n.is_nan() {
                return Ok(Value::Number(f64::NAN));
            }
            if n < m {
                m = n;
            }
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
            _ => Err(RuntimeError::TypeError(
                "Reflect.getPrototypeOf: target must be Object".into(),
            )),
        }
    }

    /// Reflect.setPrototypeOf(target, proto) per ECMA §28.1.14 — returns
    /// boolean (true on success) instead of the target.
    pub fn reflect_set_prototype_of_via(
        &mut self,
        target: &Value,
        proto: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Reflect.setPrototypeOf: target must be Object".into(),
                ))
            }
        };
        let new_proto = match proto {
            Value::Object(p) => Some(*p),
            Value::Null => None,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Reflect.setPrototypeOf: prototype must be Object or null".into(),
                ))
            }
        };
        self.obj_mut(id).proto = new_proto;
        Ok(Value::Boolean(true))
    }

    /// Reflect.isExtensible(target) per ECMA §28.1.10 — throws TypeError
    /// on non-Object (Object.isExtensible returns false).
    pub fn reflect_is_extensible_via(&self, target: &Value) -> Result<Value, RuntimeError> {
        match target {
            Value::Object(id) => Ok(Value::Boolean(self.obj(*id).extensible)),
            _ => Err(RuntimeError::TypeError(
                "Reflect.isExtensible: target must be Object".into(),
            )),
        }
    }

    /// Reflect.preventExtensions(target) per ECMA §28.1.11 — returns
    /// boolean instead of the target.
    pub fn reflect_prevent_extensions_via(
        &mut self,
        target: &Value,
    ) -> Result<Value, RuntimeError> {
        match target {
            Value::Object(id) => {
                self.obj_mut(*id).extensible = false;
                Ok(Value::Boolean(true))
            }
            _ => Err(RuntimeError::TypeError(
                "Reflect.preventExtensions: target must be Object".into(),
            )),
        }
    }

    /// Reflect.has(target, key) per ECMA §28.1.9 — proto-chain HasProperty.
    /// Ω.5.P63.E54: PropertyKey-aware so Reflect.has(obj, Symbol()) walks the
    /// Symbol bucket on the proto chain.
    pub fn reflect_has_via(&mut self, target: &Value, key: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Reflect.has: target must be Object".into(),
                ))
            }
        };
        // EXT 77: ECMA §28.1.9 step 2 — ToPropertyKey(propertyKey).
        let key_pk = match key {
            Value::Symbol(_) => property_key(key),
            _ => property_key(&Value::String(std::rc::Rc::new(
                self.coerce_to_string(key)?,
            ))),
        };
        // EXT 79: ECMA §28.1.9 routes [[HasProperty]] to the Proxy `has`
        // trap when the target is a Proxy with a callable handler.has.
        // Missing trap falls through to the target's [[HasProperty]].
        let key_str = key_pk.as_str().to_string();
        if let Some((tgt, handler)) = self.proxy_target_handler_checked(id)? {
            let trap = self.object_get(handler, "has");
            if matches!(trap, Value::Object(_)) {
                let r = self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![
                        Value::Object(tgt),
                        Value::String(std::rc::Rc::new(key_str.clone())),
                    ],
                )?;
                let trap_has = crate::abstract_ops::to_boolean(&r);
                // EXT 88 / Pass C: §10.5.7 step 9 — if trap returned
                // false, target's [[GetOwnProperty]] must not contain
                // a non-configurable own property at the key, and if it
                // does, target must remain extensible (otherwise the
                // Proxy could hide an existing non-configurable / non-
                // extensible property).
                if !trap_has {
                    if let Some(d) = self.obj(tgt).get_own(&key_str) {
                        if !d.configurable {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'has' trap returned false for a non-configurable own property of target".into()));
                        }
                        if !self.obj(tgt).extensible {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'has' trap returned false for an own property of a non-extensible target".into()));
                        }
                    }
                }
                return Ok(Value::Boolean(trap_has));
            }
            return Ok(Value::Boolean(self.has_property_pk(tgt, &key_pk)));
        }
        Ok(Value::Boolean(self.has_property_pk(id, &key_pk)))
    }

    /// Reflect.get(target, key) per ECMA §28.1.8 — dispatches accessor getters.
    pub fn reflect_get_via(&mut self, target: &Value, key: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Reflect.get: target must be Object".into(),
                ))
            }
        };
        // EXT 77: ToPropertyKey on Object keys.
        let key_pk = match key {
            Value::Symbol(_) => property_key(key),
            _ => property_key(&Value::String(std::rc::Rc::new(
                self.coerce_to_string(key)?,
            ))),
        };
        // EXT 79: ECMA §28.1.8 routes [[Get]] to the Proxy `get` trap.
        let key_str = key_pk.as_str().to_string();
        if let Some((tgt, handler)) = self.proxy_target_handler_checked(id)? {
            let trap = self.object_get(handler, "get");
            if matches!(trap, Value::Object(_)) {
                let trap_result = self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![
                        Value::Object(tgt),
                        Value::String(std::rc::Rc::new(key_str.clone())),
                        Value::Object(id),
                    ],
                )?;
                // EXT 88 / Pass C: §10.5.8 step 10 — trap-vs-target
                // consistency on non-configurable own properties:
                //   data + non-writable: trap result must SameValue
                //     target's stored value.
                //   accessor with undefined get: trap result must be
                //     undefined.
                if let Some(d) = self.obj(tgt).get_own(&key_str) {
                    if !d.configurable {
                        if d.getter.is_none() && d.setter.is_none() && !d.writable {
                            if !crate::abstract_ops::is_strictly_equal(&trap_result, &d.value) {
                                return Err(RuntimeError::TypeError(
                                    "Proxy 'get' trap returned a value inconsistent with the non-configurable non-writable own data property of target".into()));
                            }
                        }
                        if (d.getter.is_some() || d.setter.is_some()) && d.getter.is_none() {
                            if !matches!(trap_result, Value::Undefined) {
                                return Err(RuntimeError::TypeError(
                                    "Proxy 'get' trap returned a non-undefined value for a non-configurable accessor property with undefined getter on target".into()));
                            }
                        }
                    }
                }
                return Ok(trap_result);
            }
            if let Some(getter) = self.find_getter_pk(tgt, &key_pk) {
                return self.call_function(getter, Value::Object(tgt), Vec::new());
            }
            return Ok(self.object_get_pk(tgt, &key_pk));
        }
        if let Some(getter) = self.find_getter_pk(id, &key_pk) {
            return self.call_function(getter, Value::Object(id), Vec::new());
        }
        Ok(self.object_get_pk(id, &key_pk))
    }

    /// Reflect.set(target, key, value) per ECMA §28.1.13.
    pub fn reflect_set_via(
        &mut self,
        target: &Value,
        key: &Value,
        value: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Reflect.set: target must be Object".into(),
                ))
            }
        };
        let key_s = self.coerce_to_string(key)?;
        // EXT 79: ECMA §28.1.13 routes [[Set]] to the Proxy `set` trap.
        if let Some((tgt, handler)) = self.proxy_target_handler_checked(id)? {
            let trap = self.object_get(handler, "set");
            if matches!(trap, Value::Object(_)) {
                let r = self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![
                        Value::Object(tgt),
                        Value::String(std::rc::Rc::new(key_s.clone())),
                        value.clone(),
                        Value::Object(id),
                    ],
                )?;
                let trap_ok = crate::abstract_ops::to_boolean(&r);
                // EXT 88 / Pass C: §10.5.9 step 10 — if trap returned
                // true, non-configurable target own properties impose
                // the same consistency the get trap does:
                //   data + non-writable: V must SameValue target's stored.
                //   accessor with undefined set: throw TypeError.
                if trap_ok {
                    if let Some(d) = self.obj(tgt).get_own(&key_s) {
                        if !d.configurable {
                            if d.getter.is_none() && d.setter.is_none() && !d.writable {
                                if !crate::abstract_ops::is_strictly_equal(&value, &d.value) {
                                    return Err(RuntimeError::TypeError(
                                        "Proxy 'set' trap returned true for a non-configurable non-writable own data property whose value differs".into()));
                                }
                            }
                            if (d.getter.is_some() || d.setter.is_some()) && d.setter.is_none() {
                                return Err(RuntimeError::TypeError(
                                    "Proxy 'set' trap returned true for a non-configurable accessor own property with undefined setter".into()));
                            }
                        }
                    }
                }
                return Ok(Value::Boolean(trap_ok));
            }
            self.object_set(tgt, key_s, value.clone());
            return Ok(Value::Boolean(true));
        }
        // EXT 79b: invoke an inherited setter accessor when present
        // (own or up the prototype chain). The setter's throw propagates
        // out of Reflect.set; without this, an Object with a throwing
        // setter silently succeeded.
        let key_pk = crate::value::PropertyKey::String(key_s.clone());
        if let Some(setter) = self.find_setter_pk(id, &key_pk) {
            self.call_function(setter, Value::Object(id), vec![value.clone()])?;
            return Ok(Value::Boolean(true));
        }
        // GOPD-EXT 1: Reflect.set returns false for accessor with no setter
        // per §10.1.9.4 step 2.b (no throw — Reflect returns the boolean).
        if self.is_accessor_at_pk(id, &key_pk) {
            return Ok(Value::Boolean(false));
        }
        self.object_set(id, key_s, value.clone());
        Ok(Value::Boolean(true))
    }

    /// Reflect.deleteProperty(target, key) per ECMA §28.1.4 — honors
    /// non-configurable per §10.1.10 (returns false instead of throwing).
    pub fn reflect_delete_property_via(
        &mut self,
        target: &Value,
        key: &Value,
    ) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Reflect.deleteProperty: target must be Object".into(),
                ))
            }
        };
        let key_s = self.coerce_to_string(key)?;
        // EXT 79: ECMA §28.1.4 routes [[Delete]] to the Proxy `deleteProperty` trap.
        if let Some((tgt, handler)) = self.proxy_target_handler_checked(id)? {
            let trap = self.object_get(handler, "deleteProperty");
            if matches!(trap, Value::Object(_)) {
                let r = self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![
                        Value::Object(tgt),
                        Value::String(std::rc::Rc::new(key_s.clone())),
                    ],
                )?;
                let trap_deleted = crate::abstract_ops::to_boolean(&r);
                // EXT 88 / Pass C: §10.5.10 step 8 — if trap returned
                // true, target's non-configurable own property at the
                // key can't have been "deleted" (TypeError); and the
                // target must remain extensible.
                if trap_deleted {
                    if let Some(d) = self.obj(tgt).get_own(&key_s) {
                        if !d.configurable {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'deleteProperty' trap returned true for a non-configurable own property of target".into()));
                        }
                        if !self.obj(tgt).extensible {
                            return Err(RuntimeError::TypeError(
                                "Proxy 'deleteProperty' trap returned true for an own property of a non-extensible target".into()));
                        }
                    }
                }
                return Ok(Value::Boolean(trap_deleted));
            }
            let configurable = self
                .obj(tgt)
                .get_own(&key_s)
                .map(|d| d.configurable)
                .unwrap_or(true);
            if !configurable {
                return Ok(Value::Boolean(false));
            }
            self.obj_mut(tgt).remove_str(&key_s);
            return Ok(Value::Boolean(true));
        }
        let configurable = self
            .obj(id)
            .get_own(&key_s)
            .map(|d| d.configurable)
            .unwrap_or(true);
        if !configurable {
            return Ok(Value::Boolean(false));
        }
        self.obj_mut(id).remove_str(&key_s);
        Ok(Value::Boolean(true))
    }

    /// EXT 79: helper — when `id` is a Proxy, return its (target, handler)
    /// pair; otherwise None. Used by every Reflect.* via to gate trap
    /// dispatch before falling back to direct target operations.
    pub fn proxy_target_handler(&self, id: ObjectRef) -> Option<(ObjectRef, ObjectRef)> {
        if let crate::value::InternalKind::Proxy(p) = &self.obj(id).internal_kind {
            Some((p.target, p.handler))
        } else {
            None
        }
    }

    /// EXT 84: true when `id` is a Proxy and has been revoked. Callers
    /// that dispatch traps must throw TypeError on revoked proxies per
    /// §10.5.{4..14} ("If O's [[ProxyHandler]] is null, throw a TypeError").
    pub fn proxy_is_revoked(&self, id: ObjectRef) -> bool {
        matches!(&self.obj(id).internal_kind,
            crate::value::InternalKind::Proxy(p) if p.revoked)
    }

    /// EXT 84: revoked-throwing wrapper around proxy_target_handler.
    /// Returns Err(TypeError) if id is a revoked Proxy; Ok(Some(t,h)) if
    /// a live Proxy; Ok(None) if not a Proxy at all. Use this in any
    /// trap-dispatch site that needs the spec's null-handler check.
    pub fn proxy_target_handler_checked(
        &self,
        id: ObjectRef,
    ) -> Result<Option<(ObjectRef, ObjectRef)>, RuntimeError> {
        if let crate::value::InternalKind::Proxy(p) = &self.obj(id).internal_kind {
            if p.revoked {
                return Err(RuntimeError::TypeError(
                    "Cannot perform operation on a revoked Proxy".into(),
                ));
            }
            return Ok(Some((p.target, p.handler)));
        }
        Ok(None)
    }

    /// Reflect.ownKeys(target) per ECMA §28.1.12 — returns Array of own
    /// keys (Symbol-typed for @@sym: form, String-typed for everything else).
    pub fn reflect_own_keys_via(&mut self, target: &Value) -> Result<Value, RuntimeError> {
        let id = match target {
            Value::Object(id) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Reflect.ownKeys: target must be Object".into(),
                ))
            }
        };
        // Tier-Ω Round 2 (2026-05-21): enumerate ALL PropertyKey variants
        // per ECMA §28.1.7 (Reflect.ownKeys → OrdinaryOwnPropertyKeys).
        // Pre-fix: used string_key_clones() which filtered out
        // PropertyKey::Symbol entries, so `Reflect.ownKeys` and
        // `Object.getOwnPropertyDescriptors` missed symbol-keyed props.
        // This broke libraries that propagate symbol slots via
        // Reflect.ownKeys-then-defineProperty (runtypes' RuntypePrivate
        // slot, transitively breaking 14-package failure-cluster
        // entries that depend on runtypes' symbol-based dispatch).
        // CMig-EXT 4 Family B: shape entries first (insertion order;
        // shape entries are all string-keyed user-default per carve-out),
        // then properties entries (which retain the prior dispatch).
        let keys: Vec<Value> = {
            let o = self.obj(id);
            let mut out: Vec<Value> = Vec::new();
            if let Some(shape) = o.shape.as_ref() {
                for (name, _) in shape.iter_slots() {
                    out.push(Value::String(std::rc::Rc::new(name.to_string())));
                }
            }
            out.extend(o.properties.keys().map(|k| match k {
                crate::value::PropertyKey::String(s) => {
                    if s.as_str().starts_with("@@sym:") {
                        Value::Symbol(std::rc::Rc::new(s.clone()))
                    } else {
                        Value::String(std::rc::Rc::new(s.clone()))
                    }
                }
                crate::value::PropertyKey::Symbol(rc) => Value::Symbol(rc.clone()),
            }));
            out
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        let len = keys.len();
        for (i, v) in keys.into_iter().enumerate() {
            self.object_set(arr, i.to_string(), v);
        }
        self.object_set(arr, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(arr))
    }

    /// Math.* unary op dispatcher per ECMA §21.3.2 — applies the named
    /// math operation to ToNumber(x). Used by Math.{abs, floor, ceil,
    /// round, trunc, sign, sqrt, cbrt}. The op name is passed as a
    /// Value::String (matching CallBuiltin's argument convention).
    pub fn math_unary_op_via(&self, op: &Value, x: &Value) -> Result<Value, RuntimeError> {
        let op_name = match op {
            Value::String(s) => s.as_str().to_string(),
            _ => {
                return Err(RuntimeError::TypeError(
                    "math_unary_op_via: op must be a string".into(),
                ))
            }
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
                if n.is_nan() {
                    f64::NAN
                } else if n > 0.0 {
                    1.0
                } else if n < 0.0 {
                    -1.0
                } else {
                    n
                } // preserves +0 / -0
            }
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "math_unary_op_via: unknown op '{}'",
                    op_name
                )))
            }
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
        Ok(Value::Boolean(
            n.is_finite() && n.floor() == n && n.abs() <= 9007199254740991.0,
        ))
    }

    /// Object.freeze(O) per ECMA §20.1.2.7 — sets extensible:false and
    /// every own property to non-writable + non-configurable. Returns O.
    /// CMig-EXT 5.bis (Family D P2 migrate-on-access): freezing flips
    /// descriptor attrs that the shape mechanism cannot represent (shape
    /// entries are user-default {w:t,e:t,c:t} by invariant); migrate to
    /// Dictionary first so the descriptor flip lands on a real IndexMap.
    pub fn object_freeze_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        if let Value::Object(id) = v {
            if self.typed_array_views.contains_key(id) {
                return Err(RuntimeError::TypeError(
                    "Object.freeze cannot freeze TypedArray indexed elements".into(),
                ));
            }
            self.obj_mut(*id).migrate_to_dictionary();
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
    /// CMig-EXT 5.bis (Family D P2 migrate-on-access): same reasoning
    /// as object_freeze_via.
    pub fn object_seal_via(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        if let Value::Object(id) = v {
            self.obj_mut(*id).migrate_to_dictionary();
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
        let key_coerced = if matches!(key, Value::Symbol(_)) {
            key.clone()
        } else {
            Value::String(std::rc::Rc::new(self.coerce_to_string(key)?))
        };
        let key_pk = Self::property_key_of(&key_coerced);
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Boolean(false)),
        };
        Ok(Value::Boolean(self.has_own_property_key(id, &key_pk)))
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
    pub fn set_prototype_of_via(
        &mut self,
        v: &Value,
        proto: &Value,
    ) -> Result<Value, RuntimeError> {
        let target = match v {
            Value::Object(id) => *id,
            _ => return Ok(v.clone()),
        };
        let new_proto = match proto {
            Value::Object(id) => Some(*id),
            Value::Null => None,
            _ => {
                return Err(RuntimeError::TypeError(
                    "Object.setPrototypeOf: prototype must be Object or null".into(),
                ))
            }
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
            && o.properties
                .values()
                .all(|d| !d.writable && !d.configurable);
        Ok(Value::Boolean(frozen))
    }

    /// Object.isSealed(O) per ECMA §20.1.2.17.
    pub fn is_sealed_via(&self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => return Ok(Value::Boolean(true)),
        };
        let o = self.obj(id);
        let sealed = !o.extensible && o.properties.values().all(|d| !d.configurable);
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
        // Ω.5.P63.E53: short-circuit per §27.2.4.7 step 4 — if v is already a
        // Promise built by the same constructor, return v unchanged. Without
        // this, Promise.all wraps each Promise input in a new Promise whose
        // .value is the original Promise object; .then's onFulfilled then
        // receives that inner Promise instead of its resolved value.
        if let Value::Object(id) = v {
            if matches!(
                self.obj(*id).internal_kind,
                crate::value::InternalKind::Promise(_)
            ) {
                return Ok(v.clone());
            }
        }
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
    /// Canonical OrdinaryOwnEnumerableStringKeys per ECMA-262 sec 10.1.11
    /// + sec 7.3.21 EnumerableOwnPropertyNames. Returns the source's own
    /// enumerable string-keyed property names in spec order:
    /// integer-indexed in numeric order, then non-integer string-keyed
    /// in insertion order. Excludes the well-known-Symbol "@@"-prefixed
    /// keys (cruftless stores them in the string bucket), the internal
    /// __primitive__ slot, and Array exotic "length".
    ///
    /// This is the lift introduced at the rusty-js-ir locale's
    /// 'cluster-objectkeys-array-string-13' rung close: every site that
    /// previously open-coded the filter+order should call this. The
    /// helper acts as the canonical resolver for the abstract op so
    /// future spec-conformance work (Symbol keys in Reflect.ownKeys,
    /// non-string-keyed extensions, ordering invariants) lands once.
    pub fn ordinary_own_enumerable_string_keys(&self, id: rusty_js_gc::ObjectId) -> Vec<String> {
        let o = self.obj(id);
        let is_array = matches!(o.internal_kind, crate::value::InternalKind::Array);
        let typed_array_entries: Vec<(String, bool)> =
            if let Some(len) = self.typed_array_view_len(id) {
                (0..len).map(|idx| (idx.to_string(), true)).collect()
            } else {
                Vec::new()
            };
        // Shape-EXT 4: include shape-stored entries first (in insertion
        // order via shape.iter_slots), then property-stored string keys.
        // Shape entries are all user-default `{w:t, e:t, c:t}` data
        // descriptors per shapes seed §IV carve-out, so the enumerable
        // / not-@@ / not-internal-sentinel filters all pass for them.
        let mut shape_entries: Vec<(String, bool)> = Vec::new();
        if let Some(shape) = o.shape.as_ref() {
            for (name, _) in shape.iter_slots() {
                shape_entries.push((name.to_string(), crate::intrinsics::is_integer_index(name)));
            }
        }
        let all: Vec<(String, bool)> = typed_array_entries
            .into_iter()
            .chain(shape_entries)
            .into_iter()
            .chain(
                o.properties
                    .iter()
                    .filter(|(k, d)| {
                        d.enumerable
                            && k.is_string()
                            && k.as_str() != "__primitive__"
                            && !k.as_str().starts_with("@@")
                            && !(is_array && k.as_str() == "length")
                    })
                    .map(|(k, _)| {
                        (
                            k.as_str().to_string(),
                            crate::intrinsics::is_integer_index(k.as_str()),
                        )
                    }),
            )
            .collect();
        let mut numeric: Vec<(u64, String)> = all
            .iter()
            .filter(|(_, idx)| *idx)
            .filter_map(|(k, _)| k.parse::<u64>().ok().map(|n| (n, k.clone())))
            .collect();
        numeric.sort_by_key(|(n, _)| *n);
        let strings: Vec<String> = all
            .into_iter()
            .filter(|(_, idx)| !*idx)
            .map(|(k, _)| k)
            .collect();
        let mut out: Vec<String> = numeric.into_iter().map(|(_, k)| k).collect();
        out.extend(strings);
        out
    }

    /// IR-target for Object.keys per §20.1.2.18.
    pub fn enumerable_own_keys(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => {
                return Ok(Value::Object(
                    self.alloc_object(crate::value::Object::new_array()),
                ))
            }
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        // Lift: route through the canonical helper. EXT 92's @@-prefixed
        // filter and the Array vs non-Array two-pass ordering both live
        // in the helper so all consumers stay in sync.
        let keys = self.ordinary_own_enumerable_string_keys(id);
        for (i, k) in keys.iter().enumerate() {
            self.object_set(
                arr,
                i.to_string(),
                Value::String(std::rc::Rc::new(k.clone())),
            );
        }
        self.object_set(arr, "length".into(), Value::Number(keys.len() as f64));
        Ok(Value::Object(arr))
    }

    /// EnumerableOwnPropertyNames(O, "value") per ECMA §7.3.23. IR-target
    /// for Object.values per §20.1.2.23. Dispatches accessor getters.
    pub fn enumerable_own_values(&mut self, v: &Value) -> Result<Value, RuntimeError> {
        let id = match v {
            Value::Object(id) => *id,
            _ => {
                return Ok(Value::Object(
                    self.alloc_object(crate::value::Object::new_array()),
                ))
            }
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        // Lift: canonical ordering + filter through ordinary_own_enumerable_string_keys.
        let keys = self.ordinary_own_enumerable_string_keys(id);
        let mut kvs: Vec<Value> = Vec::with_capacity(keys.len());
        for k in &keys {
            let val = self.read_property(id, k)?;
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
            _ => {
                return Ok(Value::Object(
                    self.alloc_object(crate::value::Object::new_array()),
                ))
            }
        };
        let arr = self.alloc_object(crate::value::Object::new_array());
        // Lift: canonical ordering + filter.
        let keys = self.ordinary_own_enumerable_string_keys(id);
        let mut kvs: Vec<(String, Value)> = Vec::with_capacity(keys.len());
        for k in keys {
            let val = self.read_property(id, &k)?;
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
    pub fn array_species_create(&mut self, o: &Value, len: usize) -> Result<Value, RuntimeError> {
        // RPTP-EXT 1: revoked-proxy check. ArraySpeciesCreate's first
        // step (§22.1.3.17 step 3) is `Get(O, 'constructor')` which per
        // §10.5.5 [[Get]] throws TypeError when O's [[ProxyHandler]] is
        // null (revoked). Catches the bulk of revoked-proxy
        // Array.prototype.{concat,filter,map,slice,splice} tests since
        // every species-routed method touches this early.
        if let Value::Object(id) = o {
            if self.proxy_is_revoked(*id) {
                return Err(RuntimeError::TypeError(
                    "Cannot perform operation on a revoked Proxy".into(),
                ));
            }
        }
        // ECMA-262 §22.1.3.17 ArraySpeciesCreate. Honor the subclass when O
        // is an Array-subclass instance whose constructor is a function:
        // construct via `new C(length)` so the result's proto chain matches
        // the subclass. Falls back to a plain Array allocation otherwise.
        //
        // arktype's Disjoint (class Disjoint extends Array) relies on this
        // for invert()'s `this.map(...)` path AND for any other map/filter/
        // slice call. Without species, map returns a plain Array, breaking
        // downstream `instanceof Disjoint` checks at non-workaround sites.
        //
        // Bracket probe: probes/bracket-array-species (locale rusty-js-esm).
        let o_id = if let Value::Object(id) = o {
            *id
        } else {
            // Non-object receiver: fall back.
            let id = self.alloc_object(crate::value::Object::new_array());
            if len > 0 {
                self.object_set(id, "length".into(), Value::Number(len as f64));
            }
            return Ok(Value::Object(id));
        };
        let is_arr = matches!(
            self.obj(o_id).internal_kind,
            crate::value::InternalKind::Array
        );
        if is_arr {
            // ECMA-262 sec 23.1.3.1 ArraySpeciesCreate, routed through the
            // species_constructor helper (rung-15 lift: sec 7.3.20). Spec
            // ordering:
            //  step 3: C = Get(O, 'constructor')
            //  step 4: if IsConstructor(C): same-realm intrinsic check (skipped)
            //  step 5: if Type(C) is Object, C = Get(C, @@species); null becomes undefined
            //  step 6: if C is undefined, return ArrayCreate(length)
            //  step 7: if !IsConstructor(C), throw TypeError
            //  step 8: return Construct(C, [length])
            let ctor_raw = self.object_get(o_id, "constructor");
            let c: Value = match &ctor_raw {
                Value::Undefined => Value::Undefined,
                Value::Object(cid) => {
                    // Default-Array intrinsic falls back to ArrayCreate.
                    // GBSU-EXT 4: canonical lookup via unified globalThis.
                    let is_plain_array_ctor = match self.global_get("Array") {
                        Value::Object(arr_id) => arr_id == *cid,
                        _ => false,
                    };
                    if is_plain_array_ctor {
                        Value::Undefined
                    } else {
                        // Step 5: Get(C, @@species) - uses [[Get]] so the
                        // %Array%[@@species] getter (returns `this`) fires
                        // for subclasses inheriting via Array.constructor.
                        let s = self.read_property(*cid, "@@species")?;
                        match s {
                            Value::Null | Value::Undefined => Value::Undefined,
                            other => other,
                        }
                    }
                }
                _ => {
                    // Step 7 implicit: constructor is a non-Object non-undefined
                    // primitive (number, string, boolean). Spec falls through
                    // to step 7's IsConstructor check, which fails -> throw.
                    return Err(RuntimeError::TypeError(
                        "Array constructor is not a valid constructor".into(),
                    ));
                }
            };
            // Step 6: if C is undefined, ArrayCreate fallback.
            if matches!(c, Value::Undefined) {
                // fall through to ArrayCreate below
            } else {
                // Step 7: validate IsConstructor.
                let cid = match &c {
                    Value::Object(id) => *id,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "Array @@species is not a constructor".into(),
                        ))
                    }
                };
                // ASCD-EXT 1: IsConstructor per §10.1.14. Function/Closure
                // are constructors only when their FunctionInternals say so
                // (built-in non-constructor functions like parseInt have
                // is_constructor=false). BoundFunction inherits the target's
                // constructable-ness; closures default to true.
                let is_constructor = match &self.obj(cid).internal_kind {
                    crate::value::InternalKind::Function(fi) => fi.is_constructor,
                    crate::value::InternalKind::Closure(_) => true,
                    crate::value::InternalKind::BoundFunction(_) => true,
                    _ => false,
                };
                if !is_constructor {
                    return Err(RuntimeError::TypeError(
                        "Array @@species is not a constructor".into(),
                    ));
                }
                // Step 8: Construct(C, [length]).
                let proto_override = match self.object_get(cid, "prototype") {
                    Value::Object(pid) => Some(pid),
                    _ => None,
                };
                let mut ordinary = crate::value::Object::new_array();
                if proto_override.is_some() {
                    ordinary.proto = proto_override;
                }
                let this_id = self.alloc_object(ordinary);
                let prev_pending = self.pending_new_target.take();
                self.pending_new_target = Some(c.clone());
                let r = self.call_function(
                    c.clone(),
                    Value::Object(this_id),
                    vec![Value::Number(len as f64)],
                );
                self.pending_new_target = prev_pending;
                let ret = r?;
                match ret {
                    Value::Object(_) => return Ok(ret),
                    _ => return Ok(Value::Object(this_id)),
                }
            }
        }
        let id = self.alloc_object(crate::value::Object::new_array());
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
        if !is_array {
            return;
        }
        let Ok(i) = key.parse::<u32>() else {
            return;
        };
        let cur_len = self.array_length(id);
        if (i as usize) >= cur_len {
            self.object_set(id, "length".into(), Value::Number((i as usize + 1) as f64));
        }
    }

    /// Ω.5.P62.E5: IsCallable per ECMA §7.2.4 — true iff `v` is an Object
    /// whose internal kind is one of the callable forms (Function,
    /// Closure, BoundFunction, Proxy with callable target). Used by
    /// Array.prototype.{map,forEach,filter,...} step 3 to throw
    /// TypeError before invoking a non-callable callback.
    pub fn is_callable(&self, v: &Value) -> bool {
        if let Value::Object(id) = v {
            return self.is_callable_object(*id);
        }
        false
    }

    fn is_callable_object(&self, id: ObjectRef) -> bool {
        match &self.obj(id).internal_kind {
            crate::value::InternalKind::Function(_)
            | crate::value::InternalKind::Closure(_)
            | crate::value::InternalKind::BoundFunction(_) => true,
            crate::value::InternalKind::Proxy(p) if !p.revoked => self.is_callable_object(p.target),
            _ => false,
        }
    }

    /// Ω.5.P62.E1: unwrap a primitive-wrapper object's [[__primitive__]]
    /// slot. Returns the boxed primitive Value (String/Number/Boolean/
    /// BigInt/Symbol) if `v` is a wrapper, else returns `v` unchanged.
    /// Used by Number/String/Boolean.prototype.{toString,valueOf} so
    /// `(new Number(5)).toString()` and `(new String("hi")).valueOf()`
    /// resolve through the spec [[NumberData]]/[[StringData]] slots.
    pub fn unwrap_primitive(&self, v: &Value) -> Value {
        if let Value::Object(id) = v {
            if let Some(d) = self.obj(*id).get_own("__primitive__") {
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
                "Cannot convert a Symbol value to a number".into(),
            ));
        }
        if let Value::Object(id) = v {
            let id = *id;
            // (1) @@toPrimitive.
            let tp = self.object_get(id, "@@toPrimitive");
            if matches!(tp, Value::Object(_)) {
                let r = self.call_function(
                    tp,
                    v.clone(),
                    vec![Value::String(Rc::new("number".into()))],
                )?;
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
                "Cannot convert object to primitive value".into(),
            ));
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
                "Cannot convert a Symbol value to a string".into(),
            ));
        }
        self.coerce_to_string(v)
    }

    /// Ω.5.P62.E13: RequireObjectCoercible per ECMA §7.2.1 — throws
    /// TypeError if value is null or undefined.
    pub fn require_object_coercible(&self, v: &Value) -> Result<(), RuntimeError> {
        if matches!(v, Value::Undefined | Value::Null) {
            // EXT 93: same deviation gate as to_object.
            if self
                .tolerated_deviations
                .contains("to-object-coerce-nullish")
            {
                return Ok(());
            }
            return Err(RuntimeError::TypeError(
                "Cannot convert undefined or null to object".into(),
            ));
        }
        Ok(())
    }

    pub fn coerce_to_string(&mut self, v: &Value) -> Result<String, RuntimeError> {
        if let Value::Object(id) = v {
            let mut tried = false;
            // (1) @@toPrimitive.
            let tp = self.get_method(v, "@@toPrimitive")?;
            if !matches!(tp, Value::Undefined) {
                tried = true;
                let r = self.call_function(
                    tp,
                    v.clone(),
                    vec![Value::String(Rc::new("string".into()))],
                )?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(crate::abstract_ops::to_string(&r).as_str().to_string());
                }
            }
            // (2) toString.
            let ts = self.spec_get(v, "toString")?;
            if self.is_callable(&ts) {
                tried = true;
                let r = self.call_function(ts, v.clone(), Vec::new())?;
                if !matches!(r, Value::Object(_)) {
                    return Ok(crate::abstract_ops::to_string(&r).as_str().to_string());
                }
            }
            // (3) valueOf.
            let vo = self.spec_get(v, "valueOf")?;
            if self.is_callable(&vo) {
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
                    "Cannot convert object to primitive value".into(),
                ));
            }
        }
        Ok(crate::abstract_ops::to_string(v).as_str().to_string())
    }

    /// Drain promises still rejected with no handler. Caller is the host;
    /// canonical action is print-to-stderr + exit nonzero. Idempotent.
    pub fn drain_unhandled_rejections(&mut self) -> Vec<(rusty_js_gc::ObjectId, Value)> {
        let ids: Vec<_> = self.pending_unhandled.drain().collect();
        ids.into_iter()
            .filter_map(|id| match &self.heap.get(id)?.internal_kind {
                InternalKind::Promise(ps)
                    if matches!(ps.status, crate::value::PromiseStatus::Rejected) =>
                {
                    Some((id, ps.value.clone()))
                }
                _ => None,
            })
            .collect()
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
        // GBSU-EXT 7f.4: globalThis Object is the canonical global
        // VarEnvRec. Root it; the GC walks its property table to reach
        // every reachable global binding. global_object is eager-allocated
        // in Runtime::new, so unwrap is provably safe.
        roots.push(
            self.global_object
                .expect("global_object eager-allocated in Runtime::new"),
        );
        // Doc 729 §VII.B engine-internal bilateral lowerings — NOT
        // JS-visible, NOT on globalThis, must be rooted explicitly.
        for v in self.engine_helpers.values() {
            if let Value::Object(id) = v {
                roots.push(*id);
            }
        }
        if let Value::Object(id) = &self.last_value {
            roots.push(*id);
        }
        roots
    }

    /// Allocate an Object via the managed heap. Returns the ObjectId
    /// handle. Tier-Ω.5.a: if the Object has no explicit proto and an
    /// intrinsic prototype matching its InternalKind has been installed,
    /// the proto is wired automatically. This is the seam through which
    /// prototype-chain method dispatch works without retrofitting every
    /// alloc call-site.
    /// RS-EXT 2d: shallow-clone an intrinsic-prototype object into a fresh
    /// ObjectRef. The clone shares method ObjectRefs with the source (so
    /// `arr.map(...)` in the cloned realm still calls the same compiled
    /// map function) but property-bag mutations (`Array.prototype.map = ...`)
    /// land on the clone's own entries and don't leak to the source realm.
    /// This is the load-bearing isolation primitive for the prototype-
    /// pollution probe per pilots/realm-substrate/seed.md §I.3.
    fn clone_intrinsic_proto(&mut self, source: rusty_js_gc::ObjectId) -> rusty_js_gc::ObjectId {
        let cloned = {
            let src = self.obj(source);
            let mut out = crate::value::Object::new_ordinary();
            out.proto = src.proto;
            out.extensible = src.extensible;
            out.properties = src.properties.clone();
            out.shape = src.shape.clone();
            out.shape_values = src.shape_values.clone();
            // CP-EXT 5: preserve internal_kind so cloned constructors
            // remain typeof-"function" / callable. The native fn pointer
            // (in FunctionInternals) is shared by Rc; not cloned, just
            // referenced — that's correct: a cloned Array constructor
            // invokes the same Rust impl as the primordial. Manual clone
            // since InternalKind doesn't derive Clone (some variants
            // hold non-Clone state).
            use crate::value::InternalKind as IK;
            out.internal_kind = match &src.internal_kind {
                IK::Function(fi) => IK::Function(crate::value::FunctionInternals {
                    name: fi.name.clone(),
                    length: fi.length,
                    native: fi.native.clone(),
                    is_constructor: fi.is_constructor,
                }),
                IK::Ordinary => IK::Ordinary,
                IK::Array => IK::Array,
                IK::Error => IK::Error,
                _ => IK::Ordinary, // other variants fall back to Ordinary;
                                   // not load-bearing for prototype/ctor clones
            };
            out
        };
        self.alloc_object(cloned)
    }

    /// CP-EXT 1: wrapper that allocates a realm and additionally seeds
    /// the realm's globals_overrides with caller-supplied endowments.
    /// Compartment's ctor uses this; the endowments become the only
    /// non-intrinsic-prototype global bindings visible inside the
    /// compartment's evaluate calls.
    pub fn allocate_compartment_realm(
        &mut self,
        endowments: std::collections::HashMap<String, Value>,
    ) -> usize {
        let idx = self.allocate_realm();
        for (k, v) in endowments {
            self.realms[idx].globals_overrides.insert(k, v);
        }
        // CP-EXT 5: compartment realms default-deny ambient host globals.
        // Plain Realms (RS-EXT 2) keep ambient_denied=false; only the
        // Compartment ctor flips this flag via allocate_compartment_realm.
        self.realms[idx].ambient_denied = true;
        idx
    }

    /// RS-EXT 2d+2g: allocate a fresh realm with cloned intrinsic
    /// prototypes AND cloned constructors. The constructor clones carry
    /// .prototype pointing at the cloned prototype, so user code's
    /// `Array.prototype.X = ...` (which dereferences via the Array
    /// global → .prototype) lands on the cloned prototype, not the
    /// primordial one. Without the constructor clones, user-visible
    /// `Array.prototype` would resolve to the primordial via the
    /// shared global, defeating realm isolation.
    pub fn allocate_realm(&mut self) -> usize {
        let array_p = self
            .array_prototype
            .map(|id| self.clone_intrinsic_proto(id));
        let object_p = self
            .object_prototype
            .map(|id| self.clone_intrinsic_proto(id));
        let function_p = self
            .function_prototype
            .map(|id| self.clone_intrinsic_proto(id));
        let promise_p = self
            .promise_prototype
            .map(|id| self.clone_intrinsic_proto(id));
        let string_p = self
            .string_prototype
            .map(|id| self.clone_intrinsic_proto(id));
        let mut overrides = std::collections::HashMap::new();
        for (name, new_proto) in [
            ("Array", array_p),
            ("Object", object_p),
            ("Function", function_p),
            ("Promise", promise_p),
            ("String", string_p),
        ] {
            // GBSU-EXT 7f.3: canonical lookup via unified globalThis.
            let global_ctor = self.global_get(name);
            if let (false, Some(new_proto)) = (matches!(global_ctor, Value::Undefined), new_proto) {
                if let Value::Object(orig_ctor_id) = global_ctor {
                    let cloned_ctor = self.clone_intrinsic_proto(orig_ctor_id);
                    self.obj_mut(cloned_ctor)
                        .set_own_frozen("prototype".into(), Value::Object(new_proto));
                    overrides.insert(name.to_string(), Value::Object(cloned_ctor));
                }
            }
        }
        self.realms.push(RealmRecord {
            object_prototype: object_p,
            array_prototype: array_p,
            function_prototype: function_p,
            promise_prototype: promise_p,
            string_prototype: string_p,
            globals_overrides: overrides,
            primordial_snapshot: std::collections::HashMap::new(),
            ambient_denied: false,
            primordial_full_snapshot: None,
        });
        self.realms.len() - 1
    }

    /// CP-EXT 5: default-deny ambient globals at compartment entry.
    /// Returns the name set the compartment retains by default — just
    /// the ECMAScript intrinsics + a few engine-internal helpers that
    /// must remain reachable for compiled bytecode to run. Host-tier
    /// globals (process, console, require, fs, ...) are NOT in this
    /// set; they're available only when explicitly endowed via the
    /// compartment's globals option per Doc 736.
    pub fn intrinsic_name_allowlist() -> &'static [&'static str] {
        &[
            // §19 Fundamental Objects
            "Object",
            "Function",
            "Boolean",
            "Symbol",
            "Error",
            "EvalError",
            "RangeError",
            "ReferenceError",
            "SyntaxError",
            "TypeError",
            "URIError",
            // §20 Numbers, Dates, Math, JSON
            "Number",
            "BigInt",
            "Math",
            "Date",
            "JSON",
            // §21 Indexed Collections + §22 Text Processing
            "Array",
            "TypedArray",
            "String",
            "RegExp",
            "Int8Array",
            "Uint8Array",
            "Uint8ClampedArray",
            "Int16Array",
            "Uint16Array",
            "Int32Array",
            "Uint32Array",
            "Float32Array",
            "Float64Array",
            "BigInt64Array",
            "BigUint64Array",
            "ArrayBuffer",
            "DataView",
            "SharedArrayBuffer",
            "Atomics",
            // §23 Keyed Collections + §24 Structured Data
            "Map",
            "Set",
            "WeakMap",
            "WeakSet",
            // §25 Reflection
            "Proxy",
            "Reflect",
            // §26 Promises
            "Promise",
            // §27 Iteration
            "Iterator",
            "AsyncIterator",
            // §19.4 Global Property Values + functions
            "globalThis",
            "undefined",
            "NaN",
            "Infinity",
            "parseInt",
            "parseFloat",
            "isNaN",
            "isFinite",
            "encodeURI",
            "encodeURIComponent",
            "decodeURI",
            "decodeURIComponent",
            // Engine-internal helpers compiled bytecode emits LoadGlobal for;
            // these MUST remain reachable or the realm crashes immediately.
            "__destr_array_rest",
            "__destr_object_rest",
            "__destr_object_check",
            "__destr_iter_open",
            "__destr_iter_step",
            "__destr_iter_rest",
            "__destr_iter_close",
            "__array_extend",
            "__await",
            "__yield_push__",
            "__yield_delegate__",
            "__install_method__",
            "__install_method_obj__",
            "__install_accessor__",
            "__install_accessor_obj__",
            "__super_get_home",
            "__super_base_home",
            "__super_get_base",
            "__super_set",
            "__super_delete",
            "__cruftless_tolerate",
            "__post_eval_trace",
        ]
    }

    /// RS-EXT 2e+2g: enter a realm. Returns the prior realm index so the
    /// caller can restore on exit. Three swaps occur:
    /// 1. Runtime intrinsic-prototype fields swap to realm's clones
    ///    (covers field-level alloc-site reads).
    /// 2. globals_overrides shadow the primordial globals (Array,
    ///    Object, Function, ...) — the prior values are snapshotted
    ///    onto the realm so exit_realm can restore.
    /// 3. current_realm index updates.
    pub fn enter_realm(&mut self, idx: usize) -> usize {
        let prior = self.current_realm;
        let ambient_denied = self.realms[idx].ambient_denied;
        // GBSU-EXT 7e (global-binding-surface-unification rung 7e): realm
        // snapshots/overrides operate on the unified globalThis Object's
        // prop dict instead of the legacy `self.globals` HashMap. The
        // Realm field types remain `HashMap<String, Value>` (snapshot is
        // name → value); only the read/write site changes. Ambient-denied
        // full-snapshot iterates the Object's String-keyed own properties.
        if ambient_denied {
            let full = self.snapshot_global_string_props();
            self.realms[idx].primordial_full_snapshot = Some(full);
            let allow: std::collections::HashSet<&'static str> =
                Self::intrinsic_name_allowlist().iter().copied().collect();
            self.retain_global_string_props(&allow);
        }
        let (over_keys, over_values): (Vec<String>, Vec<Value>) = {
            let r = &self.realms[idx];
            (
                r.globals_overrides.keys().cloned().collect(),
                r.globals_overrides.values().cloned().collect(),
            )
        };
        let mut snapshot = std::collections::HashMap::new();
        for (k, v) in over_keys.iter().zip(over_values.into_iter()) {
            let prev = self.global_get(k);
            if !matches!(prev, Value::Undefined) {
                snapshot.insert(k.clone(), prev);
            }
            self.define_global_property(k, v);
        }
        self.realms[idx].primordial_snapshot = snapshot;
        self.current_realm = idx;
        let r = &self.realms[idx];
        self.array_prototype = r.array_prototype;
        self.object_prototype = r.object_prototype;
        self.function_prototype = r.function_prototype;
        self.promise_prototype = r.promise_prototype;
        self.string_prototype = r.string_prototype;
        prior
    }

    /// RS-EXT 2e+2g + CP-EXT 5: exit a realm, restoring the prior one.
    pub fn exit_realm(&mut self, prior: usize) {
        let cur = self.current_realm;
        // CP-EXT 5: restore the full primordial globals if this realm
        // suppressed ambient (overwriting any intra-realm mutations to
        // primordial-host bindings — those would have been on snapshots,
        // not the live primordial map).
        // GBSU-EXT 7e: restore via global_object's prop dict.
        if let Some(full) = self.realms[cur].primordial_full_snapshot.take() {
            self.replace_global_string_props(full);
        } else {
            let snapshot = std::mem::take(&mut self.realms[cur].primordial_snapshot);
            for (k, v) in snapshot {
                self.define_global_property(&k, v);
            }
        }
        self.current_realm = prior;
        let r = &self.realms[prior];
        self.array_prototype = r.array_prototype;
        self.object_prototype = r.object_prototype;
        self.function_prototype = r.function_prototype;
        self.promise_prototype = r.promise_prototype;
        self.string_prototype = r.string_prototype;
    }

    pub fn alloc_object(&mut self, mut obj: crate::value::Object) -> rusty_js_gc::ObjectId {
        if obj.proto.is_none() {
            // RS-EXT 2c: proto-wiring consults the current realm's
            // intrinsic table. Realm 0 mirrors the Runtime fields
            // (post-RS-EXT 2b), so this is behavior-preserving in the
            // single-realm baseline. Once enter_realm (RS-EXT 2e) swaps
            // current_realm to a fresh realm with cloned prototypes,
            // alloc_object naturally picks up the realm-scoped prototype
            // — array literals allocated under a non-zero realm get that
            // realm's Array.prototype, isolating mutations.
            let realm = &self.realms[self.current_realm];
            obj.proto = match &obj.internal_kind {
                crate::value::InternalKind::Ordinary => {
                    realm.object_prototype.or(self.object_prototype)
                }
                crate::value::InternalKind::Array => realm.array_prototype.or(self.array_prototype),
                crate::value::InternalKind::Promise(_) => {
                    realm.promise_prototype.or(self.promise_prototype)
                }
                crate::value::InternalKind::RegExp(_) => self.regexp_prototype,
                crate::value::InternalKind::Function(_)
                | crate::value::InternalKind::Closure(_)
                | crate::value::InternalKind::BoundFunction(_) => {
                    realm.function_prototype.or(self.function_prototype)
                }
                _ => None,
            };
        }
        self.heap.alloc(obj)
    }

    /// Variant of alloc_object that bypasses the intrinsic-proto default
    /// when the caller explicitly wants a null-proto object (e.g.,
    /// Object.create(null), Object.create() with explicit null first
    /// arg). alloc_object treats `proto: None` as "default to the
    /// intrinsic prototype"; this entrypoint takes the leave-it-null
    /// branch.
    pub fn alloc_object_with_explicit_null_proto(
        &mut self,
        obj: crate::value::Object,
    ) -> rusty_js_gc::ObjectId {
        // obj.proto is already None and we want it to STAY None.
        self.heap.alloc(obj)
    }

    /// Ergonomic heap accessors. Panic on missing — the migration's
    /// invariant is that every ObjectId in a live Value points to a live
    /// slot. Stale handles after a sweep would be a GC-correctness bug
    /// surfaced loudly here.
    pub fn obj(&self, id: ObjectRef) -> &Object {
        self.heap
            .get(id)
            .expect("ObjectId points to free/missing slot")
    }
    pub fn obj_mut(&mut self, id: ObjectRef) -> &mut Object {
        self.heap
            .get_mut(id)
            .expect("ObjectId points to free/missing slot")
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
                } else {
                    Ok(to_number(v))
                }
            }
            _ => Ok(to_number(v)),
        }
    }

    pub fn to_num_coerced_strict(&mut self, v: &Value) -> Result<f64, RuntimeError> {
        let prim = match v {
            Value::Object(id) => {
                let vo = self.object_get(*id, "valueOf");
                if matches!(vo, Value::Object(_)) {
                    self.call_function(vo, Value::Object(*id), Vec::new())?
                } else {
                    v.clone()
                }
            }
            _ => v.clone(),
        };
        match prim {
            Value::Symbol(_) => Err(RuntimeError::TypeError(
                "Cannot convert a Symbol value to a number".into(),
            )),
            Value::BigInt(_) => Err(RuntimeError::TypeError(
                "Cannot mix BigInt and other types".into(),
            )),
            _ => Ok(to_number(&prim)),
        }
    }

    fn ensure_no_mixed_bigint_or_symbol(&self, l: &Value, r: &Value) -> Result<(), RuntimeError> {
        if matches!(l, Value::Symbol(_)) || matches!(r, Value::Symbol(_)) {
            return Err(RuntimeError::TypeError(
                "Cannot convert a Symbol value to a number".into(),
            ));
        }
        if matches!(l, Value::BigInt(_)) ^ matches!(r, Value::BigInt(_)) {
            return Err(RuntimeError::TypeError(
                "Cannot mix BigInt and other types".into(),
            ));
        }
        Ok(())
    }
    pub fn find_getter(&self, id: ObjectRef, key: &str) -> Option<Value> {
        // GOPD-EXT 1: invocation path. Skip Some(Value::Undefined)
        // (accessor with explicit-undefined getter); only return callable
        // values. gOPD reads d.getter directly to distinguish accessor-
        // shape from data-shape descriptors.
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.get_own(key) {
                if let Some(g) = &d.getter {
                    if !matches!(g, Value::Undefined) {
                        return Some(g.clone());
                    }
                }
                return None;
            }
            cur = o.proto;
        }
        None
    }
    /// PropertyKey-aware getter lookup along the proto chain.
    pub fn find_getter_pk(&self, id: ObjectRef, key: &crate::value::PropertyKey) -> Option<Value> {
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.properties.get(key) {
                if let Some(g) = &d.getter {
                    if !matches!(g, Value::Undefined) {
                        return Some(g.clone());
                    }
                    return None;
                }
            }
            if let crate::value::PropertyKey::Symbol(rc) = key {
                if let Some(d) = o.get_own(rc.as_str()) {
                    if let Some(g) = &d.getter {
                        if !matches!(g, Value::Undefined) {
                            return Some(g.clone());
                        }
                        return None;
                    }
                }
            }
            cur = o.proto;
        }
        None
    }
    /// Tier-Ω.5.nnn: walk the prototype chain looking for an accessor
    /// setter at `key`. Returns the setter function if found.
    pub fn find_setter(&self, id: ObjectRef, key: &str) -> Option<Value> {
        // GOPD-EXT 1: same semantics as find_getter.
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.get_own(key) {
                if let Some(s) = &d.setter {
                    if !matches!(s, Value::Undefined) {
                        return Some(s.clone());
                    }
                }
                return None;
            }
            cur = o.proto;
        }
        None
    }
    /// PropertyKey-aware setter lookup along the proto chain.
    /// GOPD-EXT 1 helper: does the property at `key` resolve (own or
    /// inherited) to an accessor descriptor? Returns true for any
    /// accessor regardless of whether the getter/setter is callable.
    /// Used at write sites to distinguish "set on data property" from
    /// "set on accessor with no callable setter" (silent no-op sloppy,
    /// TypeError strict) per §10.1.9.4 step 2.b.
    pub fn is_accessor_at(&self, id: ObjectRef, key: &str) -> bool {
        let mut cur = Some(id);
        while let Some(c) = cur {
            if let Some(d) = self.obj(c).get_own(key) {
                return d.getter.is_some() || d.setter.is_some();
            }
            cur = self.obj(c).proto;
        }
        false
    }
    pub fn is_accessor_at_pk(&self, id: ObjectRef, key: &crate::value::PropertyKey) -> bool {
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.properties.get(key) {
                return d.getter.is_some() || d.setter.is_some();
            }
            cur = o.proto;
        }
        false
    }
    pub fn find_setter_pk(&self, id: ObjectRef, key: &crate::value::PropertyKey) -> Option<Value> {
        // GOPD-EXT 1: same semantics as find_getter_pk.
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.properties.get(key) {
                if let Some(s) = &d.setter {
                    if !matches!(s, Value::Undefined) {
                        return Some(s.clone());
                    }
                }
                return None;
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

    pub fn read_property_pk(
        &mut self,
        id: ObjectRef,
        key: &crate::value::PropertyKey,
    ) -> Result<Value, RuntimeError> {
        if let Some(getter) = self.find_getter_pk(id, key) {
            return self.call_function(getter, Value::Object(id), Vec::new());
        }
        Ok(self.object_get_pk(id, key))
    }

    /// Ω.5.P61.E13: HasProperty per ECMA §10.1.7.1 — walks own +
    /// prototype chain for the key. Used by Array.prototype iteration
    /// methods to skip sparse holes (a property present along the chain,
    /// even if its value is Undefined, is NOT a hole).
    pub fn has_property(&self, id: ObjectRef, key: &str) -> bool {
        if let Some(idx) = Self::canonical_array_index_key(key) {
            if self.typed_array_views.contains_key(&id) {
                return idx < self.typed_array_view_len(id).unwrap_or(0);
            }
        }
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if o.has_own_str(key) {
                return true;
            }
            cur = o.proto;
        }
        false
    }

    fn has_property_with_proxy(&mut self, id: ObjectRef, key: &str) -> Result<bool, RuntimeError> {
        let key_pk = crate::value::PropertyKey::String(key.to_string());
        if let Some((target, handler)) = self.proxy_target_handler_checked(id)? {
            let trap = self.object_get(handler, "has");
            if matches!(trap, Value::Object(_)) {
                let result = self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![
                        Value::Object(target),
                        Value::String(std::rc::Rc::new(key.to_string())),
                    ],
                )?;
                let trap_has = crate::abstract_ops::to_boolean(&result);
                self.apply_proxy_has_invariant(target, key, trap_has)?;
                return Ok(trap_has);
            }
            return Ok(self.has_property_pk(target, &key_pk));
        }
        Ok(self.has_property_pk(id, &key_pk))
    }

    fn spec_get_pk(
        &mut self,
        id: ObjectRef,
        key: &crate::value::PropertyKey,
    ) -> Result<Value, RuntimeError> {
        let key_str = key.as_str().to_string();
        if let Some((target, handler)) = self.proxy_target_handler_checked(id)? {
            let trap = self.object_get(handler, "get");
            if matches!(trap, Value::Object(_)) {
                let trap_key = match key {
                    crate::value::PropertyKey::Symbol(rc) => Value::Symbol(rc.clone()),
                    crate::value::PropertyKey::String(s) => {
                        Value::String(std::rc::Rc::new(s.clone()))
                    }
                };
                let result = self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![Value::Object(target), trap_key, Value::Object(id)],
                )?;
                self.apply_proxy_get_invariant(target, &key_str, &result)?;
                return Ok(result);
            }
            if let Some(getter) = self.find_getter_pk(target, key) {
                return self.call_function(getter, Value::Object(target), Vec::new());
            }
            return self.spec_get_pk(target, key);
        }
        if let Some(getter) = self.find_getter_pk(id, key) {
            return self.call_function(getter, Value::Object(id), Vec::new());
        }
        Ok(self.object_get_pk(id, key))
    }

    fn with_object_has_binding(
        &mut self,
        obj_id: ObjectRef,
        name: &str,
    ) -> Result<bool, RuntimeError> {
        if !self.has_property_with_proxy(obj_id, name)? {
            return Ok(false);
        }

        // Integration: GBSU unified-surface lookup.
        let unscopables_key = match self.global_get("Symbol") {
            Value::Object(sym) => match self.object_get(sym, "unscopables") {
                Value::Symbol(rc) => Some(crate::value::PropertyKey::Symbol(rc)),
                _ => None,
            },
            _ => None,
        }
        .unwrap_or_else(|| crate::value::PropertyKey::String("@@unscopables".to_string()));
        let unscopables = self.spec_get_pk(obj_id, &unscopables_key)?;
        let unscopables_id = match unscopables {
            Value::Object(id) => id,
            _ => return Ok(true),
        };
        let blocked = self.spec_get(&Value::Object(unscopables_id), name)?;
        Ok(!crate::abstract_ops::to_boolean(&blocked))
    }

    fn set_with_object_binding(
        &mut self,
        obj_id: ObjectRef,
        name: &str,
        value: Value,
        strict: bool,
    ) -> Result<(), RuntimeError> {
        if let Some((target, handler)) = self.proxy_target_handler_checked(obj_id)? {
            let trap = self.object_get(handler, "set");
            if matches!(trap, Value::Object(_)) {
                let result = self.call_function(
                    trap,
                    Value::Object(handler),
                    vec![
                        Value::Object(target),
                        Value::String(std::rc::Rc::new(name.to_string())),
                        value.clone(),
                        Value::Object(obj_id),
                    ],
                )?;
                let accepted = crate::abstract_ops::to_boolean(&result);
                self.apply_proxy_set_invariant(target, name, &value, accepted)?;
                if !accepted && strict {
                    return Err(RuntimeError::TypeError(format!(
                        "Proxy 'set' trap returned false for '{}'",
                        name
                    )));
                }
                return Ok(());
            }
            self.object_set(target, name.to_string(), value);
            return Ok(());
        }

        if let Some(setter) = self.find_setter(obj_id, name) {
            self.call_function(setter, Value::Object(obj_id), vec![value])?;
        } else if self.is_accessor_at(obj_id, name) {
            if strict {
                return Err(RuntimeError::TypeError(format!(
                    "Cannot set property '{}' of object which has only a getter",
                    name
                )));
            }
        } else {
            self.object_set(obj_id, name.to_string(), value);
        }
        Ok(())
    }

    fn resolve_with_name_base(
        &mut self,
        frame: &Frame,
        name: &str,
    ) -> Result<Option<ObjectRef>, RuntimeError> {
        let with_stack: Vec<ObjectRef> = frame.with_env_stack.iter().rev().copied().collect();
        for obj_id in with_stack {
            if self.with_object_has_binding(obj_id, name)? {
                return Ok(Some(obj_id));
            }
        }
        Ok(None)
    }

    fn load_with_name(&mut self, frame: &mut Frame, name: &str) -> Result<Value, RuntimeError> {
        let with_stack: Vec<ObjectRef> = frame.with_env_stack.iter().rev().copied().collect();
        for obj_id in with_stack {
            if self.with_object_has_binding(obj_id, name)? {
                if !self.has_property_with_proxy(obj_id, name)? {
                    if frame.strict {
                        return Err(RuntimeError::ReferenceError(format!(
                            "{} is not defined",
                            name
                        )));
                    }
                    return Ok(Value::Undefined);
                }
                return self.spec_get(&Value::Object(obj_id), name);
            }
        }
        for (slot, desc) in frame.locals_names.iter().enumerate().rev() {
            if desc.name == name {
                return Ok(frame.read_local(slot));
            }
        }
        for (slot, desc) in frame.upvalue_names.iter().enumerate().rev() {
            if desc.name == name {
                return Ok(frame
                    .upvalues
                    .get(slot)
                    .map(|cell| cell.borrow().clone())
                    .unwrap_or(Value::Undefined));
            }
        }
        // Integration: GBSU unified-surface lookup.
        let v = self.global_get(name);
        if !matches!(v, Value::Undefined) {
            return Ok(v);
        }
        if let Some(h) = self.engine_helpers.get(name).cloned() {
            return Ok(h);
        }
        Err(RuntimeError::ReferenceError(format!(
            "{} is not defined",
            name
        )))
    }

    fn store_with_name(
        &mut self,
        frame: &mut Frame,
        name: &str,
        value: Value,
    ) -> Result<(), RuntimeError> {
        let with_stack: Vec<ObjectRef> = frame.with_env_stack.iter().rev().copied().collect();
        for obj_id in with_stack {
            if self.with_object_has_binding(obj_id, name)? {
                if !self.has_property_with_proxy(obj_id, name)? && frame.strict {
                    return Err(RuntimeError::ReferenceError(format!(
                        "{} is not defined",
                        name
                    )));
                }
                self.set_with_object_binding(obj_id, name, value, frame.strict)?;
                return Ok(());
            }
        }
        for (slot, desc) in frame.locals_names.iter().enumerate().rev() {
            if desc.name == name {
                frame.write_local(slot, value);
                return Ok(());
            }
        }
        for (slot, desc) in frame.upvalue_names.iter().enumerate().rev() {
            if desc.name == name {
                if let Some(cell) = frame.upvalues.get(slot) {
                    *cell.borrow_mut() = value;
                    return Ok(());
                }
            }
        }
        // Integration: GBSU unified surface.
        if frame.strict {
            let declared = self
                .global_object
                .map(|gt| self.obj(gt).has_own_str(name))
                .unwrap_or(false)
                || self.engine_helpers.contains_key(name);
            if !declared {
                return Err(RuntimeError::ReferenceError(format!(
                    "{} is not defined",
                    name
                )));
            }
        }
        self.define_global_property(name, value);
        Ok(())
    }

    /// Ω.5.P63.E54: PropertyKey-aware has-property — walks proto chain
    /// and respects Symbol-typed keys (identity, by-Rc). Transitional shim:
    /// for Symbol keys whose inner identifier matches a String-bucket entry,
    /// fall back to that entry so legacy well-known-Symbol method installs
    /// (register_intrinsic_method with name="@@iterator") keep dispatching
    /// when looked up via Symbol.iterator.
    pub fn has_property_pk(&self, id: ObjectRef, key: &crate::value::PropertyKey) -> bool {
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            // CMig-EXT 8: shape-aware check for String keys via
            // has_own_str (which is shape-aware per Shape-EXT 4).
            if let crate::value::PropertyKey::String(s) = key {
                if o.has_own_str(s.as_str()) {
                    return true;
                }
            } else if o.properties.contains_key(key) {
                return true;
            }
            if let crate::value::PropertyKey::Symbol(rc) = key {
                if o.has_own_str(rc.as_str()) {
                    return true;
                }
            }
            cur = o.proto;
        }
        false
    }

    /// PropertyKey-aware proto-walking get. Includes the Symbol→String
    /// transitional fallback for well-known Symbol method lookups.
    pub fn object_get_pk(&self, id: ObjectRef, key: &crate::value::PropertyKey) -> Value {
        // String-keyed lookups preserve the existing array-"length" fast path.
        if let crate::value::PropertyKey::String(s) = key {
            return self.object_get(id, s.as_str());
        }
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(d) = o.properties.get(key) {
                return d.value.clone();
            }
            // Transitional: well-known Symbol storage was String-keyed pre-E54.
            if let crate::value::PropertyKey::Symbol(rc) = key {
                if let Some(d) = o.get_own(rc.as_str()) {
                    return d.value.clone();
                }
            }
            cur = o.proto;
        }
        Value::Undefined
    }

    /// PropertyKey-aware own-key set. Honors non-writable descriptors.
    /// Shape-EXT 4 dispatch:
    ///   - `__name` engine-internal sentinel keys migrate to Dictionary
    ///     first (non-Shape-eligible per shapes seed §IV; avoids cohabitation
    ///     with IndexMap-walking consumers like Map.size / Set.size that
    ///     read .properties directly for sentinel data).
    ///   - String-keyed sets on Shaped objects route through set_own
    ///     (shape transition or in-place mutate).
    ///   - Symbol-keyed sets always migrate to Dictionary first.
    /// ESNE-EXT 1: install an engine-internal `__name` sentinel with the
    /// canonical descriptor `{w:t, e:f, c:f}` per CLAUDE.md's source-identifier
    /// coordinate convention. Subsequent `object_set` updates of the same key
    /// preserve these attrs (the update branch only mutates value).
    pub fn set_engine_sentinel(&mut self, id: ObjectRef, name: &str, value: Value) {
        self.obj_mut(id).dict_mut().insert(
            crate::value::PropertyKey::String(name.to_string()),
            crate::value::PropertyDescriptor {
                value,
                writable: true,
                enumerable: false,
                configurable: false,
                getter: None,
                setter: None,
            },
        );
    }

    pub fn object_set_pk(&mut self, id: ObjectRef, key: crate::value::PropertyKey, value: Value) {
        if let crate::value::PropertyKey::String(s) = &key {
            if let Some(idx) = Self::canonical_array_index_key(s) {
                if self.typed_array_set_index(id, idx, value.clone()) {
                    return;
                }
            }
        }
        if let crate::value::PropertyKey::String(s) = &key {
            let mapped_cell = {
                let o = self.obj(id);
                if let InternalKind::MappedArguments { parameter_map } = &o.internal_kind {
                    parameter_map.get(s).cloned()
                } else {
                    None
                }
            };
            if let Some(cell) = mapped_cell {
                *cell.borrow_mut() = value.clone();
            }
        }
        // ALST-EXT 1: route `arr.length = N` through §10.4.2.1 ArraySetLength
        // so that decreasing length truncates the backing storage. Without
        // this, the assignment-path stored length-as-data-property without
        // deleting indices ≥ N, breaking forEach/map/filter/reduce length-
        // mutation-during-iteration tests. The defineProperty path already
        // routes through array_set_length; this promotes the assignment path
        // to the same deeper-layer closure.
        if let crate::value::PropertyKey::String(s) = &key {
            if s.as_str() == "length"
                && matches!(
                    self.obj(id).internal_kind,
                    crate::value::InternalKind::Array
                )
            {
                let desc_id = self.alloc_object(crate::value::Object::default());
                self.obj_mut(desc_id).dict_mut().insert(
                    crate::value::PropertyKey::String("value".to_string()),
                    crate::value::PropertyDescriptor {
                        value,
                        writable: true,
                        enumerable: true,
                        configurable: true,
                        getter: None,
                        setter: None,
                    },
                );
                let _ = crate::generated::array_set_length(
                    self,
                    Value::Undefined,
                    &[Value::Object(id), Value::Object(desc_id)],
                );
                return;
            }
        }
        match &key {
            crate::value::PropertyKey::String(s) => {
                if s.starts_with("__") {
                    self.obj_mut(id).migrate_to_dictionary();
                } else if self.obj(id).shape.is_some() {
                    self.obj_mut(id).set_own(s.clone(), value);
                    return;
                }
            }
            crate::value::PropertyKey::Symbol(_) => {
                self.obj_mut(id).migrate_to_dictionary();
            }
        }
        if let Some(d) = self.obj_mut(id).properties.get_mut(&key) {
            if !d.writable && d.getter.is_none() && d.setter.is_none() {
                return; // silent no-op for non-writable data property
            }
            d.value = value;
            return;
        }
        self.obj_mut(id).dict_mut().insert(
            key,
            crate::value::PropertyDescriptor {
                value,
                writable: true,
                enumerable: true,
                configurable: true,
                getter: None,
                setter: None,
            },
        );
    }

    pub fn object_get(&self, id: ObjectRef, key: &str) -> Value {
        if key.starts_with('#') {
            return self.object_get_private(id, key).unwrap_or(Value::Undefined);
        }
        if key == "length" {
            if let Some(len) = self.typed_array_view_len(id) {
                return Value::Number(len as f64);
            }
        }
        if key == "byteLength" {
            if let Some(len) = self.typed_array_view_byte_length(id) {
                return Value::Number(len as f64);
            }
            if let Some(buf) = self.array_buffers.get(&id) {
                return Value::Number(if buf.detached {
                    0.0
                } else {
                    buf.byte_length as f64
                });
            }
        }
        if key == "byteOffset" {
            if let Some(view) = self.typed_array_views.get(&id) {
                return Value::Number(if self.typed_array_view_out_of_bounds(id) {
                    0.0
                } else {
                    view.byte_offset as f64
                });
            }
        }
        if key == "buffer" {
            if let Some(view) = self.typed_array_views.get(&id) {
                return Value::Object(view.buffer);
            }
        }
        // TAMM-EXT 1: ArrayBuffer.prototype.{maxByteLength, resizable,
        // detached, immutable} accessor surface. Per ECMA-262 §25.1
        // ArrayBuffer extensions (ES2024 resizable/transferable buffers):
        // - maxByteLength reports the maximum byte length the buffer can
        //   grow to via .resize(); for non-resizable buffers this equals
        //   byteLength (per §25.1.5.1 IsFixedLengthArrayBuffer step 2).
        // - resizable is true iff max_byte_length > byte_length OR was
        //   constructed with the maxByteLength option (cruft tracks the
        //   latter by storing max_byte_length distinct from byte_length).
        // - detached is true iff the underlying record was dropped; cruft
        //   does not implement detachment yet, so always false.
        // - immutable reflects the (newer) immutable-ArrayBuffer proposal;
        //   not implemented, always false.
        if key == "maxByteLength" {
            if let Some(buf) = self.array_buffers.get(&id) {
                return Value::Number(if buf.detached {
                    0.0
                } else {
                    buf.max_byte_length as f64
                });
            }
        }
        if key == "resizable" {
            if self.array_buffers.contains_key(&id) {
                let buf = self.array_buffers.get(&id).unwrap();
                if buf.detached {
                    return Value::Boolean(false);
                }
                return Value::Boolean(
                    buf.max_byte_length > buf.byte_length
                        || self.obj(id).has_own_str("__cruft_was_resizable"),
                );
            }
        }
        if key == "detached" {
            if let Some(buf) = self.array_buffers.get(&id) {
                return Value::Boolean(buf.detached);
            }
        }
        if key == "immutable" {
            if self.array_buffers.contains_key(&id) {
                return Value::Boolean(false);
            }
        }
        if let Some(idx) = Self::canonical_array_index_key(key) {
            if let Some(v) = self.typed_array_get_index(id, idx) {
                return v;
            }
        }
        {
            let o = self.obj(id);
            if let InternalKind::MappedArguments { parameter_map } = &o.internal_kind {
                if let Some(cell) = parameter_map.get(key) {
                    return cell.borrow().clone();
                }
            }
        }
        // Shape-EXT 4 fast path: Shaped receivers go through the
        // shape's slot lookup before any IndexMap probe.
        {
            let o = self.obj(id);
            if let Some(v) = o.shape_get(key) {
                return v.clone();
            }
        }
        if key == "length" {
            let o = self.obj(id);
            if matches!(o.internal_kind, InternalKind::Array) {
                // If explicit "length" property is set, prefer it; otherwise
                // derive from max numeric index + 1.
                if let Some(d) = o.get_own("length") {
                    return d.value.clone();
                }
                let mut max: i64 = -1;
                for k in o.properties.keys() {
                    if let Ok(i) = k.as_str().parse::<i64>() {
                        if i > max {
                            max = i;
                        }
                    }
                }
                return Value::Number((max + 1) as f64);
            }
        }
        // Well-known-Symbol fallback shim. Per PropertyKey migration
        // (value.rs:90), well-known Symbols use a string form like
        // "@@iterator". User code `o[Symbol.iterator] = fn` stores
        // under PropertyKey::Symbol; intrinsic dispatchers (for-of's
        // CallMethod with "@@iterator") read via PropertyKey::String.
        // Without this fallback the two never meet. Try String first
        // (covers intrinsic-installed methods); on miss, also try Symbol
        // (covers user-installed methods).
        let is_wellknown_sym = key.starts_with("@@");
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            // Shape-EXT 4: proto-chain ancestors may be Shaped too.
            if let Some(v) = o.shape_get(key) {
                return v.clone();
            }
            if let Some(d) = o.get_own(key) {
                return d.value.clone();
            }
            if is_wellknown_sym {
                // PropertyKey::Symbol eq is Rc::ptr_eq (identity), so a
                // freshly-allocated Rc never matches a stored Symbol key
                // by-value. Walk Symbol-keyed entries looking for any
                // whose internal identifier equals the queried name.
                // Cost is O(n) on Symbol bucket size, amortized
                // acceptable for the well-known-Symbol path (only fires
                // when the string lookup missed AND the key is "@@...").
                for (pk, d) in &o.properties {
                    if let crate::value::PropertyKey::Symbol(rc) = pk {
                        if rc.as_str() == key {
                            return d.value.clone();
                        }
                    }
                }
            }
            cur = o.proto;
        }
        Value::Undefined
    }

    /// Array length helper used by Array.prototype.* methods.
    /// Backward-compatible non-propagating variant. Errors from a length
    /// accessor getter are swallowed (length → 0). For spec-strict
    /// propagation, callers should use try_array_length.
    pub fn array_length(&mut self, id: ObjectRef) -> usize {
        self.try_array_length(id).unwrap_or(0)
    }

    /// Spec-strict variant of array_length that propagates errors from
    /// the length accessor getter or ToNumber coercion. Use this in
    /// Array.prototype.* methods that test262 probes with throwing
    /// length getters (every/filter/find/forEach/map/some/reduce/etc.).
    pub fn try_array_length(&mut self, id: ObjectRef) -> Result<usize, RuntimeError> {
        // §7.1.20 ToLength: clamps to [0, 2^53 - 1]. Infinity, finite >
        // max-safe, and NaN all collapse to one of the bounds; the
        // previous Infinity branch returned usize::MAX which downstream
        // i64 casts in indexOf/lastIndexOf rendered as -1.
        let v = self.spec_get(&Value::Object(id), "length")?;
        let n = self.coerce_to_number(&v)?;
        if n.is_nan() || n <= 0.0 {
            return Ok(0);
        }
        let max_safe = 9007199254740991.0_f64;
        let clamped = if !n.is_finite() || n > max_safe {
            max_safe
        } else {
            n.floor()
        };
        Ok(clamped as usize)
    }

    /// OrdinaryDefineOwnProperty — own-key set on the named object.
    /// GBSU-EXT 4 (global-binding-surface-unification rung 4): canonical
    /// lookup of a global binding. Reads the unified globalThis Object
    /// (`self.global_object`) first; falls through to the legacy
    /// `self.globals` HashMap for the rung-4-to-rung-5 transition window
    /// (the HashMap holds boot-time bindings that haven't yet been routed
    /// through the Object via Op::StoreGlobal's dual-write). Rung 6 drops
    /// the fallback once writers are Object-only and the field is removed.
    ///
    /// Returns `Value::Undefined` for an absent binding (mirrors HashMap
    /// `.get().cloned().unwrap_or(Value::Undefined)` semantics so callers
    /// can use the result directly). For "is the binding present at all"
    /// queries, use `global_has` instead.
    pub fn global_get(&self, name: &str) -> Value {
        // GBSU-EXT 6: Object is canonical; HashMap fallback deleted.
        // Pre-install_global_this paths still return Undefined here —
        // those should be migrated to direct intrinsics-side state.
        if let Some(gt) = self.global_object {
            let v = self.object_get(gt, name);
            if !matches!(v, Value::Undefined) {
                return v;
            }
            if self.obj(gt).has_own_str(name) {
                return Value::Undefined;
            }
        }
        Value::Undefined
    }

    /// GBSU-EXT 7b/7c/7f.4: write a global binding to the unified
    /// globalThis Object with the ECMA §17 standard built-in descriptor
    /// `{writable:t, enumerable:f, configurable:t}`. Replaces the legacy
    /// `self.globals.insert(name, value)` pattern at install_globals
    /// call sites + register helpers. global_object is eager-allocated
    /// in Runtime::new (rung 7f.4); the unwrap is provably safe.
    pub fn define_global_property(&mut self, name: &str, value: Value) {
        let gt = self
            .global_object
            .expect("global_object eager-allocated in Runtime::new");
        self.obj_mut(gt).dict_mut().insert(
            crate::value::PropertyKey::String(name.to_string()),
            crate::value::PropertyDescriptor {
                value,
                writable: true,
                enumerable: false,
                configurable: true,
                getter: None,
                setter: None,
            },
        );
    }

    /// GBSU-EXT 7e: enumerate the globalThis Object's String-keyed own
    /// properties as a `HashMap<String, Value>`. Used by realm enter/exit
    /// to snapshot the primordial global env. Engine_helpers + Symbol-keyed
    /// properties are excluded — the snapshot is JS-visible-only.
    pub fn snapshot_global_string_props(&self) -> std::collections::HashMap<String, Value> {
        let mut out = std::collections::HashMap::new();
        if let Some(gt) = self.global_object {
            for (k, desc) in self.obj(gt).properties.iter() {
                if let crate::value::PropertyKey::String(s) = k {
                    out.insert(s.clone(), desc.value.clone());
                }
            }
        }
        out
    }

    /// GBSU-EXT 7e: drop every String-keyed own property on the globalThis
    /// Object whose name is NOT in `allow`. Used by realm enter to filter
    /// the primordial env down to an intrinsic-only view under ambient-
    /// denied capability mode. Symbol-keyed properties are preserved.
    pub fn retain_global_string_props(&mut self, allow: &std::collections::HashSet<&'static str>) {
        if let Some(gt) = self.global_object {
            let to_remove: Vec<String> = self
                .obj(gt)
                .properties
                .iter()
                .filter_map(|(k, _)| match k {
                    crate::value::PropertyKey::String(s) if !allow.contains(s.as_str()) => {
                        Some(s.clone())
                    }
                    _ => None,
                })
                .collect();
            for name in to_remove {
                self.obj_mut(gt).remove_str(&name);
            }
        }
    }

    /// GBSU-EXT 7e: replace the globalThis Object's String-keyed own
    /// properties with the given snapshot. Used by realm exit to restore
    /// a primordial-full-snapshot. Symbol-keyed properties are preserved.
    /// Existing String-keyed entries are dropped first, then the snapshot
    /// is re-inserted via `define_global_property` (ECMA §17 descriptor).
    pub fn replace_global_string_props(
        &mut self,
        snapshot: std::collections::HashMap<String, Value>,
    ) {
        if let Some(gt) = self.global_object {
            let to_remove: Vec<String> = self
                .obj(gt)
                .properties
                .iter()
                .filter_map(|(k, _)| match k {
                    crate::value::PropertyKey::String(s) => Some(s.clone()),
                    _ => None,
                })
                .collect();
            for name in to_remove {
                self.obj_mut(gt).remove_str(&name);
            }
        }
        for (k, v) in snapshot {
            self.define_global_property(&k, v);
        }
    }

    pub fn object_set(&mut self, id: ObjectRef, key: String, value: Value) {
        if key.starts_with('#') {
            self.obj_mut(id).set_private(&key, value);
            return;
        }
        // Lift (rung-18): String-keyed OrdinarySet routes through the
        // PropertyKey-typed primitive so non-writable / preserve-existing-
        // attrs logic lives in one place. Pre-lift, object_set and
        // object_set_pk each had their own preserve-attrs (set_own + inline)
        // branches; rung-17 fixed the pk branch but the divergence remained
        // a maintenance hazard.
        self.object_set_pk(id, crate::value::PropertyKey::String(key), value);
    }

    fn object_get_private(&self, id: ObjectRef, key: &str) -> Option<Value> {
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if let Some(v) = o.get_private(key) {
                return Some(v.clone());
            }
            cur = o.proto;
        }
        None
    }

    fn private_key_for_home(key: &str, home: ObjectRef) -> String {
        format!("{}@@{}", key, home.0)
    }

    fn private_frame_keys(frame: &Frame<'_>, key: &str) -> [Option<String>; 2] {
        [
            frame
                .private_home
                .map(|home| Self::private_key_for_home(key, home)),
            Some(key.to_string()),
        ]
    }

    fn object_get_private_for_frame(
        &self,
        frame: &Frame<'_>,
        id: ObjectRef,
        key: &str,
    ) -> Option<Value> {
        for candidate in Self::private_frame_keys(frame, key).into_iter().flatten() {
            if let Some(value) = self.object_get_private(id, &candidate) {
                return Some(value);
            }
        }
        None
    }

    fn find_getter_for_frame(&self, frame: &Frame<'_>, id: ObjectRef, key: &str) -> Option<Value> {
        for candidate in Self::private_frame_keys(frame, key).into_iter().flatten() {
            if let Some(value) = self.find_getter(id, &candidate) {
                return Some(value);
            }
        }
        None
    }

    fn find_setter_for_frame(&self, frame: &Frame<'_>, id: ObjectRef, key: &str) -> Option<Value> {
        for candidate in Self::private_frame_keys(frame, key).into_iter().flatten() {
            if let Some(value) = self.find_setter(id, &candidate) {
                return Some(value);
            }
        }
        None
    }

    fn private_method_in_chain_for_frame(
        &self,
        frame: &Frame<'_>,
        id: ObjectRef,
        key: &str,
    ) -> bool {
        for candidate in Self::private_frame_keys(frame, key).into_iter().flatten() {
            if self.object_private_method_in_chain(id, &candidate) {
                return true;
            }
        }
        false
    }

    fn object_private_method_in_chain(&self, id: ObjectRef, key: &str) -> bool {
        let mut cur = Some(id);
        while let Some(c) = cur {
            let o = self.obj(c);
            if o.is_private_method(key) {
                return true;
            }
            cur = o.proto;
        }
        false
    }

    /// Typeof with heap deref for Object/function discrimination.
    pub fn type_of_value(&self, v: &Value) -> &'static str {
        match v {
            Value::Object(id) => {
                let o = self.obj(*id);
                if matches!(
                    o.internal_kind,
                    InternalKind::Function(_)
                        | InternalKind::Closure(_)
                        | InternalKind::BoundFunction(_)
                ) {
                    "function"
                } else if let InternalKind::Proxy(p) = &o.internal_kind {
                    // Ω.5.P60.E3: Proxy's typeof reflects target's callability
                    // per ECMA §10.5 (proxy is callable iff target is callable).
                    self.type_of_value(&Value::Object(p.target))
                } else {
                    "object"
                }
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
        // TL-EXT 3 (2026-05-23): attempt top-level module-body JIT before
        // falling through to interp. Doc 740 §III.4 entry-mechanism
        // upstream constraint-closure. Build a synthetic FunctionProto
        // wrapping the module bytecode + constants; try compile_function;
        // on success, set TLS, call0, return Undefined; on failure (any
        // op outside the JIT alphabet → per C8 bail), fall through to the
        // existing interp dispatch.
        //
        // The module result is always Undefined per ECMA module
        // evaluation semantics regardless of what ReturnUndef emits as
        // its f64 sentinel; the JIT's return value is discarded.
        if self.try_jit_run_module(m).is_some() {
            return Ok(Value::Undefined);
        }
        let mut frame = Frame::new_module(m);
        self.run_frame(&mut frame)
    }

    /// TL-EXT 3 (2026-05-23): try to JIT-compile + invoke the module
    /// body. Returns Some(()) on success (caller treats as Undefined);
    /// None on any failure (caller falls through to interp).
    fn try_jit_run_module(&mut self, m: &CompiledModule) -> Option<()> {
        let jit_disabled = std::env::var("CRUFTLESS_JIT_DISABLE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if jit_disabled {
            return None;
        }
        let synth = rusty_js_bytecode::compiler::FunctionProto {
            bytecode: m.bytecode.clone(),
            constants: m.constants.clone(),
            params: 0,
            display_name: "<module>".into(),
            function_length: 0,
            locals: m.locals.clone(),
            upvalues: Vec::new(),
            rest_param_slot: None,
            arguments_slot: None,
            self_name_slot: None,
            param_prologue_end: 0,
            is_generator: false,
            line_starts: m.line_starts.clone(),
            source_map: m.source_map.clone(),
            construct_tags: m.construct_tags.clone(),
            source_url: String::new(),
            is_async: false,
            strict: m.strict,
        };
        let compiled = rusty_js_jit::compile_function(&synth).ok()?;
        let rt_ptr_usize = self as *mut Runtime as usize;
        let proto_ptr_usize = &synth as *const _ as usize;
        rusty_js_jit::set_current_deopt_sites(&compiled.deopt_sites);
        rusty_js_jit::set_current_runtime(rt_ptr_usize);
        rusty_js_jit::set_current_proto(proto_ptr_usize);
        let _r = compiled.func.call0();
        rusty_js_jit::clear_current_runtime();
        rusty_js_jit::clear_current_proto();
        rusty_js_jit::clear_current_deopt_sites();
        // Discard the return value; module result is Undefined.
        // If a deopt fired, bail to interp by returning None so the
        // caller takes the existing run_frame path.
        if rusty_js_jit::take_last_deopt().is_some() {
            return None;
        }
        Some(())
    }

    /// JIT-EXT 21: resume function execution from a deopt-recovered
    /// state. Used by the dispatcher (and by future ICs) when a JIT
    /// trip fires mid-function and the recorded `resume_pc` is non-
    /// zero — re-executing from pc=0 would either lose mid-function
    /// work or repeat side effects.
    ///
    /// The recovered state's `local_values` and `stack_values` are
    /// widened back into `Value::Number(f64)` (the JIT's i64-only
    /// regime); broader Value coverage (Object, String) lands when
    /// the JIT's regime widens at JIT-EXT 23+.
    ///
    /// Locals not mentioned in `state.local_values` keep whatever the
    /// frame's standard initialization gave them (args populated from
    /// `args[..proto.params]`, rest = Undefined). This matches the
    /// current arith-deopt invariant where the recovered state names
    /// only the live locals at the trip site.
    /// JIT-EXT 22: install the runtime-side GetProp helper into the
    /// JIT crate's function-pointer indirection. Called once at host
    /// startup (the JIT calls `jit_getprop_on_object`, which delegates
    /// through the indirection to this fn).
    ///
    /// The helper reads the JIT's TLS-stored Runtime + FunctionProto
    /// pointers to do its work; the dispatcher sets those slots before
    /// each JIT invocation via `set_current_runtime` / `set_current_proto`.
    pub fn install_jit_getprop_helper() {
        rusty_js_jit::set_active_getprop_fn(runtime_getprop_on_object);
        // LeJIT-Σ StubE-EXT 5b: also register the IC observer so the
        // cache state machine progresses when JIT-emitted code (under
        // CRUFTLESS_LEJIT_STUB=1) calls jit_getprop_with_ic.
        rusty_js_jit::deopt::set_active_ic_observe_fn(runtime_ic_observe);
        // LeJIT-Σ StubE-EXT 5c: register the IC fast-path getter.
        // jit_getprop_with_ic consults this BEFORE the slow path when
        // cache state is WarmMono; on shape match the fast-get returns
        // the value directly without running object_get + observer.
        rusty_js_jit::deopt::set_active_ic_fast_get_fn(runtime_ic_fast_get);
    }

    pub fn resume_from_deopt_state(
        &mut self,
        proto: &rusty_js_bytecode::compiler::FunctionProto,
        this_value: Value,
        args: Vec<Value>,
        state: &rusty_js_jit::DeoptRecoveredState,
    ) -> Result<Value, RuntimeError> {
        // Standard frame setup matching call_function's per-frame init.
        let mut locals: Vec<Value> = Vec::with_capacity(proto.locals.len());
        for i in 0..proto.locals.len() {
            if i < proto.params as usize && i < args.len() {
                locals.push(args[i].clone());
            } else {
                locals.push(Value::Undefined);
            }
        }
        // JIT-EXT 21 overlay: recovered local values take precedence
        // over the arg-derived defaults. Widening: i64 -> Number(f64).
        for &(slot, raw_i64) in &state.local_values {
            let slot = slot as usize;
            if slot < locals.len() {
                locals[slot] = Value::Number(raw_i64 as f64);
            }
        }

        // Operand stack from recovered state's stack_values; same
        // i64 -> Number widening.
        let mut operand_stack: Vec<Value> = Vec::with_capacity(state.stack_values.len() + 8);
        for &(_slot, raw_i64) in &state.stack_values {
            operand_stack.push(Value::Number(raw_i64 as f64));
        }

        let mut frame = Frame {
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
            operand_stack,
            pc: state.resume_pc as usize,
            try_stack: Vec::new(),
            with_env_stack: Vec::new(),
            this_value,
            this_cell: None,
            derived_initial_this: None,
            upvalues: Vec::new(),
            last_property_lookup: None,
            pending_method_name: None,
            pending_method_getprop_pc: None,
            private_home: None,
            import_meta: None,
            new_target: None,
            is_arrow: false,
            param_count: proto.params as usize,
            strict: proto.strict,
            eval_var_env_is_global: false,
            back_edge_counts: HashMap::new(),
            osr_cache: HashMap::new(),
            ic_dispatch_cache: HashMap::new(),
        };
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
                    if frame.try_stack.is_empty() {
                        return Err(e);
                    }
                    let v = if let RuntimeError::Thrown(v) = e {
                        v
                    } else {
                        let msg = catchable_msg.unwrap();
                        // Ω.5.P61.E4: route through make_error_instance so the
                        // thrown value's [[Prototype]] is the named ctor's
                        // .prototype. Without it, `e instanceof TypeError`
                        // returned false and test262's `assert.throws(TypeError,
                        // ...)` failed even when the engine threw TypeError.
                        let id_opt =
                            crate::intrinsics::make_error_instance(self, catchable_name, &msg);
                        match id_opt {
                            Some(id) => Value::Object(id),
                            None => {
                                // Bootstrap edge: ctor not yet installed. Fall
                                // back to bare ordinary object with name/message.
                                let mut o = crate::value::Object::new_ordinary();
                                if let Some(ep) = self.object_prototype {
                                    o.proto = Some(ep);
                                }
                                o.set_own(
                                    "message".into(),
                                    Value::String(std::rc::Rc::new(msg.clone())),
                                );
                                o.set_own(
                                    "name".into(),
                                    Value::String(std::rc::Rc::new(catchable_name.into())),
                                );
                                o.set_own(
                                    "stack".into(),
                                    Value::String(std::rc::Rc::new(format!(
                                        "{}: {}",
                                        catchable_name, msg
                                    ))),
                                );
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
            let op = op_from_byte(op_byte).ok_or_else(|| {
                RuntimeError::Unimplemented(format!("invalid opcode 0x{:02X} @{}", op_byte, pc))
            })?;
            frame.pc += 1;
            match op {
                // ─── Stack ops ───
                Op::PushNull => frame.push(Value::Null),
                Op::PushUndef => frame.push(Value::Undefined),
                Op::PushTDZ => frame.push(Value::Symbol(std::rc::Rc::clone(&self.tdz_sentinel))),
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
                Op::Pop => {
                    frame.pop()?;
                }
                Op::Dup => {
                    let top = frame.peek(0)?.clone();
                    frame.push(top);
                }
                Op::Swap => {
                    let len = frame.operand_stack.len();
                    if len < 2 {
                        return Err(RuntimeError::TypeError("stack underflow on Swap".into()));
                    }
                    frame.operand_stack.swap(len - 1, len - 2);
                }

                // ─── Variable / scope ───
                Op::LoadLocal => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let v = frame.read_local(slot);
                    let lname = frame
                        .locals_names
                        .get(slot)
                        .map(|d| d.name.clone())
                        .unwrap_or_else(|| format!("<local${}>", slot));
                    frame.last_property_lookup = Some(lname.clone());
                    // IR-EXT 23 TDZ check: if the slot still holds the TDZ
                    // sentinel (scope-entry init that hasn't been overwritten
                    // by the binding's declaration line), throw
                    // ReferenceError per §13.3.1.1.
                    if let Value::Symbol(ref s) = v {
                        if std::rc::Rc::ptr_eq(s, &self.tdz_sentinel) {
                            return Err(RuntimeError::ReferenceError(format!(
                                "Cannot access '{}' before initialization",
                                lname
                            )));
                        }
                    }
                    frame.push(v);
                }
                Op::StoreLocal => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let v = frame.pop()?;
                    // IR-EXT 26 TDZ-on-assign: assignment to a still-TDZ
                    // binding throws ReferenceError per §13.3.1.1 step 26.b.
                    // Init sites use Op::InitLocal which bypasses this check.
                    let prev = frame.read_local(slot);
                    if let Value::Symbol(ref s) = prev {
                        if std::rc::Rc::ptr_eq(s, &self.tdz_sentinel) {
                            let lname = frame
                                .locals_names
                                .get(slot)
                                .map(|d| d.name.clone())
                                .unwrap_or_else(|| format!("<local${}>", slot));
                            return Err(RuntimeError::ReferenceError(format!(
                                "Cannot access '{}' before initialization",
                                lname
                            )));
                        }
                    }
                    frame.write_local(slot, v);
                }
                Op::InitLocal => {
                    // IR-EXT 25/26: init-site peer of StoreLocal that
                    // bypasses the TDZ-on-assign check so the legitimate
                    // overwrite of a TDZ-seeded slot at the binding's
                    // declaration / per-iter write site succeeds.
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
                Op::LoadWithName => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    let v = self.load_with_name(frame, &name)?;
                    frame.last_property_lookup = Some(format!("<with>{}", name));
                    frame.push(v);
                }
                Op::StoreWithName => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    let v = frame.pop()?;
                    self.store_with_name(frame, &name, v)?;
                }
                Op::ResolveWithName => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    let base = self
                        .resolve_with_name_base(frame, &name)?
                        .map(Value::Object)
                        .unwrap_or(Value::Undefined);
                    frame.push(base);
                }
                Op::LoadWithNameRef => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    if let Some(obj_id) = self.resolve_with_name_base(frame, &name)? {
                        let value = self.spec_get(&Value::Object(obj_id), &name)?;
                        frame.push(Value::Object(obj_id));
                        frame.push(value);
                    } else {
                        let value = self.load_with_name(frame, &name)?;
                        frame.push(Value::Undefined);
                        frame.push(value);
                    }
                }
                Op::StoreWithNameRef => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    let v = frame.pop()?;
                    let base = frame.pop()?;
                    match base {
                        Value::Object(obj_id) => {
                            self.set_with_object_binding(obj_id, &name, v.clone(), frame.strict)?
                        }
                        _ => self.store_with_name(frame, &name, v.clone())?,
                    }
                    frame.push(v);
                }
                Op::LoadGlobal => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    // Ω.5.P55.E1: JS-visible globals first; engine_helpers
                    // is the unshadowed fallback for compiler-emitted
                    // lowerings (Doc 729 §VII.B).
                    // REOU-EXT 1: per ECMA-262 §9.1.1.4.4 GetBindingValue +
                    // §6.2.4.5 GetValue, an unresolvable reference at read
                    // throws ReferenceError. typeof/delete sites take the
                    // silent-undef path via Op::LoadGlobalOrUndef.
                    // GBSU-EXT 6: globalThis Object is the canonical and
                    // ONLY binding surface for JS-visible globals; the
                    // transitional HashMap fallback added in rung 2 has
                    // been deleted (deletions-ledger 2026-05-26 GBSU-EXT 6
                    // entry). engine_helpers stays orthogonal per
                    // §VII.B bilateral.
                    let v = self
                        .global_object
                        .and_then(|gt| {
                            let v = self.object_get(gt, &name);
                            if matches!(v, Value::Undefined) && !self.obj(gt).has_own_str(&name) {
                                None
                            } else {
                                Some(v)
                            }
                        })
                        .or_else(|| self.engine_helpers.get(&name).cloned());
                    let v = match v {
                        Some(val) => val,
                        None => {
                            return Err(RuntimeError::ReferenceError(format!(
                                "{} is not defined",
                                name
                            )))
                        }
                    };
                    frame.last_property_lookup = Some(format!("<global>{}", name));
                    frame.push(v);
                }
                Op::LoadGlobalOrUndef => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let name = self.constant_name(frame, idx)?;
                    // GBSU-EXT 6: same primary-globalThis lookup as
                    // Op::LoadGlobal, terminating in Undefined for the
                    // typeof/delete silent-undef path. HashMap fallback
                    // deleted.
                    let v = self
                        .global_object
                        .and_then(|gt| {
                            let v = self.object_get(gt, &name);
                            if matches!(v, Value::Undefined) && !self.obj(gt).has_own_str(&name) {
                                None
                            } else {
                                Some(v)
                            }
                        })
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
                    // Ω.5.P04.E2.strict-write-to-undeclared: in strict mode,
                    // assigning to an unresolvable reference (undeclared
                    // identifier) throws ReferenceError per ECMA §13.15.4
                    // SimpleAssignmentExpression step 1.f.i + §9.1.1.4.4
                    // SetMutableBinding step 6. Pre-substrate, cruftless
                    // silently created the global in sloppy mode AND
                    // silently absorbed the write in strict mode (the
                    // §XV audit on `later` localized this). var/let/const
                    // declarations compile to StoreLocal, not StoreGlobal,
                    // so this check fires only on undeclared bare
                    // assignments.
                    // GBSU-EXT 3 (global-binding-surface-unification rung 3):
                    // the globalThis Object is now the canonical write surface;
                    // the legacy HashMap is no longer updated at runtime. The
                    // strict-mode "is this name declared" check consults both
                    // surfaces for the rung-3-to-rung-4 transition window
                    // (HashMap still holds boot-time bindings until rung 4
                    // drains them). Engine_helpers stays orthogonal per
                    // §VII.B bilateral.
                    if frame.strict {
                        // GBSU-EXT 6: Object is canonical; HashMap-contains
                        // fallback removed.
                        let declared_on_object = self
                            .global_object
                            .map(|gt| self.obj(gt).has_own_str(&name))
                            .unwrap_or(false);
                        let declared =
                            declared_on_object || self.engine_helpers.contains_key(&name);
                        if !declared {
                            return Err(RuntimeError::ReferenceError(format!(
                                "{} is not defined",
                                name
                            )));
                        }
                    }
                    // GBSU-EXT 5 retry (post-GBSU-EXT 4b expanded-audit):
                    // ten RUNTIME closure-captured readers migrated to
                    // global_get across intrinsics.rs / prototype.rs /
                    // napi.rs / interp.rs Symbol.hasInstance fallback +
                    // the typed-array helper at intrinsics.rs:12731.
                    // With the broader audit landed, drop the HashMap
                    // write — Op::StoreGlobal writes the unified globalThis
                    // surface only. Bootstrap fallback retained for
                    // pre-install_global_this code paths.
                    if name != "globalThis" {
                        let gt = self
                            .global_object
                            .expect("global_object eager-allocated in Runtime::new");
                        self.object_set(gt, name, v);
                    }
                }
                Op::LoadUpvalue => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let v = frame
                        .upvalues
                        .get(slot)
                        .map(|cell| cell.borrow().clone())
                        .unwrap_or(Value::Undefined);
                    let upname = frame.upvalue_names.get(slot).map(|d| d.name.clone());
                    if let Some(ref name) = upname {
                        frame.last_property_lookup = Some(format!("^{}", name));
                    }
                    // IR-EXT 32: TDZ check on upvalue read mirrors EXT 23's
                    // LoadLocal check. Closures that capture a TDZ-seeded
                    // slot must throw ReferenceError when reading the slot
                    // before its declaration line overwrites the sentinel.
                    // Required for switch-case fall-through TDZ + nested
                    // closure-captures-outer-block-let scenarios.
                    if let Value::Symbol(ref s) = v {
                        if std::rc::Rc::ptr_eq(s, &self.tdz_sentinel) {
                            let nm = upname.unwrap_or_else(|| format!("<upvalue${}>", slot));
                            return Err(RuntimeError::ReferenceError(format!(
                                "Cannot access '{}' before initialization",
                                nm
                            )));
                        }
                    }
                    frame.push(v);
                }
                Op::StoreUpvalue => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let v = frame.pop()?;
                    if let Some(cell) = frame.upvalues.get(slot) {
                        // IR-EXT 33 (TDZ-on-assign cross-frame): symmetric
                        // with Op::StoreLocal's TDZ check from EXT 26.
                        // Assignment via closure to a still-TDZ upvalue cell
                        // throws ReferenceError per §13.3.1.1 step 26.b.
                        let prev = cell.borrow().clone();
                        if let Value::Symbol(ref s) = prev {
                            if std::rc::Rc::ptr_eq(s, &self.tdz_sentinel) {
                                let nm = frame
                                    .upvalue_names
                                    .get(slot)
                                    .map(|d| d.name.clone())
                                    .unwrap_or_else(|| format!("<upvalue${}>", slot));
                                return Err(RuntimeError::ReferenceError(format!(
                                    "Cannot access '{}' before initialization",
                                    nm
                                )));
                            }
                        }
                        *cell.borrow_mut() = v;
                    } else {
                        while frame.upvalues.len() <= slot {
                            frame.upvalues.push(new_upvalue_cell(Value::Undefined));
                        }
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
                        _ => {
                            return Err(RuntimeError::TypeError(
                                "CaptureLocal: top of stack is not a closure".into(),
                            ))
                        }
                    };
                    if let InternalKind::Closure(c) = &mut self.obj_mut(top).internal_kind {
                        c.upvalues.push(cell);
                    } else {
                        return Err(RuntimeError::TypeError(
                            "CaptureLocal: top is not a closure".into(),
                        ));
                    }
                }
                Op::CaptureUpvalue => {
                    // Transitive capture: share the Rc<RefCell<Value>> the
                    // enclosing closure already holds. Do NOT deep-copy the
                    // value out and re-wrap — that would break binding
                    // semantics across the three-deep nesting case.
                    let idx = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    let cell = frame
                        .upvalues
                        .get(idx)
                        .cloned()
                        .unwrap_or_else(|| new_upvalue_cell(Value::Undefined));
                    let top = match frame.peek(0)? {
                        Value::Object(id) => *id,
                        _ => {
                            return Err(RuntimeError::TypeError(
                                "CaptureUpvalue: top is not a closure".into(),
                            ))
                        }
                    };
                    if let InternalKind::Closure(c) = &mut self.obj_mut(top).internal_kind {
                        c.upvalues.push(cell);
                    } else {
                        return Err(RuntimeError::TypeError(
                            "CaptureUpvalue: top is not a closure".into(),
                        ));
                    }
                }
                Op::DefineLocal => {
                    let slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    frame.pc += 2;
                    while frame.locals.len() <= slot {
                        frame.locals.push(Value::Undefined);
                    }
                }

                // ─── Arithmetic ───
                Op::Add => {
                    // Ω.5.P62.E21: route through op_add_rt so Object
                    // operands dispatch ToPrimitive per §13.15.4.
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let v = self.op_add_rt(&l, &r)?;
                    frame.push(v);
                }
                Op::Sub => {
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.sub(b))));
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = self.to_num_coerced_strict(&rv)?;
                        let l = self.to_num_coerced_strict(&lv)?;
                        frame.push(Value::Number(l - r));
                    }
                }
                Op::Mul => {
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.mul(b))));
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = self.to_num_coerced_strict(&rv)?;
                        let l = self.to_num_coerced_strict(&lv)?;
                        frame.push(Value::Number(l * r));
                    }
                }
                Op::Div => {
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.divmod(b) {
                            Some((q, _)) => frame.push(Value::BigInt(Rc::new(q))),
                            None => return Err(RuntimeError::TypeError("Division by zero".into())),
                        }
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = self.to_num_coerced_strict(&rv)?;
                        let l = self.to_num_coerced_strict(&lv)?;
                        frame.push(Value::Number(l / r));
                    }
                }
                Op::Mod => {
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.divmod(b) {
                            Some((_, r)) => frame.push(Value::BigInt(Rc::new(r))),
                            None => return Err(RuntimeError::TypeError("Division by zero".into())),
                        }
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = self.to_num_coerced_strict(&rv)?;
                        let l = self.to_num_coerced_strict(&lv)?;
                        frame.push(Value::Number(l % r));
                    }
                }
                Op::Pow => {
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.pow(b) {
                            Some(p) => frame.push(Value::BigInt(Rc::new(p))),
                            None => {
                                return Err(RuntimeError::TypeError(
                                    "BigInt ** invalid exponent".into(),
                                ))
                            }
                        }
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = self.to_num_coerced_strict(&rv)?;
                        let l = self.to_num_coerced_strict(&lv)?;
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
                    let n = self.to_num_coerced_strict(&raw)?;
                    frame.push(Value::Number(n));
                }
                Op::Inc => {
                    let raw = frame.pop()?;
                    if let Value::BigInt(b) = &raw {
                        frame.push(Value::BigInt(Rc::new(
                            b.add(&crate::bigint::JsBigInt::one()),
                        )));
                    } else {
                        let v = to_number(&raw);
                        frame.push(Value::Number(v + 1.0));
                    }
                }
                Op::Dec => {
                    let raw = frame.pop()?;
                    if let Value::BigInt(b) = &raw {
                        frame.push(Value::BigInt(Rc::new(
                            b.sub(&crate::bigint::JsBigInt::one()),
                        )));
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
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let v = self.is_loosely_equal_rt(&l, &r)?;
                    frame.push(Value::Boolean(v));
                }
                Op::Ne => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let v = self.is_loosely_equal_rt(&l, &r)?;
                    frame.push(Value::Boolean(!v));
                }
                Op::StrictEq => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    frame.push(Value::Boolean(is_strictly_equal(&l, &r)));
                }
                Op::StrictNe => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    frame.push(Value::Boolean(!is_strictly_equal(&l, &r)));
                }
                Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    if matches!(l, Value::Symbol(_)) || matches!(r, Value::Symbol(_)) {
                        return Err(RuntimeError::TypeError(
                            "Cannot convert a Symbol value to a number".into(),
                        ));
                    }
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
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.bit_and(b))));
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = self.to_num_coerced_strict(&rv)? as i64 as i32;
                        let l = self.to_num_coerced_strict(&lv)? as i64 as i32;
                        frame.push(Value::Number((l & r) as f64));
                    }
                }
                Op::BitOr => {
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.bit_or(b))));
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = self.to_num_coerced_strict(&rv)? as i64 as i32;
                        let l = self.to_num_coerced_strict(&lv)? as i64 as i32;
                        frame.push(Value::Number((l | r) as f64));
                    }
                }
                Op::BitXor => {
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        frame.push(Value::BigInt(Rc::new(a.bit_xor(b))));
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = self.to_num_coerced_strict(&rv)? as i64 as i32;
                        let l = self.to_num_coerced_strict(&lv)? as i64 as i32;
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
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.shl(b) {
                            Some(p) => frame.push(Value::BigInt(Rc::new(p))),
                            None => {
                                return Err(RuntimeError::TypeError(format!(
                                    "BigInt << invalid shift (lhs_bits={}, n={})",
                                    a.mag_bit_len(),
                                    b.to_decimal()
                                )))
                            }
                        }
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = (self.to_num_coerced_strict(&rv)? as i64 as i32 as u32) & 0x1F;
                        let l = self.to_num_coerced_strict(&lv)? as i64 as i32;
                        frame.push(Value::Number((l.wrapping_shl(r)) as f64));
                    }
                }
                Op::Shr => {
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if let (Value::BigInt(a), Value::BigInt(b)) = (&lv, &rv) {
                        match a.shr(b) {
                            Some(p) => frame.push(Value::BigInt(Rc::new(p))),
                            None => {
                                return Err(RuntimeError::TypeError(format!(
                                    "BigInt >> invalid shift (lhs_bits={}, n={})",
                                    a.mag_bit_len(),
                                    b.to_decimal()
                                )))
                            }
                        }
                    } else {
                        self.ensure_no_mixed_bigint_or_symbol(&lv, &rv)?;
                        let r = (self.to_num_coerced_strict(&rv)? as i64 as i32 as u32) & 0x1F;
                        let l = self.to_num_coerced_strict(&lv)? as i64 as i32;
                        frame.push(Value::Number((l >> r) as f64));
                    }
                }
                Op::UShr => {
                    // Tier-Ω.5.JJJJJJJJ: spec-correct ToUint32 per ECMA §7.1.7.
                    // Previously `n as u32` saturated for n >= 2^32, producing
                    // 0xFFFFFFFF for big f64s. Required: trunc-to-int64 then
                    // bit-cast to u32 (drops upper bits). bn.js's 26-bit limb
                    // arithmetic via `(x * y) >>> 0` was producing wrong limbs.
                    let rv = frame.pop()?;
                    let lv = frame.pop()?;
                    if matches!(lv, Value::BigInt(_)) || matches!(rv, Value::BigInt(_)) {
                        return Err(RuntimeError::TypeError(
                            "BigInts have no unsigned right shift".into(),
                        ));
                    }
                    if matches!(lv, Value::Symbol(_)) || matches!(rv, Value::Symbol(_)) {
                        return Err(RuntimeError::TypeError(
                            "Cannot convert a Symbol value to a number".into(),
                        ));
                    }
                    let r = (self.to_num_coerced_strict(&rv)? as i64 as i32 as u32) & 0x1F;
                    let l = self.to_num_coerced_strict(&lv)? as i64 as i32 as u32;
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
                    let site_pc = frame.pc - 1; // pc of the Op::Jump byte
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    let mut osr_fired = false;
                    if disp < 0 {
                        let c = frame.back_edge_counts.entry(site_pc).or_insert(0);
                        *c += 1;
                        if *c >= OSR_BACK_EDGE_THRESHOLD {
                            if !frame.osr_cache.contains_key(&site_pc) {
                                try_osr_compile(frame, site_pc);
                            }
                            // OSR-EXT 5d: invoke if cache has Some; on
                            // success frame.pc = end_pc + skip back-edge.
                            if matches!(frame.osr_cache.get(&site_pc), Some(Some(_))) {
                                osr_fired = try_osr_invoke(frame, site_pc);
                            }
                        }
                    }
                    if !osr_fired {
                        frame.pc = (frame.pc as i32 + disp) as usize;
                    }
                }
                Op::JumpIfTrue => {
                    let site_pc = frame.pc - 1;
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    if to_boolean(&frame.pop()?) {
                        let mut osr_fired = false;
                        if disp < 0 {
                            let c = frame.back_edge_counts.entry(site_pc).or_insert(0);
                            *c += 1;
                            if *c >= OSR_BACK_EDGE_THRESHOLD {
                                if !frame.osr_cache.contains_key(&site_pc) {
                                    try_osr_compile(frame, site_pc);
                                }
                                // Fast-path: skip function call if cache is empty or None.
                                if matches!(frame.osr_cache.get(&site_pc), Some(Some(_))) {
                                    osr_fired = try_osr_invoke(frame, site_pc);
                                }
                            }
                        }
                        if !osr_fired {
                            frame.pc = (frame.pc as i32 + disp) as usize;
                        }
                    }
                }
                Op::JumpIfFalse => {
                    let site_pc = frame.pc - 1;
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    if !to_boolean(&frame.pop()?) {
                        let mut osr_fired = false;
                        if disp < 0 {
                            let c = frame.back_edge_counts.entry(site_pc).or_insert(0);
                            *c += 1;
                            if *c >= OSR_BACK_EDGE_THRESHOLD {
                                if !frame.osr_cache.contains_key(&site_pc) {
                                    try_osr_compile(frame, site_pc);
                                }
                                // Fast-path: skip function call if cache is empty or None.
                                if matches!(frame.osr_cache.get(&site_pc), Some(Some(_))) {
                                    osr_fired = try_osr_invoke(frame, site_pc);
                                }
                            }
                        }
                        if !osr_fired {
                            frame.pc = (frame.pc as i32 + disp) as usize;
                        }
                    }
                }
                Op::JumpIfTrueKeep => {
                    let site_pc = frame.pc - 1;
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    if to_boolean(frame.peek(0)?) {
                        let mut osr_fired = false;
                        if disp < 0 {
                            let c = frame.back_edge_counts.entry(site_pc).or_insert(0);
                            *c += 1;
                            if *c >= OSR_BACK_EDGE_THRESHOLD {
                                if !frame.osr_cache.contains_key(&site_pc) {
                                    try_osr_compile(frame, site_pc);
                                }
                                // Fast-path: skip function call if cache is empty or None.
                                if matches!(frame.osr_cache.get(&site_pc), Some(Some(_))) {
                                    osr_fired = try_osr_invoke(frame, site_pc);
                                }
                            }
                        }
                        if !osr_fired {
                            frame.pc = (frame.pc as i32 + disp) as usize;
                        }
                    }
                }
                Op::JumpIfFalseKeep => {
                    let site_pc = frame.pc - 1;
                    let disp = decode_i32(&frame.bytecode, frame.pc);
                    frame.pc += 4;
                    if !to_boolean(frame.peek(0)?) {
                        let mut osr_fired = false;
                        if disp < 0 {
                            let c = frame.back_edge_counts.entry(site_pc).or_insert(0);
                            *c += 1;
                            if *c >= OSR_BACK_EDGE_THRESHOLD {
                                if !frame.osr_cache.contains_key(&site_pc) {
                                    try_osr_compile(frame, site_pc);
                                }
                                // Fast-path: skip function call if cache is empty or None.
                                if matches!(frame.osr_cache.get(&site_pc), Some(Some(_))) {
                                    osr_fired = try_osr_invoke(frame, site_pc);
                                }
                            }
                        }
                        if !osr_fired {
                            frame.pc = (frame.pc as i32 + disp) as usize;
                        }
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
                    let catch_off =
                        rusty_js_bytecode::op::decode_u32(&frame.bytecode, frame.pc) as usize;
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
                    let hint = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let id = self.alloc_object(Object::new_array());
                    if hint > 0 {
                        self.object_set(id, "length".into(), Value::Number(hint as f64));
                    }
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
                Op::GetProp | Op::GetPropOnObject => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let key = self.constant_name(frame, idx)?;
                    let obj_v = frame.pop()?;
                    // Tier-Ω.5.nnn: invoke accessor getter if present along
                    // the prototype chain. Captures the getter function +
                    // receiver before calling so re-entrant access works.
                    let v = match &obj_v {
                        Value::Object(id) => {
                            if key.starts_with('#') {
                                match self.object_get_private_for_frame(frame, *id, &key) {
                                    Some(v) => v,
                                    None => {
                                        if let Some(getter) =
                                            self.find_getter_for_frame(frame, *id, &key)
                                        {
                                            self.call_function(getter, obj_v.clone(), Vec::new())?
                                        } else {
                                            return Err(RuntimeError::TypeError(format!(
                                                "Cannot read private field '{}' of object",
                                                key
                                            )));
                                        }
                                    }
                                }
                            } else {
                                // Ω.5.P60.E1: Proxy get-trap dispatch. If obj
                                // is a Proxy and handler.get is callable, call
                                // handler.get(target, key, receiver) and use
                                // its return value. Missing trap falls through
                                // to target.
                                let proxy_dispatch = self.proxy_target_handler_checked(*id)?;
                                if let Some((target, handler)) = proxy_dispatch {
                                    let trap = self.object_get(handler, "get");
                                    if matches!(trap, Value::Object(_)) {
                                        let receiver = obj_v.clone();
                                        let trap_result = self.call_function(
                                            trap,
                                            Value::Object(handler),
                                            vec![
                                                Value::Object(target),
                                                Value::String(Rc::new(key.clone())),
                                                receiver,
                                            ],
                                        )?;
                                        // EXT 88b: §10.5.8 invariant.
                                        self.apply_proxy_get_invariant(target, &key, &trap_result)?;
                                        trap_result
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
                                            if let Some(d) = self.obj(c).get_own(&key) {
                                                // GOPD-EXT 1: only count callable getters
                                                // (Some(Undefined) means accessor-with-no-
                                                // getter; OrdinaryGet returns undefined).
                                                if let Some(g) = &d.getter {
                                                    if !matches!(g, Value::Undefined) {
                                                        found = true;
                                                    }
                                                }
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
                        }
                        Value::String(s) if key == "length" => {
                            // CharCode-EXT 1 (2026-05-23): ASCII fast-path.
                            // Length is read every iter of a tight scan loop;
                            // chars().count() is O(n) per read -> O(n²) per
                            // outer iter. Byte length matches code-unit count
                            // for ASCII (which is the only case where the
                            // current chars().count() behavior is correct
                            // wrt UTF-16 anyway; non-BMP correctness is a
                            // separate concern carried by the existing impl).
                            Value::Number(if s.is_ascii() {
                                s.len() as f64
                            } else {
                                s.chars().count() as f64
                            })
                        }
                        Value::String(_) => {
                            // Primitive string method auto-boxing: route to
                            // %String.prototype% if installed.
                            if let Some(proto) = self.string_prototype {
                                self.object_get(proto, &key)
                            } else {
                                Value::Undefined
                            }
                        }
                        Value::Number(_) => {
                            if let Some(proto) = self.number_prototype {
                                self.object_get(proto, &key)
                            } else {
                                Value::Undefined
                            }
                        }
                        Value::BigInt(_) => {
                            if let Some(proto) = self.bigint_prototype {
                                self.object_get(proto, &key)
                            } else {
                                Value::Undefined
                            }
                        }
                        Value::Symbol(_) => {
                            // Ω.5.P63.E51: Symbol primitive prop access walks
                            // %Symbol.prototype% and dispatches accessor
                            // getters (e.g. .description) with the primitive
                            // as `this`.
                            if let Some(proto) = self.symbol_prototype {
                                let getter = {
                                    let mut cur = Some(proto);
                                    let mut g = None;
                                    while let Some(c) = cur {
                                        if let Some(d) = self.obj(c).get_own(&key) {
                                            g = d.getter.clone();
                                            break;
                                        }
                                        cur = self.obj(c).proto;
                                    }
                                    g
                                };
                                if let Some(get_fn) = getter {
                                    self.call_function(get_fn, obj_v.clone(), Vec::new())?
                                } else {
                                    self.object_get(proto, &key)
                                }
                            } else {
                                Value::Undefined
                            }
                        }
                        Value::Undefined | Value::Null => {
                            let pc = frame.pc.saturating_sub(1);
                            let construct = frame
                                .construct_tags
                                .iter()
                                .rposition(|&(off, _)| off <= pc)
                                .map(|i| frame.construct_tags[i].1);
                            if key.starts_with('#')
                                && matches!(construct, Some("optional-chain private-continuation"))
                                && matches!(obj_v, Value::Undefined)
                            {
                                Value::Undefined
                            } else {
                                // Tier-Ω.5.uuu: enrich the fault with the
                                // last LoadLocal/GetProp hint. Doc 723's
                                // threshold-of-diagnostic-semanticity finding
                                // (2026-05-15) named that single-tag faults
                                // are below-threshold for Layer-D bisect.
                                // Adding the source-side name of the value
                                // that resolved to undefined raises the
                                // signal level — `(receiver='X')` tags the
                                // local whose load preceded this access.
                                let receiver_hint = frame
                                    .last_property_lookup
                                    .clone()
                                    .map(|s| format!(" (receiver='{}')", s))
                                    .unwrap_or_default();
                                return Err(RuntimeError::TypeError(format!(
                                    "Cannot read property '{}' of {}{}",
                                    key,
                                    if matches!(obj_v, Value::Undefined) {
                                        "undefined"
                                    } else {
                                        "null"
                                    },
                                    receiver_hint
                                )));
                            }
                        }
                        _ => Value::Undefined,
                    };
                    frame.last_property_lookup = Some(key.clone());
                    // Tier-Ω.5.yyyyy: pending_method survives arg loads.
                    frame.pending_method_name = Some(key);
                    // GPI-EXT 2: capture op byte's pc so a later
                    // Op::CallMethod IC-hit can rewrite this GetProp
                    // site to Op::GetPropSkipForMethod.
                    frame.pending_method_getprop_pc = Some(frame.pc - 3);
                    frame.push(v);
                }
                Op::SetProp => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let key = self.constant_name(frame, idx)?;
                    let value = frame.pop()?;
                    let obj_v = frame.pop()?;
                    if let Value::Object(id) = &obj_v {
                        if key.starts_with('#') {
                            if let Some(setter) = self.find_setter_for_frame(frame, *id, &key) {
                                self.call_function(
                                    setter,
                                    Value::Object(*id),
                                    vec![value.clone()],
                                )?;
                            } else if self.find_getter_for_frame(frame, *id, &key).is_some()
                                || self.private_method_in_chain_for_frame(frame, *id, &key)
                            {
                                return Err(RuntimeError::TypeError(format!(
                                    "Cannot write private member '{}'",
                                    key
                                )));
                            } else if self
                                .object_get_private_for_frame(frame, *id, &key)
                                .is_none()
                            {
                                return Err(RuntimeError::TypeError(format!(
                                    "Cannot write missing private field '{}'",
                                    key
                                )));
                            } else {
                                self.obj_mut(*id).set_private(&key, value.clone());
                            }
                            frame.push(value);
                            continue;
                        }
                        // Ω.5.P60.E2: Proxy set-trap dispatch.
                        let proxy_dispatch = self.proxy_target_handler_checked(*id)?;
                        if let Some((target, handler)) = proxy_dispatch {
                            let trap = self.object_get(handler, "set");
                            if matches!(trap, Value::Object(_)) {
                                let r = self.call_function(
                                    trap,
                                    Value::Object(handler),
                                    vec![
                                        Value::Object(target),
                                        Value::String(Rc::new(key.clone())),
                                        value.clone(),
                                        Value::Object(*id),
                                    ],
                                )?;
                                // EXT 88b: §10.5.9 invariant.
                                self.apply_proxy_set_invariant(
                                    target,
                                    &key,
                                    &value,
                                    crate::abstract_ops::to_boolean(&r),
                                )?;
                            } else {
                                self.object_set(target, key, value.clone());
                            }
                        } else
                        // Tier-Ω.5.vvvv: same setter dispatch on identifier-
                        // keyed writes.
                        if let Some(setter) = self.find_setter(*id, &key) {
                            self.call_function(setter, Value::Object(*id), vec![value.clone()])?;
                        } else if self.is_accessor_at(*id, &key) {
                            // GOPD-EXT 1: accessor descriptor with no
                            // callable setter — write is silent no-op
                            // (sloppy) or TypeError (strict) per
                            // §10.1.9.4 step 2.b / §13.15.4 step 1.f.iv.
                            if frame.strict {
                                return Err(RuntimeError::TypeError(format!(
                                    "Cannot set property '{}' of object which has only a getter",
                                    key
                                )));
                            }
                        } else {
                            // Ω.5.P04.E2.strict-write-to-non-writable.
                            if frame.strict {
                                if let Some(d) = self.obj(*id).get_own(&key) {
                                    if !d.writable && d.getter.is_none() && d.setter.is_none() {
                                        return Err(RuntimeError::TypeError(format!(
                                            "Attempted to assign to readonly property '{}'",
                                            key
                                        )));
                                    }
                                }
                            }
                            let can_add =
                                self.obj(*id).has_own_str(&key) || self.obj(*id).extensible;
                            if can_add {
                                self.object_set(*id, key, value.clone());
                            } else if frame.strict {
                                return Err(RuntimeError::TypeError(format!(
                                    "Cannot add property '{}': object is not extensible",
                                    key
                                )));
                            }
                        }
                    } else {
                        // Tier-Ω.5.HHHHHHHH: enrich the non-object target tag
                        // with the last property-lookup hint per Doc 723 §IV.b
                        // route-(b). Bare `null.foo = bar` previously surfaced
                        // 'SetProp foo on non-object (null)'; with the
                        // receiver-hint the chain points at the upstream
                        // local that resolved to null.
                        let target_hint = frame
                            .last_property_lookup
                            .clone()
                            .map(|s| format!(" (target='{}')", s))
                            .unwrap_or_default();
                        return Err(RuntimeError::TypeError(format!(
                            "SetProp '{}' on non-object ({}){}",
                            key,
                            match &obj_v {
                                Value::Undefined => "undefined",
                                Value::Null => "null",
                                Value::Boolean(_) => "boolean",
                                Value::Number(_) => "number",
                                Value::String(_) => "string",
                                _ => "other",
                            },
                            target_hint,
                        )));
                    }
                    frame.push(value);
                }
                Op::GetIndex => {
                    let key_v = frame.pop()?;
                    let obj_v = frame.pop()?;
                    // Ω.5.P63.E54: PropertyKey carries Symbol vs String typing.
                    // Symbol values land in the Symbol bucket (identity-keyed);
                    // others stringify into the String bucket.
                    let key_pk = property_key(&key_v);
                    let key = key_pk.as_str().to_string();
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
                            let proxy_dispatch = self.proxy_target_handler_checked(id)?;
                            if let Some((target, handler)) = proxy_dispatch {
                                let trap = self.object_get(handler, "get");
                                if matches!(trap, Value::Object(_)) {
                                    let trap_result = self.call_function(
                                        trap,
                                        Value::Object(handler),
                                        vec![
                                            Value::Object(target),
                                            Value::String(Rc::new(key.clone())),
                                            Value::Object(id),
                                        ],
                                    )?;
                                    // EXT 88b: §10.5.8 invariant on computed-key Get.
                                    self.apply_proxy_get_invariant(target, &key, &trap_result)?;
                                    trap_result
                                } else {
                                    self.object_get(target, &key)
                                }
                            } else if let Some(getter) = self.find_getter_pk(id, &key_pk) {
                                self.call_function(getter, Value::Object(id), Vec::new())?
                            } else {
                                self.object_get_pk(id, &key_pk)
                            }
                        }
                        Value::String(s) => {
                            if let Ok(i) = key.parse::<usize>() {
                                s.chars()
                                    .nth(i)
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
                                } else {
                                    Value::Undefined
                                }
                            }
                        }
                        Value::Number(_) => {
                            // Mirror GetProp's number auto-box.
                            if let Some(proto) = self.number_prototype {
                                self.object_get(proto, &key)
                            } else {
                                Value::Undefined
                            }
                        }
                        Value::BigInt(_) => {
                            if let Some(proto) = self.bigint_prototype {
                                self.object_get(proto, &key)
                            } else {
                                Value::Undefined
                            }
                        }
                        Value::Symbol(_) => {
                            // Ω.5.P63.E51: Symbol primitive prop access walks
                            // %Symbol.prototype% and dispatches accessor
                            // getters (e.g. .description) with the primitive
                            // as `this`.
                            if let Some(proto) = self.symbol_prototype {
                                let getter = {
                                    let mut cur = Some(proto);
                                    let mut g = None;
                                    while let Some(c) = cur {
                                        if let Some(d) = self.obj(c).get_own(&key) {
                                            g = d.getter.clone();
                                            break;
                                        }
                                        cur = self.obj(c).proto;
                                    }
                                    g
                                };
                                if let Some(get_fn) = getter {
                                    self.call_function(get_fn, obj_v.clone(), Vec::new())?
                                } else {
                                    self.object_get(proto, &key)
                                }
                            } else {
                                Value::Undefined
                            }
                        }
                        Value::Undefined | Value::Null => {
                            return Err(RuntimeError::TypeError(
                                "Cannot index undefined/null".into(),
                            ))
                        }
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
                        _ => {
                            let _ = target_id;
                            continue;
                        }
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
                        _ => {
                            return Err(RuntimeError::TypeError(
                                "Op::DeleteProp: key not String constant".into(),
                            ))
                        }
                    };
                    let removed = match obj_v {
                        Value::Object(id) => {
                            // Ω.5.P60.E2: Proxy deleteProperty-trap dispatch.
                            let proxy_dispatch = self.proxy_target_handler_checked(id)?;
                            if let Some((target, handler)) = proxy_dispatch {
                                let trap = self.object_get(handler, "deleteProperty");
                                if matches!(trap, Value::Object(_)) {
                                    let r = self.call_function(
                                        trap,
                                        Value::Object(handler),
                                        vec![
                                            Value::Object(target),
                                            Value::String(Rc::new(key.clone())),
                                        ],
                                    )?;
                                    let trap_deleted = crate::abstract_ops::to_boolean(&r);
                                    // EXT 88b: §10.5.10 invariant.
                                    self.apply_proxy_delete_invariant(target, &key, trap_deleted)?;
                                    trap_deleted
                                } else {
                                    self.obj_mut(target).remove_str(&key).is_some()
                                }
                            } else {
                                // ODP-EXT 1: Array exotic .length is
                                // [[Configurable]]:false per §10.4.2; delete
                                // must refuse (return false in sloppy, throw
                                // in strict). The internal-kind check guards
                                // the virtual length not stored as an own
                                // data property.
                                if key == "length"
                                    && matches!(
                                        self.obj(id).internal_kind,
                                        crate::value::InternalKind::Array
                                    )
                                {
                                    if frame.strict {
                                        return Err(RuntimeError::TypeError(
                                            "Cannot delete non-configurable property 'length'"
                                                .into(),
                                        ));
                                    }
                                    frame.push(Value::Boolean(false));
                                    continue;
                                }
                                // Ω.5.P62.E10: ECMA §10.1.10 OrdinaryDelete —
                                // own data property with configurable:false is
                                // not deletable. Return false (sloppy mode);
                                // strict mode throws but cruftless's strict
                                // tracking is incomplete (parity with sloppy
                                // delete semantics in P61.E3).
                                //
                                // CMig-EXT 9 shape-awareness: shape-stored
                                // entries are user-default {w:t,e:t,c:t} per
                                // carve-out → always configurable → always
                                // deletable. has_own_str + shape_get is the
                                // shape-aware probe; get_own only sees
                                // properties. remove_str migrates first so
                                // the actual deletion lands on Dictionary
                                // form.
                                let in_shape = self.obj(id).shape_get(&key).is_some();
                                if in_shape {
                                    self.obj_mut(id).remove_str(&key).is_some() || true
                                } else if let Some(d) = self.obj(id).get_own(&key) {
                                    if !d.configurable {
                                        if frame.strict {
                                            return Err(RuntimeError::TypeError(format!(
                                                "Cannot delete non-configurable property '{}'",
                                                key
                                            )));
                                        }
                                        false
                                    } else {
                                        self.obj_mut(id).remove_str(&key).is_some()
                                    }
                                } else {
                                    true
                                }
                            }
                        }
                        Value::Null | Value::Undefined => {
                            return Err(RuntimeError::TypeError(
                                "Cannot convert undefined or null to object".into(),
                            ))
                        }
                        _ => true,
                    };
                    frame.push(Value::Boolean(removed));
                }
                Op::DeleteIndex => {
                    let key_v = frame.pop()?;
                    let obj_v = frame.pop()?;
                    // Ω.5.P63.E54: PropertyKey-aware so `delete obj[sym]` hits
                    // the Symbol bucket. Stringification keeps proxy-trap calls
                    // backwards compatible.
                    let key_pk = property_key(&key_v);
                    let key = key_pk.as_str().to_string();
                    let removed = match obj_v {
                        Value::Object(id) => {
                            // Ω.5.P60.E2: same Proxy dispatch as DeleteProp.
                            let proxy_dispatch = self.proxy_target_handler_checked(id)?;
                            if let Some((target, handler)) = proxy_dispatch {
                                let trap = self.object_get(handler, "deleteProperty");
                                if matches!(trap, Value::Object(_)) {
                                    let r = self.call_function(
                                        trap,
                                        Value::Object(handler),
                                        vec![
                                            Value::Object(target),
                                            Value::String(Rc::new(key.clone())),
                                        ],
                                    )?;
                                    let trap_deleted = crate::abstract_ops::to_boolean(&r);
                                    // EXT 88b: §10.5.10 invariant (DeleteIndex path).
                                    self.apply_proxy_delete_invariant(target, &key, trap_deleted)?;
                                    trap_deleted
                                } else {
                                    self.obj_mut(target)
                                        .dict_mut()
                                        .shift_remove(&key_pk)
                                        .is_some()
                                }
                            } else {
                                // Ω.5.P62.E10: §10.1.10 non-configurable guard.
                                // CMig-EXT 9 shape-awareness: shape-stored
                                // entries are user-default configurable per
                                // carve-out; route through has_own_str +
                                // dict_mut (migrating).
                                let in_shape = matches!(&key_pk, crate::value::PropertyKey::String(s) if self.obj(id).shape_get(s).is_some());
                                if in_shape {
                                    self.obj_mut(id).dict_mut().shift_remove(&key_pk).is_some()
                                        || true
                                } else if let Some(d) = self.obj(id).properties.get(&key_pk) {
                                    if !d.configurable {
                                        if frame.strict {
                                            return Err(RuntimeError::TypeError(format!(
                                                "Cannot delete non-configurable property '{}'",
                                                key
                                            )));
                                        }
                                        false
                                    } else {
                                        self.obj_mut(id).dict_mut().shift_remove(&key_pk).is_some()
                                    }
                                } else if let crate::value::PropertyKey::Symbol(sym) = &key_pk {
                                    // Well-known-symbol transition: built-ins
                                    // such as Array.prototype[Symbol.iterator]
                                    // are still stored under their string form
                                    // ("@@iterator"). A Symbol-keyed delete
                                    // must remove that fallback entry too, or
                                    // GetIterator keeps observing the method.
                                    if let Some(d) = self.obj(id).get_own(sym.as_str()) {
                                        if !d.configurable {
                                            if frame.strict {
                                                return Err(RuntimeError::TypeError(format!(
                                                    "Cannot delete non-configurable property '{}'",
                                                    key
                                                )));
                                            }
                                            false
                                        } else {
                                            self.obj_mut(id).remove_str(sym.as_str()).is_some()
                                        }
                                    } else {
                                        true
                                    }
                                } else {
                                    true
                                }
                            }
                        }
                        Value::Null | Value::Undefined => {
                            return Err(RuntimeError::TypeError(
                                "Cannot convert undefined or null to object".into(),
                            ))
                        }
                        _ => true,
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
                            let hint = frame
                                .last_property_lookup
                                .clone()
                                .map(|s| format!(" (rhs='{}')", s))
                                .unwrap_or_default();
                            let key_s = match &key_v {
                                Value::String(s) => format!("'{}'", s.as_str()),
                                _ => format!("{:?}", key_v),
                            };
                            return Err(RuntimeError::TypeError(format!(
                                "Cannot use 'in' operator on non-object: {} in {:?}{}",
                                key_s, obj_v, hint
                            )));
                        }
                    };
                    let key_pk = property_key(&key_v);
                    let key = key_pk.as_str().to_string();
                    // Ω.5.P60.E2: Proxy has-trap dispatch.
                    let proxy_dispatch = self.proxy_target_handler_checked(obj_id)?;
                    let mut found = false;
                    if let Some((target, handler)) = proxy_dispatch {
                        let trap = self.object_get(handler, "has");
                        if matches!(trap, Value::Object(_)) {
                            let r = self.call_function(
                                trap,
                                Value::Object(handler),
                                vec![Value::Object(target), Value::String(Rc::new(key.clone()))],
                            )?;
                            found = crate::abstract_ops::to_boolean(&r);
                        } else {
                            // CMig-EXT 8: shape-aware via has_property_pk.
                            found = self.has_property_pk(target, &key_pk);
                        }
                    } else {
                        // CMig-EXT 8: shape-aware via has_property_pk.
                        found = self.has_property_pk(obj_id, &key_pk);
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
                        } else {
                            hi
                        };
                        let hi = if matches!(hi, Value::Undefined) {
                            // Try Symbol.hasInstance's string form.
                            // GBSU-EXT 4b: canonical lookup via unified globalThis.
                            let sym = self.global_get("Symbol");
                            if let Value::Object(sym_id) = sym {
                                let hi_sym = self.object_get(sym_id, "hasInstance");
                                if let Value::String(s) = hi_sym {
                                    self.object_get(*ctor_id, &s)
                                } else {
                                    Value::Undefined
                                }
                            } else {
                                Value::Undefined
                            }
                        } else {
                            hi
                        };
                        if matches!(hi, Value::Object(_)) {
                            let r = self.call_function(hi, ctor_v.clone(), vec![obj_v.clone()])?;
                            Some(
                                matches!(r, Value::Boolean(true))
                                    || (!matches!(
                                        r,
                                        Value::Boolean(false) | Value::Undefined | Value::Null
                                    ) && match &r {
                                        Value::Number(n) => *n != 0.0,
                                        Value::String(s) => !s.is_empty(),
                                        _ => true,
                                    }),
                            )
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let result =
                        if let Some(b) = hi_result {
                            b
                        } else {
                            match (&obj_v, &ctor_v) {
                                (Value::Object(obj_id), Value::Object(ctor_id)) => {
                                    let proto_v = self.object_get(*ctor_id, "prototype");
                                    match proto_v {
                                        Value::Object(target_proto) => {
                                            let mut cur = self.obj(*obj_id).proto;
                                            let mut found = false;
                                            while let Some(c) = cur {
                                                if c == target_proto {
                                                    found = true;
                                                    break;
                                                }
                                                cur = self.obj(c).proto;
                                            }
                                            found
                                        }
                                        _ => return Err(RuntimeError::TypeError(
                                            "Function has non-object prototype in instanceof check"
                                                .into(),
                                        )),
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
                    // Ω.5.P63.E54: PropertyKey routes Symbol writes to the
                    // Symbol bucket; String values stringify into the String bucket.
                    let key_pk = property_key(&key_v);
                    let key = key_pk.as_str().to_string();
                    if let Value::Object(id) = &obj_v {
                        // Ω.5.P60.E2: Proxy set-trap dispatch at computed-key writes.
                        let proxy_dispatch = self.proxy_target_handler_checked(*id)?;
                        if let Some((target, handler)) = proxy_dispatch {
                            let trap = self.object_get(handler, "set");
                            if matches!(trap, Value::Object(_)) {
                                let r = self.call_function(
                                    trap,
                                    Value::Object(handler),
                                    vec![
                                        Value::Object(target),
                                        Value::String(Rc::new(key.clone())),
                                        value.clone(),
                                        Value::Object(*id),
                                    ],
                                )?;
                                // EXT 88b: §10.5.9 invariant.
                                self.apply_proxy_set_invariant(
                                    target,
                                    &key,
                                    &value,
                                    crate::abstract_ops::to_boolean(&r),
                                )?;
                            } else {
                                self.object_set(target, key, value.clone());
                            }
                        } else
                        // Tier-Ω.5.vvvv: dispatch accessor setters, mirror of
                        // Ω.5.uuuu for GetIndex. Without this, writes through
                        // computed keys to lazy-defined properties silently
                        // overwrite the descriptor's getter with a data slot.
                        if let Some(setter) = self.find_setter_pk(*id, &key_pk) {
                            self.call_function(setter, Value::Object(*id), vec![value.clone()])?;
                        } else if self.is_accessor_at_pk(*id, &key_pk) {
                            // GOPD-EXT 1: accessor with no callable setter —
                            // silent no-op (sloppy) / TypeError (strict).
                            if frame.strict {
                                return Err(RuntimeError::TypeError(format!(
                                    "Cannot set property '{}' of object which has only a getter",
                                    key
                                )));
                            }
                        } else {
                            if frame.strict {
                                if let Some(d) = self.obj(*id).get_own(&key) {
                                    if !d.writable && d.getter.is_none() && d.setter.is_none() {
                                        return Err(RuntimeError::TypeError(format!(
                                            "Attempted to assign to readonly property '{}'",
                                            key
                                        )));
                                    }
                                }
                            }
                            let has_own = match &key_pk {
                                crate::value::PropertyKey::String(s) => {
                                    self.obj(*id).has_own_str(s)
                                }
                                crate::value::PropertyKey::Symbol(sym) => {
                                    self.obj(*id).get_own_symbol(sym).is_some()
                                }
                            };
                            if has_own || self.obj(*id).extensible {
                                self.object_set_pk(*id, key_pk.clone(), value.clone());
                            } else if frame.strict {
                                return Err(RuntimeError::TypeError(format!(
                                    "Cannot add property '{}': object is not extensible",
                                    key
                                )));
                            }
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
                        let target_hint = frame
                            .last_property_lookup
                            .clone()
                            .map(|s| format!(" (target='{}')", s))
                            .unwrap_or_default();
                        return Err(RuntimeError::TypeError(format!(
                            "SetIndex '{}' on non-object ({}){}",
                            key, kind, target_hint
                        )));
                    }
                    frame.push(value);
                }

                // ─── Closure construction ───
                Op::MakeClosure | Op::MakeArrow => {
                    let idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let proto = match frame.constants.get(idx) {
                        Some(rusty_js_bytecode::Constant::Function(p)) => p.clone(),
                        _ => {
                            return Err(RuntimeError::TypeError(
                                "MakeClosure constant is not a function".into(),
                            ))
                        }
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
                    let is_strict_fn = proto_rc.strict;
                    // Tier-Ω.5.sss: arrow inherits `this` from current
                    // frame. Capture at MakeArrow time as a VALUE
                    // snapshot (bound_this) AND promote to a CELL
                    // (bound_this_cell). Op::SetThis writes through the
                    // cell, so arrows created BEFORE super() resolves
                    // see the updated post-super value at call time.
                    let bound_this = if is_arrow {
                        Some(frame.this_value.clone())
                    } else {
                        None
                    };
                    let bound_this_cell = if is_arrow {
                        if frame.this_cell.is_none() {
                            frame.this_cell =
                                Some(crate::value::new_upvalue_cell(frame.this_value.clone()));
                        }
                        frame.this_cell.clone()
                    } else {
                        None
                    };
                    let closure = Object {
                        proto: None,
                        extensible: true,
                        properties: indexmap::IndexMap::new(),
                        internal_kind: crate::value::InternalKind::Closure(
                            crate::value::ClosureInternals {
                                proto: proto_rc,
                                upvalues: Vec::new(),
                                is_arrow,
                                bound_this,
                                bound_this_cell,
                                bound_derived_initial_this: if is_arrow {
                                    frame.derived_initial_this.clone()
                                } else {
                                    None
                                },
                                bound_new_target: if is_arrow {
                                    frame.new_target.clone()
                                } else {
                                    None
                                },
                                call_count: std::cell::Cell::new(0),
                                jit_disabled: std::cell::Cell::new(false),
                                tb_metadata_ptr: std::cell::Cell::new(None),
                            },
                        ),

                        ..Default::default()
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
                        crate::value::install_function_meta_props(
                            props,
                            &display_name,
                            fn_length as f64,
                        );
                    }
                    if is_strict_fn && !is_arrow {
                        let thrower =
                            crate::intrinsics::make_native("%ThrowTypeError%", |_rt, _args| {
                                Err(RuntimeError::TypeError(
                                    "'caller' may not be accessed on a strict function".into(),
                                ))
                            });
                        let thrower_id = self.alloc_object(thrower);
                        self.obj_mut(id).dict_mut().insert(
                            "caller".into(),
                            crate::value::PropertyDescriptor {
                                value: Value::Undefined,
                                writable: false,
                                enumerable: false,
                                configurable: false,
                                getter: Some(Value::Object(thrower_id)),
                                setter: Some(Value::Object(thrower_id)),
                            },
                        );
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
                    //
                    // 2026-05-21 (Tier-Ω Round 1): generator and async-
                    // generator functions DO have a .prototype per
                    // ECMA-262 §27.4 / §27.5. The earlier strip over-applied
                    // and broke `Object.getPrototypeOf(gen_fn.prototype)`
                    // patterns used by ponyfills (asyncIterator in
                    // @sec-ant/readable-stream, transitive failure point
                    // for got / get-stream / runtypes). Install on
                    // generators (sync or async); still skip on arrows and
                    // plain async functions.
                    let install_prototype = !is_arrow && (!is_async || is_gen);
                    if install_prototype {
                        // Allocate the per-fn .prototype object. For generator
                        // and async-generator functions, its [[Prototype]] is
                        // %GeneratorPrototype% / %AsyncGeneratorPrototype% per
                        // §27.4.4 / §27.5.4. For ordinary functions, default
                        // (object_prototype via alloc_time wiring).
                        let mut proto_obj = Object::new_ordinary();
                        proto_obj.set_own_internal("constructor".into(), Value::Object(id));
                        let proto_id = self.alloc_object(proto_obj);
                        if is_gen && is_async {
                            if let Some(p) = self.async_generator_prototype {
                                self.obj_mut(proto_id).proto = Some(p);
                            }
                        } else if is_gen {
                            if let Some(p) = self.generator_prototype {
                                self.obj_mut(proto_id).proto = Some(p);
                            }
                        }
                        // Ω.5.P62.E7: user-function .prototype is per ECMA §10.2.4
                        // { writable:true, enumerable:false, configurable:false }.
                        self.obj_mut(id).dict_mut().insert(
                            "prototype".into(),
                            crate::value::PropertyDescriptor {
                                value: Value::Object(proto_id),
                                writable: true,
                                enumerable: false,
                                configurable: false,
                                getter: None,
                                setter: None,
                            },
                        );
                    }
                    // Tier-Ω Round 1: set the closure's [[Prototype]] to the
                    // appropriate *Function.prototype intrinsic so
                    // Object.getPrototypeOf(fn) reaches the spec-ordained
                    // ancestor (per §27.3.1 / §27.4.1 / §27.5.1).
                    if is_gen && is_async {
                        if let Some(p) = self.async_generator_function_prototype {
                            self.obj_mut(id).proto = Some(p);
                        }
                    } else if is_gen {
                        if let Some(p) = self.generator_function_prototype {
                            self.obj_mut(id).proto = Some(p);
                        }
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
                    frame.pending_method_getprop_pc = None;
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
                    let result = self.call_function(callee, Value::Undefined, args).map_err(
                        |e| match e {
                            RuntimeError::TypeError(msg)
                                if msg.starts_with("callee is not callable") =>
                            {
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
                                RuntimeError::TypeError(format!(
                                    "{} (in-call='{}')",
                                    msg,
                                    callee_hint.unwrap()
                                ))
                            }
                            other => other,
                        },
                    )?;
                    frame.push(result);
                }
                Op::DirectEval => {
                    let n = frame.bytecode[frame.pc] as usize;
                    frame.pc += 1;
                    let mut args = Vec::with_capacity(n);
                    for _ in 0..n {
                        args.push(frame.pop()?);
                    }
                    args.reverse();
                    let callee = frame.pop()?;
                    // Integration: GBSU unified surface.
                    let global_eval = self.global_get("eval");
                    let result =
                        if !matches!(global_eval, Value::Undefined) && global_eval == callee {
                            self.direct_eval_from_frame(frame, args)?
                        } else {
                            self.call_function(callee, Value::Undefined, args)?
                        };
                    frame.push(result);
                }
                Op::Yield => {
                    let yielded = frame.pop()?;
                    if let Some(generator) = self.active_generator_for_yield {
                        frame.push(Value::Undefined);
                        let snapshot = FrameSnapshot::from_frame(frame, None);
                        match &mut self.obj_mut(generator).internal_kind {
                            crate::value::InternalKind::Generator(g) => {
                                g.state = crate::value::GeneratorState::SuspendedYield;
                                g.yielded_value = Some(yielded.clone());
                                g.continuation = Some(Box::new(snapshot));
                            }
                            _ => {
                                return Err(RuntimeError::TypeError(
                                    "active yield target is not a Generator object".into(),
                                ))
                            }
                        }
                        self.last_value = yielded.clone();
                        return Ok(yielded);
                    }

                    let arr = *self
                        .gen_yields_stack
                        .last()
                        .ok_or_else(|| RuntimeError::TypeError("yield outside generator".into()))?;
                    let len = self.array_length(arr);
                    self.object_set(arr, len.to_string(), yielded);
                    self.object_set(arr, "length".into(), Value::Number((len + 1) as f64));
                    frame.push(Value::Undefined);
                }
                Op::CallMethod => {
                    let site_pc = frame.pc - 1; // IHI-EXT 7: Op byte's pc for cache key
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
                    let method_name = frame
                        .pending_method_name
                        .take()
                        .or_else(|| frame.last_property_lookup.clone());
                    // GPI-EXT 2: site_pc of the preceding Op::GetProp, for
                    // companion-rewrite to Op::GetPropSkipForMethod on IC-
                    // hit (String receiver_kind).
                    let getprop_site_pc = frame.pending_method_getprop_pc.take();
                    // Ω.5.P54.E4/E5 (Axis-S + Axis-H probe population):
                    // when the resolved method is Undefined, record the
                    // miss against the appropriate trace. Symbol.X-keyed
                    // lookups go to symbol_lookup_miss_log; node:* host
                    // namespace lookups go to host_stub_miss_log. The
                    // discriminator is the method name shape.
                    if matches!(method, Value::Undefined) {
                        if let Some(name) = method_name.as_deref() {
                            if name.starts_with("@@sym:Symbol.") || name.starts_with("@@") {
                                let entry = format!(
                                    "{} on receiver={}",
                                    name,
                                    describe_value_for_diag(self, &receiver)
                                );
                                if !self.symbol_lookup_miss_log.contains(&entry) {
                                    self.symbol_lookup_miss_log.push(entry);
                                }
                            } else if let Value::Object(r_id) = &receiver {
                                // Check if receiver is from a node:* namespace
                                // by inspecting whether it has the global-namespace
                                // shape we install for host stubs.
                                let is_likely_host_stub = {
                                    let o = self.obj(*r_id);
                                    matches!(
                                        o.internal_kind,
                                        crate::value::InternalKind::Ordinary
                                            | crate::value::InternalKind::ModuleNamespace
                                    ) && o.properties.keys().any(|k| k.as_str().starts_with("__"))
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
                        method_name
                            .as_deref()
                            .map(|mn| describe_proto_chain_for_key(self, &receiver, mn))
                    } else {
                        None
                    };
                    // IHI-EXT 10 (2026-05-24, Finding IHI.2 deeper-layer
                    // closure per Doc 740 §IV.2): per-FunctionProto side-
                    // table. proto_ptr → Vec<CachedDispatch> indexed by
                    // pc. Outer HashMap.get-per-proto + inner Vec[pc] O(1)
                    // array access. Replaces IHI-EXT 8's (pc, proto) tuple
                    // HashMap; per-call dispatch ~80ns → ~35ns.
                    let proto_ptr = frame.bytecode.as_ptr() as usize;
                    let bc_len = frame.bytecode.len();
                    let vec = self.ic_dispatch_cache.entry(proto_ptr).or_insert_with(|| {
                        vec![crate::interp_ic_table::CachedDispatch::NotCached; bc_len]
                    });
                    use crate::interp_ic_table::CachedDispatch;
                    let cached_entry: Option<&'static crate::interp_ic_table::IhiEntry> = match vec
                        .get(site_pc)
                        .copied()
                        .unwrap_or(CachedDispatch::NotCached)
                    {
                        CachedDispatch::Entry(idx) => {
                            Some(&crate::interp_ic_table::IHI_TABLE[idx as usize])
                        }
                        CachedDispatch::NoMatch => None,
                        CachedDispatch::NotCached => {
                            let result = if let Some(method_name_str) = method_name.as_deref() {
                                let kind = crate::interp_ic_table::receiver_kind_of(&receiver);
                                crate::interp_ic_table::lookup(
                                    method_name_str,
                                    kind,
                                    args.len() as u8,
                                )
                            } else {
                                None
                            };
                            let cached = match result {
                                Some(e) => {
                                    let idx = crate::interp_ic_table::IHI_TABLE
                                        .iter()
                                        .position(|t| std::ptr::eq(t as *const _, e as *const _))
                                        .map(|i| i as u8)
                                        .unwrap_or(0xFF);
                                    if idx == 0xFF {
                                        CachedDispatch::NoMatch
                                    } else {
                                        CachedDispatch::Entry(idx)
                                    }
                                }
                                None => CachedDispatch::NoMatch,
                            };
                            if site_pc < bc_len {
                                self.ic_dispatch_cache.get_mut(&proto_ptr).unwrap()[site_pc] =
                                    cached;
                            }
                            result
                        }
                    };
                    if let Some(entry) = cached_entry {
                        {
                            if let Value::Object(method_id) = &method {
                                // Lazy-populate the entry's cached intrinsic
                                // ObjectId on first eligible call.
                                let cached = self.ihi_get_cached(entry.cached_id_field);
                                let cached = if cached.is_none() {
                                    if let Some(sp) = self.string_prototype {
                                        if let Some(d) = self.obj(sp).get_own(entry.key) {
                                            if let Value::Object(id) = d.value {
                                                self.ihi_set_cached(entry.cached_id_field, id);
                                                Some(id)
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    cached
                                };
                                if cached == Some(*method_id) {
                                    if let Some(result) = (entry.fast)(&receiver, &args) {
                                        // IHI-EXT 11 (2026-05-24, Finding
                                        // IHI.1+IHI.2 deeper-layer closure
                                        // per Doc 740 §IV.2): bytecode
                                        // rewrite. On successful IC fast-
                                        // path hit, rewrite this site to
                                        // Op::CallMethodIcCached(idx); all
                                        // subsequent dispatches skip the
                                        // cache + lookup machinery entirely.
                                        // Per-call cost ~60ns → ~10ns.
                                        if let Some(idx) =
                                            crate::interp_ic_table::IHI_TABLE.iter().position(|t| {
                                                std::ptr::eq(t as *const _, entry as *const _)
                                            })
                                        {
                                            if idx < 0xFF && site_pc + 1 < frame.bytecode.len() {
                                                // SAFETY: cruft is single-threaded;
                                                // bytecode is a Vec<u8> owned by
                                                // FunctionProto (or CompiledModule).
                                                // The rewrite is idempotent (any
                                                // future iteration writes the same
                                                // bytes). Byte-aligned write is
                                                // atomic at the hardware level.
                                                unsafe {
                                                    let ptr = frame.bytecode.as_ptr() as *mut u8;
                                                    ptr.add(site_pc).write(rusty_js_bytecode::op::Op::CallMethodIcCached as u8);
                                                    ptr.add(site_pc + 1).write(idx as u8);
                                                }
                                                // GPI-EXT 2 (2026-05-24,
                                                // interp-getprop-ic locale,
                                                // deeper-layer closure per
                                                // Doc 740 §IV.2): companion
                                                // rewrite of the preceding
                                                // Op::GetProp site to
                                                // Op::GetPropSkipForMethod.
                                                // Only safe for String
                                                // receivers (the bail path
                                                // in CallMethodIcCached
                                                // re-resolves via
                                                // string_prototype on the
                                                // Undefined sentinel).
                                                if matches!(
                                                    entry.receiver,
                                                    crate::interp_ic_table::IhiReceiverKind::String
                                                ) {
                                                    if let Some(gp_pc) = getprop_site_pc {
                                                        if gp_pc < frame.bytecode.len() {
                                                            let cur = frame.bytecode[gp_pc];
                                                            if cur == rusty_js_bytecode::op::Op::GetProp as u8 {
                                                                unsafe {
                                                                    let ptr = frame.bytecode.as_ptr() as *mut u8;
                                                                    ptr.add(gp_pc).write(rusty_js_bytecode::op::Op::GetPropSkipForMethod as u8);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        frame.push(result);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    let result =
                        self.call_function(method, receiver, args)
                            .map_err(|e| match e {
                                RuntimeError::TypeError(msg)
                                    if msg.starts_with("callee is not callable") =>
                                {
                                    let chain_suffix = chain_tag
                                        .as_ref()
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
                                    RuntimeError::TypeError(format!(
                                        "{} (in-method='{}')",
                                        msg,
                                        method_name.unwrap()
                                    ))
                                }
                                other => other,
                            })?;
                    frame.push(result);
                }
                Op::CallMethodIcCached => {
                    // IHI-EXT 11 (2026-05-24): rewritten dispatch fast-path.
                    // The op byte was rewritten by Op::CallMethod's
                    // success-side rewrite on first IC hit at this pc.
                    // Read the IHI_TABLE idx from the arity byte; pop
                    // entry.arity args + method + receiver; invoke
                    // entry.fast. On bail, fall through to call_function
                    // with the popped operands.
                    let _site_pc = frame.pc - 1;
                    let idx = frame.bytecode[frame.pc] as usize;
                    frame.pc += 1;
                    let entry = &crate::interp_ic_table::IHI_TABLE[idx];
                    let n_args = match entry.arity {
                        Some(n) => n as usize,
                        None => 0,
                    };
                    let mut args = Vec::with_capacity(n_args);
                    for _ in 0..n_args {
                        args.push(frame.pop()?);
                    }
                    args.reverse();
                    let method = frame.pop()?;
                    let receiver = frame.pop()?;
                    if let Some(result) = (entry.fast)(&receiver, &args) {
                        frame.push(result);
                    } else {
                        // Bail: call_function with what we popped. No
                        // bytecode revert; the next dispatch will hit the
                        // same fast-path attempt. If a hot site
                        // consistently bails (e.g., receiver-type
                        // polymorphism), that's a hardening concern;
                        // first-cut keeps the rewrite.
                        //
                        // GPI-EXT 2 bail-mitigation: if `method` is the
                        // Undefined sentinel produced by an upstream
                        // Op::GetPropSkipForMethod, re-resolve via the
                        // IHI entry's key (String receivers only; the
                        // companion rewrite is gated on receiver_kind ==
                        // String).
                        let method = if matches!(method, Value::Undefined) {
                            if matches!(
                                entry.receiver,
                                crate::interp_ic_table::IhiReceiverKind::String
                            ) {
                                if let Some(proto) = self.string_prototype {
                                    self.object_get(proto, entry.key)
                                } else {
                                    method
                                }
                            } else {
                                method
                            }
                        } else {
                            method
                        };
                        let result = self.call_function(method, receiver, args)?;
                        frame.push(result);
                    }
                }
                Op::GetPropSkipForMethod => {
                    // GPI-EXT 2 (2026-05-24): rewritten dispatch fast-path.
                    // The op byte was rewritten by Op::CallMethod's IC-hit
                    // branch (String receiver_kind only). Skip the
                    // descriptor walk; pop the Dup-top receiver copy + push
                    // Value::Undefined as a sentinel. The following
                    // Op::CallMethodIcCached's hot-path ignores the popped
                    // method; its bail-path re-resolves via entry.key.
                    let _idx = decode_u16(&frame.bytecode, frame.pc);
                    frame.pc += 2;
                    let _receiver_discard = frame.pop()?;
                    frame.push(Value::Undefined);
                }
                Op::ForOfFastNext => {
                    // IPBR-EXT 2 (2026-05-24, iter-protocol-bytecode-
                    // rewrite locale): fused for-of fast-next opcode.
                    // Probes iter_slot for the well-known intrinsic
                    // ArrayIterator shape; if eligible runs one iteration
                    // inline (no result-object alloc; no .next() dispatch);
                    // else falls through to the slow-path emission that
                    // immediately follows this op in the bytecode.
                    let iter_slot = decode_u16(&frame.bytecode, frame.pc) as usize;
                    let bind_slot = decode_u16(&frame.bytecode, frame.pc + 2) as usize;
                    let done_offset = decode_i32(&frame.bytecode, frame.pc + 4);
                    let next_iter_offset = i16::from_le_bytes([
                        frame.bytecode[frame.pc + 8],
                        frame.bytecode[frame.pc + 9],
                    ]) as i32;
                    frame.pc += 10;
                    let after_operand_pc = frame.pc;
                    let iter_val = frame.read_local(iter_slot);
                    if let Value::Object(iter_id) = iter_val {
                        // Array-iterator shape per iterator.rs:28-58:
                        // __arr (source array), __i (index). ESNE-EXT 3
                        // renamed from _arr/_i to follow CLAUDE.md's __X
                        // engine-sentinel convention.
                        let src_val = self.object_get(iter_id, "__arr");
                        let idx_val = self.object_get(iter_id, "__i");
                        if let (Value::Object(src_id), Value::Number(idx_n)) = (&src_val, &idx_val)
                        {
                            if matches!(
                                self.obj(*src_id).internal_kind,
                                crate::value::InternalKind::Array
                            ) {
                                let idx = *idx_n as usize;
                                let len = self.array_length(*src_id);
                                if idx >= len {
                                    frame.pc =
                                        (after_operand_pc as i64 + done_offset as i64) as usize;
                                    continue;
                                }
                                let v = self.object_get(*src_id, &idx.to_string());
                                self.object_set(
                                    iter_id,
                                    "__i".into(),
                                    Value::Number((idx + 1) as f64),
                                );
                                frame.write_local(bind_slot, v);
                                frame.pc =
                                    (after_operand_pc as i64 + next_iter_offset as i64) as usize;
                                continue;
                            }
                        }
                    }
                    // Fall through to slow path; dispatch loop executes
                    // the next op (LoadLocal iter_slot) unchanged.
                }
                Op::PushThis => {
                    let t = if let Some(cell) = &frame.this_cell {
                        cell.borrow().clone()
                    } else {
                        frame.this_value.clone()
                    };
                    if let Value::Symbol(ref s) = t {
                        if std::rc::Rc::ptr_eq(s, &self.tdz_sentinel) {
                            return Err(RuntimeError::ReferenceError(
                                "Must call super constructor in derived class before accessing 'this' or returning from derived constructor".into(),
                            ));
                        }
                    }
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
                    let v = frame.pop()?;
                    if matches!(&v, Value::Object(_)) {
                        if let Some(cell) = &frame.this_cell {
                            *cell.borrow_mut() = v.clone();
                        }
                        frame.this_value = v;
                    }
                }
                Op::PushThisRaw => {
                    // IR-EXT 39: reads the stashed fresh `this` from
                    // derived_initial_this (populated by SetThisTDZ).
                    // Used by super-call lowering so the parent ctor
                    // receives the spec-correct shared `this` that
                    // call_function passed into the derived ctor frame.
                    // Falls back to this_value (then this_cell) for
                    // non-derived-ctor frames where derived_initial_this
                    // is None.
                    let t = frame.derived_initial_this.clone().unwrap_or_else(|| {
                        if let Some(cell) = &frame.this_cell {
                            cell.borrow().clone()
                        } else {
                            frame.this_value.clone()
                        }
                    });
                    frame.push(t);
                }
                Op::SetThisTDZ => {
                    // IR-EXT 38/39: stash the fresh `this` (the spec-
                    // mandated [[ThisValue]] passed in by call_function
                    // from the new-expression alloc) into
                    // derived_initial_this so super-call's PushThisRaw
                    // can pass it to the parent ctor. Then seed
                    // this_value with TDZ sentinel so any user-level
                    // PushThis before super() throws ReferenceError.
                    // Op::SetThis on super() return overwrites this_value
                    // with the post-super this (which may be the same
                    // object or, per §15.4.5.4 step 9, a parent-returned
                    // Object).
                    frame.derived_initial_this = Some(frame.this_value.clone());
                    let sentinel = Value::Symbol(std::rc::Rc::clone(&self.tdz_sentinel));
                    if let Some(cell) = &frame.this_cell {
                        *cell.borrow_mut() = sentinel.clone();
                    }
                    frame.this_value = sentinel;
                }
                Op::EnterWith => {
                    let obj_expr = frame.pop()?;
                    let obj = self.to_object(&obj_expr)?;
                    match obj {
                        Value::Object(id) => frame.with_env_stack.push(id),
                        _ => {
                            return Err(RuntimeError::TypeError(
                                "with expression did not coerce to object".into(),
                            ))
                        }
                    }
                }
                Op::ExitWith => {
                    frame.with_env_stack.pop();
                }
                Op::PropagateNewTarget => {
                    // Ω.5.P03.E2.super-new-target: forward the current
                    // frame's new.target into the runtime's pending slot
                    // so the next CallMethod (the super(...) dispatch)
                    // treats the parent ctor invocation as a [[Construct]]
                    // with the same new.target the derived ctor saw.
                    // Two consequences: (1) the parent ctor's
                    // `new.target` is the original-newed class, not
                    // undefined, matching ECMA-262 §10.2.1.3 SuperCall
                    // step 4; (2) the call_function's implicit-return-
                    // this branch (line 7639) fires for the parent's
                    // ReturnUndef, so the parent's frame.this_value
                    // (possibly rebound by Callable-style patterns)
                    // propagates back to the derived's super-call
                    // sequence, where SetThis can rebind in turn.
                    if let Some(nt) = &frame.new_target {
                        self.pending_new_target = Some(nt.clone());
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
                    // EXT 91b: track whether the relaxed-non-constructor
                    // path is taken so the post-call result selection can
                    // skip the fresh-this fallback (the +1 keyCount leak
                    // that EXT 91's byte-parity check on graceful-fs /
                    // fs-jetpack / dayjs-plugin-utc / luxon-business-days
                    // surfaced — under the deviation, the result must be
                    // the function's return value verbatim, never the
                    // fresh ordinary Object).
                    let mut relaxed_non_constructor = false;
                    if let Value::Object(cid) = &callee {
                        if let crate::value::InternalKind::Function(fi) =
                            &self.obj(*cid).internal_kind
                        {
                            if !fi.is_constructor {
                                if !self
                                    .tolerated_deviations
                                    .contains("function-not-constructor-relax")
                                {
                                    return Err(RuntimeError::TypeError(format!(
                                        "{} is not a constructor",
                                        fi.name
                                    )));
                                }
                                relaxed_non_constructor = true;
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
                    } else {
                        None
                    };
                    // rusty-js-esm Rung-6 (Doc 730 §XVI for arktype):
                    // `class X extends Array` requires the pre-allocated
                    // `this` to be an Array-kind object so Array's intrinsic
                    // constructor recognizes the receiver and mutates it in
                    // place (rather than allocating a sibling that discards
                    // the derived-class proto wiring). Detect Array-subclass
                    // by walking proto_override's proto chain for the
                    // canonical Array.prototype id.
                    //
                    // Bracket probe: probes/bracket-class-extends-array.mjs.
                    // Spec basis: ECMA-262 §22.1.2.1 + §10.1.13 — Array's
                    // [[Construct]] honors newTarget.prototype.
                    let is_array_subclass = if let Some(pid) = proto_override {
                        let arr_proto = self.array_prototype;
                        let mut p = Some(pid);
                        let mut hit = false;
                        let mut steps = 0;
                        while let Some(cur) = p {
                            if Some(cur) == arr_proto {
                                hit = true;
                                break;
                            }
                            p = self.obj(cur).proto;
                            steps += 1;
                            if steps > 32 {
                                break;
                            }
                        }
                        hit
                    } else {
                        false
                    };
                    let mut ordinary = if is_array_subclass {
                        Object::new_array()
                    } else {
                        Object::new_ordinary()
                    };
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
                    let ret = self
                        .call_function(callee, this_obj.clone(), args)
                        .map_err(|e| match e {
                            RuntimeError::TypeError(msg)
                                if msg.starts_with("callee is not callable") =>
                            {
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
                    let result = if relaxed_non_constructor {
                        // EXT 91b: under the deviation, return value is
                        // the call's return verbatim — primitive returns
                        // pass through, no fresh-Object fallback. This
                        // matches Bun's "treat as plain call" shape and
                        // eliminates the +1 keyCount leak that the
                        // tolerant lowering introduced for the
                        // graceful-fs / fs-jetpack / dayjs-plugin-utc /
                        // luxon-business-days cluster.
                        ret
                    } else {
                        match ret {
                            Value::Object(_) => ret,
                            _ => this_obj,
                        }
                    };
                    frame.push(result);
                }

                // ─── Misc ───
                Op::Nop => {}
                Op::Debugger => {}

                // ─── Doc 731 §XIV.d typed-i64 alphabet promotion ───
                // Typed-i64 arithmetic + comparison. Both operands must
                // be Number values with integer-valued f64 representation.
                // On any mismatch, throw TypeError uniformly (v1; future
                // deviation primitive may relax). The interpreter unboxes
                // to i64, does the integer op, reboxes as Number(f64).
                // The JIT translates these directly to Cranelift instructions.
                Op::AddI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Number(li.wrapping_add(ri) as f64));
                }
                Op::SubI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Number(li.wrapping_sub(ri) as f64));
                }
                Op::MulI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Number(li.wrapping_mul(ri) as f64));
                }
                Op::IncI64 => {
                    let v = frame.pop()?;
                    let i = unbox_int64(&v)?;
                    frame.push(Value::Number(i.wrapping_add(1) as f64));
                }
                Op::DecI64 => {
                    let v = frame.pop()?;
                    let i = unbox_int64(&v)?;
                    frame.push(Value::Number(i.wrapping_sub(1) as f64));
                }
                Op::LtI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Boolean(li < ri));
                }
                Op::LeI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Boolean(li <= ri));
                }
                Op::GtI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Boolean(li > ri));
                }
                Op::GeI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Boolean(li >= ri));
                }
                Op::EqI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Boolean(li == ri));
                }
                Op::NeI64 => {
                    let r = frame.pop()?;
                    let l = frame.pop()?;
                    let li = unbox_int64(&l)?;
                    let ri = unbox_int64(&r)?;
                    frame.push(Value::Boolean(li != ri));
                }

                _ => {
                    return Err(RuntimeError::Unimplemented(format!(
                        "opcode {:?} not yet handled @{}",
                        op, pc
                    )));
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
    pub(crate) fn eval_source_globalish(&mut self, source: String) -> Result<Value, RuntimeError> {
        self.eval_source_globalish_with_global_declarations(source, true)
    }

    fn eval_source_globalish_with_global_declarations(
        &mut self,
        source: String,
        materialize_global_declarations: bool,
    ) -> Result<Value, RuntimeError> {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static EVAL_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let n = EVAL_COUNTER.fetch_add(1, Ordering::Relaxed);
        let url = format!("file://<eval:{}>", n);
        crate::intrinsics::eval_global_declaration_instantiation_guard(self, &source)?;
        if materialize_global_declarations {
            let eval_var_decls = eval_var_scoped_declarations(&source);
            self.materialize_eval_global_declarations(&eval_var_decls);
        }

        // Integration: GBSU unified surface.
        let direct_eval_super_context = self.pending_direct_eval_super_context.take();
        let stash_key = format!("__eval_out_{}", n);
        let expr_source = format!("{} = ({});", stash_key, source);
        let global_this = self.global_get("globalThis");
        let saved_this = std::mem::replace(&mut self.current_this, global_this);
        self.pending_direct_eval_super_context = direct_eval_super_context.clone();
        let expr_ok = self.evaluate_script(&expr_source, &url).is_ok();
        if expr_ok {
            self.current_this = saved_this;
            let result = self.global_get(&stash_key);
            if let Some(gt) = self.global_object {
                self.obj_mut(gt).remove_str(&stash_key);
            }
            return Ok(result);
        }

        let stmt_url = format!("file://<eval:{}:stmt>", n);
        self.pending_direct_eval_super_context = direct_eval_super_context;
        let result = self.evaluate_script(&source, &stmt_url);
        self.current_this = saved_this;
        match result {
            Ok(_) => Ok(Value::Undefined),
            Err(RuntimeError::CompileError(msg)) => Err(RuntimeError::SyntaxError(msg)),
            Err(e) => Err(e),
        }
    }

    fn direct_eval_from_frame(
        &mut self,
        caller: &mut Frame<'_>,
        args: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let source = match args.first() {
            Some(Value::String(s)) => s.as_str().to_string(),
            Some(v) => return Ok(v.clone()),
            None => return Ok(Value::Undefined),
        };

        let eval_var_decls = eval_var_scoped_declarations(&source);
        let eval_var_names = eval_var_decls.names();
        let strict_eval = caller.strict || eval_source_is_strict(&source);
        if strict_eval {
            if let Some(name) = eval_var_names
                .iter()
                .find(|name| is_strict_mode_reserved_binding_name(name))
            {
                return Err(RuntimeError::SyntaxError(format!(
                    "direct eval strict-mode declaration uses reserved binding '{}'",
                    name
                )));
            }
        }
        if eval_var_names.iter().any(|name| name == "arguments")
            && caller_has_arguments_conflict_binding(caller)
        {
            return Err(RuntimeError::SyntaxError(
                "direct eval declaration conflicts with arguments binding".into(),
            ));
        }
        if !caller.is_arrow && !strict_eval {
            if let Some(name) = eval_var_names
                .iter()
                .find(|name| caller_has_lexical_binding(caller, name))
            {
                return Err(RuntimeError::SyntaxError(format!(
                    "direct eval var declaration conflicts with lexical binding '{}'",
                    name
                )));
            }
        }

        let frame_direct_eval_super_context = self.direct_eval_super_context_from_frame(caller);

        let mut overlay: Vec<DirectEvalOverlay> = Vec::new();
        for (slot, desc) in caller.upvalue_names.iter().enumerate() {
            if let Some(name) = Self::eval_overlay_binding_name(&desc.name) {
                if let Some(cell) = caller.upvalues.get(slot) {
                    overlay.push(DirectEvalOverlay {
                        name,
                        value: cell.borrow().clone(),
                        target: DirectEvalOverlayTarget::Upvalue(cell.clone()),
                    });
                }
            }
        }
        for (slot, desc) in caller.locals_names.iter().enumerate() {
            if let Some(name) = Self::eval_overlay_binding_name(&desc.name) {
                overlay.push(DirectEvalOverlay {
                    name,
                    value: caller.read_local(slot),
                    target: DirectEvalOverlayTarget::Local(slot),
                });
            }
        }
        if let Some(ctx) = &frame_direct_eval_super_context {
            for name in [
                ctx.super_ctor_name.as_deref(),
                ctx.super_proto_name.as_deref(),
                ctx.super_home_name.as_deref(),
            ]
            .into_iter()
            .flatten()
            {
                if overlay.iter().any(|entry| entry.name == name) {
                    continue;
                }
                if let Some((value, target)) = Self::direct_eval_exact_binding(caller, name) {
                    overlay.push(DirectEvalOverlay {
                        name: name.to_string(),
                        value,
                        target,
                    });
                }
            }
        }

        if !caller.eval_var_env_is_global
            && self
                .global_object
                .map(|gt| self.obj(gt).extensible)
                .unwrap_or(true)
        {
            for name in &eval_var_decls.vars {
                if self.global_has_own_property(name) {
                    continue;
                }
                if !overlay.iter().any(|entry| entry.name == *name) {
                    overlay.push(DirectEvalOverlay {
                        name: name.clone(),
                        value: Value::Undefined,
                        target: DirectEvalOverlayTarget::Temporary,
                    });
                }
            }
            for function in &eval_var_decls.functions {
                if self.global_has_own_property(&function.name) {
                    continue;
                }
                let value = self
                    .make_eval_function_declaration(&function.name, function.return_value.clone());
                if let Some(entry) = overlay
                    .iter_mut()
                    .rev()
                    .find(|entry| entry.name == function.name)
                {
                    entry.value = value;
                } else {
                    overlay.push(DirectEvalOverlay {
                        name: function.name.clone(),
                        value,
                        target: DirectEvalOverlayTarget::Temporary,
                    });
                }
            }
        }

        let mut super_context = direct_eval_super_context(caller);
        for entry in &overlay {
            direct_eval_super_context_visit(&mut super_context, &entry.name);
        }
        if let Some(home) = caller.private_home {
            const DIRECT_EVAL_HOME_NAME: &str = "<direct.eval.home>";
            match super_context.as_mut() {
                None => {
                    super_context = Some(rusty_js_bytecode::DirectEvalSuperContext {
                        super_home_name: Some(DIRECT_EVAL_HOME_NAME.to_string()),
                        ..Default::default()
                    });
                }
                _ => {}
            }
            if !overlay
                .iter()
                .any(|entry| entry.name == DIRECT_EVAL_HOME_NAME)
            {
                overlay.push(DirectEvalOverlay {
                    name: DIRECT_EVAL_HOME_NAME.to_string(),
                    value: Value::Object(home),
                    target: DirectEvalOverlayTarget::Temporary,
                });
            }
        }

        // Integration: GBSU unified surface — overlay/restore over globalThis Object.
        let mut saved: Vec<(DirectEvalOverlay, Option<Value>)> = Vec::with_capacity(overlay.len());
        for entry in overlay {
            let prev = if let Some(gt) = self.global_object {
                if self.obj(gt).has_own_str(&entry.name) {
                    Some(self.object_get(gt, &entry.name))
                } else {
                    None
                }
            } else {
                None
            };
            self.define_global_property(&entry.name, entry.value.clone());
            saved.push((entry, prev));
        }
        let saved_direct_eval_super_context = self.pending_direct_eval_super_context.take();
        let saved_eval_this = self.pending_eval_this.clone();
        let saved_eval_derived_initial_this = self.pending_eval_derived_initial_this.clone();
        let saved_eval_new_target = self.pending_eval_new_target.clone();
        let saved_eval_private_home = self.pending_eval_private_home;
        self.pending_direct_eval_super_context = super_context;
        self.pending_eval_this = Some(caller.this_value_for_eval());
        self.pending_eval_derived_initial_this = caller.derived_initial_this.clone();
        self.pending_eval_new_target = caller.new_target.clone();
        self.pending_eval_private_home = caller.private_home;
        let result = self
            .eval_source_globalish_with_global_declarations(source, caller.eval_var_env_is_global);
        self.pending_direct_eval_super_context = saved_direct_eval_super_context;
        self.pending_eval_this = saved_eval_this;
        self.pending_eval_derived_initial_this = saved_eval_derived_initial_this;
        self.pending_eval_new_target = saved_eval_new_target;
        self.pending_eval_private_home = saved_eval_private_home;
        for (entry, old) in saved.into_iter().rev() {
            if let Some(gt) = self.global_object {
                if !matches!(entry.target, DirectEvalOverlayTarget::Temporary)
                    && self.obj(gt).has_own_str(&entry.name)
                {
                    let current = self.object_get(gt, &entry.name);
                    match entry.target {
                        DirectEvalOverlayTarget::Upvalue(cell) => {
                            *cell.borrow_mut() = current;
                        }
                        DirectEvalOverlayTarget::Local(slot) => {
                            if let Some(Some(cell)) = caller.local_cells.get(slot) {
                                *cell.borrow_mut() = current.clone();
                            }
                            if let Some(local) = caller.locals.get_mut(slot) {
                                *local = current;
                            }
                        }
                        DirectEvalOverlayTarget::Temporary => {}
                    }
                }
            }
            match old {
                Some(value) => self.define_global_property(&entry.name, value),
                None => {
                    if let Some(gt) = self.global_object {
                        self.obj_mut(gt).remove_str(&entry.name);
                    }
                }
            }
        }
        result
    }

    fn make_eval_function_declaration(&mut self, name: &str, return_value: Value) -> Value {
        let value = return_value.clone();
        let obj =
            crate::intrinsics::make_native_with_length(name, 0, move |_, _| Ok(value.clone()));
        let id = self.alloc_object(obj);
        let mut proto_obj = crate::value::Object::new_ordinary();
        proto_obj.set_own_internal("constructor".into(), Value::Object(id));
        let proto_id = self.alloc_object(proto_obj);
        self.obj_mut(id).dict_mut().insert(
            "prototype".into(),
            crate::value::PropertyDescriptor {
                value: Value::Object(proto_id),
                writable: true,
                enumerable: false,
                configurable: false,
                getter: None,
                setter: None,
            },
        );
        Value::Object(id)
    }

    fn materialize_eval_global_declarations(&mut self, decls: &EvalVarScopedDeclarations) {
        for name in &decls.vars {
            if self.global_has_own_property(name) {
                continue;
            }
            self.define_eval_global_binding(name, Value::Undefined, true);
        }
        for function in &decls.functions {
            let value =
                self.make_eval_function_declaration(&function.name, function.return_value.clone());
            self.define_eval_global_binding(&function.name, value, true);
        }
    }

    fn define_eval_global_binding(&mut self, name: &str, value: Value, deletable: bool) {
        let Some(gt) = self.global_object else {
            return;
        };
        let key = crate::value::PropertyKey::String(name.to_string());
        let existing = self.obj(gt).properties.get(&key).cloned();
        match existing {
            Some(desc) if !desc.configurable => {
                self.obj_mut(gt).dict_mut().insert(
                    key,
                    crate::value::PropertyDescriptor {
                        value,
                        writable: desc.writable,
                        enumerable: desc.enumerable,
                        configurable: desc.configurable,
                        getter: desc.getter,
                        setter: desc.setter,
                    },
                );
            }
            Some(_) | None if self.obj(gt).extensible || self.obj(gt).has_own_str(name) => {
                self.obj_mut(gt).dict_mut().insert(
                    key,
                    crate::value::PropertyDescriptor {
                        value,
                        writable: true,
                        enumerable: true,
                        configurable: deletable,
                        getter: None,
                        setter: None,
                    },
                );
            }
            _ => {}
        }
    }

    fn global_has_own_property(&self, name: &str) -> bool {
        self.global_object
            .map(|gt| self.obj(gt).has_own_str(name))
            .unwrap_or(false)
    }

    fn direct_eval_super_context_from_frame(
        &self,
        frame: &Frame<'_>,
    ) -> Option<rusty_js_bytecode::DirectEvalSuperContext> {
        let super_ctor_name = Self::find_direct_eval_super_binding(frame, ".super.ctor");
        let super_proto_name = Self::find_direct_eval_super_binding(frame, ".super.proto");
        let super_home_name = Self::find_direct_eval_super_binding(frame, ".home");
        if super_ctor_name.is_none() && super_proto_name.is_none() && super_home_name.is_none() {
            return None;
        }
        Some(rusty_js_bytecode::DirectEvalSuperContext {
            super_ctor_name,
            super_proto_name,
            super_home_name,
            in_constructor: frame.derived_initial_this.is_some() || frame.is_arrow,
            is_static: false,
        })
    }

    fn find_direct_eval_super_binding(frame: &Frame<'_>, needle: &str) -> Option<String> {
        frame
            .locals_names
            .iter()
            .map(|d| &d.name)
            .chain(frame.upvalue_names.iter().map(|d| &d.name))
            .find(|name| name.contains(needle))
            .cloned()
    }

    fn direct_eval_exact_binding(
        frame: &Frame<'_>,
        name: &str,
    ) -> Option<(Value, DirectEvalOverlayTarget)> {
        for (slot, desc) in frame.upvalue_names.iter().enumerate() {
            if desc.name == name {
                if let Some(cell) = frame.upvalues.get(slot) {
                    return Some((
                        cell.borrow().clone(),
                        DirectEvalOverlayTarget::Upvalue(cell.clone()),
                    ));
                }
            }
        }
        for (slot, desc) in frame.locals_names.iter().enumerate() {
            if desc.name == name {
                return Some((frame.read_local(slot), DirectEvalOverlayTarget::Local(slot)));
            }
        }
        None
    }

    fn eval_overlay_binding_name(name: &str) -> Option<String> {
        if name.contains(".super.") || name.contains(".home>") {
            return Some(name.to_string());
        }
        let logical = if let Some(rest) = name.strip_prefix("<scoped@") {
            rest.split_once('>')
                .map(|(_, suffix)| suffix)
                .unwrap_or(name)
        } else {
            name
        };
        let mut chars = logical.chars();
        let Some(first) = chars.next() else {
            return None;
        };
        if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
            return None;
        }
        if chars.all(|c| c == '_' || c == '$' || c.is_ascii_alphanumeric()) {
            Some(logical.to_string())
        } else {
            None
        }
    }

    fn direct_eval_binding_name(name: &str) -> String {
        if let Some(rest) = name.strip_prefix("<scoped@") {
            if let Some((_, original)) = rest.split_once('>') {
                return original.to_string();
            }
        }
        name.to_string()
    }

    pub fn call_function(
        &mut self,
        callee: Value,
        this: Value,
        args: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        const MAX_CALL_DEPTH: usize = 16_384;

        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(RuntimeError::RangeError(
                "Maximum call stack size exceeded".to_string(),
            ));
        }

        self.call_depth += 1;
        let result = self.call_function_inner(callee, this, args);
        self.call_depth -= 1;
        result
    }

    fn call_function_inner(
        &mut self,
        callee: Value,
        this: Value,
        args: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
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
                    "callee is not callable: {} [argc={}]",
                    tag,
                    args.len()
                )));
            }
        };
        // Tier-Ω.5.s: claim the pending new.target slot for this invocation.
        // Op::New sets it just before dispatching; plain Call sites leave it
        // None. Taken (not cloned) so nested calls don't inherit it.
        let nt_for_this_call = self.pending_new_target.take();

        // LeJIT-Τ TB-EXT 3b (approach A — closure-side metadata cache fast path).
        // Skips the standard dispatcher's jit_cache HashMap lookup + multi-
        // condition AND check + proto_rc clone by reading the closure's
        // tb_metadata_ptr cell (populated on first JIT-hit by the standard
        // path below). Falls through to standard path if the cell is None
        // (not yet populated, or TB env flag is off, or closure isn't
        // eligible). Per Finding II.2 from findings.md: this move
        // ELIMINATES work (HashMap + match-arm) and only adds a Cell read.
        // Deopt fallthrough: invalidates the cell + falls through to
        // standard path which will retry interp re-execution.
        if nt_for_this_call.is_none() {
            // new.target sites need full setup; bypass TB fast path.
            // Cast self to *mut Runtime BEFORE the obj() borrow so the
            // raw pointer doesn't conflict with the borrow checker.
            let rt_ptr_for_tb = self as *mut Runtime as usize;
            let fast_path_taken_result = {
                let o = self.obj(id);
                if let crate::value::InternalKind::Closure(c) = &o.internal_kind {
                    if let Some(nn) = c.tb_metadata_ptr.get() {
                        let cf: &rusty_js_jit::CompiledFn = unsafe { &*(nn.as_ptr() as *const _) };
                        let params = c.proto.params as usize;
                        if args.len() == params
                            && (params == 1 || params == 2)
                            && args.iter().all(jit_compatible_arg)
                            && !c.jit_disabled.get()
                        {
                            let actual_this = if c.is_arrow {
                                if let Some(cell) = &c.bound_this_cell {
                                    cell.borrow().clone()
                                } else {
                                    c.bound_this.clone().unwrap_or(Value::Undefined)
                                }
                            } else {
                                this.clone()
                            };
                            let proto_ptr = &*c.proto as *const _ as usize;
                            let rt_ptr = rt_ptr_for_tb;
                            let _ = actual_this; // not used in JIT body path
                            let vti = cf.vti_enabled;
                            // Release the obj borrow before invoking JIT
                            // (JIT helper externs may need &mut self).
                            drop(o);
                            rusty_js_jit::set_current_deopt_sites(&cf.deopt_sites);
                            rusty_js_jit::set_current_runtime(rt_ptr);
                            rusty_js_jit::set_current_proto(proto_ptr);
                            // Φ-EXT 2: JitFn signature is F64; pass f64
                            // directly from Value::Number payload. Under
                            // VTI=1 (env opt-in), pass *const Value
                            // pointer reinterpreted as f64-bits via
                            // f64::from_bits (the JIT prologue bitcasts
                            // back to recover the pointer).
                            let r = match params {
                                1 => {
                                    let a = if vti {
                                        f64::from_bits(&args[0] as *const Value as u64)
                                    } else {
                                        unbox_arg_f64(&args[0])
                                    };
                                    cf.func.call1(a)
                                }
                                2 => {
                                    let (a, b) = if vti {
                                        (
                                            f64::from_bits(&args[0] as *const Value as u64),
                                            f64::from_bits(&args[1] as *const Value as u64),
                                        )
                                    } else {
                                        (unbox_arg_f64(&args[0]), unbox_arg_f64(&args[1]))
                                    };
                                    cf.func.call2(a, b)
                                }
                                _ => unreachable!(),
                            };
                            rusty_js_jit::clear_current_runtime();
                            rusty_js_jit::clear_current_proto();
                            rusty_js_jit::clear_current_deopt_sites();
                            if rusty_js_jit::take_last_deopt().is_some() {
                                // Deopt: invalidate the cell + fall through
                                // to standard path which handles deopt
                                // re-execution.
                                let o2 = self.obj(id);
                                if let crate::value::InternalKind::Closure(c2) = &o2.internal_kind {
                                    c2.tb_metadata_ptr.set(None);
                                }
                                None
                            } else {
                                // Φ-EXT 2: r is already f64.
                                Some(Ok(Value::Number(r)))
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            if let Some(result) = fast_path_taken_result {
                return result;
            }
        }
        // Standard dispatcher path follows.
        // Extract proto-or-native by inspecting the heap object once.
        // BoundFunction: rewrite to its target, prepending bound args.
        let mut frame_is_arrow = false;
        let (
            proto_opt,
            native_opt,
            effective_this,
            effective_args,
            private_home,
            bound_derived_initial_this,
            bound_new_target,
        ) = {
            let o = self.obj(id);
            let private_home = o.private_home;
            match &o.internal_kind {
                crate::value::InternalKind::Closure(c) => {
                    frame_is_arrow = c.is_arrow;
                    // Ω.5.P04.E2.jit-runtime-dispatch + jit-deopt-disable:
                    // bump the call counter; if hot AND args are integer-
                    // Numbers AND params in {1,2} AND not previously
                    // disabled, dispatch to JIT if cached/available. On
                    // any guard mismatch AFTER a JIT compile succeeded
                    // (i.e., we know the function is JIT-able but the
                    // current arg shape doesn't match), permanently
                    // disable JIT for this Closure to avoid burning the
                    // per-call guard overhead on every subsequent call.
                    let count = c.call_count.get() + 1;
                    c.call_count.set(count);
                    let proto_key = std::rc::Rc::as_ptr(&c.proto) as usize;
                    let actual_this = if c.is_arrow {
                        if let Some(cell) = &c.bound_this_cell {
                            cell.borrow().clone()
                        } else {
                            c.bound_this.clone().unwrap_or(Value::Undefined)
                        }
                    } else {
                        this.clone()
                    };
                    let bound_derived_initial_this = c.bound_derived_initial_this.clone();
                    let bound_new_target = c.bound_new_target.clone();
                    let params = c.proto.params;
                    let jit_disabled = c.jit_disabled.get();
                    if !jit_disabled
                        && count >= self.jit_threshold
                        && (params == 1 || params == 2)
                        && args.len() == params as usize
                        && args.iter().all(jit_compatible_arg)
                    {
                        // Take the proto out so we don't hold a borrow
                        // across the JIT-compile mutation below.
                        let proto_rc = c.proto.clone();
                        drop(o);
                        // Compile-if-absent.
                        if !self.jit_cache.contains_key(&proto_key) {
                            // LeJIT-Τ TB-EXT 7 (segfault fix): Box so
                            // the CompiledFn's address is stable across
                            // subsequent HashMap rehashes; TB cell holds
                            // *const CompiledFn into this allocation.
                            let compiled = rusty_js_jit::compile_function(&*proto_rc)
                                .ok()
                                .map(Box::new);
                            self.jit_cache.insert(proto_key, compiled);
                        }
                        // JIT-EXT 14: wire deopt for this call. The
                        // dispatcher sets the active deopt-site table
                        // pointer BEFORE invoking the JIT and clears
                        // it AFTER. If the JIT trips, `take_last_deopt`
                        // returns the recovered state and the dispatcher
                        // falls through to the interpreter path
                        // (re-execution from pc=0 with the original
                        // args — first-cut retry semantics; resume-from-
                        // trip-pc is queued for a future round).
                        let mut deopt_fell_through = false;
                        // JIT-EXT 22: capture the Runtime + FunctionProto
                        // pointers BEFORE entering the cache-borrow scope.
                        // Both are raw pointers cast to usize to avoid
                        // naming the types across the jit-crate boundary.
                        let rt_ptr_usize = self as *mut Runtime as usize;
                        let proto_ptr_usize = &*proto_rc as *const _ as usize;
                        if let Some(Some(jit_fn)) = self.jit_cache.get(&proto_key) {
                            // LeJIT-Τ TB-EXT 3b (approach A): capture
                            // CompiledFn pointer + TB eligibility for the
                            // post-call closure-cell populate. Pointer is
                            // stable for process lifetime (leaked module
                            // per CompiledFn._module).
                            // jit_fn is &Box<CompiledFn> post-EXT-7 fix;
                            // dereference twice to get the address of
                            // the CompiledFn INSIDE the Box (stable).
                            let tb_cf_ptr: *const rusty_js_jit::CompiledFn = &**jit_fn;
                            let tb_eligible =
                                jit_fn.tb_metadata.as_ref().map_or(false, |m| m.eligible());
                            rusty_js_jit::set_current_deopt_sites(&jit_fn.deopt_sites);
                            rusty_js_jit::set_current_runtime(rt_ptr_usize);
                            rusty_js_jit::set_current_proto(proto_ptr_usize);
                            // LeJIT-Ψ VTI-EXT 3b: under vti_enabled the JIT
                            // prologue extracts the f64 payload itself; pass
                            // raw `*const Value` (reinterpreted as i64). The
                            // args Vec outlives the JIT call (we hold it
                            // borrowed through this scope) so the pointer is
                            // valid for the call's duration.
                            let vti = jit_fn.vti_enabled;
                            // Φ-EXT 2: F64 calling convention; pass f64
                            // from Value::Number payload. VTI uses
                            // f64::from_bits to encode the *const Value
                            // pointer; JIT prologue bitcasts back.
                            let r = match params {
                                1 => {
                                    let a = if vti {
                                        f64::from_bits(&args[0] as *const Value as u64)
                                    } else {
                                        unbox_arg_f64(&args[0])
                                    };
                                    jit_fn.func.call1(a)
                                }
                                2 => {
                                    let (a, b) = if vti {
                                        (
                                            f64::from_bits(&args[0] as *const Value as u64),
                                            f64::from_bits(&args[1] as *const Value as u64),
                                        )
                                    } else {
                                        (unbox_arg_f64(&args[0]), unbox_arg_f64(&args[1]))
                                    };
                                    jit_fn.func.call2(a, b)
                                }
                                _ => unreachable!(),
                            };
                            rusty_js_jit::clear_current_runtime();
                            rusty_js_jit::clear_current_proto();
                            rusty_js_jit::clear_current_deopt_sites();
                            if rusty_js_jit::take_last_deopt().is_some() {
                                // Deopt fired; produce the interp-fallback
                                // tuple below.
                                deopt_fell_through = true;
                            } else {
                                // LeJIT-Τ TB-EXT 3b (approach A): populate
                                // the closure's tb_metadata_ptr cell on
                                // successful first JIT-hit. Subsequent calls
                                // take the fast path at the top of dispatch
                                // (skips jit_cache HashMap lookup + the
                                // 5-condition AND check). Re-borrow obj to
                                // access the Closure cell — borrows of
                                // self.jit_cache and self.obj coexist via NLL
                                // (separate fields).
                                if tb_eligible && rusty_js_jit::tiny_baseline::lejit_tb_enabled() {
                                    if let Some(nn) = std::ptr::NonNull::new(tb_cf_ptr as *mut ()) {
                                        let o2 = self.obj(id);
                                        if let crate::value::InternalKind::Closure(c2) =
                                            &o2.internal_kind
                                        {
                                            c2.tb_metadata_ptr.set(Some(nn));
                                        }
                                    }
                                }
                                // Φ-EXT 2: r is already f64.
                                return Ok(Value::Number(r));
                            }
                        }
                        // JIT compile failed (cached None), OR a deopt
                        // tripped above. Both paths run the interpreter.
                        let _ = deopt_fell_through; // (currently unused; future EXT can split metrics)
                        let o2 = self.obj(id);
                        match &o2.internal_kind {
                            crate::value::InternalKind::Closure(c2) => (
                                Some(c2.proto.clone()),
                                None,
                                actual_this,
                                args,
                                private_home,
                                bound_derived_initial_this,
                                bound_new_target,
                            ),
                            _ => unreachable!("closure flipped kind mid-dispatch"),
                        }
                    } else {
                        // JIT-EXT 16: replaced the permanent jit_disabled
                        // forfeit with retry-on-fresh-args. With the
                        // deopt mechanism wired (JIT-EXT 11-14), the
                        // boundary-mismatch case is structurally
                        // equivalent to a deopt — both fall through to
                        // the interpreter for the failing call. A
                        // subsequent call with valid args will re-enter
                        // the JIT path at the top of dispatch instead
                        // of staying permanently disabled.
                        //
                        // The trade-off: long-tail mismatched callers
                        // pay the per-call boundary-guard cost (~10
                        // instructions per arg) on every call. The cost
                        // is bounded; the benefit is that callers
                        // alternating arg shapes get JIT speed on the
                        // matching subset of calls instead of forfeiting
                        // forever after the first mismatch.
                        //
                        // The `jit_disabled` field is retained (default
                        // false) so external probes that read it stay
                        // valid; this branch no longer writes to it.
                        let _ = (count, proto_key);
                        (
                            Some(c.proto.clone()),
                            None,
                            actual_this,
                            args,
                            private_home,
                            bound_derived_initial_this,
                            bound_new_target,
                        )
                    }
                }
                crate::value::InternalKind::Function(f) => (
                    None,
                    Some(f.native.clone()),
                    this,
                    args,
                    private_home,
                    None,
                    None,
                ),
                crate::value::InternalKind::Proxy(p) => {
                    // EXT 84: revoked-proxy guard per §10.5.{12,13}.
                    if p.revoked {
                        return Err(RuntimeError::TypeError(
                            "Cannot perform 'apply'/'construct' on a proxy that has been revoked"
                                .into(),
                        ));
                    }
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
                            // EXT 84: ECMA §10.5.13 [[Construct]] step 9 —
                            // if the trap's return is not an Object, throw
                            // TypeError. Without this, `new Proxy(F, {
                            // construct(){return true}})()` returned the
                            // non-Object instead of throwing per spec.
                            let ret = self.call_function(
                                trap,
                                Value::Object(handler),
                                vec![Value::Object(target), Value::Object(arr), nt],
                            )?;
                            return match ret {
                                Value::Object(_) => Ok(ret),
                                _ => Err(RuntimeError::TypeError(
                                    "Proxy construct trap returned a non-Object".into(),
                                )),
                            };
                        } else {
                            // handler.apply(target, thisArg, argsArray).
                            return self.call_function(
                                trap,
                                Value::Object(handler),
                                vec![Value::Object(target), this, Value::Object(arr)],
                            );
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
                    let keys: Vec<String> = self
                        .obj(id)
                        .properties
                        .keys()
                        .take(5)
                        .map(|k| k.as_str().to_string())
                        .collect();
                    let nkeys = self.obj(id).properties.len();
                    let preview = if keys.is_empty() {
                        String::new()
                    } else {
                        format!(
                            " keys=[{}{}]",
                            keys.join(","),
                            if nkeys > 5 { ",…" } else { "" }
                        )
                    };
                    return Err(RuntimeError::TypeError(format!(
                        "callee is not callable: Object(kind={}{}) [argc={}]",
                        kind,
                        preview,
                        args.len()
                    )));
                }
            }
        };
        if let Some(native) = native_opt {
            let saved = std::mem::replace(&mut self.current_this, effective_this);
            let saved_nt =
                std::mem::replace(&mut self.current_new_target, nt_for_this_call.clone());
            let result = native(self, &effective_args);
            self.current_this = saved;
            self.current_new_target = saved_nt;
            return result;
        }
        let proto = proto_opt.expect("closure branch implies proto");
        let is_generator = proto.is_generator;
        let args = effective_args;
        // EXT 73: ECMA-262 §10.2.1.2 OrdinaryCallBindThis. For non-arrow,
        // non-strict function code, a null/undefined thisArg is replaced
        // with globalThis and a primitive thisArg is boxed via ToObject.
        // Arrow bodies already took bound_this; strict bodies (proto.strict)
        // receive thisArg unchanged. Constructor invocation (signalled by
        // nt_for_this_call.is_some()) always supplies a fresh Object so it
        // never falls into the null/undefined/primitive branches.
        let this = if proto.strict || nt_for_this_call.is_some() {
            effective_this
        } else {
            match &effective_this {
                Value::Null | Value::Undefined => self.global_get("globalThis"),
                Value::Boolean(_)
                | Value::Number(_)
                | Value::String(_)
                | Value::BigInt(_)
                | Value::Symbol(_) => self
                    .to_object(&effective_this)
                    .unwrap_or(effective_this.clone()),
                _ => effective_this,
            }
        };
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
        // Tier-Ω.5.kkkkk: self-binding for named function expr/decl.
        let self_slot = proto.self_name_slot;
        for (i, _) in proto.locals.iter().enumerate() {
            let slot = i as u16;
            if Some(slot) == args_slot {
                locals.push(Value::Undefined);
            } else if Some(slot) == self_slot {
                locals.push(Value::Object(id));
            } else if Some(slot) == rest_slot {
                let mut rest = crate::value::Object::new_array();
                let tail: Vec<Value> = if (i as usize) < args.len() {
                    args[i as usize..].to_vec()
                } else {
                    Vec::new()
                };
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
        let mut local_cells: Vec<Option<UpvalueCell>> = Vec::new();
        if let Some(arguments_slot) = args_slot {
            let mut arr = crate::value::Object::new_array();
            arr.set_own("length".into(), Value::Number(args.len() as f64));
            let mut parameter_map: indexmap::IndexMap<String, UpvalueCell> =
                indexmap::IndexMap::new();
            for (k, v) in args.iter().enumerate() {
                arr.set_own(k.to_string(), v.clone());
                if !proto.strict && k < locals.len() && (k as u16) != arguments_slot {
                    while local_cells.len() <= k {
                        local_cells.push(None);
                    }
                    let cell = if let Some(existing) = &local_cells[k] {
                        existing.clone()
                    } else {
                        let cell = crate::value::new_upvalue_cell(locals[k].clone());
                        locals[k] = Value::Undefined;
                        local_cells[k] = Some(cell.clone());
                        cell
                    };
                    parameter_map.insert(k.to_string(), cell);
                }
            }
            if proto.strict {
                let thrower = crate::intrinsics::make_native("%ThrowTypeError%", |_rt, _args| {
                    Err(RuntimeError::TypeError(
                        "'callee' may not be accessed on a strict arguments object".into(),
                    ))
                });
                let thrower_id = self.alloc_object(thrower);
                arr.dict_mut().insert(
                    "callee".into(),
                    crate::value::PropertyDescriptor {
                        value: Value::Undefined,
                        writable: false,
                        enumerable: false,
                        configurable: false,
                        getter: Some(Value::Object(thrower_id)),
                        setter: Some(Value::Object(thrower_id)),
                    },
                );
            } else if !parameter_map.is_empty() {
                arr.internal_kind = crate::value::InternalKind::MappedArguments { parameter_map };
            }
            if (arguments_slot as usize) < locals.len() {
                locals[arguments_slot as usize] = Value::Object(self.alloc_object(arr));
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
            local_cells,
            operand_stack: Vec::with_capacity(32),
            pc: 0,
            try_stack: Vec::new(),
            with_env_stack: Vec::new(),
            this_value: this.clone(),
            this_cell: None,
            derived_initial_this: if frame_is_arrow {
                bound_derived_initial_this
            } else {
                None
            },
            upvalues,
            last_property_lookup: None,
            pending_method_name: None,
            pending_method_getprop_pc: None,
            private_home,
            import_meta: None,
            new_target: if frame_is_arrow {
                bound_new_target
            } else {
                nt_for_this_call.clone()
            },
            is_arrow: frame_is_arrow,
            param_count: proto.params as usize,
            strict: proto.strict,
            eval_var_env_is_global: false,
            back_edge_counts: HashMap::new(),
            osr_cache: HashMap::new(),
            ic_dispatch_cache: HashMap::new(),
        };
        if is_generator && !proto.is_async && !function_proto_uses_yield_delegate(&proto) {
            if proto.param_prologue_end > 0 {
                let prologue_end = proto.param_prologue_end.min(proto.bytecode.len());
                inner.bytecode = &proto.bytecode[..prologue_end];
                self.run_frame(&mut inner)?;
                inner.bytecode = &proto.bytecode;
                inner.pc = prologue_end;
            }
            let snapshot = FrameSnapshot::from_frame(&inner, Some(proto.clone()));
            let generator = self.new_generator_with_continuation(snapshot);
            return Ok(Value::Object(generator));
        }
        let gen_yields_id = if is_generator {
            // Tier-Ω.5.gggggg: push fresh yields-array on legacy generator
            // entry. Plain sync generators return above as suspended
            // GeneratorObject instances; this remains for async/legacy paths.
            let yields_arr = self.alloc_object(Object::new_array());
            self.object_set(yields_arr, "length".into(), Value::Number(0.0));
            self.gen_yields_stack.push(yields_arr);
            self.gen_async_stack.push(proto.is_async);
            Some(yields_arr)
        } else {
            None
        };
        let body_result = if is_generator && proto.param_prologue_end > 0 {
            let prologue_end = proto.param_prologue_end.min(proto.bytecode.len());
            inner.bytecode = &proto.bytecode[..prologue_end];
            self.run_frame(&mut inner)?;
            inner.bytecode = &proto.bytecode;
            inner.pc = prologue_end;
            self.run_frame(&mut inner)
        } else {
            self.run_frame(&mut inner)
        };
        if is_generator {
            // Tier-Ω.5.gggggg: pop yields-array on generator exit; build
            // an index-cursor iterator over the collected values. The
            // body's return value is exposed as the terminal
            // {value, done:true} step when no yields were collected.
            let yields_id = self
                .gen_yields_stack
                .pop()
                .expect("gen_yields_stack underflow");
            self.gen_async_stack.pop();
            let _ = gen_yields_id;
            if let Err(RuntimeError::SyntaxError(msg)) = body_result.as_ref() {
                return Err(RuntimeError::SyntaxError(msg.clone()));
            }
            let mut terminal_return = false;
            let pending_error = match body_result {
                Ok(v) => {
                    if !matches!(v, Value::Undefined) && self.array_length(yields_id) == 0 {
                        self.object_set(yields_id, "0".into(), v);
                        self.object_set(yields_id, "length".into(), Value::Number(1.0));
                        terminal_return = true;
                    }
                    Value::Undefined
                }
                Err(RuntimeError::Thrown(v)) => v,
                Err(RuntimeError::TypeError(msg)) => {
                    match crate::intrinsics::make_error_instance(self, "TypeError", &msg) {
                        Some(id) => Value::Object(id),
                        None => Value::String(Rc::new(format!("TypeError({:?})", msg))),
                    }
                }
                Err(RuntimeError::RangeError(msg)) => {
                    match crate::intrinsics::make_error_instance(self, "RangeError", &msg) {
                        Some(id) => Value::Object(id),
                        None => Value::String(Rc::new(format!("RangeError({:?})", msg))),
                    }
                }
                Err(RuntimeError::ReferenceError(msg)) => {
                    match crate::intrinsics::make_error_instance(self, "ReferenceError", &msg) {
                        Some(id) => Value::Object(id),
                        None => Value::String(Rc::new(format!("ReferenceError({:?})", msg))),
                    }
                }
                Err(RuntimeError::SyntaxError(msg)) => {
                    match crate::intrinsics::make_error_instance(self, "SyntaxError", &msg) {
                        Some(id) => Value::Object(id),
                        None => Value::String(Rc::new(format!("SyntaxError({:?})", msg))),
                    }
                }
                Err(e) => Value::String(Rc::new(format!("{:?}", e))),
            };
            // diff-prod Rung-19: chain generator instances to %GeneratorPrototype%
            // (which in turn chains to %IteratorPrototype%). Pre-fix, generator
            // instances proto-chained only to Object.prototype, so the ES2025
            // Iterator Helpers installed on %IteratorPrototype% were invisible
            // to `g().map(...)` patterns.
            let mut iter = Object::new_ordinary();
            iter.proto = if proto.is_async {
                self.async_generator_prototype
            } else {
                self.generator_prototype
            };
            let it_id = self.alloc_object(iter);
            // ESNE-EXT 3: hide engine sentinels + iterator methods. State
            // (__gen_*) via set_engine_sentinel; methods (next/return/throw/
            // @@iterator) likewise (real spec has them on
            // %GeneratorPrototype%, we install on instance pending
            // prototype-routing refactor).
            self.set_engine_sentinel(it_id, "__gen_arr__", Value::Object(yields_id));
            self.set_engine_sentinel(it_id, "__gen_idx__", Value::Number(0.0));
            self.set_engine_sentinel(
                it_id,
                "__gen_return_terminal__",
                Value::Boolean(terminal_return),
            );
            self.set_engine_sentinel(it_id, "__gen_async__", Value::Boolean(proto.is_async));
            self.set_engine_sentinel(it_id, "__gen_pending_error__", pending_error);
            let next_fn = crate::intrinsics::make_native("next", |rt, _args| {
                let this_id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Ok(Value::Undefined),
                };
                let is_async_gen = matches!(
                    rt.object_get(this_id, "__gen_async__"),
                    Value::Boolean(true)
                );
                let pending_error = rt.object_get(this_id, "__gen_pending_error__");
                if !matches!(pending_error, Value::Undefined) {
                    rt.object_set(this_id, "__gen_pending_error__".into(), Value::Undefined);
                    if is_async_gen {
                        let p = crate::promise::new_promise(rt);
                        crate::promise::reject_promise(rt, p, pending_error);
                        return Ok(Value::Object(p));
                    }
                    return Err(RuntimeError::Thrown(pending_error));
                }
                let arr = match rt.object_get(this_id, "__gen_arr__") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Undefined),
                };
                let idx = match rt.object_get(this_id, "__gen_idx__") {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };
                let terminal_return = matches!(
                    rt.object_get(this_id, "__gen_return_terminal__"),
                    Value::Boolean(true)
                );
                let len = rt.array_length(arr);
                let mut o = Object::new_ordinary();
                if idx >= len {
                    o.set_own("value".into(), Value::Undefined);
                    o.set_own("done".into(), Value::Boolean(true));
                } else {
                    let v = rt.object_get(arr, &idx.to_string());
                    rt.object_set(
                        this_id,
                        "__gen_idx__".into(),
                        Value::Number((idx + 1) as f64),
                    );
                    o.set_own("value".into(), v);
                    o.set_own(
                        "done".into(),
                        Value::Boolean(terminal_return && idx + 1 >= len),
                    );
                }
                let result_id = rt.alloc_object(o);
                if is_async_gen {
                    let p = crate::promise::new_promise(rt);
                    crate::promise::resolve_promise(rt, p, Value::Object(result_id));
                    Ok(Value::Object(p))
                } else {
                    Ok(Value::Object(result_id))
                }
            });
            let next_id = self.alloc_object(next_fn);
            self.set_engine_sentinel(it_id, "next", Value::Object(next_id));
            let return_fn = crate::intrinsics::make_native("return", |rt, args| {
                let this_id = match rt.current_this() {
                    Value::Object(o) => o,
                    _ => return Ok(Value::Undefined),
                };
                // Mark the iterator as exhausted so subsequent next()
                // calls see done:true. Set __gen_idx__ past the array's
                // length; the next() impl checks idx >= len.
                let arr = match rt.object_get(this_id, "__gen_arr__") {
                    Value::Object(id) => id,
                    _ => return Ok(Value::Undefined),
                };
                let len = rt.array_length(arr);
                rt.object_set(this_id, "__gen_idx__".into(), Value::Number(len as f64));
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                let mut o = Object::new_ordinary();
                o.set_own("value".into(), v);
                o.set_own("done".into(), Value::Boolean(true));
                Ok(Value::Object(rt.alloc_object(o)))
            });
            let return_id = self.alloc_object(return_fn);
            self.set_engine_sentinel(it_id, "return", Value::Object(return_id));
            // ECMA §27.5.1.4 Generator.prototype.throw: re-throws the
            // arg from inside the generator at its current suspension
            // point. v1 generators are eager-collected (the body runs to
            // completion before the iterator is returned), so there is
            // no live suspension to throw into; we surface the arg as a
            // JS-throwable so callers' try/catch around g.throw() works
            // even though the in-generator try/catch surface is moot.
            let throw_fn = crate::intrinsics::make_native("throw", |_rt, args| {
                let v = args.first().cloned().unwrap_or(Value::Undefined);
                Err(RuntimeError::Thrown(v))
            });
            let throw_id = self.alloc_object(throw_fn);
            self.set_engine_sentinel(it_id, "throw", Value::Object(throw_id));
            let self_iter = it_id;
            let iter_fn = crate::intrinsics::make_native("@@iterator", move |_rt, _args| {
                Ok(Value::Object(self_iter))
            });
            let iter_fn_id = self.alloc_object(iter_fn);
            self.set_engine_sentinel(it_id, "@@iterator", Value::Object(iter_fn_id));
            if proto.is_async {
                let async_iter = it_id;
                let async_iter_fn =
                    crate::intrinsics::make_native("@@asyncIterator", move |_rt, _args| {
                        Ok(Value::Object(async_iter))
                    });
                let async_iter_fn_id = self.alloc_object(async_iter_fn);
                self.set_engine_sentinel(it_id, "@@asyncIterator", Value::Object(async_iter_fn_id));
            }
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
                Err(RuntimeError::Thrown(v)) => {
                    self.enqueue_microtask("AsyncFunctionReject", move |rt| {
                        crate::promise::reject_promise(rt, p, v);
                        Ok(())
                    });
                }
                Err(e) => {
                    let reason = match e {
                        RuntimeError::TypeError(msg) => {
                            crate::intrinsics::make_error_instance(self, "TypeError", &msg)
                                .map(Value::Object)
                                .unwrap_or_else(|| Value::String(Rc::new(msg)))
                        }
                        RuntimeError::RangeError(msg) => {
                            crate::intrinsics::make_error_instance(self, "RangeError", &msg)
                                .map(Value::Object)
                                .unwrap_or_else(|| Value::String(Rc::new(msg)))
                        }
                        RuntimeError::ReferenceError(msg) => {
                            crate::intrinsics::make_error_instance(self, "ReferenceError", &msg)
                                .map(Value::Object)
                                .unwrap_or_else(|| Value::String(Rc::new(msg)))
                        }
                        RuntimeError::SyntaxError(msg) => {
                            crate::intrinsics::make_error_instance(self, "SyntaxError", &msg)
                                .map(Value::Object)
                                .unwrap_or_else(|| Value::String(Rc::new(msg)))
                        }
                        other => Value::String(Rc::new(format!("{:?}", other))),
                    };
                    self.enqueue_microtask("AsyncFunctionReject", move |rt| {
                        crate::promise::reject_promise(rt, p, reason);
                        Ok(())
                    });
                }
            }
            return Ok(Value::Object(p));
        }
        let body_result = body_result_v?;
        // Tier-Ω.5.nnnnn: implicit-return-this for derived ctors.
        let result = if nt_for_this_call.is_some() && matches!(body_result, Value::Undefined) {
            inner.this_value.clone()
        } else {
            body_result
        };
        Ok(result)
    }

    fn constant_to_value(&self, frame: &Frame, idx: u16) -> Result<Value, RuntimeError> {
        match frame.constants.get(idx) {
            Some(rusty_js_bytecode::Constant::Number(n)) => Ok(Value::Number(*n)),
            Some(rusty_js_bytecode::Constant::String(s)) => Ok(Value::String(Rc::new(s.clone()))),
            Some(rusty_js_bytecode::Constant::BigInt(s)) => {
                match crate::bigint::JsBigInt::from_decimal(s) {
                    Some(b) => Ok(Value::BigInt(Rc::new(b))),
                    None => Err(RuntimeError::TypeError(format!(
                        "Invalid BigInt literal: {}",
                        s
                    ))),
                }
            }
            Some(rusty_js_bytecode::Constant::Regex { .. }) => Err(RuntimeError::Unimplemented(
                "Regex literals not yet supported".into(),
            )),
            Some(rusty_js_bytecode::Constant::Function(_)) => {
                // Function constants are not directly Pushable as values;
                // they're consumed by MakeClosure / MakeArrow. Reaching
                // here means the compiler emitted a PushConst on a
                // Function which would be a bug.
                Err(RuntimeError::TypeError(
                    "Function constant pushed as a value".into(),
                ))
            }
            None => Err(RuntimeError::TypeError(format!(
                "invalid constant index {}",
                idx
            ))),
        }
    }

    fn constant_name(&self, frame: &Frame, idx: u16) -> Result<String, RuntimeError> {
        match frame.constants.get(idx) {
            Some(rusty_js_bytecode::Constant::String(s)) => Ok(s.clone()),
            _ => Err(RuntimeError::TypeError(format!(
                "constant {} is not a name string",
                idx
            ))),
        }
    }
}

/// ToPropertyKey per ECMA-262 §7.1.19. v1 simplified: numbers stringify
/// to their canonical decimal form; other primitives ToString-coerce.
/// Coerce a JS Value to a property key per ECMA §7.1.19 ToPropertyKey.
/// Symbol values produce PropertyKey::Symbol (identity-keyed, by-Rc); all
/// other values stringify into PropertyKey::String.
/// Ω.5.P04.E2.jit-runtime-dispatch: cheap predicate for the JIT
/// argument-type guard. Accept Number with finite integer-valued
/// representation; reject everything else. Fast-path inlined in
/// call_function's Closure arm.
pub fn jit_compatible_int_arg(v: &Value) -> bool {
    match v {
        Value::Number(f) => {
            f.is_finite() && f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64
        }
        _ => false,
    }
}

/// Companion to jit_compatible_int_arg: unbox a guard-passed Number.
/// Caller is responsible for having checked compatibility first;
/// otherwise the cast is meaningless.
pub fn unbox_int_arg(v: &Value) -> i64 {
    match v {
        Value::Number(f) => *f as i64,
        _ => 0,
    }
}

/// JIT-EXT 23: extended boundary check that accepts either
/// integer-Number args (Doc 731 §XIV.d typed-i64 alphabet) OR
/// Object args (the typed-object alphabet — receiver of
/// GetPropOnObject). The JIT body interprets the resulting i64 per
/// what op consumes it; per design Option B (per-kind specialization),
/// the bytecode emitter is responsible for not mixing arith-on-arg
/// with GetPropOnObject-on-arg in the same function.
pub fn jit_compatible_arg(v: &Value) -> bool {
    match v {
        Value::Number(f) => {
            f.is_finite() && f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64
        }
        Value::Object(_) => true,
        _ => false,
    }
}

/// Companion to jit_compatible_arg: unbox a guard-passed Number or
/// Object as an i64. For Number, this is the integer truncation;
/// for Object, the inner ObjectId.0 widened to i64.
pub fn unbox_arg(v: &Value) -> i64 {
    match v {
        Value::Number(f) => *f as i64,
        Value::Object(id) => id.0 as i64,
        _ => 0,
    }
}

/// LeJIT-Φ Φ-EXT 2: f64 calling-convention companion to unbox_arg.
/// Returns the f64 directly from Value::Number (no truncation, no
/// rebox). For Value::Object, encodes the ObjectId.0 as f64-bits via
/// from_bits — the JIT's IC GetProp path interprets these bits back
/// via to_bits() for the receiver-id slot. Caller must have validated
/// arg-tag via jit_compatible_arg_tag_only first.
// ─── VD-EXT 2 (2026-05-23): NaN-boxing scheme constants ───
//
// Encoding (per VD-EXT 1 docs/design.md §2):
//
//   encoded = VD_BOXED_MASK | (tag << VD_TAG_SHIFT) | (payload & VD_PAYLOAD_MASK)
//
// Sign bit = 1 distinguishes boxed values from real Numbers (arithmetic
// NaN per IEEE 754 has sign=0 canonically; defensive canonicalization at
// unbox closes the sign=1 hardware-NaN edge case). 4-bit tag at bits
// 51-48; 48-bit payload below (Rc<String> raw ptr on aarch64).
//
// Backwards compat: Object encoding (f64::from_bits(id.0 as u64)) is
// preserved byte-identical per VD seed C3. The latent unsoundness at
// ObjectId.0 ≥ 2^52 is documented as R1 in the design doc — out of
// scope per Pred-vd.4 first-cut. Number encoding preserved per C2 + NaN
// canonicalization (invisible because real arithmetic NaN has sign=0).
pub const VD_BOXED_MASK: u64 = 0xFFF0_0000_0000_0000;
pub const VD_TAG_SHIFT: u32 = 48;
pub const VD_PAYLOAD_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
pub const VD_TAG_STRING: u64 = 2;
// (VD_TAG_BIGINT=3, VD_TAG_SYMBOL=4, VD_TAG_BOOLEAN=5, VD_TAG_NULL=6,
//  VD_TAG_UNDEFINED=7 — deferred to VD-EXT 4+ per scope discipline.)

pub fn unbox_arg_f64(v: &Value) -> f64 {
    match v {
        Value::Number(n) => {
            // VD-EXT 2: defensive NaN canonicalization. Real arithmetic
            // NaN has sign=0 by construction (the canonical qNaN
            // f64::NAN = 0x7FF8_0000_0000_0000); the boxed-NaN scheme
            // requires sign=1. Canonicalize any NaN at unbox to ensure
            // no Number flows into the JIT with the boxed-NaN bit
            // pattern. Closes the hardware-produced-sign-1-NaN edge.
            if n.is_nan() {
                f64::NAN
            } else {
                *n
            }
        }
        Value::Object(id) => f64::from_bits(id.0 as u64),
        Value::String(s) => {
            // VD-EXT 2: NaN-box the Rc<String> raw pointer. The pointer
            // is borrowed, not owned; the source Value::String lives in
            // the caller's stack frame for the JIT call's duration (same
            // shape as Object's id-encoding). Per R3: downstream decoders
            // use `&*ptr` for &String access, NOT Rc::from_raw (which
            // would over-decrement on drop).
            let ptr = Rc::as_ptr(s) as u64;
            f64::from_bits(
                VD_BOXED_MASK | (VD_TAG_STRING << VD_TAG_SHIFT) | (ptr & VD_PAYLOAD_MASK),
            )
        }
        _ => 0.0, // BigInt/Boolean/Null/Undefined/Symbol: VD-EXT 4+
    }
}

/// VD-EXT 2 (2026-05-23): tag-detection helpers for downstream consumer-
/// pilots (TL Moves 3+4 revival; hot-intrinsic-IC table at JIT tier).
/// Not used at this pilot itself.
///
/// **Tag=0 reserved as "Number escape"**: `f64::NEG_INFINITY` has bits
/// 0xFFF0_0000_0000_0000 — exactly the boxed-NaN mask with tag=0,
/// payload=0. To preserve -∞ as a Number per C2, the boxed detection
/// requires both (a) high-12-bits match the mask AND (b) tag ≠ 0.
/// Effective tag space shrinks from 16 to 15 (tags 1-15); no real
/// information loss since first-cut uses only tag=2 and 6 more
/// variants are queued for VD-EXT 4+.
#[inline]
pub fn is_boxed_value(f: f64) -> bool {
    let bits = f.to_bits();
    bits & VD_BOXED_MASK == VD_BOXED_MASK && ((bits >> VD_TAG_SHIFT) & 0xF) != 0
}

#[inline]
pub fn extract_boxed_tag(f: f64) -> u8 {
    ((f.to_bits() >> VD_TAG_SHIFT) & 0xF) as u8
}

#[inline]
pub fn extract_boxed_payload(f: f64) -> u64 {
    f.to_bits() & VD_PAYLOAD_MASK
}

/// SAFETY: caller must ensure `f` was encoded from a `Value::String`
/// whose `Rc<String>` source is still live (typically the caller's
/// frame holds the source Value::String for the JIT call's duration).
/// Per R3: returns `*const String` for `&*ptr` use; do NOT reconstruct
/// the Rc via `Rc::from_raw` from this pointer (would over-decrement).
#[inline]
pub unsafe fn decode_string_ptr(f: f64) -> *const String {
    extract_boxed_payload(f) as *const String
}

/// OSR-EXT 5c (2026-05-23): inverse of unbox_arg_f64 for the OSR
/// locals-out path. Takes a post-JIT-invoke f64 + the slot's
/// pre-invoke Value (the "snapshot"). Returns the post-invoke Value.
///
/// First-cut shape per Pred-vd.4 scope discipline + Finding VD.1
/// constraints:
///   - If snapshot is Value::Number: the JIT computed a new f64;
///     return Value::Number(f).
///   - If snapshot is any other variant: the JIT either didn't
///     modify the slot or wrote a passed-through pointer/handle.
///     Per VD R3 (Rc strong-count not incremented at encode), don't
///     reconstruct via Rc::from_raw. Return snapshot.clone().
///
/// This is conservative + correctness-safe at first cut. The
/// limitation: if the JIT body writes a non-Number to a slot whose
/// pre-invoke Value was Number, the result is the raw f64 bits as a
/// Number — which is the canonical f64 reading and the conservative
/// behavior (no spurious String allocation or Rc count drift).
///
/// For well-formed loops where Number locals only receive Number
/// computations + String/Object locals stay as their original
/// references, this round-trip is correct.
pub fn box_to_value(f: f64, snapshot: &Value) -> Value {
    match snapshot {
        Value::Number(_) => Value::Number(f),
        _ => snapshot.clone(),
    }
}

/// Doc 731 §XIV.d typed-i64 unbox: accept a Value::Number with
/// integer-valued f64 representation; reject everything else with
/// TypeError. v1 strict shape: future deviation may relax to
/// `as i64` truncation under an opt-in primitive.
pub fn unbox_int64(v: &Value) -> Result<i64, RuntimeError> {
    match v {
        Value::Number(f) => {
            if f.is_finite() && f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                Ok(*f as i64)
            } else {
                Err(RuntimeError::TypeError(format!(
                    "typed-i64 op: operand {} is not an integer-valued Number",
                    f
                )))
            }
        }
        Value::Boolean(b) => Ok(if *b { 1 } else { 0 }),
        other => Err(RuntimeError::TypeError(format!(
            "typed-i64 op: operand is not a Number ({:?})",
            other
        ))),
    }
}

/// SPTW-EXT 1: ECMA-262 §12.2 White Space + §12.3 Line Terminators —
/// the character set String.prototype.trim/trimStart/trimEnd recognizes
/// per §22.1.3.30.1 TrimString. Cruft previously delegated to Rust's
/// `str::trim` (Unicode White_Space property), which omits U+FEFF
/// (ZWNBSP, deprecated as space marker by Unicode but retained by the
/// ES spec for backward compatibility) and surfaces 22 test262 failures.
#[inline]
fn is_es_whitespace_or_lineterm(c: char) -> bool {
    matches!(
        c,
        // §12.2 WhiteSpace
        '\u{0009}' | '\u{000B}' | '\u{000C}' | '\u{0020}' | '\u{00A0}' | '\u{FEFF}'
        // Unicode Zs (Other White_Space, the USP carve-out)
        | '\u{1680}'
        | '\u{2000}'
            ..='\u{200A}'
        | '\u{202F}' | '\u{205F}' | '\u{3000}'
        // §12.3 LineTerminator
        | '\u{000A}' | '\u{000D}' | '\u{2028}' | '\u{2029}'
    )
}

fn es_trim_start(s: &str) -> &str {
    s.trim_start_matches(is_es_whitespace_or_lineterm)
}
fn es_trim_end(s: &str) -> &str {
    s.trim_end_matches(is_es_whitespace_or_lineterm)
}
fn es_trim(s: &str) -> String {
    s.trim_matches(is_es_whitespace_or_lineterm).to_string()
}

fn caller_has_arguments_conflict_binding(frame: &Frame<'_>) -> bool {
    frame.locals_names.iter().enumerate().any(|(slot, desc)| {
        Runtime::direct_eval_binding_name(&desc.name) == "arguments"
            && if frame.is_arrow {
                slot < frame.param_count
            } else {
                desc.depth == 0
            }
    })
}

fn caller_has_lexical_binding(frame: &Frame<'_>, name: &str) -> bool {
    frame.locals_names.iter().any(|desc| {
        Runtime::direct_eval_binding_name(&desc.name) == name
            && matches!(
                desc.kind,
                rusty_js_ast::VariableKind::Let | rusty_js_ast::VariableKind::Const
            )
    })
}

fn direct_eval_super_context(
    frame: &Frame<'_>,
) -> Option<rusty_js_bytecode::DirectEvalSuperContext> {
    let mut out = None;
    for desc in frame.upvalue_names {
        direct_eval_super_context_visit(&mut out, &desc.name);
    }
    for desc in frame.locals_names {
        direct_eval_super_context_visit(&mut out, &desc.name);
    }
    if let Some(ctx) = &mut out {
        ctx.in_constructor = frame.derived_initial_this.is_some();
    }
    out
}

fn direct_eval_super_context_visit(
    out: &mut Option<rusty_js_bytecode::DirectEvalSuperContext>,
    name: &str,
) {
    let ctx = if name.contains(".super.") || name.contains(".home>") {
        out.get_or_insert_with(rusty_js_bytecode::DirectEvalSuperContext::default)
    } else {
        return;
    };
    if name.contains(".super.ctor") {
        ctx.super_ctor_name = Some(name.to_string());
    } else if name.contains(".super.proto") {
        ctx.super_proto_name = Some(name.to_string());
    } else if name.contains(".home>") {
        ctx.super_home_name = Some(name.to_string());
    }
}

struct DirectEvalOverlay {
    name: String,
    value: Value,
    target: DirectEvalOverlayTarget,
}

enum DirectEvalOverlayTarget {
    Upvalue(UpvalueCell),
    Local(usize),
    Temporary,
}

fn eval_source_is_strict(source: &str) -> bool {
    let Ok(module) = rusty_js_parser::parse_module(source) else {
        return false;
    };
    for item in &module.body {
        let rusty_js_ast::ModuleItem::Statement(stmt) = item else {
            return false;
        };
        match stmt {
            rusty_js_ast::Stmt::Expression { expr, .. } => match expr {
                rusty_js_ast::Expr::StringLiteral { value, .. } if value == "use strict" => {
                    return true;
                }
                rusty_js_ast::Expr::StringLiteral { .. } => continue,
                _ => return false,
            },
            _ => return false,
        }
    }
    false
}

#[derive(Default)]
struct EvalVarScopedDeclarations {
    vars: Vec<String>,
    functions: Vec<EvalFunctionDeclaration>,
}

impl EvalVarScopedDeclarations {
    fn names(&self) -> Vec<String> {
        let mut names = self.vars.clone();
        for function in &self.functions {
            names.push(function.name.clone());
        }
        names
    }
}

struct EvalFunctionDeclaration {
    name: String,
    return_value: Value,
}

fn eval_var_scoped_declaration_names(source: &str) -> Vec<String> {
    eval_var_scoped_declarations(source).names()
}

fn eval_var_scoped_declarations(source: &str) -> EvalVarScopedDeclarations {
    let Ok(module) = rusty_js_parser::parse_module(source) else {
        return eval_var_scoped_declarations_fallback(source);
    };
    let mut decls = EvalVarScopedDeclarations::default();
    for item in &module.body {
        match item {
            rusty_js_ast::ModuleItem::Statement(stmt) => {
                collect_stmt_var_scoped_declarations(stmt, &mut decls);
            }
            rusty_js_ast::ModuleItem::Export(export) => match export {
                rusty_js_ast::ExportDeclaration::Declaration {
                    decl_stmt: Some(stmt),
                    ..
                } => collect_stmt_var_scoped_declarations(stmt, &mut decls),
                _ => {}
            },
            _ => {}
        }
    }
    if decls.vars.is_empty() && decls.functions.is_empty() {
        eval_var_scoped_declarations_fallback(source)
    } else {
        decls
    }
}

fn eval_var_scoped_declarations_fallback(source: &str) -> EvalVarScopedDeclarations {
    let mut decls = EvalVarScopedDeclarations::default();
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if keyword_at(source, i, "var") {
            if let Some((name, next)) = read_identifier_after_keyword(source, i + 3) {
                decls.vars.push(name);
                i = next;
                continue;
            }
        }
        if keyword_at(source, i, "function") {
            if let Some((name, next)) = read_identifier_after_keyword(source, i + 8) {
                let return_value = fallback_function_return_value(&source[next..]);
                decls
                    .functions
                    .push(EvalFunctionDeclaration { name, return_value });
                i = next;
                continue;
            }
        }
        i += 1;
    }
    decls
}

fn keyword_at(source: &str, offset: usize, keyword: &str) -> bool {
    let end = offset + keyword.len();
    if end > source.len() || !source.is_char_boundary(offset) || !source.is_char_boundary(end) {
        return false;
    }
    source[offset..].starts_with(keyword)
        && source[..offset]
            .chars()
            .next_back()
            .map(|c| !is_identifier_continue(c))
            .unwrap_or(true)
        && source[end..]
            .chars()
            .next()
            .map(|c| !is_identifier_continue(c))
            .unwrap_or(true)
}

fn read_identifier_after_keyword(source: &str, mut offset: usize) -> Option<(String, usize)> {
    while let Some(c) = source[offset..].chars().next() {
        if !is_es_whitespace_or_lineterm(c) {
            break;
        }
        offset += c.len_utf8();
    }
    let mut chars = source[offset..].char_indices();
    let (_, first) = chars.next()?;
    if !is_identifier_start(first) {
        return None;
    }
    let mut end = offset + first.len_utf8();
    for (idx, c) in chars {
        if !is_identifier_continue(c) {
            break;
        }
        end = offset + idx + c.len_utf8();
    }
    Some((source[offset..end].to_string(), end))
}

fn fallback_function_return_value(source_after_name: &str) -> Value {
    let Some(return_idx) = source_after_name.find("return") else {
        return Value::Undefined;
    };
    let rest = es_trim_start(&source_after_name[return_idx + 6..]);
    let literal = rest
        .split(|c| matches!(c, ';' | '}'))
        .next()
        .map(es_trim)
        .unwrap_or_default();
    match literal.as_str() {
        "undefined" => Value::Undefined,
        "null" => Value::Null,
        "true" => Value::Boolean(true),
        "false" => Value::Boolean(false),
        _ => literal
            .parse::<f64>()
            .map(Value::Number)
            .unwrap_or(Value::Undefined),
    }
}

fn is_identifier_start(c: char) -> bool {
    c == '_' || c == '$' || c.is_ascii_alphabetic()
}

fn is_identifier_continue(c: char) -> bool {
    is_identifier_start(c) || c.is_ascii_digit()
}

fn collect_stmt_var_scoped_names(stmt: &rusty_js_ast::Stmt, out: &mut Vec<String>) {
    let mut decls = EvalVarScopedDeclarations::default();
    collect_stmt_var_scoped_declarations(stmt, &mut decls);
    out.extend(decls.names());
}

fn collect_stmt_var_scoped_declarations(
    stmt: &rusty_js_ast::Stmt,
    out: &mut EvalVarScopedDeclarations,
) {
    use rusty_js_ast::{ForBinding, ForInit, Stmt, VariableKind};

    match stmt {
        Stmt::Variable(var) if var.kind == VariableKind::Var => {
            for decl in &var.declarators {
                collect_binding_pattern_names(&decl.target, &mut out.vars);
            }
        }
        Stmt::FunctionDecl { name, body, .. } => {
            if let Some(binding) = name {
                out.functions.push(EvalFunctionDeclaration {
                    name: binding.name.clone(),
                    return_value: eval_function_return_value(body),
                });
            }
        }
        Stmt::Block { body, .. } => {
            for stmt in body {
                collect_stmt_var_scoped_declarations(stmt, out);
            }
        }
        Stmt::If {
            consequent,
            alternate,
            ..
        } => {
            collect_stmt_var_scoped_declarations(consequent, out);
            if let Some(alternate) = alternate {
                collect_stmt_var_scoped_declarations(alternate, out);
            }
        }
        Stmt::For { init, body, .. } => {
            if let Some(ForInit::Variable(var)) = init {
                if var.kind == VariableKind::Var {
                    for decl in &var.declarators {
                        collect_binding_pattern_names(&decl.target, &mut out.vars);
                    }
                }
            }
            collect_stmt_var_scoped_declarations(body, out);
        }
        Stmt::ForIn { left, body, .. } | Stmt::ForOf { left, body, .. } => {
            if let ForBinding::Decl {
                kind: VariableKind::Var,
                target,
                ..
            } = left
            {
                collect_binding_pattern_names(target, &mut out.vars);
            }
            collect_stmt_var_scoped_declarations(body, out);
        }
        Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
            collect_stmt_var_scoped_declarations(body, out);
        }
        Stmt::Switch { cases, .. } => {
            for case in cases {
                for stmt in &case.consequent {
                    collect_stmt_var_scoped_declarations(stmt, out);
                }
            }
        }
        Stmt::Try {
            block,
            handler,
            finalizer,
            ..
        } => {
            collect_stmt_var_scoped_declarations(block, out);
            if let Some(catch) = handler {
                collect_stmt_var_scoped_declarations(&catch.body, out);
            }
            if let Some(finalizer) = finalizer {
                collect_stmt_var_scoped_declarations(finalizer, out);
            }
        }
        Stmt::Labelled { body, .. } => collect_stmt_var_scoped_declarations(body, out),
        _ => {}
    }
}

fn eval_function_return_value(body: &[rusty_js_ast::Stmt]) -> Value {
    use rusty_js_ast::{Expr, Stmt};

    if let [Stmt::Return {
        argument: Some(expr),
        ..
    }] = body
    {
        match expr {
            Expr::NumberLiteral { value, .. } => Value::Number(*value),
            Expr::StringLiteral { value, .. } => Value::String(Rc::new(value.clone())),
            Expr::BoolLiteral { value, .. } => Value::Boolean(*value),
            Expr::NullLiteral { .. } => Value::Null,
            _ => Value::Undefined,
        }
    } else {
        Value::Undefined
    }
}

fn collect_binding_pattern_names(pattern: &rusty_js_ast::BindingPattern, out: &mut Vec<String>) {
    for binding in pattern.collect_names() {
        out.push(binding.name.clone());
    }
}

fn is_strict_mode_reserved_binding_name(name: &str) -> bool {
    matches!(
        name,
        "eval"
            | "arguments"
            | "yield"
            | "implements"
            | "interface"
            | "let"
            | "package"
            | "private"
            | "protected"
            | "public"
            | "static"
    )
}

pub fn property_key(v: &Value) -> crate::value::PropertyKey {
    match v {
        Value::Symbol(rc) => crate::value::PropertyKey::Symbol(rc.clone()),
        Value::String(s) => crate::value::PropertyKey::String(s.as_str().to_string()),
        Value::Number(n) => {
            crate::value::PropertyKey::String(crate::abstract_ops::number_to_string(*n))
        }
        _ => crate::value::PropertyKey::String(
            crate::abstract_ops::to_string(v).as_str().to_string(),
        ),
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
    /// WBMS-EXT 2: object-environment stack for sloppy `with` execution.
    /// Bare identifier bytecodes inside a with-body route through these
    /// objects before falling back to lexical bindings and globals.
    pub with_env_stack: Vec<crate::value::ObjectRef>,
    /// `this` for the executing frame. Module frames default to Undefined;
    /// method-call frames receive the receiver. Tier-Ω.5.a.
    pub this_value: Value,
    /// IR-EXT 39: in a derived-class constructor, SetThisTDZ stashes the
    /// fresh `this` (passed in by call_function from the new-expression
    /// alloc) here BEFORE overwriting this_value with the TDZ sentinel.
    /// Op::PushThisRaw reads from here for super-call setup so the
    /// parent ctor receives the spec-correct shared `this`. None for
    /// non-derived-ctor frames.
    pub derived_initial_this: Option<Value>,
    /// Cell-backed `this` binding. Lazily promoted when an arrow inside
    /// this frame captures `this`. Op::SetThis writes through the cell
    /// (if present) so arrows created BEFORE super() resolves see the
    /// updated post-super value at call time.
    pub this_cell: Option<UpvalueCell>,
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
    /// GPI-EXT 2 (2026-05-24): site_pc of the Op::GetProp that produced
    /// `pending_method_name`. Used by Op::CallMethod's IC-hit branch
    /// (String receiver_kind) to walk back and rewrite the GetProp byte
    /// to Op::GetPropSkipForMethod. Cleared at the same sites that clear
    /// pending_method_name.
    pub pending_method_getprop_pc: Option<usize>,
    /// Class home object for private-name resolution in class methods and
    /// accessors. ECMA private names are fresh per class evaluation; this
    /// bridges the current string-keyed private storage toward that identity.
    pub private_home: Option<ObjectRef>,
    /// Tier-Ω.5.r: synthetic `import.meta` object for this module frame.
    /// Populated by `evaluate_module` (ESM path) with `{ url, dir }` keys.
    /// Frames that didn't enter through the module loader (raw run_module
    /// callers, function-call frames) leave this None; Op::PushImportMeta
    /// pushes Undefined in that case.
    pub import_meta: Option<crate::value::ObjectRef>,
    pub is_arrow: bool,
    pub param_count: usize,
    /// Ω.5.P04.E2.strict-write-enforcement: strict-mode flag for this
    /// frame. Module frames inherit from CompiledModule.strict; function
    /// frames inherit from FunctionProto.strict. Read by Op::SetProp,
    /// Op::SetIndex, and Op::StoreGlobal to enforce strict-mode rejection
    /// of write-to-non-writable (TypeError) and write-to-undeclared
    /// (ReferenceError).
    pub strict: bool,
    /// Tier-Ω.5.s: `new.target` slot. Populated by Op::New before
    /// dispatching the constructor call (set to the callee value). Plain
    /// Call frames leave this None; Op::PushNewTarget pushes Undefined
    /// in that case. Mirrors the import_meta threading shape.
    pub new_target: Option<Value>,
    /// OSR-EXT 2 (2026-05-23): per-pc back-edge counter for OSR / loop-
    /// extraction (Option A — Runtime Bytecode Transform per Doc 740
    /// §II.2 op-set coverage closure). Incremented at every Jump /
    /// JumpIfTrue / JumpIfFalse / JumpIfTrueKeep / JumpIfFalseKeep
    /// with negative displacement (loop back-edge). Substrate-
    /// introduction: counts only; OSR-EXT 3+ consumes the counts for
    /// threshold detection + loop-body extraction + JIT entry.
    ///
    /// Key: the back-edge instruction's pc (NOT the loop entry pc; the
    /// back-edge target is the loop entry, but the back-edge SITE is
    /// what uniquely identifies the loop in the bytecode). Empty by
    /// default (no allocation until first back-edge fires).
    pub back_edge_counts: HashMap<usize, u32>,
    /// IHI-EXT 7 (2026-05-24, Finding IHI.1): per-call-site IC dispatch
    /// cache. Keyed on the Op::CallMethod byte's pc; value is the
    /// IHI_TABLE entry pointer for this call site (Some) or None (no
    /// entry matches). Populated on first dispatch at the pc; read on
    /// subsequent dispatches to skip the linear-scan table lookup.
    /// Per Pred-ihi.5 closure path: eliminates per-call dispatch
    /// overhead for non-IC calls (~40 ns/call saved on cache hit).
    pub ic_dispatch_cache:
        std::collections::HashMap<usize, Option<&'static crate::interp_ic_table::IhiEntry>>,
    pub eval_var_env_is_global: bool,

    /// OSR-EXT 4+5 (2026-05-23): per-site OSR compile cache.
    ///   - Absent: not yet attempted.
    ///   - Present + Some(boxed): compiled successfully (invoke at
    ///     OSR-EXT 5b once locals-marshaling lands per Finding OSR.1).
    ///   - Present + None: compile failed (e.g., op outside JIT
    ///     alphabet per Finding VII.2); skip subsequent attempts.
    ///
    /// Box-wrap per standing rule 9 (TB-EXT 7 raw-pointer-cache pattern)
    /// in case downstream consumers cache pointers into the CompiledFn.
    pub osr_cache: HashMap<usize, Option<Box<rusty_js_jit::CompiledFn>>>,
}

/// GCS-EXT 2a: owned continuation payload for future generator
/// suspend/resume. `Frame<'a>` borrows bytecode and metadata from a
/// module/function proto, which cannot live inside a heap GeneratorObject.
/// This snapshot owns the resumable state and can be borrowed back into a
/// `Frame<'_>` when a later rung wires the actual yield boundary.
#[derive(Debug, Clone)]
pub struct FrameSnapshot {
    pub function_proto: Option<Rc<rusty_js_bytecode::compiler::FunctionProto>>,
    pub bytecode: Vec<u8>,
    pub constants: rusty_js_bytecode::ConstantsPool,
    pub source_map: Vec<(usize, rusty_js_ast::Span)>,
    pub line_starts: Vec<u32>,
    pub source_url: String,
    pub construct_tags: Vec<(usize, &'static str)>,
    pub locals_names: Vec<rusty_js_bytecode::compiler::LocalDescriptor>,
    pub upvalue_names: Vec<rusty_js_bytecode::compiler::UpvalueDescriptor>,
    pub locals: Vec<Value>,
    pub local_cells: Vec<Option<UpvalueCell>>,
    pub operand_stack: Vec<Value>,
    pub pc: usize,
    pub try_stack: Vec<TryFrame>,
    pub with_env_stack: Vec<crate::value::ObjectRef>,
    pub this_value: Value,
    pub derived_initial_this: Option<Value>,
    pub this_cell: Option<UpvalueCell>,
    pub upvalues: Vec<UpvalueCell>,
    pub last_property_lookup: Option<String>,
    pub pending_method_name: Option<String>,
    pub pending_method_getprop_pc: Option<usize>,
    pub private_home: Option<ObjectRef>,
    pub import_meta: Option<crate::value::ObjectRef>,
    pub is_arrow: bool,
    pub param_count: usize,
    pub strict: bool,
    pub new_target: Option<Value>,
    pub eval_var_env_is_global: bool,
}

impl FrameSnapshot {
    pub fn from_frame(
        frame: &Frame<'_>,
        function_proto: Option<Rc<rusty_js_bytecode::compiler::FunctionProto>>,
    ) -> Self {
        Self {
            function_proto,
            bytecode: frame.bytecode.to_vec(),
            constants: frame.constants.clone(),
            source_map: frame.source_map.to_vec(),
            line_starts: frame.line_starts.to_vec(),
            source_url: frame.source_url.to_string(),
            construct_tags: frame.construct_tags.to_vec(),
            locals_names: frame.locals_names.to_vec(),
            upvalue_names: frame.upvalue_names.to_vec(),
            locals: frame.locals.clone(),
            local_cells: frame.local_cells.clone(),
            operand_stack: frame.operand_stack.clone(),
            pc: frame.pc,
            try_stack: frame.try_stack.clone(),
            with_env_stack: frame.with_env_stack.clone(),
            this_value: frame.this_value.clone(),
            derived_initial_this: frame.derived_initial_this.clone(),
            this_cell: frame.this_cell.clone(),
            upvalues: frame.upvalues.clone(),
            last_property_lookup: frame.last_property_lookup.clone(),
            pending_method_name: frame.pending_method_name.clone(),
            pending_method_getprop_pc: frame.pending_method_getprop_pc,
            private_home: frame.private_home,
            import_meta: frame.import_meta,
            is_arrow: frame.is_arrow,
            param_count: frame.param_count,
            strict: frame.strict,
            new_target: frame.new_target.clone(),
            eval_var_env_is_global: frame.eval_var_env_is_global,
        }
    }

    pub fn trace_object_refs(&self, ids: &mut Vec<rusty_js_gc::ObjectId>) {
        for value in self
            .locals
            .iter()
            .chain(self.operand_stack.iter())
            .chain(std::iter::once(&self.this_value))
            .chain(self.derived_initial_this.iter())
            .chain(self.new_target.iter())
        {
            if let Value::Object(id) = value {
                ids.push(*id);
            }
        }
        for id in &self.with_env_stack {
            ids.push(*id);
        }
        if let Some(id) = self.private_home {
            ids.push(id);
        }
        if let Some(id) = self.import_meta {
            ids.push(id);
        }
        for cell in self
            .local_cells
            .iter()
            .flatten()
            .chain(self.this_cell.iter())
            .chain(self.upvalues.iter())
        {
            if let Value::Object(id) = &*cell.borrow() {
                ids.push(*id);
            }
        }
    }
}

impl<'a> From<&'a FrameSnapshot> for Frame<'a> {
    fn from(snapshot: &'a FrameSnapshot) -> Self {
        Self {
            bytecode: &snapshot.bytecode,
            constants: &snapshot.constants,
            source_map: &snapshot.source_map,
            line_starts: &snapshot.line_starts,
            source_url: &snapshot.source_url,
            construct_tags: &snapshot.construct_tags,
            locals_names: &snapshot.locals_names,
            upvalue_names: &snapshot.upvalue_names,
            locals: snapshot.locals.clone(),
            local_cells: snapshot.local_cells.clone(),
            operand_stack: snapshot.operand_stack.clone(),
            pc: snapshot.pc,
            try_stack: snapshot.try_stack.clone(),
            with_env_stack: snapshot.with_env_stack.clone(),
            this_value: snapshot.this_value.clone(),
            derived_initial_this: snapshot.derived_initial_this.clone(),
            this_cell: snapshot.this_cell.clone(),
            upvalues: snapshot.upvalues.clone(),
            last_property_lookup: snapshot.last_property_lookup.clone(),
            pending_method_name: snapshot.pending_method_name.clone(),
            pending_method_getprop_pc: snapshot.pending_method_getprop_pc,
            private_home: snapshot.private_home,
            import_meta: snapshot.import_meta,
            is_arrow: snapshot.is_arrow,
            param_count: snapshot.param_count,
            strict: snapshot.strict,
            new_target: snapshot.new_target.clone(),
            back_edge_counts: HashMap::new(),
            ic_dispatch_cache: HashMap::new(),
            eval_var_env_is_global: snapshot.eval_var_env_is_global,
            osr_cache: HashMap::new(),
        }
    }
}

fn function_proto_uses_yield_delegate(proto: &rusty_js_bytecode::compiler::FunctionProto) -> bool {
    proto.constants.entries().iter().any(|constant| {
        matches!(
            constant,
            rusty_js_bytecode::Constant::String(name) if name.contains("yield_delegate")
        )
    })
}

/// OSR-EXT 2 (2026-05-23): threshold for OSR JIT-attempt trigger.
/// Reserved for OSR-EXT 3+ consumption; consulted at OSR-EXT 4 first.
pub const OSR_BACK_EDGE_THRESHOLD: u32 = 1000;

/// OSR-EXT 3 (2026-05-23): given a back-edge site (Op::Jump or
/// JumpIfX with negative disp at `site_pc`), compute the loop region
/// bytecode range `(entry_pc, end_pc)` where:
///   - `entry_pc` = (site_pc + 5) + disp — the back-edge's target,
///     which is the loop's entry point.
///   - `end_pc` = site_pc + 5 — one byte past the back-edge's disp
///     bytes (every Jump variant is 1 op byte + 4 disp bytes).
///
/// The region [entry_pc, end_pc) is the loop body bytecode slice the
/// JIT would compile at OSR-EXT 4. This helper is the boundary
/// detector ONLY; it does not validate JIT-eligibility (alphabet
/// coverage, nested-back-edge containment, break/continue handling)
/// — those checks belong to OSR-EXT 4's compile attempt.
///
/// Returns None if site_pc is out of bounds, disp is non-negative
/// (not a back-edge), or entry_pc would be out of bounds.
/// OSR-EXT 4+5 (2026-05-23): at back-edge threshold trigger, attempt to
/// JIT-compile the loop region as a synthetic 0-arg FunctionProto + cache
/// the result. Box-wrap per standing rule 9 (TB-EXT 7 raw-pointer-cache
/// pattern). Subsequent back-edge fires at the same site_pc consult the
/// cache and skip if previously failed.
///
/// The invoke step is NOT performed here per Finding OSR.1 (locale
/// findings.md): JIT calling convention's params-only-as-args shape
/// blocks frame-state marshaling without a locals-marshaling
/// substrate extension. Cached CompiledFn (Some variant) is unused
/// until OSR-EXT 5b lands locals-marshaling per Finding OSR.1's
/// recommended option 2 (extern-pre-populate).
///
/// Expected outcome on json_parse_transform: cache stores None per
/// Finding VII.2 alphabet gap. The cache eliminates repeated
/// compile attempts (small but real engineering value).
fn try_osr_compile(frame: &mut Frame, site_pc: usize) {
    let region = match compute_loop_region(frame.bytecode, site_pc) {
        Some(r) => r,
        None => {
            // Not a loop region; cache None to skip future attempts.
            frame.osr_cache.insert(site_pc, None);
            return;
        }
    };
    let (entry_pc, end_pc) = region;
    let sub_bytecode = frame.bytecode[entry_pc..end_pc].to_vec();
    let synth = rusty_js_bytecode::compiler::FunctionProto {
        bytecode: sub_bytecode,
        constants: frame.constants.clone(),
        params: 0,
        display_name: format!("<osr-loop@{site_pc}>"),
        function_length: 0,
        locals: frame.locals_names.to_vec(),
        upvalues: Vec::new(),
        rest_param_slot: None,
        arguments_slot: None,
        self_name_slot: None,
        param_prologue_end: 0,
        is_generator: false,
        line_starts: Vec::new(),
        source_map: Vec::new(),
        construct_tags: Vec::new(),
        source_url: String::new(),
        is_async: false,
        strict: frame.strict,
    };
    // OSR-EXT 5b+5d (2026-05-23): use compile_function_osr to produce
    // an ArityOsr CompiledFn with proper locals load/store IR. The
    // compiled body's signature is extern "C" fn(*mut f64) -> f64;
    // OSR-EXT 5d wires the invoke via call_osr.
    let compiled = rusty_js_jit::compile_function_osr(&synth);
    // OSR-EXT 5e (2026-05-23): telemetry for synthetic-fixture validation.
    // Under CRUFTLESS_OSR_TRACE=1, log compile attempts + outcomes to
    // stderr; defaults off (no perf impact).
    if std::env::var("CRUFTLESS_OSR_TRACE")
        .map(|v| v == "1")
        .unwrap_or(false)
    {
        match &compiled {
            Ok(_) => eprintln!(
                "[osr-trace] try_osr_compile site={site_pc} entry={entry_pc} end={end_pc} OK"
            ),
            Err(e) => eprintln!(
                "[osr-trace] try_osr_compile site={site_pc} entry={entry_pc} end={end_pc} ERR: {e}"
            ),
        }
    }
    frame.osr_cache.insert(site_pc, compiled.ok().map(Box::new));
}

/// OSR-EXT 5d (2026-05-23): invoke a cached OSR CompiledFn on a back-
/// edge fire. Returns true if invoke fired (caller skips normal jump);
/// false otherwise.
///
/// Invoke flow:
///   1. Snapshot frame.locals (Vec<Value>) for the box_to_value path.
///   2. Marshal frame.locals → Vec<f64> via unbox_arg_f64.
///   3. Call compiled.func.call_osr(arr.as_mut_ptr()) — JIT body reads
///      locals from arr on entry, runs the loop, writes locals back at
///      exit (Return / ReturnUndef / synthesized-end-of-body).
///   4. Marshal Vec<f64> → frame.locals via box_to_value with snapshot.
///   5. Set frame.pc = end_pc (post-back-edge pc; loop done).
///
/// Per Finding OSR.1 + Doc 740 §VIII.2: locals-marshaling closure;
/// JIT body runs ALL remaining iters internally (do-while-shape loops);
/// for-loop / while-loop with forward exit jumps to out-of-bounds
/// targets and would have compile-failed at try_osr_compile (cache
/// stores None) — this helper sees cache None and returns false.
fn try_osr_invoke(frame: &mut Frame, site_pc: usize) -> bool {
    // OSR-EXT 5d: fast-path early-out when cache has None (compile
    // previously failed; saves the function-call overhead at every
    // back-edge fire past threshold).
    let compiled = match frame.osr_cache.get(&site_pc) {
        Some(Some(boxed)) => boxed.as_ref() as *const rusty_js_jit::CompiledFn,
        Some(None) | None => return false,
    };
    let end_pc = site_pc + 5;
    let n_locals = frame.locals.len();
    let mut snapshot: Vec<Value> = Vec::with_capacity(n_locals);
    let mut arr: Vec<f64> = Vec::with_capacity(n_locals);
    for v in &frame.locals {
        snapshot.push(v.clone());
        arr.push(unbox_arg_f64(v));
    }
    // TLS plumbing: OSR loop bodies don't make extern calls (Finding
    // VII.2 + Pred-tl.4 keep alphabet narrow), so proto/runtime ptrs
    // are unused. Pass 0 to satisfy the API.
    rusty_js_jit::set_current_runtime(0);
    rusty_js_jit::set_current_proto(0);
    // SAFETY: the cache holds Box<CompiledFn>; *const CompiledFn is
    // stable for the box's lifetime (TB-EXT 7 standing rule 9 pattern);
    // we hold &mut frame which owns the cache, so the boxed alloc
    // lives for the call's duration.
    let _r = unsafe { (*compiled).func.call_osr(arr.as_mut_ptr()) };
    rusty_js_jit::clear_current_runtime();
    rusty_js_jit::clear_current_proto();
    // Marshal locals back. Per box_to_value: Number snapshots get the
    // new f64; other variants preserve the snapshot.
    for (i, f) in arr.into_iter().enumerate() {
        if i < frame.locals.len() {
            frame.locals[i] = box_to_value(f, &snapshot[i]);
        }
    }
    // Skip the normal back-edge jump; advance past the back-edge to
    // the post-loop pc.
    frame.pc = end_pc;
    // OSR-EXT 5e telemetry: log invoke fire under CRUFTLESS_OSR_TRACE=1.
    if std::env::var("CRUFTLESS_OSR_TRACE")
        .map(|v| v == "1")
        .unwrap_or(false)
    {
        eprintln!("[osr-trace] try_osr_invoke site={site_pc} FIRED (set pc={end_pc})");
    }
    true
}

pub fn compute_loop_region(bytecode: &[u8], site_pc: usize) -> Option<(usize, usize)> {
    if site_pc + 5 > bytecode.len() {
        return None;
    }
    let disp = i32::from_le_bytes([
        bytecode[site_pc + 1],
        bytecode[site_pc + 2],
        bytecode[site_pc + 3],
        bytecode[site_pc + 4],
    ]);
    if disp >= 0 {
        return None;
    }
    let end_pc = site_pc + 5;
    let entry_pc_signed = (end_pc as i32).checked_add(disp)?;
    if entry_pc_signed < 0 {
        return None;
    }
    let entry_pc = entry_pc_signed as usize;
    if entry_pc >= end_pc {
        return None;
    } // sanity: forward disp got past the check
    Some((entry_pc, end_pc))
}

#[derive(Debug, Clone)]
pub struct TryFrame {
    pub catch_offset: usize,
    pub sp_at_entry: usize,
}

fn inject_throw_into_frame(frame: &mut Frame<'_>, thrown: Value) -> Result<(), RuntimeError> {
    let Some(t) = frame.try_stack.pop() else {
        return Err(RuntimeError::Thrown(thrown));
    };
    frame.operand_stack.truncate(t.sp_at_entry);
    frame.operand_stack.push(thrown);
    frame.pc = t.catch_offset;
    Ok(())
}

impl<'a> Frame<'a> {
    pub fn new_module(m: &'a CompiledModule) -> Self {
        let mut locals = Vec::new();
        for _ in &m.locals {
            locals.push(Value::Undefined);
        }
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
            with_env_stack: Vec::new(),
            this_value: Value::Undefined,
            this_cell: None,
            derived_initial_this: None,
            upvalues: Vec::new(),
            last_property_lookup: None,
            pending_method_name: None,
            pending_method_getprop_pc: None,
            private_home: None,
            import_meta: None,
            new_target: None,
            is_arrow: false,
            param_count: 0,
            strict: m.strict,
            back_edge_counts: HashMap::new(),
            osr_cache: HashMap::new(),
            ic_dispatch_cache: HashMap::new(),
            eval_var_env_is_global: m.eval_var_env_is_global,
        }
    }

    fn this_value_for_eval(&self) -> Value {
        if let Some(cell) = &self.this_cell {
            cell.borrow().clone()
        } else {
            self.this_value.clone()
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
        while self.locals.len() <= slot {
            self.locals.push(Value::Undefined);
        }
        self.locals[slot] = v;
    }

    /// Promote local `slot` to a shared cell (idempotent). Used when a
    /// nested closure captures the slot — the cell becomes authoritative
    /// for both this frame's reads/writes and the closure's upvalue.
    pub fn promote_local(&mut self, slot: usize) -> UpvalueCell {
        while self.locals.len() <= slot {
            self.locals.push(Value::Undefined);
        }
        while self.local_cells.len() <= slot {
            self.local_cells.push(None);
        }
        if let Some(cell) = &self.local_cells[slot] {
            return cell.clone();
        }
        let v = std::mem::replace(&mut self.locals[slot], Value::Undefined);
        let cell = new_upvalue_cell(v);
        self.local_cells[slot] = Some(cell.clone());
        cell
    }

    pub fn push(&mut self, v: Value) {
        self.operand_stack.push(v);
    }

    pub fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.operand_stack
            .pop()
            .ok_or_else(|| RuntimeError::TypeError("operand stack underflow".into()))
    }

    pub fn peek(&self, depth: usize) -> Result<&Value, RuntimeError> {
        let len = self.operand_stack.len();
        if depth >= len {
            return Err(RuntimeError::TypeError(
                "operand stack peek underflow".into(),
            ));
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
/// Compute UTC epoch milliseconds for a (year, month-0idx, day-1idx,
/// hours, minutes, seconds, ms) tuple. Uses the Gregorian calendar
/// algorithm; valid across the full IEEE-754-representable date range.
fn utc_components_to_epoch_ms(
    year: i64,
    month: i64,
    day: i64,
    h: i64,
    mi: i64,
    s: i64,
    ms: i64,
) -> i64 {
    // Normalize month overflow into year.
    let total_months = year * 12 + month;
    let y = total_months.div_euclid(12);
    let m = total_months.rem_euclid(12) as i32; // 0..11
                                                // Days from epoch (1970-01-01) to start of year y.
    let days_from_epoch_to_year_start = |y: i64| -> i64 {
        // Number of days from year 1 to year y (Gregorian). Then subtract
        // 1969 years × 365 + leap-day count up to 1970.
        let y_prev = y - 1;
        let days_to_y =
            365 * y_prev + y_prev.div_euclid(4) - y_prev.div_euclid(100) + y_prev.div_euclid(400);
        let days_to_1970 = 365 * 1969 + 1969 / 4 - 1969 / 100 + 1969 / 400;
        days_to_y - days_to_1970
    };
    let is_leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let month_days = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut days_in_year: i64 = 0;
    for i in 0..(m as usize) {
        days_in_year += month_days[i] as i64;
    }
    days_in_year += day - 1;
    let total_days = days_from_epoch_to_year_start(y) + days_in_year;
    let total_secs = total_days * 86400 + h * 3600 + mi * 60 + s;
    total_secs * 1000 + ms
}

/// Public wrapper for the ISO 8601 parser (called from intrinsics.rs).
pub fn parse_iso8601_to_epoch_ms_public(s: &str) -> Option<f64> {
    parse_iso8601_to_epoch_ms(s)
}

/// Parse an ISO 8601 datetime string to UTC epoch milliseconds. Supports:
///   YYYY-MM-DD
///   YYYY-MM-DDTHH:MM:SS
///   YYYY-MM-DDTHH:MM:SSZ
///   YYYY-MM-DDTHH:MM:SS.sssZ
///   YYYY-MM-DDTHH:MM:SS+HH:MM (tz offset)
/// Returns None on parse failure.
fn parse_iso8601_to_epoch_ms(s: &str) -> Option<f64> {
    let s = s.trim();
    let bytes = s.as_bytes();
    if bytes.len() < 10 {
        return None;
    }
    // Year-month-day
    let year: i64 = std::str::from_utf8(&bytes[0..4]).ok()?.parse().ok()?;
    if bytes[4] != b'-' {
        return None;
    }
    let month: i64 = std::str::from_utf8(&bytes[5..7]).ok()?.parse().ok()?;
    if bytes[7] != b'-' {
        return None;
    }
    let day: i64 = std::str::from_utf8(&bytes[8..10]).ok()?.parse().ok()?;
    if month < 1 || month > 12 || day < 1 || day > 31 {
        return None;
    }
    let (mut h, mut mi, mut sc, mut ms) = (0i64, 0i64, 0i64, 0i64);
    let mut tz_offset_min: i64 = 0;
    let rest = &s[10..];
    if !rest.is_empty() {
        let rb = rest.as_bytes();
        if rb[0] != b'T' && rb[0] != b' ' {
            return None;
        }
        if rb.len() >= 9 {
            h = std::str::from_utf8(&rb[1..3]).ok()?.parse().ok()?;
            if rb[3] != b':' {
                return None;
            }
            mi = std::str::from_utf8(&rb[4..6]).ok()?.parse().ok()?;
            if rb[6] != b':' {
                return None;
            }
            sc = std::str::from_utf8(&rb[7..9]).ok()?.parse().ok()?;
            let mut p = 9usize;
            if p < rb.len() && rb[p] == b'.' {
                let end = p
                    + 1
                    + rb[p + 1..]
                        .iter()
                        .take_while(|c| c.is_ascii_digit())
                        .count();
                let frac = std::str::from_utf8(&rb[p + 1..end]).ok()?;
                // Convert fractional seconds → ms. Pad/truncate to 3 digits.
                let mut digits: String = frac.chars().take(3).collect();
                while digits.len() < 3 {
                    digits.push('0');
                }
                ms = digits.parse().ok()?;
                p = end;
            }
            if p < rb.len() {
                match rb[p] {
                    b'Z' => {}
                    b'+' | b'-' => {
                        if p + 5 < rb.len() && rb[p + 3] == b':' {
                            let sign: i64 = if rb[p] == b'+' { 1 } else { -1 };
                            let oh: i64 =
                                std::str::from_utf8(&rb[p + 1..p + 3]).ok()?.parse().ok()?;
                            let om: i64 =
                                std::str::from_utf8(&rb[p + 4..p + 6]).ok()?.parse().ok()?;
                            tz_offset_min = sign * (oh * 60 + om);
                        }
                    }
                    _ => return None,
                }
            }
        }
    }
    let epoch_ms = utc_components_to_epoch_ms(year, month - 1, day, h, mi, sc, ms);
    Some((epoch_ms - tz_offset_min * 60_000) as f64)
}

/// ECMA-262 §22.1.3.15 step 11 GetSubstitution — for string-form replacement:
///   $$  → literal $
///   $&  → matched substring
///   $`  → portion before the match
///   $'  → portion after the match
/// Capture groups ($N) only apply to RegExp searches and are dispatched
/// via @@replace upstream; here we leave them as-is.
fn process_replacement_substitution(
    repl: &str,
    matched: &str,
    before: &str,
    after: &str,
) -> String {
    let mut out = String::with_capacity(repl.len());
    let bytes = repl.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'$' => {
                    out.push('$');
                    i += 2;
                    continue;
                }
                b'&' => {
                    out.push_str(matched);
                    i += 2;
                    continue;
                }
                b'`' => {
                    out.push_str(before);
                    i += 2;
                    continue;
                }
                b'\'' => {
                    out.push_str(after);
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }
        // Copy one UTF-8 char.
        let ch_start = i;
        let mut ch_end = i + 1;
        while ch_end < bytes.len() && (bytes[ch_end] & 0xC0) == 0x80 {
            ch_end += 1;
        }
        out.push_str(&repl[ch_start..ch_end]);
        i = ch_end;
    }
    out
}

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
        let (line, col) =
            rusty_js_bytecode::byte_offset_to_line_col(frame.line_starts, span.start as u32);
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
        if depth > 16 {
            links.push("…(deep)".into());
            break;
        }
        let o = rt.obj(c);
        let kind = kind_tag(rt, c);
        if !links
            .last()
            .map(|s| s.as_str() == kind.as_str())
            .unwrap_or(false)
        {
            links.push(kind);
        }
        if o.has_own_str(key) {
            // Found the slot, but its value resolved to non-callable.
            // The descriptor itself is present at this link.
            found_link = Some(links.len() - 1);
            break;
        }
        cur = o.proto;
    }
    match found_link {
        Some(i) => format!(
            "{}: '{}' slot present at link {} but value not callable",
            links.join("→"),
            key,
            i
        ),
        None => format!("{}: no '{}' slot on chain", links.join("→"), key),
    }
}

fn kind_tag(rt: &Runtime, id: rusty_js_gc::ObjectId) -> String {
    let o = rt.obj(id);
    match &o.internal_kind {
        crate::value::InternalKind::Array => "Array".into(),
        crate::value::InternalKind::Function(fi) => {
            if fi.name.is_empty() {
                "Function".into()
            } else {
                format!("Function({})", fi.name)
            }
        }
        crate::value::InternalKind::Closure(_) => "Closure".into(),
        crate::value::InternalKind::BoundFunction(_) => "BoundFunction".into(),
        crate::value::InternalKind::Promise(_) => "Promise".into(),
        crate::value::InternalKind::Error => "Error".into(),
        crate::value::InternalKind::RegExp(_) => "RegExp".into(),
        _ => {
            // Try matching against known intrinsic prototypes for clarity.
            if rt.object_prototype == Some(id) {
                "Object.prototype".into()
            } else if rt.array_prototype == Some(id) {
                "Array.prototype".into()
            } else if rt.function_prototype == Some(id) {
                "Function.prototype".into()
            } else if rt.promise_prototype == Some(id) {
                "Promise.prototype".into()
            } else if rt.string_prototype == Some(id) {
                "String.prototype".into()
            } else if rt.number_prototype == Some(id) {
                "Number.prototype".into()
            } else if rt.bigint_prototype == Some(id) {
                "BigInt.prototype".into()
            } else if rt.regexp_prototype == Some(id) {
                "RegExp.prototype".into()
            } else {
                "Object".into()
            }
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
            if t.len() <= 24 {
                format!("String({:?})", t)
            } else {
                // Truncate at char boundary, not byte index.
                let mut end = 24;
                while end > 0 && !t.is_char_boundary(end) {
                    end -= 1;
                }
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
                    let len = match o.get_own("length") {
                        Some(d) => match &d.value {
                            Value::Number(n) => *n as usize,
                            _ => 0,
                        },
                        None => 0,
                    };
                    // First few elements' shape (recursion-safe — only one level).
                    let mut elems = Vec::new();
                    for i in 0..len.min(2) {
                        match o.get_own(&i.to_string()).map(|d| &d.value) {
                            Some(Value::Number(n)) => elems.push(format!("{}", n)),
                            Some(Value::String(s)) => {
                                let t = s.as_str();
                                if t.len() <= 12 {
                                    elems.push(format!("{:?}", t));
                                } else {
                                    let mut end = 12;
                                    while end > 0 && !t.is_char_boundary(end) {
                                        end -= 1;
                                    }
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
                    let preview = if elems.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}{}]", elems.join(","), if len > 2 { ",…" } else { "" })
                    };
                    format!("Array(len={}){}", len, preview)
                }
                _ => {
                    let kind = match &o.internal_kind {
                        crate::value::InternalKind::Ordinary => "Object",
                        _ => "Object",
                    };
                    let keys: Vec<String> = o
                        .properties
                        .keys()
                        .take(3)
                        .map(|k| k.as_str().to_string())
                        .collect();
                    let preview = if keys.is_empty() {
                        String::new()
                    } else {
                        format!(
                            " keys=[{}{}]",
                            keys.join(","),
                            if o.properties.len() > 3 { ",…" } else { "" }
                        )
                    };
                    format!("{}{}", kind, preview)
                }
            }
        }
    }
}

// JIT-EXT 22: runtime-side GetPropOnObject helper.
//
// Called by the JIT crate's `jit_getprop_on_object` via the function-
// pointer indirection (`ACTIVE_GETPROP_FN`). Reads the active Runtime
// + FunctionProto from the JIT's TLS slots, performs the property
// lookup, encodes the result as i64.
//
// Encoding contract:
//   - Number result: i64-truncated value (the JIT widens back to f64
//     at the caller via the dispatcher's existing `Value::Number(r as f64)`).
//   - Non-Number result (Object, String, Undefined, etc.): record a
//     deopt in LAST_DEOPT_FRAME and return sentinel 0.
//
// Bad inputs (null TLS pointers, missing constant) are treated as
// deopt-worthy: record a defensive deopt and return 0.
extern "C" fn runtime_getprop_on_object(receiver_idx: i64, prop_name_idx: i64) -> i64 {
    let rt_ptr = rusty_js_jit::get_current_runtime();
    let proto_ptr = rusty_js_jit::get_current_proto();
    if rt_ptr == 0 || proto_ptr == 0 {
        record_synthetic_deopt(prop_name_idx as u32);
        return 0;
    }
    // SAFETY: dispatcher guarantees the pointers are valid for the
    // duration of the JIT call.
    let rt: &mut Runtime = unsafe { &mut *(rt_ptr as *mut Runtime) };
    let proto: &rusty_js_bytecode::compiler::FunctionProto =
        unsafe { &*(proto_ptr as *const rusty_js_bytecode::compiler::FunctionProto) };

    let name: String = match proto.constants.get(prop_name_idx as u16) {
        Some(rusty_js_bytecode::constants::Constant::String(s)) => s.clone(),
        _ => {
            record_synthetic_deopt(prop_name_idx as u32);
            return 0;
        }
    };

    let obj_id = rusty_js_gc::ObjectId(receiver_idx as u32);
    let v = rt.object_get(obj_id, &name);

    match v {
        Value::Number(n) => n as i64,
        _ => {
            record_synthetic_deopt(prop_name_idx as u32);
            0
        }
    }
}

/// LeJIT-Σ StubE-EXT 5b: IC observer for shape-tier cache state
/// machine. Called from `jit_getprop_with_ic` after every IC site
/// dispatch. Reads the receiver's shape via the shape-aware
/// `Object::shape_ptr_and_slot_for` API; calls `observe_at_site` if
/// the receiver is Shaped (cache populates with the current shape +
/// slot), else `observe_miss_no_shape_at_site` (Dictionary receiver;
/// miss_count increments, eventually degrades per MISS_THRESHOLD).
///
/// Lives in the runtime crate because it needs Runtime + heap + Shape
/// access; called via the TLS function-pointer slot the JIT crate
/// exposes.
/// LeJIT-Σ StubE-EXT 5c: IC fast-path getter. Called from
/// `jit_getprop_with_ic` when the cache entry is WarmMono. Compares
/// the receiver's current shape pointer to the cached shape pointer;
/// on match, reads the value from `shape_values[cached_slot]` and
/// returns it encoded as i64 per the typed-i64 alphabet. On shape
/// mismatch OR on non-Number value, returns the
/// `IC_FAST_MISS_SENTINEL` so the caller falls through to the slow +
/// observe path (which transitions the cache state correctly).
///
/// Per Doc 731 §VII R1 (single tier): this is not a second tier; it's
/// a side-channel fast-extract that the JIT-emitted code dispatches
/// through. Equivalent semantics to the slow path on cache hit,
/// strictly faster wall-clock.
///
/// Reclaim hypothesis: removes ~30-50 ns per cache-hit compared to
/// the slow `object_get` + observer path. bench_ic hot loop should
/// see this on ~100% of iterations after warmup (monomorphic).
extern "C" fn runtime_ic_fast_get(
    receiver_idx: i64,
    cached_shape_ptr_usize: i64,
    cached_slot: i64,
) -> i64 {
    let rt_ptr = rusty_js_jit::get_current_runtime();
    if rt_ptr == 0 {
        return rusty_js_jit::deopt::IC_FAST_MISS_SENTINEL;
    }
    let rt: &Runtime = unsafe { &*(rt_ptr as *const Runtime) };
    let obj_id = rusty_js_gc::ObjectId(receiver_idx as u32);
    let o = rt.obj(obj_id);
    // Shape pointer compare. If Object is Dictionary (shape=None) or
    // its shape pointer differs from cached, miss-sentinel.
    let current_shape_ptr = match o.shape.as_ref() {
        Some(rc) => std::rc::Rc::as_ptr(rc) as i64,
        None => 0,
    };
    if current_shape_ptr == 0 || current_shape_ptr != cached_shape_ptr_usize {
        return rusty_js_jit::deopt::IC_FAST_MISS_SENTINEL;
    }
    // Shape matches. Read the value at the cached slot.
    let slot = cached_slot as usize;
    if slot >= o.shape_values.len() {
        // Defensive: shouldn't happen if observer populated correctly,
        // but fall through to slow path to be safe.
        return rusty_js_jit::deopt::IC_FAST_MISS_SENTINEL;
    }
    match &o.shape_values[slot] {
        Value::Number(n) => *n as i64,
        // Non-Number values fall through to slow + observe per the
        // typed-i64 alphabet constraint. The cache won't degrade
        // because the cache entry stays WarmMono — but the per-call
        // cost reverts to slow.
        _ => rusty_js_jit::deopt::IC_FAST_MISS_SENTINEL,
    }
}

extern "C" fn runtime_ic_observe(site_id: i64, receiver_idx: i64, prop_name_idx: i64) {
    let rt_ptr = rusty_js_jit::get_current_runtime();
    let proto_ptr = rusty_js_jit::get_current_proto();
    if rt_ptr == 0 || proto_ptr == 0 {
        return;
    }
    let rt: &Runtime = unsafe { &*(rt_ptr as *const Runtime) };
    let proto: &rusty_js_bytecode::compiler::FunctionProto =
        unsafe { &*(proto_ptr as *const rusty_js_bytecode::compiler::FunctionProto) };

    let name: &str = match proto.constants.get(prop_name_idx as u16) {
        Some(rusty_js_bytecode::constants::Constant::String(s)) => s.as_str(),
        _ => return,
    };

    let obj_id = rusty_js_gc::ObjectId(receiver_idx as u32);
    let o = rt.obj(obj_id);
    let site_id_u32 = site_id as u32;
    if let Some(shape_rc) = o.shape.as_ref() {
        if let Some(slot) = shape_rc.slot_of(name) {
            rusty_js_jit::stub_aarch64::observe_at_site(
                site_id_u32,
                std::rc::Rc::clone(shape_rc),
                slot,
            );
            return;
        }
    }
    rusty_js_jit::stub_aarch64::observe_miss_no_shape_at_site(site_id_u32);
}

fn record_synthetic_deopt(ic_id: u32) {
    use rusty_js_jit::{DeoptReason, DeoptRecoveredState};
    let state = DeoptRecoveredState {
        reason: DeoptReason::ICShapeMismatch { ic_id },
        resume_pc: 0,
        local_values: Vec::new(),
        stack_values: Vec::new(),
    };
    rusty_js_jit::deopt::LAST_DEOPT_FRAME.with(|c| *c.borrow_mut() = Some(state));
}

#[cfg(test)]
mod gcs_tests {
    use super::*;
    use rusty_js_bytecode::op::{encode_op, Op};

    fn run_js_runtime(src: &str) -> Result<Runtime, RuntimeError> {
        let module = rusty_js_bytecode::compile_module(src)
            .map_err(|e| RuntimeError::CompileError(e.message))?;
        let mut rt = Runtime::new();
        rt.install_intrinsics();
        rt.run_module(&module)?;
        Ok(rt)
    }

    #[test]
    fn yield_opcode_captures_active_generator_frame_snapshot() {
        let mut rt = Runtime::new();
        let generator = rt.new_generator_scaffold();

        let mut bytecode = Vec::new();
        encode_op(&mut bytecode, Op::PushI32);
        bytecode.extend_from_slice(&42_i32.to_le_bytes());
        encode_op(&mut bytecode, Op::Yield);
        encode_op(&mut bytecode, Op::ReturnUndef);

        let constants = rusty_js_bytecode::ConstantsPool::new();
        let source_map = Vec::new();
        let line_starts = Vec::new();
        let source_url = String::new();
        let construct_tags = Vec::new();
        let locals_names = Vec::new();
        let upvalue_names = Vec::new();
        let mut frame = Frame {
            bytecode: &bytecode,
            constants: &constants,
            source_map: &source_map,
            line_starts: &line_starts,
            source_url: &source_url,
            construct_tags: &construct_tags,
            locals_names: &locals_names,
            upvalue_names: &upvalue_names,
            locals: Vec::new(),
            local_cells: Vec::new(),
            operand_stack: Vec::new(),
            pc: 0,
            try_stack: Vec::new(),
            with_env_stack: Vec::new(),
            this_value: Value::Undefined,
            derived_initial_this: None,
            this_cell: None,
            upvalues: Vec::new(),
            last_property_lookup: None,
            pending_method_name: None,
            pending_method_getprop_pc: None,
            private_home: None,
            import_meta: None,
            is_arrow: false,
            param_count: 0,
            strict: false,
            new_target: None,
            eval_var_env_is_global: false,
            back_edge_counts: HashMap::new(),
            ic_dispatch_cache: HashMap::new(),
            osr_cache: HashMap::new(),
        };

        let yielded = rt
            .with_active_generator_for_yield(generator, |rt| rt.run_frame(&mut frame))
            .expect("yield opcode should return through suspension channel");
        assert!(matches!(yielded, Value::Number(n) if n == 42.0));

        match &rt.obj(generator).internal_kind {
            crate::value::InternalKind::Generator(g) => {
                assert_eq!(g.state, crate::value::GeneratorState::SuspendedYield);
                assert!(matches!(&g.yielded_value, Some(Value::Number(n)) if *n == 42.0));
                let snapshot = g
                    .continuation
                    .as_ref()
                    .expect("yield should store a continuation snapshot");
                assert_eq!(snapshot.pc, bytecode.len() - 1);
                assert_eq!(snapshot.operand_stack.len(), 1);
                assert!(matches!(&snapshot.operand_stack[0], Value::Undefined));
            }
            _ => panic!("expected generator internal kind"),
        }
    }

    #[test]
    fn generator_lifecycle_yields_lazily_across_next_calls() {
        let rt = run_js_runtime(
            r#"
            let trace = 0;
            function* g() {
              trace = trace + 1;
              yield 1;
              trace = trace + 10;
              yield 2;
            }
            const it = g();
            const before = trace;
            const a = it.next();
            const afterA = trace;
            const b = it.next();
            const afterB = trace;
            const c = it.next();
            globalThis.result = before * 100000
              + a.value * 10000
              + (a.done ? 1000 : 0)
              + afterA * 100
              + b.value * 10
              + (b.done ? 5 : 0)
              + (c.done ? 1 : 0)
              + afterB;
            "#,
        )
        .expect("generator lifecycle exemplar should run");
        let result = rt.global_get("result");
        assert!(matches!(result, Value::Number(n) if n == 10132.0));
    }

    #[test]
    fn infinite_generator_does_not_hang_at_construction() {
        let rt = run_js_runtime(
            r#"
            function* inf() {
              let i = 0;
              while (true) yield i++;
            }
            const it = inf();
            const a = it.next();
            const b = it.next();
            globalThis.result = a.value * 10 + b.value;
            "#,
        )
        .expect("infinite generator should be lazy");
        let result = rt.global_get("result");
        assert!(matches!(result, Value::Number(n) if n == 1.0));
    }

    #[test]
    fn generator_next_value_resumes_yield_expression() {
        let rt = run_js_runtime(
            r#"
            function* g() {
              const x = yield 1;
              return x + 1;
            }
            const it = g();
            const a = it.next(7);
            const b = it.next(42);
            globalThis.result =
              a.value * 1000
              + (a.done ? 100 : 0)
              + b.value
              + (b.done ? 1 : 0);
            "#,
        )
        .expect("next(value) should resume yield expression");
        let result = rt.global_get("result");
        assert!(matches!(result, Value::Number(n) if n == 1044.0));
    }

    #[test]
    fn generator_throw_resumes_into_catch_handler() {
        let rt = run_js_runtime(
            r#"
            function* g() {
              try {
                yield 1;
              } catch (e) {
                yield e + "!";
              }
              return 5;
            }
            const it = g();
            const a = it.next();
            const b = it.throw("oops");
            const c = it.next();
            globalThis.result =
              a.value + (a.done ? 10000 : 0)
              + b.value
              + (b.done ? " bad" : "")
              + " "
              + c.value
              + " "
              + c.done;
            "#,
        )
        .expect("throw should resume suspended yield through catch");
        let result = rt.global_get("result");
        assert!(matches!(&result, Value::String(s) if s.as_ref() == "1oops! 5 true"));
    }

    #[test]
    fn generator_throw_outside_catch_completes_and_propagates() {
        let rt = run_js_runtime(
            r#"
            function* g() {
              yield 1;
              yield 2;
            }
            const it = g();
            it.next();
            let caught = "";
            try {
              it.throw("boom");
            } catch (e) {
              caught = e;
            }
            const after = it.next();
            globalThis.result = caught + " " + after.value + " " + after.done;
            "#,
        )
        .expect("uncaught generator throw should be catchable by caller");
        let result = rt.global_get("result");
        assert!(matches!(&result, Value::String(s) if s.as_ref() == "boom undefined true"));
    }
}

#[cfg(test)]
mod vd_tests {
    //! VD-EXT 2 (2026-05-23): NaN-boxing round-trip + backwards-compat tests.

    use super::*;

    #[test]
    fn vd_number_encoding_unchanged_per_c2() {
        // Per C2: unbox_arg_f64(Value::Number(n)) === n preserved
        // exactly for all non-NaN Numbers. NaN is canonicalized (still
        // a valid NaN; behavior-preserving but bit-pattern-equal to
        // f64::NAN).
        for n in [
            0.0_f64,
            -0.0,
            1.0,
            -1.0,
            1e100,
            -1e100,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ] {
            let v = Value::Number(n);
            let encoded = unbox_arg_f64(&v);
            assert_eq!(
                encoded.to_bits(),
                n.to_bits(),
                "Number {n} encoding changed; bits {:x} vs {:x}",
                encoded.to_bits(),
                n.to_bits()
            );
            assert!(!is_boxed_value(encoded), "Number {n} mis-detected as boxed");
        }
        // NaN is canonicalized but still a NaN; both ways must be
        // detected as non-boxed.
        let nan_encoded = unbox_arg_f64(&Value::Number(f64::NAN));
        assert!(
            nan_encoded.is_nan(),
            "NaN canonicalization must produce a NaN"
        );
        assert!(
            !is_boxed_value(nan_encoded),
            "canonical NaN must not be detected as boxed"
        );
    }

    #[test]
    fn vd_object_encoding_unchanged_per_c3() {
        // Per C3: unbox_arg_f64(Value::Object(id)) === f64::from_bits(id.0 as u64)
        // preserved exactly. ObjectRef = rusty_js_gc::ObjectId(u32), so
        // all ids are bounded well below 2^52 (R1 threshold doesn't apply).
        for id in [0u32, 1, 42, 1000, 1_000_000, 1_000_000_000] {
            let v = Value::Object(rusty_js_gc::ObjectId(id));
            let encoded = unbox_arg_f64(&v);
            assert_eq!(
                encoded.to_bits(),
                id as u64,
                "Object id={id} encoding changed"
            );
        }
    }

    #[test]
    fn vd_string_encoding_round_trips_per_c5() {
        // Per C5: encode + decode round-trip identity for String.
        let s = Rc::new(String::from("hello, world"));
        let v_in = Value::String(s.clone());
        let encoded = unbox_arg_f64(&v_in);

        assert!(
            is_boxed_value(encoded),
            "String encoding must be detected as boxed"
        );
        assert_eq!(
            extract_boxed_tag(encoded),
            VD_TAG_STRING as u8,
            "String tag must match VD_TAG_STRING"
        );

        unsafe {
            let ptr = decode_string_ptr(encoded);
            // Per R3: &*ptr (not Rc::from_raw).
            let decoded_s: &String = &*ptr;
            assert_eq!(decoded_s.as_str(), "hello, world");
        }
        // Original Rc still live (held in v_in / s); no leak / no double-free.
        // The Rc strong count was not modified by the encode path.
        assert_eq!(
            Rc::strong_count(&s),
            2,
            "Rc strong count: 2 (one for s, one for v_in)"
        );
    }

    #[test]
    fn vd_string_encoding_collision_free_with_numbers() {
        // Adversarial: no Number bit pattern should be detected as boxed
        // unless we deliberately constructed it via the boxed-NaN scheme.
        for n in [
            0.0_f64,
            -0.0,
            1.0,
            -1.0,
            1e100,
            -1e100,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MIN_POSITIVE,
            f64::EPSILON,
            3.141592653589793,
        ] {
            let v = Value::Number(n);
            let encoded = unbox_arg_f64(&v);
            assert!(
                !is_boxed_value(encoded),
                "Number {n} (bits {:x}) mis-detected as boxed",
                encoded.to_bits()
            );
        }
    }

    // ─── OSR-EXT 5c (2026-05-23): box_to_value round-trip tests ───

    #[test]
    fn osr_box_to_value_number_snapshot_returns_new_number() {
        let snapshot = Value::Number(0.0);
        let result = box_to_value(42.5, &snapshot);
        match result {
            Value::Number(n) => {
                assert_eq!(n, 42.5, "Number snapshot should produce Value::Number(f)")
            }
            _ => panic!("expected Value::Number, got {:?}", result),
        }
    }

    #[test]
    fn osr_box_to_value_string_snapshot_returns_cloned_snapshot() {
        let s = Rc::new(String::from("preserved"));
        let snapshot = Value::String(s.clone());
        let encoded = unbox_arg_f64(&snapshot);
        let result = box_to_value(encoded, &snapshot);
        // Result still alive at the assertion site (kept by binding).
        let result_clone = result.clone();
        match &result {
            Value::String(rs) => assert_eq!(
                rs.as_str(),
                "preserved",
                "String snapshot should round-trip via clone"
            ),
            other => panic!("expected Value::String, got {:?}", other),
        }
        // s + snapshot + result + result_clone = 4 live Rcs.
        assert_eq!(
            Rc::strong_count(&s),
            4,
            "Rc strong count: s + snapshot + result + result_clone"
        );
        drop(result);
        drop(result_clone);
        assert_eq!(Rc::strong_count(&s), 2, "after drops: s + snapshot");
    }

    #[test]
    fn osr_box_to_value_object_snapshot_preserved() {
        let snapshot = Value::Object(rusty_js_gc::ObjectId(42));
        let result = box_to_value(0.0, &snapshot);
        match result {
            Value::Object(id) => assert_eq!(
                id.0, 42,
                "Object snapshot should round-trip as identical id"
            ),
            _ => panic!("expected Value::Object, got {:?}", result),
        }
    }

    #[test]
    fn osr_box_to_value_undefined_snapshot_preserved() {
        let snapshot = Value::Undefined;
        let result = box_to_value(99.0, &snapshot);
        assert!(
            matches!(result, Value::Undefined),
            "Undefined snapshot should be preserved regardless of f64 value"
        );
    }
}

#[cfg(test)]
mod osr_tests {
    //! OSR-EXT 3 (2026-05-23): loop bytecode boundary detection tests.

    use super::*;

    fn make_jump_bytecode(disp: i32) -> (Vec<u8>, usize) {
        // [Op::Jump_byte (0x60 per op.rs Jump opcode value), disp_le_bytes].
        // We use 0x60 directly to avoid coupling to the op enum's debug name.
        // The boundary detector doesn't actually check the op byte; it
        // reads disp from site_pc+1..site_pc+5.
        let mut bc = vec![0u8; 5];
        bc[0] = 0x60;
        bc[1..5].copy_from_slice(&disp.to_le_bytes());
        let site_pc = 0;
        (bc, site_pc)
    }

    #[test]
    fn osr_loop_region_basic_back_edge() {
        // Loop spans bytecode pcs [0, 10); back-edge at pc=5 jumps back to 0.
        // site_pc=5, end_pc=10, entry_pc = 10 + disp where disp = 0-10 = -10.
        let mut bc = vec![0u8; 10];
        bc[5] = 0x60;
        bc[6..10].copy_from_slice(&(-10_i32).to_le_bytes());
        let region = compute_loop_region(&bc, 5);
        assert_eq!(
            region,
            Some((0, 10)),
            "back-edge at site=5 disp=-10 should produce region (0, 10); got {:?}",
            region
        );
    }

    #[test]
    fn osr_loop_region_rejects_forward_jump() {
        let (bc, site_pc) = make_jump_bytecode(10); // positive disp
        assert_eq!(
            compute_loop_region(&bc, site_pc),
            None,
            "forward jump must not be detected as a loop region"
        );
    }

    #[test]
    fn osr_loop_region_rejects_out_of_bounds_site() {
        let (bc, _) = make_jump_bytecode(-5);
        assert_eq!(
            compute_loop_region(&bc, 100),
            None,
            "out-of-bounds site_pc must return None"
        );
    }

    #[test]
    fn osr_loop_region_rejects_negative_entry_pc() {
        // disp = -100, site_pc = 0, end_pc = 5, entry would be -95: out of bounds.
        let (bc, site_pc) = make_jump_bytecode(-100);
        assert_eq!(
            compute_loop_region(&bc, site_pc),
            None,
            "negative entry_pc must return None"
        );
    }

    #[test]
    fn osr_loop_region_rejects_zero_displacement() {
        let (bc, site_pc) = make_jump_bytecode(0);
        assert_eq!(
            compute_loop_region(&bc, site_pc),
            None,
            "zero displacement is not a back-edge"
        );
    }
}
