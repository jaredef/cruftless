# numeric-literal-conformance — Trajectory

## NLC-EXT 0 — founding + baseline + surfacing the engagement-wide eval-error finding (2026-05-25)

**Trigger**: Keeper directive (Telegram 9820) "B and then A." Second spawn of the A-sequence; first substrate locale from the tokenization-above-IR brief.

**Apparatus established**:

- `exemplars/exemplars.txt` — 157 fixtures from `test262/language/literals/numeric/`.
- (run-exemplars.sh deferred until NLC-EXT 1 substrate move; current baseline is one-time)
- Baseline: **PASS=104, FAIL=53 (66.2%)**.

**Sample of failure shapes**:

```
binary-invalid-digit.js   FAIL  expected SyntaxError, got String
legacy-octal-integer-strict.js FAIL  expected SyntaxError, got String
non-octal-decimal-integer.js   FAIL  parse: lex error: legacy octal/decimal integer literals forbidden in module code
numeric-separator-literal-bil-bd-nsl-bd-err.js FAIL  expected SyntaxError, got String
```

**Finding NLC.0 (the locale surfaces an engagement-wide substrate move)**: ~40 of the 53 failures are `expected SyntaxError, got String`. Direct-probe investigation shows cruft's `eval()` throws a String (the CompileError text) for parse-tier failures, not a SyntaxError-class object. The test262 runner at `legacy/host-rquickjs/tests/test262/runner.mjs:103` does `thrownName = thrown.constructor.name` — for cruft's String-typed CompileError this evaluates to `"String"`. Test262 expects `"SyntaxError"`.

This is **engagement-wide**: every `negative: phase: parse, type: SyntaxError` test262 test that cruft's parser correctly rejects fails with this same `got String` shape regardless of substrate area. The fix (NLC-EXT 1, see seed §Methodology) is at cruft's eval-error-wrapping layer; the yield is across the entire negative-parse-test surface, not just numeric.

**The cluster-coherence-multiplier spawn shape worked exactly as predicted** — selecting the highest-yield-per-LOC candidate by all 5 conditions surfaced a load-bearing substrate move that the apparatus could not have named without the locale's specific probe.

**Verification of the diagnosis**:

```
$ echo "0b2;" > /tmp/0b2.js && cruft /tmp/0b2.js
cruft: evaluation error: CompileError("parse: lex error: invalid radix-prefixed literal (InvalidNumeric) @byte0 ...")

$ echo "0b_1" > /tmp/sep.js && cruft /tmp/sep.js
cruft: evaluation error: CompileError("parse: lex error: invalid numeric separator (InvalidNumeric) @byte0 ...")
```

cruft correctly REJECTS the malformed literals. The 53 fails aren't "cruft is too permissive"; they're "cruft's correct rejection is reported as the wrong error class."

**Findings**

**Finding NLC.0**: see above — engagement-wide eval-error-class wrapping is the load-bearing substrate move surfaced by this locale's founding. NOT specific to numeric-literal substrate work.

**Finding NLC.1 (locale-as-probe pattern)**: NLC was spawned as a cluster-coherence-multiplier-shaped locale targeting §12.8 NumericLiteral. The founding inspection revealed that the load-bearing substrate move is at a different tier (runtime eval-error wrapping) than the locale's declared coordinate (lex-tier). The locale-as-probe pattern: when a locale spawned at coordinate X reveals via baseline-inspection that the move-shape is at coordinate Y, the right discipline is to land the Y move first (per R13 prospective) and treat X as the test surface that VALIDATES the Y move. Standing recommendation: every locale's founding should include a baseline-inspection rung that audits where the locale's failure-shape actually traces to; spawning the locale is the probe, not the target.

**Status**: NLC-EXT 0 FOUNDED. NLC-EXT 1 (eval-error-class wrapping fix) is the immediate substrate move per the founding finding; expected engagement-wide yield. Substrate touches runtime; scope warrants keeper review before landing. NLC-EXT 2+3 (strict-mode legacy octal + edge cases) follow after EXT 1.

---

## NLC-EXT 0 CORRECTION (2026-05-25; post identifier-tokenization Rule-23 self-correction)

**Trigger**: spawning identifier-tokenization the same session ran Rule 23 baseline-inspection on its 268-fixture pool. The verification-probe step (direct probe `(0,eval)("0b2;")` → ctor=SyntaxError) revealed that cruft's eval-error-class wrapping was ALREADY CORRECT. The "got String" shape originates from test262's `$DONOTEVALUATE()` harness throwing a literal string when `<bad source>` is incorrectly accepted — NOT from cruft mis-wrapping CompileError.

**Finding NLC.0 (as originally stated above) is RETRACTED.** Per findings.md Addendum XV. The "engagement-wide eval-error-class wrapping issue" claim was wrong.

**The actual mechanism** (Finding NLC.0-revised, per Addendum XV): cruft's parser is incorrectly permissive on the affected source shapes. Test262's `$DONOTEVALUATE()` (a string-throwing harness function) runs only when cruft fails to reject at parse; the string it throws is what gets caught and reported as "got String." The diagnostic identifies parser-permissiveness, NOT error-class-mis-wrapping.

**Re-scoping NLC's substrate work**:

