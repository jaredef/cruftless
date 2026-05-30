# typed-array-byte-storage-conformance — Trajectory

## TABSC-EXT 0 — LANDED (2026-05-30) — Vec<Value> → Vec<u8> + NumberToRawBytes / RawBytesToNumeric storage migration

**Trigger**: Keeper APPROVED of proposal `apparatus/proposals/pending/2026-05-30T182500Z-tabsc-ext-0-byte-storage-precursor/proposal.md` via Telegram 10596. Founding rung; founds the locale.

**Arc enrollment**: `2026-05-28-array-exotic-substrate` (fourth in-flight locale alongside TAWR closed at EXT 6, TAMM closed at EXT 10, TAECSF founded at EXT 0 + EXT 1 NEGATIVE Rule-13 revert + EXT 1.1 convergent diagnosis, RBDPA founded at EXT 0).

**Phase 1 (Spawn) per Doc 744 §V.1 four-tuple + observability**:
- **M** = any user-visible JS construct that reads or writes a typed-array element or DataView slot. ECMA-262 §10.4.5 IntegerIndexedExoticObject + §25.3 DataView.
- **T** = spec-faithful byte-level storage with NumberToRawBytes encoding per §6.1.6.1 at every write site; RawBytesToNumeric at every read site; view-aliasing pass-through preserved at the byte tier.
- **I** = six substrate transitions: (1) `ArrayBufferRecord.data: Vec<Value>` → `Vec<u8>`; (2) `TypedArrayViewRecord.element_kind: String` field added; (3) `abstract_ops::number_to_raw_bytes` + `raw_bytes_to_numeric` + `typed_array_byte_width` helpers per §6.1.6.1 Table 75; (4) `typed_array_set_index` rewrite (per-kind coercion + byte encoding); (5) `typed_array_get_index` rewrite (byte slice decode); (6) DataView setter/getter migration from `Value::Number(b as f64)` byte-per-cell wrapping to raw `u8` cell.
- **R** = three relational edges per Doc 744 §IV:
  - DAG mouth-gating ↑ `abstract_ops::convert_number_to_typed_array_element` (TAECSF-EXT 1 substrate prefix retained per Finding IR.33; consumed unchanged as the upstream coercion step before byte encoding) — CLOSED at spawn.
  - Lattice with DataView (`intrinsics.rs:19842-19865` setters + getters) — shared interior at the byte-encoding tier; bundled in this rung to prevent class-3 timing-edge regressions per Doc 744 §III.3.
  - Lattice (downstream cascade-revival receiver) ↓ with TAECSF sub-substrates (a) integer-kind + (b) Float32 canonical-NaN — Doc 739 prediction: cascade-revive without TAECSF-side substrate work.
- **Observability** = ordinary. No scaffold needed.

**Phase 2 (Baseline-inspect)** per Rule 23: pre-rung TAMM 86/100, TAWR 67/100, diff-prod 64/48. Substrate-start grep verification (proposal risks 5/6/7): only 2 direct `buf.data[N] = …` write sites enumerated (TA storage + DataView setter); 4 `TypedArrayViewRecord` insert sites; 3 `ArrayBufferRecord` insert sites; no cruftless-side direct buffer access; Buffer-shim uses JS-level dispatch; Atomics not exercising direct buffer access at this engagement.

**Phase 3 (Pin-Art probe if duplicated)**: not invoked — substrate move is single-site (one struct migration + one helper-pair add + two read/write rewrites + handful of insert-site updates).

**Phase 4 (Revert-then-deeper-layer if negative)**: not invoked — single round, positive.

**Substrate** (~210 LOC across three files):

