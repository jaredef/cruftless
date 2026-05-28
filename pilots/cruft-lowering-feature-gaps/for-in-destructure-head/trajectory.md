# for-in-destructure-head — Trajectory

## FIDH-EXT 0 — LANDED (2026-05-28) — founding + Rule-23 baseline-inspect

**Trigger**: keeper directive Telegram 10231 selecting the parent's for-in-destructure-head tail as the next sibling-child of `cruft-lowering-feature-gaps/` (after super-reference-lowering closed +13 PASS in its child suite). Per the parent's CLFG-EXT 0 baseline, this tail covers 6 exemplars / 12 matrix rows / single coherent compiler diagnostic.

**Baseline-inspect** (Rule 23):
- All 6 exemplars in the parent's exemplar list confirm `compile: for-in with destructure head not yet supported` at both compiler emit sites (`compiler.rs:2906, 2924`).
- For-of handler in the same file (`compiler.rs:2140–2154 + 2332–2395`) already implements the equivalent destructure-head pattern; this sub-locale's substrate move mirrors that precedent.

## FIDH-EXT 1 — LANDED (2026-05-28) — for-in destructure head bytecode lowering

Per keeper directive Telegram 10231. First substrate rung; closes the 6-exemplar / 12-matrix-row cluster.

**Phase 1 (Spawn)**:
- **M** = `for ([Decl|Pattern] in iterable) ...` where the head is a non-Identifier BindingPattern (Array or Object).
- **T** = bytecode lowers to the standard for-in scan with a temp source slot for the per-iter key string, then destructures the key into bound names (Decl) or assignment targets (Pattern) per ECMA-262 §13.7.5 + §13.15.
- **I** = `compiler.rs` for-in handler. Two changes:
  1. Tuple at line 2888 extended from 3-tuple to 4-tuple `(bind_slot, destr_pat, per_iter_fresh, assign_target)`.
  2. Body emission injects `emit_destructure` (Decl-mode) or `emit_destructure_assign` (Pattern-mode) between the per-iter `Op::InitLocal` write and `compile_stmt(body)`, mirroring for-of at lines 2361–2395.
- **R** = lattice with the for-of destructure path (shared `emit_destructure` + `emit_destructure_assign` helpers; shared TDZ + `ResetLocalCell` per-iter discipline). DAG ↑ AST `ForBinding::{Decl,Pattern}` Array/Object variants. Alphabet-exchange ↑ destructuring-binding emission the parser already produces.
- **Observability**: ordinary (test262 sameValue + scope-binding assertion).
- **Mouth-gating prerequisite**: for-of destructure substrate (already landed; this rung's helpers + pattern are inherited).

**Phase 2 (Baseline-inspect)** per Rule 23: baseline confirmed pre-rung in seed.md; all 6 exemplars fail with the same compiler diagnostic.

**Phase 3**: no Pin-Art-probe-if-duplicated needed — 2 emit sites with identical shape, below 3+ threshold, single-round substrate close fits.

**Phase 4**: single-round, no negative.

**Substrate** (~60 LOC in `pilots/rusty-js-bytecode/derived/src/compiler.rs`):

1. **Tuple extension at line 2888**: added `destr_pat: Option<BindingPattern>` as second element of the 4-tuple `(bind_slot, destr_pat, per_iter_fresh, assign_target)`.

2. **`ForBinding::Decl` non-identifier arm**: replaced "not yet supported" error with the for-of's pre-allocation pattern — iterate `pat.collect_names()`, allocate each as a local under `kind`, allocate temp source slot, return `(temp_slot, Some(pat.clone()), is_let_or_const, None)`.

3. **`ForBinding::Pattern` non-identifier arm**: replaced "not yet supported" error with temp source slot allocation; route through `emit_destructure_assign` at body emission per SMDR-EXT 1 strict-mode assignment-target reference semantics. Returns `(temp_slot, Some(other.clone()), false, None)`.

4. **Per-iter `ResetLocalCell` extension** at line 3001 area: when `per_iter_fresh && destr_pat.is_some()`, also reset each pre-allocated bound-name slot per iter, mirroring for-of at compiler.rs:2332–2339.

5. **Destructure emission injection** between `Op::InitLocal` write at line 3013 and `compile_stmt(body)` at line 3020: when `destr_pat.is_some()`, branch on whether the head was a Pattern (use `emit_destructure_assign` via `binding_pattern_to_assignment_expr` conversion) or a Decl (use `emit_destructure` directly).

**Yield** (target exemplar suite, all 6 cells):
```text
PRE-FIDH-EXT 1:  all 6 FAIL with "compile: for-in with destructure head not yet supported"
POST-FIDH-EXT 1: all 6 PASS
```

Parent CLFG exemplar suite: 13/22 (super child only) → **19/32 (+6)**. The +6 corresponds exactly to the for-in-destructure cluster; remaining 13 FAILs are 9 super deferred behind eval-environment arc + 3 update-target + 1 complex-assignment-target (two tail-clusters not yet addressed by this locale).

**Gates**:
- TAMM unchanged 82/100.
- TAWR unchanged 63/100.
- diff-prod unchanged 61/51.
- Build clean.
- Sanity intact (`console.log(1+1)` → 2).

**Tag**: `cluster-for-in-destructure-head-bytecode-lowering-1`.

**Standing finding (none required)**: the rung is the canonical "mirror an existing substrate pattern from sibling-locus" close. The for-of destructure-head substrate (lines 2140–2154 + 2332–2395) was the existing pattern; for-in needed the same shape applied at its own emit sites. No new standing rule warranted; the cross-locus precedent absorbs the rationale.

**Phase 6 (deferral emission)**: no new deferrals surfaced. The locale's residual scope is empty after this rung; remaining parent-tier tail-clusters (update-target lowering, complex-assignment-target lowering) belong to separate sibling-children of the parent.

**Status**: FIDH-EXT 1 CLOSED locally. Locale telos achieved in 1 rung (Doc 744 §V.3 prediction band of ≤3-rung close honored). Arc-tier accumulation under `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/`: +6 cells to parent CLFG yield.
