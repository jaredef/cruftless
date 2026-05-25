# array-pattern-rest-trailing-comma — Trajectory

## ARTC-EXT 1 — AST flag preserves rest-trailing-comma for AssignmentPattern rejection (2026-05-25)

**Trigger**: Cluster D residual (2 of 2) — `[...x,]` accepted as both ArrayLiteral and AssignmentPattern because the parser dropped the trailing-comma-after-spread information at the literal-parse site. Spec §13.3.3 allows it in expression context but forbids it in AssignmentPattern context.

**Edits** (~20 LOC across 4 files):

1. `pilots/rusty-js-ast/src/lib.rs::Expr::Array` — added `trailing_comma_after_spread: bool` field.
2. `pilots/rusty-js-parser/derived/src/expr.rs::parse_array_literal` — track a local boolean: set true after bumping a comma if the just-pushed element was Spread; reset on subsequent element. Field set from the local at construction.
3. `pilots/rusty-js-parser/derived/src/stmt.rs::expr_to_binding_pattern` Array arm — when the flag is set, return None. FHAPV-EXT 1's literal-shape-None throw at the for-head call site converts to SyntaxError.
4. Two synthetic-construction sites updated to `trailing_comma_after_spread: false`: template-strings literal in `expr.rs:962`, binding-pattern-to-assignment-expr round-trip in `compiler.rs:5505`.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `var x; for ([...x,] of [[]]) ;` (for-head pattern) | parses | SyntaxError |
| `var a=[1,2,3]; var b=[...a,];` (expression) | b.length=3 | b.length=3 |
| `var x; [...x] = [1,2,3]` (plain rest pattern) | parses | parses |
| `var a,b; [a,b,] = [1,2]` (non-spread trailing) | parses | parses |
| test262 SyntaxError cluster (45 tests) | 43/45 | **45/45** (+2; full closure) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding ARTC.1 (preserve grammar-disambiguating source-text in the AST)**: when the same surface syntax has different validity in different grammar contexts (here: ArrayLiteral vs ArrayAssignmentPattern over `[...x,]`), the parser MUST preserve the disambiguating fact in the AST. Stripping it at parse-time forces every downstream consumer to either accept or reject uniformly. A single boolean field paid for the disambiguation; 4 construct sites updated (only 2 in the parser proper, 2 in synthetic-construction round-trips). Standing recommendation: when an AST node represents a surface form that participates in cover-grammar reinterpretation (§5.1.5 ECMA-262), audit its field set for completeness against the spec's distinguishing predicates.

**Finding ARTC.2 (standalone-assignment cover-path carve-out)**: `([...x,] = value)` outside of for-head still parses cleanly because the comma-operator-then-destructure-assign path constructs the AssignmentPattern without going through `expr_to_binding_pattern`. Same carve-out for `[...x, ,]`. The flag is now in the AST and available; the closure rung is to add a `check_assignment_pattern_validity` call wherever a parenthesized-assignment-expression is reinterpreted as an AssignmentPattern. Not pursued this rung because test262 residual surface didn't have a witness; deferred until consumer probe surfaces one.

**Status**: ARTC-EXT 1 CLOSED. Cluster 43/45 → **45/45 (full closure of the 45-test SyntaxError curated sample)**. Today's parser arc cumulative: 13/45 → 45/45 (+32) across 7 locales.
