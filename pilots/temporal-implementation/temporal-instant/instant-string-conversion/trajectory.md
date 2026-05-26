# instant-string-conversion — Trajectory

## ISC-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Third sub-rung of temporal-instant. Sibling shape to PTSC.

### Edit (~100 LOC in intrinsics.rs)

- `civil_from_days(days) -> (y, m, d)` inverse of IDTP's `days_from_civil` (Howard Hinnant).
- `instant_to_iso_string(rt, this_id) -> Result<String, RuntimeError>`:
  - Read `__ti_ns` BigInt; brand-check.
  - Decompose to (epoch_sec, frac_ns):
    - Decimal-string form; split last 9 digits as nanoseconds.
    - Negative-epoch handling: when total is negative and frac > 0, real epoch_sec = -(|sec|+1) and real frac = 10^9 - |frac|.
  - epoch_sec → (days, secs_of_day) via div_euclid / rem_euclid.
  - days → (y, m, d) via civil_from_days.
  - secs_of_day → (hour, minute, second).
  - Format `"YYYY-MM-DDTHH:MM:SS"` + optional `".fff"` (trailing-zero trim) + `"Z"`.
  - Year formatting: 4-digit pad for [0000, 9999]; ±YYYYYY expanded for outside.
- toString / toJSON / toLocaleString dispatch to instant_to_iso_string (v1 ignores options).

### Probes (Rule 23 verification at landing)

- `new Temporal.Instant(217175010123456789n).toString()` → `"1976-11-18T14:23:30.123456789Z"` ✓
- `new Temporal.Instant(-217175010876543211n).toString()` → `"1963-02-13T09:36:29.123456789Z"` ✓ (negative-epoch handling)
- `new Temporal.Instant(0n).toString()` → `"1970-01-01T00:00:00Z"` ✓
- `new Temporal.Instant(1000n).toString()` → `"1970-01-01T00:00:00.000001Z"` ✓ (microsecond fractional)
- `new Temporal.Instant(0n).toJSON()` → `"1970-01-01T00:00:00Z"` ✓

### Yield

- instant-string-conversion exemplar pool (71): **0 → 33/71 PASS (46%)**.
- Diff-prod: 42/42.
- Earlier rungs stable.

Cumulative Temporal yield post-ISC: **357/564 (63%)**.

### Residual decomposition (38 fails)

| Shape | Count | Destination |
|---|---:|---|
| options.smallestUnit + fractionalSecondDigits | ~15 | instant-options-handling |
| options.roundingMode | ~5 | same |
| options.timeZone for non-UTC display | ~5 | needs tz-string-parse + offset application |
| TypeError on wrong options type | ~3 | spec-strict option-coercion |
| Time zone string edge cases | ~5 | tz-string-parse |
| misc | ~5 | per-test |

### Findings

**Finding ISC.1 (negative-epoch handling needs care)**: cruft's BigInt decimal form is straightforward, but converting negative epoch_ns into (epoch_sec, frac_ns) requires the "borrow when frac > 0" adjustment: total = -1.5s means epoch_sec = -2, frac = 5e8 (not epoch_sec = -1, frac = -5e8). Standing recommendation: any nanosecond → (seconds, fractional) decomposition over BigInt must handle the borrow case explicitly; arithmetic-only conversion produces wrong results for negative values.

**Finding ISC.2 (the inverse-parser pattern is repeatable)**: PTSC (PlainTime toString) and ISC (Instant toString) both use the inverse-parser shape: read sentinel(s), decompose to fields, format with optional fractional-trim. Each ~80-100 LOC. Standing recommendation: when implementing toString for any per-class type whose corresponding parse is already in place, expect ~80-120 LOC for the inverse and reuse the format!() + trim_end_matches('0') pattern.

### Status

ISC-EXT 1 CLOSED. Instant now has 3 sub-rungs (ctor + static + string-conversion). Cumulative Temporal yield 63%.
