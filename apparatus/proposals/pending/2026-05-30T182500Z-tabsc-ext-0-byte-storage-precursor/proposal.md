---
helmsman_session: helmsman-2026-05-30-precursor-architectural-rung
proposed_commits:
  - pending
target_branch: main
summary: Found `pilots/typed-array-byte-storage-conformance/` (TABSC-EXT 0); migrate `ArrayBufferRecord.data` from `Vec<Value>` to `Vec<u8>` with NumberToRawBytes encoding per ECMA-262 §6.1.6.1; rewrite `typed_array_set_index` + `typed_array_get_index` with read/write symmetry; update view-construction sites + resize_array_buffer; promote `__kind` to typed `TypedArrayKind` enum on `TypedArrayViewRecord` (co-yield).
risk_class: substrate-architectural
gates_pre:
  diff_prod: 64 PASS / 48 FAIL (post-TAECSF-EXT 0 baseline; post-EXT-1 revert preserved)
  tamm_cluster: 86 / 100
  tawr_cluster: 67 / 100
  test262_sample: 86.7% per 2026-05-27 canonical
gates_post:
  build: cargo build --release --bin cruft -p cruftless — pending
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib — pending; must remain 74 passed / 0 failed / 1 ignored
  diff_prod: ≥ 64/48 (regression gate; substrate-introduction signature predicts ≈ 64/48 ± 1)
  tamm_cluster: ≥ 86/100 (regression gate; substrate-introduction signature predicts ≈ 86 ± 1)
  tawr_cluster: ≥ 67/100 (regression gate; substrate-introduction signature predicts ≈ 67 ± 1)
  view_aliasing_probe: the test262 harness's `copyIntoArrayBuffer` round-trip pattern must hold at the byte tier (specific cell: `built-ins/TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js` must PASS post-rung, the canonical view-aliasing gate cell that TAECSF-EXT 1 NEGATIVE surfaced)
  cascade_prediction: TAECSF sub-substrate (a) integer-kind + (b) Float32 canonical-NaN remain DEFERRED (cascade-revival materializes in subsequent rungs, NOT this rung)
---

## Substrate Moves

Per keeper directive Telegram 10594 ("Draft"), following the four-corpus-doc reading (721 + 739 + 741 + 744) directed by Telegram 10584. Founds the precursor architectural locale identified by:

1. **Doc 721 pipeline audit** (commit 6783bba6): alphabet origin for `ta[i] = v` is at Tier 5 (Runtime dispatch); rectification must land at Tier 6 (Storage). `Vec<Value>` is accumulated drift, not deliberate substrate.
2. **TAECSF.3 finding** (commit a20e3966): direct empirical anchor — coercion at SetIndex breaks view-aliasing pass-through under `Vec<Value>` storage; spec coercion + view aliasing cannot coexist without byte-tier storage.
3. **Doc 739 cascade-revival**: TAECSF-EXT 1's (P2.d) stall sits at a constraint-propagation node; closing the upstream `Vec<Value>` constraint will cascade-revive TAECSF without TAECSF-side substrate work. Three boundary conditions (B1 / B2 / B3) verified in audit-ledger Entry 005.
4. **Doc 744 pipeline-form discovery**: the four-tuple (M, T, I, R) + observability for this rung is fully derivable; all four-tuple elements explicit at spawn predict closure ≤3 rounds.

### §1 Doc 744 four-tuple (M, T, I, R) + observability

**M (mouth)** — any user-visible JS construct that reads or writes a typed-array element or DataView slot. ECMA-262 §10.4.5 IntegerIndexedExoticObject + §25.3 DataView. Concretely: `ta[i] = v` write, `ta[i]` read, `dv.getInt32(off)` / `dv.setInt32(off, v)`, plus the internal-method paths used by `Object.defineProperty(ta, i, desc)` and the harness aliasing pattern at test262/harness/testTypedArray.js:107.

