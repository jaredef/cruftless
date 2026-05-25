# set-ops-object-key-identity — Seed

## Telos

`Set.prototype.{union, intersection, difference, symmetricDifference, isSubsetOf, isSupersetOf, isDisjointFrom}` (ES2025 Set methods) use `abstract_ops::to_string(&v)` to derive storage keys from incoming Object values. This collapses ALL Objects to `"[object Object]"`, breaking identity semantics. `Set.prototype.add/has/delete` already use `Self::map_storage_key` which preserves Object identity (via `__objkey@<oid>`).

Identified by FIPC.1's documented-deviation audit pattern, then probe: `new Set([a, b]).intersection(new Set([a]))` returns size=0 instead of size=1 (Objects collapse to same key in the op).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs::set_proto_{union,intersection,difference,symmetric_difference,is_subset_of,is_superset_of,is_disjoint_from}_via` (lines 1350-1474).
- `Self::map_storage_key(&v)` at line 2761 — the canonical key-derivation that special-cases Objects (`__objkey@<oid>`), well-known Symbols, and primitives.

## Methodology

Replace each `abstract_ops::to_string(&v).as_str().to_string()` with `Self::map_storage_key(&v)` in the 8 sites identified.

## Carve-outs

- Spec mandates SameValueZero for Set element equality (not identity, not stringification). cruft's `__objkey@<oid>` approximates identity correctly for ordinary Objects; primitives go through stringification which matches SameValueZero except for `-0` vs `+0` (both ToString to "0" — coincidentally correct).
- Non-string primitive coercion (Number, BigInt, Symbol): same as Map's approach; preserved.

## Resume protocol

Read `trajectory.md` tail.
