# yield-in-function-params — Seed

## Telos

ECMA-262 §15.3.1 ArrowFunction early errors: "It is a Syntax Error if ArrowParameters Contains YieldExpression is true."

Parallel rule §15.5.1 (GeneratorDeclaration) bans YieldExpression in the generator's own formal parameters.

cruft's yield-branch fires when `in_generator || strict_mode`, parsing yield as YieldExpression. When the arrow appears inside a generator (e.g. `function* g() { (x = yield) => {} }`), the arrow's `(x = yield)` parses cleanly today because in_generator was inherited from the enclosing g(). Spec rejects.

test262: `language/expressions/arrow-function/param-dflt-yield-expr.js`.

## Apparatus

- `pilots/rusty-js-parser/derived/src/expr.rs` yield-branch (~line 241).
- All `parse_function_parameters()` call sites need to set a new `in_function_params: bool` flag during param parse.

## Methodology

1. Add `Parser::in_function_params: bool` (init false).
2. Around every `parse_function_parameters()` call (8 sites), save + set true; restore after.
3. In yield-branch, when `in_function_params && in_generator`, throw ParseError.

This minimum-scope move catches the arrow-in-generator case (the test262 target). The parallel case `function* g(x = yield)` requires also setting `in_generator=true` around the generator's own param parse; deferred to a sibling rung.

## Carve-outs

- Generator function's OWN params (test shape: `function* g(x = yield) {}`): needs in_generator=true around the gen's own param parse. Separate rung.
- yield delegation (`yield*`) in params: covered by the same check since yield* goes through the same branch.
- async-generator: spec extends the same rule; covered by the in_generator flag.

## Composes-with

- NSPS.2 sibling list.
- SMPT-EXT 3 (in_generator tracking — provides the precondition).

## Resume protocol

Read `trajectory.md` tail.
