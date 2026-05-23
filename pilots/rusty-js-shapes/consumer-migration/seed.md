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

---

## VIII. Chapter close (2026-05-23 — post CMig-EXT 17.bis)

The consumer-migration sub-workstream's chapter is closed at engagement-wide canonical correctness gate. Cumulative substrate landings:

| round | substrate | category |
|---|---|---|
| CMig-EXT 0 | sub-workstream founding | apparatus |
| CMig-EXT 1-4 | factory + Family A + Family B + Family C migrations | substrate-introduction |
| CMig-EXT 5/5.bis | Family D mutating + enrollment flip behind env flag | substrate |
| CMig-EXT 8 | env-flag enrollment flip (CRUFTLESS_SHAPE_ENROLL=1) | flip |
| CMig-EXT 9/10 | close 5 residuals + default-on flip | flip |
| CMig-EXT 11 | test262 regression caught; default-on REVERTED | (P2.c) recovery |
| CMig-EXT 12 | Object.create shape-aware (91% of test262 regression closed) | substrate fix |
| CMig-EXT 13/14 | Object.getOwnPropertyDescriptor shape-aware + DEFAULT-ON FLIP (second attempt, held) | substrate fix + flip |
| CMig-EXT 15 | object-spread regression close (out-of-band catch) | substrate fix |
| CMig-EXT 16 | property-bypass audit (~40 sites enumerated; 5 hypothesis NEEDS-FIX) | discipline |
| CMig-EXT 16.bis | JSON.stringify + Proxy ownKeys shape-aware | substrate fix |
| CMig-EXT 17 | canonical 2000-fixture fuzz harness (engagement-wide standing instrument) | infrastructure |
| CMig-EXT 17.bis | NEEDS-VERIFY follow-up: GC Trace + module-export shape-aware | substrate fix |

**Engagement-tier consequences captured at close**:
- Shape substrate default-on at engagement-tier (`CRUFTLESS_SHAPE_ENROLL` defaults to 1)
- 12+ consumer sites migrated to shape-aware iteration via the canonical pattern (shape-iter chain then properties-iter)
- 1 canonical fuzz harness landed as engagement-wide standing instrument (per Findings doc IV.4)
- 1 critical dormant GC use-after-free correctness bug fixed prospectively (Trace impl for Object)
- 1 audit-precision discipline lesson formalized (Finding IV.3) + refined (CMig-EXT 17.bis: NEEDS-FIX-pending-verification framing)
- (P2.c) regression rate at canonical scope: 0 across 8 configurations × 2000 fixtures × 8 patterns

**Forward-looking framework instruments seeded by this pilot**:
- `pilots/rusty-js-shapes/consumer-migration/fixtures/fuzz-canonical.mjs` — STANDING canonical fuzz; future default-on flips at any pilot run this per Finding rule 10
- `pilots/rusty-js-shapes/consumer-migration/docs/property-bypass-audit.md` — discipline reference for future property-iteration audits
- The shape-iter-chain-then-properties-iter pattern — standing engineering pattern for any future user-observable iteration site

**Composition with prior corpus work at chapter close**:
- Doc 729 §A8.13 substrate-amortization-cascade: this pilot's per-pilot cascade (shape → STUB → TB) empirically observed.
- Doc 731 §VII R1-R8: preserved by construction throughout.
- Doc 735 §X.h.b (P2) categorization: pilot reached (P2.a) at canonical scope.
- Doc 735 §X.h.c three-probe-levels: discipline applied at every default-on flip; canonical fuzz instrument standing.
- Doc 737 §IV nested locale: this pilot's existence validated as the apparatus discipline the keeper directed.
- Doc 739 cascade-revival pattern: this pilot's instruments (canonical fuzz) are the standing apparatus that cross-pilot cascade-revivals depend on for correctness verification.

**Post-chapter state**: subsequent rounds at this locale (if any) are post-first-cut maintenance, not load-bearing for the chapter close. Candidates: canonical-fuzz extension (add Trace + module-export patterns); Finding IV.3 refinement codification; new consumer sites discovered by future audits or out-of-band reports.

---

*Doc 581 standing instrument continues. The sub-workstream's chapter is closed; the standing instruments persist; the engagement's framework for future shape-related work is anchored.*
