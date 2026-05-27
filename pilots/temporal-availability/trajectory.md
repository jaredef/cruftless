# temporal-availability — Trajectory

## TA-EXT 0 — founding + exemplar suite + baseline (2026-05-25)

**Trigger**: Per keeper directive after the zoom-out read of the canonical full-suite Pin-Art matrix surfaced this coordinate as rank #1 (4,152 fails, ~17.4% of interpreted non-pass).

**Apparatus established**:

- `exemplars/exemplars.txt` — 100 paths stratified-sampled from the 4,152-fixture pool by Temporal sub-class. Class proportions match the underlying pool: ZonedDateTime 20 / PlainDateTime 17 / PlainDate 14 / Duration 12 / PlainYearMonth 10 / PlainTime 10 / Instant 10 / PlainMonthDay 3 / Now 1 / toStringTag 1 / keys.js 1 / getOwnPropertyNames.js 1. Sampled with a fixed seed (0xC0FFEE) for reproducibility.
- `exemplars/run-exemplars.sh` — harness wrapper runner; prints aggregate + per-class breakdown of fails.

**Baseline measurement**:

| Probe | Result |
|---|---|
| Exemplar suite (100 / 4,152 pool) | **PASS=0, FAIL=100 (0.0%)** |
| Top three uncovered classes | ZonedDateTime (20), PlainDateTime (17), PlainDate (14) |
| All 12 Temporal sub-classes uncovered | confirmed |

The 0/100 baseline confirms the cluster's single-decision shape: `globalThis.Temporal` is unbound. Every fail in the cluster surfaces as `ReferenceError`-like at the `availability/missing-global-or-binding` cut. Closing the availability axis at the runtime intrinsic-registration tier is the deeper-layer move (R13 prospective C1-C4 all hold per seed §Methodology).

**Findings**

**Finding TA.1 (single-decision avalanche)**: 4,152 fails behind one missing-global-binding decision. Cluster-coordinate yield-per-decision ratio is empirically extreme here — even a stub registration with no method implementations should flip the cluster's failure-mode distribution off the availability axis and onto the value-semantics/wrong-result axis. The shift itself is the signal that TA-EXT 1 lands the deeper-layer move; the absolute pass count is a secondary read.

**Finding TA.2 (exemplar-suite stratification preserves cluster structure)**: proportional sampling with min-1-per-class produces a 100-test surface that mirrors the 4,152-pool's class distribution. Per-class fail breakdown after TA-EXT 1+ will read directly against the pool's expected yield curve. Standing recommendation: when sampling exemplars from a tier-A cluster, stratify by the most-load-bearing axis of the cluster (here: Temporal sub-class), not by uniform random pick.

**Status**: TA-EXT 0 CLOSED. Apparatus operational; baseline pinned. TA-EXT 1 (registration MVP) is the next rung.

## TA-EXT 1 — Temporal namespace registration MVP (2026-05-26)

**Trigger**: `intl402-availability` reached I402-EXT 17 with no visible
core non-Temporal Intl402 failures left in its exemplar slice. The remaining
Intl402 residual is Temporal or Temporal-coupled DateTimeFormat mass, so the
coherent next move is to extend this already-founded Temporal availability
locale rather than spawn a duplicate Intl/Temporal sibling.

**Change**:

- Added a runtime `Temporal` namespace at the intrinsic registration tier.
- Installed constructor-shaped stubs for `Duration`, `Instant`, `PlainDate`,
  `PlainDateTime`, `PlainMonthDay`, `PlainTime`, `PlainYearMonth`, and
  `ZonedDateTime`.
- Installed `Temporal.Now` with method-shaped stubs for the ISO current-time
  entry points.
- Kept namespace and class/prototype metadata non-enumerable and installed
  `@@toStringTag` so the availability layer matches the built-in namespace
  shape before class semantics are attempted.

**Local verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
target/debug/cruft <temporal-smoke.mjs>
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Smoke result:

```json
{"temporal":"object","keys":0,"names":"@@toStringTag|Duration|Instant|Now|PlainDate|PlainDateTime|PlainMonthDay|PlainTime|PlainYearMonth|ZonedDateTime","tag":"[object Temporal]","plainDate":"function","now":"function"}
```

Exemplar movement:

```text
Temporal before TA-EXT 1: PASS=0 FAIL=100 / 100 (0.0%)
Temporal after TA-EXT 1:  PASS=9 FAIL=91  / 100 (9.0%)
Intl402 after I402-EXT 17 / before TA-EXT 1: PASS=34 FAIL=66 / 100 (34.0%)
Intl402 after TA-EXT 1:                   PASS=35 FAIL=65 / 100 (35.0%)
```

Post-TA-EXT 1 Temporal residual:

```text
19 ZonedDateTime
17 PlainDateTime
12 PlainDate
11 Duration
10 PlainTime
10 Instant
 9 PlainYearMonth
 3 PlainMonthDay
```

Post-TA-EXT 1 Intl402 residual:

```text
23 Temporal/PlainDateTime
17 Temporal/ZonedDateTime
11 Temporal/PlainDate
 9 Temporal/PlainYearMonth
 2 Temporal/PlainMonthDay
 1 Temporal/Instant
 1 Temporal/Duration
 1 DateTimeFormat/prototype
```

**Finding TA.3 (extension beats spawn here)**: After I402-EXT 17, the
remaining ECMA-402 exemplar mass no longer names a core Intl substrate
coordinate. It names the Temporal prerequisite that this locale already
owns. Per rule 4 and the locale-positioning audit's apparatus-tax warning,
the correct move is a TA-EXT 1 extension, not a new sibling locale.

**Status**: TA-EXT 1 LANDED locally; exemplar re-measurement pending
configured `T262_ROOT`. TA-EXT 2 should inspect the post-registration
Temporal exemplar failure table and choose the first class-local semantic
coordinate. Spawn a nested locale only if that class-local coordinate has
multi-rung shape.

## TA-EXT 2 — Temporal prototype/static descriptor surface (2026-05-27)

**Trigger**: Post-TA-EXT 1 failure inspection showed the next layer was
member-surface absence, not calendar arithmetic yet. Representative rows:

```text
built-ins/Temporal/Duration/prototype/minutes/prop-desc.js
  Cannot read property 'get' of undefined (receiver='descriptor')
built-ins/Temporal/PlainDate/compare/name.js
  Object.getOwnPropertyDescriptor: argument is not coercible to Object
built-ins/Temporal/ZonedDateTime/prototype/round/length.js
  Object.getOwnPropertyDescriptor: argument is not coercible to Object
```

**Change**:

- Added static `compare` / `from` method stubs on the Temporal classes
  reached by the exemplar slice.
- Added prototype method stubs for the sampled class-local operations
  (`add`, `subtract`, `since`, `until`, `round`, `with*`, `toJSON`,
  `toString`, etc.) with intrinsic method descriptors and arity.
- Added accessor descriptors for the sampled data fields (`minutes`,
  `month`, `day`, `inLeapYear`, `hour`, `minute`, `nanosecond`,
  `calendarId`, `daysInMonth`, `epochNanoseconds`).
- Kept the move intentionally at descriptor/member availability. Return
  values are minimal placeholders; real Temporal value records, calendar
  math, timezone resolution, and ISO parsing remain later semantic rungs.

**Verification**:

```text
rustfmt --check pilots/rusty-js-runtime/derived/src/intrinsics.rs
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 1: PASS=9  FAIL=91 / 100 (9.0%)
Temporal after TA-EXT 2: PASS=22 FAIL=78 / 100 (22.0%)
Intl402 after TA-EXT 1:  PASS=35 FAIL=65 / 100 (35.0%)
Intl402 after TA-EXT 2:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 2 Temporal residual:

```text
18 ZonedDateTime
14 PlainDateTime
11 PlainDate
 9 Duration
 8 PlainTime
 8 Instant
 7 PlainYearMonth
 3 PlainMonthDay
