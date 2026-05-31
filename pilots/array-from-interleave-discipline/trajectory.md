# array-from-interleave-discipline — Trajectory

## AFID-EXT 0 — LANDED (2026-05-31) — interleave iteration + IteratorClose on mapfn-throw + iter_close_rt helper

**Trigger**: Direct chapter-close-inspect carry-forward from ICES residual probe (post-ICES-EXT 3). Probing Array.from with a mapfn that throws on the first element of an infinite iter OOMed at 889 MB under `ulimit -v 1 GiB`. Diagnosis: `array_from_via` eagerly collected the iterable before applying mapfn, so mapfn-throws were unreachable on infinite iters. Keeper APPROVED via Telegram 10684 ("Continue").

**Substrate** (~80 LOC across two files in `pilots/rusty-js-runtime/derived/src/`):

1. `intrinsics.rs`: new `iter_close_rt(rt, iter_id) -> Result<(), RuntimeError>` (~25 LOC). Mirrors the JS-callable `__destr_iter_close` helper at the Rust-caller surface. Returns silently for null/undefined .return; throws TypeError for non-callable non-null/undefined; calls .return() and checks Object result otherwise.
2. `interp.rs:6589` `array_from_via` (~55 LOC delta): branch on iterable-vs-array-like upfront. Iterable branch rewritten as a per-element loop:
   - Call iter.next(); check result is Object (else TypeError); read .done; if done, break.
   - Read .value; if mapping, call mapfn(value, k) — on Err, call `iter_close_rt(self, iter_id)` (best-effort; close-thrown errors do NOT shadow the original per §7.4.9 step 4) and propagate the original error.
   - Write the mapped (or raw) value into the out array at index k; increment k.
3. Array-like / string branches preserved unchanged at the second half of `array_from_via`.

**Phase 2 (Baseline-inspect)** per Rule 23: confirmed `collect_iterable` (intrinsics.rs:21669) was the eager-collection site. Verified collect_iterable is still used elsewhere (other spread paths); the AFID rung does not remove or modify collect_iterable, only routes Array.from off it.

**Phase 3 (Pin-Art if duplicated)** per Rule 24: not invoked at this rung. The interleave pattern recurs at other intrinsic surfaces (Array spread, Map/Set constructors, Promise.all/allSettled iteration), but they consume different surrounding logic (Map ctor pairs key/value, Promise.all queues async results). Centralizing across them would require a generic `interleave_iter<F>` helper; deferred until 2+ more sites materialize.

**Phase 4 (Revert-then-deeper-layer if negative)** per Rule 13: not invoked — closure positive on first probe.

**Phase 5 (Chapter-close-inspect)** per Rule 15:

**Finding AFID.1** — CLOSED by ICES-EXT 3.1 (commit follow-up). ICES-EXT 3 close-throw spec divergence surfaced (cross-locale): the ICES-EXT 3 synthetic catch stub for for-of body throw is:

```
catch_pos:
  StoreLocal <thrown>
  emit_iter_close_call iter_slot   ← if this throws, propagates immediately
  LoadLocal <thrown>
  Throw                            ← never reached if close threw
```

This makes close-thrown errors REPLACE the body-thrown error. Per ECMA-262 §7.4.9 step 4, when `completion.[[Type]] is throw`, IteratorClose returns the ORIGINAL completion even if GetMethod/Call of `iter.return` themselves throw. The for-of body-throw + non-callable iter.return case should propagate the body-thrown error, not the close TypeError. The probe at /tmp/probe-ices-ext-3.js cell 4 asserted the WRONG behavior and was confirmed PASS by the wrong-shape substrate; both probe and substrate need correction.

AFID-EXT 0 implements the spec-correct semantics (`let _ = iter_close_rt(...)` discards the close error) at the Array.from surface. The ICES-EXT 3 fix is the dual: wrap the bytecode-tier close call in a nested synthetic try-catch that swallows close-thrown values before re-throwing the spilled original. Surfaces as a separate rung in the ICES locale (candidate ICES-EXT 3.1 or EXT 4 per keeper directive); not bundled into AFID per Rule 17 (locale segmentation).

**Finding AFID.2** — eager-collect-then-process is a recurring anti-pattern at runtime-tier intrinsics that consume iterables. Confirmed instances: `array_from_via` (pre-rung; rewritten by AFID-EXT 0), `array_proto_concat_via` (probe-pending), `array_proto_flat_via` (probe-pending). Candidate for cross-intrinsic audit at the findings-disposition cycle.

**Yield**:

