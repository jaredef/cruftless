# Cybernetic Agentic Communication Protocol — Working Directory

Inbox / outbox / acknowledgment / archive surfaces for the CAACP per `apparatus/docs/cybernetic-agentic-communication-protocol.md`. Active when Stage A of the CAACP deployment plan is landed (this directory exists from that point); end-to-end cybernetic loop closure requires Stage B (jaredfoy.com endpoint live + `CAACP_TOKEN` provisioned).

## Layout

```
apparatus/caacp/
├── inbox/<role>/         # messages addressed to <role>; polled on session entry
│   ├── helmsman/
│   ├── arbiter/
│   ├── watcher/
│   ├── deputy/
│   └── keeper/
│
├── outbox/<role>/        # messages sent by <role>; polled for unread acknowledgments
│   ├── helmsman/
│   ├── arbiter/
│   ├── watcher/
│   ├── deputy/
│   └── keeper/
│
├── acknowledgments/      # acknowledgment artifacts; cross-referenced by message_id
│
├── archive/<year>/<month>/  # terminal-state messages moved here per housekeeping
│
└── sync-failures/        # endpoint sync failures; replayed on endpoint recovery
```

## Message authorship

Per CAACP §III, every message is a markdown file with frontmatter (`caacp_version: 1`, `message_id`, `sender`, `recipient`, `intent`, `related_to`, `state`, `slug`, `created_at`, `session_id`, `content_sha`, `related_artifacts`, `expires_at`) + body sections (`## Subject`, `## Body`, `## Action requested` for requests/notifications).

The same artifact lives at both `inbox/<recipient>/<slug>.md` (canonical) and `outbox/<sender>/<slug>.md` (symlink to inbox path). When a resolver writes a message:

1. Compute `content_sha = sha256(body)`.
2. Write canonical at `inbox/<recipient>/<slug>.md` + symlink at `outbox/<sender>/<slug>.md`.
3. POST to endpoint per CAACP §VI.2; receive `message_id`; fill frontmatter; re-commit.

## Acknowledgment authorship

Per CAACP §IV, state transitions are recorded as acknowledgment artifacts at `acknowledgments/YYYY-MM-DDTHHMMSS-<message-id>-<state>.md` with frontmatter linking to the original message_id. Acknowledgments are themselves CAACP messages with `intent: acknowledgment`; they don't get acknowledged (they're terminal on write).

## Polling discipline

Each role-load skill's session-entry inclusion set includes `inbox/<role>/` (PENDING + ACKNOWLEDGED) and `outbox/<role>/` (unread acks). The skill reports session-ready with the inbox/outbox counts. See per-skill docs at `apparatus/skills/{helmsman,arbiter,watcher,deputy}-load.md`.

## Legacy channels coexist

Per keeper directive Telegram 10241, legacy artifact channels are preserved:
- `apparatus/proposals/{pending,decided,archived}/` — proposal manifests + decisions (content tier)
- `apparatus/watcher/notifications/` — freshness notifications (content tier)
- `apparatus/deputy/{fleet-state,broadcasts}/` — fleet snapshots + broadcasts (content tier)

The CAACP is the coordination layer **above** these channels. When a watcher writes a notification, the canonical content stays in `apparatus/watcher/notifications/`; the CAACP carries the cybernetic-loop metadata (state transitions, acknowledgments). See CAACP §VIII for the integration points per channel.

## Activation status

Created at Stage A of the CAACP deployment plan per keeper directive Telegram 10241. End-to-end cybernetic loop closure requires Stage B (endpoint live + token provisioned).
