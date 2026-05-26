# Test262 Pin-Art Matrix → Legible Decision Basis from ~23k Failures

## Induced property

A full-suite test262 run with ~23,520 FAIL records **compresses to ~246 distinct Pin-Art coordinates** that name where in the engine's DAG each failure class sits. The induced property is the matrix as a **legible decision basis**: a substrate worker can read the matrix top, partition by availability + cut-kind per the debugging heuristics §III, and select the next substrate move from a partition-comparable row set rather than from raw counts.

Anchor: `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-25-165734-p2/matrix.md` (current measurement: 246 coordinates, 12 resolver instances, 9 rungs, 9 axes, 8 availability classes, 7 cut kinds, 785 surfaces). Apparatus articulation: `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`. Reading discipline: `pilots/apparatus/test262-categorize/full-suite/debugging-heuristics.md`.

## The accumulation

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) raw PASS/FAIL counts per test path | — | "X% conformant" with no diagnostic structure |
| 1 | **Per-result resolver-instance projection** — each FAIL is tagged with the owning resolver (parser, lowering, runtime/spec-builtins, runtime/buffer-typed-array, etc.) | code-ownership marginal | property: "the engineering area for a failure is named" |
| 2 | **Per-result engine-rung projection** (per Doc 717) — E1/syntactic-grammar through E4/host-hook | tier-depth marginal | property: "the depth at which the failure surfaces is named" |
| 3 | **Per-result constraint-axis projection** (per Doc 729) — which strategic spectrum the failure belongs to | strategy marginal | property: "the strategic class of work the failure implies is named" |
| 4 | **Per-result availability projection** — `available-surface`, `absent-chapter`, `partial`, `policy-deferred`, `runner-deferred` | work-type marginal | property: "whether this is bug work / chapter work / policy work / apparatus work is named" |
| 5 | **Per-result cut-kind projection** (per Doc 716) — `widening/value-semantics`, `successor/semantic-refinement`, `K1/throw-on-use`, `version-or-policy-cut`, `measurement-residue` | substrate-move-class marginal | property: "the SHAPE of the next substrate move is named" |
| 6 | **Per-result projection-class** + **failure-mode** + **abstract-op candidate** | local-symptom marginal + spec-anchor marginal | property: "local symptom and spec path to read are named" |
| 7 | **Pin-Art coordinate** as the 4-tuple `resolver :: rung :: projection :: failure_mode` | composite coordinate | property: "FAIL records compress to ~246 distinct coordinates" |
| 8 | **Reading order discipline** (per heuristics §VI) — availability → cut-kind → engine-rung → axis → resolver → pin → surface → abstract-op marginal | partition-before-rank | property: "the largest chapter or harness row does not dominate next-move selection by raw count" |
| 9 | **Stratified exemplar suites per coordinate** (per the top-10 spawn-batch convention) — 100 fixtures per coordinate stratified by surface family for fast iteration | iteration instrument | property: "substrate moves can be measured against a coordinate-shaped instrument without re-running the full suite" |

The named composition (1+2+3+4+5+6+7+8+9) is the **Pin-Art matrix-as-decision-basis**. The induced property is that ~23k FAIL records become operationally legible — partition them by §III, rank within partitions, select by heuristics §VIII Debugging Rule, instrument with stratified exemplars.

Removing constraint 1+2+3 (no resolver/rung/axis projection) means failures are paths only; no diagnostic structure.
Removing constraint 4 (no availability marginal) means absent-chapter rows ravage the matrix top by raw count (Temporal at 4,152 dominates everything else); decision-making degrades to chasing counts.
Removing constraint 7 (no Pin-Art coordinate compression) means the matrix has 23k rows; not legible.
Removing constraint 8 (no reading-order discipline) means workers select by raw count (heuristics §III explicitly warns against this).
Removing constraint 9 (no stratified exemplars) means substrate work must re-run the full suite to measure; iteration cost dominates.

## Tag on the DAG

The matrix is itself the **measurement-apparatus tier coordinate**:

```
apparatus/measurement ::
  E0/full-suite-projection ::
  cut/coordinate-compression ::
  property/legible-decision-basis
```

The matrix is reflexive: the FAIL records it categorizes include `availability=runner-deferred` records (the apparatus's own measurement residue), and §IX explicitly names apparatus refinement as part of the work. The matrix improves as the categorizer improves; the categorizer improves as substrate work surfaces new coordinate distinctions.

Today's BBND closure (BBND-EXT 1+2, +95 tests) was driven by this matrix's PEER row (rank #5, 809 fails) + row-coherence inspection per §V. The full-suite categorize hasn't been re-run post-BBND, but the prediction is that the PEER coordinate's count drops by ~95 at next measurement (true cluster residual ~714 from current 809).

## Composes-with

- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md` — primary articulation.
- `pilots/apparatus/test262-categorize/full-suite/debugging-heuristics.md` — reading discipline.
- [`docs/fca-instances/diff-prod-cross-runtime-engine-diff-oracle.md`](diff-prod-cross-runtime-engine-diff-oracle.md) — behavioral-parity dual.
- Doc 715 — Consumer-Substrate Dependency Graph as the load-bearing object.
- Doc 716 — Stubs as Named Cuts (cut-kind taxonomy).
- Doc 717 — Apparatus above the engine boundary (rung taxonomy).
- Doc 720 — rusty-bun runtime as DAG of pipelines.

## Falsification

A substrate move informed by the matrix that produces zero measurable shift in the matrix at re-categorize would falsify the matrix's predictive value at that coordinate. BBND's predicted +95 cluster shift on next full-suite re-run is the closest near-term Fal test. If the next full-suite categorize shows PEER residual still ≥800, the matrix's projection at that pin is wrong (likely because the 100-exemplar sample under-represented the cluster's true mechanism distribution per the apparatus-gap notes in §IX).