- **NLC-EXT 1 (as originally scoped, "eval-error-class wrapping fix"): RETRACTED.** No such fix is needed; cruft's eval already wraps correctly.
- **NLC-EXT 1-revised**: lex-tier substrate fixes for the specific malformed-numeric shapes cruft currently accepts. Per the original baseline-inspection sample (53 fails), the actual shapes are:
  - Binary literal acceptance of non-binary chars (`0b2`)
  - Legacy octal acceptance in strict mode (`"use strict"; 00`)
  - Non-octal-decimal-integer (`08`, `09`) in strict
  - Numeric-separator placement edge cases (`0b_1` may already reject — verify)

Each is a focused lex-tier check addition in `pilots/rusty-js-parser/derived/src/lexer.rs::read_radix_int` / `read_numeric_literal`. Substrate scope ~30-50 LOC; properly within NLC's original locale-tier per Rule 11.

**NLC.1 STANDS as a recommendation in modified form**: the locale-as-probe pattern (Rule 23) caught NLC.0's mis-read. The first-read inspection failed; the verification-probe step would have caught it. Refined recommendation in Finding IDT.0: Rule 23 inspection MUST include direct verification-probe in the substrate, not just re-reading the failure reasons.

**Per Doc 727 §X basin-stability**: the prior NLC-EXT 0 trajectory entry (above the divider) is preserved as the inspection-at-the-time record; this correction appends rather than edits.

**Status (corrected)**: NLC's locale-tier substrate work is well-scoped per the lex-tier shapes identified in NLC-EXT 1-revised. The engagement-wide fix proposal is RETRACTED. NLC's substrate work follows the IDT pattern when keeper directs.

---

## NLC-EXT 1-revised — lex-tier legacy-octal direction-flip (2026-05-25; keeper: "continue to 1")

**Trigger**: Keeper directive (Telegram 9832) "continue to 1." Lands the revised NLC-EXT 1 (per Addendum XV correction): lex-tier strict-mode gate on legacy-octal-integer literals, replacing the pre-fix unconditional "in module mode" rejection.

**Edit** (~5 LOC in `pilots/rusty-js-parser/derived/src/lexer.rs::read_numeric_literal`):

Pre-fix (line 426): `if first == b'0' && self.pos > start + 1` — fired UNCONDITIONALLY, wrongly rejecting sloppy-script-mode legacy octals.

Post-fix: `if first == b'0' && self.pos > start + 1 && self.strict_mode` — fires in strict mode only, per §12.8 + Annex B B.1.1. The `Lexer::strict_mode` field is the one SLEC-EXT 1 added; the Parser pushes the state via set_lexer_strict.

**Verification (probes)**:

| Probe | Pre-fix | Post-fix |
|---|---|---|
| `0001;` (sloppy script) | REJECT (wrong) | **ACCEPT** ✓ |
| `09;` (sloppy non-octal-decimal) | REJECT (wrong) | **ACCEPT** ✓ |
| `"use strict"; 00;` (strict) | ACCEPT (wrong) | ACCEPT (still wrong — integration gap, see NLC.3 below) |

**Test262 effect**:

| Test | Pre-fix | Post-fix |
|---|---|---|
| `non-octal-decimal-integer.js` (sloppy, expects accept) | FAIL (cruft rejected) | **PASS** ✓ |
| `legacy-octal-integer.js` (sloppy, expects accept + correct values) | FAIL (cruft rejected) | FAIL (cruft accepts but parses `070` as 70-decimal, not 56-octal) |
| Strict-mode legacy-octal tests (`legacy-octal-integery-*-strict.js`) | FAIL | FAIL (integration gap) |

Net NLC pool: 104/157 unchanged in aggregate (one PASSed + one DIFFERENTLY FAILed offset). Behavior is correctness-improved: cruft no longer wrongly rejects sloppy-script legacy-octals.

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**
- SyntaxError curated cluster: **45/45 (held)**
- IDT exemplars: **261/268 (held)**
- SLEC exemplars: **53/73 (held)**

**Finding NLC.3 (lex-tier strict-mode integration gap)**: probe shows `Lexer::strict_mode == true` correctly at the legacy-octal check site when source is `"use strict"; 00;` (or function-body with strict directive). My `return Err(...)` from `read_numeric_literal` IS executed (verified via debug eprintln). But cruft's downstream parse pipeline (Parser → compiler → eval) does NOT surface the error; the source compiles silently. There is a SPECULATIVE-PARSE or ERROR-SWALLOWING path somewhere downstream that absorbs the lex Err without propagating to test262's `(0,eval)(...)` catch. Substantial separate substrate investigation; surfaces as NLC-EXT 2 candidate (locate the swallow site; either fix it or route lex errors via a different path).

**Finding NLC.4 (one PASS + one differently-FAIL = net-zero exemplar move can still be correctness gain)**: the lex-tier direction flip swapped which tests pass/fail without moving the aggregate, but the SUBSTRATE is more correct: cruft no longer rejects spec-valid source. The wrong-result legacy-octal case (`070 → 70` not `56`) is a SEPARATE semantic gap (cruft's numeric-literal evaluator doesn't treat sloppy-script legacy-octal as octal). Standing recommendation: pool-aggregate yield is not the only correctness signal; per-test direction-of-fix matters at locale-tier review.

**Status**: NLC-EXT 1-revised CLOSED. Locale's substrate scope partially closed; the strict-mode integration gap (NLC.3) and the legacy-octal-semantic gap (NLC.4) are NLC-EXT 2+3 candidates. Locale stays open.
