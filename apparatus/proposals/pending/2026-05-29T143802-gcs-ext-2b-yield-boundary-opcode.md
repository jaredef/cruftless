---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 32bfb91659b023edc679a1b91ea8b765e62bcb20
target_branch: main
summary: GCS-EXT 2b - add dual-mode yield-boundary opcode
risk_class: substrate
gates_pre:
  test262_full: 67.6 (not re-measured this rung)
  pass_gain: none expected until lifecycle wiring
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  targeted_exemplar: interp::gcs_tests::yield_opcode_captures_active_generator_frame_snapshot PASS
  eager_generator_smoke: function* g(){ yield 1; yield 2; } returns 1 2 true PASS
---

## Substrate Moves

Single GCS-EXT 2b rung across bytecode and runtime:

- **M** = yield suspension boundary before generator lifecycle replacement.
- **T** = introduce a zero-operand `Op::Yield` instead of modeling plain `yield` as a native helper call.
- **I** = compiler plain-yield lowering to `compile argument; Yield`, runtime dual-mode `Yield` dispatch, `GeneratorObject` continuation/yielded-value slots, active-generator guard, snapshot trace edges, and isolated runtime exemplar.
- **R** = proves `FrameSnapshot::from_frame` can capture the active interpreter frame at a yield point while the legacy eager generator path remains intact.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 54 passed, 1 ignored.
- Targeted exemplar: `interp::gcs_tests::yield_opcode_captures_active_generator_frame_snapshot` PASS.
- Legacy eager generator smoke: `function* g(){ yield 1; yield 2; } const it = g(); console.log(it.next().value, it.next().value, it.next().done);` prints `1 2 true`.

## Risk Assessment

The opcode is dual-mode. Existing generator calls do not install `active_generator_for_yield`, so `Yield` appends to the existing eager yields array and pushes `undefined`, preserving current behavior. The suspension path is reachable only by explicitly installing an active generator object, which the new test does directly.

EXT 2c should wire generator construction/resumption to the active-generator path and replace the eager `call_function` generator branch. It should not need another compiler rewrite for plain `yield`.

## Composes-With

- `generator-coroutine-suspension` locale.
- EXT 2a `FrameSnapshot` substrate.
- EXT 2c generator lifecycle routing.

**APPROVED for push** per Helmsman GCS-EXT 2b same-turn directive.
