# rest-param-trailing-comma — Trajectory

## RPTC-locale-EXT 1 — early-error: rest param + trailing comma (2026-05-25)

**Trigger**: NSPS.2 named ~8 separable parser-tier early-error rules as sibling-locale candidates. Picked smallest-bounded first: rest-param-trailing-comma is a single comma-check at parse_function_parameters.

**Edits** (~7 LOC at `stmt.rs::parse_function_parameters`):

In the trailing-comma-then-continue branch (after pushing the param), if the just-pushed param is rest, throw ParseError. Spec §15.1.1: rest cannot be followed by a trailing comma (and per the same rule, rest must be last — even a non-trailing comma is a syntax error).

**Verification**:

| Probe | Before | After |
|---|---|---|
| `var f = (...rest,) => 0` | accepted | SyntaxError |
| `function g(...rest,) {}` | accepted | SyntaxError |
| `var f = (a, b,) => a + b` (non-rest trailing) | accepted | accepted (ES2017+) |
| test262 SyntaxError cluster (45 tests) | 4/45 | **5/45** |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding RPTC-locale.1 (single-comma-check at the existing branch)**: 7 LOC at the trailing-comma branch that was already in the function. No new traversal, no AST tagging — the check fires exactly when the comma-then-continue path runs, gated by the just-set `rest` boolean. Standing recommendation pattern: parser early errors that depend only on facts already established at the current parse position belong at that parse position, not in a post-parse validator.

**Status**: RPTC-locale-EXT 1 CLOSED.
