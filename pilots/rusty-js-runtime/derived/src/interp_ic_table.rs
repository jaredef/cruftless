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
    /// IHI-EXT 3 (2026-05-24): String.prototype.toLowerCase.
    StringToLowerCase,
    /// IHI-EXT 4 (2026-05-24): String.prototype.trim.
    StringTrim,
    /// IHI-EXT 5 (2026-05-24): String.prototype.indexOf (arity 1 form;
    /// default fromIndex=0). Heavy in header-normalization loops.
    StringIndexOf,
    /// IHI-EXT 9 (2026-05-24): String.prototype.codePointAt (arity 1).
    StringCodePointAt,
    /// IHI-EXT 9: String.prototype.toUpperCase.
    StringToUpperCase,
    /// IHI-EXT 9: String.prototype.startsWith (arity 1).
    StringStartsWith,
    /// IHI-EXT 9: String.prototype.endsWith (arity 1).
    StringEndsWith,
    /// IHI-EXT 9: String.prototype.includes (arity 1).
    StringIncludes,
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

// ─── ENTRY 1: String.prototype.toLowerCase (MethodCall arity 0) ───
//
// IHI-EXT 3 (2026-05-24): ASCII byte-lower fast-path. Bypasses
// call_function + skips Unicode-aware s.to_lowercase() walk for the
// ASCII case. First cut always allocates (matches cruft's interp
// `string_proto_to_lower_case_via` semantics; preserves reference-
// inequality observable). Future IHI-EXT 3b can add return-self for
// already-lowercase input if measurements justify.

fn fast_string_to_lower_case(recv: &Value, args: &[Value]) -> Option<Value> {
    if !args.is_empty() { return None; }
    if let Value::String(s) = recv {
        if s.is_ascii() {
            let bytes = s.as_bytes();
            let mut out = Vec::with_capacity(bytes.len());
            for &b in bytes {
                out.push(if (b'A'..=b'Z').contains(&b) { b + 32 } else { b });
            }
            // SAFETY: out contains only ASCII (1-byte UTF-8 codepoints).
            let lowered = unsafe { String::from_utf8_unchecked(out) };
            return Some(Value::String(std::rc::Rc::new(lowered)));
        }
        // Non-ASCII: bail (s.to_lowercase() is Unicode-aware; complex; let
        // slow path handle).
        None
    } else { None }
}

// ─── ENTRY 2: String.prototype.trim (MethodCall arity 0) ───
//
// IHI-EXT 4 (2026-05-24): ASCII byte-scan fast-path. ECMA whitespace
// at ASCII: space (0x20), tab (0x09), LF (0x0A), CR (0x0D), VT (0x0B),
// FF (0x0C). NBSP (0xA0) is non-ASCII; bail on those strings.
//
// **Return-self optimization** (legitimate per spec — String is a
// primitive; === is value-equality not pointer-equality): if no trim
// is needed, return the same Rc<String> (cheap clone; no allocation).
// Matches V8/SpiderMonkey/Hermes behavior. Candidate Finding IHI.1 if
// any fixture surfaces dependence on reference inequality.

fn fast_string_trim(recv: &Value, args: &[Value]) -> Option<Value> {
    if !args.is_empty() { return None; }
    if let Value::String(s) = recv {
        let bytes = s.as_bytes();
        let is_ws = |b: u8| matches!(b, b' '|b'\t'|b'\n'|b'\r'|0x0B|0x0C);
        // SPTW-EXT 1 carve-back: bail to slow path on any non-ASCII byte
        // BEFORE checking trim outcome, because non-ASCII strings may
        // contain Unicode whitespace (NBSP U+00A0, BOM U+FEFF, USP set)
        // that this fast path's narrow set doesn't recognize. Pre-fix
        // the early-return "no trim needed" fired on non-ASCII strings
        // whose first/last byte was a non-ASCII leading byte, leaving
        // NBSP/BOM unstripped.
        if !s.is_ascii() {
            return None;
        }
        let mut start = 0;
        while start < bytes.len() && is_ws(bytes[start]) { start += 1; }
        let mut end = bytes.len();
        while end > start && is_ws(bytes[end - 1]) { end -= 1; }
        if start == 0 && end == bytes.len() {
            // No trim needed; return self (no allocation).
            return Some(Value::String(s.clone()));
        }
        let trimmed = unsafe { std::str::from_utf8_unchecked(&bytes[start..end]) }.to_owned();
        Some(Value::String(std::rc::Rc::new(trimmed)))
    } else { None }
}

// ─── ENTRY 3: String.prototype.indexOf (MethodCall arity 1) ───
//
// IHI-EXT 5 (2026-05-24): ASCII byte-search fast-path for the 1-arg
// form `s.indexOf(needle)` (default fromIndex=0). For ASCII strings,
// char-index == byte-index so byte-windows search returns the
// spec-correct index. Bails to slow path on non-ASCII (needle or
// haystack) where char-index ≠ byte-index.

fn fast_string_index_of_1(recv: &Value, args: &[Value]) -> Option<Value> {
    if args.len() != 1 { return None; }
    if let (Value::String(s), Value::String(needle)) = (recv, &args[0]) {
        if s.is_ascii() && needle.is_ascii() {
            let s_bytes = s.as_bytes();
            let n_bytes = needle.as_bytes();
            if n_bytes.is_empty() { return Some(Value::Number(0.0)); }
            if n_bytes.len() > s_bytes.len() { return Some(Value::Number(-1.0)); }
            match s_bytes.windows(n_bytes.len()).position(|w| w == n_bytes) {
                Some(p) => Some(Value::Number(p as f64)),
                None => Some(Value::Number(-1.0)),
            }
        } else {
            // Non-ASCII: bail; existing impl's char-index conversion is needed.
            None
        }
    } else { None }
}

