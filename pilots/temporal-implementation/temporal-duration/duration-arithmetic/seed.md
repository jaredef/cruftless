---
name: duration-arithmetic
description: Seventh sub-rung of temporal-duration. Implements add/subtract with sub-day balancing (no relativeTo support).
type: project
---

# duration-arithmetic — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-duration/`.

Per-class arithmetic for Duration. Without relativeTo, only day-and-below units can balance; year/month/week stay un-balanced (mixed signs → RangeError).

## Telos

- `add(other)`: read this + other as unit arrays; element-wise sum; balance sub-day (days through nanoseconds) via total-ns roundtrip; uniform-sign validation; return new Duration.
- `subtract(other)`: same with other negated.

## Carve-outs

- **options.relativeTo**: needed to balance years/months/weeks/days across calendar boundaries. Deferred to `duration-relative-to`.
- **Fractional H/M roll-up**: IDP rejects fractional-on-non-smallest; iso-fractional-propagation residual.
- **options.largestUnit / overflow**: deferred to duration-options-handling.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — read_duration_units + duration_add_impl + add + subtract (~150 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist for `/Temporal/Duration/prototype/{add,subtract}/`.
- **Exemplar suite**: 68 fixtures.

## Status

DA-EXT 1 LANDED 2026-05-26. 24/68 PASS (35%). Basic add/subtract functional; large balancing tests deferred via IDP fractional-residual.
