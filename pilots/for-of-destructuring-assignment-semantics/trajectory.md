# for-of-destructuring-assignment-semantics — Trajectory

## FODAS-EXT 0 — workstream founding (2026-05-24)

**Trigger**: predecessor SMDR-EXT 0/1 (commit 6b9e813f, reverted at 6fa153a4) closed T_1 alone and produced −10 PASS net at test262. Per-test diagnosis: +5 SMDR target (put-unresolvable cluster), −15 fn-name regressions (init-fn-name across array-elem, obj-id, obj-prop-elem × {arrow, class, cover, fn, gen}).

**Structural reframing per Doc 740**: this is a two-tier R = {T_1, T_2} multi-tier instance, not a single-tier locale. T_1-alone is the substrate-introduction prefix (Doc 740 §IV.2 signature: correctness-preserving for target; exposes downstream tier; deeper-layer closure nameable). T_2 = `emit_destructure_assign`'s missing §13.15.5.3 NamedEvaluation. The 15 fn-name regressions are cascade-revival downstream of T_2 closure per Doc 739 §II.4 (B1/B2/B3 all hold).

**Rule 11 coverage-axis miss in predecessor**: SMDR seed enumerated §13.7.5.5 + §13.15.2 + §9.1.1.4.4 — missed §13.15.5.3. Engagement-tier extension: spec-section enumeration as a Rule 11 axis for ECMAScript-parity sub-locales.

## FODAS-EXT 1 — combined closure (2026-05-24)

**Edits**:
1. T_1: `pilots/rusty-js-bytecode/derived/src/compiler.rs` `ForBinding::Pattern(other)` site — remove pre-allocation, convert BindingPattern→Expr, dispatch to `emit_destructure_assign` (re-applied from reverted SMDR).
2. T_2: same file, `emit_destructure_assign` array-default site (~line 3870) + object-default site (~line 3942) — route default through `compile_expr_with_name_hint` when target is `Expr::Identifier`.

**Verification**:
- cargo build --release GREEN
- Minimal repros:
  - `"use strict"; for ([x] of [[]]) {}` → ReferenceError ✓
  - `for ([x = function(){}] of [[]]) console.log(x.name)` → `"x"` ✓
  - `for ({a: y = function(){}} of [{}]) console.log(y.name)` → `"y"` ✓

**test262-sample re-measure** (vs post-revert baseline 5575):
- PASS: 5575 → 5581 (+6)
- FAIL: 1606 → 1601 (−5)
- PASS→FAIL transitions: 0
- FAIL→PASS transitions: 5 (the SMDR target put-unresolvable cluster)
- +1 incidental from total-emitted shift (7565 → 7566)

**Doc 740 (P4) confirmation**: cumulative reclaim at final-tier closure with zero regressions. The +20 projection from the prior report was a miscount — the 15 fn-name tests were PASSING pre-SMDR (the old buggy auto-allocate path routed through `emit_destructure` which has the NamedEvaluation hint), broken by SMDR-alone, restored to passing by T_2 closure. So net vs pre-SMDR baseline = SMDR target gain only. The −10 SMDR-alone outcome is now retroactively classified as substrate-introduction signature per Doc 740 §IV.2.

**Standing rule 13 corroborations**: 13 prospective + 1 retrospective (this one — SMDR-alone retrospectively + combined prospectively).

**Status**: CLOSED at FODAS-EXT 1 (1 implementation round; Pred-fodas.* all held).

### Findings

**Finding FODAS.1**: Doc 740 (P4) confirmed at the bytecode-resolver tier. Two-tier closure with zero regressions; partial-closure at T_1-alone produced empirically-observable −10 net specifically because the cascade-revival downstream (fn-name tests) had been incidentally satisfied by the upstream T_1 bug — closing T_1 exposed the T_2 gap that was previously masked. Doc 740 §IV.2 substrate-introduction signature held.

**Finding FODAS.2**: predicted +20 PASS was a cascade-direction miscount. The 15 fn-name tests do not represent "new gain" from T_2 closure — they represent prevention of the SMDR-alone regression. The actual signal at the bench is +6 PASS (the SMDR target + 1 incidental); the cascade-revival math is in the per-test transition diff (0 PASS→FAIL, 5 FAIL→PASS, vs +5/−15 split for T_1-alone). Lesson: when projecting cascade-revival, distinguish "tests this fix unblocks at the bench" from "tests this fix prevents from breaking under the prerequisite fix."
