# rusty-js-caps — Trajectory

Chronological resume anchors for the capability-passing-runtime workstream. Reads seed.md first; this file is the time-ordered record of substrate moves and their yields.

Format: one section per "CAPS-EXT" (extension round); each round closes with a status block, a cumulative numbers table, and an open-scope list. Same shape as `pilots/rusty-js-pm/trajectory.md` and `pilots/rusty-js-jit/trajectory.md`.

## CAPS-EXT 0 — 2026-05-21 (workstream founding)

### Headline

Workstream founded immediately after PM-EXT 13 close + Doc 736 publication. The dominant pilot for the "supply-chain-attack architecturally impossible" design per Doc 736 §VI Pilot α.

No code committed. This round establishes the workstream's scaffolding: seed.md + trajectory.md per the Doc 581 + Doc 733 fractal-pair shape; design target = Doc 736 §III Move 1 + §VI; §XVI oracle = synthetic-adversary probe suite (no in-ecosystem reference engine; comparison points Deno / SES / CapTP / SecurityManager documented in Doc 736 §V but not co-evaluable).

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | (workstream founding) | `pilots/rusty-js-caps/seed.md` + `trajectory.md` written. Doc 736 §VI scope is the design target. Pin-Art tag prefix `Ω.5.P05.L2.caps-*`. |

### Substrate at CAPS-EXT 0 close

- **No caps code committed.** Seed and trajectory only.
- **Ambient-authority audit**: not yet produced (queued as CAPS-EXT 1's first substrate move per seed §V).
- **Capability-API design doc**: not yet produced (CAPS-EXT 2).
- **Synthetic-adversary probe harness**: not yet produced (CAPS-EXT 3).
- **Crate scaffold**: intentionally not created. Substrate work lands inside existing crates (rusty-js-runtime, host-v2, rusty-js-pm); pilot directory holds discipline artifacts only per seed §V.

### Conjecture status

Doc 736's Pred-736.1 through Pred-736.5 are structural claims with no engagement-tier corroboration. CAPS-EXT 0 founds the workstream that will provide corroboration (or surface the falsifications: §VI LOC estimates wrong by >2×, retrofit-vs-rewrite threshold breached, probe-harness gap, etc.).

### Open scope at CAPS-EXT 0 boundary

1. **CAPS-EXT 1 (ambient-authority audit)**: produce `pilots/rusty-js-caps/docs/ambient-authority-audit.md`. Walk every effectful surface currently reachable from a loaded module. Sources: `rusty-js-runtime/derived/src/intrinsics.rs`, `host-v2/src/{lib,node_stubs,fs,crypto,os,http}.rs`. Output: per-method table classifying as **pure** (no observable side effect outside argument graph) / **effectful** (observable I/O or process state mutation) / **mixed** (both forms reachable from one entry point). The cardinality of "effectful" is the substrate-move budget for Move 1. Reading + classifying; no Cargo work required.

2. **CAPS-EXT 2 (capability-API design)**: `pilots/rusty-js-caps/docs/capability-api.md`. Structured surface: `Fs`, `Net`, `Process`, `Env`, `Time`, `Exec` types; each with its method set; constructors only at the host boundary; deputation/restriction operators; `CapabilityError` for missing-handle calls. This is the contract that Move 1's substrate moves implement.

3. **CAPS-EXT 3 (synthetic-adversary probe harness)**: `pilots/rusty-js-caps/probes/` directory with `.mjs` files representing the attack classes Doc 736 §IV catalogs. Each probe = a small dep + an application file. Pre-Move-1 state: every probe FAILS (the malicious dep succeeds in its attack). Move 1's success metric: flip every probe to PASS (attack refused with `CapabilityError`).

4. **CAPS-EXT 4-N (Move 1 substrate work)**: bisect the effectful surface in priority order. Likely first targets: `fs.readFileSync` (probe: read /etc/passwd), `http.request` (probe: exfil), `process.env` (probe: read AWS creds), `child_process.spawn` (probe: shell exec). Each EXT round refactors one or two surfaces and lands the corresponding probe flip.

5. **Verifier moves (CAPS-EXT M+1 onward)**: Move 3 (load-time SRI re-verifier, ~80 LOC) is the cheapest no-regret win; could land in parallel with the Move 1 bisection. Move 2 (caps schema, ~150 LOC), Move 4 (closed-import-graph compile check, ~120 LOC), Move 5 (publisher pinning, ~100 LOC) come after Move 1's first-cut closure.

### Resume protocol

Read seed.md, then this CAPS-EXT 0 entry. The next substrate move is the ambient-authority audit at CAPS-EXT 1. No Cargo work; the move is reading + classifying.

Pin-Art tag count: 0 substrate moves so far (workstream founding only).

---

*CAPS-EXT 0 closes the founding round. Subsequent rounds add substrate moves at the runtime tier and the PM tier per the Doc 736 §VI Pilot α + Moves 2-5 decomposition.*
