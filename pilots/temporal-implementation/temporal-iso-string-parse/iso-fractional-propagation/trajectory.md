# iso-fractional-propagation — Trajectory

## IFP-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Third shared-substrate rung in temporal-iso-string-parse.

### Edit (~40 LOC in intrinsics.rs::parse_iso_duration)

Post-loop propagation block:
- If `fractional_taken` is set with slot ∈ [4, 6], cascade `frac` downward:
  - slot 4 → minutes (frac*60), residual → seconds, residual → sub-second.
  - slot 5 → seconds, residual → sub-second.
  - slot 6 → sub-second only.
- Slot < 4 (Y/M/W/D fractional) → return None (REJECT per spec absent relativeTo).

### Two-stage Rule 23 discovery

First cut: PT1.5H gave minutes=0, seconds=1800 (skipped minutes). Probe surfaced spec semantics — fractional propagates to NEXT-SMALLER unit, not directly to seconds. Rewrote as cascading: H→M→S→sub-sec. Probes now match.

### Probes (Rule 23 verification at landing)

- `Temporal.Duration.from("PT1.5H")` → hours=1, minutes=30 ✓
- `Temporal.Duration.from("PT0.5M")` → minutes=0, seconds=30 ✓
- `Temporal.Duration.from("PT1.5S")` → seconds=1, milliseconds=500 ✓

### Yield (sibling deltas)

- duration-static: 31 → 35 (+4)
- duration-string-conversion: 33 unchanged
- duration-arithmetic: 27 unchanged
- duration-ctor-fields / derived-properties / with / sign-validation: unchanged

Net: +4 across Duration sub-rungs. Smaller than the initial 7 IDP residual estimate; some of those residuals depended on additional substrate (BigInt precision, options).

Diff-prod: 42/42.

Cumulative Temporal yield post-IFP: **605/1166 (52%)**.

### Status

IFP-EXT 1 CLOSED. ISO duration string parser now handles fractional propagation per spec. Temporal-iso-string-parse parent now has 3 sub-rungs landed (IDP + IDTP + IFP).
