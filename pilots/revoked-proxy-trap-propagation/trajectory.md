# revoked-proxy-trap-propagation — Trajectory

## RPTP-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper directive "Next sub locale" after ACDPD-EXT 1 closed. Smallest clean sub-cluster of former constraint #1.

**Pre-scoping**: 11 revoked-proxy fixtures in current sample. Sub-cluster split: 5 Array.prototype.{concat,filter,map,slice,splice} as receiver (RPTP-EXT 1 scope); 2 is-concat-spreadable proxy-as-arg (deferred); 4 JSON.* (deferred).

**Edit** (~10 LOC):
- `interp.rs::array_species_create` prologue: if receiver is a revoked Proxy, throw TypeError before `Get(O, 'constructor')`.

**Verification**:
- Minimal repro: `Array.prototype.concat.call(revokedProxy)` → TypeError ✓
- Exemplar (11 revoked-proxy fixtures): PASS 0 → 5
- Regression across concat/filter/map/slice/splice (~570 previously-passing): 0

### Findings

**Finding RPTP.1**: per-sub-spec-section projection (in-scope sub-cluster = 5 of 11; predicted +5; actual +5 = 100%). Third consecutive 100% per-sub-spec-section ratio (ACDPD.2 + RPTP.1; ASCD also 100% for its IsConstructor sub-sub-cluster of 3). The methodology refinement (per-reason-pattern → per-sub-cluster → per-sub-spec-section) converges cleanly.

**Status**: CLOSED at RPTP-EXT 1.
