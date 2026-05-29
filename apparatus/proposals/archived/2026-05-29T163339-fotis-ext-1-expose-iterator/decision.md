---
proposal_slug: 2026-05-29T163339-fotis-ext-1-expose-iterator
decision: APPROVED
arbiter_session: keeper-substituted
decided_at: 2026-05-29T16:33:39Z
covers_commits:
  - 965c3008acbf416ef2df58eca0111c4f8bc61247
---

## Findings

Approved per Helmsman directive `ff0eff29-b369-42c9-b0d7-4954e4a223f8` for FOTIS-EXT 1.

The commit closes the 18-row for-of TypedArray iterator-shape cluster by exposing `%TypedArray%.prototype[@@iterator]` at the reached prototype level and preserving `values === @@iterator`. Verification cited in the proposal is sufficient for this single rung.

**APPROVED for push.** Archive after push lands.
