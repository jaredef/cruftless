# missing-intrinsic-loader-failures — Trajectory

## MILF-EXT 0 — Phase 0 spawn + Phase 2 probe (2026-05-29)

### Trigger

CAACP directive `51049de5-ceb9-4f10-879a-577410955ced` requested Phase 0 + Phase 2 only for the top500 missing-intrinsic loader cluster:

- 14 packages with `callee is not callable: undefined`.
- 16 packages with `Cannot read property <name> of undefined`.
- Deliverable: spawn SHA, segmentation, C4 result, and proposed Phase 3 move shape.

### Source Availability

The directive source path was not mounted in this session:

`/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json`

Observed evidence:

- `/media/jaredef/T7` does not exist.
- `/media/jaredef` exists but is empty aside from the mountpoint directory.
- A full filesystem search for `parity-results-top500-20260529T111702-refined.json` returned no hits before timeout.

Fallback evidence used for Phase 2: `legacy/host-rquickjs/tools/parity-results-cluster-dyn-import.json`, which contains the same named packages and error-shape strings for the packages named in the directive.

Helmsman later sent `862bf825-1f32-4029-9024-4e3fd1f58551`, naming replacement files under `/home/jaredef/Developer/cruftless-sidecar/parity-results/`:

- `cluster-callee-not-callable.json`
- `cluster-cannot-read-property.json`
- `parity-results-top500-20260529T111702-refined.json`

That directory was also absent in this filesystem. `find /home/jaredef/Developer -path '*parity-results*' ...` did not locate the replacement files, and `find /home/jaredef/Developer/cruftless-sidecar -type f | rg 'callee|cannot-read|top500-20260529T111702|parity-results'` returned no hits. The phase therefore remains a fallback probe with an exact-source blocker.

### Sampled Rows

| Package | Family | First missing surface | Category | Phase-3 coordinate |
|---|---|---|---|---|
| `@koa/router` | callee-not-callable | `methods` result is ordinary object, `.map` missing | wrong prototype / array-like shape | Node module shim shape for `methods` / Array result normalization |
| `express` | callee-not-callable | `methods` result is ordinary object, `.map` missing | wrong prototype / array-like shape | Same as `@koa/router` |
| `rollup-plugin-node-resolve` | callee-not-callable | `builtin-modules` result is ordinary object, `.filter` missing | wrong prototype / array-like shape | CommonJS namespace / array export shape |
| `file-type` | callee-not-callable | DataView-like receiver lacks `setUint32` | missing built-in method / wrong DataView prototype | DataView intrinsic/prototype wiring |
| `chai` | callee-not-callable | event-like object lacks `dispatchEvent` | missing global/web shim | EventTarget surface or DOM event shim |
| `log4js` | callee-not-callable | `Array.prototype.findIndex` missing | missing built-in method | Array prototype ES2015 method surface |
| `@octokit/request` | callee-not-callable | Function method `call` resolves to an ordinary object, not callable | wrong prototype chain | Function.prototype / callable object shape |
| `execa` | callee-not-callable | stream helper `getDefaultHighWaterMark` missing | missing Node-compat shim | Node streams compatibility |
| `@mswjs/data` | cannot-read-property | superclass `MessageEvent` undefined, `.prototype` read | missing global/web shim | DOM `MessageEvent` global |
| `fake-indexeddb` | cannot-read-property | superclass `FDBCursor` undefined, `.prototype` read | missing global/web shim | IndexedDB global/class surface |
| `jsdom` | cannot-read-property | webidl-conversions reads `.get` on undefined prototype slot | missing built-in/web intrinsic | ArrayBuffer/DataView/WebIDL intrinsic surface |
| `mnemonist` | cannot-read-property | namespace `FibonacciHeap.MinFibonacciHeap` undefined | wrong namespace object shape | ESM/CJS namespace population |
| `mongodb` | cannot-read-property | `Long.fromInt` undefined in `bson` | missing Node package shim / namespace shape | CJS namespace constructor statics |
| `mongoose` | cannot-read-property | same `bson` `Long.fromInt` undefined | missing Node package shim / namespace shape | Same as `mongodb` |
| `brotli` | cannot-read-property | `.slice` read from null receiver `c` | non-intrinsic semantic/null-flow gap | Probe-limited until exact source row is inspected |
| `workerpool` | cannot-read-property | `self.navigator` undefined | missing global/web shim | Worker/global `self` and navigator surface |

### Segmentation

The two reason strings are not one root cause.

They are adjacent symptoms emitted by dynamic import once package initialization reaches an absent substrate surface. The sampled rows split into at least five substrate coordinates:

1. **Wrong prototype or namespace shape**: array-like objects without Array prototype methods, CJS namespace statics missing, or callable prototypes wrong. Exemplars: `@koa/router`, `express`, `rollup-plugin-node-resolve`, `mnemonist`, `mongodb`, `mongoose`, `@octokit/request`.
2. **Missing built-in prototype methods/intrinsics**: Array ES2015 methods or DataView methods absent from the receiver's prototype chain. Exemplars: `log4js`, `file-type`, `jsdom`.
3. **Missing Node-compat shims**: streams/process/util surfaces expected by packages. Exemplar: `execa`.
4. **Missing web/global shims**: DOM/EventTarget/IndexedDB/Worker-like globals expected by browser-adjacent packages. Exemplars: `chai`, `@mswjs/data`, `fake-indexeddb`, `workerpool`.
5. **Probe-limited non-intrinsic semantic/null-flow rows**: exemplar `brotli`, where the immediate receiver is null rather than an absent intrinsic object.

### C4 Reason-Coherence

- **C1 sibling**: HOLDS at the broad loader-intrinsic symptom level, and HOLDS within subfamilies such as array-like wrong-prototype (`@koa/router`, `express`, `rollup-plugin-node-resolve`) and CJS namespace statics (`mongodb`, `mongoose`, `mnemonist`).
- **C2 shape-compat**: FAILS for a single Phase 3 substrate move across all 30 cells. The rows mix prototype repair, CJS namespace population, Node stream/process shims, and web globals.
- **C3 cost-positive**: HOLDS for splitting Phase 3 by substrate coordinate. A wrong-prototype/namespace probe is likely high yield because several rows share it; per-package shims would be brittle.
- **C4 bail-safe**: HOLDS only with the stated evidence boundary. The exact refined top500 JSON was unavailable, so this rung records fallback segmentation and does not claim final package-count closure.

