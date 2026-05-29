# host-method-prologue-discipline - Trajectory

## 2026-05-28 - HMPD-EXT 0 - Phase 0 spawn

### Directive

Helmsman directed any available substrate resolver to claim EPSUA row 5 as a singleton: spawn `host-method-prologue-discipline`, perform founding baseline inspection, and yield with a Phase-3 plan. No substrate edit is authorized in this rung.

### Phase 0 - Spawn

Locale created at `pilots/host-method-prologue-discipline/derived/`.

Rule 11 pre-spawn coverage:

- A1 component-A/B: test262 host-method failures to runtime intrinsic registration and dispatch.
- A2 op-set: method prologue checks and registration metadata.
- A3 value-domain: built-in method receivers and callback/argument domains.
- A4 locals-marshaling: call frame `this` and arguments into Rust closures.
- A5 emission-shape: registration helper usage and host-method closure prologues in `intrinsics.rs`.

### Carve-Outs

HMPD-EXT 0 lands no runtime substrate. Phase 2 baseline inspection will be reported to helmsman before any Phase 3 duplicated-site probe or implementation.

### Status

Spawn artifacts created. Manifest refresh belongs in the same commit.

## 2026-05-28 - HMPD-EXT 1 - Phase 2 baseline inspection

### Baseline Source

Inspected `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-28-123833-p2/`.

Relevant matrix rows:

- Rank 18: `runtime/buffer-typed-array :: E3/intrinsic-object:ecma-262 :: abrupt-completion/throw-missing :: err:TypeError` - 276 failures.
- Rank 39: `runtime/spec-builtins :: E3/intrinsic-object:ecma-262 :: abrupt-completion/throw-missing :: err:TypeError` - 146 failures.
- Rank 54: `runtime/spec-builtins :: E2/internal-method:runtime :: abrupt-completion/throw-missing :: err:TypeError` - 96 failures.

The broad `Expected a TypeError to be thrown but no exception was thrown at all` reason occurs 1,078 times in `interpreted.jsonl`, but it does not form one coherent host-method-prologue mechanism. Path-segment buckets show the largest segment at 272 rows (ArrayBuffer/DataView/TypedArray), or 25.2% of the broad reason pool; no segment reaches the EPSUA C4 40% coherence threshold.

### Sampled Failure Segments

Sampled examples across the broad cluster:

- `annexB/built-ins/Date/prototype/getYear/this-not-date.js`: Date prototype receiver brand check missing for non-Date ordinary object.
- `annexB/built-ins/Date/prototype/setYear/this-not-date.js`: Annex B Date setter receiver brand check missing.
- `built-ins/Date/prototype/getDate/this-value-non-date.js`: Date generated getter accepts ordinary object instead of throwing.
- `annexB/built-ins/RegExp/legacy-accessors/index/this-cross-realm-constructor.js`: RegExp legacy accessor realm/receiver check missing.
- `annexB/built-ins/RegExp/legacy-accessors/input/this-subclass-constructor.js`: RegExp legacy accessor subclass receiver check missing.
- `built-ins/AggregateError/message-tostring-abrupt-symbol.js`: Error-family constructor message ToString/ToPrimitive TypeError not propagated.
- `built-ins/Error/error-message-tostring-symbol.js`: Error constructor accepts Symbol message instead of throwing.
- `built-ins/Array/isArray/proxy-revoked.js`: Array.isArray revoked-proxy abrupt completion missing.
- `built-ins/Array/from/iter-set-elem-prop-err.js`: Array.from element write/non-configurable failure missing.
- `built-ins/FinalizationRegistry/prototype/register/heldValue-same-as-target.js`: FinalizationRegistry weak-held-value validation missing.
- `built-ins/BigInt/asIntN/bigint-tobigint-errors.js`: BigInt ToBigInt TypeError conversion not thrown.
- `annexB/built-ins/escape/to-string-err-symbol.js`: Annex B escape ToString(Symbol) TypeError missing.
- `built-ins/ArrayBuffer/prototype/resize/this-is-not-resizable-arraybuffer-object.js`: ArrayBuffer resizable/brand semantics missing.

### Registration-Site Cross-Reference

Runtime registration audit in `pilots/rusty-js-runtime/derived/src/intrinsics.rs` shows multiple entry families, not one uniform missing helper:

- Date prototype methods are installed in `install_date_global` through generated wrappers at lines around 17909-18143; they rely on `Runtime::current_this()` and generated Date functions for the brand/prologue check.
- BigInt static methods `asIntN` / `asUintN` are installed around 16780-16854 and call local conversion helpers, not a receiver-prologue helper.
- WeakRef/FinalizationRegistry methods are installed around 20134-20180; `FinalizationRegistry.prototype.register` is currently a stub returning `undefined`.
- Error-family constructors and `Error.prototype.toString` are installed around 20509-20640; constructor message conversion routes through `rt.coerce_to_string`.
- The common registration helpers (`make_native*`, `register_method`, `register_intrinsic_method`) at roughly 22240-22370 already encode descriptor shape, arity, and non-constructor status, but do not encode per-family receiver brand, conversion-order, weak-reference, proxy, buffer, or internal-method semantics.

### C4 Decision

C4 fails for a single HMPD substrate landing. The apparent TypeError-not-thrown cluster fragments by missing built-in semantics: buffer/typed-array state, Date receiver branding, Array species/internal-method behavior, RegExp realm/accessor checks, Error message conversion, weak-reference validation, BigInt conversion, and Annex B string conversion.

The next Phase 3 move should not touch every host-method registration. Recommended move shape is a Pin-Art probe over generated Date prototype wrappers first: enumerate all Date methods in `install_date_global`, verify the generated Date functions share one missing `thisTimeValue`/Date-brand prologue, then close that child-sized segment if coherent. Estimated closure: 1 probe rung plus 1-2 substrate rungs for Date; broader HMPD should be treated as an umbrella until at least three child segments show an identical helper-boundary shape.
