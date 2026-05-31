# cjs-esm-namespace-pipeline - Seed

**Locale tag**: `L.cjs-esm-namespace-pipeline` (CENP).

**Status**: FOUNDED at CENP-EXT 0 (design-only). Phase 1 spawn + Phase 2 baseline-inspect + design articulation; no runtime substrate lands in this founding round.

**Parent arc**: CommonJS↔ESM interoperability convergence to Node. Adjacent to `cjs-ns-shape-diff-residual` (CNSDR, the prior Bun-referenced shape-diff residual analysis) and the `rusty-js-esm` substrate pillar, but scoped to the *policy architecture* of CJS→ESM namespace construction with Node — not Bun — as the principal compatibility oracle.

## I. Telos

Establish the resolver-pipeline architecture for CJS→ESM namespace construction such that cruft converges to Node's named-export semantics (the actual compatibility target per CLAUDE.md project identity) as the canonical contract, while preserving the prior Bun-parity investment as a separable, downstream, additive widening stage.

The motivating measurement: the 2026-05-31 top500 sweep against Node (`legacy/host-rquickjs/tools/parity-results-top500-node-2026-05-31.json`) scored 38.7% parity vs 88.0% against Bun. **84% of the 596 Node failures (503) are the CJS→ESM namespace-interop policy difference**, not engine bugs. cruft's namespace layer is currently tuned to Bun (the prior oracle); Node has the opposite policy and cruft cannot match both with a single flat policy.

## II. First principles (why Node and Bun differ)

The root cause is an ESM/CJS impedance mismatch resolved at different lifecycle phases:

- **ESM is Parse → Link → Evaluate.** The spec requires the export-name set to be *statically fixed at Link, before evaluation* — load-bearing for live bindings and circular imports.
- **CJS is Evaluate-only.** Export names are a runtime product of mutating `module.exports`.
- **Node resolves at Link**: `cjs-module-lexer` statically scans source text to approximate the named exports without executing. Preserves phase ordering and live-binding/circular correctness. Cost: dynamically-assigned exports (lodash's mixin loop) aren't named exports; you get `default` only. Optimizes spec fidelity.
- **Bun resolves at Evaluate**: eagerly runs the CJS module, enumerates the live `module.exports` object, exposes every key. Optimizes ergonomics (`import { map } from 'lodash'` works). Cost: not phase-correct, leaks function intrinsics (`name`/`length`/`prototype`), needs per-package filtering heuristics.

These are not competing policies; they are resolutions at two different pipeline phases. Node = link-phase static name set. Bun = eval-phase value enumeration.

## III. The pipeline thesis (see design.md for full articulation)

```
Stage L (Link / upstream)   → Node-faithful STATIC export-name set
                              (cjs-module-lexer name set + default + module.exports alias + __esModule)
Stage E (Evaluate)          → values populate link-determined bindings
Stage W (downstream, opt)   → Bun-style ergonomic WIDENING (runtime-enumerated keys),
                              additive over the Node-correct base, never corrupting it
```

Node is the principal target at Stage L because matching it there is what makes cruft's ESM linker spec-correct (alphabet purity upstream as the bound on downstream complexity, Doc 731). Sanctioned divergence from Node upstream only when it strictly *enables* downstream Node-compat. Bun-compat becomes a downstream additive projection (Stage W), not a root policy.

## IV. Apparatus

- Node-reference parity harness: `legacy/host-rquickjs/tools/parity-measure-ref.sh` (`REF_BIN=node`).
- Node baseline artifact: `legacy/host-rquickjs/tools/parity-results-top500-node-2026-05-31.json`.
- The two current namespace-construction sites (Bun-tuned, to be re-architected):
  - ESM path: `cruftless/src/module_ns.rs` (FinalizeModuleNamespace hook; P53.E11/E13 fn-lift, Tuple-A/B).
  - CJS path: `pilots/rusty-js-runtime/derived/src/module.rs::populate_cjs_namespace_view_at` (P38.E1/P40/P41 heuristics).
- Prior Bun-referenced analysis: `pilots/cjs-ns-shape-diff-residual/` (CNSDR design.md + classifier report).

## V. Carve-Outs

- CENP-EXT 0 is design-only; no runtime substrate lands.
- The genuine engine bugs surfaced by the same sweep (callee-not-callable cluster, empty/crash, CJS-wrapper parse) are NOT in scope here; they fail against both Bun and Node and route to their own locales.
- The Bun-parity heuristics are not deleted by this locale's design; they are re-coordinated into Stage W. Any actual demotion is a later substrate rung under its own proposal.
- `cjs-module-lexer` semantics are to be *derived* under Pin-Art discipline against the lexer's documented behavior + Node's observed output, not vendored.

## VI. Resume Protocol

Read this seed, then `design.md` (the full three-stage articulation + first-principles), then `trajectory.md`. The Node baseline JSON + the two namespace-construction source sites are sufficient to begin scoping the Stage L substrate rung. Compose against `cjs-ns-shape-diff-residual` for the prior Bun-shape analysis and `rusty-js-esm` for the linker substrate.