### Proposed Phase 3 Move Shape

Start with a narrow **wrong-prototype / namespace-shape Phase 3**, not a blanket intrinsic-loader patch:

1. Rehydrate the exact refined JSON and isolate the 30 directive cells.
2. Pick the sibling subcluster where the same substrate can flip multiple cells:
   - First candidate: array-like CJS exports that should be arrays (`methods`, `builtin-modules`) but arrive as ordinary objects, causing `.map` / `.filter` misses.
   - Second candidate: CJS namespace statics missing from constructor-like exports (`Long.fromInt`, `FibonacciHeap.MinFibonacciHeap`).
3. Probe one direct package pair from each candidate (`express` + `@koa/router`; `mongodb` + `mongoose`) before editing substrate.
4. Only after namespace/prototype shape is separated, route web-global rows (`MessageEvent`, `FDBCursor`, `self.navigator`, `dispatchEvent`) to a distinct DOM/web-global compatibility locale or sublocale.

### Status

Phase 0 locale spawned and Phase 2 fallback segmentation complete. Exact-source inspection is blocked by the missing T7 mount/source JSON and by the absent replacement sidecar parity-results directory.

## 2026-05-29 — MILF-EXT 1 core prototype intrinsic audit

### Trigger

CAACP directive `4250d53d-83c7-4c9e-b560-7a01bb981046` requested the first
substrate rung against the core prototype intrinsic segment from Phase 2:

- audit Array.prototype, DataView.prototype, and broader indexed-collection
  tables;
- implement the highest-impact missing methods, with scope-down if the audit
  reveals more than ten missing methods;
- verify build, runtime tests, exemplar packages, and the 30-cell cluster.

### Audit

Array.prototype:

- `map`, `filter`, `find`, `findIndex`, `findLast`, `findLastIndex`,
  `some`, `every`, `reduce`, `reduceRight`, `lastIndexOf`, `at`, `flat`,
  `flatMap`, `toReversed`, `toSorted`, `toSpliced`, `with`, iterator triplet,
  and `sort` are already installed through
  `pilots/rusty-js-runtime/derived/src/prototype.rs` and
  `pilots/rusty-js-runtime/derived/src/generated.rs`.
- The `log4js` first-coordinate offender, `Array.prototype.findIndex`, is
  already present on current main. It is not the remaining blocker for this
  rung's local runtime state.
- The `.map` / `.filter` offender rows from Phase 2 point at wrong receiver
  shape (`Object keys=[0,1,2,...]` or namespace object), not missing Array
  prototype methods.

TypedArray.prototype:

- The shared typed-array prototype already carries `find`, `findIndex`,
  `findLast`, `findLastIndex`, `map`, `filter`, `reduce`, `reduceRight`, and
  related indexed-collection methods, mirrored onto the spec-level
  `%TypedArray%.prototype` surface.

DataView.prototype:

- Before this rung, DataView had constructor/accessor shape
  (`byteLength`, `byteOffset`, `buffer`) but no numeric read/write methods.
- Full ECMA-262 §25.3 DataView method parity is larger than the conservative
  rung budget: the complete table includes 8-, 16-, 32-, 64-bit integer,
  float16/32/64, and BigInt get/set methods.
- Scope-down applied to the cluster-cited numeric methods and the adjacent
  ordinary Number-valued DataView table:
  `getUint8`, `getInt8`, `getUint16`, `getInt16`, `getUint32`, `getInt32`,
  `getFloat32`, `getFloat64`, `setUint8`, `setInt8`, `setUint16`,
  `setInt16`, `setUint32`, `setInt32`, `setFloat32`, `setFloat64`.

### Substrate Move

`DataView.prototype` now installs the scoped ordinary Number-valued numeric
methods through 64-bit float.
The shared helper performs:

- DataView receiver validation through the existing `__kind === "DataView"`
  sentinel and `typed_array_views` record;
- detached/missing ArrayBuffer rejection;
- byte offset coercion and bounds checking against fixed or growable view
  length;
- little-endian argument handling;
- read/write through the runtime's ArrayBuffer byte storage.

### Verification

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS, 66 passed, 1 ignored.
- Direct cruft smoke probe: PASS for
  `Array.prototype.findIndex/findLast/findLastIndex/map/filter` and
  `DataView.prototype.{get,set}{Uint8,Int8,Uint16,Int16,Uint32,Int32,Float32,Float64}`.

### Cluster Measurement Blocker

The package exemplar and 30-cell PASS-gain measurement could not be completed
in this filesystem:

- `/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json`
  is absent.
- `/home/jaredef/Developer/cruftless-sidecar/parity-results/cluster-callee-not-callable.json`
  is absent.
- `/home/jaredef/Developer/cruftless-sidecar/parity-results/cluster-cannot-read-property.json`
  is absent.
- `/home/jaredef/Developer/cruftless-sidecar/parity-results/parity-results-top500-20260529T111702-refined.json`
  is absent.
- Searches under `/home/jaredef/Developer`, `/home/jaredef/Developer/cruftless-sidecar`,
  `/home/jaredef/Developer/cruftless-r2-sidecar`, `/media/jaredef`, and `/tmp`
  did not locate those files.
- No local `parity-sandbox` directory for the directive's package cells was
  present.

### C4 Status

C4 holds for the scoped DataView intrinsic move: `file-type`'s first missing
coordinate names `setUint32`, the runtime lacked the DataView numeric method
surface, and the direct method-level probe passes after the change.

C4 does not hold for claiming combined 30-cell closure from this rung. Array
`map/filter` rows are wrong-receiver/namespace-shape rows, web/global rows are
host-shim rows, and `brotli` remains a value-flow outlier. The exact package
PASS-gain measurement remains blocked on the missing parity-result/sandbox
artifacts.

### Inline 30-Cell Measurement

Helmsman later resent the 30-cell source inline in CAACP message
`07e97eeb-7040-47ac-aa8b-01825c4bdc38`. The inline package list was run through
the local parity harness after adding the scoped DataView methods.

Artifact:
`/home/jaredef/Developer/cruftless-r2-sidecar/results/milf-ext1-inline30-20260529T191754Z.json`.

