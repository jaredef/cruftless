# duration-sign-validation — Trajectory

## DSV-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Spawned per keeper directive (Telegram 9889) following Finding DWith.2's standing-rec. First cross-cutting validation rung in the Temporal program.

### Edit (~25 LOC in intrinsics.rs::install_temporal)

- `validate_uniform_sign(&[f64; 10]) -> Result<(), RuntimeError>` inline helper.
- 3 call sites: ctor (after units assembled), from() property-bag (after collection), with() (after merge).

### Probes (Rule 23 verification at landing)

- `new Temporal.Duration(1, -2)` → RangeError "all non-zero unit fields must share sign" ✓
- `new Temporal.Duration(1, 2, 3)` (uniform positive) → succeeds ✓
- `new Temporal.Duration()` (all zero) → succeeds ✓
- `Temporal.Duration.from({years:1, months:-2})` → RangeError ✓
- `new Temporal.Duration(1, 2).with({hours:-1})` → RangeError (merge produces mixed sign) ✓

### Yield (sibling deltas)

| Rung | Pre-DSV | Post-DSV | Delta |
|---|---:|---:|---:|
| duration-ctor-fields | 64/67 | 64/67 | 0 |
| duration-derived-properties | 23/24 | 23/24 | 0 |
| duration-static | 22/81 | 23/81 | +1 |
| duration-with | 17/22 | 19/22 | +2 |
| **TOTAL** | **126/194** | **129/194** | **+3** |

Diff-prod: 42/42 maintained.

### Finding DWith.2 prediction refinement

Finding DWith.2 predicted +7 sibling yield (3 DWith + 4 elsewhere). Actual: +3. The over-estimate came from:
- DCF's "fractional-throws-rangeerror" / "infinity-throws-rangeerror" / "negative-infinity-throws-rangeerror" tests already passed via the existing integer + finite checks; they weren't sign-related.
- DStat's residuals are dominated by ISO-string-parse / relativeTo deferrals, not sign issues.

### Findings

**Finding DSV.1 (cross-cutting validation rungs land cleanly when the predicate is small)**: The 25-LOC helper + 3 call sites pattern is the canonical shape for cross-cutting validation. As Temporal classes accumulate (PlainDate, PlainTime, etc.), expect similar cross-cutting rungs: `plain-date-range-validation`, `plain-time-range-validation`, etc. Standing recommendation: when a class's construction sites all need the same downstream-of-args validation, factor it into a helper + N call sites in one rung rather than scattering the check inline at each site.

**Finding DSV.2 (predicted-yield estimates should err conservative for cross-cutting rungs)**: Sibling residuals look more uniform than they are. Without per-residual inspection, Finding DWith.2's +7 estimate over-counted. Standing recommendation: for cross-cutting rungs, sample 1-2 residuals per sibling rung BEFORE estimating yield — the categorizer's reason-text alone doesn't always discriminate the underlying mechanism.

### Status

DSV-EXT 1 CLOSED. Temporal-duration is now at 5 sub-rungs landed; cumulative yield 129/194 (66.5%). Remaining sub-rungs: duration-string-conversion (needs temporal-iso-string-parse), duration-arithmetic, duration-rounding, duration-relative-to.
