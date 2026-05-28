# Triumvirate Operational Protocol

The operational specification derived from the ontology in `triumvirate-protocol-keeper-helmsman-arbiter.md` against the affordances and gaps surfaced by `apparatus-audit-for-triumvirate-protocol.md`. Both prior documents are inputs; the protocol's design calls are opinionated, made by the principal context per keeper directive Telegram 10193 ("draft the operational protocol as is coherent against your findings"), and bounded by the carve-outs in the triumvirate doc §VI.

The protocol closes the three CRITICAL/HIGH coordination gaps as required (veto mechanism, escalation channels, resolver-state separation) and the arbiter-handover gap as a coupled concern. The three LOW–MEDIUM observation gaps are scoped to a follow-up audit-and-fill pass per the audit's §VI.

## I. Design principles

Three principles shape every choice below:

1. **The keeper's Rung-2 monopoly is preserved by construction.** No mechanism the protocol introduces can override or simulate keeper intervention. The arbiter's veto is a brake on the helmsman that escalates to the keeper; it is not a substitute for the keeper.
2. **The arbiter's epistemic value depends on real context separation.** The protocol enforces separation mechanically rather than by discipline alone. A discipline-only separation would be indistinguishable from a second helmsman opinion under stress, which is the failure mode the triumvirate exists to prevent.
3. **The veto must be the failure-resistant tier.** The helmsman is the substrate-active resolver; the helmsman is the agent that most often touches what can break. The veto must function even when the arbiter is between instantiations, even when the keeper is asleep, even when the helmsman believes its move is urgent. A pre-push gate is the only architecture that satisfies this; discipline-only veto channels fail exactly when they are needed most.

## II. Veto mechanism

The protocol adopts a **pre-push git hook gated on an arbiter sign-off artifact**. This is the strongest of the three candidates the audit surfaced; the lighter-weight alternatives (PR-required workflow, local proposals directory honored by discipline) fail in the merge-incident failure mode that motivated the triumvirate.

### II.1 Proposal artifact

When the helmsman intends to push commits to `origin/main`, it first writes a proposal manifest to `apparatus/proposals/pending/YYYY-MM-DDTHHMMSS-<slug>.md` with the following structure:

```
---
helmsman_session: <session_id>
proposed_commits: [<sha1>, <sha2>, ...]
target_branch: main
summary: <one-line description>
risk_class: substrate | apparatus | corpus | mixed
gates_pre: { tawr: ..., tamm: ..., diff-prod: ..., test262-sample: ... }
gates_post: { tawr: ..., tamm: ..., diff-prod: ..., test262-sample: ... }
---

## Substrate moves

<per-commit explanation, M-T-I-R per Doc 744 where applicable>

## Risk assessment (helmsman self-evaluation)

<failure modes the helmsman is aware of; standing rules consulted; any negative-result rungs>

## Composes-with

<related arcs, locales, prior rungs, deferrals-ledger entries surfaced or un-deferred>
```

The proposal is committed to the repo as part of the same branch as the changes being proposed; the proposal lives in the working tree the arbiter will inspect.

### II.2 Sign-off artifact

The arbiter, on inspecting a pending proposal, writes one of three response artifacts to `apparatus/proposals/decided/<same-slug>.md`:

- **APPROVED** — the helmsman may push. Body records the arbiter's findings (which standing rules were verified, what apparatus-meta concerns were considered, any qualifications).
- **VETO** — the helmsman may not push. Body records the gap or violation the arbiter identified, the apparatus discipline the proposal would have violated, and the recommended remediation (which the helmsman either implements + re-proposes, or escalates to the keeper for Rung-2 adjudication).
- **DEFER-TO-KEEPER** — the arbiter does not have enough apparatus-meta confidence to approve or veto unilaterally. Body records the dimension the arbiter cannot resolve and routes the proposal to the keeper.

### II.3 Pre-push hook

A git pre-push hook installed at `.githooks/pre-push` enforces the protocol mechanically. The hook:

1. Identifies the commits being pushed (`git rev-list origin/main..HEAD`).
2. Searches `apparatus/proposals/decided/` for an APPROVED artifact whose `proposed_commits` list covers all pushed commits.
3. If found, allows the push and moves the proposal+decision to `apparatus/proposals/archived/`.
4. If not found, prints the gap (which commits are unapproved) and exits non-zero, blocking the push.

