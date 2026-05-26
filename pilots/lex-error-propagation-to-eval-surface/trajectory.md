# lex-error-propagation-to-eval-surface — Trajectory

## LEP-EXT 0 — founding (2026-05-25)

**Trigger**: Keeper directive (Telegram 9834) "spawn the substrate locale." Per Finding NLC.3 (NLC trajectory): the lex-tier strict-mode legacy-octal rejection rule fires correctly (eprintln-verified) but its Err return doesn't surface at test262's `(0,eval)` catch. A swallow site exists somewhere in cruft's parser/eval pipeline.

**Empirical anchor** (verified via direct probes during NLC-EXT 1-revised):

```
$ echo '0b2;' | cruft /dev/stdin
cruft: evaluation error: SyntaxError("parse: lex error: invalid radix-prefixed literal ...")
exit=70                            # ✓ direct script-mode error surfaces

$ echo '"use strict"; 00;' | cruft /dev/stdin
                                   # silent, exit=0 — error swallowed somewhere
exit=0                              # ✗ multi-statement source with strict-mode lex rejection: error silent

$ cat > /tmp/f.js <<EOF
function f() { "use strict"; 00; }
console.log("compile-and-run ok");
EOF
$ cruft /tmp/f.js
compile-and-run ok
exit=0                              # ✗ function-body strict-mode lex rejection also silent
```

eprintln debug at the legacy-octal check confirmed: `Lexer::strict_mode == true`, the `return Err(...)` IS executed. The Err does NOT reach `cruftless/src/main.rs:284`'s "cruft: evaluation error:" branch.

**Apparatus established**:

- Substrate sites named in seed (cruftless/src/main.rs, rusty-js-runtime/derived/src/module.rs:920+1451, rusty-js-parser/derived/src/parser.rs::parse_module + parse_statement, stmt.rs::Stmt::Opaque fallback).
- LEP-EXT 1 methodology: binary-search the swallow site via eprintln at each `?` propagation point + each `match { Err => ... }` arm.

**Status**: LEP-EXT 0 FOUNDED. LEP-EXT 1 is the immediate substrate move; the binary-search apparatus is documented. Scope ~10-30 LOC depending on where the swallow site is.

---

## LEP-EXT 1 — locate + close the consume_semicolon swallow site (2026-05-25)

**Trigger**: Keeper directive (Telegram 9836) "continue, land it."

**Discovery** (~5 minutes of pin-pointing): inspecting parse_module's loop showed no try/catch around parse_statement; the `?` propagates. Inspecting parse_statement's expression-statement branch (`let expr = self.parse_expression()?; self.consume_semicolon_pub();`) found:

```rust
fn consume_semicolon(&mut self) {
    if self.is_punct(Punct::Semicolon) {
        let _ = self.bump_regexp();   // <-- SWALLOWS Err
    }
}
```

The `let _ = self.bump_regexp();` discards bump_regexp's `Result<Token, ParseError>`. bump_regexp's internal `mem::replace(&mut self.lookahead, lex.next_token(...).map_err(lex_to_parse)?)` correctly propagates Err from lex, but consume_semicolon's `let _` throws it away. The lookahead stays at the OLD `;` token; the function returns `()`; parse_module's loop iterates with stale lookahead; eventually hits some other path that succeeds or returns vacuously.

**Edits** (~15 LOC across 2 files):

1. `pilots/rusty-js-parser/derived/src/parser.rs::consume_semicolon` — signature change to `Result<(), ParseError>`; `let _ = bump_regexp()` → `bump_regexp()?`; explicit `Ok(())` at end.
2. `pilots/rusty-js-parser/derived/src/parser.rs::consume_semicolon_pub` — wrapper signature change to `Result<(), ParseError>`.
3. **12 call sites updated** via sed: 6 in parser.rs + 6 in stmt.rs — `self.consume_semicolon();` → `self.consume_semicolon()?;`; same for `_pub`.

**Verification (probes)**:

| Probe | Pre-LEP-EXT 1 | Post-LEP-EXT 1 |
|---|---|---|
| `"use strict"; 00;` direct script | exit=0 silent | **exit=70 "legacy octal/non-octal-decimal integer literal in strict mode"** ✓ |
| `function f(){ "use strict"; 00; }` | exit=0 "compile-and-run ok" | **exit=70 same error** ✓ |
| `0b2;` (control) | exit=70 ✓ | exit=70 ✓ (preserved) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**
- SyntaxError curated cluster: **45/45 (held)**

**Engagement-wide yield**:

| Locale | Pre-LEP | Post-LEP | Δ |
|---|---:|---:|---:|
| NLC (numeric-literal-conformance) | 104/157 | **136/157** | **+32 (+20 pp)** |
| SLEC (string-literal-and-escape-conformance) | 53/73 | **57/73** | **+4** |
| IDT (identifier-tokenization) | 261/268 | 261/268 | 0 (IDT residuals are different sub-shape) |
| SyntaxError curated cluster | 45/45 | 45/45 | held |

**Total: +36 test262 tests closed by one ~15-LOC propagation fix.** This was the prediction at LEP-EXT 0 founding ("engagement-wide reach") empirically realized.

**Findings**

**Finding LEP.1 (the swallow site was `let _ = bump_regexp()`)**: a single occurrence of `let _ =` discarding a Result<_, ParseError> at consume_semicolon swallowed every lex-tier error that fired after the immediately-prior `;`. The discipline `clippy::needless_return` would catch this; cruft's parser code doesn't currently lint for swallowed Results. Standing recommendation: add a CI-tier check (clippy or grep) that flags `let _ = .*\?` and `let _ = .*Result` patterns in the parser crate; these are almost always silent error swallows.

**Finding LEP.2 (Rule-23 verification-probe at SUBSTRATE LANDING TIME is what caught this)**: NLC-EXT 1-revised landed the lex-tier check + saw no test262 yield (104/157 unchanged). The verification-probe (eprintln) confirmed the check fires but Err doesn't propagate; that surfaced LEP.0. Without the probe, NLC-EXT 1's "no yield" reading would have led to either chasing wrong substrate moves or abandoning the rung. The probe surfaced the meta-substrate gap. Refinement of Rule 23: when a substrate move lands but doesn't move the predicted yield, the immediate next step is verification-probe at each tier-crossing the move depends on. The probe is the discipline at landing-time as well as founding-time.

**Finding LEP.3 (engagement-wide propagation fix is the highest-yield-per-LOC of today's session)**: ~15 LOC at one site closed 36 test262 tests across 2 sibling locales. The yield-per-LOC ratio is ~2.4. This exceeds BBND's prior champion (~0.68 in raw / ~7x tests-per-locale). The conditions are different (LEP is a cross-locale propagation fix, not a single-spec-rule cluster) but the structural insight is the same: when a substrate gap blocks multiple locales, fixing it once delivers their cumulative yield. Standing recommendation: when a Rule-23 probe surfaces a swallow/propagation gap, prioritize the cross-locale fix over per-locale substrate work — the leverage compounds.

**Status**: LEP-EXT 1 CLOSED. The swallow site is closed; lex errors propagate correctly from consume_semicolon paths to the eval surface. LEP-EXT 2 (additional swallow sites if any) and LEP-EXT 3 (engagement-wide re-baseline after subsequent substrate work) remain open as opportunistic rungs. Locale's primary purpose achieved.

---

## LEP-EXT 2 — lex-tier substrate moves enabled by EXT 1 propagation (2026-05-25)

**Trigger**: Keeper directive (Telegram 9838) "continue as coherent."

**Pre-EXT-2 audit per LEP.1 standing rec**: grep for additional `let _ = .*` swallow patterns across parser/stmt/expr/lexer crates — **zero additional swallows found**. LEP-EXT 1's consume_semicolon was the only such site in the parser crate. Standing recommendation discharged.

**Pivot**: with the swallow site closed, NLC residual fails are now visible as actual substrate gaps (not propagation losses). Re-survey of NLC residuals identified two clean lex-tier substrate moves:

### LEP-EXT 2.a — invalid character after radix-prefixed literal

`0b14`, `0o89`, `0b1abc` (etc.) — cruft's read_radix_int loop broke silently on the first invalid byte; the parser then saw `0b1 4` as two consecutive expression tokens. After LEP-EXT 1 those slipped through as parse errors via ASI, but the underlying lex was permissive. Spec §12.8.3 requires SyntaxError when a numeric literal ends with a continuation that's not a valid digit or BigInt suffix.

**Edit**: at end of `read_radix_int` (after the digit loop), if `peek_byte()` is ASCII digit OR identifier-start (and NOT `n` for BigInt), return InvalidNumeric.

### LEP-EXT 2.b — numeric separator in legacy-octal leading-zero form

`0_0`, `00_1`, `08_0`, `0_8` — separators inside legacy-octal-like forms (leading `0`). Spec §12.8.1 grammar excludes separators from legacy-octal forms; cruft accepted them.

**Edit**: in `read_numeric_literal` main loop, when consuming `_` and the FIRST byte was `0`, return InvalidNumeric.

**Regression-during-development** (recorded per Doc 727 basin-stability of trajectory append-only):

EXT 2.a's first cut REJECTED `0xffn` (hex BigInt with `n` suffix) because `n` is_identifier_start_byte. diff-prod regressed 41/42 (bigint-ops fixture broke). Fixed by adding `b != b'n'` carve-out (BigInt suffix is the next branch's responsibility). Re-verified: 0xffn ACCEPT preserved, 0b14 REJECT ✓.

