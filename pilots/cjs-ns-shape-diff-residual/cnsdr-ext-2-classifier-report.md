# CNSDR-EXT 2 - Null Namespace Classifier Report

## Scope

Helmsman directive `ecbadd16-e72b-4a11-9e7e-ca2186ef42d8` requested a classifier-only rung for the sixteen CNSDR rows whose prior cluster shape had `rb_kc: null` while Bun exposed a populated namespace.

No runtime substrate files were edited. Measurement used a fresh isolated Bun install sandbox at:

`/home/jaredef/Developer/cruftless-r3-sidecar/results/cnsdr-ext2-classifier/`

Primary artifact:

- `parity.json`: 16-package Bun versus cruft load-and-shape sweep with the current `target/release/cruft`.

Supplemental artifact:

- `diagnostics/*.diag.txt`: per-package Bun resolved URL, Node `require.resolve`, package entry metadata, cruft exit code, stdout, and stderr with stderr unsuppressed.

The directive package name `testing-library` is normalized here to `@testing-library/dom`, matching the package resolved by the Phase 2 row and the current Bun import surface.

## Aggregate Classification

| Class | Count | Packages |
|---|---:|---|
| Current pass / stale prior null row | 3 | `elliptic`, `secp256k1`, `keycloak-connect` |
| Load/eval noncompletion by timeout or process abort | 8 | `prettier-plugin-organize-imports`, `ethereumjs-util`, `cz-customizable`, `ethereumjs-tx`, `ethereumjs-wallet`, `typescript`, `core-js`, `sass` |
| Explicit eval error | 2 | `playwright-core`, `@testing-library/dom` |
| Native addon load failure | 2 | `argon2`, `bcrypt` |
| Namespace population after successful eval | 1 | `ejs-render` |

## Per-Package Classifier

| Package | Bun resolved entry | Package entry kind | Bun namespace | Cruft result | Stage class | Recommended fix shape |
|---|---|---|---:|---|---|---|
| `prettier-plugin-organize-imports` | `index.js` | CJS main | 3 keys | cruft timeout, no stdout | eval noncompletion | Trace CJS execution loop and dependency import chain; do not touch namespace finalization until execution completes. |
| `elliptic` | `lib/elliptic.js` | CJS main | 8 keys | PASS in 30s sweep, same 8 keys | current pass | Treat prior null row as stale for this worktree; no CNSDR substrate work. |
| `secp256k1` | `index.js` | CJS main | 20 keys | PASS in 30s sweep, same 20 keys | current pass | Treat prior null row as stale for this worktree; no CNSDR substrate work. |
| `ethereumjs-util` | `dist/index.js` | CJS main | 82 keys | cruft timeout, no stdout | eval noncompletion | Instrument module-post-eval dependency chain; likely blocked by crypto dependency execution, not namespace projection. |
| `cz-customizable` | `index.js` | CJS main | 2 keys | process abort, stack overflow | eval noncompletion | Find recursive require or object inspection path causing Rust stack overflow during CJS evaluation. |
| `ethereumjs-tx` | `dist/index.js` | CJS main | 3 keys | cruft timeout, no stdout | eval noncompletion | Same family as `ethereumjs-util`; trace shared dependency completion before namespace policy. |
| `ethereumjs-wallet` | `dist/index.js` | CJS main | 3 keys | cruft timeout, no stdout | eval noncompletion | Same family as `ethereumjs-util`; trace shared dependency completion before namespace policy. |
| `playwright-core` | `index.mjs` under Bun import condition; Node require resolves `index.js` | conditional exports, ESM import path | 10 keys | ReferenceError in `lib/utilsBundle.js` around derived-constructor `this` access | eval error | Route to class/super derived-constructor semantics, not CJS namespace synthesis. |
| `@testing-library/dom` | `dist/index.js` | CJS main with ESM module field ignored for bare import | 79 keys | TypeError assigning to constant `_getRequireWildcardCache` | eval error | Route to transform/lowering or CJS wrapper binding mutability around helper declarations. |
| `keycloak-connect` | `keycloak.js` | CJS main | 4 keys | PASS in 30s sweep, same 4 keys | current pass | Treat prior null row as stale for this worktree; no CNSDR substrate work. |
| `typescript` | `lib/typescript.js` | CJS main | 2249 keys | cruft timeout, no stdout | eval noncompletion | Large single-file CJS execution completion issue; only measure namespace after eval completes. |
| `core-js` | `index.js` | CJS main | 162 keys | exit 70 in unsuppressed run, no stdout/stderr | eval noncompletion | Trace process-exit/error path during CJS evaluation; likely global polyfill side effects, not final namespace logic. |
| `sass` | `sass.node.mjs` under Bun import condition; Node require resolves `sass.node.js` | conditional exports, ESM import path | 41 keys | cruft timeout, no stdout | eval noncompletion | Trace conditional exports plus ESM/CJS branch separately; current classifier says execution does not complete. |
| `argon2` | `argon2.cjs` | CJS main, native addon dependency | 7 keys | dynamic loader error: missing `napi_create_function` | native addon load | Implement or stub Node-API native addon surface before namespace work can close this row. |
| `bcrypt` | `bcrypt.js` | CJS main, native addon dependency | 8 keys | dynamic loader error: missing `napi_module_register` | native addon load | Implement or stub Node-API native addon registration before namespace work can close this row. |
| `ejs-render` | `index.js` | CJS implicit main | 1 key | eval completed, namespace had zero keys | namespace population | Narrow CNSDR candidate: CJS namespace population should synthesize Bun's `default` for object export after successful evaluation. |

## Stage Notes

Resolution is not the primary blocker for this set. For every row, Bun and Node identify a concrete package entry. The only conditional-export divergences with immediate diagnostic value are `playwright-core` and `sass`, where Bun's import condition selects an ESM-facing entry. Their cruft failures still occur after a file is selected, so the classifier stage is eval, not resolution.

Parse and compile are also not the dominant class here. The explicit JSON probe body runs and catches dynamic import exceptions for `playwright-core` and `@testing-library/dom`, which means the wrapper/probe compiled and the package progressed into runtime evaluation before failing. The timeout and process-abort rows do not produce a parse or compile diagnostic; they fail to complete execution.

Only `ejs-render` reaches the exact null-namespace shape targeted by CNSDR Rung B after successful evaluation. It prints its package side-effect lines, returns status OK from the probe, and reports zero enumerable namespace keys while Bun exposes one `default` key.

## Recommended Follow-Up Rungs

1. **CNSDR Rung C, namespace-only**: target `ejs-render` as the positive fixture for successful-eval empty namespace population. Add negative guards from CNSDR Rung A empty-export exclusions before changing default synthesis.
2. **Eval completion cluster**: split the timeout/abort rows into dependency-chain probes. The Ethereum family should be handled together; `cz-customizable` should be isolated as a stack-overflow require/eval bug; `typescript` and `core-js` are large-file/global-side-effect execution cases.
3. **Runtime semantics cluster**: route `playwright-core` to derived-class/super semantics and `@testing-library/dom` to const-binding/helper mutation semantics.
4. **Native addon cluster**: route `argon2` and `bcrypt` to Node-API native addon loading. These should not be claimed as CJS namespace defects.

## Closure Claim

CNSDR-EXT 2 does not justify a broad null-namespace patch. It narrows the immediate namespace-population substrate target from sixteen rows to one row (`ejs-render`) on the current worktree. The other twelve failing rows require load/eval/native-addon substrate work before namespace finalization can be meaningfully tested, and three rows already pass in the current 30s parity sweep.
