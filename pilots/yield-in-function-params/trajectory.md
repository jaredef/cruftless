# yield-in-function-params — Trajectory

## YIFP-EXT 1 — YieldExpression forbidden in arrow params inside generator (2026-05-25)

**Trigger**: NSPS.2 sibling list. test262 `param-dflt-yield-expr.js` probes `function *g() { (x = yield) => {}; }` — an arrow inside a generator with `yield` in the arrow's param default. Per §15.3.1 "It is a Syntax Error if ArrowParameters Contains YieldExpression is true."

cruft's yield-branch fires when `in_generator || strict_mode`. Inside `g()`, `in_generator=true` is inherited by the arrow's param parse; yield parses as YieldExpression and the arrow accepts. Spec rejects.

**Edits** (~20 LOC):

- `parser.rs::Parser`: add `in_function_params: bool` field (init false).
- `stmt.rs::parse_function_parameters`: split into thin wrapper + `parse_function_parameters_inner`. Wrapper sets `in_function_params=true` for the inner's duration; saves/restores the prior value (so nested function decls in param defaults don't unset the flag for the outer params).
- `expr.rs` yield-branch: when `in_function_params` is true at a YieldExpression position, throw ParseError "YieldExpression is not allowed in formal parameters".

**Verification**:

| Probe | Before | After |
|---|---|---|
| `function* g() { (x = yield) => {}; }` (test262 target) | accepted | SyntaxError ✓ |
| Sloppy `function f(x = yield) {}` (yield = ident) | accepted | accepted (unchanged) |
| Strict `function f(x = yield) {}` (yield reserved) | SyntaxError | SyntaxError (unchanged) |
| `function* g() { yield 1; }` (yield in body, not params) | works | works (unchanged) |
| test262 `param-dflt-yield-expr.js` | FAIL | **PASS** |
| test262 SyntaxError cluster (45 tests) | 8/45 | **9/45** |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding YIFP.1 (arrow inherits in_generator from enclosing)**: cruft's in_generator tracking is correct (arrows don't introduce a generator boundary; they inherit the enclosing). The bug was that the yield-branch didn't ALSO check "are we in a param list" — the spec's "Contains YieldExpression in ArrowParameters" is a static-position check that the substrate needed to encode as a flag. Standing recommendation: when a spec's early error tests STATIC POSITION (where the AST node appears) rather than DYNAMIC scope, the substrate needs a parse-time flag for that position rather than relying on inherited mode tracking.

**Finding YIFP.2 (generator's own params is a separate setup)**: `function* g(x = yield) {}` at top level fails differently — in_generator is false during the gen's own param parse because parse_function_body_gs sets it only for the body. The minimum-scope fix for this case requires also setting `in_generator=true` around the generator's param parse. That's a small follow-on rung (~5 LOC at the 3-4 generator function-decl/expr sites) but separate from YIFP-EXT 1's check.

**Status**: YIFP-EXT 1 CLOSED. test262 target test passes; cluster 8/45 → 9/45.
