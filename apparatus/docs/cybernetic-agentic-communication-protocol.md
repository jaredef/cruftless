# Cybernetic Agentic Communication Protocol (CAACP)

A protocol for closed-loop inter-resolver communication in the Cruftless apparatus. Replaces the current implicit pattern (scattered drop-box artifact passing with keeper-routed Telegram as the only synchronization point) with an explicit cybernetic system: agents send addressed messages, observe acknowledgments, and adjust their next move based on the apparatus's feedback. Drafted per keeper directive Telegram 10239.

The protocol uses a remote enforcement endpoint at jaredfoy.com authenticated by an `.env`-provided token, so cybernetic state-machine transitions are observable in near-real-time across resolver sessions without requiring keeper-Telegram-attention as the synchronization bottleneck.

This doc sits alongside `triumvirate-protocol-keeper-helmsman-arbiter.md` (ontology), `triumvirate-operational-protocol.md` (operational spec), and `service-tier-and-statefulness-protocol.md` (statefulness + watcher/deputy). The CAACP closes the inter-resolver coordination gap those three docs articulated but did not fully operationalize.

---

## I. Motivation

The current apparatus has artifact-passing channels:

- Helmsman → Arbiter via `apparatus/proposals/pending/` ← `apparatus/proposals/decided/`
- Watcher → Helmsman via `apparatus/watcher/notifications/`
- Deputy → Helmsman via `apparatus/deputy/{fleet-state,broadcasts}/`

Each channel is a one-way drop-box. Sender writes; receiver may or may not read on its next session entry; sender has no protocol-level observation of "did the message land, did it get acted on, what was the outcome". Reverse-flow acknowledgments happen by convention (proposal+decision pair archived together post-push) rather than by enforced protocol.

The single synchronization point that closes inter-resolver loops today is the keeper, via Telegram (`**[HELMSMAN] CONSULTATION** — proposal ready for arbiter review`). This makes the keeper the coordination bottleneck and undermines the resolver autonomy the triumvirate was meant to enable.

The CAACP closes the loop. Its load-bearing properties:

1. **Per-role unified inbox + outbox**. Each role polls its own inbox on session entry and its own outbox for unread acknowledgments. Discovery is uniform; routing is by recipient field.
2. **Explicit state machine per message**. PENDING → ACKNOWLEDGED → IN-FLIGHT → RESOLVED → ARCHIVED. Transitions are observable both on disk (the artifact bodies persist in repo) and through the remote enforcement endpoint (the state machine lives there).
3. **Acknowledgment as protocol primitive**. Receivers must acknowledge messages; senders see the acknowledgment without keeper relay.
4. **Intent typology**. Messages carry `intent: request | notification | response | broadcast | acknowledgment | veto-pending`, each with its own state-machine constraints.
5. **Remote enforcement endpoint** authenticated via `.env` token: provides cybernetic real-time state tracking; serves as the single source of truth for state-machine transitions across resolver sessions.
6. **Telegram demoted to escalation tier**. Routine inter-role communication uses CAACP; Telegram is reserved for keeper Rung-2 adjudication and engagement-level cross-cosmos reporting.

---

## II. Filesystem layout

```
apparatus/caacp/
├── inbox/                                      # incoming messages per recipient role
│   ├── helmsman/
│   │   └── YYYY-MM-DDTHHMMSS-<sender>-<slug>.md
│   ├── arbiter/
│   ├── watcher/
│   ├── deputy/
│   └── keeper/                                 # for messages explicitly addressed to keeper
│
├── outbox/                                     # sent messages per sender role
│   ├── helmsman/
│   ├── arbiter/
│   ├── watcher/
│   ├── deputy/
│   └── keeper/
│
└── acknowledgments/                            # acknowledgment artifacts (linked by message_id)
    └── YYYY-MM-DDTHHMMSS-<message-id>-<state>.md
```

Each message body lives at both `inbox/<recipient>/<slug>.md` and `outbox/<sender>/<slug>.md` (symlink one direction to preserve a single source of truth in the working tree; recommended: inbox/ is canonical, outbox/ is symlinked). Acknowledgments are distinct artifacts referenced from both message records.

