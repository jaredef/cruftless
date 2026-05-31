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
