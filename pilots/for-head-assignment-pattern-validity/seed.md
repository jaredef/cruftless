# for-head-assignment-pattern-validity — Seed

## Telos

ECMA-262 §14.7.5.1 ForIn/ForOfStatement early errors: when the left-hand side of a for-in / for-of is an ArrayLiteral or ObjectLiteral, the literal must be a valid AssignmentPattern. Spec text: "It is a Syntax Error if LeftHandSideExpression is either an ObjectLiteral or an ArrayLiteral and if the lexical token sequence matched by LeftHandSideExpression cannot be parsed with no tokens left over using AssignmentPattern as the goal symbol."

cruft's `expr_to_binding_pattern` correctly returns `None` when the expression is not a valid AssignmentPattern (e.g., array literal with spread followed by an element, or spread with an initializer). The for-of/for-in head reinterpretation site at `stmt.rs:905` silently falls back to a string-empty `BindingIdentifier` instead of throwing, so the runtime accepts the invalid form.

test262: ~9 of the 32 SyntaxError residuals are this shape (`array-rest-before-element`, `array-rest-before-rest`, `array-rest-init`, `array-elem-nested-array-invalid`, `array-elem-nested-obj-invalid`, `obj-prop-nested-*-invalid`, `obj-rest-not-last-element-invalid`).

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::expr_to_binding_pattern` (line 23) — conversion that returns None on invalid patterns.
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` expression-head conversion (line 905) — the silent fallback site.

## Methodology

At the for-of / for-in head conversion site (line 905), when the head expression is an ArrayLiteral or ObjectLiteral AND `expr_to_binding_pattern` returns None, throw ParseError. Use a span from the original expression.

## Carve-outs

- `[...x,]` (rest followed by trailing comma) — array literal parser allows trailing comma after spread for the LITERAL form. The AST doesn't preserve trailing-comma info, so this shape can't be detected at conversion time. Separate sibling concern; would require either threading trailing-comma metadata or rejecting at the literal parse site (which conflicts with valid expression usage `[...a, b]`).
- Non-literal LHS that isn't a valid SimpleAssignmentTarget (e.g., `[arguments] of ...` in strict): requires per-leaf IsValidSimpleAssignmentTarget check. Separate.

## Composes-with

- NSPS.2 sibling list — this closes 9 of the 32 remaining SyntaxError fails.
- The expr_to_binding_pattern function is the existing canonical AssignmentPattern validator; we just promote the None to a throw.

## Resume protocol

Read `trajectory.md` tail.