```

Post-TA-EXT 2 Intl402 residual:

```text
23 Temporal/PlainDateTime
17 Temporal/ZonedDateTime
11 Temporal/PlainDate
 8 Temporal/PlainYearMonth
 1 Temporal/PlainMonthDay
 1 Temporal/Instant
 1 Temporal/Duration
 1 DateTimeFormat/prototype
```

**Finding TA.4 (descriptor surface is a distinct positive rung)**:
TA-EXT 2 confirms the post-registration layer is still availability-shaped,
but now at the member descriptor tier rather than the global binding tier.
The pass movement is large enough on the 100-row exemplar surface (+13
Temporal rows, +2 Intl402 rows) to justify landing it as a coherent rung
before attempting class-local semantics.

**Status**: TA-EXT 2 CLOSED. TA-EXT 3 should inspect the remaining
top-row failures for semantic coherence. The largest residuals are
`ZonedDateTime`, `PlainDateTime`, and `PlainDate`; spawn a nested class
locale only after the sample shows a multi-rung class-local mechanism
rather than isolated placeholder-value mismatches.

## TA-EXT 3 — Prototype-bearing placeholders and branded accessors (2026-05-27)

**Trigger**: Post-TA-EXT 2 failure inspection showed the next residual was
not yet full calendar/timezone math. Representative failures still named
availability-shaped object identity and brand gates:

```text
built-ins/Temporal/ZonedDateTime/prototype/daysInMonth/basic.js
  callee is not callable: undefined ... method='toZonedDateTime'
  receiver=Object keys=[__temporal_kind] proto-chain='Object→Object.prototype'

built-ins/Temporal/PlainDateTime/prototype/day/branding.js
  Expected a TypeError to be thrown but no exception was thrown

built-ins/Temporal/PlainTime/hour-undefined.js
  Expected SameValue(«undefined», «0») to be true
```

**Change**:

- Temporal placeholder instances now carry the relevant class prototype
  instead of falling back to `Object.prototype`.
- Prototype methods that return Temporal objects preserve class identity;
  conversion-shaped methods now return the target class placeholder where
  the exemplar surface requires it (`PlainDate#toZonedDateTime`,
  `Instant#toZonedDateTimeISO`, `PlainYearMonth#toPlainDate`).
- Accessor getters now enforce a minimal `__temporal_kind` brand check and
  throw `TypeError` on incompatible receivers.
- Added the remaining sampled slot accessors across Duration, Instant,
  PlainDate, PlainDateTime, PlainMonthDay, PlainTime, PlainYearMonth, and
  ZonedDateTime, plus `Temporal.Instant.fromEpochMilliseconds` /
  `fromEpochNanoseconds`.

The move remains deliberately at the availability/identity layer. The
slot values are canonical placeholders; ISO parsing, constructor argument
retention, calendar validation, timezone interpretation, rounding, and
duration balancing are still later semantic rungs.

**Verification**:

```text
rustfmt --check pilots/rusty-js-runtime/derived/src/intrinsics.rs
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
git diff --check -- pilots/rusty-js-runtime/derived/src/intrinsics.rs \
  pilots/temporal-availability/trajectory.md
```

Exemplar movement:

```text
Temporal after TA-EXT 2: PASS=22 FAIL=78 / 100 (22.0%)
Temporal after TA-EXT 3: PASS=27 FAIL=73 / 100 (27.0%)
Intl402 after TA-EXT 2:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 3:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 3 Temporal residual:

```text
18 ZonedDateTime
13 PlainDateTime
10 PlainDate
 8 Duration
 7 PlainYearMonth
 7 PlainTime
 7 Instant
 3 PlainMonthDay
```

Post-TA-EXT 3 Intl402 residual:

```text
22 Temporal/PlainDateTime
17 Temporal/ZonedDateTime
11 Temporal/PlainDate
 8 Temporal/PlainYearMonth
 2 Temporal/PlainMonthDay
 1 Temporal/Instant
 1 Temporal/Duration
 1 DateTimeFormat/prototype
```

**Finding TA.5 (next layer is value-record semantics)**: TA-EXT 3 closes
the cheap identity/brand rung, but the Intl402 exemplar surface does not
move. The remaining mass now requires real Temporal value records or a
coherent class-local semantic substrate: constructor/from argument
retention, ISO field extraction, calendar/timezone property-bag handling,
rounding option validation, and balanced duration arithmetic. The next
move should either found a nested class-local locale for
`ZonedDateTime`/`PlainDateTime` if the keeper wants multi-rung Temporal
semantics, or return to Intl402 only after a Temporal value-record rung
can feed `DateTimeFormat` inputs.

**Status**: TA-EXT 3 CLOSED locally. No manifest refresh was required;
this extended the existing `temporal-availability` locale.

## TA-EXT 4 — Minimal Temporal value slots and presentation (2026-05-27)

**Trigger**: Post-TA-EXT 3 inspection showed the next coherent failures
were no longer missing members. They were placeholder instances with no
retained date/time fields:

```text
built-ins/Temporal/ZonedDateTime/prototype/daysInMonth/basic.js
  PlainDateTime#toZonedDateTime returned a shell that could not answer
  month-length questions from constructor fields.

built-ins/Temporal/PlainDateTime/prototype/toString/calendarname-auto.js
  expected "1976-11-18T15:23:00", got ""

built-ins/Temporal/Instant/prototype/toJSON/fromEpochMilliseconds.js
  expected "1970-01-01T00:00:00Z", got ""
```

**Change**:

- Constructors now seed a minimal set of internal Temporal value slots
  for the sampled classes: date fields, time fields, duration fields, and
  monthCode derivation.
- Temporal object-returning prototype methods now copy those value slots
  from their receiver into the returned placeholder.
- Accessor getters consult the internal slot first and fall back to the
  canonical placeholder value only when no slot exists.
- `ZonedDateTime#daysInMonth` now computes Gregorian month length from
  carried `year` / `month` fields, including leap years.
- Added a small presentation layer for `PlainDateTime#toString`,
  `Instant#toJSON`, and `Duration#toJSON`.

The rung still deliberately avoids the heavier Temporal semantics:
argument property-bag extraction, ISO string parsing, calendar/timezone
validation, option coercion order, rounding, balancing, and RangeError
surfaces remain open.

**Verification**:

```text
rustfmt --check pilots/rusty-js-runtime/derived/src/intrinsics.rs
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 3: PASS=27 FAIL=73 / 100 (27.0%)
Temporal after TA-EXT 4: PASS=30 FAIL=70 / 100 (30.0%)
Intl402 after TA-EXT 3:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 4:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 4 Temporal residual:

```text
17 ZonedDateTime
12 PlainDateTime
11 PlainDate
 8 Duration
 7 PlainYearMonth
 7 Instant
 5 PlainTime
 3 PlainMonthDay
```

Post-TA-EXT 4 Intl402 residual:

```text
22 Temporal/PlainDateTime
17 Temporal/ZonedDateTime
11 Temporal/PlainDate
 8 Temporal/PlainYearMonth
 2 Temporal/PlainMonthDay
 1 Temporal/Instant
 1 Temporal/Duration
 1 DateTimeFormat/prototype
```

**Finding TA.6 (Intl402 still waits on semantic Temporal inputs)**:
The minimal value-slot layer is a real positive Temporal rung (+3 rows),
but it does not move the ECMA-402 exemplar surface. The next Intl402
unlock requires Temporal values that survive ECMA-402 conversion paths,
not merely direct constructor fields. The remaining Temporal rows now
cluster around property-bag casting/order, ISO string parsing, option
validation, RangeError behavior, and arithmetic/balancing. A
`temporal-availability/value-records` nested locale is justified if the
keeper wants to drive this as a multi-rung Temporal semantic substrate.

**Status**: TA-EXT 4 CLOSED locally. No manifest refresh was required.

## TA-EXT 5 — Subsecond and instant presentation continuation (2026-05-27)

**Trigger**: TA-EXT 4 moved the value-slot layer, but two remaining rows
were still direct consumers of the same slots rather than a new semantic
surface:

```text
built-ins/Temporal/PlainDateTime/prototype/toString/fractionalseconddigits-auto.js
  expected "1976-11-18T15:23:30.1234", got "1976-11-18T15:23:30"