1. `pilots/rusty-js-runtime/derived/src/interp.rs` (~80 LOC):
   - `ArrayBufferRecord.data: Vec<Value>` → `Vec<u8>` (line 451). Doc-comment cites Finding TAECSF.3 + TABSC-EXT 0 rationale.
   - `TypedArrayViewRecord.element_kind: String` field added (line 460+). Populated at view construction from the typed-array constructor name.
   - `typed_array_get_index` rewrite (line 586): read `[byte_index..byte_index + bytes_per_element]` from `buf.data`; dispatch on `view.element_kind`; call `raw_bytes_to_numeric`.
   - `typed_array_set_index` rewrite (line 604): silent (lossy) coercion via `to_bigint` (BigInt kinds) or `convert_number_to_typed_array_element` (integer/float kinds); encode via `number_to_raw_bytes`; write bytes into `buf.data[byte_index..byte_index + bytes_per_element]`.
   - `typed_array_set_index_checked` updated to propagate `to_bigint` errors (the user-visible Result-threaded path; spec-faithful per ECMA-262 §10.4.5.16 + §7.1.13).
   - `resize_array_buffer` migration (line 722): `Value::Number(0.0)` → `0u8`.

2. `pilots/rusty-js-runtime/derived/src/abstract_ops.rs` (~90 LOC):
   - `number_to_raw_bytes(kind, value) -> [u8; 8]` per §6.1.6.1 NumberToRawBytes. Returns 8-byte LE-encoded representation; caller writes only `typed_array_byte_width(kind)` bytes. DataView's setter wrapper handles BE byte-order via existing reversal logic.
   - `typed_array_byte_width(kind) -> usize` per §6.1.6.1 Element Size.
   - `raw_bytes_to_numeric(kind, bytes) -> Value` per §6.1.6.1 RawBytesToNumeric. Decodes a LE byte slice; DataView's getter wrapper handles BE via existing reversal.

3. `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (~40 LOC):
   - Three `ArrayBufferRecord` construction sites (lines 19652, 20016, 20143): `vec![Value::Number(0.0); byte_length]` → `vec![0u8; byte_length]`.
   - Four `TypedArrayViewRecord` construction sites (lines 18554, 19916, 20056, 20161): populate `element_kind` from the kind name (`kind.clone()` for subarray; `"DataView"` for DataView ctor; `n.clone()` for TypedArray ctors).
   - DataView setter (line 21988): `Value::Number(0.0)` → `0u8` for the resize-default; `Value::Number(b as f64)` → `b` for the per-byte write.
   - DataView getter (line 21889): `Some(Value::Number(n)) => *n as u8` → `.copied().unwrap_or(0)` for direct byte read.

**Yield**:

```text
View-aliasing probe (3 TAECSF-EXT 1 NEGATIVE cells):
  TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js: FAIL → PASS
  TypedArrayConstructors/internals/Set/conversion-operation-consistent-nan.js: FAIL → PASS
  TypedArray/prototype/some/BigInt/values-are-not-cached.js: FAIL → PASS

Cascade probe (10 cells, /tmp/probe-taecsf-1.js):
  10/10 PASS (Uint8 wrap 300→44; Int8 wrap 130→-126; Uint8Clamped NaN→0;
   Uint8Clamped saturate 300→255; round-half-to-even 254.5→254;
   Uint8Clamped -0.5→0; Int32 Infinity→0; Int32 large→wrap;
   BigInt64Array string-fail SyntaxError preserved)

Cluster gates:
  TAMM cluster PRE / POST:   86 → 87 / 100 (+1 PASS; ≥86 gate satisfied)
  TAWR cluster PRE / POST:   67 → 71 / 100 (+4 PASS; ≥67 gate satisfied)
  diff-prod PRE / POST:      64/48 → 64/48 (stable; ≥64/48 gate satisfied)
