# array-exotic-virtual-property-discipline — Seed

**Locale tag**: `L.array-exotic-virtual-property-discipline` (top-level)

**Status**: **CLOSED at AEVPD-EXT 1** (1 implementation round; multi-tier closure across three substrate truths).

**Workstream**: ECMA-262 §10.4.2 Array exotic semantics for the virtual `length` property — `[[Configurable]]:false`, `[[Enumerable]]:false`, `[[Writable]]:true`. Cruft stored these flags correctly in `getOwnPropertyDescriptor` but bypassed them in three sibling code paths (`delete`, `hasOwnProperty`, `propertyIsEnumerable`). Surfaced by T262C cluster #2 (Object.defineProperty), one fixture in particular (`15.2.3.6-4-118`) requiring `verifyProperty`'s `isConfigurable` probe to round-trip correctly.

**Composes with**:
- ECMA-262 §10.4.2 Array exotic; §20.1.3.{2,3,4} Object.prototype shims
- [Doc 740 §IV.2](../../docs/corpus-ref/740-...) substrate-introduction signature (the propertyIsEnumerable sibling fix was surfaced by the has_own_str primary fix — same shape as REOU→VHTB)

## I. Telos

Three sibling code paths must honor the Array-exotic length descriptor's flags:
- `delete arr.length` → returns false (non-configurable)
- `arr.hasOwnProperty("length")` → returns true (own virtual)
- `arr.propertyIsEnumerable("length")` → returns false (non-enumerable)

## II. Apparatus + Methodology

- `interp.rs` DeleteProp/DeleteIndex: refuse delete when `key == "length" && internal_kind == Array`.
- `value.rs` `has_own_str`: recognize Array.length virtual.
- `interp.rs` `object_proto_property_is_enumerable_via`: actually check descriptor.enumerable (was returning has-own-only).

## III. Verification (exemplar suite)

Defineroperty exemplar subset (full built-ins/Object/defineProperty/*, ~1130 tests):
- 1063 PASS → 1064 PASS (+1, the motivating fixture 15.2.3.6-4-118)
- 0 regressions on previously-passing
- +13 newly-emitting tests (all FAIL with proper errors; were silently aborting pre-fix). Visibility gain.

Minimal probes (all GREEN):
- `var a = []; delete a.length` → false
- `a.hasOwnProperty("length")` → true
- `a.propertyIsEnumerable("length")` → false
- `Object.getOwnPropertyDescriptor(a, "length")` → `{value:0, writable:true, enumerable:false, configurable:false}` (unchanged)

Full test262-sample sweep deferred per keeper directive (exemplar suite only; full sweep on authorization).

## IV. Carve-outs

- Other Array-exotic semantics (indexed property descriptors) unchanged.
- Cluster #2 (38 tests, Object.defineProperty edge cases) has heterogeneous causes; AEVPD addresses the Array-length subset only. Remaining ~54 fails need per-bug investigation in follow-on rounds.

## V. Status

CLOSED at AEVPD-EXT 1.
