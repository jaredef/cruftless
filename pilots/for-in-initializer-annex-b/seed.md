---
name: for-in-initializer-annex-b
description: Annex B B.3.5 — accept `for (var X = Initializer in Expression) Statement` under sloppy mode at the parser tier.
type: project
---

# for-in-initializer-annex-b — Seed

## Substrate-pilot — Tier K disambiguation, new cluster spawned 2026-05-26.

Spawned per keeper directive (Telegram 9865) alongside HLCL from the missing-syntax-feature disambiguation map. Annex B B.3.5 carve-out cluster (~7-10 records, parser-tier).

## Telos

Annex B B.3.5 — under sloppy mode, accept the legacy form:
```
ForInOfStatement :
    for ( var BindingIdentifier Initializer in Expression ) Statement
```

cruft currently rejects with `parse: expected Semicolon` when an Initializer appears on the LHS of `for-in`. The carve-out applies only to:
- Non-strict mode.
- `var`-declared LHS (not let/const, not destructuring).
- `for-in` only (not for-of).

## Apparatus

- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` — the for-head parsing site that decides between `for-in`, `for-of`, and classic `for(;;)`.
- **Exemplar suite**: `pilots/for-in-initializer-annex-b/exemplars/exemplars.txt` — 7 fixtures (annexB/language/statements/for-in/nonstrict-initializer.js + siblings).

## Baseline (FOUNDING)

PASS=0/7 at FII-EXT 0. Verified probe: `for (var a = ++effects in {})` rejects with `parse: expected Semicolon`.

## Methodology

### FII-EXT 1 — parser carve-out (DEFERRED)

When parsing for-head and current is `var`:
- After BindingIdentifier, check if `=` follows.
- If yes, parse Initializer (AssignmentExpression).
- Then if `in` follows AND we are in sloppy mode AND LHS is plain BindingIdentifier (not pattern), accept as ForIn with Initializer.
- Otherwise classic for-stmt path (expect `;`).

The Initializer's value is bound to the var BEFORE the for-in begins iteration; this is the legacy semantics web pages depend on.

Expected LOC: ~15-20 in parse_for_statement.

## Carve-outs

- Strict mode → reject (per spec).
- `let` / `const` LHS → reject (LexicalDeclaration; not in Annex B carve-out).
- Destructuring LHS → reject (BindingPattern; not in carve-out).
- `for-of` → reject (carve-out is for-in only).

## Composes-with

- `pilots/hoistable-declaration-as-statement-body/` — sibling Annex B parser carve-out (HDSB-EXT 1).
- `apparatus/locales/CANDIDATES.md` Tier K — disambiguation map.

## R13 prospective C1-C4

- C1 (sibling): HOLDS — HDSB-EXT 1 Annex B carve-out pattern at the same parser tier.
- C2 (shape-compat): HOLDS — additive branch in for-head parsing.
- C3 (cost-positive): HOLDS — one conditional path.
- C4 (bail-safe): HOLDS — gated by sloppy + var + non-pattern + for-in.

All four hold. Expect 1-round closure.

## Status

FII-EXT 1 LANDED 2026-05-26. 6/7 on canonical Annex-B for-in/* surface. Block-rewrite lowering (hoisted var-decl + bare-name for-in) reuses existing runtime paths. Sole residual is `bare-initializer.js` (expression-headed for-in negative test); follow-on candidate `for-in-bare-assignment-rejection` left for future work.
