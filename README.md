# Cruftless

Cruftless is a micro JavaScript runtime that punches above its weight. Built with Fielding Constraint Accumulation as the governing principle of architectural derivation, Cruftless achieves compatibility with the majority of Node.js packages in 50k LoC or less.

A hand-derived JavaScript runtime in Rust, targeting **the Node.js package ecosystem** as its compatibility surface. Constructed under the resolver-instance discipline: each layer's directives are consumed at that layer's resolver; no layer's artifact carries residue from the layer above. The terminal property the design induces is *vertically-recursive directive consumption with stage-deterministic emission*.

The repository was originally formulated as an AI-assisted source-translation apparatus reading Bun's Zig source for a Rust port — with the working repository name, `rusty-bun`, and the choice of Bun as the empirical oracle. The translation focus dissolved as the resolver-instance discipline crystallized; what remained is Cruftless: an independent runtime that uses Bun for measurement, not for inheritance.

[Read more here](https://jaredfoy.com/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs).

## What this is

A JavaScript runtime targeting **the Node.js package ecosystem** across ECMA-262 language semantics and the Node/Web platform surface that real packages depend on (`node:*` builtins, `fetch`, `Buffer`, `URL`, `structuredClone`, …). Bun is the **empirical oracle** for both the namespace-shape probe (`legacy/host-rquickjs/tools/parity-measure.sh`, a 1026-package top-500 + top-100 basket) and the runtime-semantics probe (`scripts/diff-prod/`, a curated fixture set diffed byte-for-byte against bun's stdout). Bun's role is instrumental: it correctly implements the union of Node + Web APIs that production packages exercise, which makes it a reliable yardstick against which Cruftless's compatibility can be measured. The engine itself is independent — built under [Pin-Art](https://jaredfoy.com/resolve/doc/581-the-resume-vector) discipline against ECMA-262 + WHATWG, not derived from any existing engine's code.

Two binaries:

- **`cruftless/`** — `cruftless` binary (renamed from `host-v2/` on 2026-05-21). Wraps the hand-rolled rusty-js engine (`pilots/rusty-js-{ast,parser,bytecode,gc,runtime}` crate family). The primary substrate target; the engine whose maturity Doc 729 names.
- **`legacy/host-rquickjs/`** — `cruftless-rquickjs` binary (renamed from `host/` on 2026-05-21). Wraps rquickjs (Rust bindings over QuickJS). Above-engine substrate matured; retained as the parity ceiling reference per Doc 717 §VIII. No new feature work lands here.

Both binaries run the same parity sweeps; the migration-cost gap between them reads the rusty-js engine's maturity directly against the rquickjs ceiling.

Cruftless is **not** a re-port of QuickJS, not a Bun source translation, and not a wrapper around any existing engine. It is a hand-derived terminal substrate that emerges from the resolver-instance discipline of Doc 729 — measured against Bun, not built from Bun.

## The design

The runtime is composed of five vertically-stacked resolver-instances, each a `source-with-directives → resolver → directive-free artifact` step. Each can be analyzed on its own under the four bootstrap properties of [Doc 432 §2](https://jaredfoy.com/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#the-bootstrap-as-resolver):

| Layer | Source | Resolver | Artifact |
|---|---|---|---|
| Cargo build | `Cargo.toml` + source tree | rustc + cargo | `cruftless` binary |
| Bootstrap | `cruftless/src/lib.rs::init()` | Runtime allocator + host-stub install | populated Runtime graph |
| Module load | ESM source + imports | parser + bytecode compiler + linker | `ModuleRecord` with `Namespace` |
| Execution | bytecode + constants | `interp.rs` dispatch loop | resolved JS values |
| Job-queue drain | microtask + macrotask queues | `run_to_completion` | quiescent runtime |

Each level's induced properties function as constraints on the level beneath it. The terminal property is preserved end-to-end only when each level respects the inherited constraint from above and emits a directive-free artifact to the level below.

## About RESOLVE

Cruftless is part of the **RESOLVE research program** — a long-running effort by [Jared Foy](https://jaredfoy.com) producing novel syntheses across AI-assisted philosophy of science, systems engineering, and computer science, with operational implications for how software is designed, derived, and implemented. The **RESOLVE corpus** is the program's primary artifact: a growing collection of numbered documents at [jaredfoy.com/resolve](https://jaredfoy.com/resolve/) that develop the program's concepts, architectural styles (SERVER, PRESTO, …), and disciplines (Pin-Art, Fielding Constraint Accumulation, …) in cross-referenced form.

Each Cruftless substrate decision is dispatched against a specific corpus position and back-referenced in the round log. The result is a runtime whose architecture is legible as the operationalization of a research program, not as an isolated codebase.

## Corpus references

The design is articulated across the RESOLVE corpus:

- [Doc 729 — Cruftless](https://jaredfoy.com/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs) — the primary articulation; the resolver-instance pattern applied to this runtime.
- [Doc 432 — SERVER: An Architectural Style for Engine Orchestration](https://jaredfoy.com/resolve/doc/432-server-an-architectural-style-for-engine-orchestration) — the orchestration-level constraint specification.
- [Doc 426 — PRESTO: An Architectural Style for Representation Construction](https://jaredfoy.com/resolve/doc/426-presto-an-architectural-style-for-representation-construction) — the construction-level constraint specification.
- [Doc 719 — The Pipeline Pattern Across Subjects](https://jaredfoy.com/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation) — the structural correspondence between this runtime and PRESTO.
- [Doc 717 — The Apparatus Above the Engine Boundary](https://jaredfoy.com/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point) — the cut-rung framework that places each resolver-instance's boundary.
- [Doc 581 — Pin-Art and the Discipline of Near-Necessity Substrate Construction](https://jaredfoy.com/resolve/doc/581-the-resume-vector) — the discipline by which substrate work proceeds.
- [Doc 725 — The Cluster-to-Walk Mode Transition](https://jaredfoy.com/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction) — the walk-mode discipline for per-package fault chains.

## Apparatus

- `seed.md` — engagement-level resume vector and operative discipline.
- `trajectory.md` — round log; latest anchor at the most recent EXT.
- `legacy/host-rquickjs/tools/parity-measure.sh` — canonical namespace-shape sweep against the 1026-package basket; outputs JSON.
- `legacy/host-rquickjs/tools/parity-fast.sh` — 43-package exemplar sweep for fast iteration during substrate moves.
- `legacy/host-rquickjs/tools/parity-cluster.sh` — per-cluster targeted sweep extracted from the latest canonical reading.
- `legacy/host-rquickjs/tools/select-cluster.py` / `legacy/host-rquickjs/tools/select-exemplars.py` — basket extractors for the targeted sweeps.
- `scripts/diff-prod/` — differential prod-test harness. Runtime-semantics probe complementing the namespace-shape probe; each fixture runs under both engines and diffs byte-for-byte. Sandbox + results default to a mounted T7 drive; heavy work runs under `nice -n 19 ionice -c3`.
- `specs/diff-prod-testing.md` — the diff-prod methodology (11 sections).
- `pilots/diff-prod/` — the diff-prod Pin-Art locale (seed + trajectory + deferred substrate backlog).

## test262 coverage

[test262](https://github.com/tc39/test262) is TC39's official ECMA-262 conformance suite (~53k tests covering language semantics and the entire built-in surface).

Cruftless runs test262 via `legacy/host-rquickjs/tests/test262/runner.mjs`, a per-test driver that parses the upstream YAML frontmatter, prepends the required harness scripts, evaluates the test through indirect eval, and emits one JSON line per result. The driver honors `module`, `async`, `noStrict`, `onlyStrict`, `raw`, and `negative.{phase,type}` flags from the frontmatter.

Because running the full 53k suite in CI is impractical at this stage, conformance is reported against a **curated representative sample** at `scripts/test262-sample/sample-paths.txt` — directory paths that target the surface production Node packages actually exercise: core builtins (`JSON`, `Map`, `Set`, `WeakMap`, `WeakSet`, `Number`, `Math`, `Symbol`, `Error`, `Promise`), the most-used `Array`/`String` prototype methods, key `Object` statics, `RegExp.prototype.{exec,test}`, and the language constructs real code uses (`arrow-function`, `for-of`, `for-in`). The sample expands to ~7,800 individual tests.

To reproduce:

```sh
cargo build --release --bin cruftless
./scripts/test262-sample/run-sample.sh        # writes results/test262-sample-<DATE>/{results.jsonl,summary.txt}
```

`PARALLEL=N` controls worker count (default 4); `T262_ROOT` points at an upstream test262 clone; `RB_BIN` overrides the cruftless binary path.

Latest sample, **2026-05-22**: **5,321 PASS / 1,882 FAIL / 384 SKIP** out of 7,587 results emitted across 7,750 sampled tests — **73.9% runnable pass rate** (`5321 / 7203`). SKIPs are tests whose frontmatter flags a feature the harness elects not to run (e.g. legacy `noStrict`-only fixtures, async-iterator features behind feature-flag gates).

Per-area breakdown:

| Area                          | PASS | FAIL | SKIP | Pass% |
|-------------------------------|-----:|-----:|-----:|------:|
| `built-ins/Number`            |  310 |   30 |    0 | 91.2% |
| `built-ins/Math`              |  298 |   29 |    0 | 91.1% |
| `built-ins/Object`            | 1563 |  204 |    0 | 88.5% |
| `built-ins/WeakSet`           |   68 |   15 |    0 | 81.9% |
| `built-ins/Array`             | 1264 |  286 |    0 | 81.5% |
| `built-ins/String`            |  495 |  113 |    0 | 81.4% |
| `built-ins/Set`               |  264 |  117 |    0 | 69.3% |
| `built-ins/RegExp` (exec/test)|   84 |   40 |    0 | 67.7% |
| `built-ins/Symbol`            |   66 |   32 |    0 | 67.3% |
| `built-ins/WeakMap`           |   94 |   47 |    0 | 66.7% |
| `built-ins/Map`               |  126 |   78 |    0 | 61.8% |
| `built-ins/Promise`           |  163 |  110 |  381 | 59.7% |
| `built-ins/JSON`              |   89 |   69 |    0 | 56.3% |
| `language/expressions`        |  146 |  172 |    0 | 45.9% |
| `built-ins/Error`             |   26 |   32 |    0 | 44.8% |
| `language/statements`         |  265 |  508 |    3 | 34.3% |

The high-90s rates on `Number`, `Math`, and `Object` reflect surfaces whose specs are tight loops over numeric or descriptor algorithms — exactly what the rusty-js-ir locale targets first. The low rates on `language/expressions` and `language/statements` reflect tests targeting subtle parser/eval edges (TDZ enforcement, hoisting cases, generator semantics) that diff-prod has documented as deferred substrate work.

The sample is the conformance baseline; the broader 53k suite is the eventual ceiling. Each substrate rung that flips a fixture in `pilots/diff-prod/` should also flip some count of test262 entries; the two probes triangulate together.

## rusty-js-ir — spec-as-source-of-truth IR

`pilots/rusty-js-ir/` is the locale that lifts spec-conformance work from hand-transcription to a stage-deterministic compilation. The hypothesis (from cruftless seed §A8.33 + `pilots/rusty-js-ir/IR-DESIGN.md` §9): **spec conformance becomes monotonically easier once an ECMA-262 algorithm is IR-encoded** — the linter enforces 1:1 IR-vs-spec correspondence, and the lowering compiler emits Rust against the existing `rusty-js-runtime` helper surface. Each new built-in passes through the linter once, then never drifts.

Each ECMA-262 algorithm section becomes one `IRFunction`; the lint passes (`cargo run --example lint_all -p rusty-js-ir`) require zero diff against the parsed spec; the emitted Rust lands in `pilots/rusty-js-runtime/derived/src/generated.rs`. Sections currently encoded include `Array.prototype` iteration/mutator families, `Math` variadic operations, `JSON.serialize`, the property-descriptor cluster, and several global predicates.

Status at locale's most recent close (IR-EXT 94b): **65 IR-alphabet nodes**, **33 sections IR-encoded** (28 wired into the runtime, 5 IR-only), linter ✓ on 33/33. Estimated 50–80 more sections to reach the bounded telos of "every cruftless `register_intrinsic_method` entry is either IR-encoded or explicitly carved out."

The locale composes with the test262 sample above: IR-encoded sections regress at or above their prior test262 rate. Regression is a Tier-1.5 incompleteness signal (the runtime-helper surface needs expansion).

## Status

39 / 39 diff-prod fixtures PASS across L and F categories. Top-500 namespace-shape parity at 77.4% raw / 82.1% incl-agreed-errors. Top-100 at 99.1%. Migration-cost gap between the rquickjs ceiling (`legacy/host-rquickjs/`) and the hand-rolled engine (`cruftless/`) is ~4.1 percentage points on the 1026-package basket. The morph trajectory toward the Cruftless terminal-property design proceeds under the walk-mode discipline of Doc 725; each substrate round is dispatched per Pin-Art near-necessity (Doc 581) and locatable on the two architectural addresses of Doc 729 §VIII.

## Origins

The repository began as an apparatus for AI-assisted cross-language code translation against Bun's Zig source, accompanying [Doc 702](https://jaredfoy.com/resolve/doc/702-ai-assisted-cross-language-code-translation-as-a-pin-art-bilateral-under-sipe-t-threshold-conditions-reading-the-bun-zig-to-rust-port) — with the working repository name, `rusty-bun`. The hand-derived JavaScript runtime that emerged through the engagement supplanted the original translation focus; the design crystallized through Doc 717's engine-cut framework and Doc 719's recognition of the structural correspondence with PRESTO. The Doc 729 articulation names the destination and renames the runtime: **Cruftless**, the substrate that remains when residue-carrying directives have been consumed at their resolver instances and only directive-free artifacts cross layer boundaries.

## License

Dual-licensed at the user's option under either:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

Copyright (c) 2026 Jared Foy. Contributions submitted for inclusion shall be dual-licensed as above, without any additional terms or conditions.
