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

## TAWR-EXT 3 — LANDED (2026-05-28) — BigInt.asIntN / asUintN spec-faithful clamp + ToIndex ordering

Per keeper directive Telegram 10174 ("Continue"). Third rung; cross-locale lattice meet with bigint-arithmetic substrate (the substrate locus is BigInt namespace, not the TA path, but the cluster benefit is observable in TAWR exemplars and BigInt64/Uint64 typed-array shape lattice-meets here).

**Phase 1 (Spawn)**:
- **M** = `BigInt.asIntN(bits, bigint)` / `BigInt.asUintN(bits, bigint)` per §21.2.2.1 / §21.2.2.2.
- **T** = ordered side effects: `bits` ToIndex coercion fires before `bigint` ToBigInt coercion. Result is the signed/unsigned clamp of `bigint mod 2^bits`.
- **I** = inline `bigint_to_index` helper (ToPrimitive("number") → ToNumber → ToIntegerOrInfinity → range-check [0, 2^53-1]) + `bigint_clamp` arithmetic via `JsBigInt::shl(bits)` modulus + `divmod` remainder + sign-adjust to positive modulo + signed-half-modulus comparison.
- **R** = lattice with `bigint-arithmetic-wrongness` candidate locale + DAG ↑ ToPrimitive + DAG ↓ JsBigInt::{shl, divmod, sub, add, cmp}.
- **Observability** = ordinary (test262 sameValue + valueOf side-effect order).
- **Mouth-gating prerequisite**: JsBigInt::divmod returning sign-of-dividend remainder + ToPrimitive("number") routing through user @@toPrimitive / valueOf.

**Phase 2 (Baseline-inspect)**: post-EXT 2 baseline TAWR=49/100; BigInt sub-cluster 6 fails all PASS-able by replacing the v1 passthrough. Pre-existing impl was `to_bigint(arg[1])` — skipped ToIndex entirely (no `bits.valueOf` call → order-of-steps observed wrong order), and skipped clamp arithmetic (passthrough → asUintN(8,-2n) returned -2n not 254n).

**Phase 3**: no duplication signal — single emit site for each method.

**Phase 4**: single-round, no negative. Direct probes after build: asIntN(8,200n) → -56n ✅; asUintN(8,-2n) → 254n ✅; asUintN(0,5n) → 0n ✅; order-of-steps `i` → 2 ✅.

**Substrate** (~70 LOC in `pilots/rusty-js-runtime/derived/src/intrinsics.rs` BigInt-namespace registration):
- New `bigint_to_index(rt, &Value) -> Result<u64, RuntimeError>` inline-helper implementing §7.1.22 ToIndex via existing `Runtime::to_primitive` + `abstract_ops::to_number`; throws RangeError on out-of-range or Infinity.
- New `bigint_clamp(rt, args, signed: bool)` shared body: ToIndex(bits) FIRST, then ToBigInt(bigint); compute `2^bits` modulus, divmod, sign-correct to positive modulo, signed-half-modulus branch for asIntN.
- `register_intrinsic_method` for asIntN/asUintN now dispatches through `bigint_clamp` with `signed=true`/`false`.

**Yield**:
```text
TAWR cluster PRE-EXT 3:  PASS=49 FAIL=51 / 100 (49.0%)
TAWR cluster POST-EXT 3: PASS=55 FAIL=45 / 100 (55.0%)
```
**+6 PASS** this rung. BigInt sub-cluster 6 → 0 (clean close).

**Gates**: build clean; diff-prod 61/51 (parity preserved); TAMM unchanged 82/100; sanity intact.

**Tag**: `cluster-bigint-asintn-asuintn-spec-faithful-3`.

