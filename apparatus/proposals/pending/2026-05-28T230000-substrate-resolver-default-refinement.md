---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 1af2941f1dc76d8c7d05d672c08cc434ae6578f5
target_branch: main
summary: Substrate-resolver-as-default ontological refinement; helmsman becomes appointed role; helmsman-load skill completes 4-of-4 appointed roster
risk_class: apparatus
gates_pre:
  test262_full: 67.6 (32460/15537/4933 at test262-full-2026-05-28-123833-p2)
  test262_sample: 84.8 (re-measure pending)
  diff_prod: 61/51
  per_locale:
    TAWR: 63/100
    TAMM: 82/100
gates_post:
  test262_full: 67.6 (unchanged; apparatus-tier-only)
  test262_sample: 84.8 (unchanged)
  diff_prod: 61/51 (unchanged)
  per_locale:
    TAWR: 63/100 (unchanged)
    TAMM: 82/100 (unchanged)
---

## Substrate moves

This proposal covers commit 1af2941f1dc76d8c7d05d672c08cc434ae6578f5, wholly apparatus-tier (no `pilots/*/derived/src/` modifications, no substrate source touched). One ontological refinement landing across 8 files:

### The refinement (per keeper directive 10225 articulation + 10226 confirmation)

- M: an LLM resolver entering this engagement.
- T: default role is substrate resolver (worker tier, no governance authority); helmsman / arbiter / watcher / deputy are all appointed roles requiring explicit keeper Rung-2 intervention.
- I: the file changes below.
- R: lattice across the triumvirate ontology, the operational protocol, the agent-engagement doc, the per-role engagement docs, the skill roster, and the CLAUDE.md / AGENTS.md role-discipline section.

### Files changed

1. **`apparatus/docs/engagement-doc-substrate-resolver.md`** (new) — 8-section symmetric skeleton matching the other engagement docs. Defines default scope: what substrate resolvers may do (substrate edits within appointed scope; trajectory authorship; ledger appends within scope; local commits; subagent dispatch within scope; gate measurement; per-locale resume; status reporting). What they may NOT do (push to main; author push-tier proposals; coordinate parallel resolvers/fleet; decide arc rotation; adjudicate apparatus discipline; veto; promote docs to apparatus; edit corpus; claim epistemic priority over telos). 5 failure modes, with #1 (drifting into helmsman scope without appointment) called out as the most common drift.
2. **`apparatus/skills/helmsman-load.md`** (new) — completes the 4-of-4 appointed-roles skill roster. Loads the broadest inclusion set. Carries appointment-required header explicit at the top.
3. **`apparatus/docs/engagement-doc-helmsman.md`** — header updated to make appointment-required explicit; cites Telegram 10225–10226.
4. **`apparatus/skills/README.md`** — roster updated: 4 load skills for the 4 appointed roles; substrate resolver does not have a load skill.
5. **`apparatus/docs/agent-engagement.md`** §I — five roles enumerated; "you are the helmsman" → "you are the substrate resolver".
6. **`apparatus/docs/triumvirate-protocol-keeper-helmsman-arbiter.md`** §II.2 — title clarifies "(appointed)"; final paragraph cites Telegram 10225–10226 and points to engagement-doc-substrate-resolver.md.
7. **`CLAUDE.md` + `AGENTS.md`** §"Resolver role discipline" — substrate resolver as default; all four named roles require appointment; explicit examples of appointment text.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **Ontological consistency across docs.** The refinement touches 7 docs + the resolver-role-discipline sections in CLAUDE.md / AGENTS.md. Risk: lingering references to "helmsman is the default" elsewhere in apparatus prose. Mitigation: grep audit — searched for "helmsman is the default" / "default role" / "by default" across `apparatus/docs/` and `CLAUDE.md` / `AGENTS.md`; all surfaced references have been updated to substrate-resolver-as-default.

2. **Existing session continuity.** The principal context (this conversation) was appointed helmsman by keeper Telegram 10202 ("You are now the helmsman by keeper intervention"). That appointment remains in force for this session; this refinement does not retroactively de-appoint. The refinement applies to future fresh-session-entries, where the new default is substrate resolver.

3. **Skill discoverability for helmsman-load.** New skill at `apparatus/skills/helmsman-load.md` visible via `.claude/skills/` symlink (confirmed `ls .claude/skills/` returns helmsman-load.md alongside the other three).

4. **No substrate impact.** This commit is wholly apparatus-tier; gates_pre and gates_post identical because no `pilots/*/derived/src/` files are touched.

**Standing rules consulted**:

- Rule 13 (revert-then-deeper-layer): no rung; not applicable.
- Rule 15 (chapter-close-inspect): this commit closes the "substrate-resolver-as-default refinement" chapter. Post-fix inspection: grep audit clean; all 5 engagement docs (substrate-resolver, helmsman, arbiter, watcher, deputy) carry the appointment-or-default framing consistently; 4 load skills aligned with 4 appointed roles + 0 load skills for the 1 default role.
- Em-dash restraint: drafts kept under target.
- Append-only discipline (Doc 727 §X): triumvirate ontology + agent-engagement + skill README are consolidated-view (not append-only); in-place edits permitted. The helmsman engagement doc's appointment-required header is an in-place addition that does not retract any prior content; the prior content remains valid for the in-force helmsman appointment.

**Composes-with**:

- Stage 1 promotion bundle at d804777e (the original triumvirate ontology + the original engagement docs that this refinement adjusts).
- Stage 2 deployment at 95b7ba80 (pre-push hook active for this push).
- Stage 4 deployment at d26fd366 (apparatus/skills/ canonical location + watcher/deputy load skills the new helmsman-load skill is parallel to).
- Keeper directive Telegram 10225 (articulation) + 10226 (confirmation).
- Deferrals-ledger: no new entries; no new candidates surfaced.
- Deletions-ledger: no constraint-induced deletions.
