---
name: duration-derived-properties
description: Second sub-rung of temporal-duration. Implements sign / blank accessors + abs() / negated() methods on Duration.prototype.
type: project
---

# duration-derived-properties — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-duration/`.

Per parent's sub-rung topology; lands after duration-ctor-fields. ~80 LOC budget.

## Telos

- `Duration.prototype.sign` accessor returns -1 / 0 / 1 based on sign of first non-zero unit (per Duration's uniform-sign invariant).
- `Duration.prototype.blank` accessor returns `true` iff all 10 units are 0.
- `Duration.prototype.abs()` returns a new Duration with `abs(unit)` for each unit.
- `Duration.prototype.negated()` returns a new Duration with negated signs.

All four brand-check via `__td_years` sentinel presence.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — extended within the Duration install block.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO PARTIALLY_IMPLEMENTED allowlist extended with 4 path-prefixes (sign/blank/abs/negated).
- **Exemplar suite**: 24 fixtures from `built-ins/Temporal/Duration/prototype/{sign,blank,abs,negated}/`.

## Status

DDP-EXT 1 LANDED 2026-05-26. 23/24 PASS (95.8%). Single residual is `Temporal.Duration.from` not implemented — belongs to duration-static rung.
