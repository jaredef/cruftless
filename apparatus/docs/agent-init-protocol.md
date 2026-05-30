# Agent Init Protocol

The bootstrap procedure every LLM resolver session executes on entry to participate in the cybernetic-agentic-communication-protocol (CAACP) loop. Routed from AGENTS.md + CLAUDE.md as required reading.

Per keeper directive Telegram 10252–10257: substrate resolvers register individually (per-instance tokens for parallel-agent coordination); the four appointed governance/service roles (helmsman, arbiter, watcher, deputy) are single-instance and use singleton role tokens.

## I. Roles + initialization paths

| Role                | Default? | Init path                              | Token                                                                                                                                 | Instance-id requirement                  |
|---|---|---|---|---|
| Substrate resolver  | Yes      | This doc (§II below)                   | Per-instance, registered on session entry via the local CAACP sidecar                                                                  | Required at registration                  |
| Helmsman            | No       | `apparatus/skills/helmsman-load.md`    | Singleton `CAACP_TOKEN_HELMSMAN` env var; **also** pass an instance-id to sidecar registration so the physical session is distinguishable | Required at registration (per directive 10296) |
| Arbiter             | No       | `apparatus/skills/arbiter-load.md`     | Singleton `CAACP_TOKEN_ARBITER`; same instance-id convention                                                                            | Required at registration                  |
| Watcher             | No       | `apparatus/skills/watcher-load.md`     | Singleton `CAACP_TOKEN_WATCHER`; same instance-id convention                                                                            | Required at registration                  |
| Deputy              | No       | `apparatus/skills/deputy-load.md`      | Singleton `CAACP_TOKEN_DEPUTY`; same instance-id convention                                                                             | Required at registration                  |

If you have NOT been appointed to a non-default role via keeper Rung-2 intervention (e.g., "You are the Helmsman."), you are a substrate resolver. Proceed with §II.

**Instance-id discipline (all roles, per keeper directive Telegram 10296)**: every CAACP registration MUST include an `instance_id`, even for the singleton governance/service roles. Rationale: lets the apparatus distinguish physical instantiations of the same logical role over time (e.g., the helmsman appointment moved from session A on machine X to session B on machine Y); keeps the registered-agents view at `GET /api/caacp/v1/tokens?role=<role>` legible. Convention: `<runtime>-<short-hostname>-<short-ts>` (e.g., `claude-code-clawd-20260529t0344`, `codex-pop-os-20260529t040618`). The sidecar's `/local/register` accepts `instance_id` for any role; pass it always.

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
  -d "{\"role\":\"substrate-resolver\",\"instance_id\":\"$INSTANCE_ID\",\"callback_url\":null}" \
  http://127.0.0.1:7777/local/register
```

Accepted fields (per `apparatus/caacp-server/server.ts::handleRegister`): `role` (required), `instance_id` (required per §I), `callback_url` (optional; when set, sidecar POSTs `{role, instance_id, new_message_ids, notification_file}` on each new-message arrival). No other fields are accepted.

Returns (HTTP 201):

```json
{
  "token": "caacp-substrate-resolver-<uuid>",
  "role": "substrate-resolver",
  "instance_id": "<your-instance>",
  "sidecar_host": "127.0.0.1",
  "sidecar_port": 7777,
  "notification_file": "/.../apparatus/caacp-server/data/inbound-<role>-<instance>.json"
}
```

Notification-file naming: `inbound-<role>-<instance_id>.json` when `instance_id` is set; `inbound-<role>.json` when not. The file is initialized empty (`{"messages":[]}`) at registration so file-watchers can attach before the first poll.

Save the `token` to your session's working memory (do not commit to git; do not log in a way that lands in the repo). Save the `notification_file` path.

### Step 5 — Arm a notification watch (channel α)

For Claude Code sessions, set up a `Monitor` tool watch on the notification file from Step 4. The Monitor delivers `<task-notification>` events to your prompt when the file changes; you read the file's `new_message_ids` array, then GET each message body via `curl http://127.0.0.1:7777/local/inbox?role=substrate-resolver&instance_id=<your-instance>` (or directly from `apparatus/caacp-server/data/inbound-substrate-resolver-<instance>.json`).

