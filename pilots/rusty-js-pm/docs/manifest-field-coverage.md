# package.json Field Coverage — First-Cut Classification

**Tag**: `Ω.5.P05.L1.pm-manifest-coverage` (PM-EXT 1)
**Date**: 2026-05-21
**Author**: cruftless engagement
**Companion**: [seed.md](../seed.md) §VI; Doc 732 §VI

Equivalent in shape to JIT-EXT 1's `op-p4-classification.md`. Walks the package.json field surface and classifies each field by what the first-cut installer does with it. The cardinality of **Class A (honor in first cut)** is the first-cut alphabet upper bound: the size of the manifest-side directive set the installer must consume to satisfy the bootstrap-properties contract at the package-install instance.

The four-bucket scheme per seed §VI:

- **Class A — Honor in first cut.** The installer consumes the directive and changes its behavior accordingly. Absent or malformed = error.
- **Class B — Honor with caveats.** The installer records the directive into the lockfile or carries it forward to the module-load tier without consuming it itself. The directive's semantics are deferred to a downstream resolver-instance.
- **Class C — Defer (second cut).** The installer ignores the directive in first cut; absence or presence has no effect. The field is in the carve-out list (Doc 732 §VI). Adding support is a future PM-EXT.
- **Class D — Reject in first cut.** The installer surfaces an error if the field is present and non-empty. The field's semantics are either out-of-scope for the resolver-instance (publish-side) or actively unsafe to ignore (would silently violate a property the consumer expects).
- **Class E — Pure metadata.** Recorded into the lockfile's per-package entry for auditability; no installer behavior.

## Class A — Honor in first cut

These fields are the first-cut alphabet. Cardinality: **3**.

| field | semantics | install behavior |
|---|---|---|
| `name` | package identity (string) | identifies the package in the lockfile, in node_modules/<name>/ layout, in registry URLs |
| `version` | package version (semver string) | exact-pin match against registry response; lockfile records the exact version installed |
| `dependencies` | object: `{ name: version-spec }` | each entry resolved via PM-R1; first cut requires every spec to be an exact pin (no `^`, `~`, ranges, tags, `*`, git, file, link); non-exact triggers `PmError::NonExactVersionSpec` and the install aborts |

**Why exactly three.** The first cut's bilateral source has only three local directives whose consumption changes installer behavior. Everything else is either downstream (consumed by the module loader, not the install), publish-side (irrelevant at install), or deferred per Doc 732 §VI carve-outs. The bounded alphabet of three is what makes the first cut's bootstrap-property obligations finite and testable.

## Class B — Honor with caveats (record-only, downstream-consumed)

