---
name: plain-time-string-conversion
description: Fourth sub-rung of temporal-plain-time. Implements toString / toJSON / toLocaleString — ISO HH:MM:SS[.fff] formatting with trailing-zero trimming.
type: project
---

# plain-time-string-conversion — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-time/`.

Reverse direction of PTS's parse_iso_time — composes ISO 8601 time strings from PlainTime instances.

## Telos

- `Temporal.PlainTime.prototype.toString()`: returns `"HH:MM:SS"` or `"HH:MM:SS.fff…"` where fractional part is the minimum-digit representation (trim trailing zeros), absent entirely when all sub-second units are 0.
- `Temporal.PlainTime.prototype.toJSON()`: equivalent to `toString()`, ignores its argument.
- `Temporal.PlainTime.prototype.toLocaleString()`: v1 falls back to ISO form (Intl-coordinated localization deferred).

## Carve-outs (residuals expected)

- **options.smallestUnit** / **options.fractionalSecondDigits**: spec allows forcing fractional precision or truncating to a coarser unit. ~20 records deferred to `plain-time-options-handling` (sibling of PTW deferral).
- **options.roundingMode** for sub-second truncation: ~2 records, same destination.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `pt_to_iso_string` shared helper + 3 methods registered on pt_proto.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/PlainTime/prototype/{toString,toJSON,toLocaleString}/`.
- **Exemplar suite**: 54 fixtures.

## Status

PTSC-EXT 1 LANDED 2026-05-26. 26/54 PASS (48%).
