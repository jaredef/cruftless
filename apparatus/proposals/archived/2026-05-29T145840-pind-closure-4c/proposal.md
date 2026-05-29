---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - f9cbff7a36397053f171b3e4efde1b52bc18ff2f
target_branch: main
summary: PIND closure via Promise-local accessor-aware iterator acquisition
risk_class: substrate
gates_pre:
  test262_sample: PIND Rung 4b targeted matrix 39 PASS / 1 FAIL
  build: null
  per_locale:
    promise-iterator-not-callable-discipline: final accessor-aware @@iterator residual open
gates_post:
  test262_sample: PIND targeted matrix 40 PASS / 0 FAIL
  build: cargo build --release --bin cruft -p cruftless PASS
  per_locale:
    promise-iterator-not-callable-discipline: chapter closed at 40/40
---

## Substrate Moves

Commit `f9cbff7a36397053f171b3e4efde1b52bc18ff2f` lands PIND-EXT 4.

- **M** = Promise-local accessor-aware iterable acquisition.
- **T** = final `Promise.allSettled/iter-arg-is-poisoned.js` residual in the named PIND 40-row cluster.
- **I** = `Runtime::promise_collect_iterable` in `pilots/rusty-js-runtime/derived/src/interp.rs`; PIND seed status update; PIND trajectory chapter-close entry.
- **R** = Promise combinators invoke `@@iterator` accessor getters and route abrupts through the existing capability rejection wrapper; global `crate::intrinsics::collect_iterable` remains unchanged.

## Risk Assessment

The chosen path preserves narrow blast radius. A global `collect_iterable` lift would affect for-of, destructuring, spread, and intrinsic helper consumers; PIND only required Promise combinator acquisition semantics. The helper falls back to the existing `object_get(id, "@@iterator")` behavior when the PropertyKey-aware read misses, preserving existing string-keyed/data Symbol iterator behavior.

Measured post-change:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- Named 40-row PIND cluster: 40 PASS / 0 FAIL.
- Adjacent pass-smoke rows: 7/7 PASS across Promise.all/allSettled/race.

## Composes-With

Completes PIND after PIND-EXT 2 iterator rejection and PIND-EXT 3 C.resolve rejection. No residual carve-out remains for this chapter.
