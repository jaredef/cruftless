---
proposal_slug: 2026-05-29T203200-milf-ext-2-partial-land-node-shims
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T20:32:00Z
covers_commits:
  - cbca7074
  - 5b5f742e
  - af4a78c9
---

## Findings

Approved under the Helmsman MILF-EXT 2 partial-land authorization.

The proposal covers the committed partial closure:

1. `process.umask`
2. `process.features.require_module`
3. `util.debug`
4. `util.inherit`
5. `TextDecoder`/`TextEncoder` forwarding from globals

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS,
   68 passed and 1 ignored.
3. Smoke:
   - `gulp` PASS.
   - `mocha` PASS.
   - `aws-sdk` remains deferred on `AWS.util.inherit` bootstrap/circular-require behavior.
   - `forever` remains deferred on stack overflow before import completion.

**APPROVED for push.**
