# for-of-async-lookahead — Seed

## Telos

§14.7.5 ForInOfStatement grammar: `for ( [lookahead ∉ { let [, async of }]
LeftHandSideExpression of AssignmentExpression ) Statement`. The token
sequence `async of` at the start of a for-of head is a SyntaxError; it
disambiguates from `for await … of …` and prevents the bare identifier
`async` from being mistaken for an async-arrow start.

test262: `language/statements/for-of/head-lhs-async-invalid.js`
(`var async; for (async of [1]) ;`).

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` bare-ident
  fast-path (line ~872).

## Methodology

In the fast-path, when the bumped identifier name is `async` and the
following token is `of` (not `in`), return ParseError. `async of` only —
`async in {}` is permitted (identifier binding).

## Carve-outs

- `for await (async of …)` — the for-await form has its own parsing path
  not touched here. Probe confirms unchanged.
- `async` as plain identifier in other for shapes (`for (async = 0; …)`,
  `for (async in {})`) remains accepted.

## Composes-with

- FORA-EXT 1, FHLA-EXT 1 (sibling fast-path discriminations).

## Resume protocol

Read `trajectory.md` tail.
