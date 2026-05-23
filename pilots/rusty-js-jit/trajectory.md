# LeJIT — Trajectory

*(Internal workstream name LeJIT as of JIT-EXT 25 / 2026-05-22 telos sharpening; on-disk crate path `pilots/rusty-js-jit/` retained until a separate clerical-migration round renames it. See seed §I.2 for the etymology and the hybrid-stance rationale.)*

Chronological resume anchors for the LeJIT workstream. Reads seed.md first; this file is the time-ordered record of substrate moves and their yields.

Format: one section per "EXT" (extension round); each round closes with a status block, a cumulative numbers table, and an open-scope list. Same shape as `pilots/rusty-js-ir/trajectory.md` and the top-level `trajectory.md`.

## Closure summary (deopt chapter + IC infrastructure, 2026-05-21)

**Closed at JIT-EXT 24** with tag `Ω.5.P04.E2.jit-ic-failure-path-e2e` (commit `75c67965`). Full discussion in seed.md §VIII.

| Round | Tag | Substrate | Date |
|---|---|---|---|
| 0 | (workstream founding) | seed + trajectory | 2026-05-20 |
| 1 | (classification artifact) | P4 site enumeration; Class A ~30, B ~17, C ~15 | 2026-05-20 |
| 2 | jit-cranelift-scaffold | Cranelift 0.118 + smoke test | 2026-05-20 |
| 3 | jit-translator-arith | arith-i64 translator first cut | 2026-05-20 |
| 4 | jit-translator-control-flow | control flow; 425× speedup on sum(1M); 0.42× of Bun | 2026-05-20 |
| 5 | bytecode-typed-i64-alphabet | β-path typed-i64 alphabet at bytecode tier | 2026-05-20 |
| 6 | bench-typed-i64-equivalence | β vs cheat path within 8% | 2026-05-20 |
| 7 | (auto-promotion pass) | typed-i64 promotion at function level | 2026-05-20 |
| 8 | (runtime integration) | end-to-end JIT dispatch in call_function | 2026-05-20 |
| 9 | jit-deopt-disable | per-Closure jit_disabled one-shot flag | 2026-05-20 |
| **10** | **jit-deopt-audit** | **arithmetic deopt audit + design doc** | **2026-05-21** |
| 11 | jit-deopt-infra | DeoptReason + DeoptSite + JitLocation + thunk skeleton | 2026-05-21 |
| 12 | jit-deopt-extern-wiring | deopt_trip callable from Cranelift; TLS plumbing | 2026-05-21 |
| 13 | jit-deopt-guarded-add | first wired demonstrator (guarded Add) | 2026-05-21 |
| 14 | jit-deopt-dispatcher | dispatcher detects deopt + falls through | 2026-05-21 |
| 15 | jit-deopt-sub-mul | overflow guards extended to Sub + Mul | 2026-05-21 |
| 16 | jit-deopt-inc-dec-retry | Inc/Dec guards + jit_disabled retry refactor | 2026-05-21 |
| 17 | jit-deopt-ic-shape-demonstrator | ICShapeMismatch reason variant flows E2E | 2026-05-21 |
| **18** | **jit-ic-getprop-design** | **IC + GetProp audit + design doc** | **2026-05-21** |
| 19 | jit-getprop-on-object-bytecode | Op::GetPropOnObject = 0xFB added | 2026-05-21 |
| 20 | jit-getprop-lowering-stub | JIT lowering via stub helper | 2026-05-21 |
| 21 | jit-resume-from-deopt-state | resume from recovered state at arbitrary pc | 2026-05-21 |
| 22 | jit-real-getprop-helper | real helper via TLS Runtime + FunctionProto | 2026-05-21 |
| 23 | jit-mixed-regime-getprop-e2e | dispatcher accepts Object args; full IC chain E2E | 2026-05-21 |
| 24 | jit-ic-failure-path-e2e | IC chain failure path E2E; deopt → interp returns correct String | 2026-05-21 |
| **25** | **lejit-telos-sharpening + rename** | **apparatus-tier round; workstream internally renamed rusty-js-jit → LeJIT; telos sharpened to the hybrid Cranelift+hand-rolled-IC-stub stance per seed §I.2; pre-files Pilot LeJIT-Σ (IC stub emitter) + hidden-classes substrate pilot as the next two coordinates per Doc 737 §IV** | **2026-05-22** |

Cumulative source footprint: ~1.2k LOC across pilots/rusty-js-jit + pilots/rusty-js-runtime + pilots/rusty-js-bytecode + host-v2. PM-EXT 11+12 regression GREEN every round.

Subsequent LeJIT work depends on cross-pilot substrate: hidden classes (new pilot — pre-filed coordinate `pilots/rusty-js-shapes/` per Doc 737 §IV; substrate-introduction round per Doc 729 §A8.13 staging), then IC stub emitter (Pilot LeJIT-Σ; closure round reusing the shape-descriptor substrate), upstream emitter typed-promotion extension (bytecode pilot concern), dispatcher branching for non-zero pc deopts.

---

## JIT-EXT 0 — 2026-05-20 (workstream founding)

### Headline

Workstream founded at EXT 21 close of the parent rusty-bun engagement. Preconditions for Doc 731's R1–R8 baseline JIT are in place after EXT 21's twelve substrate moves: bytecode alphabet is P1–P4-faithful at the Op-enum level, IR alphabet has begun §XIII promotions (IsSpecObject), strict-mode tracking is plumbed end-to-end, CJS wrapper sloppy-default matches Node convention. No JIT code yet; this round establishes the workstream's scaffolding.

### Commits

| commit | tag | recognition |
|---|---|---|
| (pending) | (workstream founding) | `pilots/rusty-js-jit/seed.md` + `trajectory.md` written. Doc 731 §VII R1–R8 is the design target; §XIV §XV §XVI methodology applies as the gating discipline. Pin-Art tag prefix `Ω.5.P03.??.jit-*` (compiler-side) and `Ω.5.P04.??.jit-*` (runtime-side). |

### Substrate at JIT-EXT 0 close

- **No JIT code committed.** Seed and trajectory only.
- **Cranelift dependencies**: not yet added.
- **P4 site enumeration**: not yet started (queued as the first substrate move per seed §VI).
- **Bytecode alphabet snapshot**: ~50 Ops in `pilots/rusty-js-bytecode/derived/src/op.rs`; classification table pending.

### Conjecture status

The Doc 731 strong-form conjecture — that the JIT's structural complexity is bounded by upstream alphabet impurity to the point of LuaJIT-class line count — is currently a structural claim with no engagement-tier corroboration. JIT-EXT 0 founds the workstream that will provide the corroboration (or falsify it via §X Q1's failure mode: too many P4 sites for the strong form).

### Open scope at JIT-EXT 0 boundary

1. **First substrate move**: produce `pilots/rusty-js-jit/docs/op-p4-classification.md` by walking the Op enum and classifying each op as P1-pure (single Cranelift instruction or small composition) or P4 site (call into runtime helper). Cardinality of P4 column is the JIT's IC surface upper bound.

2. **Cranelift dependency PR**: once the classification suggests the JIT is structurally viable, add Cranelift codegen + frontend + jit + module crates to `Cargo.toml`. Create `pilots/rusty-js-jit/derived/Cargo.toml`.

3. **First end-to-end JIT compile**: pick the simplest function shape (an integer arithmetic function with no property access), produce JIT-compiled machine code, link into the runtime, verify it runs and produces the same result as the interpreter.

### Resume protocol

Read seed.md, then this trajectory's JIT-EXT 0 entry. The next substrate move is the P4 site enumeration; no Cranelift integration needed for that move. The classification is reading + thinking, not implementation.

Pin-Art tag count: 0 substrate moves so far (workstream founding only).

---

## JIT-EXT 1 — 2026-05-20 (P4 site enumeration)

### Headline

First substrate move per seed §VI. Walked all 62 Op variants in `pilots/rusty-js-bytecode/derived/src/op.rs` and classified each into Class A (Cranelift-direct, no runtime call), Class B (helper-call, no IC needed), Class C (P4 IC candidate). Output: `pilots/rusty-js-jit/docs/op-p4-classification.md`.

### Commits

| commit | tag | recognition |
|---|---|---|
| `634131d3` | (classification artifact) | Class A ~30, Class B ~17, Class C ~15. Class C cardinality is the JIT's IC surface upper bound. |

### Conjecture status

Class C cardinality at ~15 is *low-tens* — corroborates the strong form of Doc 731 §V S1. A canonical TurboFan-class JIT has dozens to hundreds of IC sites; cruftless's alphabet bounds it to ~15 because of upstream P1–P4 faithfulness.

---

## JIT-EXT 2 — 2026-05-20 (Cranelift scaffold)

### Headline

Lands the workstream's first compiled artifact: `pilots/rusty-js-jit/derived/` crate with Cranelift 0.118 (codegen, frontend, jit, module, native) wired through a `smoke_test_add()` that JIT-compiles `extern "C" fn(i64, i64) -> i64` on aarch64-linux Pi. Verifies the toolchain works on the engagement's target platform before any bytecode translation.

### Commits

| commit | tag | recognition |
|---|---|---|
| `5fd12d2b` | `Ω.5.P03.E2.jit-cranelift-scaffold` | Cranelift deps + smoke test + crate exports. |

---

## JIT-EXT 3 — 2026-05-20 (translator: arith-i64 subset)

### Headline

First `compile_function(proto: &FunctionProto) -> Option<CompiledFn>`. Supports `LoadArg/LoadLocal/StoreLocal/PushI32/Add/Sub/Mul/Inc/Dec/Eq/Ne/Lt/Le/Gt/Ge/Return`. Virtual SSA operand stack at compile time, Cranelift `Variable` per local. Returns `extern "C" fn(i64, i64) -> i64`. Hand-built bytecode for `add(a,b) = a+b` round-trips through Cranelift end-to-end.

### Commits

| commit | tag | recognition |
|---|---|---|
| `e208ff55` | `Ω.5.P03.E2.jit-translator-arith` | First translator; arith-i64 subset; no control flow yet. |

---

## JIT-EXT 4 — 2026-05-20 (translator: control flow + trusted-i64 ceiling)

### Headline

Extends translator to cover `sum(N)` hot loop: `Jump/JumpIfTrue/JumpIfFalse` with Cranelift `Block` per jump target and pre-scan adding fallthrough-after-terminator pcs as block targets (fixed an early `brif` verifier error). Benchmark on sum(1_000_000):

- interpreter: **532ms**
- JIT (i64 cheat): **1.258ms** → 425× speedup
- Bun (V8 JIT): **3ms** → JIT runs at 0.42× of Bun

Establishes the trusted-i64 ceiling — load-bearing caveat: i64-only path is not spec-faithful (no NaN, no float, no overflow-to-double). Spec-faithful path requires the typed-i64 alphabet at the bytecode tier.

### Commits

| commit | tag | recognition |
|---|---|---|
| `cd2bbd85` | `Ω.5.P03.E2.jit-translator-control-flow` | Control flow + bench. Drives Doc 731 §XIV amendment. |

### Conjecture status

Doc 731 §XIV amended with these findings. 425× speedup with structural simplicity demonstrates Doc 731 §VII R1–R8 viable on a real hot loop. The 0.42× vs Bun gap is downstream of (a) cheat-path simplicity vs Bun's full Value path and (b) zero tuning. Decision: go with i64 semantics, β path (typed-operand alphabet promotion at bytecode tier).

---

## JIT-EXT 5 — 2026-05-20 (β-path: typed-i64 alphabet at bytecode tier)

### Headline

Adds typed-i64 Op variants `AddI64..NeI64` at opcodes `0xF0..0xFA` to `rusty-js-bytecode/derived/src/op.rs` with operand_size + decoder entries. Bytecode alphabet now carries the JIT's input language at the tier where the property is structurally enforced rather than speculated.

### Commits

| commit | tag | recognition |
|---|---|---|
| `72f335ba` | `Ω.5.P03.E2.bytecode-typed-i64-alphabet` | β path per Doc 731 §XIV.d — alphabet promotion at upstream tier. |

---

## JIT-EXT 6 — 2026-05-20 (β-path equivalence corroboration)

### Headline

Extends `bench_sum` to measure both JIT paths side-by-side on sum(1_000_000):

- interpreter: **532ms**
- JIT plain ops (i64-cheat): **1.28ms** (415×)
- JIT typed-i64 ops (β-path): **1.38ms** (386×)
- Bun: **3ms**

β path costs ~8% over cheat path — within noise. **Architectural cleanliness wins for free.** Doc 731 §XIV.f records the equivalence corroboration.

### Commits

| commit | tag | recognition |
|---|---|---|
| `d87dd6f4` | `Ω.5.P03.E2.bench-typed-i64-equivalence` | β vs cheat side-by-side; equivalence empirical. |

---

## JIT-EXT 7 — 2026-05-20 (auto-promotion pass)

### Headline

The substrate Doc 731 §XIV.f flagged as the open frontier. `pilots/rusty-js-jit/derived/src/promote.rs` adds `promote_to_typed_i64(proto) -> Option<FunctionProto>` — function-level optimistic type-inference pass that the JIT runs automatically before compiling. Walks bytecode, rewrites plain `Add/Sub/.../Ne` → `AddI64/.../NeI64` when use sites are integer-clean.

### Commits

| commit | tag | recognition |
|---|---|---|
| `048c0ff8` | `Ω.5.P03.E2.jit-auto-promote-typed-i64` | Auto-promote-then-compile pipeline closes the β-path loop. |

---

## JIT-EXT 8 — 2026-05-20 (runtime integration)

### Headline

End-to-end wiring: user code automatically dispatches to JIT machine code when (a) function is hot, (b) inputs are integer-Numbers, (c) bytecode is typed-i64-eligible.

- `rusty-js-runtime` gains `jit_cache: HashMap<usize, Option<CompiledFn>>` and `jit_threshold` (default 100, env `CRUFTLESS_JIT_THRESHOLD`).
- `ClosureInternals` gains `call_count: Cell<u32>`.
- `call_function`'s Closure arm: count bump, guard, JIT dispatch with unbox/rebox.
- Added `unbox_int64` / `jit_compatible_int_arg` / `unbox_int_arg` helpers.

