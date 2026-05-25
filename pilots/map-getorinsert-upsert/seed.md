# map-getorinsert-upsert — Seed

**Locale tag**: `L.map-getorinsert-upsert` (top-level)

**Status**: **CLOSED at MGOI-EXT 1** (pending sweep-completion deploy + exemplar verification).

**Workstream**: TC39 upsert proposal (https://github.com/tc39/proposal-upsert, Stage 3 / Stage 4 candidate at time of seed). Adds `Map.prototype.getOrInsert(key, value)` and `Map.prototype.getOrInsertComputed(key, callbackfn)`. Cruft did not have these; 49 test262 fixtures fail (16-test sub-cluster directly testing the methods + 33-test getOrInsert-using callers / not-a-constructor probes).

**Trigger**: post-NACR matrix re-probe surfaced ~49 getOrInsert/getOrInsertComputed failures as the largest remaining single-shape cluster. Mostly Stage 3-4 proposal methods that don't exist in cruft.

**Composes with**:
- TC39 upsert proposal (Stage 3 at time of seed)
- Cruft's existing Map.prototype.{get, set, has, delete} + Map storage (`__map_data` sentinel, map_storage_key encoding)
- MPBC's per-proto brand-check wrappers (getOrInsert/getOrInsertComputed registered with the same `brand_chk` discipline)

## I. Telos

Implement Map.prototype.getOrInsert + getOrInsertComputed per spec; brand-check via the MPBC wrapper pattern; register on Map proto only (WeakMap variant deferred — keys are weak-held, different storage shape).

## II. Apparatus

Edits (~75 LOC):
1. `interp.rs::Runtime::map_proto_get_or_insert_via`: brand-checked; if has(key) return get(key); else set(key, value); track __map_orig_keys + size; return value.
2. `interp.rs::Runtime::map_proto_get_or_insert_computed_via`: brand-checked; if has(key) return get(key); else call callback with key arg; re-check has(key) post-call (callback may have inserted); set(key, result); track keys + size; return result.
3. `intrinsics.rs` Map ctor block: register both methods with arity 2 + brand_chk closure (guarded by `!is_weak_proto`).

## III. Carve-outs

- WeakMap.prototype.getOrInsert/getOrInsertComputed: deferred. WeakMap keys are weak-held; storage shape differs.
- Map.prototype.upsert (Stage 1 alternative naming): not adopted by spec; not implemented.

## IV. Verification

Expected exemplar gains: ~16 directly-test-the-method fixtures, plus indirect cascade from tests that USE the method as a probe primitive (~30 additional).

Probe: `new Map().getOrInsert("k", 1) === 1; m.getOrInsert("k", 2) === 1` (cached value returned).

## V. Status

CLOSED at MGOI-EXT 1; sweep-completion verification pending.
