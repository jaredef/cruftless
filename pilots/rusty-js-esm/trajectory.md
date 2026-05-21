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

## Rung-3 — Family B reading: enquirer over-synthesis (next)

cruftless reports keyCount=64, bun reports keyCount=43. +21 spurious keys. The mirror image of Round 3's blanket default-synthesis. Suspected root cause: `populate_cjs_namespace_view` (module.rs:1388–1552) mirroring CJS exports onto a namespace for a package bun treats as pure ESM, OR an over-eager `export *` expansion picking up internal names.

Apply the locale's standard reading rung first: bun + Node + cruftless on the enquirer entry, plus a `Object.keys(M).sort()` diff to identify which 21 names are spurious. If the names share a structural origin (e.g., all from a particular re-exported file), the move is targeted; otherwise read the bytecode-exports list to see what cruftless's compiler emitted.

## Rung-4 — Family C reading: superstruct re-export (queued)

The Family C (superstruct –1) move requires resolver-level reading of `export { x } from` chains.

Each Family will earn its own rung-N reading (no-commit) followed by a rung-(N+1) move (commit-with-sweep).

---

## Discipline checkpoints

After every closed rung, update both:
- the **Status** line in `seed.md` (`FOUNDED` → `RUNG-N CLOSED`)
- the rung's status line in this file (`queued` / `planned` → `closed`)

After every catastrophic-revert event (parity drop >2 percentage points), insert a postmortem rung documenting the over-application shape and update `seed.md` §I.2 with the new anti-pattern.
