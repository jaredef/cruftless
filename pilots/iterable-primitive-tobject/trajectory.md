# iterable-primitive-tobject — Trajectory

## IPTO-EXT 1 — collect_iterable ToObject-wraps primitives (2026-05-25)

**Trigger**: ESNE-EXT 3 verification round surfaced the pre-existing bug: `[..."abc"]` returned `[]` despite `for-of` and `Array.from("abc")` working on the same input. Root cause: `collect_iterable` short-circuited to empty Vec for any non-`Value::Object` input, dropping iteration on primitives that the spec requires to be ToObject-wrapped first.

**Edits** (~10 LOC at `intrinsics.rs::collect_iterable`):

Replace the non-Object short-circuit. Three branches per ECMA-262 §7.3.20 + §7.1.18 ToObject:
- `Value::Object` → use directly.
- `Value::Undefined` / `Value::Null` → throw TypeError (spec mandates).
- Other primitives → `rt.to_object(other)?` to wrap. String → StringWrapper with @@iterator on String.prototype. Number/Boolean/BigInt/Symbol wrap to objects with no @@iterator and hit the existing "iterator is not an object" / "callee is not callable" downstream TypeError per spec.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `[..."abc"]` | `[]` | `["a","b","c"]` |
| `Array.from(new Set("hello"))` | `[]` | `["h","e","l","o"]` |
| `Array.from(new Set([..."xy"]))` | `[]` | `["x","y"]` |
| `sum(..."abc")` (arg spread) | `0` | `294` |
| `Array.from("xyz")` (pre-existing path) | works | works |
| `[...undefined]` | empty | TypeError |
| `[...null]` | empty | TypeError |
| `[...42]` | empty | TypeError |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding IPTO.1 (single-source fix touches 17 surfaces)**: `collect_iterable` had 17 call sites across `interp.rs` + `intrinsics.rs` (Set ops, Map/Set ctor iterables, spread, Array.from, etc.). One source-layer change — the ToObject wrap at the entry — closed the bug for all. Standing Rule 13 (revert-then-deeper-layer-closure) instantiation; the surface callers needed no edits.

**Finding IPTO.2 (silent empty as failure mode)**: pre-fix returned `Ok(Vec::new())` on non-Object inputs — silent success. `for-of` and `Array.from` have parallel iteration code paths that didn't share this bug, masking it for callers who only tested those paths. Standing recommendation: a substrate function whose normal return shape is `Vec<Value>` should not silently return empty on type-mismatched input; throw or propagate a Result with TypeError so callers fail loudly rather than process empty data.

**Finding IPTO.3 (parallel-path divergence as anti-pattern)**: cruft has three iteration entry points — `for-of` (Op::IterInit fast path), `Array.from`, and `collect_iterable`. They share the engine-level iterator protocol but diverge on input handling. for-of and Array.from ToObject-wrap; collect_iterable did not. Standing recommendation: at the input-coercion boundary, all parallel iteration entry points should converge on a shared coercion helper. A future refactor could introduce a single `GetIterator(rt, src) -> Result<ObjectRef>` per spec §7.3.20 used by all three paths.

**Status**: IPTO-EXT 1 CLOSED.
