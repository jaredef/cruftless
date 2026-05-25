# strict-binding-id-in-assignment-pattern — Seed

## Telos

§13.15.1 IsValidSimpleAssignmentTarget + §13.2 BindingIdentifier early errors:
when an AssignmentPattern (object/array destructure used as assignment target,
no `let`/`const`/`var` head) contains a shorthand reference to `eval` /
`arguments` in strict mode, or to `yield` in a generator (or strict), it is a
SyntaxError. Sibling-pattern closure of SBEA-EXT 1, which handled the plain
BindingIdentifier path; this rung handles the AssignmentPattern path that goes
through `expr_to_binding_pattern`.

test262 residuals (5 of the 13 SyntaxError cluster fails):
- `for-of/dstr/obj-id-simple-strict.js` — `for ({ eval } of [{}])` strict
- `for-of/dstr/obj-id-init-simple-strict.js` — `for ({ eval = 0 } of [{}])` strict
- `for-of/dstr/obj-id-identifier-yield-ident-invalid.js` — `for ({ yield } of [{}])` strict
- `for-of/dstr/obj-id-identifier-yield-expr.js` — `for ({ yield } of [{}])` in generator
- `for-of/dstr/array-elem-target-simple-strict.js` — `for ([arguments] of [[]])` strict

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::expr_to_binding_pattern` — the
  AssignmentExpression→BindingPattern conversion that loses parser context.
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement`
  expression-head conversion (line ~913) — the call site that has `&self`
  available with `strict_mode` and `in_generator` flags in scope.
- SBEA-EXT 1 pattern (parser.rs:418) — the canonical strict-binding check.

## Methodology

After `expr_to_binding_pattern(e)` returns Some(pat) at the for-head site,
walk pat and reject leaves whose name violates the strict/generator binding
rules. New helper on Parser: `check_pattern_binding_ids(&self, pat, span)`
that recurses through Array(elements/rest) and Object(properties/rest),
checking each BindingIdentifier leaf name against `self.strict_mode` and
`self.in_generator`.

## Carve-outs

- This only fires at the for-head expression-conversion path. A standalone
  assignment expression `({ eval } = x)` in strict goes through a different
  parser route; covered by sibling closures if/when probed. For now this
  rung closes the for-head residuals only.

## Composes-with

- SBEA-EXT 1 (sibling pattern at plain BindingIdentifier path).
- FORA-EXT 1 (the for-head infrastructure this composes against).
- Per standing rule 13 prospective: C1 holds (SBEA is sibling closure
  pattern), C2 holds (Parser fields available at call site), C3 holds
  (cost is leaf-walk over a small pattern, negligible), C4 holds (failure
  is a parse error, not a runtime divergence).

## Resume protocol

Read `trajectory.md` tail.
