# date-month-convention-fix — Trajectory

## DMCF-EXT 0 — FOUNDED (2026-05-26)

Spawned per keeper directive (Telegram 9899) immediately after IDTP-EXT 1's side-finding surfaced cruft's `ymd_to_ms` month-convention bug.

### Discovery context

IDTP-EXT 1 needed correct epoch math for `Temporal.Instant.from(ISO datetime)`. Probe revealed cruft's `ymd_to_ms` interprets month=1 as January but skips February for month ≥ 2 (treating month=2 as March). IDTP wrote inline `days_from_civil` to bypass; this locale exists to fix the upstream helper.

### Verified probes (status quo bug)

- `new Date(1970, 0, 1).getTime()` → `-2678400000` (expected 0; 31 days off)
- `new Date(1970, 1, 1).getTime()` → `0` (treats month=1 as January — masks the bug for this case)
- `new Date(1975, 2, 2).getTime()` → `162950400000` (March 2, expected February 2)

### Status

DMCF-EXT 0 FOUNDED. DMCF-EXT 1 (one-function fix in `ymd_to_ms`) + DMCF-EXT 2 (cross-locale regression sweep to find bug-compensating tests) are pending; deferred per keeper directive to return to Temporal work.

Sibling reference for the fix: `pilots/temporal-implementation/temporal-iso-string-parse/iso-datetime-parse/` contains the known-good `days_from_civil` helper that can be imported / inlined.
