---
proposal_slug: 2026-05-29T152100-gcs-ext-3-next-value-injection
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T15:21:00Z
covers_commits:
  - 3d1e5f1edf100389b5900f1c700ae9c8ac3701de
---

## Findings

Approved under Helmsman GCS-EXT 3 same-turn directive for R1.

The commit injects `next(value)` by overwriting the saved yield-expression placeholder in `FrameSnapshot::operand_stack` before restoring the frame. This preserves first-call semantics and avoids an opcode rewrite.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 57 passed and 1 ignored.
3. Focused GCS tests: 4 passed.
4. CLI smoke: `it.next(7)` yields `1 false`; `it.next(42)` returns `43 true`.
5. Slice measurement: 46 PASS / 23 FAIL / 0 SKIP from the same 69-row for-of/generator slice, +12 PASS over EXT 2c.

Remaining failures are within known deferred scope: `throw`, `return`, `yield*`, iterator-close, and destructuring interactions.

**APPROVED for push.**
