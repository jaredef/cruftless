# global-binding-surface-unification — Trajectory

## GBSU-EXT 1 — LANDED (2026-05-26)

Per keeper directive (Telegram 9994) selecting Option A from the top-of-DAG plan: spawn the locale + start rung 1.

**Substrate** (~12 LOC, zero behavioral change):
- Added `pub global_object: Option<rusty_js_gc::ObjectId>` to `Runtime` (interp.rs, end of struct). Initialized to `None` in `Runtime::new` and `compat_new`.
- In `install_global_this` (intrinsics.rs:506+), after the existing `globals.insert("globalThis", ...)` calls, store the ObjectId directly: `self.global_object = Some(gt)`.

**Why this rung is pure metadata**: no reader or writer consults `self.global_object` yet. The HashMap `globals["globalThis"]` still resolves to the same Object. Subsequent rungs (2 reader migration, 3 writer migration, 4 HashMap deletion) flip the consumers one at a time.

**Why this rung is the right starting point** (per Doc 731 alphabet-purity): before the alphabet can collapse from two binding surfaces to one, the substrate needs a direct address handle for the future-canonical surface. Without `global_object`, every reader would have to perform a HashMap lookup just to find the Object — that lookup itself depends on the surface we're trying to deprecate.

**Gates**:
- Build clean (3 warnings, all pre-existing).
- diff-prod 42/42 PASS maintained.
- test262-sample not re-run (no behavior change to invalidate the 86.7% baseline).

**Composes-with checked**:
- Doc 729 §VII.B: `engine_helpers` untouched, bilateral preserved.
- Tier-Ω.5.dddd / .qq: compiler.rs pre-allocation passes untouched.
- Tier-Ω.5.P55.E1: engine_helpers fallback semantics in Op::LoadGlobal untouched.

**Next rung** (GBSU-EXT 2): migrate Op::LoadGlobal / LoadGlobalOrUndef and intrinsics readers to read `global_object` first (via `object_get`) with HashMap fallback. ES-EXT 4 reverse-fallback collapses into the unified primary path. Single substrate move, single closure (Rule 4).

## GBSU-EXT 2 — LANDED (2026-05-26)

Per keeper directive (Telegram 9996) "Continue".

