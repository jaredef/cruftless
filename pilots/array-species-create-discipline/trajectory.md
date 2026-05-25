# array-species-create-discipline — Trajectory

## ASCD-EXT 0 — workstream founding (2026-05-25)

**Trigger**: keeper directive "Do c and close this chapter, then spawn the sub locale". After EPSUA-EXT 4 closed the shared-upstream arc with Finding EPSUA.6 refinement (per-reason-pattern segmentation as the correct projection unit), this locale carries the largest CLEAN sub-cluster of the former constraint #1: ArraySpeciesCreate (~23 tests).

**Pre-spawn Rule 11 5-axis check**:
- (A1) component A/B: the species-create surface is well-named; spec §7.3.21 is small and self-contained.
- (A2) op-set coverage: Array.prototype methods returning new array (concat, filter, map, slice, splice, +copyWithin/fill arguably).
- (A3) value-domain: this.constructor (any Value); Symbol.species (any Value); IsConstructor check.
- (A4) locals-marshaling: N/A (runtime substrate).
- (A5) emission-shape: N/A.
- (A6 EPSUA-extended): spec sections enumerated (§7.3.21 ArraySpeciesCreate; §7.3.23 ArrayCreate; §10.4.2 Array exotic; §23.1.3.* prototype methods).

**Pre-scoping probe** (per Finding EPSUA.6): species-related TypeError-not-thrown tests = 23 in current sample (vs prospective doc's whole-pattern 226).

**Four Pred-ascd.* + discipline falsifier** (see seed §I.3).

**Founding artefacts**: seed.md + this trajectory.md + scaffolded dir. ASCD-EXT 1 (implementation + exemplar + close) next.

### Status

EPSUA-EXT 4 closed; ASCD-EXT 0 founded. Implementation pending keeper authorization.

## ASCD-EXT 1 — IsConstructor refinement (2026-05-25)

**Edit** (~10 LOC):
- `interp.rs::array_species_create`: replace loose `is_fn` check (any Function|Closure|BoundFunction) with strict IsConstructor per §10.1.14 — Function variant gated by `fi.is_constructor` flag (built-in non-constructor functions like parseInt have false), Closure/BoundFunction default true.

**Doc 740 substrate-introduction-prefix avoided**: initial attempt wired slice/splice through `array_species_create` too. This regressed `slice/target-array-with-non-writable-property.js` + `splice/target-array-with-non-writable-property.js` because the per-element write must use CreateDataPropertyOrThrow (which overrides descriptor flags) — cruft's `object_set` respects writable. Per Doc 740 §IV.2 + Finding T262C.5 default discipline, reverted slice/splice species-wiring to a carve-out. Sibling sub-locale needed for the per-element define path + species wiring together.

**Verification**:
- species TypeError fixtures: 23 total. PASS 0 → **3** (the concat-create-species-non-ctor variants).
- 20 remaining all need either (a) species + per-element-define together (slice/splice/map/filter target-array-non-extensible / non-configurable-property cases, ~14) or (b) IsConstructor at a different code path (flatMap/this-value-ctor, 2) or (c) defineProperty on non-extensible/non-configurable in concat too (~4).
- Regression check across concat/filter/map/slice/splice (540 previously-passing): **0 regressed**.

### Findings

**Finding ASCD.1**: the IsConstructor refinement was a single tight fix per Finding EPSUA.7 expectation. Verified per-sub-cluster: 3 of 6 (one per concat-create-species-non-ctor variant — concat being the only Array.prototype.* method that already used array_species_create cleanly).

**Finding ASCD.2 (Doc 740 substrate-introduction-prefix saved by carve-out)**: wiring slice/splice through species exposes a downstream substrate gap (per-element CreateDataPropertyOrThrow vs plain object_set). Per Doc 740 §IV.2: don't land the prefix — close both layers together as a future sub-locale, OR keep the IsConstructor-only fix at the already-wired call sites. Chose the latter for ASCD-EXT 1.

**Finding ASCD.3 (sub-cluster decomposition refinement)**: the 23 species TypeError fixtures decompose into 3 sub-sub-clusters by upstream cause:
- IsConstructor at species check (~6 across concat/slice/splice etc.; closes via array_species_create IsConstructor refinement once each method wires species)
- CreateDataPropertyOrThrow vs object_set on non-extensible target (~14; needs per-element-define path)
- Sundry other (~3)

Even within "species-related" reason, three distinct upstream causes. Finding EPSUA.6 (per-reason-pattern segmentation) generalizes one level deeper: per-sub-spec-section within a sub-cluster.

**Status**: CLOSED at ASCD-EXT 1.
