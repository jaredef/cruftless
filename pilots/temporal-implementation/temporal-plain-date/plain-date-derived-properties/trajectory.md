# plain-date-derived-properties — Trajectory

## PDDP-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

### Edit (~200 LOC in intrinsics.rs)

Inline helpers:
- `pd_days_from_civil(y, m, d) -> i64`: Howard Hinnant forward direction.
- `pd_is_leap(y) -> bool`.
- `pd_days_in_month(y, m) -> i64`.
- `pd_read_ymd(rt, id, name) -> (y, m, d)` with brand-check.
- `pd_iso_week(y, m, d) -> (year_of_week, week)`: ISO 8601 week-date computation by finding the Thursday of the week, then the year-of-week containing that Thursday; iterates ±1 candidate year for boundary cases.
- `pd_getter!` macro for accessor-PropertyDescriptor installation.

11 getters installed via the macro.

### Probes (Rule 23 verification at landing)

- `PlainDate(2020, 5, 15).dayOfWeek` → 5 (Friday) ✓
- `.dayOfYear` → 136 (Jan 31 + Feb 29 + Mar 31 + Apr 30 + 15) ✓
- `.daysInMonth` (May) → 31 ✓
- `.daysInYear` (2020 leap) → 366 ✓
- `.inLeapYear` → true ✓
- `.daysInWeek` → 7 ✓
- `.monthsInYear` → 12 ✓
- `.era / .eraYear` → undefined ✓ (ISO calendar)
- `.weekOfYear` (May 15 2020) → 20 ✓
- `.yearOfWeek` → 2020 ✓
- Jan 1 2020 → week 1, year-of-week 2020 ✓
- Jan 1 2021 → week 53, year-of-week 2020 ✓ (boundary case: Fri Jan 1 2021 belongs to last week of 2020)

### Yield

- plain-date-derived-properties exemplar pool (33): **0 → 27/33 PASS (82%)**.
- Diff-prod: 42/42.

Cumulative Temporal yield post-PDDP: **763/1423 (54%)**.

### Status

PDDP-EXT 1 CLOSED. PlainDate now at 5 sub-rungs (ctor + static + string-conversion + equals + derived-properties).