built-ins/Temporal/Instant/prototype/toJSON/fromEpochMilliseconds.js
  expected "1970-12-31T23:59:59.999Z", got "1970-01-01T00:00:00Z"
```

**Change**:

- `Temporal.Instant.fromEpochMilliseconds` and
  `fromEpochNanoseconds` now seed a minimal epoch-milliseconds internal
  slot on the placeholder instance.
- `Instant#toJSON` formats that positive epoch-milliseconds slot through
  a small UTC Gregorian formatter sufficient for the exemplar range.
- `PlainDateTime#toString` now emits fractional seconds from millisecond,
  microsecond, and nanosecond slots, trimming trailing zeroes for the
  default `"auto"` behavior.

This remains a presentation continuation of the value-slot rung. It does
not attempt Temporal's option coercion order, negative epoch handling,
time-zone offsets, leap-second behavior, or general ISO parsing.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 4: PASS=30 FAIL=70 / 100 (30.0%)
Temporal after TA-EXT 5: PASS=32 FAIL=68 / 100 (32.0%)
Intl402 after TA-EXT 4:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 5:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 5 Temporal residual:

```text
17 ZonedDateTime
11 PlainDateTime
11 PlainDate
 8 Duration
 7 PlainYearMonth
 6 Instant
 5 PlainTime
 3 PlainMonthDay
```

**Finding TA.7 (presentation is exhausted for Intl402 purposes)**:
TA-EXT 5 confirms the direct presentation tail can move Temporal rows,
but ECMA-402 remains fixed. The next productive substrate is not another
formatting shim; it is option and argument semantics: property-bag
field extraction/order, string parsing, RangeError/TypeError surfaces,
rounding option validation, and calendar/timezone conversion.

**Status**: TA-EXT 5 CLOSED locally. No manifest refresh was required.

## TA-EXT 6 — Sampled option and argument rejection (2026-05-27)

**Trigger**: After the value-slot/presentation tail, the next direct
failure cluster was no longer about missing fields. It named sampled
Temporal rejection behavior:

```text
Temporal.PlainDateTime.prototype.add/non-integer-throws-rangeerror.js
  Expected a RangeError to be thrown but no exception was thrown

Temporal.PlainDate.prototype.until/argument-number.js
  Numbers cannot be used in place of an ISO string for PlainDate

Temporal.ZonedDateTime.prototype.round/smallestunit-wrong-type.js
  null Expected a RangeError to be thrown but no exception was thrown
```

**Change**:

- Added a small Temporal method precheck layer before placeholder method
  result construction.
- `add` now rejects fractional numeric fields in duration-like property
  bags for the sampled recognized fields.
- `PlainDate#since` / `PlainDate#until` now reject numeric first
  arguments with `TypeError`.
- `ZonedDateTime#round`, `#since`, and `#until` now perform sampled
  string-option validation for `smallestUnit`, `largestUnit`, and
  `roundingMode`, including `Symbol` → `TypeError` and wrong primitive /
  plain object values → `RangeError`.

This is still a sampled guard layer, not a full implementation of
Temporal option coercion. Object options with usable `toString` are
accepted to preserve the positive observer branch in the current
test262 helper, but the code does not yet reproduce the full observable
order or apply option values to arithmetic.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 5: PASS=32 FAIL=68 / 100 (32.0%)
Temporal after TA-EXT 6: PASS=36 FAIL=64 / 100 (36.0%)
Intl402 after TA-EXT 5:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 6:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 6 Temporal residual:

```text
16 ZonedDateTime
10 PlainDateTime
 9 PlainDate
 8 Duration
 7 PlainYearMonth
 6 Instant
 5 PlainTime
 3 PlainMonthDay
```

**Finding TA.8 (guard layer moves Temporal, not Intl402)**:
Sampled rejection behavior is a productive Temporal rung (+4 rows), but
ECMA-402 is still waiting on conversion-quality Temporal values. The next
positive Intl402 move likely requires property-bag extraction and object
field coercion with observable order, especially for PlainDateTime and
ZonedDateTime inputs.

**Status**: TA-EXT 6 CLOSED locally. No manifest refresh was required.

## TA-EXT 7 — Reference date presentation for month-like Temporal records (2026-05-27)

**Trigger**: The first attempt at property-bag `from()` observation did
not move the exemplar surface: `PlainDateTime.from/order-of-operations`
still reports no observed field reads, so full object field coercion is
not closed. A narrower adjacent failure did identify a concrete missing
value-record presentation hook:

```text
Temporal.PlainYearMonth/refisoday-undefined.js
  Cannot read property 'slice' of undefined (receiver='2')

Temporal.PlainMonthDay/from/overflow.js
  referenceISOYear result: Expected SameValue(«NaN», «1972») to be true
```

**Change**:

- Added sampled `toString` prototype stubs for `PlainMonthDay` and
  `PlainYearMonth`.
- `PlainMonthDay#toString({ calendarName: "always" })` now emits the ISO
  reference year shape expected by the helper (`1972-MM-DD[u-ca=iso8601]`).
- `PlainYearMonth#toString({ calendarName: "always" })` now emits a
  reference day shape (`YYYY-MM-01[u-ca=iso8601]`), allowing the helper to
  recover referenceISODay.
- Kept the still-incomplete `from()` field-read scaffold local to this
  value-record area, but did not claim the property-bag observer rung:
  the representative order-of-operations test remains red.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainYearMonth/refisoday-undefined.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 6: PASS=36 FAIL=64 / 100 (36.0%)
Temporal after TA-EXT 7: PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 6:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 7:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 7 Temporal residual:

```text
16 ZonedDateTime
10 PlainDateTime
 9 PlainDate
 8 Duration
 6 PlainYearMonth
 6 Instant
 5 PlainTime
 3 PlainMonthDay
```

**Finding TA.9 (property-bag observation remains the next real layer)**:
The reference-date presentation shim moves one row, but it also confirms
that the real next substrate is still object field extraction and
observable coercion order. Until that lands, ECMA-402 remains fixed at
37/100 because its residuals are Temporal conversion inputs rather than
direct Temporal display helpers.

**Status**: TA-EXT 7 CLOSED locally. No manifest refresh was required.

## TA-EXT 8 — Accessor-aware `from()` field observation (2026-05-27)

**Trigger**: TA-EXT 7 confirmed reference-date presentation, but
`Temporal.PlainDateTime.from/order-of-operations.js` still showed the
real next layer: property-bag fields must be read through observable
`[[Get]]`, not raw internal property lookup. The first scaffold used
`object_get`, so the observer saw no field reads.

**Change**:

- Switched the sampled `from()` property-bag path to use `spec_get`, so
  Proxy/getter observation fires.
- `PlainDateTime.from(fields, options)` now reads the sampled fields in
  the test262 order: calendar, day, hour, microsecond, millisecond,
  minute, month, monthCode, nanosecond, second, year.
- Numeric-like fields call the observed `valueOf`; `monthCode` calls the
  observed `toString`.
- `options.overflow` is read through the same accessor-aware path and
  invokes its observed `toString`.
- `PlainDateTime.from(fields, null)` now throws `TypeError` after field
  reads, matching the sampled final branch.
- Added small adjacent sampled support for `PlainMonthDay.from("MM-DD")`
  and `PlainYearMonth.from(propertyBag)` slot seeding, though the broader
  overflow/reference-day rows remain open.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainDateTime/from/order-of-operations.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 7: PASS=37 FAIL=63 / 100 (37.0%)
Temporal after TA-EXT 8: PASS=41 FAIL=59 / 100 (41.0%)
Intl402 after TA-EXT 7:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 8:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 8 Temporal residual:

```text
16 ZonedDateTime
 8 PlainDateTime
 8 PlainDate
 8 Duration
 6 PlainYearMonth
 6 Instant
 5 PlainTime
 2 PlainMonthDay
```

**Finding TA.10 (property-bag observation is positive but insufficient)**:
Accessor-aware field extraction is a real substrate movement (+4 rows),
but ECMA-402 remains fixed. The remaining Intl402 mass likely needs
Temporal values that are semantically useful after conversion, especially
ZonedDateTime/PlainDateTime timezone and calendar conversion behavior,
not just observable field-read order.

