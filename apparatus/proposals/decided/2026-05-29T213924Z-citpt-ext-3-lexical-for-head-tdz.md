---
proposal_slug: 2026-05-29T213924Z-citpt-ext-3-lexical-for-head-tdz
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T21:39:24Z
covers_commits:
  - 6ca22e0859a413bfc537bfb0030004d237f2a5ce
---

## Findings

Approved under the helmsman same-turn authorization carried by message `8fd175f0-0c4a-4260-bd2c-4caac47a9c99`.

The substrate commit closes the current lexical loop-head TDZ residual by:

1. Replacing `StoreLocal` with `InitLocal` at the `for-of` lexical head seed site.
2. Replacing `StoreLocal` with `InitLocal` at the `for-in` lexical head seed site.
3. Recording the re-diagnosis and closure in the locale trajectory.
4. Adding a regression test for the empty lexical `for-in` head false-TDZ shape.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 71 passed and 1 ignored.
3. Targeted tests: `t10b_forof_object_entries_destructure_head`, `t10c_forin_empty_lexical_head_does_not_false_tdz`, and `t11_object_rest` all PASS.
4. CITPT-EXT 3 package smoke: 6 PASS / 3 FAIL from 9 packages, with the remaining failures attributed to non-TDZ parser and filesystem scopes.

The standing-rule concern here was false closure against the wrong emit path; the final proposal reflects the Phase-2 re-diagnosis and the actual compiler seed-path sibling of CITPT-EXT 2.

**APPROVED for push.**
