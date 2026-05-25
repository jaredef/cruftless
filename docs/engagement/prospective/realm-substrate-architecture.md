# Realm Substrate — Architectural Analysis

**Date**: 2026-05-25 (post-EPSUA arc + SPBC/MPBC + ASCD/ACDPD/RPTP closures; runnable rate ~81.6%; cruft remains single-realm).
**Author**: 2026-05-25 session.
**Status**: prospective — strategic scope for a multi-round architectural workstream.

## I. Question

Cruft is single-realm: one global env, one set of intrinsics, one identity space for `Array === Array`, `Object === Object`, etc. ECMA-262 §9.3 Realm Records define a Realm as the unit of code isolation — each realm has its own `[[Intrinsics]]`, `[[GlobalObject]]`, `[[GlobalEnv]]`, `[[TemplateMap]]`, etc.

Two concrete needs converged this arc:
1. **test262 `$262.createRealm`** (EPSUA-EXT 0.5 deferred candidate; 38 fixtures): spawn a fresh JS realm + return its globalThis, with distinct constructor identities so cross-realm `array instanceof OtherRealm.Array` returns false.
2. **Doc 736 capability-passing-runtime security model**: capabilities must be realm-scoped — a capability handed across a realm boundary must not leak intrinsic identity or shared mutable state. The model's "architecturally-impossible supply chain attack" property *depends on* realm isolation, not just module-scope isolation.

This doc analyzes what Realm substrate requires, decomposes the work, names the architectural decisions, and proposes a multi-round workstream.

## II. What Realm substrate must provide

Per ECMA-262 §9.3 a Realm Record has these slots:
- `[[Intrinsics]]` — own copy of every well-known intrinsic (Object, Array, Function, Error, Promise, ...).
- `[[GlobalObject]]` — the realm's globalThis.
- `[[GlobalEnv]]` — the realm's GlobalEnvironment Record.
- `[[TemplateMap]]` — tagged-template-literal interning per realm.
- `[[HostDefined]]` — host-specific data.

The substrate addition spans:

### II.1 RealmRecord struct + per-realm intrinsics tables

Currently cruft's `Runtime` holds intrinsics directly as fields (`function_prototype: Option<ObjectRef>`, etc.) and `globals: HashMap<String, Value>` as the single global namespace. Need to factor these into a `RealmRecord` struct so multiple Realms can coexist within one Runtime.

```rust
pub struct RealmRecord {
    pub intrinsics: Intrinsics,    // Array/Object/Function/Error/Promise/etc.
    pub global_object: ObjectRef,
    pub global_env: EnvironmentRecord,
    pub template_map: HashMap<TaggedTemplateKey, ObjectRef>,
}

pub struct Runtime {
    pub realms: Vec<RealmRecord>,   // arena of realms
    pub current_realm: RealmIndex,  // which realm is executing
    // ... GC, JIT cache, etc. stay shared at the Runtime tier ...
}
```

### II.2 GetFunctionRealm + cross-realm identity

Every callable function holds its `[[Realm]]` slot. When a function is invoked, the executing realm switches to the function's realm. Cross-realm calls switch the current_realm transparently.

Spec sites that use `[[Realm]]`:
- `GetFunctionRealm` (§10.2.10) — used at constructor invocation, instanceof, ArraySpeciesCreate, %TypedArray% subclassing
- `OrdinaryCreateFromConstructor` (§10.1.13) — proto resolution per realm
- `OrdinaryFunctionCreate` (§10.2.3) — realm slot init

### II.3 instanceof + cross-realm identity

`x instanceof OtherRealm.Array` resolves OtherRealm.Array's prototype, walks x's proto chain. Identity check on Array.prototype.

This already works in single-realm cruft trivially (both protos are the same). In multi-realm, `[].constructor !== OtherRealm.Array`, so `[] instanceof OtherRealm.Array === false`.

### II.4 Capability-passing realm semantics

Per Doc 736, a capability is a handle that can only do what its issuer authorized. Realm boundaries are the architectural mechanism: a capability handed across realms must not let the recipient access the issuer's intrinsics or globals.