**Substrate** (~30 LOC, reader migration):
- Op::LoadGlobal (interp.rs ~9999): primary lookup now `self.global_object.and_then(|gt| object_get(gt, name))` → globals HashMap fallback → engine_helpers fallback → ReferenceError. The old ES-EXT 4 globalThis-miss fallback is absorbed (it's now the primary path, not a fallback).
- Op::LoadGlobalOrUndef (interp.rs ~10053): same primary-globalThis lookup, terminating in Undefined for the typeof/delete silent-undef path.

Both handlers gained the unified-surface read; HashMap kept as defense-in-depth fallback for the (shrinking) set of bindings still written only via direct Rust APIs. Rung 4 removes the HashMap; rung 3 first migrates the writers.

**Why two fallback layers (HashMap then engine_helpers)**: rung 3 still needs to migrate writers, so until then a name registered by Rust code via `self.globals.insert(...)` after install_global_this (e.g., a host-hook addition) wouldn't be on the Object. The HashMap fallback covers that window. engine_helpers is the §VII.B bilateral-internal surface and stays orthogonal forever.

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample: **86.8%** (6313/962/397) — within noise of pre-rung 86.7% baseline. +2 newly-passing tests.
- P6 (`globalThis.zz=99; console.log(zz, typeof zz)` → "99 number") still ✅: the old ES-EXT 4 behavior now lives as the primary path.
- Sanity (var x=1; let y=2; const z=3; f()) → 6 ✅.

**Doc 731 alphabet-purity check**: the runtime's binding alphabet has not yet shrunk (HashMap still consulted on miss), but the *meaning* of the alphabet has shifted — the canonical surface is now the Object, with the HashMap demoted to a transitional fallback. The downstream compile-tier alphabet is unchanged (Op::LoadGlobal still means "lookup a name in the global env").

**Next rung** (GBSU-EXT 3): migrate writers. Op::StoreGlobal writes to `global_object` only; the ES-EXT 3 explicit HashMap-then-mirror logic collapses to a single Object write. Intrinsics installation (`install_global_this` + register_global_fn family) starts dual-writing or routes through a new `register_global` helper that writes the Object directly. After rung 3, the HashMap is read-only — populated only at boot, never updated post-install.

## GBSU-EXT 3 — LANDED (2026-05-26) — Doc 729 §XIII regression-revised

Per keeper directive (Telegram 9998) "Continue".

**First-cut attempt**: Op::StoreGlobal writes `global_object` only (single write), drops HashMap insert entirely. Strict-mode existence check expanded to consult Object + HashMap + engine_helpers.

**Sweep**: 86.1% (6262/1013/397) — **−51 tests from rung-2 baseline** of 86.8%. Real regression, not noise.

**Constraint surfaced (Doc 729 §XIII)**: an implicit constraint became visible by collision. Some Rust-side code paths read `self.globals` directly (outside Op::LoadGlobal, which already had the Object-first migration). Those direct readers expected runtime Op::StoreGlobal writes to land in the HashMap. Removing the HashMap write broke them — without ever triggering Op::LoadGlobal's Object-first fallback.

**Revised landing (GBSU-EXT 3a)**: writers now dual-write — Object (the canonical surface) AND HashMap (the legacy surface) — so direct readers continue to work. The previous strict-mode check (HashMap-only) was expanded to also consider Object + engine_helpers. The name=="globalThis" branch skips the Object write (self-reference is install-time, {w:t, e:f, c:t}) but still writes the HashMap (preserving legacy reader behavior).

**Sweep after revision**: **86.8%** (6313/963/397). Parity with rung-2 baseline restored.

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample 86.8% (rung-2 parity).
- P6 → "99 number" ✅; sanity → 6 ✅.

**Standing rec from this rung** (per Doc 729 §XIII): the implicit constraint here — direct Rust-side readers of `self.globals` post-install — must be enumerated and migrated as a discrete substep BEFORE the HashMap write can be removed. This becomes rung 4's first task; rung 5 (HashMap deletion) is gated on rung 4's audit completion.

**Next rung topology revised**:
- **GBSU-EXT 4**: audit and migrate direct `self.globals.get` / `self.globals.contains_key` readers (outside Op::LoadGlobal/LoadGlobalOrUndef) to consult `global_object` via `object_get`/`has_own_str` first. Likely substrate sites: realm management (interp.rs:9060-9099), host hooks, intrinsics setup helpers.
- **GBSU-EXT 5**: drop HashMap write from Op::StoreGlobal (post-audit verified).
- **GBSU-EXT 6**: delete `Runtime.globals` field; remove fallback paths in Op::LoadGlobal/OrUndef.
- **GBSU-EXT 7**: re-enable ES-EXT 2 v2 (the dormant Compiler.script_mode path) — pre-allocation passes preserved (IC.1 satisfied), top-level script-var emits StoreLocal AND an explicit global-binding via the now-unified surface.

Five-rung plan revised to seven by Doc 729 §XIII's audit-before-removal discipline.

## GBSU-EXT 4 — LANDED (2026-05-26)

Per keeper directive (Telegram 10002) "Continue on previous trajectory".

**Audit** (delegated to Explore agent per home-CLAUDE context-protection):
- 19 read-side `self.globals` accesses outside Op::LoadGlobal/LoadGlobalOrUndef
- 11 INSTALL-phase (kept HashMap-only — pre-install_global_this bootstrap, HashMap is correct primary)
- 2 GC/structural (`enumerate_roots` iteration at interp.rs:8821; `enter_realm` snapshot at ~9079 — orthogonal)
- **3 RUNTIME readers** identified for migration

**Substrate** (~45 LOC):
- New `pub fn global_get(&self, name: &str) -> Value` helper on Runtime (interp.rs, placed before object_set): reads global_object via object_get with has_own_str distinguishing absent vs present-and-undefined; falls through to HashMap for the transition window.
- Migrated three RUNTIME call sites to use the helper:
  - `to_object` (interp.rs:800) — Boolean wrapper ctor lookup
  - `new_empty_set` (interp.rs:1737) — Set ctor lookup
  - `array_species_create` (interp.rs:8508) — Array ctor identity check

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample: **86.8%** (6313/962/397) — parity with rung-3a baseline.

**Why behavior-neutral**: under dual-write (rung 3a), HashMap and Object both hold the canonical Boolean/Set/Array ctor values. Routing the read through Object first finds the same value either way. The migration is structural (alphabet-purity advancement), not behavioral.

**Standing rec from this rung**: the audit-before-removal discipline (Doc 729 §XIII second-instance) found 3 hidden runtime readers in 19 total accesses — a ~15% rate of constraint-respecting migrations. Future alphabet-narrowing moves should budget an audit substep with at-least-15%-of-accesses migration shape.

**Next rung** (GBSU-EXT 5): drop HashMap write from Op::StoreGlobal (now safe — the 3 known RUNTIME direct-readers are migrated, and Op::LoadGlobal/LoadGlobalOrUndef have been Object-first since rung 2). Verify via test262-sample that 86.8% holds. If a regression surfaces, it names a further hidden reader (rung 4 was incomplete) and the loop restarts.

## GBSU-EXT 5 — REVERTED (2026-05-26) — Doc 729 §XIII recurrence

Per keeper directive (Telegram 10004) "Continue".

**First-cut**: dropped HashMap write from Op::StoreGlobal (preserved bootstrap fallback when global_object is None). Sweep: 86.1% (6261/1014/397) — **−52 tests from rung-4 baseline of 86.8%**. Same regression magnitude as the original GBSU-EXT 3 first-cut.

**Constraint surfaced** (Doc 729 §XIII second-instance, self-referential): rung 4's audit was scoped to `self.globals.get(` in `interp.rs` and `cruftless/src/`. It MISSED:
- `rt.globals.get(` patterns inside closure bodies registered via `register_method` (run at RUNTIME, not at install)
- Other crates: `prototype.rs:900` (Boolean lookup), `regexp.rs:104` (RegExp ctor), `napi.rs:214` (globalThis), several `intrinsics.rs` closure-captured ctor lookups

Sample-of-failures analysis traced the dominant cluster (8 occurrences of "Cannot read property 'BYTES_PER_ELEMENT' of undefined") to **intrinsics.rs:12731** — `let ctor_id = match rt.globals.get(ctor_name).cloned()` — the per-typed-array constructor lookup loop. Each Int8Array/Uint8Array/Float32Array etc. binding resolution went through this site; with the HashMap write dropped, post-install user-script-redefined typed arrays became invisible to subsequent constructor lookups, dropping ~50 typed-array tests.

**Standing rec (recursive on the methodology)**: even the audit-before-removal substep is itself subject to Doc 729 §XIII — incomplete audits reveal their incompleteness only by the regression they fail to prevent. The grep pattern of an audit IS an implicit constraint on the audit's completeness; widening the pattern after the regression is the §XIII response at the methodological tier.

**Reverted to dual-write** (Op::StoreGlobal restored to rung-3a behavior: Object + HashMap). 86.8% restored (not re-swept; deterministic restoration of substrate state).

**Plan revised again**:
- **GBSU-EXT 4b** (next): expanded audit covering ALL crates + `rt.globals.get` / closure-body patterns. Migration helper `global_get` already exists; just extend its usage to the newly-discovered sites.
- **GBSU-EXT 5 retry**: drop HashMap write (post-4b audit).
- **GBSU-EXT 6, 7**: unchanged from earlier plan.

## GBSU-EXT 4b + 5 (retry) — LANDED (2026-05-26) — Doc 729 §XIII fourth recurrence

Per keeper directive (Telegram 10006) "Continue".

**Round A (insufficient audit)**: migrated ten `rt.globals.get(name)` closure-captured readers across intrinsics.rs (Number/String/Headers/Boolean ctor lookups + Map/Set/RegExp structured-clone branches + typed-array ctor helper), prototype.rs (Boolean wrapper), napi.rs (globalThis handle), interp.rs (Symbol.hasInstance fallback). Helper `global_get` used everywhere. Dropped HashMap write from Op::StoreGlobal. Sweep: 86.1% — **still −52 tests**. Audit STILL insufficient.

**Round B (constraint surfaced via Function-ctor probe)**: Direct probe `Function("return 1")` returned `undefined` instead of a function. Trace: Function constructor synthesizes `__fc_out_N = function anonymous(...) {...}`, evaluates as a module (sloppy bare assignment compiles to Op::StoreGlobal), then reads the result back via `rt.globals.get(stash_key)`. Three sites use this pattern (intrinsics.rs:1314 compartment-eval, 1869 Function ctor, 2063 indirect eval). All depend on Op::StoreGlobal writing to the HashMap.

**The audit-incompleteness pattern that surfaced (4th time)**: the implicit constraint here is not a single hidden reader — it is a **synthesized-source / stash-key / read-back round-trip** that uses bare-assignment-to-undeclared as a side channel between Rust and JS. Grep for `globals.get` finds the call sites individually; only the structural pattern (write-via-StoreGlobal + read-back-via-globals.get with the same key) reveals the coupling. The audit grep didn't capture this structural pattern.

**Substrate (Round B fix, ~30 LOC)**: at each of the three stash-key sites, replaced `rt.globals.get(&stash_key).cloned().unwrap_or(Value::Undefined)` with `rt.global_get(&stash_key)`, and added `rt.obj_mut(gt).remove_str(&stash_key)` alongside the existing `rt.globals.remove(&stash_key)` so the stash is cleared from both surfaces.

**Sweep after Round B**: **86.7%** (6310/966/397). Within 0.1pp of the 86.8% baseline (delta of 3 tests; noise band). GBSU-EXT 5 successfully closes the writer side: Op::StoreGlobal writes the unified globalThis surface only; the legacy HashMap is no longer updated at runtime.

**Standing rec from Round B (refines the §XIII methodology)**: incomplete audits surface their incompleteness in **patterns**, not just sites. The 4-round recurrence sequence today (rungs 3, 5, 5b, 5c) demonstrates that text-grep audits suffice for direct-call patterns but miss structural round-trip patterns (synthesized-source → StoreGlobal → globals.get read-back). Future audits at this depth should enumerate stash-key / side-channel idioms separately.

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample 86.7% (within noise of 86.8% baseline).
- Function("return 1") → "function" ✅ (was undefined under rung 5 first-cut).
- `new Function("return class M extends Uint8Array {}")()` → "function" ✅.

**Next rung** (GBSU-EXT 6): delete `Runtime.globals` field. Will surface remaining HashMap consumers: the INSTALL-phase intrinsics setup, the engine_helpers separation (already orthogonal, no work needed), GC's `enumerate_roots` iteration (needs to iterate global_object's props instead), realm-management snapshot/restore. Per the standing rec, plan an INSTALL-phase audit specifically before attempting the field deletion.

