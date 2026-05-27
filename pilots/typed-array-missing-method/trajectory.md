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

## TAMM-EXT 3 — LANDED (2026-05-27) — TypedArray.prototype methods + %TypedArray% intrinsic

Per keeper directive Telegram 10074 ("Continue").

**Substrate** (~360 LOC across intrinsics.rs):

*Part A: ES2022+ method additions on ta_proto.* Added 9 missing methods per ECMA-262 §23.2.3: `at` (§23.2.3.1), `lastIndexOf` (§23.2.3.18), `copyWithin` (§23.2.3.6), `findLast` (§23.2.3.13), `findLastIndex` (§23.2.3.14), `sort` (§23.2.3.30, ascending default + custom comparator via insertion sort for borrow-safety), `with` (§23.2.3.34), `toReversed` (§23.2.3.32), `toSorted` (§23.2.3.33).

*Part B: methods mirrored onto %TypedArray%.prototype (ta_proto_proto).* Pre-EXT 3, methods lived only on ta_proto (the per-instance shared prototype). test262 fixtures probe `Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype), name)` and expected to find them at the spec-correct level-2 prototype. Mirrored via a name-list iteration that copies each existing ta_proto method onto ta_proto_proto.

*Part C: %TypedArray% abstract intrinsic constructor.* Pre-EXT 3, `Object.getPrototypeOf(Int8Array)` returned Function.prototype (the default), so test262 harnesses doing `TA = Object.getPrototypeOf(Int8Array); TA.prototype.at(...)` got `undefined.at`. Added a %TypedArray% ctor object that throws on direct construction, with .prototype = ta_proto_proto. Wired each concrete TypedArray ctor's [[Prototype]] to point at it. Also installed `TypedArray[Symbol.species]` getter per §23.2.2.4.

**Yield**:
```text
TAMM cluster POST-EXT 2: PASS=36 FAIL=64 / 100 (36.0%)
TAMM cluster POST-EXT 3: PASS=49 FAIL=51 / 100 (49.0%)
```
**+13 PASS** this rung. TypedArray family residual **27 → 14**. TypedArrayConstructors family residual unchanged at 24 — those tests probe deeper substrate (BYTES_PER_ELEMENT shape, abstract-ctor semantics, length+name+prop-desc invariants) that needs further rungs.

**Cumulative TAMM yield since EXT 0 baseline: 3 → 49 / 100 (+46 across three rungs)**.

**Gates**: build clean; diff-prod 59/53 (parity).

**Direct probes**:
- `typeof Uint8Array.prototype.at === "function"` ✅
- `typeof Uint8Array.prototype.toSorted === "function"` ✅
- `Object.getPrototypeOf(Int8Array)` returns the %TypedArray% ctor (not Function.prototype) ✅
- `typeof TypedArray[Symbol.species] === "function"` (via the intrinsic, not globalThis) ✅

**Standing rec TAMM.3 (level-2 prototype install)**: built-in prototype hierarchies with abstract-intrinsic parents (TypedArray, Error, Iterator) need explicit %X% ctor + .prototype installation, with each concrete child ctor's [[Prototype]] wired to %X%. Cruft has additional opportunities here: %Iterator% for the iterator-helpers proposal, %TypedArray% just done, %Error% for proper Error subclass identity. Each follows the same shape.

**Status**: TAMM-EXT 3 CLOSED locally.

## TAMM-EXT 4+5 — LANDED (2026-05-27) — per-type prototype + ctor.length=3 + %TypedArray% from/of + instance.buffer

Per keeper directive Telegram 10077 ("continue with tamm").

**EXT 4 substrate** (per-type prototype + abstract-intrinsic static rehoming):
- Allocate a per-type prototype for each concrete TypedArray ctor that chains to the shared `ta_proto` (which itself chains to `ta_proto_proto` / %TypedArray%.prototype). Three-deep chain: instance → SubA.prototype → %TypedArray%.prototype → Object.prototype.
- Hosts `BYTES_PER_ELEMENT` (per §23.2.6.1) + `constructor` (per §23.2.6.2) own on the per-type prototype, with values that differ per type (1/2/4/8).
- Each ctor's `.prototype` slot now points at the per-type prototype (was: shared ta_proto).
- Each ctor's `.length` set to 3 per §23.2.5 (was: 0 from `make_native` default).
- `%TypedArray%.from` / `%TypedArray%.of` registered on the intrinsic ctor per §23.2.2.1+§23.2.2.2. Inherited by all concrete ctors via the [[Prototype]] chain wired in EXT 3.

**EXT 5 substrate** (instance.buffer backing + subarray buffer-sharing):
- Every TypedArray constructed from a length-or-arraylike now allocates a real backing ArrayBuffer (object + record + ArrayBuffer.prototype chain) per §23.2.5.1. Sets `.buffer`, `.byteOffset`, and `.byteLength` own on the instance and registers a TypedArrayViewRecord. Pre-EXT 5: `.buffer` was undefined for non-buffer-arg construction, which collapsed harness flows like `new TA(arr).buffer.byteLength` (used by `makeArrayBuffer`, `makeGrownArrayBuffer`, etc.).
- `subarray` propagates the parent's buffer and adjusts byteOffset/byteLength per §23.2.3.31, so views share storage. Pre-EXT 5: both parent.buffer and child.buffer were undefined so they aliased trivially; once parent.buffer became a real buffer, child.buffer needed to point at the same buffer to preserve the `u.buffer === sub.buffer` invariant.

