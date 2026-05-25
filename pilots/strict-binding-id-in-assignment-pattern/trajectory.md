# strict-binding-id-in-assignment-pattern — Trajectory

## SBAP-EXT 1 — leaf-walk strict/generator check on AssignmentPattern at for-head (2026-05-25)

**Trigger**: Top-row inspection of the 13-fail SyntaxError residual (R15). Cluster C: 5 of 13 fails were AssignmentPattern shorthand binding to `eval`/`arguments`/`yield` in strict or generator context, at the for-head expression-conversion path. SBEA-EXT 1 had already closed the plain-BindingIdentifier surface; this is the sibling-pattern closure at the destructure-conversion site.

**Edits** (~50 LOC across two sites in `stmt.rs`):

1. New `Parser::check_pattern_binding_ids(&self, pat, span)` helper that recurses through `BindingPattern::{Identifier, Array, Object}` leaves and applies the §13.2 + §13.15.1 rules: reject `eval`/`arguments` in strict, reject `yield` in generator or strict. Handles array `elements` + `rest`, object `properties` + `rest` (rest is `BindingIdentifier`, checked directly).
2. Call the helper at `parse_for_statement` line 913 immediately after `expr_to_binding_pattern(e)` returns Some, before constructing `ForBinding::Pattern`.

The helper reads `self.strict_mode` and `self.in_generator` directly — both flags were already maintained by the SBEA / YIFP arcs. No new state introduced.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `for ({ eval } of [{}])` strict | parses | SyntaxError |
| `for ({ eval = 0 } of [{}])` strict | parses | SyntaxError |
| `for ({ yield } of [{}])` strict | parses | SyntaxError |
| `for ([arguments] of [[]])` strict | parses | SyntaxError |
| `(function*() { for ({ yield } of [{}]) ; })` | parses | SyntaxError |
| `var x; for ({x} of [{}])` (normal) | parses | parses |
| test262 SyntaxError cluster | 32/45 | **37/45** (+5) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding SBAP.1 (sibling-pattern closure under R13 prospective)**: SBEA-EXT 1 closed the strict-binding rule at the plain-BindingIdentifier path. The AssignmentPattern path went through a different conversion (`expr_to_binding_pattern`) that lost parser context entirely, so the same rule did not apply. Walking the converted pattern at the call site (which still has `&self` with `strict_mode`/`in_generator` in scope) closed the residual without modifying the context-free conversion function. Standing recommendation: when a free-function converter loses context that the call site retains, prefer call-site post-validation over threading state through the converter — the converter stays composable and the discipline stays at the boundary.

**Finding SBAP.2 (R13 C-conditions all held for prospective application)**: spawned with C1 (SBEA sibling), C2 (Parser fields available), C3 (leaf-walk negligible cost), C4 (parse-time error, no runtime divergence) all explicitly verified at seed time. Closure was 1 implementation round per the prospective-application thesis.

**Status**: SBAP-EXT 1 CLOSED. Cluster 32/45 → 37/45.
