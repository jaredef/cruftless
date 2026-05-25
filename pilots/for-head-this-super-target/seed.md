# for-head-this-super-target — Seed

## Telos

§13.15.1 IsValidSimpleAssignmentTarget: the for-in / for-of LHS must be a
valid assignment target. `this` and `super` are explicitly NOT valid
SimpleAssignmentTargets. cruft's bare-ident fast-path at
`parse_for_statement` mis-bound `this` (tokenized as `TokenKind::Ident("this")`)
as a BindingIdentifier named "this", silently accepting the form. The
expression-head path then never saw the expression.

test262 residuals (3 of 13):
- `for-of/head-lhs-non-asnmt-trgt.js` — `for (this of [])`
- `for-of/head-lhs-cover-non-asnmt-trgt.js` — `for ((this) of [])`
- `for-in/head-lhs-non-asnmt-trgt.js` — `for (this in {})`

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` line ~867
  bare-ident fast-path keyword exclusion list.
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` line ~913
  expression-head LHS validation.

## Methodology

1. Exclude `this`/`super` from the bare-ident fast-path exclusion list so
   `for (this of …)` falls through to the expression-head path.
2. At the expression-head conversion, before `expr_to_binding_pattern`,
   unwrap `Expr::Parenthesized` layers and reject if the inner is `This`
   or `Super`. Returns ParseError.

## Carve-outs

- Other invalid assignment targets (literals, unary ops, ...) are not
  surfaced by this residual; conservative-strip per R14 keeps the
  rejection narrow to `this`/`super`.

## Composes-with

- FORA-EXT 1 (for-head infrastructure).
- FHAPV-EXT 1 (the conversion-failure rejection pattern this extends).
- SBAP-EXT 1 (sibling closure on the same for-head call site).

## Resume protocol

Read `trajectory.md` tail.
