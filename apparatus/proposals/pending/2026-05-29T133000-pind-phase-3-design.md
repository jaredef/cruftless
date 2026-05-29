---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - a80f3fd2e72c3c86e869207ce750f0fa2a0b6c7f
target_branch: main
summary: PIND Phase 3 Promise combinator factoring design
risk_class: design
gates_pre:
  test262_sample: post-EPSUA sample matrix inspected
  runtime_edits: none
gates_post:
  test262_sample: not rerun; design-only commit
  runtime_edits: none
---

## Design Move

Commit `a80f3fd2e72c3c86e869207ce750f0fa2a0b6c7f` lands PIND-EXT 1, a design-only rung for Promise combinator not-callable failures.

- **M** = Promise combinator iterable-acquisition factoring design.
- **T** = 40-row post-EPSUA sample cluster across `Promise.all`, `Promise.allSettled`, and `Promise.race`.
- **I** = `pilots/promise-iterator-not-callable-discipline/design.md` and trajectory update.
- **R** = no runtime substrate; next rung should close the `@@iterator` method-not-callable rejection path first.

## Risk Assessment

No source code changed. The only risk is design misclassification; the trajectory records that the cluster has two adjacent >40% buckets and therefore should not be treated as a one-line IsCallable fix.
