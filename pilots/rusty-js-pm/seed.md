# rusty-js-pm — Resume Vector / Seed

**Locale tag**: `L.rusty-js-pm` (per [Doc 737](../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md))

**Workstream**: the package-manager tier of the Cruftless stack, structured per Doc 732 §VI (PM-R1 specifier resolver, PM-R2 fetcher/extractor, PM-R3 linker) and Doc 730 §III–§VII (vertical-recurrence lowering-compiler shape).
**Author**: 2026-05-21 session, founded immediately after JIT-EXT 9.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Composes with**:
- [Doc 729](../../../corpus-master/corpus/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) (Cruftless five-instance enumeration; this pilot adds instance #0 below module load).
- [Doc 730](../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) §III–§VII (lowering-compiler shape) + §XII–§XVI (deviation pipeline + bidirectional engine-diff oracle).
- [Doc 732](../../../corpus-master/corpus/732-the-package-manager-as-the-resolver-instance-below-module-load-lockfile-as-artifact-registry-as-bilateral-source-and-the-sixth-layer-of-the-cruftless-stack.md) (the package manager as resolver-instance #0 of the Cruftless stack — design target).
- `pilots/rusty-js-runtime/derived/src/module.rs` — the consumer of this pilot's artifact (the `node_modules` tree this pilot lays out is what the module loader walks).
- `host-v2/src/node_stubs.rs` — the node:* stub layer; some packages depend on its surface and the install will trigger that dependency at module-load time downstream.

## I. Telos

Build a single-tier baseline package manager for cruftless that demonstrates the Doc 732 structural claim empirically: a package install whose source is bilateral (local package.json + remote registry) and whose lockfile collapses the bilateral source to a unilateral source for subsequent installs, with structural complexity bounded by the package.json field set honored and the registry response schema consumed.

The success criterion is *not* feature parity with `bun install` or `npm install`. The success criterion is the structural shape match against Doc 732 §IV–§V (four bootstrap properties at the package-install instance) and the falsifiability checks of §VIII. Each substrate move proves the bootstrap properties hold over a wider subset of inputs (more package shapes, more registry responses, more on-disk states) than the previous move.

### I.1 Bounded first-cut telos

The first cut's telos is end-to-end install of an **exact-pinned, registry-only, flat-layout, no-scripts, no-peers, no-workspaces** package.json against the public npm registry, producing a `node_modules` tree the cruftless runtime can `require` from, plus a text-format lockfile recording the closure. "Works" means: a cruftless test consumer that does `require('lodash')` (or any exact-pinned package) loads the package, runs its init, and returns the expected value, *after* `cruftless install` was run against a fresh empty directory with only `package.json` present.

The full feature surface (ranges, peers, workspaces, lifecycle scripts, private registries, binary lockfile parity) is queued for subsequent cuts per Doc 732 §VI's carve-out list. The carve-outs are spec-aligned: each is a region where the complexity-vs-yield ratio is unfavorable for the first-cut baseline.

## II. Apparatus

The package install is **resolver-instance #0** in the Doc 729 stack per Doc 732 §II. It composes with:

- **Resolver-instance #-1 (Cargo build)**: produces the `cruftless` binary that runs this pilot's code. Out of scope.
- **Resolver-instance #1 (bootstrap)**: this pilot uses host-v2's HTTP client + filesystem APIs (the bootstrap installs them; this pilot consumes them). Composition is one-way; the install does not mutate the Runtime graph the bootstrap built.
- **Resolver-instance #3 (module load)**: the consumer of this pilot's artifact. The `node_modules` tree the install lays out is what `module.rs::evaluate_module` walks. Composition is artifact-to-source per Doc 730 §IV vertical recurrence.

Per Doc 730 §XII–§XVI, every substrate move at this instance is gated on the §XVI bidirectional engine-diff oracle. The reference engine is `bun install` per Doc 732 §VII. The four-case categorization: (1) cruftless-violates-Bun-spec-correct, (2) Bun-violates-npm-doc-cruftless-correct, (3) both-diverge-from-package-expectation, (4) implementation-freedom.

The three sub-resolvers inside the package install form their own Doc 730 vertical recurrence:

- **PM-R1 (specifier resolver)**. Source: package.json `dependencies` entries. Resolver: a function from specifier to `(name, version, url, hash)`. Artifact: the lockfile's `dependencies` section.
- **PM-R2 (fetcher/extractor)**. Source: the lockfile's `(url, hash)` per dep. Resolver: HTTP GET + hash verify + tar.gz extract. Artifact: each dep present at `node_modules/<pkg>/`.
- **PM-R3 (linker)**. Source: extracted on-disk staging + resolved graph. Resolver: place each dep at its final path; create `bin` symlinks. Artifact: a `node_modules` tree.

Each sub-resolver is its own substrate site for Pin-Art moves.

## III. Methodology

The Doc 732 §VI first-cut scope is the operational template. The substrate moves proceed in order of the three sub-resolvers, each itself a small lowering-compiler:

1. **Crate scaffold.** New crate `pilots/rusty-js-pm/derived/`. Top-level entry: `pm_install(pkg_root: &Path) -> Result<InstallReport, PmError>`. Workspace member added to root Cargo.toml.

2. **PM-R1 first cut.** `parse_package_json` reads `dependencies` (exact pins only in first cut). `resolve_specifier(name, version)` GETs `https://registry.npmjs.org/{name}` and looks up the requested version's `dist.tarball` + `dist.integrity`. Output: the lockfile structure (a `Vec<ResolvedDep>` keyed on name, recording `version`, `url`, `integrity_sha512`).

3. **PM-R2 first cut.** `fetch_tarball(url, expected_integrity) -> Bytes` GETs the tarball, verifies integrity, returns the bytes. `extract_tarball(bytes, dest_dir)` gunzips and untars into `dest_dir`. Both functions return errors loudly on hash mismatch or tar malformation.

4. **PM-R3 first cut.** `link_flat(staging, pkg_root)` moves each extracted package from `staging/` to `pkg_root/node_modules/<pkg>/`. `bin` symlinks created per package.json `bin` field into `node_modules/.bin/`. Collisions in first cut: error out rather than nest.

5. **Lockfile write/read.** Text format. JSON or TOML — first cut chooses JSON for ecosystem-familiarity (`cruftless.lock.json`). Schema: `{ version: 1, dependencies: { "<name>": { version, url, integrity } } }`. Read on subsequent install; if every entry's `node_modules/<name>/package.json` matches the recorded version, install is a no-op. Otherwise refetch the missing entries.

6. **Wire to host-v2.** A `cruftless install` subcommand at the host-v2 CLI dispatches to `pm_install`. The host-v2 CLI is the keeper-facing surface; the pilot is the engine-internal substrate.

7. **End-to-end test.** A fixture: a fresh tmpdir with only `package.json` declaring `{ "dependencies": { "lodash": "4.17.21" } }`. Run `cruftless install`. Verify `node_modules/lodash/package.json` exists, version matches, integrity verifies, `cruftless -e "require('lodash').identity(42)"` returns 42.

8. **§XVI oracle gate.** Before each substrate move, run the equivalent install with `bun install` against the same fixture. Diff the resulting `node_modules` tree + lockfile. Categorize divergences per the four cases.

## IV. Carve-outs and bounded scope

Per Doc 732 §VI:

- **No semver range resolution.** Exact-pinned versions only. Caret/tilde/complex ranges deferred to second cut.
- **No peer-dependency reconciliation.** Warn-and-continue. Reconciliation deferred.
- **No workspace support.** Monorepo features queued.
- **No lifecycle scripts.** `preinstall`/`install`/`postinstall`/`prepare` recorded but not executed. Script execution is a separate resolver-instance (subprocess invocation) whose Cruftlessness deserves its own design pass per Doc 729 §VII.A's bootstrap-as-seed concern.
- **No git/file/link/workspace specifiers.** Only registry specifiers.
- **No private registry, no auth, no scope-aware routing.** Public npm only. Scoped packages (`@scope/name`) supported because URL shape is mechanical; auth is not.
- **No nested layout fallback.** Collisions error out rather than triggering nested-layout repair.
- **No binary lockfile parity** (`bun.lockb`). Text format only. Binary parity added when consumer-project compatibility requires it.
- **No global cache / hardlink layer.** Each install refetches; bytes go directly to `node_modules/<pkg>/`. A cache layer is a second-cut substrate move that does not change the bootstrap properties.

These carve-outs are spec-aligned: each is a region where the complexity-vs-yield ratio is unfavorable for a first-cut baseline.

## V. Standing artefacts

- `pilots/rusty-js-pm/derived/Cargo.toml` — crate manifest. Depends on `reqwest` (HTTP), `flate2` (gunzip), `tar` (untar), `sha2` (integrity verify), `serde` + `serde_json` (package.json + lockfile parse).
- `pilots/rusty-js-pm/derived/src/lib.rs` — top-level entry `pm_install`.
- `pilots/rusty-js-pm/derived/src/manifest.rs` — package.json parser (first cut: only the fields we honor).
- `pilots/rusty-js-pm/derived/src/resolver.rs` — PM-R1.
- `pilots/rusty-js-pm/derived/src/fetch.rs` — PM-R2.
- `pilots/rusty-js-pm/derived/src/link.rs` — PM-R3.
- `pilots/rusty-js-pm/derived/src/lockfile.rs` — lockfile read/write.
- `pilots/rusty-js-pm/docs/manifest-field-coverage.md` — table of every package.json field the first cut honors vs defers vs rejects. Cardinality of "honors" is the PM's alphabet upper bound.
- `pilots/rusty-js-pm/docs/registry-response-schema.md` — table of every registry-response field the first cut consumes. Cardinality is the consumed-surface bound.
- `trajectory.md` — time-ordered record of substrate moves and yields.

## VI. Resume protocol

Read Doc 729 (Cruftless), Doc 730 §III–§VII + §XII–§XVI, Doc 732 in full, this seed, then trajectory.md. The Doc 732 §VI first-cut scope is the design target; the §XVI four-case categorization is the gate before any substrate move; the cadence target is the ~10-minute-per-substrate-move shape the JIT workstream observed in EXT 1–9.

First substrate move (when implementation begins): the manifest field-coverage table (`docs/manifest-field-coverage.md`). Walk the package.json spec (or, operationally, the union of fields appearing in the top-100 most-installed npm packages). For each field, classify:

- **Honor in first cut**: `name`, `version`, `dependencies` (exact pins only).
- **Honor in first cut with caveats**: `bin` (linked but only after extraction), `main` / `module` / `exports` (recorded into the lockfile for module-load to read; not consumed by install itself).
- **Defer (second cut)**: `devDependencies`, `peerDependencies`, `optionalDependencies` (with range resolution), `workspaces`.
- **Defer (later)**: `scripts`, `engines`, `os`, `cpu` (no enforcement in first cut).
- **Reject in first cut**: anything we cannot honor and whose absence breaks the install (none identified yet).

Output is `pilots/rusty-js-pm/docs/manifest-field-coverage.md`. Equivalent to JIT-EXT 1's P4 enumeration: classification first, scaffold after.

Cruftless engine state at this workstream's start (JIT-EXT 9 close, 2026-05-21):
- Bytecode interpreter: ~80% Bun-load-parity.
- Module loader (`module.rs`): operational, walks node_modules, handles ESM + CJS + bare node:* specifiers.
- JIT: operational, 261× speedup on hot integer loops, deopt-disable flag landed.
- Package manager: **not yet started.** This pilot's first cut.

Pin-Art tag prefix for this workstream: `Ω.5.P05.L1.pm-*` for install-time substrate moves (P05 = Module loader pipeline per dag-coordinates.json; L1 = load-time-bounded layer; PM is upstream of module load and shares the pipeline). Open question: whether PM merits its own pipeline id (P17) in a future manifest amendment. For now, P05 is the natural composition site — the install produces what the loader walks, and the two share a fault domain. Per `host/tools/tag-grammar.md` §1, `<handle>` is `pm-resolver`, `pm-fetch`, `pm-link`, `pm-lockfile-write`, etc.
