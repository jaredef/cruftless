# Cruftless — Agent Orientation

A hand-derived JavaScript runtime in Rust, targeting the Node.js package ecosystem as its compatibility surface. The runtime is independent — built under Pin-Art discipline against ECMA-262 + WHATWG, with Bun as the empirical oracle for parity measurement, not as a source.

This document orients an agent (Claude or otherwise) to the codebase's structure, disciplines, and standing artifacts.

## Canonical tier separation: `apparatus/` vs `docs/`

The repository partitions documentation-and-state into two distinct top-level surfaces. The distinction is load-bearing for the cybernetic dyad; agents must respect it.

- **`apparatus/`** is the apparatus-tier tooling and output that **directly informs the cybernetic loop**. Required reading on every loop iteration. Agents internalize `apparatus/docs/` schemas + rules and consult `apparatus/locales/` for the locale registry.
- **`docs/`** is the **sidecar for development that the keeper utilizes in the cybernetic dyad**. It is the keeper's thinking surface (live analyses, in-flight designs at `docs/engagement/`) and a read-only mirror of the published corpus (`docs/corpus-ref/`). Agents read from `docs/` **only when the keeper explicitly directs** (e.g., "read Doc 736") or when a task requires composing against a specific corpus articulation.

Conflating the two breaks the dyadic operating model: `apparatus/` is rung-1 (machine substrate); `docs/` is rung-2 (keeper supplement) per Doc 711. Promotions from `docs/` to `apparatus/` happen only on explicit keeper directive.

**Required agent reading (apparatus tier):**
- [`apparatus/docs/repository-apparatus.md`](apparatus/docs/repository-apparatus.md) — full enumeration of the cybernetic discipline: measurement instruments (CRB, diff-prod, test262, canonical fuzz, TCC, TXC, component-A/B probes), discipline artifacts, locale structure, substrate tiers, feedback paths. **§0 of that doc is the canonical articulation of this tier separation; read it first.**
- [`apparatus/docs/predictive-ruleset.md`](apparatus/docs/predictive-ruleset.md) — consolidated 15-rule predictive view.
- [`apparatus/docs/standing-rule-13-prospective-application.md`](apparatus/docs/standing-rule-13-prospective-application.md) — revert-then-deeper-layer-closure thesis.
- [`apparatus/docs/agent-feedback-schema.md`](apparatus/docs/agent-feedback-schema.md) — schema for per-locale cross-resolver review (`pilots/<locale>/agent-feedback.md`).
- [`apparatus/locales/manifest.json`](apparatus/locales/manifest.json) — enumerated locale instances.
- [`apparatus/locales/CANDIDATES.md`](apparatus/locales/CANDIDATES.md) — consult before founding any new locale.

## Project identity

- **Project**: Cruftless. Crate name `cruftless`; user-facing CLI binary `cruft`.
- **Repository convention** (mirrors oven-sh/bun): org name is the brand, repo name is the CLI tool. The `cruftless` alias binary remains for one release of engagement-internal-script backward compatibility.
- **Architecture articulation**: the resolver-instance pattern per Doc 729. Each layer's directives are consumed at that layer's resolver; no layer's artifact carries residue from the layer above. The induced property is vertically-recursive directive consumption with stage-deterministic emission.

## Workspace layout

```
cruftless/             — host binary crate (cruft CLI + cruftless alias)
pilots/                — per-surface Pin-Art pilot crates (each under derived/)
  rusty-js-{ast,parser,bytecode,gc,runtime}/derived/
                       — engine substrate
  rusty-js-ir/derived/ — Tier-1.5 spec-as-source-of-truth IR
  rusty-js-jit/derived/
                       — LeJIT: Cranelift-backed baseline JIT
  rusty-js-shapes/derived/
                       — hidden classes substrate
  rusty-js-pm/derived/ — package manager (resolver-instance #0)
  rusty-js-caps/       — capability-passing runtime
  diff-prod/           — differential prod-test methodology + fixtures
  tls/derived/         — TLS 1.3 substrate
  web-crypto/derived/  — WebCrypto primitives
  ... (per-surface pilots for fetch, http-codec, sockets, blob, file,
       buffer, bun-serve, bun-spawn, bun-file, compression, streams,
       structured-clone, textencoder, urlsearchparams, websocket,
       x509, node-fs, node-http, node-path, asn1-der)
legacy/host-rquickjs/  — rquickjs-backed reference ceiling (no new feature
                         work; retained for parity-measurement reference)
scripts/               — operational scaffolding
  diff-prod/           — differential prod-test runner + fixtures
  locales/             — Pin-Art locale discovery + manifest
  test262-sample/      — curated test262 representative-sample runner
specs/                 — curated spec extracts and methodology docs
derive-constraints/    — apparatus binary (extract → cluster → invert)
```