The hook is installed by the apparatus's standard tooling-setup script; the repo's `core.hooksPath` config is set to `.githooks/` so contributors who clone the repo have the discipline enforced without per-clone setup. The hook is bypassable with `git push --no-verify`; per CLAUDE.md the helmsman never invokes `--no-verify` without explicit keeper directive (the standing rule against hook-skipping pre-dates this protocol and absorbs this case).

### II.4 Carve-outs from the veto

The veto applies to pushes to `origin/main`. It does not apply to:

- Pushes to feature branches (apparatus permits branch-level work without arbiter sign-off; the gate is at integration to main).
- Local commits (commits to the working tree are reversible; the gate is at pushed-irreversibility per the action-class table in audit §III).
- Documentation-only changes to `docs/engagement/prospective/` (this directory is the keeper's thinking surface per CLAUDE.md; arbiter consultation is non-load-bearing for prospective drafts that have not been promoted).
- Keeper-initiated pushes from the keeper's session (the keeper's Rung-2 monopoly is preserved; the hook may detect a keeper-author-tag and skip).

The carve-outs are themselves apparatus-tier discipline; the arbiter may, in its meta-evaluation, surface any of them for revision as the engagement matures.

## III. Escalation channels and message discipline

The protocol adopts a **single-channel prefix convention with a three-level severity taxonomy**. Separate Telegram channels per role is deferred pending keeper preference; the prefix convention requires zero new infrastructure.

### III.1 Prefix convention

Every Telegram message sent by a resolver to the keeper opens with a role-tag on its own line:

- `**[HELMSMAN]**` — substrate-flow updates, rung-landing notifications, gate reports, deferral emissions, ordinary engagement communication.
- `**[ARBITER]**` — apparatus-meta consultation, veto notifications, methodology-drift observations, periodic apparatus-coherence reports.
- `**[KEEPER]**` (when the keeper sends to a resolver) — directives. The keeper's own tag is implicit in the inbound-from-Telegram channel; the convention is included for symmetry in any future protocol that involves keeper-to-resolver-via-third-party routing.

The principal context, until the keeper appoints a distinct arbiter session, sends as `**[HELMSMAN]**` for substrate work and `**[HELMSMAN/META]**` for the rare apparatus-meta observation it surfaces in lieu of an instantiated arbiter. The dual tag preserves the future arbiter's role-space.

### III.2 Severity taxonomy

After the role tag, every message carries one of three severity markers:

- `INFO` — status update, no keeper action required. Default for helmsman rung-landing reports.
- `CONSULTATION` — the keeper's response would be valuable but is not blocking. Default for arbiter periodic apparatus-coherence reports, deferral-emission notifications, ledger-entry escalations.
- `VETO-PENDING` — keeper Rung-2 adjudication required. Reserved for: an arbiter veto the keeper must adjudicate, a helmsman failure-mode the keeper must override, any state the apparatus cannot resolve at Rung 1.

Severity is encoded as a square-bracket suffix to the role tag on the same line:

```
**[HELMSMAN] INFO** — TAWR-EXT 7 landed (...)
**[ARBITER] CONSULTATION** — methodology-drift observation (...)
**[ARBITER] VETO-PENDING** — proposal-20260528T210000-subarray vetoed (...)
```

### III.3 Bidirectional carve-out

Inbound keeper directives need no prefix; they arrive via the Telegram MCP channel with implicit keeper authority and are addressed to whichever resolver context the keeper sends them to. The resolver responds with its appropriate role-tag.

## IV. Resolver-state separation

The protocol specifies the arbiter's instantiation as a **dedicated Claude Code session with a curated context-load skill**, optionally combined with the Agent tool's `isolation: "worktree"` for substrate-touching arbiter probes. The principal session remains the helmsman; the arbiter is structurally distinct from session entry.

### IV.1 Arbiter-shaped context

The arbiter session loads on entry the following inclusion set:

- `CLAUDE.md` and `AGENTS.md` (apparatus-orientation; both resolvers share the same orientation).
- `apparatus/docs/repository-apparatus.md` (foundational articulation).
- `apparatus/docs/predictive-ruleset.md` (the 15 consolidated rules).
- `apparatus/docs/standing-rule-13-prospective-application.md`.
- `apparatus/docs/agent-feedback-schema.md`.
- `apparatus/docs/arc-as-coordinate.md`.
- `apparatus/docs/orphan-disposition-protocol.md`.
- `apparatus/docs/deferrals-ledger.md`.
- `apparatus/docs/deletions-ledger.md`.
- `apparatus/docs/coverage-gap-orphan-disposition-*.md` (all instances).
- `apparatus/locales/manifest.json` (cross-locale state-at-a-glance).
- `apparatus/locales/CANDIDATES.md` (pre-spawn registry).
- `apparatus/arcs/*/arc.md` (per-arc summaries; the per-arc `log.md` is NOT loaded by default, accessible on demand).
- `docs/engagement/prospective/triumvirate-protocol-keeper-helmsman-arbiter.md` (the ontology the arbiter calibrates against).
- `docs/engagement/prospective/apparatus-audit-for-triumvirate-protocol.md`.
- `docs/engagement/prospective/triumvirate-operational-protocol.md` (this doc).
- `apparatus/proposals/pending/*.md` (the active proposal queue).