**Yield**:
```text
TAMM cluster POST-EXT 3: PASS=49 FAIL=51 / 100 (49.0%)
TAMM cluster POST-EXT 5: PASS=68 FAIL=32 / 100 (68.0%)
```
**+19 PASS** across the combined rung. TypedArrayConstructors family residual **24 → 13**. TypedArray family residual **14 → 7**.

**Cumulative TAMM yield since EXT 0 baseline: 3 → 68 / 100 (+65 across five rungs)**.

**Gates**: build clean; diff-prod 59/53 (parity preserved through subarray buffer-sharing fix).

**Direct probes**:
- `Int8Array.length === 3` ✅
- `Int8Array.prototype.BYTES_PER_ELEMENT === 1` ✅
- `Int8Array.prototype.constructor === Int8Array` ✅
- `TypedArray.from / TypedArray.of` are functions on %TypedArray% ✅
- `new Uint8Array(4).buffer instanceof ArrayBuffer` ✅
- `u.buffer === u.subarray(1, 4).buffer` ✅ (preserved)

**Standing rec TAMM.4 (per-type prototype + shared-buffer invariant)**: substrate rungs that allocate previously-absent per-instance state (here: real `.buffer` ArrayBuffer) must audit all derivation sites that propagate that state. The subarray buffer-sharing fix was uncovered by the diff-prod typed-arrays fixture's `sub_is_view: u.buffer === sub.buffer` check, which trivially held while both were undefined and silently broke once they became real. Standing instrument: when a TAM rung adds an instance own-slot, grep for all sites that copy/derive instance state and ensure the slot propagates.

**Status**: TAMM-EXT 4+5 CLOSED locally.

## TAMM-EXT 6 — LANDED (2026-05-27) — receiver-as-ctor in %TypedArray%.from/of

Per keeper directive Telegram 10079 ("Continue").

**Substrate**:
- `%TypedArray%.from` / `%TypedArray%.of` now invoke `this` as the constructor with `[len]` per §23.2.2.1 step 7 / §23.2.2.2 step 5 (TypedArrayCreate). Pre-EXT 6 both methods produced plain objects regardless of receiver — collapsed the test class that probes `TA.from.call(CustomCtor, src)`.
- Removed per-ctor own `from`/`of` registrations on each concrete TypedArray ctor. Concrete ctors now inherit %TypedArray%.from/of via the [[Prototype]] chain wired in EXT 3, so `Int8Array.from(src)` invokes Int8Array as the constructor uniformly.

**Yield**:
```text
TAMM cluster POST-EXT 5: PASS=68 FAIL=32 / 100 (68.0%)
TAMM cluster POST-EXT 6: PASS=72 FAIL=28 / 100 (72.0%)
```
**+4 PASS** this rung. TAC residual 13 → 9.

**Cumulative TAMM yield since EXT 0 baseline: 3 → 72 / 100 (+69 across six rungs)**.

**Gates**: build clean; diff-prod 59/53 (parity).

**Direct probes**:
- `Int8Array.from === Object.getPrototypeOf(Int8Array).from` ✅ (inheritance, not own)
- `Int8Array.from([1,2,3]) instanceof Int8Array` ✅
- `Int8Array.from.call(CustomCtor, src)` invokes CustomCtor (receiver-as-ctor) ✅

**Standing rec TAMM.5 (built-in static via abstract-intrinsic inheritance)**: when an abstract intrinsic provides a static that delegates to the receiver as constructor (Array.from, TypedArray.from, Promise.all, Set/Map iterables), the concrete subclasses must NOT carry own copies — they must inherit so `Sub.from.call(OtherCtor, ...)` reaches the receiver-as-ctor logic. Own-copy on the subclass shadows inheritance and silently breaks the receiver-as-ctor contract.

**Status**: TAMM-EXT 6 CLOSED locally.

## TAMM-EXT 7 — LANDED (2026-05-27) — @@species returns `this` (ArrayBuffer + %TypedArray%)

Per keeper directive Telegram 10081 ("Continue").

**Substrate**: ArrayBuffer[@@species] and %TypedArray%[@@species] getters now return `rt.current_this()` per spec sec 23.1.5.2 / §25.1.4.3 / §23.2.2.4. Pre-EXT 7 both captured a fixed ctor id at install time, which gave the right answer for `Class[Symbol.species]` but the wrong answer for `accessor.call(thisVal)` probes (e.g. the test262 `return-value.js` pair that does `Object.getOwnPropertyDescriptor(ArrayBuffer, Symbol.species).get.call({})` and asserts the return value equals the receiver).

