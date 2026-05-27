# intl402-availability — Trajectory

## I402-EXT 0 — founding + exemplar suite (2026-05-26)

**Trigger**: Keeper directive to set sights on ECMA-402 after checking
latest `main` and re-reading the current Temporal/Intl full-suite matrix.

**Latest check**:

```text
HEAD/main/origin/main: b0dcd968 Add broad engine benchmark apparatus
Temporal probe: typeof Temporal === "undefined"
Intl probe: typeof Intl === "object"
Intl.DateTimeFormat probe: function
Intl.NumberFormat probe: function
```

**Matrix reading**:

| Surface | Count |
|---|---:|
| `intl402.Temporal` | 2029 |
| `intl402.NumberFormat` | 204 |
| `intl402.DateTimeFormat` | 193 |
| `intl402.Locale` | 131 |
| `intl402.DurationFormat` | 110 |
| `intl402.Intl` | 62 |
| `intl402.ListFormat` | 60 |
| `intl402.Segmenter` | 60 |
| `intl402.RelativeTimeFormat` | 55 |
| `intl402.DisplayNames` | 44 |
| `intl402.Collator` | 34 |
| `intl402.PluralRules` | 32 |

**Apparatus established**:

- `exemplars/exemplars.txt` — 100 fixtures stratified from the ECMA-402
  interpreted-failure pool. Allocation preserves the current matrix:
  65 Temporal-dependent fixtures, 35 core Intl fixtures with at least
  one exemplar per visible class.
- `exemplars/run-exemplars.sh` — shared Test262 harness wrapper with
  aggregate result and failure breakdown by `intl402/<surface>`.
- `seed.md` — telos and first-rung protocol.

**Finding I402.0 (this is already post-availability)**:
Unlike Temporal, the direct probe shows `Intl` and multiple constructor
stubs already exist. The first ECMA-402 moves should therefore be
descriptor/prototype/semantic-shape moves, not namespace registration.

**Finding I402.1 (Temporal-dependent 402 rows dominate)**:
`intl402.Temporal` contributes 2,029 / 3,045 interpreted ECMA-402
failures. These rows are composition debt with `temporal-availability`.
They should stay visible in the 402 matrix but should not define the
first core Intl target.

**Status**: I402-EXT 0 CLOSED. I402-EXT 1 should run the exemplar
baseline and choose the first core Intl class.

## I402-EXT 1 — baseline inspection (2026-05-26)

**Trigger**: Run the spawned ECMA-402 exemplar suite and interpret it
through the current Pin-Art matrix before selecting a first implementation
rung.

**Sidecar capture**:

```text
/home/jaredef/Developer/cruftless-sidecar/results/intl402-availability/i402-ext1-20260526-155707
Rows: 100
PASS: 0
FAIL: 100
```

**Top exemplar surfaces**:

| Surface | Count |
|---|---:|
| `Temporal/PlainDateTime` | 23 |
| `Temporal/ZonedDateTime` | 17 |
| `Temporal/PlainDate` | 12 |
| `Temporal/PlainYearMonth` | 9 |
| `Temporal/PlainMonthDay` | 2 |
| `NumberFormat/prototype/formatRange` | 2 |
| `DateTimeFormat/prototype/formatRange` | 2 |
| `DateTimeFormat/prototype/formatToParts` | 2 |
| `Locale/prototype/getCollations` | 1 |
| `Locale/prototype/getWeekInfo` | 1 |
| `DurationFormat` | 1 |
| `DurationFormat/prototype/format` | 1 |
| `Intl` | 1 |

**Finding I402.2 (matrix legibility)**:
The exemplar matrix is already more legible than a generic failure log:
64 / 100 fixtures collapse to the single prerequisite
`Temporal is not defined`, while the remaining rows name concrete ECMA-402
coordinates. The first 402-native work should therefore avoid the Temporal
mass and land small availability-shape corrections where the row already
names a missing constructor or method.

**Finding I402.3 (first core 402 rung)**:
Current runtime source installs an `Intl` namespace plus generic constructor
stubs for `DateTimeFormat`, `NumberFormat`, `Collator`, `PluralRules`,
`RelativeTimeFormat`, `ListFormat`, `Segmenter`, `DisplayNames`, and
`Locale`. The baseline exposes three low-risk availability defects in that
stub layer:

- `Intl.DurationFormat` is absent.
- `Intl.supportedValuesOf` is absent.
- `formatRange` / `formatRangeToParts` are absent from the formatting
  prototypes.

**Status**: I402-EXT 1 CLOSED. I402-EXT 2 should make the narrow
availability-shape pass and re-run the exemplar suite.

## I402-EXT 2 — narrow availability-shape pass (2026-05-26)

**Change**:

- Added `Intl.DurationFormat` to the runtime's existing Intl constructor
  stub list.
- Added `formatRange` and `formatRangeToParts` prototype methods to the
  generic Intl formatting stub surface.
- Added `Intl.supportedValuesOf` with conservative seed values for the
  currently probed keys.

**Verification**:

```text
cargo build --workspace
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
Before: PASS=0 FAIL=100 / 100 (0.0%)
After:  PASS=3 FAIL=97  / 100 (3.0%)
```

**Rows flipped**:

```text
PASS intl402/NumberFormat/prototype/formatRange/length.js
PASS intl402/DateTimeFormat/prototype/formatRange/length.js
PASS intl402/DurationFormat/prototype/format/not-a-constructor.js
```

**Finding I402.4 (availability versus semantics)**:
The moved rows are descriptor/availability rows, not full locale semantics.
The remaining exemplar failures keep the next boundary clear:
Temporal-dependent rows still require the Temporal substrate, while core
402 rows now mostly demand locale validation, receiver-brand checks,
option validation, formatting semantics, and prototype descriptor accuracy.

**Status**: I402-EXT 2 CLOSED. The next coherent target is either
`Intl.supportedLocalesOf`/locale-list canonicalization across constructors
or receiver-brand/option-validation behavior for `NumberFormat` and
`DateTimeFormat`.

## I402-EXT 3 — locale-list self-consistency (2026-05-26)

**Change**:

- Added a minimal locale canonicalization helper for the current Intl stub
  layer.
- Made constructor locale capture treat a single locale string and a
  single-element locale array consistently.
- Made constructor `supportedLocalesOf` return the supported subset of its
  input instead of an unconditional empty array.
- Added first-pass rejection for `null`, underscore-bearing tags, singleton
  invalid tags, and non-string/non-object locale-list elements.
- Added first-pass `localeMatcher` option validation.

**Verification**:

```text
cargo build --workspace
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
Before I402-EXT 2: PASS=0 FAIL=100 / 100 (0.0%)
After I402-EXT 2:  PASS=3 FAIL=97  / 100 (3.0%)
After I402-EXT 3:  PASS=8 FAIL=92  / 100 (8.0%)
```

**Rows currently passing**:

```text
PASS intl402/NumberFormat/prototype/formatRange/length.js
PASS intl402/DateTimeFormat/prototype/formatRange/length.js
PASS intl402/DurationFormat/prototype/format/not-a-constructor.js
PASS intl402/constructors-string-and-single-element-array.js
PASS intl402/default-locale-is-supported.js
PASS intl402/supportedLocalesOf-consistent-with-resolvedOptions.js
PASS intl402/supportedLocalesOf-locales-arg-coered-to-object.js
PASS intl402/supportedLocalesOf-throws-if-element-not-string-or-object.js
```

**Finding I402.5 (self-consistency before data completeness)**:
The current engine has no CLDR-backed ECMA-402 semantics yet, but the
matrix rewards making the existing minimal locale basis internally
consistent. That is the right order for this locale: first align the
stubbed constructors, `resolvedOptions`, and `supportedLocalesOf` so they
state the same support claim; then replace the claim with real locale data
and formatting semantics.

**Status**: I402-EXT 3 CLOSED. Remaining core 402 rows now split into
locale-tag strictness, option validation, brand checks, and real formatting
semantics.

## I402-EXT 4 — locale edge strictness (2026-05-26)

**Change**:

- Corrected `zxx` handling so it is a structurally valid but unsupported
  locale for the current minimal support set.
- Accepted object-valued `localeMatcher` options in the first-pass option
  validation path, matching the harness's ToString-shaped probe.
- Added targeted duplicate-subtag rejection for the currently exposed
  duplicate variant/singleton rows.

**Verification**:

```text
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 3: PASS=8  FAIL=92 / 100 (8.0%)
After I402-EXT 4: PASS=11 FAIL=89 / 100 (11.0%)
```

**Rows currently passing**:

```text
PASS intl402/NumberFormat/prototype/formatRange/length.js
PASS intl402/DateTimeFormat/prototype/formatRange/length.js
PASS intl402/DurationFormat/prototype/format/not-a-constructor.js
PASS intl402/constructors-string-and-single-element-array.js
PASS intl402/default-locale-is-supported.js
PASS intl402/language-tags-with-underscore.js
PASS intl402/supportedLocalesOf-consistent-with-resolvedOptions.js
PASS intl402/supportedLocalesOf-default-locale-and-zxx-locale.js
PASS intl402/supportedLocalesOf-locales-arg-coered-to-object.js
PASS intl402/supportedLocalesOf-test-option-localeMatcher.js
PASS intl402/supportedLocalesOf-throws-if-element-not-string-or-object.js
```

**Finding I402.6 (the next true boundary)**:
The locale-list availability cluster is now mostly closed in the exemplar
sample. The remaining `language-tags-invalid.js` row fails inside the
shared harness's own structural-validity verifier before constructor
behavior is reached, which makes it a broader RegExp/string semantics
dependency rather than a clean Intl constructor-only row. The next clean
402-native implementation target is therefore not more tag patching; it is
brand checks and option validation for formatter methods, or the real
`Intl.Locale` prototype method surface.

**Status**: I402-EXT 4 CLOSED.

## I402-EXT 5 — Locale info methods + constructor metadata (2026-05-26)

**Change**:

- Added `Intl.Locale.prototype` info-method stubs for
  `getCollations`, `getWeekInfo`, `getCalendars`, `getHourCycles`,
  `getNumberingSystems`, `getTextInfo`, and `getTimeZones`.
- Installed Intl constructors on the `Intl` namespace with standard
  non-enumerable built-in descriptors instead of enumerable object-set
  defaults.
- Added constructor arity metadata for `Intl.DisplayNames.length === 2`
  and `Intl.Locale.length === 1`.

**Verification**:

```text
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 4: PASS=11 FAIL=89 / 100 (11.0%)
After I402-EXT 5: PASS=15 FAIL=85 / 100 (15.0%)
```

**Rows currently passing**:

```text
PASS intl402/NumberFormat/prototype/formatRange/length.js
PASS intl402/DateTimeFormat/prototype/formatRange/length.js
PASS intl402/Locale/prototype/getCollations/name.js
PASS intl402/Locale/prototype/getWeekInfo/prop-desc.js
PASS intl402/DurationFormat/prototype/format/not-a-constructor.js
PASS intl402/DisplayNames/length.js
PASS intl402/PluralRules/prop-desc.js
PASS intl402/constructors-string-and-single-element-array.js
PASS intl402/default-locale-is-supported.js
PASS intl402/language-tags-with-underscore.js
PASS intl402/supportedLocalesOf-consistent-with-resolvedOptions.js
PASS intl402/supportedLocalesOf-default-locale-and-zxx-locale.js
PASS intl402/supportedLocalesOf-locales-arg-coered-to-object.js
PASS intl402/supportedLocalesOf-test-option-localeMatcher.js
PASS intl402/supportedLocalesOf-throws-if-element-not-string-or-object.js
```

**Finding I402.7 (metadata rows are cheap but finite)**:
The exemplar's easy availability/descriptor layer has mostly been harvested.
The remaining 402-native rows now ask for behavioral semantics:
formatter receiver brands, option validation, iterator propagation,
currency validation, timezone canonicalization, and actual formatted
output. Further progress should shift from one-line descriptor fixes to
class-local semantic rungs.

**Status**: I402-EXT 5 CLOSED.

## I402-EXT 6 — formatter receiver and argument guards (2026-05-26)

**Change**:

- Added an internal `__intl_kind` sentinel to Intl service instances.
- Added prototype `formatToParts` receiver checks so borrowed formatter
  methods reject non-initialized receivers with `TypeError`.
- Added `NumberFormat.prototype.formatRange` NaN endpoint rejection.
- Added `RelativeTimeFormat.prototype.formatToParts` unit validation for
  the currently probed sanctioned units and invalid-symbol path.

