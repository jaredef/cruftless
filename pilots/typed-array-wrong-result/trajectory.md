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

## TAWR-EXT 2 — LANDED (2026-05-28) — IntegerIndexedExotic [[DefineOwnProperty]] canonical-numeric-index discipline

Per keeper directive Telegram 10172 ("continue development with your selected arc").

**Phase 1 (Spawn)**:
- **M** = `Object.defineProperty(ta, K, desc)` / `Reflect.defineProperty(ta, K, desc)` call at consumer code; key K is a string per §7.1.21 CanonicalNumericIndexString.
- **T** = boolean false (per Reflect) or throw (per Object) when K is invalid for IntegerIndexedExotic per §10.4.5.3 step 3.b: IsInteger fails (NaN, fractional), K = "-0", K is negative, K is out-of-bounds, or descriptor attributes disagree with TA fixed shape (writable:true, enumerable:true, configurable:true). Boolean true when K is a valid in-bounds non-negative-integer index AND descriptor is shape-conformant; the underlying typed-element-slot is set.
- **I** = key classification (per §7.1.21) → bounds check → attribute check → element store.
- **R** = lattice with array-exotic arc + DAG ↓ Reflect.defineProperty boolean-wrapper.
- **Observability** = ordinary (test262 sameValue assertion + Reflect's boolean return surface).
- **Mouth-gating prerequisite**: TAMM-EXT 5 (typed_array_views registry) + the Reflect.defineProperty intrinsic.

**Phase 2 (Baseline-inspect)**: post-TAWR-EXT 1 baseline 47/100. Sample inspection of remaining TypedArrayConstructors fails (11) showed `internals/DefineOwnProperty/key-is-minus-zero.js`-shape failures: `Reflect.defineProperty(ta, "-0", {value:42, configurable:false, ...})` should return false (cruft returned true). Out-of-bounds + non-integer + non-configurable-attribute disagreements all share the same root: cruft's [[DefineOwnProperty]] for TA didn't implement §10.4.5.3 step 3.b's classification.

**Phase 3**: no duplication signal — single emit site (object_define_property_via data-descriptor branch).

**Phase 4**: single-round, no negative.

**Substrate** (~60 LOC across two files):

`pilots/rusty-js-runtime/derived/src/interp.rs`:
- New `NumericIndexClass` enum (ValidArrayIndex(usize) / InvalidNumericIndex) at module scope.
- New `Runtime::classify_numeric_index_key(&str) -> Option<NumericIndexClass>` helper implementing §7.1.21 CanonicalNumericIndexString classification: "-0" → Invalid; canonical non-negative integer → ValidArrayIndex; canonical-but-not-integer-or-negative → Invalid; non-canonical (e.g. "foo", "01") → None.
- `object_define_property_via` data-descriptor branch: when target is a TA AND key classifies, dispatch per `NumericIndexClass`. ValidArrayIndex: bounds-check (return false if oob); attribute-check (return false if writable=false, enumerable=false, or configurable=false); set element; return true. InvalidNumericIndex: return false.

`pilots/rusty-js-runtime/derived/src/intrinsics.rs` Reflect.defineProperty wrapper (both non-Proxy and Proxy-fallback paths):
- Match `Ok(Value::Boolean(false))` separately from `Ok(_)` so an inner-returned false propagates rather than being translated to true by the wrapper's blanket Ok→true rule.

**Direct probes** (post-rung):
- `Reflect.defineProperty(ta, "-0", {value:42, ...})` → false ✅ (was: true)
- `Reflect.defineProperty(ta, "1.5", ...)` → false ✅
- `Reflect.defineProperty(ta, "5", ...)` (oob on length-2 TA) → false ✅
- `Reflect.defineProperty(ta, "0", ...)` (valid) → true; `ta[0]` updated ✅
- `Reflect.defineProperty(ta, "foo", ...)` (non-canonical-numeric) → true; falls through to generic Object property ✅

**Yield**:
```text
TAWR cluster PRE-EXT 2:  PASS=47 FAIL=53 / 100 (47.0%)
TAWR cluster POST-EXT 2: PASS=49 FAIL=51 / 100 (49.0%)
```
**+2 PASS** this rung. TypedArrayConstructors family residual 11 → 9.

**Gates**: build clean; diff-prod 61/51 (parity); sanity intact; TAMM cluster unchanged at 82/100.

**Tag**: `cluster-typedarray-defineownproperty-canonical-numeric-index-2`.

**Finding TAWR.2 (Reflect-wrapper translation trap)**: when a substrate move adds a "return false instead of throw" path inside a function whose wrapper translates `Ok(_) → Boolean(true)`, the substrate's false-return is silently swallowed unless the wrapper distinguishes `Ok(Value::Boolean(false))` explicitly. Pattern recurs across any IR-generated function whose user-facing wrapper coerces all-ok-to-true. Standing rec: when a function's spec-defined return type is Boolean (Reflect.X family), its IR-generated implementation should preserve the Boolean rather than relying on the wrapper's translation; the wrapper's role is to convert exception → false for ergonomic-throw semantics ONLY.

**Status**: TAWR-EXT 2 CLOSED locally. Arc-tier accumulation: second rung in `2026-05-28-array-exotic-substrate`.
