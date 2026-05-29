---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 0beb1c9aa9e4bfd5e437e5f237de74a8abf4b944
target_branch: main
summary: PIND Rung 4a Promise-local iterable abrupt rejection
risk_class: substrate
gates_pre:
  test262_sample: post-EPSUA PIND matrix 40 named FAIL rows inspected
  build: null
  per_locale:
    promise-iterator-not-callable-discipline: Phase 3 design predicted +18 Bucket B
gates_post:
  test262_sample: named PIND 40-row targeted measurement 33 PASS / 7 FAIL
  build: cargo build --release --bin cruft -p cruftless PASS
  per_locale:
    promise-iterator-not-callable-discipline: Bucket B 18/18 PASS; symbol residual 3/3 PASS
---

## Substrate Moves

Commit `0beb1c9aa9e4bfd5e437e5f237de74a8abf4b944` lands PIND-EXT 2.

- **M** = Promise-local iterable acquisition abrupt-completion wrapper.
- **T** = `Promise.all`, `Promise.allSettled`, and `Promise.race` not-callable `@@iterator` rejection rows.
- **I** = `Runtime::promise_collect_iterable_or_reject` plus three combinator call-site rewires in `pilots/rusty-js-runtime/derived/src/interp.rs`; trajectory update in `pilots/promise-iterator-not-callable-discipline/trajectory.md`.
- **R** = capability promise is rejected and returned for iterable acquisition abrupts; global `crate::intrinsics::collect_iterable` remains unchanged.

## Risk Assessment

Risk is constrained to three Promise static combinators. Non-Promise consumers keep the existing synchronous `collect_iterable` behavior. `Promise.race` now validates `C.resolve` before collection for symmetry with `Promise.all` and `Promise.allSettled`; the remaining `C.resolve is not callable` failures prove Rung 4b still has a separate surface.

Measured post-change:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- Bucket B assigned non-symbol rows: 18/18 PASS.
- Named 40-row PIND cluster: 33 PASS / 7 FAIL.
- Adjacent Promise smoke: seven selected all/allSettled/race pass rows remained PASS.

## Composes-With

Composes with `pilots/promise-iterator-not-callable-discipline/design.md` and the iterator-protocol parent arc. The widened yield suggests the planned symbol cleanup rung is likely unnecessary; the next rung should focus on the `C.resolve`/constructor resolve abrupt-completion bucket.
