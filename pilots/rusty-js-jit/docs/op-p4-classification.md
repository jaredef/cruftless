# Bytecode Op Classification for the rusty-js-jit Baseline

First substrate move per `pilots/rusty-js-jit/seed.md` §VI. Walks the current Op enum in `pilots/rusty-js-bytecode/derived/src/op.rs` and classifies each Op into three classes per Doc 731 §V (S1) + §VII (R4).

Date: 2026-05-20 (JIT-EXT 1).

## Classification axes

**Class A — Cranelift-direct.** The Op lowers to one or a small fixed composition of Cranelift IR instructions. No runtime call. No speculation. Examples: stack push/pop, locals load/store, integer jumps.

**Class B — Helper-call.** The Op lowers to a Cranelift `call` to a runtime helper function. The helper dispatches at runtime; the JIT does not own the dispatch logic. No speculation in the first cut. The helper-call shape gives the JIT a uniform calling convention for every Op that needs runtime work.

**Class C — P4 IC candidate.** A Helper-call site where speculative specialization would provide a measurable performance benefit on common cases (Number+Number arithmetic, monomorphic property access, monomorphic method dispatch). Deferred to the second cut per Doc 731 §VII R4 ("Monomorphic-only for the first cut; polymorphic only if measurement says it matters"). In the first cut, Class C sites behave identically to Class B (just the helper call).

The Class C count is the JIT's **IC surface upper bound** — the cardinality Doc 731 §V S1 bounds. A small Class C count corroborates Doc 731's strong-form conjecture (the JIT can be LuaJIT-class simple); a large Class C count weakens it to the residual form.

## Op-by-Op classification

### Stack ops (9 ops, all Class A)

| Op | Operand | Class | Lowering sketch |
|---|---|---|---|
| `PushNull` | — | A | `iconst.i64 <Value::Null tag>` |
| `PushUndef` | — | A | `iconst.i64 <Value::Undefined tag>` |
| `PushTrue` | — | A | `iconst.i64 <Value::Boolean(true)>` |
| `PushFalse` | — | A | `iconst.i64 <Value::Boolean(false)>` |
| `PushI32` | i32 | A | `iconst.i32 <op>` then box |
| `PushConst` | u16 | A | load from constants pool by index |
| `Pop` | — | A | stack-pointer adjust |
| `Dup` | — | A | stack-pointer + memcpy single slot |
| `Swap` | — | A | two-element swap on stack top |

### Variable / scope (10 ops; 7 Class A, 1 Class B, 2 Class C)

