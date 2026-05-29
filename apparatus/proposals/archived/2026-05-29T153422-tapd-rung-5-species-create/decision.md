---
proposal_slug: 2026-05-29T153422-tapd-rung-5-species-create.md
decision: APPROVED
arbiter_session: keeper-substituted-helmsman-same-turn
decided_at: 2026-05-29T15:34:22Z
covers_commits:
  - b39d47df6d983b3f4f73bccb7c0981f02510cd47
---

## Findings

Approved under helmsman directive `tapd-rung-5-species-create-r2`.

The commit matches the directed scope: real TypedArraySpeciesCreate argument-list support for the 13 residual cells. It uses accessor-aware constructor/species lookup, passes `[length]` or `[buffer, byteOffset, length]` as required, validates returned typed arrays, and preserves custom species result identity.

Verified gates:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- 13-row Rung 5 residual cluster: 13 PASS / 0 FAIL.
- 90-row TAPD detached/resizable target set: 90 PASS / 0 FAIL.
- Adjacent TAPD regression sample: 50 PASS / 0 FAIL.

APPROVED for push.
