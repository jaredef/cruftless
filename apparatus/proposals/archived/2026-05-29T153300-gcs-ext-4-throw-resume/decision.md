---
proposal_slug: 2026-05-29T153300-gcs-ext-4-throw-resume
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T15:33:00Z
covers_commits:
  - 3c5f6908ff23369f1f1026e1eb5979caff783a7d
---

## Findings

Approved under Helmsman GCS-EXT 4 same-turn directive for R1.

The commit resumes `Generator.prototype.throw(value)` by restoring the saved frame and injecting the thrown value into the active catch handler through the same operand-stack/pc transition used by normal interpreter exception handling.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 59 passed and 1 ignored.
3. Focused GCS tests: 6 passed.
4. CLI smoke: caught throw yields `oops! false`; uncaught throw propagates and completes the generator.
5. Slice measurement: 46 PASS / 23 FAIL / 0 SKIP from the same 69-row for-of/generator slice, +0 PASS over EXT 3.

Remaining failures are within known deferred scope: `return`, `yield*`, IteratorClose, and destructuring interactions.

**APPROVED for push.**
