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

---

## YIFP-EXT 2 — generator's own params: in_generator=true around param parse (2026-05-25)

**Trigger**: YIFP.2 named the parallel `function* g(x = yield) {}` case as a small follow-on. Top-level generator parse: enclosing `in_generator` is false; the yield-branch's gate (`in_generator || strict_mode`) doesn't fire; yield parses as identifier. Spec §15.5.1 mandates SyntaxError ("FormalParameters Contains YieldExpression is true" is an early error for GeneratorDeclaration).

**Edits** (~15 LOC):

- `stmt.rs`: `parse_function_parameters` becomes thin wrapper over new `parse_function_parameters_g(is_generator)`. The `_g` variant additionally saves/sets `in_generator=true` for the param-parse duration when its arg is true.
- 5 generator-eligible call sites updated to pass the local `is_generator`:
  - `stmt.rs::parse_function_decl` (line 284) — function decl
  - `stmt.rs::parse_class_body` class-method site (line 507) — class methods (including `*name()`)
  - `parser.rs::parse_default_function` (line 367) — `export default function*`
  - `expr.rs` generator-method shorthand (line 728) — `{ *name(){} }`
  - `expr.rs` async-method shorthand (line 758) — `{ async *? name(){} }`
  - `expr.rs::parse_function_expression` (line 1199)
- Non-generator-eligible sites (getter/setter at 787, plain method at 818, single-arrow-ident path) call `parse_function_parameters()` (=`_g(false)`) unchanged.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `function* g(x = yield) {}` at top level | accepted | SyntaxError ✓ |
| `function* g() { yield 1; }` (yield in body) | works | works |
| Sloppy `function f(x = yield) {}` (non-generator) | accepted (yield=ident) | accepted (unchanged) |
| `var f = function*(x = yield) {}` (generator expression) | accepted | SyntaxError ✓ |
| `class C { *m(x = yield) {} }` (generator method) | accepted | SyntaxError ✓ |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**
- test262 SyntaxError cluster: 9/45 (unchanged — this shape isn't represented in the sample's residuals)

**Findings**

**Finding YIFP.3 (substrate-gap closure ≠ test-cluster movement)**: YIFP-EXT 2 closed a real spec gap (verified by probe across 5 generator forms) without moving the test262 cluster. The sample doesn't probe this shape directly. Standing recommendation: don't gate substrate fixes on cluster-movement signal alone; the test262 cluster is one observation among many, and spec-completeness on a bounded position is its own validation.

**Finding YIFP.4 (parse-time mode propagation is a recurring pattern)**: the substrate now has three parser-flag pairs that propagate over a parse region — `strict_mode`, `in_generator`, `in_function_params` (this rung). Each follows the same save/set/restore convention. A future refactor could factor a `ParserModeGuard` RAII helper to enforce the pattern uniformly; until then, the convention is documented per-flag.

**Status**: YIFP-EXT 2 CLOSED. Spec gap closed across all 5 generator-eligible param-parse sites.
