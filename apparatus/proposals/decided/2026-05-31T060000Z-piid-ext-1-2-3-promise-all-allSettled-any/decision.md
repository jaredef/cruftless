---
proposal_slug: 2026-05-31T060000Z-piid-ext-1-2-3-promise-all-allSettled-any
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10696
decided_at: 2026-05-31T06:00:00Z
covers_commits:
  - pending
---

## Findings

APPROVED per keeper Telegram 10696 ("Push all and continue"). Closes Finding PIID.1 (sync-throw-vs-capability-rejection). Surfaces Finding PIID.2 (AggregateError shape on all-reject).

## Verification

Build PASS; 12-cell probe 11/12 (one pre-existing AggregateError shape failure); cargo test 74/0/1; regression sweep preserved.

## Composes-With

- PIID-EXT 0 decision (substrate prefix source)
- Carry-forward: Finding PIID.2 (AggregateError construction) — sibling locale.
