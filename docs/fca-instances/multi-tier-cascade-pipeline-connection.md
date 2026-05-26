# Multi-Tier Cascade Closure → Pipeline-Connection Yield

## Induced property

When the hot path traverses multiple tiers (each with its own dispatch-shape and per-call cost), closing constraints along the chain in dependency order **connects the pipeline**: cumulative reclaim materializes only when all relevant tiers are closed. Each single-tier closure alone produces partial reclaim; the composition delivers the engagement-tier yield property.

Anchor: [Doc 740](../corpus-ref/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md). Empirical anchor: the LeJIT IHI → GPI → IPBR cascade on the `string_url_sweep` CRB fixture. 743 → 685 → 584 ms (-21.4% cumulative, -7.8% IHI, -4.4% GPI, -14.7% IPBR; cruft/node 8.21× → 7.83× → 6.99× → 6.21×, first sub-6.5×).

## The accumulation

The cascade IS a Fielding-style stack at the JIT-tier substrate, applied across pilot succession rather than across architectural layers:

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) interp dispatch + naive method-call path | — | per-iter cost ~260ns; 8.21× cruft/node baseline |
| 1 | **IHI (interp hot-intrinsics) table** — IHI_TABLE entries map (intrinsic-key, receiver-kind) to a fast handler; per-call dispatch shortcuts the generic dispatcher | constant-table fast-path | property: "hot intrinsic-method calls dispatch in O(1) instead of O(prototype chain)" — -3.6% CRB, first sub-8× |
| 2 | **GPI (GetProp method IC bytecode rewrite)** — Op::GetPropSkipForMethod rewrites the bytecode at the IC-hit branch, eliminating one full GetProp dispatch per call | per-bytecode-site amortization | property: "the GetProp half of method-dispatch lowers to one IcCached lookup; companion to IHI" — -4.4% additional, first sub-7× |
| 3 | **IPBR (iter-protocol bytecode rewrite)** — IterInit/IterNext envelope rewritten to IterFastLoop for Array/String receivers where the iterator is the well-known intrinsic | for-of envelope elimination | property: "per-`.next()` synthetic iterator-result object allocation is eliminated; index-based scan replaces protocol envelope" — -14.7% additional, first sub-6.5× |
| 4 *(implicit)* | **Doc 740 dependency-order discipline** — close upstream first (IHI must land before GPI's IC entries exist; GPI must land before IPBR can compose at the per-iter dispatcher) | order-of-application constraint | property: "the cascade direction is fixed; reversed-order closure would not produce the composition" |
| 5 *(implicit)* | **Correctness-preserving at each rung** — canonical fuzz + diff-prod gate every rung | per-rung correctness | property: "no rung sacrifices correctness for speed; the composition is unconditionally landed" |

The named composition (1+2+3+4+5) is the **multi-tier cascade closure**. The induced property is the connected pipeline: cumulative -21.4% on `string_url_sweep`, cruft/node ratio sub-6.5× for the first time on this fixture. Each individual rung's reclaim is partial; the composition delivers the engagement-tier property.

Removing constraint 1 (no IHI) means GPI has no IC entries to dispatch through; the GetProp bytecode rewrite has no fast-handler to call.
Removing constraint 2 (no GPI) means each method-call still pays the full GetProp dispatch even though the call-site is hot.
Removing constraint 3 (no IPBR) means the for-of envelope's per-call protocol overhead dominates the post-method-call cost surface.
Removing constraint 4 (skip dependency order) lands rungs in wrong order; partial reclaim may even register negative (IHI-EXT 7's frame-cache-tier landing was -7% before IHI-EXT 11's bytecode-rewrite-tier closure; Doc 739/740 single-tier articulation).
Removing constraint 5 (skip correctness gate) means the cascade is conditional on correctness regression; the composition does not land.

## Tag on the DAG

This is a **JIT-tier substrate coordinate** with multi-rung shape:

```
runtime/jit ::
  E3/intrinsic-call-dispatch ::
  cut/multi-tier-cascade-revival ::
  property/pipeline-connection
```

The pattern's correctness is observable as the cruft/node ratio walking down sub-X× thresholds on the empirical anchor fixture. Each crossing is itself a substrate signal — the engagement uses "first sub-7×" / "first sub-6.5×" as natural progress markers.

The Doc 740 articulation generalizes the empirical to: any hot path traversing K tiers requires K-rung closure to connect. Single-tier closure produces a (K-1) × cost cap on reclaim. The 5-rung BBND parser-arc closures within today's session are a parser-tier analog (the 32-test cluster closed across 7 rungs because no single parser-edge captures the cluster).

## Composes-with

- Doc 740 — primary articulation.
- Doc 739 — single-tier dual.
- Doc 741 — empirical materialization across four sibling pilots.
- Standing rule 13 (revert-then-deeper-layer prospective application) — the C1-C4 conditions are the per-tier substrate-introduction discipline.
- [`docs/fca-instances/resolver-instance-directive-free-artifact.md`](resolver-instance-directive-free-artifact.md) — the resolver-instance pattern is what makes per-tier closure clean.
- `pilots/interp-hot-intrinsics/`, `pilots/interp-getprop-ic/`, `pilots/iter-protocol-bytecode-rewrite/` — the empirical anchor locales.

## Falsification

A multi-tier closure that fails to materialize the predicted cumulative reclaim despite each rung's individual reclaim being correct would falsify the composition's multiplicativity at the JIT tier. Empirically held across the IHI+GPI+IPBR chain; the prediction was -21.4% cumulative (additive: -7.8 + -4.4 + -14.7 ≈ -27 naive sum; actual measurement was lower due to composition non-linearity, which is itself a Doc 740 anticipated effect — the named composition is not the naive arithmetic of per-rung reclaims).
