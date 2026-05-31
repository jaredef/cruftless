# CENP Design — CJS→ESM namespace construction as a resolver pipeline

**Status**: design-only (CENP-EXT 0). No runtime substrate authorized by this document.
**Composes-with**: Doc 729 (resolver-instance pattern), Doc 731 (alphabet purity upstream as the bound on complexity), Doc 730 (vertical recurrence of the lowering closure), `cjs-ns-shape-diff-residual` (prior Bun-referenced shape analysis), `rusty-js-esm` (linker substrate).

## 1. Problem statement

cruft's CJS→ESM namespace construction is tuned to match Bun. Against Node — the actual compatibility target per CLAUDE.md project identity — the 2026-05-31 top500 sweep scored **38.7%** (vs 88.0% against Bun), and **503 of 596 failures (84%) are this single interop-policy surface**. The remaining ~84 are genuine engine bugs (out of scope here).

Node and Bun resolve the ESM/CJS impedance mismatch at different lifecycle phases (see seed.md §II). cruft cannot match both with one flat policy. The keeper directive: converge to Node as the principal target, treating the divergence as a real gap, while keeping Bun-compat reachable.

## 2. The mismatch, precisely

| | export-name set determined | mechanism | named exports for `lodash` |
|---|---|---|---|
| **ESM spec** | at Link (before eval) | static, from module record | n/a (lodash is CJS) |
| **Node** | at Link | `cjs-module-lexer` static scan | `default` only (dynamic assigns invisible) |
| **Bun** | at Evaluate | live `module.exports` enumeration | all 312 keys |
| **cruft (today)** | at Evaluate | live enumeration + per-pkg heuristics | 312 (matches Bun) |

Concrete divergences observed against Node, by sub-cluster of the 503:
- **297** — fn-object intrinsic noise: cruft surfaces `name`/`length`/`prototype` when `module.exports` is a function; Node never does (it adds them only if the lexer/`__esModule` path warrants).
- **137** — cruft missing Node's fixed keys: the literal `module.exports` alias key and `__esModule` passthrough.
- **68** — cruft over-exports: runtime enumeration vs Node's static lexer set (lodash 312 vs 2; debug 24 vs 11).
- **1** — cruft under-exports.

## 3. The pipeline architecture

CJS→ESM namespace construction is itself a resolver-instance stage chain (Doc 729). Decompose it to mirror the ESM lifecycle phases:

```
Stage L (Link / upstream)
  Input:  CJS module source text (pre-evaluation) + package.json conditions
  Emit:   the STATIC export-name set, Node-faithful:
            - cjs-module-lexer-derived named exports (static scan)
            - always: `default` = module.exports value (bound at Stage E)
            - always: `module.exports` alias key (Node's literal)
            - passthrough: `__esModule` when the source sets it
  Property: link-correct — names known before evaluation, supporting live
            bindings and circular ESM↔CJS imports.

Stage E (Evaluate)
  Input:  the Stage L name set + the executed module
  Emit:   values bound into the link-determined namespace slots.
          `default` resolves to the final module.exports value.

Stage W (Widen / downstream, OPTIONAL, additive)
  Input:  the Node-correct namespace from L+E + the live module.exports object
  Emit:   Bun-style ergonomic widening — runtime-enumerated keys NOT in the
          Node set, layered additively. Never overwrites a Stage-L binding;
          never alters the Node-visible projection when disabled.
  Gate:   off by default (Node is canonical); enabled by an explicit
          compatibility condition (flag / package allowlist / config).
```

### 3.1 Why Node at Stage L is not a preference but a correctness requirement

The ESM spec fixes export names at Link for live bindings and circular imports. A runtime-enumeration policy (Bun's) cannot satisfy this for true ESM↔CJS cycles because the names don't exist until eval. Putting the Node-faithful static set at Stage L is therefore the spec-correct base, and it bounds downstream complexity (Doc 731): get the static set right and Stage W is a pure additive projection rather than a competing policy with per-package reconciliation.

### 3.2 Sanctioned upstream divergence

Per keeper framing: cruft may diverge from Node *at Stage L* only when doing so strictly enables downstream Node-compat. Example: Stage L may carry a *superset* name set (Node's lexer names ∪ a cruft-derived richer set) as long as the **Node-visible projection is exact**. The extra names are an upstream affordance Stage W can expose; they must not leak into the canonical Node projection.

### 3.3 Stage W as the home for the Bun investment

The current heuristics (P53.E11/E13 fn-lift in `module_ns.rs`; P38.E1/P40/P41 in `populate_cjs_namespace_view_at`; Tuple-A/B; the enquirer filter) are *not deleted*. They are re-coordinated as Stage W's widening logic — moved downstream of the Node-correct base, behind the compatibility gate. This preserves the prior parity work and makes it auditable as one stage rather than diffused through the populator.

## 4. Proposed rung sequence (each its own future proposal; NOT authorized here)

1. **CENP-EXT 1 — Stage L base (CJS path).** Replace runtime-enumeration default with a Node-faithful static name set in `populate_cjs_namespace_view_at`: cjs-module-lexer-derived names + `default` + `module.exports` alias + `__esModule`. Demote the existing fn-intrinsic/over-export heuristics behind a Stage-W gate (default off). Measure Node parity delta on the top500 basket.
2. **CENP-EXT 2 — Stage L base (ESM path).** Apply the same separation in `module_ns.rs`: the Node-correct finalize behavior canonical; P53 fn-lift demoted to Stage W.
3. **CENP-EXT 3 — Stage W widening.** Re-home the demoted heuristics as an explicit, gated additive projection; verify Bun parity is recoverable when the gate is on.
4. **CENP-EXT 4 — convergence measurement.** Re-run cruft-vs-Node and cruft-vs-Bun sweeps; confirm Node parity rises toward the ~84%-recoverable ceiling and Bun parity is preserved under the gate.

## 5. Risks and unknowns

- **cjs-module-lexer fidelity.** Node's static export detection has edge cases (re-export chains, `Object.defineProperty(exports,...)`, `module.exports = require(...)`). Stage L must derive these under Pin-Art discipline against observed Node output, not approximate loosely. Under-detection regresses named-import compat vs Node itself.
- **Live-binding depth.** Full ESM↔CJS circular live-binding correctness is a larger substrate question than the namespace name set alone; CENP scopes the name-set surface first and defers deep live-binding to a cross-reference if Phase 5 surfaces it.
- **Blast radius.** The current heuristics close specific top500 packages against Bun. Demoting them must be measured so the Bun-gated path doesn't silently regress; the existing CNSDR analysis is the cross-check.
- **Two sites, one policy.** The CJS and ESM paths are separate code sites with separately-evolved heuristics. The pipeline must apply consistently to both or the namespace shape diverges by load path.

## 6. What this design does NOT claim

It does not claim the 297 fn-intrinsic cases are "bugs" in cruft's prior frame — they were correct Bun-parity behavior. It claims they are wrong *for Node*, and that the pipeline architecture is how cruft serves Node-as-principal without discarding the Bun work.
