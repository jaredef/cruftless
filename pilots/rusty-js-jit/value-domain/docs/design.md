# VD-EXT 1 — NaN-boxing encoding scheme

*NaN-boxing bit layout + tag values + encoder/decoder reference for extending the Φ calling convention to carry all Value variants into JIT bodies. Closes the value-domain coverage tier per Finding VII.3 (engagement findings.md Addendum V).*

## 1. Design constraints (C1-C8 from seed §I.2)

```
C1. Existing Σ/Τ/Ψ/Φ default-on bench numbers byte-identical post-VD.
C2. unbox_arg_f64(Value::Number(n)) === n preserved exactly.
C3. unbox_arg_f64(Value::Object(id)) === f64::from_bits(id.0 as u64) preserved exactly.
C4. String encoding via NaN-pattern that doesn't collide with arithmetic NaN.
C5. box_to_value(unbox_arg_f64(v)) === v for any v ∈ {Number, Object, String, Boolean, Null, Undefined}.
C6. Canonical fuzz acc=-932188103 byte-identical throughout.
C7. diff-prod 42/42 throughout.
C8. JIT lib tests 38/38 throughout (existing 9 ignored remain ignored).
```

## 2. The encoding (NaN-boxing with sign-bit-set distinguishing pattern)

### 2.1 Bit layout

All boxed (non-Number) values use the IEEE 754 NaN pattern with sign bit = 1:

```
Bit:   63 | 62 ────── 52 | 51 ── 48 | 47 ───────────── 0
       │   │             │         │                    │
       sign exponent      tag       payload
       =1   = 0x7FF       (4 bits)  (48 bits)
       │   (all 1s)       │         │
       │   (NaN signal)   16 types  Rc ptr / ObjectId / boolean
```

**Mask**: `0xFFF0_0000_0000_0000`
**Encoded value**: `mask | (tag << 48) | payload`

### 2.2 Tag values (4-bit, 16 slots; only 7 used at first cut)

| tag | decimal | Value variant | payload semantics |
|---|---:|---|---|
| 0x0 | 0 | reserved (would alias mask itself with payload=0; reserve to detect bugs) | n/a |
| 0x1 | 1 | Object | ObjectId.0 truncated to 48 bits |
| 0x2 | 2 | String | Rc<String> raw pointer (48-bit virtual address) |
| 0x3 | 3 | BigInt (deferred to VD-EXT 4) | Rc<JsBigInt> raw pointer |
| 0x4 | 4 | Symbol (deferred to VD-EXT 6) | Rc<String> raw pointer (symbol name) |
| 0x5 | 5 | Boolean (deferred to VD-EXT 5) | 0 or 1 |
| 0x6 | 6 | Null (deferred to VD-EXT 5) | 0 (payload unused) |
| 0x7 | 7 | Undefined (deferred to VD-EXT 5) | 0 (payload unused) |
| 0x8-0xF | 8-15 | reserved for future variants | n/a |

Note: the tag values don't match VALUE_TAG_X constants from value.rs (which use 0-7 for Undefined-through-Object). The NaN-boxing tags are independent because (a) tag 0 must be reserved (otherwise the pure mask `0xFFF0_0000_0000_0000` would alias VALUE_TAG_UNDEFINED with payload 0, but we want Undefined to encode as `mask | 0x7 << 48` for explicit disambiguation), and (b) Number doesn't get a tag (it stays unboxed). VD-EXT 2 implementation introduces NEW constants `VD_TAG_OBJECT = 1`, etc., to make the distinction explicit.

### 2.3 Number disambiguation

A Number is unboxed: its IEEE 754 bit pattern is whatever its f64 value is. To detect "is this a Number or a boxed value?", check whether the high 12 bits match the boxed-NaN mask:

```rust
fn is_boxed(f: f64) -> bool {
    f.to_bits() & 0xFFF0_0000_0000_0000 == 0xFFF0_0000_0000_0000
}
```

**Real NaN handling**: arithmetic NaN in IEEE 754 has many possible bit patterns. The "canonical qNaN" is typically `0x7FF8_0000_0000_0000` (sign=0, exp=0x7FF, top mantissa bit=1). Note: sign bit is 0, so this does NOT match the boxed-NaN mask. **Arithmetic NaNs remain unboxed and are treated as Numbers correctly.**

