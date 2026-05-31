---
helmsman_session: substrate-resolver-2026-05-31-promise-cluster
proposed_commits: [pending]
target_branch: main
summary: "PCEXC-EXT 0: NewPromiseCapability executor throws TypeError on second invocation, regardless of stored resolve/reject values. Closes capability-executor-called-twice test family at Promise.all/race/any/allSettled/resolve."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "8/8 PASS" }
---

Substrate (~10 LOC, interp.rs): per-capability `already_called: Rc<RefCell<bool>>` flag in NewPromiseCapability's executor closure. First call sets it true; subsequent invocations throw TypeError before storing args. Matches §27.2.1.5.1 GetCapabilitiesExecutor + test262 capability-executor-called-twice expectations.

Verification: 8/8 PASS at probe across Promise.all/race/any/allSettled with first executor() taking (undef,undef) then (fn,fn). cargo test 74/0/1. Full regression sweep preserved.

Per keeper Telegram 10713.
