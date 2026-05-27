# Broad Engine Benchmark Apparatus

This document specifies the next form of `cross-runtime-bench`: a broad,
multi-engine benchmark that compares Cruft against Node, Bun, Deno, LLRT,
`cruftless-rquickjs`, and future engines without collapsing incompatible
questions into one number.

The benchmark must answer three separate questions:

1. Can the engine run the workload correctly?
2. How much process startup does the engine carry?
3. How fast is the engine once the workload is hot?

The current CRB answers part of question 1 and part of question 3 for a small
fixture set. The broad benchmark extends it into a matrix.

## I. Engine Set

Primary engines:

| Engine | Invocation | Class |
|---|---|---|
| Cruft | `$CRUFT_BIN <file>` | project engine |
| Node | `node <file>` | V8 / canonical Node surface |
| Bun | `bun <file>` | JavaScriptCore / Node-compatible runtime |
| Deno | `deno run <file>` | V8 / secure-by-default runtime |
| LLRT | `llrt <file>` | QuickJS-derived low-latency runtime |
| rquickjs host | `cruftless-rquickjs <file>` | legacy QuickJS host |

Secondary engines can be added by declaring:

- `name`
- `command template`
- supported module mode (`script`, `esm`, `cjs`)
- stdout contract
- known unsupported features

No benchmark result is valid unless the exact command, version string, fixture
path, run count, and host machine are recorded.

## II. Benchmark Lanes

### Lane A: Cold Start

Purpose: measure process launch + runtime initialization + minimal JS eval.

Fixture:

```js
globalThis.__esmResult = "ok";
```

Run count: `N=100` minimum.

Report:

- median
- mean
- p95
- p99
- min/max
- failures

Interpretation:

Cold start is its own lane. It must not be mixed with throughput. LLRT and
Cruft are expected to do well here because they carry less V8-class startup
machinery; Node and Deno are expected to carry a larger process/runtime cost.

### Lane B: Portable Core Throughput

Purpose: compare JavaScript engine execution without depending on host APIs.

Fixtures:

| Fixture | Surface | Why it exists |
|---|---|---|
| `arith_tight_loop` | integer arithmetic, loops, locals | JIT/hot-loop lower bound |
| `object_shape_churn` | object creation, property get/set, hidden-class pressure | object model + inline cache pressure |
| `array_callback_pipeline` | `map/filter/reduce`, callback dispatch | high-level array intrinsic path |
| `string_builder_scan` | concatenation, slicing, char access, regex-free scan | string representation and allocation |
| `regexp_scan` | literal regex over deterministic corpus | regexp engine |
| `json_parse_transform` | JSON parse/stringify + transform | realistic data pipeline |

Rules:

- no filesystem
- no network
- no timers
- deterministic stdout
- all engines must produce byte-identical output before speed is compared

Run count: `N=30` for reportable results; `N=10` for quick local checks.

### Lane C: Host Builtins And Web APIs

Purpose: separate engine speed from host-surface availability.

Fixtures:

| Fixture | Surface |
|---|---|
| `url_header_sweep` | `URL`, strings, request-like parsing |
| `text_encoding_batch` | `TextEncoder`, `TextDecoder`, typed arrays |
| `crypto_sha256_batch` | `crypto.subtle.digest` or runtime equivalent |
| `fetch_parse_local` | fetch-like response parse, ideally against a local in-process server |

Rules:

- if a runtime lacks the surface, mark `UNSUPPORTED`, not `FAIL`
- if a runtime has the surface but gives wrong output, mark `DIVERGE`
- if a runtime has the surface and output matches, measure speed

This lane is where LLRT, Deno, Bun, and Node will diverge most by platform
surface rather than language engine.

### Lane D: Node Compatibility

Purpose: measure compatibility with Node-shaped packages and APIs.

Fixtures:

| Fixture | Surface |
|---|---|
| `cjs_require_graph` | CommonJS loader, `require`, package lookup |
| `esm_import_graph` | ESM resolution, relative imports |
| `node_fs_batch` | `node:fs` read/stat/write tempdir workload |
| `node_path_url_mix` | `node:path`, file URL conversion |
| `npm_package_acorn_parse` | real package parse workload |
| `npm_package_lodash_mix` | broad JS library behavior without native deps |

