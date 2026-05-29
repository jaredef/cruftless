---
proposal_slug: 2026-05-29T164000-gcs-residuals-delegate-abrupt
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T16:40:00Z
covers_commits:
  - b90969f1155fe0d6f58b618f23258a9eafcf643b
---

## Findings

Approved under Helmsman consolidated GCS residuals directive for R1, using the directive's explicit scope-down option.

The commit forwards abrupt completions at active `yield*` delegate sites:

1. `gen.throw(value)` calls the delegate iterator's `throw(value)` when callable, or IteratorCloses and rethrows when absent.
2. `gen.return(value)` calls the delegate iterator's `return(value)` when callable, yielding non-done cleanup results and completing on done results.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 65 passed and 1 ignored.
3. Focused GCS tests: 12 passed.
4. Broad slices remained stable: for-of/generator 50 PASS / 19 FAIL; GeneratorPrototype/return 13 PASS / 10 FAIL.

Remaining residuals are follow-up scope: generic for-of IteratorClose, destructuring close edge cases, and EXT 5 completion-kind metadata.

**APPROVED for push.**
