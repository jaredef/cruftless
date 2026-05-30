---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - b5948e9c03fd25bd1f1e5972913cd368036d4a23
target_branch: main
summary: node:zlib sync API batch for mongoose loader residual
risk_class: substrate
gates_pre:
  mongoose_smoke: FAIL node:zlib.gunzipSync not yet implemented
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  compression_tests: cargo test --release -p rusty-compression PASS
  sync_smokes: gzip/gunzip PASS; deflate/inflate PASS; deflateRaw/inflateRaw PASS; brotliDecompress PASS
  package_smoke: mongodb PASS; redis PASS; webpack PASS; mongoose advanced to Buffer.readUInt32BE residual
---

## Substrate Moves

This closes the node:zlib sync API batch authorized by helmsman message
`8e61f482-0f4c-452d-9b2a-629426635f71`.

- **M** = `node:zlib` had method names and constants but all behavior-bearing
  sync methods were stubs; `mongoose` executes `gunzipSync` during import.
- **T** = wire `cruftless/src/zlib.rs` to the existing `rusty-compression`
  substrate for gzip/zlib/raw deflate encode/decode plus Brotli decode.
- **I** = `cruftless/Cargo.toml`, `cruftless/src/zlib.rs`, MILF trajectory,
  deferrals ledger entries for Buffer numeric readers and Brotli compression.
- **R** = the named zlib blocker is closed; `mongoose` advances to a distinct
  `Buffer.prototype.readUInt32BE` residual.

## Per-Method Results

- `gzipSync` + `gunzipSync`: PASS, round-trips `hello`.
- `deflateSync` + `inflateSync`: PASS, round-trips `hello`.
- `deflateRawSync` + `inflateRawSync`: PASS, round-trips `hello`.
- `brotliDecompressSync`: PASS on known `Hello, World!` stream.
- `brotliCompressSync`: explicit unsupported operation; no Brotli encoder
  substrate exists yet.

## Risk Assessment

The change consumes the existing compression pilot rather than introducing new
compression logic or third-party dependency surface. Encoders use stored-block
DEFLATE, which is larger than LZ77 output but format-correct and accepted by
conforming decoders.

## Composes-With

- `pilots/compression/derived`
- `pilots/missing-intrinsic-loader-failures/trajectory.md`
- `apparatus/docs/deferrals-ledger.md` Entries 014 and 015
- Sidecar artifacts under
  `/home/jaredef/Developer/cruftless-r1-sidecar/results/node-zlib-sync-r1/`

**APPROVED for push** per same-turn helmsman directive.
