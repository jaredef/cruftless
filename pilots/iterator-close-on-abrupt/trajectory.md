# iterator-close-on-abrupt — Trajectory

## ICOA-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: EPSUA-EXT 0.5 pivot. Becomes EPSUA-EXT 1.

**Edits** (~50 LOC) — see seed §II.

**Minimal repros**: GREEN.

**Exemplar verification** (previously-failing iter-close + expected-throw dstr subset, 20 tests):
- PASS: 0 → 6 (+6)
- Projected: ~25 — **under-delivered by ~76%**
- Remaining 14 are out of ICOA scope (TypeError on bad iter.next, Test262Error not propagated through inner-call paths, count-ordering mismatches).

**Regression check**:
- for-of/dstr previously-passing 387 → 387 (0)
- Random 50 across Array/Set/destructuring: 50/50 PASS (0)

### Findings

**Finding ICOA.1**: under-delivery vs projection (~+6 vs ~+25). The cluster's 25-test count was inflated by tests requiring multi-tier work beyond IteratorClose alone. Per Finding T262C.6 carry-forward (per-cluster reason-heterogeneity probe), should have segmented the 25 by reason-shape before scoping — 6 are IteratorClose-pure; 14 require additional substrate (TypeError-on-bad-iter-result, deeper iter-protocol error propagation).

**Finding ICOA.2**: Pred-epsua.3 falsified for constraint #4 at -76% of projection. Aggregate prediction (~340 across 5) now requires ~334 from the remaining 3 constraints to hold, an upward revision of per-constraint amortization.

**Finding ICOA.3 (Methodology — second corroboration of Finding T262C.6)**: the AEVPD finding that "matrix view over-aggregates when within-cluster data-axis is heterogeneous" recurs here. The 25-test iter-close cluster was scoped from the matrix's cell label `feat:Symbol.iterator;feat:destructuring-binding;err:Test262Error;expected-throw-missing` — but the cell aggregated multiple distinct sub-causes. Per-reason segmentation pre-scoping would have produced an accurate 6-test scope estimate.

**Implication for EPSUA arc**: per-reason segmentation as a hard prerequisite before each EPSUA sub-locale's scoping (strengthen C4).

**Status**: CLOSED at ICOA-EXT 1.
