# FCA Instances — Fielding Constraint Accumulation in cruftless

Survey of architectural sites in the cruftless engagement where the
Fielding-style constraint-accumulation pattern induces named properties.
Each instance is recorded as a file whose name encodes the shape of the
induced property and whose body articulates:

1. **The accumulation** — the named constraints, in order, each adding a
   constraint to the prior configuration and inducing a property absent
   from the prior configuration. Per Doc 419's reading: the framing is
   "accumulation from the Null style," not "trade-off between options."
2. **The induced property/properties** — what the named composition
   produces that no proper subset of the constraints produces.
3. **Tag on the DAG coordinates** (per Doc 728): the resolver-instance ::
   engine-rung :: cut :: failure-mode pin format used by the test262
   Pin-Art matrix, OR the equivalent apparatus-tier coordinate when the
   instance does not project onto the test262 surface.

The survey is composed in response to keeper directive 9780 after the
prospective doc `cluster-coherence-multiplier-as-sipe-t-instance.md`
extended Doc 541's lineage with Fielding and Doc 419's range to the
engagement-discipline tier.

## Index

| File | Induced property | Substrate tier |
|---|---|---|
| [capability-passing-supply-chain-impossibility.md](capability-passing-supply-chain-impossibility.md) | Architecturally-impossible supply chain attack | runtime/host-surface |
| [resolver-instance-directive-free-artifact.md](resolver-instance-directive-free-artifact.md) | Directive-free downstream artifact (stage-determinism) | cross-tier |
| [pin-art-resume-vector.md](pin-art-resume-vector.md) | Resumable substrate work; fresh-resolver-operational in one read | apparatus/locale |
| [apparatus-docs-tier-lean-cybernetic-loop.md](apparatus-docs-tier-lean-cybernetic-loop.md) | Lean main-context cybernetic loop; bounded keeper-supplement | apparatus/discipline |
| [source-identifier-as-substrate-coordinate.md](source-identifier-as-substrate-coordinate.md) | Substrate position recoverable from identifier name | source/encoding |
| [rule-11-pilot-mis-targeting-prevention.md](rule-11-pilot-mis-targeting-prevention.md) | Pilot mis-targeting prevention via 5-axis coverage | apparatus/spawn-discipline |
| [diff-prod-cross-runtime-engine-diff-oracle.md](diff-prod-cross-runtime-engine-diff-oracle.md) | Cross-runtime engine-diff oracle | apparatus/measurement |
| [pin-art-matrix-legible-decision-basis.md](pin-art-matrix-legible-decision-basis.md) | ~23k FAILs compress to ~246 coordinates; matrix is legible decision basis | apparatus/measurement |
| [multi-tier-cascade-pipeline-connection.md](multi-tier-cascade-pipeline-connection.md) | Pipeline connection across hot-path tiers (multi-tier reclaim) | runtime/jit |
| [agent-feedback-cross-resolver-legibility.md](agent-feedback-cross-resolver-legibility.md) | Cross-resolver review legibility through prior-frame metadata | apparatus/cross-resolver |

## Reading order

For first contact with the FCA-in-cruftless reading: start with
`capability-passing-supply-chain-impossibility.md` (the clearest single
instance, anchored at Doc 736 with the strongest published articulation),
then `resolver-instance-directive-free-artifact.md` (the cross-tier
recurrence), then any others in any order.

## Predecessor docs

The corpus's prior articulations of constraint accumulation:

- [Doc 419](../corpus-ref/419-progressive-code-on-demand-as-constraint-accumulation.md) — PRESTO progressive code-on-demand as Fielding accumulation from the Null style. The corpus's first explicit Fielding application.
- [Doc 463](../corpus-ref/463-constraint-thesis-as-lakatosian-programme.md) — constraint thesis as Lakatosian programme. The philosophical scaffolding.
- [Doc 541](../corpus-ref/541-systems-induced-property-emergence.md) — Systems-Induced Property Emergence (SIPE-T). The general operating-conditions layer.
- [Doc 728](../corpus-ref/728-tag-on-the-dag-sequential-index-collision-as-protocol-signal-that-the-substrate-has-become-the-coordinate-system.md) — Tag on the DAG. The naming convention this directory's docs cite.
- [`docs/engagement/prospective/cluster-coherence-multiplier-as-sipe-t-instance.md`](../engagement/prospective/cluster-coherence-multiplier-as-sipe-t-instance.md) — the prospective draft that established the engagement-tier FCA reading and triggered this survey.

The instances below are NOT new framework articulations. They are the
named application of the existing FCA pattern across cruftless's
architectural surface. Each tag on the DAG locates the constraint stack
in coordinate space; each induced property is the SIPE-T threshold the
composition crosses.