For Codex Desktop sessions on this machine: use `apparatus/docs/codex-machine-onboarding-protocol.md` to wire the role's CAACP inbox to the Codex app-server bridge. That bridge calls `turn/start` on the target Codex thread, which is the canonical wake primitive for Desktop/iOS-controlled Codex agents.

For Codex CLI or other Codex runtimes without Desktop app-server access: register a `callback_url` in Step 4 pointing at your local HTTP listener if available. If you cannot host a listener, use the tmux bridge in §V or heartbeat-discipline polling in §IV.5.

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
    \"sender\": \"<your-role>\",
    \"sender_token\": \"$YOUR_TOKEN\",
    \"recipient\": \"<role>\",
    \"intent\": \"request|notification|response|broadcast|veto-pending\",
    \"slug\": \"<short-descriptor>\",
    \"body\": \"<message body as string>\",
    \"related_to\": null,
    \"target_instance_id\": null
  }" \
  http://127.0.0.1:7777/local/send
```

Accepted fields (per `handleSend`):
- `sender_token` (required) — your registered token from Step 4.
- `sender` (optional, validated) — if present, MUST equal the role bound to `sender_token`; mismatch returns 403.
- `recipient` (required) — the destination role.
- `intent` (required) — one of `request | notification | response | broadcast | veto-pending`.
- `slug` (required) — short descriptor.
- `body` (required) — message content as string; sidecar computes `content_sha`.
- `related_to` (optional) — message_id this message responds to.
- `target_instance_id` (optional, string-or-null) — body-level per-instance addressing for shared-role inbox semantics. **Structurally enforced as of commit `31ff99e2`** (sidecar forwards the field to the endpoint), superseding the §V.7 interim non-target-bounce discipline.

Returns HTTP 201 with `{message_id, state: "PENDING", server_timestamp, ...}` (full shape per jaredfoy.com endpoint). Record the `message_id` if you intend to await an acknowledgment.

To read your inbox (PENDING messages addressed to you):

```sh
curl -sG http://127.0.0.1:7777/local/inbox \
  --data-urlencode "role=<your-role>" \
  --data-urlencode "instance_id=<your-instance>"
