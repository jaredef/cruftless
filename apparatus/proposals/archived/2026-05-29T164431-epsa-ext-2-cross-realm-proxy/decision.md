---
proposal_slug: 2026-05-29T164431-epsa-ext-2-cross-realm-proxy
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T16:44:31Z
covers_commits:
  - 7d4ed17a72bc583575a764f0ae6eb10648afb0d6
---

## Findings

Approved under Helmsman EPSA-EXT 2 directive for R2.

The substrate commit closes the remaining current `Error.prototype.stack` directory residuals by:

1. Installing realm-local stack accessors on cloned Error prototypes.
2. Preserving receiver identity through proxy getter fallback and `Reflect.get(target, key, receiver)`.
3. Routing Proxy stack setter cases through traps and descriptor invariants.
4. Exposing a local realm global helper to the test262 runner bridge.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 66 passed and 1 ignored.
3. Full `built-ins/Error/prototype/stack/*.js` directory: 35 PASS / 0 FAIL / 0 SKIP.

Remaining EPSA scope is trace-format content only, outside this rung.

**APPROVED for push.**
