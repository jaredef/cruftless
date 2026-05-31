# promise-static-ctx-validation — Trajectory

## PSCV-EXT 0 — LANDED (2026-05-31) — `this` Object-check at 6 Promise static methods

**Trigger**: Test262 sample 2026-05-31 fail-bucket analysis. `ctx-non-object.js` recurs across Promise.all/race/any/allSettled/resolve. Keeper APPROVED via Telegram 10709 ("A").

**Substrate** (~50 LOC at `pilots/rusty-js-runtime/derived/src/promise.rs`):
- Six registration closures (`Promise.all`, `race`, `any`, `allSettled`, `resolve`, `reject`) now `if !matches!(c, Value::Object(_)) { return Err(TypeError) }` before delegating.
- `Promise.resolve` + `Promise.reject` closures additionally forward `current_this` to the generated path (was discarded as `Value::Undefined`).

**Yield**:

```text
PSCV-EXT 0 probe (/tmp/probe-pscv.js): 36/39 PASS
  6 methods × 6 non-Object this (undefined, null, 86, 's', true, Symbol) = 36 cells ✓
  3 ctx-ctor subclass cells FAIL (out of EXT 0 scope; PSCV-EXT 1 carry-forward)

cargo test --release -p rusty-js-runtime --lib: 74/0/1 preserved.
Full regression sweep preserved (10 probes).
```

**Status**: PSCV-EXT 0 LANDED. Closes ~36 test262 cells across the 6 Promise/ctx-non-object surfaces. Subclass routing (ctx-ctor) carry-forward to PSCV-EXT 1.

## PSCV-EXT 1 — LANDED (2026-05-31) — Subclass routing via NewTarget + Promise.resolve/reject capability path

**Trigger**: PSCV-EXT 0 chapter-close residual. The 3 ctx-ctor probe cells failed because the Promise constructor did not honor NewTarget, and Promise.resolve / Promise.reject discarded `this` and always returned a native Promise instance. Keeper APPROVED via Telegram 10711.

**Substrate** (~50 LOC, promise.rs):

1. Promise constructor (§27.2.3.1 step 3 OrdinaryCreateFromConstructor): after `new_promise(rt)`, override the result's `[[Prototype]]` with `current_new_target.prototype` when NewTarget is set. SubP-derived constructions now inherit SubP.prototype.

2. `Promise.resolve` closure (§27.2.4.7 PromiseResolve):
   - If C === native Promise, short-circuit through `promise_resolve_via` (fast path).
   - Else: §27.2.4.7 step 1 "if IsPromise(x) and x.constructor === C, return x" check, then NewPromiseCapability(C) + Call(cap.[[Resolve]], undefined, [x]) per spec.

3. `Promise.reject` closure (§27.2.4.4): mirror of resolve — short-circuit on native; else NewPromiseCapability(C) + Call(cap.[[Reject]], undefined, [r]).

**Yield**:

```text
PSCV probe (/tmp/probe-pscv.js): 39/39 PASS (was 36/39)

  6 methods × 6 non-Object this -> TypeError (36 cells; EXT 0)
  Promise.all.call(SubP,[]) instanceof SubP                       ✓
  Promise.race.call(SubP,[Promise.resolve(1)]) instanceof SubP    ✓
  Promise.resolve.call(SubP,1) instanceof SubP                    ✓

cargo test --release -p rusty-js-runtime --lib: 74/0/1 preserved.
Full regression sweep preserved (9 probes).
```

**Status**: PSCV-EXT 1 LANDED. ctx-ctor subclass family closed. PSCV locale primary scope complete; Promise.all/race/any/allSettled/resolve/reject all spec-correct on both ctx-non-object (TypeError) and ctx-ctor (subclass-shaped result).
