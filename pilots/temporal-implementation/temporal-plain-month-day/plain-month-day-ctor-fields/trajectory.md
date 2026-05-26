# plain-month-day-ctor-fields — Trajectory

## PMDCF-EXT 0+1 — LANDED (2026-05-26)

Edit ~200 LOC: pmd_proto with day + monthCode + calendarId getters + valueOf-throws; toString/toJSON ("MM-DD" or "YYYY-MM-DD" if refYear != 1972 or calendar != iso8601); ctor length=2 (month/day required); optional calendar + referenceISOYear (default 1972).

Probes ✓: PMD(2, 29) -> day=29, monthCode='M02', toString='02-29'; PMD(12, 2, 'iso8601', 1920).toString() -> '1920-12-02'.

Yield: 0 -> 39/51 PASS (76%). Diff-prod 42/42.

Cumulative Temporal yield post-PMDCF: 1189/2358 (50%).
