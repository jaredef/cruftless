# numeric-literal-conformance — Seed

## Substrate-pilot — first tokenization-above-IR spawn from the brief.

Per keeper directive (Telegram 9820) selecting B then A. Tier-I substrate locale per refreshed CANDIDATES.md ((pp)). Lands AFTER TECR (Tier-J apparatus precedence per LPA.5).

## Telos

§12.8 NumericLiteral conformance. Coordinate:

```
tokens-to-AST / parser-form ::
  E1/lex-tier ::
  cut/numeric-literal-form-rejection ::
  property/cruft-rejects-malformed-numeric-with-SyntaxError-class
```

The induced property is that cruft's numeric literal lexer rejects every malformed form spec §12.8 disallows, **with the rejected source flagged as SyntaxError** (the test262-expected error class for negative parse tests).

## Apparatus

- `pilots/rusty-js-parser/derived/src/lexer.rs::read_numeric_literal` (line 356) — main lex entry.
- `pilots/rusty-js-parser/derived/src/lexer.rs::read_radix_int` (line 519) — radix-prefixed forms (hex/binary/octal).
- `pilots/rusty-js-parser/derived/src/lexer.rs::LexErrorKind` (line 34) — `InvalidNumeric`, `LegacyOctalInModule`.
- **Exemplar suite**: `pilots/numeric-literal-conformance/exemplars/exemplars.txt` — 157 fixtures from `language/literals/numeric/`.
- **Baseline measurement (FOUNDING)**: **PASS=104, FAIL=53 (66.2%)** at 2026-05-25.

## Founding finding — corrected by Rule 23 verification

NLC-EXT 0 first read incorrectly diagnosed the dominant `expected SyntaxError, got String` shape as eval-error-class wrapping. Identifier-tokenization's follow-on Rule 23 verification probe showed that reading was wrong: cruft's eval path already surfaces parser rejection as SyntaxError when the parser rejects the source. The `String` comes from test262's `$DONOTEVALUATE()` sentinel running after cruft incorrectly ACCEPTS source the spec requires it to reject.

Current reading: the 53 failures identify numeric-literal parser/lexer permissiveness, not runtime error wrapping.

| Failure shape | Count | Cause |
|---|---:|---|
| `expected SyntaxError, got String` | ~40 | cruft accepted malformed numeric source, allowing test262's `$DONOTEVALUATE()` string sentinel to run |
| `parse: lex error: legacy octal/decimal integer literals forbidden in module code (LegacyOctalInModule)` | 2 | cruft rejects, but the specific legacy-octal gating needs direct verification against script/module/strict context |
| strict-only legacy octals (`00`, `01`, ..., `07`) | ~7 | cruft needs strict-mode legacy-octal rejection at the lex/parser boundary |
| Numeric-separator violations (`0b_1`, `1__2`, etc.) | ~4 | verify which forms already reject; close residual accepted malformed forms |

See `trajectory.md` "NLC-EXT 0 CORRECTION" for the append-only correction record. The earlier eval-wrapping hypothesis is retracted.

## Methodology

Three rungs.

### NLC-EXT 1-revised — malformed numeric rejection at lex/parser sites

Close the malformed numeric shapes cruft currently accepts:

- Binary literal non-binary chars (`0b2`)
- Strict-mode legacy octals (`"use strict"; 00`)
- Non-octal decimal integer in strict context (`08`, `09`)
- Numeric-separator placement edge cases (`0b_1`, `1__2`, etc.; verify residuals first)

**Substrate site**: `pilots/rusty-js-parser/derived/src/lexer.rs::read_radix_int` / `read_numeric_literal`, plus strict-context plumbing if required.

**Expected yield**: residual numeric-literal negative-parse failures after direct verification. Scope is lex-tier, ~30-50 LOC.

### NLC-EXT 2 — strict-mode legacy octal detection

If not closed by EXT 1-revised, reject strict-mode legacy octal cases (`"use strict"; 00`) with the correct SyntaxError-class path.

**Substrate site**: lexer/parser strict-mode numeric handling. ~5-15 LOC depending on where strict state is available.

### NLC-EXT 3 — remaining edge cases

Per-failure inspection of any residual fails after EXT 1+2 close. Likely numeric-separator edge cases or BigInt-suffix interactions.

## Cluster-coherence-multiplier reading

The locale's pool (157) and the cluster-coherence multiplier's 5 conditions held at founding. Rule 23's corrected read keeps the substrate move at the numeric-literal lex/parser tier: the locale is a coherent §12.8 negative-parse surface, and its immediate work is to reject malformed numeric forms before `$DONOTEVALUATE()` can run.

## Composes-with

- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md` §XI Lexical-grammar coordinate class.
- `pilots/apparatus/tokenizer-error-classification-refinement/` (TECR) — sibling apparatus-pilot whose `availability/missing-lex-feature` coordinate is the post-NLC home of any remaining numeric-literal fails.
- `pilots/apparatus/test262-categorize/` — the matrix this locale's yield will shift.
- `docs/engagement/tokenization-above-ir-candidate-brief.md` — the brief that spawned this locale.

## R13 prospective C1-C4 at founding

- C1 (sibling): HOLDS — LGSS, BBND are cluster-coherence-shaped siblings at the parser tier; pattern transfers to lex tier.
- C2 (shape-compat): HOLDS — lexer.rs's structure permits the EXT 1-revised/EXT 2 fixes at lex/parser sites.
- C3 (cost-positive): TBV at NLC-EXT 1-revised; expected positive if direct probes confirm the accepted malformed shapes.
- C4 (bail safety): TBV per substrate change at NLC-EXT 1-revised; needs diff-prod + random-300 gating.

## Resume protocol

Read `trajectory.md` tail. Run `exemplars/run-exemplars.sh` (TBD) for current yield.

## Status

NLC-EXT 0 FOUNDED. Baseline 104/157 (66.2%). NLC-EXT 0 CORRECTION retracts the eval-error-class wrapping hypothesis; the immediate move is NLC-EXT 1-revised, lex/parser rejection of malformed numeric literals, following the IDT Rule-23 verification pattern.
