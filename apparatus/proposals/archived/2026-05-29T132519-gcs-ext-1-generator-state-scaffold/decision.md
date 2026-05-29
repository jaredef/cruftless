---
proposal_slug: 2026-05-29T132519-gcs-ext-1-generator-state-scaffold
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T13:25:19Z
covers_commits:
  - 6a423878048693daa9eab947826416413a0b5088
---

## Findings

Approved under Helmsman GCS-EXT 1 same-turn directive for R1.

The commit selects bytecode-level CPS / save-restore of interpreter `Frame` state as the generator suspension design. The scaffold is dormant: it adds the generator state object surface and runtime entry-point placeholders without replacing the existing eager-collected generator behavior.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS.
3. Full runtime integration tests are blocked by pre-existing `Runtime.globals` references in integration tests.
4. Diff-prod `generators` oracle comparison is blocked by missing `bun` (`rc=127`) in the R1 worktree environment.

**APPROVED for push.**
