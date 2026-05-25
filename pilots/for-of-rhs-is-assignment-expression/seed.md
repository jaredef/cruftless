# for-of-rhs-is-assignment-expression — Seed

## Telos

ECMA-262 §14.7.5 ForIn/ForOfStatement grammar:
- `for ( LeftHandSideExpression in Expression ) Statement` — for-in RHS is **Expression** (sequence/comma allowed).
- `for ( LeftHandSideExpression of AssignmentExpression ) Statement` — for-of RHS is **AssignmentExpression** (no comma operator).

cruft uses `parse_expression` for both. So `for (x of [], [])` is accepted; spec wants SyntaxError (the comma operator isn't allowed in for-of's RHS).

test262: `head-expr-no-expr.js`, `head-decl-no-expr.js`, `head-var-no-expr.js`.

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` — 4 sites parse the RHS after consuming `in`/`of`:
  - line 697 (destructure-head variant)
  - line 772 (plain-id head)
  - line 872 (fast-path for-id-in/of)
  - line 899 (expression-head)

## Methodology

At each of the 4 sites, replace:
```rust
let right = self.parse_expression()?;
```
with:
```rust
let right = if is_of {
    self.parse_assignment_expression()?
} else {
    self.parse_expression()?
};
```

## Carve-outs

- None — bounded mechanical change with a single discrimination.

## Composes-with

- NSPS.2 sibling list.
- The discrimination already exists in scope (`let is_of = self.is_contextual_keyword("of")`); just need to use it for RHS parsing too.

## Resume protocol

Read `trajectory.md` tail.
