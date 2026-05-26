# lex-error-propagation-to-eval-surface — Seed

## Substrate-pilot spawned per keeper directive (Telegram 9834) to investigate the NLC.3 swallow gap.

Surfaces a meta-substrate issue affecting any lex-tier rejection that depends on parser-tier state (strict mode in particular). Without closure, the SLEC-EXT 1 + NLC-EXT 1-revised lexer-side strict-mode rejection rules cannot surface as test262 yield because the errors don't reach `(0,eval)(...)`'s catch.

## Telos

Materialize the coordinate

```
runtime/eval-error-surface ::
  E2/eval-pipeline-error-propagation ::
  cut/lex-error-reaches-eval-catch ::
  property/cruft-lex-Err-propagates-to-test262-runner-as-SyntaxError-instance
```

The induced property is that every `LexError` returned from `Lexer::next_token` surfaces at the test262 runner's `(0,eval)(...)` catch as a SyntaxError-class object — same path that already works for direct-script-mode lex errors (`0b2;` standalone correctly surfaces as SyntaxError exit=70).

## Apparatus

- `cruftless/src/main.rs:284` — cli entry's "cruft: evaluation error: ..." print path. Direct-script-mode errors surface here.
- `pilots/rusty-js-runtime/derived/src/module.rs:920, 1451` — two `parse_module(src/wrapped)` call sites that .map_err the result. Lex errors flowing through here.
- `pilots/rusty-js-parser/derived/src/parser.rs:1505` — `lex_to_parse` mapping.
- `pilots/rusty-js-parser/derived/src/parser.rs::parse_module` — the loop at line 123+ may have a recovery path that swallows lex Err.
- `pilots/rusty-js-parser/derived/src/stmt.rs::parse_statement` — Stmt::Opaque fallback per the "Control-flow forms ... fall back to Stmt::Opaque" comment at line 7.

## Founding finding (NLC.3 + direct probes)

**Empirical evidence**:

| Source | Direct script-mode (`cruft file.js`) | Via `(0,eval)` from cli |
|---|---|---|
| `0b2;` | error exit=70 ✓ | runtime catches SyntaxError ✓ |
| `"use strict"; 00;` | **exit=0 silent** ✗ | runtime catches nothing |
| `function f(){ "use strict"; 00; }` | **exit=0, "compile-and-run ok"** ✗ | same |

cruft's lexer's `read_numeric_literal` for `"use strict"; 00;` IS reached AFTER strict-mode flips (verified via eprintln debug per NLC-EXT 1-revised trajectory). The `return Err(...)` for legacy-octal-in-strict IS executed. But the cli's `main.rs:284` "cruft: evaluation error" branch is NOT reached — exit=0 silent.

**Hypothesis** (to verify at LEP-EXT 1): cruft's `parse_module` or `parse_statement` has a recovery path that swallows lex errors mid-stream when the source has already parsed at least one statement. Possible sites:
- The Stmt::Opaque fallback at stmt.rs:291 (currently only fires for `with` statements; may fire elsewhere)
- A `match parse_X { Err(_) => skip_to_terminator }` somewhere in expression-statement parsing
- The bump's `.map_err(lex_to_parse)?` at parser.rs:898 might be inside a `match {Err(_) => ...}` block somewhere

## Methodology

Three rungs.

### LEP-EXT 1 — locate the swallow site

Use binary search: add an eprintln just AFTER each parse path's `?` propagation point + at all `match parse { Err }` arms. Identify which one absorbs the lex Err from `"use strict"; 00;`. Cost: ~10 min of grep + debug eprintlns.

### LEP-EXT 2 — fix the propagation

Once located, decide:
(a) Remove the recovery path (lex errors should always propagate)
(b) Distinguish lex errors from parse errors (lex errors propagate; parse errors may recover via Stmt::Opaque)
(c) Tag the LexError as "fatal" vs "recoverable" and recover only the latter

Most likely (b) or (c). Lex-tier errors indicate the source is malformed at the byte level; spec says SyntaxError at parse phase. Parser-tier errors (syntax errors) sometimes recover for opaque-walk in cruft's design.

### LEP-EXT 3 — measure yield engagement-wide

Once landed, re-baseline NLC + IDT + SLEC. Strict-mode legacy-octal tests (NLC) should flip to PASS. Strict-mode legacy-escape tests (SLEC, the ~12 in directive-prologue cluster) may also flip if cruft's strict-mode propagation reaches the lex tier in time. IDT residuals (7 unicode-id + yield-strict) won't move (different mechanism).

## Carve-outs

- The directive-prologue retro-reject issue (SLEC.1) is separate — that requires LEX to retro-process strings already lexed under sloppy mode when strict-mode is later determined. LEP closes the propagation of lex errors that DO fire; SLEC.1 closes the lex-tier behavior that should fire but doesn't.
- The bare-CompileError-class wrapping (the originally-claimed-and-retracted NLC.0) is fine; cruft DOES wrap lex errors as SyntaxError when they surface via the working direct-script-mode path. LEP closes the swallow gap, not the wrapping.

## Composes-with

- `pilots/numeric-literal-conformance/` (NLC) — empirical anchor; locale where the gap surfaced via Finding NLC.3.
- `pilots/string-literal-and-escape-conformance/` (SLEC) — sibling; many of its strict-mode escape rejections will surface once LEP closes the propagation.
- `pilots/identifier-tokenization/` (IDT) — likely affected for residual function/class-decl-name fails (probes show some lex rejections don't reach eval catch).
- `apparatus/docs/predictive-ruleset.md` Rule 23 — the verification-probe step (Finding IDT.0) is exactly what surfaced this. LEP is Rule-23's value-proposition empirically realized at engagement-wide scope.
- `pilots/rusty-js-jit/findings.md` Addendum XV — the prior NLC.0 mis-read was about THIS layer (the eval-error-class wrapping). The wrapping was always correct; what's wrong is the PROPAGATION. LEP names that gap.

## R13 prospective C1-C4 at founding

- C1 (sibling): HOLDS — direct-script-mode lex error propagation already works for `0b2;`; LEP extends the same shape to multi-statement and function-body contexts.
- C2 (shape-compat): TBV at LEP-EXT 1; depends on where the swallow site lives. If it's a single recovery point, C2 holds; if scattered, C2 weakens.
- C3 (cost-positive): expected positive (the fix is a discriminator at one or few sites; engagement-wide reach).
- C4 (bail safety): TBV; need to verify the recovery path it disables is genuinely unnecessary vs serving some intentional purpose.

## Resume protocol

Read `trajectory.md` tail.

## Status

LEP-EXT 0 FOUNDED. Empirical anchor is verified (NLC.3); LEP-EXT 1 (locate swallow site) is the immediate substrate move.
