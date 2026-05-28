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

## JPRS-EXT 4 — PROPERTYKEY SETTER LOOKUP STOPS AT OWN DATA (2026-05-28)

**Move**: `find_setter_pk` now stops when it observes any own descriptor at
the computed key, including ordinary data descriptors. The string-key
`find_setter` helper already had this precedence; the PropertyKey-aware path
kept walking into the prototype after own data descriptors and could invoke an
inherited setter during computed assignment.

**Expected flip**: `reviver-wrapper.js` should now pass because
`verifyProperty(wrapper, "", { writable: true, ... })` mutates the wrapper's
own data property instead of calling the inherited `Object.prototype[""]`
setter installed by the test.

Validation:

```text
cargo build --bin cruft -p cruftless

T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh

JPRS exemplars: PASS=9 FAIL=11 SKIP=0 NOJSON=0 / 20
```

Observed flip:

- `reviver-wrapper.js` PASS.

## JPRS-EXT 5 — PROXY ENUMERATE/DEFINE EDGES IN INTERNALIZE (2026-05-28)

**Move**: Internalize traversal now routes key collection through a
proxy-aware enumerable-key helper. When the revived value is a Proxy, the
helper invokes `ownKeys` if present before filtering string keys. Separately,
`CreateDataPropertyOrThrow` now dispatches a Proxy `defineProperty` trap before
falling back to the target.

**Expected flips**: the proxy abrupt rows for own-key enumeration and
defineProperty during reviver replacement should now propagate `Test262Error`
instead of silently bypassing traps.

Validation:

```text
cargo build --bin cruft -p cruftless

T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh

JPRS exemplars: PASS=15 FAIL=5 SKIP=0 NOJSON=0 / 20
```

Observed flips:

- `revived-proxy-revoked.js` PASS.
- `reviver-array-delete-err.js` PASS.
- `reviver-array-define-prop-err.js` PASS.
- `reviver-object-define-prop-err.js` PASS.
- `reviver-object-delete-err.js` PASS.
- `reviver-object-own-keys-err.js` PASS.

Residual split:

- array-length rows still bypass abrupt `length` get/coercion during array
  InternalizeJSONProperty traversal;
- source-argument rows still need JSON source-span capture;
- `revived-proxy.js` now narrows to array proxy visited-key behavior.

## JPRS-EXT 6 — ARRAY LENGTH GET USES SPEC GET (2026-05-28)

**Move**: `try_array_length` now reads `"length"` through `spec_get` instead
of `read_property`. This preserves existing accessor propagation and adds
Proxy `get` trap dispatch for every spec site that performs
`ToLength(? Get(O, "length"))`.

**Expected flips**: `reviver-array-length-get-err.js` and
`reviver-array-length-coerce-err.js` should now propagate the Test262Error
from the proxy `length` trap or the returned object's `valueOf`.

**Observed**: no movement. Internalize was still using the object
EnumerableOwnProperties branch for arrays, so the length read was not reached.

## JPRS-EXT 7 — INTERNALIZE ARRAY BRANCH (2026-05-28)

**Move**: `internalize_json_property` now splits arrays from ordinary objects
before key traversal. Arrays, including Proxies whose target is an array, take
the spec array branch: compute `len = ToLength(? Get(val, "length"))`, then
visit numeric keys `0..len`.

**Expected flips**: the two proxy length-abrupt rows should now exercise the
length `get` trap/coercion path instead of being enumerated as ordinary
objects. This also moves `revived-proxy.js` closer to the array-vs-object
visited-key distinction the test asserts.

Validation:

```text
cargo build --bin cruft -p cruftless

T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh

JPRS exemplars: PASS=18 FAIL=2 SKIP=0 NOJSON=0 / 20
```

Observed flips:

- `revived-proxy.js` PASS.
- `reviver-array-length-coerce-err.js` PASS.
- `reviver-array-length-get-err.js` PASS.

**Residual state**: the remaining two failures are both
`json-parse-with-source` rows. They now sit outside the object-operation
internalize closure and require parse-tree source-span metadata so the reviver
context's `{ source }` property can distinguish original JSON primitive leaves
from values introduced by forward mutation during traversal.

## JPRS-EXT 8 — REVIVER SOURCE CONTEXT SIDECAR (2026-05-28)

**Move**: `JSON.parse` now scans the already-valid JSON text into a sidecar
map from tree path to `(source slice, original primitive value)`. The
internalize reviver call now always receives a third context object. Primitive
leaves whose current value still matches the original parsed primitive receive
`{ source: <raw JSON slice> }`; object/array nodes and forward-mutated values
receive an empty context object.

This deliberately avoids rewriting the existing value parser. The source
scanner is a measurement-local bridge over the same text, preserving the
current recursive-descent materialization while adding the ECMA source-context
observable.

Validation:

```text
cargo build --bin cruft -p cruftless

T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh

JPRS exemplars: PASS=20 FAIL=0 SKIP=0 NOJSON=0 / 20
```

Observed flips:

- `reviver-call-args-after-forward-modification.js` PASS.
- `reviver-forward-modifies-object.js` PASS.

**Status**: the original JPRS 20-row exemplar set is closed. A follow-on
search can decide whether to broaden this locale around the remaining
`json-parse-with-source` test262 files or mark this partition complete and
return to the wider JSON carve-out ledger.