Package-level result after this rung: 1 PASS / 29 FAIL / 0 SKIP.

First-coordinate closure from this DataView rung:

| Package | Prior first error | Rung result |
|---|---|---|
| `file-type` | `DataView.prototype.setUint32` missing | package PASS |
| `pdfkit` | `DataView.prototype.getUint32` missing | first coordinate closed; package still FAILS on output shape mismatch |

Residuals in the 30-cell inline list:

- Buffer writer methods: `exceljs` (`writeUInt32LE`), `pg` (`offset`),
  `postgres` (`i`) remain Buffer/byte-writer shim work.
- Event/web globals: `chai` (`dispatchEvent`), `fake-indexeddb`,
  `twitter-api-v2`, and `agentkeepalive` remain host/web or class-surface work.
- Namespace/export shape: `@octokit/request`, `rollup-plugin-node-resolve`,
  `mnemonist`, `csurf`, and several callable-object rows remain namespace or
  wrong-prototype work.
- Node shims/globals: `gulp` (`TextDecoder`), `forever` (`process.umask`),
  `release-it` (`util.debug`), `mocha` (`features.require_module`),
  `aws-sdk` (`util.inherit`) remain Node compatibility work.
- Safe-stable-stringify `toStringTag` rows (`mongoose`, `mongodb`, `pino`,
  `pino-http`, `roarr`, `slonik`, `pino-debug`) are not DataView method rows.
- `brotli` remains a null value-flow outlier.

PASS-gain accounting for the inline 30 cells:

- Prior: 0 PASS / 30 FAIL at first error.
- MILF-EXT 1 first-coordinate closures: 2 rows (`file-type`, `pdfkit`).
- MILF-EXT 1 package PASS gain: 1 row (`file-type`); `pdfkit` advanced past
  the DataView getter but remains non-parity on package output shape.
- Direct substrate proof: cruft smoke confirmed
  `DataView.prototype.getUint32` and `setUint32` are callable and correctly
  read/write values after this rung.

Status after inline measurement: scoped DataView intrinsic rung is landable, but
the remaining MILF cluster should proceed by sub-coordinate rather than by
error string.

## 2026-05-29 — MILF-EXT 2 node-shim cluster

### Trigger

CAACP directive `7fd3f29c-a2e0-46fc-a5ab-d88e43fef338` targeted the Node shim
sub-cluster from the MILF residual set:

- `gulp`: `TextDecoder` undefined
- `forever`: `process.umask` undefined
- `release-it`: `util.debug` undefined
- `mocha`: `features.require_module` missing
- `aws-sdk`: `util.inherit` missing

### Substrate Move

The host shim surface is now present in the local worktree:

- `cruftless/src/process.rs` installs `process.umask()` and `process.features.require_module`.
- `cruftless/src/util.rs` exports `util.debug`, `util.inherit`, and forwards
  `TextDecoder` / `TextEncoder` from the global surface when available.

The node-shim rung is intentionally minimal:

- `process.umask()` returns the conventional Linux mask `0o022`.
- `util.debug()` returns a callable no-op logger.
- `util.inherit(ctor, super_)` wires `ctor.prototype` to inherit from
  `super_.prototype` and stamps `constructor`.
- `process.features.require_module` is truthy.

### Verification

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS, 68 passed, 1 ignored.

### Smoke Availability

I was able to find local package trees for `chai`, `mongoose`, `mongodb`, and
`file-type`, but not for the five directive packages as a complete local smoke
set. Package availability is therefore the blocker for a strict `import()` smoke
measurement on the exact named packages.

### C4 Status

C4 holds for the node-shim sub-coordinate: the named failures map to Node host
compatibility, not to the earlier DataView or namespace-shape coordinates. The
remaining work is package availability and, if needed, follow-up host-global
surface tuning rather than a different intrinsic family.

## 2026-05-30 — MILF-EXT 3 Buffer writer methods

### Trigger

CAACP authorization `6590d93b-2ec3-4514-876a-6bbe54183b77` followed earlier
authorization chain for `milf-ext-3` and requested landing of Buffer writer methods:

- `Buffer.prototype.write`
- `Buffer.prototype.writeInt32BE`
- `Buffer.prototype.writeUInt8`
- `Buffer.prototype.writeUInt16BE`
- `Buffer.prototype.writeUInt16LE`
- `Buffer.prototype.writeUInt32BE`
- `Buffer.prototype.writeUInt32LE`

### Substrate Move

Added the listed methods and shared value encoding support in
`cruftless/src/node_stubs.rs` (`install_buffer_methods`, plus
`encode_buffer_write_value`).

### Verification

- `cargo build --release --bin cruft -p cruftless`: PASS.
- Local target/debug run of `/tmp/milf-ext-3-smoke-r2-exact/milf-slonik-probe.mjs`:
  - `typeof buf.write === 'function'`
  - `typeof buf.writeInt32BE === 'function'`
  - `pg-protocol` `Writer().addInt32()` path executes and `join()` succeeds.
  - `slonik` and `mongoose` remain failing on:
    `Cannot read property 'get' of undefined (receiver='toStringTag')`

### C4 Status

MILF-EXT 3 is closed for the Buffer-writer coordinate and keeps strict scope:
it closes the writer-method blocker used by `pg-protocol`-style flows.
The toStringTag failure is a distinct residual and should proceed as a separate
follow-up.

### PASS-Gain Snapshot (targeted residual subset)

- First-coordinate closure achieved for the Buffer-writer subset (`exceljs`, `pg`,
  `postgres`) in the inline residual context.
- Residual `safe-stable-stringify`/`bson` toStringTag failure remains unchanged.

## 2026-05-30 — MILF-EXT 4 Symbol.toStringTag descriptor receiver

### Trigger

Helmsman directive `8fa1a44f-53f9-49e0-be40-636d83cfff9f` targeted the
post-EXT-3 residual:

`Cannot read property 'get' of undefined (receiver='toStringTag')`

The named package smokes were `mongoose` and `slonik`.

### Baseline Reproduction

Installed `mongoose` and `slonik` into the sidecar sandbox at:

`/home/jaredef/Developer/cruftless-r1-sidecar/results/milf-ext4-r1/`

Pre-fix cruft reproduced the directive error:

