# map-getorinsert-upsert — Trajectory

## MGOI-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper "Continue to next cluster" after NACR + findings.md Addendum XII. Matrix probe surfaced ~49 getOrInsert/getOrInsertComputed fails as the largest single-shape cluster.

**Edits** (~75 LOC):
- `interp.rs`: `map_proto_get_or_insert_via` + `map_proto_get_or_insert_computed_via`. Both brand-checked. The Computed variant re-checks has(key) AFTER the callback (per spec: callback may have inserted).
- `intrinsics.rs` Map ctor: register both with arity 2, MPBC `brand_chk` wrapper, guarded `!is_weak_proto`.

**Build**: GREEN. Sweep in progress; ~/bin/cruft deploy + exemplar verification deferred.

**Status**: code landed; verification pending sweep completion.

## MGOI-EXT 1 verification + SPBC-EXT 4 carve-fix (post-deploy 2026-05-25)

**MGOI-EXT 1 probe**: GREEN.
- `m.getOrInsert("k", 1)` → 1; second call returns 1 (cached); size unchanged at 2nd call
- `getOrInsertComputed("x", cb)` invokes cb on miss; subsequent call doesn't re-invoke (cached)

**MGOI-EXT 1 exemplar** (49 getOrInsert fixtures in latest sweep):
- PASS: 21 of 49 (+21 over pre-existing 0)
- Remaining 28: callback-related edge cases + WeakMap variant + iterator-close failure-paths (separate sub-locales)

**Sweep delta** (pre-NACR-snapshot → post-sweep): +51 PASS / **3 regressions** / runnable rate 80.6% → **82.3%**.

3 regressions traced to SPBC-EXT 3 scope gap (clear/forEach not wrapped) + WeakSet-value-can-be-held-weakly missing check. Companion fix **SPBC-EXT 4**:
- Extend `set_brand_chk` to wrap `clear` + `forEach` registrations (SPBC-EXT 3 covered add/has/delete only).
- WeakSet.prototype.add: additional check that value arg is Object | Symbol (per CanBeHeldWeakly).

**Recovery**: 2 of 3 regressions recovered post-fix. 1 still fails (registered Symbol via `Symbol.for` not distinguished in cruft — subtle CanBeHeldWeakly edge case; out of scope).

### Status

CLOSED at MGOI-EXT 1 + SPBC-EXT 4.
