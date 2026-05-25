# array-pattern-rest-trailing-comma — Seed

## Telos

§13.3.3 ArrayAssignmentPattern grammar: AssignmentRestElement is the last
element of an AssignmentElementList and is NOT followed by a comma. The
form `[...x,]` is a valid ArrayLiteral (expression) but an invalid
ArrayAssignmentPattern. The two require source-text distinction that the
parser was previously dropping at the literal-parse site.

test262 residuals (cluster D, 2 of 2):
- `for-of/dstr/array-rest-before-elision.js` — `for ([...x,] of [[]])`
- `for-of/dstr/array-rest-elision-invalid.js` — `for ([...x,] of [[]])`

## Apparatus

- `pilots/rusty-js-ast/src/lib.rs::Expr::Array` — adds a new
  `trailing_comma_after_spread: bool` field.
- `pilots/rusty-js-parser/derived/src/expr.rs::parse_array_literal` (line 680).
- `pilots/rusty-js-parser/derived/src/stmt.rs::expr_to_binding_pattern`
  Array arm (line 28).
- `pilots/rusty-js-parser/derived/src/expr.rs:962` template-strings synthesis.
- `pilots/rusty-js-bytecode/derived/src/compiler.rs:5505`
  binding_pattern_to_assignment_expr round-trip.

## Methodology

1. Add `trailing_comma_after_spread: bool` to `Expr::Array`.
2. `parse_array_literal`: track a local flag set true only when the most
   recent pushed element was Spread AND a trailing comma was bumped before
   RBracket. Clear when any subsequent element follows.
3. `expr_to_binding_pattern` Array arm: when flag is set, return None.
   FHAPV-EXT 1's literal-shape-throw at the for-head call site converts
   the None into a SyntaxError.
4. Update non-parse construct sites to default `false`.

## Carve-outs

- Standalone assignment `([...x,] = value)` is parsed via a path that
  doesn't pass through `expr_to_binding_pattern` (the comma-operator-then-
  destructure-assign route). Per probe, still accepts. Spec also forbids
  this; follow-on rung when residuals warrant.
- `[...x, ,]` (rest with explicit middle elision) in standalone assignment:
  same path-omission carve-out.

## Composes-with

- FHAPV-EXT 1 (the for-head literal-None → SyntaxError promotion this
  rung's flag exploits).
- FORA-EXT 1, SBAP-EXT 1, FHLA-EXT 1, FAOF-EXT 1, ALTA+RPDF-EXT 1 (the
  sibling closures from today's SyntaxError-cluster arc).

## Resume protocol

Read `trajectory.md` tail.