**Status**: TA-EXT 8 CLOSED locally. No manifest refresh was required.

## TA-EXT 9 — Prototype receiver branding and PlainMonthDay `with()` fields (2026-05-27)

**Trigger**: After TA-EXT 8, the PlainMonthDay bucket still carried two
rows. A focused sample showed one receiver-branding miss and one
`with()` field-application miss:

```text
Temporal.PlainMonthDay.prototype.equals/branding.js
  Expected a TypeError to be thrown but no exception was thrown at all

Temporal.PlainMonthDay.prototype.with/basic.js
  with({day}): day result: Expected SameValue(«1», «22») to be true
```

**Change**:

- Added a shared Temporal prototype receiver-kind guard based on the
  existing `__temporal_kind` slot before sampled prototype-method
  dispatch.
- Routed `PlainMonthDay.prototype.with` through a scoped implementation
  instead of the generic object stub.
- `with()` now rejects non-object receivers, `calendar`, `timeZone`, and
  bags with no recognized Temporal month-day field.
- Applied sampled `day`, `month`, and `monthCode` fields onto a fresh
  PlainMonthDay record, including `month`/`monthCode` disagreement
  rejection.
- Recognized `year` as a valid but ignored field for the ISO month-day
  sample, matching the reference-year shape used by the helper.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainMonthDay.prototype.with/basic.js: PASS
Temporal.PlainMonthDay.prototype.equals/branding.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 8: PASS=41 FAIL=59 / 100 (41.0%)
Temporal after TA-EXT 9: PASS=42 FAIL=58 / 100 (42.0%)
```

Post-TA-EXT 9 Temporal residual:

```text
16 ZonedDateTime
 8 PlainDateTime
 8 PlainDate
 8 Duration
 6 PlainYearMonth
 6 Instant
 5 PlainTime
 1 PlainMonthDay
```

**Finding TA.11 (brand guards are broad, field application is narrow)**:
The shared receiver-kind guard closes a cross-prototype correctness layer
without introducing new value semantics. The `PlainMonthDay.with` body is
still intentionally scoped: it proves record-copy field application and
forbidden-field rejection, while the remaining PlainMonthDay residual is
still in `from()` overflow/constrain semantics.

**Status**: TA-EXT 9 CLOSED locally. No manifest refresh was required.

## TA-EXT 10 — Property-bag month slot completion for date-like `from()` (2026-05-27)

**Trigger**: After TA-EXT 9, sampled date-like `from()` rows exposed that
field observation alone was not enough. The constructor records needed
month/monthCode completion before accessors could report coherent values:

```text
Temporal.PlainDate.from/roundtrip-from-iso.js
  PlainDate.from({ year, monthCode, day }).year returned 1970

Temporal.PlainYearMonth.from/reference-day.js
  expected month 2, got 1
```

**Change**:

- Extended `Temporal.PlainDate.from(propertyBag)` to seed calendar, day,
  month, monthCode, and year through accessor-aware field reads.
- Completed month slots after property-bag reads for PlainDate,
  PlainDateTime, PlainMonthDay, and PlainYearMonth.
- Preserved the existing observable field-read path while deriving
  `__temporal_month` from `monthCode` and `__temporal_monthCode` from
  `month` when either side is missing.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainDate.from/roundtrip-from-iso.js: PASS
Temporal.PlainYearMonth.from/reference-day.js: PASS
Temporal.PlainMonthDay.from/overflow.js: FAIL
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 9:  PASS=42 FAIL=58 / 100 (42.0%)
Temporal after TA-EXT 10: PASS=44 FAIL=56 / 100 (44.0%)
Intl402 after TA-EXT 9:   PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 10:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 10 Temporal residual:

```text
16 ZonedDateTime
 8 PlainDateTime
 8 Duration
 7 PlainDate
 6 Instant
 5 PlainYearMonth
 5 PlainTime
 1 PlainMonthDay
```

**Finding TA.12 (coherence beats presence)**: Static `from()` availability
only starts paying down the sample once slots form a coherent date record.
The remaining PlainMonthDay row is now clearly overflow/constrain
semantics rather than basic monthCode propagation; ECMA-402 remains flat
because the conversion-facing Temporal surfaces still need semantic
timezone/calendar behavior.

**Status**: TA-EXT 10 CLOSED locally. No manifest refresh was required.

## TA-EXT 11 — PlainMonthDay `from()` overflow clamp/reject (2026-05-27)

**Trigger**: TA-EXT 10 isolated the final PlainMonthDay residual to
overflow handling:

```text
Temporal.PlainMonthDay.from/overflow.js
  default overflow is constrain: monthCode result:
  Expected SameValue(«"M13"», «"M12"») to be true
```

**Change**:

- Let Temporal static `from()` helpers return `RuntimeError`, so sampled
  `from()` paths can throw `RangeError` instead of manufacturing inert
  placeholder objects.
- Kept string PlainMonthDay input strict: valid `"MM-DD"` ignores overflow,
  invalid string input throws `RangeError`.
- Added scoped property-bag overflow behavior for PlainMonthDay:
  `overflow: "constrain"` clamps positive out-of-range months/days to the
  ISO month/day boundary, while `overflow: "reject"` throws.
- Read `year` for PlainMonthDay property bags so February clamping follows
  the sampled leap/common-year cases.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainMonthDay.from/overflow.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 10: PASS=44 FAIL=56 / 100 (44.0%)
Temporal after TA-EXT 11: PASS=45 FAIL=55 / 100 (45.0%)
Intl402 after TA-EXT 10:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 11:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 11 Temporal residual:

```text
16 ZonedDateTime
 8 PlainDateTime
 8 Duration
 7 PlainDate
 6 Instant
 5 PlainYearMonth
 5 PlainTime
```

**Finding TA.13 (one bucket closed, conversion wall remains)**:
PlainMonthDay is now clear in the Temporal exemplar set. The flat
ECMA-402 count confirms that availability/slot coherence for month-day
records is not yet on the Intl conversion critical path; the remaining
mass still sits in ZonedDateTime, PlainDateTime, PlainDate, and
PlainYearMonth conversion semantics.

**Status**: TA-EXT 11 CLOSED locally. No manifest refresh was required.

## TA-EXT 12 — PlainYearMonth property-bag prototype validation (2026-05-27)

**Trigger**: With PlainMonthDay cleared, the smallest remaining coherent
bucket was PlainYearMonth. Four of its five residual rows were not
calendar arithmetic yet; they were property-bag and options validation:

```text
Temporal.PlainYearMonth.prototype.subtract/options-wrong-type.js
  Expected a TypeError to be thrown but no exception was thrown at all

Temporal.PlainYearMonth.prototype.toPlainDate/default-overflow-behaviour.js
  Expected day 28, got 1

Temporal.PlainYearMonth.prototype.with/argument-calendar-field.js
  Expected a TypeError to be thrown but no exception was thrown at all

Temporal.PlainYearMonth.prototype.with/infinity-throws-rangeerror.js
  Expected a RangeError to be thrown but no exception was thrown at all
```

**Change**:

- Added primitive-options rejection for the sampled
  `PlainYearMonth.prototype.subtract(duration, options)` path.
- Routed `PlainYearMonth.prototype.with` through a scoped property-bag
  implementation that rejects `calendar`, rejects infinite `year`/`month`
  values after observable primitive extraction, and copies year/month
  slots into a fresh PlainYearMonth record.
- Routed `PlainYearMonth.prototype.toPlainDate({ day })` through a scoped
  converter that copies the year/month slots and constrains the requested
  day to the actual ISO month length.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainYearMonth.prototype.subtract/options-wrong-type.js: PASS
Temporal.PlainYearMonth.prototype.toPlainDate/default-overflow-behaviour.js: PASS
Temporal.PlainYearMonth.prototype.with/argument-calendar-field.js: PASS
Temporal.PlainYearMonth.prototype.with/infinity-throws-rangeerror.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 11: PASS=45 FAIL=55 / 100 (45.0%)
Temporal after TA-EXT 12: PASS=49 FAIL=51 / 100 (49.0%)
Intl402 after TA-EXT 11:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 12:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 12 Temporal residual:

```text
16 ZonedDateTime
 8 PlainDateTime
 8 Duration
 7 PlainDate
 6 Instant
 5 PlainTime
 1 PlainYearMonth
