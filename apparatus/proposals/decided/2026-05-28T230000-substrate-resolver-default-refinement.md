---
proposal_slug: 2026-05-28T230000-substrate-resolver-default-refinement
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-28T23:00:00Z
covers_commits:
  - 1af2941f1dc76d8c7d05d672c08cc434ae6578f5
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out for the pre-instantiation period.

Keeper Rung-2 authorization: Telegram 10225 (articulation of the substrate-resolver-as-default refinement) + 10226 (explicit "confirmed" in response to the helmsman's design read). The 8-file commit at 1af2941f directly executes the design the keeper confirmed.

**Apparatus-tier verification**:

1. **No substrate impact.** gates_pre and gates_post identical (67.6 / 84.8 / 61-51 / TAWR 63 / TAMM 82). Commit touches no `pilots/*/derived/src/` files; cannot regress the substrate.

2. **Ontological consistency.** Helmsman read the apparatus prose for lingering default-helmsman references and verified clean: agent-engagement.md §I, triumvirate ontology §II.2, both engagement docs (substrate-resolver and helmsman), CLAUDE.md / AGENTS.md role-discipline sections, skill README all consistent.

3. **Skill roster integrity.** 4 load skills (helmsman, arbiter, watcher, deputy) for 4 appointed roles; 0 load skills for 1 default role (substrate resolver, since default-role orientation happens via standard CLAUDE.md / AGENTS.md / agent-engagement / engagement-doc-substrate-resolver read on session entry). `.claude/skills/` symlink resolves to apparatus/skills/ canonically.

4. **Existing helmsman appointment continuity.** The principal context's helmsman appointment from Telegram 10202 remains in force for this session; refinement does not retroactively de-appoint. Future fresh-session-entries default to substrate resolver until appointed.

5. **Append-only protocol honored.** No ledgers touched (no entries to add); apparatus discipline doc edits permitted in-place per consolidated-view update protocol. The engagement-doc-helmsman.md header addition preserves prior content; does not retract.

**Apparatus-meta concerns considered**:

- Triumvirate's minimality claim at the governance tier preserved: keeper Rung-2; helmsman + arbiter Rung-1 governance with substrate-active vs apparatus-meta scope. Service-tier (watcher + deputy) extension preserved. The refinement clarifies the helmsman is an appointment-required role within the existing governance structure, not a re-architecting of the structure.
- Keeper's Rung-2 monopoly preserved.
- Pre-push hook coverage check honored (this proposal+decision pair covers the substrate commit's SHA 1af2941f).

**APPROVED for push.**

Archive to `apparatus/proposals/archived/2026-05-28T230000-substrate-resolver-default-refinement/` after the push lands.
