# rusty-js-pm — Trajectory

Chronological resume anchors for the package-manager workstream. Reads seed.md first; this file is the time-ordered record of substrate moves and their yields.

Format: one section per "PM-EXT" (extension round); each round closes with a status block, a cumulative numbers table, and an open-scope list. Same shape as `pilots/rusty-js-jit/trajectory.md` and `pilots/rusty-js-ir/trajectory.md`.

## PM-EXT 0 — 2026-05-21 (workstream founding)

### Headline

Workstream founded immediately after JIT-EXT 9 close, against the published Doc 732 articulation. Preconditions are met: module loader (resolver-instance #3 in the Doc 729 stack) is operational and walks the `node_modules` tree this pilot's artifact would produce; host-v2 has HTTP + filesystem APIs the install will consume; the bytecode interpreter + JIT cover the runtime tier the installed packages will execute against.

No PM code yet. This round establishes the workstream's scaffolding: seed.md + trajectory.md per the Doc 581 shape, design target = Doc 732 §VI first-cut scope, §XVI oracle = `bun install`.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | (workstream founding) | `pilots/rusty-js-pm/seed.md` + `trajectory.md` written. Doc 732 §VI scope is the design target; Doc 730 §XVI methodology applies as gating discipline. Pin-Art tag prefix `Ω.5.P05.L1.pm-*` (install-time, layered with module loader pipeline). |

### Substrate at PM-EXT 0 close

- **No PM code committed.** Seed and trajectory only.
- **Crate scaffold**: not yet created.
- **Manifest field-coverage table**: not yet produced (queued as the first substrate move per seed §VI).
- **Registry response schema table**: not yet produced.
- **Tag-grammar question open**: whether to extend `dag-coordinates.json` with a new pipeline id (P17 = Package install pipeline) or stay with P05 (Module loader pipeline) as the composition site. First-cut decision: stay with P05.L1; revisit if the install accumulates moves that the module loader pipeline does not naturally contain.

### Conjecture status

Doc 732's Pred-732.1 through Pred-732.5 are structural claims with no engagement-tier corroboration. PM-EXT 0 founds the workstream that will provide corroboration (or surface the failures: lifecycle scripts as the §VIII Pred-732.1 candidate failure mode).

### Open scope at PM-EXT 0 boundary

1. **First substrate move (PM-EXT 1)**: produce `pilots/rusty-js-pm/docs/manifest-field-coverage.md`. Walk the package.json spec + top-100-npm-package union. Classify per seed §VI's four-bucket scheme: honor / honor-with-caveats / defer / reject. Cardinality of "honor" is the first-cut alphabet upper bound. This is reading + classifying; no Cargo work required.

2. **Companion table (PM-EXT 1 or 2)**: `pilots/rusty-js-pm/docs/registry-response-schema.md`. Walk the npm registry's per-package response (`https://registry.npmjs.org/{name}`) + per-version metadata. Classify which fields the install consumes vs ignores. Cardinality is the consumed-surface bound for the bilateral-source's remote half.

3. **Crate scaffold (PM-EXT 2 or 3)**: `pilots/rusty-js-pm/derived/Cargo.toml` + `lib.rs`. Workspace member added to root Cargo.toml. Smoke test: HTTP GET against registry.npmjs.org succeeds; tarball gunzips; tar extracts. Equivalent to JIT-EXT 2's Cranelift scaffold + smoke test: verify the toolchain stack works on the engagement's target platform before any substrate translation.

4. **PM-R1 first cut (PM-EXT 3 or 4)**: parse package.json (exact-pin subset), resolve each dep against the registry, emit lockfile structure in-memory.

5. **PM-R2 + R3 first cut (PM-EXT 5 or 6)**: fetch + extract + flat link. End-to-end against the lodash fixture.

6. **Wire to host-v2 CLI (PM-EXT 7)**: `cruftless install` subcommand. End-to-end test: `cruftless install && cruftless -e "require('lodash').identity(42)"` returns 42 from a fresh tmpdir.

7. **Lockfile read path (PM-EXT 8)**: subsequent install reads the lockfile; no-op if every dep present and matches; refetch what is missing.

### Resume protocol

Read seed.md, then this trajectory's PM-EXT 0 entry. The next substrate move is the manifest field-coverage table; no Cargo work needed for that move. The classification is reading + thinking, not implementation. Per the JIT precedent (EXT 1 = P4 enumeration before any Cranelift code), the alphabet must be bounded before the substrate that operates over it lands.

Pin-Art tag count: 0 substrate moves so far (workstream founding only).

---

*PM-EXT 0 closes the founding round. Subsequent rounds add substrate moves at the package-install tier.*

---

## PM-EXT 5 — 2026-05-21 (PM-R1 specifier resolver against npmmirror.com via engagement-internal TLS)

### Headline

First PM-pilot substrate move since PM-EXT 4 (commit 4d7115a2). PM-EXT 0–4 wired the HTTP path; the gap was the engagement-internal-TLS CDN-incompatibility. Session 1's TLS + web-crypto work (TLS-EXT 0–10, WC-EXT 0–26) closed the CDN-passable target at 3/5 endpoints. **PM-EXT 5 lands PM-R1 on top: specifier resolver against an npm-protocol-compatible registry.**

### Substrate landed

- `pilots/rusty-js-pm/derived/src/resolver.rs` — `resolve_specifier(registry, name, version) -> Result<ResolvedDep, ResolverError>`. Exact-pin-only enforcement (caret/tilde/range syntax rejected per Doc 732 §VI carve-out + manifest-field-coverage.md Class A).
- `ResolvedDep { name, version, tarball_url, integrity, shasum }` — the lockfile-tier record for one resolved package.
- `DEFAULT_REGISTRY = "https://registry.npmmirror.com"` — npm-protocol-compatible mirror, TLS 1.3 reachable through engagement substrate.

### Doc 730 §XVI Case-4 endpoint-substitution scope decision applied

The PM-EXT 4 5-endpoint probe established that:
- `registry.npmjs.org` is TLS 1.2 only (Cloudflare edge policy)
- `registry.yarnpkg.com` same (also behind Cloudflare TLS 1.2 only)
- **`registry.npmmirror.com` works** through engagement TLS 1.3 substrate (returns the per-version manifest JSON in <500ms)
- `npm.pkg.github.com` requires auth (403 without token)

Per Doc 730 §XVI Case 4 (implementation freedom at the scope tier): the npm protocol is the same; the endpoint is substitutable. The PM-EXT 5 substrate move treats `registry.npmmirror.com` as the default registry; users can override via the `registry` parameter to `resolve_specifier`. The TLS 1.2 carve-out lift (which would open registry.npmjs.org) remains queued; the PM-pilot proceeds without it.

### Measurement (PM-EXT 5 network test)

```
resolve_specifier("https://registry.npmmirror.com", "lodash", "4.17.21")
  → ResolvedDep { name: "lodash", version: "4.17.21",
                  tarball_url: ".../lodash-4.17.21.tgz",
                  integrity: Some(...), shasum: Some(...) }
  Time: 420 ms (TLS handshake + GET + JSON parse)
```

The 420 ms breakdown is dominated by the TLS handshake (which the WC-EXT 25-26 substrate work brought to ~250-400 ms for non-CloudFront/Fastly CDNs).

### What this completes

The PM workstream's first cut from PM-EXT 0 → 5 forms one complete cycle of Doc 734 §II's meta-pipeline:
- Step 1 observation: PM cannot reach npm
- Step 2 articulate: Doc 732 (PM as resolver-instance #0)
- Step 3 fractal pair: PM workstream founded
- Steps 4–6 substrate: PM-EXT 1–3 (manifest coverage, registry schema, crate scaffold)
- Step 7 recurse: PM-EXT 4 surfaced TLS gap → TLS pilot founded → WC pilot founded → 27 EXTs of upstream substrate work
- Step 9 probe-cell flip: TLS 0/5 → 3/5 PASS opens the PM's PM-R1 path
- **PM-EXT 5: the original target is now reachable**

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P05.L1.pm-r1-resolver` | PM-R1 specifier resolver landed; resolves against npmmirror.com via engagement-internal TLS+web-crypto substrate; first PM-pilot substrate move post-cross-pilot-detour |

### Probe result

- Unit tests: 2/2 PASS (caret + tilde rejection)
- Network test: 1/1 PASS (lodash 4.17.21 resolves cleanly)
- 117 web-crypto regression: unchanged PASS
- 5-endpoint TLS probe: 3/5 unchanged

### Open scope at PM-EXT 5 boundary

1. **PM-EXT 6 (PM-R2 fetcher/extractor)**: GET tarball URL via pm_http_get, verify integrity via SRI/shasum, untar to a staging dir. ~80 LOC. The substrate already has flate2 + tar + sha2 + sha1 (PM-EXT 3 scaffold).
2. **PM-EXT 7 (PM-R3 linker)**: move staging tarball contents into `node_modules/<pkg>/`. ~40 LOC.
3. **PM-EXT 8 (lockfile codec)**: serialize ResolvedDep list to JSON; deserialize for subsequent installs. ~50 LOC.
4. **PM-EXT 9 (end-to-end test)**: `pm_install` against a `package.json` with `{ "dependencies": { "lodash": "4.17.21" } }` in a tmpdir; verify `node_modules/lodash/package.json` exists; load and use via cruftless runtime.
5. Independent: TLS-EXT 9 (E2 httpbin bug), connection pooling.

The PM workstream's path to first-cut closure is now bounded: PM-EXT 6 + 7 + 8 + 9 = ~200 LOC for end-to-end install of an exact-pinned zero-transitive-dep package. Within session 2 reach.

---

*PM-EXT 5 closes the cross-pilot detour that started at PM-EXT 4. The PM workstream's original target — engagement-internal HTTPS to a registry — is reached; the substrate cascade (TLS-EXT 0–10 + WC-EXT 0–26) propagates upward into a working specifier resolver. Doc 734's meta-pipeline cycle 1 is complete; PM-EXT 6+ extends the workstream into the fetcher/extractor + linker + lockfile layers.*
