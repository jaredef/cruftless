# strict-binding-eval-arguments — Seed

## Telos

ECMA-262 §13.2 (BindingIdentifier early errors): in strict mode, `eval` and `arguments` are reserved as binding identifiers. `var eval`, `let arguments`, `function eval() {}`, parameter named `arguments`, etc. all throw SyntaxError at parse.

cruft's `parse_binding_identifier` carries an explicit v1 carve-out: "do not reject reserved-word bindings here." The check is the responsibility of this locale.

test262 cluster from NSPS.2: `for-in/var-arguments-strict.js`, `var-eval-strict.js`, `var-arguments-fn-strict.js`. All have `flags: [onlyStrict]` so the runner injects `"use strict"` at the top — strict_mode is already true when the binding-id is parsed.

## Apparatus

- `pilots/rusty-js-parser/derived/src/parser.rs::parse_binding_identifier` (line 409).
- `Parser::strict_mode` (SMPT-EXT 2 tracking) is already correct at the binding-id parse site for in-body bindings.

## Methodology

In `parse_binding_identifier`, before producing the BindingIdentifier, if `strict_mode` AND name is `"eval"` or `"arguments"`, throw ParseError.

## Carve-outs

- Function name where the function-decl is INSIDE strict body (handled correctly since strict_mode is true when the inner name parses).
- Parameter named `arguments`/`eval` where the body's `"use strict"` retroactively makes params strict: NOT handled by this rung. Params parse in non-strict (the directive hasn't been read yet); promotion is the body's responsibility. The NSPS-EXT 1 work already throws on non-simple-params + strict-body; what's needed here for params is a separate re-check at body-entry that scans the params for `eval`/`arguments`. Deferred to a sibling locale.
- Existing IdentifierReference rules (e.g., `let arguments` in expression position): the binding-id check covers binding sites only; reference sites are a separate axis.

## Composes-with

- NSPS.2 sibling list.
- SMPT-EXT 2 (strict_mode parser tracking — provides the precondition).

## Resume protocol

Read `trajectory.md` tail.
