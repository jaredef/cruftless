helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 468c34c6

target_branch: main
summary: MILF-EXT 3 - Buffer writer surface for pg-protocol-style writers
risk_class: substrate
gates_pre:
  local_smoke: pg-protocol writer path blocked by missing Buffer writer methods

gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: not rerun (node_stubs scope)
  push: pending
---

## Substrate Moves

- Added `Buffer.prototype.write`, `writeInt32BE`, `writeUInt8`, `writeUInt16BE`,
  `writeUInt16LE`, `writeUInt32BE`, and `writeUInt32LE` to `cruftless/src/node_stubs.rs`.
- Added helper `encode_buffer_write_value` to support encoding-aware
  `Buffer.prototype.write` behavior.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- Targeted smoke (`/tmp/milf-ext-3-smoke-r2-exact/milf-slonik-probe.mjs`):
  - `typeof buf.write === function`
  - `typeof buf.writeInt32BE === function`
  - `pg-protocol` Writer path can encode/join values.
  - `slonik`/`mongoose` still fail with toStringTag receiver-get in bson path.

## Risk Assessment

This rung is narrowly scoped to Buffer writer surface behavior needed by `pg-protocol`.
It is intentionally not expanding into `safe-stable-stringify` / `bson` toStringTag import
resolver behavior.

## Composes-With

- `pilots/missing-intrinsic-loader-failures/trajectory.md`
- `cruftless/src/node_stubs.rs`
- `MILF-EXT 4` residual toStringTag follow-up
