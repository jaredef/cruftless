# arrow-misc-early-errors — Seed

## Telos

Two distinct arrow-function grammar early errors that share a substrate site:

1. **§15.3.1 NoLineTerminator before `=>`** — `() \n => {}` is a SyntaxError.
   ArrowFunction's ArrowParameters and `=>` are joined by a `[no LineTerminator
   here]` production; ASI must NOT insert a semicolon and break the form.

2. **§14.1 / §15.1.1 BindingRestElement-with-Initializer** — `(...x = []) => {}`
   is a SyntaxError. A rest parameter cannot have a default initializer.
   Applies universally: arrow, function, method, generator, async.

test262 residuals (2 of 4):
- `expressions/arrow-function/syntax/early-errors/asi-restriction-invalid.js`
- `expressions/arrow-function/dflt-params-rest.js`

## Apparatus

- `pilots/rusty-js-parser/derived/src/expr.rs::parse_arrow_function` line ~1292
  the `=>` consume site (ALTA-EXT 1).
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_function_parameters_inner`
  line ~328 the default-parse branch (RPDF-EXT 1).

## Methodology

1. ALTA: at the `=>` expect-site, check `self.lookahead_preceded_by_lt()`
   on the Arrow token; if true and current is Arrow, throw.
2. RPDF: in the param-parse loop, after parsing optional default, if
   `rest && default.is_some()`, throw. Single check covers arrow, function,
   method, generator, async (all share parse_function_parameters_inner).

## Carve-outs

- The `=>` lookahead-LT check uses the same `preceded_by_line_terminator`
  flag the ASI machinery uses elsewhere (e.g., return / throw / continue
  / break / postfix++). No new lexer state.

## Composes-with

- RPTC-locale (rest-param trailing-comma) — sibling rest-element invariant
  at the same parse site.
- SBEA/SBAP — sibling grammar early-error closures.

## Resume protocol

Read `trajectory.md` tail.
