# Φ-EXT 1 — f64 calling-convention design

*Pre-implementation design doc per seed §III item 2. Enumerates per-op IR-change deltas, per-extern signature changes, and per-dispatch-site changes for the f64-default calling convention shift. The substrate-introduction (Φ-EXT 2) + closure round (Φ-EXT 3) follow this enumeration.*

## 1. Scope reminder

Φ Move 1: change the JIT calling convention from i64-everywhere to f64-everywhere. The dispatcher passes f64; the JIT body does fadd/fsub/fmul; the return is f64. The dispatcher's precheck collapses from "integer-Number-or-Object" (tag + integer-validity) to "Number-or-Object" (tag-only). VTI's revival path (Φ-EXT 7) becomes structurally winnable: inline tag-check at JIT prologue is cheap (~3-5 cycles) vs the precheck's removed work.

Out of Φ's scope: Move 2 (typed-i64 promoted fast path via bytecode tier-1.5 IR per Doc 731 §XIII).

## 2. JitFn signature change

**Current** (pilots/rusty-js-jit/derived/src/translator.rs:38-47):
```rust
pub type JitFn1 = extern "C" fn(i64) -> i64;
pub type JitFn2 = extern "C" fn(i64, i64) -> i64;

pub enum JitFn {
    Arity1(JitFn1),
    Arity2(JitFn2),
}
```

**Post-Φ**:
```rust
pub type JitFn1 = extern "C" fn(f64) -> f64;
pub type JitFn2 = extern "C" fn(f64, f64) -> f64;

pub enum JitFn {
    Arity1(JitFn1),
    Arity2(JitFn2),
}
```

`call1` / `call2` methods change parameter + return types accordingly. The enum variants are unchanged in name; only the inner fn-pointer types shift.

**VTI variant** (currently piggy-backs on JitFn under VTI=1; per VTI-EXT 3b the JIT prologue treats i64 as `*const Value` reinterpret):

Post-Φ + Φ-EXT 7, VTI's calling-convention is unchanged at the JitFn type level (still passing f64), but the JIT prologue's per-arg handling differs: load receiver from f64-as-pointer-reinterpret. This is awkward; cleaner is to define a separate `JitFnVti1: extern "C" fn(usize) -> f64` for the VTI variant. Deferred to Φ-EXT 7's design; not part of Φ-EXT 3.

## 3. Per-op IR-change deltas

The translator emits Cranelift IR for each supported bytecode op. Per Doc 731 §XIII alphabet promotion (and the future Move 2), the bytecode tier may eventually emit typed ops (Op::AddI64 etc.) that lower to iadd. For Φ-EXT 3 the bytecode is unchanged; the JIT switches its lowering of the existing untyped ops to f64.

