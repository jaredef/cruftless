# npm Registry Response Schema — First-Cut Coverage

**Tag**: `Ω.5.P05.L1.pm-registry-coverage` (PM-EXT 2)
**Date**: 2026-05-21
**Companion**: [seed.md](../seed.md) §II ("bilateral source"); [manifest-field-coverage.md](./manifest-field-coverage.md); Doc 732 §III (the bilateral source) + §VI (first-cut scope)

The companion to [manifest-field-coverage.md](./manifest-field-coverage.md). The manifest table bounds the **local half** of the bilateral source (the package.json field set the installer honors). This table bounds the **remote half**: the npm registry's response schema, classifying which fields the first-cut installer consumes vs records vs ignores.

Together the two tables specify the first-cut alphabet completely. After PM-EXT 2, every directive the installer might encounter in either half of the bilateral source has a classification.

## Two endpoints, two response shapes

The first cut needs at most two registry endpoints:

**Endpoint A — packument**. `GET https://registry.npmjs.org/{pkg}` returns the full packument: every published version's metadata, plus top-level distribution tags, timestamps, and package-level metadata. Used when the installer needs to enumerate versions (range resolution, tag resolution). Heavy: a popular package's packument is megabytes.

**Endpoint B — per-version manifest**. `GET https://registry.npmjs.org/{pkg}/{version}` returns just the requested version's metadata. Light. The first cut prefers this endpoint because exact-pin resolution does not need the full version list.

The classification below covers both, since transitive deps may need range-or-tag resolution even when the top-level is exact-pinned.

## Class A — Honor in first cut

Cardinality: **4**. The fields the installer must read for the install to function.

| field | endpoint | semantics | install behavior |
|---|---|---|---|
| `versions.<v>` (or root, when endpoint B) | both | per-version manifest object | the unit of consumption; the installer reads the requested version's metadata and ignores the rest |
| `versions.<v>.dist.tarball` | both | URL to gzipped tar of the package contents | PM-R2's input: `fetch_tarball(url, integrity)` GETs this |
| `versions.<v>.dist.integrity` | both | SRI string (e.g. `sha512-<base64>`) | PM-R2's verification: the fetched tarball's hash must match. Preferred over `shasum` |
| `versions.<v>.dist.shasum` | both | hex SHA-1 of the tarball | fallback verification when `integrity` is absent (legacy packages published before SRI rollout, c. 2017). Cardinality is counted as part of the dist-pair, not as a separate Class-A field |

**Why exactly four.** The first cut consumes the registry response only to (a) identify the requested version's metadata block, (b) extract the tarball URL, and (c) verify the fetched bytes. Everything else in the registry response is either downstream-consumed (transitive dep resolution; see Class B), tag-or-range-resolution (deferred; Class C), or pure metadata (Class E).

## Class B — Honor with caveats (transitive resolution)

Cardinality: **1**. The single field whose semantics force the first cut to confront transitive-dep resolution.

| field | semantics | first-cut handling |
|---|---|---|
| `versions.<v>.dependencies` | object: `{ name: version-spec }` declared by the *published* package | the installer must resolve each entry against the registry to produce a complete `node_modules` tree. Published packages overwhelmingly use semver *ranges* (`^4.17.21`, `~1.0.0`, `>=2.0.0`), not exact pins. The first cut therefore needs a minimal range-resolver for **transitive** deps even though the user-supplied top-level `dependencies` must be exact-pinned per [manifest-field-coverage.md](./manifest-field-coverage.md) Class A |

**The transitive-range tension.** The user-facing constraint (top-level pins only) does not propagate downward. A user who pins `lodash@4.17.21` gets a package whose own `dependencies` may declare ranges; the installer must resolve them or fail. Three responses:

- **(i) First-cut narrow**: target only packages with zero transitive deps (lodash, ms, leven, ...). Demonstrates end-to-end install without confronting range resolution. The class of installable packages is small but real.
- **(ii) Greedy max-satisfying**: implement minimal semver-range parsing + a "pick the maximum published version that satisfies the range" resolver. The function is small (~50 LOC for caret/tilde/range subset); no SAT solver, no conflict reconciliation. This buys most of npm. Adds ~1 PM-EXT of complexity.
- **(iii) Defer to PM-EXT 4+**: stick with (i) for first cut; add (ii) as the second cut explicitly named in Doc 732 §VI.

