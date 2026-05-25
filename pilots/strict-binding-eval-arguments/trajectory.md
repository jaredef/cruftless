# strict-binding-eval-arguments — Trajectory

## SBEA-EXT 1 — early-error: `eval`/`arguments` as binding-id in strict (2026-05-25)

**Trigger**: NSPS.2 sibling list. test262 cluster: `for-in/var-arguments-strict.js`, `var-eval-strict.js`, `var-arguments-fn-strict.js` (all `flags:[onlyStrict]`). Spec §13.2 BindingIdentifier early errors: in strict mode, `eval` and `arguments` are reserved as binding identifiers.

**Edits** (~30 LOC across 3 sites):

The check fires when `strict_mode` is true AND the just-seen identifier name is `"eval"` or `"arguments"`. Three parser sites had to be updated:

- `parser.rs::parse_binding_identifier` — covers function names, import bindings, etc.
- `parser.rs::parse_binding_target` Ident branch — covers var/let/const + destructuring-leaf-identifier.
- `stmt.rs::parse_for_statement` for-(var|let|const) plain-identifier head path (lines 701-722) — this site constructs `BindingIdentifier` inline without going through either helper above; bypassed the SBEA check until this rung added the explicit guard.

**Verification**:

| Probe | Before | After |
|---|---|---|
| `function f() { "use strict"; var arguments; }` | accepted | SyntaxError |
| `function f() { "use strict"; var eval; }` | accepted | SyntaxError |
| `function f() { "use strict"; let arguments; }` | accepted | SyntaxError |
| `function f() { "use strict"; const eval = 1; }` | accepted | SyntaxError |
| `function f() { var arguments; }` (sloppy) | accepted | accepted |
| `function f() { "use strict"; for (var arguments in null) {} }` | accepted | SyntaxError |
| test262 SyntaxError cluster (45 tests) | 5/45 | **8/45** |
| Specific 3 named tests | 0/3 | **3/3** |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding SBEA.1 (binding-id parse sites are not centralized)**: cruft has at least three distinct binding-id parse paths: `parse_binding_identifier` (function names/imports), `parse_binding_target` Ident branch (var/let/const + destructuring leaves), and the inline `BindingIdentifier { name, span }` literal in `parse_for_statement` (for-head plain-id). A single-site fix in `parse_binding_identifier` missed the var-decl path; adding to `parse_binding_target` missed the for-head path. Standing recommendation: at any future binding-id-validation rung, grep for `BindingIdentifier {` constructions across the parser, not just for the two parse-helper functions.

**Finding SBEA.2 (param-level eval/arguments deferred)**: the test cluster `param-arguments-non-simple-strict.js` etc. would need params to know they'll be strict before parsing. Today params parse with `strict_mode=false` then body's `"use strict"` promotes. NSPS-EXT 1 throws on non-simple-params + strict-body; the parallel rule for simple-params-but-name-is-arguments-and-body-is-strict would need a re-scan of the parsed params at body-entry (or to defer the binding-id check until body-entry knows the directive). Separate sibling locale.

**Finding SBEA.3 (function name strict check still partial)**: `"use strict"; function eval() {}` still accepted because `parse_binding_identifier` is called from the function-decl name-parse path BEFORE strict_mode promotes via the body's directive. The function-decl name belongs to the enclosing scope's strict-mode, which at top-level is the directive-prologue strict from the script start. That works in our impl (the runner injects `"use strict"` at the top so strict_mode is true by the time function eval() is parsed). Probe `eval('"use strict"; function eval() {}')` evaluates inside an eval call — the inner script's strict-mode parsing should see "use strict" first and reject. This may be a different gap; not in scope for SBEA-EXT 1.

**Status**: SBEA-EXT 1 CLOSED. 3 test262 tests pass; cluster 5/45 → 8/45.
