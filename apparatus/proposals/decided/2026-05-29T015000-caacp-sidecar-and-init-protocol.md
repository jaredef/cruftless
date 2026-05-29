---
proposal_slug: 2026-05-29T015000-caacp-sidecar-and-init-protocol
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-29T01:50:00Z
covers_commits:
  - 294d8f182458b49f4fb85fe0cb3b9b1d6a010989
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out.

Keeper Rung-2 authorization: Telegram 10252 ("Now I want you to put a git ignored htx engine server in cruftless...") + 10255 (3-input refinement) + 10257 ("i" + init protocol in AGENTS.md). The substrate commit at 294d8f18 executes Rungs 2 + 3 of the four-rung deployment under these directives.

**Apparatus-tier verification**:

1. **No substrate impact**: gates_pre and gates_post identical.

2. **Sidecar end-to-end verified** in smoke test: bun process started → agent registered with jaredfoy.com via admin token → per-agent token persisted at `data/agent-registry.json` (gitignored) → test message from helmsman to substrate-resolver via jaredfoy.com → sidecar's 5s polling loop picked it up → notification file written at `data/inbound-substrate-resolver-local-test-001.json` with `new_message_ids` array → log entry confirmed.

3. **Three-channel notification design honored**: α (file-watch) verified; β (callback_url POST) implemented in code path but not exercised in smoke (no callback-registering agent in the test); γ (Telegram fallback) explicitly deferred per the CAACP doc design.

4. **Init protocol covers both Claude Code (Monitor) and Codex (callback_url) agent runtimes** per the keeper's cross-agent requirement; substrate-resolver gets the six-step procedure; appointed roles route to their existing role-load skills.

5. **AGENTS.md + CLAUDE.md route prominently** to the init protocol; declared load-bearing for CAACP loop participation on every session entry.

6. **Rule discipline honored**: Rule 4 (Rungs 2+3 landed together as one coordinated rung; Rung 4 deliberately separated because it depends on these landing first); Rule 15 chapter-close-inspect satisfied via smoke test + doc verification.

7. **Sidecar security**: localhost-only by default; `data/` subdirectory gitignored to prevent token leakage; admin token (`CAACP_TOKEN_VERIFIER`) required at startup with fail-fast behavior.

**Apparatus-meta concerns considered**:

- Singleton role tokens for helmsman/arbiter/watcher/deputy are documented as the next step (registration via admin token) but not yet provisioned. Rung 4 will land that bootstrap.
- The four role-load skills do not yet integrate sidecar registration as part of their session-entry steps. Rung 4 will extend them.
- Stage 2 mechanical-veto coverage: this proposal+decision pair covers the substrate commit's SHA.

**APPROVED for push.**

Archive to `apparatus/proposals/archived/2026-05-29T015000-caacp-sidecar-and-init-protocol/` after push lands.
