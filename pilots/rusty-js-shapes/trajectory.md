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
