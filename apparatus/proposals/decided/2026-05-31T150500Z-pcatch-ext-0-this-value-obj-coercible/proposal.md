---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PCATCH-EXT 0: Promise.prototype.catch ToObject-coerces primitive receivers per §27.2.5.1 + Invoke abstract op (GetV ToObject-wraps). Closes catch/this-value-obj-coercible.js."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "7/7 PASS" }
---
Per keeper Telegram 10719.
