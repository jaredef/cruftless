# Shape-EXT 1 — Object Layout Survey

*Apparatus-tier round. Surveys the current `pilots/rusty-js-runtime/derived/src/value.rs` + `interp.rs` Object representation and its consumer surface. Output anchors Shape-EXT 2's data-structure design against a measured contract surface rather than a guessed one. No code change.*

**Survey snapshot**: 2026-05-22, commit `8b59ebe2` (post-Shape-EXT 0 founding).

## 1. Object anatomy

`Object` at `value.rs:217`:

```
pub struct Object {
    pub proto: Option<ObjectRef>,
    pub extensible: bool,
    pub properties: IndexMap<PropertyKey, PropertyDescriptor>,
    pub internal_kind: InternalKind,
}
```

Four fields. The `properties` IndexMap is the storage tier the Shape substrate replaces (for Ordinary objects); the other three fields stay untouched by Shape-EXT 4-7.

`PropertyKey` at `value.rs:169` is a two-variant enum: `String(String)` and `Symbol(Rc<String>)`. Symbol identity is `Rc::ptr_eq`; String equality is content-based. Symbol-keyed properties bypass the Shape substrate in v1 per seed §IV carve-out.

`PropertyDescriptor` at `value.rs:365` is a six-field record: `value`, `writable`, `enumerable`, `configurable`, `getter: Option<Value>`, `setter: Option<Value>`. Properties with non-None getter or setter are accessor descriptors and bypass the Shape substrate in v1 (only data properties with user-default `{w:t, e:t, c:t}` are eligible per seed §IV carve-out).

`InternalKind` at `value.rs:380` discriminates Object exotic-ness. Ten+ variants observed: `Ordinary`, `Array`, `Function`, `Closure`, `BoundFunction`, `Error`, `ModuleNamespace`, `Promise(PromiseState)`, `RegExp(RegExpInternals)`, `Proxy(ProxyInternals)`, plus EXT-83 primitive-wrappers. **Only `Ordinary` admits shapes in the first cut.** Other kinds retain the IndexMap path verbatim.

## 2. Construction sites

The two `Object::new_*` factories:

- `Object::new_ordinary()` at `value.rs:229` — `internal_kind = Ordinary`, empty properties, proto = None.
- `Object::new_array()` at `value.rs:238` — `internal_kind = Array`, empty properties, proto = None.

Heap allocation through `Runtime` at `interp.rs:6004`:

- `alloc_object(obj) -> ObjectRef` — if `obj.proto.is_none()`, auto-wires the intrinsic prototype matching `internal_kind` (`object_prototype`, `array_prototype`, `promise_prototype`, etc.). This is the load-bearing proto-wiring path; the recent diff-prod Rung-19 generator-proto fix (`iter.proto = self.generator_prototype;`) deliberately set proto BEFORE alloc to make the auto-wire path inert. **Shape-EXT 4 must preserve this auto-wiring discipline.**
- `alloc_object_with_explicit_null_proto(obj) -> ObjectRef` at `interp.rs:6026` — bypasses the auto-wire so `Object.create(null)` stays null-proto.

Total `alloc_object` call sites across the runtime crate: **~338** (intrinsics.rs ~183, interp.rs ~87, module.rs ~14, regexp.rs ~13, napi.rs ~13, generated.rs ~6, promise.rs ~5, prototype.rs ~21, iterator.rs ~6).

Of those, the Ordinary-only fraction (the Shape-eligible subset) is approximately the count of `alloc_object(Object::new_ordinary())` call sites. A direct grep yields ~120 such sites; the rest construct exotic kinds.

## 3. Read APIs

**Object-direct (own-only):**
- `get_own(key: &str) -> Option<&PropertyDescriptor>` — string-keyed own lookup.
- `get_own_symbol(sym) -> Option<&PropertyDescriptor>` — Symbol-keyed.
- `get_own_mut(key) -> Option<&mut PropertyDescriptor>` — mutable own.
- `has_own_str(key) -> bool` — membership.
- `string_keys() -> impl Iterator<&str>` — string-only enumeration (skips Symbols).
- `string_key_clones() -> impl Iterator<String>` — owned variant.

