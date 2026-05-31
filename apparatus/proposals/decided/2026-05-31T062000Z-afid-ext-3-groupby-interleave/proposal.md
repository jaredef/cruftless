---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "AFID-EXT 3: Object.groupBy + Map.groupBy interleave + IteratorClose-on-cb-throw per ECMA-262 §23.1.2.5 / §24.1.2.2 + GroupBy. Prior impls eager-drained the iterable then ran cb per element, missing IteratorClose on cb abrupt completion + OOMing on infinite iters when cb would throw."
risk_class: substrate
gates_post:
  build: PASS
  cargo_test: 74/0/1
  probe: 7/7 PASS
---

## Substrate

`object_group_by_via` (interp.rs:5966) + Map.groupBy intrinsic closure (intrinsics.rs:21537) rewritten to per-element iterate next → cb → push-to-bucket, with `iter_close_rt` best-effort on any abrupt completion + propagate the original error per §7.4.9 step 4.

~80 LOC across two sites.

## Verification

AFID-EXT 3 7-cell probe: 7/7 PASS. cargo test 74/0/1 preserved. Full regression sweep preserved (IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8, AFID-EXT 1 7/7, PIID-EXT 0 6/6, PIID-EXT 1+2+3 12/12, PIID-EXT 4 9/9).

## Authorization

Per keeper Telegram 10700 ("Continue") authorizing further iter-consuming intrinsic audit.
