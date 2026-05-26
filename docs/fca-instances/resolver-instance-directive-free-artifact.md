# Resolver-Instance Pattern → Directive-Free Downstream Artifact (Stage-Determinism)

## Induced property

Each tier of cruftless's substrate stack emits an artifact that **carries no residue** from upstream tiers' directives. A downstream resolver consumes the prior tier's output as data; it does not need to interpret upstream directives. The induced property is **stage-deterministic emission**: given a tier-N artifact, the tier-N+1 output is determined by the tier-N+1 resolver alone, with no cross-tier directive leak.

Anchor: [Doc 729](../corpus-ref/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) §IV. Recurrent across tiers per [Doc 730](../corpus-ref/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md).

## The accumulation

The pattern stacks per-tier:

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) ad-hoc tier coupling (each layer can re-interpret upstream directives) | — | nothing; cross-tier coupling is unbounded |
| 1 | **Per-tier directive consumption** — each tier consumes its own directives and emits an artifact representing the result | tier boundary semantics | property: "a tier's artifact is the contract" |
| 2 | **No residue carry** — emitted artifacts contain no upstream directives or upstream metadata that downstream tiers can interpret | downstream isolation | property: "downstream tiers cannot route around the prior tier's resolution" |
| 3 | **Stage-deterministic emission** — given a fixed input, a tier produces a single deterministic artifact (no nondeterminism across runs) | reproducibility | property: "the pipeline is replayable byte-for-byte" |
| 4 | **Resolver-instance interface as authority boundary** — each tier exposes its API surface as the only path for upstream input; bypass paths are removed | API uniformity | property: "tier substitution is mechanical (swap one resolver-instance for another implementing the same interface)" |
| 5 | **Vertical recurrence** — the pattern recurs at every tier from spec-text through ECMAScript IR through bytecode through interpreter through JIT through machine code | scale invariance | property: "the pipeline is a fractal of resolver-instances; a substrate move at any tier reads against the same composition rules" |

The named composition is the **resolver-instance pattern**. The induced property is directive-free downstream artifacts at every tier, with the consequence that the pipeline is byte-replayable, tier-substitutable, and bypass-resistant.

Removing constraint 1 (skip per-tier consumption) means upstream directives leak through; downstream tiers must implement upstream resolution.
Removing constraint 2 (allow residue carry) means downstream tiers can second-guess upstream resolution; the pipeline becomes coupled and unreplayable.
Removing constraint 3 (allow nondeterminism) breaks replay and breaks the diff-prod oracle.
Removing constraint 4 (allow bypass paths) means tier substitution becomes non-mechanical.
Removing constraint 5 (apply at only one tier) means the pipeline's scale-invariance is lost; each tier has to re-derive the discipline.

## Tag on the DAG

The resolver-instance pattern is a **cross-tier coordinate** — it does not project onto a single test262 cluster, but rather constrains the SHAPE of every per-tier coordinate in the matrix. The tag is:

```
* / *-tier-resolver-instance ::
  Ek/per-tier-output ::
  cut/artifact-emission ::
  property/directive-free-artifact
```

Where `*-tier-resolver-instance` ranges over `source-to-ast/parser`, `ast-to-bytecode/language-lowering`, `runtime/spec-builtins`, `runtime/buffer-typed-array`, etc. The pattern's correctness is observable in the test262 matrix as the **absence** of `uncategorized/projection` failures at tier boundaries: when a failure tag is `uncategorized` at a tier-boundary surface, it indicates a residue leak (the artifact carried upstream directives the categorizer couldn't classify).

The full-suite Pin-Art matrix shows 659 `ast-to-bytecode/language-lowering :: uncategorized/projection :: failure/other` failures (rank #6). These are partly residue leaks per Doc 729 §IV — the AST-to-bytecode lowering carries cross-tier metadata the categorizer can't read. The apparatus-hardening rung at `ast-bytecode-uncategorized-projection/` (per the heuristics §IX gap) is the empirical follow-on for tightening this coordinate.

## Composes-with

- Doc 729 — primary articulation.
- Doc 730 — vertical recurrence (the scale-invariance constraint).
- Doc 742 — resolver-instance pattern at full strength (the engagement-tier consolidation).
- [`docs/fca-instances/capability-passing-supply-chain-impossibility.md`](capability-passing-supply-chain-impossibility.md) — capability-passing is the resolver-instance pattern at the authority tier.
- [`docs/fca-instances/multi-tier-cascade-pipeline-connection.md`](multi-tier-cascade-pipeline-connection.md) — cascade-revival is the temporal dual (closure propagates downstream via the directive-free interface).

## Falsification

A cross-tier failure trace that requires reading upstream-tier directives to diagnose falsifies constraint 2 (no residue carry) for that surface. The trace becomes a substrate-tier bug to close. Per the engagement record, several have been surfaced and closed (TROI's ESM-cycle issue diagnosed as runtime-substrate concern but fixed at the resolver tier per Finding TROI.2 — a residue-carry violation that was repaired by tightening the tier boundary).
