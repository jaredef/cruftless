# class-elements-static-semantics — Seed

## Substrate-pilot — Rule-23 redirect from private-name-lexing.

Spawned 2026-05-26 after `private-name-lexing` (PNL) found direct PrivateIdentifier lexing healthy. PNL-EXT 1 closed the remaining small lex/private-name early-error slice and left parse-phase failures that live at class-elements static semantics.

## Telos

Class element parse-time static semantics for private names and field initializers. Coordinate:

```
tokens-to-AST / parser-static-semantics ::
  E2/class-elements ::
  cut/private-name-and-field-initializer-early-errors ::
  property/cruft-rejects-invalid-class-elements-before-evaluation
```

Immediate surfaced cases:

- `arguments` inside class field initializers, including nested arrow initializers.
- Undeclared private names referenced inside computed property names.

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_class_body` — parses the complete class body and now has the full member list needed for class-element static validation.
- `rusty_js_ast::{ClassMember, ClassMemberName, Expr, MemberProperty}` — existing AST substrate carries member names, computed names, and initializer expressions.
- PNL focused probe: generated path list of 194 private-name / privatename class-elements fixtures.

## Methodology

### CESS-EXT 1 — narrow static validator

Add a post-parse class-body validator:

1. Collect private bound names from the whole class body.
2. Reject field initializers whose expression tree ContainsArguments.
3. Reject computed class-member names that reference undeclared private names.

Expected yield is small but precise: the 6 remaining `expected SyntaxError, got String` rows in PNL's focused probe should flip. Runtime private-brand semantics, async harness SKIPs, and generator iterator runtime gaps remain out of scope.

## Carve-outs

- Private brand runtime semantics are not parser static semantics.
- Async-flag harness skips are apparatus/harness scope.
- Generator method runtime failures (`method='next'` on Number) are not class-element static semantics.
- Duplicate private-name semantics may become a follow-on if the `test 3` assertion rows inspect to that shape, but CESS-EXT 1 does not claim them.

## Composes-with

- `pilots/private-name-lexing/` — Rule-23 parent signal and focused exemplar source.
- `apparatus/docs/predictive-ruleset.md` Rule 23 — locale-as-probe discipline; PNL surfaced this coordinate.
- `pilots/parser-early-error-residual/block-bound-names-dup/` — sibling parser-static early-error shape.

## Resume protocol

Read `trajectory.md` tail. Rebuild `cruft`, run PNL's focused 194-path probe, and inspect the residual failure families. After CESS-EXT 1 the focused parse-phase `expected SyntaxError, got String` rows from PNL-EXT 1 are closed.

## Status

CESS-EXT 1 landed locally. Parser validator compiles; release binary keeps direct PNL at `40/40` and moves the focused PNL list from `134/194` to `140/194`.
