---
proposal_slug: 2026-05-29T134538-tapd-ext-3-access-species.md
decision: APPROVED
arbiter_session: keeper-substituted-helmsman-approval
decided_at: 2026-05-29T13:45:38Z
covers_commits:
  - 6a8c751f6435168110b05c85f93207f973e0f7e9
  - c5ee456d01b1f7bba69cbbcaee21edaf649f6902
---

## Findings

Approved by helmsman response `tapd-ext-3-approval-for-r2`, related to R2 candidate `bb68bc81-4a1e-4969-b4f7-e104575dfbf2`.

The move stays within the original TAPD-EXT 3 directive: access validation plus species construction. The scoped `subarray` boundary is accepted because the probe found a real resizable-buffer ordering regression if `subarray` is forced through immediate out-of-bounds validation.

Verified gates reported by the resolver:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- Touched-method sweep: 574 PASS / 432 FAIL baseline to 660 PASS / 346 FAIL candidate, +86 PASS / 0 regressions across 1,006 rows.

Residual Rung 4 work: `subarray` resizable-buffer ordering and direct detached-buffer rows after the host `$262.detachArrayBuffer` shim is extended.
