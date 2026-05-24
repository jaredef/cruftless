# IHI-EXT 1 — Interp-tier IC table design

*Cross-tier dual of HI-EXT 1. Per-entry shape adapted for interp-tier dispatch (no Cranelift IR; direct Rust fn pointer). Closes Pred-ihi.1's ≤50 LOC/entry budget. CharCode-EXT 2 migration is HI-EXT 2's analog (behavior-neutral; ad-hoc → table).*

## 1. Design constraints (recap from seed §I.2)

```
C1. Existing default-on paths byte-identical post-IHI.
C2. Each entry correctness-preserving on its own.
C3. Entries independent (adding N+1 doesn't change 1..N).
C4. ≤50 LOC per entry.
C5. Rule 11 5-axis pre-spawn check per entry (component A/B + op-set
    + value-domain + locals-marshaling + emission-shape; for interp-
    tier, the last two trivially pass).
C6. Override-safety gate per entry (intrinsic-ObjectId cache).
C7. CharCode-EXT 2 ad-hoc migration is behavior-neutral.
```

## 2. The IcEntry struct + registration shape (interp-tier adaptation)

```rust
/// IHI-EXT 1 (2026-05-24): one interp-tier hot-intrinsic IC entry.
///
/// Lives in pilots/rusty-js-runtime/derived/src/interp_ic_table.rs (new module).
pub struct IhiEntry {
    /// JS property name (e.g., "charCodeAt", "toLowerCase", "trim").
    pub key: &'static str,
    /// Receiver Value variant required by this entry.
    pub receiver: IhiReceiverKind,
    /// Acceptable arity (None = property-access; Some(n) = method-call).
    pub arity: Option<u8>,
    /// Field on Runtime that caches the intrinsic's ObjectId (for
    /// override-safety check). Lazy-populated at first eligible call.
    pub cached_id_field: IhiCachedField,
    /// Fast-path body. Returns Some(value) on success, None to bail.
    pub fast: fn(rt: &Runtime, recv: &Value, args: &[Value]) -> Option<Value>,
}

pub enum IhiReceiverKind {
    String,
    Array,
    Number,
}

/// Discriminator for per-entry cached intrinsic-ObjectId fields. Each
/// entry has its own cache slot to avoid contention. New entries add
/// a variant + a corresponding Option<ObjectId> field on Runtime.
pub enum IhiCachedField {
    StringCharCodeAt,
    StringCodePointAt,
    StringCharAt,
    StringToLowerCase,
    StringTrim,
    StringIndexOf,
    StringSlice,
    // Future entries add here + on Runtime.
}
```

The simplification vs HI's JIT-tier IcEntry:

