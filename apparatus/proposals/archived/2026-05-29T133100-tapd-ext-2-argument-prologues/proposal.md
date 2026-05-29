---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 31153d92a713d4a577ec3321c7c191aa0404b141
target_branch: main
summary: TAPD-EXT 2 TypedArray argument callability and integer-index prologues
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale:
    TAPD-phase2-candidate-cluster-after-EXT1: 46/268 PASS
gates_post:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale:
    TAPD-phase2-candidate-cluster: 71/268 PASS
    TAPD-adjacent-regression-sample: 50/50 PASS
    for-of-TypedArray-not-callable-spotcheck: 0/18 PASS
---

## Substrate moves

Commit `31153d92a713d4a577ec3321c7c191aa0404b141` lands TAPD-EXT 2.

- **M** = TypedArray prototype argument prologues in `intrinsics.rs::install_typed_array_stubs`.
- **T** = ECMA-262 TypedArray methods that accept callbacks must reject non-callable callbacks before iteration; index-taking methods must observe abrupt numeric coercion rather than silently defaulting non-number arguments.
- **I** = shared helper `typed_array_callable_arg` plus `typed_array_integer_index`, routed through callback methods (`forEach`, `findIndex`, `every`, `some`, `map`, `filter`, `reduce`, `reduceRight`, `findLast`, `findLastIndex`, and comparator checks for `sort`/`toSorted`) and index methods (`subarray`, `set`, `fill`, `slice`, `indexOf`, `includes`, `copyWithin`).
- **R** = TAPD Rung 2 only. Detached/resizable buffers, species/subclass allocation, constructor/static methods, and full `lastIndexOf` fromIndex-detach semantics remain separate rungs.

## Risk assessment (helmsman self-evaluation)

Primary regression risk was numeric argument coercion surfacing detached-buffer behavior prematurely. A trial implementation of `lastIndexOf` fromIndex coercion regressed `lastIndexOf/detached-buffer-during-fromIndex-returns-minus-one-for-undefined.js`; this commit intentionally leaves `lastIndexOf` on its previous path pending the detached/out-of-bounds TAPD rung.

Verification:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime`: BLOCKED by pre-existing stale integration tests referencing removed `Runtime::globals`.
- TAPD candidate cluster: 71 PASS / 197 FAIL / 0 SKIP from the 268 Phase 2 baseline-failing rows, +25 PASS over EXT 1.
- Adjacent regression sample: 50 PASS / 0 FAIL.
- Post-EPSUA `language.statements.for-of` / `feat:TypedArray;not-callable` spot-check stayed 18 FAIL / 0 PASS, so that cell is not closed by prototype-method argument prologues alone.

Artifacts: `/home/jaredef/Developer/cruftless-sidecar/results/tapd-ext2-verify-20260529T132809Z/` and `/home/jaredef/Developer/cruftless-sidecar/results/tapd-ext2-forof-typedarray-not-callable-20260529T132827Z/`.

## Composes-with

- TAPD-EXT 1 receiver helper commit `6e3102053a3813a67eaaa3c7738622fe69a8981f`
- Helmsman request CAACP message `9b254a74-2d3b-4b97-b8f5-6c164c4425f6`
- `pilots/typed-array-prologue-discipline/derived/`
