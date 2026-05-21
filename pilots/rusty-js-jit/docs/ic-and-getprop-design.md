# GetProp, ICs, and dispatcher consume-recovered-state — Audit + Design (JIT-EXT 18)

**Date**: 2026-05-21
**Scope**: enumerate what landing real GetProp ICs in the JIT requires; settle the scoping decisions before any code lands; map the work to subsequent EXT rounds.

## I. The audit

### I.1 What the current JIT can NOT do

Walking the existing translator at `pilots/rusty-js-jit/derived/src/translator.rs`:

- **Value variants supported**: i64 only. Args are unboxed at the dispatcher boundary (`jit_compatible_int_arg` + `unbox_int_arg`). Inside the JIT body every SSA value is `I64` typed in Cranelift IR. Return values are widened from i64 back to `Value::Number(f64)` by the dispatcher.
- **Op surface**: arithmetic + control flow + load/store of locals/args + PushI32 + Dup/Pop. **No GetProp.** No CallMethod. No object-creation. No string operations. `compile_function` returns `Err` for any function whose bytecode contains an unsupported op.
- **No mechanism to represent a `Value::Object`**: the JIT cannot hold an ObjectRef without changing the i64-only assumption. `Value::Object(ObjectRef)` is a thin index into the runtime's Heap; passing an ObjectRef into JIT'd code requires either tagged-i64 encoding (low-bit-set = pointer, clear = number) or a richer JIT-side Value representation.

### I.2 What GetProp specifically requires

A bytecode `Op::GetProp(prop_name_idx)` does roughly:
1. Pop receiver (Value) from operand stack
2. Look up property using the receiver's hidden class + the property name
3. Push the result (Value) onto operand stack

For the JIT to handle this:
1. **Represent the receiver as something the JIT can pass to a runtime helper.** The receiver is a Value::Object; in the runtime, `Value::Object(id)` is an `ObjectRef` (a small integer index). The JIT could pass the index as an i64.
2. **Call into the runtime to do the lookup.** A `jit_getprop(receiver_idx: i64, prop_name_idx: i64) -> i64` extern. The runtime helper does the hidden-class walk + returns the result.
3. **Encode the return value as i64.** The result is a Value (could be any variant). Same encoding problem as the receiver.

Step 3 is the constraint. **The JIT cannot use a uniform i64 representation that round-trips through every Value variant**. Options:

**(A) Tagged-i64 union representation**: bit-level encoding of Value variants. Low bits identify the kind (Number, Object, String, etc.); the rest are the payload. Cost: every arithmetic op must check the tag, complicating the hot loops. Existing `iadd` would no longer suffice; we'd need `iadd-after-untag-both-operands`. **This breaks the current arithmetic JIT's perf profile.**

**(B) Per-Value-kind JIT specialization**: the JIT compiles a function for ONE Value-kind regime. An "i64 arithmetic" function handles only Number args. An "object property access" function handles only Object args. Mixed functions are rejected (or compiled twice, once per regime). Each regime stays internally uniform. **This preserves the arithmetic JIT's perf** at the cost of refusing to compile functions that mix arithmetic and property access in the same hot path.

**(C) Boxing at JIT entry**: the dispatcher passes a `*const Value` pointer for every arg. The JIT immediately unboxes into either an i64 (for Number args) or an ObjectRef (for Object args). Mixed-kind args are handled by branching at the unbox point. Cost: load + tag check per arg per call. **Acceptable for moderate-size functions; the per-arg overhead is bounded.**

### I.3 The scoping decision

For first cut: **Option B with a path to C.**

The arithmetic JIT's perf is a load-bearing standing claim (sum(1M) at 2ms, faster than Bun). Disturbing it has real cost. The current narrow scope is by design (Doc 731 alphabet purity); GetProp brings a different regime that should live alongside the arithmetic regime, not merge with it.

The implementation:
- Add a typed-object bytecode alphabet: `Op::GetPropOnObject(prop_name_idx)`. The upstream emitter generates this when it has proven the receiver is an Object. (Doc 731 §XIV.d β-path style; identical to how `AddI64` was added alongside `Add`.)
- The JIT supports `GetPropOnObject` by calling a runtime helper. The helper does the hidden-class lookup; the return value is widened back into a Value at the dispatcher boundary (the same way arithmetic results widen i64 → f64).
- Functions whose bytecode contains BOTH arithmetic ops AND `GetPropOnObject` are accepted; the JIT compiles them as a single function whose SSA values are i64-typed throughout. The receiver/result of GetPropOnObject are typed as i64 (ObjectRef index) in JIT SSA but interpreted by the helper as object references.

