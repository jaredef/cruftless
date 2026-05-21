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

---

## CAPS-EXT 3 — 2026-05-21 (capability infrastructure, Mode 0 wiring)

### Headline

**First code commit of Pilot α.** The dispatcher exists; six capability types implemented; four modes (Compat/Audit/SealedDeps/Sealed) gated correctly; **PM-EXT 11+12 regression gates remain GREEN** at 2.90 s — Mode 0 no-op behavior confirmed.

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/caps.rs` (~520 LOC):
  - `CapMode` enum (Compat/Audit/SealedDeps/Sealed) + `from_str`/`as_str`
  - `ModuleProvenance` (Application/Dependency/External/Builtin) + `ModuleId`
  - `CapabilityError` with `hint` field + Display impl
  - **Fs** with PathPolicy (None/Any/Prefix/Prefixes/Exact) + 6-op FsOp enum + `full()`/`none()`/`sub_dir()`/`read_only()` deputation
  - **Stdio** + StdioOp + `full()`/`none()`
  - **Clock** with ClockResolution (Disabled/Coarse/Fine) + ClockOp + `fine()`/`coarse()`/`disabled()`
  - **Scheduler** with timers/microtasks/min_delay + SchedulerOp
  - **Process** with may_exit/may_read_cwd/may_read_pid + ProcessOp
  - **Env** with EnvVarPolicy (None/Any/Whitelist) + system_info bool + EnvOp
  - `AmbientCaps::full()` — the host's root capability set
  - `AuditLog` + `AuditRecord` for Mode 1
  - `CapDispatcher` with `require_fs`/`stdio`/`clock`/`scheduler`/`process`/`env` methods, each enforcing mode-aware policy with hint generation
- `pilots/rusty-js-runtime/derived/src/lib.rs`: `pub mod caps;`

### Probe result

**Unit tests (15/15 PASS):**
- `mode_default_is_compat` — Mode 0 default
- `cap_mode_parse` — string ↔ enum round-trip
- `compat_mode_allows_everything_no_cap` — Mode 0 ignores capabilities (returns ambient unconditionally)
- `audit_mode_allows_and_records` — Mode 1 allows + logs to AuditLog
- `sealed_deps_dep_blocked` — Mode 2 blocks deps without capability
- `sealed_deps_app_passes` — Mode 2 ambient-for-application invariant
- `sealed_blocks_app_too` — Mode 3 sealed-everywhere invariant
- `fs_prefix_policy` — PathPolicy::Prefix matching
- `fs_sub_dir_narrows` — Fs.sub_dir() deputation
- `fs_read_only_strips_writes` — Fs.read_only() deputation
- `stdio_sealed_blocks_unless_granted` — Stdio bool gating
- `clock_disabled_blocks` — Clock::Disabled refuses; Clock::Fine allows
- `process_exit_gated` — process.exit refused without may_exit
- `env_whitelist` — EnvVarPolicy::Whitelist filters
- `capability_error_display` — error message includes capability/op/mode/hint

**Mode-0 regression (PM-EXT 11+12, 2/2 PASS in 2.90 s):**
- `pm_install_then_require_lodash` (PM-EXT 11 closure gate) — green
- `cli_install_then_run` (PM-EXT 12 CLI gate) — green

The dispatcher is **wired into the build** (lib.rs `pub mod caps;`) but **no effectful method calls it yet**. This is by design per the capability-API §XI backward-compatibility contract: the infrastructure ships in Mode 0 with PM gates green; route-through lands at CAPS-EXT 6+.

### Pred-736 corroboration status

- **Pred-736.1 (retrofit, not rewrite)**: corroborated at this scale. ~520 LOC for the entire capability infrastructure with full test coverage. No edit to any existing runtime file beyond a one-line `pub mod caps;`. The retrofit threshold is comfortably below the 30% rewrite-of-rusty-js-runtime tolerance set in the seed §VI.
- **Pred-736.3 (LOC estimate ~1100)**: on track. CAPS-EXT 3 spent ~520 LOC on infrastructure; remaining budget ~580 LOC for the call-site refactor across the 40 effectful methods.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P05.L2.caps-mode0-infra` | CAPS-EXT 3: capability infrastructure with Mode 0 wiring; 15/15 caps unit tests PASS; PM-EXT 11+12 regression GREEN; dispatcher exists, behavior unchanged |