Concretely:
- A capability `cap` issued in Realm A and used in Realm B does its work in A's realm (the capability's `[[Realm]]` slot determines execution context).
- Plain values cross realms freely (numbers, strings, booleans).
- Objects (incl. Error, Array, Function) cross — but their `[[Prototype]]` retains its source realm's intrinsic.
- `instanceof` across realms gives the architecturally-meaningful answer: an Error from Realm A is NOT an instance of Realm B's Error class.

### II.5 Global env separation

Each realm's GlobalEnvironment Record has its own `[[VarNames]]` and `[[ObjectRecord]]` (the globalThis). Vars declared in Realm A's eval do not appear in Realm B's globalThis.

## III. Decomposition into substrate rounds

Estimated 6-8 rounds of substantial substrate work.

### Round 1 — RealmRecord struct + single-realm refactor

Introduce `RealmRecord` struct; refactor `Runtime` to hold `Vec<RealmRecord>` + `current_realm: RealmIndex`. Initial state: ONE realm (the primordial); all existing intrinsics + globals move into that realm; ALL existing tests pass unchanged. Pure structural refactor; no observable behavior change. Cost ~400-600 LOC across many files (every site that reads `self.globals` becomes `self.realms[self.current_realm].globals`).

**Discipline**: zero PASS→FAIL regressions per Doc 740 default. This is a substrate-introduction-prefix round per Doc 740 §IV.2 — no observable gain yet; closes the structural prerequisite for everything downstream.

### Round 2 — `[[Realm]]` slot on callable functions

Add `realm: RealmIndex` to FunctionInternals + Closure. Every call switches current_realm to the callee's realm before executing; restores on return. ~100 LOC.

### Round 3 — Realm allocation API + `$262.createRealm`

`Runtime::create_realm()` allocates a fresh RealmRecord with its own copy of every intrinsic (parallel to install_intrinsics but per-realm). `$262.createRealm` exposes this. ~200-300 LOC (intrinsic copy machinery is the bulk).

This is the round where the EPSUA-EXT 0.5 deferred surface closes (~38 createRealm tests).

### Round 4 — instanceof + IsArray + brand-checks across realms

`@@hasInstance`, `IsArray`, and built-in brand-checks (Array, Promise, Map, Set, etc.) must consult the value's source-realm intrinsics, not the current-realm intrinsics. Substantial because many cruft sites compare against `self.globals["Array"]` etc. ~200-300 LOC.

### Round 5 — Spec-compliant ArraySpeciesCreate + species realm

Per §22.1.3.17, ArraySpeciesCreate consults the source array's realm's `%Array%` as fallback. ASCD-EXT 1 used `self.globals.get("Array")` (current realm). Cross-realm tests would break; this round corrects. ~50 LOC + new spec test coverage.

### Round 6 — Capability-passing realm integration

Per Doc 736: capabilities carry `[[Realm]]`. When invoked from a different realm, execute in the capability's realm. Object capability tests in the engagement's capability-pilot suite become testable. ~200 LOC at the capability-grant + capability-invoke sites.

### Round 7 — Error.prototype.stack cross-realm; Promise cross-realm

