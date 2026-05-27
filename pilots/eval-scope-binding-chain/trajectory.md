# eval-scope-binding-chain — Trajectory

## ES-EXT 0 — FOUNDED (2026-05-27)

Spawned per keeper directive (Telegram 9975 / Option A) inside the
2026-05-27-eval-scope-binding-chain arc. See arc.md for the full
founding-probe + 5-probe enumeration of the indirect-eval bug shape.

## ES-EXT 1 — foundation rung LANDED (2026-05-27)

Edit ~30 LOC across 3 files:
* pilots/rusty-js-bytecode/derived/src/lib.rs: new compile_script_with_url
  entry (currently delegates to compile_module_with_url).
* pilots/rusty-js-runtime/derived/src/module.rs: new evaluate_script entry
  (currently delegates to evaluate_module).
* pilots/rusty-js-runtime/derived/src/intrinsics.rs: indirect-eval site
  switched from rt.evaluate_module to rt.evaluate_script. Two call sites
  (expression-form + statement-form).

Yield: 0 (no semantic change; foundation rung establishes substrate-
program structure only). Diff-prod 42/42 maintained — verified no
regression since the new entry points delegate identically.

Probes confirm wiring: `(0, eval)("console.log(1+1)")` → 2 (works
unchanged, routed through evaluate_script).

Standing recommendation: ES-EXT 2 (compile-tier flag for top-level var
→ StoreGlobal) is the next substrate move. Substantial (~150-300 LOC
threading script_mode through Compiler + adjusting VariableDecl emission
paths). ES-EXT 3 (runtime frame scope setup) follows.

## Status

ES-EXT 1 CLOSED. Substrate program shape in place. ES-EXT 2 + ES-EXT 3
are the semantic-change rungs that close the predicted 200-400 records.
