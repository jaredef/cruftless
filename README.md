# Cruftless

A hand-derived JavaScript runtime constructed under the resolver-instance discipline. Each layer's directives are consumed at that layer's resolver; no layer's artifact carries residue from the layer above. The terminal property the design induces is *vertically-recursive directive consumption with stage-deterministic emission*.

The corpus articulation is [Doc 729 — Cruftless](https://jaredfoy.com/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs).

## What this is

A JavaScript runtime, built against ECMA-262 + WHATWG, with two binaries:

- **`host/`** — `cruftless-rquickjs` binary. Wraps rquickjs (Rust bindings over QuickJS). Above-engine substrate matured; serves as the parity ceiling reference.
- **`host-v2/`** — `cruftless` binary. Wraps the hand-rolled rusty-js engine (`pilots/rusty-js-{ast,parser,bytecode,gc,runtime}` crate family). Active development focus per the engagement's §A8.23 directive.

Both binaries run the same parity sweep against a 1026-package basket; the migration-cost gap between them reads the engine maturity directly.

## The design

The runtime is composed of five vertically-stacked resolver-instances, each a `source-with-directives → resolver → directive-free artifact` step. Each can be analyzed on its own under the four bootstrap properties of [Doc 432 §2](https://jaredfoy.com/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#the-bootstrap-as-resolver):

| Layer | Source | Resolver | Artifact |
|---|---|---|---|
| Cargo build | `Cargo.toml` + source tree | rustc + cargo | `cruftless` binary |
| Bootstrap | `host-v2/src/lib.rs::init()` | Runtime allocator + host-stub install | populated Runtime graph |
| Module load | ESM source + imports | parser + bytecode compiler + linker | `ModuleRecord` with `Namespace` |
| Execution | bytecode + constants | `interp.rs` dispatch loop | resolved JS values |
| Job-queue drain | microtask + macrotask queues | `run_to_completion` | quiescent runtime |

Each level's induced properties function as constraints on the level beneath it. The terminal property is preserved end-to-end only when each level respects the inherited constraint from above and emits a directive-free artifact to the level below.

## Corpus references

The design is articulated across the RESOLVE corpus:

- [Doc 729 — Cruftless](https://jaredfoy.com/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs) — the primary articulation; the resolver-instance pattern applied to this runtime.
- [Doc 432 — SERVER: An Architectural Style for Engine Orchestration](https://jaredfoy.com/resolve/doc/432-server-an-architectural-style-for-engine-orchestration) — the orchestration-level constraint specification.
- [Doc 426 — PRESTO: An Architectural Style for Representation Construction](https://jaredfoy.com/resolve/doc/426-presto-an-architectural-style-for-representation-construction) — the construction-level constraint specification.
- [Doc 719 — The Pipeline Pattern Across Subjects](https://jaredfoy.com/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation) — the structural correspondence between this runtime and PRESTO.
- [Doc 717 — The Apparatus Above the Engine Boundary](https://jaredfoy.com/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point) — the cut-rung framework that places each resolver-instance's boundary.
- [Doc 581 — Pin-Art and the Discipline of Near-Necessity Substrate Construction](https://jaredfoy.com/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction) — the discipline by which substrate work proceeds.
- [Doc 725 — The Cluster-to-Walk Mode Transition](https://jaredfoy.com/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction) — the walk-mode discipline for per-package fault chains.

## Apparatus

- `seed.md` — engagement-level resume vector and operative discipline.
- `trajectory.md` — round log; latest anchor at the most recent EXT.
- `host/tools/parity-measure-v2.sh` — canonical sweep entry against the parity basket; outputs JSON.
- `host/tools/parity-fast-v2.sh` — 43-package exemplar sweep for fast iteration during substrate moves.
- `host/tools/parity-cluster-v2.sh` — per-cluster targeted sweep extracted from the latest canonical reading.
- `host/tools/select-cluster.py` / `host/tools/select-exemplars.py` — basket extractors for the targeted sweeps.

## Status

The engagement is approximately 30 substrate rounds into derivation against ECMA-262 + WHATWG. Migration-cost gap between the rquickjs ceiling (`host/`) and the hand-rolled engine (`host-v2/`) is currently ~4.1 percentage points on the 1026-package parity-top500 basket. The morph trajectory toward the Cruftless terminal-property design proceeds under the walk-mode discipline of Doc 725; each substrate round is dispatched per Pin-Art near-necessity (Doc 581) and locatable on the two architectural addresses of Doc 729 §VIII.

## Origins

The repository began as an apparatus for AI-assisted cross-language code translation, accompanying [Doc 702](https://jaredfoy.com/resolve/doc/702-ai-assisted-cross-language-code-translation-as-a-pin-art-bilateral-under-sipe-t-threshold-conditions-reading-the-bun-zig-to-rust-port). The hand-derived JavaScript runtime that emerged through the engagement supplanted the original apparatus focus; the design crystallized through Doc 717's engine-cut framework and Doc 719's recognition of the structural correspondence with PRESTO. The Doc 729 articulation names the destination and renames the runtime.

## License

[MIT](./LICENSE) — Copyright (c) 2026 Jared Foy.