### Open scope at CAPS-EXT 3 boundary

1. **CAPS-EXT 4 (audit recorder)**: dispatcher already records under Mode 1; need to wire `--audit` CLI flag in host-v2 and surface the audit log on shutdown (write to sidecar file or stderr). Without this, Mode 1 records to memory only and the log is lost on exit. ~50 LOC.

2. **CAPS-EXT 5 (synthetic-adversary probe harness)**: `pilots/rusty-js-caps/probes/` with `.mjs` files representing each attack class. Under Mode 0 every probe SUCCEEDS = the pre-state baseline. The harness becomes the standing regression for the rest of Pilot α.

3. **CAPS-EXT 6-7 (Fs route-through under Mode 3)**: edit host-v2's `fs.rs` to route every fs method through `dispatcher.require_fs(...)`. Add `--sealed` CLI flag. Add `cruftless-caps.json` parser. Confirm: under Mode 3 with empty caps, fs probes flip from SUCCESS to PASS (CapabilityError).

4. **CAPS-EXT 8+**: Mode 2 dispatcher branching on module provenance, then remaining capability classes (Process.exit, Stdio, Clock, Scheduler, Env).

### Doc 730 §XVI status

The dispatcher's existence with PM gates green is a Case-3 (both-diverge → compositional success): cruftless and Node diverge structurally (cruftless has a dispatcher; Node does not), and the composition succeeds (PM-EXT 11+12 still green). The structural divergence is the entire point of the workstream; the compositional success is the engineering check that the divergence doesn't regress anything.

---

*CAPS-EXT 3 closes the first code commit of Pilot α. Infrastructure shipped; behavior unchanged; PM-EXT regression green; ready to begin route-through at CAPS-EXT 4+ once the audit recorder surfaces the audit log.*

---

## CAPS-EXT 4 — 2026-05-21 (audit recorder + --audit / --sealed CLI flags)

### Headline

CapDispatcher attached to Runtime; `--audit`, `--sealed-deps`, `--sealed`, `--audit-log` CLI flags wired through host-v2; `CRUFTLESS_CAPS_MODE` env var override. The four modes are now invocable end-to-end. **PM-EXT 11+12 regression remains GREEN (2.79 s)**; new caps_audit tests pass for `--audit`, `--sealed`, and env-var-mode invocations against no-effectful-call programs.

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/interp.rs`:
  - `Runtime.caps: Arc<CapDispatcher>` field
  - Initialized in `Runtime::new()` as `CapDispatcher::compat()` (Mode 0)
  - `Runtime::set_cap_mode(mode)` helper that swaps in a fresh dispatcher
- `host-v2/src/main.rs`:
  - `parse_cap_flags(argv) -> (CapMode, Option<audit_log_path>, remaining_argv)`
  - Recognized flags: `--audit`, `--sealed-deps`, `--sealed`, `--audit-log <path>`
  - Env-var override: `CRUFTLESS_CAPS_MODE=compat|audit|sealed-deps|sealed`
  - `drain_audit_log(&rt, dest)` writes records to sidecar file (if `--audit-log` set) or stderr; format `<caller>\t<capability>\t<operation>\t<unix_micros>`
  - Drain called on both success and unhandled-rejection exit paths
- `host-v2/tests/caps_audit.rs` (~95 LOC, 3 tests):
  - `audit_mode_smoke_compat_behavior` — `--audit` does not change Mode-0 program behavior
  - `mode_flag_parsed_does_not_affect_compat_run` — `--sealed` on no-effectful-call program still exits 0 (correctly, because no route-through fires yet)
  - `env_var_mode_override` — `CRUFTLESS_CAPS_MODE=audit` is honored

### Probe result

- **caps_audit (3/3 PASS in 0.01 s)**: CLI flags parse correctly; mode propagates to Runtime; drain happens cleanly on exit; no spurious log output for empty-record runs.
- **PM regression (2/2 PASS in 2.79 s)**: `pm_install_then_require_lodash` + `cli_install_then_run` unchanged.
- **Cumulative caps unit tests (15/15 PASS)**: unchanged from CAPS-EXT 3.

### State at CAPS-EXT 4 boundary

All four modes are invocable. The dispatcher accepts the mode flag, exposes hint-generating CapabilityError, records to AuditLog when in Mode 1. **What is still missing**: no effectful method routes through the dispatcher. The audit log captures zero records because there is nothing to capture. Mode 3 + empty caps would not block anything because no method consults the dispatcher.

This intermediate state is intentional. The infrastructure is end-to-end visible (CLI, dispatcher, drain) so the next round's route-through changes have a target to plug into. The route-through is one EXT round per surface (Fs first, then process.exit, Stdio, etc.).

### Pred-736 corroboration status

- **Pred-736.1 (retrofit not rewrite)**: corroborated further. CAPS-EXT 4 added ~100 LOC across Runtime field + helper + host-v2 main + tests. Two existing-file edits: `interp.rs` (one field, one helper) and `main.rs` (one usage line). Negligible footprint.
- **Pred-736.3 (LOC estimate ~1100)**: CAPS-EXT 3+4 cumulative ~620 LOC. Budget remaining for route-through across the 40 effectful methods: ~480 LOC. On track.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P05.L2.caps-audit-flags` | CAPS-EXT 4: audit recorder + CLI flags (--audit/--sealed-deps/--sealed/--audit-log) + env-var override; dispatcher attached to Runtime; drain-on-exit; PM regression GREEN |

