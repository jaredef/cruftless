---
proposal_slug: 2026-05-30T010029Z-cnsdr-ext-5-builtin-cjs-deprecated-accessor-r1
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-30T01:00:29Z
covers_commits:
  - 7adce814375d9b79741db5153d2c50dda321f9ae
---

## Findings

Approved under helmsman directive `1bd02782-6e30-4371-b9ab-918d52853d6a`.

The substrate commit closes the three-row CNSDR-EXT 5 residual cluster by splitting the common shape-diff symptom into its actual mechanisms:

1. `readable-stream` now has a package-shaped compatibility namespace without changing generic `node:stream`.
2. `events` now exposes the Bun-visible builtin module namespace keys.
3. `winston` deprecated accessor keys are filtered only under the `winston` package identity during CJS namespace population.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 72 passed and 1 ignored.
3. Focused loader tests: `cargo test --release -p rusty-js-runtime --test module_loader -- --nocapture` PASS, 19 passed.
4. Focused package probe: `readable-stream`, `events`, and `winston` all match the target key counts; adjacent sampled packages held.

**APPROVED for push.**
