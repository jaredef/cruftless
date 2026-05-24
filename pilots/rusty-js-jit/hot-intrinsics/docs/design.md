# HI-EXT 1 — Hot-intrinsic-IC table design

*Specifies the registration-based apparatus where each entry adds ~30-50 LOC of JIT-side IC fast-path code. Closes Pred-hi.1's LOC-bounded promise. Composes with OSR-EXT 6b's empirical anchor (charCodeAt → -66% CRB).*

## 1. Design constraints (recap from seed §I.2)

```
C1. Existing Σ/Τ/Ψ/Φ/VD/TL/OSR default-on paths byte-identical post-HI.
C2. Each table entry correctness-preserving on its own.
C3. Table entries independent (adding N+1 doesn't change 1..N).
C4. ≤30-50 LOC per entry after infrastructure.
C5. Rule 11 5-axis pre-spawn check per entry.
C6. Override-safety gate per entry.
C7. OSR-EXT 6b's ad-hoc charCodeAt+length entries fold into the table.
```

## 2. The IcEntry struct + registration shape

### 2.1 Entry shape

```rust
/// HI-EXT 1 (2026-05-23): one hot-intrinsic-IC table entry.
pub struct IcEntry {
    /// JS property name (e.g., "length", "charCodeAt", "charAt").
    pub key: &'static str,
    /// Entry kind: property access or method call.
    pub kind: IcEntryKind,
    /// Receiver Value variant required by this entry.
    pub receiver: ReceiverKind,
    /// Extern function name (for JITBuilder::symbol + declare_function).
    pub extern_name: &'static str,
    /// Extern function pointer (for symbol pre-bind).
    pub extern_ptr: *const u8,
    /// Builder for the extern's Cranelift signature.
    pub extern_sig: fn(&mut Signature),
    /// IR lowering fn: emits the inline fast-path body.
    /// Called with (builder, stack, extern_ref). Pops the receiver
    /// (and args, if MethodCall) from stack; pushes the result.
    pub lower: fn(&mut FunctionBuilder, &mut Vec<ClValue>, FuncRef) -> Result<(), String>,
}

pub enum IcEntryKind {
    /// Op::GetProp standalone (no following CallMethod).
    /// e.g., String.length, Array.length.
    PropertyGet,
    /// Op::GetProp + Op::CallMethod(arity) paired.
    /// e.g., String.charCodeAt(i), Array.push(v).
    MethodCall { arity: u8 },
}

pub enum ReceiverKind {
    /// Value::String — receiver is VD-encoded String.
    String,
    /// Value::Object with Array internal kind.
    Array,
    /// Value::Number primitive.
    Number,
}
```

### 2.2 Static registry

```rust
pub static IC_TABLE: &[IcEntry] = &[
    // OSR-EXT 6b migration: charCodeAt entry (proven at -66% CRB).
    IcEntry {
        key: "charCodeAt",
        kind: IcEntryKind::MethodCall { arity: 1 },
        receiver: ReceiverKind::String,
        extern_name: "ic_string_char_code_at",
        extern_ptr: ic_string_char_code_at as *const u8,
        extern_sig: ic_string_char_code_at_sig,
        lower: lower_ic_string_char_code_at,
    },
    // OSR-EXT 6 migration: length entry (proven).
    IcEntry {
        key: "length",
        kind: IcEntryKind::PropertyGet,
        receiver: ReceiverKind::String,
        extern_name: "ic_string_len",
        extern_ptr: ic_string_len as *const u8,
        extern_sig: ic_string_len_sig,
        lower: lower_ic_string_len,
    },
    // HI-EXT 3+: starter-set additions per round.
    // IcEntry { key: "charAt", ... },
    // IcEntry { key: "codePointAt", ... },
];
```

### 2.3 Per-entry LOC breakdown (target: ≤50 per entry post-infrastructure)

For each new entry:
- Extern fn (10-15 LOC): the ASCII fast-path or property read.
- Extern sig builder (3-5 LOC): `sig.params.push(AbiParam::new(I64))` calls.
- IR lowering fn (15-25 LOC): bitcast + mask + extern call + push.
- IcEntry literal in IC_TABLE (5-7 LOC): the struct fields.
- **Total: 30-50 LOC per entry.**

The shared infrastructure (parse-table dispatch, extern pre-bind loop, IR-lower dispatch) is amortized in HI-EXT 2 across all current and future entries.

## 3. Parse-table dispatch

### 3.1 ParsedOp encoding

The existing ParsedOp::GetPropLength + GetPropCharCodeAt + CallMethodCharCodeAt variants are replaced by table-indexed variants:

```rust
enum ParsedOp {
    // ... existing variants ...
    /// HI-EXT (2026-05-23): IC-table-indexed property get.
    IcPropertyGet(u8),  // index into IC_TABLE
    /// HI-EXT: IC-table-indexed method-resolve (paired with IcMethodCall).
    IcMethodResolve(u8),
    /// HI-EXT: IC-table-indexed method-call (paired with IcMethodResolve).
    IcMethodCall(u8),
}
```

### 3.2 Parse-time match

```rust
Op::GetProp => {
    let idx = u16::from_le_bytes([bc[pc], bc[pc+1]]);
    pc += 2;
    let key = match constants.get(idx) {
        Some(Constant::String(s)) => s.as_str(),
        _ => return Err(/* ... */),
    };
    // Lookup in IC_TABLE.
    let entry_idx = IC_TABLE.iter().position(|e| e.key == key);
    match entry_idx {
        Some(i) => {
            match &IC_TABLE[i].kind {
                IcEntryKind::PropertyGet => ParsedOp::IcPropertyGet(i as u8),
                IcEntryKind::MethodCall { .. } => ParsedOp::IcMethodResolve(i as u8),
            }
        }
        None => return Err(format!("GetProp '{key}': no IC table entry")),
    }
}
Op::CallMethod => {
    let n = bc[pc];
    pc += 1;
    // The previous parsed op should be IcMethodResolve(idx) with matching arity.
    let prev_idx = parsed.last().and_then(|(_, op)| {
        if let ParsedOp::IcMethodResolve(i) = op {
            if let IcEntryKind::MethodCall { arity } = IC_TABLE[*i as usize].kind {
                if arity == n {
                    return Some(*i);
                }
            }
        }
        None
    });
    match prev_idx {
        Some(i) => ParsedOp::IcMethodCall(i),
        None => return Err(format!("CallMethod arity={n}: no preceding IcMethodResolve match")),
    }
}
```

### 3.3 Translate-time dispatch

```rust
ParsedOp::IcPropertyGet(idx) | ParsedOp::IcMethodCall(idx) => {
    let entry = &IC_TABLE[*idx as usize];
    let extern_ref = ic_table_refs[*idx as usize]
        .expect("FuncRef must be set when IC entry is in parsed list");
    (entry.lower)(&mut builder, &mut stack, extern_ref)?;
}
ParsedOp::IcMethodResolve(_) => {
    // Pop receiver; push sentinel. The IcMethodCall consumes the receiver
    // separately (still on stack beneath the sentinel via the Dup pattern).
    let _ = stack.pop().ok_or("IcMethodResolve: stack underflow")?;
    let sentinel = builder.ins().f64const(0.0);
    stack.push(sentinel);
}
```

### 3.4 Extern pre-bind + FuncRef setup

```rust
// At JITBuilder setup:
for entry in IC_TABLE {
    if /* entry is used in parsed */ {
        jit_builder.symbol(entry.extern_name, entry.extern_ptr);
    }
}
// At module setup, declare per-entry signatures + FuncIds:
let mut ic_table_ids: Vec<Option<FuncId>> = vec![None; IC_TABLE.len()];
for (i, entry) in IC_TABLE.iter().enumerate() {
    if /* entry is used in parsed */ {
        let mut sig = module.make_signature();
        (entry.extern_sig)(&mut sig);
        ic_table_ids[i] = Some(module
            .declare_function(entry.extern_name, Linkage::Import, &sig)
            .map_err(|e| format!("declare {}: {e}", entry.extern_name))?);
    }
}
// In builder scope, declare FuncRefs:
let ic_table_refs: Vec<Option<FuncRef>> = ic_table_ids.iter()
    .map(|id_opt| id_opt.map(|id| module.declare_func_in_func(id, &mut builder.func)))
    .collect();
```

### 3.5 Per-entry use detection

To avoid declaring all externs when only some are used, scan the parsed list for which IcPropertyGet/IcMethodCall indices appear:

```rust
let mut entry_used = vec![false; IC_TABLE.len()];
for (_, op) in &parsed {
    match op {
        ParsedOp::IcPropertyGet(i) | ParsedOp::IcMethodCall(i) => {
            entry_used[*i as usize] = true;
        }
        _ => {}
    }
}
```

## 4. Override-safety gate per entry

Each entry needs a runtime check that the resolved method (or property getter) IS the intrinsic, not a user override. The gate's shape:

```rust
// In Runtime: per-entry AtomicBool gate.
pub struct IcGateState {
    pub charcodeat_safe: AtomicBool,
    pub length_safe: AtomicBool,
    // ... per starter-set entry ...
}

// When Runtime detects an override (e.g., String.prototype.charCodeAt = ...),
// set the corresponding flag to false. JIT body reads the flag at entry to
// the table-entry IR; on flag=false, deopt to interp.
```

For first cut (HI-EXT 2): skip the gate; trust that user code doesn't override. Document as Finding HI.1 candidate for hardening round.

## 5. Composition with OSR-EXT 6b

OSR-EXT 6b's ad-hoc ParsedOp::GetPropLength + GetPropCharCodeAt + CallMethodCharCodeAt + their IR lowerings are MIGRATED to the table at HI-EXT 2. The migration is:

- Remove the 3 ParsedOp variants
- Remove their parse arms
- Remove their IR translate arms
- Add 2 IcEntry literals to IC_TABLE (charCodeAt + length)
- Move osr_string_len + osr_string_char_code_at to ic_string_len + ic_string_char_code_at (renaming + relocation)

Post-migration: charCodeAt + length entries flow through the table apparatus; the OSR-EXT 6b empirical reclaim (-66% CRB) is preserved (no behavioral change; just code reorganization).

## 6. HI-EXT 2 first-cut staging

The infrastructure round delivers:
1. IcEntry / IcEntryKind / ReceiverKind types in a new helpers/ic_table.rs (or in translator.rs).
2. IC_TABLE static with 2 entries (charCodeAt + length migrated).
3. extern fns ic_string_len + ic_string_char_code_at + per-entry sig builders + per-entry lower fns.
4. Replace 3 ParsedOp variants with IcPropertyGet/IcMethodResolve/IcMethodCall.
5. Parse-arm updates (Op::GetProp + Op::CallMethod consult IC_TABLE).
6. Translate-arm updates (call entry.lower).
7. entry_used scan + per-entry symbol pre-bind + sig declare + FuncRef.

Estimated LOC: ~200 (infrastructure + 2-entry migration). Net delta: ~50 (since OSR-EXT 6/6b's 150 LOC of ad-hoc form is removed).

## 7. HI-EXT 3+ per-entry round shape

Each subsequent round adds 1-2 entries. Per-entry work:
1. Author the extern fn (10-15 LOC).
2. Author extern_sig (3-5 LOC).
3. Author lower fn (15-25 LOC).
4. Add IcEntry literal to IC_TABLE (5-7 LOC).
5. Add per-entry synthetic fixture (one .mjs file).
6. Three-probe gate: canonical fuzz + diff-prod + synthetic fixture.

**Per-entry total LOC: 30-50.** Closes Pred-hi.1.

## 8. Risks

**R1 — Override safety**: first cut skips the gate. If user code overrides String.prototype.charCodeAt, the JIT body still emits the inline fast-path → wrong result. Mitigation: skip for first cut; standing rule 12 adversarial test would catch via diff-prod; codify as Finding HI.1 for hardening round.

**R2 — Receiver-kind mismatch at runtime**: the table entry assumes ReceiverKind::String for charCodeAt; if a Number is passed via overloaded call site, JIT would mis-decode. Mitigation: tag-check guard at the table-entry IR (skip in first cut per OSR-EXT 6b's conservative pattern; trust marshal-in contract).

**R3 — Parse-time lookback brittleness**: IcMethodCall's match requires the PREVIOUS parsed op to be IcMethodResolve with matching arity. If the bytecode shape varies (e.g., LoadLocal between GetProp and CallMethod), the match fails. Mitigation: source-read cruft's compiler to verify bytecode shape always emits `Dup; GetProp; ...args; CallMethod`; verify lookback works.

**R4 — IcEntry function-pointer + raw-ptr lifetime**: extern_ptr is a `*const u8`; valid for process lifetime (fn-item static address). SAFE per standing rule 9. The lower fn is similarly fn-item static.

## 9. Forward to HI-EXT 2

HI-EXT 2 lands the infrastructure + migrates OSR-EXT 6/6b's charCodeAt + length entries into the table. Three-probe gate: canonical fuzz + diff-prod + CRB unchanged (since post-migration behavior is identical to pre-migration). Substrate-introduction round at the apparatus level.

---

*HI-EXT 1 closes. Table apparatus designed: IcEntry struct + static registry + parse-table dispatch + translate-time IR-lower dispatch + extern pre-bind + per-entry use detection + override-safety gate (first-cut deferred). 4 risks named with mitigations. Per-entry LOC budget: 30-50. HI-EXT 2 implements infrastructure + migrates charCodeAt + length.*
