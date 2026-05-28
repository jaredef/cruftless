# Apparatus Proposals — Veto Coordination Surface

The directory structure that operationalizes the triumvirate veto workflow per `apparatus/docs/triumvirate-operational-protocol.md` §II. Helmsman writes proposals here; arbiter writes decisions; archived pairs preserve the audit trail.

## Layout

```
apparatus/proposals/
├── pending/   # helmsman has authored; arbiter has not yet decided
├── decided/   # arbiter has written APPROVED / VETO / DEFER-TO-KEEPER
└── archived/  # push has landed; proposal+decision moved here as audit record
```

## Helmsman: proposal authorship (operational protocol §II.1)

Slug format: `YYYY-MM-DDTHHMMSS-<short-descriptor>.md` (UTC timestamp + descriptive slug). Path: `apparatus/proposals/pending/<slug>.md`.

Required frontmatter:

```yaml
---
helmsman_session: <session-id>
proposed_commits:
  - <full-sha-1>
  - <full-sha-2>
target_branch: main
summary: <one-line description>
risk_class: substrate | apparatus | corpus | mixed
gates_pre:
  test262_full: <value-or-null>
  test262_sample: <value-or-null>
  diff_prod: <pass>/<fail>
  per_locale: { <locale>: <value> }
gates_post:
  test262_full: <value-or-null>
  test262_sample: <value-or-null>
  diff_prod: <pass>/<fail>
  per_locale: { <locale>: <value> }
---
```

Required body sections:

- `## Substrate moves` — per-commit explanation, M-T-I-R per Doc 744 where applicable.
- `## Risk assessment (helmsman self-evaluation)` — failure modes the helmsman is aware of; standing rules consulted; any negative-result rungs.
- `## Composes-with` — related arcs, locales, prior rungs, deferrals-ledger entries surfaced or un-deferred.

Commit the proposal as part of the branch being proposed. The proposal commit itself is auto-allowed by the pre-push hook (files are entirely under `apparatus/proposals/`).

## Arbiter: decision authorship (operational protocol §II.2)

Path: `apparatus/proposals/decided/<same-slug>.md` (same slug as the pending proposal it adjudicates).

Required frontmatter:

```yaml
---
proposal_slug: <same-slug-as-pending>
decision: APPROVED | VETO | DEFER-TO-KEEPER
arbiter_session: <session-id>
decided_at: <ISO-8601 timestamp>
covers_commits:
  - <full-sha-1>
  - <full-sha-2>
---
```

The `covers_commits` list must include EVERY commit SHA the pre-push hook will check against; missing SHAs cause the hook to block. The `decision` field must be exactly one of the three values.

Required body sections per decision type:

- **APPROVED**: `## Findings` recording what the arbiter verified (standing rules; apparatus-meta concerns considered; qualifications).
- **VETO**: `## Gap or violation` (the discipline that the proposal would have violated; specific apparatus articulation, standing rule, ledger entry, or prior decision cited); `## Recommended remediation` (what change closes the gap).
- **DEFER-TO-KEEPER**: `## Dimension unresolved` (what the arbiter cannot adjudicate unilaterally and why; routed to keeper for Rung-2 adjudication).

## Pre-push hook coverage check

The hook at `.githooks/pre-push` (active when `core.hooksPath = .githooks`) verifies that every commit being pushed to `refs/heads/main` either:

1. Touches only files within `docs/engagement/prospective/`, `docs/deprecated/`, or `apparatus/proposals/` (carve-out), OR
2. Is listed in the `covers_commits:` field of some file in `apparatus/proposals/decided/` whose `decision:` field is `APPROVED`.

Commits failing both conditions block the push with a clear error message. Bypass: `CRUFTLESS_HOOK_BYPASS=<reason> git push ...` (logged to push transcript; reserved for explicit keeper override).

## Archival

When a proposal's APPROVED decision has been verified by a successful push, the matching files in `pending/` and `decided/` should be moved to `archived/<same-slug>/` (subdirectory per slug containing both the proposal and the decision). Archival is the helmsman's responsibility after a successful push lands.

The archive directory preserves the audit trail: every push to main has, in the archive, the proposal that justified it and the arbiter decision that approved it.

## Activation status

This directory structure is created at Stage 2 of the operational protocol deployment plan. Stage 2 also installs `.githooks/pre-push` and authors `.claude/skills/arbiter-load.md`. Per protocol §VII Stage 2: "the first arbiter session processes the proposal queue end-to-end as the protocol's first live cycle."

Until the keeper appoints the first arbiter session, the helmsman self-enforces the proposal-writing discipline; the arbiter-decision side of the workflow operates by keeper-substituted approval (via Telegram `**[HELMSMAN] CONSULTATION**`).
