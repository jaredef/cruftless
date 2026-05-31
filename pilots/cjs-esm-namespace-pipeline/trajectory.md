# cjs-esm-namespace-pipeline - Trajectory

## CENP-EXT 0 — founding + pipeline design (2026-05-31, design-only)

**Trigger**: keeper directive to pivot the package-parity oracle from Bun to Node and architect CJS→ESM namespace convergence to Node. Authored under helmsman appointment (keeper-substituted arbiter approval).

**Baseline-inspect (Phase 2)**: the 2026-05-31 top500 Node sweep (`legacy/host-rquickjs/tools/parity-results-top500-node-2026-05-31.json`) — 38.7% parity vs Node (88.0% vs Bun). Of 596 Node failures, **503 (84%) are CJS→ESM namespace-interop policy**: 297 fn-object intrinsic noise, 137 missing Node fixed-keys (`module.exports` alias, `__esModule`), 68 over-export (runtime-enum vs static lexer), 1 under-export. Inspection of the two namespace-construction sites (`cruftless/src/module_ns.rs` ESM finalize; `module.rs::populate_cjs_namespace_view_at` CJS path) confirmed both are deliberately Bun-tuned (P53.E11/E13 fn-lift; P38.E1/P40/P41 heuristics) — the "failures" are correct Bun-parity behavior that is wrong for Node.

**Move-shape correction (Rule 23)**: the naive plan ("fix the 297 fn-intrinsic bug, +30pts") was wrong — fn-lift is intentional Bun parity, not a bug. The real move-shape is an architectural one: re-coordinate the namespace layer into a phase-aligned resolver pipeline (Stage L Node-static name set / Stage E evaluate / Stage W additive Bun-widening), with Node canonical and the Bun heuristics demoted downstream rather than deleted.

**Artifact**: `seed.md` + `design.md` (full first-principles + three-stage pipeline + proposed rung sequence CENP-EXT 1–4). No runtime substrate.

**Result (R)**: design landed; next rung (CENP-EXT 1, Stage L base on the CJS path) is its own future proposal and is NOT authorized by this founding round. Manifest refreshed.

**Composes-with**: Doc 729 (resolver-instance pattern), Doc 731 (alphabet purity upstream), `cjs-ns-shape-diff-residual` (prior Bun-shape analysis), `rusty-js-esm` (linker substrate), the Node-reference harness `parity-measure-ref.sh`.
