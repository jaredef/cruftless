# for-of-destructuring-assignment-semantics — Resume Vector / Seed

**Locale tag**: `L.for-of-destructuring-assignment-semantics` (top-level)

**Status as of 2026-05-24**: **CLOSED at FODAS-EXT 1** (1 implementation round; standing rule 13 prospective application; Doc 740 multi-tier closure).

**Workstream**: close the two-tier substrate gap in the for-of standalone-pattern path. T_1 = `ForBinding::Pattern` routing (was auto-allocating locals; should route through assignment-target semantics per §13.7.5.5 / §13.15.2 / §9.1.1.4.4). T_2 = `emit_destructure_assign`'s NamedEvaluation step per §13.15.5.3 (was missing; symmetric to `emit_destructure`'s hint path).

**Empirically anchored** (predecessor SMDR-EXT 0/1, reverted at 6fa153a4 after T_1-alone produced −10 PASS net): closing T_1 alone passes its target (5 put-unresolvable tests) but exposes the T_2 gap as cascade-revival downstream (15 fn-name tests pass→fail). Doc 740 §IV.2 substrate-introduction signature exactly. Doc 740 (P4): cumulative reclaim materializes at FINAL-tier closure, not single-tier.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: TCC, TXC, T262C.
**Composes with**:
- [Doc 740](../../docs/corpus-ref/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) §II.2 P4 + §IV.2 — multi-tier closure framing
- [Doc 739](../../docs/corpus-ref/739-constraint-closure-as-cascade-revival-when-lifting-an-upstream-structural-constraint-auto-resolves-stalled-sibling-pilots.md) §II.4 (B1/B2/B3) — cascade-revival conditions hold for 15 fn-name tests under T_2 closure
- [standing rule 13 prospective application](../../apparatus/docs/standing-rule-13-prospective-application.md) — C1-C4 hold; thirteenth corroboration
- [T262C trajectory](../test262-categorize/trajectory.md) Finding T262C.3 — destructuring concentration
- ECMA-262 §13.7.5.5 (for-of), §13.15.2 (DestructuringAssignmentEvaluation), §13.15.5.3 (NamedEvaluation), §9.1.1.4.4 (strict ReferenceError)

## I. Telos

**Empirical answer to**: does the combined two-tier closure (route `ForBinding::Pattern` through `emit_destructure_assign` + extend `emit_destructure_assign` with §13.15.5.3 NamedEvaluation at default-init sites) close the substrate gap without regression?

## II. Apparatus + Methodology

- T_1 edit: `pilots/rusty-js-bytecode/derived/src/compiler.rs` `ForBinding::Pattern` site (remove pre-allocation; convert BindingPattern→Expr; call `emit_destructure_assign`)
- T_2 edit: same file, `emit_destructure_assign` array + object default-init branches (route through `compile_expr_with_name_hint` when target is `Expr::Identifier`)
- Re-measure test262-sample; expect no PASS→FAIL transitions.

## III. Carve-outs

- ForBinding::Decl (with var/let/const) path UNCHANGED.
- Nested pattern targets handled by `emit_destructure_assign`'s existing Array/Object branches.
- Only single-identifier `Expr::Identifier` targets receive the NamedEvaluation hint at T_2 (per §13.15.5.3 IsAnonymousFunctionDefinition guard implicit in the hint plumbing).

## IV. Closure (FODAS-EXT 1)

**Combined edit**: SMDR T_1 routing (~80 LOC) + T_2 NamedEvaluation hint (~12 LOC at two sites).

**Minimal repros** (all GREEN):
- `"use strict"; for ([x] of [[]]) {}` → ReferenceError ✓
- `for ([x = function(){}] of [[]]) console.log(x.name)` → `"x"` ✓
- `for ({a: y = function(){}} of [{}]) console.log(y.name)` → `"y"` ✓

**Gates**: cargo build GREEN; minimal repros GREEN.

**test262-sample**: 5575 → 5581 PASS (+6); 1606 → 1601 FAIL (−5); zero PASS→FAIL regressions; 5 FAIL→PASS transitions. Doc 740 (P4) pipeline-connection moment with no regressions; the predecessor SMDR-alone −10 outcome was substrate-introduction signature per §IV.2.

**Status**: CLOSED.
