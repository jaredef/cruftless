# plain-year-month-ctor-fields — Trajectory

## PYMCF-EXT 0+1 — LANDED (2026-05-26)

Edit ~300 LOC: 10 getters (year/month/monthCode/calendarId/daysInMonth/daysInYear/monthsInYear/inLeapYear/era/eraYear); valueOf-throws; toString/toJSON ("YYYY-MM" default; "YYYY-MM-DD" if refDay != 1 or calendar non-default); ctor length=2 (year/month required); optional calendar + referenceISODay.

Reuses pda_days_in_month + pda_is_leap helpers from PDA.

Probes ✓: PYM(1976, 11) -> year=1976, month=11, monthCode='M11', daysInMonth=30, daysInYear=366, toString='1976-11'.

Yield: 0 -> 62/75 PASS (83%). Diff-prod 42/42.

Cumulative Temporal yield post-PYMCF: 1251/2433 (51%).
