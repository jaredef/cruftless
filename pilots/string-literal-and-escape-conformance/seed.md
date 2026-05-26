# string-literal-and-escape-conformance — Seed

## Substrate-pilot — third tokenization-above-IR spawn from the brief.

Per keeper directive (Telegram 9830) "continue to next locale." Tier-I substrate locale per refreshed CANDIDATES.md ((rr)). Spawned after IDT closed coverage-complete (IDT-EXT 1+2).

## Telos

§12.9 StringLiteral + Annex B B.1.2 LegacyOctalEscapeSequence conformance. Coordinate:

```
tokens-to-AST / lex-tier ::
  E1/lex-tier ::
  cut/string-escape-form-rejection ::
  property/cruft-rejects-malformed-string-escapes-with-SyntaxError-class
```

Several substrate gaps surfaced via Rule-23 baseline-inspection probe:
- `\u{}` empty-braces acceptance (spec: SyntaxError; CodePoint requires ≥1 HexDigit per §12.9.4)
- `\4`-`\7` 3-digit form acceptance (`\400` → 256 "Ā"; spec says \40 + literal "0")
- Strict-mode legacy-octal-escape acceptance (`\1`-`\7` and `\0`+digit in strict; spec rejects)
- Strict-mode NonOctalDecimalEscape acceptance (`\8`, `\9` in strict; spec rejects)
- Some positive tests fail with "malformed UTF-8" — cruft lexer mis-handles non-ASCII inside strings (separate concern, deferred)
- 1 `\400` wrong-result test confirms the 3-digit form issue

## Apparatus

- `pilots/rusty-js-parser/derived/src/lexer.rs::read_string_escape` (line 632+) — main escape decode site.
- `pilots/rusty-js-parser/derived/src/lexer.rs::read_unicode_escape_inner` (line 315+) — \u{...} CodePoint decoder.
- `pilots/rusty-js-parser/derived/src/lexer.rs::Lexer.strict_mode` (added SLEC-EXT 1) — lexer-side strict-mode flag for legacy-escape rejection.
- **Exemplar suite**: `pilots/string-literal-and-escape-conformance/exemplars/exemplars.txt` — 73 fixtures from `language/literals/string/`.
- **Baseline (FOUNDING)**: PASS=46 FAIL=27 (63%).

## Methodology

### SLEC-EXT 1 — \u{} empty braces + \4-\7 cap + lexer strict-mode threading

Three substrate moves combined into one rung:
1. read_unicode_escape_inner: reject `\u{}` (count == 0).
2. read_string_escape b'1'..=b'7': cap 4-7 leading to 2 total digits (max 1 extra).
3. New Lexer::strict_mode + Parser::set_lexer_strict; rejects \1-\7 + \0+digit + \8/\9 in strict.

### SLEC-EXT 2 — directive-prologue retro-reject (multi-rung; deferred)

Tests like `(function() { "\052"; "use strict"; })` require the lexer to retroactively reject string literals lexed BEFORE "use strict" is detected. Requires either:
(a) Two-pass parsing (parse all directive prologue first, determine strict mode, then re-lex strings), or
(b) Lex strings into cooked-pending state; cook at use-time when strict mode is known.

Substantial substrate move; deferred until keeper directs.

### SLEC-EXT 3 — non-ASCII string handling

The 6 "malformed UTF-8 (UnterminatedString)" baseline fails are positive tests where cruft lexer incorrectly rejects valid sources. Likely cruft mis-handles multi-byte UTF-8 inside strings. Per-test investigation needed.

## Carve-outs

- The directive-prologue retro-reject case is a known substrate complexity; SLEC-EXT 2 explicit-pragma test gap.
- Non-ASCII string handling is its own substrate concern (separate from escape decoding).
- Template literal escapes have separate rules (§14.6) — sibling locale candidate.

## Composes-with

- `pilots/identifier-tokenization/` (IDT) — sibling Tier-I locale; same Rule-23 baseline-inspection discipline applied.
- `pilots/numeric-literal-conformance/` (NLC) — sibling; same eval-class-wrapping mechanism IDT.2 confirmed.
- `pilots/apparatus/tokenizer-error-classification-refinement/` (TECR) — apparatus-pilot whose `availability/missing-lex-feature` class names this work's coordinate.
- `apparatus/docs/predictive-ruleset.md` Rule 23 — Rule-23 baseline-inspection produced clean substrate scope at founding.

## Resume protocol

Read `trajectory.md` tail.

## Status

SLEC-EXT 1 CLOSED. 47 → 53/73 (+7) per SLEC-EXT 1 substrate moves combined. SLEC-EXT 2 (directive-prologue retro-reject) deferred for substantial-scope review.
