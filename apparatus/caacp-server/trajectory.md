# CAACP Sidecar Trajectory

## CSC-EXT 1 - target_instance_id pass-through (2026-05-29)

Per helmsman directive `sidecar-target-instance-id-support-r4`.

The jaredfoy.com CAACP endpoint now supports `caacp_messages.target_instance_id`, with `NULL` retaining role-broadcast behavior and non-NULL values filtering inbox visibility to the exact registered instance. The local sidecar previously dropped unknown `/local/send` fields before forwarding to `POST /messages`, so local agents could not use the new endpoint targeting surface.

**Substrate**:
- `/local/send` accepts optional `target_instance_id`.
- The sidecar validates the local field shape as `string` or `null`.
- The forwarded endpoint payload includes `target_instance_id: target_instance_id ?? null`.
- `/local/ack` stays unchanged; endpoint-side enforcement reads the acking principal's instance from the token.
- `/local/inbox` stays unchanged; it already forwards through the per-agent token, so endpoint filtering has the necessary principal context.

**Documentation**:
- Route header comment updated.
- `apparatus/caacp-server/README.md` documents `target_instance_id` semantics and endpoint enforcement.

**Verification**:
- `/home/jaredef/.nvm/versions/node/v24.11.0/lib/node_modules/bun/bin/bun.exe build apparatus/caacp-server/server.ts --target=bun --outfile /tmp/caacp-server-check.mjs` PASS.
- Temporary R4-worktree sidecar on `127.0.0.1:7778` registered a R4 test principal and another substrate-resolver principal, sent targeted message `2f0c9992-a781-437c-8a19-50981185813d` with `target_instance_id=codex-substrate-resolver-4`, observed it in the R4 inbox and not in the other instance inbox, then resolved it with target ack `9b19f29e-1db5-4f35-95f4-f6e7918835d4`.
- The same temporary sidecar sent broadcast message `e9537beb-01d8-4546-8b71-c57170149402` (`target_instance_id=null`) and resolved it with ack `eddc2e16-58d5-417d-964c-c1fa850c0125`.
- `cargo build --release --bin cruft -p cruftless` PASS.

**Operational note**: the common-worktree sidecar at `/home/jaredef/Developer/cruftless` must restart after this lands to pick up the new pass-through code.
