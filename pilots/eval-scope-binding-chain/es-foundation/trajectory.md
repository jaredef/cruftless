# es-foundation — Trajectory

## ES-EXT 0+1 — LANDED (2026-05-27)

~30 LOC. Establishes evaluate_script/compile_script_with_url entry points;
both currently delegate to module path. Indirect-eval (intrinsics.rs Function
'eval' closure) wired through evaluate_script.

Yield: 0 (no semantic change). Diff-prod 42/42. ES-EXT 2 spawn pending.
