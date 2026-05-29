---
proposal_slug: 2026-05-29T011600-caacp-stage-b-closure
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-29T01:16:00Z
covers_commits:
  - 5173fd3bd18a90c8db3abd77a283d4a426847f6c
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out.

Keeper Rung-2 authorization: Telegram 10247 ("Deploy to the home/jaredef/jaredfoy repo"). The substrate commit at 5173fd3b is the cruftless-side wrapper bug fix discovered during Stage B closure; the broader Stage B deployment also executed three keeper-authorized acts off-repo (token generation, .env append, sudo systemctl restart) which are not git-trackable in cruftless but are documented in the proposal.

**Apparatus-tier verification**:

1. **Endpoint live**: GET https://jaredfoy.com/api/caacp/v1/inbox/helmsman returns 200 with valid JSON; without token returns 401. Server is the new pid post-restart.

2. **First end-to-end cybernetic loop closure**: wrapper send → real UUID + persisted artifact; wrapper ack → real ack-id + state transition; outbox listing reflects state correctly; endpoint GET returns message + ordered acknowledgments. Full CAACP §IV state machine validated.

3. **Wrapper jq bug fix**: substantively a one-line change (`select` → `if-then-else`). Smoke-tested post-fix; both send and ack now sync to endpoint correctly when token is set.

4. **No substrate impact**: gates_pre and gates_post identical (TAMM 82, TAWR 63, diff-prod 61/51, CLFG 27/32).

5. **Secret discipline honored**: token in gitignored files only; not in any commit. Bootstrap copy in /tmp/ with mode 600 for keeper retrieval.

6. **Rule discipline honored**: Rule 4 single rung; Rule 15 chapter-close-inspect via end-to-end smoke. The bug discovery itself was a Rule-15 instance — the wrapper hadn't been exercised against a real endpoint until Stage B closure, and the chapter-close-inspect surfaced the jq pruning behavior.

**Apparatus-meta concerns considered**:

- CAACP cybernetic loop is now operational. Stage C (Telegram demotion) becomes the next milestone once Stage B has demonstrated stability across some engagement cycles.
- Stage 2 mechanical-veto coverage: this proposal+decision pair covers the substrate commit's SHA.

**APPROVED for push.**

Archive to `apparatus/proposals/archived/2026-05-29T011600-caacp-stage-b-closure/` after push lands.
