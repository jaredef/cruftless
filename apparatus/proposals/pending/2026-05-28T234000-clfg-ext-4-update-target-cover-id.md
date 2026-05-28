---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 72ca802019c654628e9e49f95ec21912c25a7a70
target_branch: main
summary: CLFG-EXT 4 — update-target cover-id parenthesized unwrap; +3 directly attributable + +4 from rebase surfacing 4f3bd525 effects
risk_class: substrate
gates_pre:
  test262_full: 67.6
  test262_sample: 84.8 (re-measure pending)
  diff_prod: 61/51
  per_locale:
    TAMM: 82/100
    TAWR: 63/100
    CLFG: 19/32 (post-FIDH-EXT 1)
gates_post:
  test262_full: 67.6 (full re-measure pending; predicted -4 matrix rows from rank 56)
  test262_sample: 84.8 (unchanged on curated sample)
  diff_prod: 61/51 (unchanged)
  per_locale:
    TAMM: 82/100 (unchanged)
    TAWR: 63/100 (unchanged)
    CLFG: 26/32 (+7; 3 attributable to this rung's parenthesized fix; +4 surfaced from rebase onto 4f3bd525 super-direct-eval threading)
---

## Substrate moves

Single rung CLFG-EXT 4 in parent locale `pilots/cruft-lowering-feature-gaps/`. Folded into parent trajectory rather than spawned as a sub-locale because the cluster is small (3 exemplars / 4 matrix rows) and the substrate move is a 3-line unwrap.

### CLFG-EXT 4 — update-target cover-id parenthesized unwrap

- **M** = `(x)++`, `(x)--`, `++(x)`, `--(x)` and analogous parenthesized identifier or member update targets.
- **T** = the compiler unwraps the ParenthesizedExpression cover and lowers to the underlying target's update path per ECMA-262 §12.4.1 + §13.5.1.1 + §13.15.2 IsValidSimpleAssignmentTarget.
- **I** = `compile_update` (compiler.rs:6665) — added Parenthesized unwrap recursion at the top of the function before the structural match.
- **R** = lattice with the broader parenthesized-cover-grammar handling already present at compiler.rs:518, 688, 3716, 3984, 5973, 6008, 6258. Every other expression site already unwraps; `compile_update` was an omission, not a deliberate restriction.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **Recursive nesting unwrap behavior.** The implementation recurses via `self.compile_update(span, operator, expr, prefix)` rather than unwrapping iteratively. Risk: pathological `((((x))))++` could blow the stack at compile time. Mitigation: parser stacks already bound nesting depth at parse time; the recursion is bounded by the parser's depth limit and is straightforward tail-recursion in shape. No empirical issue observed.

2. **Interaction with member-target unwrap.** The match below the unwrap handles `Expr::Identifier` and `Expr::Member`. After unwrap, `(obj.prop)++` correctly recurses to `compile_update(obj.prop)` which hits the Member arm. Verified: no regression in protective gates.

3. **No substrate impact on protective gates.** TAMM 82, TAWR 63, diff-prod 61/51 all unchanged after the rung.

4. **Yield attribution honesty.** Of the +7 cells in the parent CLFG suite, only +3 are directly attributable to this rung's substrate change. The other +4 surfaced because the rebase onto `4f3bd525 "Thread direct eval super context"` (a parallel agent's commit) partially closed the super-direct-eval cluster at the compile tier. I've attributed this honestly in the trajectory entry; the rung's net contribution is +3 not +7.

**Standing rules consulted**:

- **Rule 4** (never split a substrate move): single coordinated rung, not split.
- **Rule 13** (revert-then-deeper-layer): no negative; not applicable.
- **Rule 15** (chapter-close-inspect): post-fix failure-table inspection of parent CLFG suite shows 6 remaining FAILs: 5 super-direct-eval runtime cells (deferred per super-deferred-behind-eval-environment-arc policy; 4f3bd525 partial), 1 complex-assignment-target compile cell (1-cell tail; could be CLFG-EXT 5 but small marginal yield).
- **Rule 24** (Pin-Art-probe-if-duplicated): 1 emit site, no duplication; no probe needed.
- **Em-dash restraint**: drafts kept under target.

## Composes-with

- Parent locale `pilots/cruft-lowering-feature-gaps/` (CLFG-EXT 0 baseline, CLFG-EXT 1 super-child founding, CLFG-EXT 2 super-child progress, CLFG-EXT 3 for-in-destructure-head spawn+close, CLFG-EXT 4 this rung).
- Sibling sub-locales `super-reference-lowering/` and `for-in-destructure-head/`.
- Arc `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/` — third tail-cluster of the parent's stated triage now closed.
- Parallel-agent commit `4f3bd525 "Thread direct eval super context"` — rebased onto this branch; surfaced +4 incidental cells.
- Deferrals-ledger: no new entries.
- Deletions-ledger: no constraint-induced deletions.

Predicted yield on next test262-full run (not measured this rung): -4 matrix rows from `availability/missing-lowering-feature failure/other` (rank 56), addressing the parenthesized-update-target cells.