**Verification (probes)**:

| Probe | Pre-EXT-2 | Post-EXT-2 |
|---|---|---|
| `0b14;` | ACCEPT | **REJECT SyntaxError** ✓ |
| `0o89;` | ACCEPT | **REJECT SyntaxError** ✓ |
| `0b1abc;` | ACCEPT | **REJECT SyntaxError** ✓ |
| `0b1n;` (BigInt suffix) | ACCEPT | ACCEPT ✓ (carve-out preserved) |
| `0xffn;` (BigInt suffix) | ACCEPT | ACCEPT ✓ (carve-out preserved) |
| `0_0;` (separator + leading zero) | ACCEPT | **REJECT SyntaxError** ✓ |
| `00_1;` | ACCEPT | **REJECT SyntaxError** ✓ |
| `08_0;` (non-octal-decimal + sep) | ACCEPT | **REJECT SyntaxError** ✓ |
| `1_0;`, `123_456;`, `0.5_0;` (valid separators) | ACCEPT | ACCEPT ✓ (preserved) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL** (after BigInt carve-out fix)
- Random 300 prev-PASS: **300/300, 0 regressions**
- SyntaxError curated cluster: **45/45 (held)**

**Yield**:

| Locale | Pre-EXT-2 | Post-EXT-2 |
|---|---:|---:|
| NLC (numeric-literal-conformance) | 136/157 | **147/157 (+11; 93.6% pool)** |
| IDT, SLEC, SE | (held) | (held) |

**Findings**

**Finding LEP.4 (LEP-EXT 1 was the unblock; LEP-EXT 2 is the harvest)**: with the propagation gap closed, NLC's actual substrate gaps became visible and addressable. LEP-EXT 1 alone delivered +36 cross-locale via behavior already correct at the lex tier (cruft was rejecting, just losing the Err). LEP-EXT 2 added +11 by closing actual lex-tier rejection rules that were missing. The pattern: a propagation fix UNBLOCKS visibility of downstream substrate gaps; the downstream gaps were always present but invisible. Standing recommendation: after a propagation/swallow fix lands, immediately re-survey affected locales' residuals — the previously-invisible gaps surface and become tractable.

**Finding LEP.5 (the BigInt-suffix regression illustrates Rule-23 verification-probe value at landing)**: my first-cut EXT-2.a check rejected `0xffn` because `n` matches `is_identifier_start_byte`. Caught immediately by diff-prod (42/41 regression — diff-prod's `bigint-ops` fixture broke). Without diff-prod gating, this would have shipped silently. Standing recommendation: every lex-tier substrate move's gating MUST include diff-prod (not just locale-specific probes); diff-prod's cross-cutting nature catches carve-out misses that locale-probes don't.

**Status**: LEP-EXT 2 CLOSED. Locale's substrate scope expanded to include the lex-tier rejection moves the EXT 1 propagation enabled. NLC at 93.6% pool pass. LEP-EXT 3 remains: opportunistic engagement-wide re-baseline after further substrate work.

**Findings**

**Finding LEP.0 (Rule 23's verification-probe surfaces engagement-wide gaps)**: NLC's baseline-inspection produced NLC.0 (now-retracted; eval-class-wrapping was already correct). IDT's baseline-inspection refined Rule 23 with the verification-probe step (Finding IDT.0). NLC-EXT 1-revised then APPLIED the verification-probe at substrate landing (eprintln in lexer to confirm reach + Err return) and surfaced NLC.3 — a meta-substrate gap that affects every lex-tier strict-mode rejection. Standing recommendation extends the Rule-23 verification-probe discipline to substrate-landing time, not just baseline-inspection time: when a substrate move depends on cross-tier state (here: parser's strict_mode → lexer's strict_mode → lex Err → eval catch), instrument the substrate move with debug-probes at each tier-crossing to verify reach before declaring landed. The probe is the discipline; the eprintln is the instrument.
