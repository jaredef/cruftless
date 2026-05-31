---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "IPHD-EXT 0: Iterator.from lazy wrapper + 9 Iterator.prototype helpers interleaved (map, filter, take, forEach, some, every, find, reduce, toArray). Founds locale pilots/iterator-helpers-discipline/. Closes Iterator-helpers eager-collect + missing-close + missing-short-circuit anti-patterns surfaced by AFID.2 audit residual."
risk_class: substrate
gates_post:
  build: PASS
  cargo_test: 74/0/1
  probe: 8/8 PASS
---

Substrate (~250 LOC, intrinsics.rs): Iterator.from returns a lazy wrapper with __wrapped_iter + next/return forwarding to inner. 9 helpers rewritten to per-element next loops with iter_close_rt on cb-throw / short-circuit / non-Object next-result. Carry-forward: drop, flatMap, true-lazy map/filter.

Per keeper Telegram 10702.
