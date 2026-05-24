//! IHI-EXT 2 (2026-05-24): interp-tier hot-intrinsic IC table apparatus.
//!
//! Cross-tier dual of pilots/rusty-js-jit/derived/src/ic_table.rs (HI's
//! JIT-tier IcEntry). The interp-tier IcEntry's fast-path runs in Rust
//! directly (no Cranelift IR); the table is consulted by Op::CallMethod's
//! dispatcher in interp.rs BEFORE call_function.
//!
//! Per IHI-EXT 1 design + the seed's starter-set priority. First entry
//! (charCodeAt) is migrated from CharCode-EXT 2's ad-hoc block at
//! interp.rs:8232-8289 — behavior-neutral; -66% CRB reclaim on
//! json_parse_transform preserved.
//!
//! Future entries (IHI-EXT 3+) add via the same 4-component template
//! (fast fn + cache field + IhiCachedField variant + IhiEntry literal).

use crate::value::Value;

/// IHI-EXT 2 (2026-05-24): one interp-tier hot-intrinsic IC table entry.
///
/// `fast` returns `Some(value)` on successful fast-path; `None` signals
/// bail to slow path (call_function). The dispatcher checks the entry's
/// `cached_id_field` against the resolved method's ObjectId before
/// invoking `fast`; mismatch bails.
pub struct IhiEntry {
    pub key: &'static str,
    pub receiver: IhiReceiverKind,
    /// None = property-access (GetProp); Some(n) = method-call (CallMethod arity n).
    /// First cut handles CallMethod only (PropertyGet via interp's existing fast paths).
    pub arity: Option<u8>,
    pub cached_id_field: IhiCachedField,
    pub fast: fn(recv: &Value, args: &[Value]) -> Option<Value>,
}

// SAFETY: IhiEntry holds fn pointers (fn-item static); Sync per Rust semantics.
unsafe impl Sync for IhiEntry {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IhiReceiverKind {
    String,
    #[allow(dead_code)]
    Array,
    #[allow(dead_code)]
    Number,
}

/// Discriminator for per-entry cached intrinsic-ObjectId fields on
/// Runtime. Each entry has its own cache slot; the dispatcher uses
/// ihi_get_cached + ihi_set_cached helpers to access them.
#[derive(Debug, Clone, Copy)]
pub enum IhiCachedField {
    StringCharCodeAt,
    // IHI-EXT 3+: add variants as entries land
    //   StringCodePointAt,
    //   StringToLowerCase,
    //   StringTrim,
    //   StringIndexOf,
    //   StringSlice,
}

// ─── ENTRY 0: String.prototype.charCodeAt (migration from CharCode-EXT 2) ───
//
// Behavior identical to the ad-hoc block at interp.rs:8232-8289 (the
// CharCode-EXT 2 first-cut). ASCII fast-path mirrors CharCode-EXT 1.

fn fast_string_char_code_at(recv: &Value, args: &[Value]) -> Option<Value> {
    if args.len() != 1 { return None; }
    if let Value::String(s) = recv {
        let pos = &args[0];
        let i_n = match pos {
            Value::Undefined => 0.0,
            Value::Number(n) => *n,
            _ => f64::NAN,
        };
        if i_n.is_finite() && i_n >= 0.0 {
            let i = i_n as usize;
            let bytes = s.as_bytes();
            let result = if s.is_ascii() {
                if i < bytes.len() {
                    Value::Number(bytes[i] as f64)
                } else {
                    Value::Number(f64::NAN)
                }
            } else {
                match s.chars().nth(i) {
                    Some(c) => Value::Number(c as u32 as f64),
                    None => Value::Number(f64::NAN),
                }
            };
            return Some(result);
        }
        // NaN/negative: bail (canonical slow path produces NaN; the
        // edge case is rare; keep ad-hoc's behavior).
        None
    } else { None }
}

// ─── IC_TABLE static registry ───
//
// Future entries register here per IHI-EXT 3+. Each entry: key +
// receiver + arity + cached_id_field + fast fn.

pub static IHI_TABLE: &[IhiEntry] = &[
    IhiEntry {
        key: "charCodeAt",
        receiver: IhiReceiverKind::String,
        arity: Some(1),
        cached_id_field: IhiCachedField::StringCharCodeAt,
        fast: fast_string_char_code_at,
    },
];

/// IHI-EXT 2 (2026-05-24): table lookup by (key, receiver-kind, arity).
/// Returns the matching entry or None.
pub fn lookup(key: &str, receiver: IhiReceiverKind, arity: u8) -> Option<&'static IhiEntry> {
    IHI_TABLE.iter().find(|e| {
        e.key == key
            && e.receiver == receiver
            && e.arity == Some(arity)
    })
}

/// IHI-EXT 2 (2026-05-24): map a receiver Value to its IhiReceiverKind.
/// First-cut conflates Object/Array (no Array entries yet); refine on
/// Array-entry arrival.
pub fn receiver_kind_of(v: &Value) -> IhiReceiverKind {
    match v {
        Value::String(_) => IhiReceiverKind::String,
        Value::Object(_) => IhiReceiverKind::Array,
        Value::Number(_) => IhiReceiverKind::Number,
        // Other variants: pick a kind that won't match any entry (bail-equivalent).
        _ => IhiReceiverKind::Number,
    }
}
