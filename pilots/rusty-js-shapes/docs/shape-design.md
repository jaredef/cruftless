# Shape-EXT 2 — Shape Data-Structure Design

*Apparatus-tier round. Designs the `Shape`, `ShapeTransition`, `ShapeRegistry`, and `ObjectStorage` types against the pre-design constraints from Shape-EXT 1 §8. Output is the design Shape-EXT 3 scaffolds into Cargo crate form. No code change in this round.*

**Design snapshot**: 2026-05-22, builds on `pilots/rusty-js-shapes/docs/object-layout-survey.md`.

## 1. Design overview

Three types form the core; one enum hosts the storage cohabitation:

```
Shape           — immutable shape descriptor; identifies a property-layout class
ShapeRegistry   — Runtime-owned; holds the root shape; not a separate type in the first cut
ObjectStorage   — Object's storage field; either Shaped or Dictionary
```

`ShapeTransition` is not a separate type in the first cut — transitions are entries in `Shape::transitions`. Promoting to its own type is a closure-round refactor if it becomes useful.

The design prioritizes **simplicity over micro-optimization** at every choice point per the substrate-introduction round discipline (Doc 729 §A8.13). Closure rounds tune; this round lands the contract.

## 2. `Shape` struct

```rust
pub struct Shape {
    /// Eager denormalized name -> slot index. Updated by transition_to
    /// when a child shape is allocated. Built by copying the parent's
    /// map + inserting the newly-added (name, slot) pair.
    ///
    /// Sized for the common case: most shapes carry < 16 slots in
    /// practice. SmallVec inline cap 8 covers the modal Object; hashmap
    /// activation above that.
    slots: SmallOrLargeMap<String, u32>,

    /// Forward transitions to child shapes. Keyed on the added property's
    /// name (first-cut transition-table key per Shape-EXT 1 §8.6:
    /// descriptor-class is invariant by carve-out construction, so
    /// keying on name alone is sufficient).
    ///
    /// Sized for the common case: most shapes branch < 4 ways. SmallVec
    /// inline cap 4 covers the modal transition fan; hashmap activation
    /// above that.
    transitions: SmallOrLargeMap<String, Rc<Shape>>,

    /// Back-pointer for the §10.1.11 enumeration walk if the eager
    /// denormalized `slots` proves memory-heavy in a closure round.
    /// First cut keeps it for diagnosability; the order-preserving
    /// enumeration uses `slots` directly.
    parent: Option<Rc<Shape>>,

    /// Number of slots in this shape. Equal to `slots.len()`.
    /// Cached as u32 for the IC bench-fixture cache-key emission.
    slot_count: u32,
}
```

**Where `SmallOrLargeMap<K, V>` is** a thin newtype around an enum:

```rust
enum SmallOrLargeMap<K, V> {
    Small(SmallVec<[(K, V); INLINE_CAP]>),  // O(n) linear scan
    Large(HashMap<K, V>),                    // O(1) hashed
}
```

with `INLINE_CAP = 8` for `slots`, `INLINE_CAP = 4` for `transitions`. Migration from Small to Large happens lazily at `INLINE_CAP + 1` insert.

The choice between `BTreeMap` and `HashMap` for the Large form: **HashMap**. The use case is exact-name lookup; ordered iteration is provided by the slot-index ordering in the eager `slots` (which is just (name, slot) pairs in insertion order). The Small form's linear scan IS ordered; the Large form is unordered but lookup-cheap; enumeration goes through the Small form's vector for shapes that stay small (modal case) or through a denormalized vector kept alongside the HashMap for shapes that grew large (closure-round optimization).

**Why eager denormalization of `slots`**: each shape carries the FULL name→slot map, not just its delta from the parent. The alternative (sparse with parent walk on lookup) saves memory but turns every property read into a parent-chain walk. Substrate-introduction round chooses lookup-cheap over memory-cheap; closure rounds revisit if memory pressure shows.

**Why `Option<Rc<Shape>>` parent**: not load-bearing for v1 lookup or enumeration (both use `slots` directly). Kept for diagnosability and for the potential closure-round refactor where the eager denormalization is dropped in favor of parent walks. Costs one pointer per shape. Negligible at the per-Object granularity.

## 3. Identity invariant: shape pointer means structural class

**The load-bearing invariant**: two Objects share the same `Rc<Shape>` iff they were constructed by the same property-addition history. This is what makes `Rc::ptr_eq(shape_a, shape_b)` the IC cache-key equality test.

The invariant holds iff:
- Shape allocation goes through ONE entry point (`Shape::transition_to`, see §4).
- That entry point checks the parent's `transitions` table first and reuses the existing child shape if the transition already happened.
- No code mutates an existing Shape; transitions allocate new shapes.