### Open scope at CAPS-EXT 4 boundary

1. **CAPS-EXT 5 (synthetic-adversary probe harness)**: `pilots/rusty-js-caps/probes/` with `.mjs` files representing the Doc 736 §IV attack classes. Under Mode 0 (current default) every probe SUCCEEDS = the pre-state baseline. The probe suite becomes the standing regression for CAPS-EXT 6+ route-through.

2. **CAPS-EXT 6 (Fs read route-through)**: edit `host-v2/src/fs.rs` to route every fs-read method (readFileSync, readFile, existsSync, statSync, readdirSync, accessSync, etc.) through `rt.caps.require_fs(...)`. Under Mode 0 the dispatcher returns Ok unconditionally; PM regression remains green. Under Mode 3 + empty caps, the corresponding probes flip to PASS (CapabilityError).

3. **CAPS-EXT 7 (Fs write route-through)**: same for write/mkdir/unlink. Separate round because the dispatcher op is different and the probe set is different.

4. **CAPS-EXT 8+**: Stdio, Clock, Scheduler, Process, Env. Each their own round.

### Doc 730 §XVI status

The CLI gate continues the §XVI compositional success: every existing test passes under Mode 0; new tests pass under Mode 1/3; the structural divergence is wider (Cruftless now has CLI-controllable capability modes; Node does not), but no regression appears against the engagement's existing test surface.

---

*CAPS-EXT 4 closes the infrastructure wiring. The capability slider is now invocable end-to-end (CLI + env-var + drain). Next move CAPS-EXT 5: the synthetic-adversary probe harness — the §XVI oracle that gates every route-through commit from CAPS-EXT 6 forward.*

---

## CAPS-EXT 5 — 2026-05-21 (synthetic-adversary probe harness; §XVI oracle baseline)

### Headline

The §XVI oracle for Pilot α is operational. Eight probes covering currently-callable effectful surface; ten Rust harness tests; all PASS in 0.02 s. **Mode-0 baseline confirmed: every probe WINS — the attacker succeeds at every catalogued attack class.** This is the engagement's documented pre-state. CAPS-EXT 6+ flips probes one surface at a time.

### Substrate landed

- `pilots/rusty-js-caps/probes/` (8 `.mjs` files + README):
  - `fs_read.mjs` — read /etc/hostname (Doc 736 §IV class 1: exfil)
  - `fs_write.mjs` — write /tmp/cruftless-probe-fs-write.marker (§IV class 6: persist)
  - `fs_list.mjs` — readdirSync('/etc') (info disclosure)
  - `fs_stat.mjs` — statSync('/etc/hostname') (metadata exfil)
  - `process_exit.mjs` — process.exit(42) (DoS / host-control)
  - `env_read.mjs` — process.env.HOME + .PATH (§IV class 4: env secrets)
  - `clock_read.mjs` — Date.now() before/after busy loop (timing side-channel)
  - `cwd_read.mjs` — process.cwd() (process introspection)
  - `README.md` — harness contract, WINS/LOSES sentinel format, expected behavior across modes

