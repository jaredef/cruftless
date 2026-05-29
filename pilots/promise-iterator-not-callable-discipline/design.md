# PIND Phase 3 Design

## Scope

This design rung covers the Promise combinator not-callable cluster identified in PIND-EXT 0. It does not land runtime substrate. The next substrate rung should edit only after this factoring is accepted.

Target combinators:

- `Promise.all`
- `Promise.allSettled`
- `Promise.race`

Excluded for this design:

- `Promise.any`, unless Helmsman widens PIND after the first substrate result.
- Generic Promise scheduling or microtask semantics.
- For-of/destructure/yield* IteratorClose surfaces owned by the parent iterator-protocol arc.

## Failure Discrimination

The 40 named post-EPSUA matrix rows split into two adjacent but distinct failure paths:

| Path | Count | Share | Shape |
|---|---:|---:|---|
| Promise combinator callsite / `C.resolve` not callable | 19 | 47.5% | `Promise.all`, `Promise.allSettled`, or `Promise.race` is called in a shape where the static method or constructor resolve path is non-callable. Current errors are synchronous `callee is not callable` or `C.resolve is not callable`. |
| `@@iterator` method not callable | 18 | 45.0% | `iter-assigned-{false,null,number,string,true,undefined}` rows. The combinator currently lets `collect_iterable` throw synchronously when `@@iterator` is not callable; the tests expect the returned promise to reject. |
| Symbol-assigned iterator noise | 3 | 7.5% | `iter-assigned-symbol` currently routes through global Symbol callability noise before the intended iterator-method check is observable. |

C4 therefore passes for the narrowed PIND coordinate, but the design must not assume one IsCallable line closes the whole cluster. The safe substrate order is to close the `@@iterator` rejection path first, then remeasure the static/callsite bucket.

## Current Runtime Shape

Generated wrappers in `pilots/rusty-js-runtime/derived/src/generated.rs` delegate to runtime methods:

- `promise_all` -> `Runtime::promise_all_via`
- `promise_all_settled` -> `Runtime::promise_all_settled_via`
- `promise_race` -> `Runtime::promise_race_via`

The implementation sites are in `pilots/rusty-js-runtime/derived/src/interp.rs`:

- `promise_all_via`, around the Promise combinator block, creates a capability, validates `C.resolve`, calls `crate::intrinsics::collect_iterable`, then calls `promise_resolve(entry).then(resolve_element, reject)`.
- `promise_all_settled_via` follows the same structure with resolve/reject element factories.
- `promise_race_via` creates a capability, calls `collect_iterable`, then validates `C.resolve` and calls `.then(cap_resolve, cap_reject)` for each entry.

Iterable acquisition is centralized in `pilots/rusty-js-runtime/derived/src/intrinsics.rs::collect_iterable`. It performs ToObject, reads `@@iterator`, calls it, validates the iterator object, reads `next`, and drains the iterator eagerly into `Vec<Value>`.

That eager helper is acceptable for this design rung, but its error surface is not Promise-combinator-shaped: abrupt completions escape as Rust `Err(RuntimeError::...)`, causing synchronous throws. Promise combinators need those iterator-acquisition abrupt completions to reject the capability promise and return it.

## Minimal Refactor

Add a Promise-local wrapper in `interp.rs`, not a global behavior change in `collect_iterable`:

```rust
fn promise_collect_iterable_or_reject(
    rt: &mut Runtime,
    iter_v: Value,
    cap_reject: &Value,
) -> Result<Result<Vec<Value>, Value>, RuntimeError>
```

Semantics:

1. Call `crate::intrinsics::collect_iterable(rt, iter_v)`.
2. On `Ok(entries)`, return `Ok(Ok(entries))`.
3. On `Err(RuntimeError::Thrown(value))`, call `cap_reject(value)` and return `Ok(Err(capability_rejection_marker))`.
4. On ordinary runtime abrupt completion such as `RuntimeError::TypeError(message)`, create or reuse a TypeError-shaped rejection value, call `cap_reject`, and return `Ok(Err(...))`.
5. If calling `cap_reject` itself fails, propagate that error.

