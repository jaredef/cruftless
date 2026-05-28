# for-in-destructure-head — Seed

**Locale tag**: `L.cruft-lowering-feature-gaps.for-in-destructure-head`

**Status**: FOUNDED 2026-05-28 as second sub-locale under `pilots/cruft-lowering-feature-gaps/`, parallel to `super-reference-lowering/`. Enrolled under arc `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/`.

## Telos

Close the `compile: for-in with destructure head not yet supported` early-error cluster surfaced by the parent locale (CLFG) baseline. Specifically: the bytecode compiler's `compile_for_in` path currently rejects any `for (let [a, b] in obj) ...` or analogous array/object destructuring pattern in the for-in head with a compile-time error at two emit sites (`compiler.rs:2906, 2924`). The for-of handler already supports destructure heads via a temp-source-slot + per-iter `emit_destructure` pattern; this locale mirrors that for for-in.

The mouth-terminus pair per Doc 744:
- **M** = `for ([Decl|Pattern] in iterable) ...` where the head is a non-Identifier BindingPattern (Array or Object).
- **T** = bytecode lowers to the standard for-in scan (Object.keys + indexed access) with a temp source slot for the per-iter key, then destructures the key into the head's bound names (Decl path) or assignment targets (Pattern path) per ECMA-262 §13.7.5 + §13.15.
- **I** = the two `for-in with destructure head not yet supported` emit sites at `compiler.rs:2906, 2924`; replace with destr_pat tracking + body-emission `emit_destructure` injection mirroring the for-of pattern at `compiler.rs:2140–2154 + 2332–2395`.
- **R** = lattice with the for-of destructure path (same `emit_destructure` + `emit_destructure_assign` helpers; same TDZ + `ResetLocalCell` discipline for let/const). DAG ↑ AST `ForBinding::Decl` / `ForBinding::Pattern` variants. Alphabet-exchange ↑ destructuring-binding semantics that the parser already produces.

## Failure-shape inventory

Parent CLFG baseline identified **6 exemplars** with this shape; matrix surface is 12 rows. Exemplars (per `pilots/cruft-lowering-feature-gaps/exemplars/exemplars.txt`):

- `language/statements/for-in/head-let-destructuring.js`
- `language/statements/for-in/head-var-bound-names-dup.js`
- `language/statements/for-in/scope-body-lex-close.js`
- `language/statements/for-in/scope-head-var-none.js`
- `staging/sm/statements/for-loop-declaration-contains-computed-name.js`
- `staging/sm/statements/for-loop-declaration-contains-initializer.js`

All 6 fail with the same compiler diagnostic; baseline-inspect confirms the projection is not a runner blur.

## Methodology

Mirror the for-of destructure-head pattern at `compiler.rs:2140–2154 + 2332–2395`:

1. Extend the for-in `bind_slot` tuple to a 4-tuple `(bind_slot, destr_pat, per_iter_fresh, assign_target)`.
2. In the `ForBinding::Decl` arm: when target is Array/Object pattern, pre-allocate every bound name as a local under `kind`; allocate a temp source slot; set `destr_pat = Some(pat.clone())`.
3. In the `ForBinding::Pattern` arm: when pat is Array/Object, allocate a temp source slot; set `destr_pat = Some(pat.clone())`; route through `emit_destructure_assign` (strict-mode reference semantics per SMDR-EXT 1).
4. After `Op::InitLocal` writes the key string into `bind_slot` (line 3012–3013), inject the destructure: call `emit_destructure(pat, bind_slot)` for Decl-mode or `emit_destructure_assign(target_expr, bind_slot)` for Pattern-mode.
5. Mirror the for-of `ResetLocalCell` per-iter pattern for let/const bound names (currently only emitted for the `bind_slot` itself; needs to also reset the bound-name slots when per_iter_fresh).
6. Mirror the for-of TDZ pattern for the bound-name slots at scope-entry (already covered by the existing IR-EXT 24 TDZ loop at lines 2936–2947 if the bound names get pre-allocated above).

## Carve-outs

- The harness-edge `head-var-bound-names-dup.js` and `scope-body-lex-close.js` may test additional scope semantics beyond the basic destructure-head lowering (TDZ behavior across iterations; var-name duplication detection). If post-fix exemplar inspection surfaces these as residual, treat them as separate rungs within this locale rather than as failures of the destructure-head fix.
- Computed-name and initializer cases in the two staging `for-loop-declaration-contains-*` exemplars use computed property names + initializers inside the destructuring pattern; the for-of pattern handles both via the recursive `emit_destructure` walk. Verify post-fix.

## Composes-with

- Parent locale `cruft-lowering-feature-gaps/` (CLFG-EXT 0 baseline, CLFG-EXT 1 super-child founding, CLFG-EXT 2 super-child progress).
- Sibling sub-locale `super-reference-lowering/` (precedent: nested locale under CLFG for a coherent cluster).
- For-of destructure precedent at `compiler.rs:2140–2395` (the substrate pattern this rung mirrors).
- Helpers `emit_destructure` (line 3118) and `emit_destructure_assign` (line 5911).
- Standing rules: Rule 4 (never split a substrate move), Rule 15 (chapter-close-inspect), IR-EXT 24+25+26 (TDZ + InitLocal discipline for head-binding-then-init).
- Arc `apparatus/arcs/2026-05-28-lowering-feature-gap-triage/`.

## Resume protocol

Per Doc 581: read this seed; read `trajectory.md` tail; verify against the for-of precedent at `compiler.rs:2140–2395`; run `bash pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh 2>&1 | grep destructure` to confirm current baseline.

## Falsifiers

- **Pred-1**: post-fix, all 6 exemplars listed above PASS or skip. If any fail with a non-`compile:` error (e.g., a runtime DestructuringPatternError), inspect — may indicate the destructuring-binding emission has a bug surface this locale should address.
- **Pred-2**: no regression in TAMM (82/100), TAWR (63/100), diff-prod (61/51), or the parent CLFG exemplar suite. If for-of destructure tests regress (they share the helpers), inspect immediately per Rule 13.
- **Pred-3**: the matrix's `availability/missing-lowering-feature` rank 56 count (88 fails) drops by approximately 12 (the projected matrix-row coverage of the for-in-destructure cluster). If it drops by significantly less, the cluster's matrix coverage was over-estimated — surface to keeper.
