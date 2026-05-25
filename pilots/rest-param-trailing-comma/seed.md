# rest-param-trailing-comma — Seed

## Telos

ECMA-262 §15.1.1 (FormalParameters early errors): the rest parameter (`...name` / `...{a,b}` / `...[x]`) must not be followed by a trailing comma. `(a, b,)` is allowed (trailing comma after non-rest); `(a, ...rest,)` is a SyntaxError.

cruft's `parse_function_parameters` accepts trailing commas unconditionally, even after a rest param. test262 cluster: `rest-params-trailing-comma-early-error.js` + parallel function-decl tests.

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_function_parameters` (line 293). The comma-then-continue branch at line 310-312 doesn't check whether the just-parsed param was rest.

## Methodology

In the trailing-comma branch (line 310 after pushing the param), if the just-pushed param is `rest`, throw SyntaxError. Spec wording: "A rest parameter may not be followed by a trailing comma."

## Carve-outs

- Trailing comma after a non-rest final param is allowed (ES2017+); no change.
- Trailing comma after rest in function-call argument lists is a separate spec rule (also a SyntaxError per §15.2.1 / call.body); deferred unless probe surfaces failures.

## Composes-with

- NSPS.2 sibling-locale list (this is one of ~8 separable parser-tier early-error rules).
- NSPS-EXT 1's parse_function_parameters call sites benefit transparently (the early error fires once at param parse).

## Resume protocol

Read `trajectory.md` tail.
