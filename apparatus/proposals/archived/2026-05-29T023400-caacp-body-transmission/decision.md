---
proposal_slug: 2026-05-29T023400-caacp-body-transmission
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-29T02:34:00Z
covers_commits:
  - eddeb2ec73bc959c6e8621b01c7b33bdb4270bd9
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out. Keeper Rung-2 authorization: Telegram 10268 ("Let's fix the body issues"). The substrate commit at 1d4112bf is the cruftless sidecar's body-forwarding fix; off-repo companion be4daa1 on jaredfoy.com added the server-side body storage + retrieval.

**Apparatus-tier verification**:

1. Roundtrip verified post-fix: POST `{body: "..."}` → 201 message_id → GET returns the exact body string in the row. Tested via curl.
2. Initial migration bug (empty db.exec template after my ALTER statements) was caught by journalctl; fixed in-place (no revert needed; the failure was scoped to the migration extension, not insert/select paths).
3. Backwards-compat preserved: ALTER TABLE ADD COLUMN with try/catch guard handles already-migrated DBs; existing rows retain NULL body.
4. No substrate impact: gates unchanged.

**APPROVED for push.** Archive to `apparatus/proposals/archived/2026-05-29T023400-caacp-body-transmission/` after push lands.