A bug that allocates a fresh `Rc<Shape>` for an already-existing transition (failing to consult the parent's `transitions`) breaks Pred-shape.2 (Shape-EXT 1's §VIII falsifier: same-history Objects should share shape pointer at `Rc::ptr_eq`). This is the single highest-risk site in the substrate.

## 4. `Shape` API

The Shape type exposes a small surface:

```rust
impl Shape {
    /// Root shape. Empty slot map, empty transitions, no parent.
    /// One Rc instance per Runtime, held by ShapeRegistry / Runtime.
    pub fn root() -> Rc<Shape>;

    /// Return the shape that results from adding `name` to this shape.
    /// Reuses an existing child shape if the transition already exists;
    /// otherwise allocates a fresh child shape, registers it in this
    /// shape's transitions table, and returns it.
    ///
    /// Self is `&Rc<Shape>` (not `&mut`) because transition allocation
    /// mutates this shape's transitions table only — which requires
    /// interior mutability. See §6 for the interior-mutability choice.
    pub fn transition_to(self: &Rc<Shape>, name: &str) -> Rc<Shape>;

    /// Lookup the slot index for `name`. Returns None if the shape
    /// does not carry the property.
    pub fn slot_of(&self, name: &str) -> Option<u32>;

    /// Number of slots in this shape.
    pub fn slot_count(&self) -> u32;

    /// Iterate (name, slot_index) pairs in insertion order. The
    /// §10.1.11 enumeration-order primary source.
    pub fn iter_slots(&self) -> impl Iterator<Item = (&str, u32)>;

    /// Raw pointer for IC stub cache-key emission. The pointer is
    /// stable for the lifetime of any Rc<Shape> the stub keeps alive.
    pub fn as_ptr(self: &Rc<Shape>) -> *const Shape {
        Rc::as_ptr(self)
    }
}
```

The API surface is small by design. Closure rounds may extend it (e.g., `Shape::iter_transitions`, `Shape::depth`) when LeJIT-Σ's bench harness or shape-mediated `Object.keys` enumeration calls for it.

## 5. `ShapeRegistry` — there isn't one (in the first cut)

The pre-design constraint named a `ShapeRegistry` type holding the root shape. On reflection it's unnecessary: the root shape is one `Rc<Shape>` stored on `Runtime`; the transition tree is self-organizing because every transition checks the parent's `transitions` table for reuse.

The `ShapeRegistry` becomes a real type when one of these calls for it:
- Cross-Runtime shape sharing (not in scope; cruftless is single-Runtime).
- Shape garbage collection (orphaned shapes whose only references are the parent's transition-table entry — these are inherently held alive by the chain; no GC pressure until the chain itself is rooted out, which is rare).
- Statistics / inspection surface (`registry.shape_count()`, `registry.dump_tree()`) — diagnostic, deferred.

The first cut adds one field to `Runtime`:

```rust
pub struct Runtime {
    // ... existing fields ...
    pub shape_root: Rc<Shape>,
}
```

Initialized in `Runtime::new()` to `Shape::root()`. Shape-EXT 4 makes `Object::new_ordinary()` start in Shaped form at this root shape.

## 6. Interior mutability for `Shape::transitions`

`Shape::transition_to(&Rc<Shape>, name)` needs to mutate `self.transitions` to insert a new child. But `self` is `&Rc<Shape>` (we want shared ownership across transition chains), so we need interior mutability on `transitions` only.

Choice: `RefCell<SmallOrLargeMap<String, Rc<Shape>>>` for transitions. The `slots` field stays plain (immutable after construction). The `parent` field stays plain.

Cost: one RefCell borrow per transition lookup. RefCell's borrow check is one comparison + flag-set; negligible vs the lookup itself.

Alternative considered: `OnceCell` per name. Rejected because transitions are added incrementally over the shape's lifetime (each property addition that hasn't been seen at this shape before allocates a new child); OnceCell expects single-shot.

## 7. `ObjectStorage` enum

```rust
pub enum ObjectStorage {
    /// First-cut storage form for property-shape-eligible Objects per
    /// shapes seed §IV carve-out. Slots Vec is indexed by Shape::slot_of(name).
    Shaped(Rc<Shape>, Vec<Value>),

    /// Fallback storage for non-Shape-eligible Objects per shapes seed
    /// §IV: non-Ordinary kinds, Symbol-keyed properties, accessor /
    /// non-default descriptors, post-delete migrations, post-ceiling
    /// migrations.
    Dictionary(IndexMap<PropertyKey, PropertyDescriptor>),
}
```

**Invariant**: a Shaped object's `Vec<Value>` length equals its `Shape::slot_count()`. The invariant holds because transitions always add one slot (the corresponding value push happens at the same call site that advances the shape pointer).