## The Pin-Art locale system

Every workstream is a **locale**: a directory containing `seed.md` + `trajectory.md` per Doc 581. The seed names the telos, apparatus, methodology, carve-outs, and resume protocol; the trajectory records substrate moves in time order. The pair recurs at every substrate depth per Doc 733 (cross-tier fractal recurrence) and within tiers per Doc 737 (sub-workstreams with multi-rung shape spawn nested locales at deeper coordinates).

### Manifest

The authoritative inventory of every locale lives at:

```
apparatus/locales/manifest.json
```

Generated by `apparatus/locales/discover.sh` (a filesystem walk that finds every `seed.md`). Each entry carries `coord`, `tag`, `parent`, `scope`, `depth`, `rung_count`, `status` per Doc 737 §IV.

**Discipline**: after spawning a new locale (creating any new `seed.md` under `pilots/`), re-run `apparatus/locales/discover.sh` and commit the refreshed manifest in the same change or immediately after. The manifest is the load-bearing record of the apparatus-tier coordinate space; staleness degrades the cross-tier convergence property between commit-tag coordinates, locale-path coordinates, and source-identifier coordinates.

### Resume protocol

To pick up work on any locale: read its `seed.md` first (telos + apparatus + methodology), then its `trajectory.md` tail (most recent rungs). The pair is sufficient for a fresh reader to become operational on that workstream in one read.

## Source-identifier coordinate conventions

The Rust source carries an encoded coordinate system in its identifier conventions. Reading a name yields substrate position without external documentation:

- **Prefix** encodes JS-observability stratum: plain `name` (user-visible), `__name` (engine-internal sentinel, non-enumerable), `@@name` (well-known Symbol property), `__engine_op` registered via `register_engine_helper` (hidden from `globalThis`).
- **Function suffix** encodes invocation surface: `_via` (Runtime-side dispatching, can call back into JS for Object→primitive coercion); pure-primitive helpers live in `abstract_ops::*` with no Runtime access.
- **Property-install helper** encodes descriptor shape: `set_own_frozen` ({w:f, e:f, c:f}, built-in ctor.prototype + namespace constants), `set_own_internal` ({w:t, e:f, c:t}, built-in proto methods + engine sentinels), `set_own` ({w:t, e:t, c:t}, user-default).
- **Registration helper** encodes binding tier: `register_method` (own property on a target), `register_intrinsic_method` (with arity + non-enumerable defaults), `register_engine_helper` (hidden table), `register_global_fn` (globalThis).
- **Module path** encodes substrate pillar (see workspace layout above). The `/derived/` segment marks the implementation as Pin-Art-derived-from-constraints.

A name whose prefix and install helper disagree (e.g., `__name` registered via `set_own` instead of `set_own_internal`) is a bug shape; the convention self-checks.

## Measurement baselines

Two gates the engagement holds at every substrate move:

- **diff-prod**: 42/42 PASS at the runtime-semantics probe. Fixtures at `scripts/diff-prod/fixtures/`; runner at `scripts/diff-prod/run-all.sh`. Each fixture runs under both `cruft` and `bun` and diffs stdout byte-for-byte. Failures categorized per Doc 730 §XVI (four-case engine-diff oracle).
- **test262-sample**: 77.6% runnable pass rate (5,594 PASS / 1,611 FAIL / 384 SKIP on the curated representative sample). Runner at `scripts/test262-sample/run-sample.sh`; sample paths at `scripts/test262-sample/sample-paths.txt` target the surface real Node packages exercise.

The two probes triangulate: each substrate fix that flips a diff-prod fixture should also flip a count of test262 entries.

