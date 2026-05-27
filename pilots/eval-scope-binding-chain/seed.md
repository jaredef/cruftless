---
name: eval-scope-binding-chain
description: cruft's indirect-eval runs Module semantics instead of Script semantics — top-level var declarations don't reach globalThis. Per ECMA-262 §19.2.1.3 PerformEval (indirect branch).
type: project
---

# eval-scope-binding-chain — Seed

## Substrate-pilot — top-level (substrate, not under any Tier-L parent).

Spawned per keeper directive (Telegram 9975 selecting Option A — structural fix) inside the 2026-05-27-eval-scope-binding-chain arc.

## Telos

cruft's indirect-eval (`(0, eval)(source)`) per ECMA-262 §19.2.1.3 PerformEval must run the eval source as a Script in the realm's global scope. Top-level `var` declarations attach to the realm's variable environment (= globalThis for Scripts). Current cruft uses Module semantics — top-level var stays module-local and never reaches globalThis.

## Failure surface

Identified during HDSB residual analysis (Finding HDSB.2, 2026-05-26). 120 records inside `assert / fnGlobalObject not defined` failures inside HDSB exemplars alone; predicted 200-400+ engagement-wide.

Founding-probe confirmed:
- `(0, eval)("var foo = 42;"); globalThis.foo` → undefined (var never reached globalThis)
- Inner eval pattern `eval('fnGlobalObject()')` in tests fails because outer-eval-frame's bindings aren't on globalThis

## Apparatus

- `pilots/rusty-js-bytecode/derived/src/lib.rs::compile_script_with_url` — new entry (ES-EXT 1 foundation).
- `pilots/rusty-js-runtime/derived/src/module.rs::evaluate_script` — new entry (ES-EXT 1 foundation).
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` indirect-eval site — wired to call evaluate_script (ES-EXT 1).
- ES-EXT 2 (compile-tier): Compiler script_mode flag; top-level VariableDecl emits StoreGlobal instead of StoreLocal.
- ES-EXT 3 (runtime-tier): Frame setup for Script mode uses realm's global env as the top-level scope.

## Sub-rung topology

| Rung | Scope | Status |
|---|---|---|
| ES-EXT 1 / es-foundation | New evaluate_script + compile_script_with_url entry points (currently delegate to module); indirect-eval wired through. No semantic change. | LANDED |
| ES-EXT 2 / es-compile-script-var-to-global | Compiler.script_mode flag; top-level VariableDecl → StoreGlobal | NOT SPAWNED |
| ES-EXT 3 / es-runtime-script-frame-scope | Runtime frame setup for Script-mode top-level scope | NOT SPAWNED |

## Status

ES-EXT 1 LANDED 2026-05-27. Foundation rung — structural sub-program in place; no semantic change yet (still delegates to module path). ES-EXT 2 + ES-EXT 3 are the semantic-change rungs that actually close the predicted 200-400 records.