```text
AFID-EXT 0 probe (/tmp/probe-afid-0.js): 8/8 PASS
  Array.from mapFn throw closes iter (length 1, threw=true)        ✓
  Interleaved next/mapFn (order n0,m0,n1,m1,n2,m2)                 ✓
  Array.from([1,2,3]) -> [1,2,3] (regression)                      ✓
  Array.from(Set) (regression)                                     ✓
  Array.from("abc") -> ["a","b","c"] (regression)                  ✓
  mapFn args (value, index) correct                                ✓
  mapFn throw + non-callable iter.return: original Error wins      ✓
  next() non-Object -> TypeError (no OOM)                          ✓

Regression sweep (3 prior probes, all under ulimit -v 2 GiB):
  Original 7-cell IPTD probe:           7/7 preserved
  Cross-consumer 7-cell probe:          7/7 preserved
  ICES-EXT 2 6-cell probe:              6/6 preserved
  ICES-EXT 3 9-cell probe:              9/9 preserved (cell 4 still asserts WRONG-shape; Finding AFID.1)
```

**Status**: AFID-EXT 0 LANDED. Array.from no longer OOMs on infinite iters when mapfn throws. Interleaved iteration spec-correct. Close-on-mapfn-throw spec-correct (original throw preserved per §7.4.9 step 4). Finding AFID.1 (ICES-EXT 3 spec divergence on close-throw shadowing) surfaced for follow-up.

## AFID-EXT 1 — LANDED (2026-05-31) — Set + WeakSet ctor interleave + close + silent-swallow removal

**Trigger**: AFID.2 audit. Survey of `collect_iterable` call sites found 6 OOM-vulnerable surfaces under `ulimit -v 512 MiB`: Promise.all/race/any/allSettled (~200+ LOC across 4 intrinsics) + Set ctor + WeakSet ctor (shared closure, ~75 LOC). Keeper directed (Telegram 10690 "3") to land the cheap pair first, defer Promise.* family.

**Substrate** (~75 LOC at one site, `intrinsics.rs:18159` inside the shared `for collection in &["Set", "WeakSet"]` loop):

Replaced the prior 12-LOC `collect_iterable` block. New flow per ECMA-262 §24.2.1.2 / §24.4.1.2 + AddEntriesFromIterable:

1. Branch on `arg ∈ {undefined, null}`: skip iteration per spec.
2. ToObject the arg if primitive.
3. GetMethod(@@iterator) → call → check Object → store iter_id.
4. Cache GetMethod(next) on iter.
5. Per-element loop: call next; check result Object (else close + TypeError); check .done; read .value.
   - WeakSet: enforce §24.4.3.1 step 4 CanBeHeldWeakly (Object or Symbol); else close + TypeError.
   - Set: insert with abstract_ops::to_string key + dedup; size += 1 on new.
   - On any per-element abrupt completion, `iter_close_rt(rt, iter_id)` best-effort then return Err(original) (§7.4.9 step 4 preservation).

Also removes the prior `if let Ok(values) = collect_iterable(...)` silent-swallow that had hidden spec-mandated TypeErrors (ToObject failures, iterator-not-callable, etc.).

**Yield**:

```text
AFID-EXT 1 probe (/tmp/probe-afid-1.js): 6/7 PASS

  WeakSet(infinite-primitive iter): close + TypeError ✓ (was OOM)
  Set(next non-Object iter):         close + TypeError ✓ (was silent empty Set)
  Set(undefined):                    empty Set ✓ (regression preserved)
  Set([1,2,2,3]) dedup:              size=3 ✓ (regression preserved)
  Set(42):                           TypeError ✓ (was silent empty Set via swallow)
  WeakSet([o1, o2]).has(o1) && .has(o2):  FAIL — pre-existing storage bug (Finding AFID.3)
  Set([1,2,3]):                      has 1+2+3 ✓ (regression preserved)

cargo test --release -p rusty-js-runtime --lib: 74 / 0 / 1 preserved.

Regression sweep preserved: IPTD 7/7, cross-consumer 7/7,
ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8.
```

**Phase 5 (Chapter-close-inspect)** per Rule 15:

**Finding AFID.3** surfaced: WeakSet storage uses `abstract_ops::to_string(&v)` as the dedup/lookup key. Distinct objects all map to "[object Object]"; only one survives at any moment in storage. `.has(o1)` returns whether the key is present, not whether o1 specifically was inserted. The prior eager-collect path hid this — baseline OOMed before reaching .has() checks. Sibling locale candidate: `weakset-object-identity-storage-discipline/` (or merged into a broader `weak-collections-identity/`). Likely also affects WeakMap key storage. Out of AFID-EXT 1 scope.

**Phase 3 (Pin-Art if duplicated)** per Rule 24: pattern `interleave_iter + close-on-abrupt + propagate-original` now appears at two runtime intrinsic sites (Array.from + Set/WeakSet ctor). Promise.* family would be the third + fourth + fifth + sixth instance. At ~3+ instances the helper-factoring threshold is met; defer until promise-iteration landing chooses its lowering shape (the per-method PromiseCapability tracking complicates a generic helper signature).

