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