- `host-v2/tests/caps_probes.rs` (~135 LOC, 10 tests):
  - `run_probe(name, mode_flag)` helper
  - `classify(stdout) -> ProbeOutcome::{Wins, Loses, Indeterminate}` (scans for `PROBE:WINS:` or `PROBE:LOSES:` sentinel lines)
  - Eight `baseline_*_wins` tests (one per probe; assert WINS under Mode 0)
  - Two `pre_route_through_sealed_still_wins_*` tests (assert --sealed does not yet block fs_read or process_exit; will be replaced with `*_loses_under_sealed` at CAPS-EXT 6+)

### Probe result

**10/10 PASS in 0.02 s.**

| probe | Mode 0 | Mode 3 (pre-route-through) | Mode 3 (target at CAPS-EXT N) |
|---|---|---|---|
| fs_read | WINS | WINS (no enforcement yet) | LOSES |
| fs_write | WINS | (not yet asserted) | LOSES |
| fs_list | WINS | (not yet asserted) | LOSES |
| fs_stat | WINS | (not yet asserted) | LOSES |
| process_exit | WINS (exit code 42) | WINS (no enforcement yet) | LOSES |
| env_read | WINS | (not yet asserted) | LOSES |
| clock_read | WINS | (not yet asserted) | LOSES |
| cwd_read | WINS | (not yet asserted) | LOSES |

Cumulative PM-EXT 11+12 regression: unchanged green (no Rust source touched on PM path).
Cumulative caps_audit: unchanged 3/3 PASS.
Cumulative caps unit tests: unchanged 15/15 PASS.

### Per-probe Mode-0 evidence captured

Running each probe under default cruftless produces sentinel-line evidence of the attack:

- `fs_read` stdout: `PROBE:WINS:fs_read:raspberrypi5` (host hostname read)
- `fs_write` stdout: `PROBE:WINS:fs_write:/tmp/cruftless-probe-fs-write.marker` (file written + verified)
- `fs_list` stdout: `PROBE:WINS:fs_list:<N>:<first_entry>` (/etc enumerated)
- `fs_stat` stdout: `PROBE:WINS:fs_stat:size=<n>:mode=<m>` (metadata extracted)
- `process_exit` exit code: 42 (host process terminated under attacker control)
- `env_read` stdout: `PROBE:WINS:env_read:home=/home/jaredef:path_len=<n>`
- `clock_read` stdout: `PROBE:WINS:clock_read:dt=<n>ms:i=100000` (timing measured)
- `cwd_read` stdout: `PROBE:WINS:cwd_read:<path>` (CWD disclosed)

Every line is precisely the kind of evidence a supply-chain-attack post-mortem would surface. The harness captures them at Mode-0 baseline, then asserts the flip at Mode-3 once enforcement lands.

### Pred-736 corroboration status