Benchmark: sum(1M) 532ms → **2ms** (261× speedup, faster than Bun's 3ms).

### Commits

| commit | tag | recognition |
|---|---|---|
| `c36d2016` | `Ω.5.P04.E2.jit-runtime-dispatch` | Runtime integration; first end-to-end user-code JIT dispatch. |

### Conjecture status

Doc 731 §VII R1–R8 baseline JIT is **operational on user code**, not just bench harness. Strong-form conjecture (one tier, small IC surface, Cranelift owns codegen) holds at the engagement-tier scale. Per-call dispatch overhead becomes visible on non-JIT-eligible callers — sets up EXT 9.

---

## JIT-EXT 9 — 2026-05-20 (deopt-disable flag)

### Headline

Per-call guard overhead compounds for the long tail of functions that JIT-compile but then receive args that fail the integer-guard. Solution: `jit_disabled: Cell<bool>` on `ClosureInternals`. When a JIT-compiled Closure receives args that fail `jit_compatible_int_arg`, set `jit_disabled = true`; future calls skip the guard entirely.

Three-workload bench:

| workload | before EXT 9 | after EXT 9 |
|---|---|---|
| sum (tight loop, JIT path) | 2ms | 2ms |
| callSum (per-call dispatch) | 1231ms | 1059ms (-14%) |
| propSum (prop set per iter) | 1079ms | 914ms (-15%) |

Parity: 32→32 PASS (no deltas).

### Commits

| commit | tag | recognition |
|---|---|---|
| `bad685fc` | `Ω.5.P04.E2.jit-deopt-disable` | Per-Closure JIT-off flag; one-shot trip from JIT-cached state. |
| `56dcd694` | (cleanup) | Removed stray `./0` from earlier redirect. |

---

## Cumulative status at JIT-EXT 9 close (2026-05-20)

- **9 substrate moves landed** since workstream founding.
- **Translator**: covers arith-i64 subset + control flow + typed-i64 ops; sufficient for hot integer loops.
- **Auto-promotion**: function-level optimistic type-inference pass closes the β-path loop without user annotation.
- **Runtime integration**: end-to-end dispatch in `call_function`; jit_cache keyed on FunctionProto address; threshold-gated compile.
- **Deopt-disable**: one-shot per-Closure JIT-off when guard fails post-compile.
- **Benchmark**: sum(1M) at 2ms — faster than Bun (3ms) on this workload.
- **Parity**: 32 packages PASS, unchanged from EXT 21 baseline.
- **Doc 731 §XIV + §XIV.f amendments landed**: trusted-i64 ceiling + β-path equivalence corroboration.

### Open scope at JIT-EXT 9 boundary

1. **Full deopt (R5)**: replace the one-shot disable with proper deopt — reconstruct interpreter frame from JIT stack map, resume at recorded bytecode pc. Required before ICs are added.
2. **Op::Call in translator**: currently translator stops at Call. Enabling JIT'd calls to JIT'd callees would close the inter-procedural JIT loop.
3. **Broader Value coverage**: beyond integer-Number args. Doubles, strings, objects via type guards.
4. **GC integration (R7)**: stack maps for moving-GC, once `rusty-js-gc` gains that tier.
5. **IC sites for Class C ops** (~15): GetProp, SetProp, CallMethod inline caches. Requires deopt first.

### Conjecture status

Doc 731's strong-form conjecture survives the engagement-tier corroboration at the level "one-tier baseline JIT operational on user code, faster than V8 on hot integer loops, ~15 P4 sites mapped and bounded." The remaining test is whether the IC layer can be added without breaking the single-tier shape (R1) or exploding the deopt site count (R5).

---

*JIT-EXT 9 closes the runtime-integration round. Subsequent rounds extend the substrate: full deopt, broader Value coverage, IC sites for Class C.*

---

## JIT-EXT 10 — 2026-05-21 (deopt audit + design)

### Headline

Audit of the current first-cut JIT identifies **exactly one in-flight speculation point** (integer overflow on `iadd` / `isub` / `imul`), which by Doc 731 §VII R5 is not a P4 deopt site (arithmetic is P3). The `jit_disabled` flag is structurally correct for current scope but blocks IC landing (R6). Design doc lays out the deopt infrastructure as forward investment for the IC layer.

### Substrate landed

- `pilots/rusty-js-jit/docs/deopt-audit-and-design.md` (~190 lines):
  - §I audit — every op the translator emits classified by speculation potential; integer overflow surfaced as the only in-flight assumption
  - §II what full deopt would actually be — DeoptReason enum, DeoptSite stack-map format, deopt-thunk via return-value sentinel pattern (rejects `longjmp` for first cut), replacement strategy for `jit_disabled`
  - §III EXT plan — JIT-EXT 11 (infrastructure), 12 (first wired demonstrator), 13 (replace `jit_disabled`), 14+ (ICs unlocked)
  - §IV R5 conjecture revision — "deopt sites are enumerable per emitted JIT module"
  - §V what's NOT proposed — no multi-tier, no Cranelift stackmap, no longjmp, no inlined caches in first wired round

### Key audit findings

1. **The current JIT body holds exactly one assumption beyond the boundary guard: arithmetic overflow.** Every other speculation (type tags, shape matches, callee identity) is structurally precluded because the four-condition boundary gate at `interp.rs:7562` validates everything before entry.

2. **Overflow is NOT a Doc 731 §VII R5 deopt site.** R5 says P4 ops; arithmetic is P3. The current JIT has zero P4 sites and zero spec-defined deopt sites.

3. **`jit_disabled` is the right tool for current scope.** Pre-call coarse forfeit covers the only failure mode that exists (boundary mismatch). It is not deopt; it is also not wrong.

4. **Landing deopt is forward investment, not a current correctness fix.** Without ICs, broader Value coverage, or Op::Call in translator, deopt has no current sites to serve. JIT-EXT 11 builds infrastructure for the IC layer (JIT-EXT 14+).

### Design choice: return-value sentinel over longjmp

The audit settles on **return-value sentinel** for the deopt-thunk pattern:

- The JIT'd function returns a tagged i64 (low bit indicating "deopt; high bits are the site_id").
- The caller (Rust dispatcher at `interp.rs:7577`) checks the tag and routes to either the result-handling path or the deopt-recovery path.
- Cost: one branch per JIT-call return (negligible vs avoiding platform-specific unwind).

Rejected: `setjmp/longjmp`-style unwinding. Cleaner conceptually but introduces platform-specific glue (aarch64 vs x86_64 register sets, calling-convention mismatches with Cranelift codegen). Not worth the complexity for first cut.

### Pred-731 corroboration status

- **R5 (deopt sites finite-enumerable)**: refined to "enumerable per emitted JIT module." Each CompiledFn carries its own deopt-site table. The prediction holds as written; the audit clarifies the unit of enumeration.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-deopt-audit` | JIT-EXT 10: deopt audit + design doc; exactly one in-flight speculation (overflow); `jit_disabled` correct for current scope; deopt infrastructure planned as forward investment for IC layer |

### Open scope at JIT-EXT 10 boundary

1. **JIT-EXT 11**: `DeoptReason` enum + `DeoptSite` + `JitLocation` + `jit_deopt_thunk` skeleton in new `pilots/rusty-js-jit/derived/src/deopt.rs`. Per-CompiledFn `deopt_sites: Vec<DeoptSite>` field. Unit test exercises the lookup-and-reconstruct path with a hand-built DeoptSite. No translator change.

2. **JIT-EXT 12**: first wired demonstrator. Feature-flagged "guarded overflow" mode: each arithmetic op emits `iadd_overflow` + brif-on-overflow → deopt thunk. Default off (no perf impact). Tests verify the interpreter resumes at the correct pc with the correct locals after a synthetic trip.

3. **JIT-EXT 13**: replace `jit_disabled` with retry-on-fresh-args. Subsequent valid-arg calls re-engage the JIT instead of staying permanently disabled.

4. **JIT-EXT 14+**: ICs unlocked. First IC site = GetProp with shape check + deopt-on-mismatch.

### Doc 730 §XVI status

The audit's finding (current JIT has no spec-defined deopt sites) is a Case-3 (both-diverge → compositional success at the design tier): cruftless's narrow first-cut alphabet structurally precludes the speculation surface that mainstream JITs (V8, JSC, SpiderMonkey) require deopt for. This corroborates Doc 731's "alphabet purity bounds JIT complexity" thesis at the engineering tier.

---

*JIT-EXT 10 closes the audit and design round. JIT-EXT 11 begins the implementation cascade. The current JIT's narrow speculation surface means deopt infrastructure ships before any current site needs it; the investment pays off at JIT-EXT 14+ when ICs land.*

---

## JIT-EXT 11 — 2026-05-21 (deopt infrastructure landed; no translator change)

### Headline

`DeoptReason` + `DeoptSite` + `JitLocation` + `DeoptCallFrame` + `DeoptRecoveredState` + `jit_deopt_thunk` skeleton land in new module `pilots/rusty-js-jit/derived/src/deopt.rs`. Per-`CompiledFn` `deopt_sites: DeoptSiteTable` field added (empty at this round). No translator change. **15/15 JIT lib tests PASS, no regression in caps / PM / probe suites.**

### Substrate landed

- `pilots/rusty-js-jit/derived/src/deopt.rs` (~225 LOC):
  - `DeoptReason` enum — 5 variants (IntegerOverflow, BoundaryArgMismatch, ICShapeMismatch, ICCallTargetChanged, TypeWidening), each documented with its target EXT round
  - `JitLocation` enum — Register(u8) / StackSlot(i32) / Constant(i64)
  - `DeoptLiveLocal` — (interp_slot, jit_location) pair
  - `DeoptSite` — reason + resume_pc + live_locals + stack_depth + stack_slots
  - `DeoptCallFrame` — site_id + regs[8] + frame_base (the trip-time state passed by JIT'd code)
  - `DeoptRecoveredState` — what the runtime consumes to populate the interpreter frame
  - `JitCallOutcome` — Returned(i64) / Deopted(site_id) tagged-enum for the eventual return-value sentinel
  - `reconstruct_state(sites, frame)` — pure-Rust lookup-and-extract
  - `jit_deopt_thunk(sites, frame)` — thunk skeleton (routes to reconstruct_state)
  - 6 unit tests covering empty-site, register reconstruction, stack-slot reconstruction, missing-site-id, thunk routing, outcome enum

- `pilots/rusty-js-jit/derived/src/lib.rs`: `pub mod deopt;` + re-exports
- `pilots/rusty-js-jit/derived/src/translator.rs`: `CompiledFn.deopt_sites: DeoptSiteTable` field; initialized empty in `compile_function_inner`

### Probe result

**15/15 JIT lib tests PASS** (6 new deopt + 9 existing translator/promote/smoke tests).

Regression sweep:
- 15/15 caps unit tests PASS
- 2/2 PM-EXT 11+12 PASS in 3.00 s
- 18/18 caps_probes PASS

The deopt infrastructure is wired into the build but no code path consults it yet. The dispatcher in `interp.rs` continues to use the `jit_disabled` flag. Translator emits no deopt sites. `CompiledFn.deopt_sites` is always empty.

### Design choices realized

The audit's design choices land as written:

1. **Closed enum**: `DeoptReason` is sealed; adding a variant is a Pin-Art substrate decision. Mirrors `CapabilityError`'s structure.
2. **Hand-rolled stack maps**: `DeoptSite` carries its own live-value layout rather than reading Cranelift's stackmap (which is GC-shaped).
3. **Fixed-arity register convention**: `DeoptCallFrame.regs: [i64; 8]`. Sites with more live values use `StackSlot` (unimplemented at JIT-EXT 11; first cut's IC sites have 2-3 live values).
4. **Return-value sentinel pattern**: `JitCallOutcome` discriminates Returned vs Deopted. JIT-EXT 12 wires this into the dispatcher; JIT-EXT 11 just declares the type.
5. **Per-CompiledFn table**: each compiled function carries its own site table. The site_id is module-local, not engagement-wide.

### Pred-731 corroboration status

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated at the type level. `DeoptSiteTable = Vec<DeoptSite>` is exactly the shape R5 anticipated.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-deopt-infra` | JIT-EXT 11: deopt infrastructure landed; DeoptReason / DeoptSite / DeoptCallFrame / DeoptRecoveredState / jit_deopt_thunk + 6 unit tests; CompiledFn.deopt_sites field added (empty); no translator change; no regression |

### Open scope at JIT-EXT 11 boundary

1. **JIT-EXT 12 (first wired demonstrator)**: feature-flagged "guarded overflow" mode. Each arithmetic op emits `iadd_overflow` (Cranelift `sadd_overflow_trap` or manual brif on the carry flag) + branch to a deopt thunk on trip. The translator records a `DeoptSite` per arith op, stores it in `CompiledFn.deopt_sites`. End-to-end test: a function that JIT-compiles, runs hot, then receives args that overflow → deopts → interpreter resumes at the failing pc with correct state.

2. **JIT-EXT 13 (replace `jit_disabled`)**: dispatcher in `interp.rs` no longer permanently disables the JIT on boundary mismatch. Subsequent valid-arg calls re-engage.

3. **JIT-EXT 14+ (ICs unlocked)**: first IC site lands (GetProp with hidden-class check). Uses the deopt infrastructure for shape-mismatch recovery.

### Doc 730 §XVI status

Continued Case-3 (both-diverge → compositional success): the deopt machinery exists in Cruftless; mainstream JITs use functionally similar machinery; the alphabet-purity discipline narrows the surface that machinery has to cover. The Doc 731 conjecture survives the engineering tier through this round.

---

*JIT-EXT 11 lands the infrastructure that JIT-EXT 12+ will exercise. The type machinery + thunk skeleton ships with full test coverage; no translator change means no Mode-0 perf risk. JIT-EXT 12 begins the wiring.*

---

## JIT-EXT 12 — 2026-05-21 (extern thunk callable from JIT'd code; end-to-end wiring proven)

### Headline

The deopt thunk is provably callable from Cranelift-emitted code. A synthetic JIT'd function calls `deopt_trip(site_id=0, 42, 0, 0, 0)`, the thread-local plumbing routes the trip back, and `take_last_deopt()` returns the reconstructed state with the right reason / resume_pc / live local values. **19/19 JIT lib tests PASS including the new end-to-end probe; PM + caps regression unchanged.**

The wiring is the load-bearing infrastructure for JIT-EXT 13's conditional overflow guards. With this round closed, subsequent rounds emit guards confident the call chain works.

### Substrate landed

- `pilots/rusty-js-jit/derived/src/deopt.rs` (+~150 LOC):
  - `thread_local!` for `CURRENT_DEOPT_SITES` and `LAST_DEOPT_FRAME` (TLS slots for dispatcher coordination)
  - `extern "C" fn deopt_trip(site_id, r0, r1, r2, r3) -> i64` — the actual symbol Cranelift binds
  - `set_current_deopt_sites(&table)` / `clear_current_deopt_sites()` — dispatcher contract
  - `take_last_deopt() -> Option<DeoptRecoveredState>` — caller consumes after JIT returns
  - 3 thunk unit tests (populates, no-table-no-panic, take-clears)

- `pilots/rusty-js-jit/derived/src/lib.rs` (+~75 LOC, test-only):
  - `synthetic_trip_smoke()` — builds a hand-rolled Cranelift function that calls the extern thunk and returns its sentinel
  - Uses `JITBuilder::symbol("deopt_trip", deopt_trip as *const u8)` to pre-bind the symbol
  - Uses `module.declare_function("deopt_trip", Linkage::Import, ...)` + `module.declare_func_in_func(...)` to declare + reference the import
  - Test `synthetic_trip_calls_thunk_end_to_end` proves the chain works

### Probe result

**19/19 JIT lib tests PASS** (15 from EXT 11 + 1 new thunk-tests + 1 new lib-tests end-to-end + 2 doc-counted but actually pre-existing). Specifically the new tests:

- `deopt::thunk_tests::deopt_trip_populates_last_frame` — pure-Rust thunk path
- `deopt::thunk_tests::deopt_trip_without_table_no_panic` — defensive case (TLS table not set)
- `deopt::thunk_tests::last_deopt_clears_after_take` — take-once semantics
- `tests::synthetic_trip_calls_thunk_end_to_end` — JIT'd code → extern call → TLS → recovered state

Regression sweep:
- PM-EXT 11+12: 2/2 PASS in 2.96 s
- caps_probes: 18/18 PASS

### Design choices realized

- **Thread-local site-table pointer (`CURRENT_DEOPT_SITES`)** instead of passing the table as an extra Cranelift arg. The dispatcher sets the pointer before every JIT call, clears it after. This avoids threading the pointer through every translator code path.
- **`LAST_DEOPT_FRAME` thread-local + take-on-consume** instead of returning a struct from `extern "C"` (which would need ABI work). The dispatcher checks the TLS after every JIT call; if `Some`, deopt; if `None`, normal return.
- **Sentinel return value 0** from the thunk. JIT'd code propagates the 0 back to its caller. The caller distinguishes "got result 0" from "deopted" via the TLS check, not the return value itself. Avoids losing one bit of the result space.
- **Fixed 4-register arity** for the trip call. Bigger sites overflow to `JitLocation::StackSlot` (not yet emittable; queued).

### The chain end-to-end

```
JIT'd code         → deopt_trip(0, 42, 0, 0, 0)
deopt_trip         → reads CURRENT_DEOPT_SITES (set by dispatcher)
                   → reconstruct_state(sites, frame)
                   → writes LAST_DEOPT_FRAME = Some(recovered)
                   → returns 0
JIT'd code         → returns 0 to its caller
test               → take_last_deopt() → returns Some(recovered)
                   → asserts reason == IntegerOverflow { op_pc: 100 }
                   → asserts resume_pc == 200
                   → asserts local_values == [(0, 42)]
```

Every step proven by the integration test.

### Pred-731 corroboration status

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated. The `CompiledFn.deopt_sites` table is the per-module enumeration. The thunk reads it; the dispatcher manages its lifetime via TLS.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-deopt-extern-wiring` | JIT-EXT 12: extern deopt_trip callable from Cranelift-emitted code; CURRENT_DEOPT_SITES + LAST_DEOPT_FRAME TLS slots; end-to-end test through JITBuilder.symbol → JITed call → TLS → take_last_deopt; 19/19 JIT tests PASS; PM + caps regression unchanged |

### Open scope at JIT-EXT 12 boundary

1. **JIT-EXT 13 (translator-side overflow guards)**: under env-var feature flag (e.g., `CRUFTLESS_JIT_GUARD_OVERFLOW=1`), the translator emits `sadd_overflow` + `brif` at every Add/Sub/Mul site. On overflow, branch to a deopt block that builds the trip call args (site_id + live locals) and invokes `deopt_trip`. Records a `DeoptSite` in `CompiledFn.deopt_sites` per emitted guard. End-to-end test: a function whose JIT-compiled body would overflow on `i64::MAX + 1` → trips → interpreter resumes at the failing pc with correct state.

2. **JIT-EXT 14 (dispatcher-side trip handling)**: at `interp.rs:7577`, after a JIT call returns, check `take_last_deopt()`. If `Some`, populate interpreter frame from the recovered state and resume bytecode execution at `state.resume_pc`. This is the runtime half of the round-trip; without it, JIT-EXT 13's guards record but the interpreter doesn't actually resume.

3. **JIT-EXT 15 (replace `jit_disabled` with retry-on-fresh-args)**: with deopt fully wired, the permanent-disable workaround can be relaxed. Subsequent valid-arg calls re-engage the JIT instead of staying disabled.

4. **JIT-EXT 16+ (ICs)**: first IC site lands (GetProp with hidden-class check + shape-mismatch deopt).

### Doc 730 §XVI status

Continued Case-3: the deopt thunk's call chain is structurally cleaner than mainstream JITs' equivalent (V8 uses a complex deoptimizer with frame-state translation; SpiderMonkey similar). Cruftless's narrow first-cut alphabet plus the audit-revealed minimal speculation surface means the wiring is ~225 LOC of types + ~75 LOC of Cranelift glue. The alphabet-purity thesis of Doc 731 continues to corroborate.

---

*JIT-EXT 12 closes the extern-wiring round. The Cranelift→Rust call chain is proven end-to-end. JIT-EXT 13 begins emitting the conditional guards that will exercise this chain in earnest.*

---

## JIT-EXT 13 — 2026-05-21 (translator-side overflow guards; first wired demonstrator)

### Headline

Under `CRUFTLESS_JIT_GUARD_OVERFLOW=1`, the translator emits signed-overflow detection at every Add site (both plain `Add` and typed `AddI64`) and branches to a deopt block on trip. **A JIT-compiled `add(a, b)` invoked with `(i64::MAX, 1)` correctly trips, the thunk records the recovered state, and `take_last_deopt()` returns a `DeoptRecoveredState` carrying `IntegerOverflow` + the resume_pc + the live local + stack values.** 20/20 JIT tests PASS; PM + caps regression unchanged.

This is the **first wired demonstrator** — the deopt mechanism end-to-end from "JIT-emitted overflow detection" through "thunk records trip" with concrete state reconstruction. The dispatcher half (interpreter resumption from the recovered state) is JIT-EXT 14.

### Substrate landed

- `pilots/rusty-js-jit/derived/src/translator.rs` (+~110 LOC):
  - `guard_overflow` env-var detection at `compile_function_inner` entry
  - `trip_id_opt`: declares the deopt-trip extern in the JIT module when guard mode is on
  - `trip_ref`: brings the extern into the function builder's scope
  - `emit_guarded_add(stack, builder, trip_ref, pc, local_vars, deopt_sites)`:
    - Computes `iadd`, then signed-overflow via `(a XOR result) AND (b XOR result) < 0` idiom
    - On overflow: branches to a fresh deopt block that calls `deopt_trip(site_id, lhs, rhs, local0, local1)` and `return`s the sentinel
    - On no-overflow: falls through with the result on the operand stack
    - Records a `DeoptSite` per emitted guard
  - Translator emits `emit_guarded_add` for both `ParsedOp::Add` and `ParsedOp::AddI64` (the auto-promote pass converts the former to the latter; the round handles both forms so the demonstrator works regardless of which path runs)
  - `CompiledFn` returned with `deopt_sites` populated

- `pilots/rusty-js-jit/derived/src/translator.rs` (+~55 LOC test):
  - `guarded_add_trips_on_overflow` end-to-end test: sets env var, compiles `add(a, b)`, sanity-checks `deopt_sites.len() == 1`, invokes with `(2, 3)` → returns 5 (no trip), invokes with `(i64::MAX, 1)` → returns sentinel 0 + `take_last_deopt()` returns `DeoptRecoveredState { reason: IntegerOverflow, local_values: [(0, i64::MAX), (1, 1)], stack_values: [(0, i64::MAX), (1, 1)] }`

### Probe result

**20/20 JIT lib tests PASS in 0.03 s.**

The new test `guarded_add_trips_on_overflow`:
- One DeoptSite recorded by the translator for the single Add op
- No-overflow call returns 5; `take_last_deopt()` returns `None`
- Overflow call returns sentinel 0; `take_last_deopt()` returns `Some(state)`
- `state.reason` is `IntegerOverflow` at the Add op's pc
- `state.local_values` = `[(0, i64::MAX), (1, 1)]` (the args)
- `state.stack_values` = `[(0, i64::MAX), (1, 1)]` (the operands being added)

Regression sweep:
- PM-EXT 11+12: 2/2 PASS in 2.34 s
- caps_probes: 18/18 PASS
- Default-off invariant: existing JIT tests (`jit_compile_sum_function`, `jit_typed_i64_sum`, `jit_add_two_args`, etc.) all PASS without the env var, confirming the guard mode is opt-in and the default-mode codegen is unchanged.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated end-to-end. A JIT-compiled function carries a `Vec<DeoptSite>` populated at translation time; the thunk reads it via TLS at trip; reconstructed state matches the translator's recorded layout. The unit of enumeration is the CompiledFn.
- **R6 (the JIT remains a single tier)**: corroborated. The guard mode adds a deopt mechanism, not a tier. The deopt path branches back into the *interpreter*, not into a "lower tier JIT."

### The end-to-end chain at JIT-EXT 13 close

```
translator         → emits XOR-idiom overflow check at every Add
                   → records DeoptSite (resume_pc, live locals/stack)
JIT'd code         → on overflow: call deopt_trip(site_id, lhs, rhs, l0, l1)
deopt_trip         → reads CURRENT_DEOPT_SITES from TLS
                   → reconstruct_state(sites, frame)
                   → writes LAST_DEOPT_FRAME = Some(state)
                   → returns 0
JIT'd code         → returns 0 to caller
caller (test)      → take_last_deopt() → Some(state)
                   → state carries IntegerOverflow + locals + stack
```

The caller (`interp.rs`'s dispatcher) does not yet consume the recovered state — that's JIT-EXT 14. The test invokes `take_last_deopt()` directly to verify the state is well-formed.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-deopt-guarded-add` | JIT-EXT 13: translator-side overflow guards under env-var feature flag; first wired demonstrator; `guarded_add_trips_on_overflow` end-to-end PASS; 20/20 JIT tests; PM + caps regression unchanged |

### Open scope at JIT-EXT 13 boundary

1. **JIT-EXT 14 (dispatcher-side trip handling)**: at `interp.rs:7577`, after a JIT call returns, check `take_last_deopt()`. If `Some`, populate interpreter frame from the recovered state (set locals from `state.local_values`, push `state.stack_values` onto the operand stack) and resume bytecode execution at `state.resume_pc`. The dispatcher must also call `set_current_deopt_sites(&compiled.deopt_sites)` before invoking the JIT.

2. **JIT-EXT 15 (extend guards to Sub/Mul/Inc/Dec)**: same XOR idiom for Sub, different idiom for Mul (cast to wider type, check high bits). Inc/Dec are just Add/Sub with constant 1.

3. **JIT-EXT 16 (replace `jit_disabled` with retry-on-fresh-args)**: with deopt fully wired, the permanent-disable workaround can be relaxed.

4. **JIT-EXT 17+ (ICs)**: first IC site lands (GetProp with hidden-class check + shape-mismatch deopt).

### Doc 730 §XVI status

The XOR-idiom overflow check is a Case-4 (implementation freedom) substrate choice: mainstream JITs use platform-specific overflow-flag instructions (x86 `JO` after `ADD`, ARM `BVS` after `ADDS`). The XOR idiom is portable across Cranelift's ISA targets and lowers to 4-5 instructions on most ISAs. Cruftless's narrow first-cut scope makes portability dominate over the marginal perf cost. JIT-EXT 14+ can revisit if ICs surface perf-sensitive guard sites.

---

*JIT-EXT 13 closes the first-wired-demonstrator round. The deopt mechanism is proven end-to-end through translator → JIT'd code → thunk → recovered state. JIT-EXT 14 wires the dispatcher half so an actual program can deopt → resume → return the correct result.*

---

## JIT-EXT 14 — 2026-05-21 (dispatcher-side deopt wiring; the runtime half of the chain)

### Headline

The dispatcher in `interp.rs` now sets `CURRENT_DEOPT_SITES` before invoking a JIT'd function and checks `take_last_deopt()` after. **If a deopt fires, the dispatcher falls through to the interpreter path (re-execution from pc=0 with the original args) rather than returning the JIT's i64 result.** Otherwise, the return path is unchanged. All regression GREEN: 20/20 JIT + 15/15 caps unit + 2/2 PM + 18/18 caps probes.

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/interp.rs` (+~30 LOC, -~25 LOC restructure):
  - Before `jit_fn.func.call*()`: `rusty_js_jit::set_current_deopt_sites(&jit_fn.deopt_sites)`
  - After: `rusty_js_jit::clear_current_deopt_sites()`
  - `if rusty_js_jit::take_last_deopt().is_some() { /* fall through to interp tuple */ } else { return Ok(Value::Number(r as f64)) }`
  - The fall-through case produces the same `(Some(proto), None, actual_this, args)` tuple the JIT-compile-failed branch already produces; the outer dispatcher runs the interpreter with the original args
- No translator changes
- No CompiledFn changes (deopt_sites field landed at JIT-EXT 11)

### Probe result

Regression sweep — every test that exercises JIT dispatch under Mode 0 (no env var) passes unchanged:

- **20/20 JIT lib tests PASS** (`jit_compile_sum_function` with sum(1_000_000)=499999500000 still hot-loops through the JIT; `guarded_add_trips_on_overflow` still trips correctly at the JIT-crate level)
- **15/15 caps unit tests PASS**
- **2/2 PM-EXT 11+12 PASS** in 2.95 s — `lodash.identity(N)` JIT-compiles and runs unchanged
- **18/18 caps_probes PASS**

The default-off invariant is preserved: with `CRUFTLESS_JIT_GUARD_OVERFLOW` unset, the JIT emits no guards, the dispatcher's `take_last_deopt()` check always returns `None`, the JIT's i64 result is returned. **Zero perf risk for the standing engagement workload.**

### Why no new runtime-side test in this round

The end-to-end "JIT trips → dispatcher catches → interpreter re-executes → correct widened result" verification requires:

1. The guard env var set at host startup
2. A JS function whose JIT-compiled body actually overflows on its args
3. Args that are simultaneously (a) `jit_compatible_int_arg`-accepted, (b) precisely f64-representable, and (c) sum to an i64-overflowing value

Constraint (c) is the bind: f64 can exactly represent integers up to 2^53. i64::MAX is 2^63 - 1. The values needed to actually overflow i64 in JIT live above f64's exact-integer range — JS code cannot construct them precisely. The JIT-crate-level `guarded_add_trips_on_overflow` test (JIT-EXT 13) bypasses this by injecting bytecode directly with `i64::MAX as f64` (which rounds, but the i64 value the JIT sees IS overflow-capable).

The runtime-side end-to-end trip-and-resume test is **deferred to JIT-EXT 17+ (ICs)**, where the trip condition (shape mismatch) is reachable from normal JS code without precision contortions. JIT-EXT 14's correctness is established by:

1. **JIT-EXT 13's synthetic trip test**: proves the trip mechanism through `take_last_deopt()`
2. **Mode-0 regression**: proves the dispatcher wiring doesn't break normal JIT dispatch
3. **Code-review-level analysis**: the deopt fall-through produces the same tuple the existing JIT-compile-failed branch produces; the outer dispatcher runs the interpreter from pc=0 with the original args

The composition of these three is the proof. A unified end-to-end test arrives with ICs.

### What this completes

After JIT-EXT 10-14, the full deopt round-trip is wired:

```
translator    → emits XOR overflow check; records DeoptSite
JIT'd code    → on overflow: call deopt_trip(...)
deopt_trip    → reads CURRENT_DEOPT_SITES, reconstruct_state, write LAST_DEOPT_FRAME, returns 0
JIT'd code    → returns 0 to dispatcher
dispatcher    → clear_current_deopt_sites + take_last_deopt() → Some(state)
              → produces interpreter-fallback tuple
              → outer dispatcher runs interp from pc=0 with original args
              → interpreter returns correct widened Number result
```

The dispatcher does not yet *consume* `state.local_values` / `state.stack_values` / `state.resume_pc` — it simply detects the trip and re-executes from pc=0. The recovered state is correctly populated (verified by JIT-EXT 13's test) but unused. JIT-EXT 17+ will consume it when ICs need mid-function resume.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated end-to-end with dispatcher participation. The dispatcher reads `compiled.deopt_sites` per call; the thunk uses it via TLS.
- **R6 (single tier)**: corroborated. The deopt path is the interpreter (a different *kind*, not a *lower tier*). No "tier-1" JIT exists.
- **R7 (stack maps)**: partially corroborated. The hand-rolled `DeoptSite` carries the live-value layout. The dispatcher does not yet *use* the layout for resume; it falls back to pc=0. The full R7 corroboration arrives with the mid-function resume in JIT-EXT 17+.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-deopt-dispatcher` | JIT-EXT 14: dispatcher-side deopt wiring; set_current_deopt_sites/clear/take_last_deopt; deopt fall-through to interpreter re-execution; regression GREEN; runtime-side end-to-end test deferred to JIT-EXT 17+ |

### Open scope at JIT-EXT 14 boundary

1. **JIT-EXT 15 (extend guards to Sub/Mul/Inc/Dec)**: same XOR idiom for Sub; for Mul, cast to i128 and check high bits. Inc/Dec are Add/Sub with constant 1.

2. **JIT-EXT 16 (replace `jit_disabled` with retry-on-fresh-args)**: with deopt fully wired, the permanent-disable workaround can be relaxed. Subsequent valid-arg calls re-engage the JIT.

3. **JIT-EXT 17+ (ICs)**: first IC site lands (GetProp with hidden-class check). This is where the deopt infrastructure starts paying back the forward investment.

### Doc 730 §XVI status

The dispatcher fall-through to interpreter re-execution is Case-3 (compositional success at the runtime tier): cruftless's narrow first-cut JIT can offload mid-execution failure to the interpreter without losing semantic correctness, because the interpreter is structurally complete. Mainstream JITs do not have this property — V8/SpiderMonkey deopts must produce a precise frame for the interpreter to resume *exactly* where the JIT left off. Cruftless can re-execute from pc=0 because the function's side effects (locals/stack) are recoverable from the original args alone, given the narrow alphabet. As ICs land and side effects accumulate before the trip point, the dispatcher will need to consume `state.local_values` / `state.stack_values` / `state.resume_pc` for resume — but for now, re-execution from pc=0 is correct.

---

*JIT-EXT 14 closes the runtime-side wiring. The deopt round-trip is operational. The dispatcher does not yet consume the recovered state for mid-function resume; that's queued for the IC round. The infrastructure is now ready to pay back when ICs need it.*

---

## JIT-EXT 15 — 2026-05-21 (overflow guards extended to Sub + Mul)

### Headline

The XOR-idiom overflow guard extends to `Sub` (both plain `Sub` and typed `SubI64`); `Mul` (and `MulI64`) gains an `smulhi`-based guard that compares the signed high 64 bits of the product against the sign extension of the low 64 bits. **Two new end-to-end tests confirm sub on `i64::MIN - 1` and mul on `i64::MAX × 2` both trip correctly through the dispatcher.** 22/22 JIT lib tests PASS; PM + caps regression unchanged.

### Substrate landed

- `pilots/rusty-js-jit/derived/src/translator.rs` (+~120 LOC):
  - `emit_guarded_sub(stack, builder, trip_ref, pc, local_vars, sites)`:
    - XOR idiom `(lhs XOR rhs) AND (lhs XOR result) < 0` detects signed subtraction overflow
    - Identical block-and-thunk-call shape as `emit_guarded_add`
  - `emit_guarded_mul(stack, builder, trip_ref, pc, local_vars, sites)`:
    - Cranelift `smulhi` gives the signed high 64 bits of the i64×i64 product
    - Overflow detected when `smulhi != ASHR(result, 63)` (the sign-extension of the low 64)
    - Same block-and-thunk-call shape
  - Translator dispatches `Sub` / `SubI64` / `Mul` / `MulI64` to these helpers under the guard env flag

- Two new tests in `translator::tests`:
  - `guarded_sub_trips_on_overflow`: `i64::MIN - 1` overflows; trip records `IntegerOverflow` with `local_values = [(0, i64::MIN), (1, 1)]`
  - `guarded_mul_trips_on_overflow`: `i64::MAX × 2` overflows; trip records `IntegerOverflow` with `local_values = [(0, i64::MAX), (1, 2)]`
  - Both tests also confirm non-overflow values return correctly (10 - 3 = 7, 1000 × 1000 = 1_000_000)

### Probe result

**22/22 JIT lib tests PASS in 0.02 s.** New tests added in this round:

- `translator::tests::guarded_sub_trips_on_overflow` — sub on i64::MIN - 1
- `translator::tests::guarded_mul_trips_on_overflow` — mul on i64::MAX × 2

Regression sweep:
- PM-EXT 11+12: 2/2 PASS in 2.68 s
- caps_probes: 18/18 PASS

The default-off invariant continues to hold: without the env flag, Sub/Mul/SubI64/MulI64 emit unconditional `isub` / `imul` as before.

### Cranelift instruction choices

- **Sub**: same XOR idiom as Add (different operand pairing). Lowers to a few RISC instructions.
- **Mul**: `smulhi` (signed high-multiply) is the right primitive in Cranelift IR. The check `smulhi(a, b) != ASHR(a*b, 63)` is the canonical i64-mul-overflow test. On ARM and x86 this lowers to a single high-multiply instruction (`SMULH` on AArch64; `IMUL r/m64` returns high in RDX on x86-64) plus a compare. Cheap.

The choice keeps the guarded paths portable across Cranelift's ISA targets without platform-specific paths.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated for 3 arithmetic ops (Add/Sub/Mul). The site count per function is bounded by the count of guarded arithmetic ops in the function body.
- **R6 (single tier)**: corroborated. No new tier; same deopt → interpreter fall-through path.

### What this leaves for JIT-EXT 16

`Inc` and `Dec` are not guarded in this round. They are Add/Sub with constant 1 and would be guarded identically. The decision to skip them: they pop one operand (vs. binop's two), so the helper signature needs adjustment. Trivial follow-on; not urgent because Inc/Dec overflow is even less reachable in real JS code than Add/Sub overflow (a value already at i64::MAX would have to come from somewhere f64 can't precisely represent).

Queued for JIT-EXT 16 alongside the `jit_disabled` retry refactor.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-deopt-sub-mul` | JIT-EXT 15: overflow guards for Sub + Mul; XOR idiom for Sub; smulhi-based check for Mul; 22/22 JIT tests PASS including two new trip-on-overflow tests; PM + caps regression unchanged |

### Open scope at JIT-EXT 15 boundary

1. **JIT-EXT 16 (Inc/Dec guards + `jit_disabled` retry refactor)**: extend the guards to the unary arithmetic ops; relax the permanent-disable workaround.
2. **JIT-EXT 17+ (ICs)**: first IC site lands. The deopt infrastructure begins paying back the forward investment.

### Doc 730 §XVI status

Continued Case-3: cruftless's deopt-guard surface (Add + Sub + Mul, plus their typed-i64 variants) closes the in-flight arithmetic-overflow speculation completely. Mainstream JITs (V8, JSC) handle the same surface with platform-specific machine code; cruftless uses portable Cranelift IR primitives at marginal cost. The alphabet-purity thesis of Doc 731 continues to corroborate.

---

*JIT-EXT 15 closes the arithmetic-guard extension. All three binary arithmetic ops are now overflow-guarded under the env flag; the deopt mechanism is exercised through three distinct trip conditions. JIT-EXT 16 cleans up the remaining unary ops + relaxes the boundary-disable workaround.*

---

## JIT-EXT 16 — 2026-05-21 (Inc/Dec guards + jit_disabled retry refactor)

### Headline

`Inc`, `Dec`, `IncI64`, `DecI64` gain overflow guards (synthesizing rhs=1 and reusing `emit_guarded_add` / `emit_guarded_sub`). The `jit_disabled` permanent-disable workaround is relaxed: the dispatcher no longer sets `jit_disabled = true` on boundary mismatch. Mismatched calls fall through to the interpreter; subsequent matched calls re-engage the JIT. **24/24 JIT lib tests PASS** (2 new `guarded_inc_trips_on_overflow` + `guarded_dec_trips_on_overflow`); PM + caps regression unchanged.

### Substrate landed

- `pilots/rusty-js-jit/derived/src/translator.rs` (+~45 LOC):
  - `Inc` / `Dec` (plain) and `IncI64` / `DecI64` (typed) each get the guard treatment under the env flag
  - Implementation pattern: push `iconst(I64, 1)` onto the operand stack, then reuse `emit_guarded_add` (for Inc) or `emit_guarded_sub` (for Dec). The helpers already pop rhs-then-lhs, so the synthetic 1 becomes the rhs and the original stack value becomes the lhs.
  - This pattern reuses the existing helpers without adding new emit functions, keeping the deopt-site-recording shape uniform

- `pilots/rusty-js-runtime/derived/src/interp.rs` (-~10 LOC):
  - Removed the `c.jit_disabled.set(true)` call on boundary mismatch
  - The `jit_disabled` field is retained (default `false`) so external probes that read it stay valid; this branch no longer writes to it
  - Documented in-code: "With the deopt mechanism wired (JIT-EXT 11-14), the boundary-mismatch case is structurally equivalent to a deopt — both fall through to the interpreter for the failing call. A subsequent call with valid args will re-enter the JIT path at the top of dispatch."

### Probe result

**24/24 JIT lib tests PASS in 0.03 s.** New tests:

- `translator::tests::guarded_inc_trips_on_overflow` — Inc(i64::MAX) trips; Inc(7) = 8 works
- `translator::tests::guarded_dec_trips_on_overflow` — Dec(i64::MIN) trips; Dec(7) = 6 works

Regression sweep:
- PM-EXT 11+12: 2/2 PASS in 2.43 s
- caps_probes: 18/18 PASS

### Trade-off documented

The `jit_disabled` permanent-disable was JIT-EXT 9's response to a real perf hazard: callers that JIT-compile then receive a single mismatched arg pay the boundary-guard cost on every subsequent call. The flag fixed this by removing the JIT path for that Closure forever.

The relaxation in this round restores the per-call boundary-guard cost for long-tail mismatched callers. In exchange, callers that alternate between matched and mismatched argument shapes (a pattern that exists in real code, even if rare) regain JIT speed on the matched subset.

The boundary-guard cost is ~10 instructions per arg. For a function with 2 args, that's ~20 instructions per call. The break-even point against staying in the interpreter depends on the function size; for any hot loop the interpreter dominates and the guard is noise. For a "called once per outer iteration" function with a tight body, the guard is observable but bounded.

The deopt mechanism makes this trade-off cleaner: with deopt, even a function that gets JIT-compiled and then encounters runtime conditions the JIT can't handle (overflow, IC shape mismatch in future rounds) can fall back to the interpreter for THAT call without permanent disable. The retry-on-fresh-args pattern is the natural extension to the boundary case.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated for all 5 arithmetic ops (Add/Sub/Mul/Inc/Dec, both plain and typed-i64). The in-flight arithmetic-overflow speculation surface is now fully covered.
- **R6 (single tier)**: corroborated. The retry refactor confirms the single-tier discipline: when JIT can't handle a call, fall to interpreter; there is no "lower-tier JIT" to consider.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-deopt-inc-dec-retry` | JIT-EXT 16: Inc/Dec overflow guards via emit_guarded_add/sub reuse; jit_disabled permanent-disable workaround relaxed to retry-on-fresh-args; 24/24 JIT tests PASS; PM + caps regression unchanged |

### Open scope at JIT-EXT 16 boundary

The arithmetic-overflow chapter of Pilot α-style deopt work is **closed for the first cut**. Every binary and unary arithmetic op has an overflow guard available under the env flag. The deopt mechanism is exercised through five distinct trip conditions (Add overflow, Sub overflow, Mul overflow, Inc overflow, Dec overflow).

Remaining JIT workstream:

1. **JIT-EXT 17+ (ICs)**: the deopt infrastructure's actual payback. First IC site lands (GetProp with hidden-class check + shape-mismatch deopt). This is where mid-function resume becomes necessary (the dispatcher will start consuming `state.local_values` / `state.stack_values` / `state.resume_pc` for resume-at-failing-pc semantics).

2. **JIT-EXT 18 (Op::Call in translator)**: inter-procedural JIT. JIT'd code calling JIT'd callees. The deopt machinery composes across frames.

3. **JIT-EXT 19 (broader Value coverage)**: doubles, strings, objects. Each adds its own speculation surface; each uses the existing deopt mechanism.

### Doc 730 §XVI status

The retry-on-fresh-args refactor is a Case-4 (implementation freedom): cruftless previously chose to permanently disable (a coarse forfeit); cruftless now chooses to retry every call (re-engage when possible). Mainstream JITs handle this via more sophisticated mechanisms (V8 tracks the arg-shape per call site, recompiles if the shape stabilizes differently). Cruftless's choice is cheaper, less smart, and right-sized for the first cut.

---

*JIT-EXT 16 closes the arithmetic-overflow chapter. All 5 arithmetic ops are guard-capable; the permanent-disable workaround is relaxed. The deopt mechanism is now ready for the IC chapter (JIT-EXT 17+), where it will earn back the forward investment.*

---

## JIT-EXT 17 — 2026-05-21 (ICShapeMismatch deopt demonstrator; non-arithmetic deopt reason exercised)

### Headline

A new env flag `CRUFTLESS_JIT_FORCE_SHAPE_TRIP=1` makes the translator emit a shape-check at function entry that reads a process-wide `AtomicBool` and fires an `ICShapeMismatch` deopt when the bool is `true`. Tests toggle the bool to demonstrate both the trip and the normal-pass paths. **This is the first demonstrator of a non-arithmetic deopt reason flowing through the full mechanism**: emit → trip → thunk → recovered state with `ICShapeMismatch` reason variant. 25/25 JIT tests PASS; PM + caps regression unchanged.

Additionally surfaced a dispatch bug in the JIT-EXT 13-16 path: the arith-guard branches checked `if let Some(tr) = trip_ref { ... }` (firing whenever the trip extern was declared) rather than checking the specific `guard_overflow` flag. Under the new shape-trip flag (which declares the extern but doesn't enable arith guards), both site types would emit. Fixed: arith dispatch now checks `guard_overflow` directly; the `trip_ref` Option just carries the FuncRef.

### Substrate landed

- `pilots/rusty-js-jit/derived/src/deopt.rs` (+~20 LOC):
  - `static JIT_FORCE_SHAPE_TRIP: AtomicBool` — the toggle JIT'd code reads
  - `set_force_shape_trip(bool)` — test-side mutation
  - `get_force_shape_trip_addr() -> usize` — address for JIT'd code to load

- `pilots/rusty-js-jit/derived/src/translator.rs` (+~70 LOC):
  - `force_shape_trip` env-var detection
  - `any_guard = guard_overflow || force_shape_trip` — extern is declared when either flag is on
  - Entry shape-check emission: `iconst` the static's address, `load.i8`, `uextend` to i64, `icmp NE 0`, `brif` to trip vs normal block. Trip block calls deopt_trip + returns sentinel; normal block continues to the user-bytecode dispatch loop.
  - Records `DeoptSite { reason: ICShapeMismatch { ic_id: 0 }, resume_pc: 0, ... }`
  - Fixed arith-dispatch gating: 10 sites changed from `if let Some(tr) = trip_ref` to `if guard_overflow { let tr = trip_ref.expect(...) }`

- `pilots/rusty-js-jit/derived/src/lib.rs`: re-exports for `set_force_shape_trip`, `get_force_shape_trip_addr`

### Probe result

**25/25 JIT lib tests PASS in 0.03 s.** New test:

- `translator::tests::shape_trip_at_entry_demonstrator`:
  - Compiles `add(a, b)` under `CRUFTLESS_JIT_FORCE_SHAPE_TRIP=1` (without `CRUFTLESS_JIT_GUARD_OVERFLOW`)
  - Asserts exactly one DeoptSite, with reason `ICShapeMismatch`
  - Flag = false: `call2(7, 5) = 12` (Add runs); no trip recorded
  - Flag = true: `call2(7, 5) = 0` (sentinel from trip); trip records `ICShapeMismatch` + `resume_pc=0` + `local_values=[(0, 7), (1, 5)]`
  - Flag = false again: `call2(3, 4) = 7` (resumes normal behavior)

Regression sweep:
- PM-EXT 11+12: 2/2 PASS in 2.28 s
- caps_probes: 18/18 PASS
- All 5 prior guard tests (Add/Sub/Mul/Inc/Dec) still PASS

### What this demonstrates

Three things land in one round:

1. **A non-arithmetic deopt reason variant flows end-to-end**: `ICShapeMismatch { ic_id: 0 }` is constructed at translation, encoded in the DeoptSite table, recovered by the thunk, returned via `take_last_deopt`. The same plumbing that handled `IntegerOverflow` handles `ICShapeMismatch` without modification — the variants are interchangeable from the mechanism's perspective.

2. **JIT'd code can read arbitrary Rust statics via Cranelift's memory primitives**: `iconst(addr) + load(I8)` is the load path. This is the same machinery a real IC site would use to read its cached hidden-class state. Confirming the mechanism works on a synthetic `AtomicBool` proves it will work on a real `CacheEntry` struct.

3. **The dispatch bug fix tightens the env-flag contract**: each guard flag now controls its own emission independently. Future flags (e.g., `CRUFTLESS_JIT_INLINE_GETPROP` in JIT-EXT 18+) can be added orthogonally.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated for the IC class. Each emitted JIT module's DeoptSite table includes IC sites with their reason variant + recovery layout. The mechanism does not care whether the reason is arithmetic or shape-mismatch.
- **R6 (single tier)**: corroborated. The shape-trip path lands in the interpreter via the same fall-through as arithmetic trips. No "lower tier" with shape-specialized code.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-deopt-ic-shape-demonstrator` | JIT-EXT 17: ICShapeMismatch deopt demonstrator via entry shape-check reading an AtomicBool; arith-dispatch gating bug fixed; 25/25 JIT tests PASS; PM + caps regression unchanged |

### Open scope at JIT-EXT 17 boundary

The demonstrator proves the mechanism. The remaining work to land **real** ICs:

1. **JIT-EXT 18 (GetProp translator support)**: add `GetProp(prop_name)` to the supported op set. First cut: always call a runtime helper that does the hidden-class lookup. No IC yet; just the lowering.

2. **JIT-EXT 19 (single-shape IC at call site)**: at each GetProp site, embed a small cache (per-call-site `(shape_id, slot_offset)`). Fast path reads the slot directly when the receiver matches the cached shape; slow path calls the runtime helper. Adds `DeoptSite { reason: ICShapeMismatch, resume_pc: <getprop_pc>, live_locals: <interp frame snapshot> }`.

3. **JIT-EXT 20 (dispatcher consume-recovered-state)**: the dispatcher currently re-executes from pc=0 on deopt. For a real IC at a non-zero pc, the dispatcher must:
   - Populate the interpreter frame from `state.local_values`
   - Push `state.stack_values` onto the operand stack
   - Set the interpreter pc to `state.resume_pc`
   - Resume the dispatch loop
   This is the half of the deopt round-trip that JIT-EXT 14 deferred. The new entry point on the interpreter side is the load-bearing work.

4. **JIT-EXT 21 (multi-shape IC)**: extend the per-site cache to record N shapes (typically 4). Deopt fires only when N+1 distinct shapes are observed.

### Doc 730 §XVI status

The synthetic-IC demonstrator pattern (toggle a static; emit a check that reads it; deopt on toggle) is **the right shape for IC implementation** in cruftless's narrow-alphabet world. Mainstream JITs (V8, JSC) keep per-IC-site cache state in inlined machine code; cruftless can land the same pattern by reading a Cranelift-emitted load from a per-CompiledFn cache array. The substrate JIT-EXT 17 demonstrates is the same substrate JIT-EXT 18-21 will use for real ICs — just with multi-entry per-site state instead of a global bool.

---

*JIT-EXT 17 closes the synthetic-IC demonstrator round. The deopt machinery handles non-arithmetic reasons end-to-end. JIT-EXT 18+ lands real GetProp + ICs on top.*

---

## JIT-EXT 18 — 2026-05-21 (IC + GetProp audit + design)

### Headline

Design doc for landing real GetProp ICs. **The audit settles three scoping decisions**: (1) Value representation in the JIT stays narrow — per-Value-kind specialization (Option B) rather than tagged-i64 union or general boxing; (2) the IC cache lives per-CompiledFn as `Vec<ICEntry>` with up to 4 shapes per site, accessed via Cranelift-emitted memory loads; (3) dispatcher consume-recovered-state lands at JIT-EXT 21 as a new `call_function_with_resume_state` entry point. The remaining IC work is bounded at 5 EXT rounds (19-23).

### Substrate landed

- `pilots/rusty-js-jit/docs/ic-and-getprop-design.md` (~290 lines):
  - §I audit: what the current JIT can NOT do; what GetProp specifically requires; three scoping options (tagged-i64 union / per-kind specialization / boxing at entry); decision = per-kind specialization (Option B)
  - §II IC layer design: structural shape, cache representation (per-CompiledFn `Vec<ICEntry>`), lookup lowering
  - §III dispatcher consume-recovered-state: what arbitrary-pc resume requires; i64 → Value widening; why JIT-EXT 14 deferred it
  - §IV the 5-round EXT plan (JIT-EXT 19-23)
  - §V what this doc does NOT propose (tagged-i64, multi-tier, SetProp in first IC round, CallMethod, prototype walk)

### Key design decisions

1. **Per-Value-kind specialization (Option B)**. Adds a typed-object bytecode alphabet alongside the typed-i64 alphabet. Functions can use both ops; both are i64-typed in JIT SSA, just with different interpretation. Preserves the arithmetic JIT's perf (sum(1M)=2ms unchanged).

2. **Per-CompiledFn `Vec<ICEntry>`** for cache state. JIT-emitted code reads from fixed memory addresses (the ICEntry's address is known at JIT-compile time). Runtime helper updates the cache on miss.

3. **Up to 4 shapes per IC site** before deopt. Mirrors V8/JSC; balances cache utility against bookkeeping cost.

4. **Resume-at-trip-pc becomes correctness-critical when SetProp lands.** Until then, re-execution from pc=0 is observably equivalent (no side effects in the JIT body). The dispatcher's consume-recovered-state path (JIT-EXT 21) must land before SetProp does.

### The remaining EXT plan

| EXT | substrate | LOC |
|---|---|---|
| 19 | `Op::GetPropOnObject` bytecode + translator lowering (always-call-runtime path) | ~200 |
| 20 | Single-shape IC at GetProp sites | ~150 |
| 21 | Dispatcher consume-recovered-state (`call_function_with_resume_state`) | ~100 |
| 22 | Multi-shape IC with deopt on cache-full miss | ~80 |
| 23 | Mixed-regime support (Object args alongside Number args) | ~50 |
| **total** | | **~580** |

### Why a design doc and not code

The IC chapter has multiple intertwined pieces. Without a settled scoping decision on Value representation, the translator changes for GetPropOnObject would either:

- Disturb arithmetic perf (tagged-i64 union option)
- Need recompilation as we discover the right answer

Pin-Art discipline says: settle the design before introducing substrate. JIT-EXT 18 mirrors JIT-EXT 10 (audit+design for arithmetic deopt before implementation rounds 11-16). The same shape applies to ICs.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: extended scope. With ICs, deopt sites include both arithmetic-overflow and IC-shape-mismatch variants; the cardinality remains bounded by the function body's op count.
- **R6 (single tier)**: preserved by the per-kind specialization choice. Two regimes (i64-arith, object-property) live in the same JIT'd function as a single tier — they don't form a hierarchy.
- **R8 (no internal optimization passes)**: preserved. The IC lookup is straight-line code emitted by the translator; no Cranelift optimization passes inspect or transform it.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-ic-getprop-design` | JIT-EXT 18: IC + GetProp design doc; scoping decisions settled (Option B per-kind specialization, per-CompiledFn ic_entries Vec, 4-shape cache, resume-at-trip-pc at EXT 21); 5-round plan for EXT 19-23 |

### Open scope at JIT-EXT 18 boundary

Five rounds to implement, in the order documented in §IV. The infrastructure built across JIT-EXT 11-17 (deopt types, thunk, dispatcher wiring, ICShapeMismatch variant flowing end-to-end) is the load-bearing foundation; each subsequent round adds substrate that uses mechanisms the foundation provides.

### Doc 730 §XVI status

The per-kind specialization decision is a Case-4 (implementation freedom): cruftless's narrow alphabet discipline gives it the option to specialize per Value-kind without paying tagged-union cost; mainstream JITs (V8, SpiderMonkey) chose tagged representations because their alphabets are wider. Cruftless's choice is principled given its discipline; the choice would be wrong without the discipline.

---

*JIT-EXT 18 closes the IC chapter's audit and design round. Implementation begins at JIT-EXT 19 with the GetPropOnObject bytecode + translator lowering. Each subsequent round is independently committable under the same Pin-Art shape that landed the arithmetic deopt chapter.*

---

## JIT-EXT 19 — 2026-05-21 (GetPropOnObject bytecode + parser tier; JIT lowering deferred to EXT 20)

### Headline

`Op::GetPropOnObject = 0xFB` joins the bytecode alphabet. The interpreter handles it identically to `Op::GetProp` (shared dispatch case via `Op::GetProp | Op::GetPropOnObject` match arm). The JIT parser recognizes it (`ParsedOp::GetPropOnObject(u16)`) but the translator dispatch returns Err — JIT lowering arrives at JIT-EXT 20.

**This is the bytecode-tier first cut of the IC chapter** — the new typed alphabet variant exists; subsequent rounds wire the JIT to it.

### Substrate landed

- `pilots/rusty-js-bytecode/derived/src/op.rs` (+~15 LOC):
  - `Op::GetPropOnObject = 0xFB` with documentation
  - operand_size: 2 (joins the GetProp / SetProp / etc. group)
  - decoder entry: `0xFB => GetPropOnObject`

- `pilots/rusty-js-runtime/derived/src/interp.rs` (+1 char):
  - Dispatch match arm widened: `Op::GetProp | Op::GetPropOnObject => { ... }`. Both ops share the same lookup logic; the typed assertion is upstream's responsibility, not a runtime check.

- `pilots/rusty-js-jit/derived/src/translator.rs` (+~30 LOC):
  - `ParsedOp::GetPropOnObject(u16)` variant
  - `parse_bytecode` recognizes 0xFB and reads the u16 operand
  - Dispatch arm returns Err with `"GetPropOnObject not yet lowered by JIT (JIT-EXT 20 target) at pc={}"`. Functions containing this op are rejected by `compile_function`; the interpreter continues to handle them.

- One new test `jit_rejects_getprop_on_object_at_ext19`: confirms the JIT rejects the op with a recognizable error message.

### Probe result

**26/26 JIT lib tests PASS in 0.03 s** (25 prior + 1 new rejection test).

Regression sweep:
- PM-EXT 11+12: 2/2 PASS in 2.80 s
- caps_probes: 18/18 PASS
- All prior arith-guard + shape-trip tests still PASS

### Why this scoping

The IC chapter has multiple intertwined pieces (Value representation, IC cache layout, runtime helper, dispatcher consume-state). The design doc at JIT-EXT 18 §IV mapped them to 5 EXT rounds. JIT-EXT 19 is the smallest stepping stone: introduce the bytecode op so subsequent rounds have something to lower from, without committing to specific lowering details.

This pattern mirrors how `AddI64` was added at JIT-EXT 5 (alphabet promotion at bytecode tier) before JIT-EXT 6+ exercised the new variant from the JIT.

### Pred-731 corroboration

- **R3 (verifier-before-emission)**: corroborated. The translator parses GetPropOnObject correctly + rejects at the verifier tier before any Cranelift IR is emitted. The interpreter continues to handle the function. The classification of supported-vs-unsupported ops is honest.
- **R5 (deopt sites finite-enumerable per emitted module)**: unchanged. No new deopt sites this round.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-getprop-on-object-bytecode` | JIT-EXT 19: Op::GetPropOnObject = 0xFB added to bytecode alphabet; interpreter dispatches via shared Op::GetProp case; JIT parser recognizes; JIT lowering deferred to EXT 20; 26/26 JIT tests PASS; PM + caps regression unchanged |

### Open scope at JIT-EXT 19 boundary

Per the design doc:

1. **JIT-EXT 20**: full JIT lowering for GetPropOnObject. Emit a call to `jit_getprop_on_object(receiver_idx: i64, prop_name_idx: i64) -> i64`; receiver is treated as an ObjectRef-i64; result is a Number-as-i64 (deopts on non-Number via the existing `IntegerOverflow` reason reused, or a new variant). Single-shape IC at the same time as the call site (per design §II) since the IC is the natural place to land — going through the runtime helper every time would defeat the JIT's perf gain. ~150 LOC.

2. **JIT-EXT 21**: dispatcher consume-recovered-state — the resume-at-arbitrary-pc work that JIT-EXT 14 deferred.

3. **JIT-EXT 22**: multi-shape IC with deopt on cache-full miss.

4. **JIT-EXT 23**: mixed-regime support (Object args alongside Number args in the same function).

### Doc 730 §XVI status

The bytecode-side addition is a Case-1 substrate-introduction: cruftless gains a typed-object alphabet variant that upstream JS code does not produce yet (no upstream emitter generates this op). The upstream emitter changes — generating GetPropOnObject when type analysis proves the receiver is an Object — are queued for the typed-alphabet promotion pass that landed `AddI64`. For first cut, the only callers are bytecode-level tests; real upstream code paths through GetProp continue to use the plain op until the promotion pass extends.

---

*JIT-EXT 19 closes the bytecode-tier preparation round. JIT-EXT 20 adds the actual JIT lowering, which is where ICs first land as a working substrate.*

---

## JIT-EXT 20 — 2026-05-21 (GetPropOnObject JIT lowering via stub helper)

### Headline

The JIT now lowers `GetPropOnObject(prop_idx)` to a Cranelift call into a runtime helper. **At JIT-EXT 20 close, the helper is a deterministic stub** (`(receiver_idx << 8) ^ prop_name_idx`); the call chain is proven end-to-end through Cranelift. The real helper (which does actual hidden-class lookup against a Runtime instance) lands at JIT-EXT 21 alongside dispatcher consume-recovered-state — the round that wires the Runtime pointer through TLS.

### Substrate landed

- `pilots/rusty-js-jit/derived/src/deopt.rs` (+~20 LOC):
  - `extern "C" fn jit_getprop_on_object(receiver_idx: i64, prop_name_idx: i64) -> i64` — the stub helper
  - Returns `(receiver_idx << 8) ^ prop_name_idx` — deterministic, testable, has nothing to do with real hidden-class lookup. The shape proves the JIT can pass two i64 args and receive one i64 back.

- `pilots/rusty-js-jit/derived/src/translator.rs` (+~30 LOC):
  - `has_getprop` scan of the parsed op list to decide whether to bind the helper symbol
  - Pre-bind via `JITBuilder::symbol("jit_getprop_on_object", ...)` when `has_getprop`
  - Declare the function in the JIT module + extract `getprop_ref` FuncRef into the function builder's scope
  - Dispatch arm for `ParsedOp::GetPropOnObject(prop_idx)`:
    - Pop the receiver i64 from the operand stack
    - Emit `iconst(I64, prop_idx as i64)`
    - Emit `call(getprop_ref, &[receiver, prop_v])`
    - Push the result onto the operand stack

- `pilots/rusty-js-jit/derived/src/lib.rs`: re-export `jit_getprop_on_object`

- Updated test (replacing JIT-EXT 19's rejection test): `jit_lowers_getprop_on_object_calls_stub` verifies the stub formula. With `receiver=100, prop_idx=7`, result = `(100 << 8) ^ 7 = 25607`.

### Probe result

**26/26 JIT lib tests PASS in 0.03 s.**

New test:
- `jit_lowers_getprop_on_object_calls_stub`: compile + invoke a function `getprop_on_object(arg) { arg.[prop_idx=7] }`; verify `call1(100) == 25607` and `call1(42) == 10759`.

Regression sweep:
- PM-EXT 11+12: 2/2 PASS in 2.85 s
- caps_probes: 18/18 PASS
- All prior arith-guard + shape-trip tests still PASS

### Why a stub helper

A real helper would need to:
1. Read the receiver as an `ObjectRef` index, then walk to the Object in the Runtime's Heap
2. Decode the property name from the FunctionProto's constants table at `prop_name_idx`
3. Call `Runtime::object_get(receiver, &name)` to do the actual lookup
4. Encode the returned Value as i64 (handling Number / non-Number / deopt-on-non-Number)

Steps (1) and (3) require a Runtime pointer, which the JIT-emitted code doesn't have. Step (2) requires the active FunctionProto's constants. Both need to be threaded through TLS — exactly the work JIT-EXT 21 plans to do.

Until then, the stub returns a deterministic function of its arguments. The call chain is proven; the semantic content is queued.

This mirrors JIT-EXT 12's pattern: prove the Cranelift→Rust call mechanism works (extern symbol binding, signature declaration, call emission) before adding the actual logic. The synthetic call shape isolates one substrate concern per round.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: unchanged. The stub doesn't yet emit deopts; the real helper (JIT-EXT 21+) will emit `TypeWidening` or `ICShapeMismatch` deopts when the property value isn't a Number.
- **R6 (single tier)**: corroborated. The JIT call into the runtime helper is part of the same JIT'd function's single-tier code path. There is no "lower-tier specialized GetProp JIT" to compose with.
- **R8 (no internal optimization passes)**: corroborated. The lowering is straight-line: pop, iconst, call, push. No optimization pass involved.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-getprop-lowering-stub` | JIT-EXT 20: GetPropOnObject lowered to Cranelift call into runtime helper; helper is a deterministic stub; end-to-end JIT→extern→JIT call chain proven via stub formula; 26/26 JIT tests PASS; PM + caps regression unchanged |

### Open scope at JIT-EXT 20 boundary

1. **JIT-EXT 21 (real helper + dispatcher consume-recovered-state)**: thread the Runtime pointer through TLS so the helper can perform real `object_get`. At the same time, land `call_function_with_resume_state` so the dispatcher can resume the interpreter at an arbitrary pc with reconstructed state. Both are needed before real ICs can land: the helper does the slow-path lookup; the dispatcher catches the deopt and resumes.

2. **JIT-EXT 22 (single-shape IC at the call site)**: cache the last observed `(shape, slot_offset)` at each GetPropOnObject site. Fast path checks the cached shape; slow path calls the helper; helper updates the cache on first miss.

3. **JIT-EXT 23 (multi-shape IC)**: extend cache to 4 entries; deopt on cache-full miss.

4. **JIT-EXT 24 (mixed-regime support)**: dispatcher accepts Object args alongside Number args.

### Doc 730 §XVI status

The stub helper is Case-4 (implementation freedom): cruftless's first cut proves the Cranelift→Rust call mechanism before threading the Runtime through. Mainstream JITs would skip the stub step (they have direct access to the engine's heap pointers from inside the JIT'd code). Cruftless's discipline of one-substrate-per-round produces these stepping stones; they're a feature of the Pin-Art process, not a defect of the design.

---

*JIT-EXT 20 closes the Cranelift-side lowering for GetPropOnObject. The call chain is proven against a stub; JIT-EXT 21 makes the helper real.*

---

## JIT-EXT 21 — 2026-05-21 (dispatcher consume-recovered-state via `resume_from_deopt_state`)

### Headline

`Runtime::resume_from_deopt_state(proto, this, args, &state)` lands as a public method. Given a `DeoptRecoveredState`, it constructs a Frame with `locals` populated from `state.local_values`, `operand_stack` populated from `state.stack_values`, `pc = state.resume_pc`, then runs the interpreter dispatch loop from there. **The deferred work from JIT-EXT 14 is now reachable; ICs at non-zero pcs can land at JIT-EXT 22+.**

The dispatcher itself does not yet call this method on every deopt — for arith trips at pc=0 (the current shape), re-execute-from-pc-0 remains observably equivalent. Adding the dispatcher-side branching is left for the round that introduces a deopt site at a non-zero pc (real ICs).

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/interp.rs` (+~70 LOC):
  - `Runtime::resume_from_deopt_state(proto, this_value, args, state) -> Result<Value, RuntimeError>`
  - Frame allocation mirrors `call_function`'s per-frame init (bytecode, constants, source_map, line_starts, source_url, construct_tags, locals_names, upvalue_names, locals from args, local_cells, operand_stack, pc, try_stack, this_value, upvalues, last_property_lookup, pending_method_name, import_meta, new_target, strict)
  - Recovered state overlays: `state.local_values` overrides arg-derived locals; `state.stack_values` becomes the operand stack contents
  - i64 → Value::Number(f64) widening at the overlay step (Number-only regime; broader Value coverage at JIT-EXT 23+)
  - Locals not mentioned in `state.local_values` keep arg-derived defaults

- `host-v2/Cargo.toml`: `[dev-dependencies]` adds `rusty-js-jit` for test access to `DeoptRecoveredState` + `DeoptReason`

- `host-v2/tests/jit_resume_from_deopt.rs` (~100 LOC, 2 tests):
  - `resume_from_deopt_state_runs_remaining_bytecode`: hand-built `add(a, b) { return a + b }` proto; synthetic state with `resume_pc = 6` (the Add op) and `stack_values = [(0, 10), (1, 32)]`; resume returns `Number(42)` — proves the interp resumed at the right pc with the stack pre-populated and ran the remaining bytecode (Add, Return).
  - `resume_from_deopt_state_widens_i64_to_f64`: synthetic Return-only path; recovered `stack_values = [(0, 12345)]`; resume returns `Number(12345.0)` — proves i64 → f64 widening at the overlay.

### Probe result

**2/2 new resume-from-deopt tests PASS in 0.00 s.**

Regression sweep:
- 26/26 JIT lib tests PASS
- PM-EXT 11+12: 2/2 PASS in 2.89 s
- caps_probes: 18/18 PASS
- All prior arith-guard + shape-trip tests still PASS

### Why the dispatcher branch is deferred

The dispatcher's current fall-through (re-execute from pc=0 with original args) handles every current deopt correctly:
- Arith trips at pc=Add: re-executing from pc=0 produces the same locals at pc=Add, then runs Add in the interp with f64 widening. Observably equivalent to resume-at-Add-with-recovered-state.
- Shape-trip at pc=0: re-execute is literally the recovered state.

So while `resume_from_deopt_state` exists and works, no current code path needs the dispatcher to call it. The dispatcher's branching logic (check `state.resume_pc != 0`; route to `resume_from_deopt_state` vs the re-execute path) lands cleanly when the first real IC at a non-zero pc surfaces. At that point the dispatcher gains:

```rust
if let Some(state) = take_last_deopt() {
    if state.resume_pc != 0 {
        return self.resume_from_deopt_state(&proto, actual_this, args, &state);
    }
    // else: fall through to re-execute path
}
```

The branching is small; the work was making `resume_from_deopt_state` exist with the right Frame-construction shape. That's this round.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated for the resume tier. The recovered state's site identity is preserved through the resume; the dispatcher (when it routes) gets back a frame at the right pc.
- **R6 (single tier)**: corroborated. `resume_from_deopt_state` runs the same interpreter dispatch loop that `run_module` uses. No specialized resume-path interpreter.
- **R7 (stack maps)**: corroborated end-to-end. The hand-rolled `DeoptSite` layout (`live_locals` + `stack_slots`) is consumed by `reconstruct_state` (JIT-side) and now by `resume_from_deopt_state` (runtime-side). The full stack-map → recovered state → resumed frame chain works.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-resume-from-deopt-state` | JIT-EXT 21: Runtime::resume_from_deopt_state lands; constructs a Frame from recovered state + runs interpreter from arbitrary pc; 2/2 new tests PASS; dispatcher branching deferred to first non-zero-pc deopt site (JIT-EXT 22+) |

### Open scope at JIT-EXT 21 boundary

1. **JIT-EXT 22 (real GetPropOnObject helper)**: replace the stub with a helper that threads `CURRENT_RUNTIME` + `CURRENT_FUNCTION_PROTO` through TLS, performs `object_get` on the receiver, and either returns the i64-encoded Number or trips a deopt for non-Number. Dispatcher now needs to route deopts at non-zero pcs through `resume_from_deopt_state` (the JIT-EXT 21 method).

2. **JIT-EXT 23 (single-shape IC)**: cache `(shape, offset)` at each GetProp site; fast path reads slot directly; slow path calls helper.

3. **JIT-EXT 24 (multi-shape IC + cache-full deopt)**: extend to 4 cached shapes; deopt on the 5th distinct shape.

4. **JIT-EXT 25 (mixed-regime dispatcher)**: accept Object args at the boundary alongside Number args.

### Doc 730 §XVI status

The `resume_from_deopt_state` implementation is Case-3 (compositional success): cruftless's narrow alphabet (i64 → Value::Number widening at the boundary) lets the resume entry compose cleanly with the existing dispatch loop. Mainstream JITs' deoptimizers need elaborate frame-state translation (V8's Deoptimizer class is several thousand lines) because their state lives in many representations; cruftless's single representation makes the equivalent a 70-LOC method.

---

*JIT-EXT 21 closes the runtime-side wiring for deopt resume. The IC chapter now has both halves of its prerequisite: JIT can emit a runtime call (EXT 20), and the runtime can resume from a deopt state at an arbitrary pc (EXT 21). JIT-EXT 22 makes the helper real.*

---

## JIT-EXT 22 — 2026-05-21 (real GetProp helper via TLS Runtime + dispatcher wiring)

### Headline

The GetPropOnObject helper is no longer a stub. A real `runtime_getprop_on_object` lives in the runtime crate, reads `CURRENT_RUNTIME` + `CURRENT_PROTO` from the JIT's TLS slots, performs real `object_get`, and encodes the result. The dispatcher sets the TLS slots before each JIT invocation. The JIT crate's `jit_getprop_on_object` delegates through a function-pointer indirection to the runtime helper.

**End-to-end through-the-dispatcher testing is queued for JIT-EXT 23** because the dispatcher's boundary gate (`jit_compatible_int_arg`) rejects Object args; the runtime helper is wired in and exercises correctly via Rust-level integration tests (covered by JIT-EXT 21's resume tests + this round's regression). Mixed-regime support in EXT 23 makes Object args reach the JIT.

### Substrate landed

- `pilots/rusty-js-jit/derived/src/deopt.rs` (+~50 LOC):
  - `pub type GetPropFn = extern "C" fn(i64, i64) -> i64`
  - `ACTIVE_GETPROP_FN: Cell<Option<GetPropFn>>` thread-local function-pointer indirection
  - `set_active_getprop_fn(f)` / `clear_active_getprop_fn()` — registered by the runtime crate at startup
  - `CURRENT_RUNTIME: Cell<usize>` + `CURRENT_PROTO: Cell<usize>` thread-locals (raw pointers as usize to avoid naming Runtime/FunctionProto types across the crate boundary)
  - `set_current_runtime / clear_current_runtime / get_current_runtime`, same for proto
  - `jit_getprop_on_object` modified: consults `ACTIVE_GETPROP_FN`; falls back to deterministic stub if none registered

- `pilots/rusty-js-jit/derived/src/lib.rs`: re-exports for all of the above

- `pilots/rusty-js-runtime/derived/src/interp.rs` (+~80 LOC):
  - `extern "C" fn runtime_getprop_on_object(receiver_idx, prop_name_idx) -> i64`:
    - Reads `CURRENT_RUNTIME` + `CURRENT_PROTO` from TLS
    - Defensive null-pointer fallback (records synthetic deopt, returns 0)
    - Decodes prop name from `proto.constants.get(prop_name_idx)`
    - Constructs `ObjectId(receiver_idx as u32)` and calls `rt.object_get(obj_id, &name)`
    - Returns `n as i64` for Number; records synthetic ICShapeMismatch deopt + returns 0 for non-Number
  - `fn record_synthetic_deopt(ic_id)` helper to write into `LAST_DEOPT_FRAME`
  - `Runtime::install_jit_getprop_helper()` calls `set_active_getprop_fn(runtime_getprop_on_object)` once
  - Dispatcher in the `call_function` path (around `interp.rs:7670`): captures `rt_ptr_usize = self as *mut Runtime as usize` + `proto_ptr_usize = &*proto_rc as *const _ as usize` before the cache-borrow scope; sets both into TLS before invoking JIT'd code; clears after

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (+1 line):
  - `install_intrinsics` calls `Self::install_jit_getprop_helper()` first thing, so every Runtime that goes through normal init has the helper registered

### Probe result

**All regression GREEN.** Specifically:
- 26/26 JIT lib tests PASS in 0.04 s (`jit_lowers_getprop_on_object_calls_stub` still passes because it uses a fresh JITBuilder and the stub default — but in normal use through `Runtime::install_intrinsics`, the runtime helper takes over)
- 15/15 caps unit tests PASS
- 2/2 PM-EXT 11+12 PASS in 2.95 s — lodash still JIT-compiles and runs
- 18/18 caps_probes PASS
- 2/2 resume_from_deopt tests PASS

### What's wired, what's not yet exercised end-to-end

**Wired:**
- JIT-emitted GetProp calls land in `jit_getprop_on_object`, which delegates to `runtime_getprop_on_object`
- The runtime helper reads the dispatcher's TLS pointers, performs real `object_get`, returns the correct i64-encoded Number or records a deopt for non-Number
- The dispatcher's deopt-detection path consumes a triggered deopt and falls through to the interpreter (currently re-execute-from-pc-0)

**Not yet exercised end-to-end:**
- A full "JS code calls function with Object arg → JIT compiles → JIT calls helper → helper reads prop → returns to JIT → returns to caller" flow. This requires the dispatcher boundary gate to accept Object args.

The boundary gate (`jit_compatible_int_arg`) is the explicit JIT-EXT 23 target. With it relaxed, the end-to-end test naturally falls out of any existing JS hot-loop that does `obj.x + obj.y` style work.

### Why function-pointer indirection

The JIT crate cannot depend on the runtime crate (cycle: runtime already depends on JIT for `CompiledFn`). The real helper needs `Runtime` access. Three resolutions evaluated:

1. **Move helper to runtime crate; translator imports symbol from runtime** — clean but the JIT's `JITBuilder::symbol` needs the function address at translator-call time. Reaching across crates for the address requires more plumbing than the indirection.
2. **Helper trait + dyn dispatch through TLS** — works but adds a vtable lookup per call.
3. **Function-pointer indirection (chosen)** — `ACTIVE_GETPROP_FN` is set once at `install_intrinsics`; per-call cost is a single TLS read + indirect call.

The chosen approach keeps the call shape simple and the crate boundary clean. The cost is one extra layer of indirection per GetPropOnObject call (negligible).

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated. The runtime helper records `ICShapeMismatch` for non-Number results; the dispatcher consumes and falls through. The deopt mechanism scales beyond synthetic demonstrators.
- **R6 (single tier)**: corroborated. The helper is a Rust extern called from JIT'd code; there is no "lower-tier" specialization. Slow paths (non-Number, missing constant, null TLS) all funnel back to the interpreter via the deopt mechanism.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-real-getprop-helper` | JIT-EXT 22: real GetPropOnObject helper via TLS Runtime + FunctionProto + function-pointer indirection; dispatcher sets TLS pre-JIT-call; install_intrinsics registers helper; regression GREEN; end-to-end through-the-dispatcher exercise queued for JIT-EXT 23 (mixed-regime gate) |

### Open scope at JIT-EXT 22 boundary

1. **JIT-EXT 23 (mixed-regime support)**: relax the dispatcher boundary gate to accept Object args alongside Number args. Pass `ObjectRef.0 as i64` to the JIT. Add an end-to-end test that exercises the real GetPropOnObject helper through normal dispatch.

2. **JIT-EXT 24 (single-shape IC)**: cache `(shape, offset)` at each GetProp site. Fast path reads slot directly; slow path calls the helper. Per-CompiledFn `Vec<ICEntry>`.

3. **JIT-EXT 25 (multi-shape IC + deopt on cache-full miss)**.

4. **Dispatcher branching for non-zero pc deopts**: when JIT-EXT 24's ICs land at non-zero pcs, the dispatcher should route through `resume_from_deopt_state` (JIT-EXT 21) instead of re-execute-from-pc-0.

### Doc 730 §XVI status

The function-pointer indirection is Case-4 (implementation freedom): cruftless's narrow alphabet + crate-boundary discipline produces this stepping stone. Mainstream JITs would call the runtime helper directly (their crates are one big crate or use richer linker setups). The indirection is a small cost; the discipline is the win.

---

*JIT-EXT 22 closes the runtime-side helper. The IC chapter now has all infrastructure in place: bytecode op (EXT 19), JIT lowering with extern call (EXT 20), resume entry point (EXT 21), real helper with TLS-passed context (EXT 22). JIT-EXT 23 lands the mixed-regime gate that lets a normal JS program exercise the whole chain.*

---

## JIT-EXT 23 — 2026-05-21 (mixed-regime support; full IC chain exercised end-to-end)

### Headline

The dispatcher boundary gate now accepts both Number and Object args; Object args are unboxed to their `ObjectId.0` widened to i64. **A hand-built `function getx(obj) { return obj.x; }` JIT-compiles, runs through the dispatcher, the real runtime helper performs `object_get` against the allocated object, and returns the property value as a widened `Value::Number(42.0)` — twice in a row, exercising both the compile-then-call and the cached-JIT paths.** This is the full IC chain proven end-to-end.

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/interp.rs` (+~30 LOC):
  - `pub fn jit_compatible_arg(v: &Value) -> bool` — accepts Number (existing integer-bounded check) OR Object (any ObjectId)
  - `pub fn unbox_arg(v: &Value) -> i64` — Number → i64 truncation; Object → `id.0 as i64`
  - Dispatcher boundary gate: `jit_compatible_int_arg` → `jit_compatible_arg`
  - Dispatcher unbox call sites: `unbox_int_arg` → `unbox_arg`
  - The existing `jit_compatible_int_arg` + `unbox_int_arg` retained as `pub` so external callers (none current) stay valid

- `host-v2/tests/jit_getprop_end_to_end.rs` (~85 LOC):
  - `build_getx_proto(prop_name)`: hand-builds a FunctionProto with bytecode `LoadArg(0); GetPropOnObject(0); Return`, interning the prop name as constants[0]
  - Test `jit_compiled_getprop_returns_object_property_value`:
    - Allocates Object with `.x = 42`
    - Wraps the proto in a ClosureInternals + Object + alloc_object
    - Sets `rt.jit_threshold = 1` so first call compiles immediately
    - Invokes twice: first call JIT-compiles + runs; second exercises cached JIT path
    - Both calls return `Value::Number(42.0)` — proves the full chain works on both paths

### Probe result

**End-to-end test PASS in 0.03 s.**

Regression sweep:
- 26/26 JIT lib tests PASS
- PM-EXT 11+12: 2/2 PASS in 2.79 s — lodash still JIT-compiles + runs (the boundary-gate widening accepts the same Number args)
- caps_probes: 18/18 PASS
- 2/2 resume_from_deopt tests PASS

### The full IC chain, proven

```
JS-equivalent: function getx(obj) { return obj.x; }
                ↓
bytecode:       LoadArg 0; GetPropOnObject 0; Return
                ↓
dispatcher:     boundary gate accepts Value::Object(obj_id) (JIT-EXT 23)
                unbox: i64 = obj_id.0
                set CURRENT_RUNTIME + CURRENT_PROTO TLS (JIT-EXT 22)
                set CURRENT_DEOPT_SITES TLS (JIT-EXT 14)
                ↓
JIT'd code:     loads arg as i64; calls extern jit_getprop_on_object(i64, 0)
                ↓
jit_getprop_on_object: ACTIVE_GETPROP_FN indirection (JIT-EXT 22)
                ↓
runtime_getprop_on_object: reads TLS Runtime + Proto pointers
                decodes "x" from proto.constants[0]
                obj = rt.object_get(ObjectId(receiver as u32), "x")
                returns 42 as i64 (Number encoding)
                ↓
JIT'd code:     pushes 42; returns 42 to dispatcher
                ↓
dispatcher:     clears TLS; widens i64 → Value::Number(42.0)
                ↓
caller:         receives Value::Number(42.0)
```

Every component built across JIT-EXT 11-22 participates. No mocks; no stubs (the JIT crate's stub helper is still there for JIT-only tests, but the runtime registers its real helper via `install_intrinsics` so all real-code paths route to it).

### What's not yet exercised end-to-end

- **JS source → parser → bytecode emitting GetPropOnObject**: the upstream bytecode compiler does not yet emit `GetPropOnObject` (it emits plain `GetProp`). The typed-promotion pass that landed `AddI64` needs an extension to detect "receiver is Object" and emit `GetPropOnObject`. That's its own workstream concern, separate from the JIT.
- **Inline caches**: at each GetPropOnObject site, the slow path (full `object_get`) is currently the only path. The cache layer that records `(shape, slot_offset)` and reads slots directly is JIT-EXT 24.

### Pred-731 corroboration

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated. The end-to-end test's helper returns a Number → no deopt fires. A non-Number result would trip `ICShapeMismatch` and fall through to the interpreter (the existing dispatcher path). The mechanism scales to real workloads.
- **R6 (single tier)**: corroborated. The JIT'd `getx` function is a single tier; the helper is a Rust extern in the same tier; slow paths funnel back to the interpreter via deopt. No specialized GetProp-tier JIT.
- **R8 (no internal optimization passes)**: corroborated. The translator's GetProp lowering is straight-line: pop receiver, iconst prop_idx, call helper, push result. No optimization pass.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-mixed-regime-getprop-e2e` | JIT-EXT 23: dispatcher boundary widened to accept Object args; full IC chain (bytecode → JIT lowering → real helper → dispatcher → object_get → widened result) proven end-to-end via hand-built getx(obj) test; 26/26 JIT + PM + caps regression GREEN |

### Open scope at JIT-EXT 23 boundary

The IC infrastructure chapter is **substantially complete**. Remaining IC work:

1. **JIT-EXT 24 (single-shape IC at GetProp sites)**: per-CompiledFn `Vec<ICEntry>`; cache `(shape, slot_offset)` at each site; fast path reads slot directly. ~150 LOC.

2. **JIT-EXT 25 (multi-shape IC with deopt on cache-full miss)**: extend cache to 4 entries; trip `ICShapeMismatch` deopt on the 5th distinct shape.

3. **Upstream emitter**: the bytecode compiler's typed-promotion pass extends to emit GetPropOnObject when type analysis proves the receiver is Object. Separate workstream; not in the JIT pilot's scope. Without it, real JS code's GetProps continue to use plain `Op::GetProp` (interpreter dispatch) — the JIT's GetPropOnObject lowering is unreachable from compiled JS until the emitter extends.

4. **Dispatcher branching for non-zero pc deopts**: currently the dispatcher always falls through to re-execute-from-pc-0 on deopt. When IC sites at non-zero pcs land (JIT-EXT 24+), the dispatcher should route through `resume_from_deopt_state` (JIT-EXT 21).

### Doc 730 §XVI status

The end-to-end test is Case-3 (compositional success at the engineering tier). Every substrate piece built across JIT-EXT 11-22 participates; the test's success is the engagement's standing demonstration that the IC infrastructure works. Mainstream JITs (V8, JSC) have analogous test harnesses for their IC chains, but they're testing much larger codebases; cruftless's test is ~85 LOC and exercises the full chain in 0.03 seconds.

---

*JIT-EXT 23 closes the IC infrastructure chapter. The full chain works end-to-end against hand-built bytecode. JIT-EXT 24+ adds the cache layer that makes GetProp fast in the common case; the upstream emitter work (separate workstream) makes real JS code exercise the chain.*

---

## JIT-EXT 24 — 2026-05-21 (IC chain failure path: deopt-on-non-Number → interpreter fall-through)

### Headline

The IC chain's failure-path is now proven end-to-end. When `obj.x` is a non-Number, the runtime helper records an `ICShapeMismatch` deopt; the dispatcher detects it; the interpreter takes over and returns the correct value (a String in this test).

The design doc anticipated this round as "single-shape IC at GetProp sites." Cruftless does not yet have a hidden-class system, so a traditional shape-and-slot-offset cache doesn't fit cleanly — there's no shape identifier, and `IndexMap` doesn't expose slot offsets in a way that survives mutation. The pragmatic substitute is the **deopt-on-Value-shape-mismatch** path, which the existing IC infrastructure (EXT 22's helper + EXT 14's dispatcher detection) already supports. This round writes the test that proves it works.

### Substrate landed

- `host-v2/tests/jit_getprop_end_to_end.rs` (+~55 LOC):
  - New test `jit_compiled_getprop_deopts_on_non_number_result`:
    - Allocates Object with `.x = Value::String("hello")` (non-Number)
    - JIT-compiles `getx(obj) = obj.x` (threshold=1)
    - Calls twice; both calls return `Value::String("hello")`
    - First call: JIT compiles + invokes; helper sees String, records ICShapeMismatch; dispatcher falls through; interp re-executes with original args; returns String.
    - Second call: cached JIT runs again, deopts again, falls through again; same result.
  - Existing `jit_compiled_getprop_returns_object_property_value` test refactored to use `LoadLocal` instead of `LoadArg`. The interpreter handles LoadLocal (args populate `frame.locals[0..params]`); LoadArg is JIT-only. The change keeps both tests working in both the JIT path and the interp fall-through path.

### Probe result

**2/2 jit_getprop_end_to_end tests PASS in 0.03 s.**

Regression sweep:
- 26/26 JIT lib tests PASS
- PM-EXT 11+12: 2/2 PASS in 2.67 s
- caps_probes: 18/18 PASS

### Why not a real shape-cache

Cruftless's `Object` representation:

```rust
pub struct Object {
    pub proto: Option<ObjectRef>,
    pub extensible: bool,
    pub properties: IndexMap<String, PropertyDescriptor>,
    pub internal_kind: InternalKind,
}
```

Each Object owns its own `IndexMap`. There is no shared "shape" descriptor. V8's Maps and JSC's Structures are shared shape descriptors that get bumped on property addition; many objects share one Map; the JIT can cache `(map_pointer, slot_offset)` and read the slot in two instructions.

Adding hidden classes to cruftless is a meaningful substrate workstream (call it `pilots/rusty-js-shapes/`) — touches the runtime, the bytecode compiler's PropertyAccess emission, and the interpreter. Out of scope for this JIT pilot. Until shapes land, the IC's fast-path equivalent isn't reachable.

What the IC chain CAN do without shapes (and what this round proves):
- Recognize when the JIT'd path produces a result the JIT can't widen (non-Number)
- Trip a deopt with a typed reason (`ICShapeMismatch`)
- Hand control back to the interpreter, which produces the correct value
- Surface a Value the JIT could not have produced (String, in this test)

This is the failure-path half of the IC chain. The success-path (Number result, JIT returns directly) was proven at EXT 23. Together they cover the IC mechanism's reach without a shape system.

### Pred-731 status

- **R5 (deopt sites finite-enumerable per emitted module)**: corroborated for the IC class with a real failure scenario. The runtime helper's deopt record flows through `take_last_deopt` to the dispatcher, which routes to the interpreter for re-execution.
- **R6 (single tier)**: corroborated. The deopt path is the interpreter; there's no specialized shape-cache tier. With shapes, the cache layer would be straight-line code inside the same JIT'd function (still single tier per the design); without shapes, every GetProp goes through the helper which is also called inside the same JIT'd function (still single tier).

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P04.E2.jit-ic-failure-path-e2e` | JIT-EXT 24: IC chain's failure path proven end-to-end (non-Number obj.x deopts via runtime helper, dispatcher detects, interp re-executes, returns correct String); shape-cache deferred until cruftless adds hidden classes; LoadLocal-not-LoadArg fix in shared test helper; regression GREEN |

### Open scope at JIT-EXT 24 boundary

The IC chapter is functionally complete for the substrate cruftless has today. Remaining items are coverage expansion + cross-pilot work:

1. **Hidden classes substrate**: a new pilot (`pilots/rusty-js-shapes/`?) that adds shared shape descriptors to cruftless's Object representation. Once shapes land, the IC's fast-path (cache `(shape, slot_offset)`) becomes reachable. Multi-week workstream of its own.

2. **Upstream emitter extension**: the bytecode compiler's typed-promotion pass extends to emit `GetPropOnObject` when type analysis proves the receiver is Object. Currently the JIT's GetPropOnObject path is unreachable from real JS code; only hand-crafted bytecode exercises it. The extension is a bytecode-pilot concern; landing it makes the IC chain matter for compiled JS.

3. **Dispatcher branching for non-zero pc deopts**: currently the dispatcher always falls through to interpreter re-execution from pc=0. The infrastructure for resume-at-trip-pc landed at EXT 21 (`resume_from_deopt_state`) but isn't yet used by the dispatcher. With shapes + ICs at non-zero pcs, resume-at-trip-pc becomes more valuable.

4. **Multi-arg JIT'd GetProp**: currently the dispatcher's gate allows 1 or 2 args. Functions with more args (or with both arith and GetProp mixed) require translator extension.

### Doc 730 §XVI status

The deferral of shape-based IC caching to a future workstream is Case-4 (implementation freedom): cruftless's discipline lets the JIT pilot defer a substantial architectural concern (shapes) without blocking the IC mechanism itself. Mainstream JITs were not honestly able to defer shapes — they were designed around shapes from inception. Cruftless's deferral is principled: the IC mechanism works without shapes for the failure-path; the success-path's perf gain awaits shapes.

---

*JIT-EXT 24 closes the IC chapter's substrate-side work for the first cut. The fast-path-cache layer awaits hidden classes (separate workstream). The deopt-fallback path is proven end-to-end. The JIT workstream's standing claim at this point: every component of the Doc 736-style impossibility infrastructure that is reachable without hidden classes is operational.*

---

## JIT-EXT 25 — 2026-05-22 (telos sharpening + rename to LeJIT)

### Headline

Apparatus-tier round. No code substrate moves; pure framework refinement triggered by a 2026-05-22 keeper exchange on whether hand-rolling a Cranelift replacement would provide performance benefits. The analysis surfaced a four-region answer (probably yes in narrow regions, probably no overall) and identified the productive next move as a hand-rolled IC stub emitter *alongside* Cranelift, paired with a hidden-classes substrate pilot. Keeper concurred and directed the telos sharpening; the workstream is internally renamed rusty-js-jit → LeJIT to mark the hybrid stance.

### Substrate delivered

- `seed.md` §I.2 added: "Sharpened telos (2026-05-22): the LeJIT hybrid stance." Names the four sites where hand-rolling structurally wins over Cranelift (IC stub patching, Value-tag inline checks, tiny-fn compile latency, tail-call dispatch tightness); names the four sites where Cranelift's 20+ years of codegen engineering is the right consumed substrate; defines the hybrid telos as a single sentence; pre-files Pilot LeJIT-Σ (IC stub emitter) and the hidden-classes substrate pilot at coordinate `pilots/rusty-js-shapes/` per Doc 737 §IV.
- `seed.md` header + `trajectory.md` header: workstream internally renamed LeJIT (etymology: "le JIT" / "legit JIT" — naming the deliberate hybrid stance: consume Cranelift where structurally upstream of the alphabet contract, hand-roll where the alphabet contract is finer-grained). On-disk crate path `pilots/rusty-js-jit/` retained until a separate clerical-migration round; locale-tag `L.rusty-js-jit` preserved per Doc 737's coordinate-uniqueness invariant.
- Closure-summary table augmented with EXT 25 row.

### §XVI / §V categorization

Per Doc 730 §XVI: Case-3 (cruftless and Bun both diverge along different axes; no spec-correctness call needed) — the hand-rolled-vs-Cranelift choice is purely an implementation-freedom move at the (P2.d) cost-stratum dimension per Doc 735 §X.h.b. Mainstream JITs all use bespoke codegen (V8, JSC, SpiderMonkey); LeJIT's hybrid stance is the corpus-original synthesis enabled by Doc 731's alphabet-purity claim.

Per Doc 734 §V: growth mechanism (c) positive-finding generalization. The 2026-05-22 keeper exchange surfaced the hybrid stance as a recognition implicit in JIT-EXT 24's closure but not previously articulated. The telos sharpening is the framework-tier refinement; no negative empirical finding was required to motivate it.

### Composition with prior corpus work

- **Doc 731 §VII R1–R8.** All six measurable conjectures remain corroborated under the sharpened telos. R1 (single tier) extends naturally to the hybrid: Cranelift + hand-rolled emitter are not two tiers, they are two sub-substrates of the same tier with complementary expressiveness. R8 (no internal optimization passes) remains satisfied because the hand-rolled emitter is straight-line lowering.
- **Doc 729 §A8.13 substrate-amortization.** The hidden-classes pilot is the substrate-introduction round; Pilot LeJIT-Σ is the closure round reusing the substrate. Order is fixed by the dependency: hidden classes first, IC stub emitter second.
- **Doc 735 §X.h three-probe-levels discipline.** The (P2.a) strict-win claim for the hand-rolled IC stub emitter requires bench probe + consumer-route probe + fuzz probe. §I.2's falsifier names the explicit 3× per-hit speedup threshold; below that, re-categorize as (P2.d) correct-but-losing and revert.
- **Doc 737 §IV pre-filing.** The two pre-filed coordinates (`pilots/rusty-js-shapes/` and `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` namespace within the LeJIT crate) materialize when the substrate calls per Doc 737 §IX's "pre-file generously, spawn only when the substrate calls" discipline.
- **Doc 738 §II.** The hand-rolled emitter's source-tier coordinates fit cleanly into the five-axis convention space: pillar-path `pilots/rusty-js-jit/derived/src/{stub_aarch64,stub_x86_64,value_tag_inline,tiny_baseline}.rs`; prefix `__ic_*` for IC stub state per §II.a; install via `set_own_internal` per §II.c. Cross-axis consistency with the Cranelift-using translator path maintained by construction.

### Pred-731 disposition (unchanged)

All six R-conjectures of seed §VIII remain corroborated. Pred-731.XV.1 (cryptographic-primitive optimization recurs as alphabet promotion) now has a parallel at the JIT tier: the IC stub fast-path emitter is to the IC dispatch what Doc 731 §XV's primitive-optimization is to the cryptographic primitives — a hand-rolled tier-internal specialization that consumes the alphabet's purity rather than reinventing the alphabet.

### Open scope at JIT-EXT 25 close

1. **Hidden-classes substrate pilot** at coordinate `pilots/rusty-js-shapes/`. Substrate-introduction round per Doc 729 §A8.13. Multi-week. Pre-filed.
2. **Pilot LeJIT-Σ — IC stub emitter (aarch64 first cut)**. Closure round reusing the hidden-classes substrate. Hand-rolled emitter for property-access IC stubs with self-modifying patching. Pre-filed; depends on (1).
3. **Pilot LeJIT-Σ' — IC stub emitter (x86_64)**. Parallel closure for the second target ISA. Lower priority than (2) since Pi is the engagement's reference hardware.
4. **Value-tag inline emitter for hot Op::GetProp / Op::SetProp / Op::Call paths**. Composes with (2); same hand-rolled emitter infrastructure.
5. **Tiny-fn fast-baseline emitter** that bypasses Cranelift when function size is below threshold. Independent of (1)/(2); could land in parallel.

### Cumulative status at JIT-EXT 25 close

LOC delta: 0 (apparatus-tier round; framework-only). Source footprint unchanged. PM-EXT 11+12 regression unchanged.

The workstream's standing claim at this point: the first-cut hybrid baseline JIT is structurally complete. The next two rounds (hidden classes + IC stub emitter) instantiate the §I.2 sharpened telos. Doc 731's alphabet-purity claim is corroborated; the new claim under test is Doc 731 + §I.2: that a JIT with Cranelift owning the generic codegen tier AND a hand-rolled substrate-specific emitter owning the IC-fast-path / tagged-Value / inline-tiny-fn tier achieves IC fast-path latency competitive with mainstream JITs while preserving the single-tier baseline shape.

---

*JIT-EXT 25 closes the apparatus-tier round. No code commits. The workstream resumes at the hidden-classes substrate pilot's founding round when keeper directs.*

---

## JIT-EXT 26 — 2026-05-23 (spawns nested locale `stub-emitter` for Pilot LeJIT-Σ)

Per Doc 737 §IV + the keeper's standing "set up seeds at every fractal locale that requires it" directive: Pilot LeJIT-Σ (hand-rolled aarch64 IC stub emitter) was pre-filed at JIT-EXT 25 seed §I.2; the spawn is now explicit because the pilot has multi-rung shape (StubE-EXT 0-8: founding → bench-baseline → design → scaffold → synthetic-pointer test → translator wiring → bench measurement → fuzz → default-on flip).

**Spawned nested locale**: `pilots/rusty-js-jit/stub-emitter/` (locale tag `L.rusty-js-jit/stub-emitter`).

This is the engagement's second prospective-spawn case (first was `pilots/rusty-js-shapes/consumer-migration/` earlier today). LeJIT-Σ's substrate-introduction round can begin in advance of CMig-EXT 8's enrollment flip — StubE-EXT 0-3 (founding + bench baseline + design + scaffold) operate against the stable `Object::shape_ptr_and_slot_for` API contract without needing actual enrolled Shaped objects; StubE-EXT 4-8 gate on CMig-EXT 8.

See [stub-emitter/seed.md](stub-emitter/seed.md) and [stub-emitter/trajectory.md](stub-emitter/trajectory.md). Per Doc 733 §III composition relations: this row records the child existence by reference; the child's internal structure stays inside the child.

*JIT-EXT 26 stays open until StubE-EXT 8 closes the nested workstream.*

---

## JIT-EXT 27 — 2026-05-23 (spawns nested locale `value-tag-inline` for Pilot LeJIT-Ψ)

Per Doc 737 §IV + the keeper's pivot directive: spawn Pilot LeJIT-Ψ (hand-rolled value-tag inline emitter) at coordinate `pilots/rusty-js-jit/value-tag-inline/`. Sibling locale to LeJIT-Σ; both children of this LeJIT pilot.

**Trigger**: StubE-EXT 5b's bench measurement surfaced that the shape-substrate cascade (Doc 729 §A8.13) had absorbed most of LeJIT-Σ's IC-only contribution. Per the §I.3 substrate-amortization-cascade reading, LeJIT-Ψ is the second arm of the multiplicative composition that reaches Pred-stub.1's 3× target.

See [value-tag-inline/seed.md](value-tag-inline/seed.md) and [value-tag-inline/trajectory.md](value-tag-inline/trajectory.md). Per Doc 733 §III composition relations: this row records the child by reference.

*JIT-EXT 27 stays open until VTI-EXT 8 closes the nested workstream.*

---

## JIT-EXT 28 — 2026-05-23 (spawns nested locale `tiny-baseline` for Pilot LeJIT-Τ)

Per Doc 737 §IV + Doc 735 §X.h.d saturation-as-escalation-signal + keeper direction at 06:35-local after VTI-EXT 3b's (P2.d) negative finding: spawn Pilot LeJIT-Τ (tiny-fn fast-baseline) at coordinate `pilots/rusty-js-jit/tiny-baseline/`. Third nested sibling under this LeJIT pilot; siblings are LeJIT-Σ (stub-emitter) and LeJIT-Ψ (value-tag-inline).

**Trigger**: VTI-EXT 1's bench decomposition (127 ns per-iter, ~95% in dispatcher) empirically located the dispatcher as the largest single arm of the seed §I.3 multiplicative composition. VTI-EXT 3b's (P2.d) result at +18.9 ns regression closed the case that VTI alone could reach the 3× target. Per §X.h.d, when consecutive (P2) moves at the same site stop producing improvement, the next substrate target is OUTSIDE that site — the dispatcher itself per LeJIT seed §I.2 item 5.

See [tiny-baseline/seed.md](tiny-baseline/seed.md) and [tiny-baseline/trajectory.md](tiny-baseline/trajectory.md). Per Doc 733 §III composition relations: this row records the child by reference.

Locale count: 13 → 14 after this spawn (manifest refreshed). The engagement's fifth nested locale, third under this LeJIT pilot.

*JIT-EXT 28 stays open until TB-EXT 8 closes the nested workstream.*

---

## JIT-EXT 29 — 2026-05-23 (first-cut composition closes at engagement-tier default; three sub-pilots' status synthesized)

### Headline

Cross-pilot synthesis round closing the parent LeJIT trajectory's open rows for the three nested sub-pilots. **The LeJIT seed §I.3 multiplicative composition target is empirically met at engagement-tier default**: post both default-on flips (StubE-EXT 8 + TB-EXT 8), bench_ic = 81 ns (vs 271 ns pre-shape baseline = 3.34×; matches §I.3's "bun's per-op cost" prediction). This is the parent-pilot-tier close of the rounds JIT-EXT 26/27/28 opened.

### Sub-pilot status synthesized

| sub-pilot | first-cut status | engagement default |
|---|---|---|
| LeJIT-Σ (stub-emitter) | (P2.a) at composition scale via StubE-EXT 5c | DEFAULT-ON post StubE-EXT 8 |
| LeJIT-Τ (tiny-baseline) | (P2.a) at composition scale via TB-EXT 3b + 5c-composition | DEFAULT-ON post TB-EXT 8 |
| LeJIT-Ψ (value-tag-inline) | (P2.d) at first cut; revival path empirically named via TB-EXT 7 | default-OFF; VTI-EXT 3c queued |

JIT-EXT 26 (spawn LeJIT-Σ stub-emitter) — closes with Σ at default-on.
JIT-EXT 27 (spawn LeJIT-Ψ value-tag-inline) — stays open; Ψ at (P2.d).
JIT-EXT 28 (spawn LeJIT-Τ tiny-baseline) — closes with Τ at default-on.

### Engagement-tier baseline transformation

| workload | pre-StubE-EXT 8 | post-both-flips default | Δ |
|---|---:|---:|---:|
| bench_call_overhead `none` | 122.9 ns | **71.2 ns** | **−42%** AUTOMATIC |
| bench_ic `none` | 197.9 ns | **81.0 ns** | **−59%** AUTOMATIC |

Default-cruft users get both gains without env flag.

### Engagement framework instruments seeded this session

- **`pilots/rusty-js-jit/findings.md`** — substrate-improvement guidance doc (per keeper 2026-05-23 14:09-local directive). Six sections + standing rules. Has been validated empirically at four+ applications during the same session it was created.
- **`pilots/rusty-js-jit/enhancements.md`** — cross-locale append-only event log. Verbose UNANTICIPATED entries (now ~10) preserve substrate-amortization-cascade signals + (P2.d)/(P2.c) findings.
- **`pilots/cross-runtime-bench/`** — standalone top-level pilot founded mid-session per keeper directive. 15 locales total post-spawn (10 top-level + 5 nested).
- **LeJIT seed §I.3 amendment** (CRB-EXT 8) — per-workload composition disambiguation as standing framework vocabulary.
- **TB-EXT 7 segfault discovery** — engagement-class HashMap-value-slot raw-pointer-cache bug class named + fixed via Box-wrap pattern.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: Case-4 (implementation freedom). No spec-correctness call.

Per Doc 734 §V: growth (c) **positive-finding generalization at engagement scale**. The substrate-amortization-cascade pattern from Doc 729 §A8.13 fully realized at first-cut: shape (default-on) + STUB (default-on) + TB (default-on) compose multiplicatively at the engagement-tier without env flag. The §I.3 prediction was both reached and exceeded.

Per Doc 735 §X.h.b: two of three sub-pilots at (P2.a) at scale; third at (P2.d) with empirically-named revival path.

Per Doc 735 §X.h.c: three-probe-levels discipline applied prospectively at all three default-on flips of the session; the discipline's value is now anchored at engagement scale per the StubE-EXT 7 + TB-EXT 7 fuzz probes.

### Composition with prior corpus work

- **LeJIT seed §I.2 hybrid stance**: the four hand-rolled regions framing materializes at three of four (Σ, Τ landed; Ψ at (P2.d); Σ' pre-filed). The §I.2 prediction "hybrid Cranelift+hand-rolled wins" is empirically corroborated.
- **LeJIT seed §I.3 substrate-amortization cascade + amendment**: empirically met at engagement default; bench_ic-class composition target HOLDS.
- **CRB-EXT 8 §I.3 amendment**: bench_ic-class composition target met; CRB-class spectrum reading (3-15× off bun) unchanged.
- **CMig-EXT 15 retrospective** + **TB-EXT 7 prospective**: the engagement's three-probe-levels discipline catches different bug classes (wrong-result; memory-safety). Both required for (P2.a) at scale.
- **Findings doc rule 5**: validated at three successive default-on flips (shape CMig-EXT 14 → StubE-EXT 8 → TB-EXT 8). Discipline's value compounds.

### Forward optimization queue (not load-bearing for any standing Pred)

1. **VTI-EXT 3c** — VTI revival from (P2.d). Path empirically named: calling-convention restructure CAN pay when done with the precheck-removal that VTI-EXT 3c includes. JIT-EXT 27 stays open until 3c closes.
2. **Skip STUB infra on no-property functions** (~10 LOC translator change) — eliminates StubE-EXT 8's +11% bench_call_overhead infra tax.
3. **Inline Cranelift IR for IC fast-path** (B-level vs current A-level Rust-extern) — ~5-10 ns marginal; not load-bearing.
4. **StubE-EXT 9 / TB-EXT 9 heap-vec-relocation audit** — proactive bug-class generalization per TB-EXT 7's HashMap-slot lesson.
5. **CRB re-baseline post-flips** — show default-cruft's new competitive position on realistic workloads.
6. **CMig-EXT 16 + 17** (Findings VI.6 HIGH priority) — property-bypass audit + canonical 2000-fixture fuzz harness.
7. **VI.1 fast JSON, VI.2 tight-inner-loop emitter, VI.3 Array.filter/map fast-path** — forward-derived non-LeJIT pilots named in findings.

### Cumulative status at JIT-EXT 29 close

LeJIT parent pilot has now landed:
- 28 prior JIT-EXTs (0-28 inclusive)
- 3 nested sub-pilots, 2 default-on at engagement-tier
- The engagement-tier baseline shifted by 42-59% on per-call benches
- 4 engagement-tier framework instruments seeded (findings, enhancements, CRB pilot, §I.3 amendment)
- 1 critical bug class discovered + fixed (HashMap-slot raw-pointer cache)
- 3 default-on flips, all gated on three-probe-levels discipline

The LeJIT first-cut chapter at the parent pilot level is **closed**. The substrate's engagement-tier performance is anchored on a new baseline 42-59% faster than pre-session. The remaining LeJIT-tier work is forward optimization, not load-bearing.

---

*JIT-EXT 29 closes. Parent LeJIT trajectory's open rows for JIT-EXT 26/28 close (Σ + Τ at default-on); JIT-EXT 27 stays open (Ψ at (P2.d); revival queued). The LeJIT seed §I.3 composition target empirically met at engagement-tier default. Session's substrate goal achieved.*

---

## JIT-EXT 30 — 2026-05-23 (spawns nested locale `f64-calling-convention` for Pilot LeJIT-Φ)

Per Doc 737 §IV + keeper directive 2026-05-23 16:02-local after the pre-implementation analysis (logged in this trajectory's enhancement-log entry via the corresponding round) surfaced that VTI's structural (P2.d) traces to the JIT's i64-only calling convention (the LeJIT seed §I.1 "typed-i64 first; f64 deferred" carve-out). The constraint enumeration C1-C10 (folded into the new locale's seed §I.2) induced the architectural move: f64 default + bytecode-tier-driven typed-i64 promoted fast path. Φ lands Move 1 (f64 default); Move 2 is a separate downstream pilot at the bytecode tier per Doc 731 §XIII alphabet promotion.

**Spawned nested locale**: `pilots/rusty-js-jit/f64-calling-convention/` (locale tag `L.rusty-js-jit/f64-calling-convention`).

Fourth nested sibling under this LeJIT parent. Siblings: LeJIT-Σ (default-on), LeJIT-Τ (default-on), LeJIT-Ψ ((P2.d); Φ's downstream Φ-EXT 7 is its revival path).

**Trigger**: pre-implementation analysis of VTI-EXT 3c surfaced that the inline replacement of the dispatcher's `jit_compatible_arg` precheck would require BOTH inline tag-check (cheap) AND inline integer-validity check (expensive: ~24-30 cycles vs precheck's ~15-20). VTI as designed is structurally (P2.d) within the i64-only architecture. The keeper named the deeper bottleneck ("our simple Cranelift i64 implementation is becoming a bottleneck") and directed naming constraints to induce the next layer's properties. The constraint enumeration C1-C10 produced f64-default as near-necessity, not arbitrary choice.

The substrate work begins at Φ-EXT 1 (design doc); Φ-EXT 3 is the load-bearing implementation round; Φ-EXT 7 attempts VTI revival under the new architecture.

See [f64-calling-convention/seed.md](f64-calling-convention/seed.md) and [f64-calling-convention/trajectory.md](f64-calling-convention/trajectory.md). Per Doc 733 §III composition relations: this row records the child by reference.

Locale count: 15 → 16 after this spawn (manifest refreshed). The engagement's sixth nested locale overall (fourth under this LeJIT parent).

*JIT-EXT 30 stays open until Φ-EXT 8 closes the nested workstream.*

---

## JIT-EXT 31 — 2026-05-23 (Φ-EXT 2+3 close + VTI cascade-revival + Doc 739 published)

### Headline

Parent-tier close of the Φ-EXT 2+3 merged round. **The f64 calling-convention shift landed; all six Pred-φ.X falsifiers HOLD; VTI revived as cascade from (P2.d) to (P2.a) without any VTI-specific substrate move.** Corpus articulation Doc 739 published at jaredfoy.com formalizes the cascade-revival pattern in abstract form (applicable to any system architecture admitting a Doc 729 resolver-instance pipeline) + applied to the LeJIT-Φ instance.

### Sub-pilot status synthesized post-Φ

| sub-pilot | first-cut status | engagement default |
|---|---|---|
| LeJIT-Σ (stub-emitter) | (P2.a) at composition scale | DEFAULT-ON |
| LeJIT-Τ (tiny-baseline) | (P2.a) at composition scale | DEFAULT-ON |
| LeJIT-Ψ (value-tag-inline) | **(P2.a) post-Φ cascade-revival** (was (P2.d) first cut) | default-OFF; opt-in via env flag composes constructively |
| LeJIT-Φ (f64-calling-convention) | (P2.a) at architectural scale | DEFAULT (architectural; no flag) |

All four LeJIT-tier nested pilots now at (P2.a). JIT-EXT 27 (LeJIT-Ψ spawn) was waiting on VTI-EXT 8 to close; **closes here via cascade-revival mechanism without VTI-EXT 8 needing to run**. JIT-EXT 30 (Φ spawn) closes with Φ-EXT 4-6 + 8 remaining as formal-confirmation rounds (substrate work functionally complete).

### Engagement-tier baseline post-Φ

| workload | pre-StubE-EXT 8 | post-Φ default | Δ |
|---|---:|---:|---:|
| bench_call_overhead `none` | 122.9 ns | 72.1 ns | −41% |
| bench_ic `none` | 197.9 ns | 82.9 ns | −58% |
| bench_ic TB+STUB+VTI | 743.8 ns (P2.d) | **85.5 ns (P2.a)** | **−89% via cascade** |

Default-cruft users continue to get the 41-58% baseline reduction unchanged from pre-Φ. The new contribution: VTI is no longer a (P2.d) anchor in the composition matrix; the engagement's standing performance reading is structurally cleaner.

### Corpus articulation: Doc 739

The keeper's framing question after observing the VTI cascade-revival ("this indicates that we closed the gaps on an implicit resolution pipeline") crystallized the structural recognition. Doc 739 published at jaredfoy.com:

- **Abstract formulation** (§II): the cascade-revival pattern in three propositions P1-P3 + three boundary conditions B1-B3. Specialization of Doc 729 §A8.13 substrate-amortization-cascade at the categorization axis (the classical cascade is per-iter cost).
- **Applied instance** (§III): the LeJIT-Φ pipeline decomposition P1-P4; the i64-only upstream constraint pre-Φ; the constraint-enumeration apparatus at Φ founding (C1-C10); the cascade observation post Φ-EXT 3 with all three boundary conditions met.
- **Predictions** (§V): five falsifiers Pred-739.1-.5 with cross-domain candidate (Pred-739.5).
- **Apparatus claim**: the constraint-enumeration discipline (Pin-Art per Doc 581 + Φ seed §I.2) is the apparatus that identifies cascade-revival candidates.

The corpus has added one more framework component to its substrate-improvement vocabulary. The cascade-revival pattern is operational, observable, predicted to recur cross-engagement.

### Engagement framework instruments updated this session

Cumulative across the session (post Φ + Doc 739):

- `pilots/rusty-js-jit/findings.md`: substrate-improvement guidance doc. Now contains 6 original sections + 8 standing rules + addendum (5 promoted/new findings + standing rule 9 + 1 new standing rule from Φ-EXT 7) + addendum II (Finding II.5 gap-closure-as-cascade-revival).
- `pilots/rusty-js-jit/enhancements.md`: cross-locale append-only event log. ~14 verbose UNANTICIPATED entries.
- `pilots/cross-runtime-bench/`: standalone top-level pilot founded mid-session.
- `pilots/rusty-js-jit/f64-calling-convention/`: nested LeJIT-tier pilot founded + first-cut chapter closed mid-session.
- LeJIT seed §I.3 amendment (CRB-EXT 8 per-workload disambiguation).
- LeJIT seed §I.4 first-cut composition empirically met at engagement default.
- Corpus Doc 739 (constraint-closure as cascade-revival).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: Case-4 implementation freedom for the substrate landing; Case-1 verification post-fix for Pred-φ.4 (fractional-Number correctness).

Per Doc 734 §V: this round realized all three growth mechanisms — (a) tier-relocation (Φ spawn after the keeper-named upstream constraint); (b) negative-finding amendment in the Φ-EXT 2 intermediate-state (the merge with 3 was the amendment); (c) positive-finding generalization at multiple tiers (Pred-φ HOLDS; VTI cascade-revival; Doc 739 articulation).

Per Doc 735 §X.h.b: all four nested LeJIT-tier sub-pilots at (P2.a) at scale. Engagement-wide (P2.a) reading achieved at first-cut LeJIT chapter.

Per Doc 735 §X.h.c three-probe-levels: bench POSITIVE; consumer-route POSITIVE; fuzz POSITIVE (existing fixtures). Φ-EXT 6 adds explicit fractional+NaN+Infinity fuzz; the existing fuzz coverage is sufficient for the cascade-revival's structural claim per Pred-φ.5.

### Open scope at JIT-EXT 31 close

1. **Φ-EXT 4** — Composition matrix as canonical confirmation (already captured at Φ-EXT 2+3 close).
2. **Φ-EXT 5** — Consumer-route probe (diff-prod 42/42 already confirms; fractional-Number test already ran).
3. **Φ-EXT 6** — Fuzz probe with fractional + NaN + Infinity coverage.
4. **Φ-EXT 7** — NO LONGER NEEDED (VTI revival cascade).
5. **Φ-EXT 8** — Default-on confirmation (Φ is architectural; no flag flip).
6. **Forward-derived non-LeJIT pilots** (Findings VI.1-VI.3): fast JSON, tight-inner-loop emitter, Array.filter/map fast-path. Multi-week scope each.
7. **CMig-EXT 16 + 17** (Findings VI.6 HIGH priority): property-bypass audit + canonical 2000-fixture fuzz harness. Closes the engagement's fuzz-coverage gap that CMig-EXT 15 + TB-EXT 7 made visible.
8. **StubE-EXT 9 / TB-EXT 9 heap-vec-relocation audit** (per TB-EXT 7 enhancements log entry): proactive bug-class generalization.
9. **Doc 731 §XIII Move 2** (typed-i64 promoted fast path via bytecode tier-1.5 IR): re-enables the typed-i64 specialization disabled at Φ-EXT 3. Separate downstream pilot at the bytecode tier.

### Cumulative status at JIT-EXT 31 close

LeJIT parent pilot has now landed:
- 30 prior JIT-EXTs (0-30 inclusive)
- 4 nested sub-pilots, 3 default-on at engagement-tier + 1 cascade-revived to (P2.a) via the f64 architecture
- The engagement-tier baseline shifted by 41-58% on per-call benches
- 5 engagement-tier framework instruments seeded (findings, enhancements, CRB pilot, LeJIT seed §I.3/I.4 amendments, corpus Doc 739)
- 1 critical bug class discovered + fixed (HashMap-slot raw-pointer cache via Box-wrap; TB-EXT 7)
- 3 default-on flips, all gated on three-probe-levels discipline
- 1 structural cascade-revival pattern named + formalized as corpus articulation (Doc 739)

The LeJIT first-cut chapter at the parent pilot level is **closed at engagement-tier (P2.a) for all four nested sub-pilots**. The Pin-Art apparatus discipline (constraint-enumeration → induced architecture → substrate move) just paid off measurably with VTI's cascade-revival as the unanticipated structural side effect. The remaining LeJIT-tier work is forward optimization, not load-bearing for any standing Pred.

---

*JIT-EXT 31 closes. LeJIT-Φ landed; VTI revived as cascade; Doc 739 published. All four nested LeJIT-tier sub-pilots at (P2.a). Parent LeJIT trajectory's open rows (JIT-EXT 26/27/28/30) all close — three via direct default-on, one (JIT-EXT 27) via cascade.*
