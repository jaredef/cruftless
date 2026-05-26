# plain-date-equals — Trajectory

## PDE-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

### Edit (~80 LOC in intrinsics.rs)

`equals(other)`: brand-check; coerce other (string → parse_iso_date | brand-check → sentinel-read | property-bag → y/m/d required); compare (y, m, d) tuple; return Boolean.

### Yield

- 18/40 PASS (45%). Diff-prod 42/42.

### Status

PDE-EXT 1 CLOSED.
