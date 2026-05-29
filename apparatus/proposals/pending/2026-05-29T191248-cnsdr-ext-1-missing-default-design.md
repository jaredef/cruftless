---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 40503fc68bc7f86c1c36e54658a548107b38d126
target_branch: main
summary: CNSDR-EXT 1 missing-default/null-namespace design probe
risk_class: apparatus
gates_pre:
  parity_cluster: CNSDR-EXT 0 inline 56-row shape-diff-no-error cluster
  build: null
  per_locale:
    cjs-ns-shape-diff-residual: missing-default family identified, not discriminated
gates_post:
  parity_cluster: no runtime change; 20 missing-default rows split into 4 zero-key CJS synthesis candidates plus 16 null namespace/load candidates
  build: null
  per_locale:
    cjs-ns-shape-diff-residual: Phase 4 move shapes recorded in design.md and trajectory
---

## Substrate Moves

Commit `40503fc68bc7f86c1c36e54658a548107b38d126` lands CNSDR-EXT 1.

- **M** = design-only discrimination of the CNSDR missing-default/null-namespace residual.
- **T** = 20 packages whose Bun namespace has `default` and whose cruftless namespace lacks it.
- **I** = `pilots/cjs-ns-shape-diff-residual/design.md` and CNSDR trajectory.
- **R** = no runtime substrate edit; Phase 4 plan split into a four-row CJS empty-exports default policy probe and a 16-row null namespace load-completion probe.

## Verification

No build gate was run because this is documentation/design only and touches no Rust or generated code. The design cross-references the current CJS namespace implementation in `pilots/rusty-js-runtime/derived/src/module.rs` and the ESM finalization hook in `cruftless/src/module_ns.rs`.

## Risk Assessment

The main risk is overclaiming the missing-default family. The design explicitly rejects a single 20-row default patch. The immediate PASS prediction is only the four zero-key rows (`reflect-metadata`, `joi-extract-type`, `nx`, `express-async-errors`) after direct positive/negative package probes are available.

## Composes-With

- CNSDR-EXT 0 shape-diff segmentation.
- Prior CJS namespace populator work around `caller`/`arguments` filtering.
- Follow-up null namespace load-completion instrumentation.
