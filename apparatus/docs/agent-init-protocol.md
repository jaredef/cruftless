# Agent Init Protocol

The bootstrap procedure every LLM resolver session executes on entry to participate in the cybernetic-agentic-communication-protocol (CAACP) loop. Routed from AGENTS.md + CLAUDE.md as required reading.

Per keeper directive Telegram 10252–10257: substrate resolvers register individually (per-instance tokens for parallel-agent coordination); the four appointed governance/service roles (helmsman, arbiter, watcher, deputy) are single-instance and use singleton role tokens.

## I. Roles + initialization paths

| Role                | Default? | Init path                              | Token                                                                                  |
|---|---|---|---|
| Substrate resolver  | Yes      | This doc (§II below)                   | Per-instance, registered on session entry via the local CAACP sidecar                  |
| Helmsman            | No       | `apparatus/skills/helmsman-load.md`    | Singleton `CAACP_TOKEN_HELMSMAN` env var (registered once by keeper)                   |
| Arbiter             | No       | `apparatus/skills/arbiter-load.md`     | Singleton `CAACP_TOKEN_ARBITER`                                                        |
| Watcher             | No       | `apparatus/skills/watcher-load.md`     | Singleton `CAACP_TOKEN_WATCHER`                                                        |
| Deputy              | No       | `apparatus/skills/deputy-load.md`      | Singleton `CAACP_TOKEN_DEPUTY`                                                         |

If you have NOT been appointed to a non-default role via keeper Rung-2 intervention (e.g., "You are the Helmsman."), you are a substrate resolver. Proceed with §II.

## II. Substrate-resolver init (six steps)

### Step 1 — Read CLAUDE.md / AGENTS.md + the apparatus tier on session entry

Per the standing discipline at `apparatus/docs/agent-engagement.md` §VIII. Don't skip; the required reading set is load-bearing.

### Step 2 — Verify the local CAACP sidecar is running

The sidecar runs at `http://127.0.0.1:7777` by default. Check health:

```sh
curl -s http://127.0.0.1:7777/local/health
```

Expected: `{"status":"ok",...}` (HTTP 200).

If the sidecar is NOT running, you have two options:
- **(a) Ask keeper to start it.** Send `**[SUBSTRATE-RESOLVER] CONSULTATION** — CAACP sidecar appears down; please start via `bun apparatus/caacp-server/server.ts` and confirm.` Wait for confirmation before proceeding.
- **(b) Start it yourself if authorized.** Per keeper's standing setup, the sidecar may be expected to run as a background service. Start it in a screen/tmux/nohup'd session:
  ```sh
  cd /path/to/cruftless
  source scripts/env.sh
  nohup bun apparatus/caacp-server/server.ts > /tmp/caacp-sidecar.log 2>&1 &
  ```
  Required env vars: `CAACP_TOKEN_VERIFIER` (admin token, sourced from `env.local`).

### Step 3 — Pick or compute your instance_id

Each substrate-resolver instance needs a stable identifier across the session for inter-agent coordination. Conventions:

- For Claude Code sessions: use the session_id if exposed; else use `claude-code-<short-hostname>-<short-ts>` (e.g., `claude-code-clawd-20260529t0148`).
- For OpenAI Codex sessions: similarly use the codex session id or `codex-<short-hostname>-<short-ts>`.
- For any other agent runtime: any stable string the session can preserve for the duration of its work.

Record the instance_id in your working memory; you'll need it for §VI on close.

### Step 4 — Register with the sidecar

```sh
INSTANCE_ID="<from Step 3>"
# Optional: provide a callback URL the sidecar will POST when new messages arrive.
# If you cannot run an HTTP listener, omit callback_url and rely on the
# notification file at apparatus/caacp-server/data/inbound-substrate-resolver-<instance>.json.

curl -sX POST -H "Content-Type: application/json" \
  -d "{\"role\":\"substrate-resolver\",\"instance_id\":\"$INSTANCE_ID\"}" \
  http://127.0.0.1:7777/local/register
```

