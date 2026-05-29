---
proposal_slug: 2026-05-29T154900-gcs-ext-5-return-early-termination
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T15:49:00Z
covers_commits:
  - 9706f371c475b187163a713704b281e736f36122
---

## Findings

Approved under Helmsman GCS-EXT 5 same-turn directive for R1.

The commit resumes `Generator.prototype.return(value)` through the saved generator frame, stores a pending return value across finally cleanup yields, traces object-valued pending returns, and completes with the requested return value after cleanup.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 61 passed and 1 ignored.
3. Focused GCS tests: 8 passed.
4. Test262 return slice: 13 PASS / 10 FAIL from 23 tests, +9 PASS over the latest full-suite baseline for the same paths.

Remaining failures are within known deferred scope: descriptor metadata, nested catch/finally discrimination, and finally-return override completion records.

**APPROVED for push.**
