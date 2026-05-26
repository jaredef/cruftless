---
name: line-terminator-conformance
description: ECMA-262 §11.3 LineTerminator (LF, CR, U+2028 LS, U+2029 PS) recognition at every lex-tier site that gates on line termination.
type: project
---

# line-terminator-conformance — Seed

## Substrate-pilot — tokenization-above-IR, Tier I (LTC).

Per keeper directive (Telegram 9840) selecting the next tokenization cluster after LEP landing. Tier I substrate locale alongside NLC / IDT / SLEC.

## Telos

§11.3 LineTerminator conformance across every lex site that gates on line termination. The induced property: U+2028 (LS) and U+2029 (PS) are recognized as LineTerminator wherever LF/CR are, except inside StringLiteral bodies (where the post-2019 JSON-superset amendment permits them literally).

Coordinate:

```
tokens-to-AST / parser-form ::
  E1/lex-tier ::
  cut/line-terminator-recognition ::
  property/cruft-recognizes-LS-and-PS-as-LineTerminator-everywhere-except-string-body
```

## Apparatus

- `pilots/rusty-js-parser/derived/src/lexer.rs`
  - `Lexer::peek_lt_bytes` (new helper) — byte-length of LT at cursor.
  - `Lexer::peek_is_ident_start_strict` (new helper) — post-numeric ident-start check that excludes LT and Unicode whitespace at high-byte positions.
  - `skip_trivia` line-comment terminator — extended to LS/PS.
  - `read_numeric_literal` / `read_radix_int` post-checks — use the strict helper.
  - `read_string_escape` — \<LS> and \<PS> are LineContinuation per §12.9.4.
  - regex body LT-rejection — extended to LS/PS.
  - `is_id_start` / `is_id_continue` — exclude LS/PS and Unicode whitespace from non-ASCII permissive fallback.
- **Exemplar suite**: `pilots/line-terminator-conformance/exemplars/exemplars.txt` — 41 fixtures from `language/line-terminators/`.
- **Baseline (FOUNDING, LTC-EXT 0)**: PASS=21, FAIL=20 (51.2%) at 2026-05-25.
- **Post LTC-EXT 1**: PASS=31, FAIL=10 (75.6%).

## Founding finding shape

Per Rule 23 verification-probe, the 20 failures decompose:

| Shape | Count | Cause |
|---|---:|---|
| `expected SyntaxError, got String` | 8 | LS/PS not terminating `//` comment; LS/PS not rejected in regex body |
| `identifier directly after numeric literal` | 2 | post-numeric check treats 0xE2 (LS first byte) as ident-start |
| `y is not defined` | 8 | eval-scoping cluster (out of scope, not LT-mechanism) |
| `Test262Error to be thrown, none thrown` | 2 | follow-on of accepted LS/PS in comment/regex contexts |

## Composes-with

- `apparatus/docs/ecma-conformance-...md` §XI Lexical-grammar coordinate class.
- `pilots/string-literal-and-escape-conformance/` — sibling, shares \<LS>/\<PS> LineContinuation fix at `read_string_escape`.
- `pilots/numeric-literal-conformance/` — sibling, shares post-numeric ident-start check.
- `pilots/identifier-tokenization/` — sibling, shares `is_id_continue` separator-exclusion.

## Status

LTC-EXT 1 LANDED. 21→31 PASS on locale (+10). Cross-locale: SLEC 57→59 (+2 from \<LS> LineContinuation). NLC and IDT unchanged. diff-prod 42/42 maintained.

Residuals (10): 8 eval-scoping (non-LT-mechanism), 2 ASI cluster (cruft accepts `line comment` two-ident sequence — separate cluster).
