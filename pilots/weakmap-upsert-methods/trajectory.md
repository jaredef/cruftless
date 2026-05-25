# weakmap-upsert-methods — Trajectory

## WMUM-EXT 1 — install getOrInsert/getOrInsertComputed on WeakMap.prototype (2026-05-25)

**Trigger**: top-failure-reason audit (32 test262 fails with "callee is not callable" under WeakMap getOrInsert path).

**Edits** (~2 LOC at `intrinsics.rs::install_map_and_weakmap`):
- Remove `if !is_weak_proto` gate on the getOrInsert / getOrInsertComputed registrations. Per-proto brand_chk already in scope.

**Verification**:
- Probe: `new WeakMap().getOrInsert({}, 1)` → 1 ✓; `.getOrInsertComputed(k, ()=>99)` returns existing 1 ✓
- test262 `built-ins/WeakMap/prototype/{getOrInsert, getOrInsertComputed}/`: **34/39 pass** (was 0)
- test262 `built-ins/Map/prototype/{getOrInsert, getOrInsertComputed}/`: 29/33 (unchanged baseline; no regression)
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding WMUM.1 (TC39 proposal-tracking gap)**: the upsert proposal added methods to BOTH Map.prototype and WeakMap.prototype. cruft's MGOI-EXT 1 installed on Map only; the WeakMap omission was a single-line `if` gate at the install site. Standing recommendation: TC39 proposal-derived methods need both-proto coverage by default; gate via `if` only with explicit justification.

**Finding WMUM.2 (shared storage simplifies dual-install)**: `__map_data` storage is shared between Map and WeakMap (per the existing dual-proto pattern). `map_proto_get_or_insert_via` works on the shared storage without modification. No WeakMap-specific impl needed; the install-site gate was the entire bug.

**Status**: WMUM-EXT 1 CLOSED. Locale at 34/39 + Map baseline unchanged.
