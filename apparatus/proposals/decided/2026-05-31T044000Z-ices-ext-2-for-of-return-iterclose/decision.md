---
proposal_slug: 2026-05-31T044000Z-ices-ext-2-for-of-return-iterclose
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10678
decided_at: 2026-05-31T04:40:00Z
covers_commits:
  - pending
---

## Findings

APPROVED per keeper Telegram 10678 ("Ext 2"). Empirically validates Finding ICES.1: a single optional iter slot per LoopFrame is sufficient for both break and return paths without auxiliary state.

## Verification

1. Build PASS (~1m 13s).
2. EXT 2 6-cell probe: 6/6 PASS (close-on-return, innermost-first, value preservation, TypeError on non-callable, plain-return positive control, while-loop positive control).
3. Regression sweep preserved: original IPTD 7/7; cross-consumer 7/7; labelled-break order ["B","A"].
4. Substrate cost: ~10 LOC at one site (Stmt::Return arm). No new helpers, no new opcodes, no LoopFrame changes.

## Composes-With

- `apparatus/proposals/decided/2026-05-31T041500Z-ices-ext-1-for-of-break-iterclose/decision.md`
- `pilots/iterator-close-emission-sites/trajectory.md` ICES-EXT 2 entry
- Carry-forward to ICES-EXT 3 (for-of throw)
