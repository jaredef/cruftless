---
name: plain-date-ctor-fields
description: First sub-rung of temporal-plain-date. Implements ctor + 5 accessor getters (year, month, day, calendarId, monthCode) + valueOf-throws + leap-year-aware day validation.
type: project
---

# plain-date-ctor-fields — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-date/`.

First per-class ctor-fields rung for a calendar-dependent class. Validates that the per-class template extends with minimal additions.

## Telos

- `Temporal.PlainDate` ctor: `new PlainDate(year, month, day, calendarLike="iso8601")`.
  - NewTarget check.
  - year/month/day: ToNumber + finite + integer.
  - calendarLike: undefined → "iso8601"; string → lowercase + accept only "iso8601" (v1 carve-out).
  - Range checks: month [1, 12]; day [1, max] with leap-year aware February.
  - Year bound ±999,999 (spec ±271820 deferred to plain-date-year-range-validation).
- Instance: 4 sentinels (__pd_year, __pd_month, __pd_day, __pd_calendar).
- Getters: year/month/day return Number sentinels; calendarId returns String; monthCode formats as "M{:02}".
- valueOf throws TypeError.
- @@toStringTag = "Temporal.PlainDate" (configurable:true per TTSTD pattern).
- ctor.length = 3.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — PlainDate class block (~150 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with 20 PlainDate test paths.
- **Exemplar suite**: 38 fixtures.

## Status

PDCF-EXT 1 LANDED 2026-05-26. 28/38 PASS (74%).
