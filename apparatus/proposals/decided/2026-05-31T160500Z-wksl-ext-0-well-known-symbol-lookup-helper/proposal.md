---
helmsman_session: substrate-resolver-2026-05-31-well-known-symbols
proposed_commits: [pending]
target_branch: main
summary: "WKSL-EXT 0: Runtime::lookup_well_known_method helper that resolves a well-known Symbol-keyed property using the shared Symbol Rc from globalThis.Symbol, falling back to the string-keyed @@name bucket. Refactors ACSP-EXT 0 (concat @@isConcatSpreadable) to use the helper and applies it at Array.from's @@iterator detection + method-fetch. Closes the Array.from getter-installed-@@iterator case (was returning empty array)."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "Array.from+@@iterator-getter 2/2 PASS" }
---

Carry-forward: applying the helper at the bytecode Op::GetProp dispatch (so for-of slow-path also resolves Symbol-keyed @@iterator) is the natural extension; ~10 sites in interp.rs + intrinsics.rs that read well-known @@ properties via the string-keyed path remain candidates.

Per keeper Telegram 10731.