**Runtime-mediated (proto-chain walking):**
- `object_get(id, key: &str) -> Value` at `interp.rs:6214` — proto-chain walk with two special cases: (a) **Array.length synthetic** — if `key == "length"` and kind is Array and no own "length" property, computes max-numeric-index + 1; (b) **Well-known-Symbol @@-prefix fallback** — if key starts with `@@`, also scans Symbol-bucket entries by string identifier on miss. Returns Undefined on miss.
- `object_get_pk(id, key: &PropertyKey) -> Value` at `interp.rs:6168` — PropertyKey-aware; routes String to `object_get`, walks proto chain by-PropertyKey for Symbol.
- `read_property(id, key) -> Result<Value, RuntimeError>` at `interp.rs:6126` — getter-dispatching variant; invokes accessor getter if present.
- `has_property(id, key: &str) -> bool` at `interp.rs:6137` — proto-chain membership.
- `has_property_pk(id, key) -> bool` at `interp.rs:6153` — PropertyKey-aware membership.
- `find_getter(id, key) / find_setter(id, key)` — accessor-walk along proto chain.

Total `object_get` call sites across the runtime crate: **~360** (intrinsics.rs ~193, interp.rs ~144, module.rs ~12, napi.rs ~14, prototype.rs ~5, iterator.rs ~3, regexp.rs ~2).

## 4. Write APIs

**Object-direct:**
- `set_own(key: String, value: Value)` at `value.rs:308` — user-default descriptor `{w:t, e:t, c:t}`. **Preserves existing attrs on key re-insertion** (only updates value), per §10.1.9 OrdinarySet. Load-bearing for Array.length, function .name/.length, deliberate-descriptor fields.
- `set_own_internal(key, value)` at `value.rs:337` — `{w:t, e:f, c:t}`. Engine-internal sentinels (`__map_data`, `__primitive__`, etc.). Bypasses Shape substrate.
- `set_own_frozen(key, value)` at `value.rs:352` — `{w:f, e:f, c:f}`. Built-in `.prototype` slots, namespace constants. Bypasses Shape substrate.
- `insert_str(key, desc)` — full-descriptor insert. Used by accessor-property install sites. Bypasses Shape substrate (non-data).
- `remove_str(key) -> Option<PropertyDescriptor>` — delete. **Trigger for Shaped→Dictionary migration** per seed §III Shape-EXT 6.

**Runtime-mediated:**
- `object_set(id, key, value)` at `interp.rs:6294` — routes through `object_set_pk` (rung-18 unification per `cruftless` seed §A8.28).
- `object_set_pk(id, key: PropertyKey, value)` at `interp.rs:6191` — the actual OrdinarySet implementation; preserves existing descriptor attrs.

Total `object_set` call sites across the runtime crate: **~417** (intrinsics.rs ~160, interp.rs ~172, module.rs ~19, regexp.rs ~27, napi.rs ~19, iterator.rs ~8, generated.rs ~0 because IR-lowered ops route through `_via` helpers).

## 5. Eligibility filter for the Shape substrate

A property addition is **Shape-eligible** iff ALL hold:
1. `Object.internal_kind == InternalKind::Ordinary`.
2. The key is `PropertyKey::String(_)` (not Symbol).
3. The install path is `set_own` or `object_set` / `object_set_pk` (user-default descriptor `{w:t, e:t, c:t}`), NOT `set_own_internal`, `set_own_frozen`, or `insert_str` with accessor / non-default flags.

A property read is **Shape-eligible** iff the receiving object's storage form is currently Shaped (which requires every prior property addition on it to have been Shape-eligible).

