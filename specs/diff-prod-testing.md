# Differential Prod-Testing Methodology

## I. What and why

**What**: Run a production-shaped workload through both engines (`bun` as reference; `cruftless` as the engine under test) and compare results. Decide whether cruftless can stand in for bun on that workload.

**Why**: The parity-measure harness (`legacy/host-rquickjs/tools/parity-measure.sh`) covers load-and-shape only — `import * as M from 'pkg'` followed by `Object.keys(M)` and `typeof` per key. That answers "can the package's namespace be reconstructed." It does not answer:
- Does each exported function, given representative input, produce the same output?
- Do error paths produce equivalent throws?
- Are async operations sequenced equivalently?
- Are the observable side effects (file writes, network calls, stdout) equivalent?

Prod testing extends the diff oracle from namespace-shape to execution semantics. The same Doc 730 §XVI bidirectional-engine-diff instrument applies; the input set widens and the comparator deepens.

## II. Categorical scope

Four categories of differential probe, in increasing depth of equivalence assertion. A workload's promotion gate selects which categories must pass.

### II.a Load-and-shape (L)

The existing parity-measure probe. Asserts: `Object.keys(import * as M from 'pkg')` and per-key `typeof`. Cheap; covers cuttover of the namespace surface. Already in production.

### II.b Pure-function output equivalence (F)

For each exported function with a deterministic shape, assert: `M.fn(...inputs)` produces output that round-trips through `JSON.stringify` to a byte-identical string under both engines, OR (for non-JSON-serializable returns like functions or Symbols) matches a structural comparator. Inputs come from per-fixture seed corpora.

Tolerances:
- Floating-point output: tolerate `Math.abs(a-b) < 1e-12 * Math.max(1, Math.abs(a))` for IEEE-754 difference.
- Order-independent collections (Sets, Map iteration order pre-ES2015): canonicalize before comparison.
- `Error.stack` strings: strip to the head line only; never compare full stack traces (engine-specific framing).

### II.c Error-equivalence (E)

Assert: when bun throws, cruftless throws an error of the *same constructor* (TypeError ≈ TypeError) with a *prefix-matched* message. Message-prefix is the discriminator because both engines' error messages diverge in suffix detail (line/column, fingerprint tags). Prefix length capped at the first 32 characters or up to the first `:` / `(`, whichever comes first.

Bracket: a fixture's expected-throw cases are explicit; an unexpected throw on either side is a fail.

### II.d Side-effect equivalence (S)

For workloads that touch the filesystem, stdout, or network, assert: the observable side-effect trace is equivalent. Wrap with a host-side recorder that captures `fs.writeFile`, `console.log`, `process.stdout.write`, and `fetch` calls; diff the traces. This requires the **capability-passing runtime** (Pilot α / Doc 736) to be active so the recorder can interpose deterministically; in `cruftless --audit` mode the trace is free to obtain.

Network nondeterminism: fixtures must either run against a recorded HTTP mock (per-fixture `cassette.json`) or be marked offline-only.

## III. Fixture shape

A **fixture** is a directory under `scripts/diff-prod/fixtures/<name>/` with:

```
fixtures/<name>/
  manifest.json     -- categories enabled, deps, env, tolerances
  setup.mjs         -- (optional) one-shot prep, runs once before exec
  exec.mjs          -- the workload; must produce deterministic output to stdout
  expected/         -- (optional) golden outputs from a prior run
  cassette.json     -- (optional) HTTP recording for S-category
```

`manifest.json`:

```json
{
  "name": "express-hello-world",
  "categories": ["L", "F", "E"],
  "deps": ["express"],
  "env": {},
  "tolerances": { "float": 1e-12, "stack-strip": true },
  "timeout-ms": 30000
}
```

The runner is responsible for installing `deps` into the fixture's isolated sandbox (mirrors parity-measure's `PARITY_SANDBOX` pattern), copying `exec.mjs` into the sandbox, then running it under both engines.

## IV. Runner shape

`scripts/diff-prod/run.sh <fixture-name>` does:

1. Resolve fixture dir.
2. Install deps into `$PROD_SANDBOX/<fixture-name>` (idempotent).
3. Copy `setup.mjs` + `exec.mjs` into sandbox.
4. Run setup once under both engines.
5. Run exec under bun → capture stdout/stderr/exit-code/wall-time → write `$RESULTS/<fixture>/bun.json`.
6. Run exec under cruftless → capture same → write `$RESULTS/<fixture>/cruftless.json`.
7. Run comparator per enabled category → emit per-category PASS/FAIL + per-divergence diff.
8. Aggregate into a per-fixture `result.json` and an engagement-wide `summary.json`.

`scripts/diff-prod/run-all.sh` iterates every fixture, parallelizes per CPU count, and emits a final summary.

## V. Comparator (per category)

### V.a L: namespace shape diff

Reuse `legacy/host-rquickjs/tools/parity-probe.mjs` shape. Output: PASS / FAIL with key set delta.

