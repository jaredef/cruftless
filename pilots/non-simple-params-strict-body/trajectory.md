# non-simple-params-strict-body — Trajectory

## NSPS-EXT 1 — early-error: "use strict" in body with non-simple params (2026-05-25)

**Trigger**: SyntaxError-residual audit. test262 cluster of arrow-function tests probed `({a}) => { "use strict"; }`, `(...rest) => { "use strict"; }`, `(x = 1) => { "use strict"; }`, and the parallel function-declaration shapes. All should throw SyntaxError at parse per ECMA-262 §15.2.1 / §15.3.1; cruft accepted them silently.

**Edits** (~20 LOC):

- `stmt.rs`: new `parse_function_body_gs(is_generator, is_simple)` underlying helper; thin `parse_function_body_g` wraps with `is_simple=true` for static-block + entry-point callers.
- `stmt.rs`: new `Parser::is_simple_param_list(&[Parameter]) -> bool` per §15.2.1.4 (all params are `Identifier` && `default.is_none()` && `!rest`).
- 9 call sites (function-decl, function-expr, generator-method, async-method, getter/setter, plain-method, arrow-block, default-export-function) switched from `parse_function_body_g` to `parse_function_body_gs` with `Self::is_simple_param_list(&params)`.
- New ParseError raised inside `parse_function_body_gs` when `peek_use_strict_directive()` AND `!is_simple` — before strict-mode promotion, before any body parse.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `({a}) => { "use strict"; }` | accepted | SyntaxError |
| `(...rest) => { "use strict"; }` | accepted | SyntaxError |
| `(x = 1) => { "use strict"; }` | accepted | SyntaxError |
| `function f({a}) { "use strict"; }` | accepted | SyntaxError |
| `(x) => { "use strict"; }` (simple, control) | accepted | accepted |
| test262 "expected SyntaxError, got String" cluster (45 tests) | 0/45 | **4/45** |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding NSPS.1 (early-error fence at body-entry is the right seam)**: rather than tag the FunctionDecl/ArrowFunction AST node with non-simple/has-strict-directive flags for a separate post-parse validation, the substrate detects both at the body-entry parse site where the directive is already being peeked. ~3 LOC of additional check at the existing peek_use_strict_directive branch. Standing recommendation: when a spec's early error couples two parse-time facts that are already independently observed at one seam, fence the error at that seam — not at a separate AST-validation pass.

**Finding NSPS.2 (residual SyntaxError cluster decomposition)**: 41/45 SyntaxError tests still fail. Spot-check shows other parser-tier gaps:
- `dflt-params-rest.js`: param default referencing later rest param (TDZ in param scope)
- `param-dflt-yield-expr.js`: yield in param default (yield not allowed in params per §15.3.1)
- `rest-params-trailing-comma-early-error.js`: `(...rest,) => 0` — rest can't have trailing comma
- `asi-restriction-invalid.js`: `a\n=>b` — no LineTerminator before `=>`
- `var-arguments-fn-strict.js`: `var arguments` in strict function body
- `head-const-bound-names-let.js`: `for (const x of [x])` TDZ in head
- `array-elem-nested-array-invalid.js`: invalid AssignmentPattern inside for-of head
- `dstr/obj-id-init-let.js`: TDZ in destructuring default

Each is a separate parser-tier early-error rule; the SyntaxError cluster is not single-shape but a cluster of cluster. Standing recommendation: treat each rule as a separable sibling locale; do not estimate the cluster as one bug.

**Status**: NSPS-EXT 1 CLOSED.
