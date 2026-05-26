# duration-string-conversion — Trajectory

## DSC-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Sixth sub-rung of temporal-duration. Reaches feature parity with PTSC + ISC at the string-conversion layer.

### Edit (~100 LOC in intrinsics.rs)

`duration_to_iso_string(rt, this_id) -> Result<String, RuntimeError>`:
- Brand-check via `__td_years`.
- Read 10 unit sentinels.
- Detect sign (any non-zero negative); abs all units; emit `-` prefix once.
- Sub-second roll-up: `total_ns = ms*1e6 + μs*1e3 + ns`; `carry = total_ns / 1e9`, `frac = total_ns % 1e9`; `seconds_total = seconds + carry`.
- Build `P[nY][nM][nW][nD][T[nH][nM][nS]]` emitting only non-zero units (except seconds is force-emitted when frac > 0 OR T has no other units OR everything is zero).
- Fractional: 9-digit zero-pad + trim trailing zeros.

3 methods (toString / toJSON / toLocaleString) all dispatch to the helper; v1 ignores options/locale.

### Probes (Rule 23 verification at landing)

- `new Temporal.Duration().toString()` → `"PT0S"` ✓
- `new Temporal.Duration(1,2,3,4,5,6,7,8,9,1).toJSON()` → `"P1Y2M3W4DT5H6M7.008009001S"` ✓
- `new Temporal.Duration(0,0,0,0,0,0,4,3,2,1).toJSON()` → `"PT4.003002001S"` ✓ (sub-second composition)
- `new Temporal.Duration(0,0,0,0,0,0,-4,-3,-2,-90080001).toJSON()` → `"-PT4.093082001S"` ✓ (negative + larger subseconds + carry)
- `new Temporal.Duration(1).toString()` → `"P1Y"` ✓ (date-only, no T part)
- `new Temporal.Duration(0,0,0,0,1,30).toString()` → `"PT1H30M"` ✓ (time-only, no seconds emit)

### Yield

- duration-string-conversion exemplar pool (63): **0 → 33/63 PASS (52%)**.
- Diff-prod: 42/42.
- Earlier rungs stable.

Cumulative Temporal yield post-DSC: **390/627 (62%)**.

### Residual decomposition (30 fails)

| Shape | Count | Destination |
|---|---:|---|
| options.smallestUnit / fractionalSecondDigits / roundingMode | ~10 | duration-options-handling |
| RangeError edge cases | ~6 | per-test |
| BigInt precision (PT5188146770.73…) | 1 | duration-bigint-precision |
| Fractional unit out of position (from-string round-trip) | 1 | iso-fractional-propagation |
| TypeError on wrong options type | ~3 | spec-strict option-coercion |
| misc | ~9 | per-test |

### Findings

**Finding DSC.1 (toString tier reached parity for all three pure-fields classes)**: Duration / Instant / PlainTime now each have their toString/toJSON/toLocaleString rung landed. The shape: read sentinels + decompose / compose + format!() with trim_end_matches('0') for fractional. Per-class LOC ranged 80-100. Standing recommendation: the same pattern will apply to PlainDate (when its parser lands) and any other future per-class string conversion; the standing template is now mature.

**Finding DSC.2 (sub-second roll-up is a Duration-specific concern not present in PlainTime)**: PlainTime's ms/μs/ns each have range [0, 999] so they don't roll over. Duration's units are unbounded so `total_ns = ms*1e6 + μs*1e3 + ns` can exceed 1e9 and must carry into seconds. Standing recommendation: Duration's toString needs the carry; other classes whose sub-second units have spec-bounded ranges (PlainTime, PlainDateTime time-portion) don't.

### Status

DSC-EXT 1 CLOSED. Duration now has 6 sub-rungs (ctor + derived + static + with + sign-validation + string-conversion). Cumulative Temporal yield 62%.
