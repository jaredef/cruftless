# cjs-ns-shape-diff-residual - Trajectory

## 2026-05-29 - CNSDR-EXT 0 - Phase 0 spawn + Phase 2 inline-data probe

### Directive

Helmsman directed R3 to probe the 56-row shape-diff-no-error cluster. Two file-path directives were blocked by filesystem namespace isolation; resend #2 inlined the source data directly. Scope is Phase 0 plus Phase 2 only; no substrate edits are authorized in this founding round.

### Phase 0

Locale founded at `pilots/cjs-ns-shape-diff-residual/`.

Rule 11 pre-spawn coverage:

- **A1 component A/B**: Bun namespace key set versus cruftless namespace key set for packages that import without a hard error in the refined sweep.
- **A2 op-set**: dynamic import, CJS module evaluation, `module.exports` to ESM namespace projection, built-in/shim namespace construction, function metadata inclusion/exclusion.
- **A3 value-domain**: CJS function exports, object exports, built-in shims, native/large package entrypoints, packages with missing `default`, packages with extra userland/static fields.
- **A4 locals-marshaling**: module namespace object population and `Object.keys` visibility.
- **A5 emission-shape**: runtime/module namespace finalization and host shim namespace policy, not parser/lowering.

### Phase 2 Source

Source: inline JSON in CAACP message `4a44dcc0-5e3a-4d3b-afdf-3f73d7a26ce1`, 56 rows with precomputed `extra_in_rb` and `missing_in_rb`.

Prior paths were unavailable in R3:

- `/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json`
- `/home/jaredef/Developer/cruftless-sidecar/parity-results/cluster-shape-diff-no-error.json`

### Segmentation

Row-level segmentation:

| Bucket | Count | Share | Shape |
|---|---:|---:|---|
| Any missing-in-rb keys | 35 | 62.5% | Bun exposes keys that cruftless omits. Includes full namespace absence (`rb_kc: null`), default export absence, function metadata/static fields, and built-in shim completeness gaps. |
| Missing-only | 31 | 55.4% | No extra cruftless keys; only omitted Bun keys. This is the strongest C4-positive family. |
| Extra-only | 14 | 25.0% | Cruftless exposes keys Bun strips or does not synthesize for that package. |
| Mixed extra + missing | 4 | 7.1% | Both overexposure and omissions in the same package. |
| No concrete key diff in inline row | 7 | 12.5% | Null-count or unchanged-shape rows included in the cluster artifact (`collections`, `ipc-bus`, `ava`, `jest-environment-node`, `parcel-bundler`, `testdouble`, `plotly.js-dist`). |

Recurring key patterns:

| Pattern | Count | Packages |
|---|---:|---|
| Missing `default` | 20 | `prettier-plugin-organize-imports`, `elliptic`, `secp256k1`, `ethereumjs-util`, `reflect-metadata`, `joi-extract-type`, `nx`, `cz-customizable`, `ethereumjs-tx`, `ethereumjs-wallet`, `playwright-core`, `testing-library`, `express-async-errors`, `keycloak-connect`, `typescript`, `core-js`, `sass`, `argon2`, `bcrypt`, `ejs-render` |
| `rb_kc: null` with missing keys | 16 | `prettier-plugin-organize-imports`, `elliptic`, `secp256k1`, `ethereumjs-util`, `cz-customizable`, `ethereumjs-tx`, `ethereumjs-wallet`, `playwright-core`, `testing-library`, `keycloak-connect`, `typescript`, `core-js`, `sass`, `argon2`, `bcrypt`, `ejs-render` |
| Missing function metadata (`length`/`name`/`prototype`) | 5 | `readable-stream`, `events`, `decimal.js-light`, `keycloak-connect`, `typescript` |
| Extra `default` | 4 | `later`, `xstate`, `shellwords`, `proxyquire` |
| Extra function metadata (`length`/`name`/`prototype`) | 4 | `@databases/sql`, `proxyquire`, `isomorphic-fetch`, `typed-array-buffer` |
| Extra ws extension internals (`PerMessageDeflate`/`extension`/`subprotocol`) | 2 | `ws`, `isomorphic-ws` |

C4 result:

- Broad family C4 passes for missing-in-rb namespace incompleteness: 35/56 (62.5%).
- A narrower subpattern, missing `default`, is 20/56 (35.7%) and does not pass C4 alone.
- Extra-in-rb leakage does not pass C4: 18/56 if extra-only + mixed are combined (32.1%).