The edge case: if hardware or user code produces a NaN with sign=1 and the boxed-NaN mask pattern, that NaN would be mis-decoded as a boxed value. Mitigation: at unbox time for Value::Number(n), if `n.is_nan()`, canonicalize to `f64::NAN` (which is `0x7FF8_0000_0000_0000`, sign=0). This ensures no Number flows into the JIT with the boxed-NaN bit pattern.

### 2.4 Coexistence with existing Object encoding

Current `unbox_arg_f64(Value::Object(id))` produces `f64::from_bits(id.0 as u64)`. For typical ObjectId.0 values (sequential small integers), this lands at subnormal f64 patterns near zero — distinct from the boxed-NaN range.

VD-EXT 2 migrates Object encoding to the new tagged scheme: `f64::from_bits(MASK | (VD_TAG_OBJECT << 48) | (id.0 & 0x0000_FFFF_FFFF_FFFF))`. The migration is byte-different from the old encoding, BUT all existing JIT-IR consumers that bitcast f64 → I64 and use the result as ObjectId.0 must also migrate to mask off the high 16 bits.

Concrete consumer-update sites (per VD-EXT 1 source-read):
- translator.rs:653 GetPropOnObject lowering: `receiver = builder.ins().bitcast(I64, ..., receiver_f64)` — needs `receiver = receiver & 0x0000_FFFF_FFFF_FFFF` after bitcast.
- All extern call sites that pass `receiver` as i64 (jit_getprop_on_object, jit_getprop_with_ic): same mask required, OR the externs themselves mask on entry.

**Backwards-compatibility option**: keep Object encoding unchanged at first cut. Only add the boxed-NaN tag scheme for STRING (and later variants). Object continues to use the raw `f64::from_bits(id.0 as u64)` scheme. Detection at unbox-decode time:
```rust
fn is_boxed_non_object(f: f64) -> bool {
    f.to_bits() & 0xFFF0_0000_0000_0000 == 0xFFF0_0000_0000_0000
    // Object's raw encoding doesn't have this pattern for small ids
}
```

This option avoids the JIT-IR consumer-update churn at VD-EXT 2 but leaves the latent unsoundness in Object encoding (an ObjectId.0 that happens to look like a boxed-NaN would mis-decode). For the first cut, the **conservative choice** is to migrate Object too, but document the option.

**Pred-vd.4 scope discipline**: pilot's first cut adds String encoding only. The Object-encoding-migration is REQUIRED for full coexistence (per C2/C3 byte-identical preservation under the round-trip C5), so it IS in VD-EXT 2 scope. BigInt / Boolean / Symbol / Null / Undefined defer.

**Pre-implementation correction**: re-reading C3 (`unbox_arg_f64(Value::Object(id)) === f64::from_bits(id.0 as u64) preserved exactly`): this constraint says the existing OBJECT encoding stays. So the design's option "keep Object encoding unchanged at first cut" matches C3. The boxed-NaN scheme covers STRING (and future variants) atop the existing Object encoding. Object's latent unsoundness for large ids is documented but unaddressed at this pilot — out of scope per Pred-vd.4.

**Revised design**: VD-EXT 2 lands:
- String encoding: NEW boxed-NaN pattern with VD_TAG_STRING=2.
- Object encoding: UNCHANGED (preserves C3 byte-identical).
- Number encoding: UNCHANGED + NaN canonicalization (preserves C2 byte-identical; canonicalization is invisible because no realistic Number flow produces sign=1 NaN with our specific mask).
- Decoder logic: first check `is_boxed && tag != 0x0 && tag is_known_boxed_tag → boxed variant`; else if value looks like raw Object encoding (heuristic: subnormal range, small payload) → Object; else Number. This is brittle.