Rules:

- this lane is not expected to be fair to LLRT or Deno by default
- unsupported Node APIs are classified as `UNSUPPORTED`
- dependency installation must be lockfile-pinned

### Lane E: Async And Job Queue

Purpose: measure Promise/job-queue behavior separately from raw computation.

Fixtures:

| Fixture | Surface |
|---|---|
| `promise_chain` | microtask chain depth |
| `promise_all_fanout` | promise aggregation |
| `async_function_loop` | `await` overhead in a deterministic loop |
| `timer_zero_batch` | `setTimeout(0)` scheduling, where supported |

Rules:

- use deterministic completion and stdout
- record if timers are absent, stubbed, or semantically different

### Lane F: Memory And GC Pressure

Purpose: detect allocator and GC behavior that throughput-only fixtures hide.

Fixtures:

| Fixture | Surface |
|---|---|
| `many_small_objects` | object allocation + short-lived garbage |
| `array_buffer_churn` | typed-array allocation and copy |
| `string_intern_pressure` | repeated strings and property keys |
| `retained_graph_walk` | retained object graph traversal |

Report:

- wall-clock
- peak RSS when available
- process exit status

Peak RSS is optional at first cut but should become canonical once the runner
uses `/usr/bin/time -v` or `/proc/<pid>/status` sampling.

### Lane G: Semantic Probe Slice

Purpose: keep speed honest by sampling behavior near the Test262 matrix.

Fixtures are not full Test262. They are small executable probes selected from
high-yield matrix coordinates:

- iterator abrupt completion
- destructuring binding edge cases
- class fields/private elements
- property descriptors
- `SameValue`/coercion corners
- regexp Unicode property escapes

Report only pass/fail and runtime. These probes prevent a runtime from looking
fast because the fixture accidentally avoids semantics Cruft is currently
missing.

## III. Result Taxonomy

Every fixture/runtime cell has one of these states:

| State | Meaning |
|---|---|
| `PASS` | output matches oracle and timing is valid |
| `DIVERGE` | runtime completed but stdout differs |
| `UNSUPPORTED` | required API or module mode is absent |
| `FAIL` | runtime errored unexpectedly |
| `TIMEOUT` | runtime exceeded the fixture timeout |
| `INVALID` | harness could not classify the result |

Only `PASS` cells enter speed ratios. `UNSUPPORTED` is a capability result,
not a speed result.

## IV. Measurement Protocol

Default protocol:

- sequential execution, never concurrent
- one uncounted warmup launch for cold-start lane only
- `N=30` for throughput lanes
- `N=100` for cold-start lane
- timeout per run defaults to `30s`
- stdout/stderr captured per run
- results written as JSONL
- markdown summary generated from JSONL

For each cell record:

```json
{
  "run_id": "broad-engine-YYYY-MM-DD-HHMMSS",
  "lane": "portable-core",
  "fixture": "json_parse_transform",
  "engine": "cruft",
  "command": ["/path/to/cruft", "/path/to/main.mjs"],
  "version": "cruft 0.1.0",
  "run": 1,
  "status": "PASS",
  "ms": 452.0,
  "stdout_sha256": "...",
  "stderr_sha256": "...",
  "reason": ""
}
```

## V. Reporting Shape

The top-level report has one table per lane.

For speed lanes:

| fixture | engine | status | median | p95 | ratio vs node | ratio vs bun |
|---|---|---:|---:|---:|---:|---:|

For compatibility lanes:

| fixture | cruft | node | bun | deno | llrt | rquickjs |
|---|---|---|---|---|---|---|

The report must also include:

- engine versions
- host CPU/kernel
- git commit
- fixture count
- invalid cell count
- fastest engine per fixture
- Cruft rank per fixture
- aggregate counts by state

No single aggregate score is canonical. If a summary number is needed, use
three separate numbers:

