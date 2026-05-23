# VTI-EXT 2 — Inline tag-check emitter design

*Establishes cruft's Value encoding, surfaces the structural recognition that the LeJIT-Ψ telos as originally framed has no tag-check inside the JIT body to inline, lays out three design options against the empirical reality, and recommends a path under Doc 735 §X.h.b discipline.*

## 1. cruft's Value encoding

Read from `pilots/rusty-js-runtime/derived/src/value.rs:75-97`:

```rust
#[derive(Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(Rc<String>),
    BigInt(Rc<crate::bigint::JsBigInt>),
    Symbol(Rc<String>),
    Object(ObjectRef),
}
```

**Encoding facts**:
- Tagged Rust enum, eight variants. NOT NaN-boxed.
- NO `#[repr]` attribute. Layout is Rust-compiler-discretion.
- Variant payloads are mixed size: `bool` (1 byte), `f64` (8 bytes), `Rc<T>` (8 bytes, two pointers post-monomorphization for the strong+weak count headers), `ObjectRef` (whatever ObjectRef is — read below).
- Rustc's layout algorithm picks discriminant placement + padding to minimize size; the resulting layout is stable across a single build but not specified or pinned.

**Consequence for inline emission**: the JIT cannot read the discriminant at a known offset because there is no known offset. Without `#[repr(C, u8)]` or equivalent layout pinning, any inline discriminant load is undefined behavior even if it happens to work on aarch64 against the rustc 1.x layout.

## 2. The arg-coerce path — where the cost actually lives

