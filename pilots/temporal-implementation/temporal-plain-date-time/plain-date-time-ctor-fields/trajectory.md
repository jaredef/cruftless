# plain-date-time-ctor-fields — Trajectory

## PDTCF-EXT 0+1 — LANDED (2026-05-26)

Edit ~250 LOC in intrinsics.rs: pdt_proto with 9 accessor getters (year/month/day/hour/minute/second/millisecond/microsecond/nanosecond) + calendarId + monthCode + valueOf-throws + ctor with 9-arg + calendar.

Probes — all 9 expected outcomes ✓

Yield: 0 → 48/56 PASS (86%). Diff-prod 42/42.

Cumulative Temporal yield post-PDTCF: 900/1752 (51%).