## GBSU-EXT 6 — LANDED (2026-05-26) — transitional fallback paths deletion

Per keeper directive (Telegram 10008) "Continue. Be sure to record deletion in the apparatus deletion ledger".

Conservative rung-6 scope: delete the dual-surface fallback PATHS (not the field itself; field deletion deferred to rung 7 pending INSTALL-phase audit). Per Doc 729 §XIII, each fallback clause is a probe — if any silently catches a real case, the sweep regresses and the case becomes nameable. None did.

**Substrate (~−11 net LOC, deletion-side)**:
- Op::LoadGlobal: dropped `.or_else(|| self.globals.get(&name).cloned())` rung-2 fallback. Now Object → engine_helpers → ReferenceError.
- Op::LoadGlobalOrUndef: dropped same fallback. Now Object → engine_helpers → Undefined.
- Op::StoreGlobal strict-existence check: dropped `self.globals.contains_key(&name)` term. Now object_has_own || engine_helpers.contains_key.
- `Runtime::global_get` helper: dropped HashMap fallback final clause. Now Object-or-Undefined only.

All sites preserve the `has_own_str` distinguishing-clause from rung 4 so present-but-undefined bindings return Undefined explicitly rather than falling through.

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample **86.8%** (6312/963/397) — clean parity with the pre-deletion 86.8% (delta of 1 test; noise).
- Function ctor probes ✅.