- `mongoose`: failed in `bson/lib/bson.cjs` while evaluating
  `Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype), Symbol.toStringTag).get`
- `slonik`: failed in `safe-stable-stringify/index.js` while evaluating
  `Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Object.getPrototypeOf(new Int8Array())), Symbol.toStringTag).get`

A reduced probe showed:

- `Symbol.toStringTag` existed.
- `%TypedArray%.prototype` already carried the `"@@toStringTag"` accessor.
- `Object.getOwnPropertyDescriptor(proto, Symbol.toStringTag)` returned
  `undefined`, causing the subsequent `.get` property read on `undefined`.

### Root Cause

The typed-array toStringTag accessor had already been installed at the correct
prototype level by the earlier typed-array work, but much of the runtime still
stores well-known symbol properties under transitional string keys such as
`"@@toStringTag"`.

Property reads already had a Symbol-to-string fallback through
`object_get_pk` / `find_getter_pk`, but `Object.getOwnPropertyDescriptor` used
a direct `properties.get(&Symbol(...))` path for symbol keys. That made the
descriptor invisible to package code even though ordinary symbol property reads
could find it.

### Substrate Move

`Runtime::get_own_property_descriptor_pk` now applies the same
well-known-symbol fallback used by property reads:

- first check the true `PropertyKey::Symbol` slot;
- then check the transitional string key from the symbol description.

`Object.getOwnPropertyDescriptor` now routes ordinary descriptor lookup through
that shared helper instead of duplicating a string-only shape path plus direct
symbol map lookup.

### Verification

- Focused regression:
  `cargo test --release -p rusty-js-runtime --test run_golden typed_array_tostringtag_descriptor_is_visible_by_symbol_key -- --nocapture`
  PASS.
- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS (`72 passed`, `1 ignored`).

Post-fix reduced probe:

- `Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype), Symbol.toStringTag)` returns an object.
- `.get` is callable.
- `.get.call(new Uint8Array())` returns `Uint8Array`.

Post-fix package smoke:

- `slonik`: PASS.
- `mongoose`: advanced past the toStringTag receiver failure and now fails later
  on a distinct module-resolution residual:
  `module not found: '..'` from `mongodb/lib/operations/drop.js`.

### C4 Status

C4 holds for the toStringTag descriptor receiver coordinate. The shared
descriptor lookup now composes with the existing well-known-symbol transitional
storage strategy, and both named package smokes no longer fail on
`receiver='toStringTag'`.

The new `mongoose` blocker is not part of this rung. It is a CJS parent-directory
resolution residual and should be tracked as a MILF-EXT 5 candidate if it
recurs across the package cluster.

### Finding

**Finding MILF.4.1**: Well-known symbol transitional storage must be symmetric
across read and reflection paths. A property read fallback alone is insufficient:
ecosystem packages reflect descriptor objects and call accessors directly, so
`Object.getOwnPropertyDescriptor(_, Symbol.toStringTag)` must share the same
Symbol-to-`"@@..."` compatibility lookup as `[[Get]]`.

## 2026-05-30 — MILF-EXT 5 SharedArrayBuffer byteLength descriptor

### Trigger

Helmsman directive `77473bb4-7590-4ccb-986c-cfadaecb1bd6` targeted the
post-EXT-4 `mongoose` residual:

`module not found: '..'` from `mongodb/lib/operations/drop.js`.

### Baseline Reproduction

Installed `mongoose`, `mongodb`, and `redis` into the sidecar sandbox at:

`/home/jaredef/Developer/cruftless-r1-sidecar/results/milf-ext5-r1/`

The stale local release binary reproduced the nominal parent-directory failure,
but a fresh build of current main showed that R2 commit `ae0f98b6` had already
closed the `require("..")` / dot-directory coordinate:

- reduced nested-package fixture: `require("..")` from
  `node_modules/pkg/lib/operations/drop.js` resolves to `pkg/lib/index.js`;
- `mongodb`: PASS;
- `redis`: PASS (`keyCount=58`);
- `mongoose`: advanced to
  `Cannot read property 'get' of undefined (receiver='prototype')` in
  `webidl-conversions/lib/index.js`.

### Root Cause

`webidl-conversions` checks:

`Object.getOwnPropertyDescriptor(SharedArrayBuffer.prototype, "byteLength").get`

Cruft exposed `SharedArrayBuffer` as a function with a prototype object, but the
prototype did not carry an accessor descriptor for `byteLength`. The constructor
was emitted through the typed-array constructor loop, so the name existed while
the ArrayBuffer-style internal-slot/accessor surface was absent.

### Substrate Move

`SharedArrayBuffer` now takes a conservative special branch inside the existing
typed-array installation loop:

- `SharedArrayBuffer.prototype` remains an ordinary prototype rather than a
  typed-array prototype;
- `SharedArrayBuffer.prototype.byteLength` is installed as a real accessor
  descriptor;
- `new SharedArrayBuffer(n)` allocates an object with a backing
  `ArrayBufferRecord`, allowing the accessor getter to return the recorded byte
  length.

This keeps the change scoped to the descriptor and constructor surface needed by
real package loaders; it does not claim full shared-memory or Atomics semantics.

### Verification

- Focused regression:
  `cargo test --release -p rusty-js-runtime shared_array_buffer_bytelength_descriptor_is_visible --test run_golden`
  PASS.
- Required R2 regression:
  `cargo test --release -p rusty-js-runtime module::tests::resolve_module_treats_dot_as_relative_directory`
  PASS.
- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS (`73 passed`, `1 ignored`).

Post-fix probe:

- `typeof SharedArrayBuffer === "function"`;
- `Object.getOwnPropertyDescriptor(SharedArrayBuffer.prototype, "byteLength")`
  returns an object;
- descriptor `.get` is callable;
- `Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength").get`
  remains callable.

Post-fix package smoke:

- `mongodb`: PASS;
- `redis`: PASS (`keyCount=58`);
- `mongoose`: advanced past the parent-directory and SharedArrayBuffer
  descriptor failures, then stopped at the independent host intrinsic residual
  `node:zlib.gunzipSync not yet implemented (Tier-Ω.5.y stub)`.

### C4 Status

C4 holds for the actual MILF-EXT 5 substrate coordinate surfaced by current
main: SharedArrayBuffer descriptor visibility for package loaders. The nominal
`require("..")` blocker is closed upstream by R2's dot-directory resolution
commit and remains covered by the required regression.