The arbiter session does NOT load on entry:

- Per-locale `pilots/*/trajectory.md` (loaded on demand when adjudicating a proposal touching the locale).
- Per-locale `pilots/*/seed.md` (loaded on demand).
- Source files under `pilots/*/derived/src/` (the arbiter is not a substrate-editor; the source is read only when adjudicating a substrate proposal).
- `docs/corpus-ref/` (read only on keeper directive).
- The principal session's conversational history (the arbiter is a clean instance; its only inheritance is the apparatus-tier loaded set above).

The inclusion set is canonicalized in a Claude Code skill at `.claude/skills/arbiter-load.md` (to be authored as part of this protocol's deployment) that the arbiter invokes as the first action of every session.

### IV.2 Instantiation mechanism

The keeper opens an arbiter session by starting a fresh Claude Code instance in the repository root and immediately invoking `/arbiter-load`. The skill reads the inclusion set, summarizes the loaded state, and reports `**[ARBITER] INFO** — session instantiated, N proposals pending review`. The keeper may then either (a) leave the arbiter to process the pending queue and report results, or (b) issue directives to the arbiter for specific apparatus-meta evaluation.

For arbiter probes that require touching the working tree (e.g., reproducing a helmsman gate measurement to verify a proposal's claims), the arbiter invokes the Agent tool with `isolation: "worktree"` to operate on a copy of the repo, preserving the principal helmsman's working tree.

### IV.3 Handover discipline

When the arbiter session approaches its context budget, the arbiter writes a handover summary to `apparatus/docs/arbiter-handover-log.md` (append-only, modeled on findings.md basin-stability discipline) before closing. The summary records:

- Sessions's start + end timestamps.
- Proposals processed (slug + decision).
- Apparatus-meta observations surfaced during the session.
- Open evaluations the next arbiter instance should pick up.
- Methodology-drift indicators noted (with citations).

The next arbiter instance, on `/arbiter-load`, reads the handover-log tail as the last entry of its inclusion set. The handover discipline preserves the apparatus-meta state across instances without conflating it with the helmsman's substrate-tier trajectory tails.

## V. Veto-incident workflow (end-to-end)

The protocol's three mechanisms compose into the canonical veto-incident sequence:

1. **Helmsman lands rungs locally**, writes trajectory entries, runs gates.
2. **Helmsman prepares to push.** Writes proposal manifest at `apparatus/proposals/pending/<slug>.md`. Stages + commits proposal as part of the branch. Attempts `git push origin main`.
3. **Pre-push hook fires.** No APPROVED decision exists. Push blocked.
4. **Helmsman sends Telegram** `**[HELMSMAN] CONSULTATION** — proposal <slug> ready for arbiter review` so the keeper knows an arbiter session is needed.
5. **Keeper instantiates arbiter session.** Arbiter loads via `/arbiter-load`, sees pending proposal, evaluates.
6. **Arbiter writes decision** to `apparatus/proposals/decided/<same-slug>.md` (APPROVED / VETO / DEFER-TO-KEEPER). Sends Telegram `**[ARBITER] INFO/VETO-PENDING** — decision on proposal <slug>: <decision>`.
7. **If APPROVED**: helmsman retries push; hook now finds the decision; push succeeds; proposal+decision archived.
8. **If VETO**: helmsman addresses the gap (revises commits or escalates to keeper). New proposal slug for any revised attempt.
9. **If DEFER-TO-KEEPER**: keeper adjudicates at Rung 2, either issuing a directive that resolves the dimension the arbiter couldn't or overriding both resolvers and redirecting.

## VI. Carve-outs and non-applicability

The protocol does not apply to:

1. **Solo-keeper engagement epochs.** When the keeper directs the engagement personally without delegating to a helmsman, the protocol is suspended; the keeper's Rung-2 authority absorbs all roles.
2. **Pre-instantiation periods.** Until the keeper appoints distinct arbiter sessions, the principal helmsman context is the only resolver. During this period the helmsman self-evaluates proposals at the apparatus-meta tier and sends `**[HELMSMAN/META]**` consultation messages in lieu of arbiter reports. The pre-push hook is not yet installed during this period; the proposal-writing discipline is self-enforced by the helmsman.
3. **Emergency keeper override.** If the keeper directs a push that violates the protocol (e.g., to land a hotfix), the keeper authors the push under their own identity and the hook skips per IV.4's keeper-author-tag carve-out.

## VII. Deployment plan

The protocol becomes load-bearing in three stages:

**Stage 1 — Articulation tier (immediate).** This doc is committed once the keeper approves, alongside the triumvirate ontology and the audit, into `apparatus/docs/` (promoted from `docs/engagement/prospective/`). CLAUDE.md and AGENTS.md required-reading lists are extended to include all three. The helmsman begins self-applying the proposal-writing discipline on the next push.

**Stage 2 — Mechanical-veto tier (when the keeper appoints the first arbiter session).** The `.githooks/pre-push` hook is installed; `core.hooksPath` is configured; the `apparatus/proposals/` directory structure is created. The `.claude/skills/arbiter-load.md` skill is authored. The first arbiter session processes the proposal queue end-to-end as the protocol's first live cycle.

**Stage 3 — Coverage-expansion tier (deferred).** The three LOW–MEDIUM observation gaps from audit §V are filled: rule-drift aggregator at `apparatus/docs/rule-invocation-log.md`, cross-instrument measurement summary at `apparatus/docs/measurement-state.md`, per-arc apparatus-meta summary as a new section in each arc's `arc.md`. These improve the arbiter's read but are not load-bearing for the protocol's correctness; deferred to a follow-up apparatus pass after Stage 2 has run for some engagement cycles.

**Stage 4 — Service-tier activation (when the keeper appoints the first watcher and deputy sessions, per keeper directive Telegram 10211).** Adds the two non-governance resolver roles articulated at `apparatus/docs/service-tier-and-statefulness-protocol.md`. Concretely: (a) `.claude/skills/watcher-load.md` and `.claude/skills/deputy-load.md` skills authored to instantiate each role with its curated context; (b) `apparatus/watcher/{notifications,notifications/closed}/` and `apparatus/deputy/{fleet-state,broadcasts}/` directory structures created; (c) the erasure-stateful surfaces in the freshness protocol's §V.2 inventory each annotated with `(last-refreshed: YYYY-MM-DD)` in the apparatus docs that cite them, beginning the load-bearing freshness annotation discipline; (d) the watcher's polling cadence (§VI.3 of the freshness protocol) configured per surface; (e) the deputy's fleet-state-summary cadence configured per the keeper's fleet-management policy. Telegram conventions extended with `**[WATCHER] INFO**` and `**[DEPUTY] INFO**` role tags. Service-tier roles do not gate pushes (Stage 2 mechanical-veto remains the only push gate); they prevent staleness drift and fleet coordination failures upstream of the push tier.

The keeper's directive on which stages to authorize and on what timeline is the next decision point. The protocol as drafted is internally complete; it can be deployed in full or in stages without redesign. Stage ordering is recommended (1 → 2 → 4 → 3) but not mandatory: Stage 4 service-tier activation depends on Stage 1 articulations being load-bearing and Stage 2 proposal/veto machinery existing, but does not require Stage 3 observation-gap fills.

---

**Status**: PROSPECTIVE — primary articulation per keeper directive Telegram 10193, third sibling to the triumvirate articulation (10189) and the audit (10191); extended with Stage 4 per keeper directive Telegram 10211 (service-tier activation paired with the service-tier-and-statefulness-protocol.md draft). Pending: (1) keeper review and approval of design calls; (2) keeper authorization for Stage 1 promotion to `apparatus/docs/`; (3) keeper appointment of first arbiter session for Stage 2 activation; (4) keeper appointment of first watcher + deputy sessions for Stage 4 activation.

**Promotion**: CANONICAL at apparatus tier 2026-05-28 per keeper directive Telegram 10214. The Stage 1 promotion bundle (9 docs: triumvirate ontology + audit + operational protocol + 5 engagement docs + service-tier-and-statefulness protocol) landed as one coordinated commit. Stage 2 mechanical-veto tier, Stage 3 observation-gap fills, and Stage 4 service-tier activation remain pending keeper appointment of arbiter / watcher / deputy sessions per the operational protocol §VII.
