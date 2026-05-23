# TB-EXT 2 — Dispatcher decomposition audit

*Partitions `Runtime::call_function`'s ~125 ns shape-invariant per-iter cost into named source-tier components. The named-cost partition is TB-EXT 3b's substrate-move target list: each component is classifiable as eliminable-by-compile-time-resolution, eliminable-by-thunk-inlining, eliminable-by-Vec-replacement, or unavoidable.*

## 1. Scope and method

**Source read**: `pilots/rusty-js-runtime/derived/src/interp.rs:8331-8460` — the `call_function` method's hot JIT-success branch for a Closure callee with 1-2 compatible-Number args.

**Estimation method**: per-component ns estimates anchored against the measured ~125 ns shape-invariant baseline (TB-EXT 1) and the empirical measurements from VTI-EXT 1/3a/3b. Where the sum of identified-component estimates falls short of the total, the gap is named explicitly as the TB-EXT 6 micro-profiling target.

**Hardware target**: Raspberry Pi (Cortex-A76 @ 2.4 GHz). Per-op costs are roughly 2-3× the equivalent x86_64 desktop estimates from canonical optimization literature.

## 2. Per-iter cost decomposition

**Caller side** (the bench's `rt.call_function(closure_v.clone(), Value::Undefined, vec![arg.clone()])` invocation):

| component | est. ns | location |
|---|---:|---|
| `closure_v.clone()` (Value::Object → Rc::clone) | 1-2 | bench harness |
| `vec![arg.clone()]` (Vec alloc + Value::Number clone) | 3-5 | bench harness |

**Subtotal caller**: ~4-7 ns. Eliminable by Vec replacement (`call_function_inline_args(closure_v, this, &[arg])` or direct register-passing thunk).

**Callee side** (`call_function` body through the JIT call and back):

| # | component | est. ns | source line | classification |
|---|---|---:|---:|---|
| 1 | Value match on callee | 1-2 | 8332-8353 | thunk-inline |
| 2 | `pending_new_target.take()` | 1 | 8357 | thunk-inline (typically None) |
| 3 | `self.obj(id)` heap-vec index | 1-2 | 8361 | compile-time-resolve (thunk holds &Closure) |
| 4 | InternalKind::Closure match | 1 | 8362 | thunk-inline |
| 5 | `call_count` get+set Cell ops | 1 | 8373-8374 | thunk-inline (or skip; already hot) |
| 6 | `proto_key` compute (Rc::as_ptr cast) | <1 | 8375 | compile-time-resolve |
| 7 | is_arrow + actual_this Value::clone | 2-3 | 8376-8386 | compile-time-resolve (per-closure constant) |
| 8 | `params` field load | <1 | 8387 | compile-time-resolve |
| 9 | `jit_disabled.get()` Cell::get | <1 | 8388 | thunk-inline |
| 10 | `jit_compatible_arg` per-arg match | 3-5 | 8393 | thunk-inline (or VTI-style inline tag-check) |
| 11 | `proto_rc.clone()` Rc::clone | 1 | 8397 | compile-time-resolve |
| 12 | `jit_cache.contains_key()` HashMap | 3-5 | 8400 | compile-time-resolve (thunk holds slot pointer) |
| 13 | `jit_cache.get()` HashMap | 3-5 | 8420 | compile-time-resolve |
| 14 | rt + proto pointer captures (as_ptr) | 1-2 | 8418-8419 | compile-time-resolve |
| 15 | `set_current_*` 3 TLS writes | 3-5 | 8421-8423 | restructure or amortize |
| 16 | `vti_enabled` bool field load | <1 | 8430 | thunk-inline |
| 17 | `unbox_arg` per-arg match | 2 | 8436 | thunk-inline |
| 18 | `jit_fn.func.call1` match + extern call + id body | 5-10 | 8438 | unavoidable (real call) |
| 19 | `clear_current_*` 3 TLS writes | 3-5 | 8451-8453 | restructure or amortize |
| 20 | `take_last_deopt` TLS read + check | 1 | 8454 | thunk-inline (deopt-set => fallback) |
| 21 | `Value::Number(r as f64)` rebox | 1-2 | 8459 | thunk-inline |
| 22 | `Result::Ok` wrap | <1 | 8459 | thunk-inline |

**Subtotal callee identified**: ~35-58 ns.

**Total identified (caller + callee)**: ~39-65 ns.

**Measured per-iter (id1, working baseline)**: ~125 ns.

**Unidentified gap**: ~60-86 ns. This is the load-bearing finding of TB-EXT 2.

## 3. The unidentified gap

The named-component estimates sum to roughly half the measured cost. The gap of ~60-86 ns is real and lives somewhere in the per-call path. Plausible locations:

**(a) Cache misses on Pi.** The dispatcher touches at least eight pieces of memory across distant regions: `args` Vec on the stack, `callee` Value on the stack, `self.heap[id]` in the runtime, `c.proto.params`, `c.call_count`, `c.jit_disabled`, `c.bound_this`, the JIT cache HashMap, the three TLS slots, the JitFn vtable, the deopt_sites slice. On the Cortex-A76's 64-byte cache lines and 64 KB L1D, sustained cache pressure across these accesses likely costs 20-40 ns of memory-stall time. Estimates above assume L1 hits; Pi reality is likely L2 or worse for many accesses.

**(b) Branch mispredicts on the multi-condition `if`.** Lines 8389-8393 form a five-condition AND. Branch predictors are unlikely to predict all five clauses correctly on every iteration even in a hot loop. Each mispredict on Cortex-A76 costs ~10-15 cycles (~4-6 ns). 1-2 mispredicts per call accounts for 5-10 ns.

**(c) HashMap lookup overhead is higher than estimated.** `HashMap<usize, Option<CompiledFn>>` on the rusty-js-runtime is std's SipHash-13 by default; per-lookup cost is closer to 10-15 ns than my estimate of 3-5 ns. Two lookups per call ≈ 20-30 ns. This likely accounts for the largest single gap component.

**(d) TLS slot access on Pi.** `std::thread_local!` on aarch64 Linux uses TPIDR_EL0 + dispatch table lookup; per-access cost is ~5-10 ns. Six TLS accesses per call ≈ 30-60 ns. This is also likely a substantial gap component.

**(e) `Value::clone` on `this` (Undefined here) is cheaper than estimated** for trivially-copyable variants; but for the Object/String cases it includes Rc strong-count atomic bump (~3-5 ns).

**(f) Vec deallocation at call_function return** when the args Vec is dropped: free-list path is fast but non-zero (~1-2 ns).

**Hypothesis**: (c) HashMap + (d) TLS together account for ~40-70 ns of the gap. (a) cache misses + (b) branch mispredicts account for the remainder. The TB-EXT 6 micro-profiling round will pin this empirically via per-component instrumentation.

**Implication for the substrate move**: every component on the eliminable side of §2's classification is in scope for TB-EXT 3b. The two largest single sources of reclaim are:
- **(c) HashMap lookups** → compile-time-resolve to a stable slot pointer in the thunk's per-function metadata; reclaim ~20-30 ns.
- **(d) TLS writes/reads** → restructure to set/clear at thunk-construction time only, OR move to a single-write-per-thunk-entry pattern; reclaim ~20-40 ns.

If both are landed cleanly, the substrate move alone reclaims ~40-70 ns — well within striking distance of Pred-tb.1's ≥40 ns target.

## 4. Component classification for TB-EXT 3b targeting

Grouped by elimination mechanism:

**Eliminable by compile-time-resolution** (~12-22 ns directly + ~20-30 ns HashMap gap absorption):
- Components 3, 6, 7, 8, 11, 12, 13, 14
- Mechanism: at JIT-compile time, build a `TinyBaselineMetadata` struct per JIT-eligible function holding: `&Closure` (resolved from id), `Rc<FunctionProto>`, JIT-cache slot pointer, baked `actual_this` per closure (when bindable), `params`, raw thunk function pointer. The dispatcher under TB=1 reads this struct once instead of computing/looking-up each component.

**Eliminable by thunk-inlining** (~10-15 ns directly):
- Components 1, 2, 4, 5, 9, 10, 16, 17, 20, 21, 22
- Mechanism: under TB=1, the dispatcher's match-on-Value + InternalKind dispatch + per-arg unbox + result rebox happens inside an inline call thunk specialized per closure. The thunk's preamble is straight-line code (~10-15 aarch64 instructions); no Rust match codegen overhead.

**Eliminable by Vec-replacement** (~4-7 ns):
- Caller-side `vec![arg.clone()]`
- Mechanism: introduce `call_function_n` variants for n ∈ {0, 1, 2} that take `&[Value]` (or direct register args via a hand-rolled calling convention). The thunk's call ABI uses these directly. Eliminates Vec alloc + dealloc per call.

**Restructure-or-amortize** (~12-30 ns including gap):
- Components 15, 19 (TLS sets/clears)
- Mechanism: the TLS slots (`CURRENT_DEOPT_SITES`, `CURRENT_RUNTIME`, `CURRENT_PROTO`) exist so JIT-emitted extern callbacks can read state without parameter-passing. For TB-eligible functions: bake the same state into the per-function metadata; the thunk passes a metadata pointer to the JIT body; the extern callbacks read from there instead of TLS. Eliminates per-call TLS traffic entirely.

**Unavoidable** (~5-10 ns):
- Component 18 (the JIT call itself + id body)
- Plus 1-2 ns of irreducible function-call ABI cost

**Total reclaim estimate**: 12-22 + 10-15 + 4-7 + 12-30 = **38-74 ns**. Mid-range estimate ~55 ns. Comfortably above Pred-tb.1's ≥40 ns target.

## 5. The capability-passing constraint (Doc 736 §IX.6)

The TB metadata struct holds pointers to closure + proto + JIT cache slot. Under capability-passing Mode-3 (sealed), the same struct exists but the thunk's entry preamble must check the runtime's mode flag and route to the standard dispatcher path if Mode > 0. This is a single load + compare + branch (~2 ns); cheaper than the per-call work it conditionally skips.

The TB-EXT 3b first cut carve-out (per seed §IV "Mode-0 only at first cut") means the thunk does not need to handle Mode 1/2/3 fast-path correctness — it simply falls through to the standard dispatcher whenever the mode flag is non-zero. The carve-out is removable once the verification work has run; the framework permits the fast path under all modes structurally.

## 6. Thunk shape

The TB-EXT 3b inline call thunk for a 1-arg function compiles to roughly this shape on aarch64:

```
tb_thunk_<id>:
  // (mode check: skip for first cut Mode-0-only)
  // ldr w_mode, [x_runtime, #MODE_OFFSET]
  // cbnz w_mode, fallback_to_standard_dispatcher
  
  // Receive arg in x0 (calling-convention determined)
  // Per VTI lessons learned: pass i64-unboxed value, not pointer.
  
  // Set TLS slots ONCE per thunk type? Or skip — use metadata-passing.
  // (Decision: pass metadata pointer in x1; extern callbacks read from there.)
  
  // ldr x_jit_fn, [x_metadata, #JIT_SLOT_OFFSET]
  // mov x1, x_metadata        // optional, for callbacks
  // br x_jit_fn               // tail-call into JIT body
  
  // On return: rebox r as Value::Number, wrap Ok, return.
  // ...
```

Estimated size: ~15-25 aarch64 instructions. Compile latency: microseconds (per LeJIT seed §I.2 item 5's Sparkplug-style claim). Per-call cost: ~30-50 ns (the unavoidable JIT call body + minimal preamble), down from ~125 ns standard.

## 7. Forward to TB-EXT 3a

TB-EXT 3a (substrate-introduction per Doc 729 §A8.13): build the `TinyBaselineMetadata` struct + the per-JIT-function table that maps proto_key → metadata. No thunk emission yet; the table is the apparatus. Verify the table's content matches what the dispatcher currently computes per call.

TB-EXT 3b (closure round): emit the inline thunk for ≤2-arg ≤20-op functions under `CRUFTLESS_LEJIT_TB=1`. Dispatcher under flag checks per-call eligibility; eligible calls route through thunk; ineligible fall to standard dispatcher.

---

*TB-EXT 2 closes. ~125 ns dispatcher cost decomposed: ~40-65 ns identified across 22 components; ~60-86 ns unidentified gap attributed primarily to HashMap (~20-30 ns) + TLS (~20-40 ns). TB-EXT 3b's named-component target list yields ~38-74 ns reclaim estimate; mid-range ~55 ns is well above Pred-tb.1's ≥40 ns falsifier threshold.*
