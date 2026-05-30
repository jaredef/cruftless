---
proposal_slug: 2026-05-30T015646Z-redis-dot-directory-resolution
decision: APPROVED
arbiter_session: helmsman-approved-caacp
decided_at: 2026-05-30T01:56:46Z
covers_commits:
  - ae0f98b6
---

## Findings

Approved under Helmsman CAACP message
`cbb8832a-6eae-4664-b86e-bb0a150d5ec7`.

The substrate move is clean and scoped: `.` and `..` now resolve through the
relative-directory path instead of package-name lookup. This closes the redis
`require(".")` package-json lookup failure without changing package field
priority, conditional exports, or adding a package-specific shim.

Verification:

1. `cargo build --release --bin cruft -p cruftless`: PASS.
2. `cargo test --release -p rusty-js-runtime module::tests::resolve_module_treats_dot_as_relative_directory`: PASS.
3. `cargo test --release -p rusty-js-runtime --lib`: PASS, 72 passed and 1 ignored.
4. Redis sidecar smoke reaches populated namespace, `keyCount=58`.

Residual after-import async behavior remains:
`TypeError("callee is not callable: undefined [argc=1]")`.

**APPROVED for push.**
