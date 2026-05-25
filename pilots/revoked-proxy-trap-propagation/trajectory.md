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

## RPTP-EXT 2 — JSON.stringify traversal (2026-05-25)

**Trigger**: keeper "Spawn next" after RPTP-EXT 1; same workstream (revoked-proxy trap propagation) at a different traversal site.

**Pre-scoping**: 4 JSON revoked-proxy fixtures. In-scope: 2 (JSON.stringify value-array + value-object). Out-of-scope: revived-proxy-revoked (revocation DURING JSON.parse via reviver — different control flow) + replacer-array-proxy-revoked-realm (cross-realm; $262 dependency).

**Edit** (~6 LOC):
- `interp.rs::json_serialize_compound_via` prologue: if value is a revoked Proxy, throw TypeError per §25.5.2.5 step 1 + §10.5.{12,13}.

**Verification**:
- Targeted: PASS 0 → 2 (`value-array-proxy-revoked`, `value-object-proxy-revoked`)
- Regression on JSON.parse (57) + JSON.stringify (32): 0 regressed.

### Findings

**Finding RPTP.2**: per-sub-spec-section ratio held at 100% (in-scope 2/2). Fourth consecutive 100% per-sub-spec-section round (ASCD IsConstructor 3/3, ACDPD target-array 10/10, RPTP-EXT 1 5/5, RPTP-EXT 2 2/2). The methodology converges: when scoping reaches per-sub-spec-section granularity, the projection-actual ratio reaches 100% cleanly.

**Status**: CLOSED at RPTP-EXT 2.
