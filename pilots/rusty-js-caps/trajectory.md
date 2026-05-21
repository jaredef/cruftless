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

---

## CAPS-EXT 2 — 2026-05-21 (capability-API design)

### Headline

Capability-API design document landed. Names the six capability types (Fs/Stdio/Clock/Scheduler/Process/Env — six, not Doc 736's five, because the audit found Clock and Scheduler operationally separable), the dispatcher protocol, the mode-aware enforcement contract per Doc 736 §IX, CapabilityError shape, JS-side handle semantics, the `require(spec, opts?)` extension, and the seven-step implementation order CAPS-EXT 3 onward.

### Substrate landed

- `pilots/rusty-js-caps/docs/capability-api.md` (~340 lines):
  - §I dispatcher protocol (Rust-side `require_capability` + JS-side capability handle threading)
  - §II-§III six capability types with policy enums + deputation operators
  - §IV CapabilityError shape (hint field is ergonomically load-bearing: error tells dev exactly what declaration unblocks the call)
  - §V mode-aware dispatcher (CapMode = Compat / Audit / SealedDeps / Sealed)
  - §VI Capability trait
  - §VII module provenance (Application vs Dependency vs External, established at load time)
  - §VIII host's ambient_caps() construction
  - §IX JS-side handle guarantees (cannot be constructed by JS, deputable but not broadenable, opaque under serialization, reference-equal across passes)
  - §X require(spec, opts?) API extension with ESM `with { caps }` form
  - §XI backward compat (Mode 0 default; PM-EXT 11/12/13 gates remain green)
  - §XII deferred features (async capability passing, cross-realm, revocation, persistent storage)
  - §XIII implementation order (CAPS-EXT 3-13)

### Key design decisions

1. **Single dispatcher, no bypass paths**. Every effectful method's Rust implementation begins with one `require_capability` call. Mode 0 makes the call a no-op (returns ambient). Mode 3 makes it the gate. Adding a new mode = adding a match arm; adding a new capability = implementing one trait. No effectful method bypasses the dispatcher.

2. **Six capability types (not five)**. CAPS-EXT 1 audit showed Clock and Scheduler are operationally separable. A dep can need timer scheduling without clock-read access (it sets up the timer but never reads time). A dep can need clock-read without scheduling (cache expiry check). Splitting them costs nothing and produces sharper capability declarations.

3. **PathPolicy as closed enum**. Fs read/write/list/stat/mkdir/remove each get their own PathPolicy (None/Any/Prefix/Prefixes/Exact). The enum is closed so future variants (glob, regex, content-type-aware) can be added without breaking existing constructions. Coarse-grained in first cut; refinable later.

4. **CapabilityError.hint**. The error tells the developer exactly what to add to package.json caps to unblock the call. Combined with Mode 1 audit log, this means a dev hitting CapabilityError can either accept the audit log's suggestion or hand-edit the hint into their manifest. No spelunking through dep source.

5. **JS-side handles use Symbol.for('cruftless.cap') as identity slot, with sealed Rust pointer behind it**. A dep that tries to forge a capability by constructing an object with that symbol fails: the host reads the underlying Rust pointer through a sealed slot, not the JS-visible field.

6. **require(spec, opts?) extension records intent in Mode 0/1**. In compat/audit modes the `opts.caps` field doesn't change behavior but the dispatcher records what the application *intended* to pass. The audit log thereby captures both "what the dep called" and "what the application passed", letting CI surface the gap between intended caps and actual usage.

### Implementation order (queued)

- **CAPS-EXT 3 (next)**: capability infrastructure with Mode 0 wiring. Introduces `Capability` trait, `CapDispatcher`, `CapMode` enum, six capability types, host's `ambient_caps()`. Routes every check to ambient. **No observable behavior change.** PM-EXT 11/12/13 gates remain green.
- **CAPS-EXT 4**: audit recorder + sidecar log + `--audit` flag.
- **CAPS-EXT 5**: synthetic-adversary probe harness (Mode 0 baseline; every probe SUCCEEDS = attacker wins as pre-state measurement).
- **CAPS-EXT 6-7**: Fs capability enforcement under Mode 3 + `--sealed` flag.
- **CAPS-EXT 8**: Mode 2 wiring (`--sealed-deps`).
- **CAPS-EXT 9-12**: remaining capability classes (Process.exit, Stdio, Clock, Scheduler, Env).
- **CAPS-EXT 13**: closure — full probe suite under Mode 3 with empty caps; every probe PASSES.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P05.L2.caps-api-design` | CAPS-EXT 2: capability-API design contract; six capability types; mode-aware dispatcher; CapabilityError.hint; implementation order CAPS-EXT 3-13 |

### Probe status

No code yet — design only. The probe harness arrives at CAPS-EXT 5. The contract becomes falsifiable when CAPS-EXT 3 wires the dispatcher in Mode 0: PM-EXT gates must remain green.

### Open scope at CAPS-EXT 2 boundary

1. **CAPS-EXT 3**: capability infrastructure in Mode 0. This is the first code commit of Pilot α. Estimated ~400 LOC across rusty-js-runtime (dispatcher + traits + types) + host-v2 (ambient_caps + install). Behavior unchanged.

2. The remaining open scope from CAPS-EXT 1 carries forward unchanged.

---

*CAPS-EXT 2 closes the design tier. CAPS-EXT 3 begins the code tier. The dispatcher exists from CAPS-EXT 3 forward; enforcement lands incrementally; the impossibility claim is reachable at CAPS-EXT 13.*
