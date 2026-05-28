# cruft-lowering-feature-gaps — Seed

**Locale tag**: `L.cruft-lowering-feature-gaps`

**Status**: FOUNDED 2026-05-28 under
`apparatus/arcs/2026-05-28-lowering-feature-gap-triage/`.

## Telos

Close the current bytecode compiler "not yet supported" residue surfaced by
the Test262 categorizer projection:

```text
projection == availability/missing-lowering-feature
reason starts with compile:
```

The founding matrix source is:

```text
pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-27-161641/interpreted.jsonl
```

At founding, a tolerant JSONL pass finds **113 rows**. The pool is not one
mechanism:

- 96 rows: `super` reference/call/member lowering and context validation.
- 12 rows: for-in destructuring-head lowering.
- 4 rows: invalid update-target lowering/early-error placement.
- 1 row: complex assignment-target lowering.

## Apparatus

- **Arc**: `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/`.
- **Candidate source**: `apparatus/locales/CANDIDATES.md` candidate `(w)`.
- **Methodology source**:
  `pilots/apparatus/locale-positioning-audit/findings/repartition-audit-algorithm.md`.
- **Exemplar suite**: `exemplars/exemplars.txt`, 32 paths stratified across
  every current reason shape.
- **Exemplar runner**: `exemplars/run-exemplars.sh`, using `scripts/env.sh`
  for `T262_ROOT` and `CRUFT_BIN`.
- **Primary substrate site**:
  `pilots/rusty-js-bytecode/derived/src/compiler.rs`.

## Methodology

This is a Rule-23 baseline-first parent locale. The founding move is not a
substrate edit. The first task is to run the exemplar suite and classify actual
failure roots before spawning child locales or landing compiler work.

1. Run the 32-path exemplar suite.
2. Partition failures by reason and actual owning layer:
   - `super` context-frame/lowering residue,
   - for-in destructuring-head lowering,
   - update-target early-error versus bytecode rejection,
   - complex assignment-target lowering,
   - runner/policy residue.
3. If `super` rows remain coherent after baseline, spawn a nested locale for
   `super-reference-lowering` rather than broadening this parent.
4. If for-in destructuring rows overlap
   `for-of-destructuring-assignment-semantics/`, record explicit absorption or
   carve-out before substrate edits.
5. If update-target rows are negative early-error tests, redirect to parser or
   syntax-error projection rather than implementing invalid bytecode targets.

## Carve-outs

- This locale does not own the entire `ast-to-bytecode/language-lowering`
  bucket.
- This locale does not own general class semantics, private fields, or class
  field initialization except where a current compile diagnostic proves the
  missing-lowering-feature coordinate.
- This locale does not own parser acceptance for new syntax forms unless
  baseline proves a row is misclassified at the lowering tier.
- This locale does not own staging/policy rows as substrate unless an
  available-surface exemplar proves the same mechanism.

## First Rungs

1. **CLFG-EXT 0**: founding, arc registration, stratified exemplar suite, and
   manifest refresh.
2. **CLFG-EXT 1**: Rule-23 baseline inspection. Run exemplars, classify by
   current reason shape, and decide first child locale.
3. **CLFG-EXT 2**: likely `super` child spawn, if baseline confirms the
   96-row dominance is a single context-frame/lowering mechanism.

## Composes With

- `apparatus/docs/arc-as-coordinate.md` for arc-level coordination.
- `apparatus/docs/predictive-ruleset.md` Rules 13, 15, 23, 24, 25, and 26.
- `pilots/apparatus/locale-positioning-audit/` repartition discipline.
- Existing language-lowering locales:
  `for-of-destructuring-assignment-semantics/`,
  `async-generator-and-for-await-lowering/`, and
  `finally-abrupt-completion-lowering/`.

## Resume Protocol

Read this seed, then `trajectory.md`, then the arc log. Run:

```sh
pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh
```

Before editing substrate, inspect `git status --short` and avoid colliding with
active bytecode or eval-environment work from other agents.