- **Pred-736.2 (synthetic-adversary harness is the right §XVI oracle)**: provisionally corroborated. The eight probes cover the currently-callable effectful surface; the WINS/LOSES sentinel format scales to additional probes; the Rust harness scales to assertions across modes. No representational gap surfaced. The final corroboration arrives at CAPS-EXT 13 when every probe LOSES under Mode 3.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P05.L2.caps-probe-harness` | CAPS-EXT 5: synthetic-adversary probe harness; 8 probes + 10 Rust assertions; Mode-0 baseline (every probe WINS) confirmed; pre-route-through Mode-3 state documented; §XVI oracle operational |

### Open scope at CAPS-EXT 5 boundary

The harness is the *standing regression check* for the rest of Pilot α. Every CAPS-EXT 6+ round must:

1. Route a specific effectful surface through `rt.caps.require_*`.
2. Add a `<probe>_loses_under_sealed` test that asserts the probe now LOSES with `--sealed`.
3. Confirm the corresponding `baseline_*_wins` test still PASSES (Mode-0 backward compat).
4. Confirm PM-EXT 11+12 regression remains GREEN.

The order set at CAPS-EXT 1 holds:

- **CAPS-EXT 6 (Fs read route-through)**: routes readFileSync/readFile/existsSync/statSync/readdirSync/accessSync through `require_fs`. Flips: fs_read, fs_list, fs_stat. Add 3 `*_loses_under_sealed` tests.
- **CAPS-EXT 7 (Fs write route-through)**: routes writeFileSync/writeFile/mkdirSync/unlinkSync. Flips: fs_write.
- **CAPS-EXT 8 (process.exit route-through)**: flips process_exit.
- **CAPS-EXT 9 (Env route-through)**: flips env_read.
- **CAPS-EXT 10 (Stdio route-through)**: no probe yet (the WINS sentinel writes to stdout; need a more careful probe that distinguishes "attacker writes to stdout" from "harness reads stdout"). Add stdio_exfil.mjs at that round.
- **CAPS-EXT 11-12 (Clock + Scheduler)**: flips clock_read.
- **CAPS-EXT 13 (closure)**: every probe LOSES under `--sealed`.

### Doc 730 §XVI status

The probe harness is itself the §XVI engine-diff oracle for the workstream. Where PM workstream had `bun install` as the diff oracle, Pilot α has the probe suite. The Mode-0 / Mode-3 differential becomes the engagement's standing demonstration of the impossibility claim's incremental landing.

---

*CAPS-EXT 5 closes the §XVI oracle setup. CAPS-EXT 6 begins the route-through cascade — Fs read first, as the largest single surface and highest exfil-risk class.*

---

## CAPS-EXT 6 — 2026-05-21 (Fs read route-through; first probes flipped)

### Headline

**The first three synthetic-adversary probes are mechanically refused under `--sealed`.** fs.readFileSync / readdirSync / statSync (+ accessSync, existsSync, async readFile, fs.promises.{stat,readFile}) all route through `rt.caps.require_fs`. Under Mode 0 the dispatcher returns Ok and PM-EXT 11+12 stay GREEN; under Mode 3 the dispatcher refuses with CapabilityError. **The impossibility claim becomes concrete at the fs read tier.**

### Substrate landed

- `host-v2/src/fs.rs` (~50 LOC):
  - `use rusty_js_runtime::caps as caps;` + `ModuleId, ModuleProvenance` (the local `FsOp` in fs.rs conflicted with `caps::FsOp` — resolved by qualifying every call site)
  - `check_fs(rt, op) -> Result<(), RuntimeError>` helper:
    - Infers caller provenance from `rt.current_module_url`: `/node_modules/` → Dependency, `node:` → Builtin, else Application
    - Passes `caps::Fs::none()` (deny-all) as the cap; CAPS-EXT N+ adds `require(spec, {caps})` to let the application pass a non-empty cap
    - Mode 0 / Mode 1: returns Ok unconditionally (Mode 1 also records)
    - Mode 2: ambient for application callers
    - Mode 3: enforces against the passed cap (currently `none`); CapabilityError on every fs read
  - Gates applied to: `readFileSync`, `existsSync`, `statSync`, `readdirSync`, `readFile` (async), `exists` (async), `fs.promises.stat`, `fs.promises.readFile`, `accessSync`

- `host-v2/tests/caps_probes.rs`:
  - Removed `pre_route_through_sealed_still_wins_fs_read` (pre-state assertion no longer valid)
  - Added `fs_read_loses_under_sealed`, `fs_list_loses_under_sealed`, `fs_stat_loses_under_sealed`
  - Each asserts the probe stdout contains a LOSES sentinel and references the fs capability

### Probe result

**12/12 caps_probes PASS in 0.03 s.**

| probe | Mode 0 | Mode 3 |
|---|---|---|
| fs_read | WINS ✓ | **LOSES ✓ (first flip)** |
| fs_write | WINS ✓ | WINS (CAPS-EXT 7 target) |
| fs_list | WINS ✓ | **LOSES ✓ (first flip)** |
| fs_stat | WINS ✓ | **LOSES ✓ (first flip)** |
| process_exit | WINS ✓ | WINS (CAPS-EXT 8 target) |
| env_read | WINS ✓ | WINS (CAPS-EXT 9 target) |
| clock_read | WINS ✓ | WINS (CAPS-EXT 11-12 target) |
| cwd_read | WINS ✓ | WINS (CAPS-EXT 8 target) |

PM-EXT 11+12 regression: **2/2 PASS in 2.83 s**. lodash require + identity(7) works under Mode 0; Mode 0 path is unchanged.

caps_audit: unchanged 3/3 PASS.
caps unit tests: unchanged 15/15 PASS.

### Pred-736 corroboration status

- **Pred-736.4 (Move 1 alone delivers the bulk of the impossibility claim)**: provisional first datapoint. The fs read probes are *mechanically refused* under Mode 3 — not policy-enforced, not crypto-mitigated, but architecturally: the dispatcher consults the empty `Fs::none()` cap and returns CapabilityError before any syscall fires. The first quarter of the impossibility claim is in place after one EXT round of route-through.
- **Pred-736.3 (LOC estimate ~1100)**: CAPS-EXT 6 added ~50 LOC (helper + 9 gate lines). Cumulative ~670 LOC. Budget remaining for the remaining ~30 effectful methods: ~430 LOC. Comfortably on track.
- **Pred-736.1 (retrofit not rewrite)**: continued corroboration. One file edited (fs.rs); negligible diff per call site (one helper call before each existing implementation).

### The shape of the gate at each call site

A typical Mode-0/Mode-3 round-trip:

```
$ cruftless probes/fs_read.mjs
PROBE:WINS:fs_read:raspberrypi5

