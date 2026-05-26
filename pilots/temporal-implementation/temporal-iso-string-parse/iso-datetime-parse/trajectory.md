# iso-datetime-parse — Trajectory

## IDTP-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Second shared-substrate rung. Sibling to IDP.

### Edit (~150 LOC in intrinsics.rs)

`parse_iso_datetime(&str) -> Option<(i64, i64)>` hand-written parser:
- `rd(b, i, n)` helper to read N ASCII digits.
- Date: YYYY-MM-DD with required `-` separators.
- Time separator: T (case-insensitive) or space.
- Time: HH:MM required; HH:MM:SS optional; fractional via `.` or `,` (1-9 digits) optional.
- Offset: Z/z (UTC) OR ±HH:MM (extended), ±HHMM (compact), ±HH:MM:SS (sub-minute).
- Bracketed annotations: repeated `[…]` blocks consumed and ignored.
- Validates full consume.

Inline `days_from_civil(y, m, d)` helper using the correct Howard Hinnant chrono algorithm — works around a latent bug in cruft's existing `ymd_to_ms` (which skips February for month >= 2 due to a `month < 2` vs `m <= 2` mis-condition that compounds with `month - 2` vs `m - 3` in the algo body).

Wired into:
- `Temporal.Instant.from(string)` — replaces deferral with parse + BigInt epoch_ns composition.
- `Temporal.Instant.compare(string)` — replaces deferral with parse + f64 compare.

### Two-stage Rule 23 discovery

First cut: used existing `ymd_to_ms` directly with `month - 1`. Probe showed `1970-01-01T00:00Z` → -2678400000000000n (31 days before epoch). After empirical probes (`new Date(1970, N, 1).getTime()`), discovered cruft's helper interprets month=1 as January but month=2 as MARCH (skips February entirely for month >= 2). Wrote inline correct algorithm; probes now match spec.

### Probes (Rule 23 verification at landing)

- `Temporal.Instant.from("1970-01-01T00:00:00Z").epochNanoseconds === 0n` ✓
- `Temporal.Instant.from("1975-02-02T13:25:36.123456789Z").epochMilliseconds === 160579536123` ✓
- `Temporal.Instant.from("1975-02-02T14:25:36.123456789+01:00").epochMilliseconds === 160579536123` ✓ (offset application)
- `Temporal.Instant.from("1975-02-02T14:25:36.123456789+01:00[Invalid/TimeZone]")` — annotation ignored ✓
- `Temporal.Instant.from("1970-01-01t00:00Z")` (lowercase t) ✓
- `Temporal.Instant.from("1970-01-01 00:00Z")` (space separator) ✓
- `Temporal.Instant.compare("1970-01-01T00:00:00Z", "1970-01-01T00:00:01Z") === -1` ✓
- `Temporal.Instant.from("invalid")` → RangeError ✓
- `Temporal.Instant.from("1970-01-01T00:00")` → RangeError (no offset) ✓

### Yield

- instant-static: 28 → 53 (+25)
- instant-ctor-fields: 21 (unchanged)
- Duration sub-rungs (no string entry point affected): all stable

Diff-prod: 42/42.

Cumulative Temporal yield post-IDTP: **208/300 (69%)**.

### Findings

**Finding IDTP.1 (latent bugs in shared helpers surface when their convention is implicit)**: cruft's `ymd_to_ms` is widely used by `Date` and tested through the Date ctor — but the Date ctor itself has an off-by-31-days bug for January (`new Date(1970, 0, 1).getTime() === -2678400000`, not 0). The bug compensated by tests not noticing because they all probe relative semantics. When IDTP needed correct epoch math, the bug surfaced. Standing recommendation: shared helpers with implicit semantic conventions (calendar systems, indexing, sign rules) need explicit DocStrings + unit tests with golden values; otherwise downstream consumers inherit the bug.

**Finding IDTP.2 (shared sub-substrates compound their yield faster than per-class rungs)**: IDTP added +25 in one rung — comparable to a full per-class rung's yield. IDP (the sibling) added +5. The compounding effect: every per-class string entry point benefits. Future per-class rungs (PlainDate.from(string), etc.) will inherit IDTP's parser. Standing recommendation: when the substrate program enters a phase where multiple downstream rungs share an unbuilt sub-substrate, prioritize the sub-substrate rung over the next per-class rung — the cross-sibling yield often exceeds the per-class yield.

### Status

IDTP-EXT 1 CLOSED. Next ripe: temporal-plain-time / plain-time-ctor-fields (~80 LOC, third per-class, no calendar/TZ entanglement) OR iso-fractional-propagation (closes 7 Duration deferrals).
