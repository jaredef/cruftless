---
helmsman_session: helmsman-2026-05-28-principal
proposed_commits:
  - 6e3102053a3813a67eaaa3c7738622fe69a8981f
target_branch: main
summary: TAPD-EXT 1 typed-array receiver prologue helper
risk_class: substrate
gates_pre:
  test262_full: 67.6% runnable pass rate (baseline full-suite p2)
  test262_sample: null
  diff_prod: null
  per_locale:
    TAPD-phase2-candidate-cluster: 0/268 PASS baseline-failing rows
gates_post:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale:
    TAPD-phase2-candidate-cluster: 46/268 PASS baseline-failing rows
    TAPD-adjacent-regression-sample: 50/50 PASS
---

## Substrate moves

Commit `6e3102053a3813a67eaaa3c7738622fe69a8981f` lands TAPD-EXT 1.

- **M** = runtime/buffer-typed-array duplicated receiver prologues in `intrinsics.rs::install_typed_array_stubs`.
- **T** = ECMA-262 TypedArray prototype methods reject non-object and non-TypedArray receivers with `TypeError` before continuing method body execution.
- **I** = shared `validate_typed_array_this(rt, method_name)` helper plus symmetric routing for the approved 22 method closures: `set`, `fill`, `slice`, `values`, `keys`, `entries`, `@@iterator`, `reverse`, `indexOf`, `forEach`, `findIndex`, `every`, `some`, `join`, `map`, `filter`, `reduce`, `reduceRight`, `toString`, `at`, `lastIndexOf`, and `findLast`.
- **R** = TAPD Rung 1 only. Argument/callability order, detached/resizable buffers, constructor/static methods, species allocation, and unimplemented neighbor methods remain deferred to later TAPD rungs.

## Risk assessment (helmsman self-evaluation)

Primary risk is over-tightening the receiver test against a backing-view table that current construction paths do not consistently populate. The helper therefore uses the existing `__ta_kind` sentinel as the local ValidateTypedArray approximation and defers full internal-slot/backing-view discipline to the detached/resizable rung.

Verification:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime`: BLOCKED before test execution by existing integration-test compile errors against removed `Runtime::globals` fields.
- TAPD candidate cluster: 46 PASS / 222 FAIL / 0 SKIP from the 268 Phase 2 baseline-failing rows.
- Adjacent regression sample: 50 PASS / 0 FAIL from 430 baseline PASS rows across adjacent TypedArray prototype method directories.

Artifacts: `/home/jaredef/Developer/cruftless-sidecar/results/tapd-ext1-verify-20260529T061610Z/`.

## Composes-with

- `pilots/typed-array-prologue-discipline/derived/`
- Helmsman approval CAACP message `1a249136-01a3-484f-bee2-cc6985ba01dd`
- Phase 2 TAPD probe response `5e26dab8-a172-43dd-9f0c-a548d8080251`
