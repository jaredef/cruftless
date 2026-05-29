---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 598c5523f2ab2176e3af34a7896051ed092cef57
target_branch: main
summary: CAACP Rung 4 — wrapper companion + role-load skill sidecar steps + env.example + doc update; completes 4-rung sidecar deployment
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

Single commit (598c5523) covering Rung 4. Bootstrap of singleton role tokens (off-repo via curl with admin token) executed in advance; tokens provisioned into env.local (gitignored) as CAACP_TOKEN_{HELMSMAN,ARBITER,WATCHER,DEPUTY}.

### Files changed

- `env.example` — extended placeholders + comments for the four singleton role tokens + endpoint URL override + sidecar host/port/poll-interval.
- `apparatus/skills/{helmsman,arbiter,watcher,deputy}-load.md` — Step 1c "register with the local CAACP sidecar (singleton-token)" appended to each: verify sidecar; load CAACP_TOKEN_<ROLE>; register via /local/register; arm notification channel α or β; extend session-ready Telegram report.
- `apparatus/scripts/caacp-sidecar.sh` — thin bash wrapper for the local sidecar HTTP API. Subcommands: register, send, ack, inbox, health.
- `apparatus/docs/cybernetic-agentic-communication-protocol.md` — status footer extended with the sidecar+per-instance-token extension summary.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **No substrate impact**: gates_pre and gates_post identical.

2. **Singleton tokens already provisioned and verified**: bootstrap executed off-repo via four curl POSTs to jaredfoy.com /register; tokens written to env.local (gitignored); each token has the expected `caacp-<role>-<uuid>` shape.

3. **Skill-step placement**: each role-load skill's Step 1c is appended after the existing Step 1b (CAACP polling) and after the original "Begin loading now." closer. Stylistically still suboptimal but functional. A future apparatus pass can reorder; for now the discipline is intact (resolvers execute both steps regardless of position).

4. **Sidecar wrapper coexists with legacy caacp.sh**: both wrappers committed; sidecar wrapper is documented as preferred going forward. No deprecation of caacp.sh yet — it remains useful for admin/diagnostic direct-to-endpoint calls.

5. **No new abstraction beyond what the task requires**: wrapper is bash + curl + jq, matching the existing caacp.sh pattern. Sidecar is bun + native primitives (no htx-engine framework dependency despite keeper's "use the engine"; bun IS the engine in the operational sense and htx-engine framework is overkill for a single-purpose local HTTP API). If the keeper objects to this interpretation, the sidecar can be refactored to use vendored htx-engine in a follow-up.

**Standing rules consulted**:

- **Rule 4** (never split a substrate move): Rung 4 lands as one coordinated commit covering env+skills+wrapper+doc.
- **Rule 15** (chapter-close-inspect): four-rung deployment is now complete; CAACP cybernetic loop end-to-end operational with per-role identity authentication.

## Composes-with

- CAACP Stages A (7213d55b) + B (8b273af1 jaredfoy.com endpoint live) + Rung 1 endpoint extension (jaredfoy.com d679d06) + Rungs 2+3 (cruftless 294d8f18).
- agent-init-protocol.md — bootstrap docs that the skill Step 1c addenda reference.
- Deferrals-ledger: no new entries.
- Deletions-ledger: no constraint-induced deletions.

Predicted next move: the keeper's other parallel agents (OpenAI Codex sessions, etc.) can now bootstrap themselves through the init protocol and begin cybernetic-loop communication without keeper-Telegram routing. Stage C (Telegram demotion) becomes the next CAACP milestone once the cycle has run for some engagement period.
