# plain-date-string-conversion — Trajectory

## PDSC-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

### Edit (~60 LOC in intrinsics.rs)

`pd_to_iso_string(rt, this_id)`: brand-check via __pd_year; read y/m/d; format `"{year}-{:02}-{:02}"` with year-form selection (4-digit / ±YYYYYY). All 3 methods dispatch to it.

### Yield

- 27/33 PASS (82%). Diff-prod 42/42.

### Status

PDSC-EXT 1 CLOSED.
