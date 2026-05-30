---
proposal_slug: 2026-05-30T030307Z-redis-post-load-promise-catch
decision: APPROVED
arbiter_session: helmsman-approved-caacp
decided_at: 2026-05-30T03:03:07Z
covers_commits:
  - 725f985b
---

## Findings

Approved under Helmsman CAACP message
`d1d79ea9-e251-492c-a430-5f6fd0e4c1b0`.

The substrate move is scoped to Promise instance combinator semantics:
`Promise.prototype.catch` now dispatches through ordinary `Get(this, "then")`,
and `Promise.prototype.then` no longer stores non-callable values as reaction
handlers. This closes the Redis after-import terminal unhandled rejection
without package-specific code.

Verification:

1. `cargo test --release -p rusty-js-runtime --test promise_golden`: PASS,
   `8 passed`.
2. `cargo build --release --bin cruft -p cruftless`: PASS.
3. Reduced CLI `.then(...).catch(...)` smoke: PASS.
4. Redis exact import smoke: PASS, `OK 58` with no unhandled rejection.

The later `redis.createClient().connect()` retry-loop failure is a distinct
downstream socket/timers residual and is not required for this post-load closure.

**APPROVED for push.**
