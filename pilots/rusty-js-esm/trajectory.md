# rusty-js-esm — Trajectory

Pin-Art rung log for the ESM ↔ CJS namespace-synthesis locale. Each rung = one substrate move + one §XVI yield (probe flip) + one sweep-tier confirmation.

See [`seed.md`](./seed.md) for telos, anti-telos, apparatus, and ceiling.

---

## Rung-0 — Founding (2026-05-21)

**Status**: closed.

**Substrate**: locale created. `seed.md` + `trajectory.md` + `probes/` + `docs/` scaffold in place.

**Baseline parity**: 94.9% (113/119) measured against the rusty-js-runtime-backed `cruftless` binary, post-Round-3-revert. Six failures: arktype, superstruct, node-fetch, superagent, entities (timeout), enquirer.

**Notes carried forward from the parent audit**:
- Tier-Ω cutover is already in-flight; this locale operates on the **deployed** hand-rolled engine, not a future engine.
- The Round 3 incident (commit `1746bc72`, reverted as `25d4bd95`) is the canonical anti-pattern: a blanket synthesis branch dropped parity from 94.9% → 67.2%. The locale's anti-telos §I.2 absorbs the lesson.

---

## Rung-1 — Family A reading: node-fetch (queued)

**Move**: zero substrate change yet. Read pass first.

**Reading recorded** (from inspection at locale founding):

- Package: `node-fetch@3.3.2`, `type: module`, `main: ./src/index.js`, no `exports` field.
- Source `src/index.js` exports:
  - `export {FormData, Headers, Request, Response, FetchError, AbortError, isRedirect};`
  - `export {Blob, File, fileFromSync, fileFrom, blobFromSync, blobFrom};`
  - `export default async function fetch(url, options_) { ... }`
- Bun's `Object.keys(M).sort()`: 16 keys. Two are absent from the source's export statements:
  - `fetch` — named alias of the default function. Bun appears to expose the *name of the default-exported function declaration* as an additional named export. This is non-spec but plausible (the default binding carries the identifier `fetch`).
  - `FetchBaseError` — declared in `src/errors/base.js`, imported transitively but **not re-exported** from `src/index.js`. Bun's source for this name is currently unaccounted for. Candidates: (a) bun reads `@types/index.d.ts` to seed names, (b) bun walks the import graph and synthesizes names for classes that are publicly reachable via instanceof checks, (c) historical bun-specific decision.
- cruftless's `Object.keys(M).sort()`: 14 keys. Matches the source's export statements exactly. Spec-aligned.
- Function `.name` property: bun reports proper class names (`AbortError`, `Blob`, `FetchError`, …); cruftless reports `?` for class declarations re-exported by `export {Name}` form. **Separate substrate issue** — class-declaration `.name` inheritance through aggregating re-exports. Not blocking the keyCount probe but worth recording.

**Decision for Rung-2**: do not mirror `FetchBaseError` until bun's source for it is understood. Mirroring blindly risks a Round-3-shaped incident. The `fetch`-named-alias case is more tractable and may be the right first move under Family A.

**Sweep posture**: not run; reading-only rung. No commit.

---

## Rung-2 — Family A move: default-function named-alias (REJECTED)

**Status**: closed; move abandoned, no commit.

**Move attempted**: a gated synthesis pass between the ESM export-binding loop and the host-finalize-namespace hook in `module.rs` (around line 1075). Conditions: default value is a function with non-empty `.name`, name ≠ "default"/"anonymous", name not already a namespace key. Implementation built clean; sweep not run.

**Rejection cause — counter-probe**:

A minimal raw-ESM probe (`export default function myFunc() {}` plus one other named export) returned **the same 2-key namespace under both bun and cruftless**. Cross-checked against Node (`node main.mjs`) returning the same 14-key namespace as cruftless for node-fetch — bun's extra 2 keys are not present.

This means:
- The named-alias-of-default behavior is **not** a general bun ESM feature.
- node-fetch's extra `fetch` and `FetchBaseError` keys are bun-specific to that package, almost certainly served by bun's built-in **node-fetch compatibility shim**. Bun ships its own native `fetch` implementation and intercepts `import "node-fetch"` to expose a superset of the package's surface (defensible since bun's `fetch` is faster and more compatible than the npm package).

