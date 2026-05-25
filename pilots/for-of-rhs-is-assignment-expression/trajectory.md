# for-of-rhs-is-assignment-expression — Trajectory

## FORA-EXT 1 — for-of RHS parses AssignmentExpression, not Expression (2026-05-25)

**Trigger**: SyntaxError residual analysis. 3 of 16 remaining fails were `head-{expr,decl,var}-no-expr.js` — `for (x of [], [])` accepted. ECMA-262 §14.7.5 grammar: for-of RHS is **AssignmentExpression** (no comma operator); for-in RHS is **Expression** (sequence allowed).

cruft's `parse_for_statement` used `parse_expression` at all 4 RHS sites without discriminating `is_of`.

**Edits** (4 LOC, one targeted sed):

At each of the 4 RHS-parse sites in `parse_for_statement` (697, 772, 872, 899), replace `let right = self.parse_expression()?;` with `let right = if is_of { self.parse_assignment_expression()? } else { self.parse_expression()? };`. The `is_of` boolean was already in scope at every site from the existing `let is_of = self.is_contextual_keyword("of")` discrimination — no new state needed.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `for (x of [], [])` (expression head) | accepted | SyntaxError |
| `for (let x of [], [])` (decl head) | accepted | SyntaxError |
| `for (var x of [], [])` (var head) | accepted | SyntaxError |
| `for (x in o, o) {}` (for-in still allows comma) | parses | parses |
| `for (x of [1,2,3]) {}` (normal) | parses | parses |
| test262 SyntaxError cluster (45 tests) | 29/45 | **32/45** |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding FORA.1 (grammar parameterization on already-in-scope flag)**: the discrimination spec mandates (Expression vs AssignmentExpression based on `in` vs `of` keyword) was already encoded in `is_of` at every call site. The substrate just needed to use it for one more decision. Standing recommendation: when a single boolean already discriminates an enclosing branch, audit nearby code for shared-state-but-not-shared-decision opportunities; the discrimination usually applies more broadly than its initial introduction.

**Status**: FORA-EXT 1 CLOSED. Cluster 29/45 → 32/45.