**Yield**:
```text
TAMM cluster POST-EXT 6: PASS=72 FAIL=28 / 100 (72.0%)
TAMM cluster POST-EXT 7: PASS=74 FAIL=26 / 100 (74.0%)
```
**+2 PASS** this rung (ArrayBuffer/Symbol.species/return-value + TypedArray/Symbol.species/result). ArrayBuffer residual 9 → 8; TypedArray residual 7 → 6.

**Cumulative TAMM yield since EXT 0 baseline: 3 → 74 / 100 (+71 across seven rungs)**.

**Gates**: build clean; diff-prod 59/53 (parity).

**Standing rec TAMM.6 (@@species is always receiver-returning)**: every well-known @@species accessor must read its receiver, never close over a fixed ctor id. The id-captured shape gives correct answers in the common direct-access path but silently breaks when the accessor is extracted via getOwnPropertyDescriptor and called against a foreign receiver. Pattern applies uniformly: Array, ArrayBuffer, %TypedArray%, Map, Set, Promise, RegExp.

**Status**: TAMM-EXT 7 CLOSED locally.

## TAMM-EXT 8 — LANDED (2026-05-27) — ValidateTypedArray + IsCallable + ToIntegerOrInfinity throws

Per keeper directive Telegram 10083 ("Continue").

**Substrate** (~50 LOC across four TypedArray prototype methods):
- `find`, `findLastIndex`, `includes`, `copyWithin` now:
  - Check `__ta_kind` internal slot existence on `this` and throw TypeError if absent (ValidateTypedArray per §23.2.3.intro). Pre-EXT 8 these silently returned a sentinel value on non-TypedArray `this`.
  - `find` + `findLastIndex` also check `IsCallable(predicate)` and throw TypeError if non-callable.
  - `copyWithin` index args now coerce via `coerce_to_number` which propagates the spec-mandated TypeError when the input is a Symbol (ToIntegerOrInfinity step 1 / ToNumber rejection).

**Yield**:
```text
TAMM cluster POST-EXT 7: PASS=74 FAIL=26 / 100 (74.0%)
TAMM cluster POST-EXT 8: PASS=79 FAIL=21 / 100 (79.0%)
```
**+5 PASS** this rung. TypedArray family residual **6 → 1**. Remaining families: TAC 9, AB 8, DV 3, TA 1.

**Cumulative TAMM yield since EXT 0 baseline: 3 → 79 / 100 (+76 across eight rungs)**.

**Gates**: build clean; diff-prod 59/53 (parity).

**Standing rec TAMM.7 (TypedArray method discipline)**: every TypedArray.prototype method needs three uniform checks at entry: (a) `this` is an Object, (b) `__ta_kind` slot exists, (c) any predicate arg is callable. Index args must propagate Symbol-→-TypeError via the real ToNumber path. Methods that silently return sentinel values on bad `this` silently break under test262 receiver-validation probes. Applies to: every, some, forEach, map, filter, reduce, reduceRight, find, findIndex, findLast, findLastIndex, includes, indexOf, lastIndexOf, copyWithin, fill, set, slice, subarray, sort, toReversed, toSorted, with, at, join, keys, values, entries.

**Status**: TAMM-EXT 8 CLOSED locally.

## TAMM-EXT 9 — LANDED (2026-05-27) — IntegerIndexedExotic descriptor synth + iterable construction

Per keeper directive Telegram 10085 ("Continue").

**Substrate**:
- `object_get_own_property_descriptor_via` now special-cases TypedArray receivers (`typed_array_views.contains_key(id)`) with canonical-array-index keys: synthesizes a data descriptor `{value, writable:true, enumerable:true, configurable:true}` per §10.4.5.1 [[GetOwnProperty]] for IntegerIndexedExotic (ES2023+). Pre-EXT 9 typed-array elements weren't stored as ordinary dict properties so descriptor reflection missed them entirely.
- TypedArray ctor now handles iterable construction: if source has `@@iterator` and no numeric `length`, drain the iterator into a Vec first to get the correct length, then construct backing buffer + populate. Pre-EXT 9 `new TA(iterable)` produced a zero-length TA because `object_get(arr, "length")` returned undefined on pure iterables.

**Yield**:
```text
TAMM cluster POST-EXT 8: PASS=79 FAIL=21 / 100 (79.0%)
TAMM cluster POST-EXT 9: PASS=81 FAIL=19 / 100 (81.0%)
```
**+2 PASS** this rung. TAC residual 9 → 7.

**Cumulative TAMM yield since EXT 0 baseline: 3 → 81 / 100 (+78 across nine rungs)**.

**Gates**: build clean; diff-prod 59/53 (parity).

**Standing rec TAMM.8 (descriptor-synth for exotic indexed slots)**: built-ins whose indexed elements live in a backing store rather than the property dict (TypedArray, Array sometimes via length, Proxy with target) need explicit synth in `[[GetOwnProperty]]`. The default-dict lookup will silently miss them and tools that probe via `Object.getOwnPropertyDescriptor` (debuggers, serializers, immer-style patches) will think the slot doesn't exist.

**Status**: TAMM-EXT 9 CLOSED locally.
