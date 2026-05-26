//! HI-EXT 2 (2026-05-23): hot-intrinsic-IC table apparatus.
//!
//! Per HI-EXT 1 design + Doc 741 §V.1 generalization. Each IcEntry
//! describes one intrinsic method's JIT-tier IC fast-path: the property
//! key, kind (PropertyGet vs MethodCall), receiver Value variant,
//! extern fn + signature, and IR lowering. The static IC_TABLE is
//! consulted at parse-time (Op::GetProp / Op::CallMethod) and at
//! translate-time (per-entry lower fn).
//!
//! Future entries (HI-EXT 3+) add ~30-50 LOC each per the Pred-hi.1
//! budget: extern fn (10-15) + extern_sig (3-5) + lower fn (15-25) +
//! IcEntry literal (5-7).

use cranelift_codegen::ir::condcodes::IntCC as _; // for IR ops below
use cranelift_codegen::ir::types::{F64, I64};
use cranelift_codegen::ir::{
    AbiParam, FuncRef, InstBuilder, MemFlags, Signature, Value as ClValue,
};
use cranelift_frontend::FunctionBuilder;

/// HI-EXT 2 (2026-05-23): one hot-intrinsic-IC table entry.
pub struct IcEntry {
    pub key: &'static str,
    pub kind: IcEntryKind,
    pub receiver: ReceiverKind,
    pub extern_name: &'static str,
    pub extern_ptr: *const u8,
    pub extern_sig: fn(&mut Signature),
    pub lower: fn(&mut FunctionBuilder, &mut Vec<ClValue>, FuncRef) -> Result<(), String>,
}

// SAFETY: IcEntry holds *const u8 (a fn-item static address) and fn
// pointers (also static). Both are Sync per Rust semantics. The static
// IC_TABLE is read-only at runtime.
unsafe impl Sync for IcEntry {}

#[derive(Debug, Clone, Copy)]
pub enum IcEntryKind {
    /// Op::GetProp standalone (no following CallMethod).
    PropertyGet,
    /// Op::GetProp + Op::CallMethod(arity) paired (lookback at parse).
    MethodCall { arity: u8 },
}

#[derive(Debug, Clone, Copy)]
pub enum ReceiverKind {
    String,
    #[allow(dead_code)]
    Array,
    #[allow(dead_code)]
    Number,
}

// ─── ENTRY 0: String.prototype.length (PropertyGet) ───
//
// Migrated from OSR-EXT 6's ad-hoc GetPropLength path. Behavior-
// neutral: same extern fn (renamed); same IR shape; same -66% CRB
// reclaim on json_parse_transform preserved.

pub extern "C" fn ic_string_len(payload: i64) -> f64 {
    let ptr = payload as *const String;
    let s: &String = unsafe { &*ptr };
    s.len() as f64
}

fn ic_string_len_sig(sig: &mut Signature) {
    sig.params.push(AbiParam::new(I64));
    sig.returns.push(AbiParam::new(F64));
}

fn lower_ic_string_len(
    builder: &mut FunctionBuilder,
    stack: &mut Vec<ClValue>,
    extern_ref: FuncRef,
) -> Result<(), String> {
    let recv_f64 = stack.pop().ok_or("ic_string_len: stack underflow")?;
    let recv_bits = builder.ins().bitcast(I64, MemFlags::new(), recv_f64);
    let payload_mask = builder.ins().iconst(I64, 0x0000_FFFF_FFFF_FFFF_u64 as i64);
    let payload = builder.ins().band(recv_bits, payload_mask);
    let call_inst = builder.ins().call(extern_ref, &[payload]);
    let result = builder.inst_results(call_inst)[0];
    stack.push(result);
    Ok(())
}

// ─── ENTRY 1: String.prototype.charCodeAt (MethodCall arity 1) ───
//
// Migrated from OSR-EXT 6b's ad-hoc CallMethodCharCodeAt path. Same
// extern fn + IR shape preserved.

pub extern "C" fn ic_string_char_code_at(payload: i64, i: i64) -> f64 {
    let ptr = payload as *const String;
    let s: &String = unsafe { &*ptr };
    if i < 0 {
        return f64::NAN;
    }
    let i = i as usize;
    let bytes = s.as_bytes();
    if s.is_ascii() {
        if i < bytes.len() {
            bytes[i] as f64
        } else {
            f64::NAN
        }
    } else {
        match s.chars().nth(i) {
            Some(c) => c as u32 as f64,
            None => f64::NAN,
        }
    }
}

fn ic_string_char_code_at_sig(sig: &mut Signature) {
    sig.params.push(AbiParam::new(I64));
    sig.params.push(AbiParam::new(I64));
    sig.returns.push(AbiParam::new(F64));
}

