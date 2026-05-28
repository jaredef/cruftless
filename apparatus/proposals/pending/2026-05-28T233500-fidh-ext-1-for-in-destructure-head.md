---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 572fa6828655075a73314ee734d01db04f2cce0b
target_branch: main
summary: FIDH-EXT 1 — for-in destructure head bytecode lowering; +6 cells in CLFG parent exemplar suite
risk_class: substrate
gates_pre:
  test262_full: 67.6
  test262_sample: 84.8 (re-measure pending)
  diff_prod: 61/51
  per_locale:
    TAMM: 82/100
    TAWR: 63/100
    CLFG: 13/22 (super child only)
gates_post:
  test262_full: 67.6 (next full run will reflect +12 matrix-row drop; not re-measured this rung)
  test262_sample: 84.8 (unchanged — for-in destructure exemplars are not on the curated sample list per current sample-paths.txt)
  diff_prod: 61/51 (unchanged)
  per_locale:
    TAMM: 82/100 (unchanged)
    TAWR: 63/100 (unchanged)
    CLFG: 19/32 (+6 from for-in destructure cluster; locale-suite expansion to 32 cells reflects parent's full exemplar set)
---

## Substrate moves

Single rung in the new sub-locale `pilots/cruft-lowering-feature-gaps/for-in-destructure-head/` (FIDH-EXT 0 founding + FIDH-EXT 1 substrate). Arc enrollment: `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/`.

### FIDH-EXT 1 — for-in destructure head bytecode lowering

- **M** = `for ([Decl|Pattern] in iterable) ...` where the head is a non-Identifier BindingPattern (Array or Object).
- **T** = bytecode lowers to the standard for-in scan with a temp source slot for the per-iter key string, then destructures the key into bound names (Decl) or assignment targets (Pattern) per ECMA-262 §13.7.5 + §13.15.
- **I** = `compiler.rs` for-in handler: tuple at line 2888 extended to 4-tuple with `destr_pat: Option<BindingPattern>`; `ForBinding::Decl` + `ForBinding::Pattern` non-identifier arms replaced compile errors with pre-allocation + temp source slot patterns mirroring for-of (compiler.rs:2140–2154); per-iter `ResetLocalCell` extended for pre-allocated bound-name slots; `emit_destructure` / `emit_destructure_assign` injected between `Op::InitLocal` key write and `compile_stmt(body)`.
- **R** = lattice with for-of destructure path (shared helpers; shared TDZ + per-iter discipline). DAG ↑ AST `ForBinding::{Decl,Pattern}` Array/Object variants. Alphabet-exchange ↑ destructuring-binding emission the parser already produces.

## Risk assessment (helmsman self-evaluation)

**Failure modes considered**:

1. **For-of destructure regression.** The substrate move shares `emit_destructure` and `emit_destructure_assign` with the for-of handler. Risk: a for-of destructure test regresses if the change inadvertently touches the shared helpers' contract. Mitigation: my edit only touches the for-in handler (the bind_slot tuple match arms + body-emission injection). The shared helpers are unmodified. Verified: TAMM unchanged 82/100 (TAMM exercises for-of destructure across TypedArray fixtures); diff-prod unchanged 61/51.

2. **TDZ for pre-allocated bound names.** The existing IR-EXT 24 TDZ loop at lines 2936–2947 walks `scope_snapshot..self.locals.len()` and emits `PushTDZ + StoreLocal` for every let/const local whose name doesn't start with `<`. The new pre-allocated bound names (from `pat.collect_names()`) are added to the locals in the Decl arm; the IR-EXT 24 loop already covers them. No additional TDZ emission needed.

3. **`ResetLocalCell` per-iter.** For let/const destructure heads, each iteration of the loop must produce a fresh binding cell per spec §13.7.5.13. The for-in handler's existing `if per_iter_fresh { ResetLocalCell(bind_slot) }` covers the temp source slot; my edit extends this to also reset each pre-allocated bound-name slot via `pat.collect_names()`. Mirrors for-of compiler.rs:2332–2339.

4. **No substrate impact on protective gates.** Verified empirically: TAMM 82, TAWR 63, diff-prod 61/51 all unchanged. The change is scoped to bytecode lowering of a syntactic surface (for-in with destructure head) that was previously rejected at compile time.

**Standing rules consulted**:

- **Rule 11** (5-axis pre-spawn coverage): not applicable; this is a sub-locale spawn under an existing parent, not a new top-level locale spawn.
- **Rule 23** (baseline-inspect at founding): satisfied; all 6 target exemplars confirmed pre-rung with the diagnostic.
- **Rule 24** (Pin-Art-probe-if-duplicated): 2 emit sites, below 3+ threshold; no probe needed.
- **Rule 13** (revert-then-deeper-layer): no negative result; not applicable.
- **Rule 4** (never split a substrate move): single coordinated rung; not split.
- **Rule 15** (chapter-close-inspect): satisfied; post-fix exemplar inspection confirms all 6 target cells PASS; failure-table inspection of parent suite confirms remaining 13 FAILs are 9 super-deferred (eval-environment arc) + 3 update-target + 1 complex-assignment-target (tail-clusters not in this locale's scope).
- **Em-dash restraint**: drafts kept under target.

## Composes-with

- Parent locale `pilots/cruft-lowering-feature-gaps/` (CLFG-EXT 0 baseline; this rung is parent CLFG-EXT 3 in spirit).
- Sibling sub-locale `pilots/cruft-lowering-feature-gaps/super-reference-lowering/` (precedent: nested locale under CLFG for a coherent cluster; this locale follows the same pattern).
- For-of destructure precedent at `compiler.rs:2140–2395` (the substrate pattern this rung mirrors).
- Arc `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/`.
- Manifest refresh: 215 locales (was 214; +1 for this sub-locale spawn).
- Deferrals-ledger: no new entries.
- Deletions-ledger: no constraint-induced deletions.

Predicted yield on next test262-full run (not measured this rung): -12 matrix rows from `availability/missing-lowering-feature failure/other` (rank 56; pre-rung 88 → post-rung ~76).
