# let-as-bound-name-in-lexical — Seed

## Telos

ECMA-262 §13.3.1.1: "It is a Syntax Error if the BoundNames of LexicalDeclaration contains 'let'." Universal rule (not strict-only); also applies to ForBinding (§14.7.5). cruft silently accepts `let let = 1`, `const let = 1`, `for (let let in ...)`, `for (const let of ...)`, and the destructuring shapes `let {let} = ...`, `let [let] = ...`.

test262: `for-in/head-let-bound-names-let.js`, `head-const-bound-names-let.js`, plus the for-of counterparts (`flags: [noStrict]` — universal).

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_variable_statement` — let/const variable decls.
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` — 3 sites with let/const + binding-target.
- `pilots/rusty-js-ast/src/lib.rs::BindingPattern::collect_names` — walks pattern leaves; already used by PPAE-EXT 3 for dup-binding checks.

## Methodology

Add helper `Parser::check_no_let_bound_name(kind, &target, span) -> Result<(), ParseError>`:
- If kind is `Var`, return Ok (rule is lexical-only).
- Walk `target.collect_names()` for any name == "let".
- If found, return ParseError "Lexical declaration cannot bind name 'let'".

Call after each let/const binding-target parse at the 4 sites.

## Carve-outs

- `var let = 1` is accepted in sloppy mode (legitimate sloppy-mode behavior; spec only restricts lexical). Strict mode rejects it via SBEA-style mechanism (separate rule for `let` as ident in strict — IdentifierReference). Not covered here.
- Object-binding-pattern leaf with renamed alias `let {a: let}` is also forbidden; collect_names should surface the leaf binding-name regardless of the outer key.

## Composes-with

- NSPS.2 sibling list.
- SBEA-EXT 1 (eval/arguments strict) — same shape, different reserved name and different scope (lexical vs strict).
- PPAE-EXT 3 (dup binding check) — already uses collect_names for its walk; reuse the same iteration.

## Resume protocol

Read `trajectory.md` tail.
