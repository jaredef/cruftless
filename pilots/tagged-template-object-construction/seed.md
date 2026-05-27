# tagged-template-object-construction — Seed

## Substrate-pilot — compiler+runtime locale for template call-site object construction.

Per keeper directive 2026-05-27, spawned from the LPA-EXT 9 lexer substrate gap analysis (gap L.2). The lexer correctly produces both `cooked` and `raw` fields on template tokens; the gap is in the compiler and runtime.

## Telos

Construct the frozen template call-site object per ECMA-262 §13.2.8.3 (GetTemplateObject). When a tagged template is invoked, the first argument is a frozen array of cooked strings with a frozen `.raw` property containing the raw source strings. The object is cached per call-site (same tagged template expression yields the same object identity on repeated calls).

## Current state

The compiler lowers `TemplateLiteral` to a left-to-right `Add` chain (`compiler.rs:3871–3897`), using only the `cooked` values from the template quasis. The `raw` values from `TokenKind::Template { raw, .. }` are parsed into the AST but the compiler does not:
1. Emit the raw strings into the template call-site object
2. Freeze the template object or its `.raw` property
3. Cache the template object per call-site

Observable deviations:
- `strings.raw` is `undefined` in tag function calls
- `String.raw` (built-in tag that reads `.raw`) returns wrong output
- Template object is not frozen
- Template object identity differs across calls to the same tagged template

## Constraints

- `compiler.rs:3871–3897` — TemplateLiteral compilation (currently Add chain only)
- `token.rs:41–45` — `TokenKind::Template { cooked, raw, part }` (raw IS available)
- AST: `Expr::TemplateLiteral { quasis, expressions }` — quasis carry only cooked; raw must be threaded from token to AST
- Runtime: needs a `__createTemplateObject(cooked[], raw[])` helper or a dedicated opcode

## Methodology

1. **Rung 1**: Thread `raw` quasis from the lexer's `TokenKind::Template` through the parser into the AST node. Currently `quasis` in the AST are `Vec<Box<str>>` (cooked only); extend to pairs or add a parallel `raw_quasis` field.
2. **Rung 2**: In the compiler, emit a call to a runtime helper `__getTemplateObject(cooked[], raw[])` that constructs and caches the frozen template object.
3. **Rung 3**: In the runtime, implement the helper: create a frozen Array of cooked strings, attach a frozen `.raw` Array of raw strings, cache by call-site identity.

## Composes-with

- `string-literal-and-escape-conformance/` — escape sequences in template raw vs cooked
- `es-recent-methods` fixture — String.raw depends on this

## Carve-outs

- Tagged template with invalid escapes (cooked=None per spec): deferred; the lexer already handles this (`cooked: Option<String>`)

## Resume protocol

Read this seed, then `trajectory.md` tail.

## Diff-prod anchors

| Fixture | Status | Connection |
|---|---|---|
| `tagged-template-raw` | FAIL | `strings.raw` is null; entire tagged template protocol broken |
| `string-escapes` | FAIL | String.raw returns cooked instead of raw |
| `template-literals` | PASS | Untagged templates work (Add chain is correct for those) |