**Implication for the locale**: node-fetch is **out-of-locale**. The gap is not in cruftless's ESM namespace-synthesis substrate; it is in cruftless's compatibility-shim layer. That layer belongs in the host (`cruftless/src/node_stubs.rs` or a sibling shim module), not in `module.rs`. Re-categorize node-fetch as a `fetch-api` / `node-http` locale concern.

**Anti-telos win**: this is exactly what the locale's discipline §I.2 is for. The Round 3 incident taught us not to ship blanket synthesis without cross-probing; the same discipline caught a different over-application **before** the commit. The synthesis branch never landed; baseline remains 94.9%.

**Carries forward**:
- Family A re-scoped: the 3 known failures inside this locale are now **enquirer (+21 over-synthesis)** and **superstruct (–1 re-export)**, plus any others a focused sweep reveals. node-fetch is reassigned.
- The cross-engine counter-probe pattern (bun + Node + cruftless + minimal raw-ESM repro) becomes the locale's standard reading rung. Add to discipline.

---

## Rung-3 — Family B reading: enquirer over-synthesis (closed, deferred)

**Cross-engine probe**:
- Node: 1 key (`default`) — pure CJS, no ESM bridge.
- Bun: 43 keys.
- cruftless: 64 keys. Δ = +21 spurious.

**Reading**:
- enquirer is CJS: `main: index.js`, no `module`, no `type`, no `exports`. Both bun and cruftless route through CJS namespace mirroring (`populate_cjs_namespace_view`, module.rs:1388–1552).
- `module.exports = Enquirer` where `Enquirer extends EventEmitter`. Bun's own-property count of the constructor: 66. cruftless: 63. Engines see a near-identical object but differ by 3 own keys (unrelated; not in the spurious-21 set).
- The 23 spurious keys (cruftless minus bun) are all `enumerable: false` per `Object.getOwnPropertyDescriptor` (verified in both engines). They split into 20 PascalCase class refs (`AutoComplete`, `BasicAuth`, `Confirm`, `Editable`, …) and 3 utility names (`prompt`, `prompts`, `types`).
- Bun keeps 18 OTHER `enumerable: false` properties: the EventEmitter API surface (`addListener`, `emit`, `on`, `off`, `once`, `eventNames`, `getMaxListeners`, `listenerCount`, `listeners`, `prependListener`, `prependOnceListener`, `rawListeners`, `removeAllListeners`, `removeListener`, `setMaxListeners`) and the function intrinsics (`name`, `length`, `prototype`).

**Bun's filter rule — unresolved**: bun keeps SOME `enumerable: false` properties (EventEmitter/Function intrinsics) and drops OTHERS (`AutoComplete`, `prompts`, `types`). A pure enumerable-filter would drop both groups; a pure "keep all own" would keep both. Hypotheses considered and rejected:
1. *Walks prototype chain*: `addListener` is own on Enquirer (not on EventEmitter.prototype), so a walk-up scheme doesn't explain its inclusion.
2. *Whitelist of EventEmitter API names*: would mean bun has a hard-coded EventEmitter-method allow-list for `enumerable: false` CJS exports. Plausible but uncodified anywhere in bun's public surface; would be fragile to mirror.
3. *Discriminator on capitalization*: rejected because `prompt`/`prompts`/`types` (lowercase) are also dropped.

**Verdict**: bun's filter for this package is package-shape-dependent in a way the locale's reading cannot fully recover without source access to bun. Shipping a substrate move here risks a Round-3-shaped over-application. **Defer enquirer** until either (a) a smaller-shape reading reveals the rule, or (b) the locale completes Rungs 5–6 on simpler packages and the residual count is small enough to accept package-level allow-lists.

**No commit.**

---

## Rung-4 — Family C reading: superstruct re-export (closed, move designed)

**Cross-engine probe**:
- Bun: 52 keys.
- cruftless: 51 keys. Δ = –1: missing `default`.

**Reading**:
- superstruct package.json: `type: "module"`, `main: "./dist/index.cjs"`, `module: "./dist/index.mjs"`, no `exports` field. **Dual-package shape**.
- The ESM file `dist/index.mjs` has exactly one statement: `export { Struct, StructError, any, ... };` (51 named exports, no default).
- Bun's `M.default` is an object with own keys equal to the 51 named exports. Confirmed `import D from 'superstruct'` returns that same object. It is a synthesized default constructed from the namespace's own enumerable properties.

