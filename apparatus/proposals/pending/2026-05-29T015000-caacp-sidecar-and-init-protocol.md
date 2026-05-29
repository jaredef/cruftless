---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 294d8f182458b49f4fb85fe0cb3b9b1d6a010989
target_branch: main
summary: CAACP Rungs 2-3 — local sidecar at apparatus/caacp-server/ + agent init protocol doc + CLAUDE/AGENTS routing
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

Single commit (294d8f18) covering Rungs 2 + 3 of the four-rung CAACP shared-sidecar + per-instance-token deployment per keeper directive Telegram 10252–10257.

### Rung 2 — local sidecar at apparatus/caacp-server/

- `server.ts` — bun HTTP server on 127.0.0.1:7777, single shared process per keeper's (i) selection. Five-endpoint local HTTP API (`/local/register`, `/local/send`, `/local/ack`, `/local/inbox`, `/local/health`). Background polling loop every 5s per registered agent against jaredfoy.com.
- Three-channel notification on new message arrival: α (file-watchable inbound JSON), β (registered callback URL POST), γ (Telegram fallback deferred).
- `data/` subdirectory gitignored (contains sensitive per-agent tokens + per-agent notification files).
- README + .gitignore committed.

### Rung 3 — agent init protocol doc

- `apparatus/docs/agent-init-protocol.md` — VI sections: role-tokens table, substrate-resolver six-step init, sending messages, session close, failure modes, cross-references.
- CLAUDE.md + AGENTS.md gained prominent "Agent init protocol — read first on session entry" section pointing to the new doc; declared load-bearing for CAACP loop participation.

### What landed in Rung 1 (already committed at jaredfoy.com d679d06)

For context: the jaredfoy.com endpoint extension landed at d679d06 (off-repo for cruftless but part of the coordinated rollout). `/api/caacp/v1/register` (admin-only) + `/api/caacp/v1/tokens?role=X` (admin-only) + `caacp_tokens` table + per-token auth with role-binding enforcement.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **No substrate impact**: gates_pre and gates_post identical. Pilots untouched.

2. **Sidecar smoke-test verified end-to-end**: started in background, registered agent (token from jaredfoy.com received + persisted at data/agent-registry.json), test message sent from helmsman to substrate-resolver via direct curl, sidecar's 5s poll picked it up and wrote `data/inbound-substrate-resolver-local-test-001.json` with the message + `new_message_ids` array. Logged "new messages count: 1". Cleanup verified.

3. **Sidecar requires `CAACP_TOKEN_VERIFIER` at startup**: fails fast if unset. Documented in README. This is the admin token already provisioned at `env.local` for Stage B.

4. **Per-agent tokens are gitignored**: `data/` subdir excluded. Tokens themselves are `caacp-<role>-<uuid>` format generated server-side by jaredfoy.com; never logged in commit messages or tracked artifacts.

5. **Init protocol doc declares CAACP_TOKEN_HELMSMAN / ARBITER / WATCHER / DEPUTY as singleton tokens** but those have NOT yet been registered with jaredfoy.com. Need a follow-up keeper-directed bootstrap that calls `POST /register {role: "helmsman"}` etc. via admin token; each appointed-role session would then load its singleton token from `env.local`. The init protocol doc explicitly notes this as the pattern; Rung 4 will land the wrapper + skill changes to actually use it.

6. **Cross-tool notification design (α + β + γ)**: α verified via the smoke test (file-watch path works). β not exercised yet (no callback_url-registering agent in the smoke). γ deferred to v2 per the design.

**Standing rules consulted**:

- **Rule 4** (never split a substrate move): Rungs 2+3 landed together as one coordinated apparatus rung; Rung 1 was the jaredfoy.com side (off-repo) prior. Rung 4 separates because it depends on Rungs 1–3 being in place + has its own apparatus-meta scope (wrapper/skill integration).
- **Rule 15** (chapter-close-inspect): post-rung verification covered the sidecar end-to-end + init protocol fully written + AGENTS/CLAUDE routing.
- **Em-dash restraint**: drafts under target.

## Composes-with

- CAACP Stage A (apparatus tier at 7213d55b) + Stage B (endpoint live at 8b273af1).
- Rung 1 endpoint extension at jaredfoy.com d679d06.
- Existing `apparatus/scripts/caacp.sh` wrapper (continues to work for direct-to-jaredfoy.com cases; the sidecar becomes preferred path).
- Substrate-resolver engagement doc + agent-engagement doc + role-load skills (Rung 4 will extend the four role-load skills to use the sidecar).
- Deferrals-ledger: no new entries.
- Deletions-ledger: no constraint-induced deletions.

Predicted next move: Rung 4 — extend `apparatus/scripts/caacp.sh` (or add a `caacp-sidecar.sh` companion) to route through localhost sidecar; update the four role-load skills to call `/local/register` + arm notification channel α at session entry; bootstrap the four singleton role tokens via admin token; update operational-protocol + CAACP doc to reflect the sidecar architecture.
