# length-of-array-like-propagate — Trajectory

## LOAL-EXT 1 — propagate length-getter throws per §7.3.19 (2026-05-25)

**Trigger**: top-failure-reason audit identified 11 "Expected a Test262Error but got a TypeError" failures in `Array.prototype.{forEach,filter,map,reduce,find}` tests that probe the spec's `?`-propagation with a throwing length getter. cruft's `length_of_array_like` silently swallowed those throws via `array_length`'s `unwrap_or(0)`.

**Edits** (1-line change at `interp.rs::length_of_array_like`):

Replace `Ok(self.array_length(id))` with `self.try_array_length(id)`. `try_array_length` already implements spec-strict propagation per `read_property` + `coerce_to_number`; the comment at its install site already named Array.prototype.{every,filter,find,forEach,map,some,reduce,etc.} as the test262-probed surface — but the indirect `length_of_array_like` route through the silent `array_length` was the actual call path from the generated methods.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `Array.prototype.forEach.call(obj, undefined)` with throwing length getter | silent return / TypeError | original throw propagated |
| Same shape for map / find / filter / reduce | silent or TypeError | original throw |
| test262 "Expected Test262Error but got TypeError" cluster (11 tests) | 0/11 | **7/11** |

The 4 residuals likely have additional asymmetries — separate concerns; the LOAL fix closes the propagation channel without resolving them.

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding LOAL.1 (silent-default at the shared closure masks spec errors)**: `array_length`'s `unwrap_or(0)` was a sensible default for substrate-internal callers that want best-effort length, but `length_of_array_like` is a spec-bound method whose contract is `?`-propagation. Routing the spec entry through the silent helper inverted the substrate's semantics from spec. Standing recommendation: when both a propagating and a swallowing variant exist for the same primitive (here `try_array_length` vs `array_length`), spec-entry helpers must route to the propagating variant; the swallowing variant is for internal callers only.

**Finding LOAL.2 (the comment was right, the call site wasn't)**: `try_array_length`'s install comment correctly identified the test262-probed surface ("every/filter/find/forEach/map/some/reduce/etc."), but the actual code path in generated.rs called `length_of_array_like` not `try_array_length` directly. The comment encoded the spec intent but the substrate didn't realize it. Standing recommendation: substrate-internal helpers whose install comments name the spec surface they serve should be unit-tested or call-site-audited to confirm the wiring matches the comment.

**Status**: LOAL-EXT 1 CLOSED. Single source-layer fix closed 7/11 of the test262-probed cluster; 4 residuals are separable.

---

## LOAL-EXT 2 — spec-order length-before-callable on 4 Array.proto methods (2026-05-25)

**Trigger**: LOAL-EXT 1 residual analysis. 2 of the 4 remaining failures (Array.prototype.reduce 4-10/4-11) had a different shape: reduce checked callable-arg BEFORE reading length, so a throwing length getter with `undefined` callback hit the substrate's "callback not callable" TypeError before reaching the length read. Spec §23.1.3.25 step 2 reads length first.

Grep for the same pattern surfaced 4 sibling methods with the same bug: reduce, reduceRight, flatMap, findLast, findLastIndex — all hand-written `_via` paths in `interp.rs` (vs the generated.rs methods which were already correct because their IR encoded the spec order).

**Edits** (~16 LOC across 5 sites):

Swap callable-check + length-read order in:
- `array_proto_reduce_via`
- `array_proto_reduce_right_via`
- `array_proto_flat_map_via`
- `array_proto_find_last_via`
- `array_proto_find_last_index_via`

**Verification**:

| Probe | Before | After |
|---|---|---|
| reduce.call(obj, undefined) w/ throwing length | "callback not callable" TypeError | original throw propagated |
| Same for reduceRight/flatMap/findLast/findLastIndex | same | same |
| test262 LOAL cluster (11 tests) | 7/11 | **9/11** |

The 2 residuals (JSON.stringify replacer-array-abrupt, Promise.prototype.catch this-value-then-throws) are different methods — separate concerns.

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding LOAL.3 (hand-written via paths diverge from IR-lowered counterparts)**: cruft has both IR-lowered `array_prototype_*` (in generated.rs) and hand-written `array_proto_*_via` paths (in interp.rs). The IR-lowered paths got spec-order right because the IR encoded the step sequence; the hand-written paths reorganized for ergonomics (callable-check up front for early-exit) and silently inverted the spec order. Standing recommendation: at hand-written `_via` methods, audit the function-prologue order against the spec's numbered steps. The 5 fixed here are a sample — other `_via` methods may carry the same shape.

**Finding LOAL.4 (early-exit ergonomics vs spec-order)**: the hand-written ordering optimizes for "fail fast on bad args" which feels like good defensive programming. But spec mandates length-read first because the length getter's side effects (and throws) must happen even when subsequent args are invalid. Standing recommendation: when a method's spec has observable side effects in early steps, the substrate cannot reorder validation around them — even when the validation is cheap and the side effect is expensive.

**Status**: LOAL-EXT 2 CLOSED. Cluster at 9/11; 2 residuals (JSON, Promise) are separable.
