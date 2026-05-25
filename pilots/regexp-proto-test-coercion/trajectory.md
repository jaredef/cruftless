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
