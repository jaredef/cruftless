---
proposal_slug: 2026-05-30T044800Z-caacp-multi-bridge-and-registry
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-keeper-telegram-10516
decided_at: 2026-05-30T04:48:00Z
covers_commits:
  - 6c5a3f194f163ce0d5963860eca58c8cd53289b7
---

## Findings

Approved under explicit keeper directive (Telegram 10516): "First land the updates to the bridge you were going to implement." Helmsman authority covers apparatus-tier changes that consolidate bridge process supervision.

The substrate commit `6c5a3f19` introduces:

1. `apparatus/scripts/caacp-codex-multi-bridge.mjs` — successor to the single-runner `caacp-codex-app-bridge.mjs`. Manages N `{role, instance_id, thread_id}` runners in one process. Per-runner seen-cache + active-directive ledger files preserved verbatim from the single-bridge convention (`bridge-<role>[-<instance>]-codex-app-{seen,active}.json`) so a single-bridge clone can swap to the multi-bridge without losing state. Stop-continue re-injection per the watcher 2026-05-29 design ported unchanged.

2. `apparatus/scripts/caacp-bridge-config.example.json` — JSON config schema example documenting the runner-tuple shape.

3. `apparatus/caacp-server/server.ts` additions — three new local-only endpoints:
   - `POST /local/bridge-announce {host, pid, started_at, runners}` — bridge publishes its managed runners at startup; sidecar persists the record at `apparatus/caacp-server/data/active-bridges/<bridge_id>.json`.
   - `POST /local/bridge-shutdown {host, pid}` — bridge clears the record on graceful exit (SIGTERM/SIGINT).
   - `GET /local/bridges` — surfaces all currently-registered bridges, so the helmsman can answer "what runners exist and which bridge is supervising each one" by `curl http://127.0.0.1:7777/local/bridges` instead of `ps`-archaeology on a remote host.

4. `apparatus/docs/agent-init-protocol.md` reconciled against the actual handler surface: register accepted-fields enumerated (with `callback_url`); send body now documents `sender` (validated against token role) + `target_instance_id` (structurally landed at `31ff99e2`); inbox GET example added; §V.7 bounce-ack discipline re-headed as superseded by the structural fix.

## Verification

1. `node --check apparatus/scripts/caacp-codex-multi-bridge.mjs` — PASS (syntax clean).
2. `bun build apparatus/caacp-server/server.ts --target=bun --no-bundle` — PASS (parses + type-aligns).
3. Endpoints smoke-tested against the currently-running sidecar would require sidecar restart to pick up the new handlers; keeper to restart before the new Claude Code mobile agents come up. Pre-restart, smoke confirmed the routes return the expected `unknown sidecar path` 404 (proving the additions don't affect the running build).

## Scope notes

- The single-bridge `caacp-codex-app-bridge.mjs` is left in place untouched. Migration path: keeper writes a config JSON listing the runners that previously each had a dedicated bridge process, then launches `node apparatus/scripts/caacp-codex-multi-bridge.mjs <config>` instead of N bridge processes.
- The bridge-registry endpoints are local-only (no upstream forwarding). Helmsman inspection is via `curl http://127.0.0.1:7777/local/bridges`. Future enhancement could fold the bridge state into the existing `/local/health` response if cardinality stays small.
- No proposal.md is provided — the work was authorized directly by keeper imperative; the decision.md alone is the audit trail per the same-turn approval convention precedent in `apparatus/proposals/decided/2026-05-29T203200-milf-ext-2-partial-land-node-shims/`.