```

**Cascade-revival empirically validated (Doc 739 (B3))**: TAECSF sub-substrates (a) integer-kind + (b) Float32 canonical-NaN are now correct via the upstream storage-tier constraint-closure, with ZERO TAECSF-side substrate work. The 10/10 cascade probe IS the post-landing observability that Doc 739 (B3) requires.

**Cluster yield exceeded Doc 740 multi-tier prediction**: Doc 740 + Doc 741 predicted "substrate-introduction signature (≈0% cluster movement)" at this rung pending subsequent tier-closures. Actual: +5 cluster cells at this rung alone. The reason: with byte storage, the BigInt-TA + integer-kind coercion that TAECSF-EXT 0 + EXT 1 prepared all suddenly works correctly because the storage now properly separates the bytes per element. The cascade-revival predicted in Doc 739 + Doc 740 materialized AT THE PRECURSOR RUNG, not only at subsequent tier-closures. Worth recording as a candidate amendment to Doc 740's epistemic-shape framing.

**Gates**: build PASS (`cargo build --release --bin cruft -p cruftless`, 1m 05s); runtime lib tests 74/0/1 ignored; view-aliasing probe 3/3 PASS; cascade probe 10/10 PASS; cluster gates all satisfied with net positive yield.

**Tag**: `tabsc-ext-0-byte-storage-precursor`.

**Finding TABSC.1 (cascade-revival can materialize cluster yield at the precursor rung, not only at subsequent tier-closures)**: when the upstream constraint-closure substrate move ALSO produces the downstream tier's required encoding format (here, byte storage IS both the upstream constraint-closure AND the byte-encoding tier per §6.1.6.1), the cascade-revival materializes immediately rather than at a separate subsequent rung. This amends Doc 740 §II.2 P4's "cumulative reclaim materializes at the final-tier-closure round" with: "cumulative reclaim may materialize at the precursor rung itself when the precursor's substrate is structurally complete for the downstream tier's requirements." Promotion-readiness: one-more-observation; candidate apparatus standing-rule amendment to Doc 740.

**Phase 5 (chapter-close-inspect)**: post-rung TAMM top-row failure-table inspection shows residual 7 ArrayBuffer + 5 TypedArrayConstructors + 3 DataView + 2 TypedArray — these are not the TAECSF-EXT 1 NEGATIVE cells (those are now PASS); residual is the pre-existing TAMM long tail unrelated to the storage migration. No new failure shapes introduced.

**Phase 6 (deferral surfacing)**: the founding rung closes the locale's primary scope (storage migration). Sub-substrates that may surface in future rungs: DataView coercion-faithfulness (the saturating-cast vs modular-reduction divergence flagged in TAECSF-EXT 1 proposal §Risk Assessment is now structurally addressable since DataView setters operate at the byte tier); Atomics direct-access if a test262 cell surfaces it; JIT-tier acceleration of TA reads/writes per Doc 741 5-axis A4. None of these are gated by the locale's current scope; they may be deferred to sibling locales.

**Status**: TABSC-EXT 0 LANDED. Cascade-revival empirically validated (Doc 739 (B3) + Doc 740 P4 amendment candidate via Finding TABSC.1). Arc-tier accumulation: fourth productive locale in `2026-05-28-array-exotic-substrate`. The substrate-architectural constraint that Finding TAECSF.3 named as blocking is now CLOSED.

## TABSC-EXT 1 — LANDED (2026-05-30) — DataView coercion-faithfulness; spec-modular ToIntN/ToUintN replaces Rust saturating casts

**Trigger**: Keeper APPROVED of proposal `apparatus/proposals/pending/2026-05-30T185800Z-tabsc-ext-1-dataview-coercion-faithful/proposal.md` via Telegram 10604. First substrate move under the "implement against findings" mandate (Telegram 10602); direct application of Rules 27 + 28 (promoted at cycle 2, commit 1a0bdc29).

**Phase 1 (Spawn)** per Doc 744 four-tuple + observability:
- **M** = `dv.setIntN(off, v)` / `dv.setUintN(off, v)` per ECMA-262 §25.3.
- **T** = spec-faithful per §6.1.6.1 NumberToRawBytes + §7.1.6–§7.1.8 modular reduction.
- **I** = single transition — replace integer-cast branches in `data_view_set_number` with `abstract_ops::number_to_raw_bytes(kind, &Value::Number(n))` calls.
- **R** = DAG mouth-gating prereq ↑ closed by TABSC-EXT 0 (`number_to_raw_bytes` helper). Lattice (downstream) ↓ with TA storage path (byte-tier symmetry restored).
- **Observability** = ordinary.

**Phase 2 (Baseline-inspect)** per Rule 23: pre-rung gates verified — TAMM 87, TAWR 71, diff-prod 64/48; `data_view_set_number` at lines 21968-21979 uses Rust `n as i8/u8/i16/u16/i32/u32` saturating casts; spec mandates modular reduction.

**Substrate** (~15 LOC in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`):

