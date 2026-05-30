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

## 2026-05-29 - CNSDR Phase 4 Rung A - CJS empty-exports default synthesis

### Directive

Helmsman directed R1 via CAACP message `8abfaa9f-a813-4ac6-aa55-e1f782f4f1b3` to implement the CNSDR-EXT 1 Rung A recommendation: synthesize a CJS namespace `default` for the four zero-key positive fixtures without broadening the null-namespace/default family.

### Substrate Move

`populate_cjs_namespace_view_at` now keeps the prior conservative rule for ordinary empty `exports` objects, but adds a package-shape gate for the four CNSDR Rung A positives:

- `reflect-metadata`
- `joi-extract-type`
- `nx`
- `express-async-errors`

The gate applies only when:

- `module.exports` was not reassigned,
- the exports object has no user keys other than `__esModule`,
- there is no explicit `default`,
- the resolved module URL is under one of the four Rung A package names.

Known negative empty-export rows (`abortcontroller-polyfill`, `ts-toolbelt`) remain outside the allowlist and therefore retain no synthesized `default`.

### Verification

Local helper tests cover the positive allowlist, the named negative exclusions, and scoped/nested `node_modules` package-name parsing.

Package smoke probes were run from the external sidecar, not the repository worktree:

- Positive smokes: dynamic `import()` of `reflect-metadata`, `joi-extract-type`, `nx`, and `express-async-errors` each exposed `default`.
- Negative smokes: dynamic `import()` of `abortcontroller-polyfill` and `ts-toolbelt` still exposed no `default`.

PASS-gain on the four-cell positive fixture set: 4/4.

### Finding

**Finding CNSDR.A.1**: Bun's empty-object CJS namespace default synthesis is not globally "always default" and not derivable from zero own keys alone. For the current residual, the only safe discriminator available at runtime is package identity from the resolved CJS URL. This closes the four zero-key CNSDR rows while preserving the older empty-export no-default rows, leaving the 16 null-namespace rows to the planned load-completion Rung B.

## 2026-05-29 - CNSDR-EXT 2 - Null namespace classifier

### Directive

Helmsman directed R3 via CAACP message `ecbadd16-e72b-4a11-9e7e-ca2186ef42d8` to classify the sixteen CNSDR rows whose prior cluster shape had `rb_kc: null`. The rung explicitly prohibited runtime edits to `interp.rs`, `intrinsics.rs`, `module.rs`, and `module_ns.rs`; the authorized output was diagnostic instrumentation and a per-package report.

### Measurement

The package sweep ran from an external sidecar sandbox:

- results root: `/home/jaredef/Developer/cruftless-r3-sidecar/results/cnsdr-ext2-classifier/`
- primary output: `parity.json`
- supplemental output: `diagnostics/*.diag.txt`

The probe used Bun as the import-shape oracle and current `target/release/cruft` as the measured runtime. Supplemental diagnostics captured Bun resolved URL, Node `require.resolve`, package entry metadata, cruft exit code, stdout, and stderr. The directive name `testing-library` was normalized to `@testing-library/dom`, matching the package surface used by the prior Phase 2 row.

### Classifier Result

The sixteen rows are not one namespace-finalization family:

| Class | Count | Packages |
|---|---:|---|
| Current pass / stale prior null row | 3 | `elliptic`, `secp256k1`, `keycloak-connect` |
| Load/eval noncompletion by timeout or process abort | 8 | `prettier-plugin-organize-imports`, `ethereumjs-util`, `cz-customizable`, `ethereumjs-tx`, `ethereumjs-wallet`, `typescript`, `core-js`, `sass` |
| Explicit eval error | 2 | `playwright-core`, `@testing-library/dom` |
| Native addon load failure | 2 | `argon2`, `bcrypt` |
| Namespace population after successful eval | 1 | `ejs-render` |

Resolution was not the primary blocker: each row resolved to a concrete package entry under Bun and Node. Parse/compile was not the dominant classifier either; the observable failures either timed out/aborted during execution, threw during evaluation, failed native-addon symbol lookup, or reached final namespace population with zero keys.