The new `mongoose` blocker is outside this rung. It is a host `node:zlib`
intrinsic residual and is recorded in the deferrals ledger as
`node-zlib-gunzip-sync-host-intrinsic`.

### Finding

**Finding MILF.5.1**: A global constructor stub is not loader-compatible unless
its reflected prototype descriptor surface exists. Ecosystem packages commonly
probe intrinsic support via `Object.getOwnPropertyDescriptor(...).get`, so
partial constructor exposure must include the accessor descriptors that package
feature-detection code observes.

## 2026-05-30 — MILF-EXT 6 node:zlib sync API batch

### Trigger

Helmsman directive `8e61f482-0f4c-452d-9b2a-629426635f71` targeted the
post-EXT-5 `mongoose` residual:

`node:zlib.gunzipSync not yet implemented (Tier-Ω.5.y stub)`.

The directive requested a sync-API batch rather than a single-method closure,
because package loaders commonly use multiple zlib sync entrypoints.

### Baseline

`cruftless/src/zlib.rs` exposed a populated `node:zlib` namespace with constants,
constructors, and method names, but every behavior-bearing method was still a
Tier-Ω.5.y stub. `mongoose` therefore stopped as soon as `mongodb` reached
`zlib.gunzipSync`.

### Substrate Move

The host `node:zlib` layer now composes with the existing
`pilots/compression/derived` substrate:

- `gzipSync` returns a gzip-wrapped stored-block DEFLATE Buffer;
- `gunzipSync` decodes gzip-wrapped DEFLATE;
- `deflateSync` returns a zlib-wrapped stored-block DEFLATE Buffer;
- `inflateSync` decodes zlib-wrapped DEFLATE;
- `deflateRawSync` returns raw stored-block DEFLATE;
- `inflateRawSync` decodes raw DEFLATE;
- `brotliDecompressSync` decodes Brotli via the existing RFC 7932 pilot path;
- `brotliCompressSync` remains a meaningful unsupported operation because no
  Brotli encoder substrate exists yet.

The returned objects are Buffer-like: indexed bytes, `length`, `__is_buffer__`,
and a minimal `toString([encoding])` method for UTF-8/hex/latin1/ascii consumer
flows.

### Verification

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS (`73 passed`, `1 ignored`).
- `cargo test --release -p rusty-compression`: PASS (`17 passed` across unit +
  verifier tests).

Sidecar sync-method smokes at
`/home/jaredef/Developer/cruftless-r1-sidecar/results/node-zlib-sync-r1/`:

- `gzipSync(Buffer.from("hello"))` then `gunzipSync(...).toString()` prints
  `hello`.
- `deflateSync` then `inflateSync` prints `hello`.
- `deflateRawSync` then `inflateRawSync` prints `hello`.
- `brotliDecompressSync` on a known encoded `Hello, World!` stream prints
  `Hello, World!`.
- `brotliCompressSync` throws the explicit not-yet-implemented message.

Post-fix package smoke:

- `mongodb`: PASS.
- `redis`: PASS (`keyCount=58`).
- `webpack`: PASS (`Object.keys(ns).length === 96`).
- `fastify`: advanced independently to `Error: the ESCAPE_REGEXP is not safe,
  update this module` (not a zlib residual).
- `mongoose`: advanced past zlib to a distinct Buffer numeric-read residual:
  `readUInt32BE` missing on Buffer-like objects in
  `@mongodb-js/saslprep/dist/memory-code-points.js`.

### C4 Status

C4 holds for the node:zlib sync decode/encode batch except Brotli compression,
which is explicitly out of scope due to absent encoder substrate. The named
mongoose zlib blocker is closed and the package advances to a separate Buffer
numeric-reader coordinate.

The new `mongoose` blocker is outside this rung and is recorded in the
deferrals ledger as `buffer-read-uint32be-host-method`.

### Finding

**Finding MILF.6.1**: Import-time zlib parity requires behavior, not only shape.
The previous namespace-stub was enough for feature detection, but package code
executes sync decompression during module initialization. The existing
compression pilot made the host closure cheap: zlib should consume the lower
compression substrate rather than duplicate compression logic in `cruftless`.

## 2026-05-30 - MILF-EXT 6: Redis Post-Load Promise.catch Terminal Rejection

### Directive

Investigate the Redis post-load failure reproduced by:

```js
import('redis').then(m => {
  console.log('OK', Object.keys(m).length);
  console.log(Object.keys(m).slice(0, 8).join(','));
}).catch(err => console.error(err));
```

Pre-fix cruft successfully loaded Redis and printed the export list, then ended
the turn with:

```text
cruft: unhandled promise rejection: "TypeError(\"callee is not callable: undefined [argc=1]\")"
```

### Root Cause

The Redis package itself was not the failing mechanism. Reduced probes showed
that `Promise.resolve(1).then(...).catch(...)` produced the same terminal
unhandled rejection.

Two Promise instance-method gaps composed into the Redis post-load failure:

- `Promise.prototype.catch` manually walked the prototype chain with
  `get_own("then")`. The runtime installs `Promise.prototype.then` through the
  shape-backed property path, so the manual lookup missed it and attempted to
  call `undefined`.
- `Promise.prototype.then` preserved non-callable handlers as reaction handlers.
  A `.catch(...)` call dispatches to `.then(undefined, onRejected)`; the
  fulfilled path later tried to call the `undefined` fulfillment handler instead
  of applying the spec identity propagation path.

### Substrate Move

`Runtime::promise_catch_via` now resolves `this.then` through `Runtime::get_via`,
matching the spec `Get` path and the runtime's shape-backed property storage.

`Runtime::promise_then_via` now normalizes `onFulfilled` and `onRejected` to
`None` unless the supplied value is callable. The existing promise reaction
machinery already implements the required identity/thrower propagation for
missing handlers, so this keeps the change local to handler normalization.

### Verification

- Focused Promise regression:
  `cargo test --release -p rusty-js-runtime --test promise_golden`: PASS
  (`8 passed`).
- CLI build:
  `cargo build --release --bin cruft -p cruftless`: PASS.
- Reduced CLI smoke:
  `Promise.resolve(1).then(v => console.log('then', v)).catch(...)`: PASS,
  no unhandled rejection.