**Simpler revised design**: just gate the decoder by tag:
```rust
fn unbox_arg_f64(v: &Value) -> f64 {
    match v {
        Value::Number(n) => {
            // Canonicalize NaN to avoid collision with boxed-NaN mask.
            if n.is_nan() { f64::NAN } else { *n }
        }
        Value::Object(id) => f64::from_bits(id.0 as u64),  // UNCHANGED per C3
        Value::String(s) => {
            let ptr = Rc::as_ptr(s) as u64;
            f64::from_bits(MASK | (VD_TAG_STRING << 48) | (ptr & 0x0000_FFFF_FFFF_FFFF))
        }
        _ => 0.0,  // BigInt/Boolean/Symbol/Null/Undefined: deferred
    }
}
```

Decode is implicit per consumer-site:
- A consumer expecting Object bitcasts f64→I64 directly and uses the result (matches the old path; works as before).
- A consumer expecting String bitcasts f64→I64, masks off high 16 bits, treats result as `*const String`.
- A consumer that doesn't know the receiver type does a tag check first.

For the first-cut downstream pilot (TL Moves 3+4 revival), the consumer knows the receiver type by parse-time inference (the GetProp pattern recognized String receivers from the bytecode pattern). So the decoder is implicit at the JIT body emission site.

`box_to_value` (the round-trip-back helper, for cases where JIT needs to push a value back to Rust): not needed at first cut. Defer to VD-EXT 3 if a downstream pilot requires it.

## 3. Encoder / decoder reference

### 3.1 Constants

```rust
// New constants in value.rs (or interp.rs near unbox_arg_f64):
const VD_BOXED_MASK: u64 = 0xFFF0_0000_0000_0000;
const VD_TAG_SHIFT: u32 = 48;
const VD_PAYLOAD_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
const VD_TAG_STRING: u64 = 2;
// (BigInt=3, Symbol=4, Boolean=5, Null=6, Undefined=7 — VD-EXT 4+)
```

### 3.2 Encoder (extended unbox_arg_f64)

```rust
pub fn unbox_arg_f64(v: &Value) -> f64 {
    match v {
        Value::Number(n) => {
            // VD-EXT 2: canonicalize NaN to f64::NAN (sign=0). Real
            // arithmetic NaNs don't collide with the boxed-NaN mask
            // (which requires sign=1) by construction, but defensive
            // canonicalization closes the edge case of hardware-
            // produced sign=1 NaNs.
            if n.is_nan() { f64::NAN } else { *n }
        }
        Value::Object(id) => f64::from_bits(id.0 as u64),  // C3 preserved
        Value::String(s) => {
            let ptr = Rc::as_ptr(s) as u64;
            f64::from_bits(
                VD_BOXED_MASK | (VD_TAG_STRING << VD_TAG_SHIFT)
                | (ptr & VD_PAYLOAD_MASK)
            )
        }
        _ => 0.0,  // BigInt/Boolean/Null/Undefined/Symbol: VD-EXT 4+
    }
}
```

### 3.3 Decoder helpers (used by downstream consumer-pilots)

```rust
pub fn is_boxed_value(f: f64) -> bool {
    f.to_bits() & VD_BOXED_MASK == VD_BOXED_MASK && f.to_bits() != f64::NAN.to_bits()
}

pub fn extract_boxed_tag(f: f64) -> u8 {
    ((f.to_bits() >> VD_TAG_SHIFT) & 0xF) as u8
}

pub fn extract_boxed_payload(f: f64) -> u64 {
    f.to_bits() & VD_PAYLOAD_MASK
}

/// SAFETY: caller must ensure f was encoded from a Value::String whose
/// Rc<String> source is still live (typically the caller's frame).
pub unsafe fn decode_string_ptr(f: f64) -> *const String {
    extract_boxed_payload(f) as *const String
}
```

### 3.4 Round-trip identity test