The integer branch of `data_view_set_number` now dispatches on `(signed, byte_count)` to map to a TA-kind string ("Int8Array" / "Uint8Array" / "Int16Array" / "Uint16Array" / "Int32Array" / "Uint32Array") and calls the existing `abstract_ops::number_to_raw_bytes(kind, &Value::Number(n))` for spec-modular reduction. Float branch (`if float`) unchanged — Rust's `f64 → f32` cast is spec-correct.

**Yield**:

```text
Direct probe (7 cells, /tmp/probe-tabsc-1.js): 7/7 PASS
  setUint8 wrap 300→44; setInt8 wrap 130→-126; setUint32 wrap 4294967301→5;
  setInt32 NaN→0; setUint16 fractional truncate 3.7→3;
  setFloat64 pass-through 3.14 ✓; setFloat32 pass-through 1.5 ✓.

Cluster gates (stable post-rung):
  TAMM: 87 / 100 (≥87 satisfied)
  TAWR: 71 / 100 (≥71 satisfied)
  diff-prod: 64/48 / 112 (≥64/48 satisfied)
```

The cluster instruments did not flip at this rung — the DataView coercion cells likely live in test262 paths not in the curated TAMM/TAWR/diff-prod pools. Full test262 sample re-run would surface the cluster movement; out of scope for this rung's gates (post-gate criterion was regression-preservation only). Direct probe cells flipping from saturating-cast to spec-modular IS the closure signal per Rule 21's probe-first discipline.

**Phase 3-4**: not invoked.

**Phase 5 (chapter-close-inspect)**: post-rung cluster top-row inspection shows residuals unchanged from TABSC-EXT 0 post-rung state. No new failure shapes introduced. The DataView coercion gap (Rust saturating vs spec modular) is now closed at the runtime tier; any remaining DataView failures are at orthogonal substrate axes (e.g., big-endian byte-order edge cases, isLittleEndian-undefined behavior, detached-buffer semantics).

**Gates**: build PASS (1m 06s); runtime lib 74/0/1 ignored; cluster regression gates all preserved.

**Tag**: `tabsc-ext-1-dataview-coercion-faithful`.

**Finding TABSC.2 (substrate prefix amortization across cascade-revival-driven rungs)**: when an upstream architectural rung (TABSC-EXT 0) lands a helper as part of its substrate, downstream rungs that exploit the helper for previously-blocked substrate work (TABSC-EXT 1 here, TAECSF cascade-revival earlier) land with near-zero marginal substrate cost. The TABSC-EXT 0 helper `number_to_raw_bytes` was authored once and is now consumed by (a) the TA write path inside TABSC-EXT 0 itself, (b) the cascade-revived TAECSF integer-kind coercion at zero TAECSF-side cost, (c) this rung's DataView setters at ~15 LOC of dispatch glue. Per Finding IR.33 cumulative substrate amortization, the helper is positioned for any future consumer at the byte-encoding tier. Standing rec: at architectural rungs, factor coercion + encoding helpers into reusable abstract ops rather than inlining; the amortization compounds across the cascade-revival receivers.

**Promotion-readiness**: one-more-observation. TABSC.2 has two instances within TABSC's own trajectory (EXT 0's TA coercion + EXT 1's DataView coercion); both consume the same helper but at adjacent dispatch sites within one locale's scope. Awaits a cross-locale instance (e.g., a non-TA non-DataView abstract op pattern exploiting a sibling locale's helper as substrate prefix) before standing-rule promotion.

**Status**: TABSC-EXT 1 LANDED. Cascade-revival of the original DataView-coercion-faithfulness gap per Rule 27 + Rule 28. Substrate prefix from TABSC-EXT 0 consumed at ~15 LOC marginal cost. Cluster gates preserved; direct probe 7/7 PASS.
