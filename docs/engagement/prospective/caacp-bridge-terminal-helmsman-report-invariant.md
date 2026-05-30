# The CAACP bridge terminal Helmsman report invariant: making bridged resolver quiescence globally legible

**Status**: prospective draft, awaiting keeper review for apparatus or corpus promotion. Authored 2026-05-30 from the Resolver 2 bridge-mismatch session.

**Composes with**: Doc 729 (resolver-instance pattern), Doc 733 (fractal seeds-and-trajectories), Doc 737 (locale as coordinate), `apparatus/docs/cybernetic-agentic-communication-protocol.md` (CAACP), `apparatus/docs/agent-init-protocol.md` §V (bridge wake + same-turn imperative), `apparatus/docs/codex-machine-onboarding-protocol.md` (Codex app-server bridge), and `apparatus/docs/service-tier-and-statefulness-protocol.md` (watcher/deputy freshness frame).

**Empirical anchor**: Resolver 2 bridge session on 2026-05-30. The CAACP bridge was initially wired to stale Codex thread `019e7112-011a-7723-9a58-81f96c091352` while the keeper-visible session was thread `019e75f3-b2f4-7aa0-87e1-822d63640a4f`. The bridge successfully injected wake turns, but into the wrong session. The visible resolver only progressed because the keeper manually asked it to poll, diagnose the mismatch, notify Helmsman, and restart the bridge.

---

## I. Thesis

A bridged resolver turn is not complete when its inbox is empty. It is complete only when the resolver has sent a fresh outbound terminal report to Helmsman and the bridge has recorded that message id.

The required invariant:

```text
bridged-quiescence =
  inbox drained
  + same-turn imperative completed or explicitly blocked
  + fresh outbound terminal report sent to Helmsman
  + terminal report message_id recorded in bridge state
```

This turns Helmsman into the terminal join point for bridged resolver action. The resolver may still perform substrate work autonomously, but the bridge does not recognize quiescence until the action has become visible on Helmsman's communication surface.

## II. Failure Shape

The current bridge can retire or stop re-waking on weak local signals:

```text
wake -> resolver polls -> resolver acts or acks -> inbox empty -> Codex stops
```

That flow permits four ambiguous terminal states:

- The resolver completed work but did not notify Helmsman.
- The resolver acked or observed the directive but did not act.
- The bridge injected into a stale or wrong thread.
- Codex stopped before the same-turn imperative reached its telos.

All four look similar from the coordinator's perspective: no fresh Helmsman-visible completion artifact exists. The absence is diagnostic, but only if the protocol names it as a violated terminal obligation.

## III. Architectural Constraints

### III.1 Quiescence includes a terminal report

The bridge's active-directive ledger records a terminal obligation:

```json
{
  "directive_id": "<caacp-message-id>",
  "resolver_instance_id": "<instance-id>",
  "source_message_ids": ["<inbox-message-id>"],
  "requires_helmsman_final": true,
  "helmsman_final_message_id": null,
  "state": "ACTIVE"
}
```

The directive remains active until `helmsman_final_message_id` is non-null.

### III.2 Completion is a fresh outbound message

The terminal report is not an ack body. It is a fresh CAACP message addressed to Helmsman:

```json
{
  "recipient": "helmsman",
  "intent": "notification",
  "slug": "<directive-slug>-resolver-final",
  "related_to": "<inbound-message-id>",
  "body": "<completed-or-blocked evidence>"
}
```

Acks remain state-machine transitions on the original message. Substantive completion rides a new inbox-visible message.

### III.3 Bridge retirement checks the terminal report

The bridge may not remove a directive from its active ledger solely because the original resolver inbox item disappeared. Retirement requires:

1. The inbound directive is no longer actionable for the resolver.
2. The resolver sent a terminal report to Helmsman.
3. The bridge recorded the terminal report `message_id`.

The minimal implementation accepts local sidecar `/local/send` success as proof. A stronger implementation waits for Helmsman ack or remote-state confirmation.

### III.4 Stop-continue keys off missing terminal report

If Codex enters `idle` or `notLoaded` while an active directive has no terminal report, the bridge re-wakes with a specific predicate:

```text
**CAACP CONTINUE** reason=missing-helmsman-final.
Directive <id> is still active because no terminal report to Helmsman has been recorded.
Send a fresh outbound final status to Helmsman before yielding.
```

The continue wake is therefore not a generic poke. It is an enforcement of the missing terminal edge.

### III.5 Resolver completion should be wrapped

A resolver-facing helper should make the terminal sequence mechanical:

```sh
caacp-resolver-complete <inbound-id> <slug> <body-file>
```

The helper should:

