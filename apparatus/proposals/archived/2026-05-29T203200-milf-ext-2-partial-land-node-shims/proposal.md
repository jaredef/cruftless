---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - cbca7074
  - 5b5f742e
  - af4a78c9
target_branch: main
summary: MILF-EXT 2 - partial land of the node-shim cluster
risk_class: substrate
gates_pre:
  local_smoke: gulp/mocha/TextDecoder PASS; aws-sdk and forever deferred
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  push: pending
---

## Substrate Moves

This is the partial closure for the MILF-EXT 2 node-shim cluster.

- `process.umask()` returns the conventional Linux mask `0o022`.
- `process.features.require_module` is truthy.
- `util.debug()` is a callable no-op logger.
- `util.inherit(ctor, super_)` wires the constructor prototype chain.
- `TextDecoder` and `TextEncoder` are forwarded from the global surface when present.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 68 passed, 1 ignored.
- Smoke:
  - `gulp` PASS.
  - `mocha` PASS.
  - `release-it` hit a deeper runtime stack overflow before a clean named-shim failure.
  - `aws-sdk` still fails on `AWS.util.inherit` during CommonJS bootstrap.
  - `forever` still aborts with a stack overflow before a normal import result.

## Risk Assessment

The landed subset is confined to host-shim and utility compatibility. The two
residual failures are now named as separate follow-up coordinates:

1. `aws-sdk` circular/bootstrap behavior around its own `AWS.util.inherit`.
2. `forever` stack overflow before import completion.

## Composes-With

- `pilots/missing-intrinsic-loader-failures/`.
- `cruftless/src/process.rs`.
- `cruftless/src/util.rs`.
- MILF-EXT 3 residual follow-ups for `aws-sdk` and `forever`.

**APPROVED for push** per Helmsman MILF-EXT 2 partial-land authorization.
