---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PSCV-EXT 1: Promise subclass-routing via NewTarget OrdinaryCreateFromConstructor + Promise.resolve/reject route through NewPromiseCapability when C != native Promise."
risk_class: substrate
gates_post:
  build: PASS
  cargo_test: "74/0/1"
  probe: "39/39 PASS (was 36/39)"
---

Substrate (~50 LOC, promise.rs):
- Promise constructor (§27.2.3.1 step 3): override new Promise's [[Prototype]] from `current_new_target.prototype` when NewTarget is set. Closes `class SubP extends Promise; new SubP(...) instanceof SubP`.
- Promise.resolve closure (§27.2.4.7 PromiseResolve): when C === native Promise, short-circuit via promise_resolve_via; else route through NewPromiseCapability(C) + cap.[[Resolve]](x). Honors §27.2.4.7 step 1 "if IsPromise(x) and x.constructor === C, return x" short-circuit.
- Promise.reject closure (§27.2.4.4): same shape — short-circuit on native, route via NewPromiseCapability(C) + cap.[[Reject]](r) for subclasses.

Verification: PSCV probe 39/39 PASS (was 36/39 after EXT 0; closes 3 subclass cells). cargo test 74/0/1. Full regression sweep preserved (9 probes).

Per keeper Telegram 10711.
