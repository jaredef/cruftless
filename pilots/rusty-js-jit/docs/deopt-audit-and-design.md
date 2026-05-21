# JIT deopt — Audit + Design (JIT-EXT 10)

**Date**: 2026-05-21
**Scope**: enumerate every actual and potential speculation point in the current rusty-js-jit substrate; articulate the deopt mechanism per Doc 731 §VII R5; map the work to land full deopt and replace the `jit_disabled` workaround.

## I. The audit

### I.1 What the current first-cut JIT emits

The translator at `pilots/rusty-js-jit/derived/src/translator.rs` (679 LOC) emits unconditional i64 arithmetic + control flow. Op coverage:

- **Loads / stores**: LoadArg, LoadLocal, StoreLocal, PushI32, Dup, Pop
- **Arithmetic**: Add, Sub, Mul, Inc, Dec (untyped); AddI64, SubI64, MulI64, IncI64, DecI64 (typed-i64 alphabet)
- **Comparisons**: Lt, Le, Gt, Ge, Eq, Ne, StrictEq, StrictNe; LtI64, LeI64, GtI64, GeI64, EqI64, NeI64
- **Control flow**: Jump, JumpIfTrue, JumpIfFalse, Return, ReturnUndef

Every op lowers to a single Cranelift instruction. Locals are Cranelift `Variable`s (mem2reg promotes them). Operands stay i64 throughout. The translator returns `Err` on any unsupported op (verifier-before-emission, R3).

### I.2 The boundary guard

At `interp.rs:7562`, the dispatcher pre-screens every call with four conditions:

1. `!jit_disabled`
2. `count >= jit_threshold`
3. `params == 1 || params == 2`
4. `args.iter().all(jit_compatible_int_arg)`

Only if all four hold does the call enter JIT code. `jit_compatible_int_arg` accepts `Value::Number(f)` where `f` is a safe integer (no NaN, fits i32 / IsFinite, integer-valued).

### I.3 In-flight speculation surface (exhaustive)

Walking every op the translator emits, the only in-flight assumption the JIT makes that could diverge from JS Number semantics is **integer overflow on arithmetic**:

| Op | Cranelift instruction | Failure mode |
|---|---|---|
| Add / AddI64 | `iadd` | wraps i64 on overflow; JS would promote to f64 |
| Sub / SubI64 | `isub` | wraps i64 on overflow; JS would promote to f64 |
| Mul / MulI64 | `imul` | wraps i64 on overflow; JS would promote to f64 |
| Inc / IncI64 | `iadd` w/ 1 | wraps on overflow |
| Dec / DecI64 | `isub` w/ 1 | wraps on overflow |
| Lt / Le / Gt / Ge | `icmp` signed | none — the boundary guard ensures args are safe integers in [-2^53, 2^53] range |
| Eq / Ne / StrictEq / StrictNe | `icmp` equal | none — same reason |
| Jump / JumpIfTrue / JumpIfFalse | `jump` / `brif` | none — branches on already-computed truthy |
| Return / ReturnUndef | `return_` | none — caller widens the i64 back to f64 |
| LoadArg / LoadLocal / StoreLocal | `use_var` / `def_var` | none — Cranelift Variables carry typed i64 |
| PushI32 | `iconst` | none — constant |
| Dup / Pop | stack manipulation | none |

**Conclusion**: integer overflow is the *only* in-flight speculation. Everything else is structurally precluded by the boundary guard. The JIT body holds exactly one assumption beyond what the guard validates.

### I.4 Is overflow a deopt site?

Per Doc 731 §VII R5: *"Deopt sites are a finite enumerable set, declared as P4 dispatch points."* P4 ops are speculation-flavored (GetProp / SetProp / CallMethod). Arithmetic is P3 (typed).

So by the Doc 731 taxonomy, **overflow is NOT a P4 deopt site**. The current JIT has zero P4 sites and zero deopt sites per the spec.

What the JIT *does* do on overflow: silently wraps i64. JS spec says: promote to f64 and lose integer precision (e.g., `Number.MAX_SAFE_INTEGER + 1 === Number.MAX_SAFE_INTEGER`). The JIT diverges from JS spec exactly here.

The divergence is currently undetected because all benches and parity tests stay well below `2^53`. A bench that overflows i64 would surface the divergence; the engagement has not constructed one.

### I.5 The `jit_disabled` workaround

The flag at `interp.rs:7615` is a coarse pre-call substitute for deopt. When a Closure has a successful JIT compile cached but receives args failing the boundary guard, the flag flips to `true` and the Closure permanently routes to the interpreter.

