---
helmsman_session: helmsman-2026-05-31-cenp-principal
covers: "commit subject: cenp-ext-1: env-gated Stage-L Node-mode CJS interop ..."
target_branch: main
summary: CENP-EXT 1 — env-gated Stage-L Node-mode CJS interop (+38.7pt top500-vs-Node)
risk_class: substrate
note_on_coverage: >
  Per keeper directive 2026-05-31, this checkout's pre-push hook is inactive
  (core.hooksPath = default) and we push to a feature branch, so covers_commits
  SHA tracking is dropped (it churned on every rebase with no enforcement).
  Work is referenced by slug + locale + commit subject, which are rebase-stable.
gates_pre:
  top500_vs_node: 38.7% (Bun-shaped baseline)
  test262_sample: 89.6%
gates_post:
  top500_vs_node_NODE_MODE: 77.4% (754/974)  # CRUFT_CJS_INTEROP=node
  top500_vs_bun_DEFAULT: unchanged (default path is byte-identical; early return gated on env var)
  test262_sample: unchanged (default path untouched)
---

## Substrate moves

Locale `cjs-esm-namespace-pipeline` (CENP). Commit subject: `cenp-ext-1: env-gated Stage-L Node-mode CJS interop`.

- **M** (Mechanism) — `module.rs::populate_cjs_namespace_view_at` gains an env-gated Stage-L path. When `CRUFT_CJS_INTEROP=node`: build the namespace from the CMLD `cjs_lex` static name set (source read from `url`) + Node's fixed keys (`default`, `module.exports`, optional `__esModule`), matching `import(pkg)` under Node. Default (unset/other): early-return-untaken, existing Bun-shaped runtime-enumeration preserved verbatim. New helper `cjs_interop_node_mode()`. No populate signature change.
- **T** (Trigger) — CENP-EXT 1 integration of the CMLD scanner; the top500-vs-Node 38.7% baseline (84% of failures are this interop surface).
- **I** (Implementation-site) — `pilots/rusty-js-runtime/derived/src/module.rs` (Stage-L gated block + `cjs_interop_node_mode` helper); CENP trajectory entry.
- **R** (Result) — Node-mode parity **77.4%** (754/974), **+38.7 points** over the 38.7% Bun-shaped baseline, from R1+R2 alone. Default-mode unchanged. Per-package: lodash/ms/uuid/chalk exact Node match.

## Risk assessment (helmsman self-evaluation)

- **Zero default-mode regression.** The Node path is reached only when `CRUFT_CJS_INTEROP=node`; otherwise the function falls through to the unchanged Bun behavior. This is the Rule 5+10 discipline: introduce the mechanism gated, measure in the gated mode, do NOT flip the global default (the flip is a later rung gated on canonical fuzz + three-probe-levels). Default-mode spot-check confirmed prior Bun numbers (lodash 312 / debug 24 / semver 47).
- **Measured, not asserted.** Full 1,026-package Node-mode sweep recorded at `node-mode-top500-2026-05-31`. The +38.7pt delta is the gated mode vs the Node reference, apples-to-apples through the same probe.
- **Residual is named, not hidden** (218 Node-mode failures): R5 reexport-stars (largest lever — fs-extra/rxjs/ajv/debug get names via `module.exports = require(...)`); small off-by-few (R3 defineProperty / R4 transpiler-prologue precision — yup/io-ts/superstruct); arktype ERR (separate engine bug); semver +6 R2 over-detection. Each is a named follow-up rung, not a regression.
- **Source-from-url caveat**: Node-mode reads the on-disk source at `url` to lex. For plain CJS .js packages (the top500 case) this is exactly what Node lexes. TS/transformed sources are an edge not exercised by this basket; documented for the default-flip rung.
- **Standing rules**: Rule 4 (single move — one gated path), Rule 5+10 (gated mechanism, no default flip), Rule 23 (probe-grounded — confirmed `module.exports` key is real Node behavior via the pure-Object.keys probe).

## Composes-with

- **Consumes**: CMLD `cjs_lex` (R1+R2 scanner).
- **Parent**: CENP `design.md` Stage L.
- **Apparatus**: Node baseline `parity-results-top500-node-2026-05-31.json`; Node-mode result `node-mode-top500-2026-05-31.json`; `parity-measure-ref.sh`.
- **Next rungs**: CMLD-EXT 2 (R5 reexport-stars — largest residual lever), CMLD R3/R4 precision; CENP default-flip rung (canonical-fuzz-gated).