**T (terminus)** — spec-faithful byte-level storage with `NumberToRawBytes(kind, value, isLittleEndian)` per §6.1.6.1 at every write site; `RawBytesToNumeric(kind, bytes, isLittleEndian)` per §6.1.6.1 at every read site; view-aliasing pass-through preserved at the byte tier (multiple views over the same `ArrayBuffer` observe each other's writes via byte representation, kind-dispatched at read).

**I (interior contour)** — six substrate-tier transitions:

1. `ArrayBufferRecord.data: Vec<Value>` → `Vec<u8>`. `byte_length` becomes the authoritative length (currently shadowed by `data.len()`); `data.len() == byte_length` invariant explicit.
2. `TypedArrayViewRecord` gains `element_kind: TypedArrayKind` field (typed enum: Int8, Uint8, Uint8Clamped, Int16, Uint16, Int32, Uint32, Float32, Float64, BigInt64, BigUint64). The String `__kind` slot on the instance object retains its current role for spec-observable `Object.getPrototypeOf(ta).constructor.name` semantics but is no longer the runtime dispatch key.
3. `abstract_ops` gains `number_to_raw_bytes(kind, value) -> SmallVec<u8>` + `raw_bytes_to_numeric(kind, bytes) -> Value`. Little-endian native (per §6.1.6.1 default; DataView's isLittleEndian arg dispatches the byte swap). The TAECSF-EXT 1 substrate-prefix `convert_number_to_typed_array_element` (retained per Finding IR.33) becomes the upstream coercion step before bytes are encoded.
4. `typed_array_set_index` rewrite: dispatch on `view.element_kind`; call `convert_number_to_typed_array_element(value, kind)` then `number_to_raw_bytes(kind, coerced)`; write the resulting bytes into `buf.data[byte_index..byte_index + bytes_per_element]`. Spec-faithful per §10.4.5.16 IntegerIndexedElementSet + §6.1.6.1.
5. `typed_array_get_index` rewrite: read `buf.data[byte_index..byte_index + bytes_per_element]`; dispatch on `view.element_kind`; call `raw_bytes_to_numeric(kind, bytes)`. Spec-faithful per §10.4.5.15 IntegerIndexedElementGet + §6.1.6.1.
6. View-construction sites update: every `typed_array_views.insert(id, TypedArrayViewRecord { ... })` populates the new `element_kind` field. `resize_array_buffer` migrates from `buf.data.resize(new_len, Value::Number(0.0))` to `buf.data.resize(new_len, 0u8)`. Constructor populate paths (`for (i, v) in values.into_iter().enumerate() { rt.object_set(id, i.to_string(), v); }`) continue to route through `object_set` → `object_set_pk` → `typed_array_set_index`; the new spec-faithful storage path is invoked transparently.

**R (relations)** — three relational edges per Doc 744 §IV:

- **DAG (mouth-gating prerequisite) ↑** with `abstract_ops`: the spec-coercion helper `convert_number_to_typed_array_element` already exists on disk (substrate prefix from TAECSF-EXT 1, retained per Finding IR.33). T₁ (helper) feeds M₂ (storage write). Prerequisite is CLOSED at spawn; no Doc 744 §IV.1.a mouth-gating block.
- **Lattice with DataView** (`pilots/rusty-js-runtime/derived/src/intrinsics.rs:19842–19865`): shared interior at the byte-encoding tier; distinct mouth (DataView API methods `setInt32`/`getInt32`/etc.) + distinct terminus (DataView spec-mandated little/big-endian handling). DataView setters currently use Rust saturating `as` casts on Value-stored representation — same architectural conflict as TA at Tier 6. Lattice-meet at the byte-encoding tier is the natural simultaneous closure. Addressed jointly in this rung to prevent class-3 timing-edge regressions (Doc 744 §III.3).
- **Lattice (downstream cascade-revival receiver) ↓** with TAECSF: TAECSF locale sub-substrates (a) integer-kind + (b) Float32 canonical-NaN are downstream consumers. Per Doc 739 (P3) the cascade-revival materializes WITHOUT TAECSF-side substrate work once this rung lands. The cascade gate cell is the byte-storage view-aliasing probe (the same test262 cell TAECSF-EXT 1 surfaced).

**Observability** — ordinary. No panic/abort/NOJSON/timeout expected. Doc 744 §III.4 observability scaffold not introduced at this rung. The risk that a refactor-induced class-3 timing edge produces a class of cell regressions is explicit and gated by the post-gates (any cluster regression triggers Rule 13 + the Doc 740 multi-tier framing's "is this an additional tier in R?" probe).

### §2 Doc 721 chain-bundle gated population (G)

Per Doc 721 Step 1, enumerate the gated population for the named symptom class "typed-array element-write / read coercion + view-aliasing pass-through":

- **G₁ — TAECSF-EXT 1 NEGATIVE direct cells (3)**: `TypedArrayConstructors/internals/GetOwnProperty/BigInt/index-prop-desc.js`, `TypedArrayConstructors/internals/Set/conversion-operation-consistent-nan.js`, `TypedArray/prototype/some/BigInt/values-are-not-cached.js`. These were the three cells whose regression under TAECSF-EXT 1 surfaced the architectural constraint.
- **G₂ — TAWR locale residual coercion cells (~10 estimated)**: BigInt64/BigUint64 string-tobigint, key-is-numericindex-desc-not-writable, conversion-operation paths.
- **G₃ — DataView coercion cells (~6 estimated)**: setInt32/setUint8/etc. with non-spec saturating-cast vs modular-reduction divergence (the secondary gap surfaced in TAECSF-EXT 1 proposal §Risk Assessment).
- **G₄ — Resizable-buffer cross-view cells (~3 estimated)**: view-aliasing through `copyIntoArrayBuffer` harness pattern with resize.

Predicted gated-population cardinality |G| ≈ 22 cells. Per Doc 721 Step 4, predicted unlock count U is **bounded above by 22** but per the multi-tier-cascade signature (Doc 740 + 741), this rung's standalone yield is predicted at **substrate-introduction (≈0%)**; cumulative yield materializes at subsequent tier-closure rungs (TABSC-EXT 1 integer coercion, TABSC-EXT 2 Float32 NaN, TABSC-EXT 3 DataView migration if not bundled here).

The substrate-introduction signature (Doc 741 P2 + Finding II.2-bis) is the predicted gate movement at this rung: cluster instruments ±1 cell variance, no large delta. The cluster-connection round will be the final TABSC rung when read/write symmetry + DataView lattice-meet + integer-kind coercion all compose; expected at TABSC-EXT 2 or 3.

### §3 Doc 741 rule-11 5-axis check

1. **A1 component A/B probe**: ✓. Empirical disambiguator already run: pipeline-alphabet-audit-2026-05-30.md walked all 7 tiers; TAECSF.3 finding empirically identified Tier 6 as the dominator. The audit IS the rule-11 axis-1 closure for this rung.
2. **A2 op-set coverage**: ✓ at spawn. Three opcodes touch the storage: `Op::SetIndex` (line 14058), `Op::GetIndex` (read site; not yet read-traced but symmetric), `Op::DefineProperty` (line 3848 path). All three call sites of `typed_array_set_index` / `typed_array_get_index` migrate in lockstep.
3. **A3 value-domain coverage**: ✓ at spawn. 9 element kinds × {LE, BE} for DataView dispatch = 18 encoding/decoding cells. Spec-mappable per §6.1.6.1 Table 75 (Element Size + Conversion Operation + IsBigIntElementType columns). Helper `number_to_raw_bytes` + `raw_bytes_to_numeric` cover the 18 cells.
4. **A4 locals-marshaling coverage**: N/A at storage tier. JIT-tier acceleration of TA reads/writes is a separate downstream concern (potential future locale `pilots/typed-array-jit-acceleration/`); does not interact with this rung's substrate.
5. **A5 emission-shape coverage**: ✓ at spawn. The emission-shape risk is read/write symmetry — every site that constructs a `TypedArrayViewRecord`, resizes a buffer, or initializes buffer defaults must migrate jointly. Enumerated emission sites: (a) ArrayBuffer ctor (intrinsics.rs ~line 19850); (b) TypedArray ctor closure (`make_native_with_length` family + per-name ctor branches around line 20040); (c) DataView ctor (line 19909); (d) `subarray` / `slice` paths producing new views; (e) `resize_array_buffer` (interp.rs line 627); (f) Buffer-on-Uint8Array shim (cruftless/src/node_stubs.rs `install_buffer_methods`); (g) `Atomics` buffer access (if any; needs grep verification). Each emission site must populate the new `element_kind` field.

All applicable axes resolvable at spawn. Per Doc 741 §III the 5-axis check is the prerequisite for predicting a non-zero reclaim ceiling; with all axes closing in dependency order this rung's substrate is structurally complete.

### §4 Doc 744 §V.1 spawn discovery — checklist resolution

- (1) M named: above (§1 M).
- (2) T named: above (§1 T).
- (3) I sketched: above (§1 I, six transitions).
- (4) R discriminated: above (§1 R, three relations per Doc 744 §IV.4).
- (5) Mouth-gating prerequisites: ✓ — `abstract_ops::convert_number_to_typed_array_element` is the upstream coercion artifact and lives on disk (TAECSF-EXT 1 substrate prefix). No Doc 744 §IV.1.a block.
- (6) Observability classification: ordinary per §1.

All six spawn-discovery elements explicit. Per Doc 744 §VI rounds-to-closure formula (count of implicit elements + 1), closure prediction is **≤1 round** for this rung's named substrate. The Doc 740 multi-tier framing applies separately: this rung is one tier of a multi-tier R; cumulative substrate-amortization materializes at the final tier-closure in the TABSC locale's trajectory (likely EXT 2 or 3).

### §5 Substrate scope estimate

~250–350 LOC across:

- `pilots/rusty-js-runtime/derived/src/interp.rs` (~120 LOC): `ArrayBufferRecord.data` type change; `TypedArrayViewRecord.element_kind` field add + `TypedArrayKind` enum; `typed_array_set_index` + `typed_array_get_index` rewrites; `resize_array_buffer` migration.
- `pilots/rusty-js-runtime/derived/src/abstract_ops.rs` (~80 LOC): new `number_to_raw_bytes` + `raw_bytes_to_numeric` helpers per §6.1.6.1 Table 75; existing `convert_number_to_typed_array_element` retained (provides the coercion step upstream of byte encoding).
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (~70 LOC): every `typed_array_views.insert` site adds `element_kind`; ArrayBuffer / DataView / TypedArray constructors migrate buffer init from `Value::Number(0.0)` defaults to `0u8`; DataView setters/getters at lines 19842–19865 route through the new helpers.
- `cruftless/src/node_stubs.rs` (~30 LOC if Buffer storage interacts; needs grep verification): Buffer shim on Uint8Array may have direct buffer-data access that needs migration.

### §6 Founds new locale

`pilots/typed-array-byte-storage-conformance/` with:

- `seed.md` — telos (Vec<u8> + NumberToRawBytes storage with view-aliasing-at-byte-tier invariant); apparatus (this rung + the 22-cell gated population G; subsequent rungs close the cascade); methodology; carve-outs (no JIT-tier acceleration; no SharedArrayBuffer atomics beyond preservation); composes-with (Doc 721 + 739 + 740 + 741 + 744 + TAECSF locale).
- `trajectory.md` — TABSC-EXT 0 founding rung with the four-tuple (M, T, I, R) per Doc 744 §V.1; cite Doc 739 cascade-revival prediction for TAECSF + Doc 741 epistemic-shape framing.
- `findings.md` — defer extraction until second productive rung (per per-locale convention; one rung is not enough to warrant a findings.md per Finding TAECSF.1 precedent).

Arc enrollment: `2026-05-28-array-exotic-substrate` as the fourth in-flight locale alongside TAWR (closed at EXT 6), TAMM (closed at EXT 10), RBDPA (founded EXT 0 + pending), TAECSF (founded EXT 0 + EXT 1 NEGATIVE rule-13 revert).

Ledger updates:
- `apparatus/docs/deferrals-ledger.md`: no new entry; the sub-substrates (a) + (b) within TAECSF remain deferred and will un-defer post-cascade via the existing Entry 010 → Entry 018 PROMOTED chain (no in-place edit needed; the cascade is post-rung, not pre-rung).
- `apparatus/docs/findings-ledger.md`: at next chapter close, promote TAECSF.3 + APP.PIPELINE-1 from "one-more-observation" toward standing-rule status — this rung's success is the second observation of both.
- `apparatus/locales/manifest.json`: regenerate via `apparatus/locales/discover.sh`. Expected 229 → 230 locales.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — must PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — must PASS (74/0/1 ignored baseline).
3. **Critical**: re-build `bin/cruft` from `target/release/cruft` before measurement gates (the binary-staleness issue from TAECSF-EXT 1 measurement-discipline incident).
4. **View-aliasing probe**: run `T262_TEST_PATH=...index-prop-desc.js T262_HARNESS_DIR=... cruft runner.mjs` on the three TAECSF-EXT 1 NEGATIVE cells; all three must report `PASS`. This is the Doc 740 + Doc 741 substrate-introduction signature: even if cluster gates don't move, these specific cells flipping from FAIL to PASS validates the read/write symmetry + view-aliasing pass-through at the byte tier.
5. Regression gate — TAMM cluster: ≥ 86/100.
6. Regression gate — TAWR cluster: ≥ 67/100.
7. Regression gate — diff-prod: ≥ 64/48.
8. **Cascade prediction probe**: re-attempt TAECSF-EXT 1's substrate edit at `typed_array_set_index_checked` (the integer-kind branch); verify it now PASSES the 3 cells WITHOUT regressing TAMM. If true, Doc 739 cascade-revival pattern is empirically validated for this engagement-tier instance. This probe is INFORMATIVE only — landing the TAECSF-EXT 1 substrate is OUT OF SCOPE for this rung per Doc 740 multi-tier discipline (one tier at a time); the probe just measures the cascade.

## Risk Assessment

Per the pipeline alphabet audit's four named risks (`apparatus/docs/pipeline-alphabet-audit-2026-05-30.md` §C):

1. **Read/write symmetry must migrate jointly**. Partial migration (e.g., write rewritten but read still returns Value) breaks the architecture worse than pre-migration. Mitigation: this proposal explicitly bundles read + write rewrite (transitions 4 + 5 of I); single commit; no intermediate state.
2. **DataView setters/getters share the same `Vec<Value>` assumption**. Mitigation: this proposal bundles DataView migration (transition I §6 + Lattice R₂). Doc 744 §III.3 class-3 timing-edge regression risk avoided by lattice-meet closure.
3. **View-aliasing harness pattern** (test262/harness/testTypedArray.js:107 `copyIntoArrayBuffer`) is the most reliable gate cell. Mitigation: explicit post-gate (verification §4) on the three TAECSF-EXT 1 NEGATIVE cells.
4. **`__kind` typed-enum co-yield optimization** is non-precondition. Mitigation: this proposal includes `TypedArrayKind` enum promotion (transition I §2) as part of the rung; defers the optimization risk by treating it as in-scope rather than out-of-scope.

Additional risks surfaced during proposal authoring:

5. **Atomics.* BigInt overloads** (if implemented) may access buffer storage directly. Mitigation: grep verification at substrate-start; if Atomics paths exist, fold into the rung's substrate; if not, name explicitly as out-of-scope and add deferrals-ledger entry.
6. **Buffer shim on Uint8Array** (cruftless/src/node_stubs.rs) reads/writes Uint8Array storage directly for Node.js compat (`Buffer.from`, `readUInt32BE`, etc.). MILF-EXT 7 family landed an extensive Buffer numeric reader/writer surface. Mitigation: substrate-start grep verification; Buffer methods must migrate jointly with TA storage. Substantial; may extend this rung's scope.
7. **Engine-internal direct buffer access** (e.g., crypto, compression, fetch body bytes) may touch `buf.data` directly assuming Value-cells. Mitigation: substrate-start grep `\.data\[` + `array_buffers.get` site enumeration; classify each as TA-storage-aware or generic-storage and migrate accordingly.
8. **Cascade-revival prediction (B3) is only verifiable post-landing**. Mitigation: explicit cascade probe (verification §8) as the cascade-validation discipline.
9. **Doc 740 multi-tier epistemic shape**: this rung's gates ARE substrate-introduction (≈0% reclaim expected). Reader should not interpret 0% cluster movement as failure. Mitigation: gate-text explicitly says "substrate-introduction signature predicts ≈ X ± 1"; closure declaration relies on the view-aliasing probe + cascade probe, not on cluster movement.

## Composes-With

- `apparatus/docs/pipeline-alphabet-audit-2026-05-30.md` — the upstream apparatus audit justifying this rung.
- `apparatus/docs/predictive-ruleset.md` rules 1, 4, 5, 6, 11, 13, 15, 17, 18, 19, 20, 21, 22, 23, 25 — full 5-axis pre-spawn + symmetry checks + chapter-close + Rule 13 revert discipline.
- Corpus docs: 721 (cross-pipeline diagnostic), 729 (resolver-instance), 730 (vertical recurrence), 731 (alphabet purity), 739 (single-tier cascade), 740 (multi-tier cascade), 741 (cascade pipeline connects), 744 (pipeline-form discovery).
- `pilots/ta-element-coercion-spec-faithful/` — downstream cascade-revival receiver.
- `pilots/typed-array-wrong-result/` — regression-gate cluster (lattice neighbor).
- `pilots/typed-array-missing-method/` — regression-gate cluster (lattice neighbor).
- `pilots/resizable-buffer-detection-per-access/` — sibling locale, also enrolled in array-exotic arc; may cascade-revive depending on its EXT 0 substrate.
- `apparatus/arcs/2026-05-28-array-exotic-substrate/` — enrolling arc (fourth in-flight locale).
- `apparatus/docs/findings-ledger.md` entries 005 (TAECSF.1), 012 (TAECSF.3), 013 (APP.PIPELINE-1) — cross-locale findings this rung tests for second-observation promotion.

## Authorization

Awaiting keeper APPROVED. Per the established session discipline + Doc 744's six-step spawn-discovery checklist, this proposal is the canonical authoring vehicle for the precursor architectural rung. Landing is gated on the decision. Per Doc 739 (B3) the cascade-revival is observable only post-landing; per Doc 741 the epistemic shape requires accepting near-zero gate movement at this rung in exchange for the pipeline-connection materialization at subsequent tier-closure rungs.
