# resizable-buffer-detection-per-access — Seed

## Telos

Materialize the engine-DAG coordinate

```
runtime/buffer-typed-array :: E3/intrinsic-object:ecma-262 :: resizable-buffer/per-access-OOB :: assertion/expected-mismatch
```

Promote DataView (and, by lattice, TypedArray) byte-length and indexed-access paths to recompute the in-bounds window on every read/write when the underlying ArrayBuffer is resizable, per ECMA-262 §25.1.5 (Resizable ArrayBuffer + Growable SharedArrayBuffer) + §25.3 (DataView abstract operations GetViewByteLength / IsViewOutOfBounds) + §10.4.5 (TypedArray IntegerIndexedElementGet/Set OOB semantics).

## Work shape

**Heuristics §IV classification**: D (Runtime Intrinsic Semantics) — view-vs-buffer length contract under resize.

Currently DataView caches `fixed_length` at construction; access does not re-derive against the underlying ArrayBuffer's current byte length. The three exemplars exhibit the same substrate shape:

- `DataView` constructed with implicit-length over a resizable ArrayBuffer.
- `ArrayBuffer.prototype.resize` shrinks (or grows) the buffer.
- Subsequent `getInt8` / `setInt8` (or sibling) call must check current view extent against current buffer length per IsViewOutOfBounds + GetViewByteLength.

The locale's mouth is the access-time check; the terminus is RangeError-vs-success per the spec dispatch.

## Origin

Surfaced from `pilots/typed-array-wrong-result/trajectory.md` TAWR-EXT 5 Phase 6
(deferral emission). Carried in `apparatus/docs/deferrals-ledger.md` Entry 009
(2026-05-28) as spawn-threshold-at-promotion-readiness pending arc Phase 5
chapter-close. Promoted to founded locale on 2026-05-30 per keeper directive
(Telegram 10558) after the helmsman deferrals-vs-substrate audit identified the
≥3-cells threshold as met and the gating arc-closure predicate as effectively
satisfied (TAWR-EXT 5 LANDED + TAWR-EXT 6 REVERT closed the productive-rung
sequence at Phase-5 inflection per `pilots/typed-array-wrong-result/trajectory.md`
EXT 6 conclusion).

## Apparatus

- **Exemplar suite**: three DataView surface cells inherited from the TAWR
  residual at EXT 5 close:
  - `test262/built-ins/DataView/.../custom-proto-access-resizes-buffer-invalid-by-length.js`
  - `test262/built-ins/DataView/.../custom-proto-access-resizes-buffer-invalid-by-offset.js`
  - `test262/built-ins/DataView/.../custom-proto-access-resizes-buffer-valid-by-offset.js`
- **Expansion candidates** (to enumerate at EXT 0 baseline): the TypedArray-side
  resizable-buffer-length-tracking cluster (~10 exemplars per the deferrals-ledger
  entry's lattice prediction) is the next-shape ring; locale scope grows to the
  TypedArray cell ring if/when the first DataView-side rung lands without
  resolving them via shared substrate.
- **Inventory** (to land at EXT 0): `exemplars/exemplars.txt` + pool count +
  per-shape breakdown (DataView access vs. TypedArray indexed-access vs.
  byteLength getter).

## Methodology

Per heuristics §VIII Debugging Rule, every substrate rung against this
coordinate must satisfy:

- large enough to matter — 3+ DataView cells confirmed; expected ≥10 with the
  TypedArray ring on first inspection
- coherent across examples — to be verified at EXT 0 baseline against the
  three DataView cells' failure signatures
- comparable within one availability class — yes (single intrinsic-object
  availability, single resizable-ArrayBuffer cut)
- owned by one resolver instance or one shared abstract op — IsViewOutOfBounds
  + GetViewByteLength (DataView) and IntegerIndexedElementGet/Set OOB branch
  (TypedArray) are the two shared abstract ops; first rung pulls 5+ records
  per heuristics §V row-coherence
- not measurement residue — confirmed (TAWR-EXT 5 Phase 6 emission cites the
  residual cells with named substrate cause: per-access recompute missing)
- measurable by matrix shift after landing — yes (test262 sample +
  diff-prod re-run will report cluster pass-rate delta)

Per heuristics §V, before any substrate edit:

```
rg -l 'custom-proto-access-resizes-buffer' /path/to/test262 | head -3
```

Inspect availability + cut_kind + abstract_op + surface + reason on the
three cells; confirm shared mechanism before editing.

## Carve-outs

- **DataView-first scope**. The first productive rung addresses DataView per
  the surfacing cells; TypedArray indexed-access OOB-on-resize is sibling
  scope under `typed-array-resizable-buffer-indexed-access` (already enrolled
  in the arc). If the first rung's substrate is shared between the two
  locales via a `Runtime`-level helper (per TAWR Finding TAWR.5 standing
  rec), promote the helper and cross-link rather than re-implementing.
- **No growable-SharedArrayBuffer in scope at founding**. Resizable
  ArrayBuffer is the substrate of record; the SAB-growable surface is
  deferred (lattice candidate, separate spec section, separate cell ring)
  pending an explicit spawn signal.
- **Coordinate scope, not surface scope**. Sub-shapes that diverge from
  the per-access-OOB shape (e.g., view aliasing across construction-time
  byte-offset arithmetic) spawn nested locales per Doc 737 §II.

## Composes-with

- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`
- `apparatus/docs/predictive-ruleset.md` (rules 4, 5, 6, 11, 13, 15 most
  relevant; rule 6 surface-completeness audit applies if DataView storage
  shape changes to add buffer-back-reference)
- `pilots/typed-array-wrong-result/` (parent surface; TAWR-EXT 5 Phase 6
  emitter)
- `pilots/typed-array-resizable-buffer-indexed-access/` (sibling locale
  in arc; TypedArray-side resizable-buffer cells)
- `pilots/buffer/` (host ArrayBuffer + resize substrate)
- `apparatus/arcs/2026-05-28-array-exotic-substrate/arc.md` (enrolling arc)

## Resume protocol

Read `trajectory.md` tail; if no rungs landed, the founding TAWR-EXT 5
Phase 6 emission + the three cited DataView cells are the entry point.
Run the exemplar suite once it exists; pick the first rung's mechanism
from a fresh per-cell inspection of the failure signatures.

**Status**: FOUNDED 2026-05-30 (helmsman session, keeper directive Telegram 10558). EXT 0 founding pending: apparatus scaffold (exemplar suite + runner + pool count) + baseline inspection of the three DataView cells.
