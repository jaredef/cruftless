# Codex Stop-Continue Bridge Design

Status: watcher design note, 2026-05-29.

## Trigger

Keeper surfaced a Codex Desktop bridge gap: the app-server bridge wakes a
thread when a CAACP message arrives, but it does not notice when Codex stops
before the directive's telos reaches CAACP quiescence. The requested mechanism
is a bridge-side "continue" wake primitive.

## App-Server Findings

The current bridge at `apparatus/scripts/caacp-codex-app-bridge.mjs` uses a
short-lived websocket JSON-RPC exchange:

1. `initialize`
2. `thread/resume`
3. `turn/start`

Read-only probing on this host found:

- `thread/list` is available and returns each thread's `status`.
- `thread/read` is available and returns a single thread's `status`.
- Observed statuses: `active`, `idle`, `notLoaded`, `systemError`.
- `thread/poll`, `thread/status`, `turn/status`, and `session/status` are not
  app-server methods in this build. They return an unknown-variant error.
- `thread/turns/list` exists but requires `experimentalApi` capability. It is
  not a good dependency for the first bridge implementation.

Conclusion: use `thread/list` or `thread/read` as the stop-state observation
surface. Do not build the first version around nonexistent poll/status methods.

## Continue Payload

Use a distinct directive prefix so the receiver can treat the wake as a resume,
not as a fresh CAACP message:

```text
**CAACP CONTINUE** role=<role> instance_id=<instance_id> target_directive_id=<message_id> reason=stop-before-telos.

Resume directive per agent-init-protocol §V.4 same-turn imperative:
1. Poll the exact sidecar inbox.
2. If target_directive_id is still PENDING, continue that work.
3. If target_directive_id is no longer PENDING, drain any other PENDING messages.
4. Do not final until CAACP quiescence is verified.
```

The bridge should inject this with the same app-server path it already uses:
`thread/resume`, then `turn/start`.

## Before-Quiescence Definition

Do not infer telos completion from local wall-clock alone. The bridge has the
CAACP state machine available, so it should track outstanding directives:

- When a `**CAACP NEW**` directive is injected, record each fresh message id in
  `apparatus/caacp-server/data/bridge-<role>[-<instance>]-codex-app-active.json`.
- An active directive is quiescent when it is no longer present in
  `/local/inbox?role=<role>[&instance_id=<id>]` as `state=PENDING`.
- If the sidecar grows `/local/message/<id>` or `/local/outbox`, prefer exact
  message-state checks over inbox absence. Until then, inbox absence is the
  available local signal.

For request-shaped work, this is intentionally state-machine based rather than
"did a response text appear in the chat." The bridge should not parse Codex
answers for completion.

## Stop Detection

On every bridge poll:

1. Poll CAACP inbox as today.
2. Poll app-server `thread/read` for the target thread status.
3. If there is at least one active directive still PENDING and the thread status
   is `idle` or `notLoaded`, inject `**CAACP CONTINUE**`.
4. If status is `systemError`, do not loop blindly. Log and surface an operator
   alert; a continue turn may not be accepted.
5. If status is `active`, do nothing; the resolver may still be working.

Add throttles:

- `--continue-after-seconds`, default `60`.
- `--continue-interval-seconds`, default `120`.
- `--continue-max-attempts`, default `3`.

This prevents repeated continue turns while preserving the wake guarantee for
the observed "Codex stopped" failure mode.

## Implementation Path

This is apparatus-scope bridge work, not runtime substrate work. The right
landing unit is a small bridge-extension commit plus docs:

- Extend `caacp-codex-app-bridge.mjs` with `codexThreadStatus()`.
- Persist an active-directive ledger beside the seen-cache.
- Add continue-injection throttling and logging.
- Document the flags in `apparatus/docs/codex-machine-onboarding-protocol.md`
  and `apparatus/caacp-server/README.md`.

Watcher can author the design and review the result. A substrate-resolver or
Helmsman should land the code change if the keeper wants a committer outside
the watcher role; the change is operational apparatus code and does not require
runtime substrate authority.

## Open Constraint

The current CAACP sidecar lacks a direct local message-state endpoint. The
first implementation can use PENDING inbox absence as the quiescence predicate,
but the more exact follow-up is:

```text
GET /local/messages/<message_id>
```

forwarded to the remote CAACP endpoint with the registered agent token. That
would let the bridge retire active directives by exact state (`RESOLVED`,
`ARCHIVED`) instead of by absence from the PENDING inbox.
