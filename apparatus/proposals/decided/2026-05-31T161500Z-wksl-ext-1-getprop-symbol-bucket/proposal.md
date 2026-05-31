---
helmsman_session: substrate-resolver-2026-05-31-well-known-symbols
proposed_commits: [pending]
target_branch: main
summary: "WKSL-EXT 1: Op::GetProp + find_getter fall through to Symbol-keyed bucket for '@@'-prefixed keys. Enables bytecode-emitted Symbol-property access (for-of slow-path @@iterator getter, Op::CallMethod @@toPrimitive, etc.) to fire accessor-installed Symbol getters."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "for-of @@iterator getter PASS" }
---
Per keeper Telegram 10733. Carry-forward: spread `[...obj]` + Set ctor still route through `__array_extend` engine helper which doesn't yet consult the Symbol bucket; separable.
