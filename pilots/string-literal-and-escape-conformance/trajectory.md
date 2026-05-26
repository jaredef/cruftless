# string-literal-and-escape-conformance — Trajectory

## SLEC-EXT 0 — founding + Rule-23 baseline-inspection (2026-05-25)

**Trigger**: Keeper directive (Telegram 9830) "continue to next locale." Third Tier-I tokenization substrate spawn from the brief.

**Apparatus established**:
- 73-fixture exemplar pool from `language/literals/string/`.
- Baseline: **PASS=46 FAIL=27 (63%)**.

**Rule-23 verification-probe** (post-IDT discipline refinement):

| Probe | Result | Interpretation |
|---|---|---|
| `(0,eval)('"\\u12";')` | REJECT SyntaxError | ✓ |
| `(0,eval)('"\\u{};";')` | ACCEPT (BAD) | substrate gap — empty braces should be SyntaxError |
| `(0,eval)('"\\u{110000}";')` | REJECT SyntaxError | ✓ |
| `(0,eval)('"\\x1";')` | REJECT SyntaxError | ✓ |
| `(0,eval)('"use strict"; "\\1";')` | ACCEPT (BAD) | substrate gap — strict legacy octal should reject |
| `(0,eval)('"\\400"')` produces "Ā" | wrong-result | substrate gap — \400 should be \40+"0" |
| Valid A, \x41, \n, line-continuation | OK | ✓ |

Substrate gaps surfaced cleanly via probe (no first-read mis-interpretation, per IDT.0's discipline refinement). Three categories of fix candidates: \u{} empty braces, \4-\7 3-digit cap, strict-mode legacy-octal rejection.

## SLEC-EXT 1 — three combined substrate fixes (2026-05-25)

**Edits** (~30 LOC across 2 files):

1. `pilots/rusty-js-parser/derived/src/lexer.rs::read_unicode_escape_inner` — reject `\u{}` when count == 0.

2. `pilots/rusty-js-parser/derived/src/lexer.rs::read_string_escape` b'1'..=b'7' arm — cap leading 4-7 to 2 total digits (max 1 extra; per Annex B B.1.2 LegacyOctalEscapeSequence grammar: 3-digit form restricted to leading 0-3).

3. New `Lexer::strict_mode` field + `Lexer::set_strict` + `Parser::set_lexer_strict` helper. Parser pushes strict-mode state to lexer at every flip: parse_module's "use strict" prologue detect (parser.rs:120) + parse_function_body's directive prologue detect + restore on body exit (stmt.rs:582+597). Lexer's read_string_escape gates b'0'+digit, b'1'..=b'7', b'8'|b'9' on strict_mode; strict rejects with LexErrorKind::InvalidEscape.

**Verification (probes)**:

| Probe | Pre-EXT-1 | Post-EXT-1 |
|---|---|---|
| `\u{}` empty braces | ACCEPT | **REJECT SyntaxError** ✓ |
| `\400` | "Ā" (256) | **" 0" (32 + literal "0")** ✓ (spec-correct) |
| `"use strict"; "\1"` | ACCEPT | **REJECT SyntaxError** ✓ |
| Valid escapes (\n, A, \x41, line-continuation) | preserved | preserved ✓ |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**
- SyntaxError curated cluster: **45/45 (held)**
- IDT exemplars: **261/268 (held)**

**Yield**:

| Surface | Pre-EXT-1 | Post-EXT-1 |
|---|---:|---:|
| SLEC exemplars (73-fixture pool) | 46 | **53** |
| Net Δ | — | **+7** |

**Residual fails analysis** (20 remaining):

- **Directive-prologue retro-reject cases** (~12 tests): `(function() { "\052"; "use strict"; })` — string lexed BEFORE "use strict" detected in prologue. Requires two-pass parsing or deferred-cook lex strategy. Deferred to SLEC-EXT 2.
- **Non-ASCII string handling** (~6 tests): "malformed UTF-8" — positive tests cruft incorrectly rejects. Lexer multi-byte UTF-8 mishandling inside strings. Deferred to SLEC-EXT 3.
- **Misc edge cases** (~2 tests): line-continuation variants, S7.8.4 series. Defer to per-test inspection.

**Findings**

**Finding SLEC.1 (directive-prologue retro-reject is the dominant remaining gap)**: 12 of 20 remaining SLEC fails require lex-tier behavior conditional on PARSER-tier decisions about whether `"use strict"` will follow in the directive prologue. Cruft's lexer is forward-streaming; it cooks strings as encountered. The retro-reject discipline requires either (a) two-pass parsing (parse prologue first to determine strict, then re-lex strings) or (b) deferred cooking (lex strings as raw, cook at use-time). Both are substantial substrate moves. Standing recommendation: lex-tier behaviors that depend on parser-tier decisions about siblings-not-yet-seen create a directionality conflict; the cleanest resolution is deferred cooking (option b), which decouples lex from parse-tier-strict-state. Pattern is similar to LGSS's lexer↔parser feedback edge but at a different surface; could compose against §XI.1.b's articulation.

**Finding SLEC.2 (lexer strict-mode threading is small but consequential)**: ~10 LOC of plumbing (Lexer field + setter + 2 Parser push-sites + 1 helper) unblocked ~7 test262 tests + made cruft's lex tier spec-conformant on legacy-octal rejection. Standing recommendation: lex-tier state that mirrors parser-tier state should be threaded via single helper rather than implicit-channel; cleanly mirrors LGSS's current_lex_goal pattern at a different surface.

**Status**: SLEC-EXT 1 CLOSED. 53/73 (72.6% pool pass; was 63%). SLEC-EXT 2 (directive-prologue retro-reject) deferred; SLEC-EXT 3 (non-ASCII string handling) deferred. Locale stays open; current substrate work is the highest-yield-per-LOC subset of the §12.9 conformance surface.
