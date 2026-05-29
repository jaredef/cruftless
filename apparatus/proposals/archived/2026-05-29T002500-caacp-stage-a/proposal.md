---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 7213d55b732f23a9b42120603a8eaf7bfb7e3077
target_branch: main
summary: CAACP Stage A — promote articulation + scaffold apparatus/caacp/ + extend role-load skills + env.example + required-reading
risk_class: apparatus
gates_pre:
  test262_full: 67.6
  test262_sample: 84.8 (re-measure pending)
  diff_prod: 61/51
  per_locale:
    TAMM: 82/100
    TAWR: 63/100
    CLFG: 27/32
gates_post:
  test262_full: 67.6 (unchanged; apparatus-tier-only)
  test262_sample: 84.8 (unchanged)
  diff_prod: 61/51 (unchanged)
  per_locale:
    TAMM: 82/100 (unchanged)
    TAWR: 63/100 (unchanged)
    CLFG: 27/32 (unchanged)
---

## Substrate moves

This proposal covers commit 7213d55b, wholly apparatus-tier (no `pilots/*/derived/src/` modifications, no substrate source touched). CAACP Stage A deployment per keeper directive Telegram 10241.

Six concurrent moves:

1. **Promote CAACP doc** from `docs/engagement/prospective/` to `apparatus/docs/cybernetic-agentic-communication-protocol.md`. Status footer updated to CANONICAL + Stage A landing date + Stage B/C pending. Legacy artifact channels preserved per keeper directive.

2. **Create `apparatus/caacp/` scaffolding**: `inbox/<role>/` and `outbox/<role>/` per 5 roles (helmsman, arbiter, watcher, deputy, keeper); `acknowledgments/`; `archive/`; `sync-failures/`. README documents per-directory authorship discipline + reconciliation with legacy channels.

3. **Extend `env.example`** with `CAACP_TOKEN` variable (Stage B endpoint authentication; degraded-mode operation while empty).

4. **Extend four appointed-role load skills** with Step 1b (CAACP inbox + outbox polling). Each role polls its CAACP inbox (PENDING + ACKNOWLEDGED) and outbox (unread acknowledgments) on session entry. Session-ready Telegram reports extended with CAACP counts. Degraded-mode discipline if `CAACP_TOKEN` unset.

5. **Update CLAUDE.md + AGENTS.md** required-reading lists with pointer to the CAACP doc.

6. **Update operational-protocol §VII** to record the parallel CAACP deployment stream + its integration at the proposal+veto workflow.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **No substrate impact**: gates_pre and gates_post identical because no `pilots/*/derived/src/` files are touched. The commit cannot regress the substrate.

2. **Degraded-mode coherence**: Stage A lands BEFORE Stage B (endpoint deployment + token provision). Without `CAACP_TOKEN` set, resolver sessions operate per the artifact-only legacy convention. The four role-load skills explicitly handle this: "If `CAACP_TOKEN` is unset (Stage A degraded mode), the on-disk artifacts ARE the state". No resolver execution breaks if the token is missing.

3. **Skill-extension placement**: each role-load skill's CAACP Step 1b is appended after "Begin loading now." in the existing skills. Placement is suboptimal stylistically (Step 1b reads after the "begin" closer) but is functional — the resolver will execute both steps. A future apparatus pass may reorder the skills for cleaner narrative flow; deferred.

4. **Legacy channel preservation per keeper directive**: explicitly verified that proposals/, watcher/notifications/, and deputy/{fleet-state,broadcasts}/ remain untouched. CAACP coordinates above; legacy artifacts persist as content tier. CAACP doc §VIII documents the integration points.

5. **No new abstraction without need**: Stage A introduces apparatus discipline + filesystem layout + env var contract; it does not introduce the bash-wrapper script `apparatus/scripts/caacp.sh` (deferred to Stage B per CAACP §IX), nor the endpoint client logic (deferred to Stage B). The Stage A landing is the minimum apparatus surface required to make Stage B's endpoint deployment a drop-in addition rather than a redesign.

**Standing rules consulted**:

- **Rule 4** (never split a substrate move): six concurrent moves are one coordinated apparatus-tier rung; splitting would have left the apparatus in an incoherent state (CAACP doc canonical but caacp/ scaffolding missing, etc.).
- **Rule 15** (chapter-close-inspect): post-fix verification — apparatus/caacp/ directory tree present; all four load skills carry Step 1b; env.example has CAACP_TOKEN; CLAUDE.md + AGENTS.md route to the canonical doc; operational-protocol §VII records the parallel stream.
- **Em-dash restraint**: drafts kept under target.

## Composes-with

- CAACP draft at 8cb1a795 (the prospective doc that this Stage A promotes).
- Operational protocol Stages 1/2/4 (apparatus tier this CAACP layers above).
- Legacy channels (proposals/, watcher/notifications/, deputy/{fleet-state,broadcasts}/) preserved as content tier per keeper directive.
- Deferrals-ledger: no new entries.
- Deletions-ledger: no constraint-induced deletions.

Predicted next move: CAACP Stage B (jaredfoy.com endpoint live + CAACP_TOKEN provisioned in env.local) requires keeper Rung-2 work outside this repo. Once Stage B lands, the next routine proposal+veto cycle exercises the CAACP request → arbiter decision → acknowledgment closed-loop path end-to-end.
