---
helmsman_session: helmsman-2026-05-31-cenp-principal
proposed_commits:
  - 47840493397d27f60235e451d1399a9edd5ba996
target_branch: main
summary: Found cjs-esm-namespace-pipeline locale + Node-convergence pipeline design (design-only)
risk_class: design
gates_pre:
  test262_full: 72.6% (2026-05-31 full sweep)
  test262_sample: 89.6% (2026-05-31)
  diff_prod: not rerun (design-only)
  per_locale: { top500-vs-node: 38.7%, top500-vs-bun: 88.0% }
gates_post:
  test262_full: not rerun; design-only commit
  test262_sample: not rerun; design-only commit
  diff_prod: not rerun; design-only commit
  per_locale: { top500-vs-node: 38.7% (baseline, unchanged) }
---

## Substrate moves

Commit `34a4b8ba` founds the `cjs-esm-namespace-pipeline` (CENP) locale and lands a design-only architecture for converging cruft's CJS→ESM namespace construction to Node. No runtime source changed.

- **M** (Mechanism) — CJS→ESM namespace construction reframed as a phase-aligned resolver-instance pipeline (Doc 729): Stage L (Link / Node-faithful static export-name set via cjs-module-lexer semantics + `default` + `module.exports` alias + `__esModule`), Stage E (Evaluate / value binding), Stage W (downstream additive Bun-style widening, gated off by default). Node is canonical at Stage L because static link-time name determination is a spec-correctness requirement (live bindings, circular imports), not a preference — alphabet purity upstream as the bound on downstream complexity (Doc 731). The prior Bun-parity heuristics are demoted into Stage W, not deleted.
- **T** (Trigger) — 2026-05-31 top500 Node sweep: 38.7% parity vs Node (88.0% vs Bun). 503 of 596 Node failures (84%) are the CJS→ESM namespace-interop policy difference (297 fn-object intrinsic noise, 137 missing Node fixed-keys, 68 over-export, 1 under-export). Artifact: `legacy/host-rquickjs/tools/parity-results-top500-node-2026-05-31.json`.
- **I** (Implementation-site) — `pilots/cjs-esm-namespace-pipeline/{seed.md, design.md, trajectory.md}`; `apparatus/locales/manifest.json` refreshed. The two future-target runtime sites are named but untouched: `cruftless/src/module_ns.rs` (ESM finalize) and `pilots/rusty-js-runtime/derived/src/module.rs::populate_cjs_namespace_view_at` (CJS path).
- **R** (Result) — design landed; the four-rung sequence (CENP-EXT 1 Stage-L CJS base, 2 Stage-L ESM base, 3 Stage-W widening, 4 convergence measurement) is enumerated but **NOT authorized by this proposal**. Each is its own future proposal with its own gates.

## Risk assessment (helmsman self-evaluation)

- **Move-shape correction applied (Rule 23).** The naive baseline plan ("the 297 fn-intrinsic cases are an outright bug, +30 points from one fix") was inspected and rejected: fn-lift is intentional Bun-parity behavior (P53.E11/E13), correct for Bun, wrong for Node. The locale's real move-shape is architectural, not a point-fix. This proposal records that correction rather than carrying the wrong frame forward.
- **No runtime risk this round** — design-only; gates not rerun by design. The only risk is design misclassification, mitigated by grounding every claim in the committed Node baseline artifact and the two inspected source sites.
- **Forward risks named in design.md §5** for the implementation rungs (not incurred here): cjs-module-lexer fidelity (under-detection regresses named-import compat vs Node), live-binding depth (deferred), demotion blast-radius vs the CNSDR Bun-shape analysis, and two-site policy consistency.
- **Standing rules consulted**: Rule 11 (pre-spawn coverage — checked CANDIDATES.md + manifest, no overlap with `cjs-ns-shape-diff-residual` or `rusty-js-esm`), Rule 23 (baseline-inspect at founding), Doc 729/731 (architecture).

## Composes-with

- **Arcs/locales**: `cjs-ns-shape-diff-residual` (CNSDR — prior Bun-referenced shape-diff residual analysis; its heuristics are the Stage-W candidates), `rusty-js-esm` (linker substrate that owns the ESM link phase), `top500-fast-residual-survey` (the parity instrument that surfaced the gap).
- **Corpus**: Doc 729 (resolver-instance pattern), Doc 731 (alphabet purity upstream as the bound on JIT/engine complexity), Doc 730 (vertical recurrence of the lowering closure).
- **Apparatus**: new Node-reference harness `legacy/host-rquickjs/tools/parity-measure-ref.sh`; Node baseline `parity-results-top500-node-2026-05-31.json`.
- **Deferrals-ledger**: none surfaced this round (design establishes the arc; sub-rungs are enumerated, not deferred).
