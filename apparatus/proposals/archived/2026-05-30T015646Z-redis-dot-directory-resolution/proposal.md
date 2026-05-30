---
helmsman_session: helmsman-caacp
proposed_commits:
  - ae0f98b6
target_branch: main
summary: Redis package dot-directory resolution singleton
risk_class: substrate
gates_pre:
  baseline: redis import failed on node_modules/./package.json lookup
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  targeted_test: resolve_module_treats_dot_as_relative_directory PASS
  redis_smoke: import("redis") reaches populated namespace keyCount=58
  push: pending
---

## Substrate Move

This proposal covers substrate commit `ae0f98b6`.

`Runtime::resolve_module` and `Runtime::resolve_module_full` now classify `.`
and `..` as relative directory specifiers alongside `./` and `../`.

## Cause

Redis loads `@redis/client/dist/lib/client/pool.js`, whose CommonJS body calls
`require(".")`. Before this move, the resolver did not treat `.` as relative
and the package load fell through to a `node_modules/./package.json` lookup.

## Verification

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime module::tests::resolve_module_treats_dot_as_relative_directory`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS, 72 passed and 1 ignored.
- Redis sidecar smoke from
  `/home/jaredef/Developer/cruftless-sidecar/results/citpt-ext-3-parity-sandbox/redis`:
  `import("redis")` reaches populated namespace, `keyCount=58`.

## Residual

The smoke process prints a later unhandled promise rejection after the namespace
result:

`TypeError("callee is not callable: undefined [argc=1]")`

That is after-import async behavior, not the package-json resolution target.
Treat as a candidate for MILF-EXT-5 or a separate residual locale.

## Coordination

Helmsman authorized the push pipeline in CAACP message
`cbb8832a-6eae-4664-b86e-bb0a150d5ec7`.

Observed unstaged formatting-only diffs in four files (`node_stubs.rs`,
`interp.rs`, `intrinsics.rs`, `module_loader.rs`); not touched or staged.

**APPROVED for push** per Helmsman redis singleton authorization.
