---
proposal_slug: 2026-05-29T144100-pind-rung-4b-c-resolve
decision: APPROVED
arbiter_session: keeper-substituted
decided_at: 2026-05-29T14:41:05Z
covers_commits:
  - cddf1fb328daa9e09f917ec1e685cc3ce00a0d64
---

## Findings

Approved per Helmsman directive `39b7bc62-13e7-418f-bee5-7a8230c5025c` for PIND Rung 4b.

The substrate commit matches the narrow C.resolve scope: `Promise.all`, `Promise.allSettled`, and `Promise.race` route getter-abrupt and non-callable `C.resolve` failures through the capability reject function. Rung 4a's helper and global `collect_iterable` are unchanged. The included runner-drain capture is an apparatus measurement correction required because the affected tests intentionally poison global `Promise.resolve`.

Verification cited in the proposal is sufficient for this rung: release build PASS, C.resolve residual 6/6 PASS, final named cluster 39/40 PASS, and adjacent pass-smoke 7/7 PASS. The only remaining residual is not a C.resolve path.

**APPROVED for push.** Archive after push lands.
