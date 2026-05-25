# arrow-misc-early-errors — Trajectory

## ALTA+RPDF-EXT 1 — arrow LT-before-=> and rest-with-default early errors (2026-05-25)

**Trigger**: Cluster E residual (2 of 4) — the two non-AST-shape arrow-tier grammar gaps.

**Edits** (~15 LOC across two sites):

1. **ALTA-EXT 1** (`expr.rs::parse_arrow_function` line ~1292): before `expect_punct(Punct::Arrow)`, when current token is `=>` and `self.lookahead_preceded_by_lt()` is true, return ParseError. Uses the existing accessor (no new lexer state).

2. **RPDF-EXT 1** (`stmt.rs::parse_function_parameters_inner` line ~328): after the default-parse branch, when `rest && default.is_some()`, return ParseError. One check covers arrow, function, method, generator, async — they all share this inner.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `var af = ()\n=> {};` | parses (ASI breaks it) | SyntaxError |
| `var af = () => {}; af();` (same line) | parses | parses |
| `var f = x => x+1; f(3)` (ident-style) | parses | parses |
| `var f = (...x = []) => x;` (rest-default arrow) | parses | SyntaxError |
| `var f = (...x) => x; f(1,2,3)` (plain rest) | parses | parses |
| `var f = (a = 1) => a; f()` (plain default) | parses | parses |
| `function g(...x = []) {}` (rest-default fn) | parses | SyntaxError |
| test262 SyntaxError cluster | 41/45 | **43/45** (+2) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding ALTA.1 (NoLineTerminator-here productions are uniformly check-the-flag)**: §15.3.1's `[no LineTerminator here]` is the same grammatical device used at return/throw/continue/break/postfix++. Cruft already has the `preceded_by_line_terminator` flag in scope at every token boundary. Adding new no-LT productions is a one-line check; the cost surface is the discovery + enumeration of the productions, not the implementation. Standing recommendation: when the spec adds a no-LT-here production, the substrate move is one accessor call at the consume site.

**Finding RPDF.1 (shared-inner check covers all callers)**: parse_function_parameters_inner is shared by arrow, function, method, generator, async, set, ctor parameter contexts. Adding the rest-with-default check at the inner closes the spec gap across all five surface forms in one edit. Standing recommendation: when a spec early-error applies to a Parameter universally (independent of which Function-form encloses it), enforce at the shared parser inner; per-form duplication would invite drift. Same shape as RPTC-locale's trailing-comma check.

**Status**: ALTA + RPDF EXT 1 CLOSED. Cluster 41/45 → 43/45.
