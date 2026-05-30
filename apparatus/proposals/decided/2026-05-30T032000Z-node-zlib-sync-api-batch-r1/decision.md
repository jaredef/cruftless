---
proposal_slug: 2026-05-30T032000Z-node-zlib-sync-api-batch-r1
decision: APPROVED
arbiter_session: helmsman-self-adjudicated-per-same-turn-approval
decided_at: 2026-05-30T03:20:00Z
covers_commits:
  - b5948e9c03fd25bd1f1e5972913cd368036d4a23
---

## Findings

Approved under helmsman directive `8e61f482-0f4c-452d-9b2a-629426635f71`.

The substrate commit closes the directed `node:zlib.gunzipSync` residual and
lands the requested sync-method batch:

1. Reuses `pilots/compression/derived` for gzip, zlib, raw DEFLATE, and Brotli
   decode.
2. Adds host `node:zlib` sync methods for `gzipSync`, `gunzipSync`,
   `deflateSync`, `inflateSync`, `deflateRawSync`, `inflateRawSync`, and
   `brotliDecompressSync`.
3. Leaves `brotliCompressSync` as an explicit unsupported operation and records
   the missing Brotli encoder as a deferral.
4. Records the newly surfaced `mongoose` `Buffer.readUInt32BE` residual as a
   separate deferral.

Verification:

1. Build: `cargo build --release --bin cruft -p cruftless` PASS.
2. Runtime lib tests: `cargo test --release -p rusty-js-runtime --lib` PASS,
   73 passed and 1 ignored.
3. Compression tests: `cargo test --release -p rusty-compression` PASS, 17
   passed across unit and verifier tests.
4. Sidecar smokes: gzip/gunzip, deflate/inflate, deflateRaw/inflateRaw, and
   Brotli decompress all PASS.
5. Package smoke: `mongodb` PASS, `redis` PASS, `webpack` PASS, `mongoose`
   advances past zlib to `readUInt32BE`; `fastify` advances independently to
   `ESCAPE_REGEXP is not safe`.

**APPROVED for push.**
