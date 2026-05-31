---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "ICES-EXT 3.1: spec-correctness fix for §7.4.9 step 4. The ICES-EXT 3 synthetic catch stub at the for-of body-throw close site let close-thrown errors replace body-thrown errors; per ECMA-262 §7.4.9 step 4, the original (body) throw must be preserved even when GetMethod / Call of iter.return themselves throw. Wraps the emit_iter_close_call invocation in a nested synthetic try-catch that swallows close-thrown values (Op::Pop in the inner catch arm) before re-throwing the spilled original. Addresses Finding AFID.1."
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: 88.7%
gates_post:
  build: "cargo build --release --bin cruft -p cruftless"
  cargo_test_runtime_lib: "74 / 0 / 1 preserved"
  probe_cells:
    - "body throws Error + iter.return non-callable (42) -> ORIGINAL Error wins"
    - "body throws Error + iter.return throws RangeError -> ORIGINAL Error wins"
    - "body completes normally (break) + iter.return non-callable -> close TypeError observed (no original)"
    - "body throws + iter.return null -> ORIGINAL preserved + close silent"
    - "Nested for-of inner body throws + both close -> ORIGINAL + close order IN,OUT"
---

## Substrate

~10 LOC at the for-of synthetic catch stub. Replaces:

```
catch_pos:
  StoreLocal <thrown>
  emit_iter_close_call iter_slot    ; if close throws, propagates immediately
  LoadLocal <thrown>
  Throw                              ; never reached if close threw
```

with:

```
catch_pos:
  StoreLocal <thrown>
  TryEnter <swallow_pos>
  emit_iter_close_call iter_slot
  TryExit
  Jump <after_swallow>
swallow_pos:
  Pop                                ; discard close-thrown
after_swallow:
  LoadLocal <thrown>
  Throw                              ; always the spilled original
```

The nested try-catch is for the close call's potential abrupt completion only; on normal close (or silent skip for null/undefined .return), the TryExit pops the synthetic frame and Jump skips the swallow arm.

## Verification

1. `cargo build --release --bin cruft -p cruftless` PASS (~1m 12s).
2. Direct probe `/tmp/probe-ices-3-1.js` (5 cells): 5/5 PASS.
3. `cargo test --release -p rusty-js-runtime --lib`: 74 / 0 / 1 preserved.
4. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, AFID 8/8, labelled-break ["B","A"].

## Composes-With

- `apparatus/proposals/decided/2026-05-31T045000Z-ices-ext-3-for-of-throw-iterclose/decision.md` (the substrate this fixes)
- `apparatus/proposals/decided/2026-05-31T050000Z-afid-ext-0-interleave-and-close/decision.md` (Finding AFID.1 surfaced this)
- `apparatus/docs/predictive-ruleset.md` Rule 13 (deeper-layer closure of a partially-spec-correct rung), Rule 17 (segmentation: spec-correctness-only scope)

## Authorization

Per keeper Telegram 10686 ("Continue") authorizing the AFID-EXT 0 surfaced finding (AFID.1) for in-place fix at the ICES locale.