The existing channels (proposals, watcher notifications, deputy fleet-state) are preserved as their own tier of artifact-passing for content that doesn't naturally fit the CAACP message shape (e.g., proposal manifests are long-form and carry their own pre-push-hook gating; fleet-state summaries are snapshots not messages). The CAACP is the cybernetic layer **above** the existing channels: when a proposal is ready for arbiter review, the helmsman drops a CAACP `request` into the arbiter's inbox pointing at the proposal artifact. The proposal artifact remains in `apparatus/proposals/pending/`; the CAACP message is the cybernetic notification + acknowledgment trace.

---

## III. Message schema

Every CAACP message is a markdown file with frontmatter + body. Schema:

```yaml
---
caacp_version: 1
message_id: <uuid-v7-or-server-assigned>
sender: helmsman | arbiter | watcher | deputy | keeper
recipient: helmsman | arbiter | watcher | deputy | keeper
intent: request | notification | response | broadcast | acknowledgment | veto-pending
related_to: <message-id-this-responds-to>     # null for fresh messages; required for response/acknowledgment
state: PENDING | ACKNOWLEDGED | IN-FLIGHT | RESOLVED | ARCHIVED
slug: <short-descriptor>                       # human-readable for cross-references
created_at: <ISO timestamp>
session_id: <sender's session-id>
content_sha: <sha256 of body>                  # for endpoint integrity verification
related_artifacts:                              # optional: paths in repo this message references
  - apparatus/proposals/pending/<slug>.md
  - pilots/<locale>/trajectory.md
expires_at: <ISO timestamp>                     # optional; messages past expiry auto-archive
---

## Subject

<one-line summary that appears in inbox listings>

## Body

<message content; markdown allowed; what the receiver needs to know>

## Action requested

<what the receiver should do; absent for notifications/broadcasts>
```

For acknowledgment messages specifically:

```yaml
---
caacp_version: 1
message_id: <new-uuid>
sender: <ack-author-role>
recipient: <original-sender-role>
intent: acknowledgment
related_to: <message-id-being-acknowledged>
state: ACKNOWLEDGED | IN-FLIGHT | RESOLVED   # the state the original message transitions to
slug: ack-<original-slug>
created_at: <ISO>
session_id: <ack-author-session>
content_sha: <sha256>
---

## Acknowledged

<message-id> from <original-sender>, intent <original-intent>, slug <original-slug>.

## Action taken

<what the receiver did or will do; "noted, will action on next session entry" is a valid response>
```

---

## IV. State machine per message

```
                        ┌──────────────┐
       (sender posts)──→│   PENDING    │
                        └──────┬───────┘
                               │ (recipient writes acknowledgment with state=ACKNOWLEDGED)
                               ▼
                        ┌──────────────┐
                        │ ACKNOWLEDGED │
                        └──────┬───────┘
                               │ (recipient begins work; writes acknowledgment with state=IN-FLIGHT)
                               ▼
                        ┌──────────────┐
                        │  IN-FLIGHT   │
                        └──────┬───────┘
                               │ (recipient completes work; writes acknowledgment with state=RESOLVED)
                               ▼
                        ┌──────────────┐
                        │   RESOLVED   │
                        └──────┬───────┘
                               │ (sender confirms; or expiry; or keeper directive)
                               ▼
                        ┌──────────────┐
                        │   ARCHIVED   │
                        └──────────────┘
```

Transitions are recorded both as acknowledgment artifacts in `apparatus/caacp/acknowledgments/` AND as state changes at the remote endpoint (per §VI). The artifact is the persistent record; the endpoint is the cybernetic loop.

ACKNOWLEDGED can transition directly to RESOLVED when the action is instant (e.g., a notification that the receiver simply reads and acts on immediately). The intermediate IN-FLIGHT state is for work that takes time across multiple session entries.

