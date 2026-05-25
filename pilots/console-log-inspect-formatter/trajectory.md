# console-log-inspect-formatter — Trajectory

## CLIF-EXT 1 — basic util.inspect for console.log/error/warn (2026-05-25)

**Trigger**: SLV.2 spot-check misattributed `console.log("union(Set):", Array.from(setOp.result))` printing `[object Object]` to a Set wrapper-type gap. Re-probe under a corrected lens (storing `Array.from(u)` in a variable + `console.log(JSON.stringify(u))` versus inline-arg console.log) showed Array.from returns the right Array but console.log's value-formatter collapses every non-string second-arg to `[object Object]`. The substrate gap is at the console formatter, not at Set ops.

**Edits** (~170 LOC at `intrinsics.rs`):

- `install_console`: log/error/warn now route each arg through `console_format` which dispatches to `inspect_value` for non-string args; top-level strings remain unquoted per Node semantics.
- `console_format(rt, args)`: space-joined formatter.
- `inspect_value(rt, v) -> String`: top-level entry; allocates visited-set for cycle break.
- `inspect_inner(rt, v, depth, visited, in_container)`: recursive dispatch on Value variants. Primitives formatted naturally; strings quoted only when nested inside a container.
- `format_number`: handles NaN, Infinity, -Infinity, -0, finite numbers via existing number_to_string.
- `inspect_object`: dispatches on `InternalKind` (Function, Closure, BoundFunction, RegExp, Error variant, Array, else plain).
- `inspect_array`: reads length via `rt.object_get(id, "length")` (which handles the Array branch via &self); recurses up to INSPECT_MAX_DEPTH=2 then prints `[Array]`.
- `inspect_plain_object`: detects Set/Map/WeakSet/WeakMap by `__set_data`/`__map_data` sentinels + `__is_weakmap`/`__is_weakset` flags; detects Error subclasses by walking the proto chain looking for the per-class `name` set by EIPD-EXT 1. Filters `__`/`@@` engine sentinels from plain-object key enumeration.
- `inspect_set_like` / `inspect_map_like`: render as `Set(N) { … }` / `Map(N) { k => v, … }`.
- `detect_error_class`: proto-chain walk up to 5 hops, matches name against the canonical error-class set.
- Cycle break: visited HashSet<ObjectRef.0 (u32)>, returns `[Circular]` when re-entering.

**Verification probes**:

| Input | Output |
|---|---|
| `console.log([1, 2, 3])` | `[ 1, 2, 3 ]` |
| `console.log({a: 1, b: [2,3]})` | `{ a: 1, b: [ 2, 3 ] }` |
| `console.log([{x:1},{x:2}])` | `[ { x: 1 }, { x: 2 } ]` |
| `console.log("hello", [1,2,3])` | `hello [ 1, 2, 3 ]` |
| `console.log(null, undefined, true, 0, -0, NaN, 1.5)` | `null undefined true 0 -0 NaN 1.5` |
| `console.log(/abc/gi)` | `/abc/gi` |
| `console.log(new Error("oops"))` | `Error: oops` |
| `console.log(function foo(){})` | `[Function: foo]` |
| `console.log(()=>1)` | `[Function (anonymous)]` |
| `var o={x:1}; o.self=o; console.log(o)` | `{ x: 1, self: [Circular] }` |
| `console.log({a:{b:{c:{d:1}}}})` (depth cap 2) | `{ a: { b: [Object] } }` |
| `console.log(new Set([1,2]))` | `Set(2) { 1, 2 }` |
| `console.log(new Map([['a',1]]))` | `Map(1) { a => 1 }` |
| `console.log("top-level string")` | `top-level string` (unquoted) |
| `console.log(["nested", "string"])` | `[ 'nested', 'string' ]` (quoted) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL** (no fixture output shape regressions)
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding CLIF.1 (probe-format choice masks bugs)**: SLV.2's spot-check used `console.log("label:", arr)` which silently routes through the broken formatter; a probe of the form `var x = arr; console.log(x)` would have shown the same `[object Object]` and surfaced the formatter as the gap. Standing recommendation: when a probe shows an unexpected value-render, factor out the value into a separate variable + a JSON.stringify or for-of inspection before attributing the bug to a layer.

**Finding CLIF.2 (cruft uses sentinels + proto for type discrimination where node uses InternalKind)**: real Node v8 has distinct internal types for Set, Map, Error, etc.; cruft represents them as `Ordinary` objects with engine-internal sentinels (`__set_data`, `__map_data`, `__is_weakmap`) and per-class prototype properties (`name = "TypeError"`). Substrate code that wants to format an object Node-equivalently must read these conventions, not match on InternalKind. CLIF-EXT 1's `inspect_plain_object` is the canonical example: sentinel-presence + proto-walk before falling through to ordinary-object printing.

**Finding CLIF.3 (engine sentinels still leak under enumeration)**: post EIPD-EXT 1 + GBNE-EXT 1, several `__`-prefixed engine slots remain enumerable on instances (Map/Set/Date/etc.). CLIF.1 filters them at the formatter level; the underlying enumeration leak is a separate hygiene candidate. Standing recommendation: a sweep pass that re-installs every engine sentinel via `dict_mut().insert` with `{w:t,e:f,c:f}` per CLAUDE.md's source-identifier coordinate convention would close both this masking-at-formatter and the broader `Object.keys` leak.

**Status**: CLIF-EXT 1 CLOSED.