### V.b F: stdout-as-canonical-JSON

`exec.mjs` MUST emit deterministic output via a single `console.log(JSON.stringify(result, replacer))`. The replacer canonicalizes (sorted object keys, function bodies stripped, dates ISO-formatted). The comparator string-compares the two engines' single-line stdout.

For non-deterministic workloads (entropy, timestamps), the fixture's manifest names the keys to mask before comparison.

### V.c E: error shape diff

If both engines throw, compare `(error.name, error.message-prefix)`. If only one throws, FAIL. If neither throws when an expected-throw fixture is run, FAIL.

### V.d S: side-effect trace diff

Trace format: one JSON Lines entry per side effect: `{op, target, byteCount?, contentHash?, timestamp-relative}`. The comparator allows reordering within a configurable window (default: 0ms, i.e., strict) but flags non-equivalence in op count or operand shape.

## VI. Promotion gates

A substrate change that lands in cruftless must:

- **Gate 1**: pass all enabled L-category fixtures. Required.
- **Gate 2**: pass all enabled F-category fixtures whose manifest declares `gate2: true`. Default is `true` for fixtures shipped in this repo.
- **Gate 3**: produce no E-category regressions (a passing fixture cannot transition to fail).
- **Gate 4** (production deploy only): S-category trace diffs flagged for human review. S-failures do not block but emit a deployment warning.

Gates 1-3 run on every PR via the existing parity sweep machinery. Gate 4 runs as a manual or weekly job.

## VII. Anti-telos

These are the same anti-patterns as the rusty-js-esm locale, applied to test methodology:

- **No tolerance-creep**: a fixture that needs an expanded tolerance is a fixture whose workload is non-deterministic. Fix the workload; do not relax the tolerance.
- **No oracle-pollution**: bun is the oracle. Do not write expected-output files; recompute on every run from bun fresh.
- **No probe-skipping at promotion**: every promotion gate runs every fixture in its category. A gate that hides a single fixture is a regression-permitting gate.
- **No silent flake**: a fixture that fails intermittently is flagged for redesign within one engagement-day. It cannot be marked `flaky:true` and ignored.

## VIII. Composition with existing infrastructure

- The **parity-measure** harness becomes the L-category runner. Existing 119-package basket + top-500 list both qualify as L-fixtures.
- The **rusty-js-esm deviation-resolution pipeline** (Doc 730 §XII–§XVII) handles per-fixture deviations the same way it handles namespace-shape deviations: §XII capture → §XIII reduce → §XIV localize → §XV bracket → §XVI yield → §XVII iterate.
- The **rusty-js-caps capability-passing runtime** (Pilot α) is the S-category prerequisite. Audit mode (`cruftless --audit`) provides the trace surface.
- The **Pin-Art seed.md / trajectory.md** pair (Doc 581) applies per-fixture-cluster. A fixture cluster (e.g., "react-server-render" or "express-routes") gets its own locale once it exceeds a single fixture.

## IX. Promotion of a workload from F to prod

A workload exits the differential test surface and enters production-deploy candidacy when:

1. All four categories pass for thirty consecutive runs across the substrate's commit history.
2. The S-category trace shows no engine-specific side effects (no `cruftless`-only file writes, no missed `bun`-only operations).
3. The workload's runtime under cruftless is within 10× of bun's (the spec wallclock-divergence target; engagement-level performance work tightens later).

A workload that meets these three is marked `prod-eligible: true` in its manifest and routed to the engagement's deployment-candidate set.

## X. Initial scaffolding

Shipped with this methodology:

- `scripts/diff-prod/run.sh` — single-fixture runner.
- `scripts/diff-prod/runners/comparator.mjs` — JSON-canonical comparator.
- `scripts/diff-prod/fixtures/express-hello/` — F-category fixture for `express`.
- `scripts/diff-prod/fixtures/json-roundtrip/` — F-category fixture pure-stdlib.

The scaffolding is intentionally minimal — three fixtures, one runner. Expansion is per-cluster, driven by the engagement's next-substrate-priority list.

## XI. Successor questions

- **Q1**: When does a fixture's failure surface a substrate gap versus a fixture-internal nondeterminism? The L/F/E/S category split is one cut; a finer cut may be needed when F-failures cluster on a specific intrinsic (e.g., `Map` iteration order).
- **Q2**: How is the cassette mechanism for HTTP-mocking authored? A standalone tool that records once via bun and replays in both engines would extend the S-category reach without per-fixture handwriting.
- **Q3**: What is the engagement-level dashboard for fixture results? The parity-baselines JSON pattern extends naturally; a daily roll-up shipped to a corpus-tier visualization is the natural next instrument.

---

*Doc rev 1, 2026-05-22. Composes with Doc 581 (Pin-Art), Doc 730 §XVI (bidirectional engine-diff), Doc 736 (capability-passing runtime).*