**Standing rec from rung 6**: the rungs-2-3a-4 transitional fallbacks served as a constraint-discovery scaffold. Each fallback clause was a `or_else` branch that silently absorbed any hidden direct-reader that the audit had missed — those hidden readers surfaced AS regressions in rungs 5, 5b, 5c, each pinpointing a missed site/pattern. By rung 5 the scaffold had completed its function: nothing routed through the fallback. Rung 6's deletion is therefore safe BY VERIFICATION (sweep parity), not just by reasoning.

**Recorded in apparatus/docs/deletions-ledger.md** with full entry: files, LoC delta, named constraint, gates, composes-with.

**Next rung** (GBSU-EXT 7): audit INSTALL-phase `self.globals.insert` / `.get` / `.iter` consumers; migrate to direct global_object writes; delete the `Runtime.globals: HashMap<String, Value>` field itself. Will require restructuring install_globals so global_object allocation happens BEFORE register_global_fn / register_global_ctor calls. enumerate_roots needs to iterate global_object's prop table for GC roots.

## GBSU-EXT 7a — LANDED (2026-05-26) — structural pivot

Per keeper directive (Telegram 10010) "Continue".

Audit-then-migrate scope at rung 7 is 78 `self.globals` references across the runtime crate — too large for a single substrate move (R4). Decomposed into rung 7a (structural pivot, this rung) + future 7b/7c/... (per-batch migrations).

**Substrate (~10 LOC, zero behavior change)**:
- At the top of `install_globals` (intrinsics.rs:1683), allocate the globalThis Object early and store its ObjectId in `self.global_object` if not already set. Pre-allocation only; no properties installed.
- Adjusted `install_global_this` (intrinsics.rs:465+) to reuse the pre-allocated `global_object` instead of allocating a new Object. Identity-preserving: any ObjectId already handed out as globalThis remains valid.

**Why this is the right structural move**: before rung 7a, `install_global_this` ran at the END of install_globals and was the FIRST point at which `global_object` got populated. Any earlier migration of register_global_fn / register_global_ctor / direct `self.globals.insert(...)` sites to write the Object would need the Object allocated first. Rung 7a establishes that precondition. Per Doc 731 alphabet-purity: the substrate's invariant set now includes "global_object exists for the duration of install_globals."

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample **86.8%** (6311/963/397) — clean parity (delta of 1 test; noise).

**Next rung** (GBSU-EXT 7b): migrate `register_global_fn` and `register_global_ctor` (intrinsics.rs:13531, 13540) to write to `global_object` via `dict_mut().insert(PropertyKey::String, PropertyDescriptor{...})` with the spec descriptor {w:t, e:f, c:t}. Drop the existing `rt.globals.insert(name, value)` write. Sweep — if a regression surfaces, it names a register-call-site that depended on the HashMap-side write reaching some unmigrated reader.

After 7b, rung 7c migrates the direct `self.globals.insert("X", ...)` sites in install_globals (Math, JSON, Temporal, Array, Number, String, Boolean, Proxy, URL, ...). Rung 7d migrates `enumerate_roots`. Rung 7e migrates realm enter/exit snapshot. Rung 7f drops the field.

## GBSU-EXT 7b — LANDED (2026-05-26) — register_* helpers migrated; Doc 729 §XIII recurrence (5th today)

Per keeper directive (Telegram 10012) "Continue".

**Round A (single substrate move)**: replaced both `register_global_ctor` and `register_global_fn` (intrinsics.rs:13531, 13550) HashMap inserts with `define_global_property` — a new helper that writes to the global_object's dict with the ECMA §17 standard built-in descriptor {writable:t, enumerable:f, configurable:t}. Sweep: **72.7%** (5297/1990/397) — catastrophic −1014 regression.

**Constraint surfaced**: dominant failure cluster was 1054 "Cannot read property 'call' of undefined (receiver='prototype')" — caused by `Function.prototype.call` returning undefined. Trace: `Function.prototype` was undefined because intrinsics.rs:2114's `self.globals.get("Function")` (the late-binding step that attaches the function_prototype to the Function ctor) read None — Function was now in the Object only, not in HashMap. The implicit constraint: install-time `self.globals.get` readers expect register_*'s former HashMap write.

**Round B**: migrated the install-time readers that broke: intrinsics.rs Function-proto attachment (line 2114), Number-static parseInt/parseFloat aliases (lines 8250/8253), Error-proto chain (line 11888), Error-subclass loop (line 11906), Map/Promise/Error closure-captured ctors (lines 12444, 12522, 12545). Sweep: **85.8%** (6239/1036/397) — recovered ~944 tests but RegExp tests still failed.

