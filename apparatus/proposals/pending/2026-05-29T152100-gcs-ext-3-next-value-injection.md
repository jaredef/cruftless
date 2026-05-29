---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 3d1e5f1edf100389b5900f1c700ae9c8ac3701de
target_branch: main
summary: GCS-EXT 3 - inject Generator.prototype.next(value) into suspended yield
risk_class: substrate
gates_pre:
  forof_generator_slice_ext2c: 34 PASS / 35 FAIL / 0 SKIP from 69 baseline FAIL rows
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  forof_generator_slice: 46 PASS / 23 FAIL / 0 SKIP from same 69 rows
---

## Substrate Moves

GCS-EXT 3 wires `Generator.prototype.next(value)` sent-value delivery into the saved continuation from EXT 2c.

- **M** = sent-value injection for suspended generator resume.
- **T** = distinguish `SuspendedStart` from `SuspendedYield`; only resumed yields receive the `next(value)` argument.
- **I** = `generator_next_scaffold` mutates the saved `FrameSnapshot` operand-stack top before frame restore.
- **R** = the yield expression evaluates to the value passed by the next resume call.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 57 passed, 1 ignored.
- Focused GCS tests PASS: 4 passed.
- CLI smoke PASS: `function* g(){ let x = yield 1; return x + 1; }`, `it.next(7)` returns `1 false`, `it.next(42)` returns `43 true`.
- Post-EXT 2c for-of/generator slice: 46 PASS / 23 FAIL / 0 SKIP from 69 rows, +12 PASS over EXT 2c. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext3-forof-generators-20260529T152014Z/summary.json`.

## Risk Assessment

The change uses the existing `Op::Yield` resume placeholder as the injection slot and leaves first `.next(value)` from `SuspendedStart` ignored. `throw`, `return`, and `yield*` remain out of scope and are still represented by terminal scaffolds or the legacy path.

## Composes-With

- `generator-coroutine-suspension` locale.
- GCS-EXT 4 `throw(err)` resume-with-exception.
- GCS-EXT 5 `return(value)` early termination.
- GCS-EXT 6 `yield*` delegation and IteratorClose forwarding.

**APPROVED for push** per Helmsman GCS-EXT 3 same-turn directive.