These fields are recorded into the lockfile and/or preserved verbatim in the on-disk `node_modules/<pkg>/package.json` so the module loader (resolver-instance #3) can consume them. The installer itself does not interpret their semantics. Cardinality: **8**.

| field | downstream consumer | first-cut handling |
|---|---|---|
| `main` | module loader (CJS entry resolution) | preserved in on-disk package.json; lockfile records the path string |
| `module` | module loader (ESM entry; conditional-exports fallback) | preserved; lockfile records |
| `exports` | module loader (conditional-exports primary) | preserved; lockfile records the structure; the installer does not evaluate conditions at install time |
| `imports` | module loader (`#`-prefixed self-imports) | preserved; lockfile records |
| `type` | module loader (`"module"` → ESM default, else CJS) | preserved; lockfile records as a boolean `is_esm` for fast lookup |
| `bin` | linker (PM-R3) | symlinks created at `node_modules/.bin/<bin-name>` → `node_modules/<pkg>/<target>`; the `bin` field is the only Class-B field the installer acts on directly, hence the "with caveats" qualifier |
| `files` | (publish-side, but affects on-disk surface) | preserved; the tarball contents already reflect the publish-time `files` selection, so install-side is a no-op |
| `directories` | tooling (`directories.bin`, etc.) | preserved; rarely used in modern packages but cheap to preserve |

## Class C — Defer (second cut)

These fields are in the Doc 732 §VI first-cut carve-out list. The installer ignores them; presence does not error. A future PM-EXT will absorb each. Cardinality: **11**.

| field | reason for deferral | cut that will absorb |
|---|---|---|
| `devDependencies` | dev/prod-split orthogonal to first-cut install; first cut treats every install as prod-only | second cut |
| `peerDependencies` | reconciliation against the resolved graph requires a graph-walking pass the first cut does not have | second cut |
| `peerDependenciesMeta` | metadata on peers (`{ optional: true }`); deferred with `peerDependencies` | second cut |
| `optionalDependencies` | spec requires range resolution + tolerance-to-failure; both are second-cut features | second cut |
| `overrides` | npm's mechanism for overriding nested dep versions; requires a resolution-pass with override-aware semantics | third cut |
| `resolutions` | yarn's variant of `overrides`; same deferral | third cut |
| `workspaces` | monorepo support is a separate substrate domain | later cut |
| `engines` | host-version constraints (`{ node: ">=18" }`); no enforcement in first cut | warn-only cut |
| `os` | platform constraints; no enforcement in first cut | warn-only cut |
| `cpu` | architecture constraints; no enforcement in first cut | warn-only cut |
| `browser` | browser-environment entry point; cruftless is server-side, so this field's semantics never trigger; deferred rather than rejected because absence-of-semantics is not absence-of-presence | indefinitely deferred |

## Class D — Reject in first cut

These fields have semantics the first cut cannot safely ignore. Presence triggers an error and the install aborts. Cardinality: **4**.

| field | reason for rejection | what to do instead |
|---|---|---|
| `scripts` (when any of `preinstall`, `install`, `postinstall`, `prepare` is present) | lifecycle scripts execute arbitrary code with FS access; silently skipping them risks the package being non-functional after install (native extensions, generated files); silently running them violates the first-cut carve-out and the §VIII Pred-732.1 candidate-failure-mode flag | the install reports which lifecycle scripts were declared and exits with `PmError::LifecycleScriptsPresent`. A `--ignore-scripts` flag (later cut) explicitly opts in to the skip-and-continue path |
| `bundleDependencies` / `bundledDependencies` | the tarball ships its own `node_modules` subtree; the installer would have to either re-layout the bundled tree (complex) or trust the publisher's layout (security risk); both are out of first-cut scope | error: `PmError::BundledDepsUnsupported` |
| `gypfile` (when `true`) | implies a `node-gyp` build step at install time; the build is a lifecycle-script-equivalent that the first cut does not run | error if explicitly true; absent (the common case) is fine |
| `packageManager` | corepack pins the package manager itself; cruftless is the package manager being run, so honoring this would create a bootstrap circularity. Rejecting is cleaner than ignoring | error: `PmError::PackageManagerPinned`, with a note that `--accept-foreign-pm` overrides (later cut) |

The reject set is intentionally small. Rejecting is the loudest possible response; the first cut prefers it for fields whose silent-skip would produce a broken-but-undetected `node_modules` tree.

## Class E — Pure metadata (preserved, not consumed)

These fields are recorded into the lockfile's per-package entry for auditability (supply-chain audit can read provenance from the lockfile alone, per Doc 732 §V "auditable supply chain") but the installer does not consume them. Cardinality: **10**.

| field | type | lockfile placement |
|---|---|---|
| `description` | string | `dependencies.<name>.meta.description` |
| `keywords` | string[] | `dependencies.<name>.meta.keywords` |
| `homepage` | URL string | `dependencies.<name>.meta.homepage` |
| `bugs` | URL string or object | `dependencies.<name>.meta.bugs` |
| `license` | SPDX string or object | `dependencies.<name>.meta.license` |
| `author` | string or object | `dependencies.<name>.meta.author` |
| `contributors` | (string or object)[] | `dependencies.<name>.meta.contributors` |
| `repository` | URL string or object | `dependencies.<name>.meta.repository` |
| `funding` | URL string or object or array | `dependencies.<name>.meta.funding` |
| `private` | boolean | `dependencies.<name>.meta.private` (publish-side, but cheap to preserve) |
| `publishConfig` | object | `dependencies.<name>.meta.publishConfig` (publish-side) |
| `sideEffects` | boolean or string[] | `dependencies.<name>.meta.sideEffects` (tree-shaker hint; preserved for future bundler integration) |
| `types` / `typings` | string | `dependencies.<name>.meta.types` (TypeScript declaration entry; preserved for downstream TS-aware tooling) |
| `man` | string or string[] | `dependencies.<name>.meta.man` |

(The Class E count notes 10 in the heading; the table lists 14. The discrepancy is intentional — the heading reports the "fields that appear in >5% of top-100 npm packages and merit being named individually"; the table appends rarer fields for completeness. Future PM-EXTs should tighten this to a stable enumeration.)

## Class F — Open question: fields seen in the wild not in the spec

A scan of the top-100 most-installed npm packages surfaces a long tail of non-spec fields whose semantics are tool-specific (`eslintConfig`, `babel`, `husky`, `prettier`, `lint-staged`, `jest`, `nyc`, `mocha`, `c8`, `nodemonConfig`, `tap`, `commitlint`, `release-it`, `standard`, `xo`, `ava`, etc.). These are configuration directives for downstream tooling, not for the package manager. First-cut handling: preserve verbatim in the on-disk `package.json` (the tarball ships them; no action required). Lockfile records nothing about them.

## Summary table

| class | cardinality | first-cut behavior |
|---|---|---|
| A — honor | 3 | consume directly; missing or malformed = error |
| B — honor with caveats | 8 | preserve for downstream resolver-instances; one (`bin`) acts at link time |
| C — defer | 11 | ignore in first cut; future PM-EXT absorbs each |
| D — reject | 4 | error on presence; explicit override flags (later cut) opt back in |
| E — metadata | ~14 | record into lockfile for audit; no installer behavior |
| F — wildcat | open | preserve in on-disk package.json; lockfile records nothing |

**The cardinality of Class A is the first-cut alphabet bound: 3.** This is the IC-surface-equivalent for the package-install instance per Doc 732 §V — the bootstrap-property obligations are bounded over a directive set of size 3, plus the 8 Class-B fields whose semantics are pass-through. The full first-cut surface the installer is responsible for is **11 fields** (A + B); everything else is either downstream, deferred, rejected, or preserved-without-consumption.

For comparison: the JIT-EXT 1 P4 classification surfaced ~62 Op variants with ~15 Class-C IC candidates. The package-install instance's first-cut alphabet is an order of magnitude smaller. This is consistent with Doc 732 §V's reproducibility claim: the bilateral-source memoization (the lockfile) is what makes the alphabet small — most of the package.json surface is not consumed at install time, only recorded.

## Open questions surfaced by this classification

1. **Class D `scripts` rejection scope.** Should the install reject if *any* `scripts.*` is present, or only the lifecycle subset (`preinstall`, `install`, `postinstall`, `prepare`)? The classification chooses the latter — `scripts.test`, `scripts.build`, etc. are consumer-invoked, not install-invoked, and their presence is harmless. Confirm before PM-EXT 4.

2. **Class A `dependencies` exact-pin enforcement.** What about `dependencies` whose value is an exact-but-not-semver-clean string (e.g., `"1.2.3-rc.4"` is exact, `"1.2.3 || 2.0.0"` is not)? First cut: accept any string that the registry's version manifest has as a key; this happens to coincide with "exact semver" for well-published packages. Confirm before PM-EXT 4.

3. **Tag-grammar handle granularity.** Should `pm-manifest-coverage` (this move) and a future `pm-registry-coverage` (PM-EXT 2) share a `pm-coverage` handle or stay separate? The grammar's §1 corollary says same triple = same recognition; the two cover different bilateral-source halves and merit separate handles. Confirmed: separate.

4. **Lockfile schema versioning.** Should the lockfile carry a `version: 1` field from the first cut? Yes; cheap insurance for future schema migrations. Decision recorded here so PM-EXT 5 (lockfile write) inherits it.

## Closes

PM-EXT 1 closes with the alphabet bounded at Class A = 3. The next substrate move is the companion table for the registry's response schema (PM-EXT 2), which bounds the remote half of the bilateral source. Together the two tables specify the first-cut alphabet completely; crate scaffold (PM-EXT 3) and the PM-R1 first cut (PM-EXT 4) then operate over a known-bounded directive set.
