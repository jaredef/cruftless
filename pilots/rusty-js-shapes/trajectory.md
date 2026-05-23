# rusty-js-shapes — Trajectory

Chronological resume anchors for the Shape (hidden classes) workstream. Reads seed.md first; this file is the time-ordered record of substrate moves and their yields.

Format: one section per "Shape-EXT" (extension round). Same shape as `pilots/rusty-js-jit/trajectory.md` and the top-level `trajectory.md`.

---

## Shape-EXT 0 — 2026-05-22 (workstream founding)

### Headline

Apparatus-tier round. Pilot founded per LeJIT seed §I.2's substrate-amortization staging (JIT-EXT 25 pre-file). No substrate code; the pilot's seed.md + trajectory.md + docs/ scaffold land as the locale's coordinate-uniqueness anchor per Doc 737 §IV.

The trigger was the keeper's "Begin" directive following JIT-EXT 25's telos sharpening. Per Doc 729 §A8.13 substrate-amortization, this pilot is the substrate-introduction round; Pilot LeJIT-Σ (IC stub emitter, hand-rolled aarch64) is the closure round queued behind it. The two compose at the `Object::shape_ptr_and_slot_for(name) -> Option<(*const Shape, u32)>` API boundary (specified in seed §III Shape-EXT 7).

### Substrate delivered

- `pilots/rusty-js-shapes/seed.md` — workstream telos, apparatus, methodology, carve-outs, standing artefacts, resume protocol, composition with LeJIT, falsifiers, hypostatic boundary. ~155 lines.
- `pilots/rusty-js-shapes/trajectory.md` (this file) — per-EXT log.
- `pilots/rusty-js-shapes/docs/` — directory scaffold for Shape-EXT 1 / 2 outputs.

### Locale registration

Per Doc 737 §IV the locale's coordinate is the filesystem path `pilots/rusty-js-shapes/` relative to the engagement's locale root `pilots/`. Locale tag `L.rusty-js-shapes`. Parent reference: cruftless engagement (`/home/jaredef/rusty-bun`). Sibling cross-reference: `L.rusty-js-jit` (LeJIT seed §I.2 pre-files this pilot as the dependency).

The locale tree at engagement root post-founding:

```
pilots/
  rusty-js-ast/
  rusty-js-bytecode/
  rusty-js-caps/
  rusty-js-esm/
  rusty-js-gc/
  rusty-js-ir/
  rusty-js-jit/             (LeJIT, per JIT-EXT 25 internal rename)
  rusty-js-parser/
  rusty-js-pm/
  rusty-js-runtime/
  rusty-js-shapes/          (this pilot, founded Shape-EXT 0)
  diff-prod/
  tls/
  web-crypto/
```

Twelve top-level locales. The fractal coverage density per Doc 733 §V advances one tick.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable. This round produces no substrate that admits engine-diff probing; the workstream's first probe will land at Shape-EXT 4 (the first round with code that affects runtime behavior).

Per Doc 734 §V: growth mechanism (a) tier-relocation recursion — the JIT-EXT 25 sharpening identified that the IC fast-path lives at the Shape substrate tier, downstream of LeJIT's tier. The pilot's founding is the structural acknowledgement that the substrate work belongs at its own coordinate per Doc 737 §IV's promotion threshold.

### Composition with prior corpus work

- **Doc 581 — Pin-Art and the Resume Vector.** The seed.md + trajectory.md pair is Doc 581's standing instrument. This pilot's founding instantiates the instrument at the `pilots/rusty-js-shapes/` coordinate per Doc 733's fractal recurrence + Doc 737's within-tier coordinate discipline.
- **Doc 729 §A8.13 — substrate-amortization staging.** Substrate-introduction round (this pilot) precedes closure rounds (LeJIT-Σ family). Order is fixed by the consumer-substrate dependency.
- **Doc 731 — JIT as lowering-compiler tier.** The Shape substrate is the IC fast-path's cache key supplier; per Doc 731 §VII R5 (deopt sites finite-enumerable per emitted module), the (shape_ptr, slot_offset) tuple is the IC's monomorphic key, and shape transitions are the deopt triggers at the IC-cache-miss layer.
- **Doc 735 §X.g — substrate-classification space.** Shape descriptors are a T1 (process-start) substrate by lifetime (allocated as objects are constructed; the shape tree persists across the process) at the cost-stratum tier where shared-shape-pointer comparison replaces name-key hashmap probing.
- **Doc 737 §IV — locale as coordinate.** The founding registers `pilots/rusty-js-shapes/` as the workstream's coordinate; coordinate-uniqueness is filesystem-structural.
- **Doc 738 §II — source-tier coordinate system.** Shape-tier identifiers will fit the five-axis convention space: `__shape_*` prefix per §II.a; install via `set_own_internal` per §II.c when shape fields land on Object; `shape_lookup_via` suffix per §II.b for Runtime-dispatching shape accessors; pillar-path `pilots/rusty-js-shapes/derived/src/` per §II.e.

