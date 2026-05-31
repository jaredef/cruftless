---
proposal_slug: 2026-05-31T051000Z-ices-ext-3-1-close-throw-shadow-fix
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10686
decided_at: 2026-05-31T05:10:00Z
covers_commits:
  - pending
---

## Findings

APPROVED per keeper Telegram 10686 ("Continue"). Resolves Finding AFID.1: ICES-EXT 3's synthetic catch stub now preserves the original body-thrown error per §7.4.9 step 4 by swallowing close-thrown errors via a nested synthetic try-catch (`Op::Pop` in the inner catch arm).

## Verification

1. Build PASS (~1m 12s).
2. ICES-EXT 3.1 5-cell probe: 5/5 PASS (original Error preserved across non-callable, throwing-callable, missing, and nested-throw close paths; positive control on break with non-callable return TypeError still observed).
3. cargo test rusty-js-runtime --lib: 74/0/1 preserved.
4. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, AFID 8/8, labelled-break ["B","A"].

## Composes-With

- ICES-EXT 3 decision (substrate this corrects)
- AFID-EXT 0 decision (Finding AFID.1 source)
- Closes Finding AFID.1.
