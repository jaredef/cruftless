# typed-array-missing-method — Trajectory

## TAMM-EXT 0 — founding + exemplar suite + baseline-TBD (2026-05-25)

**Trigger**: Top-10 spawn batch per keeper directive after canonical
full-suite Pin-Art zoom-out. This is rank #9 of the matrix
(469 fails) and is implement-chapter / intrinsic completion work per heuristics §IV.A+D.

**Apparatus established**:

- `exemplars/exemplars.txt` — 100 stratified-sample paths.
- `exemplars/run-exemplars.sh` — runner.
- `exemplars/pool-size.txt`, `exemplars/family-breakdown.txt` —
  inventory.

**Baseline**: TBD on next run of `exemplars/run-exemplars.sh`. Expected
near 0/100 given the cluster coherence; record value here.

**Status**: TAMM-EXT 0 founding closed. Apparatus operational; first
substrate rung pending exemplar-fail family-marginal inspection per
heuristics §V row-coherence protocol.

## TAMM-EXT 1 — LANDED (2026-05-27) — ArrayBuffer.prototype accessor surface

Per keeper directive Telegram 10070 ("Let's avoid other agents while grabbing the highest yield locales"). Parallel agent's recent surface was Temporal/intl402/parser/direct-eval/tagged-template; the buffer-typed-array surface (matrix ranks 2 + 10 + 20 + 21 ≈ 1800 fails) was not on theirs. TAMM is the missing-method-or-intrinsic stratum.

**Substrate** (~110 LOC in intrinsics.rs + ~30 LOC in interp.rs fast-path):
- New `install_ab_accessor` helper installs a real accessor descriptor (getter + writable:f + enumerable:f + configurable:t) on ArrayBuffer.prototype.
- Installed accessors: `byteLength`, `maxByteLength`, `resizable`, `detached`. Each reads through `rt.array_buffers`; absent record throws TypeError per §25.1.5.{1,2,3,4}.
- Installed `ArrayBuffer[Symbol.species]` getter on the ctor per §25.1.4.3.
- Mirror fast-path lookups in `interp.rs::object_get` for the same accessors.

**Yield**:
```text
TAMM cluster exemplars PRE-EXT 1:  PASS=3 FAIL=97 / 100 (3.0%)
TAMM cluster exemplars POST-EXT 1: PASS=18 FAIL=82 / 100 (18.0%)
```
**+15 PASS** in one rung. ArrayBuffer family residual 24 → 9. Remaining 82 fails: TypedArray prototype methods (27), TypedArrayConstructors (24), DataView prototype methods (21), ArrayBuffer residual (9).

**Gates**: build clean; diff-prod 59/53 (parity).

**Standing rec TAMM.1**: per-property accessor installation on a built-in prototype must register a real PropertyDescriptor with the `getter` field set (not just route through the engine's fast-path), so `Object.getOwnPropertyDescriptor` reports the accessor correctly. Fast-path lookups remain as an optimization; the accessor descriptor is the spec-conformant ground truth.

**Status**: TAMM-EXT 1 CLOSED locally.

## TAMM-EXT 2 — LANDED (2026-05-27) — DataView.prototype accessor surface

Per keeper directive Telegram 10072 ("Continue").

**Substrate** (~160 LOC in intrinsics.rs):
- Allocated a dedicated `dv_proto` (DataView.prototype) — pre-EXT 2 DataView shared `ta_proto` with TypedArrays, which was structurally wrong (DataView.prototype !== TypedArray.prototype per ECMA-262 §25.3).
- Installed accessors on dv_proto via the same accessor-descriptor pattern as TAMM-EXT 1: `byteLength` / `byteOffset` / `buffer`. Each performs a receiver-type check (`__kind === "DataView"`) and reads through `rt.typed_array_views` (the buffer+offset+length registry; DataView uses bytes_per_element=1 since DataView is byte-granular).
- Replaced DataView's path through the generic typed-array ctor loop with its own dedicated ctor that requires the first arg to be an ArrayBuffer, parses byteOffset + fixed_length, and stores the record in typed_array_views with the correct __kind tag.
- DataView removed from the generic typed-array-name loop (no longer initialized through the TypedArray pathway).

**Yield**:
```text
TAMM cluster exemplars POST-EXT 1: PASS=18 FAIL=82 / 100 (18.0%)
TAMM cluster exemplars POST-EXT 2: PASS=36 FAIL=64 / 100 (36.0%)
```
**+18 PASS** in one rung. DataView family residual 21 → 3.

**Gates**: build clean; diff-prod 59/53 (parity).

**Direct probes**:
- `(new DataView(new ArrayBuffer(8), 2, 4)).byteLength === 4` ✅ (was 0)
- `(new DataView(new ArrayBuffer(8), 2, 4)).byteOffset === 2` ✅ (was undefined)
- `typeof (new DataView(new ArrayBuffer(8), 2, 4)).buffer === "object"` ✅ (was undefined)

**Standing rec TAMM.2**: when a built-in's prototype is structurally distinct from its sibling's, the locale founding should verify prototype-identity (`SubA.prototype !== SubB.prototype`) before installing shared methods. Cruft's pre-EXT 2 ta_proto-shared-with-DataView was a single-line setup that papered over a real spec divergence; the per-prototype install pattern is now established for DataView and ready for TAMM-EXT 3 (TypedArray method surface) + TAMM-EXT 4 (DataView getInt8/setInt8/...) without re-architecting.

**Next rung options inside TAMM**:
- TAMM-EXT 3: TypedArray.prototype.{at, slice, subarray, copyWithin, fill, ...} method surface (27 TypedArray fails).
- TAMM-EXT 4: DataView.prototype.getInt8/setInt8/getFloat64/setFloat64 etc. method surface (remaining 3 DataView fails likely; could expand to actual byte read/write methods).
- TAMM-EXT 5: ArrayBuffer.prototype.transfer / .transferToFixedLength / .slice (the 9 remaining ArrayBuffer fails).

**Status**: TAMM-EXT 2 CLOSED locally.
