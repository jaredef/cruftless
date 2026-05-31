---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PTHEN-EXT 1: Promise.prototype.then SpeciesConstructor lookup per §27.2.5.4 step 3 + §7.3.22. Resolves Get(promise, 'constructor'); throws TypeError on non-Object; fast-path through new_promise when C === native Promise; subclass path via NewPromiseCapability(C) preserves SubP-derived chain shape."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "6/6 PASS" }
---
Per keeper Telegram 10721. Closes ctor-null + ctor-throws + ctor-poisoned + ctor-custom families at Promise.prototype.then. Promise.prototype.catch inherits the fix via delegation.

Carry-forward: @@species getter resolution (PropertyKey::Symbol vs string-keyed) — current impl falls through but doesn't always invoke string-keyed getters.
