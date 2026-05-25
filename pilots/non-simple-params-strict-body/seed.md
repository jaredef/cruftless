# non-simple-params-strict-body — Seed

## Telos

ECMA-262 §15.2.1 (FunctionDeclaration) / §15.3.1 (ArrowFunction) early error: when `ContainsUseStrict(body)` is true AND `IsSimpleParameterList(parameters)` is false, throw SyntaxError at parse.

A "simple parameter list" is one where every parameter is a plain `BindingPattern::Identifier` with no default initializer and no rest. Destructured params (`{a}` / `[a]`), default-valued params (`x = 1`), and rest params (`...args`) all make the list non-simple.

The rule prevents ambiguity over what `"use strict"` in the body means when params already contain syntactic shapes that strict-mode treats differently (per spec, the directive could change parameter-name validation, but only if params haven't already used non-simple forms).

cruft's parser doesn't enforce this. test262 cluster: 4 confirmed shapes across arrow + function declarations.

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_function_body_g` — current "use strict" directive detection at body entry.
- `pilots/rusty-js-ast/src/lib.rs::Parameter` — `target: BindingPattern`, `default: Option<Expr>`, `rest: bool`.
- 8 call sites of `parse_function_body_g` carry their parsed params.

## Methodology

1. Add `is_simple_param_list(&[Parameter]) -> bool` helper (every param is `Identifier && default.is_none() && !rest`).
2. Extend `parse_function_body_g` signature with `is_simple: bool`.
3. In the existing `peek_use_strict_directive` branch, if `!is_simple`, throw ParseError ("Illegal 'use strict' directive in function with non-simple parameter list").
4. Update 8 call sites to pass `is_simple_param_list(&params)`.

## Carve-outs

- Class methods + constructor: spec treats constructor non-simple-params identically; already covered by going through parse_function_body_g.
- Object-literal method shorthand: same path.
- Arrow expression body (`x => x`): never has a `"use strict"` directive since there's no body block, so non-simple check doesn't fire. Arrow block body (`x => { ... }`) takes the standard path.

## Composes-with

- SMPT-EXT 2/3 (strict-mode parser tracking): "use strict" directive detection already in place at parse_function_body_g.
- VMA.5 (own vs proto): unrelated, but same _via-audit-driven discipline ethos.

## Resume protocol

Read `trajectory.md` tail.