### Open scope at Shape-EXT 0 close

1. **Shape-EXT 1 — Object layout survey.** Read `pilots/rusty-js-runtime/derived/src/value.rs` Object representation. Document every function that constructs / mutates / reads Objects. Output: `pilots/rusty-js-shapes/docs/object-layout-survey.md`. Apparatus-tier round; no code.
2. **Shape-EXT 2 — Shape data-structure design.** Output: `pilots/rusty-js-shapes/docs/shape-design.md`.
3. **Shape-EXT 3 — Crate scaffold.** First code round; introduces `pilots/rusty-js-shapes/derived/` Cargo crate with Shape + ShapeTransition + ShapeRegistry. Test-only; not wired.
4. **Shape-EXT 4 — Shape-form storage in Object.** First round with diff-prod + test262-sample gates active.

### Resume protocol

Read seed §I (telos) + seed §III (methodology) + this entry. Next substrate move is Shape-EXT 1: the Object layout survey. The work is reading + classifying; no code.

---

*Shape-EXT 0 closes the founding round. The workstream's locale exists; the coordinate is registered. The substrate work begins at Shape-EXT 1 when keeper directs.*

---

## Shape-EXT 1 — 2026-05-22 (Object layout survey)

### Headline

Apparatus-tier round. No code change. Surveys the current `pilots/rusty-js-runtime/derived/src/value.rs` + `interp.rs` Object representation against the substrate the Shape-EXT 4 introduction will reach into. Anchors Shape-EXT 2's design against measured numbers rather than guessed ones.

### Substrate delivered

- `pilots/rusty-js-shapes/docs/object-layout-survey.md` (~150 lines). Surveys Object anatomy, the construction / read / write API surface, eligibility filter for the Shape substrate, spec invariants that must be preserved, risk areas (~30 direct `properties.insert` sites all non-Shape-eligible by descriptor shape), and pre-design constraints for Shape-EXT 2.

### Measured contract surface

| API | Call-site count (runtime crate) | Shape-eligible subset |
|---|---:|---:|
| `alloc_object` | ~338 | ~120 (Ordinary kind) |
| `object_get` | ~360 | varies by receiver kind |
| `object_set` / `object_set_pk` | ~417 | majority (string-keyed user-default) |
| Direct `obj.properties.insert(...)` | ~30 | 0 (all install accessor / non-default descriptors) |

The migration's blast radius is bounded by `set_own` / `object_set_pk` (write path) + `object_get` (read path). No call-site grep-and-fix required; the branch lives inside the helper functions.

### Eligibility filter derived

A property addition is Shape-eligible iff: (1) `Object.internal_kind == InternalKind::Ordinary`, (2) the key is `PropertyKey::String(_)` (not Symbol), (3) the install path is `set_own` or `object_set` / `object_set_pk` (user-default `{w:t, e:t, c:t}`). Shape→Dictionary migration triggers on delete, non-default descriptor, Symbol-key addition, or shape-tree complexity ceiling. One-way per seed §IV carve-out.

### Spec invariants the Shape substrate must preserve (catalogued)

