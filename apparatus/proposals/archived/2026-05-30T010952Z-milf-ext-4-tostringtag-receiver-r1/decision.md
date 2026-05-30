---
proposal_slug: 2026-05-30T010952Z-milf-ext-4-tostringtag-receiver-r1
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-30T01:09:52Z
covers_commits:
  - df8e5899497816841aa457b4e77a9c391ab03b48
---

## Findings

Approved under helmsman directive `8fa1a44f-53f9-49e0-be40-636d83cfff9f`.

The substrate commit closes the directed toStringTag receiver failure:

1. Reproduces the `mongoose` and `slonik` `receiver='toStringTag'` failures in a sidecar package sandbox.
2. Identifies the root cause as descriptor reflection asymmetry, not missing `Symbol.toStringTag` installation.
3. Makes `Object.getOwnPropertyDescriptor(_, Symbol.toStringTag)` share the same transitional well-known-symbol fallback used by property reads.
4. Adds a focused runtime regression for typed-array toStringTag descriptor visibility.
5. Records the advanced `mongoose` `require("..")` blocker as a deferred CJS parent-directory resolution candidate.

Verification:

1. Focused regression: `cargo test --release -p rusty-js-runtime --test run_golden typed_array_tostringtag_descriptor_is_visible_by_symbol_key -- --nocapture` PASS.
2. Build: `cargo build --release --bin cruft -p cruftless` PASS.
3. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS, 72 passed and 1 ignored.
4. Smoke: `slonik` PASS; `mongoose` no longer fails on `receiver='toStringTag'`.

**APPROVED for push.**