This is *not* deopt because:
- No stack-map reconstruction (boundary failure has no in-flight state)
- No resume-pc (the call hasn't entered JIT code yet)
- Permanent forfeit (subsequent calls with valid args also go to interpreter)

It is the right tool for the current scope: the only failure mode is at the boundary, and the boundary is checked before entering JIT code. Inside the JIT body, there is nothing to recover from (overflow is silently wrapped, not trapped).

## II. What full deopt would actually be

Per Doc 731 §VII R5 + seed §III.6: deopt = (a) detect at a P4 site, (b) reconstruct the interpreter frame from the JIT's stack map, (c) resume interpretation at the recorded continuation bytecode pc.

The mechanism has three components:

### II.1 DeoptReason enum

A typed, finite set of trip causes. The first cut needs entries for every reason a JIT site might trip; the enum grows as new P4 sites land:

```rust
pub enum DeoptReason {
    // Future P4 sites — empty for now since the current JIT has no P4 ops.
    ICShapeMismatch { ic_id: u32 },
    ICCallTargetChanged { ic_id: u32 },
    IntegerOverflow { op_pc: u32 },             // optional: if we choose to gate overflow
    TypeWidening { local_slot: u32 },           // future, when broader Value coverage lands
    BoundaryArgMismatch,                        // for replacing jit_disabled
}
```

The enum is closed; adding a variant is a substrate decision (lift requires an EXT round). This mirrors Doc 736's CapabilityError taxonomy.

### II.2 Stack-map format

For each potential deopt point, the JIT must record:

- Which **interpreter local slots** are live, and which Cranelift `Variable` (or SSA value) currently holds each.
- The **operand-stack depth** at the deopt point (and the SSA values for each stack slot).
- The **continuation bytecode pc** at which the interpreter should resume.

In practice this is a `Vec<DeoptSite>` per CompiledFn, indexed by deopt-site-id (which the JIT emits inline as a small immediate at the deopt callsite):

```rust
pub struct DeoptSite {
    pub reason: DeoptReason,
    pub resume_pc: u32,                  // bytecode offset to resume at
    pub live_locals: Vec<DeoptLiveLocal>, // (interpreter_slot, JIT-side-register-or-stack-slot)
    pub stack_depth_at_trip: u8,
    pub stack_slots: Vec<DeoptLiveLocal>, // operand stack slots
}

pub struct DeoptLiveLocal {
    pub interp_slot: u16,
    pub jit_location: JitLocation,       // Register(idx) | StackSlot(offset) | Constant(i64)
}
```

Cranelift's `stackmap` support exists but is geared toward GC roots. For deopt we want a different shape (we don't need to scan, we need to extract values at a specific pc), so the simplest approach is to hand-roll the stack-map table per CompiledFn.

### II.3 Deopt thunk

A Rust function the JIT calls when it decides to trip:

```rust
extern "C" fn jit_deopt_thunk(
    reason_idx: i64,
    site_id: i64,
    // saved register values (from a known register-save convention):
    arg0: i64, arg1: i64, arg2: i64, arg3: i64,
) -> i64 {
    // 1. Look up DeoptSite for site_id in the active CompiledFn's table.
    // 2. Read live local values from the saved registers per the stack map.
    // 3. Convert each i64 back to Value::Number(f64).
    // 4. Walk back to the calling interpreter frame; populate interp.locals + interp.stack.
    // 5. Resume interpretation at site.resume_pc.
    // 6. Return the eventual result to the caller (the JIT'd entry-point's caller).
}
```

The thunk has to climb the C stack back to the Rust frame that called the JIT'd function. This is the load-bearing platform work: catching the JIT mid-execution and unwinding to a controlled landing pad. Two patterns work:

1. **Trampoline / longjmp-like**: the JIT'd entry is wrapped in a trampoline that sets up a `setjmp`-style landing pad before calling the JIT. Deopt = `longjmp` to the landing pad, which then runs the interpreter from the recovered state.
2. **Return-value sentinel**: the JIT returns a tagged i64 indicating "I tripped, run the interpreter from saved state X." The caller checks the tag and dispatches. Cleaner but adds one branch per JIT return.

For first cut the return-value sentinel is simpler — Cranelift integration is straightforward, no platform-specific unwind work. Performance cost is one branch per JIT-call return; negligible.

### II.4 Replacing `jit_disabled`

Once deopt infrastructure exists, the boundary-mismatch case (`!args.iter().all(jit_compatible_int_arg)` after JIT compile) becomes:

- Pre-call: detect mismatch, run interpreter for THIS call (same as today, no JIT entry).
- Don't set `jit_disabled = true`.
- Subsequent calls with valid args go through JIT as normal.