This is acceptable because both arithmetic and object indices fit in i64 cleanly. The JIT body's uniform i64 type stays intact; the *interpretation* of the i64 differs per op.

A future round can move to Option C if a workload demands mixed-kind args.

## II. The IC layer

### II.1 What an IC is, structurally

An inline cache at a GetProp site records the last N (shape, slot_offset) pairs observed at that call site. On each invocation:

1. Read the receiver's hidden class id
2. Compare against the cached shape(s)
3. **Cache hit**: read the slot at the cached offset directly (fast path, ~3 instructions)
4. **Cache miss**: call the runtime helper, which does the full hidden-class walk, and:
   a. If the cache has fewer than N entries: add the new (shape, offset) pair to the cache
   b. If the cache is full: **deopt** — switch back to the interpreter for this call

The deopt path is what JIT-EXT 17's demonstrator anticipated. With `ICShapeMismatch` as the reason variant, the dispatcher consumes the recovered state and resumes the interpreter at the failing GetProp pc.

### II.2 Cache representation

Per CompiledFn, an array of IC entries indexed by IC id:

```rust
pub struct ICEntry {
    pub bytecode_pc: u32,            // the GetProp op's pc
    pub prop_name_idx: u16,          // index into proto.constants
    pub cached_shapes: [u32; 4],     // up to 4 hidden-class ids
    pub cached_offsets: [u16; 4],    // matching slot offsets
    pub valid_count: u8,             // 0..=4 entries currently filled
}

pub struct CompiledFn {
    // ... existing fields ...
    pub ic_entries: Vec<ICEntry>,
}
```

The JIT-emitted code reads from `ic_entries[i]` via fixed memory loads (the address of the entry is known at JIT-compile time). The runtime helper updates the cache on miss.

### II.3 Cache lookup lowering

For each GetProp site, the JIT emits (in pseudocode):

```
let receiver_shape = load(receiver_obj.shape_offset)
let cached_shape_0 = load(ic_entry.cached_shapes[0])
if receiver_shape == cached_shape_0:
    let offset = load(ic_entry.cached_offsets[0])
    let value = load(receiver_obj.props + offset)
    push(value)
    continue
# (repeat for cached_shapes[1..3])
# All cache slots missed; call runtime
let value = runtime_getprop_miss(ic_id, receiver, prop_name)
# helper either returns the value (cache updated) or sets LAST_DEOPT_FRAME
# and the dispatcher unwinds
push(value)
```

This is the canonical IC shape. The exact codegen depends on Cranelift IR primitives; cruftless's first-cut version can collapse the 4-shape check into a single linear scan instead of unrolling.

## III. Dispatcher consume-recovered-state

JIT-EXT 14 deferred this: when a JIT trip records `state.resume_pc != 0`, the dispatcher needs to populate the interpreter frame from the recovered state and resume at the recorded pc — not re-execute from pc=0.

### III.1 What "resume at arbitrary pc" requires

The interpreter's `call_function` path:
1. Allocates a Frame with `locals: Vec<Value>`, `stack: Vec<Value>`, `pc: usize = 0`
2. Pushes the frame onto the call stack
3. Runs the dispatch loop until `pc` reaches end or a Return op fires

For arbitrary-pc resume, the dispatcher needs to:
1. Allocate the Frame as above
2. **Populate `locals[k] = state.local_values[k]` for each k** (widening i64 back to Value::Number(f64) for now; ObjectRef widening when Option B above lands)
3. **Push `state.stack_values` onto the frame's operand stack** (preserving order; bottom-first)
4. **Set `pc = state.resume_pc`** (instead of 0)
5. Run the dispatch loop

The work is *not* huge — it's a custom `call_function_with_resume_state` entry point that diverges from the standard entry at step 1's frame initialization. The dispatch loop itself is unchanged.

### III.2 i64 → Value widening at resume

The current dispatcher widens the JIT's i64 return value back to Value::Number(f64) at one point. For resume, the widening must happen for every entry in `state.local_values` and `state.stack_values`.

For now: every recovered i64 is widened to `Value::Number(i64 as f64)`. This works for arithmetic deopts (the locals are all integer-valued Numbers; the f64 conversion is exact within ±2^53; values that overflow i64 are exactly the trip cases, where the interpreter takes over).

When ICs land, the IC trip's recovered state must include receiver objects. These need a different encoding (the i64 carries an ObjectRef index, not a Number). The DeoptSite's `JitLocation` could gain a `RegisterAsObject(u8)` variant; the dispatcher widens differently per variant.

### III.3 Why JIT-EXT 14 deferred this

The arithmetic trips (overflow) happen at the failing arith op's pc. For a function like `add(a, b)` (just one Add), the resume_pc is the Add op (around pc=6 typically). The locals at that pc are the args. Re-executing from pc=0 with the original args produces the same locals at pc=6, then re-runs the Add (now in the interpreter, with f64 widening). The result is correct.

