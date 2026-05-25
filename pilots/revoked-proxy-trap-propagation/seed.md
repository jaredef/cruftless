# revoked-proxy-trap-propagation — Seed

**Locale tag**: `L.revoked-proxy-trap-propagation` (top-level)

**Status**: **CLOSED at RPTP-EXT 1**.

**Workstream**: ECMA-262 §10.5.{5,6,...} — operations on a Proxy whose [[ProxyHandler]] is null (revoked) must throw TypeError. Cruft's helpers `proxy_is_revoked` and `proxy_target_handler_checked` exist, but several call paths bypass the check by going through the non-dispatching `object_get` instead of trap-dispatching get.

**Trigger**: identified in EPSUA-EXT 4 chapter close as constraint-#1 sub-cluster (~10 tests); spawned as next-sub-locale per keeper directive.

**Pre-scoping probe**: 11 revoked-proxy fixtures in current sample.

**Composes with**:
- ECMA-262 §10.5.{5,6} [[Get]] [[GetOwnProperty]] etc.
- [EPSUA-EXT 4 chapter close](../ecmascript-parity-shared-upstream-arc/trajectory.md) — surfaced as sub-cluster of constraint #1

## I. Telos

Add revoked-proxy early-throw checks at the high-traffic Array.prototype paths that don't dispatch through proxy-trap-aware get/set. Bulk-cascade via `array_species_create`, which sits at the first contact-with-receiver for concat/filter/map/slice/splice.

## II. Apparatus + Methodology

R = single substrate site at `array_species_create` prologue.

Edits (~10 LOC):
- `interp.rs::array_species_create`: prologue revoked-proxy check that throws TypeError before any `Get(O, 'constructor')` work.

## III. Carve-outs

- Revoked proxy as ARG (not receiver) to concat: IsArray/IsConcatSpreadable path needs separate trap-aware dispatch. Deferred.
- Revoked proxy inside JSON.parse/stringify: separate code path. Deferred.

## IV. Verification

Minimal repro: `var r = Proxy.revocable([], {}); r.revoke(); Array.prototype.concat.call(r.proxy)` → TypeError ✓.

Exemplar (revoked-proxy fixtures, 11):
- PASS: 0 → 5 (+5; the 5 species-routed methods: concat/filter/map/slice/splice)
- Still-failing 6: 2 is-concat-spreadable proxy-as-arg (different path), 4 JSON (different path) — separate sub-locale candidates.

Regression check across concat/filter/map/slice/splice (~570 previously-passing): 0 regressed.

## V. Status

CLOSED at RPTP-EXT 1.
