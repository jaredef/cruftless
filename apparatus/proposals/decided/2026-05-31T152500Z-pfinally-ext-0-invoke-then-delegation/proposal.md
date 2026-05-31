---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PFINALLY-EXT 0: Promise.prototype.finally delegates through Invoke(this, 'then', thenFinally, catchFinally) per §27.2.5.3. Builds ThenFinally/CatchFinally closures (name '', length 1) when onFinally is callable; passes onFinally directly when not. Replaces the prior hand-rolled finally that bypassed this.then and synchronously dispatched onFinally — closes the finally/invokes-then-* family at Promise.prototype.finally."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "14/14 PASS" }
---
Per keeper Telegram 10723.
