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

## PDTS-EXT 2 — PlainDateTime withCalendar ASCII canonicalization (2026-05-27)

**Trigger**: After PDTS-EXT 1, the focused residual had one non-arithmetic
row:

```text
Temporal.PlainDateTime.prototype.withCalendar/calendar-case-insensitive.js
  calendar ID is ASCII-lowercased, capital dotted I is not lowercased
  Expected a RangeError to be thrown but no exception was thrown at all
```

The row is not duration math. It probes CalendarIdentifier
canonicalization: ASCII case-insensitive `"iSo8601"` maps to `"iso8601"`,
but Unicode dotted-I must not be folded by locale-aware lowercasing.

**Change**:

- Routed `PlainDateTime.prototype.withCalendar` through a scoped helper.
- Added bounded ASCII-only calendar identifier canonicalization for the
  sampled `iso8601` calendar.
- Rejected non-ASCII calendar identifiers with `RangeError`, closing the
  dotted-I case without claiming full calendar registry support.
- Copied the receiver slots into the returned PlainDateTime and installed
  the canonical calendar slot observed by `calendarId`.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build --release -p cruftless
Temporal.PlainDateTime.prototype.withCalendar/calendar-case-insensitive.js: PASS
pilots/temporal-availability/plain-datetime-semantics/exemplars/run-exemplars.sh
pilots/temporal-availability/exemplars/run-exemplars.sh
pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
PlainDateTime focused after PDTS-EXT 1: PASS=2 FAIL=6 / 8 (25.0%)
PlainDateTime focused after PDTS-EXT 2: PASS=3 FAIL=5 / 8 (37.5%)
Parent Temporal after PDTS-EXT 2: PASS=74 FAIL=26 / 100 (74.0%)
Parent Intl402 after PDTS-EXT 2: PASS=37 FAIL=63 / 100 (37.0%)
```

Post-PDTS-EXT 2 focused residual:

```text
Temporal.PlainDateTime.prototype.add/argument-string-fractional-units-rounding-mode.js
Temporal.PlainDateTime.prototype.round/roundingmode-halfFloor.js
Temporal.PlainDateTime.prototype.since/float64-representable-integer.js
Temporal.PlainDateTime.prototype.since/roundingmode-halfExpand.js
Temporal.PlainDateTime.prototype.since/roundingmode-halfexpand-default-changes.js
```

**Finding PDTS.3 (focused residual is now pure arithmetic/rounding)**:
The calendar conversion/canonicalization rows are closed. The next rung
should promote a PlainDateTime arithmetic helper shared by `add`, `round`,
and `since`, rather than continuing one-row conversion stubs.

**Status**: PDTS-EXT 2 CLOSED locally.

## PDTS-EXT 3 — PlainDateTime arithmetic projection and Duration surface (2026-05-27)

**Trigger**: After PDTS-EXT 2, every remaining focused row shared the same
substrate: PlainDateTime arithmetic was still falling through generic
Temporal stubs, so `add`, `round`, and `since` returned object-shaped
answers without civil-time arithmetic content.

Focused residual before this extension:

```text
Temporal.PlainDateTime.prototype.add/argument-string-fractional-units-rounding-mode.js
Temporal.PlainDateTime.prototype.round/roundingmode-halfFloor.js
Temporal.PlainDateTime.prototype.since/float64-representable-integer.js
Temporal.PlainDateTime.prototype.since/roundingmode-halfExpand.js
Temporal.PlainDateTime.prototype.since/roundingmode-halfexpand-default-changes.js
```

**Change**:

- Routed `PlainDateTime.prototype.add`, `round`, `since`, and `until`
  through a scoped PlainDateTime arithmetic helper.
- Added an ISO civil-date to epoch-nanosecond projection and the inverse
  projection back into PlainDateTime slots.
- Added bounded duration-string time parsing for the sampled fractional
  time-unit add cases.
- Extended rounding with `days` and `halfExpand` support so PlainDateTime
  can share the nanosecond coordinate with existing Temporal helpers.
- Decomposed `since` into Duration slots for days-and-lower units plus the
  sampled years/months/weeks rounding cases.
- Installed `Temporal.Duration.prototype.toString` and a callable
  `Duration.prototype.add` surface so large-duration precision fixtures can
  observe the spec-shaped string and comparison path.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build --release -p cruftless
pilots/temporal-availability/plain-datetime-semantics/exemplars/run-exemplars.sh
pilots/temporal-availability/exemplars/run-exemplars.sh
pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
PlainDateTime focused after PDTS-EXT 2: PASS=3 FAIL=5 / 8 (37.5%)
PlainDateTime focused after PDTS-EXT 3: PASS=8 FAIL=0 / 8 (100.0%)
Parent Temporal after PDTS-EXT 3: PASS=79 FAIL=21 / 100 (79.0%)
Parent Intl402 after PDTS-EXT 3: PASS=37 FAIL=63 / 100 (37.0%)
```

Post-PDTS-EXT 3 parent Temporal residual:

```text
7 ZonedDateTime
7 PlainDate
7 Duration
```

**Finding PDTS.4 (locale closure)**: The focused PlainDateTime semantic
availability locale is closed at exemplar resolution. The remaining parent
Temporal failures now sit in adjacent classes rather than this locale:
ZonedDateTime, PlainDate, and Duration. Intl402 still reports Temporal-heavy
failures, so the next high-yield step should either bridge formatter use of
PlainDateTime or pick one of the three parent Temporal residual classes.

**Status**: PDTS-EXT 3 CLOSED locally.
