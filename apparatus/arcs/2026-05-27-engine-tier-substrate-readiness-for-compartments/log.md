# 2026-05-27-engine-tier-substrate-readiness-for-compartments — log

Append-only event log per arc-as-coordinate.md formalization. Each entry timestamped per the originating Telegram message ID and dated 2026-05-27 unless noted.

## 13:32 UTC (Telegram 9985-9991) — Triage of premature ESBC closure

The day's first event is actually the tail of the prior ESBC arc's close-then-reopen cycle. Telegram 9985 keeper directive "Continue" drove ES-EXT 4 (reverse bridge); 9986 reported closure; 9987 directed test262-sample measurement. The sweep returned 33.2% (2435/4907/397) — catastrophic regression from 77.6% baseline. Triage: bisection ran ES-EXT 2 disabled with ES-EXT 3+4 retained → 86.7% (+9.1pp over baseline). ES-EXT 2 reverted; ES-EXT 3+4 retained. ESBC arc reopened.

This is the event chain that triggered the keeper's "straighten this out at the top of the alphabet / DAG / lattice" directive at Telegram 9992.

## 13:39 UTC (Telegram 9990) — Doc 729 §XIII first read

Keeper pointed to Doc 729 §XIII (regression-as-implicit-constraint-probe) as the methodology to apply to the regression. Explore agent located the doc and quoted the load-bearing paragraphs. The §XIII case study (P53.E11/E12/E13 module-namespace lift) is named as precedent for the recurrence pattern.

## 13:45 UTC (Telegram 9992) — Top-of-lattice directive

Keeper: "We need to straighten this out at the top of the alphabet / DAG / lattice."

Plan agent delegation produced the 5-rung migration plan for the global-binding-surface-unification (GBSU) arc. Diagnosis: the bifurcated DAG-top primitive is the realm's global Variable Environment Record (ECMA §9.1.1.4, §16.1) carried on two surfaces (Runtime.globals HashMap + globalThis Object) with runtime bridges (ES-EXT 3+4) patching the divergence. Straightening move: globalThis Object IS the global VarEnvRec; HashMap demoted then deleted.

## 13:48 UTC (Telegram 9994) — GBSU-EXT 1 LANDED

Locale spawned: pilots/global-binding-surface-unification/ with seed.md + trajectory.md. Manifest refreshed: 188 locales (106 top, 82 nested). Substrate: Runtime.global_object: Option<ObjectId> added; install_global_this populates it; zero behavioral change. Gates: build clean; diff-prod 42/42.

## 14:00 UTC (Telegram 9996) — GBSU-EXT 2 LANDED

Reader migration. Op::LoadGlobal / LoadGlobalOrUndef route through global_object first; HashMap fallback retained. ES-EXT 4 reverse-fallback absorbed (now primary path). Sweep: 86.8% (6313/962/397) — within noise of baseline; +2 newly passing. Doc 731 alphabet status: meaning shifted (Object canonical, HashMap transitional) but cardinality unchanged.

## 14:14 UTC (Telegram 9998) — GBSU-EXT 3a LANDED (via §XIII recurrence #1)

First-cut writer migration dropped HashMap insert at Op::StoreGlobal → sweep 86.1% (−51 tests). Implicit constraint surfaced: Rust-side code paths read `self.globals` directly, outside Op::LoadGlobal (which had migrated). Revised: dual-write retained (Object canonical + HashMap mirror). 86.8% restored. Plan revised from 5 rungs to 7 (audit-before-removal substep interposed).

## 14:23 UTC (Telegram 10000) — Doc 729 §XIII first amendment

Keeper directive: "Amend doc 729 with this experience." Stage 1 (corpus-master), stage 2 (mirror+push to jaredef/resolve commit cb3dd7c), stage 3 (bun run seed at jaredef/jaredfoy). The "second instance at the runtime tier (GBSU-EXT 3)" paragraph entered §XIII immediately after the P53.E11/E12/E13 case.

## 14:33 UTC (Telegram 10002) — GBSU-EXT 4 LANDED

Audit of 19 read-side self.globals accesses outside Op::Load*; 3 RUNTIME readers identified (to_object Boolean lookup, new_empty_set Set lookup, array_species_create Array ctor identity). Migrated to new helper `Runtime::global_get`. Sweep 86.8% parity. Standing rec: 15% of audited accesses migrate.

