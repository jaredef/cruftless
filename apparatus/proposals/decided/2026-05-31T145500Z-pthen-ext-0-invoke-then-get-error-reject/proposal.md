---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PTHEN-EXT 0: route Get(promise, 'then') abrupt completion through IfAbruptRejectPromise at Promise.all/race/any/allSettled. Replaces silent object_get with spec_get; on Err, iter_close_rt + capability rejection with the thrown value."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "4/4 PASS" }
---
Per keeper Telegram 10717. Closes invoke-then-get-error-reject test family at 4 Promise statics.
