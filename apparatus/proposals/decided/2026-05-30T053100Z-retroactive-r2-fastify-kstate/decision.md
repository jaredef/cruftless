---
proposal_slug: 2026-05-30T053100Z-retroactive-r2-fastify-kstate
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-retroactive-coverage
decided_at: 2026-05-30T05:31:00Z
covers_commits:
  - 3e9ba56da563e14b91927ebf0f394e6c90a6abde
---

## Findings

Retroactive decision covering R2's prior fastify[kState] landing at commit `3e9ba56d bytecode: let lexical names shadow function self`. Surfaced as uncovered by the freshly-fixed pre-push hook (commit `1c5b9da0`); the (then-broken) hook had silently no-op'd the gate when R2 originally pushed.

Substrate quality is high: function-body let/const with the same name as the function declaration now shadows the self-name slot for closure capture; nested closures (fastify's `throwIfAlreadyStarted` capturing the body binding) correctly resolve to the instance object rather than the function object. Regression test `function_body_lexical_name_shadows_declaration_self_name_for_closure` carries the diagnosis forward.

Verification (per R2's original landing summary, message `6c42a585`):
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- Reduced shadow probe prints expected output.
- fastify smoke prints `function`; express + koa cross-smoke PASS.
