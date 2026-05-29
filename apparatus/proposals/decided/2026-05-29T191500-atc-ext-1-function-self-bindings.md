---
proposal_slug: 2026-05-29T191500-atc-ext-1-function-self-bindings
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T19:15:00Z
covers_commits:
  - 62794d17882953bf33f8c58e08e5ab582b9e43de
---

## Findings

Approved under the Helmsman ATC Phase 3 directive.

The substrate commit splits named function expression self-name const bindings
from function declaration mutable bindings. The focused regressions cover both:
function declaration self-rewrite succeeds, named function expression
self-rewrite still throws.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS,
   68 passed and 1 ignored.
3. Focused self-reassignment tests: 2 passed.
4. ATC 26-package slice: 13 PASS / 13 FAIL, +13 over baseline.

Remaining package failures are downstream non-ATC residuals.

**APPROVED for push.**