- §10.1.11 OrdinaryOwnPropertyKeys insertion order (the shape's name→slot map IS the order record).
- §10.1.9 OrdinarySet preserve-attrs-on-re-set semantics (in-place value mutation at the slot; no shape transition for re-set).
- Array.length synthetic computation (Arrays bypass Shape; carve-out preserved).
- Well-known-Symbol @@-prefix fallback in `object_get` (fires on Dictionary path or proto-chain ancestors; orthogonal to Shape).
- Proto-chain walking (independent of storage form per object).
- Accessor descriptor dispatch (accessors are non-Shape-eligible; Dictionary path unchanged).

### Pre-design constraints for Shape-EXT 2 (carried forward)

1. `Shape` struct: name→slot map (smallvec for tiny, hashmap above), transition table keyed on `(String, descriptor-class)`, parent shape pointer.
2. `ShapedObject` storage layout: `(Rc<Shape>, Vec<Value>)`. Rc-clone cost amortized via shape sharing; Vec realloc amortized O(1).
3. `ShapeRegistry` lifetime: root shape singleton per Runtime; transitions stored on parent shapes.
4. `ObjectStorage` enum: `Shaped(Rc<Shape>, Vec<Value>) | Dictionary(IndexMap<PropertyKey, PropertyDescriptor>)`. Object.properties becomes Object.storage.
5. IC consumer API: `Object::shape_ptr_and_slot_for(name: &str) -> Option<(*const Shape, u32)>`. Stable pointer for IC stub lifetime because shapes are Rc-shared and immutable post-construction (transitions create new shapes; existing ones are never mutated).
6. Transition-table key: `(String, ())` name-only for first cut; descriptor-class invariant by carve-out construction. Future-flex to `(String, DescriptorClass)` if Symbol-keyed or non-default-descriptor closure rounds need it.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization.** This round produces the substrate-introduction round's reading material; Shape-EXT 2 produces its design; Shape-EXT 3 lands code.
- **Doc 730 §XII–§XVI deviation pipeline.** No probe gated on this round (no behavioral change). Shape-EXT 4 is the first round under the §XVI bidirectional engine-diff oracle.
- **Doc 738 §II source-tier conventions.** The survey maps the existing identifier conventions in `value.rs` / `interp.rs` (`set_own*` family, `object_get` / `object_set_pk` family) onto the Shape pilot's planned `__shape_*` prefix and `_via` suffix conventions. Cross-axis consistency preserved by construction.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no probe gated). Per Doc 734 §V: growth mechanism (a) tier-relocation recursion — the survey identified that the substrate work belongs at the Object-tier code locations (`value.rs:217+`, `interp.rs:6004+`, `interp.rs:6168+`) and that the call-site migration burden is zero (the abstraction boundary is at `set_own` / `object_set_pk` / `object_get`).

### Open scope at Shape-EXT 1 close

1. **Shape-EXT 2** — Shape data-structure design. Decides the smallvec vs hashmap thresholds, the transition-table keying scheme, the migration-trigger ceiling, the `ObjectStorage` enum layout. Output: `pilots/rusty-js-shapes/docs/shape-design.md`. Apparatus-tier round; no code.
2. **Shape-EXT 3** — Crate scaffold. First code round. `pilots/rusty-js-shapes/derived/Cargo.toml`, `src/shape.rs`, `src/transition.rs`, `src/registry.rs`, `tests/`. Test-only; not yet wired into Object.
3. **Shape-EXT 4** — Object gains `ObjectStorage` field. **First round with diff-prod 42/42 + test262-sample 77.6% gates active.**

### Cumulative status at Shape-EXT 1 close

LOC delta: 0 (apparatus-tier round). docs/ artifacts: 1. Locale state unchanged.

The substrate-introduction round's reading phase is complete. The design phase (Shape-EXT 2) begins when keeper directs.

---

*Shape-EXT 1 closes. The contract surface is measured. Shape-EXT 2 designs against it.*

---

## Shape-EXT 2 — 2026-05-22 (Shape data-structure design)

### Headline

Apparatus-tier round. No code change. Designs the `Shape`, `ObjectStorage`, transition table, and IC consumer API against Shape-EXT 1's pre-design constraints. Output: `pilots/rusty-js-shapes/docs/shape-design.md` (~250 lines).

### Substrate delivered

- `pilots/rusty-js-shapes/docs/shape-design.md` — concrete design for Shape struct (eager-denormalized name→slot map + transition table + parent backptr); ObjectStorage enum (Shaped | Dictionary cohabitation); transition_to API with parent-reuse to satisfy Pred-shape.2; interior-mutability via RefCell on the transitions table only; migration helper (Shaped → Dictionary on delete / non-default descriptor / Symbol key / complexity ceiling); IC consumer API `Object::shape_ptr_and_slot_for(name) -> Option<(*const Shape, u32)>` with the raw-pointer safety story (IC stub keeps `Rc<Shape>` alongside `*const Shape` cache).

### Design decisions (the load-bearing ones)

1. **No `ShapeRegistry` type in the first cut.** Root shape lives as one `Rc<Shape>` on `Runtime::shape_root`. Transition tree is self-organizing via per-shape `transitions` tables. Registry promoted to a type if cross-Runtime sharing or shape GC pressure surfaces (neither in v1 scope).
2. **`SmallOrLargeMap<K, V>` newtype enum** for both `slots` (inline cap 8) and `transitions` (inline cap 4). Linear scan for the modal case; HashMap activation lazily at cap+1.
3. **Eager denormalization of `slots`** — every shape carries the FULL name→slot map, not just the parent-delta. Trades memory for O(1) lookup (no parent walk per property read). Closure-round revisit if memory pressure surfaces.
4. **Transition-table key is `String` name alone** for first cut. Descriptor-class invariant by carve-out construction (only `{w:t, e:t, c:t}` data properties are Shape-eligible). Future-flex to `(String, DescriptorClass)` if closure rounds lift the Symbol-keyed or non-default-descriptor carve-outs.
5. **Interior mutability via `RefCell<SmallOrLargeMap<...>>` on `transitions` only**; `slots` and `parent` stay plain after construction. One RefCell borrow per transition lookup; negligible vs the lookup cost itself.
6. **`Object.properties` → `Object.storage: ObjectStorage`** is a one-line type rename at `value.rs:224`. The Object API surface (`set_own*`, `get_own*`, etc.) gets shimmed to dispatch through the storage variant; call-site API unchanged.
7. **Migration thresholds for Shaped → Dictionary on complexity**: `slot_count > 32 && transition_fanout > 16` (V8 dictionary-mode precedent, conservative).
8. **`Vec<Value>` for the slot store** (cheap push for transitions, amortized O(1) realloc) rather than `Box<[Value]>` (compact but realloc-and-replace).
9. **Identity invariant load-bearing**: two Objects with the same property-addition history share `Rc::ptr_eq` shape pointers. Single bug-risk site: `Shape::transition_to` MUST consult the parent's transitions table first and reuse. Falsifier wired into Shape-EXT 3 unit tests.
10. **IC raw-pointer safety story**: `*const Shape = Rc::as_ptr(&shape)` is valid as long as some Rc<Shape> references the allocation. IC stub keeps a `Rc<Shape>` alongside the cached `*const Shape` to guarantee the allocation outlives the stub.

### Anti-design points (carve-outs from this design)

The first cut excludes: polymorphic IC support beyond `shape_ptr_and_slot_for` (LeJIT-Σ closure round), shape garbage collection (acceptable v1 memory leak), hidden-class-aware enumeration acceleration (O(N) Vec scan is fine), cross-Runtime sharing, concurrent shape access (Rc not Arc), shape-mediated Object.keys/entries/values bulk acceleration.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization.** Shape-EXT 2's design feeds Shape-EXT 3 (scaffold) feeds Shape-EXT 4 (Object integration). Each round bounded; together the substrate-introduction.
- **Doc 729 §A8.28 descriptor-shape discipline.** The transition-table key's descriptor-class invariance comes from the carve-out (only `set_own`-installed user-default descriptors are Shape-eligible); `set_own_internal` and `set_own_frozen` install paths route to Dictionary.
- **Doc 735 §X.h three-probe-levels discipline.** The Shape pilot's (P2.a) strict-win claim will need bench + consumer-route + fuzz probes when Shape-EXT 4+ measure against the diff-prod 42/42 + test262-sample 77.6% baselines + a hidden-classes-specific fuzz probe over the property-addition-history space.
- **Doc 738 §II source-tier conventions.** The shape pilot's identifiers conform: `Shape` / `ShapeTransition` / `ObjectStorage` for public types (PascalCase per Rust); `slot_of` / `transition_to` / `shape_ptr_and_slot_for` for methods (snake_case); `__shape_data` internal sentinel reserved for if Object ends up carrying a shape-pointer slot directly (currently lives inside the ObjectStorage enum, so no `__`-prefixed leak to JS).

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no probe gated by this round). Per Doc 734 §V: growth mechanism (a) tier-relocation recursion — the design crystallized one structural decision the keeper hadn't named (the no-Registry-type call) and one risk site the survey hadn't located (the `Shape::transition_to` parent-reuse invariant as the single load-bearing identity gate).

