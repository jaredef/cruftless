# rusty-js-shapes — Resume Vector / Seed

**Locale tag**: `L.rusty-js-shapes` (per [Doc 737](../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md))

**Status as of 2026-05-22**: **WORKSTREAM FOUNDED (Shape-EXT 0)**. No substrate code yet. Pre-filed at the LeJIT seed §I.2 (JIT-EXT 25) as the substrate-introduction round per Doc 729 §A8.13 staging; spawn triggered by the JIT-EXT 25 telos sharpening and the keeper's "Begin" directive. The seed names the workstream; the first substrate move (Shape-EXT 1) surveys the current Object layout to anchor the contract surface.

**Workstream**: hidden classes (a.k.a. shapes, structures, maps) for cruftless's Object representation. Shared shape descriptors per property-addition history; transition tree between shapes; (shape_ptr, slot_index) IC fast-path target.

**Author**: 2026-05-22 session.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Sibling pre-file**: Pilot LeJIT-Σ (IC stub emitter at `pilots/rusty-js-jit/derived/src/stub_*.rs`; per LeJIT seed §I.2, closure round reusing this pilot's substrate).
**Composes with**:
- [Doc 731](../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — the alphabet-purity claim. Shapes are the substrate that lets the IC fast-path inline at the (shape_ptr, slot_offset) granularity Doc 731's strong form predicts.
- [Doc 729 §A8.13](../../../corpus-master/corpus/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — substrate-amortization staging. This pilot is the substrate-introduction round; LeJIT-Σ is the closure round.
- [Doc 738](../../../corpus-master/corpus/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md) — source-tier coordinate system. Shape-tier internal slots use `__shape_*` prefix per §II.a; install via `set_own_internal` per §II.c; pillar-path encoding lives at `pilots/rusty-js-shapes/derived/src/...` per §II.e.
- `pilots/rusty-js-runtime/derived/` — the consumer; Object layout in `value.rs` is the substrate this pilot extends.
- `pilots/rusty-js-jit/derived/` (LeJIT) — the downstream consumer; the IC stub emitter pilot reads (shape_ptr, slot_offset) as its fast-path cache key.

## I. Telos

Land shared shape descriptors for cruftless's Object representation such that:

1. **Two Objects constructed by the same property-addition sequence share the same shape pointer.** Identity is by structural history, not by reference. The shape carries the (property-name → slot-index) mapping.

2. **Property addition causes a shape transition.** Each shape carries a transition table keyed on (added-property-name, descriptor-shape per Doc 729 §A8.28); the result is a child shape with one more slot. The transition tree builds incrementally as user code constructs Objects.

3. **Property lookup by name becomes (shape lookup, slot index) instead of IndexMap walk.** `rt.object_get(id, key)` post-substrate: read object's shape pointer, look up the name in the shape's name→slot map (single hashmap probe, or array scan for tiny shapes), index into the object's slot vector. Pre-substrate path is preserved as fallback for non-shaped objects.

4. **The (shape_ptr, slot_index) tuple is the IC fast-path cache key.** Pilot LeJIT-Σ's hand-rolled stub emitter reads this tuple from a per-call-site cache; on shape match, inlines a 2-3-instruction shape-pointer-compare + slot-load + return; on miss, patches the stub or falls through to the slow path.

The success criterion is: **(a)** Object operations under the shaped representation match the pre-substrate behavior byte-for-byte across the diff-prod 42-fixture suite + test262-sample 5594-PASS baseline (no regressions), AND **(b)** the shape descriptor is consumable by LeJIT-Σ's IC stub emitter with stable layout (shape pointer is a stable `*const Shape` for the duration of the cached IC stub).

### I.1 Bounded first-cut telos

Per Doc 581 D2's bounded-first-cut discipline:

The first-cut Shape-substrate covers ordinary Objects allocated via `Object::new_ordinary()`. Exotic Objects (Arrays with their length-tracking, TypedArrays, Maps/Sets with their internal storage, Proxies) carry their own InternalKind and bypass the shape mechanism in this first cut; they retain the existing IndexMap-keyed storage. The shape mechanism applies where:

- The Object is `InternalKind::Ordinary`.
- Property additions are at non-Symbol string keys (Symbol keys handled at a later closure round; small surface, ~2% of property additions in the engagement's measured workload).
- Property descriptors are user-default `{w:t, e:t, c:t}` per Doc 729 §A8.28 `set_own`. Non-default descriptors (frozen / non-enumerable internal sentinels) bypass the shape mechanism since their install paths already discriminate them.

The first-cut closure criterion: ordinary Objects allocated and mutated through the standard `set_own` / `object_set` paths carry shape pointers; `object_get` reads through the shape; the diff-prod + test262-sample baselines hold; LeJIT-Σ can consume `(*const Shape, u32)` as the IC cache key.

## II. Apparatus

The Shape substrate is **a parallel storage stratum at the Object tier**, not a new resolver-instance. Object instances post-substrate carry one of two storage forms:

- **Shaped form**: `(Rc<Shape>, Vec<Value>)`. Shape carries the name→slot map and the transition table; Vec carries the per-instance values at slot indices. Property reads walk shape; property writes either follow an existing transition (shape pointer advances) or create a new transition (shape tree extends).
- **Dictionary form**: existing `IndexMap<PropertyKey, PropertyDescriptor>`. Used for Objects with non-default descriptors, deleted properties, Symbol-keyed properties, or properties past the shape-tree's complexity ceiling (when transition-tree branching exceeds a threshold, Object falls back to dictionary form per V8's "dictionary mode" pattern).

The two forms coexist per Object; the form-flag determines which dispatch path the property accessors take. The transition from shaped to dictionary is one-way (back-promotion to shape form is deferred work; not in first cut).

Per [Doc 730 §XII–§XVI](../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md), substrate moves at the Shape tier operate under the bidirectional engine-diff oracle: every shape-mechanism change is gated on the diff-prod 42-fixture suite passing under the change, and the test262-sample readings holding within ±0.5pp. A regression on either probe localizes the change to the substrate move that introduced it.

## III. Methodology

Each substrate round is its own Pin-Art cycle per Doc 581. The methodology template, specialized to this pilot:

1. **Shape-EXT 0 (this round)** — workstream founding. seed.md + trajectory.md + docs/ scaffold. No substrate code.

2. **Shape-EXT 1 — Object layout survey.** Read the current `pilots/rusty-js-runtime/derived/src/value.rs` Object representation; document the contract surface (which functions construct Objects, which functions mutate them, which functions read them). Output: `pilots/rusty-js-shapes/docs/object-layout-survey.md`. Apparatus-tier round; no code.

3. **Shape-EXT 2 — Shape data structure design.** Design the `Shape` struct: name→slot map (small-vector for tiny shapes, hashmap for larger), transition table keyed on (name, descriptor-class), parent shape pointer for traversal. Design the `ShapedObject` storage form. Output: `pilots/rusty-js-shapes/docs/shape-design.md`. Apparatus-tier round; no code.

4. **Shape-EXT 3 — Shape crate scaffold.** New crate `pilots/rusty-js-shapes/derived/` with `Shape` + `ShapeTransition` + `ShapeRegistry` types and a minimal API: `Shape::root() -> Rc<Shape>`, `Shape::transition_to(self, name, descriptor_class) -> Rc<Shape>`, `Shape::slot_of(&self, name) -> Option<u32>`. Test-only at this round; not yet wired into Object.

5. **Shape-EXT 4 — Shape-form storage in Object.** Object gains a `storage: ObjectStorage` field where `ObjectStorage = Shaped(Rc<Shape>, Vec<Value>) | Dictionary(IndexMap<...>)`. Existing IndexMap path renamed to Dictionary. New ordinary Objects start in Shaped form at the root shape. Property accessors branch on storage form. **Diff-prod gate**: 42/42 holds. **test262-sample gate**: within ±0.5pp of 77.6%.

6. **Shape-EXT 5 — Property addition through transitions.** `set_own` on a shaped Object follows or creates the appropriate transition; the Object's shape pointer advances. Shape-EXT 4 only covered reads against pre-populated shapes; this round adds the write path. **Diff-prod gate + test262-sample gate** as above.

7. **Shape-EXT 6 — Dictionary fallback triggers.** Deleted properties, non-default descriptors, Symbol-key additions trigger a one-way migration from Shaped form to Dictionary form. Migration preserves all existing slots. **Diff-prod gate + test262-sample gate** as above.

8. **Shape-EXT 7 — IC consumer surface.** Expose `Object::shape_ptr_and_slot_for(name) -> Option<(*const Shape, u32)>` as the stable IC cache key. LeJIT-Σ pilot consumes this in its substrate-introduction round.

9. **Shape-EXT 8+ — closure rounds.** Symbol-keyed shapes, back-promotion from Dictionary to Shaped form, polymorphic-IC support (Object with stable per-property type), shape-mediated `Object.keys` enumeration. Each is its own closure round under the §A8.13 amortization pattern.

## IV. Carve-outs and bounded scope

- **No JIT integration in this pilot.** Shape produces the IC cache key; LeJIT-Σ consumes it. The two pilots compose at the `shape_ptr_and_slot_for` API boundary; this pilot does not touch the JIT crate.

- **No exotic-Object integration in the first cut.** Arrays, TypedArrays, Maps, Sets, Proxies retain their existing per-kind storage. Shape applies only to `InternalKind::Ordinary`.

- **No back-promotion** from Dictionary to Shaped form. One-way Shaped→Dictionary only.

- **No shape-mediated polymorphic IC** in the first cut. Per-call-site shape caches are monomorphic (one cached shape per site); polymorphic (multiple shapes per site, with linear-scan dispatch) is queued for a later closure round.

- **No hidden-class GC integration.** Shapes live in an Rc-shared registry; GC sees them as ordinary heap-rooted objects. Moving-GC integration is queued with the broader GC substrate work.

These carve-outs preserve the substrate-introduction round's narrow scope per Doc 729 §A8.13. Each carve-out is a candidate closure round when its substrate becomes a bottleneck.

## V. Standing artefacts

- `pilots/rusty-js-shapes/seed.md` (this file).
- `pilots/rusty-js-shapes/trajectory.md` — per-EXT log.
- `pilots/rusty-js-shapes/docs/object-layout-survey.md` — Shape-EXT 1 output.
- `pilots/rusty-js-shapes/docs/shape-design.md` — Shape-EXT 2 output.
- `pilots/rusty-js-shapes/derived/` — Cargo crate from Shape-EXT 3 onward. `src/shape.rs`, `src/transition.rs`, `src/registry.rs`, `tests/`.

## VI. Resume protocol

Read [Doc 731](../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md), LeJIT seed §I.2 (the sibling pilot's sharpened telos), [Doc 729 §A8.13](../../../corpus-master/corpus/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md), [Doc 738 §II](../../../corpus-master/corpus/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md), then this seed, then trajectory.md. The next substrate move is Shape-EXT 1: the Object-layout survey. Output is documentation, not code. The work is reading + classifying; the deliverable is the contract-surface map that Shape-EXT 2 designs against.

Pin-Art tag prefix for this workstream: `Ω.5.P04.E0.shape-*` (runtime-side; the shape substrate sits at the Object-tier boundary inside the runtime). Per `host/tools/tag-grammar.md`, the handle is the substrate node the move touches.

## VII. Composition with LeJIT

The two pilots are staged per Doc 729 §A8.13:

| Round | Workstream | Substrate |
|---|---|---|
| Substrate-introduction | **rusty-js-shapes (this)** | Shape descriptors + ShapedObject form + transition tree + IC consumer surface. ~600–900 LOC estimate. |
| Closure 1 | LeJIT-Σ | Hand-rolled IC stub emitter (aarch64). Reads `(*const Shape, u32)` from this pilot. ~500-800 LOC. |
| Closure 2 | LeJIT-Σ' | IC stub emitter (x86_64). Same shape-key reader. |
| Closure 3 | LeJIT-Σ'' | Value-tag inline emitter for hot Op::GetProp / Op::SetProp paths. |
| Closure 4 | LeJIT-Σ''' | Polymorphic-IC support (depends on this pilot's polymorphic-shape closure round). |

The substrate-introduction round's LOC budget dominates; the closure rounds are small per the §A8.13 amortization pattern. Total work: ~2-3k LOC across the five rounds.

## VIII. Falsifiers

**Pred-shape.1.** Shaped Objects' property-read path is per-op-cheaper than IndexMap dictionary path. Measurable: micro-benchmark of `obj.x` reads in a hot loop, shaped vs dictionary. Falsifier: shape path within 10% of dictionary path or slower (in which case shape provides no per-op benefit and the §I claim weakens to "only useful for IC fast-path cache key").

**Pred-shape.2.** Two Objects constructed by the same property-addition sequence share the same shape pointer at the level of `Rc::ptr_eq`. Falsifier: two such Objects with distinct shape pointers (indicates the transition tree's keying is non-deterministic and the shape registry is broken).

**Pred-shape.3.** Shape transition tree does not grow unboundedly under normal workloads. Measurable: count shapes after running the diff-prod fixture suite; predict O(N) growth in N = unique property-addition sequences across the corpus. Falsifier: exponential growth indicating cycle bugs or non-canonical transition keying.

**Pred-shape.4.** The (shape_ptr, slot_offset) IC cache key is stable for the duration of a cached IC stub. Pilot LeJIT-Σ's bench harness exercises this directly: an IC stub cached with (shape_ptr_A, slot_3) should remain hit-correct across N invocations on objects that did not undergo shape transitions. Falsifier: cache misses where no shape transition occurred.

**Pred-shape.5.** Per Pred-738.4 cross-articulation convergence: shape-tier commits at coordinate `pilots/rusty-js-shapes/derived/src/shape.rs` produce identifiers fitting Doc 738 §II's five-axis convention space. Falsifier: a Shape-tier identifier that violates one of the five axes without an explicit (and corpus-articulated) reason.

## IX. Hypostatic boundary

Per [Doc 372](../../../corpus-master/corpus/372-the-method-of-the-corpus-as-derivation-not-collection.md), this seed operates at the functional layer. The Shape substrate is one realization of the hidden-classes pattern that V8 / JSC / SpiderMonkey have all implemented; cruftless's Pin-Art derivation against this pattern is not corpus-original at the algorithmic tier (the pattern is published codegen-literature substrate per Doc 735 §X.h.b). The corpus-original contribution is in the composition: hidden classes as the specific substrate-introduction round that LeJIT's hybrid stance (per LeJIT seed §I.2) requires, staged per Doc 729 §A8.13, with the §II two-form storage cohabitation pattern as the v1 compatibility-preservation move.

---

*Doc 581 standing instrument: this seed is the workstream's stable kernel. Changes to telos, apparatus, methodology, carve-outs, or composition relations land here; substrate moves land in trajectory.md.*
