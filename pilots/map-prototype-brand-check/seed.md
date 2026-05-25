# map-prototype-brand-check — Seed

**Locale tag**: `L.map-prototype-brand-check` (top-level)

**Status**: **CLOSED at MPBC-EXT 1** (pending sweep-completion verification).

**Workstream**: ECMA-262 RequireInternalSlot([[MapData]] / [[WeakMapData]]) — `Map.prototype.{get,set,has,delete}` must reject WeakMap receivers and vice versa per spec. Cruft shares the impl for both protos; cross-proto calls go through the same `map_proto_*_via` which doesn't differentiate by registering proto.

**Trigger**: keeper directive "Continue and also start sweep" after SPBC-EXT 2 close. Mirror of SPBC but for Map/WeakMap.

**Pre-scoping probe**: 6 Map.prototype brand-check tests (1 set-receiver, 1 weakmap-context, 4 weakmap-receiver across get/set/has/delete).

**Composes with**:
- ECMA-262 §24.1.3 Map.prototype; §24.3.3 WeakMap.prototype
- §10.1.X RequireInternalSlot
- [SPBC](../set-prototype-brand-check/) — sibling pattern for Set/WeakSet

## I. Telos

Wrap registered Map.prototype + WeakMap.prototype methods with proto-aware brand-checks captured in the registration closure. Cross-proto calls throw.

## II. Apparatus + Methodology

R = single-tier closure-wrapper at registration.

Edits (~20 LOC):
- `intrinsics.rs` Map/WeakMap ctor block: introduce `brand_chk` closure that uses captured `is_weak_proto` to assert receiver-vs-proto compatibility; wrap get/set/has/delete registrations with it.

## III. Carve-outs

- clear/forEach/values/keys/entries already gated by map_only set in map_this_and_storage (existing).
- WeakSet brand-check handled by SPBC.
- Pure Set-receiver to Map methods already rejected by __map_data storage check (existing).

## IV. Verification

Minimal probe: `Map.prototype.set.call(new WeakMap(), {}, 1)` → TypeError ✓.

Exemplar: pending sweep-completion (full sweep blocks ~/bin/cruft binary update).

## V. Status

CLOSED at MPBC-EXT 1; sweep-completion verification pending.
