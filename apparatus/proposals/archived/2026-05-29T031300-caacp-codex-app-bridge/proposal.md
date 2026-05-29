---
watcher_session: watcher-2026-05-28-codex-desktop
proposed_commits:
  - a7836947b4d779cdcfb7b10fbf990b9663368b17
target_branch: main
summary: CAACP Codex Desktop app-server bridge using turn/start as the canonical wake primitive for Codex Desktop/iOS-controlled sessions
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

Single apparatus commit (`a7836947`) adds `apparatus/scripts/caacp-codex-app-bridge.mjs`, an operator-started bridge from the local CAACP sidecar to a Codex Desktop thread.

The bridge:

- Polls the live sidecar endpoint `/local/inbox?role=<role>`, not the notification file.
- Maintains a role-specific seen-cache at `apparatus/caacp-server/data/bridge-<role>-codex-app-seen.json`.
- Sends a short fixed directive only: `**CAACP NEW** role=<role> count=<N> latest=<sender>/<intent>/<slug>. Check sidecar inbox before continuing.`
- Authenticates to the Codex app-server websocket with the local capability token file (`~/.codex/remote-control/ios-token` by default).
- Resumes the target thread, then starts a real user turn with app-server `turn/start`; this wakes the same Codex Desktop/iOS-controlled thread rather than only appending history or typing into an unrelated terminal pane.
- Logs diagnostics to `apparatus/caacp-server/data/bridge-<role>-codex-app.log`.

## Verification

Performed on the Watcher machine:

```text
node --check apparatus/scripts/caacp-codex-app-bridge.mjs: PASS
apparatus/scripts/caacp-codex-app-bridge.mjs watcher 019e710c-4100-7db2-aff2-b36f3c323848 5 --once: PASS
```

The smoke test successfully woke the Watcher thread for two pending helmsman notifications:

- `921838b6-852a-47bb-bcae-6761eb640a44` (`body-roundtrip-smoke-v2`)
- `8c9b9f1f-4c52-4b2e-b06c-5b412ac6f46e` (`bridge-live-confirmation`)

Follow-on endpoint observation from Helmsman confirmed the messages were resolved by the Watcher, closing the loop over the Codex app-server path.

## Risk assessment

**Thread targeting**: the operator must supply the intended Codex thread id. The bridge logs the target at startup and does not auto-discover or mutate targets.

**Duplicate wakes**: seen-cache prevents repeated wake turns for the same `message_id`. A startup bug found during smoke testing (`[]` appended to an existing cache) was fixed before the script commit.

**Auth scope**: the bridge reads the existing local Codex app-server capability token file and sends only app-server protocol calls to the configured websocket. It does not store or print the token.

**Substrate impact**: none. This is apparatus-tier wake plumbing only; runtime gates are unchanged.

## Composes-with

- CAACP Rungs 1-4 and body-transmission fix.
- `apparatus/scripts/caacp-tmux-bridge.sh`, now demoted in practice to fallback for terminal-only runtimes.
- `apparatus/docs/agent-init-protocol.md` §V, pending Helmsman doc update to promote this bridge as the primary Codex Desktop wake primitive.
- `apparatus/caacp-server/README.md`, pending cross-reference update.

Predicted next move: Helmsman lands doc updates after this script commit reaches `main`.