- Redis exact smoke: PASS. The script prints `OK 58` and the first eight Redis
  exports, then exits without the prior terminal unhandled rejection.

### Residual

`redis.createClient().connect().catch(...)` now reaches Redis socket retry code
and reports a distinct runtime gap:

```text
connect-error callee is not callable: undefined [argc=1] (callee='<scoped@9>retryIn') ... socket.js:221:50
```

That path is downstream of the post-load fix and appears tied to the
`timers/promises.setTimeout(retryIn)` retry loop, not to the import/post-load
Promise.catch defect.

### Finding

**Finding MILF.6.1**: Promise instance combinators must use shared `[[Get]]`
semantics and callable-handler normalization. Ecosystem packages commonly end
dynamic imports with `.then(...).catch(...)`; if `.catch` bypasses the normal
property lookup path or `.then` treats `undefined` as callable, otherwise
successful package loads become terminal unhandled rejections.

## 2026-05-30 — MILF-EXT 7 Buffer numeric read/write integer/float family

Extends the MILF-EXT 3 Buffer-writer rung with the full numeric reader surface
plus the missing writer counterparts. Surfaced by R1's node:zlib cross-package
smoke (MILF-EXT 6): mongoose `createConnection` advanced through zlib into
`Buffer.readUInt32BE` undefined.

### Substrate

`cruftless/src/node_stubs.rs` — added under `install_buffer_methods` between
the existing `readUInt8` and `indexOf`:

- **Unsigned readers**: `readUInt16{LE,BE}`, `readUInt32{LE,BE}`.
- **Signed readers**: `readInt8`, `readInt16{LE,BE}`, `readInt32{LE,BE}`.
- **Float/double readers**: `readFloat{LE,BE}`, `readDouble{LE,BE}`.
- **BigInt readers**: `readBigUInt64{LE,BE}`, `readBigInt64{LE,BE}` —
  return `Value::BigInt` via `JsBigInt::from_u64`/`from_i64`.
- **Writers** (previously absent): `writeInt8`, `writeInt16{LE,BE}`,
  `writeInt32LE` (BE already present), `writeFloat{LE,BE}`,
  `writeDouble{LE,BE}`, `writeBigUInt64{LE,BE}`, `writeBigInt64{LE,BE}`.

Implementation uses two `reg_read!` / `reg_write!` declarative macros over
shared `buf_read_bytes` / `buf_write_bytes` helpers; bounds-failure paths
return `RangeError` whose message contains the literal `ERR_OUT_OF_RANGE`
so consumer code that pattern-matches on the Node error code sees the
expected shape. Negative-offset arguments also return `ERR_OUT_OF_RANGE`
per Node semantics.

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- `cargo test --release -p cruftless --lib`: 11 passed.
- Smoke per `/tmp/smoke/buf.mjs`:
  - `writeUInt32BE(0xdeadbeef,0)` + `readUInt32BE(0)` → `"deadbeef"`.
  - `UInt16LE` / `Int16BE` / `FloatLE` (3.14) / `DoubleBE` (2.718281828) all
    round-trip exactly.
  - `readUInt32BE(100)` on a 16-byte buffer throws with
    `message.includes('ERR_OUT_OF_RANGE')` truthy.
  - `writeBigUInt64BE(0xfedcba9876543210n, 0)` + `readBigUInt64BE(0)` round-
    trips to the same BigInt; `.toString(16)` prints `"fedcba9876543210"`.

### Lineage

Direct continuation of the R1 Option-C compounding result: zlib batch
(MILF-EXT 6) advanced mongoose through the import path into the Buffer
numeric residual; closing that residual is the same rung-shape as MILF-EXT 6
but at the byte-decoding coordinate rather than the compression coordinate.

## 2026-05-30 — MILF-EXT 7.1 Buffer-method install gaps closed (cross-package smoke triage)

