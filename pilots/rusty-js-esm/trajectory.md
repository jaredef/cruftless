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

## Rung-2 — Family A move: default-function named-alias (planned)

**Move design** (not yet implemented):

When an ESM module's default export is a named function declaration (`export default function NAME() { ... }` or `export default async function NAME() { ... }`), expose `NAME` as an additional named export on the namespace bag, **only if** `NAME` is not already a key.

**Conditional gate** (anti-telos §I.2 compliance):
- The default export's RHS must be a `FunctionDeclaration` / `AsyncFunctionDeclaration` with a non-empty identifier (not an anonymous arrow or expression).
- The identifier must not collide with an existing namespace key.
- The package's `package.json` must not have an `exports` field (paranoia gate — modern packages with explicit `exports` are unlikely to need this).

**Edit surface**: `module.rs:1448–1552` (`populate_cjs_namespace_view` is the wrong site; the right site is the ESM export-binding emitter in `compiler.rs` or the namespace-finalization step in `evaluate_module`). Reading pass required to identify the precise insertion point.

**Probe flip target**: node-fetch (–2 → –1). The `FetchBaseError` case stays open.

**Estimated parity delta**: 0 packages flipped at sweep tier (node-fetch needs both names; landing one leaves it FAIL). Move is still worth doing if (a) the substrate is correct, (b) Rung-3 lands the `FetchBaseError` source-of-truth and flips node-fetch.

**Sweep posture**: full 119-package sweep mandatory before commit. The default-function-named-alias path could over-apply in the same shape Round 3 did. The conditional gate is the defense; the sweep is the verification.

---

## Rung-3+ — Family B (over-synthesis) and Family C (re-export) (queued)

Deferred until Rung-1 reading complete and Rung-2 closure decided. The Family B (enquirer +21) move is the inverse of Round 3: remove the over-synthesis branch that triggers for enquirer's package shape. The Family C (superstruct –1) move requires resolver-level reading of `export { x } from` chains.

Each Family will earn its own rung-N reading (no-commit) followed by a rung-(N+1) move (commit-with-sweep).

---

## Discipline checkpoints

After every closed rung, update both:
- the **Status** line in `seed.md` (`FOUNDED` → `RUNG-N CLOSED`)
- the rung's status line in this file (`queued` / `planned` → `closed`)

After every catastrophic-revert event (parity drop >2 percentage points), insert a postmortem rung documenting the over-application shape and update `seed.md` §I.2 with the new anti-pattern.