This isn't really "deopt" — it's still a pre-call check. But the behavior improves: instead of permanently disabling JIT after one mismatched call, the JIT remains available for subsequent valid calls. The mechanism is mode-of-operation rather than infrastructure.

### II.5 What deopt unlocks

Once the infrastructure is in place, the next substrate moves become tractable:

1. **IC sites for Class C ops (R6)** — GetProp / SetProp / CallMethod each get an IC with a shape check + deopt-on-mismatch.
2. **Broader Value coverage** — JIT'd doubles / strings / objects can have type-tag deopt sites.
3. **Op::Call in translator** — inter-procedural JIT requires inter-frame deopt (the callee can deopt and the caller must unwind correctly).
4. **Optional overflow checks** — convert silent i64 wrap into deopt-on-overflow if spec conformance becomes load-bearing.

Without deopt, none of these are reachable. With deopt, each is its own substrate move on top of a stable infrastructure.

## III. The EXT plan

Five rounds, Pin-Art shape consistent with PM and caps:

### JIT-EXT 10 (this entry): audit + design doc

**Substrate**: this document. No code. Audit identifies overflow as the only current in-flight speculation; documents the structural reasons full deopt is forward investment for IC support rather than a current correctness fix.

### JIT-EXT 11: DeoptReason + DeoptSite + thunk skeleton

**Substrate**: new module `pilots/rusty-js-jit/derived/src/deopt.rs`:
- `DeoptReason` enum (initial variants per §II.1)
- `DeoptSite`, `DeoptLiveLocal`, `JitLocation` types
- `jit_deopt_thunk` Rust function (registered as an external in JITModule)
- Per-CompiledFn `Vec<DeoptSite>` field

No translator changes. No JIT-side emission of deopt sites. Just the type machinery + thunk skeleton + a unit test exercising the lookup-and-reconstruct path with a hand-built DeoptSite.

Bench: unchanged (no codegen change).
Probe: caps_probes + PM-EXT 11+12 + sum(1M)=2ms unchanged.

### JIT-EXT 12: first wired demonstrator

**Substrate**: introduce one synthetic deopt site. Easiest candidate: a feature-flagged "guarded overflow" mode where each arithmetic op emits `iadd_overflow` + brif-on-overflow → deopt thunk. Default off (no perf impact). Test exercises both paths.

This is the proof-of-concept for the deopt mechanism. Tests verify the interpreter resumes at the correct pc with the correct locals after a synthetic trip.

Bench: default-off keeps sum(1M)=2ms; with feature flag on, sum(1M) measures the cost of overflow checks (expected ~10-30% slower).

### JIT-EXT 13: replace `jit_disabled` with retry-on-fresh-args

**Substrate**: at `interp.rs:7611-7616`, remove the permanent-disable side effect. The current call still runs through the interpreter (no JIT for this call), but subsequent calls re-evaluate the four-condition gate. If a future call passes all four, JIT re-engages.

This is not strictly deopt but it's the cleanup the deopt workstream enables: with the deopt infrastructure landed, we have a cleaner pattern for "the JIT is unavailable for this call but might be available for the next one."

Bench: long-tail callers see modest improvement vs current permanent-disable. Hot-path callers unchanged.

### JIT-EXT 14+: ICs unlocked

**Substrate**: the first IC site (GetProp on a typed Closure with stable hidden class) gets a translator emit + IC table + deopt-on-shape-mismatch wiring. This is Doc 731 §VII R6's substrate. Bench measures impact on a property-heavy workload.

## IV. Conjecture revision (Doc 731 R5)

Doc 731 §VII R5 said "the deopt site count is small and enumerable." After this audit, R5 should be refined: **"deopt sites are enumerable per emitted JIT module."** Each CompiledFn carries its own deopt-site table. The cardinality is bounded by the number of P4 ops + speculative arithmetic guards in the function's body. Across the engagement, the union of all deopt-site tables is the JIT's structural speculation surface.

The R5 prediction holds as written; the audit clarifies the unit of enumeration.

## V. What this doc does NOT propose

- **No stack-map walking via Cranelift's built-in stackmap** — that surface is GC-oriented and the wrong shape for deopt. We hand-roll.
- **No `longjmp`-style unwind** — return-value sentinel is simpler for the first cut.
- **No multi-tier JIT** — Doc 731 §VII R1 (one tier) holds. Deopt is the recovery mechanism, not a "lower" tier.
- **No inlined caches in the first wired round (JIT-EXT 12)** — those are JIT-EXT 14+. The first wired round just demonstrates the deopt path on a synthetic site.

---

*JIT-EXT 10 closes the audit and design round. The remainder of the workstream is implementation; each round adds one measurable substrate move on top of the previous.*