- **No extern fn**: fast-path runs in Rust directly; no JITBuilder symbol or extern signature.
- **No FuncRef**: no Cranelift IR to declare.
- **No IR lower fn**: the fast-path IS the body.
- **Return type Option<Value>**: None signals bail; Some signals success. Errors (e.g., out-of-bounds → NaN; positive-but-Inf → spec'd value) are spec-correct successes per intrinsic semantics.

Per-entry LOC budget shifts:

| component | HI (JIT-tier) LOC | IHI (interp-tier) LOC |
|---|---:|---:|
| extern fn | 10-15 | (merged into fast) |
| extern_sig | 3-5 | n/a |
| lower IR fn | 15-25 | n/a |
| fast fn (the body) | n/a | 15-25 |
| IhiEntry literal | 5-7 | 5-7 |
| Runtime cache field + init | 0 | 2-3 |
| **per-entry total** | **30-50** | **20-35** |

Interp-tier per-entry LOC is SMALLER than JIT-tier (no IR scaffolding). Comfortably within Pred-ihi.1's ≤50 budget.

## 3. Dispatch integration

### 3.1 Op::CallMethod handler insertion

In `pilots/rusty-js-runtime/derived/src/interp.rs` at the Op::CallMethod handler (line 8007+), the CharCode-EXT 2 ad-hoc block at lines 8232-8289 is REPLACED with a table-driven lookup:

```rust
// IHI-EXT 2 (2026-05-24): table-driven hot-intrinsic IC fast-path.
// Replaces CharCode-EXT 2's ad-hoc charCodeAt path. The IC_TABLE
// lookup is per-receiver-kind + per-method-name + per-arity. On hit,
// the entry's fast fn runs; on Some(v) result, push v and continue.
// On None or no match, fall through to call_function.
if let Some(method_name_str) = method_name.as_deref() {
    if let Some(entry) = crate::interp_ic_table::lookup(
        method_name_str,
        receiver_kind_of(&receiver),
        args.len(),
    ) {
        if let Some(method_id) = match &method {
            Value::Object(id) => Some(*id),
            _ => None,
        } {
            // Override-safety: verify cached intrinsic ObjectId matches.
            // Lazy-populate cache on first eligible call.
            let cached = self.ihi_get_cached(entry.cached_id_field);
            let cached = match cached {
                Some(id) => Some(id),
                None => {
                    if let Some(sp) = self.string_prototype {
                        if let Some(d) = self.obj(sp).get_own(entry.key) {
                            if let Value::Object(id) = d.value {
                                self.ihi_set_cached(entry.cached_id_field, id);
                                Some(id)
                            } else { None }
                        } else { None }
                    } else { None }
                }
            };
            if cached == Some(method_id) {
                if let Some(result) = (entry.fast)(self, &receiver, &args) {
                    frame.push(result);
                    continue;
                }
            }
        }
    }
}
```

The original `call_function` slow path follows unchanged. The IC's failure modes ALL fall through to the slow path (no behavioral change).

### 3.2 Receiver-kind helper

```rust
fn receiver_kind_of(v: &Value) -> IhiReceiverKind {
    match v {
        Value::String(_) => IhiReceiverKind::String,
        Value::Object(_) => IhiReceiverKind::Array,  // first cut conflates Object/Array
        Value::Number(_) => IhiReceiverKind::Number,
        _ => IhiReceiverKind::String,  // bail-equivalent (won't match any entry)
    }
}
```

For first cut: Object/Array conflated (no Array entry in starter set; Array entries deferred). Future refinement when Array entries arrive.

### 3.3 Per-entry cached-ObjectId field on Runtime

Each entry's `cached_id_field` discriminates which Runtime field caches the intrinsic ObjectId. The fields:

```rust
// On Runtime struct (interp.rs around line 200):
pub intrinsic_string_charcodeat_id: Option<ObjectId>,        // existing
pub intrinsic_string_code_point_at_id: Option<ObjectId>,     // new
pub intrinsic_string_char_at_id: Option<ObjectId>,           // new
pub intrinsic_string_to_lower_case_id: Option<ObjectId>,     // new
pub intrinsic_string_trim_id: Option<ObjectId>,              // new
pub intrinsic_string_index_of_id: Option<ObjectId>,          // new
pub intrinsic_string_slice_id: Option<ObjectId>,             // new

// Helper methods on Runtime:
fn ihi_get_cached(&self, field: IhiCachedField) -> Option<ObjectId> { ... }
fn ihi_set_cached(&mut self, field: IhiCachedField, id: ObjectId) { ... }
```

The helpers are match-dispatches; small.

## 4. Migration of CharCode-EXT 2 (IHI-EXT 2 first task)

The existing CharCode-EXT 2 ad-hoc path at interp.rs:8232-8289 is REPLACED by:

1. A `charCodeAt` entry in IC_TABLE
2. The dispatch block at §3.1
3. `intrinsic_string_charcodeat_id` field reused (already exists on Runtime)

Behavior is identical pre/post migration:
- Same method-name check
- Same receiver-kind check (Value::String)
- Same arity check (1)
- Same intrinsic-ObjectId verification (with lazy-populate)
- Same fast-path body (ASCII byte fetch + non-ASCII chars().nth())
- Same bail-to-slow-path conditions (override mismatch; non-Number arg; NaN/negative arg)

The CharCode-EXT 2 ad-hoc block (~58 lines) is removed; the entry (~25 lines including fast fn) + dispatch block (~30 lines, shared infrastructure) net to ~55 lines. Net delta minimal; readability improved (one block; future entries pluggable).

## 5. Per-entry starter-set sketches

### 5.1 charCodeAt (migration anchor)

Already specified in §4.

### 5.2 toLowerCase (highest priority for string_url_sweep)

```rust
fn fast_string_to_lower_case(_rt: &Runtime, recv: &Value, args: &[Value]) -> Option<Value> {
    if !args.is_empty() { return None; }
    if let Value::String(s) = recv {
        // ASCII fast-path: scan for any uppercase byte; if none, return self.
        if s.is_ascii() {
            let bytes = s.as_bytes();
            let mut has_upper = false;
            for &b in bytes {
                if b >= b'A' && b <= b'Z' { has_upper = true; break; }
            }
            if !has_upper {
                // No allocation needed; return the same Rc (clone is cheap).
                return Some(Value::String(s.clone()));
            }
            // ASCII with uppercase: allocate lowered string in O(n).
            let mut out = Vec::with_capacity(bytes.len());
            for &b in bytes {
                out.push(if b >= b'A' && b <= b'Z' { b + 32 } else { b });
            }
            // SAFETY: out is valid UTF-8 (ASCII).
            let s = unsafe { String::from_utf8_unchecked(out) };
            return Some(Value::String(std::rc::Rc::new(s)));
        }
        // Non-ASCII: bail to slow path (chars().flat_map(.to_lowercase()) is
        // complex; existing impl handles it).
        return None;
    }
    None
}
```

Per-entry LOC: ~25 (fast fn) + 5 (cache field + match arms) + 6 (IhiEntry literal) = ~36. Within budget.

### 5.3 trim (high priority for string_url_sweep)

```rust
fn fast_string_trim(_rt: &Runtime, recv: &Value, args: &[Value]) -> Option<Value> {
    if !args.is_empty() { return None; }
    if let Value::String(s) = recv {
        // ECMA whitespace at ASCII: space (0x20), tab (0x09), LF (0x0A),
        // CR (0x0D), VT (0x0B), FF (0x0C), NBSP (0xA0; non-ASCII; bail).
        let bytes = s.as_bytes();
        let is_ws = |b: u8| matches!(b, b' '|b'\t'|b'\n'|b'\r'|0x0B|0x0C);
        // Find first non-WS.
        let mut start = 0;
        while start < bytes.len() && is_ws(bytes[start]) { start += 1; }
        // Find last non-WS.
        let mut end = bytes.len();
        while end > start && is_ws(bytes[end - 1]) { end -= 1; }
        if start == 0 && end == bytes.len() {
            // No trim needed; return self (no allocation).
            return Some(Value::String(s.clone()));
        }
        // Trim needed; allocate substring.
        let trimmed = std::str::from_utf8(&bytes[start..end]).ok()?.to_owned();
        // For non-ASCII strings, the byte-range slice may not respect char
        // boundaries; bail in that case.
        if !s.is_ascii() { return None; }
        Some(Value::String(std::rc::Rc::new(trimmed)))
    } else { None }
}
```

Per-entry LOC: ~30 (fast fn) + 5 (cache) + 6 (literal) = ~41. Within budget.

### 5.4 indexOf (arity 1 form first; arity 2 is HI's already)

```rust
fn fast_string_index_of_1(_rt: &Runtime, recv: &Value, args: &[Value]) -> Option<Value> {
    if args.len() != 1 { return None; }
    if let (Value::String(s), Value::String(needle)) = (recv, &args[0]) {
        // Mirror cruft's interp char-index semantics for non-ASCII.
        if s.is_ascii() && needle.is_ascii() {
            // Byte-search; char-index == byte-index.
            let s_bytes = s.as_bytes();
            let n_bytes = needle.as_bytes();
            if n_bytes.is_empty() { return Some(Value::Number(0.0)); }
            match s_bytes.windows(n_bytes.len()).position(|w| w == n_bytes) {
                Some(p) => Some(Value::Number(p as f64)),
                None => Some(Value::Number(-1.0)),
            }
        } else {
            // Non-ASCII: bail to slow path (existing impl handles).
            None
        }
    } else { None }
}
```

Per-entry LOC: ~20 + 5 + 6 = ~31. Within budget.

### 5.5 slice (deferred — needs Finding HI.2 analog for allocation discipline)

Substring allocation per call; complex. Defer to IHI-EXT N+ after toLowerCase + trim + indexOf land. Finding IHI.1 candidate.

## 6. IHI-EXT 2 staging

1. Create `pilots/rusty-js-runtime/derived/src/interp_ic_table.rs` (~80 LOC: IhiEntry + IhiReceiverKind + IhiCachedField + lookup helper + receiver_kind_of helper).
2. Add cache fields to Runtime (one per starter-set entry; ~10 LOC).
3. Add `ihi_get_cached` + `ihi_set_cached` helper methods on Runtime (~15 LOC).
4. Migrate CharCode-EXT 2's ad-hoc charCodeAt block into the table (1 IhiEntry literal; ~25 LOC for the fast fn — equivalent to the existing inline body); REMOVE the ad-hoc block.
5. Add dispatch integration block at Op::CallMethod (~30 LOC).

Total IHI-EXT 2: ~160 LOC added + ~58 LOC removed = ~100 LOC net. The migration is behavior-neutral; canonical fuzz + diff-prod + CharCode bench all preserved.

## 7. Per-entry round LOC budget verified

| entry | fast fn | cache | literal | total |
|---|---:|---:|---:|---:|
| charCodeAt (migration) | 25 | 0 (existing) | 6 | 31 |
| codePointAt | 15 | 5 | 6 | 26 |
| toLowerCase | 25 | 5 | 6 | 36 |
| trim | 30 | 5 | 6 | 41 |
| indexOf (1-arg) | 20 | 5 | 6 | 31 |
| indexOf (2-arg) | 25 | 0 (shared cache) | 6 | 31 |

All within Pred-ihi.1's ≤50 budget.

## 8. Risks

**R1 — Override-safety cache miss**: first call to a method always pays the slow path (cache empty). For very-low-frequency-method-overridden code, the cache populates incorrectly (cached id = user override's id). Mitigation: populate at Runtime init time (Doc 730 §XIII semantics) instead of lazy. Defer to hardening round.

**R2 — Receiver-kind conflation**: Object/Array conflated in first cut. Array intrinsics (push/pop) would route to the wrong entry if Array becomes a target. Mitigation: refine receiver_kind_of when Array entries arrive.

**R3 — Non-ASCII bail rate**: toLowerCase + trim + indexOf bail on non-ASCII to the slow path. For string_url_sweep (all ASCII headers), 100% fast-path coverage. For other workloads with non-ASCII strings, the IC may bail more often. Mitigation: per-fixture A/B probe at IHI-EXT N+1 to measure.

**R4 — toLowerCase return-self vs allocate-new behavior change**: returning the same Rc<String> for already-lowercase input is a behavior change vs the existing impl (which always allocates). Tests that depend on `s.toLowerCase() !== s` (reference inequality) would break. Mitigation: spec doesn't require new String allocation; legitimate optimization. Verify via canonical fuzz + diff-prod.

## 9. Forward to IHI-EXT 2

IHI-EXT 2 implements: new interp_ic_table.rs module + cache fields + helpers + CharCode-EXT 2 migration + dispatch block. ~100 LOC net delta. Behavior-neutral; canonical fuzz + diff-prod + CharCode bench preserved.

IHI-EXT 3+ adds per-entry rounds (toLowerCase first; trim second; indexOf-1 third; codePointAt + indexOf-2 follow).

IHI-EXT N+1 composition probe + CRB string_url_sweep re-measurement + Pred-ihi.5 gate (≥30% header-loop reclaim).

---

*IHI-EXT 1 closes. Per-entry shape designed; dispatch integration shape designed; CharCode-EXT 2 migration specified; per-entry LOC budget verified at 26-41 (all within Pred-ihi.1's 50 budget). 4 risks named with mitigations. IHI-EXT 2 implements infrastructure + migration.*
