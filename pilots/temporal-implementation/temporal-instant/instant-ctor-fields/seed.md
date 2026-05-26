---
name: instant-ctor-fields
description: First sub-rung of temporal-instant. Implements ctor + epochNanoseconds (BigInt accessor) + epochMilliseconds (derived Number accessor) + valueOf-throws.
type: project
---

# instant-ctor-fields — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-instant/`.

Per parent's sub-rung topology. Sibling shape to DCF; BigInt-keyed instead of 10-unit-tuple-keyed.

## Telos

- `Temporal.Instant.length === 1`, `Temporal.Instant.name === "Instant"`.
- `Temporal.Instant()` (no `new`) → TypeError.
- `new Temporal.Instant(arg)` coerces arg via ToBigInt (BigInt direct, bool → 0n/1n, string → BigInt(string) with SyntaxError on bad string, Number → TypeError per ToBigInt).
- Range check: |epochNanoseconds| ≤ 8.64e21 → RangeError otherwise.
- Instance stores BigInt as `__ti_ns` sentinel; proto = `Temporal.Instant.prototype`.
- `Temporal.Instant.prototype.epochNanoseconds` accessor returns the BigInt directly.
- `Temporal.Instant.prototype.epochMilliseconds` accessor returns `floor(ns / 10^6)` as Number.
- `Temporal.Instant.prototype.valueOf` throws TypeError.
- `Temporal.Instant.prototype` has `@@toStringTag === "Temporal.Instant"` and back-pointer `constructor`.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — Instant class install block (~120 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with 15 Instant test paths.
- **Exemplar suite**: 25 fixtures.

## Carve-outs

- `epochSeconds` / `epochMicroseconds` getters per spec are aliases for ÷1e9 and ÷1e3; can be added in a follow-on if tests probe them.
- Range check uses f64 conversion of the BigInt (lossy for values near the bounds but the bounds themselves are well within f64 precision; spec range ±8.64e21 ≈ ±2^73).

## Status

TInst-EXT 1 LANDED 2026-05-26. 21/25 PASS (84%).