fn lower_ic_string_char_code_at(
    builder: &mut FunctionBuilder,
    stack: &mut Vec<ClValue>,
    extern_ref: FuncRef,
) -> Result<(), String> {
    let arg_f64 = stack
        .pop()
        .ok_or("ic_string_char_code_at: stack underflow (arg)")?;
    let _sentinel = stack
        .pop()
        .ok_or("ic_string_char_code_at: stack underflow (sentinel)")?;
    let recv_f64 = stack
        .pop()
        .ok_or("ic_string_char_code_at: stack underflow (receiver)")?;
    let recv_bits = builder.ins().bitcast(I64, MemFlags::new(), recv_f64);
    let payload_mask = builder.ins().iconst(I64, 0x0000_FFFF_FFFF_FFFF_u64 as i64);
    let payload = builder.ins().band(recv_bits, payload_mask);
    let arg_i64 = builder.ins().fcvt_to_sint_sat(I64, arg_f64);
    let call_inst = builder.ins().call(extern_ref, &[payload, arg_i64]);
    let result = builder.inst_results(call_inst)[0];
    stack.push(result);
    Ok(())
}

// ─── ENTRY 2: String.prototype.codePointAt (MethodCall arity 1) ───
//
// HI-EXT 3 (2026-05-24): per-entry round. Behavior matches cruft's
// interp `string_proto_code_point_at_via` (which is non-spec for
// non-BMP: uses chars().nth() char index, not UTF-16 code unit index).
// ASCII fast-path mirrors ic_string_char_code_at.

pub extern "C" fn ic_string_code_point_at(payload: i64, i: i64) -> f64 {
    let ptr = payload as *const String;
    let s: &String = unsafe { &*ptr };
    if i < 0 {
        return f64::NAN;
    }
    let i = i as usize;
    let bytes = s.as_bytes();
    if s.is_ascii() {
        if i < bytes.len() {
            bytes[i] as f64
        } else {
            f64::NAN
        }
    } else {
        match s.chars().nth(i) {
            Some(c) => c as u32 as f64,
            None => f64::NAN,
        }
    }
}

fn ic_string_code_point_at_sig(sig: &mut Signature) {
    sig.params.push(AbiParam::new(I64));
    sig.params.push(AbiParam::new(I64));
    sig.returns.push(AbiParam::new(F64));
}

fn lower_ic_string_code_point_at(
    builder: &mut FunctionBuilder,
    stack: &mut Vec<ClValue>,
    extern_ref: FuncRef,
) -> Result<(), String> {
    let arg_f64 = stack
        .pop()
        .ok_or("ic_string_code_point_at: stack underflow (arg)")?;
    let _sentinel = stack
        .pop()
        .ok_or("ic_string_code_point_at: stack underflow (sentinel)")?;
    let recv_f64 = stack
        .pop()
        .ok_or("ic_string_code_point_at: stack underflow (receiver)")?;
    let recv_bits = builder.ins().bitcast(I64, MemFlags::new(), recv_f64);
    let payload_mask = builder.ins().iconst(I64, 0x0000_FFFF_FFFF_FFFF_u64 as i64);
    let payload = builder.ins().band(recv_bits, payload_mask);
    let arg_i64 = builder.ins().fcvt_to_sint_sat(I64, arg_f64);
    let call_inst = builder.ins().call(extern_ref, &[payload, arg_i64]);
    let result = builder.inst_results(call_inst)[0];
    stack.push(result);
    Ok(())
}

// ─── ENTRY 3: String.prototype.indexOf (MethodCall arity 2; first arity-2 entry) ───
//
// HI-EXT 5 (2026-05-24): exercises the apparatus's arity-2 path. The
// 2-arg form `s.indexOf(needle, fromIndex)` is the standard ECMA form.
// Mirrors cruft's interp `string_proto_index_of_via` (interp.rs:4624):
// char-index semantics for clamping the start position; byte-search
// via str::find for ASCII or non-ASCII alike (works correctly since
// substring search is byte-equivalent when both sides UTF-8-encoded).
//
// For ASCII strings (the common case) char-index == byte-index, so the
// clamp is direct; for non-ASCII the start position converts via
// char_indices.

pub extern "C" fn ic_string_index_of(haystack: i64, needle: i64, from: i64) -> f64 {
    let s: &String = unsafe { &*(haystack as *const String) };
    let n: &String = unsafe { &*(needle as *const String) };
    let start_char = if from < 0 { 0 } else { from as usize };
    if s.is_ascii() && n.is_ascii() {
        // Fast-path: byte-search; char-index == byte-index.
        let s_bytes = s.as_bytes();
        let n_bytes = n.as_bytes();
        if start_char > s_bytes.len() {
            return -1.0;
        }
        if n_bytes.is_empty() {
            return start_char as f64;
        }
        if start_char + n_bytes.len() > s_bytes.len() {
            return -1.0;
        }
        match s_bytes[start_char..]
            .windows(n_bytes.len())
            .position(|w| w == n_bytes)
        {
            Some(p) => (start_char + p) as f64,
            None => -1.0,
        }
    } else {
        // Non-ASCII: char-index clamp + str::find + byte→char re-index.
        let char_count = s.chars().count();
        let clamped = start_char.min(char_count);
        let start_byte = s
            .char_indices()
            .nth(clamped)
            .map(|(b, _)| b)
            .unwrap_or(s.len());
        match s[start_byte..].find(n.as_str()) {
            Some(rel_byte) => s[..start_byte + rel_byte].chars().count() as f64,
            None => -1.0,
        }
    }
}

