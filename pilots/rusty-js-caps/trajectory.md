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

---

## CAPS-EXT 1 — 2026-05-21 (ambient-authority audit)

### Headline

Static walk of host-v2 + rusty-js-runtime produced a per-method classification table covering ~625 JS-callable surfaces. **Only ~40 currently-callable effectful methods** (6% of the enumerated total); 130 are throw-on-call stubs (21%); ~450 are pure (72%); ~5 mixed (1%). The Move 1 gating budget is much smaller than Doc 736 §VI estimated — revised LOC estimate ~1100, well under the original 2-3k.

### Substrate landed

- `pilots/rusty-js-caps/docs/ambient-authority-audit.md` (~280 lines).
  Per-namespace classification table, mixed-surface splitting recommendations, absent-surface inventory, distilled Move 1 gating budget (~40 methods), substrate-move ordering for CAPS-EXT 4+.

### Key findings

1. **Most "dangerous" Node surface is unreachable in cruftless today**:
   - `node:child_process` (all RCE vectors) → all stubs
   - `node:net` / `node:dgram` → not exposed
   - `node:http` / `node:https` / `node:dns` / `node:vm` → all stubs
   - `crypto.randomBytes` / WebCrypto → stubs
   - `fs` fd surface (open/close/read/write) → not exposed
   - `fs` symlink ops → not exposed

   The substrate cascade hasn't built these yet. We gate what exists; we document the absent surface; we require future implementations to land behind capability gates from the start.

2. **The 40 currently-callable effectful surfaces cluster into 5 capability classes**:
   - `Fs`: ~17 methods (sync + async fs ops, plus require's filesystem walk)
   - `Stdio`: ~7 methods (process.stdout.write/stderr.write, console.log/error/warn/info/debug)
   - `Clock` + `Scheduler`: ~7 methods (Date.now, hrtime, performance.now, setTimeout/setInterval/setImmediate, nextTick, queueMicrotask)
   - `Process`: ~3 methods (exit, cwd, pid)
   - `Env`: ~5 methods (os.hostname/homedir/tmpdir/userInfo, os.cpus)

3. **Revised LOC estimate (Pred-736.3 corroboration in progress)**:
   - Gating: ~600 LOC
   - Capability constructors/deputation/restriction: ~300 LOC
   - install_bun_host rework: ~200 LOC
   - **Total: ~1100 LOC**, below Doc 736 §VI's 2-3k.

   Pred-736.1 (retrofit, not rewrite) appears corroborated at the design tier; final corroboration awaits CAPS-EXT 4+ implementation.

### Substrate-move ordering (queued)

Priority by attack-severity × implementation-cost:

1. CAPS-EXT 4-5: `Fs` capability (largest surface, highest exfil risk, biggest single win — 2 rounds because sync + async have separate dispatch sites)
2. CAPS-EXT 6: `process.exit` gating (trivial; one method; high severity)
3. CAPS-EXT 7: `Stdio` capability (observable side channel via stdout; ergonomically tricky because console.log is everywhere)
4. CAPS-EXT 8-9: `Clock` + `Scheduler` (timing side channels)
5. CAPS-EXT 10: `Env` (low severity, completes the model)

Move 3 (load-time SRI re-verifier, ~80 LOC) can land in parallel with any of the above as a no-regret hardening.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P05.L2.caps-audit` | CAPS-EXT 1 ambient-authority audit; 625 methods enumerated; ~40 effectful methods identified as Move 1 gating budget; revised LOC estimate ~1100 |

### Probe status

No code yet — audit only. The synthetic-adversary probe harness arrives at CAPS-EXT 3. The audit's gating-budget recommendation becomes a falsifiable claim at CAPS-EXT 4+ when implementation begins.

### Open scope at CAPS-EXT 1 boundary

1. **CAPS-EXT 2 (capability-API design)**: `pilots/rusty-js-caps/docs/capability-api.md`. Structured types (`Fs`, `Stdio`, `Clock`, `Scheduler`, `Process`, `Env`), constructors at the host boundary only, deputation/restriction operators (`Fs.subDir(path)`, `Net.allowHost(name)`), CapabilityError shape.

2. **CAPS-EXT 3 (synthetic-adversary probes)**: `pilots/rusty-js-caps/probes/`. One `.mjs` per attack class catalogued in Doc 736 §IV. Pre-Move-1 state: all probes succeed (attacker wins). Move 1 success metric: every probe flips to PASS (CapabilityError refused).

3. **CAPS-EXT 1.1 (deeper intrinsics walk, optional)**: per-method classification of `rusty-js-runtime/derived/src/intrinsics.rs`. Currently classified at namespace level (Math/JSON/Promise/etc. ~all pure). Defer unless CAPS-EXT 4+ surfaces an effectful intrinsic that the namespace-level classification missed.

### Doc 730 §XVI status

The audit's "absent surface" finding is a Case-3 (both-diverge) compositional success at the design tier: cruftless's substrate didn't build the dangerous Node surfaces in the first place, so they don't need explicit refusal. This is the kind of finding that justifies engagement-internal substrate construction over adaptation: the absences were earned by the discipline.

---

*CAPS-EXT 1 closes the audit. The Move 1 gating budget is bounded at ~40 methods / ~1100 LOC. Next move CAPS-EXT 2: capability-API design.*
