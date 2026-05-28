---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - d26fd3666967479a9113f9cab9a7e53f6cf97fa3
target_branch: main
summary: Stage 4 deployment — skills promotion to apparatus/skills/ + watcher-load + deputy-load skills + apparatus/{watcher,deputy}/ scaffolding
risk_class: apparatus
gates_pre:
  test262_full: 67.6 (32460/15537/4933 at test262-full-2026-05-28-123833-p2)
  test262_sample: 84.8 (re-measure pending; pre-arc-closure baseline)
  diff_prod: 61/51
  per_locale:
    TAWR: 63/100
    TAMM: 82/100
gates_post:
  test262_full: 67.6 (unchanged; this is apparatus-tier work, no substrate touch)
  test262_sample: 84.8 (unchanged)
  diff_prod: 61/51 (unchanged)
  per_locale:
    TAWR: 63/100 (unchanged)
    TAMM: 82/100 (unchanged)
---

## Substrate moves

This proposal covers a single commit (d26fd3666967479a9113f9cab9a7e53f6cf97fa3) which is wholly apparatus-tier (no `pilots/*/derived/src/` modifications, no substrate source touched). Two concurrent moves:

### Move 1: Skills promotion to canonical apparatus location

- M: skill discovery for non-helmsman role-loads.
- T: skills live at `apparatus/skills/<role>-load.md` as the canonical apparatus-tracked location; Claude Code finds them via `.claude/skills/` symlink.
- I: `git mv .claude/skills/arbiter-load.md apparatus/skills/arbiter-load.md`; rmdir `.claude/skills/`; `ln -s ../apparatus/skills .claude/skills`. `apparatus/skills/README.md` documents the roster + canonical location + invocation discipline.
- R: lattice with the agent-engagement doc + the operational protocol's §IV.2 instantiation mechanism (both updated to cite the apparatus path with the symlink note).

### Move 2: Stage 4 service-tier activation

- M: keeper appointment of watcher / deputy roles (when keeper opens those sessions in the future).
- T: each role has a load skill that curates context per its role-specific frame; working directory exists with format-documenting README.
- I:
  - `apparatus/skills/watcher-load.md` — watcher-role instantiation; reads engagement-doc-watcher + service-tier-and-statefulness-protocol + erasure-stateful surface inventory; directs initial surface scan + session-ready Telegram.
  - `apparatus/skills/deputy-load.md` — deputy-role instantiation; reads engagement-doc-deputy + helmsman/fleet state + recent broadcasts; directs initial fleet-state snapshot + session-ready Telegram.
  - `apparatus/watcher/{notifications,notifications/closed}/` + README documenting notification authorship per service-tier-and-statefulness-protocol §VI.4 + archival format.
  - `apparatus/deputy/{fleet-state,broadcasts}/` + README documenting fleet-state summary structure + broadcast attribution discipline.
- R: lattice with the triumvirate-operational-protocol §IV.2 (skill paths) + §VII deployment stages (Stage 2 + Stage 4 both updated to LANDED with directive citations).

## Risk assessment (helmsman self-evaluation)

**Failure modes the helmsman is aware of**:

1. **Symlink-not-followed risk.** If Claude Code's skill discovery does not follow symlinks, the skills land at `apparatus/skills/` but are not discoverable. Mitigation: smoke-test via `ls .claude/skills/` confirms the symlink resolves correctly and lists the apparatus-tracked skills. Verified: `ls .claude/skills/` returns `arbiter-load.md` (and watcher-load.md, deputy-load.md). Claude Code's skill-loader typically follows symlinks; if it does not, fallback is to copy rather than symlink, with documentation of the copy-on-clone discipline.

2. **Premature load-bearing of last-refreshed annotations.** Stage 4 element (c) (per service-tier-and-statefulness-protocol §VI.2) calls for `(last-refreshed: YYYY-MM-DD)` annotations on every erasure-stateful citation. This commit does NOT yet apply those annotations across the apparatus docs; that is deferred to the first watcher session per the stage 4 spec. No drift introduced.

3. **No substrate gate impact.** This commit is wholly apparatus-tier; gates_pre and gates_post are identical because no `pilots/*/derived/src/` files are touched. Build is not re-required; this commit cannot regress the substrate.

**Standing rules consulted**:

- Rule 13 (revert-then-deeper-layer): no rung here, not applicable.
- Rule 15 (chapter-close-inspect): this commit closes Stage 4's deployment-tier "chapter"; inspection passes — symlink resolves; skills load; working directories exist with READMEs.
- Em-dash restraint (CLAUDE.md): drafts kept under target.
- Append-only discipline (Doc 727 §X): triumvirate-operational-protocol's Stage 2 and Stage 4 sections were edited in place to record landed status, which is permitted on apparatus discipline docs (they are consolidated-view per the operational protocol's update protocol; not append-only).
- Carve-out for IR-generated files: not applicable (no IR-generated files touched).

**Composes-with**: the substrate moves compose with Stage 1 (apparatus articulations) and Stage 2 (proposal+veto mechanism); they activate the Stage 4 service-tier described in service-tier-and-statefulness-protocol.md (promoted in the Stage 1 bundle).

## Composes-with

- Stage 1 promotion bundle at d804777e (9 docs landed; included service-tier-and-statefulness-protocol.md which articulates the watcher + deputy roles this Stage 4 deployment activates).
- Stage 2 deployment at 95b7ba80 (pre-push hook + apparatus/proposals/ + .claude/skills/arbiter-load.md — the latter now relocated to apparatus/skills/ per this commit).
- Keeper directive Telegram 10219 — appointed the skills relocation + Stage 4 activation.
- Deferrals-ledger: no new entries; no new candidates surfaced this round.
- Deletions-ledger: no constraint-induced deletions; the .claude/skills/arbiter-load.md "removal" is a relocation, not a deletion.