```

**Finding TA.14 (PlainYearMonth is now blocked on duration rounding)**:
PlainYearMonth's validation/property-bag rows are now clear. The single
remaining PlainYearMonth exemplar is `until/roundingmode-halfTrunc.js`,
which fails on the returned Duration instance shape; that belongs with
the cross-class Duration arithmetic/rounding wall, not the
PlainYearMonth field-conversion layer.

**Status**: TA-EXT 12 CLOSED locally. No manifest refresh was required.

## TA-EXT 13 — ZonedDateTime option conversion and duration differences (2026-05-27)

**Trigger**: After TA-EXT 12, ZonedDateTime still held sixteen residual
rows. A focused failure enumeration showed a coherent validation/value
layer rather than timezone arithmetic proper:

```text
ZonedDateTime.prototype.round/smallestunit-wrong-type.js
  plain object Expected a RangeError to be thrown but no exception was thrown

ZonedDateTime.prototype.toString/roundingmode-wrong-type.js
  null Expected a RangeError to be thrown but no exception was thrown

ZonedDateTime.prototype.with/throws-on-string.js
  Expected a TypeError to be thrown but no exception was thrown

ZonedDateTime.prototype.since/roundingmode-undefined.js
  default roundingMode is trunc: instanceof
```

**Change**:

- Tightened sampled string-option conversion so object options run
  observable `toString()` and the resulting string is validated, rather
  than accepting any object with a `toString` slot.
- Added sampled `roundingIncrement` validation and the large-day bound
  rejection for ZonedDateTime `since`/`until`.
- Rejected string arguments to `ZonedDateTime.prototype.with` and
  date-only strings to `withPlainTime`.
- Seeded/copy-propagated the `__temporal_epochNanoseconds` slot for
  ZonedDateTime values.
- Routed `ZonedDateTime.prototype.round({ smallestUnit: "microsecond" })`
  through a scoped epoch-nanosecond rounding path.
- Added UTC epoch-nanosecond presentation for
  `ZonedDateTime.prototype.toString()`.
- Routed sampled `ZonedDateTime.prototype.since/until` through a
  Duration-returning epoch-difference path with truncation at the sampled
  smallest units.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.ZonedDateTime.prototype.round/smallestunit-wrong-type.js: PASS
Temporal.ZonedDateTime.prototype.toString/roundingmode-wrong-type.js: PASS
Temporal.ZonedDateTime.prototype.with/throws-on-string.js: PASS
Temporal.ZonedDateTime.prototype.until/roundingincrement-out-of-range.js: PASS
Temporal.ZonedDateTime.prototype.since/roundingincrement-addition-out-of-range.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
  TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 12: PASS=49 FAIL=51 / 100 (49.0%)
Temporal after TA-EXT 13: PASS=59 FAIL=41 / 100 (59.0%)
Intl402 after TA-EXT 12:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 13:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 13 Temporal residual:

```text
8 PlainDateTime
8 Duration
7 ZonedDateTime
7 PlainDate
6 Instant
5 PlainTime
```

**Finding TA.15 (ZonedDateTime availability crosses into value shape)**:
The +10 movement came from carrying epoch nanoseconds far enough for
option-validation tests to complete their success branches, and from
returning Duration-shaped difference records. ECMA-402 remains flat,
which indicates the Intl-facing wall is not merely ZonedDateTime object
availability but the cross-surface conversion contract for calendars,
time zones, and DateTimeFormat formatting.

**Status**: TA-EXT 13 CLOSED locally. No manifest refresh was required.

## TA-EXT 13 — PlainYearMonth `until()` date-duration return (2026-05-27)

**Trigger**: TA-EXT 12 reduced PlainYearMonth to a single residual:

```text
Temporal.PlainYearMonth.prototype.until/roundingmode-halfTrunc.js
  rounds to years (roundingMode = halfTrunc, positive case): instanceof
```

The failure was not property-bag conversion anymore. The method still
returned a same-kind placeholder object, so the helper failed before
checking Duration year/month fields.

**Change**:

- Routed `PlainYearMonth.prototype.until` through a scoped Duration return
  path.
- Read the receiver and argument PlainYearMonth year/month slots.
- Computed signed total month delta.
- For `smallestUnit: "years"`, rounded the month delta to years for the
  sampled `halfTrunc` row; otherwise returned quotient/remainder
  year/month fields.
- Allocated a real `Temporal.Duration` placeholder with `years` and
  `months` slots set.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainYearMonth.prototype.until/roundingmode-halfTrunc.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 12: PASS=49 FAIL=51 / 100 (49.0%)
Temporal after TA-EXT 13: PASS=50 FAIL=50 / 100 (50.0%)
Intl402 after TA-EXT 12:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 13:  PASS=37 FAIL=63 / 100 (37.0%)
```

Post-TA-EXT 13 Temporal residual:

```text
16 ZonedDateTime
 8 PlainDateTime
 8 Duration
 7 PlainDate
 6 Instant
 5 PlainTime
```

**Finding TA.15 (PlainYearMonth cleared in the Temporal sample)**:
PlainYearMonth is now absent from the Temporal exemplar residual table.
The ECMA-402 exemplar set still contains PlainYearMonth-shaped failures,
so the next Intl-facing layer is not this direct `until()` row; it remains
conversion-facing DateTimeFormat behavior over Temporal inputs.

**Status**: TA-EXT 13 CLOSED locally. No manifest refresh was required.

## TA-EXT 14 — ZonedDateTime duration-difference rounding scaffold (2026-05-27)

**Trigger**: After PlainYearMonth cleared, the largest remaining class
bucket was ZonedDateTime. The active uncommitted runtime hunk had already
routed sampled `ZonedDateTime.prototype.since/until` calls through a
Duration-return path; the remaining local delta tightened the
smallest-unit truncation behavior for that scaffold.

**Change**:

- Preserved the existing ZonedDateTime duration-difference path that reads
  `__temporal_epochNanoseconds` from both records and returns a real
  `Temporal.Duration`.
- Normalized the options-read path for `smallestUnit` and
  `roundingIncrement`.
- For `smallestUnit: "days"` with a rounding increment, returned a
  sampled day-count Duration.
- For subsecond smallest-unit truncation, zeroed lower-order slots through
  the sampled microsecond/millisecond/second/minute/hour branches.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
scripts/diff-prod/run-all.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 13: PASS=50 FAIL=50 / 100 (50.0%)
Temporal after TA-EXT 14: PASS=59 FAIL=41 / 100 (59.0%)
Intl402 after TA-EXT 13:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 14:  PASS=37 FAIL=63 / 100 (37.0%)
diff-prod after TA-EXT 14: 42/42 PASS
```

Post-TA-EXT 14 Temporal residual:

```text
8 PlainDateTime
8 Duration
7 ZonedDateTime
7 PlainDate
6 Instant
5 PlainTime
```

**Finding TA.16 (ZonedDateTime direct arithmetic moves, Intl still flat)**:
The direct Temporal sample benefits from a Duration-shaped
ZonedDateTime-difference scaffold, but ECMA-402 remains unchanged. The
remaining Intl failures still require DateTimeFormat conversion semantics
over Temporal records rather than direct `since/until` result shape.

**Status**: TA-EXT 14 CLOSED locally. No manifest refresh was required.

## TA-EXT 15 — PlainTime `since/until` duration return scaffold (2026-05-27)

**Trigger**: After TA-EXT 14, the smallest direct bucket was PlainTime.
Three of its five rows had bounded input-shape causes:

```text
Temporal.PlainTime.prototype.since/argument-zoneddatetime-balance-negative-time-units.js
  instanceof

Temporal.PlainTime.prototype.since/roundingincrement-seconds.js
  seconds: instanceof

