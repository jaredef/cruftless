# rusty-js-jit — Trajectory

Chronological resume anchors for the JIT workstream. Reads seed.md first; this file is the time-ordered record of substrate moves and their yields.

Format: one section per "EXT" (extension round); each round closes with a status block, a cumulative numbers table, and an open-scope list. Same shape as `pilots/rusty-js-ir/trajectory.md` and the top-level `trajectory.md`.

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
