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
