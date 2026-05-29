---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - fbc7943c852c4dcbd9226d710c4b447c0869cbbe
target_branch: main
summary: MILF-EXT 1 - scoped DataView numeric methods
risk_class: substrate
gates_pre:
  inline_30_cell_first_error: 0 PASS / 30 FAIL
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  direct_smoke: DataView ordinary numeric methods PASS
  inline_30_cell_package_run: 1 PASS / 29 FAIL / 0 SKIP
  inline_30_cell_first_coordinate: 2 rows advanced past DataView
---

## Substrate Moves

This is the scoped core-prototype intrinsic rung from the
missing-intrinsic-loader-failures locale.

- **M** = DataView numeric method surface absent from `%DataView.prototype%`.
- **T** = install the ordinary Number-valued DataView get/set table through
  64-bit float.
- **I** = shared DataView receiver validation, byte offset coercion, endian byte
  read/write helpers, and methods for Uint8/Int8/Uint16/Int16/Uint32/Int32/
  Float32/Float64.
- **R** = inline first-error list advances the two DataView rows: `file-type`
  (`setUint32`) reaches package PASS, while `pdfkit` (`getUint32`) advances past
  the DataView coordinate and remains non-parity on output shape.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 66 passed, 1 ignored.
- Direct cruft smoke PASS for:
  - `Array.prototype.findIndex/findLast/findLastIndex/map/filter`;
  - `DataView.prototype.{get,set}{Uint8,Int8,Uint16,Int16,Uint32,Int32,Float32,Float64}`.
- Inline 30-cell package measurement:
  `/home/jaredef/Developer/cruftless-r2-sidecar/results/milf-ext1-inline30-20260529T191754Z.json`
  reports 1 PASS / 29 FAIL / 0 SKIP.
- Inline 30-cell first-coordinate accounting: 2 rows advance past DataView;
  package PASS gain is 1 row (`file-type`).

## Risk Assessment

The change is scoped to `%DataView.prototype%` and the existing ArrayBuffer
record storage. It does not alter Array, TypedArray, Buffer, namespace, or host
shim behavior. Remaining MILF rows are explicitly outside this substrate move:
Buffer writer methods, CJS/ESM namespace shape, Node shims, web globals,
safe-stable-stringify `Symbol.toStringTag` access, output shape mismatch, and
the `brotli` null-flow outlier.

## Composes-With

- `pilots/missing-intrinsic-loader-failures/`.
- TypedArray/DataView substrate under `pilots/rusty-js-runtime/derived/src/intrinsics.rs`.
- Follow-up Buffer writer and namespace-shape rungs.

**APPROVED for push** per Helmsman MILF-EXT 1 directive.
