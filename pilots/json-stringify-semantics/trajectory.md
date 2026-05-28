# JSON.stringify Semantics Trajectory

## Baseline — 2026-05-28

Local `built-ins/JSON/stringify` began at roughly `35 PASS / 24 FAIL /
7 NORESULT` over the local slice. The failures split into circular detection,
spec `[[Get]]` traversal, proxy internal-method continuation, replacer
PropertyList construction, replacer-function holder/root-wrapper semantics,
gap/space, BigInt/toJSON, wrapper value substitution, and string escaping.

## JSS-EXT 1 — circular stack detection

Added `Runtime::json_stringify_stack` and compound-value stack checks.

Yield: circular rows for arrays, objects, toJSON arrays/objects, and replacer
function circular structures moved.

## JSS-EXT 2 — spec-get traversal

Compound array/object traversal now uses `spec_get` instead of raw storage
reads.

Yield: accessor/object-abrupt/cycle rows moved.

## JSS-EXT 3 — proxy internal-method continuation

JSON array detection became proxy-aware, proxy `[[Get]]` no-trap fallback
recurses into the target, proxy `ownKeys` and `getOwnPropertyDescriptor` are
consumed for object key enumeration, and proxy `ownKeys` no-trap fallback
recurses.

Yield: proxy array/object traversal rows moved.

## JSS-EXT 4 — replacer PropertyList construction

Replacer-array detection uses JSON's proxy-aware `IsArray`; PropertyList
element reads run through spec `[[Get]]`; boxed String/Number replacer entries
unwrap then `ToString`; callable checks recurse through non-revoked proxy
targets.

Focused rows:

- `replacer-array-abrupt.js` PASS.
- `replacer-array-number-object.js` PASS.
- `replacer-array-string-object.js` PASS.
- `replacer-array-proxy.js` PASS.

Measurement: full stringify slice `51 PASS / 14 FAIL / 1 NOJSON / 66`.

Carve-out: `replacer-array-proxy-revoked-realm.js` fails before JSON because
`OProxy.revocable` is absent.

## JSS-EXT 5 — replacer function holder/root wrapper (2026-05-28)

**Move**:

- The IR-lifted `SerializeJSONProperty` now consumes `(holder, key)` and
  performs `Get(holder, key)` at step 1.
- The active replacer function is called with `holder` as `this` and
  `(key, value)` as arguments.
- `JSON.stringify` creates the spec root wrapper object and defines `""` with
  `CreateDataPropertyOrThrow`, avoiding inherited setters.
- Object-literal accessor install helpers route through `dict_mut`, so shaped
  objects migrate before accessor descriptors are inserted. This preserves
  source insertion order for mixed accessor/data object literals under shape
  enrollment.

Focused rows:

- `replacer-function-arguments.js` PASS.
- `replacer-function-object-deleted-property.js` PASS.
- `replacer-function-wrapper.js` PASS.
- `property-order.js` PASS.

Measurements:

```text
cargo build --bin cruft -p cruftless

JSS exemplars adhoc: PASS=19 FAIL=11 SKIP=0 NOJSON=1 / 31
JSON.stringify: PASS=54 FAIL=11 SKIP=0 NOJSON=1 / 66
```

**Status**: POSITIVE. Replacer-function holder/root-wrapper semantics are
closed for the local exemplar set. The shape/accessor ordering fix is a
storage-boundary correction surfaced by the deleted-property row.