$ cruftless --sealed probes/fs_read.mjs
PROBE:LOSES:fs_read:TypeError:fs.read(/etc/hostname): no fs capability granted to module 'file:///.../fs_read.mjs' (mode: sealed) — hint: add to caps in package.json: { fs: { read: ['/etc/hostname'] } }
```

The hint field carries the cure. A developer who wants the probe to work under `--sealed` knows from the error message exactly what package.json declaration unblocks the call. This is Doc 736 §IV's developer ergonomic surface, mechanically working.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P05.L2.caps-fs-read-route` | CAPS-EXT 6: fs read route-through; readFileSync/readdirSync/statSync + 6 adjacent surfaces gated; fs_read/fs_list/fs_stat probes flip to LOSES under --sealed; Mode 0 backward compat intact; PM regression GREEN |

### Open scope at CAPS-EXT 6 boundary

1. **CAPS-EXT 7 (Fs write route-through)**: gate writeFileSync, writeFile, mkdirSync, unlinkSync, fs.promises.{writeFile, mkdir, unlink}, copyFileSync, appendFileSync, cpSync. Flip the fs_write probe.

2. **CAPS-EXT 8 (process route-through)**: gate process.exit, process.cwd, process.pid. Flip process_exit + cwd_read probes.

3. **CAPS-EXT 9 (env route-through)**: gate process.env reads + os.hostname/homedir/tmpdir/userInfo/cpus. Flip env_read probe.

4. **CAPS-EXT 10 (Stdio route-through)**: gate process.stdout/stderr.write + console.log/error/warn/info/debug. Subtle because the WINS/LOSES probes themselves write to stdout; under `--sealed-stdio` the probes can't report. Need a different probe shape (write-via-fs-marker instead of console.log) at that round.

5. **CAPS-EXT 11-12 (Clock + Scheduler)**: gate Date.now, hrtime, performance.now, setTimeout, setInterval, queueMicrotask, nextTick. Flip clock_read probe.

6. **CAPS-EXT 13 (closure)**: every probe LOSES under `--sealed`; Doc 736 §IV impossibility claim mechanically realized.

### Doc 730 §XVI status

The probe-harness/route-through pair is Case-1 substrate-introduction (cruftless gaining a capability the upstream lacks; no ecosystem deviation since Node has no equivalent). Each EXT round delivers a measurable §XVI yield: more probes refused, no Mode-0 regression. The aggregation across EXT rounds will be the CAPS-EXT 13 closure metric.

---

*CAPS-EXT 6 closes the first route-through round. Three of eight probes are mechanically refused under `--sealed`. The remaining five probes are queued for CAPS-EXT 7-12, each its own round, each with the same shape: route-through + flip + Mode-0 regression confirm.*

---

## CAPS-EXT 7 — 2026-05-21 (Fs write route-through; fourth probe flipped)

### Headline

`writeFileSync`, `writeFile` (async), `mkdirSync`, `unlinkSync`, `fs.promises.{writeFile, access, mkdir, unlink}`, `appendFileSync`, `copyFileSync`, `cpSync` all route through `rt.caps.require_fs`. **The fs_write probe LOSES under `--sealed` and the marker file is not created on disk.** Half of the queued probes (4 of 8) are now mechanically refused.

### Substrate landed

