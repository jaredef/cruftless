# json-parse-reviver-semantics — Trajectory

## JPRS-EXT 0 — FOUNDING FROM JSON CARVE-OUT SEARCH (2026-05-28)

Search input:

```text
pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-27-161641/interpreted.jsonl
```

JSON-adjacent failures partitioned as:

```text
JSON.stringify          24
JSON.parse              20
JSON.rawJSON            10
JSON.isRawJSON           6
JSON namespace           1
Date.prototype.toJSON    9
Temporal *.toJSON       52+ mostly Temporal-owned
```

Founded this locale for the `JSON.parse` slice because it has the cleanest
pipeline form and does not collide with existing active eval/global or IHI
work.

Current source read:

- `Runtime::json_parse_via` ignores `args[1]` entirely;
- `json_parse_via` uses static `abstract_ops::to_string`, so object text
  coercion does not dispatch user `toString` / `@@toPrimitive` and currently
  falls through to parser `SyntaxError` rows;
- `intrinsics::json_parse` materializes object and array values but does not
  construct the spec root holder or run `InternalizeJSONProperty`.

**Finding JPRS.1 (JSON.parse residual is an internalize pipeline, not parser
grammar)**: the 20-row cluster is mostly not about JSON grammar parsing. The
missing substrate is the post-parse `InternalizeJSONProperty` walk plus input
coercion before parse. This is a distinct mouth/terminus from
`JSON.stringify`'s `SerializeJSONProperty` pipeline.

Focused baseline:

```text
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh

JPRS exemplars: PASS=0 FAIL=20 SKIP=0 NOJSON=0 / 20
```

Baseline split:

- text coercion rows: `text-non-string-primitive`, `text-object`,
  `text-object-abrupt`;
- root-holder / reviver call-order rows: `reviver-wrapper`,
  `reviver-call-order`, `reviver-call-args-after-forward-modification`;
- delete/define/get/ownKeys abrupt rows: array and object reviver abrupt
  propagation rows;
- prototype/proxy visibility rows: revived proxy and prototype lookup rows.

**Status**: founded and baselined. First substrate rung should start with
strict text coercion and callable-reviver gating because it is the upstream
mouth and should close or reclassify the three non-string text rows before the
larger internalize walk lands.

## JPRS-EXT 1 — RUNTIME TOSTRING MOUTH (2026-05-28)

**Move**: `Runtime::json_parse_via` now performs runtime `ToString` via
`to_string_strict` on the first argument, defaulting absent text to
`undefined`. This replaces the previous static `abstract_ops::to_string`
mouth, which could not call object `toString` / `valueOf` or propagate abrupt
completion from the text coercion path.

**Expected flips**:

- `text-non-string-primitive.js`: primitive text values parse through the
  regular JSON parser and Symbol text throws TypeError before parsing.
- `text-object.js`: object text is coerced through callable `toString`.
- `text-object-abrupt.js`: abrupt `toString` / `valueOf` completion propagates
  instead of becoming a JSON syntax failure.

**Still intentionally open**: callable reviver traversal remains unimplemented:
the root holder, post-order `InternalizeJSONProperty` walk, reviver call
arguments, deletion on `undefined`, and proxy/prototype/abrupt internal-method
edges remain the next substrate partition.

Validation:

```text
cargo build --bin cruft -p cruftless

T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh

JPRS exemplars: PASS=2 FAIL=18 SKIP=0 NOJSON=0 / 20
```

Observed flips:

- `text-non-string-primitive.js` PASS.
- `text-object.js` PASS.

**Finding JPRS.2 (text abrupt is a nested ToPrimitive/property-get gap)**:
`text-object-abrupt.js` still fails as `SyntaxError` instead of propagating
`Test262Error`. That row is no longer blocked by static JSON.parse text
stringification; it now points at the object-to-primitive interior, where a
getter on `valueOf` must throw during method lookup after a non-callable
`toString: null`.

## JPRS-EXT 2 — TOSTRING METHOD LOOKUP USES SPEC GET (2026-05-28)

**Move**: `Runtime::coerce_to_string` now reads `@@toPrimitive` through
`GetMethod` and reads ordinary `toString` / `valueOf` through `spec_get`.
This preserves the OrdinaryToPrimitive rule that non-callable `toString` /
`valueOf` values are skipped, while accessor abrupt completions during method
lookup are propagated.

**Expected flip**: `text-object-abrupt.js` should now pass: the
`get valueOf() { throw new Test262Error(); }` branch is observed during
runtime `ToString` instead of falling through to parsing `"[object Object]"`.

Validation:

```text
cargo build --bin cruft -p cruftless

T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh

JPRS exemplars: PASS=3 FAIL=17 SKIP=0 NOJSON=0 / 20
```

Observed flip:

- `text-object-abrupt.js` PASS.

**Status**: all three text-coercion exemplars now pass. The residual 17-row
cluster is the reviver/internalize pipeline proper: root holder creation,
post-order traversal, property-key enumeration, proxy/prototype-aware `Get`,
`Delete`, and `CreateDataProperty` side effects.

## JPRS-EXT 3 — MINIMAL INTERNALIZE SPINE (2026-05-28)

**Move**: `JSON.parse` now detects callable `reviver`, creates the spec root
holder with `CreateDataPropertyOrThrow(root, "", unfiltered)`, and routes
through a recursive `internalize_json_property` helper. The helper performs
own enumerable string-key traversal, recursively internalizes child values,
deletes properties whose reviver result is `undefined`, writes replacement
values with `CreateDataPropertyOrThrow`, and calls the reviver with
`this = holder` plus `(name, value)`.

**Expected flips**: this should close the root-wrapper and simple call-order
rows. Remaining failures, if any, should now concentrate around source-text
third-argument support, array length/proxy-specific internal methods, and
prototype lookup edge behavior.

Validation:

```text
cargo build --bin cruft -p cruftless

T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh

JPRS exemplars: PASS=8 FAIL=12 SKIP=0 NOJSON=0 / 20
```

Observed flips:

- `reviver-array-get-prop-from-prototype.js` PASS.
- `reviver-call-err.js` PASS.
- `reviver-call-order.js` PASS.
- `reviver-get-name-err.js` PASS.
- `reviver-object-get-prop-from-prototype.js` PASS.

Residual split:

- `reviver-wrapper.js` now reaches the wrapper but exposes a generic
  assignment-path gap: an inherited setter for `""` is still invoked by the
  test helper's write probe even though the wrapper has an own data property.
- `reviver-call-args-after-forward-modification.js` and
  `reviver-forward-modifies-object.js` now fail on missing third `source`
  reviver argument support.
- Delete/define/ownKeys/array-length/proxy rows remain internal-method edge
  work beyond the minimal spine.