### Open scope at Shape-EXT 2 close

1. **Shape-EXT 3** — Crate scaffold. `pilots/rusty-js-shapes/derived/Cargo.toml` + `src/{lib,shape,storage}.rs` + tests. LOC estimate: ~480 (~250 shape.rs, ~80 storage.rs, ~150 tests). Test-only; Object struct unchanged in Shape-EXT 3.
2. **Shape-EXT 4** — Object integration. `Object.properties` → `Object.storage`. `object_get` / `object_set_pk` / `set_own` dispatch through ObjectStorage variant. **First round with diff-prod 42/42 + test262-sample 77.6% gates active.**

### Cumulative status at Shape-EXT 2 close

LOC delta: 0. docs/ artifacts: 2 (survey + design). Locale state unchanged.

The substrate-introduction round's design phase is complete. The first-cut scaffolding (Shape-EXT 3) begins when keeper directs.

---

*Shape-EXT 2 closes. Output: docs/shape-design.md. Shape-EXT 3 scaffolds against it.*

---

## Shape-EXT 3 — 2026-05-22 (crate scaffold)

### Headline

First code round. Crate `pilots/rusty-js-shapes/derived/` scaffolded against the Shape-EXT 2 design. `Shape` type + `SmallOrLarge{Slot,Transition}Map` two-form variants + `transition_to` identity gate + IC consumer pointer (`as_raw_ptr`) + ten unit tests. **10/10 tests PASS** on first build; Pred-shape.2 (same-history-same-pointer) and Pred-shape.3 (linear shape count in unique paths) corroborated at the unit-test layer. Test-only; Object struct unchanged in rusty-js-runtime.