**Memory cost**: the enum discriminant is 1 byte (Rust niche-optimizes when possible). `Shaped` variant is `(Rc<Shape>, Vec<Value>)` = 8 + 24 = 32 bytes. `Dictionary` variant is `IndexMap<...>` = ~56 bytes. Object's storage footprint shrinks for shape-eligible objects (modal case) and stays the same for dictionary-form objects.

**Object field rename**: `Object.properties: IndexMap<...>` becomes `Object.storage: ObjectStorage`. This is a one-line type change at `value.rs:224` with the rest of the file's API surface shimmed to dispatch through the storage form.

## 8. Migration triggers (Shaped → Dictionary)

A Shaped object transitions to Dictionary form on:

1. **Delete**: `remove_str(name)` while in Shaped form. Slot-based storage cannot represent a deleted slot without complex re-keying that v1 defers. Migration: build IndexMap from the shape's slot iter + Vec<Value> contents, switch storage variant, perform the delete on the resulting IndexMap.

2. **Non-default descriptor**: any install path that produces accessor properties or non-`{w:t, e:t, c:t}` data properties. Migration: same as delete, then perform the descriptor install on the IndexMap.

3. **Symbol key**: `object_set_pk` with `PropertyKey::Symbol(_)`. Migration: same as delete, then perform the Symbol-keyed install.

4. **Shape-tree complexity ceiling**: when a shape's slot count exceeds a threshold AND the transition table has fanout above a threshold. First-cut thresholds: `slot_count > 32 && transition_fanout > 16`. Numbers chosen conservatively from V8's "dictionary mode" precedent; tunable per closure round.

The migration helper lives at `Object` level:

```rust
impl Object {
    pub fn migrate_to_dictionary(&mut self) {
        if let ObjectStorage::Shaped(shape, values) = &self.storage {
            let mut dict = IndexMap::new();
            for (name, slot) in shape.iter_slots() {
                dict.insert(
                    PropertyKey::String(name.to_string()),
                    PropertyDescriptor {
                        value: values[slot as usize].clone(),
                        writable: true, enumerable: true, configurable: true,
                        getter: None, setter: None,
                    }
                );
            }
            self.storage = ObjectStorage::Dictionary(dict);
        }
    }
}
```

Idempotent: no-op if already Dictionary. Cost: O(N) in slot_count + N Value clones (mostly cheap for the common cases — small integers, strings via Rc, object handles).

## 9. `object_get` / `object_set` dispatch

Shape-EXT 4's modification to the read path at `interp.rs:6214`:

```rust
pub fn object_get(&self, id: ObjectRef, key: &str) -> Value {
    // (existing Array.length special case, etc.)
    let mut cur = Some(id);
    while let Some(c) = cur {
        let o = self.obj(c);
        match &o.storage {
            ObjectStorage::Shaped(shape, values) => {
                if let Some(slot) = shape.slot_of(key) {
                    return values[slot as usize].clone();
                }
            }
            ObjectStorage::Dictionary(props) => {
                if let Some(d) = props.get(&PropertyKey::String(key.to_string())) {
                    return d.value.clone();
                }
            }
        }
        // (existing well-known-Symbol @@ fallback)
        cur = o.proto;
    }
    Value::Undefined
}
```

Shape-EXT 5's modification to `object_set_pk` is more involved — see §10.

## 10. Write path: `object_set_pk` + `set_own` flow

The write path has three sub-cases per receiver:

**(a) Receiver is Shaped, key is already in the shape.** In-place value mutation at `values[slot]`. No shape transition. Matches §10.1.9 OrdinarySet preserve-attrs semantics (descriptor stays `{w:t, e:t, c:t}` by invariant).