### Sampled Key Diffs

Sampled packages across magnitude and mechanism range:

| Package | Bun KC | rb KC | Shape | Concrete key diff |
|---|---:|---:|---|---|
| `typescript` | 2249 | null | full namespace absence / massive missing-in-rb | Missing all 2249 Bun keys, including `ANONYMOUS`, `AccessFlags`, `AssignmentKind`, `CharacterCodes`, `Debug`, `Diagnostics`, and `version`. |
| `process-nextick-args` | 4 | 34 | mixed, large extra-in-rb | Extra process shim keys: `addListener`, `arch`, `argv`, `binding`, `cwd`, `emit`, `eventNames`, `execArgv`, `exit`, `getBuiltinModule`, `hrtime`, `nextTick`, stdio keys, `versions`, etc.; missing Bun's `_exiting`. |
| `should` | 27 | 4 | large missing-in-rb | Missing `Assertion`, `AssertionError`, `_prevShould`, `config`, `deepEqual`, `doesNotThrow`, `equal`, `exist`, `extend`, `fail`, `format`, `modules`, assertion aliases, `throws`, `use`. |
| `readable-stream` | 26 | 17 | mixed stream shim | Extra `EventEmitter`, `EventEmitterAsyncResource`, `getDefaultHighWaterMark`, `isWritable`, `setDefaultHighWaterMark`; missing `ReadableState`, `_fromList`, `_isUint8Array`, `_uint8ArrayToBuffer`, `addAbortSignal`, `compose`, `destroy`, `from`, `fromWeb`, `length`, `name`, `prototype`, `toWeb`, `wrap`. |
| `es-object-atoms` | 6 | 27 | large extra-in-rb | Extra Object constructor statics: `assign`, `create`, `defineProperties`, `entries`, `freeze`, `fromEntries`, `getOwnPropertyDescriptor(s)`, `getOwnPropertyNames`, `getOwnPropertySymbols`, `getPrototypeOf`, `is`, `keys`, `seal`, `setPrototypeOf`, `values`, etc. |
| `winston` | 39 | 47 | medium extra-in-rb | Extra mutable logger fields: `emitErrs`, `exceptions`, `exitOnError`, `level`, `levelLength`, `padLevels`, `rejections`, `stripColors`. |
| `node-fetch-native` | 9 | 16 | medium extra-in-rb | Extra fetch implementation exports: `AbortError`, `FetchError`, `blobFrom`, `blobFromSync`, `fileFrom`, `fileFromSync`, `isRedirect`. |
| `events` | 17 | 7 | built-in shim missing-in-rb | Missing `addAbortListener`, `captureRejectionSymbol`, `captureRejections`, `defaultMaxListeners`, `errorMonitor`, `getEventListeners`, `getMaxListeners`, `init`, `prototype`, `usingDomains`. |
| `xlsx` | 17 | 19 | small extra-in-rb | Extra `set_cptable`, `set_fs`. |
| `ws` | 11 | 9 | mixed WebSocket shim | Extra `PerMessageDeflate`, `extension`, `subprotocol`; missing constants `CLOSED`, `CLOSING`, `CONNECTING`, `OPEN`, and `Server`. |
| `abort-controller` | 4 | 3 | small missing-in-rb | Missing `__esModule`. |
| `prettier-plugin-organize-imports` | 3 | null | null rb namespace | Missing `default`, `options`, `parsers`. |

### Sanity Check Against Local Import

Attempted to import eight sampled packages with `./target/release/cruft` from this worktree:

- `readable-stream`: OK; keys matched the inline rb shape (`Duplex`, `EventEmitter`, `EventEmitterAsyncResource`, `PassThrough`, `Readable`, `Stream`, `Transform`, `Writable`, `default`, `finished`, `getDefaultHighWaterMark`, `isDisturbed`, `isErrored`, `isReadable`, `isWritable`, `pipeline`, `setDefaultHighWaterMark`).
- `events`: OK; keys matched the inline rb shape (`EventEmitter`, `EventEmitterAsyncResource`, `default`, `listenerCount`, `on`, `once`, `setMaxListeners`).
- `process-nextick-args`, `should`, `es-object-atoms`, `winston`, `xlsx`, `abort-controller`: not locally importable from this filesystem (`bare specifier ... not found`). The original parity sandbox is not mounted in this Codex namespace, so the inline JSON remains the empirical anchor for those rows.