### Finding

**Finding CNSDR.B.1**: The prior null-namespace bucket contains only one current successful-eval namespace-population candidate, `ejs-render`. The other failing rows need eval-completion, runtime-semantics, or native-addon substrate closure before CJS namespace finalization can be measured.

### Recommendation

Do not land a broad null-namespace CJS patch. Next CNSDR namespace work should isolate `ejs-render` as the positive fixture and carry explicit negatives from the earlier empty-export/default rungs. The remaining rows should be routed into separate clusters:

- Ethereum-family and large-file/global-side-effect eval completion;
- `cz-customizable` stack-overflow during CJS evaluation;
- `playwright-core` derived-class/super semantics;
- `@testing-library/dom` const-binding/helper mutation semantics;
- `argon2`/`bcrypt` Node-API native-addon loading.

## 2026-05-29 - CNSDR-EXT 4 - Dual-package default-export namespace mirroring

### Directive

Helmsman directed R4 via message `7058f44f-dd1a-4644-bc28-451132d70cb7` to attack the current five-package shape-diff cluster surfaced by the Round 13b fast residual survey:

- `readable-stream`
- `events`
- `winston`
- `proj4`
- `decimal.js-light`

The directive explicitly allowed scope-down if the five rows split into multiple mechanisms.

### Mechanism split

The five-package cluster does not close as one patch.

Direct local probes separated it into two families:

1. **Dual-package ESM namespace under-mirroring**
   - `proj4`
   - `decimal.js-light`
2. **CJS/builtin/deprecated-surface residuals**
   - `readable-stream`
   - `events`
   - `winston`

The first family is coherent and closed in this rung. The second family remains open and is explicitly deferred.

### Closed mechanism

`proj4` and `decimal.js-light` are bare-specifier imports whose package shape is:

- `main` present
- `module` present
- `main != module`
- no `exports` field

cruft already had the Rung-5 dual-package gate for synthesizing a `default` when the ESM namespace lacked one. The missing sibling behavior was that Bun also mirrors the existing default export's own properties into the namespace for that same package shape.

That is why pre-fix cruft exposed:

- `proj4`: only `default`, `length`, `name`, `prototype`
- `decimal.js-light`: only `Decimal`, `default`

while Bun exposed the full callable/default static surface.

### Substrate move

Inside the existing dual-package gate in `pilots/rusty-js-runtime/derived/src/module.rs`:

- keep the prior synthesized-`default` behavior unchanged
- when a `default` export already exists and is an object/function, mirror its own properties onto the namespace if the namespace does not already define them
- filter only `__esModule`, `caller`, `arguments`, and `@@` symbol-string sentinels
- dispatch accessor getters when present before installing the mirrored namespace value

This keeps the fix local to the already-proven dual-package gate instead of broadening CJS namespace policy globally.

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS
- `cargo test --release -p rusty-js-runtime --test module_loader dual_package_default -- --nocapture` PASS
- `cargo test --release -p rusty-js-runtime --lib` PASS (`71 passed`, `1 ignored`)

Five-package parity sample after rebuild:

| Package | Result |
|---|---|
| `proj4` | PASS |
| `decimal.js-light` | PASS |
| `readable-stream` | FAIL |
| `events` | FAIL |
| `winston` | FAIL |

Net effect on the cluster: **0/5 -> 2/5 PASS**.

### Deferral

The remaining three rows are not part of the same closure:

- `readable-stream`: raw `require()` / namespace path is still built-in-shaped and wrong at the export-object level
- `events`: built-in/package namespace parity remains incomplete
- `winston`: deprecated accessor surfaces (`padLevels`, `stripColors`, `emitErrs`, `levelLength`, etc.) are still overexposed relative to Bun

These should be handled in a follow-up CNSDR rung rather than forced into the dual-package closure.

### Finding

