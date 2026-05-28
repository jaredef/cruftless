---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 08897a3c4f6406b4c18db51a46d3382f48824b37
target_branch: main
summary: CLFG-EXT 5 — complex-assignment-target close (paren unwrap + NamedEvaluation in compound short-circuit); +1 cell + engagement-wide NamedEval improvement
risk_class: substrate
gates_pre:
  test262_full: 67.6
  test262_sample: 84.8 (re-measure pending)
  diff_prod: 61/51
  per_locale:
    TAMM: 82/100
    TAWR: 63/100
    CLFG: 26/32 (post-CLFG-EXT 4)
gates_post:
  test262_full: 67.6 (full re-measure pending; may show incidental gains beyond the targeted -1 row)
  test262_sample: 84.8 (unchanged on curated sample)
  diff_prod: 61/51 (unchanged)
  per_locale:
    TAMM: 82/100 (unchanged)
    TAWR: 63/100 (unchanged)
    CLFG: 27/32 (+1)
---

## Substrate moves

Single rung CLFG-EXT 5 in parent locale `pilots/cruft-lowering-feature-gaps/`. Closes the final originally-named tail-cluster (complex-assignment-target) and incidentally fixes NamedEvaluation for compound short-circuit assignments engagement-wide.

### CLFG-EXT 5 — paren unwrap + NamedEvaluation in compound short-circuit

- **M** = `(a) ??= function(){}` and analogous `(a) ||=` / `(a) &&=`; bare `a ??= function(){}` etc. where NamedEvaluation should apply per ECMA-262 §13.15.4 step 1.f.
- **T** = compiler unwraps Parenthesized cover; for bare-identifier targets threads identifier name as NamedEvaluation hint to value expression; for parenthesized targets suppresses NamedEvaluation per spec.
- **I** = `compile_logical_assign` (compiler.rs:6463): three changes — Parenthesized unwrap → new `_no_named_eval` variant; name-hint threading via `compile_expr_with_name_hint`; parallel `compile_logical_assign_no_named_eval` function.
- **R** = lattice with CLFG-EXT 4 (same paren-unwrap pattern at sibling expression handler); DAG ↑ `compile_expr_with_name_hint` (line 3642) helper already used for `let a = ...` initializers.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **NamedEvaluation regression on existing compound assignments.** The change makes anonymous function values in `a ||= function(){}` receive name "a" where previously they got "". Risk: existing tests that asserted `a.name === ""` after compound-assigning a function would now fail. Mitigation: scanned protective gates; no regression observed (TAMM/TAWR/diff-prod all unchanged). The compound-short-circuit-on-anonymous-function pattern is narrow; tests asserting empty-name after `a ||= function(){}` would have been pinning bug-behavior, not spec-correct expectations.

2. **No-NamedEval variant duplication.** Introducing `compile_logical_assign_no_named_eval` as a near-duplicate function is mild code duplication (~50 LOC). Alternative considered: a boolean `name_eval_allowed` parameter on the main function. Rejected because the param-threading would touch every internal site for a one-bit dimension. The duplicate-function approach honors the "don't add abstractions beyond what the task requires" principle from CLAUDE.md.

3. **Member-target arm shared.** Both `compile_logical_assign` and the new `_no_named_eval` variant delegate to `compile_logical_assign_member` for `Expr::Member` targets. That helper does not currently apply NamedEvaluation (member-target compound assigns also have NamedEvaluation in spec, but cruft doesn't yet). Out of scope; the member-target path's NamedEvaluation gap is a separate substrate move not surfaced by this cell.

4. **No substrate impact on protective gates.** Verified empirically: TAMM 82, TAWR 63, diff-prod 61/51 all unchanged.

5. **Direct probe of all six sub-assertions**: verified individually via `/tmp/sccaaf.js`. Both name="a" cases (3 sub-assertions) and name="" cases (3 sub-assertions) match spec.

**Standing rules consulted**:

- **Rule 4** (never split a substrate move): single coordinated rung covering both the paren-unwrap and the NamedEvaluation hint. Both were required to close the cell; splitting would have left the cell still failing.
- **Rule 13** (revert-then-deeper-layer): no negative; not applicable.
- **Rule 15** (chapter-close-inspect): parent CLFG suite now 27/32; remaining 5 FAILs are all the super-direct-eval runtime cells deferred behind the eval-environment arc.
- **Rule 24** (Pin-Art-probe-if-duplicated): 1 emit site, no duplication; no probe needed.
- **Em-dash restraint**: drafts kept under target.

## Composes-with

- Parent locale `pilots/cruft-lowering-feature-gaps/` — three originally-named tail-clusters now exhausted (super: child + 4f3bd525 + 5 deferred; for-in destructure: FIDH-EXT 1; update-target: CLFG-EXT 4; complex-assignment-target: this rung).
- Arc `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/` — parent locale ready for chapter-close declaration pending eval-environment arc disposition of the super-direct-eval runtime residual.
- CLFG-EXT 4 (parenthesized cover-grammar pattern at sibling site); this rung's paren-unwrap is the second site to apply the same fix shape today.
- Helper `compile_expr_with_name_hint` (line 3642) — pre-existing infrastructure for NamedEvaluation; this rung consumes it at a new caller site.
- Deferrals-ledger: no new entries.
- Deletions-ledger: no constraint-induced deletions.

Predicted incidental yield on next test262-full run (not measured this rung): unknown but likely positive — NamedEvaluation in compound short-circuit assignments is now functional, which may surface additional cells under `availability/missing-lowering-feature failure/other` or `value-semantics/wrong-result` that share this shape.