## 14:50 UTC (Telegram 10004) — GBSU-EXT 5 REVERTED (§XIII recurrence #2)

Dropped HashMap write from Op::StoreGlobal. Sweep 86.1% (−52). Audit still incomplete — sample failures traced to typed-array BYTES_PER_ELEMENT cluster, which routes through `rt.globals.get(ctor_name)` at intrinsics.rs:12731 (a per-typed-array closure body). Rung 4's audit was scoped to `self.globals.get(` only, missing `rt.globals.get(` and closure-captured patterns. Plan revised again: 4b expanded audit.

## 15:02 UTC (Telegram 10006) — GBSU-EXT 4b + 5 retry, three rounds (§XIII recurrence #3 + #4)

Round A migrated 10 closure-captured `rt.globals.get` readers across intrinsics.rs / prototype.rs / napi.rs / interp.rs. Sweep 86.1% — STILL regressed. Round B: direct Function-ctor probe `Function("return 1")` returned undefined. Trace: Function ctor synthesizes `__fc_out_N = function anonymous(...)`, evaluates as module (bare assignment → Op::StoreGlobal), reads back via `rt.globals.get(stash_key)`. Three sites use this synthesized-source-stash-key round-trip pattern (intrinsics.rs:1314, 1869, 2063). Migrated to global_get + obj_mut(gt).remove_str. Sweep 86.7% (6310/966/397) — recovered. Standing rec: text-grep audits miss structural round-trip patterns; enumerate stash-key / side-channel idioms as separate audit scopes.

## 15:34 UTC (Telegram 10008) — GBSU-EXT 6 LANDED (transitional fallback paths deletion)

Keeper directive: "Continue. Be sure to record deletion in the apparatus deletion ledger." Substrate: dropped rung-2/3a/4 transitional fallback clauses (Op::LoadGlobal `or_else(globals.get)`, Op::LoadGlobalOrUndef same, Op::StoreGlobal `globals.contains_key` strict-check term, global_get HashMap-fallback final clause). Sweep 86.8% (6312/963/397) — clean parity. Doc 731 alphabet: cardinality unchanged (transitional fallbacks deleted, field retained). Recorded in apparatus/docs/deletions-ledger.md as 2026-05-26 GBSU-EXT 6 entry with full template (files, LoC delta, named constraint, gates, composes-with).

## 15:59 UTC (Telegram 10010) — GBSU-EXT 7a LANDED (structural pivot)

global_object eager-allocated at the START of install_globals so subsequent register_* and direct insert sites can be migrated. install_global_this reuses the pre-allocated Object. Identity-preserving; zero behavioral change. Sweep 86.8% parity. The structural precondition for the per-cluster migration rungs (7b-7e).

## 16:12 UTC (Telegram 10012) — GBSU-EXT 7b LANDED (via §XIII recurrence #5)

register_global_ctor + register_global_fn migrated via new `define_global_property` helper. Sweep 72.7% (−1014) — catastrophic. Round B: 1054 "Function.prototype.call undefined" failures traced to intrinsics.rs:2114 — `self.globals.get("Function")` (late-binding step attaching function_prototype to Function ctor) returned None after register_global_ctor migration. 7 install-time direct readers migrated to global_get. Sweep 85.8% — recovered most, RegExp.prototype still undefined. Round C: regexp.rs has its OWN `register_global_native` helper that ALSO wrote to HashMap. Migrated to write Object directly. Sweep 86.5% — within ~22 of baseline (residual is pre-existing ES2024 Error.isError surface). Standing rec: register-helper migrations cluster by REGISTER FUNCTION, not by call site.

## 16:24 UTC (Telegram 10014, resumed after overnight pause) — GBSU-EXT 7c LANDED (direct-insert sites migrated)

Promoted define_global_property to Runtime impl method. Batch-migrated 24 direct self.globals.insert("X", v) sites in intrinsics.rs via 3 sed patterns. Sweep 85.9% — delta of ~40 from 86.5%. Cluster diff: "argument is not coercible" 18→28 (+10), traces to pre-existing ES2024 unimplemented-method tests whose failure-surface shifted under the install-order change. Sanity probes pass. Standing rec: large-batch sed migrations must verify with sweep that no shape-system/installation-order invariant changed.