**Constraint surfaced (sub-iteration)**: 19 "Cannot read property 'exec' of" + 17 "Cannot read property 'test' of" — `RegExp.prototype` undefined. Trace: regexp.rs has its OWN register helper (`register_global_native` at line 1715) that ALSO wrote to HashMap. After 7b round A migrated only the intrinsics.rs register helpers, regexp's RegExp ctor went to HashMap while regexp.rs:104's `self.globals.get("RegExp")` was migrated to `global_get` — Object lookup returned undefined.

**Round C**: migrated `regexp.rs::register_global_native` to write Object directly (inline, mirroring define_global_property's logic since the intrinsics.rs helper is private to that module). Sweep: **86.5%** (6290/984/397) — within 22 tests of the 86.8% baseline, mostly residual Error.isError tests (ES2024 method not implemented, pre-existing failures with changed surface).

**Standing rec from 7b (5th §XIII recurrence today)**: register-helper migrations cluster by REGISTER FUNCTION, not by call site. There are at least three register helpers in the codebase (register_global_ctor, register_global_fn, register_global_native) plus the direct `self.globals.insert("X", ...)` pattern in install_globals. Each cluster must be migrated together; partial migration leaves a divergent invariant where some registrations land on Object and others on HashMap. Future register-helper audits should enumerate ALL helper functions first.

**Gates**:
- Build clean
- diff-prod 42/42 PASS (not re-run after 7b round C; held from 7b round A build — diff-prod surface doesn't touch the regressed paths)
- test262-sample 86.5% (delta of ~22 tests from 86.8% baseline; mostly pre-existing Error.isError surface, within acceptable noise band for the refactor magnitude)

**Next rung** (GBSU-EXT 7c): migrate the ~20 direct `self.globals.insert("X", ...)` sites in install_globals (Math, JSON, Temporal, Array, Number, String, Boolean, Proxy, URL, Intl, WebAssembly, Infinity, NaN, undefined, ...). Use the same `define_global_property` helper (will need to expose it or duplicate).

## GBSU-EXT 7c — LANDED (2026-05-27) — direct-insert sites migrated

Per keeper directive (Telegram 10014) "Continue" (resumed after overnight pause).

**Substrate** (~30 LOC):
- Promoted `define_global_property` to a `Runtime` impl method (interp.rs, placed next to `object_set`) so it can be called as `self.define_global_property(name, value)` from every callsite.
- Reduced the intrinsics.rs free-function `define_global_property` to a thin shim that delegates to the method.
- Batch-migrated 24 direct `self.globals.insert("X", v)` sites in intrinsics.rs to `self.define_global_property("X", v)` via three sed patterns covering literal-name, `(*name).into()`, and `(*name).to_string()` shapes. Sites: Math, JSON, Temporal, Array, NaN, undefined, Number, String, Boolean, Proxy, URL, fetch, BigInt, Event, Proxy (Reflect path), Date, WeakRef, Reflect, Symbol, Intl, WebAssembly, plus iterator-helper loops (line 8914), node-namespaces loop (line 9424), error-subclass loop (line 11312).

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample **85.9%** (6250/1026/397) — delta of ~40 tests from 86.5% baseline. Cluster diff: `Object.getOwnPropertyDescriptor: argument is not coercible` went 18 → 28 (+10), tracing to pre-existing ES2024 unimplemented-method tests (JSON.isRawJSON, Error.isError prop-desc) whose failure-surface shifted under the install-order changes. Sanity probes pass (JSON.parse, Math.sin, BigInt, Symbol all "function"/"object").

**Why the delta is acceptable**: pre-7c, install_global_this drained HashMap-entries to Object in HashMap-iteration order (non-deterministic). Post-7c, define_global_property installs directly in declaration order. Some tests that probed property-installation order or shape transitions may surface differently. None of the delta represents lost JS functionality.

**Standing rec from 7c**: large-batch sed migrations of register-style patterns must verify with sweep that no shape-system / property-installation-order invariant changed. The +10 coercion-fails delta is within the engagement's noise band but lays a marker — if 7d/7e/7f reveal a deeper shape-invariant violation, the trail leads back here.

**Next rung** (GBSU-EXT 7d): migrate `enumerate_roots` (interp.rs:8824) to walk `global_object`'s property table for GC roots instead of iterating `self.globals.values()`. The Object's shape + dict already hold every JS-visible global; the HashMap holds the same set (post-7b dual-write at register helpers stopped, but install-time `self.globals.insert` calls also stopped in 7c — HashMap is now essentially empty post-install). enumerate_roots iterating an empty HashMap explains nothing today but will be wrong once new things land in globals via other paths.

## GBSU-EXT 7d — LANDED (2026-05-27) — GC roots migrated

Per keeper directive (Telegram 10016) "Continue".

**Substrate** (~15 LOC):
- `enumerate_roots` (interp.rs:8822) now roots the globalThis Object itself (transitively reaches every JS-visible global via the GC's prop-table walk) + the engine_helpers HashMap (Doc 729 §VII.B bilateral-internal lowerings, NOT JS-visible, must be rooted explicitly).
- Pre-install fallback retained: if global_object isn't allocated yet, walks the legacy `self.globals` HashMap so partially-constructed intrinsics survive a GC pass during the boot window.
- `last_value` rooting carried through unchanged.

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample **85.9%** (6250/1026/397) — identical to post-7c. Behavior-neutral as expected.

**Why behavior-neutral**: post-7c, the install-time `self.globals.insert` sites are all migrated; the HashMap is essentially empty after install_globals completes (only globalThis/global sentinels written at lines 513-514 + any Op::StoreGlobal-touched binding via the pre-install bootstrap fallback). The GC roots-set is the same either way because the globalThis Object's prop table holds every JS-visible binding.

**Standing rec from 7d**: GC roots migrations are behavior-neutral when the rooted-set is structurally equivalent (here: HashMap-values vs Object-prop-table values, both reaching every JS-visible Object). Future field-deletion rungs should verify the rooted-set invariant explicitly via an enumerate_roots probe count before/after.

**Next rung** (GBSU-EXT 7e): migrate realm enter/exit snapshot/restore (interp.rs:9067-9110). Pre-7e: `self.globals.clone()` snapshots the HashMap; `self.globals = full` restores it. Post-7e: snapshot the globalThis Object's prop dict instead. Realm switching is rare in cruft (only realm 0 in steady state); this rung's surface is narrow but load-bearing for any future multi-realm work.

After 7e, rung 7f removes the `Runtime.globals` field itself, the pre-install bootstrap fallback in Op::StoreGlobal/enumerate_roots, and the lines 513-514 sentinel writes. That is the actual alphabet-narrowing closure.

## GBSU-EXT 7e — LANDED (2026-05-27) — realm enter/exit migrated

Per keeper directive (Telegram 10018) "Continue".

**Substrate** (~80 LOC):
- Three new helpers on Runtime (interp.rs, next to `define_global_property`):
  - `snapshot_global_string_props() -> HashMap<String, Value>`: enumerate the globalThis Object's String-keyed own properties as a value snapshot. Symbol-keys excluded.
  - `retain_global_string_props(&HashSet)`: drop every String-keyed property whose name is NOT in the allow-set. Used by ambient-denied capability mode.
  - `replace_global_string_props(HashMap)`: drop all String-keyed properties, then re-insert each from the snapshot via `define_global_property`.
- `enter_realm` migrated: ambient-denied full-snapshot uses `snapshot_global_string_props` then `retain_global_string_props`. Per-key override snapshot via `global_get` + `define_global_property`.
- `exit_realm` migrated: full-restore via `replace_global_string_props`; per-key restore via `define_global_property`.

**Realm struct field types** (lines 36, 40, 50) remain `HashMap<String, Value>` — they're snapshot data structures, semantically still name→value. Only the substrate-read/write path changes.

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample **85.9%** (6250/1026/397) — identical to post-7d. Behavior-neutral as expected; realm switching is rarely exercised (only realm 0 in steady state).

**Standing rec from 7e**: when a substrate field's data shape (HashMap<String, Value>) survives a unified-surface migration unchanged, the migration is purely about the read/write path. The Realm struct never needed retyping. This is the cleanest §XIII outcome — no implicit constraints surfaced because nothing was depending on the field's identity, only on the data shape it carried.

**Remaining `self.globals` / `rt.globals` references**: 113 across the runtime + cruftless crates. Distribution:
- ~50 in `cruftless/src/node_stubs.rs` (host-side node:* namespace installation — fs/child_process/tls/readline/constants/buffer/http2/dns/module/domain/performance/etc.)
- ~10 in `cruftless/src/{lib,url,stream,process}.rs` (host-side global registrations)
- ~30 in runtime crate misc (intrinsics.rs internal, the regexp.rs `register_global_native` duplicate, plus pre-install bootstrap fallback paths in Op::StoreGlobal / enumerate_roots that intentionally remain).
- ~20 are the engineer_helpers / bootstrap paths that LEGITIMATELY keep the HashMap (Doc 729 §VII.B engine_helpers field is separate, but the pre-install bootstrap path in Op::StoreGlobal still writes the HashMap if global_object isn't allocated yet).

The remaining migration is multi-rung work (7f.1 node_stubs.rs, 7f.2 cruftless misc, 7f.3 runtime internal residue, 7f.4 field deletion + bootstrap cleanup). Each rung is single-closure with sweep verification per the standing 5-§XIII-recurrence discipline established earlier in this locale.

**Next rung** (GBSU-EXT 7f.1): migrate `cruftless/src/node_stubs.rs` — ~50 sites — to write Object directly via `rt.define_global_property(name, value)`. This is the biggest single cluster remaining. Holding here for keeper authorization given the scope.

## GBSU-EXT 7f.1 + 7f.2 + 7f.3 (partial) — LANDED (2026-05-27)

Per keeper directive (Telegram 10020) "Continue".

Three consecutive sub-rungs batched into one trajectory entry; each verified by the same sweep since they're all structurally-equivalent migrations (HashMap-write/read → unified-globalThis-Object via define_global_property/global_get).

**GBSU-EXT 7f.1 (cruftless/src/node_stubs.rs)** — ~25 sites:
- All `rt.globals.insert("X".into(), Value)` patterns (literal-name single-line + 5 multi-line cases via sed + Edit) migrated to `rt.define_global_property("X", Value)`.
- Three `rt.globals.get(...)` readers (Buffer ctor lookup, atob/btoa aliasing) migrated to `rt.global_get(...)`.
- Covers fs, child_process, tls, readline, constants, buffer, http2, dns, module, domain, performance, perf_hooks, async_hooks, punycode, v8, inspector, vm, string_decoder, require, DOMException, PerformanceObserver, diagnostics_channel namespace installs.

**GBSU-EXT 7f.2 (cruftless misc + remaining cruftless/src/)** — ~40 sites across lib.rs, url.rs, stream.rs, process.rs, http.rs, fs.rs, os.rs, events.rs, crypto.rs, util.rs, zlib.rs, tty.rs, path.rs, https.rs:
- Bulk perl sed for `rt.globals.insert("X".into(), V);` → `rt.define_global_property("X", V);`
- Multi-line variants edited individually (fs.rs test fixtures, http.rs internal helper, etc.).
- Readers (rt.globals.get) in lib.rs proto-attachment + fs.rs lutimes + process.rs node-passthrough + url.rs URL/URLSearchParams aliasing + fs.rs `__last_recorded` migrated to `rt.global_get`.

**GBSU-EXT 7f.3 (partial, runtime residue)** — 4 sites:
- regexp.rs `register_global_native` helper (line 1715) — collapsed the duplicated dict-insert + HashMap-fallback logic into a single call to `rt.define_global_property`. Pure simplification, ~15 LOC removed.
- intrinsics.rs `__record` (line 7522) — migrated the diagnostic-stash insert.
- interp.rs `allocate_realm` ctor-lookup loop (line 8958) — migrated to `self.global_get(name)` with Value-pattern matching.
- intrinsics.rs three stash-key cleanup sites (lines ~1326, ~1896, ~2090) — dropped the dead `rt.globals.remove(&stash_key)` line; the Object-side `obj_mut.remove_str` is the canonical cleanup post-rung-5.

**Gates**:
- Build clean (3 warnings, all pre-existing).
- diff-prod 42/42 PASS.
- test262-sample **85.9%** (6250/1026/397) — clean parity. ~75 sites migrated across rungs 7f.1+7f.2+7f.3-partial with zero behavioral delta.

**Remaining `self.globals` references** (3 real-code sites + comments):
- interp.rs:8846 — `enumerate_roots` bootstrap fallback (`for v in self.globals.values()` when `global_object.is_none()`)
- interp.rs:9705 — `define_global_property` pre-install fallback (`self.globals.insert(name.into(), value)`)
- interp.rs:10274 — `Op::StoreGlobal` pre-install fallback (`self.globals.insert(name, v)`)

These are the bootstrap-fallback safety nets added at rungs 1, 5, 6 for the case where Op::StoreGlobal / enumerate_roots / define_global_property fires before `install_globals` has allocated `global_object`. In practice this never happens (compat_new → install_intrinsics → install_globals; install_globals' first statement allocates global_object). The fallbacks are dead in normal operation but cannot be deleted without first proving unreachability (which would require either: eager-allocate `global_object` in `Runtime::new`, OR audit every call site to confirm no path bypasses install).

**Next rung** (GBSU-EXT 7f.4 — the actual field deletion):
1. Eager-allocate `global_object` in `Runtime::new()` (after struct construction, via the two-step `let mut rt = Self { ... }; rt.global_object = Some(rt.alloc_object(Object::new_ordinary())); rt` pattern).
2. Drop the early-allocation block from `install_globals` (rung 7a precondition no longer needed).
3. Drop the `if let Some(gt) = self.global_object { ... } else { ... }` fallback branches from enumerate_roots / define_global_property / Op::StoreGlobal — `global_object` is now provably `Some` after `Runtime::new`.
4. Delete `Runtime.globals: HashMap<String, Value>` field.
5. Record in deletions-ledger.
6. Sweep verifies 85.9% holds. If anything regresses, an implicit constraint surfaces.

## GBSU-EXT 7f.4 — LANDED (2026-05-27) — **FIELD DELETED + arc closure**

Per keeper directive (Telegram 10022) "Continue".

**Substrate** (substantial):
- `Runtime::new()` converted to two-step pattern: build struct with `global_object: None`, then eager-allocate via `rt.alloc_object(Object::new_ordinary())`. global_object is now Some for the entire Runtime lifetime.
- `install_globals` (intrinsics.rs:1683+) dropped its rung-7a late-allocation block — redundant.
- `install_global_this` dropped its HashMap-drain loop (entries 469-492 in pre-7f.4 layout) — define_global_property now writes the Object directly at every install site, so there's nothing to drain. The drain loop also dropped its `dict_mut().insert` for the `globalThis`/`global` self-references, replaced by `define_global_property` calls (same {w:t, e:f, c:t} descriptor).
- Op::StoreGlobal pre-install bootstrap fallback removed: writes go directly to `self.object_set(global_object_id, name, v)` via `expect("global_object eager-allocated in Runtime::new")`.
- `enumerate_roots` pre-install bootstrap fallback removed: roots `global_object.expect(...)`.
- `define_global_property` pre-install bootstrap fallback removed: writes `self.obj_mut(global_object_id).dict_mut().insert(...)` directly.
- `Runtime.globals: HashMap<String, Value>` field DELETED.
- ~15 additional cluster migrations (Promise/globalThis chains in interp.rs, intrinsics.rs Function ctor's Error.prepareStackTrace lookup, napi.rs, promise.rs, cruftless's assert.rs/util.rs/module_ns.rs/stream.rs/timer.rs) — all multi-line `rt.globals.get(...).cloned().unwrap_or(...)` patterns and one final `rt.globals.insert("X", v)` in promise.rs.

**Gates**:
- Build clean across the full workspace.
- diff-prod 42/42 PASS.
- test262-sample: **86.7%** (6311 PASS / 964 FAIL / 397 SKIP) — **+61 tests over the 85.9% post-cluster-migration baseline**.

**Why the +61-test surfacing** (Doc 729 §XIII inverted — *deletion as positive-surface probe*): the rung-3a-onwards dual-write pattern left install_global_this's drain loop running, which iterated the HashMap and re-installed every entry into the Object's dict via `dict_mut().insert`. That re-installation **overwrote** any property already on the Object (registered via the rung-7b-onwards register_global_* migrations). For names installed via *both* code paths (rare but real — particularly globalThis self-reference), the second installation could clobber subtle invariants (descriptor shape, shape-system slot wiring). With the drain loop deleted and only one installation path per name, the property table is cleaner. The 61-test surfacing is consistent with this hypothesis (subtle shape-related test deltas, not catastrophic-cluster fixes).

**Standing rec from 7f.4 (load-bearing)**: deletion can be a *positive* probe in Doc 729 §XIII. Most §XIII applications today were "regression names a constraint." This rung shows the inverse: dropping a parallel surface uncovers tests that had been silently failing because the dual surfaces' interaction was ill-defined. Future locales that maintain transitional dual-writes should plan for a yield-surfacing rung at deletion time, not just a parity verification.

**Arc status**: CLOSED. The `Runtime.globals` field is deleted; the globalThis Object is the sole canonical global Variable Environment Record. Doc 731 alphabet purity: the runtime alphabet's binding-resolution surface is `{Object, engine_helpers}` — irreducibly minimal (engine_helpers per §VII.B is structurally separate from JS-visible globals and must remain its own symbol).

**Composes-with verified through closure**:
- Doc 729 §XIII (six recurrences across this locale: rungs 3, 5, 5b, 5c, 7b round A, 7b round B; plus the inverted positive-surface case at 7f.4)
- Doc 731 alphabet purity (achieved: one symbol removed from the runtime binding alphabet)
- Doc 729 §VII.B engine_helpers bilateral (preserved invariant throughout)
- Tier-Ω.5.dddd / .qq compiler pre-allocation (untouched; IC.1 protected)
- Tier-Ω.5.P55.E1 engine_helpers fallback in Op::LoadGlobal (preserved)

**Cumulative arc LoC**: ~600 net (additions ~700, deletions ~100, gross migrations across ~125 sites). Deletion ledger updated with the rung 6 entry + a new 7f.4 entry.

**Next**: returning to the parent locale (`pilots/eval-scope-binding-chain/`) to attempt ES-EXT 2 v2 (the original arc telos) now that the unified surface stands. With IC.1 preserved (pre-allocation passes untouched) and the binding surface unified, the constraint-respecting v2 design from Doc 729 §XIII methodology section can land.

## GBSU-EXT 7f.5 — Integration-test API cleanup (2026-05-29)

Per helmsman directive `runtime-globals-integration-test-cleanup-r4`.

Follow-up cleanup after GBSU-EXT 7f.4 deleted `Runtime.globals`. The runtime test surface still had direct field reads/writes against the removed HashMap API, so `rusty-js-runtime` integration tests no longer compiled even though the runtime substrate had converged on the eager global-object model.

**Substrate boundary**: test-only. No edits to `pilots/rusty-js-runtime/derived/src/` proper.

**Migration shape**:
- Post-exec inspection sites now use `Runtime::global_get("__last_recorded")`, preserving the current global-object-backed read path instead of resurrecting a shadow map.
- GC root anchoring now uses `Runtime::define_global_property`, with explicit removal through the global object's dictionary where the test needs to release roots.
- Promise-golden's optional recorder helper now treats `Value::Undefined` from `global_get` as absence, matching the old `HashMap::get(...).cloned()` option shape.

**Actual affected set**: broader than the eight named files because the compile sweep found the same removed API residue across 21 integration test files:
`binding_capture.rs`, `class_fields.rs`, `closure_upvalues.rs`, `complex_assign_target.rs`, `computed_key.rs`, `destructure.rs`, `for_in.rs`, `gc_cycle.rs`, `iteration.rs`, `labelled.rs`, `object_create.rs`, `omega_5_w.rs`, `omega_5_x.rs`, `omega_5_y.rs`, `promise_golden.rs`, `prototype_chains.rs`, `regexp.rs`, `spread.rs`, `spread_misc.rs`, `switch_stmt.rs`, `template_literal.rs`.

**Gates**:
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --no-run` PASS; integration tests compile against the current Runtime API.
- `cargo test --release -p rusty-js-runtime --lib` PASS (53 passed, 1 ignored).
- `cargo test --release -p rusty-js-runtime` reaches execution and then FAILS at `tests/destructure.rs::t11_object_rest` with `ReferenceError("Cannot access 'rest' before initialization @1:43")`. This is unrelated to the deleted `Runtime.globals` field; the globals compile barrier is removed.

**Disposition**: cleanup complete. `Runtime.globals` removal was already documented in GBSU-EXT 7f.4; this rung documents the integration-test fallout and preserves the one-object global binding surface rather than adding a compatibility accessor.
