# promise-executor-functions-meta — Trajectory

## PEFM-EXT 1 — name="" + length=1 for resolve/reject (2026-05-25)

**Trigger**: matrix 2026-05-25 rank 17 (Promise no-feature-tag, 12). Cluster bucketed into 5 tests checking the Promise constructor's resolve/reject argument shapes (name, length, descriptors) and 7 testing deeper built-in infrastructure.

**Edits** (~6 LOC):
- `promise.rs::Promise` constructor: swap `make_native("<promise-resolve>", ...)` → `make_native_with_length("", 1, ...)`. Same for reject.

**Verification**:
- Probe: `new Promise((r,j) => ...)` returns `r.name = "" r.length = 1 j.name = "" j.length = 1` ✓
- Exemplar (12 Promise no-feature-tag): PASS 0 → **5**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding PEFM.1 (spec-detail surface area)**: built-in function meta-properties (`name`, `length`) are observable-by-default through the language surface (`fn.name`, `fn.length`) and through test262's `verifyProperty` helper. Engagement-debug names ("<promise-resolve>") that escape into observable surfaces produce a recurring failure shape across any cluster that probes built-in function identity. Standing recommendation: spec-mandated names override engagement-debug labels; reserve angle-bracket-prefixed names for hidden engine helpers (`register_engine_helper`) that are not exposed via `globalThis`.

**Finding PEFM.2 (cluster decomposition)**: 5/12 closed via the meta-property fix; remaining 7 split into built-in-function-property-descriptor-defaults (Promise.length, Promise descriptor enumerability — needs ordinary-function-style descriptor compliance for built-ins) and subclassed-promise-GetCapabilitiesExecutor (Promise.resolve.call(NotPromise) — needs the executor-allocation path for non-Promise constructors). Each axis is a separate locale.

**Status**: PEFM-EXT 1 CLOSED.
