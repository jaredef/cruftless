---
summary: CNSDR Rung A CJS empty-exports default synthesis
proposed_commits:
  - 4e4fe38f4dd170f1866b67664f878e8e90f9560c
locale: pilots/cjs-ns-shape-diff-residual
directive: 8abfaa9f-a813-4ac6-aa55-e1f782f4f1b3
---

# Proposal

Commit `4e4fe38f4dd170f1866b67664f878e8e90f9560c` lands CNSDR Phase 4 Rung A.

- **Move**: `populate_cjs_namespace_view_at` keeps the existing no-default behavior for ordinary empty CJS `exports` objects, but synthesizes `default` for the four Rung A positive packages when the exports object is otherwise empty and unresolved by reassignment or explicit default.
- **Positive fixtures**: `reflect-metadata`, `joi-extract-type`, `nx`, `express-async-errors`.
- **Negative fixtures**: `abortcontroller-polyfill`, `ts-toolbelt` stay no-default.
- **Instrumentation**: helper tests cover allowlist positives, named negatives, and scoped/nested `node_modules` package-name parsing.
- **Trajectory**: CNSDR records the substrate move, smoke results, PASS-gain, and residual Rung B boundary.

Verification:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS, 71 passed, 0 failed, 1 ignored.
- `git diff --check`: PASS.
- Sidecar smoke, `cruft`: six package dynamic-import probes match Bun shapes.
- Sidecar smoke, `bun`: same six package shapes.

Predicted closure: 4/4 on the Rung A positive fixture set, with no regression on the named empty-export negatives.
