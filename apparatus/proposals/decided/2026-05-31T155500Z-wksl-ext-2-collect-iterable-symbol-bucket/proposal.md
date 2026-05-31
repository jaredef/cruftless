---
helmsman_session: substrate-resolver-2026-05-31-well-known-symbols
proposed_commits: [pending]
target_branch: main
summary: "WKSL-EXT 2: route @@iterator lookups in collect_iterable + Set/WeakSet ctor + Map.groupBy through lookup_well_known_method. Closes the spread / new Set / Map.groupBy with Symbol-keyed-getter @@iterator carry-forward from WKSL-EXT 1."
risk_class: substrate
gates_post: { build: PASS, cargo_test: "74/0/1", probe: "8/8 PASS" }
---
Per keeper Telegram 10745. Closes ITER-ARC.6 carry-forward.
