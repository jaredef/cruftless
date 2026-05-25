# var-hoisting-through-try-block — Trajectory

## VHTB-EXT 0 — workstream founding (2026-05-25)

**Trigger**: REOU-EXT 1 surfaced 2 regressions at `for-in/S12.6.4_A{1,2}.js`. Diagnostic: `var key` inside `try { for(__key in undefined){ var key=__key; } }` was not pre-allocated at the module/function level. Per §15.2.10 var-scoped declarations hoist across every non-function syntactic boundary.

## VHTB-EXT 1 — implementation + close (2026-05-25)

**Edit** (~55 LOC):
- `compiler.rs` Phase A.6 extension: invoke new `collect_hoisted_var_names()` walker that recurses through Stmt::Block, If, For (incl. ForInit::Variable head), ForIn/ForOf (incl. ForBinding::Decl head), While, DoWhile, Switch (cases), Try (block+handler+finalizer), Labelled. Skips Stmt::FunctionDecl + Stmt::ClassDecl (fresh hoisting scopes). Pre-allocates locals for all collected `var` names not already pre-allocated.

**Carve-out**: `let`/`const` are block-scoped, NOT hoisted; walker only collects when `VariableKind::Var`.

**Verification**:
- cargo build GREEN
- Minimal repro: `try { for(__key in undefined){ var key=__key; } } catch(e){}; console.log(typeof key)` → `"undefined"` ✓
- test262 fixture `S12.6.4_A1.js` PASS post-fix ✓

**test262-sample** (results booked post-measurement).

**Status**: VHTB-EXT 1 CLOSED.

**test262-sample** (vs pre-VHTB baseline 5787):
- PASS: 5787 → 5792 (+5)
- FAIL: 1506 → 1501 (−5)
- PASS→FAIL regressions: **0**
- FAIL→PASS transitions: 5 (the 2 originally-regressed for-in tests + 3 cascade fixes)
- Runnable pass rate: 79.4% → 79.4% (within rounding)

**Cumulative REOU + VHTB**: +73 PASS / 0 regressions vs pre-REOU baseline (5719). The two-locale closure is the corrected R for this substrate axis — REOU alone surfaced VHTB; combined they deliver Doc 740 (P4) pipeline-connection with zero regression.

### Findings

**Finding VHTB.1**: REOU's substrate-introduction-signature regressions resolved cleanly via the sibling fix in 1 round, ~55 LOC. The corpus-surfaced bug list (2 tests) precisely identified the fix scope, and the cascade was small (+3 additional fixed) — consistent with Doc 740 §III.5's "tier-closure-magnitude depends on R-membership; near-leaf fixes deliver focused gains."
