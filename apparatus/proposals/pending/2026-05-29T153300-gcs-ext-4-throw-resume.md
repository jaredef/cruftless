---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 3c5f6908ff23369f1f1026e1eb5979caff783a7d
target_branch: main
summary: GCS-EXT 4 - resume generator throw through catch handlers
risk_class: substrate
gates_pre:
  forof_generator_slice_ext3: 46 PASS / 23 FAIL / 0 SKIP from 69 baseline FAIL rows
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  forof_generator_slice: 46 PASS / 23 FAIL / 0 SKIP from same 69 rows
---

## Substrate Moves

GCS-EXT 4 wires `Generator.prototype.throw(value)` into the saved continuation for SuspendedYield generators.

- **M** = throw-resume through restored generator frame.
- **T** = inject the thrown value at the suspended yield-expression site by entering the saved catch handler.
- **I** = `inject_throw_into_frame`, `generator_throw_scaffold` SuspendedYield resume path, and focused runtime tests.
- **R** = in-generator `try/catch` can intercept `gen.throw(value)`; uncaught throws propagate to the caller and complete the generator.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 59 passed, 1 ignored.
- Focused GCS tests PASS: 6 passed.
- CLI smoke PASS: `function* g(){ try { yield 1; } catch (e) { yield e + '!'; } }`, `it.throw('oops')` returns `oops! false`; uncaught `throw('boom')` propagates and subsequent `next()` is done.
- Post-EXT 3 for-of/generator slice: 46 PASS / 23 FAIL / 0 SKIP from 69 rows, +0 PASS over EXT 3. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext4-forof-generators-20260529T153213Z/summary.json`.

## Risk Assessment

The change deliberately reuses the interpreter's existing `TryFrame` catch-entry semantics. SuspendedStart and Completed generator throws still complete/throw to the caller. `return(value)` and `yield*` delegation remain out of scope.

## Composes-With

- `generator-coroutine-suspension` locale.
- GCS-EXT 5 `return(value)` early termination.
- GCS-EXT 6 `yield*` delegation and IteratorClose forwarding.

**APPROVED for push** per Helmsman GCS-EXT 4 same-turn directive.
