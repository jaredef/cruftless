---
proposal_slug: 2026-05-29T191803-milf-ext-1-dataview-numeric-methods
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-29T19:18:03Z
covers_commits:
  - fbc7943c852c4dcbd9226d710c4b447c0869cbbe
---

## Findings

Approved under the Helmsman MILF-EXT 1 directive and inline resend.

The substrate commit installs the scoped ordinary Number-valued DataView
numeric method surface:

1. `getUint8`, `getInt8`, `getUint16`, `getInt16`, `getUint32`, `getInt32`,
   `getFloat32`, `getFloat64`.
2. `setUint8`, `setInt8`, `setUint16`, `setInt16`, `setUint32`, `setInt32`,
   `setFloat32`, `setFloat64`.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS,
   66 passed and 1 ignored.
3. Direct smoke PASS for Array indexed helpers and DataView numeric get/set.
4. Inline 30-cell first-coordinate accounting: `file-type` and `pdfkit` close
   at the DataView coordinate; 28 residual rows remain in non-DataView
   sub-coordinates.

**APPROVED for push.**
