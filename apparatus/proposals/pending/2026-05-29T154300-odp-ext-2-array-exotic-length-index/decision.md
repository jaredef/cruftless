# Decision: ODP-EXT 2 Array exotic length/index boundaries

**Decision**: APPROVED
**Decider**: helmsman directive
**Date**: 2026-05-29
**Approved commits**:

- `0497b2c6` - `runtime: handle array defineProperty indices`

## Rationale

The move closes the approved ODP-EXT 2 residual cluster from the ODP-EXT 1 roadmap. It repairs the shared array-index predicate, length update behavior, non-writable length rejection, and own data shadowing over inherited accessors without taking over arguments/prototype-shadow substrate.

## Gate basis

Build passed. The named target set passed 8/8. The full Object.defineProperty surface gained 9 passes and the adjacent 80-file Object.defineProperty sample passed completely.
