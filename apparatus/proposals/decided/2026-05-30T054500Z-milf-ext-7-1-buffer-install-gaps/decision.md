---
proposal_slug: 2026-05-30T054500Z-milf-ext-7-1-buffer-install-gaps
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10526
decided_at: 2026-05-30T05:45:00Z
covers_commits:
  - 67f711cc97bdea019bd1bac9b410318ef767e529
---

## Findings

Approved per keeper Telegram 10526 ("1 then 2 then 3"), within the scope of #3 (mongoose cross-package smoke verification). The smoke discovered that the new MILF-EXT 7 readers were unreachable via three pre-existing buffer-creation paths; landing the install-gap fix is the natural completion of #3's intent.

Substrate commit `67f711cc` adds `install_buffer_methods` calls at three production buffer-creation sites that previously installed only a subset (or nothing) on the resulting buffer object:

1. `cruftless/src/zlib.rs::buffer_from_bytes` — was calling `install_zlib_buffer_methods` (toString-only) without first installing the full prototype surface. zlib outputs are Buffers and consumers expect the full numeric reader surface (e.g. `gunzipSync(...).readUInt32BE(...)`).
2. `cruftless/src/node_stubs.rs::Buffer.allocUnsafeSlow` — alloc-shape factory created the `__is_buffer__`-tagged object and returned it without installing methods at all.
3. `cruftless/src/node_stubs.rs::Buffer.concat` — concat result was returned without installing methods.

`install_buffer_methods` was promoted from `fn` to `pub(crate) fn` so `zlib.rs` could call it. The zlib path still layers `install_zlib_buffer_methods` (the toString override) on top.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed, 1 ignored.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. Isolated Buffer smoke `/tmp/smoke/buf.mjs` — all round-trips PASS, OOB throws `ERR_OUT_OF_RANGE`.
5. Mongoose smoke `legacy/host-rquickjs/tests/fixtures/consumer-mongoose-app/main.mjs` — advanced past the prior `readUInt32BE` failure inside saslprep's `memory-code-points.js:12` (called via mongodb's SCRAM auth path); new residual surfaced at `mongoose/lib/cast/bigint.js:18:65` with `Cannot mix BigInt and other types` on the template-literal `ERROR_MESSAGE` that interpolates `${MIN_BIGINT}` / `${MAX_BIGINT}`.

## Named follow-up

`bigint.js:18:65 Cannot mix BigInt and other types` is a parser/runtime-tier gap: template-literal interpolation calls `ToNumber` on BigInt instead of `ToString`. That's the next mongoose blocker but lives at a distinct coordinate from the Buffer surface. Named in `pilots/missing-intrinsic-loader-failures/trajectory.md` as a follow-up rung, not landed here.
