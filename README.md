# Cruftless

Cruftless is a micro JavaScript runtime that punches above its weight. Built with Fielding Constraint Accumulation as the governing principle of architectural derivation, Cruftless achieves compatibility with the majority of Node.js packages in 50k LoC or less.

A hand-derived JavaScript runtime in Rust, targeting **the Node.js package ecosystem** as its compatibility surface. Constructed under the resolver-instance discipline: each layer's directives are consumed at that layer's resolver; no layer's artifact carries residue from the layer above. The terminal property the design induces is *vertically-recursive directive consumption with stage-deterministic emission*.

The repository was originally formulated as an AI-assisted source-translation apparatus reading Bun's Zig source for a Rust port — hence the working directory name `rusty-bun`, and the choice of Bun as the empirical oracle. The translation focus dissolved as the resolver-instance discipline crystallized; what remained is Cruftless: an independent runtime that uses Bun for measurement, not for inheritance.

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

## Status

39 / 39 diff-prod fixtures PASS across L and F categories. Top-500 namespace-shape parity at 77.4% raw / 82.1% incl-agreed-errors. Top-100 at 99.1%. Migration-cost gap between the rquickjs ceiling (`legacy/host-rquickjs/`) and the hand-rolled engine (`cruftless/`) is ~4.1 percentage points on the 1026-package basket. The morph trajectory toward the Cruftless terminal-property design proceeds under the walk-mode discipline of Doc 725; each substrate round is dispatched per Pin-Art near-necessity (Doc 581) and locatable on the two architectural addresses of Doc 729 §VIII.

## Origins

The repository began as an apparatus for AI-assisted cross-language code translation against Bun's Zig source, accompanying [Doc 702](https://jaredfoy.com/resolve/doc/702-ai-assisted-cross-language-code-translation-as-a-pin-art-bilateral-under-sipe-t-threshold-conditions-reading-the-bun-zig-to-rust-port) — hence the working directory name `rusty-bun`. The hand-derived JavaScript runtime that emerged through the engagement supplanted the original translation focus; the design crystallized through Doc 717's engine-cut framework and Doc 719's recognition of the structural correspondence with PRESTO. The Doc 729 articulation names the destination and renames the runtime: **Cruftless**, the substrate that remains when residue-carrying directives have been consumed at their resolver instances and only directive-free artifacts cross layer boundaries.

## License

Dual-licensed at the user's option under either:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

Copyright (c) 2026 Jared Foy. Contributions submitted for inclusion shall be dual-licensed as above, without any additional terms or conditions.
