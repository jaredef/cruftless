---
proposal_slug: 2026-05-28T223600-stage-4-service-tier-deployment
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-28T22:36:00Z
covers_commits:
  - d26fd3666967479a9113f9cab9a7e53f6cf97fa3
---

## Findings

This decision is keeper-substituted per the operational-protocol §VI.2 carve-out for the pre-instantiation period (no arbiter session has been appointed yet; the helmsman self-enforces the proposal-writing discipline and the keeper substitutes for the arbiter on push decisions).

The keeper's authorization for this commit is keeper directive Telegram 10219 ("continue to the next stage. but first prompt the skills to the cruftless repository itself under apparatus/skills"). The two substrate moves in the proposal (skills relocation + Stage 4 deployment) directly execute that directive.

**Apparatus-tier verification**:

1. **No substrate impact**: the proposal's gates_pre and gates_post are identical (67.6 / 84.8 / 61-51 / TAWR 63 / TAMM 82). The commit touches no `pilots/*/derived/src/` files; cannot regress the substrate. Gate verification is moot.

2. **Symlink mechanics**: the `.claude/skills/` symlink resolves correctly via `ls .claude/skills/` per the helmsman's smoke test. Claude Code's skill discovery typically follows symlinks; fallback documented.

3. **Append-only protocol honored**: the deletions-ledger and deferrals-ledger were not touched (no entries to add); the apparatus discipline edits to operational-protocol §IV.2 + §VII Stage 2 + Stage 4 are permitted in-place per that doc's consolidated-view update protocol.

4. **Skill-discipline integrity**: each new load skill (watcher-load, deputy-load) explicitly states canonical-path-is-apparatus + symlink-note + invocation-only-on-keeper-appointment. Honors the resolver-role-discipline clause in CLAUDE.md/AGENTS.md.

5. **Stage 4 element (c) deferred**: the `(last-refreshed: YYYY-MM-DD)` annotation discipline is intentionally NOT applied in this commit; deferred to the first watcher session per the Stage 4 spec. The Stage 4 deployment landing without this element is consistent with the spec (the annotations are first-watcher-session work, not deployment-time work).

**Apparatus-meta concerns considered**:

- The triumvirate's minimality claim at the governance tier is preserved (watcher + deputy are service-tier, not governance).
- The keeper's Rung-2 monopoly is preserved (no resolver acquires intervention authority via this commit).
- The pre-push hook's coverage check is honored (this proposal+decision pair covers the substrate commit's SHA).
- The arbiter handover discipline is unaffected (no arbiter sessions exist yet to hand over from).

**APPROVED for push.**

Once pushed, this proposal + decision should be moved to `apparatus/proposals/archived/2026-05-28T223600-stage-4-service-tier-deployment/` per the operational protocol's archival discipline.
