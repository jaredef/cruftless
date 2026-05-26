# for-head-non-binding-lhs — Seed

## Spinoff from PPIF-EXT 1's Finding PPIF.2 (amortization-conjecture corroboration).

PPIF-EXT 1 unblocked the parse path for `for (o.x in {p:1}) ...` (and analogous Member/Call LHS shapes). PPIF.2 surfaced that cruft's runtime fails to actually assign to `o.x` on each iteration — the body sees `o.x = undefined`, and after the loop `o.x` is unchanged. This locale closes that downstream gap.

Per the keeper's amortization conjecture (Telegram 9794), the LGSS pattern repeats at every tier where a constraint is implicit; this is the runtime-tier instance, two tiers downstream of LGSS, one tier downstream of PPIF.

## Telos

Materialize the engine-DAG coordinate

```
ast-to-bytecode/language-lowering ::
  E2/internal-method:execution-semantics ::
  value-semantics/wrong-result ::
  property/for-in-of-non-binding-lhs-puts-to-reference
```

ECMA-262 §14.7.5.6 ForIn/OfBodyEvaluation: when the LHS is an assignment target (LeftHandSideExpression) rather than a binding pattern, **evaluate the LHS to a Reference Record and PutValue(ref, iterationValue)** on each iteration. cruft's compiler currently lowers ALL `ForBinding::Pattern` cases as if they were Identifier-target local-slot bindings — for Identifier LHS this is correct; for MemberExpression / CallExpression / non-Identifier-shape LHS it silently drops the assignment.

The induced property is **for-head LHS = LeftHandSideExpression** (not just BindingPattern). The Member-LHS, Call-LHS, and any other valid AssignmentTarget form binds correctly per iteration.

## Apparatus

