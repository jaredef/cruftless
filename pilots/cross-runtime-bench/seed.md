# cross-runtime-bench — Resume Vector / Seed

**Locale tag**: `L.cross-runtime-bench` (top-level per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (CRB-EXT 0)**. Spawned per keeper directive 2026-05-23 07:34-local. Cross-runtime speed benchmark for cruft, bun, and node executing realistic JS workloads. Independent of diff-prod (which tests semantic parity); this pilot tests *speed* on workloads representative of what real Node-ecosystem code does.

**Workstream**: A set of small fixed JavaScript workloads that all three runtimes execute identically; a runner that measures wall-clock per workload per runtime; multi-run median; reports both human-readable and machine-readable. Cruft's per-workload position vs the canonical Node baseline + Bun's typical 2-4× speedup is the empirical anchor for cruft's competitive standing.

**Author**: 2026-05-23 session.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Composes with**:
- `pilots/diff-prod/` — sibling pilot that tests semantic parity across cruft + bun (the same fixture-set discipline applies; this pilot's fixtures are speed-focused, diff-prod's are correctness-focused).
- `pilots/rusty-js-jit/` (LeJIT) — cruft's perf claims are gated on the JIT's contribution; this pilot's measurements feed back into LeJIT seed §I.3's composition reading.
- `pilots/rusty-js-shapes/consumer-migration/` — recent CMig-EXT 15 fix means cross-runtime spread workloads now produce comparable output.
- [Doc 735 §X.h.c](../../../corpus-master/corpus/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — three-probe-levels discipline applies here: bench-probe is the wall-clock, consumer-route is "the fixture is realistic enough to represent a class of real workloads," fuzz isn't directly applicable (workloads are fixed).
- [Doc 737 §IV](../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md) — standalone locale (no parent under pilots/).
- [Doc 738 §II](../../../corpus-master/corpus/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md) — fixtures in `fixtures/`, runner in `scripts/`, results in `results/`. Per §II.e pillar-path encoding.

## I. Telos

**Empirical answer to**: how fast is cruft relative to bun and node on workloads representative of what real Node-ecosystem packages do?

Not "how fast is cruft on the cruft team's preferred microbenchmark" (that's bench_ic / bench_call_overhead). Not "how fast is cruft on test262 semantics" (that's test262-sample). The gap: a runner that exercises realistic Node-shaped work on all three runtimes identically.

### I.1 First-cut workloads (4 fixtures)

Per the keeper's selection from the scope discussion (2026-05-23):

1. **`json_parse_transform`** — Parse a representative JSON payload (config-file or API-response shape, ~50KB), perform a small transform (filter + map), JSON.stringify the result. Tests JSON.parse + Array methods + Object iteration. Runs everywhere; no spread (avoids any latent shape-bypass surprises).

2. **`acorn_parse`** — Parse a representative JS file using acorn (the canonical npm parser). Tests real-world parser hot loop: closures, recursion, property access on AST nodes, string handling. Runs everywhere via npm-installed acorn (already in `host/tests/fixtures/consumer-acorn-app/`).

3. **`string_url_sweep`** — URL/URI parsing + header normalization + regex sweep over a corpus of fake HTTP request lines. Tests `URL`, `string.split` / `.replace` / `.toLowerCase`, regex matching. Representative of what Node servers do per-request.

4. **`crypto_sha256_batch`** — Hash a batch of small payloads (e.g., 1000 × 200-byte strings) with SHA-256 via WebCrypto's `subtle.digest`. Tests the crypto subsystem cruft's web-crypto pilot exposes. Bun has native; Node has crypto module + WebCrypto; cruft has the rusty-js-runtime web-crypto implementation.

### I.2 Falsifiers

**Pred-crb.1**: cruft completes all 4 fixtures correctly (output bytes match node's output bytes, byte-for-byte). Falsifier: a fixture where cruft produces semantically-different output. If true, the fixture or the runtime has a correctness issue blocking the speed measurement.

**Pred-crb.2**: cruft is at most 10× slower than node on every fixture. Falsifier: a fixture where cruft is >10× slower. If true, surface as a structural-performance issue for the substrate workstream that owns the slow path.

**Pred-crb.3**: bun is faster than node on every fixture (the conventional industry result). Falsifier: a fixture where bun is slower than node. If true, either the workload doesn't exercise bun's strengths or the fixture has a measurement artifact.

**Pred-crb.4**: multi-run variance is bounded (95% of runs within ±10% of the median). Falsifier: a fixture whose runs span >20% variance. If true, the workload has a structural variance source (GC, JIT warmup, IO) the bench should isolate.

**Pred-crb.5**: cruft's relative position improves under JIT-eligible workloads vs JIT-ineligible (i.e., the JIT pilot has measurable cross-runtime effect). Falsifier: cruft's relative position to node is the same on JIT-eligible (acorn_parse, json_parse_transform body work) and JIT-ineligible (heavy regex, crypto) fixtures. If true, the JIT contribution to cruft's competitive position is below measurement-noise threshold on realistic workloads — meaningful signal for LeJIT seed §I.3's composition reading.

## II. Apparatus

The bench is **a runner script + fixture set + result-format spec**. It composes with:

- **Cruft binary**: `~/bin/cruft` (auto-copied from `target/release/cruft` per the CLAUDE.md convention).
- **Bun binary**: `~/.bun/bin/bun` (system-installed).
- **Node binary**: `/usr/bin/node` (system-installed).
- **Fixture format**: each fixture is a `.mjs` file in `fixtures/<name>/main.mjs` plus optional `fixtures/<name>/input.*` data files. The fixture is self-contained: reads its own data, prints final result to stdout, exits. The runner times the full process.
- **Runner**: `scripts/run-bench.sh` — iterates fixtures × runtimes × N runs, captures wall-clock per run, computes median, writes JSONL + markdown.
- **Result format**: `results/<date>/{summary.md, results.jsonl}` per Doc 735 §X.h.c reporting discipline.

Per Doc 738 §II.e: fixtures and runners live at `pilots/cross-runtime-bench/{fixtures,scripts,results}/`. Fixture filenames use snake_case (`json_parse_transform`); runner scripts use kebab-case per shell convention.

## III. Methodology

Per the LeJIT-tier methodology refined across the engagement:

1. **CRB-EXT 0** — Workstream founding (this seed + trajectory + dir scaffold).

2. **CRB-EXT 1** — Runner scaffold. Build `scripts/run-bench.sh` that:
   - Discovers fixtures under `fixtures/*/main.mjs`
   - Runs each fixture × {node, bun, cruft} × N (default N=5 runs)
   - Captures wall-clock per run via `/usr/bin/time -f %e` or `date +%s%N` diff
   - Computes median per (fixture, runtime); aggregates min, max, stddev for context
   - Writes JSONL to `results/<date>/results.jsonl`
   - Writes human-readable summary to `results/<date>/summary.md`
   - Optionally verifies stdout-bytes-match across the three runtimes (Pred-crb.1)
   - Output: a single working invocation produces a table for one fixture.

3. **CRB-EXT 2** — First fixture: `json_parse_transform`. Smallest viable shape; verify three-runtime correctness + measurement.

4. **CRB-EXT 3** — Second fixture: `acorn_parse`. Reuses `host/tests/fixtures/consumer-acorn-app/` package + main.mjs as a starting point.

5. **CRB-EXT 4** — Third fixture: `string_url_sweep`. Self-contained; no npm deps.

6. **CRB-EXT 5** — Fourth fixture: `crypto_sha256_batch`. Uses WebCrypto's `subtle.digest`.

7. **CRB-EXT 6** — Baseline measurement run. Run all four fixtures across all three runtimes with N=10. Write summary.md. This is the first cut's reportable result.

8. **CRB-EXT 7** — Variance characterization. Multi-run variance reading per Pred-crb.4.

9. **CRB-EXT 8** — Cross-pilot composition reading. Compare per-fixture results against LeJIT seed §I.3's composition predictions; surface findings for the LeJIT enhancements log if any fixture's reading is unanticipated.

## IV. Carve-outs and bounded scope

- **Wall-clock only** at first cut. No per-component instrumentation (Cranelift-internal timing, GC pauses, syscall traces). The benchmark measures end-to-end realistic-workload latency from the user's perspective.
- **Median of N runs** as the reported value; min/max/stddev as context. Single-run is not load-bearing.
- **Same-machine, same-shell** — all three runtimes run in the same shell session on the Pi, sequentially. No concurrent execution.
- **Cruft binary is `~/bin/cruft`** per the CLAUDE.md auto-copy convention. The Pi-stability fix (binary on local SD, not USB-mounted T7) applies.
- **Stdout-bytes-equality** is a Pred-crb.1 gate; bytes-different is a fixture-level FAIL.
- **No micro-optimization** of fixtures to favor any runtime. Fixtures are written in straightforward portable JS that any of the three runtimes can compile/interpret without bespoke tuning.
- **Skip fixtures cruft cannot run**. If a fixture exercises a surface cruft hasn't implemented, the fixture is marked SKIP for cruft (not FAIL). Pred-crb.1 only applies to fixtures cruft attempts.

## V. Standing artefacts

- `pilots/cross-runtime-bench/seed.md` (this file)
- `pilots/cross-runtime-bench/trajectory.md`
- `pilots/cross-runtime-bench/fixtures/<name>/main.mjs` + optional input data
- `pilots/cross-runtime-bench/scripts/run-bench.sh`
- `pilots/cross-runtime-bench/results/<date>/{summary.md, results.jsonl}` — per-baseline-run output

## VI. Resume protocol

Read this seed, then trajectory.md tail. The runner at `scripts/run-bench.sh` is the single entry point; invocation: `scripts/run-bench.sh [--fixtures <pattern>] [--runs N]`. Default N=5; `--runs 10` for baseline measurement runs.

## VII. Doc 738 §II conventions checklist

- Locale path: `pilots/cross-runtime-bench/` (§II.e top-level pillar; standalone pilot per seed §I).
- Fixture filenames: snake_case (`json_parse_transform`, `acorn_parse`, `string_url_sweep`, `crypto_sha256_batch`).
- Runner script: kebab-case shell convention (`run-bench.sh`).
- Result files: `summary.md` + `results.jsonl` per Doc 735 §X.h.c reporting discipline.
- No engine-internal sentinels; no `__` prefixes; no `_via` suffixes (this pilot has no Runtime-side Rust code; it's a JS-fixture + shell-runner pilot).