### Substrate delivered

- `pilots/rusty-js-shapes/derived/Cargo.toml` — crate manifest. `smallvec = "1.13"` is the only dep.
- `pilots/rusty-js-shapes/derived/src/lib.rs` — `pub use shape::Shape;` + a no-deps-on-runtime architectural note (see §Design divergence below).
- `pilots/rusty-js-shapes/derived/src/shape.rs` (~330 LOC including tests):
  - `Shape` struct with `slots: SmallOrLargeSlotMap`, `transitions: RefCell<SmallOrLargeTransitionMap>`, `parent: Option<Rc<Shape>>`, `slot_count: u32`.
  - `SmallOrLargeSlotMap` two-form enum: `Small(SmallVec<[(String, u32); 8]>)` linear scan; `Large(Vec<(String, u32)>, HashMap<String, u32>)` denormalized (Vec preserves insertion order for §10.1.11 enumeration; HashMap provides O(1) lookup). Promotion at SLOTS_INLINE_CAP + 1.
  - `SmallOrLargeTransitionMap` two-form enum: `Small(SmallVec<[(String, Rc<Shape>); 4]>)`; `Large(HashMap<String, Rc<Shape>>)`. Promotion at TRANSITIONS_INLINE_CAP + 1.
  - `Shape::root()` — empty singleton-shape constructor.
  - `Shape::transition_to(self: &Rc<Shape>, name)` — the load-bearing identity gate. Consults parent's transitions table; reuses existing child shape if present; otherwise allocates new child + registers transition + returns. The reuse IS Pred-shape.2.
  - `Shape::slot_of(&self, name) -> Option<u32>`, `slot_count`, `iter_slots`, `parent`, `as_raw_ptr`, `transition_count`.
  - `Debug` impl for diagnostic dumps.
- `Cargo.toml` (workspace root) — `pilots/rusty-js-shapes/derived` registered as workspace member.

### Test results

10/10 PASS:

| test | corroborates |
|---|---|
| `root_is_empty` | constructor invariant |
| `single_transition_assigns_slot_zero` | first-slot assignment |
| `same_transition_same_shape` | **Pred-shape.2** (Rc::ptr_eq identity gate) |
| `different_transitions_distinct_shapes` | non-collision of distinct names |
| `chain_preserves_insertion_order_and_identity` | §10.1.11 enumeration order + Pred-shape.2 across chains |
| `order_divergent_chains_distinct` | Object{x,y} ≠ Object{y,x} at shape tier |
| `slot_map_promotes_past_inline_cap` | SmallOrLarge promotion behaves identically across the boundary |
| `transition_map_promotes_past_inline_cap` | identity invariant holds across transition-map promotion |
| `shape_count_linear_in_unique_paths` | **Pred-shape.3** bounded (5 distinct paths → 5 distinct leaf shapes, replayed 100× per object) |
| `as_raw_ptr_is_rc_pointer` | IC consumer-API stable-pointer contract |

Build: `cargo build --release -p rusty-js-shapes` finished in 0.81s. Test: `cargo test --release -p rusty-js-shapes` finished in 0.00s (10 tests). The crate is isolated; no other workspace member affected.

### Design divergence from Shape-EXT 2

One design decision was revised at scaffold-time:

- **`ObjectStorage` will live in `rusty-js-runtime`, not in `rusty-js-shapes`.** Shape-EXT 2 §1 + §7 placed it in this crate; doing so would force `rusty-js-shapes` to depend on `rusty-js-runtime::value::{Value, PropertyKey, PropertyDescriptor}`, but `rusty-js-runtime` will depend on `rusty-js-shapes` for `Shape`. That is a cycle. Resolution: `Shape` is value-payload-agnostic and lives here; `ObjectStorage` and its `Vec<Value>` payload live in `rusty-js-runtime` where `Value` is defined. The IC consumer API `Object::shape_ptr_and_slot_for` likewise lives in `rusty-js-runtime` because `Object` does. The crate boundary is the clean fix.

The `lib.rs` carries the architectural note for future readers.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no probe gated; the integration tests at Shape-EXT 4 will be the first §XVI-eligible work).

