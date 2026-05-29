# cjs-ns-shape-diff-residual - Design Probe

## 2026-05-29 - Missing Default / Null Namespace Probe

Directive: helmsman/request/cnsdr-design-probe-missing-default-r3. Scope is design only. No runtime substrate is landed in this rung.

## Empirical Slice

The CNSDR-EXT 0 inline table has 20 rows whose Bun namespace includes `default` and whose cruftless namespace omits it. They divide cleanly before source inspection:

| Subset | Count | Packages |
|---|---:|---|
| `rb_kc: null` plus missing `default` | 16 | `prettier-plugin-organize-imports`, `elliptic`, `secp256k1`, `ethereumjs-util`, `cz-customizable`, `ethereumjs-tx`, `ethereumjs-wallet`, `playwright-core`, `testing-library`, `keycloak-connect`, `typescript`, `core-js`, `sass`, `argon2`, `bcrypt`, `ejs-render` |
| `rb_kc: 0` plus missing `default` | 4 | `reflect-metadata`, `joi-extract-type`, `nx`, `express-async-errors` |
| `rb_kc > 0` plus missing `default` | 0 | none |

This matters because the four zero-key rows are plausible CJS namespace synthesis misses, while the 16 null-count rows are not. A null cruftless key count means the package did not produce a usable namespace observation in the parity row; a default-synthesis patch inside namespace population cannot recover packages that never reached that population path.

## Sampled Row Diagnosis

| Package | Bun KC | rb KC | Diagnosis | Mechanism |
|---|---:|---:|---|---|
| `reflect-metadata` | 1 | 0 | Bun exposes only `default`; cruftless exposes an empty namespace. This fits CJS side-effect module/default wrapper policy. | CJS-default-synthesis-gap |
| `joi-extract-type` | 1 | 0 | Same one-key shape as `reflect-metadata`: empty cruftless namespace, Bun default-only namespace. | CJS-default-synthesis-gap |
| `nx` | 1 | 0 | Same one-key shape. The earlier shebang work made some `nx` entrypoints parse, so this row should be treated as post-load namespace policy unless a package-specific repro proves otherwise. | CJS-default-synthesis-gap |
| `express-async-errors` | 1 | 0 | Side-effect patch package; Bun still gives a default namespace member while cruftless leaves the namespace empty. | CJS-default-synthesis-gap |
| `elliptic` | 8 | null | Bun exposes named crypto exports plus default; cruftless has no namespace observation. Default synthesis cannot explain the missing named exports. | package-load-failure-silent or module-resolution-difference |
| `typescript` | 2249 | null | Full namespace absence for a very large CJS entry. Missing 2249 keys means the first target is load/eval/resolve completion, not `default`. | package-load-failure-silent or module-resolution-difference |
| `core-js` | 163 | null | Bun exposes global/polyfill namespace keys plus default; cruftless null indicates no namespace shape reached comparison. | package-load-failure-silent |
| `bcrypt` | 8 | null | Native package surface; null shape likely follows native binding/load path limitations before namespace synthesis. | package-load-failure-silent |

The local package sandbox used for the top-500 parity run is not mounted in this Codex namespace, so this rung cannot trace package source for those imports directly. The inference above is grounded in the inline shape fields and in the current loader/namespace code paths.

## Code Path Cross-Reference

`cruftless/src/module_ns.rs` is not the primary target for these CJS rows. Its host-finalize hook explicitly delegates CJS-shimmed packages to `evaluate_cjs_module` / `populate_cjs_namespace_view`, and the visible default synthesis there applies to ESM namespace finalization only.

The relevant runtime path is `pilots/rusty-js-runtime/derived/src/module.rs`:

- Direct CJS default imports already return raw `module.exports` in the import-binding path.
- Namespace imports call `cjs_namespace_view_at`, which allocates a module namespace view and calls `populate_cjs_namespace_view_at`.
- Real CJS evaluation computes `exports_reassigned`, refreshes the placeholder namespace view in place, and records the final `module.exports`.
- `populate_cjs_namespace_view_at` copies object export keys, then synthesizes `default` only when `exports_reassigned || exports_has_user_keys || has_explicit_default`, except for transpiled-ESM explicit default preservation.

That policy intentionally excludes an unwritten initial `exports` object. It previously matched abortcontroller-polyfill / ts-toolbelt-style rows, but the CNSDR zero-key slice shows a second Bun shape: some side-effect or empty-object CJS packages still receive `default` in Bun's namespace.

## Discrimination

The missing-default family should not be implemented as one broad "always add default" patch:

1. **CJS default synthesis for empty object exports**: four rows are currently the strongest target. They have `rb_kc: 0`, Bun `keyCount: 1`, and the only missing key is `default`.
2. **Null namespace/load failures**: 16 rows need a load-completion probe first. Their missing key lists include substantial named surfaces, so a namespace default patch would still leave them divergent.
3. **ESM finalization**: no row in this missing-default slice points first to `HostFinalizeModuleNamespace`; the hook is relevant only if a package resolves as ESM or module-field ESM.
4. **Other package-resolution differences**: some null rows (`typescript`, `core-js`, `sass`, native bindings) may be resolver/package-loader issues rather than CJS namespace projection.

## Phase 4 Recommendation

### Rung A: CJS Empty-Exports Default Policy Probe

Design a narrow probe in `populate_cjs_namespace_view_at` for evaluated CJS object exports with zero user keys and no explicit `__esModule` signal. The probe should decide whether Bun's default-only behavior can be predicted by package/source shape without regressing known no-default empty-export rows.

Required fixtures:

- Positive: `reflect-metadata`, `joi-extract-type`, `nx`, `express-async-errors`.
- Negative: prior rows named in the existing comment, especially abortcontroller-polyfill and ts-toolbelt, because they justified the current "unwritten initial exports object has no default" exclusion.

Expected closure if accepted: 4/56 CNSDR rows become PASS (`reflect-metadata`, `joi-extract-type`, `nx`, `express-async-errors`). Rung count: one design+substrate rung, but only after direct package probes are available.

### Rung B: Null Namespace Load-Completion Probe

Before touching namespace synthesis for the 16 null rows, build a per-package trace that records:

- resolved URL and package condition path,
- detected module kind,
- `evaluate_cjs_module` entry/exit status,
- CJS wrapper parse/compile/eval error if any,
- post-eval `module_post_eval_trace`,
- final namespace key count if population happened.

Expected closure from this probe alone: no direct PASS flips. It should classify the 16 null rows into package-load-failure-silent, module-resolution-difference, native binding unsupported, and genuine namespace population misses. Rung count: one instrumentation/probe rung, then one or more substrate rungs depending on the dominant classified bucket.

### Rung C: ESM Finalize Guard Only If Repro Points There

Do not adjust `cruftless/src/module_ns.rs` for the zero-key CJS rows unless a traced package resolves through ESM finalization. If such a package appears, the design should be separate from CJS namespace policy because the current ESM hook already has package-shape gates and previous regressions around broad default synthesis.

## PASS Prediction

Conservative expected closure:

- Rung A: 4 rows, if direct package probes confirm Bun default-only behavior and no prior no-default negatives regress.
- Rung B follow-up: unknown until trace classification; maximum 16 rows but likely split across resolver, native, package-loader, and runtime-eval gaps.
- Combined immediate Phase 4 plan: two rungs before any broad closure claim.

Do not claim the 20-row missing-default family as a single PASS prediction. The actionable first closure is the four-row zero-key subfamily.
