---
name: plain-date-derived-properties
description: Fifth sub-rung of temporal-plain-date. Implements 11 derived getters (dayOfWeek/dayOfYear/daysInMonth/daysInWeek/daysInYear/inLeapYear/monthsInYear/weekOfYear/yearOfWeek/era/eraYear).
type: project
---

# plain-date-derived-properties — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-date/`.

Calendar-arithmetic getters. ISO calendar only in v1 (era/eraYear → undefined).

## Telos

- `dayOfWeek`: Mon=1..Sun=7. Uses Howard Hinnant `days_from_civil` mod 7 with epoch-day=Thursday alignment.
- `dayOfYear`: 1..366. Sum of prior months + day.
- `daysInMonth`: 28/29/30/31 per month + leap.
- `daysInWeek`: always 7.
- `daysInYear`: 365 or 366.
- `monthsInYear`: always 12.
- `inLeapYear`: bool.
- `weekOfYear` / `yearOfWeek`: ISO 8601 week date (W01 contains Jan 4; weeks Mon-Sun; year-of-week is the year containing the Thursday of the week — may differ from calendar year at year boundaries).
- `era` / `eraYear`: undefined per ISO 8601 (no eras).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `pd_days_from_civil` / `pd_is_leap` / `pd_days_in_month` / `pd_read_ymd` / `pd_iso_week` helpers + 11 getter accessors via macro (~200 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist for 11 PlainDate.prototype.X paths.
- **Exemplar suite**: 33 fixtures.

## Status

PDDP-EXT 1 LANDED 2026-05-26. 27/33 PASS (82%).