### Phase 3 Recommendation

The C4-positive closure is broad missing-in-rb namespace incompleteness, but it contains at least two different substrate shapes. Do not treat all 35 missing rows as a single code patch.

Recommended Phase 3 split:

1. **Default/null namespace completion rung**: target missing `default` and `rb_kc: null` rows. Inspect CJS namespace finalization for cases where load/evaluation succeeds enough for Bun to expose a namespace but cruftless returns a null/empty namespace or omits `default`. Expected impact ceiling from inline data: up to 20 missing-default rows, with 16 null-rb rows needing package/load-specific discrimination.
2. **Built-in shim completeness rung**: target concrete built-in/shim missing keys (`events`, `readable-stream`, `ws`, `process-nextick-args`). This is not the same mechanism as package default synthesis; it is host shim namespace surface parity.
3. **Extra-in-rb filter rung** only after the missing family: extra exposure lacks C4 on this cluster (18/56 combined, 32.1%), but has obvious local patterns such as function metadata/static leakage and object/process shim overexposure.

Estimated closure: two to three substrate rungs. First rung should be a design probe against missing-default/null namespace rows before any broad namespace filter is attempted.

## 2026-05-29 - CNSDR-EXT 1 - Missing-default design probe

### Directive

Helmsman directed a design-only probe for the 20 missing-default rows surfaced in CNSDR-EXT 0, with explicit discrimination between null namespace/load failures and CJS default synthesis. No runtime substrate edit was authorized.

### Probe Result

The 20 missing-default rows split before implementation:

| Bucket | Count | Packages |
|---|---:|---|
| `rb_kc: null` plus missing `default` | 16 | `prettier-plugin-organize-imports`, `elliptic`, `secp256k1`, `ethereumjs-util`, `cz-customizable`, `ethereumjs-tx`, `ethereumjs-wallet`, `playwright-core`, `testing-library`, `keycloak-connect`, `typescript`, `core-js`, `sass`, `argon2`, `bcrypt`, `ejs-render` |
| `rb_kc: 0` plus missing `default` | 4 | `reflect-metadata`, `joi-extract-type`, `nx`, `express-async-errors` |
| `rb_kc > 0` plus missing `default` | 0 | none |

The zero-key subfamily is the CJS default-synthesis candidate. The null-count subfamily is not: those rows are missing substantial named namespace surfaces too, so a `default` patch cannot close them without first recovering load/eval/resolve completion.

### Code Path Reading

`cruftless/src/module_ns.rs` is an ESM namespace finalization hook and explicitly routes CJS-shimmed packages to runtime CJS namespace handling. The relevant implementation is `pilots/rusty-js-runtime/derived/src/module.rs`:

- direct CJS default import already returns raw `module.exports`;
- namespace import calls `cjs_namespace_view_at`;
- evaluated CJS modules refresh a placeholder via `populate_cjs_namespace_view_at`;
- default is synthesized only when `exports_reassigned || exports_has_user_keys || has_explicit_default`, except for transpiled-ESM explicit-default preservation.

That policy deliberately excludes unwritten initial `exports` objects. CNSDR-EXT 1 shows that exclusion is not complete enough to explain Bun's default-only behavior for four side-effect/empty-object packages, but broadening it blindly risks regressing the prior no-default empty-export cases recorded in the code comments.

### Recommendation

Authoritative design note: `pilots/cjs-ns-shape-diff-residual/design.md`.

Recommended Phase 4 order:

1. **CJS empty-exports default policy probe**: positive fixtures `reflect-metadata`, `joi-extract-type`, `nx`, `express-async-errors`; negative fixtures from the prior comment, especially abortcontroller-polyfill and ts-toolbelt. Expected CNSDR closure if accepted: 4/56.
2. **Null namespace load-completion probe**: trace resolved URL, package condition path, module kind, CJS wrapper parse/compile/eval status, `module_post_eval_trace`, and final namespace population for the 16 null rows. Expected immediate PASS flips: none; it is a classifier rung.
3. **ESM finalize only if traced**: do not modify `cruftless/src/module_ns.rs` for this family unless a package demonstrably reaches ESM finalization.

The missing-default family is therefore a two-rung plan, not a one-patch 20-row closure claim.
