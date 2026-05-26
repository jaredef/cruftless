---
name: with-body-multi-statement-parse
description: §14.11 WithStatement — fix parse-tier bug in skip_to_top_terminator that breaks on line-terminator-preceded `}` closing the with-body, leaving `}` as stray lookahead.
type: project
---

# with-body-multi-statement-parse — Seed

## Substrate-pilot — Tier K, missing-syntax-feature concentration (WBMS).

Spawned per keeper directive (Telegram 9855) selecting Cluster B from the TECR-EXT 2 lift surface. Sub-cluster of Punct(RBrace) failure shape (264 of 323 records).

## Telos

§14.11 WithStatement parser — accept body that contains line-terminator-preceded close-brace. `with(p){x;}` parses correctly; `with(p){\n x \n}` previously failed with `parse: unexpected token in expression: Punct(RBrace)` because the parse-tier helper `skip_to_top_terminator` broke via its ASI fallback on the `}` BEFORE bumping it.

Note: WBMS-EXT 1 was purely parser-tier. WBMS-EXT 2 promotes `with` out of
`Stmt::Opaque` into a typed AST node plus bytecode/runtime support for sloppy
object-environment lookup and identifier assignment.

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::skip_to_top_terminator` — the parse-tier helper used by the `with` branch in `parse_statement`.
- **Exemplar suite**: `pilots/with-body-multi-statement-parse/exemplars/exemplars.txt` — 264 fixtures (all Punct(RBrace) records).

## Baseline (FOUNDING)

PASS=0/264 at WBMS-EXT 0 (all 264 records fail with the parse error). Sub-distribution:
- 130 language/statements/with/
- ~30 in Proxy/has + sm/lexical-environment
- 44 in language/expressions/compound-assignment/
- ~60 in other dirs (mostly with-statement adjacent or RBrace-as-recovery-token patterns)

## Methodology

### WBMS-EXT 1 — close brace-bodied stub on depth-drop (LANDED)

When `}` decrements `depth_brace` to 0 inside `skip_to_top_terminator` AND paren/bracket depths are also 0, the brace-bodied statement is complete. Bump the `}` and return immediately rather than falling through to the ASI fallback, which would break BEFORE bumping (because the `}` itself is LT-preceded when the body has a line terminator before the close).

Edit: ~12 LOC in stmt.rs.

### WBMS-EXT 2 — with-runtime-semantics (LANDED)

The 227 residuals from WBMS-EXT 1 were tests that exercise real `with`
semantics: scope-chain extension, property-shadowing, `with`-bound variable
visibility, and assignment PutValue against an object environment record.

Landed substrate:
- `Stmt::With { object, body }`
- bytecode `EnterWith` / `ExitWith`
- dynamic `LoadWithName` / `StoreWithName`
- reference-preserving `ResolveWithName`, `LoadWithNameRef`, and
  `StoreWithNameRef` for assignment and compound-assignment sites where the
  RHS/getter deletes the originally-resolved property before PutValue.

Yield: 37/264 -> 73/264 PASS. Residuals are now dominated by adjacent deeper
surfaces: global-this/global-object aliasing, `@@unscopables`, Proxy `has`
trap integration, direct/indirect eval environment records, call-base `this`
binding, destructuring target evaluation order, and abrupt-completion cleanup.

## Composes-with

- `pilots/hoistable-declaration-as-statement-body/` — sibling Tier K substrate-pilot landed just before this.
- `apparatus/locales/CANDIDATES.md` Tier K — other ripe clusters (IMM, DIA, CAR).

## R13 prospective C1-C4

- C1 (sibling): HOLDS — `skip_to_top_terminator`'s semicolon-branch already does bump-and-return on depth-0 completion.
- C2 (shape-compat): HOLDS — additive branch in the RBrace match arm.
- C3 (cost-positive): HOLDS — single comparison + bump + return.
- C4 (bail-safe): HOLDS — only fires when all depths drop to 0 simultaneously; identical structurally to the semicolon-branch.

## Status

WBMS-EXT 2 LANDED. 73/264 (27.7%) direct yield. Next rung should choose among
the residual families above, with `@@unscopables` / Proxy `has` integration and
global-this binding as the most obvious cross-cutting closures.
