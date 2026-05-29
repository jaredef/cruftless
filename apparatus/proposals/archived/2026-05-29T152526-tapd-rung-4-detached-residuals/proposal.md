---
helmsman_session: helmsman
proposed_commits:
  - ffa70b8ae07d34d79ea512697772ec3dfb38edb6
target_branch: main
summary: TAPD Rung 4 detached receiver and mid-coercion residual closure
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale: { typed-array-prologue-discipline: "62 PASS / 28 FAIL on 90-row target baseline" }
gates_post:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale: { typed-array-prologue-discipline: "77 PASS / 13 FAIL on 90-row target; +15 PASS / 0 regressions; adjacent 50/0" }
---

## Substrate Moves

`ffa70b8ae07d34d79ea512697772ec3dfb38edb6` closes the TAPD Rung 4 detached receiver and detached-mid-coercion subset.

- M: TypedArray prototype methods observing detached backing buffers after H262S-EXT 2 made `$262.detachArrayBuffer` exercisable on TypedArray views.
- T: direct detached receivers throw where the method requires TypeError, while mid-coercion detachment follows each method's snapshot behavior.
- I: `Runtime::typed_array_view_detached`, targeted method routing in `intrinsics.rs`, and TAPD trajectory update.
- R: follows TAPD-EXT 3 access validation and H262S-EXT 2 host-shim bridge; leaves TypedArraySpeciesCreate argument-list ordering for a later rung.

## Risk Assessment

The main risk is conflating detached buffers with resizable-buffer out-of-bounds views. The change introduces a detached-only predicate and keeps `subarray` off broad out-of-bounds validation. That preserves `subarray/coerced-begin-end-grow.js` while letting already-detached `subarray` receivers throw after observable begin/end coercions.

The measurement claims are narrow: the 89 direct detached-buffer TypedArray prototype rows plus `subarray/coerced-begin-end-grow.js` improved from 62 PASS / 28 FAIL to 77 PASS / 13 FAIL, with 0 regressions. The adjacent 50-row TAPD sample stayed 50 PASS / 0 FAIL.

## Composes-With

- Locale: `pilots/typed-array-prologue-discipline/derived/`.
- Prior rungs: TAPD-EXT 1 receiver prologue, TAPD-EXT 2 argument prologues, TAPD-EXT 3 access validation and species bridge.
- Upstream enabling rung: H262S-EXT 2 `$262.detachArrayBuffer` TypedArray view bridge.
- Residual Rung 5: 12 `slice` species/custom-constructor detached-ordering rows plus `subarray/byteoffset-with-detached-buffer.js`, requiring real TypedArraySpeciesCreate argument-list support.
