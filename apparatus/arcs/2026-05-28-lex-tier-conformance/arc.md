---
arc: 2026-05-28-lex-tier-conformance
trigger: Plan agent's back-fit analysis 2026-05-28 (keeper directive Telegram 10158); empirical: Tier-I tokenization-above-IR brief (docs/engagement/tokenization-above-ir-candidate-brief.md, 2026-05-25)
opened: 2026-05-28
closed: IN PROGRESS
close_condition: ECMA-262 §11-§12 production coverage measured against the seven enrolled locales' exemplar suites; TECR projection class "lex-tier missing-X-feature" empty; cross-locale findings on bit-flag preservation across the lex/parse boundary recorded.
---

# Lex-Tier Conformance Arc

## Trigger

Plan agent's back-fit (2026-05-28, per keeper Telegram 10158) identified seven top-level locales sharing the lex-tier mouth-terminus: raw source bytes per §11-§12 productions → token stream with preserved bit-flags (had-escape, raw/cooked, line-terminator, private-ident). Empirically anchored in the Tier-I tokenization-above-IR candidate brief (2026-05-25) + subsequent per-coordinate locale foundings + the NLC arc's correction (Addendum XV) that surfaced the lex-tier-vs-IR-tier scope distinction.

## Telos

Subsume the seven lex-tier locales under one arc with explicit (M, T, I, R) per Doc 744. The arc-tier mouth is "raw source bytes per ECMA-262 §11-§12 productions"; the arc-tier terminus is "token stream with preserved bit-flags (had-escape, raw/cooked, line-terminator, private-ident) per spec"; the arc-tier interior is the lex-goal selection + reserved-word + escape-decode + private-ident + numeric-literal sub-pipelines; the arc-tier relations: alphabet-exchange ↓ parser arc (the parser's mouth IS this arc's terminus).

## Sub-locale roster

| Locale | Role in arc | Status pre-arc |
|---|---|---|
| `identifier-tokenization` | reserved-word vs identifier disambiguation | LANDED at IT-EXT 2 (115 fails → cluster close per Addendum XV) |
| `numeric-literal-conformance` | numeric-literal grammar conformance | LANDED |
| `string-literal-and-escape-conformance` | string-escape decoding + bit-flag preservation | LANDED |
| `private-name-lexing` | `#name` lex per §11.6 | LANDED |
| `line-terminator-conformance` | LT/LS line-terminator handling | LANDED |
| `lexer-goal-symbol-selection` (LGSS) | lex-goal (RegExp vs Div) selection | LANDED (canonical chain spinoff) |
| `html-like-comment-lexing` | Annex B HTML-like comment lexing | LANDED |

## Cumulative yield

Pre-arc: per-locale closures spanning Addendum XV (NLC arc correction + IT-EXT 2 closure of 115 fails) + Tier-I brief execution. Aggregated yield rendered as engagement-wide rate movement.

## Cross-arc relations

- **Alphabet-exchange ↓ `2026-05-28-parser-early-error-conformance`**: this arc's terminus = parser arc's mouth. Strict alphabet-exchange contract: token-bit-flag preservation across the boundary.
- **Lattice with `2026-05-28-annex-b-language-partition` (proposed)**: html-like-comment-lexing is cross-listed (Annex B web-legacy semantics at lex tier).

## Cross-locale findings

To be promoted. Initial sketch:

**Finding LTC.1 (pending)**: bit-flag preservation across the lex/parse boundary is the arc's load-bearing contract. Tokens carry had-escape / raw / cooked / line-terminator / is-private bits; the parser consumes via alphabet-exchange. Loss of any bit at the boundary surfaces as a parser arc residual whose root cause is in this arc. Standing rec: when a parser-tier locale's baseline-inspection (Phase 2) surfaces a "should-throw early error didn't throw" residual, audit the token-bit set first.

## Status

IN PROGRESS — scaffolded 2026-05-28 per keeper directive 10158. Per the Plan agent's recommendation, this is the fourth new arc to be back-fit (canonical example of an arc spawned by apparatus-projection-class — TECR's matrix coordinate refinement). Strong cross-arc DAG-relation with the parser arc.
