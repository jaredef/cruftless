---
proposal_slug: 2026-05-29T150400-gcs-ext-2c-lifecycle-wiring
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T15:04:00Z
covers_commits:
  - f00214efde7b46235e27802784811ace5326a300
---

## Findings

Approved under Helmsman GCS-EXT 2c same-turn directive for R1.

The commit routes plain sync generator construction through `GeneratorObject` with an initial `FrameSnapshot`, resumes through `Generator.prototype.next`, and lets `Op::Yield` capture the next continuation. This removes eager body execution for plain generators while keeping async/legacy generator paths isolated.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 56 passed and 1 ignored.
3. CLI smoke: finite generator `.next()` values are `1`, `2`, then done; infinite generator construction returns immediately and first two values are `0`, `1`.
4. Post-EPSUA for-of/generator slice: 34 PASS / 35 FAIL / 0 SKIP from 69 baseline FAIL rows at `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext2c-forof-generators-20260529T150231Z/summary.json`.

Remaining failures are within known deferred scope: `next(value)`, `throw`, `return`, `yield*`, iterator-close, and destructuring interactions.

**APPROVED for push.**
