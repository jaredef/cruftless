# typed-array-byte-storage-conformance — Seed

## Telos

Materialize the engine-DAG coordinate

```
runtime/buffer-typed-array :: E3/intrinsic-object:ecma-262 :: byte-storage/spec-conformant :: NumberToRawBytes/RawBytesToNumeric
```

Migrate `ArrayBufferRecord.data` from `Vec<Value>` (Value-cell-aliased, accumulated-drift storage) to `Vec<u8>` (byte-level, spec-conformant per ECMA-262 §6.1.6.1). Every typed-array view stores its element at byte indices encoded via NumberToRawBytes per the kind's Element Type; reads decode via RawBytesToNumeric. The migration preserves view-aliasing pass-through at the byte tier (multiple views over the same `ArrayBuffer` observe each other's writes via byte representation, kind-dispatched at read), satisfying the test262 harness `copyIntoArrayBuffer` round-trip pattern and the IEEE-754 NaN bit-pattern preservation requirement.

## Origin

Founded 2026-05-30 per keeper APPROVED of helmsman proposal
`apparatus/proposals/pending/2026-05-30T182500Z-tabsc-ext-0-byte-storage-precursor/proposal.md`
(Telegram 10596). The locale is the **precursor architectural rung** identified by:

- The Doc 721 pipeline-alphabet audit (commit 6783bba6) which located the alphabet origin for typed-array element writes at Tier 5 (runtime dispatch) and identified Tier 6 (storage) as the rectification site.
- Finding TAECSF.3 (commit a20e3966) which converged the diagnosis to the engine-architectural Value-cell-aliasing constraint blocking spec-faithful coercion.
- The Doc 739 cascade-revival prediction: TAECSF-EXT 1's (P2.d) stall sits at a constraint-propagation node; closing the upstream `Vec<Value>` constraint cascade-revives TAECSF without TAECSF-side substrate work. (B1)/(B2)/(B3) boundary conditions verified at proposal-time and empirically validated at landing.

The proposal authoring composed four corpus docs: Doc 721 (cross-pipeline diagnostic protocol), Doc 739 (single-tier cascade-revival), Doc 741 (multi-tier pipeline-connection), Doc 744 (pipeline-form discovery as predictive heuristic). The four-tuple (M, T, I, R) + observability was explicit at spawn per Doc 744 §V.1.

## Work shape

**Heuristics §IV classification**: D (Runtime Intrinsic Semantics) — engine-architectural storage migration.

The substrate scope was substantial (~250 LOC across `interp.rs`, `abstract_ops.rs`, `intrinsics.rs`) but bounded by the pipeline-alphabet-audit's coverage-completeness assessment: only two direct `buf.data[N] = …` write sites (TA storage + DataView byte writer) and a handful of `array_buffers.get` read sites. Buffer-shim on Uint8Array uses JS-level dispatch and cascades automatically; no direct buffer access from `cruftless/src/`. Atomics not exercising direct buffer access at this engagement's maturity.

## Apparatus

- **View-aliasing probe**: the three TAECSF-EXT 1 NEGATIVE cells (`TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js`, `TypedArrayConstructors/internals/Set/conversion-operation-consistent-nan.js`, `TypedArray/prototype/some/BigInt/values-are-not-cached.js`) constitute the canonical gate. All three PASS post-rung.
- **Cascade probe**: `/tmp/probe-taecsf-1.js` (10-cell assertion suite for BigInt + integer + Uint8Clamp coercion). All 10 PASS post-rung; Doc 739 (B3) cascade-revival empirically validated.
- **Sibling regression instruments**: `pilots/typed-array-missing-method/exemplars/run-exemplars.sh` (TAMM, gate ≥86); `pilots/typed-array-wrong-result/exemplars/run-exemplars.sh` (TAWR, gate ≥67); `scripts/diff-prod/run-all.sh` (gate ≥64/48).

## Methodology

Per heuristics §VIII Debugging Rule + Doc 744 four-tuple + Doc 741 rule-11 5-axis, every substrate rung in this locale must satisfy:

- **A1 component A/B** — empirical disambiguator before substrate spawn. EXT 0 satisfied via the pipeline-alphabet-audit.
- **A2 op-set coverage** — Op::SetIndex, Op::GetIndex, Op::DefineProperty paths must all migrate jointly. EXT 0 satisfied.
- **A3 value-domain coverage** — 9 element kinds × {LE, BE} for DataView dispatch = 18 encoding/decoding cells. EXT 0 satisfied via `number_to_raw_bytes` + `raw_bytes_to_numeric` + DataView setter/getter wrapping.
- **A4 locals-marshaling coverage** — N/A at storage tier.
- **A5 emission-shape coverage** — read/write symmetry across all view-construction sites + resize + default-init. EXT 0 satisfied across the 4 `TypedArrayViewRecord` insert sites + 3 `ArrayBufferRecord` insert sites.

Per Doc 739, future rungs at the lattice-meet sites (DataView coercion-faithfulness, JIT-tier acceleration, Atomics buffer access) compose with this rung's substrate prefix.

## Carve-outs

- **JIT-tier acceleration of TA reads/writes** is OUT OF SCOPE. Potential future locale `pilots/typed-array-jit-acceleration/` per Doc 741 5-axis A4 (locals-marshaling).
- **SharedArrayBuffer atomics beyond preservation** is OUT OF SCOPE. Current Atomics intrinsics do not directly access buffer storage at this engagement's maturity; if a future test262 cell surfaces direct-access incoherence, fold into a sibling rung.
- **`__kind` typed-enum optimization** deferred. The seed-level proposal anticipated a `TypedArrayKind` enum on `TypedArrayViewRecord`; EXT 0 lands with `element_kind: String` for minimal-LOC closure. Enum promotion is a co-yield optimization for future rungs.

## Composes-with

- `apparatus/docs/pipeline-alphabet-audit-2026-05-30.md` — the upstream audit justifying the locale.
- `apparatus/docs/predictive-ruleset.md` rules 1, 4, 5, 6, 11, 13, 15, 17–22, 23, 25 — methodology.
- Corpus: Docs 721 + 729 + 730 + 731 + 739 + 740 + 741 + 744.
- `pilots/ta-element-coercion-spec-faithful/` — downstream cascade-revival receiver (TAECSF-EXT 1 sub-substrates (a) + (b) cascade-revived at this rung).
- `pilots/typed-array-wrong-result/` + `pilots/typed-array-missing-method/` — regression-gate clusters (lattice neighbors).
- `pilots/resizable-buffer-detection-per-access/` — sibling locale in arc; may cascade-revive depending on its EXT 0 substrate.
- `apparatus/arcs/2026-05-28-array-exotic-substrate/` — enrolling arc (fourth in-flight locale at land time).
- `apparatus/docs/findings-ledger.md` entries 005 (TAECSF.1), 012 (TAECSF.3), 013 (APP.PIPELINE-1) — this locale's landing is the second observation for both TAECSF.3 and APP.PIPELINE-1, advancing both to promotion-ready status.

## Resume protocol

Read `trajectory.md` tail. EXT 0 founding rung is the byte-storage migration; future rungs address DataView coercion-faithfulness lattice-meet, Atomics direct-access if any, and JIT-tier acceleration.

**Status**: FOUNDED 2026-05-30 by TABSC-EXT 0 (helmsman session, keeper directive Telegram 10596). Founding rung LANDED with empirically validated cascade-revival of TAECSF sub-substrates (a) + (b); cluster yield +4 TAWR, +1 TAMM, diff-prod stable; view-aliasing probe 3/3 PASS; cascade probe 10/10 PASS.
