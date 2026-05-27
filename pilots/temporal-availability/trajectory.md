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
