# temporal-plain-date — Trajectory

## TPD-EXT 0 — FOUNDED (2026-05-26)

Fourth per-class parent. First calendar-dependent class in the program.

## PDCF-EXT 1 — plain-date-ctor-fields LANDED (2026-05-26)

### Edit (~150 LOC in intrinsics.rs)

- Removed `"PlainDate"` from foundation stub-classes loop.
- pd_proto with @@toStringTag "Temporal.PlainDate" (PropertyDescriptor with configurable:true).
- 3 unit getters (year/month/day) via accessor PropertyDescriptors.
- calendarId getter reads __pd_calendar sentinel.
- monthCode getter formats as `"M{:02}"` from __pd_month.
- valueOf throws TypeError.
- Ctor:
  - NewTarget check
  - ToNumber + finite + integer for year/month/day
  - Calendar arg defaults "iso8601"; lowercased; reject non-iso8601
  - Month range [1, 12]
  - Day range [1, max_day_for(year, month)] with leap-year aware Feb
  - Year bound ±999,999
  - Allocate + 4 sentinels (year/month/day/calendar)
- ctor.length=3 (calendar arg optional).

### Probes (Rule 23 verification at landing)

- `new PlainDate(2020, 12, 24, "iso8601")` → year=2020, month=12, day=24, calendarId="iso8601", monthCode="M12" ✓
- `new PlainDate(2020, 1, 1)` default calendarId="iso8601" ✓
- `PlainDate(2020, 1, 1)` (no new) → TypeError ✓
- `new PlainDate(2020, 13, 1)` → RangeError ✓
- `new PlainDate(2020, 2, 30)` → RangeError ✓
- `new PlainDate(2020, 2, 29)` → ok (leap year) ✓
- `new PlainDate(2021, 2, 29)` → RangeError (not leap) ✓
- `new PlainDate(2020, 1, 1, "gregory")` → RangeError (only iso8601 in v1) ✓
- `instanceof Temporal.PlainDate` → true ✓
- `name="PlainDate"`, `length=3` ✓

### Yield

- plain-date-ctor-fields exemplar pool (38): **0 → 28/38 PASS (74%)**.
- Diff-prod: 42/42.

Cumulative Temporal yield post-PDCF: **637/1204 (53%)**.

### Residuals (10)

| Shape | Count | Destination |
|---|---:|---|
| Temporal.PlainDate.from not callable | 6 | plain-date-static rung |
| Spy-based valueOf trace | 2 | brand-check observer pattern (sibling DCF) |
| @@toStringTag own-property descriptor | 1 | blocked by cruft-symbol-key-hasown-bridge (TTSTD residual) |
| valueOf trace | 1 | sibling DCF residual |

### Findings

**Finding PDCF.1 (per-class template extends to calendar-dependent classes with minimal additions)**: PlainDate adds ~30 LOC over the pure-fields template (PT/Instant) for calendar string handling + leap-year-aware day validation. The skeleton (prototype + accessors + valueOf-throws + ctor.prototype frozen) transfers directly. Standing recommendation: PlainMonthDay and PlainYearMonth ctor-fields rungs will reuse PDCF's calendar + day-validation logic; expect similar ~150 LOC per rung. PlainDateTime composes PD + PT (date + time fields); expect ~250 LOC.

### Status

PDCF-EXT 1 CLOSED. PlainDate now operational at the ctor + field-getters level. Cumulative Temporal yield 53%.
