# 2026-05-27-eval-scope-binding-chain — log

## 2026-05-27 — arc opens

- Telegram 9973: keeper directive "Now select an arc" after Tier-M candidate register landed.
- Selected ESBC from 6 Tier-M candidates: largest non-Temporal yield prospect (predicted 200-400+ records), well-scoped to single mechanism, twice-deferred.
- Arc spawned per arc-as-coordinate.md formalization (first arc spawned at directive-time, not retroactively).

## 2026-05-27 — founding-probe (Rule 23)

### Probes

1. `(0, eval)("var foo = 42;"); foo` → ReferenceError (foo not in script scope after indirect eval)
2. `(0, eval)("var foo = 42;"); globalThis.foo` → undefined (foo never reached globalThis)
3. `(0, eval)("var foo = 42; eval('console.log(foo)')")` → undefined (inner direct eval doesn't see outer's var)
4. Runner-style `(0, eval)(harness + test)` works (one eval, one scope)
5. Inner-eval pattern in test `eval('fnGlobalObject()')` fails because outer-eval-frame's bindings aren't on globalThis

### Root cause

**cruft's indirect-eval uses `evaluate_module()` for the eval body, which runs Module semantics. Modules per spec keep top-level `var` declarations as MODULE-LOCAL bindings — they do NOT attach to globalThis.**

Per ECMA-262 §19.2.1.3 PerformEval (indirect-eval branch): the eval source runs as a Script in the realm's global scope. Top-level `var` declarations MUST be added to the realm's variable environment (which IS the global object for Scripts). cruft is running Module semantics instead.

Substrate site: `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_globals` Function('eval') closure (around line 2034) calls `rt.evaluate_module(...)`. The fix needs Script-semantics execution, not Module.

### Substrate scope (honest assessment)

Two implementation paths:

**Option A — Structural fix (multi-day)**: Implement a `evaluate_script` entry point in `rusty-js-runtime` that parses+lowers as Script (top-level var → global env). New module-equivalent path through bytecode emission with different scope-class for the top-level frame. Substantial.

**Option B — Targeted workaround (~50-100 LOC)**: After `evaluate_module(eval_source)` returns, walk the eval-frame's top-level var declarations (already in the parsed AST) and copy each to globalThis. Approximates Script semantics; misses edge cases (lexical decls, function decls also need to be checked). Quick.

**Option C — Pre-wrap source (~30 LOC, hacky)**: Pre-process the eval source string before invoking evaluate_module, rewriting top-level `var X` to `globalThis.X` semantics. Brittle.

Per discipline (no destructive shortcuts), Option A is correct but exceeds per-rung budget. Option B is the substrate-equivalent of WBMS-EXT 1's parser-only carve-out. Option C is rejected.

## 2026-05-27 — checkpoint before substrate commit

Arc opened + founding probe complete. Substrate decision (A vs B) awaits keeper alignment per arc-as-coordinate.md "keeper-directed multi-locale program" rhythm. Probe finding (Module-vs-Script execution-mode mismatch) is larger than original HDSB.2 prediction estimated.

## 2026-05-27 — ES-EXT 1 LANDED (foundation rung; Option A start)

Per keeper directive (Telegram 9975) selecting Option A — structural fix.

* Spawned `pilots/eval-scope-binding-chain/` (top-level substrate locale) + `es-foundation/` nested sub-rung.
* Added `evaluate_script` (rusty-js-runtime/module.rs) + `compile_script_with_url` (rusty-js-bytecode/lib.rs) entry points, currently delegating to module-path.
* Wired indirect-eval (`intrinsics.rs` Function('eval') closure, both expression-form and statement-form call sites) through `evaluate_script`.

Yield: 0 (foundation; no semantic change). Diff-prod 42/42 maintained.

ES-EXT 2 (compile-tier `script_mode` flag, top-level VariableDecl → StoreGlobal) and ES-EXT 3 (runtime-tier frame setup with realm's global env as top-level scope) are the semantic-change rungs that will close the predicted 200-400 records.
