# statement-declaration-in-body-position — Seed

**Locale tag**: `L.statement-declaration-in-body-position` (top-level)

**Status**: **CLOSED at SDIBP-EXT 1**.

**Workstream**: ECMA-262 §13.1 / §14.1.1 — Statement grammar excludes Declaration (HoistableDeclaration, ClassDeclaration, LexicalDeclaration). The body positions of for / for-in / for-of / if / else / while / do-while require Statement, not Declaration. Cruft's parse_statement accepts Declaration in every position; the body sites for control-flow needed a Statement-only entry.

**Trigger**: T262C cluster (was #9 pre-PPA, retained post-cascade) — `for-{in,of}/decl-*.js`, 14 fixtures testing `for (var x of [])  class C {}` style forbidden patterns.

**Composes with**:
- ECMA-262 §13.1 Statement grammar; §14.1.1 Static Semantics: Early Errors
- [PPA-EXT 1](../parser-permissiveness-audit/trajectory.md) — same engagement-tier surface (parser permissiveness), adjacent fixes

## I. Telos

Add a `parse_substatement()` entry that rejects the obvious Declaration tokens (let, const, class, function, async function) before delegating to parse_statement. Wire at every body-of-control-flow position in stmt.rs.

## II. Apparatus + Methodology

- `pilots/rusty-js-parser/derived/src/stmt.rs`: new pub fn `parse_substatement()` + replacements at for/for-in/for-of/if/else/while/do-while/labelled body sites.

Exemplar verification:
- All 14 for-{in,of}/decl-* tests PASS (was 0).
- 0 regressions across previously-passing for-of (95), for-in (67), if (0 baseline), while (0 baseline), do-while (0 baseline).

## III. Carve-outs

- Annex B B.3.2 allows plain function-decl as if-body in sloppy non-strict mode. SDIBP rejects strictly per spec; if Annex B compat becomes needed, gate by sloppy-mode + plain-function-only.
- `let` followed by `.` / `(` / `=` / `;` / EOF / `\n` is identifier-reference, NOT lexical-declaration. SDIBP only rejects when followed by binding-starting token ([ or { or alphabetic-ident).
- Labelled-statement body has a slight carve-out for plain FunctionDeclaration per §13.13; SDIBP rejects strictly here too.

## IV. Status

CLOSED at SDIBP-EXT 1.