1. `compatibility coverage`: `PASS / total fixtures attempted`
2. `portable-core geomean`: geometric mean of median ratios for `PASS` cells
3. `cold-start median`: median launch time for Lane A

## VI. Apples-To-Apples Rules

1. Do not compare Lambda cold start to local CLI cold start.
2. Do not compare a runtime on a fixture it failed or skipped.
3. Do not treat missing host APIs as speed failures.
4. Do not let a fixture read network or ambient filesystem unless the lane
   explicitly requires it.
5. Do not bundle dependencies differently per engine unless the lane is testing
   bundling.
6. Do not optimize a fixture for one engine's idioms.
7. Do not collapse startup and hot throughput.

## VII. Immediate Implementation Plan

### BEB-EXT 1: Runner Generalization

Extend `run-bench.sh` or add `scripts/run-broad-bench.py`.

The Python runner is preferred because it can:

- declare engines as structured command templates
- support Deno's `deno run` command shape
- classify status from stdout/stderr/exit code
- compute p95/p99 directly
- write stable JSONL without shell quoting hazards

### BEB-EXT 2: Lane A Canonicalization

Move the existing cold-start harness into the CRB apparatus:

- `fixtures/cold_start_esm_result/main.mjs`
- output under `${CRUFTLESS_CROSS_RUNTIME_RESULTS_ROOT}/cold-start/<run-id>/`
- engines: node, bun, deno, llrt, cruft, rquickjs

### BEB-EXT 3: Portable Core Fixture Expansion

Add:

- `object_shape_churn`
- `array_callback_pipeline`
- `string_builder_scan`
- `regexp_scan`

Keep current:

- `arith_tight_loop`
- `json_parse_transform`
- `string_url_sweep`

### BEB-EXT 4: Capability Classification

Make `UNSUPPORTED` a first-class cell state. This is essential for LLRT,
Deno, and Cruft because host APIs differ.

### BEB-EXT 5: Node Compatibility Lane

Add pinned npm fixtures under a dependency-specific fixture dir. The fixture
must carry its own lockfile and setup notes. Do not make the broad runner run
`npm install` implicitly.

### BEB-EXT 6: Report Generator

Generate:

- `summary.md`
- `results.jsonl`
- `matrix.md`
- `engines.json`

`matrix.md` should be Pin-Art compatible: each failed cell gets a coordinate
like:

```text
engine/runtime-surface :: fixture/lane :: status :: reason
```

## VIII. First-Cut Fixture Matrix

The first reportable broad run should contain:

| Lane | Fixtures |
|---|---|
| Cold start | `cold_start_esm_result` |
| Portable core | `arith_tight_loop`, `json_parse_transform`, `object_shape_churn`, `array_callback_pipeline`, `string_builder_scan`, `regexp_scan` |
| Host builtins | `string_url_sweep`, `text_encoding_batch`, `crypto_sha256_batch` |
| Async | `promise_chain`, `promise_all_fanout`, `async_function_loop` |
| Memory | `many_small_objects`, `array_buffer_churn` |
| Semantic slice | `iterator_abrupt_completion`, `descriptor_shape`, `class_private_fields`, `regexp_unicode_property` |

This gives approximately 20 fixtures × 6 engines = 120 cells per run.

At `N=30` for throughput and `N=100` for cold start, the whole suite should be
small enough to run locally while broad enough to expose Cruft's position:

- startup advantage
- hot-loop competitiveness
- JSON/Array gap
- host API availability gaps
- async/job-queue residuals
- Node compatibility boundaries
- semantic correctness guardrails

## IX. Telos

The broad benchmark is not a leaderboard. It is a coordinate instrument.

Its job is to tell us where Cruft is strong, where it is absent, and where it
is semantically present but too slow. The expected output is a matrix that can
spawn substrate locales:

- `fast-json`
- `array-callback-inline`
- `tight-loop-emitter`
- `crypto-subtle-digest`
- `promise-job-queue`
- `node-loader-compat`
- `regexp-unicode-tables`

The benchmark succeeds when a future agent can read one row and know whether
the next move is a parser/runtime feature, a host API, a JIT/backend problem,
a GC/allocator problem, or a measurement artifact.
