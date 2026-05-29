---
proposal_slug: 2026-05-29T133906-gcs-ext-2a-frame-snapshot
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T13:39:06Z
covers_commits:
  - 33989d0df08c5569b5b2208ee17487a89d2a41e4
---

## Findings

Approved under Helmsman GCS-EXT 2a same-turn directive for R1.

The commit introduces the owned `FrameSnapshot` continuation payload and frame restore helper without wiring generator behavior. The semantic-sensitive clone boundary is binding-cell identity: `local_cells`, `this_cell`, and `upvalues` clone the shared cell handles, not the pointed values. Runtime caches are excluded and can restart cold on the first correctness implementation.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 53 passed and 1 ignored.
3. Behavior change: none, because the snapshot type is dormant.

**APPROVED for push.**
