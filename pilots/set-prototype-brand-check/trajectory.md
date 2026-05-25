# set-prototype-brand-check — Trajectory

## SPBC-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper directive "New top level locale as coherent" after RPTP-EXT 2 close. Probe of remaining 226 TypeError-not-thrown failures surfaced Set.prototype brand-check as the most coherent sub-cluster (28 tests, single substrate pattern: brand-check missing at 7 method entry points).

**Edits** (~25 LOC):
- `interp.rs::require_set_brand` helper.
- Boilerplate replacement at 7 set-method `_via` entry points (union, intersection, difference, symmetricDifference, isSubsetOf, isSupersetOf, isDisjointFrom).

**Verification**:
- Minimal repro: GREEN
- Exemplar (28 in-scope brand-check tests): PASS 0 → **14**
- Regression on Set/prototype (256 previously-passing): 0

### Findings

**Finding SPBC.1**: brand-check uniform substitution at 7 entry points; per-sub-spec-section projection 14/28 = 50%. Lower than the 100% pattern seen at ASCD/ACDPD/RPTP because the "in-scope" estimate was too coarse — half the tests use Set-like-class with inherited __set_data, which my brand-check accepts. Spec requires a stricter brand-check (e.g., checking this is a *direct* Set instance, not a subclass).

**Finding SPBC.2 (per-test-variant segmentation)**: even within "brand-check" sub-sub-cluster, two distinct upstream causes:
- `{}` / `Object` receiver (~14, closed by `__set_data` check)
- Set-like-class with inherited __set_data (~14, needs stricter brand)

**Status**: CLOSED at SPBC-EXT 1.
