---
proposal_slug: 2026-05-29T052733-h262s-ext-1-detacharraybuffer-shim
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T05:27:33Z
covers_commits:
  - c9119619e07f1ecff011fc74b95ac08f094056f6
---

## Findings

Approved under Helmsman H262S-EXT 1 same-turn imperative for R1.

The proposed commit stays within the approved `$262.detachArrayBuffer` cheap-shim scope. It does not implement `createRealm`, IsHTMLDDA, agent hooks, GC hooks, or the heterogeneous `$262` tail.

Substrate-tier verification:

1. Approved focused probes: 6/6 PASS.
2. Adjacent ArrayBuffer/DataView regression probes: 7/7 PASS.
3. Build: `cargo check -p cruftless -p rusty-js-runtime` PASS.
4. Build: `cargo build --release --bin cruft -p cruftless` PASS.

**APPROVED for push.**
