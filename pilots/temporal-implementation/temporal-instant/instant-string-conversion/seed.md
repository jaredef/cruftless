---
name: instant-string-conversion
description: Third sub-rung of temporal-instant. Implements toString / toJSON / toLocaleString — inverse of IDTP parser; outputs YYYY-MM-DDTHH:MM:SS[.fff]Z with proper negative-epoch handling.
type: project
---

# instant-string-conversion — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-instant/`.

Sibling shape to PTSC. Reverse direction of IDTP parser.

## Telos

- `Temporal.Instant.prototype.toString()`:
  - Decompose `__ti_ns` BigInt into (epoch_sec, frac_ns) handling negative epoch correctly.
  - epoch_sec → (year, month, day) via inverse Howard Hinnant `civil_from_days`.
  - epoch_sec % 86400 → (hour, minute, second).
  - Format `"YYYY-MM-DDTHH:MM:SS[.fff]Z"` where fractional trims trailing zeros and is absent when zero.
  - Year formatting: 4-digit zero-pad for [0000, 9999]; ±YYYYYY expanded form otherwise.
- `toJSON()`: same as toString.
- `toLocaleString()`: v1 falls back to ISO form.

## Carve-outs (residuals expected)

- **options.smallestUnit** / **fractionalSecondDigits** / **roundingMode**: ~25 records deferred to `instant-options-handling` (sibling of PTW/PTSC options deferral).
- **options.timeZone** for non-UTC display: ~5 records deferred (needs tz-string-parse + offset application in reverse).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `instant_to_iso_string` + `civil_from_days` helpers + 3 methods on inst_proto (~100 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/Instant/prototype/{toString,toJSON,toLocaleString}/`.
- **Exemplar suite**: 71 fixtures.

## Status

ISC-EXT 1 LANDED 2026-05-26. 33/71 PASS (46%). Basic forms (incl. negative epoch) correct.
