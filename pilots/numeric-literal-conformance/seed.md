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

## Founding finding — the 53 fails are dominated by an error-class mismatch, not a missing lex rule

Inspecting the 53 fails (sample of 20):

| Failure shape | Count | Cause |
|---|---:|---|
| `expected SyntaxError, got String` | ~40 | cruft's `eval()` throws a **String** (CompileError text) for parse-tier failures; test262 expects a SyntaxError instance |
| `parse: lex error: legacy octal/decimal integer literals forbidden in module code (LegacyOctalInModule)` | 2 | cruft's lexer fires LegacyOctalInModule even outside module mode (the check at lexer.rs:426 doesn't gate on module-mode) |
| `expected SyntaxError, got String` on strict-only legacy octals (`00`, `01`, …, `07`) | 7 | cruft accepts the lex but doesn't surface a SyntaxError in strict mode (and would have the eval-error-class issue if it did) |
| Numeric-separator violations (`0b_1`, `1__2`, etc.) | ~4 | cruft's lex DOES reject these (per probe: `0b_1` raises InvalidNumeric) — but again CompileError class, not SyntaxError |

**The root cause is the eval-error-class wrapping, not missing lex rules.** Cruft's lexer is already substantially correct — the 53 fails are MOSTLY cases where cruft correctly rejects the source but presents the error as a `String` (from the `CompileError("parse: lex error: ...")` shape) instead of a SyntaxError-class object.

## Methodology

Three rungs.

### NLC-EXT 1 — eval-error-class wrapping (the load-bearing move)

Edit cruft's eval-side error pathway so parse-tier CompileError throws are wrapped as SyntaxError instances reaching JS-tier `catch`. The wrapper needs to set `.constructor.name === "SyntaxError"` (matching test262's `thrownName = thrown.constructor.name` check at `legacy/host-rquickjs/tests/test262/runner.mjs:103`).

**Substrate site**: cruft's eval implementation; likely in `pilots/rusty-js-runtime/derived/src/...` — the code path that turns the host-side CompileError into a JS-tier throw value.

**Expected yield**: ~40+ tests in this locale alone; large engagement-wide yield because every `negative: phase: parse, type: SyntaxError` test262 test currently fails with the same shape regardless of substrate area. **This is engagement-wide, not numeric-specific** — landing NLC-EXT 1 would unblock the test262 negative-parse surface across all coordinates that fail with the same shape (parser-early-error, ast-bytecode-early-error, all currently-error-as-String cases).

**This is the cross-coordinate yield Finding LPA.4 predicted: the eval-error-class fix is engagement-wide; spawning NLC was the locale that surfaced it.**

### NLC-EXT 2 — strict-mode legacy octal detection

After NLC-EXT 1 lands, the strict-mode legacy octal cases (`"use strict"; 00`) still need to detect-and-reject. Cruft's current LegacyOctalInModule check (lexer.rs:430) only fires in module mode; needs extension to strict-mode-in-script.

**Substrate site**: lexer.rs:426-435. ~5 LOC.

### NLC-EXT 3 — remaining edge cases

Per-failure inspection of any residual fails after EXT 1+2 close. Likely numeric-separator edge cases or BigInt-suffix interactions.

## Cluster-coherence-multiplier reading

The locale's pool (157) and the cluster-coherence multiplier's 5 conditions held at founding. BUT — the founding finding (NLC-EXT 1's engagement-wide yield) shows the load-bearing substrate move is NOT at the numeric-literal tier; it's at the eval-error-wrapping tier. The NLC spawn was correct as a SPECIFIC instance, but the substrate move it surfaced is broader. **Standing observation**: cluster-coherence-multiplier-shaped spawns sometimes surface engagement-wide substrate moves; the locale's apparent narrowness is a probe-shape that reveals broader work. (Finding NLC.0)

## Composes-with

- `apparatus/docs/ecma-conformance-...md` §XI Lexical-grammar coordinate class.
- `pilots/apparatus/tokenizer-error-classification-refinement/` (TECR) — sibling apparatus-pilot whose `availability/missing-lex-feature` coordinate is the post-NLC home of any remaining numeric-literal fails.
- `pilots/apparatus/test262-categorize/` — the matrix this locale's yield will shift.
- `docs/engagement/tokenization-above-ir-candidate-brief.md` — the brief that spawned this locale.

## R13 prospective C1-C4 at founding

- C1 (sibling): HOLDS — LGSS, BBND are cluster-coherence-shaped siblings at the parser tier; pattern transfers to lex tier.
- C2 (shape-compat): HOLDS — lexer.rs's structure permits the EXT 2+3 fixes; EXT 1 is at a different tier (runtime) which is a known surface.
- C3 (cost-positive): TBV at NLC-EXT 1; expected positive (eval-error wrapping is one site, large reach).
- C4 (bail safety): TBV per substrate change at NLC-EXT 1; needs diff-prod + random-300 gating.

## Resume protocol

Read `trajectory.md` tail. Run `exemplars/run-exemplars.sh` (TBD) for current yield.

## Status

NLC-EXT 0 FOUNDED. Baseline 104/157 (66.2%). The load-bearing finding (NLC.0) is that the 53 fails are dominated by an engagement-wide eval-error-class wrapping issue, NOT by missing lex rules. NLC-EXT 1 (the eval-wrapping fix) is the immediate substrate move — substantial scope (touches runtime), engagement-wide yield. Deferred to keeper direction on scope.