| Op | Current IR | Post-Φ IR | Notes |
|---|---|---|---|
| Op::Add | iadd | fadd | f64-add; NaN/Inf semantics from JS |
| Op::Sub | isub | fsub | |
| Op::Mul | smulhi+iadd (overflow-checked) | fmul | f64 doesn't overflow in i64-sense; existing IntegerOverflow deopt site becomes moot for fmul. Cleanup deferred. |
| Op::Inc | iadd const(1) | fadd fconst(1.0) | |
| Op::Dec | isub const(1) | fsub fconst(1.0) | |
| Op::PushI32 | iconst | fconst | The bytecode's i32 immediate becomes f64 (lossless: i32 fits in f64 mantissa). |
| Op::LoadLocal | use_var of I64 var | use_var of F64 var | Local declared as F64 instead of I64 |
| Op::StoreLocal | def_var of I64 var | def_var of F64 var | |
| Op::Lt | icmp slt | fcmp lt | Result is i8; promoted to f64 (0.0 or 1.0) for stack. |
| Op::Le | icmp sle | fcmp le | |
| Op::Gt | icmp sgt | fcmp gt | |
| Op::Ge | icmp sge | fcmp ge | |
| Op::Eq | icmp eq | fcmp eq | NaN ≠ NaN per JS == semantics — fcmp eq returns false for NaN comparisons, which matches JS. |
| Op::Ne | icmp ne | fcmp ne | |
| Op::StrictEq | icmp eq | fcmp eq | Same as Eq for Number-Number comparisons (per JS §7.2.16 SameValueZero collapses to ==). |
| Op::StrictNe | icmp ne | fcmp ne | |
| Op::JumpIfTrue | brif on i64 | brif on i8 from fcmp | Need to handle the stack-pushed boolean-as-f64 from cmp results: load and check non-zero. |
| Op::JumpIfFalse | brif on i64 | brif on i8 from fcmp | Same. |
| Op::Jump | jump | jump | Unchanged |
| Op::Dup | duplicate i64 | duplicate f64 | Stack type changes |
| Op::Pop | pop | pop | Unchanged |
| Op::Return | return i64 | return f64 | |
| Op::ReturnUndef | iconst 0 → return | fconst 0.0 → return | Will need a sentinel for actual Undefined; current path returns 0 which the dispatcher rebox as Value::Number(0.0). Post-Φ same shape but with f64 0.0. **Open question**: how do JIT-emitted functions return non-Number (e.g., undefined from a void function)? Current path returns 0 + rebox as Number(0). Post-Φ same. Workaround: JIT can only compile functions whose body returns Number (already true by the deopt mechanism's "non-Number return → deopt" pattern); no semantic change.

**Cranelift IR type changes**: every `I64` becomes `F64` for value-domain operations. Control-flow types (block-params, jump conditions) stay i8/i32 as Cranelift defaults. Comparison-result-on-stack needs a type-promotion: fcmp produces i8 → need to convert to f64 for stack consistency (0.0 / 1.0). Alternative: keep stack as i64 holding raw bits, convert to f64 only at fadd-input boundary. Simpler model: stack is f64 throughout; cmp results promoted via fcvt.

**Recommendation**: stack-is-f64-throughout model. Cleaner type model; cmp-result promotion via `iadd_const + fcvt_from_uint` or equivalent (cmp returns 0/1 in i8; uextend to i64; fcvt_from_sint to f64).

## 4. Per-extern signature changes

Externs that return Number-encoded values currently return i64 (which the JIT body interprets as truncated Number). Post-Φ they return f64 directly.

| Extern | Current signature | Post-Φ signature | Notes |
|---|---|---|---|
| `jit_getprop_on_object` | `(i64, i64) -> i64` | `(i64, i64) -> f64` | Args (receiver_idx, prop_name_idx) stay i64 (raw ids, not Number). Return is the property value as f64. |
| `jit_getprop_with_ic` | `(i64, i64, i64) -> i64` | `(i64, i64, i64) -> f64` | Same as above for the return. |
| `runtime_ic_fast_get` | `(i64, i64, i64) -> i64` | `(i64, i64, i64) -> f64` | Reads shape_values[slot]; if Number, returns f64; else returns sentinel. **Sentinel choice**: f64::NAN with a specific bit pattern? Or change return to `(i64, i64, i64) -> (f64, bool)` via a struct? Simpler: keep i64 return; let JIT interpret the bits via `f64::from_bits`. **Or**: use a quiet-NaN with a specific signaling bit set as the miss sentinel. JS's Number type never produces a quiet-NaN with set sign-bit (canonical NaN is 0x7FF8000000000000); a NaN with sign-bit set (0xFFF8000000000000) is unobservable to JS and serves as our sentinel. |
| `deopt_trip` | `(i64, i64, i64, i64, i64) -> i64` | `(i64, i64, i64, i64, i64) -> i64` | Live values r0-r3 stay i64 as raw-bits storage. The DeoptLiveLocal's type info (which we may need to add) tells reconstruct whether to interpret as i64 or as `f64::from_bits`. **For Φ-EXT 3 first cut**: assume all live values are f64-bits when JIT body is f64; reconstruct interprets accordingly. Backward-compat for typed-i64 fast path (Move 2): per-DeoptLiveLocal type tag added at that pilot. |

**Sentinel discipline for runtime_ic_fast_get**: use `f64::from_bits(0xFFF8000000000001)` as the miss sentinel — quiet-NaN with sign-bit set + bit-0 set. JS cannot produce this value; safe. JIT compares result via `f64::to_bits() == 0xFFF8000000000001` (one i64 cmp).

## 5. Per-dispatch-site changes (interp.rs call_function)

Three dispatch sites need updating: the standard JIT path, the TB fast-path, the deopt fallthrough.

### 5.a Standard path (interp.rs ~8485)

```rust
// Current:
if !jit_disabled
    && count >= self.jit_threshold
    && (params == 1 || params == 2)
    && args.len() == params as usize
    && args.iter().all(jit_compatible_arg)   // ← integer-validity check
{
    ...
    let r = match params {
        1 => { let a = unbox_arg(&args[0]); jit_fn.func.call1(a) }
        2 => { let a = unbox_arg(&args[0]); let b = unbox_arg(&args[1]); jit_fn.func.call2(a, b) }
        ...
    };
    ...
    return Ok(Value::Number(r as f64));   // ← i64→f64 rebox
}
```

```rust
// Post-Φ:
if !jit_disabled
    && count >= self.jit_threshold
    && (params == 1 || params == 2)
    && args.len() == params as usize
    && args.iter().all(jit_compatible_arg_tag_only)   // ← tag-only check
{
    ...
    let r = match params {
        1 => { let a = unbox_arg_f64(&args[0]); jit_fn.func.call1(a) }
        2 => { let a = unbox_arg_f64(&args[0]); let b = unbox_arg_f64(&args[1]); jit_fn.func.call2(a, b) }
        ...
    };
    ...
    return Ok(Value::Number(r));   // ← no cast; r is already f64
}
```

New helpers:
```rust
pub fn jit_compatible_arg_tag_only(v: &Value) -> bool {
    matches!(v, Value::Number(_) | Value::Object(_))
}

pub fn unbox_arg_f64(v: &Value) -> f64 {
    match v {
        Value::Number(f) => *f,
        Value::Object(id) => f64::from_bits(id.0 as u64),  // raw bits of ObjectId
        _ => 0.0,
    }
}
```

`jit_compatible_arg` is RENAMED to `jit_compatible_arg_tag_only` to make the semantic shift explicit. The Object branch uses `from_bits` to preserve the i64 pattern as a NaN-boxed-ish f64 (the JIT body still interprets as i64 for ObjectId arms when getting/setting properties — but for arithmetic ops on Object args the result is undefined behavior since Objects can't arith). Acceptable because the dispatcher's tag-only check still rejects non-Number/non-Object args, and the JIT body's IC GetProp path is the only Object-using op.

### 5.b TB fast-path (interp.rs ~8378)

```rust
// Current TB fast path:
let r = match params {
    1 => {
        let a = if vti {
            &args[0] as *const Value as i64
        } else { unbox_arg(&args[0]) };
        cf.func.call1(a)
    }
    ...
};
...
Some(Ok(Value::Number(r as f64)))
```

```rust
// Post-Φ TB fast path:
let r = match params {
    1 => {
        let a = if vti {
            f64::from_bits(&args[0] as *const Value as u64)  // ptr reinterpret
        } else { unbox_arg_f64(&args[0]) };
        cf.func.call1(a)
    }
    ...
};
...
Some(Ok(Value::Number(r)))
```

The VTI ptr-pass through f64 via `from_bits` is ugly but preserves the calling convention. Φ-EXT 7 introduces a cleaner `JitFnVti1: extern "C" fn(usize) -> f64` to avoid this. For Φ-EXT 3 first cut, the from_bits pattern is acceptable since VTI is default-OFF.

### 5.c Deopt fallthrough

No signature changes. The `take_last_deopt` mechanism is unchanged.

## 6. Cross-pilot composition checks

**Shape substrate**: shape_values stores Value enum; Φ doesn't touch this. The `runtime_ic_fast_get` extern reads shape_values[slot] and extracts f64 from Value::Number variant — currently returns `*n as i64`, post-Φ returns `*n` directly (no cast).

**STUB IC observer**: `runtime_ic_observe` doesn't return a value; signature unchanged.

**STUB IC fast-path (runtime_ic_fast_get)**: signature returns i64; post-Φ returns f64. The sentinel (`IC_FAST_MISS_SENTINEL = i64::MIN`) becomes the NaN-with-sign-bit pattern described in §4.

**TB metadata cache**: holds `NonNull<()>` (`*const CompiledFn`). CompiledFn's `func: JitFn` field type changes; TB cell value-type unchanged (still NonNull<()>). The dispatcher cast `&*(nn.as_ptr() as *const CompiledFn)` still works since CompiledFn's address is what's cached.

**VTI under env flag**: existing VTI implementation passes `*const Value` reinterpreted as i64; post-Φ it'd be reinterpreted as f64-bits. JIT prologue under VTI loads f64 payload at offset 8 from the pointer (same as current). The integer-validity check that was in the dispatcher's precheck is now... GONE entirely (no replacement), which is exactly what makes VTI's revival cheap. The JIT prologue's tag-check (currently skipped per VTI-EXT 3b "payload-extract-only") needs to be added back in Φ-EXT 7.

## 7. Backward-compat for typed-i64 path (Move 2 preparation)

Φ removes the i64 calling convention for the default JIT path. Move 2 (separate pilot) will re-add typed-i64 ops at the bytecode tier — but those ops will be Op::AddI64 etc., NOT the current Op::Add. The current Op::Add is forever-f64 post-Φ; the typed-i64 variant gets a NEW Op variant.

This preserves Doc 731 §VII R1 single-tier: there's still one JIT, but its translation table has both Op::Add (f64-lowered) and Op::AddI64 (i64-lowered) once Move 2 lands. Φ doesn't preempt Move 2's design.

**For Φ-EXT 3 first cut**: only Op::Add etc. are converted; no new typed Op variants added. Move 2 adds them when it lands.

## 8. Per-fixture validation plan (Φ-EXTs 4-6)

**Φ-EXT 4 composition re-bench**: run pilots/rusty-js-jit/tiny-baseline/scripts/composition-matrix.sh post-Φ-EXT 3. Compare against the post-StubE-EXT 8 + TB-EXT 8 baseline (71.2 / 81.0 ns):
- `none` bench_call_overhead: expect ~75-85 ns (small f64 overhead vs i64 on Pi)
- `none` bench_ic: expect ~83-90 ns
- TB+STUB bench_ic: expect ~80-90 ns (composition unchanged in shape)

**Φ-EXT 5 consumer-route probe**: 
- diff-prod 42/42 GREEN (existing fixtures don't depend on JIT i64 specifically)
- Cross-runtime-bench against current default; expect within ±5%
- NEW: fractional-Number fixture. e.g., `function half(x) { return x / 2; }` 1M iter. Currently can't JIT (returns non-integer); post-Φ should JIT and return correct fractional value.

**Φ-EXT 6 fuzz probe**: extend fuzz-tb.mjs with fractional Number args + non-Number args + NaN/Infinity edge cases. Verify all configs byte-identical with node.

## 9. Φ-EXT 7 VTI re-attempt design preview

Post-Φ, VTI re-attempt:
- Dispatcher under VTI=1: skip `jit_compatible_arg_tag_only`; pass `*const Value` reinterpret as f64-bits.
- JIT prologue: load discriminant byte from offset 0; cmp NUMBER_TAG; b.ne to deopt; load f64 payload from offset 8; def_var.
- Cost: ~5-10 cycles per arg (vs current dispatcher's tag-only check at ~3-5 cycles).
- Net: VTI may be neutral or slightly negative on bench_call_overhead. Win comes when VTI composes with TB (TB skips dispatcher entirely; VTI's prologue handles the precondition).

This is the genuine revival path. Φ-EXT 7's measurement determines whether VTI is (P2.a) or honestly remains (P2.d).

## 10. Risks + mitigations

**R1 — f64 arith slower per-op than i64**: aarch64 fadd 3-cycle latency vs iadd 1-cycle. Hot loops may slow ~10-30%. Mitigated by: (a) acceptable per C10 (engagement-tier baseline preserved within ±15% per Pred-φ.1); (b) Move 2 typed-i64 promoted fast path recovers performance for proven-integer loops; (c) most JS code is f64-dominant.

**R2 — Sentinel collision**: a runtime fixture might construct a Number that happens to have the bit pattern `0xFFF8000000000001` (our miss sentinel). Mitigated by: JS specifies canonical NaN bit pattern `0x7FF8000000000000`; producing the sign-bit-set NaN would require explicit bit manipulation via DataView. The fuzz fixture in Φ-EXT 6 should explicitly probe this.

**R3 — Deopt path live-value type drift**: live values in DeoptLiveLocal are i64-bits today; post-Φ they're f64-bits. Reconstruct must interpret correctly. Mitigated by: per-LiveLocal type tag added if needed; Φ-EXT 3 first cut assumes all live values f64-bits.

**R4 — TB cell-cached pointer breakage**: TB-EXT 7 fix (Box-wrap) protects against HashMap rehash; the type change here doesn't reintroduce dangling. Standing rule 9 audited: no new raw-pointer caches introduced.

**R5 — VTI under env flag becomes useless mid-Φ**: existing VTI-EXT 3b code passes `*const Value as i64`; post-Φ it'd need `as u64 as f64::from_bits()`. Mitigated by: VTI default-OFF; the patched VTI path requires Φ-EXT 7 anyway. Existing VTI code stays compilable + functionally wrong; users opting in get (P2.d)+wrong until 3c-equivalent lands at Φ-EXT 7.

## 11. Forward to Φ-EXT 2 (substrate-introduction)

Φ-EXT 2 lands the JitFn signature change + dispatcher arg-passing change WITHOUT changing the JIT body's IR. The JIT body still does iadd etc., but receives args as f64 (then internally `fcvt_to_sint_sat` to i64 — same op VTI-EXT 3b used). Performance neutral; substrate in place. Verify all gates GREEN.

Φ-EXT 3 then flips the JIT body to fadd. The substrate is the apparatus; the closure is the codegen change.

This staged-validation discipline mirrors TB-EXT 3a (substrate-introduction) + TB-EXT 3b (closure round). The pattern compounds.

---

*Φ-EXT 1 closes. Design enumerated. Φ-EXT 2 begins the substrate-introduction.*
