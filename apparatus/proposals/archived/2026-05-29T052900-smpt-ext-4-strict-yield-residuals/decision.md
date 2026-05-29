---
proposal_slug: 2026-05-29T052900-smpt-ext-4-strict-yield-residuals
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T05:29:00Z
covers_commits:
  - ecc474f3273bc6197a5a07203809a3a44952a281
---

## Findings

Approved under Helmsman EPSUA parallel-R4 adjudication and same-turn imperative clarification.

The proposed commit closes the four named parser-owned `yield` residual rows without crossing into R3's non-strict PPAE for-head surface.

Substrate-tier verification:

1. Target exemplars: 4/4 PASS, all by expected SyntaxError.
2. Protective probes: strict non-generator `yield` still errors; sloppy `var yield` still runs; top-level for-of default using `yield` still runs.
3. Build: `cargo build --release --bin cruft -p cruftless` PASS.
4. Parser crate tests: blocked by unrelated `legacy_octal_rejected` lexer unit, recorded in the trajectory and proposal.

**APPROVED for push.**
