---
arc: 2026-05-28-array-exotic-substrate
trigger: Plan agent's back-fit analysis 2026-05-28 (keeper directive Telegram 10158); empirical: EPSUA arc residual + ASCD/ACDPD spawns 2026-05-25
opened: 2026-05-28
closed: IN PROGRESS
close_condition: ECMA §10.4.2 (Array exotic objects) + §23.1 (Array) + §23.2 (TypedArray) length/species/data-property paths spec-conformant across the enrolled locales' exemplar suites; cross-locale findings recorded for the indexed-exotic discipline.
---

# Array Exotic Substrate Arc

## Trigger

Plan agent's back-fit analysis (2026-05-28, per keeper Telegram 10158) identified twelve top-level locales sharing the indexed-exotic-object mouth-terminus pair: Array / TypedArray ctor + own-property + length semantics, terminus = ECMA §10.4.2 + §23.1 + §23.2 conformant indexed access + length + species behavior. Empirically anchored in the EPSUA arc's residual + the 2026-05-25 ASCD/ACDPD spawns + the TAMM-EXT closures (2026-05-27, ten rungs, +79 cluster yield).

## Telos

Subsume the twelve indexed-exotic locales under one arc with explicit (M, T, I, R) per Doc 744. The arc-tier mouth is "Array/TypedArray exotic-object receiver plus user args"; the arc-tier terminus is "spec-conformant own-property + length + species + indexed-access behavior per §10.4.2 + §23.1 + §23.2". The arc-tier interior is the chain ArraySetLength → ArraySpeciesCreate → IntegerIndexedExotic [[GetOwnProperty]] → IndexedDelete plus the per-method dispatch surface. The arc-tier relations: lattice with the iterator-protocol arc (shared LengthOfArrayLike + species); DAG ↑ runtime intrinsics (length setter, type coercion); alphabet-exchange ↑ bytecode emit for typed-array element access.

Per the substrate-shaped-work discipline (CLAUDE.md §Substrate-shaped-work discipline), this arc's spawn discharges Phase 1 for every enrolled locale's pre-existing trajectory + sets the arc-tier coordinate for subsequent per-locale Phase 1 emissions.

## Sub-locale roster

| Locale | Role in arc | Status pre-arc | Direct yield |
|---|---|---|---|
| `array-create-data-property-discipline` | own-property creation contract | LANDED | — |
| `array-exotic-virtual-property-discipline` | virtual indexed-property surface | LANDED | — |
| `array-length-setter-truncation` | length setter semantics | LANDED | — |
| `array-literal-elision-length` | elision-aware length computation | LANDED | — |
| `array-search-arg-strict-coerce` | indexOf/lastIndexOf coercion | LANDED | — |
| `array-sort-tostring-dispatch` | sort comparator dispatch | LANDED | — |
| `array-species-create-discipline` | ArraySpeciesCreate per §10.4.2.3 | LANDED | — |
| `length-of-array-like-propagate` | LengthOfArrayLike propagation | LANDED | — |
| `iterable-primitive-tobject` | iterable → ArrayLike conversion | LANDED | — |
| `typed-array-missing-method` (TAMM) | TypedArray.prototype method surface | LANDED at EXT 10 (10 rungs, +79 yield) | major |
| `typed-array-wrong-result` | TA element-access semantics | OPEN | — |
| `typed-array-resizable-buffer-indexed-access` | resizable-buffer indexed access | OPEN | — |

## Cumulative yield

Pre-arc baseline: cumulative yield across the twelve locales has been measured per-locale; aggregated yield rendered as engagement-wide rate movement at test262 sample re-baselines.

Per IR-EXT 35 (2026-05-28 sample re-baseline): engagement-wide rate 77.6% → 84.3% includes substantial contribution from this cluster (TAMM alone delivered ~79 cluster-exemplar PASS gain over 10 rungs).

Future arc-tier measurement: tracked per IR-EXT 37 chapter-fold pattern; report rate-delta + telos-progress-delta per finding IR.34 standing rec.

## Cross-arc relations

- **Lattice with `2026-05-28-iterator-protocol-substrate`**: shared LengthOfArrayLike consumption + shared species-creation at iterator/destructure boundaries. Meet at TypedArray iteration interior.
- **DAG ↑ engine-pillar locales** (`rusty-js-runtime`, `rusty-js-shapes`): array indexed-storage interacts with shape substrate.
- **Alphabet-exchange with `2026-05-28-engine-hot-path-amortization`** (proposed; not yet scaffolded): IHI array-entries locale fits both arcs (lattice-meet).

## Cross-locale findings

To be promoted as the arc operates. Standing entries to be populated per finding IR.34 + IR.36 patterns. Initial sketch:

**Finding AES.1 (pending)**: indexed-exotic discipline composes IntegerIndexedExotic [[GetOwnProperty]] (TAMM-EXT 9, finding TAMM.8) with ArraySpeciesCreate (per §10.4.2.3) such that user descriptor reflection through Object.getOwnPropertyDescriptor on TypedArray indices returns spec-conformant data descriptors. The composition pattern recurs across Array.prototype.slice / map / filter / TypedArray.prototype equivalents.

## Status

IN PROGRESS — scaffolded 2026-05-28 per keeper directive 10158. Arc spawned to subsume already-existing locales; per the Plan agent's recommended scaffold order, this is the first new arc to be back-fit (highest substrate coherence, single ECMA-262 spec section, dense roster, plentiful canonical examples). Cumulative yield + cross-locale findings populated as the arc's per-locale Phase 5 chapter-closes proceed.
