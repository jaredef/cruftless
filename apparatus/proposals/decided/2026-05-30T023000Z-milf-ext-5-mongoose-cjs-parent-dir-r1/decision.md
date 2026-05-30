---
proposal_slug: 2026-05-30T023000Z-milf-ext-5-mongoose-cjs-parent-dir-r1
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-30T02:30:00Z
covers_commits:
  - b8b0be02527f7d56b8be6e1f239cf451b424ced8
---

## Findings

Approved under helmsman directive `77473bb4-7590-4ccb-986c-cfadaecb1bd6`.

The substrate commit verifies that the directed parent-directory CJS failure is
already closed on current main by R2's dot-directory resolution, then closes the
actual current `mongoose` blocker surfaced by the fresh build:

1. Reproduces the stale-release `module not found: '..'` failure, then rebuilds current main and verifies the minimal nested `require("..")` fixture passes.
2. Confirms `mongodb` and `redis` package smokes pass on current main.
3. Identifies the active `mongoose` failure as missing `SharedArrayBuffer.prototype.byteLength` descriptor reflection in `webidl-conversions`.
4. Installs a real `SharedArrayBuffer.prototype.byteLength` accessor descriptor and backs constructed instances with an `ArrayBufferRecord`.
5. Adds a focused runtime regression for SharedArrayBuffer byteLength descriptor visibility.
6. Records the advanced `mongoose` `node:zlib.gunzipSync` blocker as a deferred host intrinsic candidate.

Verification:

1. Focused regression: `cargo test --release -p rusty-js-runtime shared_array_buffer_bytelength_descriptor_is_visible --test run_golden` PASS.
2. Dot-directory regression: `cargo test --release -p rusty-js-runtime module::tests::resolve_module_treats_dot_as_relative_directory` PASS.
3. Build: `cargo build --release --bin cruft -p cruftless` PASS.
4. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 73 passed and 1 ignored.
5. Smoke: `mongodb` PASS, `redis` PASS, `mongoose` advances to `node:zlib.gunzipSync not yet implemented`.

**APPROVED for push.**