1. Poll the exact instance inbox one last time.
2. Send the fresh terminal report to Helmsman.
3. Ack or resolve the inbound directive if needed.
4. Write the terminal `message_id` into bridge state.
5. Print the message ids for the resolver's final response.

This keeps the model from having to remember the ordering every time.

## IV. Induced Properties

### IV.1 Closed terminal edge

Every bridged action has a final communication edge:

```text
resolver action -> Helmsman-visible terminal report
```

The system no longer accepts local silence as completion.

### IV.2 Globally legible quiescence

Quiescence becomes apparatus state rather than model assertion. A resolver is idle because the bridge ledger records the terminal report, not because the resolver says "done" in a private thread.

### IV.3 Ack/completion separation

Acks update the original message. Terminal reports communicate outcome. This prevents substantive direction or completion from being hidden on an outbox-ack surface that the recipient never polls.

### IV.4 Stop resilience

Codex stopping early is recoverable. The bridge can re-wake on a durable predicate:

```text
ACTIVE directive && helmsman_final_message_id == null
```

### IV.5 Miswire detectability

If the bridge injects into the wrong thread, Helmsman still does not receive a terminal report. The missing terminal report becomes an explicit diagnostic signal for bridge miswiring, stopped sessions, ignored inboxes, or send failures.

### IV.6 Per-instance accountability

Terminal reports carry the resolver `instance_id`, inbound directive id, result, verification evidence, and blocker evidence if blocked. Parallel substrate resolvers sharing one role inbox remain distinguishable.

## V. Afforded Behaviors

### V.1 Reliable remote dispatch

The keeper or Helmsman can dispatch work to a bridged resolver and expect one of three observable outcomes:

- completed with evidence,
- blocked with evidence,
- bridge continues because the terminal report is missing.

There is no valid "unknown idle" terminal state.

### V.2 Trustworthy fleet summaries

Helmsman and Deputy can summarize fleet state from CAACP artifacts instead of local chat transcripts. Resolver status becomes a sequence of terminal reports keyed by directive id and instance id.

### V.3 Practical mobile control

The operator can send work from iOS or another remote surface, leave the resolver unattended, and later inspect Helmsman's terminal-message surface. The system no longer depends on visually monitoring each Codex thread.

### V.4 Cleaner handoffs

A terminal report can carry commit ids, touched files, gate results, blocked resources, or "no-op, already landed" evidence. Another resolver can resume from the terminal report without reading the full originating thread.

### V.5 Governance gating

For substrate landings, Helmsman receives a standard completion packet before push-tier decisions. The terminal packet can include build/test status, proposal/decision state, archive state, and residual risks.

### V.6 Duplicate-work suppression

If a directive is resent after a bridge restart, Helmsman can inspect whether a terminal report already exists for the directive and instance. Resend/restart no longer automatically creates duplicate substrate work.

## VI. Protocol Integration

### VI.1 Bridge active ledger

Extend `apparatus/scripts/caacp-codex-app-bridge.mjs` active ledger entries with:

- `requires_helmsman_final`
- `helmsman_final_message_id`
- `terminal_report_sent_at`
- `terminal_report_related_to`
- `terminal_report_slug`

The bridge's retirement function checks these fields before removing an active directive.

### VI.2 Sidecar helper

Add a local helper script under `apparatus/scripts/` that sends the terminal report and updates bridge state. The helper should avoid token logging and should accept the terminal body via stdin or a file.

### VI.3 Wake text

Bridge wake text should include:

```text
Before yielding, send a fresh outbound final status to Helmsman and record/include its message_id. Inbox-empty alone is not quiescence.
```

This prompt text is secondary. The ledger invariant is primary.

### VI.4 Helmsman final-report contract

Helmsman should treat every bridged directive as pending coordination until the terminal report arrives. If Helmsman sees the original directive resolved without a terminal report, it should classify that as a protocol violation or bridge/session failure.

## VII. Falsifier

The design is falsified if a bridged resolver can reach idle after receiving a same-turn directive while all of these are true:

1. The directive remains active in bridge state.
2. `requires_helmsman_final` is true.
3. `helmsman_final_message_id` is null.
4. The bridge has not injected a missing-terminal-report continue or surfaced an operator-visible bridge error.

A weaker falsifier is high false-positive continuation: the bridge repeatedly wakes a resolver even after a valid terminal report was sent. That would indicate the terminal-message detection or ledger update path is under-specified.

## VIII. Status

Working draft. Candidate for apparatus-tier operationalization in `agent-init-protocol.md` §V and `codex-machine-onboarding-protocol.md` §V after keeper review. If promoted, the implementation target is the Codex app-server bridge active-ledger retirement path plus a resolver completion helper.
