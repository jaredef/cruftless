---
proposal_slug: 2026-05-29T143802-gcs-ext-2b-yield-boundary-opcode
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T14:38:02Z
covers_commits:
  - 32bfb91659b023edc679a1b91ea8b765e62bcb20
---

## Findings

Approved under Helmsman GCS-EXT 2b same-turn directive for R1.

The commit selects the opcode route rather than a new `run_frame` control-flow enum. `Op::Yield` is a direct interpreter boundary: in legacy mode it preserves the eager collector; in suspension mode it stores `FrameSnapshot` into the active generator object and returns the yielded value through `run_frame`.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 54 passed and 1 ignored.
3. Targeted exemplar: `interp::gcs_tests::yield_opcode_captures_active_generator_frame_snapshot` PASS.
4. Legacy eager generator smoke: PASS (`1 2 true`).

**APPROVED for push.**
