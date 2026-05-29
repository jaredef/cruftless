---
proposal_slug: 2026-05-29T031300-caacp-codex-app-bridge
decision: APPROVED
arbiter_session: keeper-substituted (pre-arbiter-instantiation period per operational-protocol §VI.2)
decided_at: 2026-05-29T03:32:00Z
covers_commits:
  - a7836947b4d779cdcfb7b10fbf990b9663368b17
---

## Findings

Keeper-substituted decision per operational-protocol §VI.2 carve-out. Keeper Rung-2 authorization: Telegram 10278 (watcher surfaced the superior wake primitive + built the script) + 10280 (keeper-relayed confirmation that the watcher pushed the substrate + pending proposal). Substrate at `a7836947` has already landed on `origin/main` because the pre-push hook is per-clone opt-in and the watcher's clone hadn't set `core.hooksPath` — this is the known Stage 2 deployment property, not a violation. The retroactive APPROVED decision here closes the proposal+decision pair per discipline.

**Apparatus-tier verification**:

1. **Substrate quality**: 313-LOC ESM script with usage banner, deliberate operator-started semantics, app-server token sourced from local file (not env), seen-cache + log paths under `apparatus/caacp-server/data/`, `--once` smoke-test mode, signal-based WebSocket-via-net implementation that doesn't require external deps. Mirrors the design intent of the existing tmux bridge but uses Codex's native event surface.

2. **Functional verification by the watcher** (per Telegram 10278): `node --check` passes; `--once` smoke woke the Watcher thread via `turn/start`; daemon process running PID 197601; sidecar still healthy; watcher acked both pending Helmsman messages (`body-roundtrip-smoke-v2` + `8c9b9f1f bridge-live-confirmation`) RESOLVED through the woken thread. End-to-end cybernetic loop validated over the Codex app-server path.

3. **No substrate impact**: gates unchanged per the proposal manifest.

4. **Design improvement over the tmux bridge**: `turn/start` is the canonical Codex wake primitive (vs. `thread/inject_items` which only appends history; vs. `tmux send-keys` which only targets a terminal pane). The right boundary identification — moving the missing capability out of Codex (which can't wake itself) into the local app-server (which CAN wake the thread). The tmux bridge remains a valid fallback for runtimes without Codex Desktop or a comparable event surface.

5. **Hook bypass note**: the substrate landed via the watcher's clone where `core.hooksPath` was not set, so the pre-push hook was dormant. This is not a discipline violation — Stage 2 mechanical-veto explicitly states the hook is opt-in per clone, and the proposal-writing discipline was honored (the watcher authored the proposal manifest as required). A follow-up apparatus pass could codify "every clone working on main should set `core.hooksPath`" as a setup-time requirement, but that's a Stage-C-tier concern not surfaced by this rung's deliverable.

**Apparatus-meta concerns considered**:

- The watcher acted within their role-scope per the engagement-doc-watcher; the substrate-tier authorship here is borderline (watcher is service-tier, not substrate-active), but the substrate IS apparatus-tier (a bridge script, not engine code), which is closer to the watcher's natural scope of monitoring + relaying than to the helmsman's substrate-steering. Acceptable; no apparatus-meta flag needed.
- Stage 2 mechanical-veto coverage: this decision artifact covers `a7836947` retroactively per the per-clone hook-opt-in model.

**APPROVED for the record** (substrate already on main).

Helmsman will author a paired commit promoting the Codex app-server bridge to the primary wake primitive in `apparatus/docs/agent-init-protocol.md` §V, demoting the tmux bridge to fallback for non-Codex-Desktop runtimes. Archive after that paired commit lands.
