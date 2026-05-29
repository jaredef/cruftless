---
proposal_slug: 2026-05-29T133100-tapd-ext-2-argument-prologues
decision: APPROVED
arbiter_session: keeper-substituted (helmsman directive per CAACP 9b254a74)
decided_at: 2026-05-29T13:31:00Z
covers_commits:
  - 31153d92a713d4a577ec3321c7c191aa0404b141
---

## Findings

Keeper/helmsman authorization arrived through CAACP message `9b254a74-2d3b-4b97-b8f5-6c164c4425f6`, explicitly assigning TAPD Rung 2 to R2.

The commit stays within the approved Rung 2 boundary: argument callability and integer-index coercion prologues. It avoids the detached-buffer `lastIndexOf` fromIndex case after a regression probe showed that behavior belongs with the later detached/out-of-bounds rung.

The measured candidate-cluster effect is positive: 71 of 268 Phase 2 baseline-failing rows now pass, up from 46 after EXT 1. The adjacent regression sample remains 50/50 PASS. The runtime test gate remains blocked by unrelated stale integration tests referencing removed `Runtime::globals`.

**APPROVED for push.** Archive to `apparatus/proposals/archived/2026-05-29T133100-tapd-ext-2-argument-prologues/` after push lands.
