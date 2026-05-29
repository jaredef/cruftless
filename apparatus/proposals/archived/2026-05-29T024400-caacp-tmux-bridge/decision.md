---
proposal_slug: 2026-05-29T024400-caacp-tmux-bridge
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-29T02:44:00Z
covers_commits:
  - 2c26757f72f6879562aabe3fa9d358567f6229e5
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out. Keeper Rung-2 authorization: Telegram 10270 (problem statement) + 10272 (watcher's design refinements + endorsement of helmsman's option-1+document-2 path). Substrate commit at 2c26757f implements the agreed design.

**Apparatus-tier verification**:
- Bridge script bash -n syntax-clean; usage/missing-target failure paths verified.
- All five watcher refinements implemented (endpoint-source, seen-cache, short-directive, pre-flight, operator-started).
- Init protocol §IV.5 + §V landed; numbering renumbered cleanly.
- No substrate impact: gates unchanged.

**APPROVED for push.** Archive to `apparatus/proposals/archived/2026-05-29T024400-caacp-tmux-bridge/` after push lands.
