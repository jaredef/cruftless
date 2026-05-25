# via-method-audit — Trajectory

## VMA-EXT 1 — hasOwnProperty + propertyIsEnumerable ToPropertyKey (2026-05-25)

**Trigger**: LOAL.3 audit recommendation. Audit pass over 228 `_via` methods in `interp.rs` ran two grep patterns:

1. **cb-before-len (LOAL.3 shape)**: `is_callable` line preceding `try_array_length` / `length_of_array_like` line within a single `_via`. **3 hits, all spec-correct** on inspection (alternative branches in json_stringify_via; array_from_via + array_proto_sort_via correctly check callable first per spec).

2. **static-coerce on user-arg (RPTC.7 shape)**: `abstract_ops::to_string(&args.first()...)` at user-argument coercion boundaries. **2 hits in `_via` methods**: `object_proto_has_own_property_via` + `object_proto_property_is_enumerable_via`.

Both use `has_own_str` against a key from static `abstract_ops::to_string`, which (a) collapses Object args to `"[object Object]"` and (b) stringifies Symbol args to their `@@sym:N` internal form. Spec §20.1.3.2 + §20.1.3.4 mandate ToPropertyKey (§7.1.19) which is ToPrimitive + Symbol pass-through.

**Edits** (~30 LOC at `interp.rs`):

- New `Runtime::property_key_of(v)` helper — same shape as the top-level `property_key()` but as an associated fn (callable from within Runtime methods). Maps Value to PropertyKey per spec: Symbol → Symbol bucket; Number → number-to-string in String bucket; String → as-is; else → static to_string fallback (already-coerced primitives only).
- `object_proto_has_own_property_via`: coerce non-Symbol args via `coerce_to_string` (dispatching @@toPrimitive); Symbol args pass through. Look up via PropertyKey-aware `contains_key` for Symbol keys, `has_own_str` for String keys (preserves shape-awareness per CMig-EXT 8).
- `object_proto_property_is_enumerable_via`: same coercion path; key extracted as String (Symbol Rc-as-str fallback) for the existing get_own lookup.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `o.hasOwnProperty({toString: () => "foo"})` (o has foo) | `false` | `true` |
| `o.propertyIsEnumerable({toString: () => "foo"})` | `false` | `true` |
| `o.hasOwnProperty(Symbol.for("k"))` (o[Symbol] set) | `false` | `true` |
| `o.hasOwnProperty("foo")` (basic case) | `true` | `true` |
| test262 `Object/prototype/{hasOwnProperty,propertyIsEnumerable}/` (79 tests) | n/a | **67/79 pass** |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding VMA.1 (LOAL.3 audit's cb-before-len pattern was already largely-correct)**: of 3 cb-before-len hits, all 3 were spec-correct (alternative branches or methods where spec really does mandate callable-check first like Array.from / sort). The LOAL-EXT 2 batch already caught the 5 actual bugs (reduce/reduceRight/flatMap/findLast/findLastIndex). The cb-before-len audit pattern is now exhausted on `_via` methods.

**Finding VMA.2 (RPTC.7 still finds new sites)**: the parallel static-coerce-on-user-arg audit found 2 more sites despite multiple prior sweeps. Standing recommendation: the RPTC.7 grep should be re-run as part of every audit pass; it surfaces new sites as new `_via` methods land.

**Finding VMA.3 (ToPropertyKey is its own bug class)**: hasOwnProperty / propertyIsEnumerable / __lookupGetter__ / __lookupSetter__ / __defineGetter__ / __defineSetter__ all take a property-key argument that requires ToPropertyKey (§7.1.19) coercion, NOT ToString. The substrate's static `abstract_ops::to_string` for these is wrong on two axes:
- Object args don't dispatch (RPTC.7's classic gap).
- Symbol args get stringified to internal `@@sym:N` form instead of routing to the Symbol bucket.

VMA-EXT 1 fixed the first two methods; the remaining four (`__lookupGetter__`/`__lookupSetter__`/`__defineGetter__`/`__defineSetter__`) carry the same bug and are sibling-locale candidates.

**Status**: VMA-EXT 1 CLOSED. cb-before-len audit exhausted; RPTC.7 audit closed 2 more sites with a 67/79 cluster pass; 4 sibling ToPropertyKey sites queued for a follow-on VMA-EXT 2.
