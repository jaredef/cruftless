---
proposal_slug: 2026-05-29T134000-pind-rung-4a-promise-iterable-rejection
decision: APPROVED
arbiter_session: keeper-substituted
decided_at: 2026-05-29T13:40:26Z
covers_commits:
  - 0beb1c9aa9e4bfd5e437e5f237de74a8abf4b944
---

## Findings

Approved per Helmsman directive `ac77efff-e043-47ae-a4fd-e5467d7156b5` for PIND Rung 4a.

The substrate commit matches the approved Phase 3 design: the runtime adds a Promise-local wrapper around `collect_iterable`, wires only `Promise.all`, `Promise.allSettled`, and `Promise.race`, and leaves the global iterable helper unchanged.

Verification cited in the proposal is sufficient for this rung: release build PASS, Bucket B 18/18 PASS, named PIND cluster 33/40 PASS after the rung, and adjacent Promise smoke rows PASS. The seven remaining named failures are the expected `C.resolve`/poisoned residuals for a later rung.

**APPROVED for push.** Archive after push lands.