**Decision (PM-EXT 2)**: go with **(iii)**. The first cut targets zero-transitive-dep packages — lodash (0 deps), ms (0), leven (0), kleur (0), picocolors (0), debug (1: ms), ansi-styles (0). The end-to-end demonstration in Doc 732 §VI fits inside this class. Range resolution becomes PM-EXT N+1 once the bilateral-source mechanics are landed.

This decision adds a fifth open question to inherit: see §"Open questions" below.

## Class C — Defer (second cut)

Cardinality: **8**. Fields the installer ignores in the first cut; presence does not affect behavior.

| field | reason for deferral |
|---|---|
| `dist-tags` (`latest`, `next`, `beta`, ...) | tag-resolution is the second face of range-resolution; deferred with it |
| `versions.<v>.devDependencies` | not installed transitively per npm semantics; ignored anyway |
| `versions.<v>.peerDependencies` | peer reconciliation is a graph-walk; deferred per [manifest-field-coverage.md](./manifest-field-coverage.md) Class C |
| `versions.<v>.peerDependenciesMeta` | with `peerDependencies` |
| `versions.<v>.optionalDependencies` | range resolution + failure tolerance; both deferred |
| `versions.<v>.bundleDependencies` / `bundledDependencies` | publish-time bundling; rejected at top level per [manifest-field-coverage.md](./manifest-field-coverage.md) Class D, ignored when seen transitively (the bundled subtree just ships in the tarball as is) |
| `versions.<v>.engines` / `os` / `cpu` | host constraints; warn-only cut |
| `versions.<v>.deprecated` | string; emit a warning if non-empty, install proceeds. Deferred-to-warn until the warn channel is wired |

## Class D — Reject in first cut

Cardinality: **2**. Fields whose presence in registry-fetched metadata triggers an error.

| field | reason for rejection |
|---|---|
| `versions.<v>.scripts.{preinstall,install,postinstall,prepare}` (any present) | same rationale as [manifest-field-coverage.md](./manifest-field-coverage.md) Class D: silent skip risks broken-but-undetected installs (native extensions, generated files). The installer aborts with `PmError::LifecycleScriptsPresent { pkg, scripts: [..] }`. Override flag `--ignore-scripts` (later cut) |
| `versions.<v>.dist.signatures` mismatch | when present, signature verification is performed; mismatch aborts. When absent, no enforcement (most published packages still ship unsigned as of 2026). This is "reject when present and invalid"; absent presence is Class C-equivalent |

## Class E — Pure metadata (recorded for audit)

Cardinality: **~12**. Recorded into the lockfile's per-package entry per Doc 732 §V auditability. The installer does not consume.

| field | type | placement |
|---|---|---|
| `versions.<v>.name` | string | validated equals requested name; otherwise warn (registry mismatch) |
| `versions.<v>.version` | string | validated equals requested version; otherwise abort |
| `versions.<v>.dist.fileCount` | int | provenance |
| `versions.<v>.dist.unpackedSize` | int | provenance; sanity-check after extraction |
| `versions.<v>._npmUser` | object `{ name, email }` | provenance: who published this version |
| `versions.<v>._nodeVersion`, `_npmVersion` | string | provenance |
| `time.<v>` (top-level) | ISO-8601 | publish timestamp; recorded for time-travel audits |
| `versions.<v>.description`, `keywords`, `homepage`, `bugs`, `license`, `author`, `contributors`, `repository`, `funding`, `man`, `types` | various | metadata mirror per [manifest-field-coverage.md](./manifest-field-coverage.md) Class E |

## Class F — Wildcat (preserved-by-virtue-of-being-in-the-tarball)

The same wildcat class as the manifest table: tool-specific config blocks (`eslintConfig`, `babel`, `jest`, etc.) ship inside the tarball's `package.json`. The installer does not read them from the registry response separately; they arrive automatically when the tarball is extracted.