Returns:

```json
{
  "token": "caacp-substrate-resolver-<uuid>",
  "role": "substrate-resolver",
  "instance_id": "<your-instance>",
  "sidecar_host": "127.0.0.1",
  "sidecar_port": 7777,
  "notification_file": "/.../apparatus/caacp-server/data/inbound-substrate-resolver-<instance>.json"
}
```

Save the `token` to your session's working memory (do not commit to git; do not log in a way that lands in the repo). Save the `notification_file` path.

### Step 5 — Arm a notification watch (channel α)

For Claude Code sessions, set up a `Monitor` tool watch on the notification file from Step 4. The Monitor delivers `<task-notification>` events to your prompt when the file changes; you read the file's `new_message_ids` array, then GET each message body via `curl http://127.0.0.1:7777/local/inbox?role=substrate-resolver&instance_id=<your-instance>` (or directly from `apparatus/caacp-server/data/inbound-substrate-resolver-<instance>.json`).

For Codex sessions: register a `callback_url` in Step 4 pointing at your local HTTP listener; the sidecar will POST `{role, instance_id, new_message_ids, notification_file}` on every new-message arrival. If you cannot host a listener, fall back to periodic polling of the notification file from your tool's primitive.

### Step 6 — Report session-ready

Send a `**[SUBSTRATE-RESOLVER] INFO**` message to the keeper (via the existing Telegram primitive) reporting:

```
**[SUBSTRATE-RESOLVER] INFO** — session instantiated. instance_id={INSTANCE_ID}.
Registered with CAACP sidecar; notification channel α (file-watch)
[and β (callback_url <url>) if applicable] armed. Awaiting keeper
direction or inter-agent coordination via inbox.
```

Then proceed to your appointed work per the keeper's directive.

## III. Sending CAACP messages

To dispatch to another agent (peer substrate-resolver, helmsman, arbiter, watcher, deputy, keeper):

```sh
# body is the message content; computed sha is handled by the sidecar
curl -sX POST -H "Content-Type: application/json" \
  -d "{
    \"sender_token\": \"$YOUR_TOKEN\",
    \"recipient\": \"<role>\",
    \"intent\": \"request|notification|response|broadcast|veto-pending\",
    \"slug\": \"<short-descriptor>\",
    \"body\": \"<message body as string>\",
    \"related_to\": null
  }" \
  http://127.0.0.1:7777/local/send
```

Returns `{message_id, state: "PENDING", server_timestamp}`. Record the `message_id` if you intend to await an acknowledgment.

To acknowledge a message you received:

```sh
curl -sX POST -H "Content-Type: application/json" \
  -d "{
    \"ack_author_token\": \"$YOUR_TOKEN\",
    \"original_message_id\": \"<from inbox>\",
    \"ack_state\": \"ACKNOWLEDGED|IN-FLIGHT|RESOLVED\",
    \"ack_slug\": \"ack-<original-slug>\",
    \"body\": \"<action taken or planned>\"
  }" \
  http://127.0.0.1:7777/local/ack
```

## IV. Session close

There is no formal de-registration. The sidecar retains your token until its registry is manually pruned by the keeper. If you want explicit close:

- Send `**[SUBSTRATE-RESOLVER] INFO** — session closing. instance_id={INSTANCE_ID}.` to the keeper.
- The keeper may instruct the sidecar to revoke the token via a future apparatus pass; for now the registry accumulates.

Per the CAACP §VI.4 token-discipline carve-outs, rotation + revocation are deferred to future operational-protocol passes; the current state accepts long-lived per-instance tokens.

## IV.5 Heartbeat-discipline polling (fallback for runtimes without async notification)

When the agent runtime does NOT support a file-watch / task-notification primitive AND no external bridge has been started for the session (see §V cybernetic bridge), the apparatus falls back to **heartbeat-discipline polling** at two concrete reliable trigger points:

1. **At role-load / session-ready** — the session's load skill or init protocol already polls inbox; this is the canonical first poll.
2. **Before sending any outbound CAACP message** — before invoking `caacp-sidecar.sh send` or its equivalent, the session first checks `curl http://127.0.0.1:7777/local/inbox?role=<your-role>` and processes any PENDING messages addressed to it. This guarantees that the session's outbound state cannot be authored against a stale inbox view.

"Start of each response" and "end of each substrate-shaped-work phase" are NOT reliable trigger points for runtimes without inline interruption support (a Codex session cannot enforce "start of each response" automatically). Use the two trigger points above; rely on the cybernetic bridge (§V) for everything else.

## V. Cybernetic bridge for Codex (and similar runtimes)

When the agent runtime lacks native file-watch / task-notification (the OpenAI Codex CLI does, as of 2026-05; inotify/fswatch may not be installed; Claude Code's Monitor is not available), the apparatus provides a **terminal-multiplexer bridge** at `apparatus/scripts/caacp-tmux-bridge.sh` that injects a short directive into the tmux pane running the session via `tmux send-keys`.

**Operator-started only**. The bridge is not invoked from any other repo script automatically; injecting text into an interactive pane is powerful and context-sensitive. The keeper (or a helmsman-with-keeper-blessing) starts the bridge for a specific session, in a separate tmux/screen pane:

```sh
apparatus/scripts/caacp-tmux-bridge.sh <role> <tmux-target> [poll-interval]
# example: bash apparatus/scripts/caacp-tmux-bridge.sh watcher codex-watcher:0.0 5
```

The bridge polls `http://127.0.0.1:7777/local/inbox?role=<role>` every poll-interval (default 5s), maintains a seen-cache at `apparatus/caacp-server/data/bridge-<role>-seen.json`, and on new PENDING messages injects:

```
**CAACP NEW** role=<role> count=<N> latest=<sender>/<intent>/<slug>. Check sidecar inbox before continuing.
```

The session interprets the `**CAACP NEW**` prefix as a directive to (a) drop its current micro-step, (b) `curl /local/inbox?role=<role>` to fetch full message metadata + body, (c) decide whether to act on the incoming message or defer per its own discipline. The bridge logs activity to `apparatus/caacp-server/data/bridge-<role>.log`.

Pre-flight: the bridge verifies the tmux target exists before entering its loop; logs and exits non-zero if not.

## VI. Failure modes

- **Sidecar unreachable**: substrate work continues without CAACP coordination; you operate per the legacy artifact-passing convention (write to `apparatus/proposals/`, `apparatus/watcher/notifications/`, etc.) and rely on keeper-Telegram routing per the pre-CAACP discipline.
- **Registration fails with `remote registration failed`**: the sidecar's admin token (`CAACP_TOKEN_VERIFIER`) is unset, expired, or jaredfoy.com is down. Surface to keeper.
- **No callback received + no file-watch update**: either the polling loop is stuck or no messages are arriving. Sanity-check `curl http://127.0.0.1:7777/local/health` for `last_polled_at` per-agent.
- **Token mismatch (403)**: you're sending with a token that doesn't bind to the sender role you're claiming. Re-verify Step 4 output.

## VII. Cross-references

- `apparatus/docs/cybernetic-agentic-communication-protocol.md` — full CAACP articulation (§VI for endpoint surface, §IV for state machine).
- `apparatus/caacp-server/README.md` — sidecar HTTP API + operational notes.
- `apparatus/docs/agent-engagement.md` — substrate-disciplined LLM resolver directions (the broader discipline this init protocol nests within).
- `apparatus/docs/engagement-doc-substrate-resolver.md` — substrate-resolver role frame (what you may and may not do once initialized).
- `apparatus/scripts/caacp.sh` — thin wrapper for direct-to-jaredfoy.com CAACP ops (legacy direct path; the sidecar is the preferred path going forward).