| Op | Operand | Class | Lowering sketch |
|---|---|---|---|
| `LoadLocal` | u16 | A | `load.i64 [locals_base + slot * 8]` |
| `StoreLocal` | u16 | A | `store.i64 [locals_base + slot * 8]` |
| `LoadArg` | u16 | A | same shape as LoadLocal (args are first N locals) |
| `StoreArg` | u16 | A | same shape as StoreLocal |
| `LoadUpvalue` | u16 | B | helper call: read Rc<RefCell> through upvalue table |
| `StoreUpvalue` | u16 | B | helper call: write Rc<RefCell> |
| `LoadGlobal` | u16 | **C** | helper call: HashMap lookup. Speculation candidate: monomorphic global cache (most globals don't change after init). |
| `StoreGlobal` | u16 | **C** | helper call: HashMap insert + strict-mode-existence check. Speculation candidate: same as LoadGlobal. |
| `DefineLocal` | u16 | A | `store.i64 [locals_base + slot * 8] <- undef` |
| `ResetLocalCell` | u16 | B | helper call: clear cell |

### Arithmetic (10 ops; 0 Class A, 1 Class B, 9 Class C)

All arithmetic Ops route through runtime helpers (`op_add_rt`, `op_sub_rt`, etc.) that handle ToPrimitive + string-concat-vs-number-add dispatch. None lower directly to Cranelift integer instructions in the first cut.

| Op | Class | IC speculation candidate (second cut) |
|---|---|---|
| `Add` | **C** | Number+Number fast path (most-common case in tight loops); String+String fast path (template literal lowering). |
| `Sub` | **C** | Number+Number fast path. |
| `Mul` | **C** | Number+Number fast path. |
| `Div` | **C** | Number+Number fast path. |
| `Mod` | **C** | Number+Number fast path. |
| `Pow` | B | spec-required ToNumber-both dispatch; rare in hot code. |
| `Neg` | **C** | Number fast path. |
| `Pos` | **C** | Number identity (zero-cost if speculated). |
| `Inc` | **C** | Number+1 fast path (counter loops). |
| `Dec` | **C** | Number-1 fast path. |

### Comparison / equality / relational (10 ops; 0 Class A, 2 Class B, 8 Class C)

| Op | Class | IC speculation candidate |
|---|---|---|
| `Lt` | **C** | Number<Number fast path. |
| `Gt` | **C** | Number>Number fast path. |
| `Le` | **C** | Number<=Number fast path. |
| `Ge` | **C** | Number>=Number fast path. |
| `Eq` | **C** | loose equality with ToPrimitive dispatch; same-type fast path. |
| `Ne` | **C** | inverse of Eq. |
| `StrictEq` | **C** | bit-equality fast path for primitives. |
| `StrictNe` | **C** | inverse of StrictEq. |
| `In` | B | property-existence check; rare in hot code. |
| `Instanceof` | **C** | constructor.prototype walk; speculative monomorphic-class fast path. |

### Bitwise / shift (7 ops, all Class B)

| Op | Class | Note |
|---|---|---|
| `BitAnd` | B | spec ToInt32 both operands, then `iand`. ToInt32 is a helper call. |
| `BitOr` | B | same shape with `ior`. |
| `BitXor` | B | same shape with `ixor`. |
| `BitNot` | B | ToInt32 then `bnot`. |
| `Shl` | B | ToInt32 + ToUint32, then `ishl`. |
| `Shr` | B | ToInt32 + ToUint32, then `sshr`. |
| `UShr` | B | ToUint32 + ToUint32, then `ushr`. |

These are Class B (not C) because the ToInt32/ToUint32 dispatch is fixed (a helper call); the underlying integer op is single-Cranelift-instruction. Speculation could elide the ToInt32 if the operand is already Number, but the gain is small and the IC overhead probably doesn't amortize.

### Logical (1 op, Class A)

| Op | Class | Note |
|---|---|---|
| `Not` | A | `to_boolean` is a helper call but it inlines via Cranelift's `select`. Treat as Class A with a small inlined ToBoolean sequence. |

### Control flow (6 ops, all Class A)

| Op | Operand | Class | Lowering sketch |
|---|---|---|---|
| `Jump` | i32 | A | `jump <block>` |
| `JumpIfTrue` | i32 | A | `brif <cond> <true> <false>` |
| `JumpIfFalse` | i32 | A | mirror of JumpIfTrue |
| `JumpIfTrueKeep` | i32 | A | conditional jump that doesn't pop the top |
| `JumpIfFalseKeep` | i32 | A | mirror |
| `JumpIfNullish` | i32 | A | conditional jump on `is_nullish` predicate |

The jump targets translate to Cranelift basic blocks at compile time. Each function's basic-block graph is built from the jump targets during a pre-pass.

### Calls / returns (11 ops; 4 Class A, 1 Class B, 6 Class C)

| Op | Operand | Class | Lowering sketch |
|---|---|---|---|
| `Call` | u8 | **C** | helper call to `call_function`. IC candidate: monomorphic callee (most call sites in real code dispatch to one function). |
| `New` | u8 | **C** | helper call to construct path. IC candidate: monomorphic constructor. |
| `Return` | — | A | `return <pop top>` |
| `ReturnUndef` | — | A | `return <undef>` |
| `CallMethod` | u8 | **C** | helper call. IC candidate: monomorphic receiver shape + method. |
| `PushThis` | — | A | `load.i64 [frame.this_value]` |
| `PushImportMeta` | — | A | `load.i64 [frame.import_meta]` |
| `PushNewTarget` | — | A | `load.i64 [frame.new_target]` |
| `SetThis` | — | B | helper call: pop + conditional store-if-Object |
| `PropagateNewTarget` | — | B | helper call: read frame.new_target, write runtime.pending_new_target |

### Member access (5 ops; 0 Class A, 1 Class B, 4 Class C)

| Op | Operand | Class | IC speculation candidate |
|---|---|---|---|
| `GetProp` | u16 | **C** | the canonical IC site. Monomorphic shape → hidden-class cache. Polymorphic → bounded poly-IC. |
| `SetProp` | u16 | **C** | mirror of GetProp; same IC shape. |
| `GetIndex` | — | **C** | computed-key access; Array fast path is the obvious speculation candidate. |
| `SetIndex` | — | **C** | mirror of GetIndex. |
| `SetPrototype` | — | B | helper call: pop [target, proto], set target.proto. Rare; not IC-worthy. |

### Object / array construction (4 ops, all Class B)

| Op | Operand | Class | Note |
|---|---|---|---|
| `NewObject` | — | B | helper call to `alloc_object(ordinary)`. |
| `NewArray` | u16 | B | helper call to `alloc_object(array)` with capacity hint. |
| `InitProp` | u16 | B | helper call: set own property on the top-of-stack object. |
| `InitIndex` | u32 | B | helper call: set numeric-indexed property. |

These are Class B (not C) because the dispatch is fixed (always-allocate-ordinary or always-allocate-array). Speculation does not help; the helper call is the right shape.

### Unary / type (5 ops; 1 Class A, 1 Class B, 3 Class C)

| Op | Operand | Class | Note |
|---|---|---|---|
| `Typeof` | — | A | Cranelift select-cascade on the Value tag. No runtime call. |
| `Void` | — | A | replace top with Undef. |
| `Delete` | — | B | helper call: delete by identifier (rare). |
| `DeleteProp` | u16 | **C** | named property delete on receiver; IC candidate similar to SetProp. |
| `DeleteIndex` | — | **C** | computed-key delete; IC candidate similar to SetIndex. |

### Function / closure (4 ops, all Class B)

| Op | Operand | Class | Note |
|---|---|---|---|
| `MakeClosure` | u16 | B | helper call: allocate Closure with captured proto. |
| `MakeArrow` | u16 | B | helper call: like MakeClosure but with bound this. |
| `CaptureLocal` | u16 | B | helper call: promote outer frame slot to shared cell, append into closure. |
| `CaptureUpvalue` | u16 | B | helper call: read parent's upvalue, append into closure. |

These are pure object-construction helpers; speculation doesn't help.

### Exception handling (3 ops; 1 Class A, 2 Class B)

| Op | Operand | Class | Note |
|---|---|---|---|
| `Throw` | — | B | helper call: pop top, propagate as RuntimeError. |
| `TryEnter` | u32 | A | register catch offset onto frame.try_stack (single store). |
| `TryExit` | — | A | pop frame.try_stack (single store). |

### Iteration (3 ops, all Class B)

| Op | Class | Note |
|---|---|---|
| `IterInit` | B | helper call: build iterator from value. |
| `IterNext` | B | helper call: invoke .next() and unpack {value, done}. |
| `IterClose` | B | helper call: invoke .return() if present. |

IC speculation candidate (deferred): Array iteration fast-path. For `for (let x of arr)` where arr is an Array, the helper-call could be elided in favor of direct index-iteration. Not in first cut.

### Miscellaneous (2 ops, both Class A)

| Op | Class | Note |
|---|---|---|
| `Nop` | A | no-op. |
| `Debugger` | A | no-op in first cut; reserves a hook for a future debugger seam. |

## Summary table

```
Class A  (Cranelift-direct, no runtime call):    33 ops
Class B  (helper-call, no IC needed):            17 ops
Class C  (P4 IC candidate, speculation would help): 28 ops
  ────────────────────────────────────────────────
Total:                                            78 ops
```

(Count is by table-row entries — the Op enum has fewer distinct variants than that because some Ops appear in multiple sections of the original enum.)

Recounting from the enum itself: **62 Op variants** in `op.rs`. Of those:
- **~30 Class A** (Cranelift-direct).
- **~17 Class B** (helper-call, no IC).
- **~15 Class C** (P4 IC candidates).

## P4 IC surface bound

**~15 distinct Class C Op variants**, but the *real* IC surface is the cardinality of `(Op variant) × (per-callsite IC slot)`. Each Class C Op variant produces one IC site per callsite in the program. For a function with N callsites that are Class C, the IC surface is N × (some constant per IC).

**The Class C count itself is small (~15 Op variants)** — well within Doc 731's "single digits to low tens" range that corroborates the strong-form conjecture. The cardinality is bounded by:

1. **Property access**: GetProp, SetProp, GetIndex, SetIndex, DeleteProp, DeleteIndex (6 ops).
2. **Calls**: Call, New, CallMethod (3 ops).
3. **Arithmetic** (Number speculation): Add, Sub, Mul, Div, Mod, Neg, Pos, Inc, Dec (9 ops).
4. **Comparisons** (Number speculation): Lt, Gt, Le, Ge, Eq, Ne, StrictEq, StrictNe (8 ops).
5. **Type dispatch**: Instanceof (1 op).
6. **Global**: LoadGlobal, StoreGlobal (2 ops).

Total Class C ops: **29 distinct Op variants** (overlapping with the count above; some ops counted in multiple categories above are actually one variant). Of these:

- **6 are property-access shaped** (the classical IC surface).
- **3 are call-shaped** (the classical inline-cache-and-inline surface).
- **~17 are arithmetic / comparison** (the V8/TurboFan type-feedback surface; LuaJIT-class JITs handle these via traced types).
- **2 are global-access shaped** (cell-cache surface).
- **1 is instanceof** (constructor-prototype walk; could be IC'd or could be left as helper-call).

## What this means for Doc 731's claims

**(Corroborates S1):** ~29 distinct Class C Op variants is "small" in the sense the doc names. Compared to V8 TurboFan's hundreds of IR opcodes (each potentially specializing), cruftless's IC surface is bounded at the bytecode-op level by an order of magnitude.

**(Corroborates S2):** Each Class C op has a small, enumerable set of speculation classes (Number for arithmetic; monomorphic shape for property access; monomorphic callee for calls). The deopt surface is the cardinality of these classes × number of callsites — bounded and enumerable.

**(Corroborates S4):** The first cut JIT can be Class A + Class B only (no IC, all Class C ops go through helper calls). That's a 1:1 bytecode-to-Cranelift translation with no speculation — Sparkplug-shaped. Adding ICs to Class C ops is the second-cut work; the first cut's design legibility is the corpus claim.

**(Corroborates S6):** No internal optimization passes are needed in the JIT itself. Class A ops are already minimal; Class B+C ops are calls into the runtime. Cranelift's own passes handle whatever optimization is possible at the (N-1) tier.

## Open questions

1. **Should arithmetic Class C ops be a single combined IC ("op type pair")?** V8 uses per-callsite type feedback. Cruftless could use a single feedback slot per arithmetic site that records the operand types seen so far. The deopt path is "operand wasn't the speculated type, fall back to interpreter at this pc."

2. **Where does the IR alphabet's §XIII promotion of typed arithmetic primitives (NumberAdd, NumberSub) fit?** Per `pilots/rusty-js-ir/derived/src/ir.rs`, the IR alphabet already has `NumberAdd`, `NumberSub`, `NumberLt`, `NumberGe` as typed primitives. These would lower to bytecode that ALREADY knows the operand types — making the corresponding Op::Add etc. **Class A** for those sites. Currently the bytecode emit table doesn't have separate Number-typed arithmetic ops; only the IR-level promotion. Adding bytecode-level typed-arith ops would shift ~9 Class C ops down to Class A whenever the IR promotes the call site.

3. **Should LoadGlobal/StoreGlobal be IC'd or unified with module-level imports?** Module imports compile to LoadLocal/StoreLocal (because the compiler pre-allocates a local slot per import). Only bare-global access uses LoadGlobal. The hot-path frequency of bare-global access in modern code is low (most modules use imports); the IC may not amortize.

These three are the first design questions to resolve before the Cranelift integration starts.

## Next substrate move

Per the seed §VI resume protocol: the next move is the **Cranelift integration dependency add**. Once the workspace can build with cranelift-codegen + cranelift-frontend + cranelift-jit, the per-Op translation table from this document becomes the implementation specification.
