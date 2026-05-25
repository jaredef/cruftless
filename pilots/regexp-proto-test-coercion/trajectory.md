# regexp-proto-test-coercion — Trajectory

## RPTC-EXT 1 — length=1 + spec coerce + route through exec (2026-05-25)

**Trigger**: matrix 2026-05-25 rank 12 (RegExp.prototype.test, 13).

**Edits** (~12 LOC):
- `regexp.rs::RegExp.prototype.test`: register_method → register_intrinsic_method(..., 1, ...). Body: coerce_to_string(arg) + regexp_exec + `!= Null`.

**Verification**:
- Probe: `RegExp.prototype.test.length` = 1 ✓
- Probe: `/\d+/.test({toString(){return "abc456"}})` = true ✓
- Exemplar (13 RegExp.prototype.test no-feature-tag): PASS 0 → **11**
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding RPTC.1 (helper-divergence: static-coerce vs Runtime-coerce)**: cruft has two ToString helpers — `abstract_ops::to_string` (static; cannot dispatch JS-callable @@toPrimitive/toString/valueOf because it has no Runtime) and `rt.coerce_to_string` (dispatching; per §7.1.17). Built-in methods that take user-supplied arguments at coercion boundaries MUST use the dispatching variant. The static helper is correct only for engine-internal Values whose types are known to be primitive.

**Predicts**: any built-in that uses `abstract_ops::to_string` (or to_number, to_object) on a user-supplied argument will fail Object-coercion test262 fixtures. Grep candidate: `abstract_ops::to_string(&args.first()...)` is the bug pattern. Standing recommendation: at user-argument coercion boundaries, the dispatching `rt.coerce_to_*` is the required idiom; the static `abstract_ops::to_*` should be reserved for internal known-primitive values.

**Finding RPTC.2 (algorithm-via-shared-closure beats divergent re-implementation)**: routing test through regexp_exec rather than reimplementing match-detection inline closes the cluster's sticky/global lastIndex-bookkeeping consistency for free — Standing Rule 13 (revert-then-deeper-layer-closure) instantiation. The 2 remaining y-flag failures are inside regexp_exec, not at the test entry; one substrate fix at the deeper layer closes both.

**Status**: RPTC-EXT 1 CLOSED.

## RPTC-EXT 2 — deeper-layer: sticky-anchor enforcement in regexp_exec (2026-05-25)

**Trigger**: keeper "Go to deeper substrate layer for fix" after RPTC-EXT 1 left 2 y-flag residuals. RPTC.2 predicted both fail inside `regexp_exec`, not at .test.

**Edits** (~8 LOC) at `regexp.rs::regexp_exec`:
- Hoist sticky flag into the (is_global, is_sticky, has_compiled) destructure.
- Post-`captures_at`: if `is_sticky && mstart != start`, treat as failure (None). Spec §22.2.7.2 step 23.a — sticky anchors the match at lastIndex; the engine's scanning search must be filtered.

**Verification**:
- Probe: `/c/y` lastIndex=1, `.test('abc')` → false, lastIndex reset to 0 ✓
- Probe: `/b/y.test('ab')` → false ✓; `/a/y.test('ab')` → true ✓
- Exemplar (13 RegExp.prototype.test no-feature-tag): PASS 11 → **13** (closes 2 residuals)
- Collateral on RegExp.prototype.exec sticky/y-flag tests: +2 (3 in scope, 2 newly pass)
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42**

**Findings**

**Finding RPTC.3 (deeper-layer prediction held)**: RPTC.2 predicted the residual lived in `regexp_exec`; RPTC-EXT 2's 8-LOC fix at that single deeper-layer site closed both .test residuals AND 2 collateral .exec failures. Standing Rule 13 instantiation: when a method routes through a shared closure, residual surface-method failures often live in the closure itself, with surface-fix collateral.

**Finding RPTC.4 (engine-vs-spec scoping responsibility)**: the regex engine (rust regex crate / hand-rolled NFA) returns scanning-search results; sticky anchoring is a SPEC-level responsibility, not an engine-level one. The substrate's job at `regexp_exec` is to filter engine output against the spec's match-position invariant. Conflating these would require either a sticky-aware regex engine (large cost) or per-call substring prefixing (slow). The post-filter approach lives at the spec layer and costs ~8 LOC.

**Status**: RPTC-EXT 2 CLOSED. Locale at 13/13 in-scope + 2 collateral.
