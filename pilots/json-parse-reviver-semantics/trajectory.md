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
