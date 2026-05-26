---
name: plain-time-ctor-fields
description: First sub-rung of temporal-plain-time. Implements ctor + 6 accessor-property getters (hour, minute, second, millisecond, microsecond, nanosecond) + valueOf-throws + range validation.
type: project
---

# plain-time-ctor-fields — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-time/`.

Per parent's sub-rung topology. Sibling shape to DCF + instant-ctor-fields.

## Telos

- `Temporal.PlainTime.length === 0`, `Temporal.PlainTime.name === "PlainTime"`.
- `Temporal.PlainTime()` (no `new`) → TypeError.
- `new Temporal.PlainTime(h=0, m=0, s=0, ms=0, μs=0, ns=0)` — all optional, all integer, all in range per §11.7.2:
  - hour: 0-23
  - minute, second: 0-59
  - millisecond, microsecond, nanosecond: 0-999
  - non-finite / non-integer / out-of-range → RangeError
- Instance stores 6 `__pt_<unit>` sentinels; proto = `Temporal.PlainTime.prototype`.
- 6 accessor getters return their sentinels with brand-check.
- `valueOf` throws TypeError.
- Prototype has `@@toStringTag === "Temporal.PlainTime"`.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — PlainTime class install (~120 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with 16 PlainTime test paths.
- **Exemplar suite**: 34 fixtures.

## Status

PTCF-EXT 1 LANDED 2026-05-26. 32/34 PASS (94%). 2 residuals: @@toStringTag descriptor + Temporal.PlainTime.from (next rung).
