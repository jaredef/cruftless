---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PRESOLVE-EXT 0: thenable resolution per §27.2.1.4 PromiseResolveThenableJob. When resolve_promise is called with a non-Promise Object whose .then is callable, invoke value.then(resolveFn, rejectFn). Get(.then) throws are routed to rejection. Closes Promise/resolve/arg-poisoned-then + resolve-poisoned-then-immed/deferred."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "5/5 PASS" }
---
Per keeper Telegram 10725.
