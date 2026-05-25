# length-of-array-like-propagate — Seed

## Telos

ECMA-262 §7.3.19 LengthOfArrayLike(obj) is specced with `?` propagation: `ToLength(? Get(obj, "length"))`. The `?` means abrupt completions (thrown errors) from the length-property getter or from `ToLength`'s `ToNumber` coercion must propagate to the caller, not be silently turned into `0`.

cruft's `Runtime::length_of_array_like` routed through `array_length` which `unwrap_or(0)`'d the error from `try_array_length`, silently dropping every throw. Result: Array.prototype methods called on an Array-like with a throwing length getter (a test262-common probe shape) returned cleanly with `len=0` instead of propagating the throw. The downstream `assert.throws(Test262Error, ...)` then reported "expected Test262Error but got [no throw or wrong throw]".

Identified via top-failure-reason audit ("Expected a Test262Error but got a TypeError", 11 tests).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs::length_of_array_like` (~line 668) — single source layer.
- `pilots/rusty-js-runtime/derived/src/interp.rs::try_array_length` (~line 7121) — spec-strict variant already exists with the right semantics; comment at install already named the relevant Array.prototype.* methods as the test262-probed surface.
- 8 callers in `generated.rs` (every/filter/find/forEach/map/some/reduce + a JSON one) and the JSON parse path.

## Methodology

One-line change: `Ok(self.array_length(id))` → `self.try_array_length(id)`. Propagates errors from the length-getter + ToNumber coercion per spec.

## Carve-outs

- `array_length` (the silent-`unwrap_or(0)` variant) is kept for substrate-internal callers that want a best-effort length without error propagation. Existing call sites not directly via length_of_array_like are untouched.

## Composes-with

- Standing Rule 13 (revert-then-deeper-layer-closure): the shared closure is the fix-once site; 8 generated callers benefit transparently.
- IPTO.2 (silent-empty as failure mode): same pattern — substrate returning a default on type-mismatched / error input where the spec mandates propagation.

## Resume protocol

Read `trajectory.md` tail.
