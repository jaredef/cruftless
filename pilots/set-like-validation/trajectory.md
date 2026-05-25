# set-like-validation — Trajectory

## SLV-EXT 1 — GetSetRecord validation at Set op entry (2026-05-25)

**Trigger**: top-failure-reason audit. Set.prototype op cluster had 14 failures of shape `array-throws.js` + `called-with-object.js` — spec mandates TypeError when argument isn't Set-like.

**Edits** (~35 LOC at `interp.rs`):
- New `Self::validate_set_like(self, other, op)` per §24.2.1.2 GetSetRecord: throws TypeError if other isn't Object, or if `size` (ToNumber → NaN), `has` (not callable), `keys` (not callable) fail.
- Inserted call at top of each of 7 Set ops (union/intersection/difference/symmetricDifference/isSubsetOf/isSupersetOf/isDisjointFrom).

**Verification**:
- Probe: `new Set([1,2,3]).union([1,2])` → TypeError ✓ (was no-throw)
- Probe: `new Set().union({})` → TypeError ✓
- test262 `Set/prototype/{op}/` (186 tests): 116 → **137 pass** (+21)
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding SLV.1 (spec validation pre-iteration is a substrate-bridge pattern)**: SLV-EXT 1 is a pure validation rung — no algorithmic change to the Set ops. The substrate had collect_iterable doing duck-typed iteration; spec mandates type-strict GetSetRecord before iteration. Same shape as the bridge audits — engine output (collect_iterable's success) was correct for what it tried; substrate just didn't filter on spec's input-type invariant. Standing recommendation: at spec-method entry, audit whether spec mandates input validation (TypeError throws) BEFORE the algorithmic body. Many "Expected TypeError" cluster failures are validation-pre-body gaps.

**Finding SLV.2 (validation-only rung leaves algorithmic residuals visible)**: 49/186 residuals remain. Spot-check: real Set union returns wrong wrapper type ("[object Object]" via Array.from); custom set-like with non-Symbol.iterator keys() fails collect_iterable. These are algorithmic gaps separable from validation. Validation-only rungs naturally surface the next layer's gaps without touching them.

**Status**: SLV-EXT 1 CLOSED.
