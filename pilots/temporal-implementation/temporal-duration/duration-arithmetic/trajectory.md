# duration-arithmetic — Trajectory

## DA-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Seventh sub-rung of temporal-duration. Per-class arithmetic.

### Edit (~150 LOC in intrinsics.rs)

- `read_duration_units(rt, v)`: coerce arg to Duration (string via parse_iso_duration | object brand-or-bag); return [f64; 10].
- `duration_add_impl(rt, dur_proto, a, b)`:
  - Element-wise sum.
  - Sub-day balancing: total_ns from days through nanoseconds; decompose abs into days/h/m/s/ms/μs/ns with carry; sign-prefix per total sign.
  - Mixed-sign check: if date (year/month/week) and sub-day have opposite signs → RangeError (cannot balance without relativeTo).
  - validate_uniform_sign on year/month/week portion.
- add(other) / subtract(other) dispatch.

### Probes (Rule 23 verification at landing)

- `Duration.from({days:1, minutes:5}).add({days:2, minutes:5})` → days=3, minutes=10 ✓
- Fractional ISO duration ("P50DT50.5S") still blocked by IDP fractional-residual (deferred to iso-fractional-propagation).

### Yield

- duration-arithmetic exemplar pool (68): **0 → 24/68 PASS (35%)**.
- Diff-prod: 42/42.

Cumulative Temporal yield post-DA: **601/1166 (52%)**.

### Status

DA-EXT 1 CLOSED. Duration now at 7 sub-rungs (ctor + derived + static + with + sign-validation + string-conversion + arithmetic). Cumulative Temporal yield 52%.
