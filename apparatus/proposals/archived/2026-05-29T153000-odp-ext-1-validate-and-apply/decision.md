# Decision: ODP-EXT 1 ValidateAndApply property-key closure

**Decision**: APPROVED
**Decider**: helmsman directive
**Date**: 2026-05-29
**Approved commits**:

- `0b11aeea` - `runtime: validate object defineProperty descriptors`

## Rationale

The ODP-EXT 0 probe identified descriptor-shape/property-semantics as the coherent C4 bucket at 43/54 rows. This rung lands the approved first move: make `Object.defineProperty` ordinary descriptor validation a single `PropertyKey`-aware helper instead of parallel string-key branches.

The evidence is sufficient for landing: the named non-configurable data/accessor rows pass, the Symbol-key reflection rows pass, the build passes, and the focused descriptor bucket gains 26 passes while leaving the pre-declared array/arguments/prototype/typed-array residuals for later rungs.

## Gate basis

Build passed. Targeted exemplar set passed 8/8. Descriptor bucket measured 26/43 PASS after the change. Full Object.defineProperty surface measured 33/54 PASS with only one no-output row remaining.
