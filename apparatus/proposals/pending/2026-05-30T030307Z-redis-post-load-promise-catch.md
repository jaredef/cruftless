---
helmsman_session: helmsman-caacp
proposed_commits:
  - 725f985b
target_branch: main
summary: Redis post-load Promise.catch terminal rejection closure
risk_class: substrate
gates_pre:
  baseline: redis import printed namespace then ended with unhandled callee undefined rejection
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_test: cargo test --release -p rusty-js-runtime --test promise_golden PASS
  redis_smoke: exact import("redis").then(...).catch(...) smoke PASS without unhandled rejection
  push: pending
---

## Substrate Move

This proposal covers substrate commit `725f985b`.

`Runtime::promise_catch_via` now resolves `this.then` through the runtime's
normal `get_via` path. `Runtime::promise_then_via` now normalizes non-callable
handlers to absent handlers before storing or enqueuing Promise reactions.

## Cause

Redis post-load was not failing on Redis package evaluation. The reduced
`Promise.resolve(1).then(...).catch(...)` probe produced the same terminal
unhandled rejection.

`Promise.prototype.catch` was manually walking prototypes with `get_own("then")`,
which missed the shape-backed `Promise.prototype.then` installation and tried to
call `undefined`. After that lookup was corrected, `.catch(...)` still exercised
`.then(undefined, onRejected)`, and `promise_then_via` incorrectly treated the
`undefined` fulfillment handler as callable reaction state instead of applying
identity propagation.

## Verification

- `cargo test --release -p rusty-js-runtime --test promise_golden`: PASS,
  `8 passed`.
- `cargo build --release --bin cruft -p cruftless`: PASS.
- Reduced CLI smoke
  `Promise.resolve(1).then(v => console.log('then', v)).catch(...)`: PASS,
  no unhandled rejection.
- Redis sidecar exact smoke from
  `/home/jaredef/Developer/cruftless-r2-sidecar/redis-post-load-r2`:
  `import("redis")` prints `OK 58` and the first eight exports, then exits
  without the prior terminal unhandled rejection.

## Residual

`redis.createClient().connect().catch(...)` now reaches Redis socket retry code
and reports a distinct downstream runtime gap at
`@redis/client/dist/lib/client/socket.js:221:50`, in the
`timers/promises.setTimeout(retryIn)` retry loop. This is separate from the
post-load import terminal rejection closed here.

## Coordination

Helmsman authorized this directive in CAACP message
`d1d79ea9-e251-492c-a430-5f6fd0e4c1b0`.

**APPROVED for push** per Helmsman redis post-load directive.
