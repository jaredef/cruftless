# for-head-non-binding-lhs — Trajectory

## FHNB-EXT 0 — founding + R13-prospective-check (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9798) "continue with spin off." Spinoff from PPIF-EXT 1's Finding PPIF.2 (the runtime gap surfaced when PPIF-EXT 1 unblocked the parse path for Member-LHS for-in/of). Third locale in the FCA-amortization spinoff chain (LGSS → PPIF → FHNB), each at a deeper substrate tier than the prior (lexer → precedence-climber → bytecode lowering).

**Empirical anchor at founding**: cruft prints "undefined" for `var o={}; for (o.x in {p:1}) console.log(o.x)`; bun prints "p". Confirmed via probe under both runtimes. The "undefined" output traces to cruft's `ForBinding::Pattern` lowering allocating a local slot for an empty-name Identifier (the FHAPV-EXT 1 None-fallback shape), rather than evaluating the LHS to a Reference and PutValue'ing per iteration.

**Locale founded** with the constraint stack mirroring LGSS+PPIF:

- Coordinate: `ast-to-bytecode/language-lowering :: E2/internal-method:execution-semantics :: value-semantics/wrong-result :: property/for-in-of-non-binding-lhs-puts-to-reference`
- Telos: §14.7.5.6 ForIn/OfBodyEvaluation correctly evaluates non-binding LHS to a Reference and PutValues per iteration; the Member/Call/Parenthesized-LHS shape unblocked by PPIF-EXT 1 now binds correctly at runtime.
- Three-rung methodology: FHNB-EXT 1 (AST `AssignmentTarget(Expr)` variant + parser routing), FHNB-EXT 2 (bytecode lowering for the new variant, composing existing assignment-target machinery), FHNB-EXT 3 (extend to ParenthesizedExpression-around-pattern).

**R13 C1-C4 prospective check (per seed §Methodology)**:

- C1 (sibling closure pattern): HOLDS — LGSS + PPIF as parser-tier siblings; FHLA + FHAPV as for-head LHS validity siblings.
- C2 (shape-compat with substrate APIs): HOLDS — AST extension follows existing ForBinding shape; lowering composes existing compile-as-assignment-target machinery.
- C3 (cost-positive when integrated): TBV at FHNB-EXT 2; expected positive (per-iter cost is one assignment evaluation, comparable to bun's reference).
- C4 (bail safety): HOLDS — IsValidSimpleAssignmentTarget check at parse time prevents invalid LHS from reaching the runtime.

All four conditions hold prospectively.

**Status**: FHNB-EXT 0 FOUNDED. Awaiting FHNB-EXT 1 substrate move (AST variant + parser routing). The substrate move is multi-tier (AST + parser + bytecode); per R4 the three rungs should land as a coherent unit (no partial commit that leaves the AST extended but lowering unimplemented), so the keeper may want to gate the next move on multi-tier scope review before substrate begins.

**Findings**

**Finding FHNB.0 (the spinoff chain reaches three tiers and ten total LOC of named constraint)**: LGSS named current_lex_goal (1 field) + derive_lex_goal_after (1 predicate) at lexer-tier; PPIF named in_disallowed (1 field) + climber-gate (1 line) at precedence-climber tier; FHNB will name AssignmentTarget(Expr) variant (1 AST shape) + for-in/of-lowering branch (~40 LOC) at bytecode-tier. Each rung adds ONE named carrier and eliminates a class of downstream surface contamination. The keeper's conjecture (Telegram 9794) is being empirically corroborated as a stack: the FCA pattern at any tier surfaces the NEXT tier's named-constraint candidate, and the constraint at THIS tier remains small (1-2 carriers each) only because the upstream tiers have correctly absorbed their own concerns. This is Doc 729's resolver-instance pattern operating reflexively at the engagement-discipline tier.
