# temporal-availability/plain-datetime-semantics — Trajectory

## PDTS-EXT 0 — Founding residual extraction (2026-05-27)

**Trigger**: Parent `temporal-availability` reached TA-EXT 18 with the
founding missing-global coordinate closed and the direct Temporal
exemplar suite at 71/100. The largest remaining direct bucket was
`PlainDateTime` with eight rows.

**Change**:

- Promoted a nested locale under the parent Temporal availability
  coordinate.
- Added a focused PlainDateTime residual exemplar list.
- Added a focused runner that reports local pass/fail counts while using
  the same test262 harness wrapper and `scripts/env.sh` path discipline
  as the parent suite.

**Baseline**:

```text
PlainDateTime focused residual: PASS=0 FAIL=8 / 8 (0.0%)
Parent Temporal after TA-EXT 18: PASS=71 FAIL=29 / 100 (71.0%)
Parent Intl402 after TA-EXT 18: PASS=37 FAIL=63 / 100 (37.0%)
```

**Finding PDTS.1 (semantic surface, not binding surface)**:
The PlainDateTime residual is not a missing constructor/prototype
problem. It splits across object conversion, calendar identifier
canonicalization, time-string parsing, and duration arithmetic/rounding.
The first substrate move should prefer the smallest reusable conversion
helper rather than adding isolated fixture branches.

**Status**: PDTS-EXT 0 FOUNDED locally.