- `pilots/rusty-js-ast/src/lib.rs:548` — `ForBinding` enum. Currently has `Decl { kind, target }` and `Pattern(BindingPattern)`. Needs a third variant for non-binding assignment targets, or the `Pattern` variant needs to carry an `Expr` alternative.
- `pilots/rusty-js-parser/derived/src/stmt.rs::expr_to_binding_pattern` (line 23) — currently returns `Option<BindingPattern>`; the None case for Member/Call/etc. falls back to opaque empty-name Identifier (per FHAPV-EXT 1's None-handling). New behavior: when the Expr is a valid SimpleAssignmentTarget but not a BindingPattern, return the Expr wrapped as the new `ForBinding` variant.
- `pilots/rusty-js-bytecode/derived/src/compiler.rs:2103+` — ForIn lowering. `bind_slot` allocation assumes Identifier-shape; needs a new branch for the AssignmentTarget variant that evaluates the LHS to a Reference and PutValues the iteration value.
- `pilots/rusty-js-bytecode/derived/src/compiler.rs:1515+` — ForOf lowering. Symmetric to ForIn.
- The compiler already has assignment-target-evaluation machinery for plain `o.x = value` statements; the new for-in/of branch should compose this existing machinery rather than invent new bytecode shapes.

## Methodology

Three rungs.

### Rung 1 — AST + parser plumbing (FHNB-EXT 1)

Add `ForBinding::AssignmentTarget(Expr)` variant. Update `expr_to_binding_pattern` (or add a sibling `expr_to_for_binding`) that returns:
- `Some(ForBinding::Pattern(p))` when the Expr is a BindingPattern (Identifier / Array / Object).
- `Some(ForBinding::AssignmentTarget(e))` when the Expr is a non-pattern valid SimpleAssignmentTarget (MemberExpression, ParenthesizedExpression-around-LHS, etc.).
- `None` when the Expr is not a valid LHS (literals, unary expressions, etc.) — caller then promotes to SyntaxError per FHAPV-EXT 1's discipline.

Update `parse_for_statement`'s expression-head path to route through the new function. Update `is_pattern_literal`-check carve-out from FHAPV-EXT 1 if needed.

Pattern-validity check: per §13.15.1 IsValidSimpleAssignmentTarget. The check applies at the for-head LHS conversion site.

### Rung 2 — Bytecode lowering for AssignmentTarget (FHNB-EXT 2)

In `ForIn` and `ForOf` compile, add a branch for `ForBinding::AssignmentTarget(e)`:
- DO NOT allocate a local slot (no name to bind).
- For each iteration: compile the AssignmentTarget Expr as a reference (the same as the LHS of `x = value` assignment); emit a put-value op with the iteration value.

Reuses the compiler's existing assignment-target-evaluation machinery — the per-iter cost is one assignment evaluation, no new opcodes.

### Rung 3 — extend to ParenthesizedExpression-around-pattern (FHNB-EXT 3)

`for (({a,b}) of [{a:1,b:2}])` — the LHS is a parenthesized ObjectPattern, which is structurally an Expr (Parenthesized wrapping Object). Per ECMA-262 cover-grammar rules, this re-parses as AssignmentPattern. Verify FHNB-EXT 1's `expr_to_for_binding` handles the unwrap correctly.

## Carve-outs

- **`eval` and `arguments` as LHS in strict mode** — IsValidSimpleAssignmentTarget rejects these in strict per §13.15.1. The pattern-validity check in FHNB-EXT 1 should apply this.
- **`this` / `super` as LHS** — already rejected by FHLA-EXT 1 (today's earlier work); the FHNB rung composes against that closure.
- **CallExpression as LHS** — spec allows in non-strict (legacy) but disallows in strict. Conservative-strip per R14: reject in both modes for now; revisit if test262 surface demands non-strict acceptance.

## Composes-with

- `pilots/parser-precedence-in-flag/` (PPIF) — surfaced this gap via PPIF-EXT 1's parse-shape unblock; this locale closes the runtime-tier follow-on.
- `pilots/lexer-goal-symbol-selection/` (LGSS) — the grandparent in the spinoff chain; same FCA pattern at a different tier.
- `pilots/for-head-this-super-target/` (FHLA-EXT 1) — sibling LHS-validity check at the for-head; composes via the IsValidSimpleAssignmentTarget machinery.
- `pilots/for-head-assignment-pattern-validity/` (FHAPV-EXT 1) — sibling AssignmentPattern-validity at the for-head; provides the None-handling discipline this locale extends.
- `apparatus/docs/ecma-conformance-...md` §XI — the lowering coordinate class this locale instantiates at the value-semantics/wrong-result cut.

## R13 prospective C1-C4 at founding

- **C1 (sibling closure pattern)**: HOLDS — LGSS + PPIF are siblings at parser tiers; FHLA + FHAPV are siblings at the for-head LHS validity tier. Both empirical anchors.
- **C2 (shape-compat with substrate APIs)**: HOLDS — AST extension follows existing ForBinding shape; bytecode lowering composes existing assignment-target machinery; parser conversion follows existing expr_to_binding_pattern shape.
- **C3 (cost-positive when integrated)**: TO BE VERIFIED at FHNB-EXT 2 — per-iter cost is one assignment evaluation; should be comparable to bun's reference behavior.
- **C4 (bail safety)**: HOLDS — runtime-tier discrimination; the IsValidSimpleAssignmentTarget check at parse time prevents invalid LHS from reaching the runtime.

All four conditions hold prospectively. Per R13 thirteenth-corroboration discipline, expect ≤3-round closure.

## Empirical anchors (test262)

Tests that exercise non-binding LHS in for-head — exemplar suite to be assembled at FHNB-EXT 1 close:

- `test262/test/language/statements/for-in/head-lhs-member.js` (and similar) — member-LHS for-in
- `test262/test/language/statements/for-of/head-lhs-cover-non-asnmt-trgt.js` (already passing post-FHLA-EXT 1; this locale extends coverage to valid non-binding shapes)
- `test262/test/language/statements/for-in/iteration-mutable-binding-changes.js` — for-in LHS mutation across iterations
- TXC (TypeScript execute corpus) — any rxjs/ajv/pino source that uses member-LHS in for-in/of will surface

## Predicted yield

- Net executable LoC: small grow at AST + parser + 2 lowering sites (AssignmentTarget variant ~20 LOC AST + ~30 LOC parser + ~40 LOC × 2 lowering = ~120 LOC).
- Test262: a small cluster of LHS-shape tests previously failing as PARSE errors (now succeeding parse-wise post-PPIF but emitting wrong runtime) will flip to PASS. Estimate ~5-15 tests; full count at exemplar-suite assembly.
- **Substrate WIN beyond test count**: the runtime gap surfaced by Finding PPIF.2 closes; cruft becomes correct for the WHOLE class of non-binding for-head LHS (Member, Call-with-non-eval, Parenthesized-LHS, etc.). The keeper's amortization-conjecture stack now reaches three tiers (lexer-state → precedence-climber-state → runtime-LHS-assignment-evaluation), each tier picking up the previous tier's unblocked shape and naming the constraint at the right tier to make new correctness possible.

## Resume protocol

Read `trajectory.md` tail.
