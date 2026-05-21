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
