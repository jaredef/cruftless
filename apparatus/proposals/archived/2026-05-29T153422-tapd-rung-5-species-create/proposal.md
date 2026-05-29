---
helmsman_session: helmsman
proposed_commits:
  - b39d47df6d983b3f4f73bccb7c0981f02510cd47
target_branch: main
summary: TAPD Rung 5 TypedArraySpeciesCreate argument-list closure
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale: { typed-array-prologue-discipline: "77 PASS / 13 FAIL on 90-row target after Rung 4" }
gates_post:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale: { typed-array-prologue-discipline: "90 PASS / 0 FAIL on 90-row target; adjacent 50/0" }
---

## Substrate Moves

`b39d47df6d983b3f4f73bccb7c0981f02510cd47` implements the TypedArraySpeciesCreate argument-list closure for the TAPD Rung 5 residuals.

- M: species-created TypedArray result construction for `slice` and `subarray`.
- T: pass `[length]` for length-created results and `[buffer, byteOffset, length]` for subarray, then validate returned values are non-detached TypedArrays.
- I: accessor-aware `SpeciesConstructor` lookup, `make_typed_array_like_args`, TypedArray constructor detached-buffer guard, and TAPD trajectory update.
- R: closes the 13 residual rows left after TAPD-EXT 4, enabled by H262S-EXT 2.

## Risk Assessment

The main risk is disturbing prior same-kind object shape. The implementation keeps the existing length-only helper for map/filter/slice callers and adds an explicit argument-list path for subarray. It also stops overriding custom species return prototypes, which is required for other-target constructor identity.

Verification was scoped to the target and adjacent TAPD sample:

- 13-row residual cluster: 13 PASS / 0 FAIL.
- 90-row detached/resizable target set: 90 PASS / 0 FAIL, +13 PASS / 0 regressions over TAPD-EXT 4.
- Adjacent TAPD sample: 50 PASS / 0 FAIL.
- Build: `cargo build --release --bin cruft -p cruftless` PASS.

## Composes-With

- TAPD-EXT 4 detached receiver and mid-coercion residuals.
- H262S-EXT 2 `$262.detachArrayBuffer` TypedArray view bridge.
- No new deferrals. The H262S-enabled TAPD detached/resizable target set is closed.