## Commit and authorization discipline

- **No commits without explicit user request.** Every commit is user-authorized. The substrate worker drafts changes and verifies build + gates; the user authorizes the landing.
- **No `Co-Authored-By` lines.** Commits are single-author.
- **Em-dash restraint** in prose: target 0-1 per 1000 words. Prefer commas, parens, periods.
- **Trajectory entries land with the commit they describe.** Each substrate move updates the locale's `trajectory.md` as part of the commit.

## Operational quick-reference

### Local script environment

Repo scripts that need filesystem-local paths source `scripts/env.sh`. The loader reads `env.local` when present, then supplies portable repo-relative fallbacks. `env.example` is the documented contract for portable configuration; `env.local` is the machine-specific instantiation for this checkout.

Use env variables for operational paths instead of baking workstation paths into scripts. Current variables include `CRUFTLESS_ROOT`, `CRUFT_BIN`, `BUN_BIN`, `NODE_BIN`, `T262_ROOT`, `CRUFTLESS_SIDECAR`, `TEST_ARTIFACTS_DIR`, `PROD_SANDBOX`, `RESULTS_DIR`, `PROBE_ROOT`, `CRUFTLESS_TEST262_RESULTS_ROOT`, `CRUFTLESS_CROSS_RUNTIME_RESULTS_ROOT`, and `LOCAL_CRUFT`. New scripts should compute their nearby root, source `scripts/env.sh`, and then derive any remaining paths from these variables or from repo-relative defaults.

Generated test and benchmark artifacts live in the external sidecar, not in the repository. On this machine `env.local` sets `CRUFTLESS_SIDECAR=/home/jaredef/Developer/cruftless-sidecar` and `TEST_ARTIFACTS_DIR=/home/jaredef/Developer/cruftless-sidecar/results`; test262 samples, diff-prod output, and cross-runtime benchmark output should write there unless a caller explicitly overrides the relevant env var. Do not introduce new repo-local `results/` writers.

| Task | Command |
|---|---|
| Build cruft binary | `cargo build --release --bin cruft -p cruftless` |
| Run diff-prod (all fixtures) | `scripts/diff-prod/run-all.sh` |
| Run diff-prod (single fixture) | `scripts/diff-prod/run.sh <name>` |
| Run test262 sample (cruft) | `scripts/test262-sample/run-sample.sh` |
| Run test262 sample (bun baseline) | `scripts/test262-sample/run-sample-bun.sh` |
| Refresh locale manifest | `apparatus/locales/discover.sh` |
| Workspace test (all pilots) | `cargo test --release --workspace` |
| Per-pilot test | `cargo test --release -p <crate-name>` |

`CRUFT_BIN` is supplied by `env.local` on this machine. The `run-sample.sh` auto-copy convention still supports `LOCAL_CRUFT` for hosts that want to execute from a local binary cache instead of directly from `target/`.

## Standing corpus references

Architecture and methodology are articulated at [jaredfoy.com/resolve/](https://jaredfoy.com/resolve/). Load-bearing for the runtime's structure:

- Doc 729 — Cruftless: the resolver-instance pattern as the comprehensive design.
- Doc 731 — The JIT as a lowering-compiler tier; alphabet purity upstream as the bound on JIT complexity.
- Doc 730 — Vertical recurrence of the lowering compiler closure across substrate tiers (with §XII–§XVI deviation-resolution pipeline and bidirectional engine-diff oracle).
- Doc 581 — Pin-Art and the resume vector discipline.
- Doc 733 — Fractal seeds-and-trajectories: pair recurrence across substrate depth.
- Doc 737 — The locale as coordinate: nested seed-trajectory pairs as Pin-Art substrate positions.
- Doc 738 — The source identifier as coordinate: naming-convention encoding at the source tier.
- Doc 736 — Capability-passing runtime: architecturally impossible supply chain attacks.

Per-locale composes-with lists in each `seed.md` cite the specific docs that load-bear for that workstream.

## When in doubt

Read the locale's `seed.md` + `trajectory.md` tail. If the work would spawn a nested sub-workstream with multi-rung shape (per Doc 737 §II promotion threshold), create the nested locale first, refresh `apparatus/locales/manifest.json`, then begin the substrate work.
