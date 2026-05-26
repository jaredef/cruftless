# tokenizer-error-classification-refinement — Seed

## Apparatus-pilot — splits `availability/missing-parser-feature` into lex-tier + syntax-tier sub-classes.

Spawned per keeper directive (Telegram 9820) selecting B then A. Tier-J apparatus-pilot per the refreshed CANDIDATES.md. **Lands BEFORE Tier-I substrate locales** per LPA-EXT 3 Finding LPA.5 (apparatus-tier refinement precedes substrate-tier spawns).

PCR-EXT 1 named `availability/missing-parser-feature` for cruft parser-feature gaps (471 fails @ post-PCR rank 11). The class collapses BOTH lex-tier and syntax-tier parser failures into one coordinate. The apparatus §XI Lexical-grammar coordinate class wants these separable; TECR makes them so.

## Telos

Materialize the coordinate refinement at

```
apparatus/measurement :: E0/full-suite-projection ::
  cut/categorizer-rule-extension ::
  property/lex-tier-vs-syntax-tier-discrimination
```

The induced property is that the apparatus surfaces lex-tier and syntax-tier feature gaps as DIFFERENT coordinates, so substrate locales targeting one tier don't confound work on the other.

## Apparatus

- `pilots/apparatus/test262-categorize/derived/src/bin/full_pinart.rs::projection_axis` — the categorizer's projection-class rule chain (PCR-EXT 1+2 modified). TECR extends the `availability/missing-parser-feature` rule with a discriminator.

## Discriminator (already empirically verified)

A pre-implementation survey (against `test262-full-2026-05-25-165734-p2/interpreted.jsonl`) shows the discriminator is unambiguous:

| Pattern | Tier | Count |
|---|---|---:|
| Reason starts with `parse: lex error:` | lex | 61 |
| Reason contains a `LexErrorKind::*` name (Unterminated*, InvalidEscape, InvalidNumeric, etc.) | lex | (additional, subset of above) |
| Otherwise (parser syntactic error: unexpected token, expected X, HoistableDeclaration-as-Stmt, etc.) | syntax | 1,067 |

**Total split**: 61 lex-tier + 1,067 syntax-tier = 1,128 records (matches PCR's missing-parser-feature pool of 471 at rank 11 plus the syntax-tier slop spread across other ranks — the rank-11 count of 471 includes BOTH plus some downstream).

## Methodology

Single rung.

### TECR-EXT 1

Edit `projection_axis`:
- Old: `if r.starts_with("parse: ") || r.contains("parse error") { "availability/missing-parser-feature" }`
- New: `if r.starts_with("parse: lex error:") { "availability/missing-lex-feature" } else if r.starts_with("parse: ") || ...) { "availability/missing-syntax-feature" }`

The rename of `missing-parser-feature` → `missing-syntax-feature` is per discriminator. `missing-lex-feature` is the new sibling class (joining `missing-parser-feature` and `missing-lowering-feature` as the third in the apparatus's missing-X-feature family per PCR.5).

## Carve-outs

- TECR doesn't yet touch reason-texts that DON'T start with `parse:` but are actually lex-tier (e.g., a runtime-tier reason about regex pattern features). Those reach via different projection paths (regexp-semantics) and are not in scope.
- Symmetric renames for downstream consumers (positioning-gaps.md, the prospective Doc 743 reference) deferred; LPA's audit will re-render against the new matrix.

## Composes-with

- `pilots/apparatus/pinart-categorizer-refinement/` (PCR) — sibling apparatus-pilot; extends the same categorizer.
- `apparatus/docs/ecma-conformance-...md` §XI Lexical-grammar coordinate class — gives the substrate concept this categorizer enacts.
- `docs/engagement/tokenization-above-ir-candidate-brief.md` — the brief that triggered this spawn.

## R13 prospective C1-C4 at founding

- C1 (sibling): HOLDS — PCR is the empirical sibling at the same categorizer site.
- C2 (shape-compat): HOLDS — extension is purely additive to the rule chain.
- C3 (cost-positive): TBV at TECR-EXT 1; expected positive (one string-match per record).
- C4 (bail safety): HOLDS — apparatus-only edit.

All four hold. Expect 1-round closure.

## Resume protocol

Read `trajectory.md` tail.