The terminal state ARCHIVED means the message's coordination work is complete; the artifact may be moved to `apparatus/caacp/archive/<year>/<month>/`. Archival is the helmsman's housekeeping responsibility (or any role's, per session).

---

## V. Intent typology

| Intent | Semantics | Acknowledgment required | Default state-machine path |
|---|---|---|---|
| `request` | sender asks recipient to do work | yes | PENDING → ACK → IN-FLIGHT → RESOLVED |
| `notification` | sender informs recipient of an event | yes (lightweight ACK only) | PENDING → ACK → RESOLVED |
| `response` | sender answers a prior `request` | yes (closes the request) | PENDING → ACK → RESOLVED, also transitions related request to RESOLVED |
| `broadcast` | sender informs all of a class (e.g., the fleet) | no (broadcast is fire-and-forget) | PENDING → ARCHIVED (no ACK collected) |
| `acknowledgment` | meta-message that records a state transition | no (acknowledgments don't get acknowledged) | always RESOLVED on write |
| `veto-pending` | helmsman or arbiter escalates a blocked move to keeper | yes (keeper directive required) | PENDING → ACK → RESOLVED (with keeper directive content) |

The state machine constraints differ slightly by intent: `broadcast` skips the ACK requirement; `acknowledgment` is itself the meta-message that transitions other messages.

---

## VI. Remote enforcement endpoint

The protocol's cybernetic loop closes through an API at `https://jaredfoy.com/api/caacp/v1/`. Authentication is via `X-CAACP-Token: <token>` header, with the token loaded from `.env` (variable name: `CAACP_TOKEN`). The token's value lives in `env.local` per the standing env-discipline; `env.example` documents the contract.

### VI.1 Endpoint surface

```
POST   /api/caacp/v1/messages
       Body: { sender, recipient, intent, slug, related_to?, content_sha, related_artifacts? }
       Returns: { message_id, state: "PENDING", server_timestamp }

GET    /api/caacp/v1/inbox/{role}?state=PENDING|ACKNOWLEDGED|IN-FLIGHT
       Returns: [{ message_id, sender, intent, slug, created_at, state, content_sha }]

GET    /api/caacp/v1/outbox/{role}?unread_acks=true
       Returns: [{ message_id, recipient, intent, slug, state, last_ack_at }]

POST   /api/caacp/v1/messages/{message_id}/acknowledge
       Body: { ack_author, ack_intent: ACKNOWLEDGED|IN-FLIGHT|RESOLVED, ack_slug, content_sha }
       Returns: { state, ack_id, server_timestamp }

GET    /api/caacp/v1/messages/{message_id}
       Returns: full message record including all acknowledgments + state transitions
```

### VI.2 Apparatus convention

When a resolver writes a CAACP message artifact:

1. Compute `content_sha = sha256(body)`.
2. Write the artifact to `apparatus/caacp/inbox/<recipient>/<slug>.md` (+ symlink to outbox).
3. POST to `/api/caacp/v1/messages` with the message metadata + content_sha.
4. Receive `message_id` from server; fill into the artifact's frontmatter.
5. Re-commit the artifact (the message_id was server-assigned).

When a resolver polls its inbox on session entry:

1. GET `/api/caacp/v1/inbox/<my-role>?state=PENDING` → list of pending message_ids.
2. For each, read the artifact at `apparatus/caacp/inbox/<my-role>/<slug>.md` and verify content_sha matches.
3. Triage; act; write acknowledgment artifact; POST acknowledgment to the endpoint.

The endpoint is authoritative for state machine transitions. The repo artifacts are authoritative for message content. The two are reconciled via content_sha verification; mismatches surface as `**[<role>] INFO**` Telegram alerts.

### VI.3 Degraded-mode fallback

If the endpoint is unreachable (network down, token misconfigured, server failing), the apparatus degrades gracefully:

1. Resolver writes the artifact normally.
2. POST to endpoint fails; resolver logs the failure to `apparatus/caacp/sync-failures/YYYY-MM-DDTHHMMSS-<slug>.md`.
3. Resolver still surfaces the message via the legacy artifact-only path (the recipient role's inbox directory).
4. Receiver reads the artifact on next session entry per the legacy convention; writes ACK artifact normally.
5. On endpoint recovery (next successful POST), the resolver replays its sync-failure log to backfill state-machine transitions.

Degraded mode preserves the existing artifact-passing channels as the underlying transport; the endpoint is the cybernetic accelerator, not a single point of failure for the apparatus.

### VI.4 Token discipline

`CAACP_TOKEN` is set in `env.local` per the standing env-discipline (CLAUDE.md §"Operational quick-reference" + the scripts/env.sh loader). The token is shared across all resolver sessions on the same machine; each session loads it on entry via `source scripts/env.sh`. The token is keeper-managed; rotation is keeper Rung-2 work.

`env.example` is updated to document `CAACP_TOKEN=<your-token-here>` per the contract. The actual token value never lands in git; `env.local` is gitignored.

---

## VII. Per-role polling discipline

Each role-load skill's session-entry inclusion set extends to include the CAACP inbox/outbox. The polling discipline:

### Helmsman (`/helmsman-load`)
- Poll `apparatus/caacp/inbox/helmsman/` (PENDING + ACKNOWLEDGED).
- Poll `apparatus/caacp/outbox/helmsman/` for unread acknowledgments on messages I sent.
- Report at session-ready: `**[HELMSMAN] INFO** — N pending inbox, K unread acks in outbox`.

### Arbiter (`/arbiter-load`)
- Poll `apparatus/caacp/inbox/arbiter/` — pending requests from helmsman (typically "review pending proposal <slug>").
- Cross-reference each request's `related_artifacts` against `apparatus/proposals/pending/` for the proposal manifest.
- Report at session-ready: `**[ARBITER] INFO** — session instantiated. N proposal-review requests pending. K decisions outstanding from prior sessions`.

### Watcher (`/watcher-load`)
- Poll `apparatus/caacp/inbox/watcher/` — typically requests from helmsman to remeasure specific surfaces.
- Poll `apparatus/caacp/outbox/watcher/` for unread acks on prior notifications.
- Report at session-ready alongside the existing erasure-state surface scan.

### Deputy (`/deputy-load`)
- Poll `apparatus/caacp/inbox/deputy/` — typically broadcast requests from helmsman ("relay this to the fleet").
- Author broadcasts to `apparatus/deputy/broadcasts/` per the existing channel; emit CAACP `acknowledgment` (state=RESOLVED) once broadcast lands.
- Report at session-ready alongside the existing fleet-state snapshot.

### Keeper
- Keeper polling is optional (the keeper reads via terminal session or Telegram); when active, keeper may poll `apparatus/caacp/inbox/keeper/` for messages explicitly addressed to keeper.
- Keeper-tier escalations (`veto-pending` intent) also surface via Telegram per the existing escalation discipline; the dual channel is intentional (the cybernetic loop captures the state machine; Telegram captures the keeper's attention).

---

## VIII. Operational protocol integration

The CAACP slots into the existing operational protocol at three points:

### VIII.1 Proposal+veto workflow extension

When the helmsman writes a proposal at `apparatus/proposals/pending/<slug>.md`, the helmsman ALSO writes a CAACP request:

```yaml
---
caacp_version: 1
recipient: arbiter
intent: request
slug: review-<proposal-slug>
related_artifacts:
  - apparatus/proposals/pending/<slug>.md
---

## Subject
Proposal <slug> ready for arbiter review

## Action requested
Adjudicate (APPROVED / VETO / DEFER-TO-KEEPER) per operational-protocol §II.2.
```

The arbiter's next session-entry inbox poll surfaces this request. The arbiter writes the decision at `apparatus/proposals/decided/<slug>.md` AND a CAACP acknowledgment (state=RESOLVED) referencing the original request. The helmsman's outbox poll on its next push attempt surfaces the acknowledgment; the pre-push hook continues to check `apparatus/proposals/decided/` for the APPROVED record.

This replaces the current `**[HELMSMAN] CONSULTATION** — proposal ready for arbiter review` Telegram, removing the keeper from the routine routing loop.

### VIII.2 Watcher notification ACK closure

When the watcher writes a notification at `apparatus/watcher/notifications/<slug>.md`, the watcher ALSO writes a CAACP notification:

```yaml
intent: notification
recipient: helmsman
slug: staleness-<surface>
related_artifacts:
  - apparatus/watcher/notifications/<slug>.md
```

The helmsman acknowledges (state=ACKNOWLEDGED), works the refresh, and writes a second acknowledgment (state=RESOLVED) when the refresh commit lands. The watcher's outbox poll on its next session-entry sees both acknowledgments and moves the notification artifact to `apparatus/watcher/notifications/closed/` per the existing archival convention.

### VIII.3 Deputy broadcast request

When the helmsman wants the deputy to broadcast something to the fleet, the helmsman writes a CAACP request:

```yaml
intent: request
recipient: deputy
slug: broadcast-<topic>

## Subject
Please broadcast the following to the fleet

## Body
<verbatim message text>

## Action requested
Author broadcast at apparatus/deputy/broadcasts/<topic>.md with verbatim attribution.
```

The deputy reads on next session entry, authors the broadcast, writes the CAACP acknowledgment (state=RESOLVED). This closes the apparatus gap I flagged earlier (the helmsman-to-deputy direction was implicit; this makes it explicit through CAACP).

---

## IX. Deployment plan

Three stages, modeled on the operational protocol's deployment plan:

### Stage A — Articulation tier (this doc)
Promote this doc + the operational protocol's CAACP-integration updates to `apparatus/docs/`. CLAUDE.md / AGENTS.md required-reading lists extended. `apparatus/caacp/` directory structure created. Per-role load skills extended to include CAACP inbox/outbox polling. `env.example` updated with `CAACP_TOKEN=`. Zero net-new infrastructure beyond apparatus articulation.

### Stage B — Endpoint deployment
Endpoint deployed at `jaredfoy.com/api/caacp/v1/` per §VI.1 schema. Keeper provisions the token, populates `env.local` with `CAACP_TOKEN=<value>`, distributes to active resolver clones as needed. Resolvers' Bash tool surface used to invoke `curl` for endpoint interactions (or a thin `apparatus/scripts/caacp.sh` wrapper that handles the auth header + JSON encoding). First end-to-end CAACP cycle exercises the proposal+veto workflow via §VIII.1 path.

### Stage C — Telegram demotion
Once Stage B has run cleanly for some engagement cycles, the Telegram escalation discipline is updated: routine inter-role coordination uses CAACP; Telegram reserves to keeper Rung-2 adjudication (`VETO-PENDING`) and engagement-level cross-cosmos reporting. The CLAUDE.md / AGENTS.md "Telegram escalation discipline" section is updated to reflect the demoted scope.

---

## X. Carve-outs and non-claims

- This doc does not specify the endpoint's server-side implementation (database schema, authentication backend, rate limiting, retry semantics). Those are keeper-tier deployment decisions for jaredfoy.com.
- This doc does not deprecate the existing artifact channels (proposals, notifications, fleet-state, broadcasts). Those remain the persistent content channels; CAACP is the cybernetic coordination layer above them.
- The protocol does not assume the endpoint is always reachable; §VI.3 degraded-mode preserves apparatus operability under endpoint failure.
- The protocol does not specify keeper-Telegram replacement; Telegram remains the keeper's tier-2 surface. Stage C demotes Telegram's *routine* role, not its escalation role.
- Token rotation policy, multi-token (per-role distinct tokens), and token-revocation flows are keeper Rung-2 concerns deferred to Stage B operational deployment.

---

## XI. Status

**CANONICAL at apparatus tier 2026-05-29** per keeper directive Telegram 10241 ("Begin stage A. Keep legacy channels"). Stage A landed: this doc promoted to `apparatus/docs/`; `apparatus/caacp/` scaffolding created with READMEs; four role-load skills extended with CAACP polling steps; `env.example` extended with `CAACP_TOKEN`; CLAUDE.md / AGENTS.md required-reading lists extended; `triumvirate-operational-protocol.md` updated per §VIII integration points.

Pending: (1) **Stage B** — keeper provision of the jaredfoy.com endpoint + `CAACP_TOKEN` value in `env.local`; first end-to-end cybernetic loop exercise (likely the next proposal+veto cycle); (2) **Stage C** — Telegram demotion to keeper Rung-2 escalation only, after Stage B has run cleanly for some engagement cycles.

Legacy artifact channels preserved per the keeper's Stage A directive: `apparatus/proposals/{pending,decided,archived}/`, `apparatus/watcher/notifications/`, `apparatus/deputy/{fleet-state,broadcasts}/` remain the content tier under CAACP coordination per §VIII.
