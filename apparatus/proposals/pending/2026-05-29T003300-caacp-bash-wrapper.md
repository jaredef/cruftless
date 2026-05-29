---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 6c0409b9299c7737134c5c81ba5f1cc7a2b0dc3c
target_branch: main
summary: apparatus/scripts/caacp.sh — thin bash wrapper for CAACP send/inbox/outbox/ack subcommands
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

Single apparatus-tier commit (6c0409b9). One file added: `apparatus/scripts/caacp.sh` (339 LOC including comments).

### CAACP bash wrapper

- **M** = resolver session invokes `caacp.sh send|inbox|outbox|ack` from the apparatus shell to participate in the CAACP without re-implementing the artifact authorship + endpoint sync logic per session.
- **T** = wrapper covers the four primary CAACP operations end-to-end: write canonical artifact at the right path with the right frontmatter schema (CAACP §III); symlink between inbox and outbox per the convention; POST to endpoint with auth header (Stage B); log sync-failures for replay (degraded mode); parse frontmatter for inbox/outbox listings; preserve message_id reconciliation when Stage B activates.
- **I** = `apparatus/scripts/caacp.sh` (bash) using curl + jq + sha256sum. Endpoint base defaults to `https://jaredfoy.com/api/caacp/v1` per CAACP §VI.1, overridable via `CAACP_ENDPOINT` env var.
- **R** = lattice with the four role-load skills (each invokes `caacp.sh inbox <my-role>` and `caacp.sh outbox <my-role>` per Step 1b polling discipline once they want to use the wrapper rather than direct artifact-read/write).

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **No substrate impact**: gates_pre and gates_post identical because no `pilots/*/derived/src/` modifications. Commit cannot regress the substrate.

2. **Smoke-tested in degraded mode**: end-to-end cycle verified (`send` → `inbox` listing → `ack` → `outbox` listing showing ACK state). All artifacts written to the expected paths; symlinks resolved correctly; sync-failure logs written. Cleanup of smoke-test artifacts verified.

3. **Endpoint-side compatibility**: the POST payload schemas in `cmd_send` and `cmd_ack` match the CAACP §VI.1 endpoint surface as drafted in the canonical doc. If the actual Stage B server-side implementation diverges in field naming or required-field set, the wrapper will surface 4xx errors and log them to sync-failures; not a catastrophic mismatch.

4. **`message_id` reconciliation**: in degraded mode, artifacts start with `message_id: pending-endpoint-assignment` and the wrapper returns `local-only-<slug>` to the caller as a stable identifier. When Stage B activates, fresh artifacts get real server-assigned IDs immediately; legacy degraded-mode artifacts retain the pending marker until a separate backfill pass replays them from `apparatus/caacp/sync-failures/`.

5. **Dependencies**: requires `bash`, `curl`, `jq`, `sha256sum`. All four are present in standard Linux dev environments + the engagement's existing tooling. The wrapper checks for `jq` only when an operation actually requires it (send + ack); inbox + outbox parsing uses awk so they work without jq.

6. **No replacement of existing channels**: the wrapper only writes to `apparatus/caacp/*` paths; it does not touch `apparatus/proposals/`, `apparatus/watcher/notifications/`, or `apparatus/deputy/{fleet-state,broadcasts}/`. The legacy channels per the keeper's directive remain authoritative for their content tiers.

**Standing rules consulted**:

- **Rule 4** (never split a substrate move): single-file commit; not split.
- **Rule 15** (chapter-close-inspect): smoke test verified all four subcommands end-to-end.
- **Em-dash restraint**: drafts kept under target.

## Composes-with

- CAACP Stage A at 7213d55b (apparatus discipline + scaffolding this wrapper operates within).
- CAACP doc §VI.1 (endpoint surface schema the wrapper's POST payloads conform to) + §VI.2 (apparatus convention the wrapper implements: content_sha + write canonical + symlink + POST + patch message_id).
- `apparatus/skills/{helmsman,arbiter,watcher,deputy}-load.md` Step 1b (the wrapper enables their polling instructions to be operationalized via subcommands rather than direct filesystem ops).
- Deferrals-ledger: no new entries.
- Deletions-ledger: no constraint-induced deletions.

Predicted next move on Stage A completion: nothing further needed for Stage A. Stage B activation (endpoint live + `CAACP_TOKEN` provisioned by keeper in `env.local`) makes the wrapper's endpoint-sync paths active immediately; no wrapper changes required.