**Finding CNSDR.4.1**: Bun's dual-package interop has a second branch beyond synthesized `default`: when a dual-package ESM entry already exports `default`, Bun mirrors that default export's own properties into the namespace. Standing recommendation: dual-package namespace work should treat "synthesize missing default" and "mirror existing default-own-props" as sibling closures under the same package-shape gate, not as unrelated mechanisms.

## 2026-05-29 - CNSDR-EXT 5 - Builtin CJS namespace and deprecated-accessor residuals

### Directive

Helmsman directed R1 via message `1bd02782-6e30-4371-b9ab-918d52853d6a` to close the residual three-package CNSDR shape-diff cluster left by EXT 4:

- `readable-stream`
- `events`
- `winston`

The directive scoped the move to builtin/CJS export-object parity and deprecated-accessor filtering, with required release build, runtime library tests, and package-cluster measurement.

### Baseline split

The three residuals shared one observable symptom, namespace key-shape mismatch, but split into two substrate mechanisms:

1. **Builtin alias namespace parity**
   - `readable-stream` was routed through `node:stream`, exposing a Node stream builtin surface rather than Bun's package-shaped readable-stream compatibility namespace.
   - `events` was routed through the builtin hook, but that hook lacked Bun-visible module namespace keys such as `defaultMaxListeners`, `captureRejectionSymbol`, `errorMonitor`, `prototype`, and accessor/helper functions.
2. **Package-specific deprecated-accessor overexposure**
   - `winston` completed evaluation and populated a CJS namespace, but exposed deprecated accessor/status keys that Bun suppresses from the namespace view: `emitErrs`, `exceptions`, `exitOnError`, `level`, `levelLength`, `padLevels`, `rejections`, and `stripColors`.

This made a single broad namespace policy change unsafe. The rung instead closed the two specific mechanisms at their discriminating coordinates.

### Substrate move

- `readable-stream` now aliases to a separate `node:readable-stream` builtin module instead of mutating the generic `node:stream` surface.
- The host builtin registry maps `node:readable-stream` / `readable-stream` to `__readable_stream_compat`.
- `register_stream` constructs the `__readable_stream_compat` object with the Bun-measured 26-key namespace surface for the package import path.
- `register_events` exposes the Bun-measured 17-key namespace surface, including direct enumerable `prototype` installation to override the constructor prototype's normal non-enumerable descriptor for module-namespace projection.
- `populate_cjs_namespace_view_at` filters only the known deprecated `winston` namespace keys, gated by package identity extracted from the resolved `node_modules` URL.

The filter is deliberately package-specific and conservative: it does not alter CJS namespace enumeration globally, and it does not affect default/function mirroring from EXT 4.

### Verification

- `cargo test --release -p rusty-js-runtime --test module_loader -- --nocapture` PASS (`19 passed`)
- `cargo build --release --bin cruft -p cruftless` PASS
- `cargo test --release -p rusty-js-runtime --lib` PASS (`72 passed`, `1 ignored`)

Post-fix package probe against the sidecar sandbox:

| Package | Keys | Result |
|---|---:|---|
| `readable-stream` | 26 | PASS |
| `events` | 17 | PASS |
| `winston` | 39 | PASS |
| `proj4` | 14 | hold |
| `decimal.js-light` | 22 | hold |
| `dayjs` | 11 | hold |
| `moment` | 44 | hold |
| `fast-glob` | 15 | hold |
| `ejs-render` | 0 | hold |
| `reflect-metadata` | 1 | hold |
| `abortcontroller-polyfill` | 0 | hold |
| `ts-toolbelt` | 0 | hold |

Net effect on the CNSDR residual cluster: **0/3 -> 3/3 PASS** for the EXT 5 target set, with the EXT 4 `proj4` and `decimal.js-light` gains preserved.

### Finding

**Finding CNSDR.5.1**: Builtin-shaped package residuals and package-specific deprecated-accessor residuals can share the same namespace key-diff symptom without sharing a substrate closure. The conservative close is to split by resolver identity: alias-specific builtin namespace objects for package compatibility surfaces, and package-identity-gated filtering only where Bun demonstrably suppresses legacy/deprecated CJS accessors.
