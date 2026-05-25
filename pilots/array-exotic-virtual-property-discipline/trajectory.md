# array-exotic-virtual-property-discipline — Trajectory

## AEVPD-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper directive "Continue. Don't run the full test sweep; rather, run an exemplar suite that pertains to the substrate." Recon on T262C cluster #2 (Object.defineProperty, 38 tests). Probe of fixture `15.2.3.6-4-118` identified `verifyProperty`'s `isConfigurable` round-trip as the failure mechanism: `delete arr.length` returns true (should be false); `hasOwnProperty("length")` returns false (should be true).

**Multi-tier R**: {delete-refusal, has_own-virtual-recognition, propertyIsEnumerable-actually-checks-enumerable}. Last fix is sibling-revealed-by-primary (REOU→VHTB pattern again).

**Edits**:
1. `interp.rs` DeleteProp: refuse when `key=="length" && Array`.
2. `value.rs` has_own_str: recognize Array.length virtual.
3. `interp.rs` propertyIsEnumerable_via: actually return descriptor.enumerable (was returning has-own-only — accidentally correct for Array.length when has_own was wrong; broken once has_own was fixed).

**Exemplar verification**:
- `built-ins/Object/defineProperty/*` (~1130 tests):
  - PASS: 1063 → 1064 (+1; motivating fixture 15.2.3.6-4-118)
  - Regressions on previously-passing: 0
  - Newly-emitting tests: +13 (all FAIL with proper errors; were aborting pre-fix). Visibility gain.

Full sweep deferred per keeper directive.

### Findings

**Finding AEVPD.1**: Doc 740 multi-tier shape recurs at the smallest scope here too. R = 3 sibling code paths, all needed for the spec's verifyProperty round-trip. The propertyIsEnumerable bug was accidentally masked by the has_own_str bug — closing one exposed the other within the same round (avoided the REOU→VHTB-style cross-round substrate-introduction-signature delay).

**Finding AEVPD.2**: cluster-level heterogeneity. Cluster #2 (38 matrix tests, 55 raw fails) has many distinct causes (TypeError-not-thrown, descriptor-property-precedence, Symbol-key edge cases, writable-flag-mishandling, etc.). One round closes one cause; full cluster needs many rounds. The matrix view (top-N cells) over-aggregates when the data-axis is heterogeneous within a cell.

**Status**: CLOSED.
