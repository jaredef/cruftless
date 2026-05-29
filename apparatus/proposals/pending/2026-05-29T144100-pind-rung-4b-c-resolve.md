---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - cddf1fb328daa9e09f917ec1e685cc3ce00a0d64
target_branch: main
summary: PIND Rung 4b C.resolve rejection closure
risk_class: mixed
gates_pre:
  test262_sample: PIND Rung 4a targeted matrix 33 PASS / 7 FAIL
  build: null
  per_locale:
    promise-iterator-not-callable-discipline: C.resolve residual bucket open
gates_post:
  test262_sample: PIND targeted matrix 39 PASS / 1 FAIL
  build: cargo build --release --bin cruft -p cruftless PASS
  per_locale:
    promise-iterator-not-callable-discipline: C.resolve residual 6/6 PASS
---

## Substrate Moves

Commit `cddf1fb328daa9e09f917ec1e685cc3ce00a0d64` lands PIND-EXT 3.

- **M** = Promise combinator `C.resolve` abrupt-completion routing through capability rejection.
- **T** = `Promise.all`, `Promise.allSettled`, and `Promise.race` C.resolve getter-abrupt and non-callable residual rows.
- **I** = combinator-local `spec_get(&ctor, "resolve")` plus non-callable TypeError rejection in `pilots/rusty-js-runtime/derived/src/interp.rs`; test262 runner async-drain capture in `legacy/host-rquickjs/tests/test262/runner.mjs`; PIND trajectory update.
- **R** = returned capability promise rejects instead of throwing synchronously; Rung 4a helper and global `collect_iterable` remain unchanged.

## Risk Assessment

Runtime risk is confined to three Promise combinators. `Promise.any` was explicitly left unchanged. The runner change is apparatus-only: capture `Promise.resolve.bind(Promise)` before the evaluated test body so tests that intentionally poison global `Promise.resolve` do not poison the runner's post-test async drain.

Measured post-change:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- Named 40-row PIND cluster: 39 PASS / 1 FAIL.
- C.resolve residual rows: 6/6 PASS, +6 against the Rung 4a targeted result.
- Adjacent pass-smoke rows: 7/7 PASS across Promise.all/allSettled/race.

## Composes-With

Composes with PIND-EXT 2's Promise-local iterable rejection. The remaining single PIND residual is `Promise.allSettled/iter-arg-is-poisoned.js`, an `@@iterator` accessor-getter abrupt path outside this rung's C.resolve scope.