**(b) Receiver is Shaped, key is new.** Allocate or reuse child shape via `shape.transition_to(name)`; push new value to `values`; advance object's shape pointer to the child. Slot index of the new property is `(parent shape's slot_count)`.

**(c) Receiver is Dictionary.** Existing IndexMap path unchanged.

If any of the migration triggers fires (§8), call `migrate_to_dictionary` before performing the operation.

## 11. IC consumer API

```rust
impl Object {
    /// Returns the IC cache-key tuple iff the object is in Shaped form
    /// and the name resolves to a slot. The (*const Shape, u32) is
    /// stable for the lifetime of any Rc<Shape> held by the caller
    /// (which is what an IC stub must keep alive to safely dereference
    /// the slot in the receiver Object).
    pub fn shape_ptr_and_slot_for(&self, name: &str) -> Option<(*const Shape, u32)> {
        match &self.storage {
            ObjectStorage::Shaped(shape, _) => {
                shape.slot_of(name).map(|slot| (Rc::as_ptr(shape), slot))
            }
            ObjectStorage::Dictionary(_) => None,
        }
    }
}
```

**Safety story for the raw pointer**: the `*const Shape` returned is valid as long as some Rc<Shape> in the program references the same allocation. The IC stub must:
- Keep a `Rc<Shape>` alongside the `*const Shape` cache to guarantee the allocation outlives the stub.
- Check `Rc::as_ptr(receiver_shape) == cached_ptr` on stub entry (the fast-path comparison).
- On match: load `receiver.storage` as Shaped, read the slot at the cached index.
- On miss: fall through to the slow path (Cranelift-emitted runtime helper call), optionally patch the stub with the new shape.

LeJIT-Σ Pilot's substrate-introduction round consumes this API as documented.

## 12. Public surface for downstream consumers

Outside the Shape pilot crate, exposed types:

```rust
// in pilots/rusty-js-shapes/derived/src/lib.rs
pub use crate::shape::Shape;
pub use crate::storage::ObjectStorage;
```

Consumed by `pilots/rusty-js-runtime/derived/Cargo.toml` as a path dependency. The runtime's `value.rs` imports `ObjectStorage` and uses it as the `Object.storage` field type. The runtime's `interp.rs` does not depend on Shape internals; it dispatches through `ObjectStorage` variants.

LeJIT consumes `Shape` directly via `Rc::as_ptr` for IC cache-key emission. Pilot LeJIT-Σ's Cargo.toml adds the path dep when its substrate-introduction round lands.

## 13. Test surface

The Shape-EXT 3 crate scaffold ships with these tests:

- `Shape::root()` returns a shape with `slot_count() == 0` and empty `iter_slots`.
- Two `transition_to` calls with the same name from the same parent return `Rc::ptr_eq` shapes (Pred-shape.2 corroboration).
- Two `transition_to` calls with different names from the same parent return distinct shapes.
- A chain of N transitions produces a shape with `slot_count() == N` and `iter_slots` yielding the names in addition order.
- `slot_of` returns the right index for added properties; None for unknown.
- Shape allocation count after a randomized property-addition workload: linear in unique transition paths, not exponential (Pred-shape.3 corroboration; bounded probe — full corroboration awaits Shape-EXT 4 + diff-prod gate).

The integration tests (Shape-EXT 4 onward) live in the runtime crate, gated on diff-prod 42/42 + test262-sample 77.6% baselines.

## 14. Anti-design points (carve-outs from this design)

The first cut **does not** include:

- **Polymorphic IC support** beyond what Object::shape_ptr_and_slot_for produces. LeJIT-Σ's stub caches one shape per IC site; polymorphic-IC (linear-scan of N cached shapes) is a closure round on the LeJIT side, not the Shape side.
- **Shape garbage collection.** Shapes are held alive by their parent's `transitions` table + every Object holding them. Orphaned shapes (rare in practice — a deleted-then-recreated chain) keep the chain in memory. Acceptable for v1; tunable if memory pressure surfaces.
- **Hidden-class-aware enumeration acceleration.** §10.1.11 enumeration through `iter_slots` is O(N); a Vec scan. Faster paths (slot bitmap, etc.) are not v1.
- **Cross-Runtime shape sharing.** Single-Runtime is the only shape consumer.
- **Concurrent shape access.** Rc not Arc; cruftless's runtime is single-threaded.
- **Shape-mediated Object.keys / Object.entries / Object.values acceleration.** First-cut version walks `iter_slots` like the IndexMap version walks `keys`. Faster bulk-iteration is a closure round.

## 15. Forward to Shape-EXT 3

Shape-EXT 3 scaffolds `pilots/rusty-js-shapes/derived/` Cargo crate:

```
pilots/rusty-js-shapes/derived/
  Cargo.toml                    # name = "rusty-js-shapes", deps: smallvec, indexmap
  src/
    lib.rs                      # pub use shape::Shape; pub use storage::ObjectStorage;
    shape.rs                    # Shape + SmallOrLargeMap + Shape::root + transition_to
    storage.rs                  # ObjectStorage enum + migrate_to_dictionary
    tests/                      # unit tests per §13
```

LOC estimate: ~250 for `shape.rs`, ~80 for `storage.rs`, ~150 for tests. Test-only; Object struct unchanged in Shape-EXT 3. The crate is consumable but unwired.

Shape-EXT 4 changes `Object.properties` → `Object.storage` and rewires `object_get` / `object_set_pk` / `set_own` to dispatch through the storage form. **First round with diff-prod + test262-sample gates active.**

---

*Shape-EXT 2 closes. Output: this file. No code change. Next round: Shape-EXT 3 (crate scaffold).*
