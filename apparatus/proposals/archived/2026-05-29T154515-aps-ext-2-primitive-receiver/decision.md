---
proposal_slug: 2026-05-29T154515-aps-ext-2-primitive-receiver
decision: APPROVED
arbiter_session: keeper-substituted
decided_at: 2026-05-29T15:45:15Z
covers_commits:
  - f1b83a9990030cc5b542674ac02e0eeff4a13214
---

## Findings

Approved per Helmsman directive `80319577-580a-4d1a-8236-beb63efba392` for APS-EXT 2.

The commit closes the `call-with-primitive.js` residual while staying scoped to Array.prototype.sort. It also applies the minimal shared Symbol `ToObject` wrapper correction needed by the test. Verification cited in the proposal is sufficient: release build PASS, APS target 26/26 PASS, and full sort-directory mirror 50/54 PASS.

**APPROVED for push.** Archive after push lands.