Per Doc 734 §V: growth mechanism (a) tier-relocation recursion — the cycle-fix on the crate dependency direction surfaced at scaffold time and shifted ObjectStorage one tier (from rusty-js-shapes to rusty-js-runtime). The shift is recorded inline in `lib.rs` so future-readers see the rationale at the load site.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization.** Crate scaffolded; the substrate-introduction round's code phase begins. Shape-EXT 4 integrates against Object; Shape-EXT 5-7 close the substrate. Total runway estimate from Shape-EXT 0 founding to LeJIT-Σ-consumable surface: ~600-900 LOC per pilot seed §I.1, now revised to ~600 + ~120 (the ObjectStorage rust file in rusty-js-runtime) ≈ 720 LOC.
- **Doc 735 §X.h three-probe-levels discipline.** This round's unit tests are the **bench probe** (deterministic small-input corroboration); consumer-route probe activates at Shape-EXT 4 (the diff-prod 42/42 + test262-sample 77.6% gates); fuzz probe activates at Shape-EXT 5 with property-addition-history fuzz over the transition tree.
- **Doc 738 §II source-tier conventions.** The crate's identifiers conform: PascalCase types (`Shape`, `SmallOrLargeSlotMap`, `SmallOrLargeTransitionMap`); snake_case methods (`slot_of`, `transition_to`, `iter_slots`, `as_raw_ptr`); module-internal `SLOTS_INLINE_CAP` / `TRANSITIONS_INLINE_CAP` constants; pillar-path `pilots/rusty-js-shapes/derived/src/shape.rs` (§II.e); no engine-internal `__`-prefixed identifiers because this crate doesn't touch the JS-observability surface (Shape is a Rust-internal type; only the `Object::shape_ptr_and_slot_for` API will expose anything to LeJIT's IC tier).

### Pred disposition

- **Pred-shape.2** (same property-addition sequence → Rc::ptr_eq shape): corroborated in `same_transition_same_shape`, `chain_preserves_insertion_order_and_identity`, `transition_map_promotes_past_inline_cap`.
- **Pred-shape.3** (transition tree O(N) in unique add-sequences): corroborated bounded in `shape_count_linear_in_unique_paths` (5 sequences × 100 replays → 5 distinct leaf shapes). Full corroboration awaits Shape-EXT 4+ with the diff-prod fixture corpus driving real workloads through the tree.
- **Pred-shape.4** (stable IC pointer for stub lifetime): corroborated at the unit-test layer in `as_raw_ptr_is_rc_pointer`. Full corroboration awaits LeJIT-Σ's bench harness.
- **Pred-shape.1** (shaped per-op-cheaper than dictionary) and **Pred-shape.5** (Doc 738 §II convention conformance) await Shape-EXT 4+ measurement.

### Open scope at Shape-EXT 3 close

1. **Shape-EXT 4** — Object integration. Add `pub mod storage;` to `pilots/rusty-js-runtime/derived/src/`; define `ObjectStorage` enum there (per the §Design divergence reasoning); change `Object.properties: IndexMap<...>` → `Object.storage: ObjectStorage`; shim the Object API surface (`set_own*`, `get_own*`) to dispatch through the storage variant; add `Runtime::shape_root: Rc<Shape>` initialized in `Runtime::new()`. **First round with diff-prod 42/42 + test262-sample 77.6% gates active.** LOC estimate: ~150-200 (storage.rs ~80, value.rs shims ~50, Runtime init ~10, intrinsics.rs / interp.rs dispatch tweaks at the chokepoints).
2. **Shape-EXT 5** — Property addition through transitions (the write path). `set_own` on a Shaped object follows or creates the transition. `object_set_pk` branches on storage form. Diff-prod + test262-sample gates active. Add property-addition-history fuzz tests.
3. **Shape-EXT 6** — Migration triggers (delete, non-default descriptor, Symbol key, complexity ceiling).
4. **Shape-EXT 7** — IC consumer surface `Object::shape_ptr_and_slot_for` lifted to public; first call site documented for LeJIT-Σ.

### Cumulative status at Shape-EXT 3 close

LOC delta: ~340 (shape.rs 332 + lib.rs 22 + Cargo.toml). docs/ artifacts unchanged. Workspace builds clean; new crate isolated. PM-EXT 11+12 regression untouched (no integration yet).

The substrate-introduction round's code phase begins. Shape-EXT 4 wires the crate into Object.

---

*Shape-EXT 3 closes. The crate exists, builds, tests pass. The Rc::ptr_eq identity gate (Pred-shape.2) is corroborated at the unit-test layer. Shape-EXT 4 carries the integration risk.*

---

## Shape-EXT 4 — 2026-05-23 (Object integration: infrastructure only; enrollment deferred)

### Headline

First code round on the runtime side. `Object` gains `shape: Option<Rc<Shape>>` + `shape_values: Vec<Value>` parallel slots; the IC consumer API + migration helpers + shape-aware fast paths land in value.rs / interp.rs; the rusty-js-shapes path-dep wires into rusty-js-runtime. **42/42 diff-prod PASS** held; behavior identical to pre-shape baseline.

Scope narrowed mid-round from "auto-enroll new ordinary objects into Shaped form" to **"infrastructure only, enrollment deferred to Shape-EXT 5"** when a first enrollment attempt regressed diff-prod 39→31/42 PASS. The regression localized to ~41 direct `.properties` consumer sites (Map/Set internal storage iteration, JSON.stringify enumeration, for-in dispatch) that bypass the shape mechanism and miss shape-stored entries. Per Doc 729 §A8.13 substrate-amortization, that consumer-site migration belongs in its own focused round (Shape-EXT 5), not bundled into the infrastructure-introduction round.

### Substrate landed

- `pilots/rusty-js-runtime/derived/Cargo.toml` — `rusty-js-shapes = { path = "../../rusty-js-shapes/derived" }` added.
- `pilots/rusty-js-runtime/derived/src/value.rs`:
  - Object fields `shape: Option<Rc<Shape>>` + `shape_values: Vec<Value>`.
  - `impl Default for Object` — supports `..Default::default()` spread in literals.
  - Object methods `is_shaped`, `shape_get`, `shape_ptr_and_slot_for` (IC consumer API per shapes pilot docs/shape-design.md §11), `migrate_to_dictionary` (idempotent Shaped→Dictionary).
  - `set_own` shape-aware (in-place value mutation on existing slot; transition + push on new key; migrate-first on `__`-prefixed keys per shapes seed §IV).
  - `set_own_internal` / `set_own_frozen` / `insert_str` / `remove_str` migrate to Dictionary first.
  - `has_own_str` / `string_keys` / `string_key_clones` shape-aware (read shape slots before properties).
  - `new_ordinary()` starts `shape: None` (deferred enrollment; Shape-EXT 5 flips this default).
  - `new_array()` starts `shape: None` (Arrays bypass shapes per carve-out).
- `pilots/rusty-js-runtime/derived/src/interp.rs`:
  - `object_get` shape-aware fast path (slot lookup before IndexMap probe) on the receiver + proto-chain ancestors.
  - `object_set_pk` dispatches: `__`-prefixed keys migrate first (engine sentinels stay in Dictionary), string-keyed Shaped objects route through `set_own`, Symbol-keyed sets migrate first.
  - `ordinary_own_enumerable_string_keys` shape-aware (shape slots concatenate before properties entries in insertion order).
- 7 runtime crate source files (interp / intrinsics / module / promise / prototype / iterator / regexp) gained `..Default::default()` spread in 15 Object literal sites via a careful Python helper (regex matches `Object {` / `Self {` but skips `-> Object {` return-type and `impl Object {` block declarations).
- 1 cruftless host file (process.rs) + 1 register.rs site likewise updated.

### Build + gate results

| measure | before | after |
|---|---:|---:|
| `cargo build --release -p rusty-js-runtime` | clean | clean |
| `cargo build --release --bin cruft -p cruftless` | clean | clean |
| diff-prod PASS | 42/42 | **42/42** (no regression) |
| (intermediate auto-enroll attempt) | n/a | 31/42 (rolled back) |

### Why the scope narrowed

The first integration attempt set `Object::new_ordinary()` to start at the shape root. Cruft's runtime has ~41 sites that iterate `.properties` directly (most concentrated in JSON enumeration, Map/Set storage iteration, intrinsics enumeration helpers, for-in / Object.keys dispatch). When ordinary Objects started Shaped, those sites missed all shape-stored entries → empty enumerations → cascading test failures.

Two paths considered:
- **Path A**: make every consumer site shape-aware (touch 41 call sites). Big blast radius for one round; violates substrate-introduction discipline.
- **Path B**: enroll in Shape only on opt-in (closure round) and stabilize the infrastructure in this round. Narrow blast radius; clean substrate-introduction.

Path B chosen. Shape-EXT 5 lands the enrollment with carefully scoped migration shims per consumer family (Map/Set storage migrates on alloc; JSON enumeration adds shape-iteration; for-in dispatch likewise).

### Pred disposition

- **Pred-shape.2** (identity invariant) — preserved (unit-test layer); not exercised at integration tier yet (no Shaped objects in this round).
- **Pred-shape.3** (linear transition tree growth) — not exercised at integration tier yet.
- **Pred-shape.4** (stable IC pointer for stub lifetime) — API exists at Object::shape_ptr_and_slot_for, returns None for all objects in this round (no Shaped objects). Pilot LeJIT-Σ can scaffold against the API surface; IC hits become measurable when Shape-EXT 5 turns enrollment on.
- **Pred-shape.1** (shaped read per-op-cheaper than Dictionary) — not measurable this round.
- **Pred-shape.5** (Doc 738 §II convention conformance) — preserved by construction; all new identifiers fit the source-tier coordinate system.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization.** The narrowing IS the discipline operating as designed: when a substrate move's blast radius exceeded a focused round, split into infrastructure (this round) + enrollment (Shape-EXT 5). The empirical signal (diff-prod 39→31 on auto-enroll) localized the split point.
- **Doc 730 §XVI bidirectional engine-diff oracle.** The 31/42 regression localized the 11 failing fixtures to enumeration-pattern divergence; the four-case categorization is Case-1 (cruftless violated spec via missing-shape-enumeration), but the fix is structural (consumer migration) not local (per-fixture patch), so the move belongs in its own round.
- **Doc 735 §X.h three-probe-levels discipline.** Bench probe (Shape-EXT 3 unit tests) passes; consumer-route probe (diff-prod 42/42) passes; fuzz probe not yet activated (no enrollment to fuzz against).
- **Doc 738 §II source-tier coordinate system.** All new identifiers conform: `shape` / `shape_values` (snake_case fields), `shape_get` / `shape_ptr_and_slot_for` / `migrate_to_dictionary` / `is_shaped` (snake_case methods), no `__`-prefixed JS-visible names (the shape itself is engine-internal data structure, not JS-observable).

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable directly (no spec divergence introduced; behavior preserved). The 31/42 intermediate regression triggered a Case-1 read internally (cruftless violated own-property enumeration semantics under the auto-enroll attempt) → corpus growth mechanism (b) negative-finding amendment to the Shape-EXT 4 scope (narrowing to infrastructure-only).

Per Doc 734 §V: growth mechanism (a) tier-relocation recursion + (b) negative-finding amendment. The negative finding (auto-enroll regresses 11 fixtures) refined the framework's understanding of the substrate-introduction round's blast-radius limit; the trajectory entry records the refinement so Shape-EXT 5 plans against it explicitly.

### Open scope at Shape-EXT 4 close

1. **Shape-EXT 5** — Enrollment + consumer-site migration. Two-sub-step plan:
   - **5a**: identify the consumer-site families (Map/Set storage, JSON enumeration, for-in dispatch, Object.keys/values/entries, etc.). Document each family's migration shape (add shape-iteration vs migrate-first).
   - **5b**: implement the migration shims family by family, gating each on diff-prod 42/42 + test262-sample 77.6% baselines. When all consumers are shape-aware, flip `new_ordinary()` to start Shaped.
2. **Shape-EXT 6** — Dictionary fallback complexity-ceiling triggers (per design §8).
3. **Shape-EXT 7** — IC consumer surface promoted to public; first LeJIT-Σ call site documented.

### Cumulative status at Shape-EXT 4 close

LOC delta: ~150 (Object field additions, methods, dispatch shims, Default impl, 17 `..Default::default()` spreads). diff-prod 42/42 PASS unchanged. test262-sample not re-run this round (no behavior change to measure).

The shape data structure is now live on every Object. Pilot LeJIT-Σ can scaffold against `Object::shape_ptr_and_slot_for` with stable API contract; the API returns None today, will return Some on Shape-EXT 5 enrollment.

---

*Shape-EXT 4 closes. Infrastructure landed; enrollment deferred. The substrate-introduction-round discipline of Doc 729 §A8.13 held the round's blast radius even when the first attempt overreached.*

---

## Shape-EXT 5 — 2026-05-23 (open: spawns nested locale `consumer-migration`)

Per Doc 737 §II promotion threshold + the keeper's "Continue + set up seeds at every fractal locale that requires it" directive (2026-05-23 01:37-local), Shape-EXT 5's consumer-migration substrate has multi-rung shape (one rung per consumer family × 5 families + an enrollment-flip rung + a measurement rung), so it earns its own nested locale rather than inline accumulation in this trajectory.

**Spawned nested locale**: `pilots/rusty-js-shapes/consumer-migration/` (locale tag `L.rusty-js-shapes/consumer-migration`).

The nested locale's seed + trajectory + survey landed at CMig-EXT 0 (its founding round). When the nested workstream reaches its CMig-EXT 9 closure (post-enrollment-flip + Pred-shape.4 measurement), this Shape-EXT 5 row gets updated with the closure summary; until then it points to the nested locale.

See [consumer-migration/seed.md](consumer-migration/seed.md) and [consumer-migration/trajectory.md](consumer-migration/trajectory.md).

This row IS the load-bearing record of Shape-EXT 5; the substantive substrate work happens at the nested coordinate. Per Doc 733 §III composition relations: parent row records child existence by commit hash + one-line recognition; child's internal structure stays inside the child.

*Shape-EXT 5 stays open until CMig-EXT 9 closes the nested workstream.*
