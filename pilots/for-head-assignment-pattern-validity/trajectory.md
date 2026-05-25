# for-head-assignment-pattern-validity — Trajectory

## FHAPV-EXT 1 — promote conversion-None to SyntaxError at for-head LHS (2026-05-25)

**Trigger**: NSPS.2 residual analysis. 21 of 32 remaining SyntaxError fails were `for-of/dstr/*` shapes — array/object literal in for-of/for-in head that should reject invalid AssignmentPattern shape.

`expr_to_binding_pattern` already correctly returns `None` for invalid shapes (rest-not-last, rest-with-init, nested invalid LHS, object-rest-not-last). The for-of/for-in head conversion at `stmt.rs:905` silently fell back to an empty `BindingIdentifier { name: "" }` — runtime accepted invalid form.

**Edits** (~10 LOC at `stmt.rs::parse_for_statement` expr-head conversion):

When the head expression is an Array or Object literal AND `expr_to_binding_pattern` returns None, return ParseError. Non-literal Nones (other LHS shapes that can't be patterns) keep the existing fallback to preserve any current behavior on non-test-262-probed shapes.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `for ([...x, y] of [[]])` (rest+element) | accepted | SyntaxError |
| `for ([...x, ...y] of [[]])` (rest+rest) | accepted | SyntaxError |
| `for ([...x = 1] of [[]])` (rest+init) | accepted | SyntaxError |
| `for ({...x, y} of [{}])` (obj-rest-not-last) | accepted | SyntaxError |
| `for ([x, y] of [[1,2]])` (valid pattern) | parses | parses |
| `for ([...x] of [[1]])` (valid rest at end) | parses | parses |
| test262 SyntaxError cluster (45 tests) | 13/45 | **29/45** (+16) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding FHAPV.1 (silent-fallback was masking a high-yield closure)**: `expr_to_binding_pattern` had been correctly returning None on invalid patterns since its inception; the for-head conversion just dropped that signal. 10 LOC promoting None-to-throw closed 16 test262 tests. Standing recommendation echo of IPTO.2 / LOAL.1 / GBNE.1: silent default-on-None / Result-ignoring patterns at the substrate-spec boundary are high-leverage audit targets; the validator was right, the consumer ignored it.

**Finding FHAPV.2 (literal-vs-non-literal carve-out preserves behavior)**: the conversion can return None for non-literal LHS too (e.g., a CallExpression that's not a valid AssignmentTarget). Throwing on every None would be the spec-strict path but risks regressions on shapes not tested here. The carve-out gates the throw on `Expr::Array | Expr::Object` matches — exactly the literal shapes spec §14.7.5.1 names. Standing recommendation: when converting silent-fallback to throw, gate the throw on the shape spec specifically names rather than promoting all None unconditionally; the gate makes the rung tightly scoped to the test262-validated surface and defers shape-unenumerated cases.

**Status**: FHAPV-EXT 1 CLOSED. Cluster 13/45 → 29/45 (+16); largest single-rung jump in today's parser arc.
