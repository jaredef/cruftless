# diff-prod — differential production-test harness

See [`specs/diff-prod-testing.md`](../../specs/diff-prod-testing.md) for the methodology. This directory is the scaffolding.

## Layout

```
scripts/diff-prod/
  run.sh                — single-fixture runner
  run-all.sh            — iterate every fixture, emit aggregate
  runners/
    comparator.mjs      — per-category PASS/FAIL diff oracle
  fixtures/
    <name>/manifest.json
    <name>/exec.mjs
    <name>/setup.mjs       (optional)
    <name>/cassette.json   (optional, S-category)
  results/<name>/         (gitignored — re-emitted per run)
    bun.json
    cruftless.json
    result.json
```

## Quickstart

```sh
# Build cruftless first (if not already)
cargo build --release -p cruftless

# Run a single fixture
./scripts/diff-prod/run.sh string-ops

# Run every fixture
./scripts/diff-prod/run-all.sh
```

## Runtime hygiene

All heavy work runs behind `nice -n 19 ionice -c3` so the harness can
share a workstation without disrupting interactive use. The sandbox and
results default to the **T7 mounted drive** (`/media/jaredef/T7/rusty-bun/diff-prod-{sandbox,results}/`)
to keep system disk lean — same convention as the parity-measure harness.

Override via env if needed:

```sh
PROD_SANDBOX=/tmp/diff-prod-sandbox RESULTS_DIR=/tmp/diff-prod-results ./run-all.sh
```

If `ionice` isn't installed, the wrapper falls back to `nice -n 19` only
(visible at the top of `run.sh`).

## Shipped fixtures (rev 1)

| Fixture | Category | Status (2026-05-22) | Note |
|---|---|---|---|
| `json-roundtrip` | F | FAIL | Surfaces `JSON.stringify` replacer-honoring substrate gap |
| `buffer-encode` | F | FAIL | Surfaces transitive divergence from above |
| `string-ops` | F | FAIL | Same root cause |

All three fixtures currently FAIL on a single shared substrate divergence: cruftless's `JSON.stringify` does not honor the replacer function's *return value* (returns the original object's serialization with original key order, not the replacer-returned reordered object).

Repro (~3 lines):

```js
JSON.stringify({b:1,a:2}, (k,v) => {
  if (v && typeof v==='object' && !Array.isArray(v)) {
    const o={}; for (const k of Object.keys(v).sort()) o[k]=v[k]; return o;
  }
  return v;
});
// bun:       {"a":2,"b":1}
// cruftless: {"b":1,"a":2}
```

This is exactly the kind of finding the harness is designed to catch — a real ECMA-262 §25.5.2 conformance gap that the load-and-shape parity probe misses entirely (every package PASSES at L-category since no namespace key is mutated). The bug is filed as a substrate gap for the rusty-js-runtime locale; not addressed in this rev because the methodology landing intentionally holds off on substrate moves.

## Adding a fixture

1. `mkdir scripts/diff-prod/fixtures/<name>`
2. Write `manifest.json`:

```json
{
  "name": "<name>",
  "categories": ["F"],
  "deps": ["some-npm-pkg"],
  "timeout-ms": 5000
}
```

3. Write `exec.mjs` — must emit one canonical JSON line to stdout. Use the helper:

```js
function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {};
      for (const k of Object.keys(v).sort()) out[k] = v[k];
      return out;
    }
    return v;
  });
}
console.log(canon(result));
```

4. `./scripts/diff-prod/run.sh <name>` to test.

## Categories (recap)

- **L** — load-and-shape (namespace key/typeof diff)
- **F** — pure-function output (canonical JSON stdout diff)
- **E** — error equivalence (constructor + message-prefix diff)
- **S** — side-effect trace (capability-passing runtime prereq; stub for now)

## Promotion gates (recap)

- Gate 1: L-category PASS — required
- Gate 2: F-category PASS for fixtures with `gate2: true` — default true
- Gate 3: no E-category regressions
- Gate 4: S-category diffs flagged for human review (non-blocking)

## Composition with existing infrastructure

- The existing `legacy/host-rquickjs/tools/parity-measure.sh` IS the L-category runner for the 119-package basket + top-500 list.
- The rusty-js-esm deviation-resolution pipeline (Doc 730 §XII–§XVII) handles per-fixture deviations.
- The capability-passing runtime (Pilot α / Doc 736) is the S-category prerequisite.