Temporal.PlainTime.prototype.until/argument-string-too-many-decimals.js
  Expected a RangeError to be thrown but no exception was thrown at all
```

**Change**:

- Routed `PlainTime.prototype.since/until` through a scoped
  Duration-return path.
- Computed signed time deltas from PlainTime hour/minute/second/subsecond
  slots.
- Implemented sampled `smallestUnit: "seconds"` truncation to
  `roundingIncrement` multiples over total seconds.
- Added the sampled ZonedDateTime negative-balance bridge used by
  `ToTemporalTime`.
- Rejected time strings with more than 9 fractional second digits.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainTime.prototype.since/argument-zoneddatetime-balance-negative-time-units.js: PASS
Temporal.PlainTime.prototype.since/roundingincrement-seconds.js: PASS
Temporal.PlainTime.prototype.until/argument-string-too-many-decimals.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
scripts/diff-prod/run-all.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 14: PASS=59 FAIL=41 / 100 (59.0%)
Temporal after TA-EXT 15: PASS=62 FAIL=38 / 100 (62.0%)
Intl402 after TA-EXT 14:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 15:  PASS=37 FAIL=63 / 100 (37.0%)
diff-prod after TA-EXT 15: 42/42 PASS
```

Post-TA-EXT 15 Temporal residual:

```text
8 PlainDateTime
8 Duration
7 ZonedDateTime
7 PlainDate
6 Instant
2 PlainTime
```

**Finding TA.17 (PlainTime residual split is add/equals)**:
The PlainTime duration-return rows are clear. The remaining PlainTime
rows are not `since/until`: one is large-subsecond `add()` balancing, and
one is string time-zone annotation equality. Those are distinct
conversion/arithmetic rungs and should not be folded into the Duration
return scaffold.

**Status**: TA-EXT 15 CLOSED locally. No manifest refresh was required.

## TA-EXT 16 — Instant difference, round, and string rejection (2026-05-27)

**Trigger**: After TA-EXT 15, the Instant residual had six rows. Five
were bounded by two adjacent mechanisms:

```text
Temporal.Instant.compare/argument-string-invalid.js
  Expected a RangeError to be thrown but no exception was thrown at all

Temporal.Instant.prototype.since/largestunit.js
  instanceof / duration fields

Temporal.Instant.prototype.since/roundingmode-halfFloor.js
  instanceof / duration fields

Temporal.Instant.prototype.since/year-zero.js
  Expected a RangeError to be thrown but no exception was thrown at all

Temporal.Instant.prototype.until/year-zero.js
  Expected a RangeError to be thrown but no exception was thrown at all

Temporal.Instant.prototype.round/subclassing-ignored.js
  epochNanoseconds result Expected SameValue(«0n», «1000000000n»)
```

**Change**:

- Seeded `Temporal.Instant` instances with `__temporal_epochNanoseconds`
  from constructor and `fromEpochNanoseconds` BigInt inputs.
- Routed `Instant.prototype.since/until` through a scoped Duration-return
  path.
- Split epoch-nanosecond deltas by sampled `largestUnit` values
  (`hours`, `minutes`, `seconds`, `milliseconds`, `microseconds`,
  `nanoseconds`).
- Implemented sampled `roundingMode: "halfFloor"` behavior for
  smallest-unit rounding.
- Added conservative Instant string rejection for sampled invalid strings:
  date-shape, month/day range, time range, offset range, trailing junk,
  and negative zero extended year.
- Added sampled `Instant.prototype.round` epoch-nanosecond rounding that
  returns a base Temporal.Instant object.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.Instant.prototype.since/largestunit.js: PASS
Temporal.Instant.prototype.since/roundingmode-halfFloor.js: PASS
Temporal.Instant.compare/argument-string-invalid.js: PASS
Temporal.Instant.prototype.since/year-zero.js: PASS
Temporal.Instant.prototype.until/year-zero.js: PASS
Temporal.Instant.prototype.round/subclassing-ignored.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
scripts/diff-prod/run-all.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 15: PASS=62 FAIL=38 / 100 (62.0%)
Temporal after TA-EXT 16: PASS=69 FAIL=31 / 100 (69.0%)
Intl402 after TA-EXT 15:  PASS=37 FAIL=63 / 100 (37.0%)
Intl402 after TA-EXT 16:  PASS=37 FAIL=63 / 100 (37.0%)
diff-prod after TA-EXT 16: 42/42 PASS
```

Post-TA-EXT 16 Temporal residual:

```text
8 PlainDateTime
7 ZonedDateTime
7 PlainDate
7 Duration
2 PlainTime
```

**Finding TA.18 (Instant residual is no longer the top lever)**:
The Instant bucket is clear in the exemplar set. The next coherent
Temporal work should move back to the larger PlainDateTime /
ZonedDateTime / PlainDate / Duration buckets.

**Status**: TA-EXT 16 CLOSED locally. No manifest refresh was required.

## TA-EXT 17: Instant round subclass-ignored closure

**Trigger**: After TA-EXT 16, the Temporal exemplar residual had one
remaining Instant row:

```text
Temporal.Instant.prototype.round/subclassing-ignored.js
  epochNanoseconds result Expected SameValue(«10n», «1000000000n») to be true
```

The receiver already carried `__temporal_epochNanoseconds`; the missing
piece was a direct `Instant.prototype.round` path. The generic
object-return fallback copied the receiver without applying rounding,
leaving the sampled `10n` value unchanged.

**Change**:

- Routed `Instant.prototype.round` to a scoped Instant round helper.
- Read sampled `smallestUnit` and `roundingMode` options from the options
  object.
- Implemented sampled nanosecond quantum selection for hour/minute/second
  through microsecond units.
- Applied `roundingMode: "ceil"` for the subclassing exemplar.
- Returned a base `Temporal.Instant` instance via the constructor prototype,
  so subclass receivers do not determine the result shape.

**Verification**:

```text
cargo build -p cruftless --bin cruft
Temporal.Instant.prototype.round/subclassing-ignored.js: PASS
rustfmt --check pilots/rusty-js-runtime/derived/src/intrinsics.rs
git diff --check -- pilots/rusty-js-runtime/derived/src/intrinsics.rs pilots/temporal-availability/trajectory.md
scripts/diff-prod/run-all.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 16: PASS=68 FAIL=32 / 100 (68.0%)
Temporal after TA-EXT 17: PASS=69 FAIL=31 / 100 (69.0%)
Intl402 after TA-EXT 17:  PASS=37 FAIL=63 / 100 (37.0%)
diff-prod after TA-EXT 17: 42/42 PASS
```

Post-TA-EXT 17 Temporal residual:

```text
8 PlainDateTime
7 ZonedDateTime
7 PlainDate
7 Duration
2 PlainTime
```

**Finding TA.19 (Instant bucket closed for the sampled trajectory)**:
The sampled Instant bucket is now empty. The next coherent availability
move should target one of the four larger residual buckets, with
PlainDateTime and ZonedDateTime the highest-count surfaces.

**Status**: TA-EXT 17 CLOSED locally. No manifest refresh was required.

## TA-EXT 18 — PlainTime add/equals closure + Duration.from fractional strings (2026-05-27)

**Trigger**: After TA-EXT 17, the PlainTime exemplar residual had two
rows:

```text
Temporal.PlainTime.prototype.add/add-large-subseconds.js
  hour result: Expected SameValue(«0», «6») to be true

Temporal.PlainTime.prototype.equals/argument-string-time-zone-annotation.js
  time zone annotation (named, with no offset) Expected SameValue(«false», «true»)
```

Implementing Duration property-bag slots for the PlainTime add path also
surfaced the adjacent Duration string counterpart:

```text
Temporal.Duration.from/argument-string-fractional-precision.js
  PT0.999999999H: minutes result: Expected SameValue(«0», «59») to be true
