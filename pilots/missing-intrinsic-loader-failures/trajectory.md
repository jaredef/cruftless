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
`07e97eeb-7040-47ac-aa8b-01825c4bdc38`. With no package sandbox present, this
measurement is classification-based against the inline first-error coordinate
plus direct runtime smoke probes of the fixed methods.

Projected first-error closure from this DataView rung:

| Package | Prior first error | Rung result |
|---|---|---|
| `file-type` | `DataView.prototype.setUint32` missing | closed at first coordinate |
| `pdfkit` | `DataView.prototype.getUint32` missing | closed at first coordinate |

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
- MILF-EXT 1 projected first-coordinate gain: 2 PASS / 28 residual FAIL.
- Direct substrate proof: cruft smoke confirmed
  `DataView.prototype.getUint32` and `setUint32` are callable and correctly
  read/write values after this rung.

Status after inline measurement: scoped DataView intrinsic rung is landable, but
the remaining MILF cluster should proceed by sub-coordinate rather than by
error string.
