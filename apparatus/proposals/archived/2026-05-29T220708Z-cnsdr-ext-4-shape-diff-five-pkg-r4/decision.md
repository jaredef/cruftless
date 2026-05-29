---
proposal_slug: 2026-05-29T220708Z-cnsdr-ext-4-shape-diff-five-pkg-r4
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T22:07:08Z
covers_commits:
  - e2ed677a0fb6915d097adf5c4434c0864da9876e
---

## Findings

Approved under helmsman directive `7058f44f-dd1a-4644-bc28-451132d70cb7`.

The substrate commit closes the dominant coherent submechanism inside the five-package CNSDR cluster:

1. Reuses the existing dual-package gate for `main` + `module` packages with no `exports`.
2. Mirrors the existing default export's own properties into the namespace when Bun does so for dual-package interop.
3. Adds focused loader regressions for bare-specifier `node_modules` fixtures matching the real package resolution path.
4. Records the three remaining rows as a deferred sibling family instead of forcing a mixed-mechanism patch.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 71 passed and 1 ignored.
3. Focused loader tests: `cargo test --release -p rusty-js-runtime --test module_loader dual_package_default -- --nocapture` PASS.
4. Focused five-package parity sample: `proj4` and `decimal.js-light` flipped to PASS; cluster improved from 0/5 to 2/5.

The remaining `readable-stream`, `events`, and `winston` rows are not regressions from this move and do not share the same mechanism. Deferring them is the correct conservative closure.

**APPROVED for push.**
