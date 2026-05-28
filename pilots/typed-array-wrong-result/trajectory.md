# typed-array-wrong-result — Trajectory

## TAWR-EXT 0 — founding + exemplar suite + baseline-TBD (2026-05-25)

**Trigger**: Top-10 spawn batch per keeper directive after canonical
full-suite Pin-Art zoom-out. This is rank #8 of the matrix
(614 fails) and is the highest-yield parity lane shape per heuristics §IV.B.

**Apparatus established**:

- `exemplars/exemplars.txt` — 100 stratified-sample paths.
- `exemplars/run-exemplars.sh` — runner.
- `exemplars/pool-size.txt`, `exemplars/family-breakdown.txt` —
  inventory.

**Baseline**: TBD on next run of `exemplars/run-exemplars.sh`. Expected
near 0/100 given the cluster coherence; record value here.

**Status**: TAWR-EXT 0 founding closed. Apparatus operational; first
substrate rung pending exemplar-fail family-marginal inspection per
heuristics §V row-coherence protocol.

## TAWR-EXT 1 — LANDED (2026-05-28) — per-type prototype chain shortened to %TypedArray%.prototype

Per keeper directive Telegram 10168 (arc pick) following the 2026-05-28 arc back-fit operationalization. First substrate rung in the locale; arc enrollment in `2026-05-28-array-exotic-substrate`.

**Phase 1 (Spawn) per Doc 744 §V.1**:
- **M** = `Object.getPrototypeOf(TypedArrayKind.prototype)` query at consumer test code per ECMA-262 §22.2.6.
- **T** = `%TypedArrayPrototype%` (the abstract intrinsic's prototype) returned per spec; both intrinsics evaluate to the same Object identity.
- **I** = per-type-prototype allocation in `install_typed_array_globals` + the per-type prototype's `[[Prototype]]` slot.
- **R** = lattice with TAMM (typed-array-missing-method) arc-tier work; same substrate locus, different cell.
- **Observability** = ordinary (test262 sameValue assertion).
- **Mouth-gating prerequisite**: TAMM-EXT 3 + EXT 4 substrate (per-type prototype + %TypedArray% intrinsic + ta_proto_proto-as-%TypedArray%.prototype) is the upstream DAG terminus this rung consumes.

**Phase 2 (Baseline-inspect)** per Rule 23:
- Baseline measurement: 36/100 cluster exemplars PASS (substantially higher than the seed's "expected near 0" because TAMM-EXT 1-10 incidentally moved this cluster's dial).
- Sample inspection (TypedArrayConstructors top family, 22 fails): 7 of the fails are the `Float32Array.prototype.proto.js`-shape assertion `Object.getPrototypeOf(F32.prototype) === TypedArray.prototype`. Cruft's chain has an extra tier (per_type → ta_proto → ta_proto_proto) where the spec wants two-deep (per_type → ta_proto_proto). The extra `ta_proto` tier was introduced at TAMM-EXT 4 as the shared per-instance prototype + TAMM-EXT 3 mirrored its methods onto `ta_proto_proto` (= `%TypedArray%.prototype`) so the redundancy is benign for method lookup but visible to `Object.getPrototypeOf` reflection.

**Substrate** (~5 LOC in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`):
- Change per_type_proto's `proto` slot from `ta_proto` to `ta_proto_proto`. The instance method chain now walks `instance → per_type_proto → ta_proto_proto → Object.prototype` (two-deep prototype chain, spec-conformant). Methods are mirrored on ta_proto_proto per TAMM-EXT 3, so lookup still resolves.

**Yield**:
```text
TAWR cluster PRE-EXT 1:  PASS=36 FAIL=64 / 100 (36.0%)
TAWR cluster POST-EXT 1: PASS=47 FAIL=53 / 100 (47.0%)
```
**+11 PASS** this rung. TypedArrayConstructors family residual 22 → 11; eleven `*proto.js` and adjacent fails per type-class close at once.

**Cross-arc impact**:
- TAMM cluster: 82/100 (unchanged). Direct probe: `a.at(0)` still resolves; `BYTES_PER_ELEMENT` still own on per_type_proto; instance methods intact.
- diff-prod: 61/51 (parity preserved; pre and post measure identically).

**Tag**: `cluster-typedarray-proto-chain-shortened-1`.

**Finding TAWR.1**: when an apparatus adds an intermediate substrate tier (here: ta_proto inserted between per_type_proto and ta_proto_proto) and ALSO mirrors the tier's methods onto the upstream tier (TAMM-EXT 3 mirror), the intermediate tier becomes redundant for method lookup but visible to spec-reflective queries (`Object.getPrototypeOf`). The mirror's purpose was substrate-correctness; the chain-shortening is the spec-shape that completes it. Standing rec: when introducing a method-mirror across two tiers, also consider whether the lower tier is still needed in the prototype chain; if not, drop it.

**Status**: TAWR-EXT 1 CLOSED locally. Arc-tier accumulation: this is the first substrate rung enrolled under `2026-05-28-array-exotic-substrate` arc since scaffolding; per Doc 745 candidate §II's per-Phase emission protocol, this rung's six-section emission (header / baseline / no-duplication / single-round / close / substrate) is the canonical first instance of the structured emission shape in the arc.
