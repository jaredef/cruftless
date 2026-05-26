---
name: dynamic-import-attributes
description: §13.3.10 ImportCall — accept the stage-4 import-attributes second-arg form `import(specifier, { with: { type: 'json' } })`.
type: project
---

# dynamic-import-attributes — Seed

## Substrate-pilot — Tier K, missing-syntax-feature concentration (DIA).

Spawned per keeper directive (Telegram 9859) selecting Cluster D from the TECR-EXT 2 lift surface. Stage-4 import-attributes is in ECMA-262 (ratified) — substrate work, not apparatus SKIP.

## Telos

§13.3.10 ImportCall parser accepts the import-attributes grammar:
```
ImportCall :
    import ( AssignmentExpression , ? )
    import ( AssignmentExpression , AssignmentExpression , ? )
```

cruft currently bails at the comma with `parse: expected ')' in dynamic import()`. DIA extends the ImportCall parser to consume an optional second AssignmentExpression and optional trailing commas, then preserves both args in the AST.

Runtime semantics: `__dynamic_import` is already a throwing stub in cruft. The second-arg attributes are stored in the AST so a future runtime can read them; no runtime change in DIA-EXT 1.

## Apparatus

- `pilots/rusty-js-parser/derived/src/expr.rs` — the `"import"` keyword branch's dynamic-import parse path.
- **Exemplar suite**: `pilots/dynamic-import-attributes/exemplars/exemplars.txt` — 41 fixtures from `language/expressions/dynamic-import/import-attributes/`.

## Baseline (FOUNDING)

PASS=0/41 at DIA-EXT 0 (all 41 reject at `,` byte offset).

## Methodology

### DIA-EXT 1 — parser extension (LANDED)

After parsing the first AssignmentExpression, if the next token is `,`:
- Bump the comma.
- If the next token is `)`, treat as trailing comma (single-arg form).
- Else parse the second AssignmentExpression, then accept optional trailing comma.

Then expect `)` as before. AST stores both args; runtime semantics unchanged.

Net LOC: ~18 in expr.rs.

### Probes (Rule 23 verification at landing)

- `import('x')` → parses ✓
- `import('x', { with: { type: 'json' } })` → parses ✓
- `import('x',)` → parses ✓ (trailing comma single-arg)
- `import('x', {with:{type:'json'}},)` → parses ✓ (trailing comma two-arg)
- `import('x', {with:{type:'json'}}, extra)` → REJECTS ✓ (arity-3 not allowed)

## Composes-with

- `pilots/apparatus/runner-features-skip-deliberate-omissions/` — DIA does NOT add `import-attributes` to the SKIP-list because the feature is stage-4 and cruft implements its syntax.
- `apparatus/locales/CANDIDATES.md` Tier K — other clusters in the missing-syntax-feature concentration.

## R13 prospective C1-C4

- C1 (sibling): HOLDS — existing argument-list parsing in CallExpression.
- C2 (shape-compat): HOLDS — additive grammar extension at one parse site.
- C3 (cost-positive): HOLDS — O(1) per ImportCall.
- C4 (bail-safe): HOLDS — parser-only; runtime stub unchanged.

## Status

DIA-EXT 1 LANDED. 40/41 PASS (97.6%). The single residual is a runtime evaluation-sequence semantic test (`Expected SameValue 1 vs 0`) that cruft's __dynamic_import stub doesn't satisfy — runtime-tier residual; not parser-tier.