Cross-package smoke against mongoose (per keeper directive #3 in Telegram 10526)
surfaced that the new MILF-EXT 7 numeric readers were unreachable via three
production buffer-creation paths that pre-existed in the codebase:

1. `cruftless/src/zlib.rs::buffer_from_bytes` — called `install_zlib_buffer_methods`
   (toString-only) without first installing the full Buffer prototype surface.
   Result: `gunzipSync(...).readUInt32BE(...)` and similar dead-ended.
2. `cruftless/src/node_stubs.rs::allocUnsafeSlow` — alloc-shape factory that
   didn't call `install_buffer_methods` at all.
3. `cruftless/src/node_stubs.rs::Buffer.concat` — concat result didn't call
   `install_buffer_methods`.

Fix: each site now calls `install_buffer_methods` (exposed `pub(crate)`).
zlib still layers its `toString` override on top.

Mongoose smoke advanced: the readUInt32BE failure inside saslprep's
memory-code-points.js (called transitively from mongodb's SCRAM auth path)
no longer dead-ends. New residual surfaced one level deeper:
`mongoose/lib/cast/bigint.js:18:65` errors `Cannot mix BigInt and other types`
on the template-literal ERROR_MESSAGE that interpolates `${MIN_BIGINT}` /
`${MAX_BIGINT}`. That's a parser/runtime-tier gap (template literals should
call `ToString` on BigInt, not `ToNumber`), and is named here as a follow-up
rung, distinct from the Buffer surface coordinate.

### Verification
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- `cargo test --release -p cruftless --lib`: 11 passed.
- Isolated Buffer smoke (`/tmp/smoke/buf.mjs`) unchanged: all numeric
  round-trips PASS, OOB throws ERR_OUT_OF_RANGE.
- Mongoose smoke advances past Buffer surface into the bigint-template
  residual — confirms the install-gap fix unblocks the chain.

## 2026-05-30 — MILF-EXT 8 BigInt template-literal ToString coercion

Closes the residual MILF-EXT 7.1 surfaced in mongoose: `cast/bigint.js:18:65`
template-literal `${MIN_BIGINT}` / `${MAX_BIGINT}` interpolation throwing
`Cannot mix BigInt and other types`.

### Diagnosis

The bytecode compiler lowers template literals to a left-to-right `Op::Add`
chain seeded by the first quasi (a String constant). The comment in
`compile_template_literal` (compiler.rs:4833) correctly observes "op_add
coerces non-string operands when the LHS is a String, so explicit ToString
is unnecessary" — but `op_add_rt` in `interp.rs:1507` checked
`BigInt ^ other` BEFORE the String-concat fast path and rejected BigInt-in-
template-literal as if it were arithmetic `+`. The mix rule is correct for
numeric `+` (where the spec throws on heterogeneous BigInt mix) but wrong
for `+` with a String operand (where the spec applies `ToString` to both).

### Substrate

`pilots/rusty-js-runtime/derived/src/interp.rs::op_add_rt` — gate the
BigInt-mix rejection on `!either_string`. `abstract_ops::to_string` already
calls `JsBigInt::to_decimal` for the BigInt path, so the existing
String-concat fast path produces the correct result with no other changes.

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- `cargo test --release -p cruftless --lib`: 11 passed.
- `cargo test --release -p rusty-js-bytecode`: 61 passed.
- Smoke `/tmp/smoke/bigint_tpl.mjs`:
  - `` `range: ${MIN} to ${MAX}` `` → `"range: -9223372036854775808 to 9223372036854775807"`.
  - `"prefix" + 42n` → `"prefix42"`.
  - `42n + "suffix"` → `"42suffix"`.
  - `1n + 1` still throws `Cannot mix BigInt and other types` (numeric path
    unchanged).
- Mongoose smoke now fully PASSES:
  `{"hasSchema":true,"hasModel":true,"hasConnect":true,"hasMongo":true,"schemaHasPath":true}`.

### Compounding

Mongoose was the trigger; any package that does `${someBigInt}` interpolation
benefits. The fix is in the universal Add path so the gain ripples
implicitly to every BigInt-using package whose load path touched this
expression form. Cross-package re-smoke is a candidate follow-up rung.

## 2026-05-30 — MILF-EXT 8.1 allocUnsafe inline-override removal

Continuation audit (post MILF-EXT 7.1) of buffer-creation sites for missing
install_buffer_methods calls. All __is_buffer__ creation sites had install
in scope; one site instead had a worse problem: ordering.

### Diagnosis

`Buffer.allocUnsafe` (node_stubs.rs ~1325) registered methods in this order:
1. Inline `readUInt8`
2. `install_buffer_methods(rt, id)` (full prototype surface)
3. Inline `subarray`

The inline subarray ran AFTER install, overriding the install version. The
inline version did not set `__is_buffer__` on its output and did not call
install_buffer_methods — so `nanoid.allocUnsafe(N).subarray(0, n)` returned
an object that failed `Buffer.isBuffer` and lacked all numeric readers/writers.

### Substrate

Removed both inline registrations; install_buffer_methods is now the single
source of truth for Buffer.allocUnsafe outputs (matching the alloc /
allocUnsafeSlow / Buffer.from / Buffer.concat paths after MILF-EXT 7.1).

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- `cargo test --release -p cruftless --lib`: 11 passed.
- Smoke `/tmp/smoke/subarr.mjs`:
  - `Buffer.allocUnsafe(8).subarray(2, 6).length` → 4.
  - `s[0]` → 2, `s[3]` → 5 (byte-indexed access through subarray).
  - **`Buffer.isBuffer(s)` → true** (was false before this rung).
  - `s.readUInt16BE(0)` → 515 (= 0x0203, byte 2 + byte 3 BE-decoded).

### Compounding

nanoid is the named consumer per the comment; any package that does
`allocUnsafe(...).subarray(...).readXxx(...)` benefits. This is the third
buffer-shape closure rung in 24h (7 → 7.1 → 8.1); the cumulative effect
is that all five Buffer-construction paths (alloc, allocUnsafe,
allocUnsafeSlow, Buffer.from, Buffer.concat) and the subarray path all
produce uniformly-shaped Buffers with the full prototype surface.

## 2026-05-30 — MILF-EXT 9 MessageChannel + MessagePort global stubs

While MILF-EXT 7+7.1+8+8.1 sweep ran, audited current top-failure cluster
against the new binary. cheerio's residual was `ReferenceError: MessagePort
is not defined` in undici's `webidl.is.MessagePort = MakeTypeAssertion(MessagePort)`
at module-init. MessageChannel was also unregistered.

### Substrate

`pilots/rusty-js-runtime/derived/src/intrinsics.rs` — stubs added next to the
existing BroadcastChannel stub (which uses the same shape rationale per
Tier-Ω.5.tttttt):

- `MessagePort` constructor: returns an Object with `postMessage` /
  `close` / `start` / `addEventListener` no-op methods + `onmessage = null`.
- `MessageChannel` constructor: returns an Object with `port1` / `port2`
  each MessagePort-shaped.

Both expose a `prototype` with `constructor` backref so `class X extends
MessagePort {}` and `webidl.is.MessagePort = MakeTypeAssertion(MessagePort)`
both find a callable + a prototype slot.

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- `cargo test --release -p cruftless --lib`: 11 passed.
- Smoke `typeof MessagePort === 'function'` / `typeof MessageChannel === 'function'`
  / `new MessageChannel()` returns `{port1: obj, port2: obj}` / port methods
  callable.
- **cheerio loads**: `import('cheerio')` now produces an Object with 7 keys
  (was `ReferenceError: MessagePort is not defined`).

### Side audit

Re-ran 15 packages from the postround11 top-FAIL cluster against the new
binary (MILF-EXT 7+7.1+8+8.1 + this rung). PASS now: redis, svgo, rehype,
ramda, puppeteer-core, xlsx, cheerio. FAIL with NEW shapes (each is a
distinct follow-up coordinate):
- arktype: ParseError "'generic' is unresolvable" — parser tier
- stylelint: readFileSync url-as-path bug (fs.url-shim coordinate)
- sequelize: `toString is not defined` — same TDZ-shape, module-context
- slonik: `callee is not callable [argc=1] (callee='fn')` — CJS-export propagation
- csv-parser: `value is not iterable (in-call='<destr.src>')` — destructure-of-non-iterable
- mnemonist: missing intrinsic `MinFibonacciHeap`

The cumulative install-gap-and-friends substrate (7+7.1+8+8.1+9) has
material cross-package PASS gain pending the in-flight sweep's quantification.

## 2026-05-30 — MILF-EXT 10 fs.* URL-object path coercion

stylelint failed module-load on `readFileSync(new URL('../../package.json',
import.meta.url), 'utf8')` (FileCache.mjs:19): the runtime's `arg_string`
helper calls `to_string` on a WHATWG URL Object, which yields
`"[object Object]"`, then `std::fs::read("[object Object]")` ENOENTs.
Node accepts `URL | string | Buffer` for the path argument across the entire
fs surface; cruft was string-only.

### Substrate

`cruftless/src/fs.rs` — added `arg_path_or_url(rt, args, i)` helper that
recognizes the URL shape via the `href` slot starting with `file://` and
strips the prefix, falling back to `arg_string` otherwise. Substituted into
all 33 sites where `let path = arg_string(args, 0);` was used in fs methods.
Five closures previously declared `|_rt, args|`; renamed to `|rt, args|`
to hand the runtime handle into the helper.

The helper avoids URL.pathname so the result is path-correct under
`file://host/p` shapes too (consumers in scope are POSIX so this is moot,
but is the safer default).

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- `cargo test --release -p cruftless --lib`: 11 passed.
- Smoke `/tmp/smoke/fs_url.mjs`: `readFileSync(new URL('./fs_url.mjs', import.meta.url), 'utf8')`
  returns 545 bytes (was ENOENT); pathname-string path still works (length
  parity).
- **stylelint loads**: 2 keys (was `readFileSync ... (in-call='url')`).

### Compounding

Any package that does `readFileSync(new URL('...', import.meta.url))` — a
very common pattern in ESM packages reading bundled data files — benefits.
URL-as-path is also accepted by other fs surface methods (statSync,
existsSync, readdirSync, mkdirSync, …), all 33 of which now share the
same helper. Cumulative cluster-A gain pending re-sweep.

## 2026-05-30 — MILF-EXT 11 Buffer @@iterator returns real Iterator object

csv-parser failed on `const [cr] = Buffer.from('\r')` (index.js:3). Two-stage
diagnosis:
1. Buffer instances had no `@@iterator` slot → destructure dead-ended on
   "value is not iterable (no @@iterator)".
2. Aliasing `@@iterator` to the pre-existing `values()` (which returns a
   bare Array, not an Iterator) hit the next protocol step: the destructure
   protocol's `__destr_iter_step` calls `.next()` on whatever `@@iterator`
   returned, and Arrays don't have `.next()`.

### Substrate

Real Iterator factory registered on each Buffer instance in
`install_buffer_methods`. The returned iterator carries closure state on
itself via internal slots (`__i` cursor, `__src` source-buffer Object,
`__len` cached length) and exposes a `.next()` method that returns
`{value, done}` per the iterator protocol. Source is held via
`Value::Object(ObjectRef)` directly (no need for the gc crate dep — Object
values ARE ObjectRefs wrapped).

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- `cargo test --release -p cruftless --lib`: 11 passed.
- Smoke `/tmp/smoke/bufiter.mjs`:
  - `typeof Buffer.from('\r')[Symbol.iterator]` → `"function"` (was undefined).
  - `const [cr] = Buffer.from('\r')` → `cr` is 13 (carriage-return byte).
- **csv-parser loads**: 3 keys (was `value is not iterable`).

### Named follow-up

`Buffer.from([10, 20, 30])` returns a buffer with length=0 (indexed slots
not populated). The current Buffer.from-array path is broken — separate
locale from this rung's iterator surface. csv-parser uses
`Buffer.from(string)` which works, so this rung closes the immediate
blocker; Buffer.from-array deserves its own rung.

## 2026-05-30 — MILF-EXT 11.1 Buffer.from(array | Uint8Array | Buffer)

Closes the named follow-up from MILF-EXT 11. Buffer.from only handled the
String input shape; all object-shaped inputs (Array, Uint8Array, Buffer)
fell through to an empty byte vector, producing a length=0 Buffer.

### Substrate

`cruftless/src/node_stubs.rs::Buffer.from` — added an Object branch that
reads `length` + indexed slots from the source. The uniform property-bag
storage means the same path works for Array, Uint8Array, Buffer, and
indexed views; no per-shape dispatch needed.

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed.
- `cargo test --release -p cruftless --lib`: 11 passed.
- Smoke `Buffer.from([10,20,30])` → length 3, `[...]` → `[10,20,30]`.
- `Buffer.from(new Uint8Array([7,8,9]))` → `[7,8,9]`.
- `Buffer.from(Buffer.from([1,2,3]))` (round-trip) → `[1,2,3]`.
- `Buffer.from('abc')` (string path) unchanged → `[97,98,99]`.
- Wider iteration smoke (from MILF-EXT 11) now reports `for-of: [10,20,30]`
  / `spread: [10,20,30]` / `[a, ...rest] = ...` works on array-input buffers
  (was empty before this rung).

## 2026-05-30 — MILF-EXT 12 self / window / navigator browser-alias globals

workerpool failed at `self is not defined` (environment.js). Survey of
unblocked-by-self alternatives showed it then immediately reads
`navigator.hardwareConcurrency` — a two-stage gap.

### Substrate

`pilots/rusty-js-runtime/derived/src/intrinsics.rs` — added next to the
existing `globalThis` + `global` defines:

- `self` = `globalThis` (HTML5 WorkerGlobalScope alias; used by isomorphic
  npm packages that target both Node and browser/worker contexts).
- `window` = `globalThis` (browser-target compatibility shim).
- `navigator` = a stub object with `hardwareConcurrency: 1` (engine is
  single-threaded) and `userAgent: "cruft/0.1 (node-compat)"` (feature-
  detection paths read it).

Same `{w:t, e:f, c:t}` descriptor as `global`/`globalThis` so consumer
code that reassigns (e.g. for testing) isn't blocked.

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib`: 74 passed, 1 ignored.
- `cargo test --release -p cruftless --lib`: 11 passed.
- Smoke: `typeof self === 'object'`, `self === globalThis`, `window === globalThis`,
  `navigator.hardwareConcurrency === 1`, `navigator.userAgent` string set.
- **workerpool loads**: 10 keys (was `self is not defined`).

### Compounding scope

Any browser-isomorphic package that uses `typeof self !== 'undefined'`
or reads `navigator.userAgent` for environment detection benefits.
Common across React-companion libs, web-worker pool managers, and any
package built with `--target=neutral` bundlers.
