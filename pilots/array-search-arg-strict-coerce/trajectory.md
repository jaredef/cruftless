# array-search-arg-strict-coerce — Trajectory

## ASAS-EXT 1 — dispatching to_number on Array search methods (2026-05-25)

**Trigger**: top-failure-reason audit (36 Array.prototype TypeError-missing). Symbol → Number must throw per §7.1.4 / ToNumber, but static `abstract_ops::to_number` returns NaN silently. Same RPTC.7 bug pattern, applied to to_number.

**Edits** (~30 LOC at `interp.rs`):
- `array_proto_at_via` (line 4258): user-arg index switched from static `abstract_ops::to_number` to dispatching `self.coerce_to_number(?)`. Symbol throws; Objects dispatch valueOf/toString.
- `array_proto_index_of_via` (line 4519): fromIndex switched.
- `array_proto_last_index_of_via` (line 4299): fromIndex switched.
- `array_proto_includes_via` (line 4539): fromIndex argument added (was missing entirely; iteration always started at 0). Spec-order: len==0 check BEFORE ToIntegerOrInfinity(fromIndex) per §23.1.3.14 step 3 — test262 `length-zero-returns-false` probes the order via valueOf side-effect counter.

**Verification**:
- Probe: `[1,2].at(Symbol())` → TypeError ✓ (was no-throw)
- Probe: `[1,2,3].indexOf(1, Symbol())` → TypeError ✓
- Probe: `[1,2,3].includes(1, 1)` → false ✓ (fromIndex now respected)
- Probe: `[1,2,3,1].includes(1, 1)` → true ✓
- Probe: `[].includes(0, {valueOf(){throw}})` → false (len-zero short-circuit before fromIndex coerce) ✓
- Probe: `[1,2].at({valueOf(){return 1}})` → 2 ✓ (dispatch works)
- test262 Array.prototype/{at,indexOf,lastIndexOf,includes} prev-fails (47): 19 newly pass
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding ASAS.1 (RPTC.7 generalizes to to_number)**: the helper-divergence pattern RPTC.1 identified for `to_string` recurs identically for `to_number`. Static `abstract_ops::to_number` returns NaN for both Symbol and Object (with a "deferred" comment); dispatching `rt.coerce_to_number` throws on Symbol and dispatches Object → primitive. Grep candidate: `abstract_ops::to_number(.*args...)`. 57 sites total in the substrate; this rung touched 4. Standing recommendation: extend the periodic grep-sweep to to_number too.

**Finding ASAS.2 (spec-order matters when coercion has side effects)**: ToIntegerOrInfinity (or our `coerce_to_number`) on a fromIndex arg invokes the arg's valueOf — observable. test262 deliberately probes argument-evaluation order via valueOf-counting fromIndex objects. Standing recommendation: when adding spec-correct coercion to a method, verify spec's argument-evaluation order; the length-zero short-circuit pattern is canonical for `at/indexOf/includes/lastIndexOf`.

**Status**: ASAS-EXT 1 CLOSED. Locale at 19/47.
