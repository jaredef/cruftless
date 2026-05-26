---
name: temporal-plain-date
description: Parent locale for Temporal.PlainDate substrate work. First class with calendar dependency (v1: iso8601 only).
type: project
---

# temporal-plain-date — Seed

## Parent locale under `pilots/temporal-implementation/`.

Fourth per-class parent. Surface: ~1143 tests (650 built-ins + 493 intl402). First Temporal class with calendar machinery (v1 supports only iso8601).

## Sub-rung topology

| Rung | Sub-locale | Scope | Status |
|---|---|---|---|
| 1 | `plain-date-ctor-fields/` | ctor + year/month/day/calendarId/monthCode getters + valueOf-throws + leap-year validation | LANDED |
| 2 | `plain-date-static/` | from / compare | NOT SPAWNED |
| 3 | `plain-date-derived-properties/` | dayOfWeek / dayOfYear / daysInMonth / daysInWeek / daysInYear / inLeapYear / monthsInYear / weekOfYear / yearOfWeek / era / eraYear | NOT SPAWNED |
| 4 | `plain-date-arithmetic/` | add / subtract / since / until (with calendar) | NOT SPAWNED |
| 5 | `plain-date-string-conversion/` | toString / toJSON / toLocaleString | NOT SPAWNED |
| 6 | `plain-date-conversion/` | toPlainDateTime / toPlainMonthDay / toPlainYearMonth / toZonedDateTime | NOT SPAWNED |
| 7 | `plain-date-with/` + `plain-date-withCalendar/` + `plain-date-equals/` | small per-method rungs | NOT SPAWNED |

## Carve-outs

- v1 supports only iso8601 calendar; other calendar IDs → RangeError. Hebrew, Islamic, Chinese, etc. deferred to plain-date-calendar-extensions.
- Year range bounded to ±999,999 (spec's exact ±271820 deferred to plain-date-year-range-validation).

## Status

TPD-EXT 0 FOUNDED 2026-05-26. First sub-rung (`plain-date-ctor-fields/`) LANDED 2026-05-26 with 28/38 PASS (74%).
