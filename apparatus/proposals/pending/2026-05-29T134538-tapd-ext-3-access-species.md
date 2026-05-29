---
helmsman_session: helmsman
proposed_commits:
  - 6a8c751f6435168110b05c85f93207f973e0f7e9
  - c5ee456d01b1f7bba69cbbcaee21edaf649f6902
target_branch: main
summary: TAPD-EXT 3 access-validation plus TypedArray species-create bridge
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale: { typed-array-prologue-discipline: "574 PASS / 432 FAIL touched-method baseline" }
gates_post:
  test262_full: null
  test262_sample: null
  diff_prod: null
  per_locale: { typed-array-prologue-discipline: "660 PASS / 346 FAIL touched-method candidate; +86 PASS / 0 regressions" }
---

## Substrate Moves

`6a8c751f6435168110b05c85f93207f973e0f7e9` preserves the TAPD-EXT 2 trajectory boundary clarification after the R2 branch rebase. It carries no substrate behavior change.

`c5ee456d01b1f7bba69cbbcaee21edaf649f6902` adds the TAPD-EXT 3 substrate move:

- M: duplicated TypedArray prototype method access prologues and species-created result allocation.
- T: access-validating methods reject detached/out-of-bounds views at receiver-access entry, while map/filter/slice/subarray route result allocation through TypedArray species construction.
- I: `validate_typed_array_access` layers `typed_array_view_out_of_bounds` on `validate_typed_array_this`; `make_typed_array_like` now returns `Result<ObjectRef, RuntimeError>` and consults `species_constructor`.
- R: composes after TAPD-EXT 1 receiver validation and TAPD-EXT 2 callback/index prologues; defers resizable-buffer ordering and host `$262.detachArrayBuffer` shim rows.

## Risk Assessment

The principal risk was over-applying detached/out-of-bounds TypeError behavior to methods whose argument coercions must observe resizable-buffer ordering. Probe results showed that `subarray` regressed when routed through immediate access validation, so this proposal keeps `subarray` receiver-only while still using species allocation for its result.

Direct detached-buffer rows remain partially blocked by the host `$262.detachArrayBuffer` shim shape, so this is not claimed as full detached-buffer closure. The landed empirical claim is the touched-method sweep: 1,006 rows, baseline 574 PASS / 432 FAIL, candidate 660 PASS / 346 FAIL, +86 PASS and 0 regressions.

## Composes-With

- Locale: `pilots/typed-array-prologue-discipline/derived/`.
- Prior rungs: TAPD-EXT 1 receiver prologue helper; TAPD-EXT 2 argument callability and integer-index prologues.
- Deferred follow-up: Rung 4 should pair `subarray` resizable-buffer ordering with the host `$262.detachArrayBuffer` shim extension before attempting direct detached-buffer rows.
