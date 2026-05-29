---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 33989d0df08c5569b5b2208ee17487a89d2a41e4
target_branch: main
summary: GCS-EXT 2a - add owned FrameSnapshot continuation substrate
risk_class: substrate
gates_pre:
  test262_full: 67.6 (not re-measured this rung)
  test262_sample: post-EPSUA measurement external to this rung
  pass_gain: none expected
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  behavior_change: none
---

## Substrate Moves

Single GCS-EXT 2a prerequisite rung in `pilots/rusty-js-runtime/derived/src/interp.rs` and the `generator-coroutine-suspension` locale trajectory.

- **M** = generator continuation persistence boundary, before any observable generator lifecycle rewrite.
- **T** = introduce an owned continuation payload for `Frame<'a>` so a future heap `GeneratorObject` can hold resumable interpreter state without borrowing bytecode or function metadata.
- **I** = `FrameSnapshot`, `FrameSnapshot::from_frame`, `impl From<&FrameSnapshot> for Frame<'_>`, `TryFrame: Clone`, and GCS trajectory finding GCS.2.
- **R** = unblocks the later yield-boundary rung by separating semantic continuation state from borrowed interpreter execution state.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 53 passed, 1 ignored.
- No behavior gate is expected to move; the new type is not wired into generator execution.

## Risk Assessment

The rung is dormant substrate. It clones mutable execution vectors on restore and preserves binding identity by cloning `Rc<RefCell<Value>>` handles for local cells, `this_cell`, and upvalues. IC, back-edge, and OSR caches are intentionally cold on restore because they are optimization state rather than JS-observable continuation state.

The next GCS-EXT 2b implementation must connect the actual yield boundary to `FrameSnapshot::from_frame` while retaining function-proto identity for diagnostics and metadata.

## Composes-With

- `generator-coroutine-suspension` locale.
- `async-generator-and-for-await-lowering`, which depends on resumable generator continuation state.
- Future generator lifecycle wiring that replaces the current eager-collected `__gen_*` path.

**APPROVED for push** per Helmsman GCS-EXT 2a same-turn directive.
