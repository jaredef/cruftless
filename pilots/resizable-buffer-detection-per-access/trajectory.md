# resizable-buffer-detection-per-access — Trajectory

## RBDPA-EXT 0 — founding (2026-05-30)

**Trigger**: Helmsman deferrals-vs-substrate audit (2026-05-30) under keeper directive Telegram 10558 ("Begin with 1") promoting `apparatus/docs/deferrals-ledger.md` Entry 009 from DEFERRED to PROMOTED. The audit found the ≥3-cell spawn threshold met at TAWR-EXT 5 Phase 6 emission and the gating predicate (arc `2026-05-28-array-exotic-substrate` reaches Phase 5 chapter-close) effectively satisfied — TAWR-EXT 5 LANDED + TAWR-EXT 6 REVERT closed the productive-rung sequence at the Phase-5 inflection per `pilots/typed-array-wrong-result/trajectory.md` EXT 6 conclusion.

**Originating rung**: `pilots/typed-array-wrong-result/trajectory.md` TAWR-EXT 5 Phase 6, verbatim:

> Phase 6 (deferral emission): surfaces `resizable-buffer-detection-per-access` as a candidate locale — the residual DataView `custom-proto-access-resizes-buffer-*` failures (3 cells: invalid-by-length, invalid-by-offset, valid-by-offset) all share the shape "per-access OOB check when the underlying buffer is resizable and was resized between construction and access". Currently DV stores `fixed_length` at construction; the resizable-buffer path needs a per-access recompute.

**Arc enrollment**: `2026-05-28-array-exotic-substrate` (founding row appended to sub-locale roster at this rung).

**Apparatus** (to land at EXT 1 founding-baseline):

- `exemplars/exemplars.txt` — exemplar paths covering the three DataView cells plus the TypedArray-side resizable-buffer-length-tracking ring expansion candidates (~10 cells per the deferral lift prediction).
- `exemplars/run-exemplars.sh` — runner; aggregate pass/fail + per-shape breakdown.
- `exemplars/pool-size.txt`, `exemplars/family-breakdown.txt` — inventory.
- **Baseline**: TBD. Expected near 0/N on the DataView cells (mechanism known absent); TypedArray-side cells require fresh inspection before claiming shared-substrate prediction.

**Phase 1 (Spawn) per Doc 744 §V.1**:
- **M** = DataView (and TypedArray sibling) access on a view whose underlying ArrayBuffer is resizable.
- **T** = view byte-length recomputed against the buffer's current byte length per IsViewOutOfBounds + GetViewByteLength (DataView) / IntegerIndexedElementGet/Set OOB branch (TypedArray); RangeError on OOB, success on in-bounds.
- **I** = per-access dispatch from DataView intrinsic methods (`getInt8` / `setInt8` and the typed-width siblings) reads the current ArrayBuffer length rather than the cached `fixed_length`; equivalent recompute at the TypedArray indexed-access lowering.
- **R** = lattice with `typed-array-resizable-buffer-indexed-access` (sibling locale in arc, TypedArray-side cells); DAG ↑ runtime intrinsics (DataView storage shape, ArrayBuffer.prototype.resize tracking).
- **Observability** = ordinary (test262 cell PASS/FAIL transitions; diff-prod re-run for any added fixtures).
- **Mouth-gating prerequisite**: DataView storage shape must carry a back-reference to the ArrayBuffer instance (or to its length-getter) reachable from each access site. If currently absent, EXT 1's substrate move includes the storage extension. Audit at EXT 1 baseline.

**Phase 2 (Baseline-inspect)**: deferred to EXT 1. Inspect the three DataView cell signatures, confirm shared mechanism vs. spurious per-cell differences, enumerate TypedArray-side cell ring.

**Status**: RBDPA-EXT 0 FOUNDED locally. Apparatus scaffold pending; first productive rung pending baseline + mechanism inspection. Carrying deferrals-ledger Entry 009's lift forward into Pin-Art-tracked substrate work.
