# Decision: H262S-EXT 2 detachArrayBuffer view bridge

**Decision**: APPROVED
**Decider**: helmsman directive, keeper-substituted authorization
**Date**: 2026-05-29
**Approved commits**:

- `b87721c1` - `test262 host: detach typed array backing buffers`

## Rationale

The move is the narrow host-exercisability bridge surfaced by TAPD detached-buffer rows. `$262.detachArrayBuffer` already detached ArrayBuffer objects, but ArrayBuffer-backed TypedArray construction did not expose the backing buffer through `.buffer`, so harness calls could pass `undefined` into the host shim. Exposing `.buffer`/`.byteOffset` from the view record and allowing the host hook to resolve view objects to their backing buffer removes that host limitation without taking over the remaining method-level TAPD semantics.

## Gate basis

Build passed. The named `lastIndexOf` exemplar passed. The focused TypedArray detached sweep removed all shim argument failures and raised the sweep from 8/89 PASS to 61/89 PASS.