Error stack-frame Realm tagging; cross-realm Promise resolution (Promise.resolve(otherRealmValue) inherits source realm's [[Realm]]). ~100 LOC.

### Round 8 — Module / eval realm scope

Indirect eval defaults to the current realm; direct eval inherits caller's realm. Module Records have a `[[Realm]]` slot. ~100 LOC.

## IV. Architectural decisions to surface NOW

These decisions shape the substrate; defer them and Round 1 reworks itself.

### IV.1 Realm = Runtime, or Realm < Runtime?

Two designs:
- **A. Realm = Runtime**: each realm is its own `Runtime` instance. createRealm spawns a fresh Runtime; cross-realm calls bridge two Runtimes. Heavy isolation; matches Workers/Isolates model.
- **B. Realm < Runtime**: one Runtime owns multiple Realms (this doc's assumption). Cross-realm calls switch `current_realm` index. Lighter; matches Node.js vm module + Doc 736's "shared Runtime, isolated realms" framing.

**Recommend (B)** for cruft because:
- Shared GC + JIT cache lets cross-realm calls stay cheap.
- Doc 736 capability-passing assumes shared substrate (Runtime) with realm-scoped views.
- $262.createRealm tests don't probe GC isolation; (B) is sufficient.
- (A) is reservable for a later v2 (web-worker-style isolation if needed).

### IV.2 Intrinsic copy: deep clone or factory pattern?

When createRealm allocates a fresh realm, every intrinsic (Array, Object.prototype, Function.prototype, ...) must be a fresh ObjectRef.

- **Deep-clone the primordial realm** (~heavy ObjectRef graph traversal at createRealm time)
- **Factory pattern**: refactor install_intrinsics into a function `install_intrinsics_for(realm: RealmIndex)` and re-run it.

**Recommend factory**: existing install_intrinsics is already monolithic; lift its `self` to `(self, realm_idx)`; instances index through current_realm at runtime.

### IV.3 Current-realm tracking: stack or field?

A function with realm R, invoked from another function in realm R', needs current_realm = R during execution.

- **Stack-based** (RAII): push current_realm at call entry, pop at return.
- **Field with save/restore at call sites**: simpler; matches cruft's existing `current_this` pattern.

**Recommend field + save/restore** (matches existing patterns).

### IV.4 GlobalThis identity across realms

When a script in realm R does `globalThis`, it gets R's globalThis. When that script returns the value to realm R', is the returned globalThis-of-R recognizable as "the globalThis of some realm" or just an opaque Object?

Per spec, globalThis-of-R is just an Object from R's perspective; from R''s perspective it's a foreign Object with R's intrinsics in its proto chain. No special tracking needed — it's just an object that crossed realms.

### IV.5 Module loader realm-scoping

`require('foo')` and `import 'foo'` resolve modules. If realm R imports a module that realm R' already imported, does R get a fresh module instance or share R''s?

Per Doc 736 (capability-passing-runtime), modules must NOT share across realms — that would break the security model (a shared module could be a mutation channel). Each realm has its own module graph.

**This implies a per-realm module cache + per-realm module-loader state.** Round 8 territory.

## V. The arc this substrate enables

Once landed, the substrate unlocks:

- **$262.createRealm test cluster** (~38 fixtures, deferred at EPSUA-EXT 0.5; round 3 closure)
- **Cross-realm test262 fixtures**: many spec tests probe cross-realm `instanceof`, ArraySpeciesCreate, etc. (estimated ~50-80 additional fixtures across rounds 4-5)
- **Doc 736 capability-passing security model**: architecturally-impossible-supply-chain-attack becomes testable via capability isolation tests at the engagement's pilot tier (rounds 6-7)
- **Web Workers / Node.js vm parity**: future runtime surfaces that need realm isolation (rounds 1-8 enable; doesn't ship them)

## VI. Anti-targets

- **Worker isolation** (separate threads, separate heaps): out of scope. Realms are same-thread, same-heap isolated namespaces. Workers are a separate substrate concern.
- **Realm SharedArrayBuffer**: SAB crosses realms by spec; out of scope for v1.
- **Cross-realm Symbol.species / @@toPrimitive dispatch with full spec realm-tagging**: large surface; v1 can defer to "current realm intrinsics" for these and refine in round 5 if needed.

## VII. Resume-vector summary for the new locale

The locale `pilots/realm-substrate/` carries this resume vector through the 8 rounds. Each round closes one Doc 740-multi-tier R. Round-by-round Pred-* predictions and discipline falsifiers per the per-round seeds spawned at locale founding.

The architectural decisions in §IV are made before Round 1; each is a one-line entry in the locale's seed with its rationale.

## VIII. Status

Prospective; awaits keeper authorization to spawn `pilots/realm-substrate/` as a new top-level locale and begin Round 1 (the structural refactor).