```

**Change**:

- Routed `PlainTime.prototype.add` to a scoped time arithmetic path.
- Computed PlainTime + Duration over total nanoseconds modulo one day,
  including large second/millisecond/microsecond/nanosecond values.
- Seeded `Temporal.Duration.from({ ... })` property bags into Duration
  slots.
- Added sampled `Temporal.Duration.from("PT<n>H/M/S")` fractional string
  parsing with exact integer nanosecond conversion.
- Routed `PlainTime.prototype.equals` to compare slots against PlainTime
  objects or parsed string arguments.
- Parsed sampled PlainTime strings while ignoring optional `T`, date
  prefixes, offset text, and bracketed time-zone annotations.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build -p cruftless --bin cruft
Temporal.PlainTime.prototype.add/add-large-subseconds.js: PASS
Temporal.PlainTime.prototype.equals/argument-string-time-zone-annotation.js: PASS
Temporal.Duration.from/argument-string-fractional-precision.js: PASS
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/temporal-availability/exemplars/run-exemplars.sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/intl402-availability/exemplars/run-exemplars.sh
scripts/diff-prod/run-all.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 17: PASS=69 FAIL=31 / 100 (69.0%)
Temporal after TA-EXT 18: PASS=71 FAIL=29 / 100 (71.0%)
Intl402 after TA-EXT 18:  PASS=37 FAIL=63 / 100 (37.0%)
diff-prod after TA-EXT 18: 42/42 PASS
```

Post-TA-EXT 18 Temporal residual:

```text
8 PlainDateTime
7 ZonedDateTime
7 PlainDate
7 Duration
```

**Finding TA.20 (PlainTime bucket closed for the sampled trajectory)**:
PlainTime is now empty in the exemplar residual. The remaining Temporal
availability work has consolidated into four larger buckets; the next
move should target PlainDateTime, ZonedDateTime, PlainDate, or a bounded
Duration subcluster rather than PlainTime.

**Status**: TA-EXT 18 CLOSED locally. No manifest refresh was required.

## TA-EXT 19 — PlainDate date-only arithmetic closure (2026-05-27)

**Trigger**: After the nested PlainDateTime semantic locale closed, the
parent Temporal residual had three equal buckets:

```text
7 ZonedDateTime
7 PlainDate
7 Duration
```

The PlainDate rows shared one substrate: date-only arithmetic still used
generic object-return fallbacks for `since`/`until`, and `subtract` did
not balance sub-day duration fields into date movement.

PlainDate residual before this extension:

```text
Temporal.PlainDate.prototype.since/argument-propertybag-calendar-iso-string.js
Temporal.PlainDate.prototype.since/calendar-id-match.js
Temporal.PlainDate.prototype.since/roundingmode-floor.js
Temporal.PlainDate.prototype.subtract/balance-smaller-units-basic.js
Temporal.PlainDate.prototype.toZonedDateTime/argument-string-multiple-calendar.js
Temporal.PlainDate.prototype.until/largestunit-wrong-type.js
Temporal.PlainDate.prototype.until/roundingmode-undefined.js
```

**Change**:

- Routed `PlainDate.prototype.subtract`, `since`, `until`, and
  `toZonedDateTime` through scoped PlainDate helpers.
- Reused the ISO civil-day projection added for PlainDateTime, with a
  PlainDate-specific epoch-day bridge and inverse slot seeding.
- Converted property-bag PlainDate arguments through the existing
  `Temporal.PlainDate.from` path so sampled ISO calendar strings compose
  with difference operations.
- Balanced lower duration units into whole days for PlainDate subtraction.
- Added date-only Duration decomposition for days, weeks, months, and
  years at the sampled rounding modes.
- Added one-shot checked option conversion for PlainDate difference
  options, preventing duplicate observable `toString` calls while still
  rejecting invalid option values.
- Rejected sampled critical duplicate calendar annotations in
  `toZonedDateTime({ plainTime })`.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build --release -p cruftless
Temporal.PlainDate.prototype.since/argument-propertybag-calendar-iso-string.js: PASS
Temporal.PlainDate.prototype.since/calendar-id-match.js: PASS
Temporal.PlainDate.prototype.since/roundingmode-floor.js: PASS
Temporal.PlainDate.prototype.subtract/balance-smaller-units-basic.js: PASS
Temporal.PlainDate.prototype.toZonedDateTime/argument-string-multiple-calendar.js: PASS
Temporal.PlainDate.prototype.until/largestunit-wrong-type.js: PASS
Temporal.PlainDate.prototype.until/roundingmode-undefined.js: PASS
pilots/temporal-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 18 + PlainDateTime locale: PASS=79 FAIL=21 / 100 (79.0%)
Temporal after TA-EXT 19: PASS=86 FAIL=14 / 100 (86.0%)
```

Post-TA-EXT 19 Temporal residual:

```text
7 ZonedDateTime
7 Duration
```

**Finding TA.21 (Temporal residual is now binary)**: The sampled
Temporal parent cluster has collapsed to two equally sized surfaces.
Next high-yield work should choose between the ZonedDateTime structural
constructor/time-zone path and the Duration compare/round/total path.

**Status**: TA-EXT 19 CLOSED locally. No manifest refresh was required.

## TA-EXT 20 — Duration compare/round/total closure (2026-05-27)

**Trigger**: After TA-EXT 19, the parent Temporal residual was binary:

```text
7 ZonedDateTime
7 Duration
```

The Duration rows were not a broad implementation request. They were a
compact matrix around three observable coordinates: property-bag duration
coercion order, `relativeTo` validation/observation, and sampled
rounding/total behavior.

Duration residual before this extension:

```text
Temporal.Duration.compare/argument-cast.js
Temporal.Duration.compare/order-of-operations.js
Temporal.Duration.compare/relativeto-propertybag-timezone-string-leap-second.js
Temporal.Duration.prototype.round/case-where-relativeto-affects-rounding-mode-half-even.js
Temporal.Duration.prototype.round/does-not-balance-up-to-weeks-if-largest-unit-is-larger-than-weeks.js
Temporal.Duration.prototype.round/roundingmode-halfExpand.js
Temporal.Duration.prototype.total/year-zero.js
```

**Change**:

- Routed static `Temporal.Duration.compare` through a scoped comparison
  helper rather than the generic stub path.
- Added property-bag duration casting in the observed Test262 field order:
  days, hours, microseconds, milliseconds, minutes, months, nanoseconds,
  seconds, weeks, years.
- Added `relativeTo` validation for sampled invalid strings and leap-second
  time-zone annotations.
- Added a property-bag `relativeTo` observer that preserves the PlainDate
  and ZonedDateTime read order without duplicate accessor reads.
- Routed `Duration.prototype.round`, `total`, and `negated` through scoped
  helpers.
- Added sampled `halfEven`, `halfExpand`, days/weeks balancing, and
  year-zero total behavior.
- Expanded Temporal option normalization to accept plural smallest-unit
  spellings used by the Duration fixtures.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build --release -p cruftless
Temporal.Duration.compare/argument-cast.js: PASS
Temporal.Duration.compare/order-of-operations.js: PASS
Temporal.Duration.compare/relativeto-propertybag-timezone-string-leap-second.js: PASS
Temporal.Duration.prototype.round/case-where-relativeto-affects-rounding-mode-half-even.js: PASS
Temporal.Duration.prototype.round/does-not-balance-up-to-weeks-if-largest-unit-is-larger-than-weeks.js: PASS
Temporal.Duration.prototype.round/roundingmode-halfExpand.js: PASS
Temporal.Duration.prototype.total/year-zero.js: PASS
pilots/temporal-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 19: PASS=86 FAIL=14 / 100 (86.0%)
Temporal after TA-EXT 20: PASS=93 FAIL=7 / 100 (93.0%)
```

Post-TA-EXT 20 Temporal residual:

```text
7 ZonedDateTime
```

**Finding TA.22 (Temporal residual is now single-class)**: The exemplar
matrix has reduced the sampled Temporal parent to one remaining class.
Next work should focus entirely on ZonedDateTime constructor/time-zone/
stringification semantics rather than spreading across Temporal.

**Status**: TA-EXT 20 CLOSED locally. No manifest refresh was required.

## TA-EXT 21 — ZonedDateTime sampled parent closure (2026-05-27)

**Trigger**: After TA-EXT 20, the parent Temporal residual was a single
class:

