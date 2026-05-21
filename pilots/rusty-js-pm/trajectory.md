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
