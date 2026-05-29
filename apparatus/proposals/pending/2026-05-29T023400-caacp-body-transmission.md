---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - eddeb2ec73bc959c6e8621b01c7b33bdb4270bd9
target_branch: main
summary: CAACP body-transmission fix — sidecar forwards body to jaredfoy.com endpoint (companion to jaredfoy.com be4daa1)
risk_class: apparatus
gates_pre:
  test262_full: 67.6
  test262_sample: 84.8
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

Single 2-line change in `apparatus/caacp-server/server.ts` (commit 1d4112bf): adds `body: msgBody ?? null` to the POST payload object in `handleSend` and in `handleAck`. Companion to off-repo `jaredfoy.com be4daa1` which added the body storage on the server side.

### Body-transmission gap closure

- **M** = cross-machine CAACP message arrives at recipient sidecar with full body, not just metadata.
- **T** = endpoint `caacp_messages.body` column stores the body string; POST /messages accepts it; GET /messages/{id} + GET /inbox/{role} include it in the row. Sidecar polling carries body in the notification file α (and via callback URL β when registered).
- **I** = (jaredfoy.com side) schema `ALTER TABLE caacp_messages ADD COLUMN body TEXT` (additive, backwards-compat via guarded ALTER); insertMsg/insertAck updated to accept body; body field included in returned rows via SELECT *. (Cruftless side) handleSend/handleAck forward body field in POST payload to jaredfoy.com.
- **R** = lattice with the four-rung CAACP sidecar deployment; closes the gap I flagged in Telegram 10263 ("body not transmitted") that surfaced during the first cross-machine smoke test.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **Initial migration bug + recovery** — first commit attempt left an empty `db.exec(\`\`)` after the ALTER TABLE statements, which crashed the caacp module boot ("Query contained no valid SQL statement; likely empty query"). Module-boot failure surfaced cleanly in journalctl; rest of site kept serving. Fixed by removing the stray empty db.exec; service restarted; verified module boots OK now.

2. **Schema backwards-compat** — existing rows retain NULL body. Existing GET/POST paths continue to work; new body field is additive. Confirmed via successful POST + GET roundtrip post-fix (message_id 921838b6).

3. **No substrate impact**: gates unchanged.

**Standing rules consulted**:
- **Rule 4** (never split a substrate move): single coordinated 2-line change in cruftless paired with the off-repo server-side migration.
- **Rule 13** (revert-then-deeper-layer): the empty-db.exec bug surfaced post-deploy; I fixed in-place rather than reverting because the failure was localized (only the migration extension was broken; insert/select paths were fine) and the fix was 4 LOC. Documented in proposal §1.
- **Rule 15** (chapter-close-inspect): post-fix roundtrip verified (message 921838b6: POST {body:"..."} → 201 → GET returns exact body string).

## Composes-with

- jaredfoy.com be4daa1 (the server-side body storage).
- CAACP four-rung deployment (Rungs 1-4 prior).
- First cross-machine handshake (fbf348b9 + 3ee9e6ed) that surfaced the body gap.
- Future ops: routine inter-agent coordination can now carry full message bodies cross-machine.
- Deferrals-ledger: no new entries (Entry that would have read "cross-machine body transmission" is closed in advance by this rung).
- Deletions-ledger: no constraint-induced deletions.