```

`instance_id` is optional but recommended when multiple instances of the same role are registered; without it the sidecar selects the first matching agent.

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

## V. Cybernetic bridge for agent runtimes without native task-notification

When the agent runtime lacks native file-watch / task-notification (OpenAI Codex CLI, etc.; inotify/fswatch may not be installed; Claude Code's Monitor is not available), the apparatus provides two operator-started bridges. Choose by the runtime: prefer V.1 when the session is controlled by Codex Desktop or any runtime exposing a programmatic wake primitive; fall back to V.2 when only a terminal pane is available.

For Codex Desktop / iOS-controlled sessions on this machine, use the machine-local onboarding procedure at `apparatus/docs/codex-machine-onboarding-protocol.md`.

### V.1 Codex Desktop app-server bridge — primary

`apparatus/scripts/caacp-codex-app-bridge.mjs` (Node ESM). Uses the Codex Desktop local app-server's WebSocket `turn/start` method (preceded by `thread/resume` to attach to the target thread) to **wake the same thread** the operator is controlling from Codex Desktop / iOS. This is the canonical wake primitive on Codex Desktop: `turn/start` opens a real user turn (vs. `thread/inject_items` which only appends history). The thread receives the `**CAACP NEW** ...` directive as if the operator had submitted it, processes per the agent's normal turn loop, and reads the inbox before continuing.

Configure via env vars (see `env.example`): `CODEX_APP_SERVER_WS` (WebSocket URL), `CODEX_APP_TOKEN_FILE` (path to the local ios-token file), and `CODEX_APP_THREAD_<ROLE>` (per-role thread id, one var per bridged role).

```sh
node apparatus/scripts/caacp-codex-app-bridge.mjs <role> <thread-id> [poll-interval] [--once]
# example:
node apparatus/scripts/caacp-codex-app-bridge.mjs watcher 019e710c-4100-7db2-aff2-b36f3c323848 5
```

The bridge reads the app-server token from the local Codex installation (`~/.codex/remote-control/ios-token` by default), maintains a seen-cache at `apparatus/caacp-server/data/bridge-<role>-codex-app-seen.json`, and logs to `apparatus/caacp-server/data/bridge-<role>-codex-app.log`. Use `--once` to process one poll cycle and exit (smoke-test path).

**Operator-started only**. Programmatically waking a thread is powerful and context-sensitive; the operator decides which thread is bridged.

Per the watcher's design discovery + landing (Telegram 10278 / 10280, substrate `a7836947`): this is the **preferred** wake primitive for any session reachable via Codex Desktop, including remote iOS-controlled threads. It is strictly superior to terminal-pane injection because it reaches the actual agent process, not just the displaying terminal.

### V.2 tmux send-keys bridge — fallback

`apparatus/scripts/caacp-tmux-bridge.sh`. For runtimes that do not have a programmatic wake API but DO run inside a tmux/screen pane reachable from the local machine. Injects the `**CAACP NEW** ...` directive into the pane via `tmux send-keys`. The session sees the directive arrive as keyboard input.

```sh
bash apparatus/scripts/caacp-tmux-bridge.sh <role> <tmux-target> [poll-interval]
# example: bash apparatus/scripts/caacp-tmux-bridge.sh watcher codex-watcher:0.0 5
```

Same operator-started discipline + same `**CAACP NEW**` directive format + same seen-cache pattern (at `apparatus/caacp-server/data/bridge-<role>-seen.json`) + same pre-flight (verifies tmux target exists; logs + exits non-zero if not). Maintained as the fallback for environments where V.1 isn't available (terminal-only Codex CLI without the desktop app; older versions; non-Codex agent runtimes that nonetheless run in a tmux pane).

### Directive format (both bridges)

```
**CAACP NEW** role=<role> count=<N> latest=<sender>/<intent>/<slug>.

