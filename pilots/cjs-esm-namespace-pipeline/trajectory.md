# cjs-esm-namespace-pipeline - Trajectory

## CENP-EXT 0 — founding + pipeline design (2026-05-31, design-only)

**Trigger**: keeper directive to pivot the package-parity oracle from Bun to Node and architect CJS→ESM namespace convergence to Node. Authored under helmsman appointment (keeper-substituted arbiter approval).

**Baseline-inspect (Phase 2)**: the 2026-05-31 top500 Node sweep (`legacy/host-rquickjs/tools/parity-results-top500-node-2026-05-31.json`) — 38.7% parity vs Node (88.0% vs Bun). Of 596 Node failures, **503 (84%) are CJS→ESM namespace-interop policy**: 297 fn-object intrinsic noise, 137 missing Node fixed-keys (`module.exports` alias, `__esModule`), 68 over-export (runtime-enum vs static lexer), 1 under-export. Inspection of the two namespace-construction sites (`cruftless/src/module_ns.rs` ESM finalize; `module.rs::populate_cjs_namespace_view_at` CJS path) confirmed both are deliberately Bun-tuned (P53.E11/E13 fn-lift; P38.E1/P40/P41 heuristics) — the "failures" are correct Bun-parity behavior that is wrong for Node.

**Move-shape correction (Rule 23)**: the naive plan ("fix the 297 fn-intrinsic bug, +30pts") was wrong — fn-lift is intentional Bun parity, not a bug. The real move-shape is an architectural one: re-coordinate the namespace layer into a phase-aligned resolver pipeline (Stage L Node-static name set / Stage E evaluate / Stage W additive Bun-widening), with Node canonical and the Bun heuristics demoted downstream rather than deleted.

**Artifact**: `seed.md` + `design.md` (full first-principles + three-stage pipeline + proposed rung sequence CENP-EXT 1–4). No runtime substrate.

**Result (R)**: design landed; next rung (CENP-EXT 1, Stage L base on the CJS path) is its own future proposal and is NOT authorized by this founding round. Manifest refreshed.

**Composes-with**: Doc 729 (resolver-instance pattern), Doc 731 (alphabet purity upstream), `cjs-ns-shape-diff-residual` (prior Bun-shape analysis), `rusty-js-esm` (linker substrate), the Node-reference harness `parity-measure-ref.sh`.

---

## CENP-EXT 1 — Stage-L integration (Node-mode), env-gated (2026-05-31)

**Trigger**: wire the CMLD `cjs_lex` scanner into the CJS namespace path and measure the real top500-vs-Node delta.

**Substrate**: `module.rs::populate_cjs_namespace_view_at` gains an env-gated Stage-L path. When `CRUFT_CJS_INTEROP=node`, the namespace is built from the static lexer name set (source read from `url`) + Node's fixed keys (`default`, `module.exports`, optional `__esModule`), matching `import(pkg)` under Node. Default (unset/other) preserves the existing Bun-shaped runtime-enumeration behavior — an early return gated solely on the env var, so default behavior is provably unchanged (Rule 5+10: no default-on flip without canonical fuzz; the flip is a later rung). Helper `cjs_interop_node_mode()` added. No populate signature change (reads source from `url`).

**Probe confirmation**: `parity-probe.mjs` is pure `Object.keys(import(pkg))` — so Node's literal `module.exports` namespace key is real, not an artifact; Stage-L synthesizes it.

**Gate (measured)**: full top500 sweep in Node-mode (`node-mode-top500-2026-05-31`): **77.4% parity vs Node** (754/974), up from the 38.7% Bun-shaped baseline — **+38.7 points** from R1+R2 alone. Default-mode spot-check unchanged (lodash 312, debug 24, semver 47 = prior Bun numbers). Per-package: lodash/ms/uuid/chalk exact Node match.

**Residual (named follow-ups, not regressions)**: 218 Node-mode failures cluster as (a) R5 reexport-stars — fs-extra 34→2, rxjs 176→7, ajv 13→5, debug (names arrive via `module.exports = require(...)`); (b) small off-by-few — yup 32/30, io-ts 89/87, superstruct 53/52 (R3 defineProperty / R4 transpiler-prologue precision); (c) arktype ERR (separate engine bug). semver shows a +6 R2 over-detection to chase.

**Result (R)**: Stage-L integration landed behind the Node-mode gate; architecture validated with a +38.7pt measured delta and zero default-mode regression. Next levers: R5 (largest), then R3/R4 precision, then the eventual default-flip rung (gated on canonical fuzz).

**Composes-with**: CMLD `cjs_lex` (R1+R2 scanner), Node baseline `parity-results-top500-node-2026-05-31.json`, Node-mode result `node-mode-top500-2026-05-31`.
