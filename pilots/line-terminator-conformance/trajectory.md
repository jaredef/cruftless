# line-terminator-conformance — Trajectory

## LTC-EXT 0 — FOUNDING (2026-05-25)

Baseline against `language/line-terminators/` (41 fixtures): PASS=21, FAIL=20.

Rule 23 verification-probe on six representative fails:
- `invalid-regexp-ps.js`, `invalid-comment-single-ls.js`, `invalid-comment-single-cr.js` — `expected SyntaxError, got String`
- `S7.3_A7_T4.js`, `S7.3_A7_T8.js` — `y is not defined`
- `between-tokens-ls.js` — `identifier directly after numeric literal`

Root cause: cruft's lex tier recognizes only ASCII LF/CR as LineTerminator. ECMA-262 §11.3 requires LF, CR, U+2028 (LS), U+2029 (PS). Single substrate-mechanism shape affecting multiple lex sites.

## LTC-EXT 1 — LANDED (2026-05-25)

Substrate edits to `pilots/rusty-js-parser/derived/src/lexer.rs`:

1. New helper `Lexer::peek_lt_bytes` — returns byte-length of LineTerminator at cursor (1/2/3 for LF/CRLF/LS-PS) or None.
2. New helper `Lexer::peek_is_ident_start_strict` — post-numeric ident-start check that excludes LS/PS/Unicode-whitespace at high bytes.
3. `skip_trivia` single-line comment terminator loop — extended to break on LS/PS.
4. `read_numeric_literal` post-check (site ~537) — uses the strict helper.
5. `read_radix_int` post-check (site ~608) — adds `high_lt_or_ws` carve-out.
6. `read_string_escape` — `\<LS>` and `\<PS>` recognized as LineContinuation per §12.9.4 (consume 3 bytes, contribute nothing).
7. Regex body LT-rejection (sites ~956, ~967) — extended to LS/PS.
8. `is_id_start` / `is_id_continue` non-ASCII fallback — exclude 0x2028, 0x2029, and any `is_unicode_whitespace` codepoint.

Net LOC: ~50 lines added across 8 sites.

### Yield

- **line-terminator-conformance**: 21 → 31 PASS (+10).
- **string-literal-and-escape-conformance**: 57 → 59 PASS (+2, from \<LS>/\<PS> LineContinuation closing `line-continuation-double.js` etc.).
- **numeric-literal-conformance**: 147 unchanged.
- **identifier-tokenization**: 261 unchanged.
- **diff-prod**: 42/42 maintained.

### Rule 23 verification-probe at substrate-landing

Initial cut rejected literal U+2028/U+2029 inside StringLiteral bodies, regressing SLEC 57 → 55 on `language/literals/string/{line,paragraph}-separator{,-eval}.js`. Probe of test source surfaced ECMA-262's post-2019 JSON-superset amendment (§12.9.4.1): LS/PS are permitted literally inside string bodies; only LF/CR remain forbidden. Reverted the string-body site; corrected predicate now: "LS/PS are LineTerminator everywhere except inside StringLiteral bodies."

This pattern (Rule-23 verification at substrate-landing time per LEP.2) caught the over-application before it propagated.

### Residuals (10)

- 8 `y is not defined` on `S7.3_A7_T*` — eval-scoping mechanism (not LT-tier; sibling cluster).
- 2 `expected SyntaxError, got String` on `invalid-comment-single-{lf,cr}.js` — cruft incorrectly ASIs between two consecutive identifiers on the same line (`line comment`). Belongs to an ASI cluster, not LT.

### Cross-tier composition

The `is_id_continue` separator-exclusion is the deepest fix — it changes identifier-scanning everywhere, not just at post-numeric sites. Identifier-tokenization unchanged at 261 confirms no regression on legitimate non-ASCII identifiers. The change closes the latent bug where `var<LS>x` was scanned as a single identifier `var\u{2028}x`.
