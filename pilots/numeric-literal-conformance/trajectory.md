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
