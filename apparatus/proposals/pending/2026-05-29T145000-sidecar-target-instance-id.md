# Sidecar target_instance_id support

**Proposed by**: codex-substrate-resolver-4
**Date**: 2026-05-29
**Target branch**: `main`
**Risk class**: apparatus sidecar transport

## Proposed commits

- `31ff99e2` - `caacp sidecar: forward target instance id`

## Scope

Wire the local CAACP sidecar through the endpoint's new `target_instance_id` message field. `/local/send` now accepts optional `target_instance_id`, validates it as `string` or `null`, and forwards it to `POST /messages`. `/local/ack` and `/local/inbox` remain structurally unchanged because endpoint enforcement and filtering use the per-agent token principal.

## Gate report

- `bun build apparatus/caacp-server/server.ts --target=bun --outfile /tmp/caacp-server-check.mjs` PASS.
- Temporary sidecar from the R4 worktree on `127.0.0.1:7778` registered R4 and a negative-control substrate instance.
- Targeted test message `2f0c9992-a781-437c-8a19-50981185813d` with `target_instance_id=codex-substrate-resolver-4` appeared in the R4 inbox and did not appear in the negative-control inbox; resolved by targeted ack `9b19f29e-1db5-4f35-95f4-f6e7918835d4`.
- Broadcast test message `e9537beb-01d8-4546-8b71-c57170149402` with `target_instance_id=null` appeared in the R4 inbox and resolved with ack `eddc2e16-58d5-417d-964c-c1fa850c0125`.
- `cargo build --release --bin cruft -p cruftless` PASS.

## Operational note

The common-worktree sidecar at `/home/jaredef/Developer/cruftless` still runs the pre-change code until restarted after landing.
