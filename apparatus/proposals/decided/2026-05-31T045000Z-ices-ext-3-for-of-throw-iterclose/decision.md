---
proposal_slug: 2026-05-31T045000Z-ices-ext-3-for-of-throw-iterclose
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10680
decided_at: 2026-05-31T04:55:00Z
covers_commits:
  - pending
---

## Findings

APPROVED per keeper Telegram 10680 ("Continue"). Closes the ICES primary scope (break + return + throw). Empirically validates Finding ICES.1's third predicted application — per-frame `(try_open, close_slot)` unwind is identical at all three abrupt-completion shapes (break, continue-crossed-frames, return) and at one new substrate site (synthetic catch stub).

## Verification

1. Build PASS (~1m 11s).
2. cargo test rusty-js-runtime --lib: 74 / 0 / 1 preserved.
3. EXT 3 9-cell probe: 9/9 PASS — body-throw close, nested innermost-first, value preservation, close-throw replaces body-throw, outer try-catch sees close before catch, break/return regressions preserved, continue keeps iterating with balanced try-stack.
4. Regression sweep: IPTD 7/7, cross-consumer 7/7, labelled-break ["B","A"], ICES-EXT 2 6/6.

## Findings surfaced

- **Finding ICES.1 — third application confirmed** (LoopFrame as cross-frame anchor across break/return/throw): predicted in EXT 1 trajectory, validated in EXT 2 (first cross-path application), and now in EXT 3 (third + synthetic catch stub). Recurrence threshold for standing-rule promotion satisfied.
- **Finding ICES.2 — Rule 24 emit-site coherence threshold met**: three structurally-identical `(try_open, close_slot)` unwind walks across `loop_stack` (break, continue-crossed, return). Candidate for `emit_frame_unwind` helper promotion at a Rule-24 follow-up rung if a fourth site appears (yield* delegation, async-iter unwind).

## Composes-With

- ICES-EXT 1 + 2 decision docs
- `pilots/iterator-close-emission-sites/trajectory.md` ICES-EXT 3 entry
- Carry-forward: for-await body throw (sibling), yield* delegation close (`iterator-close-on-abrupt/` locale).