fn ic_string_index_of_sig(sig: &mut Signature) {
    sig.params.push(AbiParam::new(I64)); // haystack payload
    sig.params.push(AbiParam::new(I64)); // needle payload
    sig.params.push(AbiParam::new(I64)); // from index
    sig.returns.push(AbiParam::new(F64));
}

fn lower_ic_string_index_of(
    builder: &mut FunctionBuilder,
    stack: &mut Vec<ClValue>,
    extern_ref: FuncRef,
) -> Result<(), String> {
    // Arity 2: stack from bottom is [receiver, sentinel, needle, from].
    let from_f64 = stack
        .pop()
        .ok_or("ic_string_index_of: stack underflow (from)")?;
    let needle_f64 = stack
        .pop()
        .ok_or("ic_string_index_of: stack underflow (needle)")?;
    let _sentinel = stack
        .pop()
        .ok_or("ic_string_index_of: stack underflow (sentinel)")?;
    let recv_f64 = stack
        .pop()
        .ok_or("ic_string_index_of: stack underflow (receiver)")?;
    let recv_bits = builder.ins().bitcast(I64, MemFlags::new(), recv_f64);
    let needle_bits = builder.ins().bitcast(I64, MemFlags::new(), needle_f64);
    let payload_mask = builder.ins().iconst(I64, 0x0000_FFFF_FFFF_FFFF_u64 as i64);
    let recv_payload = builder.ins().band(recv_bits, payload_mask);
    let needle_payload = builder.ins().band(needle_bits, payload_mask);
    let from_i64 = builder.ins().fcvt_to_sint_sat(I64, from_f64);
    let call_inst = builder
        .ins()
        .call(extern_ref, &[recv_payload, needle_payload, from_i64]);
    let result = builder.inst_results(call_inst)[0];
    stack.push(result);
    Ok(())
}

// ─── IC_TABLE static registry ───
//
// Future entries register here. Each entry: key + kind + receiver +
// extern_name + extern_ptr + extern_sig + lower. Indices are stable
// (don't reorder; downstream parsed-op tags index into this table).

pub static IC_TABLE: &[IcEntry] = &[
    IcEntry {
        key: "length",
        kind: IcEntryKind::PropertyGet,
        receiver: ReceiverKind::String,
        extern_name: "ic_string_len",
        extern_ptr: ic_string_len as *const u8,
        extern_sig: ic_string_len_sig,
        lower: lower_ic_string_len,
    },
    IcEntry {
        key: "charCodeAt",
        kind: IcEntryKind::MethodCall { arity: 1 },
        receiver: ReceiverKind::String,
        extern_name: "ic_string_char_code_at",
        extern_ptr: ic_string_char_code_at as *const u8,
        extern_sig: ic_string_char_code_at_sig,
        lower: lower_ic_string_char_code_at,
    },
    IcEntry {
        key: "codePointAt",
        kind: IcEntryKind::MethodCall { arity: 1 },
        receiver: ReceiverKind::String,
        extern_name: "ic_string_code_point_at",
        extern_ptr: ic_string_code_point_at as *const u8,
        extern_sig: ic_string_code_point_at_sig,
        lower: lower_ic_string_code_point_at,
    },
    IcEntry {
        key: "indexOf",
        kind: IcEntryKind::MethodCall { arity: 2 },
        receiver: ReceiverKind::String,
        extern_name: "ic_string_index_of",
        extern_ptr: ic_string_index_of as *const u8,
        extern_sig: ic_string_index_of_sig,
        lower: lower_ic_string_index_of,
    },
];

/// HI-EXT 2 (2026-05-23): lookup an IC_TABLE entry by key. Returns the
/// index for parsed-op encoding. None if no entry matches.
pub fn lookup_by_key(key: &str) -> Option<u8> {
    IC_TABLE.iter().position(|e| e.key == key).map(|i| i as u8)
}

/// HI-EXT 2 (2026-05-23): IcMethodResolve lowering helper. Used for the
/// GetProp side of a method-call pair: pop receiver, push sentinel.
pub fn lower_ic_method_resolve(
    builder: &mut FunctionBuilder,
    stack: &mut Vec<ClValue>,
) -> Result<(), String> {
    let _ = stack.pop().ok_or("IcMethodResolve: stack underflow")?;
    let sentinel = builder.ins().f64const(0.0);
    stack.push(sentinel);
    Ok(())
}
