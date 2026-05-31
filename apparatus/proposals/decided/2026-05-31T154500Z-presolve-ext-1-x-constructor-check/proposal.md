---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PRESOLVE-EXT 1: Promise.resolve only short-circuits when IsPromise(x) AND x.constructor === C. Closes arg-uniq-ctor (x.constructor=null returns NEW promise instead of x). Also removes Promise.reject fast-path (spec has no short-circuit; always NewPromiseCapability(C) + Reject(r))."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "6/6 PASS" }
---
Per keeper Telegram 10727.