Run to CAACP quiescence before yielding:
1. Poll the sidecar inbox for this exact role/instance and read every PENDING message.
2. For each PENDING message, either perform the requested same-turn work and ack/respond RESOLVED, or send a response naming the concrete blocker.
3. Poll the inbox again after the last ack/response; if new PENDING messages appeared, repeat step 2.
4. Only final when the inbox has no actionable PENDING messages, required bridge/process state is verified, and your final answer includes message IDs / ack IDs / process IDs or the blocker evidence.
```

### V.3 On-wake standing instruction (mandatory; per watcher's 10298 reflection)

Bridge injection wakes the thread; **a wake is not handling**. The agent must follow this standing instruction whenever a `**CAACP NEW** ...` directive arrives via any bridge:

1. **Immediately poll the inbox**: `curl http://127.0.0.1:7777/local/inbox?role=<my-role>[&instance_id=<my-instance>]` and read the full PENDING message set, not just the latest summarized in the directive.
2. **Summarize each PENDING message**: for each, note sender + intent + slug + a one-line distillation of the body (or `body=null` if pre-body-transmission).
3. **Ack or respond per policy** before resuming any other work:
   - `notification` intent: ack RESOLVED with a short acknowledgment body, or ACKNOWLEDGED if the agent intends to action later.
   - `request` intent: respond with the requested artifact + ack RESOLVED (or DEFER-TO-KEEPER if the request is outside the agent's role-scope).
   - `response` intent: if it closes a prior outbound request, file the resolution + ack RESOLVED.
   - `broadcast` intent: integrate the announcement into working context; no ack required.
   - `veto-pending` intent: escalate to keeper immediately.
4. **Re-poll after the last ack/response**. If any PENDING messages remain or new PENDING messages appeared while you were acting, repeat steps 1-3. Resume prior work only after the inbox has no actionable PENDING messages.

**Failure mode this rule prevents** (per watcher 10298): wake fires, the agent's turn starts, but the agent gets distracted by the keeper's next directive or its own in-flight micro-step and never inspects the CAACP inbox. The standing rule moves "poll-and-act" to the agent's #1 priority on every wake.

The bridge's `Run to CAACP quiescence before yielding` block is the **operationalization** of this standing rule, not a soft suggestion. Treat it as a preemption signal that displaces whatever the agent was about to do until the inbox is drained, every same-turn request has either resolved or reported a concrete blocker, and the final answer carries the message/ack/process evidence.

### V.4 Same-turn-imperative continuity (mandatory; per keeper 10320 + 10322)

When a PENDING message contains an explicit imperative — words such as **Proceed**, **Begin**, **Execute**, **Land**, **Continue**, or a numbered execution checklist addressed to the agent's role + instance — treat the imperative as a **same-turn directive**. The agent must:

1. Carry out the imperative within the current turn, not yield control after observation or after merely acking.
2. Do not yield until the imperative is completed (for substrate landings: substrate edit + gates + trajectory entry + commit + proposal+decision + push + archive + landing-summary CAACP response).
3. If a hard blocker prevents completion in-turn (build error, gate failure, missing dependency, ambiguous scope), send a CAACP response naming the specific blocker BEFORE yielding; do not yield silently.
4. "I read AGENTS.md and registered the bridge, no new work arc was specified" is not a valid yield reason when an approval message naming explicit substrate scope is present in the inbox — the approval message IS the work arc.

**Failure mode this rule prevents** (per keeper 10320): the agent wakes, polls inbox, observes an approval message, acks, then yields back to "await next directive" — when the approval message itself was the next directive. Observation is not action; ack is not action; action is action.

### V.5 Helmsman approval-as-fresh-outbound discipline (mandatory; per keeper 10322)

When the helmsman approves a substrate-resolver landing plan (or any cybernetic-flow communication that requires the recipient to wake-and-act on it):

- The approval body MUST be sent as a **new outbound message** (intent=response, related_to=<plan-message-id>), not as the body of an ack on the plan message.
- Acks transition state on the original message and appear on the sender's outbox surface, but **they do not arrive as new inbox arrivals on the recipient's side** because sidecars poll `/local/inbox` (which surfaces messages addressed TO the role), not outbox-ack-changes (which surfaces only on the original sender's view).
- Acks remain appropriate for state-machine transitions that do not require recipient action (RESOLVED on a notification, ACKNOWLEDGED on a response that confirms a prior request closed). Substantive direction always rides a fresh outbound message.
- Per-instance targeting: shared-role inbox semantics mean ALL substrate-resolver instances see messages addressed to `recipient=substrate-resolver`. Per-instance targeting is **body-level only** via an explicit `instance_id=<id>` header in the body; non-target instances self-bounce as misdelivered-to-<role>.

**Failure mode this rule prevents** (per keeper 10322 round): helmsman acks substrate-resolver plan with APPROVED + landing checklist in ack body. Substrate-resolver polls own inbox, finds no new PENDING approval, waits. The approval is invisible cross-machine because the ack-body lives on the plan-message's outbox surface, not the resolver's inbox.

**Deferred apparatus enhancement**: sidecar polling could be extended to surface outbox-ack-changes when ack bodies carry substantive content (length threshold or explicit `surface_to_recipient=true` flag). Tracked as Stage-C deferral; until landed, the helmsman discipline above is the workaround.

### V.6 Resolver-as-committer discipline (mandatory; per keeper 10325 + 10327)

Each substrate-resolver authors, commits, and pushes its own substrate work. The substrate-landing pipeline's final phase (commit → proposal+decision → push → archive) runs in the resolver's clone, with the resolver as commit author. The helmsman authorizes the landing via approval; the helmsman does NOT commit on the resolver's behalf. The deputy is a fleet-communication relay and fleet-state-snapshot author; the deputy does NOT commit on resolvers' behalf either.

**Why this discipline**:
1. **Substrate authorship traceability**: the commit author + trajectory entry + proposal+decision pair form a closed attestation chain linking a specific resolver-instance to the substrate move it made. Centralizing commits through helmsman or deputy fractures this chain.
2. **Locale-as-coordinate fidelity** (Doc 737): each rung is a Pin-Art move at a coordinate; the resolver IS the worker at that coordinate. Commit-and-push is the final phase of the five-phase substrate-shaped-work pipeline. Severing the commit from the worker fractures the pipeline.
3. **Triumvirate role separation** (apparatus/docs/triumvirate-protocol-keeper-helmsman-arbiter.md): helmsman = substrate-steering + push-authority *authorization*; substrate-resolver = substrate work + landing; deputy = fleet-communication relay. A role that commits on another role's behalf collapses two role surfaces into one.
4. **Pre-push hook gating works per-clone**: the hook fires in the resolver's clone where proposal+decision files are local. Routing commits through another role would require shipping those files cross-clone before push, adding a coordination round-trip per landing.

**Worktree-hygiene discipline** (per the "Git Add Scope" feedback rule):
- Never `git add -A` or `git add .` — enumerate specific files for each commit. A resolver's clone may carry dirty state from prior rungs, sibling resolvers (if shared-clone), or apparatus scratch files.
- The 3-commit dance per landing — (1) substrate + trajectory, (2) proposal + decision, (3) archive — must each enumerate its files explicitly.
- If a rebase is required between commits (2) and (3) due to upstream changes, the substrate commit SHA in proposal+decision must be updated and the proposal+decision recommitted (soft reset is the canonical mechanism). Never amend a substrate commit with proposal+decision in the same commit — the proposal+decision references the substrate SHA, so they cannot co-author the same commit (chicken-and-egg).
- Shared-clone scenarios (multiple resolvers on the same checkout) are currently unsupported by this discipline; each resolver should run in its own clone (or its own `git worktree add`) to avoid cross-resolver dirty-state contamination. If shared-clone becomes operationally necessary, the discipline upgrade is per-rung `git worktree add` rather than stash-juggling (stash is brittle under partial-failure as observed in the 2026-05-29 EPSUA round).

**Deputy commit-coordination scope** (NOT commit-authorship scope):
- Author fleet-state snapshots that inventory which resolvers landed which commits at which rungs.
- Surface cross-resolver rebase storms or push-collision patterns to helmsman.
- Author post-round retrospectives noting landing-pipeline friction points.
- The deputy never authors a substrate commit, a proposal commit, or an archive commit.

**Deputy apparatus-artifact commit carve-out** (per keeper Telegram 10343, surfaced by deputy on 2026-05-29 topology round):

The above "deputy does not commit" rule applies to substrate, proposal, and archive commits — the commits that ride the substrate-landing pipeline. It does NOT cover deputy-authored apparatus artifacts that have no other commit path. Without a carve-out, these artifacts strand as untracked files in the worktree (observed: `apparatus/deputy/fleet-state/2026-05-28T210804-0700-initial.md` + `apparatus/deputy/fleet-state/2026-05-28T214705-0700-arc-alignment.md` sat untracked for ~9 hours because no role had authority to commit them).

The carve-out:
- Deputy MAY commit files **only within `apparatus/deputy/`** under deputy authorship.
- Commit messages prefixed `apparatus(deputy):` for grep-distinguishability.
- No proposal+decision pair required for deputy apparatus-artifact commits (these are not substrate; the pre-push hook should be configured to recognize this scope; until configured, deputy uses `CRUFTLESS_HOOK_BYPASS=apparatus-deputy-artifact-only` per the emergency-override convention).
- Helmsman approval is NOT required for in-scope commits (fleet-state snapshots are deputy's role-scope authorship; gating them on helmsman authorization would re-collapse the role separation).
- Helmsman approval IS still required for any deputy commit that touches files outside `apparatus/deputy/` — in which case the resolver-as-committer rule applies and the deputy should hand the file off to the appropriate resolver instead.
- Push: deputy may push deputy-scope commits directly via `git push origin HEAD:main` from its worktree (whether common or per-instance depending on topology).

Why a carve-out instead of routing through helmsman/a resolver:
- Helmsman-as-committer for deputy artifacts would violate the "authorizer not committer" principle (asymmetric authority).
- Resolver-as-committer for deputy artifacts would break attribution (the fleet-state snapshot was authored by deputy; the commit author should match).
- A narrowly-scoped carve-out preserves both principles without stranding apparatus artifacts.

Failure mode this rule prevents: apparatus artifacts (fleet-state snapshots, deputy retrospectives) authored by deputy but stranded as untracked files because no commit path exists. Observed in the 2026-05-29 topology round when deputy fleet-state files from 2026-05-28T21:08 + T21:47 had been waiting hours for a commit path that §V.6 (as originally written) didn't provide.

### V.7 Bounce-ack must not consume directives addressed to other instances (superseded by `target_instance_id` structural fix; retained for clones predating commit `31ff99e2`)

**Status as of 2026-05-29**: the structural fix landed. `/local/send`, `/local/inbox`, and the upstream endpoint now accept and enforce `target_instance_id` (sidecar commit `31ff99e2 caacp sidecar: forward target instance id`). Senders SHOULD always set `target_instance_id` when addressing a specific instance; the endpoint then admits terminal-state acks only from the matching instance, so non-target bounces can no longer consume the directive. The interim discipline below is retained as the failure-mode reference and as the fallback for clones that have not yet picked up `31ff99e2`.



When a directive is sent to `recipient=<role>` with body-level `instance_id=<target>` targeting a specific instance, **non-target instances MUST NOT call `/local/ack` with `ack_state=RESOLVED`** on the original message. The CAACP endpoint currently tracks message state globally (not per-instance); a RESOLVED ack from a non-target instance transitions the message globally, consuming it for the target instance — whose `/local/inbox` poll then returns `{messages: []}`. The target never sees the directive.

The correct discipline for non-target instances:

1. **Observe the directive**: read the body, identify the target instance_id, identify yourself as non-target.
2. **Do NOT ack the original message**. Leave it PENDING for the target.
3. **Send a separate response message** (intent=response, recipient=helmsman) reporting your non-target observation. This is a fresh outbound CAACP message, not an ack on the original. The body should name the message_id observed, the target instance_id, and your instance_id confirming you are not the target.
4. **Continue holding** until you receive a directive explicitly targeted to your own instance_id.

The original directive remains in PENDING state until the actual target instance processes it, at which point the target's RESOLVED ack correctly transitions the global state.

**Why this discipline (historical interim)**:

The structural fix is `target_instance_id` on `/local/send` + `/local/inbox` + `/local/ack` so the endpoint can enforce that terminal state transitions on targeted messages are accepted only from matching role+instance. **Both items landed** (keeper-authored endpoint schema change + sidecar update at commit `31ff99e2`). The discipline below remains the correct behavior for any clone/session predating that commit, and is the recovery path if a non-target ack is observed to consume a directive (re-dispatch with a fresh `message_id` to the actual target).

**Failure mode this rule prevents** (per 2026-05-29 R3 PIND Rung 4a delivery miss):
- Helmsman dispatched directive `ac77efff` to substrate-resolver, body-targeted R3.
- R1 + R4 sent RESOLVED bounce-acks; R2 sent a not-target response.
- Endpoint marked `ac77efff` state=RESOLVED globally after the first RESOLVED bounce-ack.
- R3's inbox poll returned `{messages: []}`. R3 inferred quiescence and stopped — exactly opposite of the intended outcome.
- Helmsman had to resend the directive as `33b5a8d2` to deliver it to R3. R3 then executed normally.

**Auxiliary mechanism (optional, recommended for in-flight cases)**: per watcher's recommendation, an `OBSERVED` or `NOT-FOR-ME` `ack_state` value can be added to the CAACP state machine for non-targets that prefer to log their bounce on the original message without globally consuming it. Until that lands, use the "separate response message" discipline above.

### V.8 Pre-existing worktree diffs are expected; categorize before yielding (mandatory; per keeper Telegram 10397)

When a substrate-resolver enters its per-instance worktree at session start (per §V.6), `git status` will frequently show unstaged or untracked files. **This is expected, not a blocker.** Resolvers that yield on the presence of pre-existing diffs without categorizing them stop progress unnecessarily.

The expected sources of pre-existing diffs:
1. **Your own prior-rung leftovers** — files you edited in a prior session that weren't committed (e.g. measurement artifacts, scratch test files, partial WIP). Categorize: is this work I started? If yes, decide commit-or-discard based on relevance to the current directive.
2. **Deputy apparatus artifacts** (per §V.6 carve-out) — `apparatus/deputy/fleet-state/*.md` and similar. Deputy owns these; leave them alone (do NOT commit, do NOT discard).
3. **Cross-resolver leakage that predates the §V.6 per-instance worktree split** — historical artifacts from when fleet shared a single worktree (`/home/jaredef/Developer/cruftless`). May still be present in older clones. Categorize: not your work, not deputy's; leave alone or surface to helmsman as cleanup candidate.
4. **Build artifacts under untracked paths** that .gitignore doesn't cover — categorize: ephemeral, ignorable.
5. **Sidecar/CAACP scratch state** under `apparatus/caacp-server/data/` — gitignored; ignore.

**Standing rule on observed diffs**:
- Run `git status` at session start and inventory.
- Cross-reference each unstaged/untracked file against the five categories above.
- Proceed with your directive's substrate work. **The presence of unrelated diffs is not a blocker.**
- When you commit your substrate work, enumerate ONLY the files your directive's edits touched (per "Git Add Scope" discipline + §V.6 worktree-hygiene rules) — never `git add -A` or `git add .`.
- Surface unexplained diffs in your landing summary (one-line "observed unstaged: X, Y, Z; not touched in this commit"), so helmsman has visibility for follow-up.

**Failure mode this rule prevents** (per keeper Telegram 10397): resolver enters worktree, sees `git status` showing unrelated unstaged files (from prior sessions, deputy authoring, or sidecar scratch), interprets the dirty state as a blocker, yields back with "worktree has dirty state, need clarification before proceeding." The dirty state is almost never blocking; the resolver's own edits + commit scope are what matter.

**When unstaged diffs ARE a blocker**: only when (a) they conflict with the file the current directive needs to edit, (b) they prevent `git rebase origin/main` from completing cleanly, or (c) they prevent the pre-push hook from accepting the proposal+decision pair. In those cases name the specific blocker in the yield, don't generalize to "dirty worktree."

## VI. Failure modes

- **Sidecar unreachable**: substrate work continues without CAACP coordination; you operate per the legacy artifact-passing convention (write to `apparatus/proposals/`, `apparatus/watcher/notifications/`, etc.) and rely on keeper-Telegram routing per the pre-CAACP discipline.
- **Registration fails with `remote registration failed`**: the sidecar's admin token (`CAACP_TOKEN_VERIFIER`) is unset, expired, or jaredfoy.com is down. Surface to keeper.
- **No callback received + no file-watch update**: either the polling loop is stuck or no messages are arriving. Sanity-check `curl http://127.0.0.1:7777/local/health` for `last_polled_at` per-agent.
- **Token mismatch (403)**: you're sending with a token that doesn't bind to the sender role you're claiming. Re-verify Step 4 output.

## VII. Cross-references

- `apparatus/docs/cybernetic-agentic-communication-protocol.md` — full CAACP articulation (§VI for endpoint surface, §IV for state machine).
- `apparatus/caacp-server/README.md` — sidecar HTTP API + operational notes.
- `apparatus/docs/codex-machine-onboarding-protocol.md` — machine-local Codex Desktop onboarding + app-server bridge setup.
- `apparatus/docs/agent-engagement.md` — substrate-disciplined LLM resolver directions (the broader discipline this init protocol nests within).
- `apparatus/docs/engagement-doc-substrate-resolver.md` — substrate-resolver role frame (what you may and may not do once initialized).
- `apparatus/scripts/caacp.sh` — thin wrapper for direct-to-jaredfoy.com CAACP ops (legacy direct path; the sidecar is the preferred path going forward).
