---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 2c26757f72f6879562aabe3fa9d358567f6229e5
target_branch: main
summary: CAACP tmux-bridge for Codex (+ similar runtimes) + heartbeat-discipline polling fallback; per watcher's design refinements
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

Single commit (2c26757f) covering the cybernetic-bridge design from helmsman+watcher consultation (Telegram 10270 → 10271 → 10272).

### Bridge script (new file)

`apparatus/scripts/caacp-tmux-bridge.sh` — operator-started shell daemon. Per the watcher's five refinements:
- Polls live `/local/inbox?role=<role>` endpoint (not file).
- Seen-cache at `apparatus/caacp-server/data/bridge-<role>-seen.json`.
- Short directive only (sender/intent/slug, not body).
- Pre-flight tmux target verification + log + non-zero exit on failure.
- Operator-started; not auto-invoked from repo scripts.

### Init protocol §IV.5 + §V (new sections)

- **§IV.5 Heartbeat-discipline polling**: two concrete reliable trigger points (at role-load/session-ready; before outbound CAACP). Explicit non-reliable triggers ("start of each response", "end of each phase") called out as un-enforceable in Codex.
- **§V Cybernetic bridge for Codex**: documents the new tmux-bridge, operator-started discipline, directive format, log location, pre-flight.
- §VI Failure modes + §VII Cross-references renumbered.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **Pane targeting fragility**: `tmux send-keys` requires the target pane to exist + be running the expected agent session. Bridge pre-flights; on missing-session fatal-exits with log. Operator responsibility to start in the right pane.

2. **Injection-as-attack surface**: sending arbitrary text into an interactive pane is powerful. Mitigation: operator-started only (NOT auto-launched from any other script). Directive is fixed-format (`**CAACP NEW** ...`) so the agent can recognize bridge-injections vs human input. Future enhancement could sign the directive with a known token; deferred to v2.

3. **Polling load**: 5s default interval × N bridged roles × persistent loop. Negligible at apparatus scale.

4. **No substrate impact**: gates unchanged.

**Standing rules consulted**:
- **Rule 4**: single coordinated rung.
- **Rule 15**: post-fix verification — bash -n syntax clean; missing-args usage; missing-tmux-session fatal-log behavior.
- **Em-dash restraint**: drafts under target.

## Composes-with

- CAACP four-rung deployment (Rungs 1-4 + body-transmission fix).
- Watcher's cross-machine handshake (fbf348b9 + 3ee9e6ed) that surfaced the Codex notification-primitive gap.
- Init protocol — bridge section becomes the canonical answer for Codex-like runtimes.
- Stage C (Telegram demotion) becomes the next CAACP milestone whenever you direct.
- Deferrals-ledger: no new entries.
- Deletions-ledger: no constraint-induced deletions.

Predicted next move: the keeper or watcher's operator can start the bridge against the watcher's Codex tmux pane, exercising the wake-and-check loop the first time a helmsman→watcher message arrives.
