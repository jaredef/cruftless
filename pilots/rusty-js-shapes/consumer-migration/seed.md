# rusty-js-shapes/consumer-migration — Resume Vector / Seed

**Locale tag**: `L.rusty-js-shapes/consumer-migration` (nested per [Doc 737](../../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md) §IV)

**Status as of 2026-05-23**: **FOUNDED (CMig-EXT 0)**. Sub-workstream of `pilots/rusty-js-shapes/` spawned per Shape-EXT 4's deferred-enrollment finding. The parent's Shape-EXT 4 round landed shape infrastructure on Object but kept `new_ordinary()` defaulting to `shape: None` because the first enrollment attempt regressed diff-prod 39→31/42 PASS. Root cause: ~41 sites across the runtime crate iterate `.properties` directly and bypass the shape mechanism. This sub-workstream migrates those sites family by family so enrollment can flip back on without regression.

**Workstream**: catalog + migrate the direct-`.properties` consumer sites to be shape-aware (either by adding shape-iteration or by migrating to Dictionary on access). When all consumer families are shape-aware, flip `Object::new_ordinary()` to start at the shape root → user-code `{}` literals are shaped by default → Pred-shape.1/.3/.4 become integration-measurable → Pilot LeJIT-Σ can consume real shape pointers.

**Author**: 2026-05-23 session.
**Parent**: `pilots/rusty-js-shapes/` (cruftless engagement's hidden-classes substrate-introduction pilot).
**Composes with**:
- [Parent seed](../seed.md) §III methodology — this sub-workstream IS the Shape-EXT 5 staging.
- [Parent Shape-EXT 4 trajectory entry](../trajectory.md) — names the deferred-enrollment finding that spawned this locale.
- [Doc 737 §IV](../../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md) — coordinate registration discipline.
- [Doc 729 §A8.13](../../../../corpus-master/corpus/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — substrate-amortization: this sub-workstream is one consumer-fanout from the infrastructure-introduction round above it.

## I. Telos

Migrate every direct `.properties` consumer site (~41 sites across the runtime crate) to one of three shape-aware patterns:

1. **Shape-iterate then properties-iterate**: enumeration sites that need to see every own-property (e.g., `Object.keys`, `JSON.stringify`, for-in dispatch). Pattern: walk `shape.iter_slots()` first (insertion order), then walk `properties.iter()` for non-shape-eligible entries.

2. **Migrate-on-access**: sites that mutate `.properties` directly in ways the shape mechanism cannot represent (e.g., installing accessor descriptors via `properties.insert(PropertyDescriptor { getter: Some(...), ... })`). Pattern: call `migrate_to_dictionary()` before the mutation.

3. **Migrate-on-construct**: container objects whose role is to BE a dictionary (Map internal storage, Set internal storage, listener lists). These should never carry shape. Pattern: allocate via `Object::new_ordinary()` then immediately call `migrate_to_dictionary()`, OR (after CMig-EXT 1) via a new `Object::new_dictionary()` constructor.

When all consumer families are migrated under one of the three patterns, the closure round flips `Object::new_ordinary()` to start Shaped → modal user-code `{}` literals enroll → integration tier of the shape substrate becomes live.

### I.1 Bounded first-cut telos

The first-cut closure criterion:
- (i) Every direct `.properties` site in `pilots/rusty-js-runtime/derived/src/` has been classified into one of the three patterns.
- (ii) Every site has been migrated to the chosen pattern under green diff-prod 42/42 + green test262-sample (within ±0.5pp of 77.6%).
- (iii) `Object::new_ordinary()` flipped to `shape: Some(Shape::root())`.
- (iv) The post-flip diff-prod + test262-sample readings hold within the same gates.
- (v) `Object::shape_ptr_and_slot_for` returns Some on at least 80% of `{}`-literal-allocated Objects in a representative workload (the "Pred-shape.4 first integration measurement").

## II. Apparatus

The sub-workstream operates at the **consumer-tier of the shape substrate**: it does not introduce new types or APIs, it migrates existing call sites to be shape-aware. Per Doc 729 §A8.13, this is the closure round to the parent Shape-EXT 4 infrastructure round.

Per Doc 730 §XII–§XVI, each consumer-family migration operates under the bidirectional engine-diff oracle. Specifically: diff-prod 42/42 is the green-gate at each rung; test262-sample 77.6% is the second gate. Family migrations that flip a diff-prod fixture to FAIL are rolled back and re-categorized per Doc 730 §XVI (Case-1 cruftless-violates-spec, Case-2 spec-divergence-via-Bun-laxness, Case-3 both-diverge, Case-4 implementation-freedom).

## III. Methodology

Each CMig-EXT is a family migration. The ordering is dependency-driven:

1. **CMig-EXT 0 (this round)** — workstream founding + survey. Output: `docs/consumer-site-survey.md` cataloging the ~41 sites by family.
2. **CMig-EXT 1** — `Object::new_dictionary()` constructor. New Object factory that starts in Dictionary form for container-role allocations (Map storage, Set storage, listener lists). Used by family 3 (migrate-on-construct).
3. **CMig-EXT 2** — Family C migration: direct `.properties.insert` accessor installs migrate first. Touches ~15-20 sites; mechanical.
4. **CMig-EXT 3** — Family A migration: Map/Set internal storage. Either migrate-on-construct via `new_dictionary` (preferred) or shape-iterate at the storage iteration sites.
5. **CMig-EXT 4** — Family B migration: enumeration helpers (ordinary_own_enumerable_string_keys is already partly done; cover the remaining enumerable_own_keys / Object.values / Object.entries / Object.getOwnPropertyNames variants).
6. **CMig-EXT 5** — Family D migration: descriptor introspection (Object.getOwnPropertyDescriptor, Object.freeze, Object.isFrozen). These read `o.properties.values()` to inspect descriptor attrs; shape entries are user-default, so introspection can synthesize a default descriptor for shape-stored names.
7. **CMig-EXT 6** — Family E migration: module namespace enumeration.
8. **CMig-EXT 7** — Family F migration: residual direct-array-index sites in interp.rs.
9. **CMig-EXT 8** — **Enrollment flip**. `Object::new_ordinary()` defaults to `shape: Some(Shape::root())`. Diff-prod + test262-sample gates active. If green, enrollment is on; if red, localize the surviving consumer site and add a CMig-EXT 8.bis.
10. **CMig-EXT 9** — Pred-shape.4 first integration measurement (the 80% target from §I.1.v).

Each CMig-EXT lands as one substrate move per Doc 729 §A8.13. Sub-rounds that grow multi-step (e.g., if CMig-EXT 3 Map/Set discovers iterator-protocol subtleties) spawn their own nested locales per Doc 737 §IV.

## IV. Carve-outs and bounded scope

- **No new corpus articulations.** This sub-workstream is substrate-tier; no corpus doc lands from it. If a structural recognition surfaces that warrants articulation (e.g., a new pattern emerges from the consumer-site fan-out), it goes through the parent's trajectory, not this nested one.
- **No changes to Shape API.** The Shape struct stays as Shape-EXT 3 landed it. If the consumer migration surfaces a need for a new Shape API method, it goes back to the parent's Shape-EXT-N for that addition before continuing here.
- **No JIT integration.** Pilot LeJIT-Σ scaffolds against `Object::shape_ptr_and_slot_for` independently; CMig-EXT 8's enrollment flip is what makes the API start returning Some.
- **No performance optimization.** Migration is correctness-only; per-op-cost measurement (Pred-shape.1) waits for CMig-EXT 9+.

## V. Standing artefacts

- `pilots/rusty-js-shapes/consumer-migration/seed.md` (this file).
- `pilots/rusty-js-shapes/consumer-migration/trajectory.md` — per-CMig-EXT log.
- `pilots/rusty-js-shapes/consumer-migration/docs/consumer-site-survey.md` — CMig-EXT 0 output.

## VI. Resume protocol

Read [parent seed](../seed.md), [parent trajectory's Shape-EXT 4 entry](../trajectory.md), then this seed, then trajectory.md. The next substrate move is CMig-EXT 1 (`Object::new_dictionary()` constructor) when CMig-EXT 0's survey lands.

Pin-Art tag prefix: `Ω.5.P04.E0.cmig-*` per `host/tools/tag-grammar.md`.

## VII. Composition with parent

Per parent seed §VII, Shape-EXT 5 is the closure round to the substrate-introduction round Shape-EXT 4. The keeper's "Continue + set up seeds at every fractal locale that requires it" (2026-05-23) made this nested locale's spawn explicit per Doc 737 §IV — the consumer-migration sub-workstream has multi-rung shape (one rung per consumer family), so it earns its own coordinate.

When this sub-workstream closes (CMig-EXT 9), the parent's Shape-EXT 5 closes simultaneously — the parent's trajectory records the closure as one row pointing to this nested locale's final state. Per Doc 733 §III composition relations: child seed cites parent (this §VII); child trajectory's terminal moves close parent's open-scope entries (parent's Shape-EXT 4 open-scope item 1 closes when CMig-EXT 9 lands).

---

*Doc 581 standing instrument: this seed is the sub-workstream's stable kernel. Changes to telos / apparatus / carve-outs land here; per-family migrations land in trajectory.md.*
