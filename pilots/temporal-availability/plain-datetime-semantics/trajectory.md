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

## PDTS-EXT 1 — PlainDateTime conversion fast paths (2026-05-27)

**Trigger**: The focused residual had two conversion rows that reused
existing Temporal slots rather than requiring new arithmetic semantics:

```text
Temporal.PlainDateTime.prototype.withPlainTime/argument-string-without-time-designator.js
  time-like string works: hour result: Expected SameValue(«3», «12») to be true

Temporal.PlainDateTime.prototype.equals/argument-plaindate.js
  Expected true but got false
```

**Change**:

- Routed `PlainDateTime.prototype.withPlainTime` to a scoped helper.
- Reused the PlainTime string parser from the parent Temporal trajectory
  so `"12:34"` replaces only the time slots on the copied DateTime.
- Routed `PlainDateTime.prototype.equals` through a slot comparison path.
- Implemented the sampled PlainDate fast path by comparing date slots
  and treating the PlainDate time as midnight without reading observable
  properties.

**Verification**:

```text
cargo build -p cruftless --bin cruft
Temporal.PlainDateTime.prototype.withPlainTime/argument-string-without-time-designator.js: PASS
Temporal.PlainDateTime.prototype.equals/argument-plaindate.js: PASS
pilots/temporal-availability/plain-datetime-semantics/exemplars/run-exemplars.sh
pilots/temporal-availability/exemplars/run-exemplars.sh
pilots/intl402-availability/exemplars/run-exemplars.sh
scripts/diff-prod/run-all.sh
```

Exemplar movement:

```text
PlainDateTime focused after PDTS-EXT 0: PASS=0 FAIL=8 / 8 (0.0%)
PlainDateTime focused after PDTS-EXT 1: PASS=2 FAIL=6 / 8 (25.0%)
Parent Temporal after PDTS-EXT 1: PASS=73 FAIL=27 / 100 (73.0%)
Parent Intl402 after PDTS-EXT 1: PASS=37 FAIL=63 / 100 (37.0%)
diff-prod after PDTS-EXT 1: 42/42 PASS
```

Post-PDTS-EXT 1 focused residual:

```text
Temporal.PlainDateTime.prototype.add/argument-string-fractional-units-rounding-mode.js
Temporal.PlainDateTime.prototype.round/roundingmode-halfFloor.js
Temporal.PlainDateTime.prototype.since/float64-representable-integer.js
Temporal.PlainDateTime.prototype.since/roundingmode-halfExpand.js
Temporal.PlainDateTime.prototype.since/roundingmode-halfexpand-default-changes.js
Temporal.PlainDateTime.prototype.withCalendar/calendar-case-insensitive.js
```

**Finding PDTS.2 (conversion rows are now closed)**:
The focused PlainDateTime residual is now entirely arithmetic/rounding
plus one calendar-canonicalization row. The next coherent move should
not extend the conversion helper unless a new focused failure appears;
it should target duration arithmetic or the isolated calendar case.

**Status**: PDTS-EXT 1 CLOSED locally.
