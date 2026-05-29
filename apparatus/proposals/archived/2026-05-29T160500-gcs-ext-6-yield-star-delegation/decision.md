---
proposal_slug: 2026-05-29T160500-gcs-ext-6-yield-star-delegation
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T16:05:00Z
covers_commits:
  - 4ba9d47c28f5b066266aabcb94e75eca57f86a2e
---

## Findings

Approved under Helmsman GCS-EXT 6 same-turn directive for R1.

The commit introduces `Op::YieldDelegate`, lowers `yield*` to that opcode, lets generator functions containing `yield*` use the suspended lifecycle, and stores active delegate iterator state on the generator object.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 63 passed and 1 ignored.
3. Focused GCS tests: 10 passed.
4. For-of/generator slice: 50 PASS / 19 FAIL from 69 paths, +4 PASS over the post-EXT 4 baseline.

Remaining failures are deferred residuals: delegate abrupt forwarding, IteratorClose, destructuring iterator-close, and prior return-slice completion-kind metadata.

**APPROVED for push.**