## 17:01 UTC (Telegram 10016) — GBSU-EXT 7d LANDED (GC roots migrated)

enumerate_roots now roots globalThis Object (transitively reaches every JS-visible global via prop-table walk) + engine_helpers (NOT JS-visible per §VII.B, must be rooted explicitly). Pre-install fallback retained. Sweep 85.9% identical — behavior-neutral (rooted-set structurally equivalent).

## 17:14 UTC (Telegram 10018) — GBSU-EXT 7e LANDED (realm enter/exit migrated)

Three new Runtime helpers (snapshot_global_string_props / retain_global_string_props / replace_global_string_props). enter_realm + exit_realm migrated. Realm field types unchanged (HashMap<String, Value> stays as snapshot data structure). Sweep 85.9% identical. Standing rec: when a field's data shape survives a unified-surface migration unchanged, the migration is purely read/write path — cleanest §XIII outcome.

## 17:32 UTC (Telegram 10020) — GBSU-EXT 7f.1 + 7f.2 + 7f.3-partial LANDED

Three consecutive sub-rungs batched. 7f.1: ~25 sites in node_stubs.rs (fs, child_process, tls, readline, constants, buffer, http2, dns, module, domain, performance, perf_hooks, async_hooks, punycode, v8, inspector, vm, string_decoder, require, DOMException, PerformanceObserver, diagnostics_channel). 7f.2: ~40 sites across cruftless/src/ (lib, url, stream, process, http, fs, os, events, crypto, util, zlib, tty, path, https, assert, module_ns, timer). 7f.3 partial: regexp register_global_native simplification, intrinsics __record, allocate_realm ctor loop, stash-key dead-remove drops. Sweep 85.9% — zero behavioral delta across 75 sites.

## 17:59 UTC (Telegram 10022) — GBSU-EXT 7f.4 LANDED + ARC CLOSED + Deletion ledger updated

Field deleted. Runtime::new two-step pattern eager-allocates global_object via rt.alloc_object after struct construction. install_globals drops rung-7a late-allocation. install_global_this drops the HashMap-drain loop. Op::StoreGlobal / enumerate_roots / define_global_property drop bootstrap-fallback branches (all use expect). ~15 additional cluster migrations (Promise/globalThis chains, Function ctor Error.prepareStackTrace lookup, napi.rs, promise.rs, cruftless assert/util/module_ns/stream/timer). Sweep **86.7%** — **+61 tests over the 85.9% pre-deletion baseline**. §XIII INVERSION: deletion as positive-surface probe. Doc 731 alphabet: {Object, HashMap, engine_helpers} → {Object, engine_helpers}. Deletion ledger entry added. GBSU arc CLOSED.

## 18:20 UTC (Telegram 10024) — ESBC ES-EXT 2 v2 LANDED + ESBC ARC RE-CLOSED

v2 trial #1: re-enabled dormant v1 script_mode under unified surface — IDENTICAL 33.2% regression. Confirmed empirically that unified surface alone doesn't fix IC.1; LoadUpvalue bypasses globalThis. v2 trial #2 proper: restored pre-allocation passes (IC.1 protected); after Op::StoreLocal at script-top-var decl, ALSO emit Op::LoadLocal + Op::StoreGlobal to mirror to globalThis at declaration time. ~15 LOC compile-tier addition. All P1-P7 probes green. Sweep 86.6% (−15 noise from extra StoreGlobal per top-level var). ESBC arc fully closed against the unified surface. Standing rec ARC.M.7: v2 mirror is one-shot at declaration — subsequent reassignment via local slot doesn't update globalThis. Future rung: compile-tier flip of top-level-script-var ASSIGNMENT targets.

## 18:32 UTC (Telegram 10026) — Doc 729 §XIII second amendment

Keeper directive: "Amend the doc and then mirror to jaredef/resolve then seed at jaredef/jaredfoy" — for the GBSU arc closure. Three additions to §XIII: (1) six further recurrences after rung 3 (audit-scope as implicit constraint on audit completeness); (2) deletion-as-positive-§XIII-probe methodological inversion; (3) induced engine-tier property — alphabet narrowing at the resolver-instance level. Stage 1+2+3 published (commit 1c7ec61).

