# strict-mode-parser-tracking — Trajectory

## SMPT-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: EPSUA-EXT 3; keeper "2" (constraint #2 — strict-mode parser tracking). Pre-scoping per EPSUA C4 narrowed scope from prospective ~80 to in-scope ~12 (top-level yield-ident-valid only); strict-mode + generator-context axes deferred.

**Edits** (~20 LOC) — see seed §II.

**Verification**:
- Minimal probe: `var yield = 4; for ([x = yield] of [[]]) print(x)` → 4 ✓
- Exemplar (yield-ident-valid, 12 tests): PASS 0 → 8 (+8, 67%)
- Regression: for-of (495), for-in (79), arrow-function (264) → unchanged (0)

### Findings

**Finding SMPT.1**: simple parser-state addition (function_body_depth counter, 20 LOC) closed 8/12 of the top-level yield-as-identifier sub-cluster cleanly. The remaining 4 require deeper tracking (generator-vs-non-generator OR strict-vs-sloppy within function-bodies) — deferred to SMPT-EXT 2+ candidates.

**Finding SMPT.2 (EPSUA arc-tier)**: fourth constraint sub-locale; cumulative EPSUA = 21 actual / ~163 projected = **13% of prospective amortization** (essentially flat). Pred-epsua.4 (≥2 within projection) appears falsified across constraints #4, #5, #2. The prospective doc's projections were uniformly inflated by the matrix-aggregation pattern.

**Status**: CLOSED at SMPT-EXT 1.
