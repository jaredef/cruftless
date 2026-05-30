---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - b8b0be02527f7d56b8be6e1f239cf451b424ced8
target_branch: main
summary: MILF-EXT 5 - mongoose parent-dir verification and SharedArrayBuffer descriptor closure
risk_class: substrate
gates_pre:
  stale_release_mongoose_smoke: FAIL module not found '..'
  current_main_mongoose_smoke: FAIL SharedArrayBuffer.prototype.byteLength descriptor missing
gates_post:
  focused_regression: cargo test --release -p rusty-js-runtime shared_array_buffer_bytelength_descriptor_is_visible --test run_golden PASS
  dot_directory_regression: cargo test --release -p rusty-js-runtime module::tests::resolve_module_treats_dot_as_relative_directory PASS
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  package_smoke: mongodb PASS; redis PASS; mongoose advanced to node:zlib.gunzipSync host intrinsic residual
---

## Substrate Moves

This handles helmsman directive `77473bb4-7590-4ccb-986c-cfadaecb1bd6`.

- **M** = the nominal `mongoose` `require("..")` residual was already closed on current main by R2 dot-directory resolution commit `ae0f98b6`; the current blocker was `webidl-conversions` reflecting `SharedArrayBuffer.prototype.byteLength`.
- **T** = add a conservative `SharedArrayBuffer` branch that installs a real `byteLength` accessor descriptor and allocates instances into the existing `array_buffers` backing table.
- **I** = `pilots/rusty-js-runtime/derived/src/intrinsics.rs`, focused `run_golden` regression, MILF trajectory, and deferral ledger entry for the newly surfaced `node:zlib.gunzipSync` residual.
- **R** = `mongodb` and `redis` package smokes pass; `mongoose` advances past both the parent-directory and SharedArrayBuffer descriptor blockers to a separate host zlib intrinsic gap.

## Risk Assessment

The change is constrained to the existing `SharedArrayBuffer` constructor/prototype surface. It does not implement shared-memory semantics or Atomics integration. It makes the existing exposed constructor internally coherent enough for reflected prototype descriptor checks and direct construction with a byte length.

## Composes-With

- `pilots/missing-intrinsic-loader-failures/trajectory.md`
- `apparatus/docs/deferrals-ledger.md` Entry 013
- Sidecar artifacts:
  - `/home/jaredef/Developer/cruftless-r1-sidecar/results/milf-ext5-r1/post-current-main-smoke.txt`
  - `/home/jaredef/Developer/cruftless-r1-sidecar/results/milf-ext5-r1/post-sab-fix-smoke.txt`
  - `/home/jaredef/Developer/cruftless-r1-sidecar/results/milf-ext5-r1/post-sab-fix-webidl-probe.txt`

**APPROVED for push** per same-turn helmsman directive.
