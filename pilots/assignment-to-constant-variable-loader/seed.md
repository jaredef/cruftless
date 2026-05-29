---
name: assignment-to-constant-variable-loader
description: Phase-2 probe for top500 dynamic-import failures reporting "Assignment to constant variable" from the module loader.
type: project
---

# assignment-to-constant-variable-loader — Seed

## Substrate-pilot — ATC loader probe

Spawned per Helmsman CAACP directive `atc-loader-phase-0-phase-2-r1`
(`b0467a28-ea48-4bf0-b91a-8fd721542f90`). This is a Phase 0 + Phase 2
locale only; no runtime, parser, compiler, or loader substrate edit is
authorized in the founding round.

## Telos

Classify the 2026-05-29 top500 dynamic-import cluster where cruft reports
`TypeError: Assignment to constant variable '<name>'` from the module-loader
path. The phase-2 question is whether the cluster is:

- user package code that really assigns to a `const` binding;
- cruft compiler/lowering installing `const` for a binding that should be
  mutable;
- destructuring or initializer-order re-assignment;
- Babel/TypeScript/CommonJS wrapper output that cruft parses or lowers wrong;
- heterogeneous across those mechanisms.

## Apparatus

- Source cluster requested by directive:
  `/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json`
  filtered for `Assignment to constant` in `rb.message`.
- Package source inspection from the package installation/sandbox that produced
  the refined sweep.
- Cruft source cross-reference:
  - `pilots/rusty-js-parser/derived/src/parser.rs`
  - `pilots/rusty-js-bytecode/derived/src/compiler.rs`
  - `pilots/rusty-js-runtime/derived/src/module.rs`
  - `pilots/rusty-js-runtime/derived/src/interp.rs`

## Methodology

1. Extract the full package list from the refined sweep JSON.
2. Sample at least eight of the 26 packages, including the named examples:
   `mathjs`, `abortcontroller-polyfill`, `mobx`, `mobx-state-tree`, and
   `postcss-selector-parser`.
3. For each sampled package, identify the exact source-code line that triggers
   the thrown assignment-to-constant error.
4. Segment by mechanism, then run C4 reason-coherence:
   one mechanism dominates if it explains at least 40% of the inspected sample
   and the examples share the same operational shape.
5. Propose Phase 3 as single-rung if C4 passes, or multi-rung if the sample is
   heterogeneous.

## Carve-outs

This locale does not authorize substrate edits in Phase 0 or Phase 2. If the
refined sweep artifact or its package sandbox is unavailable, the correct
output is a blocked trajectory entry naming the missing artifact, not a
guessed segmentation.

## Resume protocol

Read this seed, then the latest entry in `trajectory.md`. If the refined sweep
artifact becomes available, resume at the Phase 2 extraction step and append
the package-line segmentation before proposing Phase 3.