**Rule (proposed)**: when an ESM module finalizes with no `default` key on its namespace, AND the package.json has BOTH `main` and `module` fields (the dual-package shape), bun synthesizes a `default` whose value is a plain object snapshot of the namespace's named exports. The motivation is CJS↔ESM unification: the .cjs entry's `module.exports` is shape-identical to the synthesized default, so consumers writing `import D from 'pkg'` interop seamlessly across both forms.

**Conditional gate** (anti-telos §I.2 compliance):
- ESM module's namespace.default is Undefined after the export-binding loop.
- The package.json (resolved via walking up from the module URL) has BOTH `main` and `module` keys.
- The package.json has NO `exports` field (modern packages use `exports` to declare the dual-package shape explicitly; the heuristic doesn't apply).
- The synthesized `default` is a plain object holding `(name, value)` pairs from the namespace's current own-property table, excluding `default` itself.

**Edit surface**: `module.rs` between line 1075 (end of export-binding loop) and line 1077 (host-finalize-namespace hook). Add a package.json read helper if one doesn't already exist; cache by package-root URL to avoid re-reading per module.

**Probe flip target**: superstruct (–1 → 0; flips to PASS).

**Estimated parity delta**: +1 package at sweep tier (superstruct). Possible additional flips if other corpus packages share the dual-package-shape-without-default condition; possible regressions if the gate is too loose. Sweep is mandatory.

**Status**: design closed; implementation deferred to Rung-5.

---

## Rung-5 — Family C move: dual-package default synthesis (CLOSED, LANDED)

**Status**: closed; substrate committed; parity 94.9% → **95.7% (114/119)**, net +1 (superstruct flipped, no regressions).

**Implementation**: `module.rs` between the export-binding loop and the host-finalize-namespace hook. Walks up from the module URL (file:// only, capped at 32 parent steps) to the nearest `package.json`, reads it through the existing `read_package_json` cache, and synthesizes `default = <plain object holding namespace's own properties excluding default>` iff the namespace's default is currently `Undefined`.

**Final gate (4 conditions, all required)**:
1. `namespace.default` is `Undefined` after the export-binding loop.
2. `package.json` has both `main` and `module` fields.
3. `package.json` has NO `exports` field.
4. `pkg.main != pkg.module_field` — the two fields point to **different files** (genuine dual-package shape, not a same-file ESM-only package masquerading via the `module` field).

**Iteration cost**: first sweep with the 3-condition gate showed net-zero — superstruct flipped to PASS but lodash-es flipped to FAIL with `new WeakMap` callee-not-callable inside `_metaMap.js`. Diagnosis: lodash-es sets `main: lodash.js` and `module: lodash.js` (same file). The 3-condition gate matched, triggering default-synthesis on lodash-es's internal helper modules; the synthesized ordinary-object default replaced the real WeakMap-class default that the helper modules expected. Adding condition 4 (`main != module`) excluded lodash-es and closed the regression. Second sweep: net +1, no regressions.

**Anti-telos checkpoint**: probe-before-commit caught the over-application at sweep tier, exactly as §I.2 requires. The 4-condition gate is now the load-bearing artifact of this rung; future rungs in this Family should extend (not loosen) it.

**Counter-probe completeness**: superstruct (probe target) — PASS, keyCount 51 → 52, matches bun exactly. lodash-es (regression candidate) — keyCount 322, unchanged. No other corpus packages were affected.

---

## Rung-6 — Family B revisit (queued)

Return to enquirer with a smaller-shape reading: instrument cruftless to log which property descriptors it sees during `populate_cjs_namespace_view`, run on enquirer, and diff against bun's IR (if available). If a clear rule emerges, ship a gated move; otherwise close as out-of-locale.

Each Family will earn its own rung-N reading (no-commit) followed by a rung-(N+1) move (commit-with-sweep).

---

## Discipline checkpoints

After every closed rung, update both:
- the **Status** line in `seed.md` (`FOUNDED` → `RUNG-N CLOSED`)
- the rung's status line in this file (`queued` / `planned` → `closed`)

After every catastrophic-revert event (parity drop >2 percentage points), insert a postmortem rung documenting the over-application shape and update `seed.md` §I.2 with the new anti-pattern.
