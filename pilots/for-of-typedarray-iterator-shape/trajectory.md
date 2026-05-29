# for-of-typedarray-iterator-shape - Trajectory

## 2026-05-29 - FOTIS-EXT 0 - Phase 0 spawn + Phase 2 probe

### Directive

Helmsman directed R3 to spawn `for-of-typedarray-iterator-shape` for post-EPSUA matrix rank 6: `language.statements.for-of / feat:TypedArray;not-callable`, 18 rows. Scope is Phase 0 plus Phase 2 only; no substrate edits are authorized in this founding round.

### Phase 0

Locale founded at `pilots/for-of-typedarray-iterator-shape/`.

Rule 11 pre-spawn coverage:

- **A1 component A/B**: `for-of` iterator acquisition versus TypedArray prototype method exposure.
- **A2 op-set**: `GetMethod(obj, @@iterator)`, call iterator method, iterator `next`, loop value binding, and live index reads during mutation fixtures.
- **A3 value-domain**: concrete numeric TypedArray instances, float/integer/clamped variants, mutation during traversal, non-mutating traversal.
- **A4 locals-marshaling**: bytecode for `Stmt::ForOf`, iterator local, bound loop variable local, and slow/fast iterator next paths.
- **A5 emission-shape**: runtime/intrinsics prototype wiring, not parser or method-prologue validation.

### Phase 2 Baseline

Inspected `pilots/apparatus/test262-categorize/results/2026-05-29/matrix.md` and `categorized.jsonl`.

Matrix entry:

- Rank 6: `language.statements.for-of` / `feat:TypedArray;not-callable` - 18 rows, example `language/statements/for-of/float32array-mutate.js`.

The 18 rows are:

- `float32array-mutate.js`, `float32array.js`
- `float64array-mutate.js`, `float64array.js`
- `int8array-mutate.js`, `int8array.js`
- `int16array-mutate.js`, `int16array.js`
- `int32array-mutate.js`, `int32array.js`
- `uint8array-mutate.js`, `uint8array.js`
- `uint8clampedarray-mutate.js`, `uint8clampedarray.js`
- `uint16array-mutate.js`, `uint16array.js`
- `uint32array-mutate.js`, `uint32array.js`

All 18 `categorized.jsonl` reasons have the same shape:

- `callee is not callable: undefined [argc=0] (method='@@iterator')`
- receiver is a TypedArray-shaped object with `length`, `byteLength`, `buffer`, etc.
- proto-chain report: `Object -> Object.prototype`, no `@@iterator` slot on chain.

### Sampled Failures

Sampled eight rows:

- `float32array.js`: plain Float32Array traversal expects four yielded values.
- `float32array-mutate.js`: Float32Array traversal expects live mutation of index 1 to be reflected.
- `float64array-mutate.js`: same live mutation shape for Float64Array.
- `int8array.js`: plain Int8Array traversal expects four yielded values.
- `int32array-mutate.js`: integer mutation traversal expects updated second yield.
- `uint8array-mutate.js`: Uint8Array mutation traversal expects updated second yield.
- `uint8clampedarray.js`: plain Uint8ClampedArray traversal expects four yielded values.
- `uint32array.js`: plain Uint32Array traversal expects four yielded values.

The sampled files do not exercise detached buffers or iterator-close. They all fail before loop-body semantics because `@@iterator` resolves to `undefined`.

### Segmentation

| Bucket | Count | Share | Shape |
|---|---:|---:|---|
| TypedArray `@@iterator` not exposed on reached prototype chain | 18 | 100.0% | For-of attempts to call `@@iterator`, but concrete TypedArray instances do not see an `@@iterator` method through their current prototype chain. |
| For-of iterator-not-callable error handling | 0 | 0.0% | The thrown TypeError is downstream of a missing method; no evidence that the not-callable branch itself is wrong for this cluster. |
| Detached/out-of-bounds during iteration | 0 | 0.0% | No sampled row reaches iterator `next`; no detached-buffer reason appears in the 18 categorized rows. |
| Other | 0 | 0.0% | No alternate reason shape found. |

C4 passes: the TypedArray `@@iterator` exposure bucket is 18/18 (100%), above the 40% coherence threshold.

### Runtime Cross-Reference

For-of execution:

- `pilots/rusty-js-runtime/derived/src/interp.rs::Op::ForOfFastNext` only fast-paths Array iterator shape and otherwise falls through to the slow path.
- Existing iterator-acquisition patterns, such as `Runtime::promise_collect_iterable`, read `@@iterator` and call it on the receiver; they are not the cause of this cluster's missing-method shape.

TypedArray installation:

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` registers `values`, `keys`, `entries`, and `@@iterator` on `ta_proto`.
- Later in the same installation flow, TAWR/TAMM wiring sets each concrete typed-array prototype's `[[Prototype]]` directly to `ta_proto_proto` (`%TypedArray%.prototype`) for spec-correct `Object.getPrototypeOf(Float32Array.prototype) === TypedArray.prototype`.
- A mirror pass copies many methods from `ta_proto` to `ta_proto_proto`, but the mirror list includes `values`, `keys`, and `entries` and omits `@@iterator`.
- Therefore concrete instances reach `%TypedArray%.prototype` without seeing `@@iterator`, while `.values()` may still be reachable. That matches the 18-row for-of failure exactly.

### C4 Decision

C4 passes for a coherent Phase 3 move.

Recommended Phase 3 move: mirror or install `%TypedArray%.prototype[@@iterator]` at the reached `ta_proto_proto` level, aliasing the existing values-iterator implementation. The narrowest fix is to add `@@iterator` to the `ta_proto` -> `ta_proto_proto` mirror list or register it directly on `ta_proto_proto` after the prototype split. The move should preserve `values === @@iterator` semantics and verify both non-mutating and mutating for-of fixtures.

Estimated closure: one substrate rung. The named 18-row cluster should close if `@@iterator` becomes visible on the concrete TypedArray instance prototype chain. Adjacent probes should include `typedArray.values()` and direct `typedArray[Symbol.iterator]()` shape, because the implementation already has iterator next semantics and live index reads.