```rust
#[test]
fn roundtrip_string_encoding() {
    let s = Rc::new(String::from("hello, world"));
    let v_in = Value::String(s.clone());
    let encoded = unbox_arg_f64(&v_in);
    assert!(is_boxed_value(encoded));
    assert_eq!(extract_boxed_tag(encoded), VD_TAG_STRING as u8);
    unsafe {
        let ptr = decode_string_ptr(encoded);
        let decoded_s: &String = &*ptr;
        assert_eq!(decoded_s.as_str(), "hello, world");
    }
    // Original Rc still live (held in v_in / s); no leak / no double-free.
}

#[test]
fn number_encoding_unchanged() {
    for n in [0.0, -0.0, 1.0, -1.0, 1e100, -1e100, f64::INFINITY, f64::NEG_INFINITY] {
        let v = Value::Number(n);
        let encoded = unbox_arg_f64(&v);
        assert_eq!(encoded.to_bits(), n.to_bits(), "Number {n} encoding changed");
        assert!(!is_boxed_value(encoded), "Number {n} mis-detected as boxed");
    }
}

#[test]
fn object_encoding_unchanged() {
    for id in [0u64, 1, 42, 1000, 1_000_000] {
        let v = Value::Object(ObjectRef(id));
        let encoded = unbox_arg_f64(&v);
        assert_eq!(encoded.to_bits(), id, "Object id={id} encoding changed");
    }
}
```

## 4. Risks

**R1 — Object encoding shadowing**: if a future ObjectId.0 reaches a value high enough to alias the boxed-NaN mask (~2^52), the decoder would mis-tag it. Mitigation: documented as latent; cruft ObjectIds are bounded well below this in practice (the heap-vec index is u32-sized). Out of scope per Pred-vd.4; addressed at a future VD-EXT round if it ever surfaces.

**R2 — Rc pointer stability**: the encoded f64 holds a raw pointer; if the source Rc is dropped before the JIT body consumes it, the pointer dangles. Mitigation: the source Value lives in the caller's stack frame for the JIT call's duration (same shape as Object's id-encoding which depends on the heap-vec being stable). No GC moves during JIT call.

**R3 — Rc strong count not incremented at encode**: the encoder doesn't `Rc::clone` because it returns f64, not Value. So the pointer is borrowed, not owned. If a downstream pilot needs to `Value::String(Rc::from_raw(ptr))`, that path would over-decrement on Rc drop. Mitigation: don't reconstruct Value::String via Rc::from_raw at decode; instead use `&*ptr` for &String access only. For full Value reconstruction, the caller maintains a parallel mapping or rebuilds from source.

**R4 — Endianness**: NaN-boxing assumes little-endian f64 layout for bit-level patterns. cruft is aarch64-only (engagement reference); aarch64 is little-endian by default. Mitigation: documented; non-aarch64 ports would need verification.

**R5 — Cranelift bitcast semantics**: when the JIT body bitcasts f64 → I64 → bitwise-AND mask, Cranelift must respect the bit pattern exactly. The existing GetPropOnObject lowering already does bitcast F64→I64 successfully. Mitigation: verified via canonical fuzz at VD-EXT 3.

**R6 — Performance cost of NaN canonicalization at unbox**: `n.is_nan()` branch on every Number unbox adds 1-2 cycles per call. Bench Pred-vd.5 measures this; if regression > 5%, switch to assume-no-sign-bit-NaN and document the assumption.

## 5. Forward to VD-EXT 2

VD-EXT 2 implements:
- Add VD_BOXED_MASK, VD_TAG_SHIFT, VD_PAYLOAD_MASK, VD_TAG_STRING constants (interp.rs or value.rs).
- Extend `unbox_arg_f64` per §3.2.
- Add `is_boxed_value`, `extract_boxed_tag`, `extract_boxed_payload`, `decode_string_ptr` helpers per §3.3.
- Add round-trip + Number-preserve + Object-preserve unit tests per §3.4.
- Three-probe gate: canonical fuzz + diff-prod + JIT lib tests + Pred-vd.5 composition bench.

Substrate-introduction round; no downstream consumer at this pilot. The TL pilot revival + hot-intrinsic-IC table generalization consume the encoding in follow-on pilots.

---

*VD-EXT 1 closes. NaN-boxing scheme designed with sign-bit-set distinguishing pattern; mask 0xFFF0_..., 4-bit tag at bits 51-48, 48-bit payload. String encoding via VD_TAG_STRING=2 + Rc<String> raw pointer. Number + Object encodings preserved byte-identical per C2/C3. 6 risks named. VD-EXT 2 implements.*