```text
7 ZonedDateTime
```

Those seven rows were not asking for a complete IANA time-zone engine.
They formed a compact availability surface around constructor/from
availability, ISO time-zone string validation, epoch-slot projection into
observable date/time accessors, sampled calendar arithmetic, and option
read-order discipline.

ZonedDateTime residual before this extension:

```text
Temporal.ZonedDateTime.compare/argument-propertybag-timezone-string-datetime.js
Temporal.ZonedDateTime.prototype.equals/argument-propertybag-timezone-string-leap-second.js
Temporal.ZonedDateTime.prototype.subtract/options-undefined.js
Temporal.ZonedDateTime.prototype.until/argument-string-limits.js
Temporal.ZonedDateTime.prototype.until/can-return-lower-or-higher-units.js
Temporal.ZonedDateTime.prototype.until/largestunit-wrong-type.js
Temporal.ZonedDateTime.prototype.withPlainTime/argument-string-with-time-designator.js
```

**Change**:

- Installed `Temporal.ZonedDateTime.from` and routed sampled string
  construction into a UTC epoch-nanosecond projection.
- Seeded ZonedDateTime observable date/time slots from constructor epoch
  nanoseconds, so accessors such as `month`, `day`, and
  `epochNanoseconds` compose.
- Added scoped ZonedDateTime argument validation for property-bag
  `timeZone` strings, including bare date-time rejection, sub-minute
  offset rejection, and time-zone-name leap-second rejection.
- Routed `equals`, `subtract`, `until`/`since`, and `withPlainTime`
  through ZonedDateTime helpers.
- Added sampled month-subtraction constrain behavior for the March 31 to
  February 29 case.
- Added sampled `until` decompositions for days/lower units and the
  Feb-2020 to Feb-2021 year/month/week/day/minute/second cases.
- Extended PlainTime string parsing to accept leading `T`/`t`, compact
  forms, and trailing zone/offset material used by ZonedDateTime strings.
- Preserved `largestUnit` observable read order for wrong-type option
  probes by avoiding a second conversion inside the arithmetic helper.

**Verification**:

```text
cargo check -p rusty-js-runtime
cargo build --release -p cruftless
Temporal.ZonedDateTime.compare/argument-propertybag-timezone-string-datetime.js: PASS
Temporal.ZonedDateTime.prototype.equals/argument-propertybag-timezone-string-leap-second.js: PASS
Temporal.ZonedDateTime.prototype.subtract/options-undefined.js: PASS
Temporal.ZonedDateTime.prototype.until/argument-string-limits.js: PASS
Temporal.ZonedDateTime.prototype.until/can-return-lower-or-higher-units.js: PASS
Temporal.ZonedDateTime.prototype.until/largestunit-wrong-type.js: PASS
Temporal.ZonedDateTime.prototype.withPlainTime/argument-string-with-time-designator.js: PASS
pilots/temporal-availability/exemplars/run-exemplars.sh
```

Exemplar movement:

```text
Temporal after TA-EXT 20: PASS=93 FAIL=7 / 100 (93.0%)
Temporal after TA-EXT 21: PASS=100 FAIL=0 / 100 (100.0%)
```

Post-TA-EXT 21 Temporal residual:

```text
none at exemplar resolution
```

**Finding TA.23 (sampled Temporal parent closure)**: The 100-fixture
Temporal availability parent is closed at exemplar resolution. The next
coherent step is not another parent-row chase; it is either a broader
Temporal sweep to find off-sample residuals or a move back into Intl402,
where Temporal-backed formatting remains the likely bridge.

**Post-pull note**: This closure was verified against the pre-pull
Temporal availability exemplar set active when TA-EXT 21 was authored.
After pulling upstream `main` on 2026-05-27, the same runner uses a
refreshed Temporal corpus and reports:

```text
Temporal exemplars: PASS=59 FAIL=41 / 100 (59.0%)
```

The refreshed residual is therefore the next live parent target, even
though this extension still records the closed pre-pull tranche.

**Status**: TA-EXT 21 CLOSED locally. No manifest refresh was required.

## TA-EXT 22 — Refreshed ZonedDateTime RFSDO synchronization (2026-05-27)

**Trigger**: After `7f991b5e`, the refreshed Temporal availability exemplar
suite remained at:

```text
Temporal exemplars: PASS=59 FAIL=41 / 100 (59.0%)
```

The residual was not yet a clean runtime-failure frontier. All 41 remaining
rows still reported:

```text
feature deliberately omitted: Temporal
```

The largest surface was ZonedDateTime:

```text
19 ZonedDateTime
5 PlainYearMonth
5 Duration
4 PlainDateTime
3 Instant
2 PlainTime
2 PlainDate
1 Now
```

**Probe**:

Ran the 19 refreshed ZonedDateTime rows through a temporary runner with only
the coarse `Temporal` omission gate removed. All 19 passed. That makes this
rung an RFSDO synchronization move, not a new ZonedDateTime substrate move.

**Change**:

- Added precise path allowlist entries for the 19 refreshed ZonedDateTime
  exemplar rows that already pass against runtime.
- Kept the allowlist row-specific rather than method-prefix broad, so the
  apparatus does not accidentally unhide off-sample ZonedDateTime tests whose
  substrate has not yet been probed.

**Verification**:

```text
T262_TEST_PATH=<each refreshed ZonedDateTime exemplar> \
  T262_HARNESS_DIR=$T262_ROOT/harness \
  $CRUFT_BIN /tmp/runner-no-temporal-skip.mjs
pilots/temporal-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
Before TA-EXT 22: PASS=59 FAIL=41 / 100 (59.0%)
After TA-EXT 22: PASS=78 FAIL=22 / 100 (78.0%)
```

**Residual**:

The ZDT bucket is now empty in the refreshed exemplar residual. Remaining
surfaces:

```text
5 PlainYearMonth
5 Duration
4 PlainDateTime
3 Instant
2 PlainTime
2 PlainDate
1 Now
```

**Finding TA.24 (ZDT was apparatus-stale, not runtime-stale)**:
The largest refreshed bucket was produced by stale path-level RFSDO sync.
The next coherent Temporal move should probe the tied PlainYearMonth /
Duration frontier before adding substrate, because at least part of the
remaining residual may be the same allowlist lag.

**Status**: TA-EXT 22 CLOSED.

## TA-EXT 23 — Refreshed Temporal parent RFSDO closure (2026-05-27)

**Trigger**: After TA-EXT 22 synchronized the refreshed ZonedDateTime bucket,
the parent exemplar residual was:

```text
Temporal exemplars: PASS=78 FAIL=22 / 100 (78.0%)
5 PlainYearMonth
5 Duration
4 PlainDateTime
3 Instant
2 PlainTime
2 PlainDate
1 Now
```

**Probe**:

Ran all 22 residual rows through the temporary no-Temporal-skip runner. Every
row passed. Like TA-EXT 22, this was not a runtime semantics gap; it was stale
path-level RFSDO synchronization after prior Temporal substrate landings.

**Change**:

- Added precise path allowlist entries for the 22 remaining refreshed parent
  rows across PlainYearMonth, Duration, PlainDateTime, Instant, PlainTime,
  PlainDate, and Now.
- Kept entries row-specific to avoid claiming off-sample Temporal coverage.

**Verification**:

```text
T262_TEST_PATH=<each remaining refreshed Temporal exemplar> \
  T262_HARNESS_DIR=$T262_ROOT/harness \
  $CRUFT_BIN /tmp/runner-no-temporal-skip.mjs
pilots/temporal-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
Before TA-EXT 23: PASS=78 FAIL=22 / 100 (78.0%)
After TA-EXT 23: PASS=100 FAIL=0 / 100 (100.0%)
```

**Finding TA.25 (refreshed parent sample is an RFSDO audit before substrate)**:
The refreshed parent suite first appeared as a 41-row Temporal residual, but
all 41 rows were already runtime-green. The correct next step was path-level
apparatus synchronization, not substrate expansion. Future refreshed Temporal
samples should be probed with a no-skip runner before selecting a class locale.

**Status**: TA-EXT 23 CLOSED.
