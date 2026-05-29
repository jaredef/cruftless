---
proposal_slug: 2026-05-29T152526-tapd-rung-4-detached-residuals.md
decision: APPROVED
arbiter_session: keeper-substituted-helmsman-same-turn
decided_at: 2026-05-29T15:25:26Z
covers_commits:
  - ffa70b8ae07d34d79ea512697772ec3dfb38edb6
---

## Findings

Approved under helmsman directive `tapd-rung-4-detached-residuals-r2`, which allowed scope-down if the 28 residuals split into distinct substrate moves.

The proposed commit lands the coherent detached receiver and detached-mid-coercion subset:

- direct detached receiver TypeError routing for `find`, `lastIndexOf`, `sort`, and `subarray`;
- detached-mid-`fromIndex` behavior for `includes`, `indexOf`, and `lastIndexOf`;
- detached-mid-separator behavior for `join`;
- detached-only `subarray` guard preserving resizable out-of-bounds ordering.

Verified gates:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- 90-row target sweep: 62 PASS / 28 FAIL baseline to 77 PASS / 13 FAIL candidate, +15 PASS / 0 regressions.
- Adjacent TAPD regression sample: 50 PASS / 0 FAIL.

Residual Rung 5 is explicitly named: `slice` species/custom-constructor detached ordering plus `subarray/byteoffset-with-detached-buffer.js`, requiring TypedArraySpeciesCreate argument-list support.
