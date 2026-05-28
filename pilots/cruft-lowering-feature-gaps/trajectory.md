# cruft-lowering-feature-gaps — Trajectory

## CLFG-EXT 0 — parent locale founded and current reason partition captured (2026-05-28)

**Trigger**: keeper selected the lowering-feature-gap candidate after the
top-level arc survey and directed the work to proceed under the apparatus
methodology.

**Orientation loaded**:

- `AGENTS.md`
- `apparatus/docs/repository-apparatus.md`
- `apparatus/docs/predictive-ruleset.md`
- `apparatus/docs/standing-rule-13-prospective-application.md`
- `apparatus/docs/agent-feedback-schema.md`
- `apparatus/docs/arc-as-coordinate.md`
- `apparatus/locales/manifest.json`
- `apparatus/locales/CANDIDATES.md`
- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`
- `pilots/apparatus/locale-positioning-audit/findings/repartition-audit-algorithm.md`

**Matrix input**:

`pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-27-161641/interpreted.jsonl`

The file contains seven malformed JSONL rows with embedded control characters,
so the founding aggregation used a tolerant line-by-line JSON parser and
ignored malformed rows. The selector was:

```text
projection == "availability/missing-lowering-feature"
  OR reason starts with "compile:"
```

**Current partition**:

| Count | Reason |
|---:|---|
| 68 | `compile: super reference outside of a class` |
| 18 | `compile: bare \`super\` reference is only valid as \`super(...)\` or \`super.method(...)\`` |
| 8 | `compile: super reference in a class with no \`extends\` clause` |
| 12 | `compile: for-in with destructure head not yet supported` |
| 4 | `compile: update on non-identifier non-member target not yet supported` |
| 2 | `compile: super(...) outside of a class` |
| 1 | `compile: complex assignment target not yet supported` |

**Founding decision**:

This is an arc-shaped parent locale, not an immediate substrate-edit locale.
`super` dominates the current coordinate, but the for-in, update-target, and
complex-assignment tails are separate lowering mechanisms. The parent locale's
job is to run Rule-23 baseline inspection, then spawn or redirect the first
coherent child.

**Artifacts added**:

- `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/arc.md`
- `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/log.md`
- `pilots/cruft-lowering-feature-gaps/seed.md`
- `pilots/cruft-lowering-feature-gaps/trajectory.md`
- `pilots/cruft-lowering-feature-gaps/exemplars/exemplars.txt`
- `pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh`

**Next**: run `pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh`
and use the actual current failures to choose either a `super` child locale or
a redirect for rows that are parser/early-error residue rather than true
bytecode lowering.

## CLFG-EXT 1 — Rule-23 baseline confirms compiler-residue shape and first child (2026-05-28)

**Probe**:

```sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh
```

**Result**:

```text
CLFG exemplars: PASS=0 FAIL=32 SKIP=0 NOJSON=0 / 32
```

**Failure distribution**:

| Exemplars | Reason |
|---:|---|
| 10 | `compile: super reference outside of a class` |
| 6 | `compile: bare \`super\` reference is only valid as \`super(...)\` or \`super.method(...)\`` |
| 4 | `compile: super reference in a class with no \`extends\` clause` |
| 2 | `compile: super(...) outside of a class` |
| 6 | `compile: for-in with destructure head not yet supported` |
| 3 | `compile: update on non-identifier non-member target not yet supported` |
| 1 | `compile: complex assignment target not yet supported` |

**Read**:

The Rule-23 baseline confirms the categorizer's projection is not a runner
blur: all exemplars still fail before execution with compiler diagnostics.
The current coordinate should split. The first child locale should own
`super` reference/call/member lowering plus context-frame propagation, because
it covers 96/113 matrix rows and 22/32 exemplars. For-in destructuring heads,
invalid update targets, and complex assignment targets remain separate tails
to triage after the `super` child is either spawned or redirected.

**Follow-on**: collision check found no active exact owner, so
`super-reference-lowering/` was spawned as a nested child locale. The next move
belongs there: SRL-EXT 1 child baseline classification.

## CLFG-EXT 2 — child super-lowering progress checkpoint (2026-05-28)

`super-reference-lowering/` has now closed two low-collision subclusters:

- SRL-EXT 2: object-literal HomeObject `super` for computed method/accessor
  rows, `+3`.
- SRL-EXT 3: object-method `super[key]` PutValue base/key ordering for
  compound assignment and update rows, `+2`.

Child suite checkpoint:

```text
CLFG exemplars: PASS=5 FAIL=17 SKIP=0 NOJSON=0 / 22
```

Remaining `super` rows partition into direct-eval context capture,
delete/bare-super routing, base-class no-extends runtime behavior, and
derived-constructor direct-eval `super()` capture. Direct eval should stay
deferred behind the active eval-environment arc.