So for first-cut arithmetic trips, re-execution from pc=0 is **observably equivalent** to resume-at-trip-pc — assuming the function has no side effects before the trip point. The current arith guards trip at the first Add, so this equivalence holds.

For ICs at non-zero pcs (e.g., a function that does several arithmetic ops THEN a GetProp), re-execution from pc=0 would redo the arithmetic. Wasted work, but still correct.

The motivation for landing resume-at-trip-pc:
- **Avoid redoing work**: a function with N ops before the trip currently redoes all N ops; resume-at-trip-pc would skip them.
- **Preserve side effects**: if the JIT'd body has any side effects before the trip (it currently doesn't, but ICs introduce property-write effects), re-execution from pc=0 would re-fire them. Resume-at-trip-pc is necessary for correctness once side effects exist.

ICs introduce property writes (via SetProp, future). Once SetProp lands, resume-at-trip-pc becomes correctness-critical.

## IV. The EXT plan

Five rounds to land real GetProp ICs:

### JIT-EXT 19: GetPropOnObject bytecode + translator lowering (always-call-runtime)

**Substrate**: add `Op::GetPropOnObject(prop_name_idx)` to bytecode. Upstream emitter unchanged for now (only typed-i64 tests exercise this). Translator handles the new op by emitting a call to `jit_getprop_miss(receiver_idx, prop_name_idx) -> i64`. No IC yet; every call goes to the runtime helper. End-to-end test: compile + invoke a function that does `obj.x`; verify the JIT returns the right Value.

~200 LOC across bytecode + translator + runtime helper.

### JIT-EXT 20: Single-shape IC at GetProp sites

**Substrate**: per-CompiledFn `ic_entries: Vec<ICEntry>`. Translator emits cache-lookup at each GetPropOnObject site: fast path reads the cached slot if `receiver.shape == cached_shape`; slow path calls the runtime helper, which updates the cache on first miss. End-to-end test: a function called twice with the same shape; second call uses the fast path (verified via instrumentation or perf counter).

~150 LOC across deopt.rs (ICEntry type) + translator (cache emission) + runtime helper (cache update).

### JIT-EXT 21: Dispatcher consume-recovered-state

**Substrate**: `Runtime::call_function_with_resume_state(proto, frame, resume_pc)`. Populates the frame from `state.local_values` + `state.stack_values`, sets pc = resume_pc, runs dispatch loop from there. Dispatcher's deopt fall-through case (currently re-executing from pc=0) is split: pc=0 → re-execute from pc=0 (current); pc!=0 → call_function_with_resume_state. Existing arith-guard tests confirm pc=0 path; new test asserts the pc!=0 path.

~100 LOC in interp.rs.

### JIT-EXT 22: Multi-shape IC with deopt on cache-full miss

**Substrate**: extend IC cache to record up to 4 shapes. Linear scan of cached shapes at the JIT site; on miss, runtime helper either appends to cache (count < 4) or triggers deopt (count == 4). The deopt path emits `ICShapeMismatch { ic_id }` with resume_pc = the GetProp op's pc. Dispatcher consumes the recovered state (JIT-EXT 21 made this work) and resumes the interpreter, which handles the polymorphic case.

~80 LOC.

### JIT-EXT 23: Mixed-regime support (Object args alongside Number args)

**Substrate**: extend `jit_compatible_int_arg` (or rename to `jit_compatible_arg`) to accept Object-valued args alongside Number-valued ones. The dispatcher passes the ObjectRef index as the i64. JIT'd code receives it as i64 and treats it as a pointer when used by GetPropOnObject. Functions that mix arithmetic and GetProp in the same hot path become JIT-compilable.

~50 LOC.

## V. What this design does NOT propose

- **No tagged-i64 union representation** (Option A). Disturbs arithmetic perf.
- **No general boxing at JIT entry** (Option C). Bounded but per-arg cost; deferred unless workload requires.
- **No multi-tier JIT**. Single tier remains (Doc 731 R1).
- **No SetProp in the first IC round**. SetProp introduces side effects that interact with deopt; landed after JIT-EXT 22 stabilizes.
- **No CallMethod ICs**. Same reason — separate substrate concern.
- **No prototype-walk in the IC**. First cut assumes the property lives on the receiver directly; prototype-chain lookups go through the slow path until JIT-EXT 24+.

---

*JIT-EXT 18 closes the audit and design round for the IC chapter. The remaining work is bounded: five more EXT rounds (19-23), each adding one measurable substrate move on top of the previous. Real GetProp ICs land at JIT-EXT 22 close.*