**Finding TAWR.3 (passthrough-as-stub trap)**: when a v1 substrate registers a method as `pass-through of one argument` because the arithmetic was deferred, the deferment is silently observable as a wrong-result failure for any test that exercises the spec arithmetic — and silently observable as a wrong-order failure for any test that exercises spec side-effect ordering (because the passthrough only calls one of the coercions). Standing rec: when registering a v1 stub, prefer "throw NotImplemented" over "passthrough" so the stub surfaces in failure tables rather than being absorbed as a wrong-result; AND when implementing a method whose spec has ordered coercions, the ordering is itself a substrate concern not just an arithmetic one.

**Cross-locale note**: per orphan-disposition Pattern III.2 (lattice-meet repetition), this rung is a candidate for spawning a `bigint-arithmetic-wrongness` locale on the next BigInt-namespace move; the substrate locus repeats across `asIntN`/`asUintN`/`BigInt(arg)` constructor / `BigInt.prototype.toLocaleString` etc. Defer until duplication count reaches the spawn threshold.

**Status**: TAWR-EXT 3 CLOSED locally. Arc-tier accumulation: third rung in `2026-05-28-array-exotic-substrate` arc (enrolled by lattice-meet via BigInt64Array shape, even though the substrate locus is the BigInt namespace).

## TAWR-EXT 4 — LANDED (2026-05-28) — prototype.constructor slot installation (ArrayBuffer, DataView, BigInt)

Per keeper directive Telegram 10179 ("go"). Fourth rung; arc enrollment `2026-05-28-array-exotic-substrate`.

**Phase 1 (Spawn)**:
- **M** = `Object.getPrototypeOf(instance).constructor` at consumer code; equivalently the class-name diagnostic `instance.constructor.name`.
- **T** = the originating constructor function (ArrayBuffer / DataView / BigInt) per §25.1.5.1 / §25.3.4.1 / §21.2.3.
- **I** = the `constructor` own-property slot on each ctor's `.prototype` object.
- **R** = lattice with array-exotic + BigInt namespaces; DAG ↑ Object.getPrototypeOf walk + Object.prototype.constructor fallback.
- **Observability** = ordinary.
- **Mouth-gating prerequisite**: the constructor functions exist (already true).

**Phase 2 (Baseline-inspect)**: post-EXT 3 TAWR=55/100; DataView exemplars (10) and ArrayBuffer exemplars (4) dominated the residual. Direct probe (`ctorprobe.js` scanning 30 built-ins): three offenders — ArrayBuffer, DataView, BigInt. Other 27 built-ins (Map, Set, Promise, Date, RegExp, Array, Object, Function, Error*, all TypedArrays, Symbol, Number, String, Boolean, SharedArrayBuffer) had own-constructor slot installed correctly. Three-way coincidence indicates a known apparatus-shape: when a ctor + proto pair is wired via `set_own_frozen("prototype", proto)` but the reverse-edge `proto.constructor = ctor` is omitted, lookups walk past proto to Object.prototype.constructor which returns Object.

**Phase 3**: duplication of three (ArrayBuffer, DataView, BigInt) of the same shape at three sites. Below Doc 737 §II's ≥5 spawn threshold for a Pin-Art LIFT probe, but at-or-above the threshold for emitting Deferral Ledger Entry 008 (see below).

**Phase 4**: single-round, no negative. Direct probe post-fix shows all three ctor.prototype.constructor === ctor with OWN slot.