**Status**: AFID-EXT 1 LANDED. Set + WeakSet ctors no longer OOM on infinite iters; spec-mandated TypeErrors no longer suppressed; close-on-abrupt + original-error preservation aligned with AFID-EXT 0. Promise.* family remains carry-forward to a dedicated session. Finding AFID.3 (WeakSet/WeakMap identity-storage) surfaces a separable sibling locale.

## AFID-EXT 2 — LANDED (2026-05-31) — Set + WeakSet ctor identity-stable storage key (closes Finding AFID.3)

**Trigger**: Finding AFID.3 surfaced by AFID-EXT 1's chapter-close (probe cell 6 unmasked once OOM closed). Keeper APPROVED via Telegram 10692.

**Substrate** (~3 LOC):
- Promote `Runtime::map_storage_key` from private `fn` to `pub(crate) fn` (interp.rs:5221).
- Route the AFID-EXT 1 Set/WeakSet ctor block's storage write through `Runtime::map_storage_key(&v)` instead of `abstract_ops::to_string(&v).as_str().to_string()`. Identity-stable encoding: `__objkey@{oid}` for Object, `@@sym:{s}` for Symbol, fall-through for primitives.

**Yield**:

```text
AFID-EXT 1 probe re-run: 7/7 PASS (was 6/7; cell 6 closed).
Identity probe (new Set([o1,o2,1,2,2])): size=4; has(o1)+has(o2)+has(1)+has(2) all true.
new WeakSet([o1,o2]).has(o1)+.has(o2) both true.

cargo test --release -p rusty-js-runtime --lib: 74 / 0 / 1 preserved.

Regression sweep preserved: IPTD 7/7, cross-consumer 7/7,
ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8.
```

**Finding AFID.3 CLOSED**. WeakMap ctor would have the analogous issue if it had its own ctor block (current WeakMap ctor was OK at the ctor stage per the earlier triage, so no separate AFID-EXT 2 work needed for it — Map ctor path doesn't go through the same Set/WeakSet ctor closure).

**Status**: AFID-EXT 2 LANDED. Set + WeakSet ctor now uses identity-stable storage keys matching set_proto_add_via + Set.has + Set.delete. Object members compare by reference at every dispatch surface.

## AFID-EXT 3 — LANDED (2026-05-31) — Object.groupBy + Map.groupBy interleave + close-on-cb-throw

**Trigger**: AFID.2 audit residual. Object.groupBy and Map.groupBy still used eager-collect (collect_iterable / array-length-fallback + drain_iterator) + for-loop, missing IteratorClose on cb abrupt completion. Keeper APPROVED via Telegram 10700 ("Continue").

**Substrate** (~80 LOC across two sites in `pilots/rusty-js-runtime/derived/src/`):
- `interp.rs:5966` `object_group_by_via`: rewrite to per-element next → cb → bucket-assign loop. On any per-element abrupt, iter_close_rt-best-effort + propagate original.
- `intrinsics.rs:21537` Map.groupBy intrinsic closure: same shape but constructs the output Map via the ctor + stores into __map_data.

**Yield**:

```text
AFID-EXT 3 7-cell probe: 7/7 PASS
  Object.groupBy([1..5], odd/even) regression                 ✓
  Object.groupBy cb throws -> close + propagate cb Error      ✓
  Object.groupBy next non-Object -> close + TypeError         ✓
  Object.groupBy(42) -> TypeError                             ✓
  Map.groupBy([1..5], odd/even) regression                    ✓
  Map.groupBy cb throws -> close + propagate                  ✓
  Object.groupBy(Set([1..4])) regression                      ✓

cargo test --release -p rusty-js-runtime --lib: 74 / 0 / 1 preserved.
Full regression sweep preserved (9 probes across IPTD, ICES, AFID, PIID).
```

**Phase 3 (Pin-Art if duplicated)** per Rule 24: the interleave + iter_close_rt + propagate-original pattern now appears at NINE runtime-tier intrinsic sites: Array.from, Set ctor, WeakSet ctor, Promise.race, Promise.all, Promise.allSettled, Promise.any, Object.groupBy, Map.groupBy. Generic `interleave_iter_with<F>` helper threshold long met; deferred pending body-shape variance review (the Promise.* variants have PromiseCapability tracking; AFID variants are direct mutation; gnerifying would need a closure boxing approach).

**Status**: AFID-EXT 3 LANDED. Object.groupBy + Map.groupBy now spec-correct on interleave + close-on-throw. The runtime-tier iterable-consumer audit's primary scope (collect_iterable callers needing spec-correct IteratorClose) is now closed across the surveyed intrinsics.
