---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 47f0509a97d93eada61c90b02a7fe5336c8bd6af
target_branch: main
summary: GCS-EXT 6 - yield-star delegation opcode and suspended delegate state
risk_class: substrate
gates_pre:
  forof_generator_slice: 46 PASS / 23 FAIL from 69 paths
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  forof_generator_slice: 50 PASS / 19 FAIL from 69 paths
---

## Substrate Moves

GCS-EXT 6 replaces the eager `__yield_delegate__` lowering with a suspended generator opcode path.

- **M** = sync `yield*` delegation for generator continuations.
- **T** = resume at the delegation opcode until the inner iterator completes, forwarding `next(value)` into the delegate.
- **I** = `Op::YieldDelegate`, compiler lowering for `UnaryOp::YieldDelegate`, `GeneratorDelegate` state on `GeneratorObject`, GC tracing for delegate iterator/method, and focused runtime tests.
- **R** = `yield* [1,2,3]` yields lazily; `return yield* inner()` completes with the inner generator's return value.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 63 passed, 1 ignored.
- Focused GCS tests PASS: 10 passed.
- For-of/generator slice: 50 PASS / 19 FAIL from the 69-row slice. Baseline from `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext4-forof-generators-20260529T153213Z/results.jsonl` was 46 PASS / 23 FAIL, for +4 PASS. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext6-forof-generators-20260529T160327Z/summary.json`.

## Risk Assessment

The opcode path covers the basic delegate pump, sent-value forwarding, and inner return-value propagation. Residual risk remains in abrupt forwarding through delegate `throw`/`return`, IteratorClose, and destructuring iterator-close interactions; these are recorded as residual follow-up scope in the trajectory.

## Composes-With

- `generator-coroutine-suspension` locale.
- Follow-up GCS residual rungs for delegate abrupt forwarding and IteratorClose.
- EXT 5a/5b return-slice residuals for metadata and completion-kind handling.

**APPROVED for push** per Helmsman GCS-EXT 6 same-turn directive.
