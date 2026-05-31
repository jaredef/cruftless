---
proposal_slug: 2026-05-31T054000Z-piid-ext-0-promise-race-interleave
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10694
decided_at: 2026-05-31T05:40:00Z
covers_commits:
  - pending
---

## Findings

APPROVED per keeper Telegram 10694. Founds locale `pilots/promise-iteration-interleave-discipline/`. Promise.race becomes the third runtime-tier intrinsic with interleaved iteration + iter_close_rt + error-to-rejection plumbing.

## Verification

Build PASS; PIID-EXT 0 6-cell probe 6/6; cargo test 74/0/1; regression sweep preserved.

## Findings surfaced

- **PIID.1**: synchronous-throw-instead-of-capability-rejection at Promise.all/any/allSettled (Promise.race closed by this rung). Standing-rule promotion candidate.

## Composes-With

- AFID-EXT 0 decision (substrate prefix source)
- IPTD-EXT 1 decision (helper-tier source)
- Carry-forward to PIID-EXT 1/2/3 (Promise.all/allSettled/any)