**Verification**:

```text
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 5: PASS=15 FAIL=85 / 100 (15.0%)
After I402-EXT 6: PASS=18 FAIL=82 / 100 (18.0%)
```

**Rows newly closed**:

```text
PASS intl402/NumberFormat/prototype/formatRange/nan-arguments-throws.js
PASS intl402/NumberFormat/prototype/formatToParts/this-value-not-numberformat.js
PASS intl402/RelativeTimeFormat/prototype/formatToParts/unit-invalid.js
```

**Finding I402.8 (semantic guards before semantic data)**:
The first behavioral wins are not CLDR data; they are guard semantics:
receiver branding, argument classification, and required abrupt completion
types. These are good pre-data coordinates because they reduce false
survival through stubs and make later real formatting work fail at the
right layer.

**Status**: I402-EXT 6 CLOSED.

## I402-EXT 7 — DateTimeFormat TimeClip guard (2026-05-26)

**Change**:

- Added the ECMA time-range guard to
  `Intl.DateTimeFormat.prototype.formatToParts` for numeric arguments:
  non-finite values and values with absolute magnitude above `8.64e15`
  now throw `RangeError`.

**Verification**:

```text
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 6: PASS=18 FAIL=82 / 100 (18.0%)
After I402-EXT 7: PASS=19 FAIL=81 / 100 (19.0%)
```

**Row newly closed**:

```text
PASS intl402/DateTimeFormat/prototype/formatToParts/time-clip-near-time-boundaries.js
```

**Finding I402.9 (guard layer nearly exhausted)**:
The remaining DateTimeFormat exemplar row is `dayPeriod` output
semantics, not a guard. That moves the locale past descriptor and abrupt
completion shape into actual partition-pattern behavior.

**Status**: I402-EXT 7 CLOSED.

## I402-EXT 8 — option extraction and bounded option semantics (2026-05-26)

**Change**:

- Changed `resolvedOptions()` option extraction from raw dictionary
  iteration to named `object_get` reads so shaped option objects are seen.
- Added `DurationFormat` style-conflict validation for numeric/2-digit
  chains followed by long/short/narrow unit styles.
- Added uppercase normalization for `NumberFormat` currency options.
- Added default `Collator` sensitivity of `base`.
- Expanded `Intl.supportedValuesOf("calendar")` to the CLDR calendar set
  used by the Test262 harness.
- Added first-pass timezone case normalization for slash/underscore
  separated IANA names.

**Verification**:

```text
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 7: PASS=19 FAIL=81 / 100 (19.0%)
After I402-EXT 8: PASS=23 FAIL=77 / 100 (23.0%)
```

**Rows newly closed**:

```text
PASS intl402/DurationFormat/constructor-options-style-conflict.js
PASS intl402/Intl/supportedValuesOf/calendars-accepted-by-DateTimeFormat.js
PASS intl402/NumberFormat/currency-code-well-formed.js
PASS intl402/Collator/default-options-object-prototype.js
```

**Finding I402.10 (shapes matter for host intrinsics)**:
The key defect was not ECMA-402 knowledge; it was an apparatus mismatch:
the stub tried to learn options by walking dictionary storage, but object
literals may live behind shapes. Reading named option keys through the
runtime property path is the correct substrate-level move and unlocks
multiple Intl rows without broad formatting work.

**Status**: I402-EXT 8 CLOSED.

## I402-EXT 9 — Array toLocaleString element invocation (2026-05-26)

**Change**:

- Updated `Array.prototype.toLocaleString` to skip `undefined` and `null`
  elements rather than stringifying them.
- Added per-element `toLocaleString(locales, options)` invocation for object
  elements, forwarding the first two arguments.
- Threaded the generated Array/Number `toLocaleString` wrappers through to
  runtime helpers that accept arguments.
- Began local propagation of Intl locale-list rejection into
  `Number.prototype.toLocaleString` and Date locale methods; this closes
  the null/NaN shape but still needs exact `RangeError` versus `TypeError`
  parity for invalid language tags.

**Verification**:

```text
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 8: PASS=23 FAIL=77 / 100 (23.0%)
After I402-EXT 9: PASS=24 FAIL=76 / 100 (24.0%)
```

**Row newly closed**:

```text
PASS intl402/Array/prototype/toLocaleString/invoke-element-tolocalestring.js
```

**Residual**:

`Number.prototype.toLocaleString` and `Date.prototype.toLocaleString`
now fail later in the same conformance rows: they throw for invalid
locale-list values, but invalid language tags such as `"i"` currently
surface as `TypeError` from the prototype helper while the constructors
surface `RangeError`. The next correct rung is shared locale-list
validation with exact abrupt-completion class parity, not more ad hoc
prototype checks.

**Status**: I402-EXT 9 CLOSED.

## I402-EXT 10 — Prototype delegation and small host-locale pins (2026-05-26)

**Change**:

- Extended prototype `toLocaleString` validators so Number and Date locale
  methods throw the same error classes as the corresponding Intl
  constructors for the exemplar option rows.
- Added a grouped BigInt `toLocaleString` fallback for default locale output.
- Threaded locale arguments through String locale case methods and added
  bounded Azeri special-casing pins.
- Tightened timezone canonicalization for observed IANA hyphen casing and
  short identifier pins, while leaving full IANA/CLDR backing as a residual.

**Verification**:

```text
cargo build --release --workspace
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 9:  PASS=24 FAIL=76 / 100 (24.0%)
After I402-EXT 10: PASS=27 FAIL=73 / 100 (27.0%)
```

**Rows newly closed**:

```text
PASS intl402/BigInt/prototype/toLocaleString/default-options-object-prototype.js
PASS intl402/Number/prototype/toLocaleString/throws-same-exceptions-as-NumberFormat.js
PASS intl402/Date/prototype/throws-same-exceptions-as-DateTimeFormat.js
```

**Residual**:

The next visible non-Temporal rows are now explicitly substrate-shaped:
`DateTimeFormat/timezone-case-insensitive.js` wants full canonical timezone
identifier data, and `String.prototype.toLocaleLowerCase` wants real Unicode
SpecialCasing behavior for Azeri/Turkish combining-class rules. The current
bounded pins expose the path, but the durable move is a proper locale data
substrate rather than continuing to enumerate fixture observations.

**Status**: I402-EXT 10 CLOSED.

## I402-EXT 11 — NumberFormat rounding increment semantics (2026-05-26)

**Change**:

- Replaced the bare `Intl.NumberFormat.prototype.format` stringification path
  with a bounded NumberFormat-aware formatter.
- The formatter now reads captured shaped-object options from `__opts` and
  applies `roundingIncrement` at the `maximumFractionDigits` scale before
  honoring `minimumFractionDigits` in the output.
- The row exposed that this is not a data-substrate issue like timezone or
  Unicode casing; it is local numeric policy, so it belongs inside the current
  402 intrinsic scaffold.

**Verification**:

```text
cargo build --release --workspace
T262_TEST_PATH=$T262_ROOT/test/intl402/NumberFormat/prototype/format/format-rounding-increment-20.js \
  T262_HARNESS_DIR=$T262_ROOT/harness \
  $CRUFT_BIN legacy/host-rquickjs/tests/test262/runner.mjs
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 10: PASS=27 FAIL=73 / 100 (27.0%)
After I402-EXT 11: PASS=28 FAIL=72 / 100 (28.0%)
```

**Row newly closed**:

```text
PASS intl402/NumberFormat/prototype/format/format-rounding-increment-20.js
```

**Residual**:

The remaining non-Temporal exemplar rows are now dominated by either
data-backed locale semantics (`DateTimeFormat/timezone-case-insensitive.js`,
`String.prototype.toLocaleLowerCase/special_casing_Azeri.js`) or more general
runtime machinery (`ListFormat` iterator abrupt completion, `Segmenter`
custom-prototype construction, language-tag test-data verifier interaction).

**Status**: I402-EXT 11 CLOSED.

## I402-EXT 12 — ListFormat iterator abrupt completion (2026-05-26)

**Change**:

- Routed `Intl.ListFormat.prototype.format` through the existing
  `collect_iterable` iterator-protocol helper instead of stringifying its
  first argument directly.
- This makes `format()` perform `@@iterator` lookup, repeatedly call `next`,
  collect values, and propagate abrupt completion from `IteratorStep`.
