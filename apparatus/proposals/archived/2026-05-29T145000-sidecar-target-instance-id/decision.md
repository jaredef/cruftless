# Decision: Sidecar target_instance_id support

**Decision**: APPROVED
**Decider**: helmsman directive, keeper-substituted authorization
**Date**: 2026-05-29
**Approved commits**:

- `31ff99e2` - `caacp sidecar: forward target instance id`

## Rationale

The endpoint schema now supports exact-instance message targeting. The sidecar was the remaining local transport gap because it accepted `/local/send` requests but did not pass `target_instance_id` through to the endpoint. This change is a narrow pass-through plus documentation and an apparatus trajectory entry.

## Gate basis

Targeted and broadcast flows were verified end to end through a temporary sidecar running from the R4 worktree. The targeted smoke message was visible to `codex-substrate-resolver-4` and absent from the negative-control instance inbox; both targeted and broadcast messages resolved through `/local/ack`. Rust build remains green.
