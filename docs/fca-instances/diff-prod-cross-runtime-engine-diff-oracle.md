# diff-prod → Cross-Runtime Engine-Diff Oracle

## Induced property

Every substrate move's effect on runtime semantics is **immediately legible** against an external oracle (bun). Per-fixture byte-identical stdout under cruft vs bun produces a categorizable engine-diff signal: PASS = parity, FAIL = engine divergence categorized into one of four Doc 730 §XVI shapes. The induced property is the **engine-diff oracle**: the substrate worker never lands a change without knowing whether cross-runtime parity moved.

Anchor: `scripts/diff-prod/` (42-fixture suite). Doc 730 §XVI articulates the four-case engine-diff oracle. CLAUDE.md baseline: 42/42.

## The accumulation

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) substrate moves verified only against in-house tests | — | parity only as far as the test suite covers; cross-runtime divergence invisible |
| 1 | **Fixture under cruft** — execute the fixture, capture stdout | per-fixture cruft signal | property: "cruft's output is observable" |
| 2 | **Same fixture under bun** — execute under the oracle runtime | external comparison reference | property: "bun's output is the comparison anchor" |
| 3 | **Byte-identical stdout comparison** — diff cruft vs bun byte-for-byte; PASS iff identical | strict equality semantics | property: "any divergence, however small, registers as FAIL" |
| 4 | **42-fixture coverage spanning consumer surfaces** — JSON, Buffer, Map/Set, async, RegExp, generators, structured-clone, etc. | breadth-coverage gate | property: "regressions in any major consumer surface surface at first run" |
| 5 | **Per-failure four-case categorization** (Doc 730 §XVI) — cruft-correct/bun-wrong; cruft-wrong/bun-correct; both-wrong-different; both-right-different | failure-class discrimination | property: "FAILs are categorized into engineering action class, not just 'broken'" |
| 6 | **Re-verified at every substrate move** — every cruft binary rebuild triggers a diff-prod sweep | per-move gate | property: "a substrate move's effect on cross-runtime parity is part of its landing gate" |

The named composition (1+2+3+4+5+6) is **diff-prod**. The induced property is a cross-runtime engine-diff oracle that surfaces divergences before they ship, categorizes them into engineering classes, and serves as the per-move correctness gate.

Removing constraint 2 (no external runtime) means in-house tests only; cross-runtime divergence is invisible.
Removing constraint 3 (allow normalized comparison) means small encoding differences (number formatting, error message phrasing) are silently ignored; subtle divergence accumulates.
Removing constraint 4 (narrow coverage) means surface-specific regressions surface only when end users hit them.
Removing constraint 5 (no categorization) means FAILs become "broken" without engineering action; substrate workers don't know whether to fix cruft or report to bun.
Removing constraint 6 (skip per-move gate) means regressions accumulate between explicit re-measurements.

## Tag on the DAG

This is an **apparatus-tier measurement coordinate**:

```
apparatus/measurement ::
  E0/cross-runtime-comparison ::
  cut/byte-identity-comparison ::
  property/engine-diff-oracle
```

Test262 measures conformance to ECMA-262 spec; diff-prod measures parity against a concrete runtime (bun). The two triangulate: a substrate move that flips a diff-prod fixture should also flip a count of test262 entries (per CLAUDE.md's "two probes triangulate" articulation). Coverage axis dual: test262 is wide and spec-anchored; diff-prod is narrow and behavior-anchored.

## Composes-with

- Doc 730 §XVI — four-case engine-diff oracle articulation.
- `scripts/diff-prod/run-all.sh` — runner.
- `scripts/diff-prod/fixtures/` — the 42-fixture surface.
- [`docs/fca-instances/pin-art-matrix-legible-decision-basis.md`](pin-art-matrix-legible-decision-basis.md) — the test262 matrix is the conformance-tier dual; diff-prod is the behavioral-parity dual.
- Standing rule 5 (three probes before default-on) — diff-prod is the consumer-route probe.

## Falsification

A substrate move that ships with diff-prod 42/42 but produces a real cross-runtime regression in a consumer surface not in the 42 falsifies the coverage breadth (constraint 4). Empirically: the 42-fixture surface is intentionally curated to span consumer-bearing operations; additions land as new fixtures when a surface surfaces a regression class the existing 42 don't catch.