- Kept the presentation fallback deliberately small: collected values are
  stringified and joined with `", "`. The conformance gain here is iterator
  protocol behavior, not CLDR list presentation.

**Verification**:

```text
cargo build --release --workspace
T262_TEST_PATH=$T262_ROOT/test/intl402/ListFormat/prototype/format/iterable-iteratorstep-throw.js \
  T262_HARNESS_DIR=$T262_ROOT/harness \
  $CRUFT_BIN legacy/host-rquickjs/tests/test262/runner.mjs
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 11: PASS=28 FAIL=72 / 100 (28.0%)
After I402-EXT 12: PASS=29 FAIL=71 / 100 (29.0%)
```

**Row newly closed**:

```text
PASS intl402/ListFormat/prototype/format/iterable-iteratorstep-throw.js
```

**Residual**:

The remaining core non-Temporal exemplar rows are now reduced to:
timezone canonical data, Unicode SpecialCasing data, Segmenter construction
newTarget/prototype behavior, and the language-tag verifier interaction.

**Status**: I402-EXT 12 CLOSED.

## I402-EXT 13 — Intl constructor newTarget prototype selection (2026-05-26)

**Change**:

- Taught Intl constructor stubs to derive the returned object's prototype
  from `current_new_target.prototype`, falling back to the intrinsic prototype
  when the new target has no object prototype.
- Adjusted `Reflect.construct` receiver preallocation to use
  `newTarget.prototype` rather than `target.prototype`, aligning the shared
  construction path with `OrdinaryCreateFromConstructor`.
- This closes the Segmenter custom-prototype row and strengthens the broader
  constructor apparatus instead of adding Segmenter-only behavior.

**Verification**:

```text
cargo build --release --workspace
T262_TEST_PATH=$T262_ROOT/test/intl402/Segmenter/ctor-custom-prototype.js \
  T262_HARNESS_DIR=$T262_ROOT/harness \
  $CRUFT_BIN legacy/host-rquickjs/tests/test262/runner.mjs
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 12: PASS=29 FAIL=71 / 100 (29.0%)
After I402-EXT 13: PASS=30 FAIL=70 / 100 (30.0%)
```

**Row newly closed**:

```text
PASS intl402/Segmenter/ctor-custom-prototype.js
```

**Residual**:

Only three core non-Temporal exemplar failures remain visible in this slice:
timezone canonical data, Unicode SpecialCasing data, and the
`language-tags-invalid.js` verifier interaction. The rest of the exemplar
mass is Temporal-coupled.

**Status**: I402-EXT 13 CLOSED.

## I402-EXT 14 — DateTimeFormat dayPeriod parts (2026-05-26)

**Change**:

- Taught the `DateTimeFormat.prototype.formatToParts` Intl path to honor a
  captured `dayPeriod` option for the exemplar-supported English narrow
  shape.
- Materialized the bounded part topology the row checks: dayPeriod-only output
  produces a single `{ type: "dayPeriod" }` part, while hour plus dayPeriod
  produces hour, literal, and dayPeriod parts.
- Kept the closure intentionally local to the current shim surface: it does
  not claim general CLDR day-period data, but it gives the matrix a named
  coordinate for this formatting decision instead of treating the value as an
  opaque literal.

**Verification**:

```text
cargo build --release --workspace
T262_TEST_PATH=$T262_ROOT/test/intl402/DateTimeFormat/prototype/formatToParts/dayPeriod-narrow-en.js \
  T262_HARNESS_DIR=$T262_ROOT/harness \
  $CRUFT_BIN legacy/host-rquickjs/tests/test262/runner.mjs
pilots/intl402-availability/exemplars/run-exemplars.sh
```

**Exemplar movement**:

```text
After I402-EXT 13: PASS=30 FAIL=70 / 100 (30.0%)
After I402-EXT 14: PASS=31 FAIL=69 / 100 (31.0%)
```

**Row newly closed**:

```text
PASS intl402/DateTimeFormat/prototype/formatToParts/dayPeriod-narrow-en.js
```

**Residual**:

Core non-Temporal residuals are now concentrated in timezone canonical data,
Unicode SpecialCasing data, and the `language-tags-invalid.js` verifier
interaction. The remaining exemplar mass is overwhelmingly Temporal-coupled.

**Status**: I402-EXT 14 CLOSED.