Read from `pilots/rusty-js-runtime/derived/src/interp.rs:8420-8438` (dispatcher's JIT-call site) + `:8851-8869` (the unbox helpers):

```rust
// Dispatcher arg-coerce, called BEFORE the JIT body:
1 => {
    let a = unbox_arg(&args[0]);          // Rust match on Value
    jit_fn.func.call1(a)                  // JIT receives i64
}

// unbox_arg:
pub fn unbox_arg(v: &Value) -> i64 {
    match v {
        Value::Number(f) => *f as i64,
        Value::Object(id) => id.0 as i64,
        _ => 0,
    }
}
```

**The structural finding**: by the time the JIT body executes, args are already unboxed to `i64`. The JIT-emitted prologue does not tag-check — there is no tag in scope at JIT-body time. The cost VTI-EXT 1 measured at 127 ns/iter is in the Rust dispatcher's match + arg-coerce + `Rc::clone` for `closure_v` + `Vec<Value>` allocation, NOT in any JIT-prologue tag-check.

**The LeJIT-Ψ telos as originally framed (LeJIT seed §I.2 (b))** reads: *"Value-tag inline checks. cruftless's Value encoding is finer-grained than Cranelift's IR sees. A hand-rolled emitter that knows the Value layout can emit one inline branch-on-tag where Cranelift routes through a function-call abstraction."*

The framing assumes there is a tag-check inside the JIT body that an extern call currently routes through. Empirically there is not. The tag-check happens in Rust at the dispatcher's `match`, and the JIT receives a pre-unboxed `i64`. The "one inline branch-on-tag where Cranelift routes through a function-call abstraction" does not exist to be replaced.

## 3. Three options against the empirical reality

### Option A — Push arg-coerce into JIT emission (the original intent, plus the work to make it real)

**Move**: change the JIT calling convention so the JIT body receives `*const Value` instead of `i64`. The JIT-emitted prologue takes responsibility for the tag-check + payload-extract + deopt-on-non-Number. The dispatcher's `unbox_arg` becomes a slow-path fallback used only when the JIT body deopts.

**Substrate work**:
1. Pin Value layout. Add `#[repr(C, u8)]` to `pub enum Value`. Stabilizes discriminant at offset 0 (one byte), payload at offset aligned to max-payload-alignment (8 bytes for f64/Rc). Verify with `std::mem::offset_of!` and a const assertion.
2. Change `JitFn::call1` / `call2` signatures from `fn(i64) -> i64` to `fn(*const Value) -> i64`. Translator changes correspondingly.
3. JIT translator's function prologue emits per-arg: load discriminant byte → cmp NUMBER_TAG (= rustc's chosen value, read from the const assertion) → b.ne to deopt_thunk(WrongTag) → load f64 at payload offset → fcvtzs to i64 → store in local slot.
4. Dispatcher drops `unbox_arg` from the hot path; passes `&args[0] as *const Value` directly. The Vec<Value> allocation cost stays (that's tiny-baseline's target, not VTI's).
5. Deopt path: `WrongTag` deopt branches to interpreter via the existing `take_last_deopt` mechanism per StubE-EXT 5b.

**Cost estimate**:
- Layout pinning: 5 LOC + 1 const assertion + risk of one compile error to fix.
- Translator prologue: ~80 LOC for the per-arg inline sequence.
- Calling-convention switch: ~30 LOC across JitFn signature + dispatcher + bench harness.
- Deopt wiring for WrongTag: ~20 LOC (new DeoptReason variant + reconstruct path).
- Total: ~135 LOC, comparable to StubE-EXT 1-5b's budget.

**Expected speedup** (against 127 ns baseline):
- Removes the Rust `match` from the dispatcher: ~5-10 ns.
- Removes one branch-mispredict potential per non-monomorphic call site: not measurable at the bench's monomorphic Number-only workload.
- Adds the inline cmp + b.eq (3-4 aarch64 instructions, 1-2 cycles when predicted): negligible cost on the hot path.
- Net: ~5-10 ns reclaimed. Pred-vti.1's 20-ns threshold remains tight; Pred-vti.4 composition target is reachable IF tiny-baseline lands the dispatcher refactor alongside.

**Risk** (per Doc 735 §X.h.b sub-cases):
- (P2.c) illegal-speed if the layout assumption drifts under a rustc upgrade. Const-assertion catches at compile time; the risk is bounded but real.
- (P2.d) correct-but-losing if the 5-10 ns reclaim is dwarfed by tiny-baseline's larger arm. Currently the empirical reading per VTI-EXT 1 places dispatcher-refactor at 1.5-2× of the per-iter cost; VTI's contribution at this size is ~5-10% of that. Composition is load-bearing.

### Option B — Tag-read via raw pointer in Rust dispatcher (faster Rust, not JIT-emitted)

**Move**: replace `match` with a `transmute_copy` or unsafe discriminant read in `unbox_arg`. Bypasses Rust's match-codegen overhead. Stays in Rust; no JIT-side change.

**Substrate work**: ~20 LOC of unsafe in `unbox_arg`. Layout pinning still required.

**Expected speedup**: 1-3 ns (Rust match is already well-optimized; the savings are marginal).

**Risk**:
- (P2.c) illegal-speed risk identical to Option A.
- This is not really LeJIT-Ψ work — it's Rust dispatcher microoptimization. Misfit with the pilot's hybrid-codegen telos.

### Option C — Recognize VTI as (P2.d) and pivot to dispatcher refactor

**Move**: per Doc 735 §X.h.b, today's empirical reading at VTI-EXT 1 places LeJIT-Ψ's contribution at 5-15 ns out of 127 ns dispatcher cost. The pilot may be correct-but-losing relative to the dispatcher refactor (tiny-baseline, pre-filed at LeJIT seed §I.2 item 5) which targets the dominant 1.5-2× arm. Re-categorize VTI as (P2.d), defer it, spawn `pilots/rusty-js-jit/tiny-baseline/` per Doc 737 §IV's "pre-file generously, spawn only when the substrate calls" — and the substrate is now calling.

**Substrate work**: zero at VTI; close VTI-EXT 2 with this finding recorded; spawn tiny-baseline.

**Risk**:
- (P2.b) wrong-stratum-composition if tiny-baseline cannot achieve its 1.5-2× target without VTI's arg-coerce inlining as substrate. Empirically the dispatcher refactor's targets (Vec<Value> allocation, Frame setup, closure-bound-this clone) are independent of arg-coerce; VTI is a separate arm of the multiplicative composition per LeJIT §I.3.
- Foregoing 5-10 ns of reclaim. Acceptable if tiny-baseline's 1.5-2× lands.

## 4. Recommendation

**Option A**, staged behind a `CRUFTLESS_LEJIT_VTI=1` env flag per the StubE-EXT precedent. Reasoning:

1. **Aligned with the pilot's named telos**. LeJIT seed §I.2 (b) names value-tag inline checks as a structural site Cranelift cannot reach. Today's recognition is that the current dispatcher hides the tag-check from the JIT entirely; Option A is the substrate move that makes the tag-check JIT-visible so the inline emission has something to replace.

2. **Layout pinning is a substrate-amortization-cascade enabler**. Per Doc 729 §A8.13, the `#[repr(C, u8)]` move pays once and is consumed at every JIT call. Future optimizations (LeJIT seed §I.2 (a) IC stub patching's value-payload extraction, dispatcher refactor's Frame setup) all benefit from the pinned layout. The work is not VTI-specific.

3. **Composition with tiny-baseline is intact**. VTI's ~5-10 ns reclaim + tiny-baseline's 1.5-2× arm + LeJIT-Σ's IC contribution + shape's 1.36× already-landed cascade are the four arms of the §I.3 multiplicative composition. None is sufficient alone; all four together reach Pred-stub.1's 3× target.

4. **Honest budget**: Pred-vti.1 (≥20 ns per-iter reduction on arg-coerce) is not reachable from VTI alone. Update Pred-vti.1 to read "≥5 ns" against `bench_call_overhead`; keep Pred-vti.4 (composition target ≤120 ns on bench_ic) as the load-bearing falsifier.

**Pred-vti.1 update** (proposed for VTI-EXT 3): the inline tag-check + layout pinning reduces per-iter cost on `bench_call_overhead` by ≥5 ns (was: ≥20 ns). Pred-vti.4 remains: bench_ic under shape + LEJIT_STUB + LEJIT_VTI ≤ 120 ns/iter.

## 5. Open items for VTI-EXT 3 scaffold

- **Discriminant value for Number variant**: rustc picks 0..7 in declaration order for `#[repr(u8)]` enums absent explicit assignment. With `Value::Number` at position 3, the const NUMBER_TAG = 3. Verify via `std::mem::discriminant`-based const assertion in `value.rs`.
- **Payload offset**: with `#[repr(C, u8)]`, the payload starts at the max-alignment boundary after the 1-byte tag. For f64 (alignment 8), payload offset = 8.
- **WrongTag deopt**: new `DeoptReason::WrongArgTag { arg_index: u8, expected: u8, observed: u8 }` variant. Reconstruct path adds the arg back to the locals slot per the existing arith-deopt template.
- **Calling-convention switch**: the bench harness `bench_call_overhead.rs` plus `host-v2/tests/jit_*.rs` consumers need the `call1(*const Value)` signature update. Done as part of EXT 3 scaffolding.

## 6. Forward to VTI-EXT 3

Scaffold `pilots/rusty-js-jit/derived/src/value_tag_inline.rs` per Doc 738 §II.e pillar-path. Function names follow §II.b post-§A8.32 receiver-discriminated form (no `_via` since these are JIT-emitter functions, not Runtime-dispatching helpers): `emit_inline_number_tag_check`, `emit_inline_payload_extract`, `emit_arg_coerce_prologue`. Behind `CRUFTLESS_LEJIT_VTI=1` env flag, default OFF.

Const assertions in `value.rs` lock NUMBER_TAG + payload offset; one round trip with the layout-pinning commit before any emission code lands. EXT 3a (layout pinning + const assertions) is the substrate-introduction round per Doc 729 §A8.13; EXT 3b+ (emission code) is the closure round.

---

*VTI-EXT 2 closes. Value is a tagged Rust enum with unpinned layout. The original LeJIT-Ψ framing assumed a tag-check inside the JIT body that does not exist; the cost lives in the Rust dispatcher's `match`. Option A (push arg-coerce into JIT emission via layout pinning) is recommended; estimated reclaim ~5-10 ns/iter; load-bearing falsifier shifts to Pred-vti.4 composition target. VTI-EXT 3a begins with the layout pinning commit.*