**Substrate** (~9 LOC across three call sites in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`):
- After `array_buffer_proto` is wired to `ab_id`: `set_own_internal("constructor", ab_id)` on the proto.
- After `dv_proto` is wired to `dv_ctor_id`: `set_own_internal("constructor", dv_ctor_id)` on the proto.
- After `bi_proto` is wired to `bi_id`: `set_own_internal("constructor", bi_id)` on the proto.
- `set_own_internal` is the per-engine convention for `{writable:true, enumerable:false, configurable:true}` own properties; matches the descriptor shape the other ctor.prototype.constructor slots use (verified by inheritance probe showing OWN matches across all other built-ins).

**Yield**:
```text
TAWR cluster PRE-EXT 4:  PASS=55 FAIL=45 / 100 (55.0%)
TAWR cluster POST-EXT 4: PASS=61 FAIL=39 / 100 (61.0%)
```
**+6 PASS** this rung. DataView family residual 10 → ~4; ArrayBuffer family residual 4 → ~2.

**Gates**: build clean; diff-prod 61/51 (parity preserved); TAMM unchanged 82/100; sanity intact.

**Tag**: `cluster-proto-constructor-slot-installation-4`.

**Finding TAWR.4 (proto.constructor reverse-edge omission)**: when a constructor + prototype pair is wired with the forward edge (`ctor.prototype = proto` via `set_own_frozen("prototype", proto)`) but the reverse edge (`proto.constructor = ctor`) is omitted, the omission is invisible to method-dispatch tests (proto's methods still resolve) and visible only to spec-reflective queries (`Object.getPrototypeOf(instance).constructor`). The query walks past proto to `Object.prototype.constructor` which silently returns `Object`. Standing rec: pair every `set_own_frozen("prototype", proto)` with `set_own_internal("constructor", ctor)` on the proto. Audit the apparatus for any constructor-creating helper that registers only one side of the pair.

**Phase 6 (deferral emission)**: surfaces candidate `prototype-constructor-reverse-edge-audit` as an apparatus-pilot rather than a substrate locale (audit-tier work per orphan-disposition Pattern III.3). Emitted as Ledger Entry 008 in `apparatus/docs/deferrals-ledger.md` (see ledger).

**Status**: TAWR-EXT 4 CLOSED locally. Arc-tier accumulation: fourth rung in `2026-05-28-array-exotic-substrate` arc. Cumulative TAWR across arc: 36 → 47 → 49 → 55 → 61 (+25 total over four rungs).

## TAWR-EXT 5 — LANDED (2026-05-28) — GetPrototypeFromConstructor honoring new.target.prototype

Per keeper directive Telegram 10181 ("continue"). Fifth rung; arc enrollment `2026-05-28-array-exotic-substrate`.

**Phase 1 (Spawn)**:
- **M** = `new SubclassOf(BuiltinCtor)(...)` or `Reflect.construct(BuiltinCtor, args, NewTarget)` where NewTarget !== BuiltinCtor; the resulting instance's `[[Prototype]]` must be NewTarget.prototype if it is an Object, else the BuiltinCtor's intrinsic-default prototype.
- **T** = the resulting instance has `Object.getPrototypeOf(inst) === NewTarget.prototype` when NewTarget.prototype is an Object; else === intrinsicDefault.
- **I** = the `o.proto = Some(...)` slot in each native constructor body.
- **R** = lattice with PCM-EXT 1 (TAWR-EXT 4 ctor.prototype.constructor wiring); same `instance.[[Prototype]]` cell, different sub-shape (forward edge from new.target vs. reverse edge to ctor).
- **Observability** = ordinary.
- **Mouth-gating prerequisite**: `current_new_target` already wired in `interp.rs` Call/Construct dispatch (line ~14290 native-frame branch).

**Phase 2 (Baseline-inspect)**: post-EXT 4 TAWR=61/100. Residual surface: DataView `custom-proto-if-object-is-used` + TypedArrayConstructors `ctors-bigint/buffer-arg/use-custom-proto-if-object` both fail with `Object.getPrototypeOf(instance).constructor === Object` instead of the subclass-expected ctor. Sub-shape of the prior PCM-EXT 1 work: now that proto.constructor walks correctly, the next failure surface is the proto itself being wrong when a subclass is in play.

**Phase 3**: no duplication signal at this rung — the substrate move targets a small set of named native constructors (ArrayBuffer, DataView, TypedArray ×12 via shared `make_native_with_length` closure). The pattern is structurally shared via `Runtime::prototype_from_new_target_or` helper rather than re-implemented per ctor.

**Phase 4**: single-round, no negative.

**Substrate** (~30 LOC across two files):

`pilots/rusty-js-runtime/derived/src/interp.rs`:
- New `Runtime::prototype_from_new_target_or(default_proto) -> ObjectId` method implementing §10.1.14 GetPrototypeFromConstructor: read `current_new_target`; if Object with `"prototype"` own-slot whose value is an Object, return that ObjectId; else fall back to `default_proto`.

`pilots/rusty-js-runtime/derived/src/intrinsics.rs` — three call-site updates:
- DataView ctor: `o.proto = Some(rt.prototype_from_new_target_or(dv_proto_for_ctor))`.
- ArrayBuffer ctor: same shape with `ab_proto_for_ctor`.
- TypedArray per-kind ctor (`make_native_with_length` closure shared by all 12 typed-array kinds): both `o.proto = Some(proto_id)` sites updated to `rt.prototype_from_new_target_or(proto_id)`.

**Yield**:
```text
TAWR cluster PRE-EXT 5:  PASS=61 FAIL=39 / 100 (61.0%)
TAWR cluster POST-EXT 5: PASS=63 FAIL=37 / 100 (63.0%)
```
**+2 PASS** this rung. DataView `custom-proto-if-object-is-used` ✅; TAC `ctors-bigint/buffer-arg/use-custom-proto-if-object.js` ✅.

**Gates**: build clean; diff-prod 61/51 (parity preserved); TAMM unchanged 82/100; sanity intact.

**Tag**: `cluster-getprototype-from-constructor-newtarget-5`.

**Finding TAWR.5 (NewTarget-honoring as shared substrate helper)**: when several native constructors share the same "default proto via fixed slot" idiom, the per-ctor edit to honor `new.target.prototype` would duplicate across every constructor. Promote the read to a shared `Runtime::prototype_from_new_target_or(default)` helper; each ctor becomes a single-line call-site update. Standing rec: when a substrate move would touch ≥3 closures with the same shape, promote the shared logic to a method on `Runtime` rather than inlining at each site (lattice-meet via helper-tier per Doc 744 §IV.2 relational-form).

**Phase 6 (deferral emission)**: surfaces `resizable-buffer-detection-per-access` as a candidate locale — the residual DataView `custom-proto-access-resizes-buffer-*` failures (3 cells: invalid-by-length, invalid-by-offset, valid-by-offset) all share the shape "per-access OOB check when the underlying buffer is resizable and was resized between construction and access". Currently DV stores `fixed_length` at construction; the resizable-buffer path needs a per-access recompute. Below spawn threshold for a dedicated locale (3 of one shape); emitted as Ledger Entry 009.

**Status**: TAWR-EXT 5 CLOSED locally. Arc-tier accumulation: fifth rung in `2026-05-28-array-exotic-substrate` arc. Cumulative TAWR across arc: 36 → 47 → 49 → 55 → 61 → 63 (+27 total over five rungs).

## TAWR-EXT 6 — NEGATIVE (Rule 13 revert) (2026-05-28) — ConvertNumberToTypedArrayElement integer-cast attempt

Per keeper directive Telegram 10183 ("would we know better if phase 5 is closing if you did another round?"). Sixth rung — explicitly invoked as a Phase-5 inflection probe.

**Phase 1 (Spawn)**:
- **M** = `intTA[i] = -0` or `intTA[i] = 1.5` etc.
- **T** = stored value normalized per element kind: -0 → 0 for integer kinds; out-of-range Numbers wrapped; non-integer Numbers truncated.
- **I** = the `value` argument's coercion path inside `Runtime::typed_array_set_index`.
- **R** = lattice with bigint-arithmetic-wrongness candidate (Entry 001) — BigInt TAs would need ToBigInt; lattice-meet via the same `typed_array_set_index` helper.
- **Observability** = ordinary.

**Phase 2 (Baseline-inspect)**: post-EXT 5 TAWR=63. Two cheap-looking residuals: `from/new-instance-from-zero.js` (`-0 => 0`) and `from/mapfn-this-with-thisarg.js`. Direct probe: `Int32Array.from([-0])[0]` returned -0 (Object.is(-0)=true) — confirming raw-storage path.

**Phase 4 (single-round attempt)**: added per-kind integer-cast at `typed_array_set_index` reading `__ta_kind` from the obj's internal slot. Cast: `(n as i32) as f64` for Int32, etc. Built clean; TAWR moved 63 → 64 (+1); **TAMM regressed 82 → 81**.

**Phase 4-negative diagnosis**: my coercion broke a TAMM exemplar (net -1). Sub-shape: the integer-cast is too eager for the helper's contract — `typed_array_set_index` is called both from element-set hot path AND from `[[DefineOwnProperty]]` (object_define_property_via, line ~2968) where the spec semantics differ. A test that stores a Number via `Reflect.defineProperty` and reads back via `[i]` may expect a different cell of behavior than a test that stores via `[i] = v`. Pre-coercion, raw storage was wrong for both but symmetric; post-coercion, asymmetric.

**Phase 4 (Rule 13 revert)**: helper reverted to pre-EXT-6 state. Float32 cast also attempted (per the deeper-NaN-canonicalization concern from `Set/conversion-operation-consistent-nan`); also reverted as part of the unified revert. TAMM restored 82, TAWR restored 63, diff-prod 61/51 preserved.

**Phase 5 (chapter-close-inspect, ARC TIER)**: This rung answers the keeper's Phase-5 inflection question empirically. Three signals taken together:
1. **Yield-curve flattening**: arc yields 36→47 (+11), +2, +6, +6, +2, **(−1)** — the per-rung yield is monotonically decreasing in magnitude (or net-negative) across the last three rungs.
2. **Rule-13 trip on first deeper rung**: the next coherent substrate move past PCM-EXT 1/2 (constructor + new.target proto) requires crossing into ConvertNumberToTypedArrayElement (Number coercion) + canonical-NaN preservation + ToBigInt error-propagation. This is `ta-element-coercion-spec-faithful` substrate — a *different locale*, not a residual of array-exotic-substrate.
3. **Residual-shape coherence**: remaining ~37 fails compress to {resizable-buffer-detection (3 DV + ~10 TA), ta-element-coercion (~10 TAC), mapfn-this-binding (1), BigInt-key-not-writable (2)}. None of these are array-exotic-substrate residuals; all are substrate-locus deeper or lateral.

**Conclusion**: `2026-05-28-array-exotic-substrate` arc reaches Phase-5 inflection at TAWR-EXT 5. EXT 6 is the negative-result rung that confirms the inflection per Rule 13 + Doc 744 §V.3 yield-curve heuristic.

**Phase 6 (deferral emission)**: surfaces `ta-element-coercion-spec-faithful` as a new candidate locale (Entry 010 in deferrals-ledger). Lattice-meets with Entry 001 (bigint-arithmetic-wrongness) on the BigInt-TA branch; emit shared substrate when both un-defer.

**Tag**: `phase-5-inflection-confirmed-via-negative-rung-6`.

**Finding TAWR.6 (yield-curve negative as Phase-5 confirmation)**: Doc 744 §V.3's "≤3-rung close prediction" has a corollary: when an attempted rung within an arc returns negative AND the substrate move's diagnosis routes to a different locale, this is the canonical Phase-5 inflection signal. A negative rung that diagnoses to "deeper substrate within this locale" calls Rule-13 deeper-layer closure; a negative rung that diagnoses to "different locale's substrate" calls arc-close + deferral-emission. Standing rec: when sequencing closing rungs in an arc, deliberately probe a low-priority residual as the Phase-5 inflection test — if it yields ≥3 cleanly, the arc has more structure; if it goes negative AND routes laterally, the arc is closed.

**Status**: TAWR-EXT 6 REVERTED locally (Rule 13). Arc `2026-05-28-array-exotic-substrate` PHASE-5 CLOSED. Cumulative TAWR across arc (final): 36 → 47 → 49 → 55 → 61 → 63. **+27 over five productive rungs + 1 negative inflection probe**. Gates intact at every productive rung.
