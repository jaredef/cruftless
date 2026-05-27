# zoned-date-time-ctor-fields — Trajectory

## ZDTCF-EXT 0+1 — LANDED (2026-05-26)

Edit ~180 LOC. Ctor takes BigInt epochNs + timeZone string + optional calendar (iso8601 only). 4 getters (epochNanoseconds/epochMilliseconds/timeZoneId/calendarId). TZ string accepted-but-not-validated (full IANA TZ db deferred).

Probes ✓: ZDT(217175010123456789n, 'America/Los_Angeles') -> {epochNs, epochMs=217175010123, tzId, calId='iso8601'}.

Yield: 0 -> 25/27 PASS (93%). Diff-prod 42/42.

Cumulative Temporal yield post-ZDTCF: 1408/2808 (50%). **All 8 classes operational.**