The exact return shape can be simpler in implementation:

```rust
fn promise_collect_iterable_or_reject(...) -> Result<Option<Vec<Value>>, RuntimeError>
```

`None` means "capability was rejected; return `capability_promise` now." This matches the Promise combinator algorithms: abrupt completion before per-entry processing rejects the newly created promise rather than synchronously throwing.

Do not change `collect_iterable` in the first substrate rung. Other consumers may rely on synchronous throw semantics. If repeated Promise or iterator helpers need this shape later, lift the wrapper into a shared helper after a second site proves it.

## Edit Sequence

### Rung 4a: Iterator-Acquisition Rejection

1. In `interp.rs`, add a local helper near the Promise combinator methods:
   - captures errors from `collect_iterable`;
   - converts non-thrown runtime errors into a rejection value;
   - invokes the provided reject function;
   - returns `Ok(None)` after successful rejection.
2. In `promise_all_via`, replace:
   - `let entries = crate::intrinsics::collect_iterable(self, iter_v)?;`
   with the helper and early `return Ok(capability_promise)` on `None`.
3. Apply the same replacement in `promise_all_settled_via`.
4. In `promise_race_via`, validate `C.resolve` before iterable collection for symmetry with `Promise.all` and `Promise.allSettled`, then use the same helper.
5. Target the 18 exact `iter-assigned-{false,null,number,string,true,undefined}` rows across all three combinators.

Predicted PASS gain: up to 18 rows from the named 40-row cluster, plus nearby non-top-10 Promise combinator rows if their assertions check promise rejection rather than synchronous throw.

### Rung 4b: Static/`C.resolve` Callability

Only run this after Rung 4a remeasurement.

1. Inspect whether the remaining 19-row bucket is a real runtime defect or a test harness expectation mismatch in the sample runner's async rejection observation.
2. If real, route abrupt `C.resolve` acquisition/callability failures through the same capability rejection path where the spec requires promise rejection.
3. Preserve `.then` callability checks separately; those are post-`promise_resolve` thenable handling and may have different expected rejection timing.

Predicted PASS gain: up to 19 rows if all callsite/`C.resolve` rows are the same rejection-shape defect. This is less certain than Rung 4a because some rows may be asserting synchronous static-method call behavior rather than iterator acquisition.

### Rung 4c: Symbol-Assigned Iterator Residual

Only run if the 3 symbol rows remain after Rung 4a.

Likely issue: the symbol primitive path reports global Symbol callability before reaching the intended non-callable iterator-method failure. Inspect `Symbol.iterator` property representation and ToObject/property-key conversion before editing Promise code.

Predicted PASS gain: 0-3 rows.

## Checks To Preserve

The refactor must preserve these existing checks:

- `C.resolve` is read from the constructor and checked with `Runtime::is_callable`.
- Each resolved entry's `.then` is checked with `Runtime::is_callable`.
- `Promise.all` / `Promise.allSettled` per-element resolve/reject factories and remaining-element cells are unchanged.
- `Promise.race` keeps its first-settlement behavior and does not create per-element result arrays.
- Synchronous non-Promise consumers of `collect_iterable` keep their current synchronous-error behavior.

## Recommended Next Rung

Close the `@@iterator` method not-callable path first. It is narrow, symmetric across all three combinators, directly tied to the parent iterator-protocol arc, and has the clearest spec requirement: Promise combinator iterable acquisition abrupt completions reject the capability promise.

Estimated substrate-rung count: two. Rung 4a should close the iterator-acquisition bucket. Rung 4b likely handles the static/`C.resolve` bucket if it remains after remeasurement. A third small rung may be needed for symbol-assigned iterator residuals.
