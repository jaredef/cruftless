---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PCRT-EXT 0: route abrupt completion from cap.[[Resolve]] (done-branch finalize) through IfAbruptRejectPromise per §27.2.4.1 step 8 / §27.2.4.2 step 8. Wraps promise_all_maybe_complete_via call at Promise.all + Promise.allSettled done-branches; on cap_resolve throw, route through promise_reject_with_error(cap_reject, ...)."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "2/2 PASS" }
---
Per keeper Telegram 10715.
