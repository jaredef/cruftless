---
name: html-like-comment-lexing
description: Annex B B.1.3 SingleLineHTMLOpenComment `<!--` and SingleLineHTMLCloseComment `-->` recognized at lex tier; also spec-aligned Function-ctor source synthesis with proper newline placement.
type: project
---

# html-like-comment-lexing — Seed

## Substrate-pilot — Tier K disambiguation, new cluster spawned 2026-05-26.

Spawned per keeper directive (Telegram 9865) from the missing-syntax-feature disambiguation map. Annex B B.1.3 HTML-like comment markers cluster (11 records, lex-tier substrate sibling to LTC).

## Telos

§Annex B B.1.3 — accept `<!--` and `-->` as single-line comment markers at the lex tier:
- `SingleLineHTMLOpenComment :: <!--` (anywhere; behaves like `//`).
- `SingleLineHTMLCloseComment :: -->` (only when LineTerminator-preceded or at start-of-source; behaves like `//`).

Also: spec-align cruft's Function constructor source synthesis to place `\n` between params and `)` per ECMA-262 §20.2.1.1.1 step 13 — without this, the HTML-comment in params context would consume the closing `)`.

## Apparatus

- `pilots/rusty-js-parser/derived/src/lexer.rs::skip_trivia` — new branches for `<!--` and `-->`.
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (Function ctor synthesis) — `\n` between params and `)`.
- **Exemplar suite**: `pilots/html-like-comment-lexing/exemplars/exemplars.txt` — 11 fixtures (createdynfn-html-* + siblings).

## Baseline (FOUNDING)

PASS=0/11 at HLCL-EXT 0. All fail with `parse: unexpected token in expression: Punct(Lt|Gt)` or `parse: expected binding identifier or pattern`.

## Methodology

### HLCL-EXT 1 — lex + Function-ctor synthesis (LANDED)

Two-part edit:

1. **Lexer** (~30 LOC in skip_trivia):
   - `<!--` → consume rest of line (LF/CR/LS/PS terminators).
   - `-->` → consume rest of line, gated on `at_start || saw_line_terminator` per spec.

2. **Function ctor** (~5 LOC in intrinsics.rs):
   - Synthesized source changed from `function anonymous({params}) {\n{body}\n};` to
     `function anonymous({params}\n) {\n{body}\n};` per spec §20.2.1.1.1 step 13.
   - Without the `\n`, HTML-comment in params would swallow the closing `)`.

### Probes (Rule 23 verification at landing)

- `var x = 1; <!-- ignored\n console.log(x);` → `1` ✓
- `var x = 1;\n--> ignored\n console.log(x);` → `1` ✓
- `var x = 3; var y = 1; if (x --> y) {}` → silent (parses as `x-- > y`) ✓
- `Function("\n-->", "")` → parses (params is a comment) ✓
- `Function("-->", "")` → REJECTS (no leading LT; `-->` not a comment in this position) ✓

## Composes-with

- `pilots/line-terminator-conformance/` — sibling lex-tier locale (LT recognition); HLCL reuses `peek_lt_bytes`.
- `pilots/apparatus/runner-features-skip-deliberate-omissions/` — HLCL does NOT add Annex B B.1.3 to a SKIP-list because it's normative for web-compat hosts (cruft accepts other Annex B already).

## R13 prospective C1-C4

- C1 (sibling): HOLDS — LTC's bytewise pattern in skip_trivia.
- C2 (shape-compat): HOLDS — additive branches.
- C3 (cost-positive): HOLDS — 3-4 byte peeks per trivia loop iter.
- C4 (bail-safe): HOLDS — lex-tier only; `-->` is gated on LT-precedence so it doesn't conflict with `x-->y` (decrement-then-gt) at non-line-start.

## Status

HLCL-EXT 1 LANDED. 10/11 PASS (91%). Residual: `createdynfn-no-line-terminator-html-close-comment-params.js` expects SyntaxError, but cruft's `Function` ctor propagates inner CompileError without wrapping it as a user-catchable SyntaxError. Belongs to a separate `dynamic-function-error-wrapping` substrate move; not HLCL.