An object's storage form transitions Shaped → Dictionary on ANY of:
- `remove_str(key)` against any property (deletion is shape-tree-incompatible without complex re-keying that v1 defers).
- `set_own_internal` / `set_own_frozen` / `insert_str` (non-data or non-default descriptor).
- A property addition at a Symbol key.
- A property addition past the shape-tree complexity ceiling (per V8's "dictionary mode" pattern; specific threshold TBD at Shape-EXT 2).

The transition is one-way in v1. Back-promotion (Dictionary → Shaped after a fresh enumeration history) is deferred to a closure round.

## 6. Spec invariants the Shape substrate must preserve

Surveyed against `interp.rs` and the existing `cruftless` seed:

- **Insertion order at enumeration sites.** §10.1.11 OrdinaryOwnPropertyKeys requires integer-indexed keys first (ascending), then string keys in insertion order, then Symbol keys in insertion order. The shape's name→slot map IS the insertion-order record (slot index 0 = first added, etc.). Integer-indexed keys at Ordinary objects are rare in practice but must compose correctly when present.
- **Existing-key set preserves descriptor attrs.** `set_own` updates `value` only when the key exists; Shape-EXT 5 must match this on the shaped path (in-place value mutation at the slot; no shape transition required for re-set).
- **Array.length synthetic computation.** Arrays bypass the Shape substrate (per §4 carve-out), so Array.length stays in its current code path.
- **Well-known-Symbol @@-prefix fallback.** `object_get` falls back from String bucket to Symbol bucket on `@@`-prefixed keys; Shape-eligible objects only carry String-keyed properties so this fallback fires only on the Dictionary path or on proto-chain ancestors (which may be Shaped or Dictionary).
- **Proto-chain walking.** The walk is independent of storage form; each `obj.proto` step reads `Object.proto` regardless of whether the current object is Shaped or Dictionary.
- **Accessor descriptor dispatch.** Accessor (getter/setter) descriptors are non-Shape-eligible per §5; they live in Dictionary storage and dispatch through `find_getter` / `find_setter` unchanged.

## 7. Risk areas (sites that bypass the safe API)

Direct `obj.properties.insert(...)` calls outside the `set_own*` family are the riskiest sites for Shape-EXT 4's introduction. Grep `obj_mut(.*)\.properties\.insert` across the crate yields ~30 sites, concentrated in:

- `intrinsics.rs` accessor-descriptor installs (use `PropertyDescriptor { getter: Some(...), ... }` directly).
- `interp.rs` Op dispatch sites that need to set non-default descriptors atomically (e.g., the §A8.28 P62.E3 frozen-Math-constants install).
- `prototype.rs` install_*_proto helpers that wire intrinsics with specific descriptors.

These sites are ALL non-Shape-eligible (they install accessor or non-default descriptors). Shape-EXT 4's introduction does not need to touch them; they continue to use the Dictionary path. **No grep-and-fix migration required.**

The set_own/object_set sites (the Shape-eligible majority) DO need the path branch added — but the branch can live inside `set_own` and `object_set_pk` themselves rather than at every call site, so the caller surface is unchanged.

## 8. Pre-design constraints for Shape-EXT 2

The design round (Shape-EXT 2) must answer:

1. **`Shape` struct shape.** Fields: name→slot map (smallvec for ≤8 properties? hashmap above), transition table (also smallvec? FxHashMap?), parent shape pointer for traversal, optional descriptor-class tag per slot for the migration trigger.
2. **`ShapedObject` storage layout.** `(Rc<Shape>, Vec<Value>)` is the obvious form. Concern: Rc-clone cost on every shape transition; concern: Vec realloc cost on every property addition past capacity. Mitigations: shape sharing means Rc clones are cheap; Vec capacity grown by 2× per realloc is amortized O(1).
3. **`ShapeRegistry` lifetime + sharing.** Root shape is a singleton per Runtime. Transitions are stored on parent shapes (each parent owns a map from `(name, descriptor-class)` to child shape). Concern: hash collision on the transition map; mitigation: keying on `(String, descriptor-class)` works; for hot paths a smallvec linear scan beats hashmap probe at small N.
4. **`ObjectStorage` enum form.** `enum ObjectStorage { Shaped(Rc<Shape>, Vec<Value>), Dictionary(IndexMap<PropertyKey, PropertyDescriptor>) }`. Object's `properties` field becomes `storage: ObjectStorage`. Concern: enum tag overhead per Object; mitigation: the tag is one byte, the Object struct is already non-trivially sized.
5. **The IC consumer API.** `Object::shape_ptr_and_slot_for(name: &str) -> Option<(*const Shape, u32)>` returns the cache-key tuple iff the object is Shaped and the name resolves to a slot. The `*const Shape` is stable for the IC stub's lifetime because shapes are Rc-shared and never mutated (only transitioned to new shapes).
6. **Transition-table key.** Two candidates: (a) `(String, ()) — name only`, ignoring descriptor-class (correct iff all Shape-eligible adds carry the same `{w:t, e:t, c:t}` shape, which seed §IV carve-out ensures); (b) `(String, DescriptorClass)` for future flexibility. Recommendation: (a) for the first cut; the carve-out makes the descriptor class invariant by construction.

## 9. Forward to Shape-EXT 2

Shape-EXT 2's deliverable is `pilots/rusty-js-shapes/docs/shape-design.md`. The pre-design constraints above feed directly into it. Shape-EXT 3 then scaffolds the crate from the design; Shape-EXT 4 introduces `ObjectStorage` and gates on the diff-prod 42/42 + test262-sample 77.6% baselines.

The contract surface is now measured rather than guessed:
- ~338 alloc_object sites (~120 Ordinary, the Shape-eligible subset)
- ~360 object_get sites
- ~417 object_set sites
- ~30 direct properties.insert sites (all non-Shape-eligible by descriptor shape)

The migration's blast radius is bounded by the `set_own` / `object_set_pk` family — touching those two functions covers the Shape-eligible write path; touching `object_get` covers the read path. No call-site migration required.

---

*Shape-EXT 1 closes. Output: this file. No code change. Next round: Shape-EXT 2 (Shape data-structure design).*
