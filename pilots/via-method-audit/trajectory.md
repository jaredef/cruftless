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

---

## VMA-EXT 2 — Annex B `__defineGetter__`/`__defineSetter__`/`__lookupGetter__`/`__lookupSetter__` (2026-05-25)

**Trigger**: VMA.3's queued sibling-locale closure. The four Annex-B-Object.prototype methods carry the same ToPropertyKey bug pattern hasOwnProperty had, plus two additional spec divergences surfaced during the fix:

1. **Wrong callable check**: pre-fix used `matches!(fn_v, Value::Object(_))` which accepts any Object instead of spec's IsCallable (§B.2.2.{2,3} step 1). Misses the spec intent though doesn't break in practice since BoundFunction/Closure are Object-tagged.
2. **Lookup was own-only**: `__lookupGetter__` / `__lookupSetter__` used `get_own(key)` but spec §B.2.2.{4,5} mandates a proto-chain walk for the accessor.

**Edits** (~50 LOC across 4 methods):

- All 4 methods: key coercion via `coerce_to_string` for non-Symbol args; Symbol args pass through. Use `property_key_of` to extract the spec PropertyKey.
- `__defineGetter__` / `__defineSetter__`: switch callable check from `matches!(_, Object)` to `self.is_callable(fn_v)`. Store under the proper PropertyKey (Symbol bucket for Symbols).
- `__lookupGetter__` / `__lookupSetter__`: walk proto chain (per spec); return undefined for accessor-with-undefined-getter/setter (per GOPD-EXT 1's accessor-with-undefined-field semantics).

**Verification** (focused probes):

| Probe | Result |
|---|---|
| Basic `o.__defineGetter__("a", fn); o.a` | 42 ✓ |
| Object-key: `o.__defineGetter__({toString:()=>"b"}, fn); o.b` | 99 ✓ |
| Symbol-key: `o.__defineGetter__(Symbol.for("k"), fn); o[s]` | "sym-val" ✓ |
| BoundFunction via .bind(): `o.__defineGetter__("c", fn.bind(null)); o.c` | 1 ✓ |
| Inherited lookup: `Object.create(p).__lookupGetter__("x")` where p has accessor | function ✓ (was undefined) |

**test262 yield**: 0. The local test262 corpus has no `__defineGetter__` / `__lookupGetter__` tests (Annex-B browser-compat methods are not deeply probed by the test262 cuts cruft tracks). The fix benefits real-world consumer code that uses these legacy idioms.

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding VMA.4 (test262 cluster size does not always predict bug-pattern reach)**: VMA-EXT 1 had a 67/79 test262 yield; VMA-EXT 2 had 0. Both fixes addressed the same bug class (ToPropertyKey) at the same surface (Object.prototype methods). The test262 cut focuses on spec-mandatory surface and under-samples Annex-B optional features. Standing recommendation: bug-pattern fixes whose architectural shape is consistent across siblings should land even when the test262 yield for some siblings is zero; the test262 cluster size is a sampling artifact, not a substrate-importance signal.

**Finding VMA.5 (own vs proto for lookup methods is a separable spec divergence)**: `__lookupGetter__` / `__lookupSetter__`'s spec mandates proto-chain walk (Annex B step "Walk prototype chain"), while `__defineGetter__` operates only on the instance. The pre-fix path had `get_own` for both — wrong for lookup, right for define. Standing recommendation: when porting Annex-B methods, check spec for "own vs proto" semantics per method; the symmetry of the name pair (define/lookup) doesn't imply symmetry of the lookup site.

**Status**: VMA-EXT 2 CLOSED. ToPropertyKey bug class on Object.prototype Annex-B + ES2015 surface closed (6 methods total across VMA-EXT 1+2).