## 18:37 UTC (Telegram 10028) — Keeper question: Compartments telos

Telegram 10028: "How does this fit with our telos for JS Compartments?" Five threads articulated in reply: (1) one realm = one globalThis post-GBSU, compartment switch becomes ObjectRef substitution; (2) realm enter/exit rung-7e helpers ARE the compartment-switching primitives; (3) engine_helpers separation per §VII.B is the bilateral that keeps compartments from leaking internals; (4) Compartment ctor scaffolding exists at intrinsics.rs:1476; (5) Doc 731 alphabet purity at resolver-instance level IS the Compartments precondition.

## 18:50 UTC (Telegram 10030) — Doc 743 (Cruft Compartments) primary articulation

Keeper directive: "produce a primary articulation in the corpus master for Cruft Compartments." Explore agent assembled research (compartments-as-primitive.md prospective design, realm-substrate-architecture.md, CP-EXT 1-4 implementation, realm machinery, CruftScript candidate, standing corpus references, capability-passing intersection). Drafted Doc 743 in 10 sections + appendix. Names P-C as the fifth induced engine-tier property (after Doc 729 §XIII's P-M/P-N/P-S/P-E). Stage 1+2+3 published (commit e561a35; seeded; +1 doc, +1 prompt, +1 edge). Appendix preserves keeper prompt 10030 verbatim per standing rec ARC.MR.6.

## 19:21 UTC (Telegram 10032) — Keeper question: coherent step before implementation

Telegram 10032: "What is the coherent step before implementing against the compartments formalization?" Replied: falsifier-driven probe of P-C against the current substrate. Three reasons: property named but unverified (Rule 23 prospective application); falsifiers are the constraint set the implementation will be measured against (Doc 729 §IX); today's two-arc lesson (§XIII applied to the engagement itself — probe first, ship after). Listed 8 concrete probes P-C.1 through P-C.8.

## 19:32 UTC (Telegram 10034) — P-C probe set EXECUTED — P-C REFUTED

Keeper directive: "Continue" → execute probes. 8 probes + 3 deeper, run against current cruft build. Results: 4 REFUTED, 1 mixed, 1 holds, 2 inconclusive. Three implicit constraints named:
- **IC.CP1**: Compartment.prototype.globalThis accessor returns function-shaped object (only `length`/`name` own-props), not the compartment's globalThis Object. pc9 + pc11 both confirm.
- **IC.CP2**: Endowments dictionary doesn't actually inject into compartment globalThis. pc2 confirms (`c.globalThis.x` undefined after `new Compartment({globals: {x:42}})`).
- **IC.CP3**: Cross-evaluate persistence broken. compartment.evaluate routes through evaluate_module (Module semantics, fresh scope per call) instead of evaluate_script (Script semantics, persists on realm globalThis per ECMA §16.1). pc7 + pc8 + pc10 all confirm.

Coherent next step: a CP-EXT 5 audit-fix rung addressing all three ICs. Scope bounded. Each repair verifiable by corresponding P-C probe re-running green.

## 19:50 UTC (Telegram 10036) — This arc formalized

Keeper directive: "Create an apparatus arc for the entirety of this work. You should see how arcs are formalized in the repo." Arc spawned at apparatus/arcs/2026-05-27-engine-tier-substrate-readiness-for-compartments/ with arc.md + this log.md per arc-as-coordinate.md formalization. Arc CLOSED on creation since all sub-rungs have landed and the close-condition (substrate cleared + corpus articulated + property probed) is met. Next arc opens on directive.

## Arc-level finding summary

Six cross-locale findings entered in arc.md (ARC.MR.1 through ARC.MR.6) capture the methodological lessons:
- Engine-tier alphabet narrowing as resolver-instance closure
- §XIII recurrence count as quantifiable substrate-depth signal
- Deletion as positive §XIII probe (methodological inversion, codified in Doc 729 §XIII)
- Formalization-before-implementation discipline (falsifier probe set in same trajectory step as articulation)
- Corpus articulation as forcing function (writing the external face surfaces internal gaps)
- The prompt is part of the artifact (preserve keeper prompts in articulation appendices)

Arc closure timestamp: 2026-05-27 20:00 UTC approximately.
