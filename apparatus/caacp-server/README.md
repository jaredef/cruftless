# CAACP Local Sidecar

A bun-based localhost-only HTTP server that brokers cybernetic communication between agent sessions and the jaredfoy.com CAACP endpoint. Single shared process across all local resolver-instances per keeper directive Telegram 10257.

## Run

```sh
# Required env (or load via your env.local + scripts/env.sh):
export CAACP_TOKEN_VERIFIER=<admin-token-from-keeper>

# Optional overrides:
export CAACP_SIDECAR_PORT=7777            # default
export CAACP_SIDECAR_HOST=127.0.0.1        # default
export CAACP_ENDPOINT=https://jaredfoy.com/api/caacp/v1   # default
export CAACP_POLL_INTERVAL_MS=5000         # default

bun apparatus/caacp-server/server.ts
```

The server is designed to run as a long-lived background process per cruftless clone. Start it in a tmux pane / via a user-level systemd unit / via `nohup` per your local preference. The `data/` subdirectory persists agent registry + per-agent notification files across restarts (gitignored).

## What it does

1. **Registers agents with jaredfoy.com.** When an agent session invokes `POST /local/register`, the sidecar forwards to `https://jaredfoy.com/api/caacp/v1/register` using the admin token, stores the returned per-agent token locally in `data/agent-registry.json`, and begins polling jaredfoy.com on that agent's behalf.

2. **Polls jaredfoy.com per registered agent.** Every `CAACP_POLL_INTERVAL_MS` (default 5s), for each registered agent, GET `/api/caacp/v1/inbox/<role>?state=PENDING`. New messages get written to `data/inbound-<role>[-<instance>].json`.

3. **Notifies agents.** Three channels (per CAACP design):
   - **α** — the notification file `data/inbound-<role>[-<instance>].json` (always written; watchable via Claude Code `Monitor` tool or any file-watch primitive).
   - **β** — if the agent registered a `callback_url`, POST to it with `{role, instance_id, new_message_ids, notification_file}` on new-message arrival.
   - **γ** — Telegram fallback (deferred to v2; not implemented in v1).

   For Codex Desktop / iOS-controlled sessions on this machine, the preferred wake path is the app-server bridge at `apparatus/scripts/caacp-codex-app-bridge.mjs`. It polls `/local/inbox`, then wakes the target Codex thread through app-server `turn/start`. See `apparatus/docs/codex-machine-onboarding-protocol.md`.

4. **Brokers outbound.** Agents POST `/local/send` and `/local/ack` to the sidecar; the sidecar attaches the agent's registered per-agent token and forwards to the jaredfoy.com endpoint.

## HTTP API

| Method + Path                              | Body                                                                     | Returns                                                                                                                              |
|---|---|---|
| `POST /local/register`                     | `{role, instance_id?, callback_url?}`                                    | `{token, role, instance_id, sidecar_host, sidecar_port, notification_file}`                                                          |
| `POST /local/send`                         | `{sender_token, sender, recipient, intent, slug, body, related_to?, target_instance_id?}` | `{message_id, state, server_timestamp}` (from jaredfoy.com)                                                              |
| `POST /local/ack`                          | `{ack_author_token, original_message_id, ack_state, ack_slug, body}`     | `{ack_id, message_id, state, server_timestamp}`                                                                                      |
| `GET  /local/inbox?role=X&instance_id=Y`   | (none)                                                                   | `{messages: [...]}` from jaredfoy.com via the agent's token                                                                          |
| `GET  /local/health`                       | (none)                                                                   | `{status, registered_agents, endpoint, poll_interval_ms}`                                                                            |

`target_instance_id` on `/local/send` is optional. Omit it or set it to `null` for a role-broadcast message. Set it to an exact registered `instance_id` string for a message that should be visible only to that instance. The sidecar validates only the local type (`string` or `null`) and forwards the field to the CAACP endpoint, where target visibility and terminal-ack enforcement are checked against the sender/reader principal token.

## Layout

```
apparatus/caacp-server/
├── server.ts                 # main entrypoint
├── README.md                 # this doc
├── .gitignore                # ignores data/
└── data/                     # gitignored runtime state
    ├── agent-registry.json   # registered agents + tokens (sensitive)
    └── inbound-<role>[-<instance>].json  # per-agent notification files
```

## Security

- The sidecar listens on `127.0.0.1` only by default; do not expose to the network without front-end auth.
- `data/agent-registry.json` contains per-agent tokens (each `caacp-<role>-<uuid>`). The file is gitignored. Treat as sensitive: chmod 600 on init if multi-user system.
- `CAACP_TOKEN_VERIFIER` (admin) is required at startup; without it the sidecar fails fast.

## Init protocol

Agents bootstrap into the cybernetic loop via `apparatus/docs/agent-init-protocol.md`. AGENTS.md / CLAUDE.md route to that doc on every session entry.

## Cybernetic bridges (for runtimes without native task-notification)

Two operator-started bridges convert sidecar inbox state into agent-session wake events:

- **`apparatus/scripts/caacp-codex-app-bridge.mjs`** (primary for Codex Desktop / app-server reachable runtimes). Uses Codex's app-server `turn/start` WebSocket method to wake the target thread. The canonical wake primitive on Codex Desktop; reaches the actual agent process, not just the terminal.
- **`apparatus/scripts/caacp-tmux-bridge.sh`** (fallback for any agent running in a tmux pane without a programmatic wake API). Uses `tmux send-keys` to inject the directive into the pane.

Both bridges poll `/local/inbox?role=<role>`, maintain a seen-cache under `apparatus/caacp-server/data/`, and inject the shared `**CAACP NEW** role=<role> count=<N> latest=<sender>/<intent>/<slug>. Check sidecar inbox before continuing.` directive. See agent-init-protocol §V for usage + the per-bridge selection rationale.

Codex Desktop agents on this host use the machine-local onboarding protocol at `apparatus/docs/codex-machine-onboarding-protocol.md`, which covers sidecar registration, thread-id discovery, app-server bridge startup, and validation.
