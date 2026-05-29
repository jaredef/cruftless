---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 9706f371c475b187163a713704b281e736f36122
target_branch: main
summary: GCS-EXT 5 - generator return early termination through finally cleanup
risk_class: substrate
gates_pre:
  generator_return_slice_latest_full: 4 PASS / 19 FAIL from 23 paths
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  generator_return_slice: 13 PASS / 10 FAIL from 23 paths
---

## Substrate Moves

GCS-EXT 5 wires `Generator.prototype.return(value)` into the saved SuspendedYield continuation.

- **M** = early return completion for suspended sync generators.
- **T** = preserve a pending return completion across cleanup `yield`s from `try/finally`.
- **I** = `GeneratorObject.pending_return`, `generator_return_scaffold` SuspendedYield resume path, `inject_return_into_frame`, GC tracing for object return payloads, and focused runtime tests.
- **R** = `gen.return(value)` either completes immediately with `{ value, done:true }` or runs a finally cleanup yield first and completes with the requested value on the next resume.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 61 passed, 1 ignored.
- Focused GCS tests PASS: 8 passed.
- `built-ins/GeneratorPrototype/return/*` slice: 13 PASS / 10 FAIL from 23 tests. Baseline from `/home/jaredef/Developer/cruftless-sidecar/results/test262-full-2026-05-28-123833-p2/results.jsonl` was 4 PASS / 19 FAIL, for +9 PASS. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext5-generator-return-20260529T154741Z/summary.json`.

## Risk Assessment

The implementation deliberately stays within the existing try-stack substrate. Residual failures remain where the current `TryFrame` does not distinguish catch from finally, and where finally-return override semantics need a richer abrupt-completion record than a pending value slot.

## Composes-With

- `generator-coroutine-suspension` locale.
- GCS-EXT 6 `yield*` delegation and IteratorClose forwarding.
- A follow-on try-stack completion-kind rung for nested catch/finally discrimination.

**APPROVED for push** per Helmsman GCS-EXT 5 same-turn directive.
