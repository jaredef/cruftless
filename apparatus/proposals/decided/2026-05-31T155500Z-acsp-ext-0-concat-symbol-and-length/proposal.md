---
helmsman_session: substrate-resolver-2026-05-31-array-cluster
proposed_commits: [pending]
target_branch: main
summary: "ACSP-EXT 0: Array.prototype.concat — resolve @@isConcatSpreadable via the well-known Symbol's shared Rc (PropertyKey::Symbol uses Rc::ptr_eq for identity) and use try_array_length for spec-strict length-getter throws. Closes concat/is-concat-spreadable-get-err + concat_length-throws families."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "2/2 PASS" }
---
Per keeper Telegram 10729.
