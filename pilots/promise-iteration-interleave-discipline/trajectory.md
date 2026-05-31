# promise-iteration-interleave-discipline — Trajectory

## PIID-EXT 0 — LANDED (2026-05-31) — Promise.race interleave + IteratorClose + error-to-rejection plumbing

**Trigger**: AFID.2 audit. Surveyed 6 OOM-vulnerable iterable-consuming intrinsics; Promise.* family (Promise.all/race/any/allSettled) accounts for 4. Per keeper Telegram 10690 ("3"), Set + WeakSet ctors landed first (AFID-EXT 1+2); Promise.* family deferred to a dedicated session. Per keeper Telegram 10694 ("Continue"), founding PIID with Promise.race as the simplest of the four.

**Substrate** (~180 LOC in `pilots/rusty-js-runtime/derived/src/interp.rs`):

1. `promise_race_via` (interp.rs:5159) refactored: replaced the `promise_collect_iterable_or_reject` + for-loop pattern with a call to the new `promise_race_interleave` helper.

2. New `promise_race_interleave(iter_v, ctor, promise_resolve, cap_resolve, cap_reject)` (~80 LOC). Per ECMA-262 §27.2.4.5 + PerformPromiseRace:
   - GetIterator(iter_v) via `promise_iter_get_iterator`. On error → `promise_reject_with_error(cap_reject, ...)` and return early.
   - Cache GetMethod(next).
   - Loop: call next; on error → reject capability (no close: IteratorNext failures don't trigger close per spec). Check result Object (else close + reject TypeError). Check .done. Read .value.
   - Per-element: Call Promise.resolve(C, value); on error → close + reject. Call value.then(cap_resolve, cap_reject); on error → close + reject.

3. New `promise_iter_get_iterator(iter_v)` helper (~25 LOC). Rust-side GetIterator: ToObject the arg if primitive; resolve @@iterator via PropertyKey::Symbol path with fall-through to string-keyed lookup; call the method with this=arg; require Object result.

4. New `promise_reject_with_error(cap_reject, err)` helper (~30 LOC). Converts a `RuntimeError` to a JS error object (Error/TypeError/RangeError/ReferenceError/SyntaxError) and calls cap_reject with it. Mirrors the prior `promise_collect_iterable_or_reject` error-coercion path; factored out so all four Promise.* siblings can share it.

**Yield**:

```text
PIID-EXT 0 probe (/tmp/probe-piid-0.js): 6/6 PASS
  race finite [resolved,rejected] -> first wins ("a")               ✓
  race([]) returns Promise (still pending)                         ✓
  race(42) -> reject TypeError (was: synchronous TypeError throw)  ✓
  race(iter next non-Object) -> close + reject TypeError           ✓
  race(iter next throw) -> reject with thrown error                ✓
  race(generator) -> first yielded wins                            ✓

cargo test --release -p rusty-js-runtime --lib: 74 / 0 / 1 preserved.

Regression sweep preserved: IPTD 7/7, cross-consumer 7/7,
ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8, AFID-EXT 1 7/7.
```

**Phase 2 (Baseline-inspect)** per Rule 23: confirmed `promise_collect_iterable_or_reject` was the eager-collect site for all four Promise.* statics. Verified that the synchronous-throw vs capability-rejection plumbing was already broken pre-rung at multiple sites: `Promise.race(42)` synchronously threw TypeError instead of returning a rejected Promise. PIID-EXT 0 fixes this for Promise.race; siblings still synchronously throw (their fix is the PIID-EXT 1/2/3 scope).

**Phase 3 (Pin-Art if duplicated)** per Rule 24: the interleave + iter_close_rt + error-to-rejection pattern now appears at three runtime-tier intrinsic sites (Array.from + Set/WeakSet ctor + Promise.race). Rule 24 threshold met. Helper `promise_reject_with_error` is the first promotion (shared across the four Promise.* siblings); `promise_iter_get_iterator` is the second. A fully generic `interleave_with_close<F>` helper deferred until PIID-EXT 1/2/3 lands to gauge per-method body variance.

**Phase 4 (Revert-then-deeper-layer if negative)** per Rule 13: not invoked — closure positive on first probe.

**Phase 5 (Chapter-close-inspect)** per Rule 15: three rungs remain — Promise.all, Promise.any, Promise.allSettled. Each uses the same body shape (GetIterator + loop + close on abrupt) with method-specific .then-chain construction:
- **Promise.all**: per-element Resolve Element closure + shared remaining-count + values array; on completion (remaining=0), cap_resolve(values).
- **Promise.any**: per-element Reject Element closure + shared remaining-count + errors array; on completion, cap_reject(AggregateError(errors)).
- **Promise.allSettled**: per-element settle-this-index closure (always fulfills with `{status, value/reason}`) + cap_resolve.

**Findings**:

- **Finding PIID.1**: synchronous-throw-instead-of-capability-rejection was a class of bugs at all four Promise.* statics pre-rung. Promise.race fixed in this rung; siblings still synchronously throw. Surfaces as standing-rule promotion candidate: runtime-tier intrinsics whose return type is a Promise must convert all internal errors to capability rejections, never synchronous throws.

**Carve-outs (out of scope)**:

- **Infinite-iter OOM is by-spec**: Promise.race per spec iterates until done. Infinite iters are supposed to be infinite. PIID-EXT 0 does NOT add short-circuit-on-capability-settlement (would be a spec divergence).
- **Async iterables**: not applicable — Promise.* statics consume sync iterables.

**Status**: PIID-EXT 0 LANDED. Promise.race now spec-correct on error-to-rejection plumbing + iter_close on abrupt + interleaved iteration. PromiseCapability tracking surface fully exercised; sibling rungs reuse the helpers without modification.

## PIID-EXT 1+2+3 — LANDED (2026-05-31) — Promise.all + allSettled + any interleave

**Trigger**: Keeper Telegram 10696 ("Push all and continue") authorizing the full Promise.* family.

**Substrate** (~450 LOC across one file, `pilots/rusty-js-runtime/derived/src/interp.rs`):

Three new `promise_*_interleave` helpers, each ~150 LOC, all sharing the PIID-EXT 0 helpers (`promise_iter_get_iterator`, `promise_reject_with_error`, `iter_close_rt`):

- `promise_all_interleave` (§27.2.4.1.2 PerformPromiseAll): per element append undefined to values + remaining++; per-index `promise_all_resolve_element_factory`; chain `.then(resolve_element, cap_reject)`. On done, `promise_all_maybe_complete_via`.
- `promise_all_settled_interleave` (§27.2.4.2.3 PerformPromiseAllSettled): per element append undefined to values + remaining++; per-index resolve + reject element factories; chain `.then(resolve_element, reject_element)`. On done, `promise_all_maybe_complete_via`.
- `promise_any_interleave` (§27.2.4.3.2 PerformPromiseAny): per element append undefined to errors + remaining++; per-index `promise_any_reject_element_factory`; chain `.then(cap_resolve, reject_element)`. On done, `promise_any_maybe_reject_via` (AggregateError when all rejected).

Promise.any additionally migrated from synchronous-throw to capability-rejection at the C.resolve resolution path (matching race/all/allSettled). Closes Finding PIID.1 across all four Promise.* statics.

**Yield**:

```text
PIID-EXT 1+2+3 12-cell probe: 11/12 PASS

  Promise.all([resolved x3]) -> [1,2,3]                  ✓
  Promise.all with reject -> rejects                     ✓
  Promise.all(bad iter) -> close + reject TypeError      ✓
  Promise.all(42) -> reject TypeError                    ✓ (was sync TypeError throw)
  Promise.all([]) -> []                                  ✓
  Promise.allSettled mixed -> statuses                   ✓
  Promise.allSettled(bad iter) -> close + reject         ✓
  Promise.allSettled([]) -> []                           ✓
  Promise.any first fulfilled wins                       ✓
  Promise.any all-reject -> AggregateError               ✗ (constructor.name 'Object' — pre-existing Finding PIID.2)
  Promise.any(42) -> reject TypeError                    ✓ (was sync TypeError throw)
  Promise.any(bad iter) -> close + reject TypeError      ✓

cargo test --release -p rusty-js-runtime --lib: 74 / 0 / 1 preserved.

Regression sweep preserved: IPTD 7/7, cross-consumer 7/7,
ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8, AFID-EXT 1 7/7,
PIID-EXT 0 6/6.
```

**Finding PIID.1 CLOSED**: synchronous-throw class was at Promise.all/any/allSettled (Promise.race fixed at EXT 0). All four now route iterator errors to capability rejection per §27.2.4 + IfAbruptRejectPromise.

**Finding PIID.2 SURFACED** (pre-existing, unmasked): Promise.any's all-reject rejection wraps errors in an object whose `constructor.name` is `Object`, not `AggregateError`. The `promise_any_maybe_reject_via` helper does not construct a proper AggregateError instance. Sibling locale candidate: `aggregate-error-construction-discipline/` (or merged with other error-class branding work).

**Phase 3 (Pin-Art if duplicated)** per Rule 24: the interleave + iter_close_rt + error-to-rejection pattern now appears at SEVEN runtime-tier intrinsic sites:
- Array.from (AFID-EXT 0)
- Set ctor (AFID-EXT 1)
- WeakSet ctor (AFID-EXT 1)
- Promise.race (PIID-EXT 0)
- Promise.all (PIID-EXT 1)
- Promise.allSettled (PIID-EXT 2)
- Promise.any (PIID-EXT 3)

The four Promise.* interleavers are structurally near-identical — only the per-element factory + done-completion differ. A generic `promise_iterate_with<F1, F2>` helper taking the per-element factory and the done-completion as closures is the LIFT candidate. Deferred until per-method body variance is measured at the test262-yield level (might surface body-shape differences we'd want to keep flat for clarity).

**Status**: PIID-EXT 1+2+3 LANDED. Promise.* family complete on interleave + iter_close + error-to-rejection plumbing + spec-correct C.resolve error routing. Finding PIID.2 (AggregateError shape) surfaces as separable sibling. The Promise.* iteration-interleave-discipline locale primary scope is now closed; future rungs at this locale would address per-method spec nuances (Promise.allSettled value-coercion, Promise.any AggregateError aggregation, etc.) not the core iteration surface.

## PIID-EXT 4 — LANDED (2026-05-31) — AggregateError construction closes Finding PIID.2

**Trigger**: Finding PIID.2 surfaced by PIID-EXT 1+2+3 (Promise.any all-reject produced an object with `constructor.name === "Object"` instead of `"AggregateError"`). Keeper APPROVED via Telegram 10698 ("Push and continue").

**Substrate** (~40 LOC at `make_aggregate_error_via`, interp.rs:3435):
- Lookup `AggregateError.prototype` via `global_get("AggregateError")` + `object_get(cid, "prototype")`.
- New object: `internal_kind = InternalKind::Error`, `proto = Some(aggregate_error_proto)`.
- Install `message` (canonical "All promises were rejected") + `errors` as non-enumerable own properties per §20.5.7.4 CreateNonEnumerableDataPropertyOrThrow. `name` inherits from prototype chain.

**Yield**:

```text
PIID-EXT 4 probe (/tmp/probe-piid-4.js): 9/9 PASS
  constructor.name === "AggregateError"           ✓ (was "Object")
  instanceof AggregateError                       ✓
  instanceof Error (AggregateError extends Error) ✓
  message canonical                               ✓
  errors array shape                              ✓
  .name on prototype (not own)                    ✓
  .message + .errors own non-enumerable           ✓

PIID-EXT 1+2+3 re-run: 12/12 PASS (was 11/12; PIID.2 cell now passes).
cargo test --release -p rusty-js-runtime --lib: 74 / 0 / 1 preserved.
Regression sweep preserved: IPTD 7/7, cross-consumer 7/7,
ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8, AFID-EXT 1 7/7,
PIID-EXT 0 6/6.
```

**Finding PIID.2 CLOSED**. Promise.any all-reject now produces a proper AggregateError instance with prototype-correct branding.

**Status**: PIID-EXT 4 LANDED. The PIID locale's primary scope (Promise.* iteration + close + error-rejection + AggregateError construction) is now closed. All test262 entries that depend on `aggregate-error.constructor.name === "AggregateError"` for Promise.any all-reject paths should now pass.