// ─── ENTRIES 4-8 (IHI-EXT 9): String prototype methods batch ───
//
// codePointAt: shape identical to charCodeAt per cruft interp; reuse
// fast_string_char_code_at (char-index semantics; ASCII fast-path).

// toUpperCase: ASCII byte upper-shift; mirror of toLowerCase.
fn fast_string_to_upper_case(recv: &Value, args: &[Value]) -> Option<Value> {
    if !args.is_empty() { return None; }
    if let Value::String(s) = recv {
        if s.is_ascii() {
            let bytes = s.as_bytes();
            let mut out = Vec::with_capacity(bytes.len());
            for &b in bytes {
                out.push(if (b'a'..=b'z').contains(&b) { b - 32 } else { b });
            }
            let upper = unsafe { String::from_utf8_unchecked(out) };
            return Some(Value::String(std::rc::Rc::new(upper)));
        }
        None
    } else { None }
}

// startsWith (1-arg): byte prefix check; ASCII-only fast-path.
fn fast_string_starts_with(recv: &Value, args: &[Value]) -> Option<Value> {
    if args.len() != 1 { return None; }
    if let (Value::String(s), Value::String(prefix)) = (recv, &args[0]) {
        if s.is_ascii() && prefix.is_ascii() {
            let s_bytes = s.as_bytes();
            let p_bytes = prefix.as_bytes();
            if p_bytes.len() > s_bytes.len() { return Some(Value::Boolean(false)); }
            return Some(Value::Boolean(&s_bytes[..p_bytes.len()] == p_bytes));
        }
        None
    } else { None }
}

// endsWith (1-arg): byte suffix check; ASCII-only fast-path.
fn fast_string_ends_with(recv: &Value, args: &[Value]) -> Option<Value> {
    if args.len() != 1 { return None; }
    if let (Value::String(s), Value::String(suffix)) = (recv, &args[0]) {
        if s.is_ascii() && suffix.is_ascii() {
            let s_bytes = s.as_bytes();
            let f_bytes = suffix.as_bytes();
            if f_bytes.len() > s_bytes.len() { return Some(Value::Boolean(false)); }
            let off = s_bytes.len() - f_bytes.len();
            return Some(Value::Boolean(&s_bytes[off..] == f_bytes));
        }
        None
    } else { None }
}

// includes (1-arg): byte substring scan; ASCII-only fast-path.
fn fast_string_includes(recv: &Value, args: &[Value]) -> Option<Value> {
    if args.len() != 1 { return None; }
    if let (Value::String(s), Value::String(needle)) = (recv, &args[0]) {
        if s.is_ascii() && needle.is_ascii() {
            let s_bytes = s.as_bytes();
            let n_bytes = needle.as_bytes();
            if n_bytes.is_empty() { return Some(Value::Boolean(true)); }
            if n_bytes.len() > s_bytes.len() { return Some(Value::Boolean(false)); }
            return Some(Value::Boolean(s_bytes.windows(n_bytes.len()).any(|w| w == n_bytes)));
        }
        None
    } else { None }
}

// ─── IC_TABLE static registry ───
//
// Future entries register here per IHI-EXT 3+. Each entry: key +
// receiver + arity + cached_id_field + fast fn.

/// IHI-EXT 10 (2026-05-24): per-pc IC dispatch cache cell. Outer enum
/// distinguishes "not yet attempted" vs "no match" vs "match at index".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachedDispatch {
    NotCached,
    NoMatch,
    Entry(u8),  // IHI_TABLE index
}

pub static IHI_TABLE: &[IhiEntry] = &[
    IhiEntry {
        key: "charCodeAt",
        receiver: IhiReceiverKind::String,
        arity: Some(1),
        cached_id_field: IhiCachedField::StringCharCodeAt,
        fast: fast_string_char_code_at,
    },
    IhiEntry {
        key: "toLowerCase",
        receiver: IhiReceiverKind::String,
        arity: Some(0),
        cached_id_field: IhiCachedField::StringToLowerCase,
        fast: fast_string_to_lower_case,
    },
    IhiEntry {
        key: "trim",
        receiver: IhiReceiverKind::String,
        arity: Some(0),
        cached_id_field: IhiCachedField::StringTrim,
        fast: fast_string_trim,
    },
    IhiEntry {
        key: "indexOf",
        receiver: IhiReceiverKind::String,
        arity: Some(1),
        cached_id_field: IhiCachedField::StringIndexOf,
        fast: fast_string_index_of_1,
    },
    IhiEntry {
        key: "codePointAt",
        receiver: IhiReceiverKind::String,
        arity: Some(1),
        cached_id_field: IhiCachedField::StringCodePointAt,
        fast: fast_string_char_code_at,  // shape-identical per cruft interp
    },
    IhiEntry {
        key: "toUpperCase",
        receiver: IhiReceiverKind::String,
        arity: Some(0),
        cached_id_field: IhiCachedField::StringToUpperCase,
        fast: fast_string_to_upper_case,
    },
    IhiEntry {
        key: "startsWith",
        receiver: IhiReceiverKind::String,
        arity: Some(1),
        cached_id_field: IhiCachedField::StringStartsWith,
        fast: fast_string_starts_with,
    },
    IhiEntry {
        key: "endsWith",
        receiver: IhiReceiverKind::String,
        arity: Some(1),
        cached_id_field: IhiCachedField::StringEndsWith,
        fast: fast_string_ends_with,
    },
    IhiEntry {
        key: "includes",
        receiver: IhiReceiverKind::String,
        arity: Some(1),
        cached_id_field: IhiCachedField::StringIncludes,
        fast: fast_string_includes,
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
