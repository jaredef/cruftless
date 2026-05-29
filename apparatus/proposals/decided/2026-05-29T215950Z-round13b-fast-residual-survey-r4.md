---
proposal_slug: 2026-05-29T215950Z-round13b-fast-residual-survey-r4
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T21:59:50Z
covers_commits:
  - bfcee498097e399ba3c6b83990b42d2b298969da
---

## Findings

Approved under helmsman directive `1a067a1b-93ad-4db2-98b9-9221492f3740`.

The commit is apparatus/measurement work only. It:

1. Founds a scoped survey locale for the post-Round-13 residual check.
2. Records the reduced Bun-vs-cruft parity sample and persists the raw artifact.
3. Refreshes the locale manifest.
4. Names the next dominant residual family as CNSDR namespace-shape diff, not TDZ.

Verification:

1. Rebase: `git fetch origin main && git rebase origin/main` PASS.
2. Build: `cargo build --release --bin cruft -p cruftless` PASS.
3. Parity sample: 19 packages, 7 PASS / 12 FAIL / 0 SKIP.
4. Dominant cluster by count: namespace-shape diff on `readable-stream`, `events`, `winston`, `proj4`, and `decimal.js-light`.

The only operational wrinkle was missing local Bun and the absent `/media/jaredef/T7/...` sandbox path. The final measurement still used the real parity harness with a freshly installed local Bun binary and `/tmp/parity-sandbox`, so the result is a genuine current sample rather than a stale-json reconstruction.

**APPROVED for push.**
