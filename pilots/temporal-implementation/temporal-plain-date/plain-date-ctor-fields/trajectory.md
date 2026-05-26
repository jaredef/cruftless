# plain-date-ctor-fields — Trajectory

## PDCF-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

First per-class rung for a calendar-dependent Temporal class.

### Edit (~150 LOC in intrinsics.rs)

Detail in seed.md. Highlights:
- Inline leap-year computation (`(y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)`).
- max_day per-month table (28/29 for Feb based on leap, else 30 or 31).
- Calendar carve-out: lowercase string; reject anything other than "iso8601".

### Yield

- 28/38 PASS (74%).
- Diff-prod 42/42.
- Cumulative Temporal post-PDCF: 637/1204 (53%).

### Residuals (10)

- 6 PlainDate.from not callable → plain-date-static next.
- 2 spy-based valueOf trace → brand-check observer pattern (sibling DCF).
- 1 @@toStringTag prop-desc → blocked by cruft-symbol-key-hasown-bridge.
- 1 valueOf trace.

### Status

PDCF-EXT 1 CLOSED. Next: plain-date-static (from + compare, with ISO date parsing).
