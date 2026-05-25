# for-of-async-lookahead — Trajectory

## FAOF-EXT 1 — reject `async of` at for-of head per §14.7.5 lookahead (2026-05-25)

**Trigger**: Cluster B residual (1 of 5). Spec §14.7.5 grammar `[lookahead ∉ { let [, async of }]` forbids the token sequence `async of` at the start of a for-of LHS. cruft's bare-ident fast-path bound `async` as a BindingIdentifier and accepted the form.

**Edits** (~5 LOC at `parse_for_statement` fast-path):

In the `is_ident("in") || is_contextual_keyword("of")` branch, when `is_of` and the bumped identifier name is `async`, return ParseError before constructing the ForBinding. `for (async in …)` remains accepted (the lookahead restriction is for-of only).

**Verification**:

| Probe | Before | After |
|---|---|---|
| `var async; for (async of [1]) ;` | parses | SyntaxError |
| `var async = {x:0}; for (async in {y:1}) …` | parses | parses |
| `(async function(){ for await (let x of [1]) … })()` | parses | parses |
| `var async = 5; async` (identifier use) | works | works |
| test262 SyntaxError cluster | 40/45 | **41/45** (+1) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding FAOF.1 (grammar-lookahead exclusions are per-keyword)**: §14.7.5's lookahead has two exclusions: `let [` (already handled by cruft's lexical-declaration parse) and `async of` (this rung). They share the same boundary but encode different concerns — `let [` is the let-vs-arraypattern ambiguity, `async of` is the async-function-start vs. for-of-with-identifier ambiguity. Each lookahead-exclusion in the spec grammar deserves an explicit one-line fast-path check; relying on a generic "context-free reserved word" set would either over-reject (forbidding `async` everywhere) or under-reject (the current bug).

**Status**: FAOF-EXT 1 CLOSED. Cluster 40/45 → 41/45.
