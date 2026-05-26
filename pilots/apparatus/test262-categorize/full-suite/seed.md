# test262-categorize/full-suite — Pin-Art Apparatus Seed

**Locale tag**: `L.test262-categorize.full-suite`

**Parent**: `test262-categorize`

**Status**: founded 2026-05-25 as the full-suite interpretation apparatus for the upstream 53,289-path Test262 run.

## I. Telos

Interpret full-suite Test262 results against cruftless's engine DAG.

The parent `test262-categorize` locale proved the sample-scale matrix useful but too coarse when clusters contain heterogeneous causes. This nested locale lifts the apparatus to full-suite scale and changes the unit of interpretation:

```
raw test result
  -> engine surface
  -> resolver instance
  -> engine tier
  -> engine rung
  -> constraint axis
  -> availability / cut kind
  -> projection class
  -> failure mode
  -> abstract-operation candidate
  -> Pin-Art coordinate
```

The goal is not just to count failures. The goal is to make each non-pass result addressable as a candidate substrate coordinate in the ECMAScript parity arc.

## II. Constraints

C1. Consume sidecar full-suite artifacts; do not store raw full results in the repo.

C2. Preserve raw-result caveats. If the runner emits split JSON, no-output records, or malformed fragments, the apparatus records that as a runner/measurement surface rather than silently dropping it.

C3. Classify by engine coordinate, not only by Test262 path. The minimum coordinate is `(resolver instance, engine rung, projection, failure mode)`, with availability, cut kind, constraint axis, and abstract-operation candidate emitted as lattice fields.

C4. Keep interpretation heuristic and falsifiable. Unknowns must remain visible as `uncategorized/*` coordinates.

C5. Produce stable artifacts under `pilots/test262-categorize/full-suite/results/<run-id>/`: `summary.md`, `matrix.md`, and `interpreted.jsonl`.

## III. Falsifiers

Pred-full.1: The interpreter consumes the 53k-path full-suite run and emits all three artifacts in under one minute.

Pred-full.2: The top resolver-instance marginal accounts for a structurally meaningful portion of non-pass records, making the next substrate spectrum selectable without hand-reading thousands of files.

Pred-full.3: Runner/apparatus failures (`NO_OUTPUT`, malformed fragments) are separated from engine failures so they can be fixed as measurement substrate rather than confused with ECMA semantics.

Pred-full.4: At least the top 20 Pin-Art coordinates have inspectable example tests and route to an owning engine tier.

## IV. Apparatus

- Binary: `pilots/test262-categorize/derived/src/bin/full_pinart.rs`
- Cargo target: `cargo run --release -p test262-categorize --bin t262-full-pinart -- <sidecar-results.jsonl>`
- Input: `/home/jaredef/Developer/cruftless-sidecar/results/test262-full-*/results.jsonl`
- Output: `pilots/test262-categorize/full-suite/results/<run-id>/`
- Debugging heuristics: `pilots/test262-categorize/full-suite/debugging-heuristics.md`

## V. Composition

Composes with:

- Parent locale `pilots/test262-categorize/seed.md`
- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`
- Doc 720 runtime DAG
- Doc 729 resolver-instance pattern
- Doc 730 lowering recurrence
- Doc 737 locale-as-coordinate

## VI. Resume Protocol

Read this seed, then `trajectory.md`, then `debugging-heuristics.md`, then the latest `results/<run-id>/summary.md` and `matrix.md`.
