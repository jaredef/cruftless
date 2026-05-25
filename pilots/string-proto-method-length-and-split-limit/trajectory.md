# string-proto-method-length-and-split-limit — Trajectory

## SPML-EXT 1+2 — function.length + ToUint32 limit (2026-05-25)

**Trigger**: matrix 2026-05-25 rank 18 (String.prototype.split, 11).

**Edits** (~20 LOC):
- `regexp.rs`: switch `register_method` → `crate::intrinsics::register_intrinsic_method` with length=2 for `replace`, `replaceAll`, `split`.
- `regexp.rs::split`: replace `Option<usize>` limit with spec `ToUint32(limit)` — `None`/`Undefined` → `u32::MAX`; non-finite → 0; otherwise `trunc().rem_euclid(2^32) as u32`. Early-return empty array on limit==0.

**Verification**:
- Probe: `String.prototype.split.length` = 2 ✓, `replace.length` = 2 ✓, `replaceAll.length` = 2 ✓
- Exemplar split (11 no-feature-tag): PASS 0 → **7**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding SPML.1 (override-order on prototype methods)**: when two install passes both register methods on the same prototype, the later pass wins (silent override). Prototype.rs registered split/replace/replaceAll with length=2; regexp.rs overrode with length=0 via its local `register_method`. The override-pattern is invisible without grep-by-name across modules — Standing Rule 20 (cross-module discipline-drift) instantiation.

**Finding SPML.2 (ToUint32 vs Option<usize>)**: pre-fix used `Option<usize>` for limit (NaN → None → no-limit); spec uses `ToUint32` (NaN → 0 → empty array). The Option-based encoding silently mistreats NaN as "absent" rather than as "zero" — a type-level encoding error, not a per-call bug. Standing recommendation: built-in numeric coercion sites should use the spec's named abstract op (ToUint32, ToLength, ToIntegerOrInfinity) by name, not ad-hoc `Option<numeric>` wrappers.

**Status**: SPML-EXT 1+2 CLOSED.