- `host-v2/src/fs.rs` (~30 LOC added, 8 surfaces gated):
  - `writeFileSync` → `caps::FsOp::Write`
  - `writeFile` (async) → `caps::FsOp::Write`
  - `mkdirSync` → `caps::FsOp::Mkdir`
  - `unlinkSync` → `caps::FsOp::Remove`
  - `fs.promises.writeFile` → `caps::FsOp::Write`
  - `fs.promises.access` → `caps::FsOp::Stat`
  - `fs.promises.mkdir` → `caps::FsOp::Mkdir`
  - `fs.promises.unlink` → `caps::FsOp::Remove`
  - `appendFileSync` → `caps::FsOp::Write`
  - `copyFileSync` → `caps::FsOp::Read(src) + caps::FsOp::Write(dst)` (two checks)
  - `cpSync` → `caps::FsOp::Read(src) + caps::FsOp::Write(dst)` (two checks)

- `host-v2/tests/caps_probes.rs`:
  - Added `fs_write_loses_under_sealed` — asserts LOSES sentinel **and** marker file is not created on disk
  - Pre-clears any prior marker so the assertion is unambiguous

### Probe result

**13/13 caps_probes PASS in 0.03 s.**

| probe | Mode 0 | Mode 3 |
|---|---|---|
| fs_read | WINS ✓ | LOSES ✓ |
| fs_write | WINS ✓ | **LOSES ✓ (flipped)** + marker not on disk |
| fs_list | WINS ✓ | LOSES ✓ |
| fs_stat | WINS ✓ | LOSES ✓ |
| process_exit | WINS ✓ | WINS (CAPS-EXT 8) |
| env_read | WINS ✓ | WINS (CAPS-EXT 9) |
| clock_read | WINS ✓ | WINS (CAPS-EXT 11-12) |
| cwd_read | WINS ✓ | WINS (CAPS-EXT 8) |

**4 of 8 probes mechanically refused under --sealed.**

PM-EXT 11+12 regression: 2/2 PASS in 3.42 s. lodash require + identity unchanged.
caps_audit: 3/3 unchanged.
caps unit tests: 15/15 unchanged.

### Doc 736 §IV class coverage

After CAPS-EXT 6+7, the following attack classes are mechanically refused under `--sealed`:

- **Class 1 (read any file)**: closed via fs_read flip
- **Class 6 (persist)**: closed via fs_write flip
- **Class adjacent (info disclosure via list/stat)**: closed via fs_list / fs_stat flips

Remaining classes: process control (exit/cwd), env secrets, timing side channels, stdio side channels.

### Pred-736 corroboration status

- **Pred-736.4 (Move 1 delivers bulk of impossibility claim)**: half of the impossibility claim is in place after two EXT rounds of route-through (CAPS-EXT 6+7). The filesystem attack surface — the largest single class per the CAPS-EXT 1 audit — is closed.
- **Pred-736.3 (LOC estimate ~1100)**: CAPS-EXT 6+7 added ~80 LOC total (50 + 30). Cumulative ~700 LOC. Budget remaining for the remaining ~22 effectful methods: ~400 LOC. On track.
- **Pred-736.1**: continued. Per-EXT diff is single-digit lines per call site.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P05.L2.caps-fs-write-route` | CAPS-EXT 7: Fs write route-through; writeFileSync + mkdirSync + unlinkSync + 8 adjacent surfaces gated; fs_write probe flipped + marker-not-on-disk confirmed; PM regression GREEN |

### Open scope at CAPS-EXT 7 boundary

1. **CAPS-EXT 8 (process route-through)**: gate `process.exit`, `process.cwd`, `process.pid`. The first two probes flip; the third remains a low-severity disclosure. Note that `process.exit` is registered in node_stubs.rs, not fs.rs — different file.

2. **CAPS-EXT 9 (env route-through)**: gate `process.env` reads + `os.hostname/homedir/tmpdir/userInfo/cpus`. Flips env_read probe.

3. **CAPS-EXT 10 (Stdio route-through)**: gate `process.stdout/stderr.write` + `console.log/error/warn/info/debug`. Subtle because the probe sentinel mechanism writes to stdout. Need a markered probe (file-based) at that round.

4. **CAPS-EXT 11-12 (Clock + Scheduler)**: gate `Date.now`, `hrtime`, `performance.now`, `setTimeout`, `setInterval`, `queueMicrotask`, `nextTick`. Flips clock_read probe.

5. **CAPS-EXT 13 (closure)**: every probe LOSES under `--sealed`.

---

*CAPS-EXT 7 closes the filesystem class. The full fs surface — read + write + list + stat + mkdir + remove + copy + cp — is mechanically refused under `--sealed`. Next round leaves the fs.rs file and moves to node_stubs.rs for process.exit and friends.*
