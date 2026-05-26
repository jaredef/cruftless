# parser-early-error-residual — Seed

## Telos

Materialize the engine-DAG coordinate

```
source-to-ast/parser-early-error :: E1/algorithm-step:syntactic-grammar :: parser-form/early-error :: err:SyntaxError
```

This coordinate ranks **#5** in the full-suite Pin-Art matrix
(`pilots/test262-categorize/full-suite/results/test262-full-2026-05-25-165734-p2/matrix.md`),
**809 fails** in the canonical full-suite run. Per the apparatus
articulation (`apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`),
the telos is the explicit DAG coordinate that had to exist for the result
to pass, not the raw count.

## Work shape

**Heuristics §IV classification**: C (Parser/Early-Error) — pure parser tier

Direct successor to today's parser arc (FHAPV/FORA/SBAP/FHLA/FAOF/ALTA/RPDF/ARTC closed 32 of these inside the curated 45-cluster). 809 remain in the full pool — broader parser-early-error surface. Each rung adds ~3-15 tests at ~5-15 LOC.

## Apparatus

- **Exemplar suite**: `exemplars/exemplars.txt` — 100 paths stratified-
  sampled from the 809-fixture pool by surface family (proportional +
  min 1 per family where the family count allows). Fixed seed 0xC0FFEE.
- **Runner**: `exemplars/run-exemplars.sh` — harness wrapper; prints
  aggregate pass/fail + top fails by surface family.
- **Inventory**: `exemplars/pool-size.txt`, `exemplars/family-breakdown.txt`.
- **Baseline**: TBD (run `exemplars/run-exemplars.sh`). Expected near 0/100
  given the coordinate's mass and the cluster's coherence; record once run.

## Methodology

Per heuristics §VIII Debugging Rule, every substrate rung against this
coordinate must satisfy:

- large enough to matter — confirmed by pool size (809 fails)
- coherent across examples — to be verified per rung via family marginal
- comparable within one availability class — yes (single pin, single
  availability)
- owned by one resolver instance or one shared abstract op — to be
  refined; first rung pulls 5+ records per heuristics §V row-coherence
- not measurement residue — confirmed (cut-kind is not measurement-residue)
- measurable by matrix shift after landing — yes (re-running the
  full-suite categorize at landing time will report the cluster's new
  rank + remaining count)

Per heuristics §V, before any substrate edit:

```
rg '"pin":"<pin>"' pilots/test262-categorize/full-suite/results/test262-full-2026-05-25-165734-p2/interpreted.jsonl | head -5
```

Inspect availability + cut_kind + abstract_op + surface + reason on the
first 5; if mechanism is shared, proceed; if not, split before editing.

## Carve-outs

- This locale tracks the **coordinate**, not the surface. Sub-clusters
  with sharply different mechanisms (visible after a few exemplar reads)
  may spawn nested locales per Doc 737 §II multi-rung promotion threshold.
- The 100-exemplar suite is the iteration instrument; the 4,152-pool
  full-cluster yield is the validation horizon (re-categorize after
  closures land).

## Composes-with

- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`
- `apparatus/docs/predictive-ruleset.md` (rules 4, 5, 11, 13, 14, 15
  particularly relevant)
- `pilots/test262-categorize/full-suite/debugging-heuristics.md` (§IV
  work-shape classification this locale enacts)
- `pilots/temporal-availability/` (sibling spawn 2026-05-25 from the same
  top-10 batch)

## Resume protocol

Read `trajectory.md` tail; run `exemplars/run-exemplars.sh` for current
yield; pick the next rung's mechanism from a fresh exemplar-fail family
marginal.