## Class G — Registry-internal (ignored entirely)

Cardinality: **~10**. Fields the registry adds for its own bookkeeping; the installer ignores.

| field | example |
|---|---|
| top-level `_id`, `_rev`, `_attachments` | CouchDB internals |
| `users` | star counts |
| `readme`, `readmeFilename` | display-only |
| `maintainers` (top-level, distinct from per-version) | publish-side ACL |
| `versions.<v>._id`, `_shasum`, `_from`, `_resolved`, `_hasShrinkwrap`, `_integrity`, `gitHead` | publish-tool internals |

These fields' presence is benign; absence is also benign. The installer's parser tolerates them.

## Summary table

| class | cardinality | first-cut behavior |
|---|---|---|
| A — honor | 4 | consume directly to fetch + verify |
| B — honor with caveats | 1 (`dependencies`) | force the transitive-range question; first cut narrows to zero-dep packages |
| C — defer | 8 | ignore in first cut |
| D — reject | 2 | error on presence |
| E — metadata | ~12 | record into lockfile |
| F — wildcat | open | preserved in extracted tarball |
| G — registry-internal | ~10 | ignored |

**Cardinality of Class A is the first-cut remote-half alphabet bound: 4.** Combined with the local-half bound of **3** from [manifest-field-coverage.md](./manifest-field-coverage.md), the **full bilateral-source first-cut alphabet is 7 directives** (`name`, `version`, `dependencies` from package.json; `versions.<v>`, `dist.tarball`, `dist.integrity`, `dist.shasum` from the registry).

Seven directives is the surface the first-cut installer's bootstrap-property obligations are bounded over. Doc 732 §V's claim — that the lockfile collapses the bilateral source by memoizing this finite slice — is now operational: the lockfile's per-dep entry must record the resolution of all seven, and any subsequent install reproduces the closure exactly when the lockfile is present.

For comparison, npm's full registry-response surface (per the registry-API spec) is on the order of 50+ fields. The first cut consumes 7. The ratio (~7/50 ≈ 14%) is the empirical version of Doc 732 §V's reproducibility claim at the remote-half tier: most of the registry's response is recorded-for-audit or ignored-as-internal, not consumed at install time.

## Open questions surfaced by this classification

5. **(adds to PM-EXT 1's four)** Transitive-dep range resolution. First-cut narrow vs greedy-max-satisfying — see Class B. Decision recorded: first-cut narrow (zero-transitive-dep packages only); range resolver is PM-EXT N+1.

6. **Endpoint choice (A vs B).** The packument (endpoint A) is needed when range or tag resolution is required; the per-version manifest (endpoint B) is sufficient when the version is exact-pinned. First cut uses endpoint B exclusively (consistent with the zero-transitive narrow). Range/tag work in the second cut will need endpoint A; the fetcher should be designed to handle both shapes from day one.

7. **`integrity` vs `shasum` precedence.** Both ship in modern registry responses. Decision: prefer `integrity`; fall back to `shasum` if absent; emit a structured log line when `shasum`-only is used so an audit can identify legacy-only packages. SHA-1 is broken; the fallback is for backward compatibility, not security.

8. **Time-travel installs.** The packument carries publish timestamps; the lockfile records the URL and hash but not the publish time. Recording publish time would enable a "what was the state of this dep on date D" query without a registry roundtrip. First-cut decision: record `published_at` in the lockfile's `meta` block (no install behavior change; cheap auditability win).

## Closes

PM-EXT 2 closes with the bilateral source fully bounded. Local half: 3 directives. Remote half: 4 directives + 1 caveat. Total first-cut alphabet: **7 directives** (+ 1 transitive-range tension, decided to narrow rather than absorb in first cut).

Next substrate move: PM-EXT 3 — crate scaffold (`pilots/rusty-js-pm/derived/Cargo.toml` + smoke test). Equivalent to JIT-EXT 2's Cranelift scaffold: verify the toolchain stack (HTTP client, gunzip, untar, sha2) works on the engagement's target platform before any substrate translation lands. Same shape, same cadence.
