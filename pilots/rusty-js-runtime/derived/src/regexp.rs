//! Tier-Ω.5.i — RegExp object, %RegExp.prototype%, the `__createRegExp`
//! lowering helper, the `RegExp` constructor, and regex-aware String
//! prototype extensions (.match / .search / .replace / .replaceAll /
//! .split with a RegExp argument).
//!
//! Pattern translation strategy: JS regex syntax overlaps the Rust `regex`
//! crate's syntax for the common case. Flag handling differs — JS uses a
//! trailing flag string, Rust uses an inline `(?flags)` prefix. We
//! translate `i`, `m`, `s` directly; `g` (global) and `y` (sticky) are
//! consumed at the API level (stateful exec); `u` (unicode) and `d`
//! (indices) are accepted as no-ops; anything else is an error.
//!
//! Compilation failures (lookbehind, backreferences, named groups in
//! incompatible syntax, etc.) are non-fatal: the RegExp object is still
//! constructed with `compiled = None`. Calling .test / .exec / etc. then
//! throws a TypeError. This is a v1 deviation from spec, documented in
//! the round's trajectory row.

use crate::abstract_ops;
use crate::interp::{Runtime, RuntimeError};
use crate::intrinsics::make_native;
use crate::value::{
    CompiledRegex, InternalKind, Object, ObjectRef, PropertyDescriptor, RegExpInternals, Value,
};
use std::collections::HashMap;
use std::rc::Rc;

impl Runtime {
    /// Install %RegExp.prototype%, the `RegExp` constructor, the
    /// `__createRegExp` helper that the compiler lowers literals into,
    /// and the regex-aware String.prototype extensions. Called from
    /// install_intrinsics.
    pub fn install_regexp(&mut self) {
        // Allocate the prototype first so subsequent alloc_object calls
        // for RegExp instances auto-wire through the InternalKind seam.
        let proto = self.alloc_object(Object::new_ordinary());
        self.regexp_prototype = Some(proto);
        install_regexp_proto(self, proto);

        // Ω.5.P55.E1: compiler-emitted lowering. Lives behind the
        // engine-internal bilateral boundary (Doc 729 §VII.B) so
        // `globalThis.__createRegExp` reads as undefined from JS.
        let crx_obj = make_native("__createRegExp", |rt, args| {
            let pattern = abstract_ops::to_string(
                &args.first().cloned().unwrap_or(Value::Undefined)
            ).as_str().to_string();
            let flags = abstract_ops::to_string(
                &args.get(1).cloned().unwrap_or(Value::Undefined)
            ).as_str().to_string();
            Ok(Value::Object(new_regexp(rt, &pattern, &flags)?))
        });
        let crx_id = self.alloc_object(crx_obj);
        self.engine_helpers.insert("__createRegExp".into(), Value::Object(crx_id));

        // RegExp constructor — `new RegExp(p, f)` and `RegExp(p, f)` are
        // both routed through this. If `p` is itself a RegExp the spec
        // says to return a fresh copy; v1 just rebuilds from its source.
        register_global_native(self, "RegExp", |rt, args| {
            // RPTC-EXT 4: ECMA-262 §22.2.4.1 RegExp(pattern, flags) coerces
            // both args via spec ToString (§7.1.17 — dispatches @@toPrimitive
            // / toString / valueOf for Object inputs). Pre-fix used static
            // abstract_ops::to_string which yields "[object Object]" for any
            // Object, breaking `new RegExp({toString(){return 'foo'}})`.
            let first = args.first().cloned().unwrap_or(Value::Undefined);
            let (pattern, flags) = match &first {
                Value::Object(id) => {
                    if let InternalKind::RegExp(re) = &rt.obj(*id).internal_kind {
                        let src = (*re.source).clone();
                        let f = match args.get(1) {
                            Some(Value::Undefined) | None => (*re.flags).clone(),
                            Some(v) => rt.coerce_to_string(v)?,
                        };
                        (src, f)
                    } else {
                        let p = rt.coerce_to_string(&first)?;
                        let f = match args.get(1).cloned().unwrap_or(Value::Undefined) {
                            Value::Undefined | Value::Null => String::new(),
                            v => rt.coerce_to_string(&v)?,
                        };
                        (p, f)
                    }
                }
                Value::Undefined => match args.get(1).cloned().unwrap_or(Value::Undefined) {
                    Value::Undefined | Value::Null => (String::new(), String::new()),
                    v => (String::new(), rt.coerce_to_string(&v)?),
                },
                v => {
                    let p = rt.coerce_to_string(v)?;
                    let f = match args.get(1).cloned().unwrap_or(Value::Undefined) {
                        Value::Undefined | Value::Null => String::new(),
                        v => rt.coerce_to_string(&v)?,
                    };
                    (p, f)
                }
            };
            Ok(Value::Object(new_regexp(rt, &pattern, &flags)?))
        });

        // Tier-Ω.5.wwww: expose RegExp.prototype so libs that capture
        // RegExp.prototype.toString at module init (yup, validator, …)
        // see a real function rather than undefined.
        if let Some(Value::Object(ctor_id)) = self.globals.get("RegExp").cloned() {
            self.obj_mut(ctor_id).set_own_frozen("prototype".into(), Value::Object(proto));
            // Ω.5.P58.E4: RegExp.prototype.constructor = RegExp per ECMA §10.2.12.
            self.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(ctor_id));
        }

        install_string_regex_methods(self);
    }
}

/// Allocate a RegExp instance and populate its accessor own-properties.
pub fn new_regexp(rt: &mut Runtime, pattern: &str, flags: &str) -> Result<ObjectRef, RuntimeError> {
    let compiled = compile_either(pattern, flags);
    let internals = RegExpInternals {
        source: Rc::new(pattern.to_string()),
        flags: Rc::new(flags.to_string()),
        compiled,
        last_index: 0,
    };
    let obj = Object {
        proto: None,
        extensible: true,
        properties: indexmap::IndexMap::new(),
        internal_kind: InternalKind::RegExp(internals),
    
        ..Default::default()
    };
    let id = rt.alloc_object(obj);
    // Plain own-properties for the accessor surface — v1 stand-in for
    // real getter/setter accessor descriptors (deferred).
    rt.object_set(id, "source".into(), Value::String(Rc::new(pattern.to_string())));
    rt.object_set(id, "flags".into(), Value::String(Rc::new(flags.to_string())));
    rt.object_set(id, "global".into(),     Value::Boolean(flags.contains('g')));
    rt.object_set(id, "ignoreCase".into(), Value::Boolean(flags.contains('i')));
    rt.object_set(id, "multiline".into(),  Value::Boolean(flags.contains('m')));
    rt.object_set(id, "sticky".into(),     Value::Boolean(flags.contains('y')));
    rt.object_set(id, "unicode".into(),    Value::Boolean(flags.contains('u')));
    rt.object_set(id, "dotAll".into(),     Value::Boolean(flags.contains('s')));
    rt.object_set(id, "hasIndices".into(), Value::Boolean(flags.contains('d')));
    rt.object_set(id, "lastIndex".into(),  Value::Number(0.0));
    Ok(id)
}

/// Translate `pattern` + JS `flags` into a Rust `regex::Regex`. Returns
/// Err if the pattern uses features the Rust `regex` crate doesn't
/// support (lookbehind, backreferences) or if a flag is unsupported.
/// EXT 76 / 76b: translate JS UTF-16 surrogate-pair alternatives into
/// Rust-acceptable scalar ranges, or elide when translation isn't safe.
///
/// JS regex literals targeting environments without `\p{...}` property
/// classes emulate them with huge alternations like:
///   /[A-Z...]|\uD800[\uDC00-\uDC0B\uDC0D-\uDC26...]|[\uD80C\uD81C-\uD820][\uDC00-\uDFFF]|.../
/// Each `\uD8XX..\uDBXX` is a high surrogate and the bracketed run is the
/// matching low-surrogate ranges. The pair represents a single
/// supplementary-plane code point. The Rust regex crate rejects bare
/// surrogates (Rust `char` is a Unicode scalar; surrogates aren't), so
/// the whole pattern fails to compile.
///
/// 76b translates each (high)(low) alternative into the equivalent
/// disjoint supplementary-plane scalar ranges using `\u{NNNNN}` notation,
/// which the Rust crate accepts directly. For each high surrogate H
/// (singleton or each value in a range) and each low range [La..Lb], the
/// resulting scalar range is:
///   [0x10000 + ((H - 0xD800) << 10) + (La - 0xDC00) ..
///    0x10000 + ((H - 0xD800) << 10) + (Lb - 0xDC00)]
/// Adjacent ranges are merged before emission.
///
/// Alternatives that can't be parsed as a clean `(high)(low)` pair —
/// for example an unpaired high surrogate, or a surrogate-bearing
/// alternative wrapped around a non-class — are dropped (replaced with
/// nothing in the alternation, preserving the structural shape). If
/// every alternative is dropped, the segment becomes `(?!)` (empty
/// negative lookahead) so the structural shape survives.
///
/// Returns Some(cleaned) when the pattern contained surrogate alternatives
/// and was rewritten; None when no change was needed.
fn elide_surrogate_pair_alternatives(pattern: &str) -> Option<String> {
    let bytes = pattern.as_bytes();
    // Detect a `\uD[89AB]XX` (high-surrogate) escape at position p.
    let is_high_surrogate_at = |p: usize| -> bool {
        if p + 6 > bytes.len() || &bytes[p..p+2] != b"\\u" { return false; }
        let hex = &bytes[p+2..p+6];
        if !hex.iter().all(|b| b.is_ascii_hexdigit()) { return false; }
        let val = u32::from_str_radix(std::str::from_utf8(hex).unwrap(), 16).unwrap();
        (0xD800..=0xDBFF).contains(&val)
    };
    // Recursive walk: split the segment between `start..end` into top-level
    // alternatives (depth-0 `|` inside that segment), drop any alternative
    // whose body contains a high-surrogate escape anywhere, and recurse
    // into the bodies of `(?...)` groups that are kept. Returns the
    // cleaned segment text. Outer wrapper preserves `(` and `)` of groups.
    fn clean_segment(bytes: &[u8], start: usize, end: usize,
                     is_high_surrogate_at: &dyn Fn(usize) -> bool,
                     changed: &mut bool) -> String
    {
        // Split into top-level alternatives within [start, end).
        let mut alt_starts: Vec<usize> = vec![start];
        let mut alt_ends: Vec<usize> = Vec::new();
        let mut group_depth: i32 = 0;
        let mut class_depth: i32 = 0;
        let mut i = start;
        while i < end {
            match bytes[i] {
                b'\\' if i + 1 < end => { i += 2; }
                b'(' if class_depth == 0 => { group_depth += 1; i += 1; }
                b')' if class_depth == 0 => { group_depth -= 1; i += 1; }
                b'[' if class_depth == 0 => { class_depth = 1; i += 1; }
                b']' if class_depth > 0 => { class_depth = 0; i += 1; }
                b'|' if group_depth == 0 && class_depth == 0 => {
                    alt_ends.push(i);
                    alt_starts.push(i + 1);
                    i += 1;
                }
                _ => { i += 1; }
            }
        }
        alt_ends.push(end);

        let mut kept: Vec<String> = Vec::new();
        for (&s, &e) in alt_starts.iter().zip(alt_ends.iter()) {
            // Scan at this alt's top level only: outside any `(...)`
            // group (groups recurse). High surrogates appearing at the
            // top level of the alt — bare `\uHHHH` or inside a top-level
            // `[...]` class — disqualify the alt entirely. Surrogates
            // nested inside `(?:...)` groups are left for the recursive
            // pass to handle when it cleans the inner segment.
            let mut has_surrogate = false;
            let mut k = s;
            let mut scan_group_depth: i32 = 0;
            while k < e {
                match bytes[k] {
                    b'\\' if k + 1 < e => {
                        if scan_group_depth == 0 && is_high_surrogate_at(k) {
                            has_surrogate = true; break;
                        }
                        k += 2;
                    }
                    b'(' => { scan_group_depth += 1; k += 1; }
                    b')' => { scan_group_depth -= 1; k += 1; }
                    _ => { k += 1; }
                }
            }
            if has_surrogate {
                *changed = true;
                if let Some(translated) = translate_surrogate_alt(bytes, s, e) {
                    kept.push(translated);
                }
                continue;
            }
            // Recurse into `(?...)` groups inside this alternative so
            // that nested alternations with surrogate-bearing branches
            // also get cleaned.
            let mut rebuilt = String::with_capacity(e - s);
            let mut p = s;
            let mut cd = 0i32;
            while p < e {
                match bytes[p] {
                    b'\\' if p + 1 < e => {
                        rebuilt.push(bytes[p] as char);
                        rebuilt.push(bytes[p+1] as char);
                        p += 2;
                    }
                    b'[' if cd == 0 => { cd = 1; rebuilt.push('['); p += 1; }
                    b']' if cd > 0 => { cd = 0; rebuilt.push(']'); p += 1; }
                    b'(' if cd == 0 => {
                        // Find matching `)` at depth 0.
                        let group_start = p;
                        let mut d = 1i32;
                        let mut q = p + 1;
                        // Copy `(?:` / `(?=` / `(?!` / `(?<...>` prefix verbatim.
                        let mut inner_start = p + 1;
                        if q < e && bytes[q] == b'?' {
                            // Capture the prefix up to and including the marker.
                            q += 1;
                            while q < e && bytes[q] != b':' && bytes[q] != b'=' && bytes[q] != b'!' && bytes[q] != b'<' && bytes[q] != b'>' {
                                q += 1;
                            }
                            if q < e {
                                if bytes[q] == b'<' {
                                    while q < e && bytes[q] != b'>' { q += 1; }
                                }
                                q += 1;
                                inner_start = q;
                            }
                        }
                        // Now scan q..end balancing `(` `)` to find the close.
                        let mut cd2 = 0i32;
                        let mut close = q;
                        while close < e && d > 0 {
                            match bytes[close] {
                                b'\\' if close + 1 < e => { close += 2; }
                                b'[' if cd2 == 0 => { cd2 = 1; close += 1; }
                                b']' if cd2 > 0 => { cd2 = 0; close += 1; }
                                b'(' if cd2 == 0 => { d += 1; close += 1; }
                                b')' if cd2 == 0 => { d -= 1; if d == 0 { break; } close += 1; }
                                _ => { close += 1; }
                            }
                        }
                        if d == 0 && close < e {
                            // Copy `(?...:` prefix verbatim.
                            for b in &bytes[group_start..inner_start] {
                                rebuilt.push(*b as char);
                            }
                            // Recurse into the inner body.
                            let inner = clean_segment(bytes, inner_start, close, is_high_surrogate_at, changed);
                            rebuilt.push_str(&inner);
                            rebuilt.push(')');
                            p = close + 1;
                        } else {
                            // Unbalanced; bail by copying the rest verbatim.
                            rebuilt.push_str(std::str::from_utf8(&bytes[p..e]).unwrap());
                            p = e;
                        }
                    }
                    _ => { rebuilt.push(bytes[p] as char); p += 1; }
                }
            }
            kept.push(rebuilt);
        }
        if kept.is_empty() { "(?!)".to_string() } else { kept.join("|") }
    }

    let mut changed = false;
    let cleaned = clean_segment(bytes, 0, bytes.len(), &is_high_surrogate_at, &mut changed);
    if changed { Some(cleaned) } else { None }
}

/// EXT 76b: parse a `\uHHHH` escape at position p. Returns (codepoint, end_pos).
fn parse_unicode_esc(bytes: &[u8], p: usize) -> Option<(u32, usize)> {
    if p + 6 > bytes.len() || &bytes[p..p+2] != b"\\u" { return None; }
    let hex = std::str::from_utf8(&bytes[p+2..p+6]).ok()?;
    let v = u32::from_str_radix(hex, 16).ok()?;
    Some((v, p + 6))
}

/// EXT 76b: parse a character class `[...]` whose body is exclusively
/// `\uHHHH` chars and `\uHHHH-\uHHHH` ranges. Returns the inclusive
/// ranges as (lo, hi) pairs, or None if the class contains anything else
/// (ASCII chars, named classes, escapes other than \u, ...). Caller must
/// ensure bytes[start] == b'['.
fn parse_uesc_class(bytes: &[u8], start: usize) -> Option<(Vec<(u32, u32)>, usize)> {
    if start >= bytes.len() || bytes[start] != b'[' { return None; }
    let mut p = start + 1;
    if p < bytes.len() && bytes[p] == b'^' { return None; }
    let mut ranges = Vec::new();
    while p < bytes.len() && bytes[p] != b']' {
        let (lo, q) = parse_unicode_esc(bytes, p)?;
        p = q;
        let hi = if p < bytes.len() && bytes[p] == b'-'
            && p + 1 < bytes.len() && bytes[p+1] != b']'
        {
            let (h, q2) = parse_unicode_esc(bytes, p + 1)?;
            p = q2;
            h
        } else { lo };
        ranges.push((lo, hi));
    }
    if p >= bytes.len() { return None; }
    Some((ranges, p + 1))
}

/// EXT 76b: emit a character class containing the given scalar ranges,
/// each as `\u{HEX}` or `\u{HEX}-\u{HEX}`. Adjacent ranges are merged
/// before emission so the output is canonical.
fn emit_scalar_class(mut ranges: Vec<(u32, u32)>) -> String {
    if ranges.is_empty() { return "(?!)".to_string(); }
    ranges.sort();
    let mut merged: Vec<(u32, u32)> = Vec::new();
    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r.0 <= last.1.saturating_add(1) {
                last.1 = last.1.max(r.1);
                continue;
            }
        }
        merged.push(r);
    }
    let mut s = String::from("[");
    for (a, b) in merged {
        if a == b {
            s.push_str(&format!("\\u{{{:X}}}", a));
        } else {
            s.push_str(&format!("\\u{{{:X}}}-\\u{{{:X}}}", a, b));
        }
    }
    s.push(']');
    s
}

/// EXT 76b: translate one (high)(low) alternative into scalar ranges.
/// Recognized shapes:
///   `\uHHHH[\uLLLL...]`      bare high surrogate + low-surrogate class
///   `[\uHHHH...][\uLLLL...]` high-surrogate class + low-surrogate class
/// Returns None when the alternative is not a clean pair (e.g. an
/// unpaired high surrogate, or a pair surrounded by extra atoms the
/// translator doesn't model). Caller drops the alternative in that case.
fn translate_surrogate_alt(bytes: &[u8], start: usize, end: usize) -> Option<String> {
    let mut p = start;
    // Parse the high component: either a single \uHHHH or a class.
    let high_ranges: Vec<(u32, u32)>;
    if p + 6 <= end && &bytes[p..p+2] == b"\\u" {
        let (v, q) = parse_unicode_esc(bytes, p)?;
        high_ranges = vec![(v, v)];
        p = q;
    } else if p < end && bytes[p] == b'[' {
        let (rs, q) = parse_uesc_class(bytes, p)?;
        if q > end { return None; }
        high_ranges = rs;
        p = q;
    } else {
        return None;
    }
    // Validate the high component is exclusively high surrogates.
    for &(a, b) in &high_ranges {
        if !(0xD800..=0xDBFF).contains(&a) || !(0xD800..=0xDBFF).contains(&b) {
            return None;
        }
    }
    // Parse the low component: must be a class of low surrogates.
    if p >= end || bytes[p] != b'[' { return None; }
    let (low_ranges, q) = parse_uesc_class(bytes, p)?;
    if q != end { return None; } // anything trailing → not a clean shape.
    for &(a, b) in &low_ranges {
        if !(0xDC00..=0xDFFF).contains(&a) || !(0xDC00..=0xDFFF).contains(&b) {
            return None;
        }
    }
    // Compute scalar ranges. For each high surrogate H in [Ha..Hb] and
    // each low range [La..Lb], emit [base+La-0xDC00 .. base+Lb-0xDC00]
    // where base = 0x10000 + ((H - 0xD800) << 10).
    let mut scalars: Vec<(u32, u32)> = Vec::new();
    for &(ha, hb) in &high_ranges {
        for h in ha..=hb {
            let base = 0x10000u32 + ((h - 0xD800) << 10);
            for &(la, lb) in &low_ranges {
                scalars.push((base + (la - 0xDC00), base + (lb - 0xDC00)));
            }
        }
    }
    Some(emit_scalar_class(scalars))
}

fn translate(pattern: &str, flags: &str) -> Result<regex::Regex, String> {
    let mut flag_set = String::new();
    for c in flags.chars() {
        match c {
            'i' => flag_set.push('i'),
            'm' => flag_set.push('m'),
            's' => flag_set.push('s'),
            // g (global) and y (sticky) are handled at the API level —
            // they govern stateful exec/replace, not the regex compile.
            // u (unicode) is the default for the Rust crate; d (indices)
            // is a no-op in v1.
            'g' | 'y' | 'u' | 'd' => {}
            _ => return Err(format!("unsupported regex flag '{}'", c)),
        }
    }
    let cleaned = elide_surrogate_pair_alternatives(pattern);
    let body = cleaned.as_deref().unwrap_or(pattern);
    let prefixed = if flag_set.is_empty() {
        body.to_string()
    } else {
        format!("(?{}){}", flag_set, body)
    };
    regex::Regex::new(&prefixed).map_err(|e| format!("{}", e))
}

/// Tier-Ω.5.ggg: dual-engine compile. Try the Rust `regex` crate first
/// (fast for the patterns it supports). On rejection, fall back to the
/// hand-rolled backtracking engine which supports lookaround.
pub fn compile_either(pattern: &str, flags: &str) -> Option<CompiledRegex> {
    if let Ok(r) = translate(pattern, flags) {
        return Some(CompiledRegex::Rust(r));
    }
    if let Ok(h) = crate::regex_hand::compile(pattern, flags) {
        return Some(CompiledRegex::Hand(h));
    }
    None
}

// ──────────────── %RegExp.prototype% ────────────────

fn install_regexp_proto(rt: &mut Runtime, host: ObjectRef) {
    // §22.2.6 accessors on RegExp.prototype. Each is a brand-checked
    // getter that reads the corresponding per-instance data property.
    for name in &["source", "flags", "global", "ignoreCase", "multiline",
                  "sticky", "unicode", "dotAll", "hasIndices"] {
        install_regexp_proto_accessor(rt, host, name);
    }

    // Ω.5.P61.E12: RegExp.prototype.compile per ECMA Annex B.2.4
    // (legacy). Re-binds the receiver's source + flags to a new pattern.
    // Returns the receiver. v1 noop after re-parse — sufficient for
    // module-init presence-probes that check `typeof rx.compile`.
    register_method(rt, host, "compile", |rt, _args| {
        Ok(rt.current_this())
    });
    // RPTC-EXT 1: ECMA-262 §22.2.5.5 — RegExp.prototype.test( S )
    // 1. R = this; 2. S = ToString(S); 3. match = RegExpExec(R, S);
    // 4. return match !== null.
    // Length is 1 per §22.2.5.5. Coerce via rt.coerce_to_string (which
    // dispatches @@toPrimitive / toString / valueOf per §7.1.17).
    // Route through regexp_exec so sticky/global lastIndex bookkeeping
    // matches the .exec path exactly.
    crate::intrinsics::register_intrinsic_method(rt, host, "test", 1, |rt, args| {
        let this_id = current_regexp_this(rt, "RegExp.prototype.test")?;
        let arg = args.first().cloned().unwrap_or(Value::Undefined);
        let input = rt.coerce_to_string(&arg)?;
        let r = regexp_exec(rt, this_id, &input)?;
        Ok(Value::Boolean(!matches!(r, Value::Null)))
    });

    crate::intrinsics::register_intrinsic_method(rt, host, "exec", 1, |rt, args| {
        let this_id = current_regexp_this(rt, "RegExp.prototype.exec")?;
        // Ω.5.P60.E4: full ECMA §7.1.17 ToString on the argument — for
        // Object inputs this dispatches @@toPrimitive('string') first, then
        // toString(), then valueOf() per OrdinaryToPrimitive. is-regex
        // detects RegExps by passing a badStringifier-Object whose
        // @@toPrimitive / toString / valueOf throw a marker; pre-P60.E4
        // cruftless's static to_string returned "[object Object]" for
        // Objects without dispatch and is-regex returned undefined → 27-
        // package regression visible post-P59.E1 once Symbol.toPrimitive
        // became a real Value::Symbol and the polyfill entered the
        // toPrimitive-trap branch.
        let arg = args.first().cloned().unwrap_or(Value::Undefined);
        let input = rt.coerce_to_string(&arg)?;
        regexp_exec(rt, this_id, &input)
    });

    register_method(rt, host, "toString", |rt, _args| {
        let this_id = current_regexp_this(rt, "RegExp.prototype.toString")?;
        let s = match &rt.obj(this_id).internal_kind {
            InternalKind::RegExp(r) => format!("/{}/{}", r.source, r.flags),
            _ => unreachable!(),
        };
        Ok(Value::String(Rc::new(s)))
    });

    // Ω.5.P62.E25: RegExp.prototype @@match / @@search / @@replace / @@split
    // per ECMA §22.2.5. String.prototype.{match,search,replace,split}
    // dispatch to these when the searchValue is a RegExp (via the
    // IsRegExp/@@match probe). Pre-E25 cruftless didn't install these
    // Symbol-named methods, so `"abc".match(/a/)` resolved through the
    // String.prototype impl directly — which worked for simple cases
    // but broke any consumer that used `regex[Symbol.match](str)` form.
    register_method(rt, host, "@@match", |rt, args| {
        let this_id = current_regexp_this(rt, "RegExp.prototype[@@match]")?;
        let s = rt.to_string_strict(&args.first().cloned().unwrap_or(Value::Undefined))?;
        let is_global = matches!(&rt.obj(this_id).internal_kind,
            InternalKind::RegExp(r) if r.flags.contains('g'));
        if !is_global {
            return regexp_exec(rt, this_id, &s);
        }
        let rx = match &rt.obj(this_id).internal_kind {
            InternalKind::RegExp(r) => r.compiled.clone(),
            _ => None,
        };
        let rx = match rx {
            Some(r) => r,
            None => return Ok(Value::Null),
        };
        let ms: Vec<String> = rx.find_iter_owned(&s).into_iter().map(|(_,_,s)| s).collect();
        if ms.is_empty() { return Ok(Value::Null); }
        let arr = rt.alloc_object(Object::new_array());
        for (i, m) in ms.iter().enumerate() {
            rt.object_set(arr, i.to_string(), Value::String(Rc::new(m.clone())));
        }
        rt.object_set(arr, "length".into(), Value::Number(ms.len() as f64));
        Ok(Value::Object(arr))
    });
    register_method(rt, host, "@@search", |rt, args| {
        let this_id = current_regexp_this(rt, "RegExp.prototype[@@search]")?;
        let s = rt.to_string_strict(&args.first().cloned().unwrap_or(Value::Undefined))?;
        let rx = match &rt.obj(this_id).internal_kind {
            InternalKind::RegExp(r) => r.compiled.clone(),
            _ => None,
        };
        let rx = match rx { Some(r) => r, None => return Ok(Value::Number(-1.0)) };
        match rx.find_first(&s) {
            Some((start, _)) => Ok(Value::Number(byte_to_char_index(&s, start) as f64)),
            None => Ok(Value::Number(-1.0)),
        }
    });
    register_method(rt, host, "@@replace", |rt, args| {
        let _ = current_regexp_this(rt, "RegExp.prototype[@@replace]")?;
        let s = rt.to_string_strict(&args.first().cloned().unwrap_or(Value::Undefined))?;
        let repl = args.get(1).cloned().unwrap_or(Value::Undefined);
        // Reuse the existing string_replace_impl; pass `this` (the regex)
        // as the pattern arg.
        string_replace_impl(rt, &s, rt.current_this(), repl, false)
    });
    register_method(rt, host, "@@split", |rt, args| {
        let this_id = current_regexp_this(rt, "RegExp.prototype[@@split]")?;
        let s = rt.to_string_strict(&args.first().cloned().unwrap_or(Value::Undefined))?;
        let limit = args.get(1).cloned().and_then(|v| {
            if matches!(v, Value::Undefined) { None }
            else { rt.coerce_to_number(&v).ok().filter(|n| n.is_finite() && *n >= 0.0).map(|n| n as usize) }
        });
        let rx = match &rt.obj(this_id).internal_kind {
            InternalKind::RegExp(r) => r.compiled.clone(),
            _ => None,
        };
        let rx = match rx {
            Some(r) => r,
            None => {
                let arr = rt.alloc_object(Object::new_array());
                rt.object_set(arr, "0".into(), Value::String(Rc::new(s.clone())));
                rt.object_set(arr, "length".into(), Value::Number(1.0));
                return Ok(Value::Object(arr));
            }
        };
        let parts: Vec<String> = rx.split_str(&s);
        let arr = rt.alloc_object(Object::new_array());
        let take = limit.unwrap_or(parts.len()).min(parts.len());
        for (i, p) in parts.iter().take(take).enumerate() {
            rt.object_set(arr, i.to_string(), Value::String(Rc::new(p.clone())));
        }
        rt.object_set(arr, "length".into(), Value::Number(take as f64));
        Ok(Value::Object(arr))
    });
}

/// Per §22.2.5.2 RegExpBuiltinExec. v1 surface: returns null on no match,
/// else an Array with [match, ...groups] plus .index / .input properties.
/// Honors the 'g' flag via lastIndex.
/// RPTC-EXT 3: ECMA-262 §7.3.4 Set(R, "lastIndex", v, true) with the
/// Throw flag. Mirrors the OrdinarySet path: if the own "lastIndex"
/// data property is non-writable, throw TypeError. Used wherever
/// spec says `Set(R, "lastIndex", ..., true)`.
fn set_last_index_strict(rt: &mut Runtime, id: ObjectRef, n: f64) -> Result<(), RuntimeError> {
    let writable = rt.obj(id).get_own("lastIndex").map(|d| d.writable).unwrap_or(true);
    if !writable {
        return Err(RuntimeError::TypeError(
            "Cannot assign to read only property 'lastIndex' of object '#<RegExp>'".into()));
    }
    rt.object_set(id, "lastIndex".into(), Value::Number(n));
    Ok(())
}

/// RPTC-EXT 3: byte-offset → UTF-16 code-unit position. JS strings are
/// UTF-16; lastIndex and the .index property on match results are both
/// in code-unit positions per §22.2.7.2 / §22.2.7.3. The engine returns
/// byte positions into the UTF-8 backing; the substrate must filter.
/// (Pre-fix, both were treated as char-counts for index and bytes for
/// lastIndex — wrong for any non-BMP input.)
fn byte_to_utf16(s: &str, byte_off: usize) -> usize {
    let off = byte_off.min(s.len());
    s[..off].chars().map(|c| if (c as u32) >= 0x10000 { 2 } else { 1 }).sum()
}

pub fn regexp_exec(rt: &mut Runtime, this_id: ObjectRef, input: &str) -> Result<Value, RuntimeError> {
    let (is_global, is_sticky, has_indices, has_compiled) = {
        let o = rt.obj(this_id);
        let re = match &o.internal_kind {
            InternalKind::RegExp(r) => r,
            _ => return Err(RuntimeError::TypeError("RegExp.prototype.exec: this is not a RegExp".into())),
        };
        let is_sticky = re.flags.contains('y');
        // Both `g` and `y` honor lastIndex bookkeeping; only `y` anchors.
        let is_global = re.flags.contains('g') || is_sticky;
        // RES-EXT 2: `d` flag enables .indices on the result (§22.2.7.2 step 31).
        let has_indices = re.flags.contains('d');
        (is_global, is_sticky, has_indices, re.compiled.is_some())
    };
    // ECMA-262 §22.2.7.2 RegExpBuiltinExec step 4: read lastIndex from
    // the JS property (user-settable) and ToLength-coerce. The coercion
    // is observable (the property's valueOf is called) and happens even
    // for non-global / non-sticky regexes — spec step 8 then resets the
    // working start to 0 for the non-(global|sticky) case AFTER the
    // ToLength side effects have fired.
    let last_index_v = rt.object_get(this_id, "lastIndex");
    let last_index_n = {
        let n = rt.coerce_to_number(&last_index_v)?;
        if n.is_nan() || n <= 0.0 { 0.0 } else if !n.is_finite() { 9007199254740991.0 } else { n.floor() }
    };
    let start: usize = if is_global { last_index_n as usize } else { 0 };
    if !has_compiled {
        let (src, flags) = match &rt.obj(this_id).internal_kind {
            InternalKind::RegExp(r) => ((*r.source).clone(), (*r.flags).clone()),
            _ => unreachable!(),
        };
        return Err(RuntimeError::TypeError(format!(
            "RegExp pattern uses features unsupported by the v1 regex engine: /{}/{}",
            src, flags)));
    }
    if start > input.len() {
        if is_global {
            if let InternalKind::RegExp(r) = &mut rt.obj_mut(this_id).internal_kind {
                r.last_index = 0;
            }
            set_last_index_strict(rt, this_id, 0.0)?;
        }
        return Ok(Value::Null);
    }

    // Snapshot of the captures we need. We borrow the regex immutably,
    // collect everything into owned strings, then release.
    let captures_opt: Option<(usize, usize, Vec<Option<String>>)> = {
        let re = match &rt.obj(this_id).internal_kind {
            InternalKind::RegExp(r) => r,
            _ => unreachable!(),
        };
        let rx = re.compiled.as_ref().unwrap();
        rx.captures_at(input, start)
    };

    // RPTC-EXT 2: ECMA-262 §22.2.7.2 step 23.a — sticky flag anchors the
    // match at lastIndex. If the engine's scanning search returns a match
    // that doesn't start at `start`, treat as failure under sticky.
    let captures_opt = match captures_opt {
        Some((ms, _, _)) if is_sticky && ms != start => None,
        other => other,
    };
    match captures_opt {
        None => {
            if is_global {
                if let InternalKind::RegExp(r) = &mut rt.obj_mut(this_id).internal_kind {
                    r.last_index = 0;
                }
                set_last_index_strict(rt, this_id, 0.0)?;
            }
            Ok(Value::Null)
        }
        Some((mstart, mend, groups)) => {
            // RPTC-EXT 3: lastIndex + .index are UTF-16 code-unit positions
            // per §22.2.7.2 step 17/18 and §22.2.7.3 step 15. The engine
            // returns byte offsets into UTF-8; convert at the substrate
            // boundary. Pre-fix produced byte-offset lastIndex and char-
            // count .index — both wrong for non-BMP input.
            let mend_u16 = byte_to_utf16(input, mend);
            let mstart_u16 = byte_to_utf16(input, mstart);
            if is_global {
                if let InternalKind::RegExp(r) = &mut rt.obj_mut(this_id).internal_kind {
                    r.last_index = mend_u16;
                }
                set_last_index_strict(rt, this_id, mend_u16 as f64)?;
            }
            // RES-EXT 1: collect named groups before the result array
            // is built; the borrow on internal_kind ends here.
            let named = match &rt.obj(this_id).internal_kind {
                InternalKind::RegExp(r) => r.compiled.as_ref().map(|c| c.named_groups()),
                _ => None,
            };
            let arr = rt.alloc_object(Object::new_array());
            for (i, g) in groups.iter().enumerate() {
                let v = match g {
                    Some(s) => Value::String(Rc::new(s.clone())),
                    None => Value::Undefined,
                };
                rt.object_set(arr, i.to_string(), v);
            }
            rt.object_set(arr, "length".into(), Value::Number(groups.len() as f64));
            rt.object_set(arr, "index".into(), Value::Number(mstart_u16 as f64));
            rt.object_set(arr, "input".into(), Value::String(Rc::new(input.to_string())));
            // RES-EXT 1: build .groups per ECMA-262 §22.2.7.2 step 33.
            // Spec mandates a null-prototype Object whose own properties
            // are name → matched-substring (or undefined for non-
            // participating named groups). If the pattern has no named
            // groups, .groups is undefined.
            let named_list = named.unwrap_or_default();
            if !named_list.is_empty() {
                let g_obj = rt.alloc_object_with_explicit_null_proto(Object::new_ordinary());
                for (name, idx) in &named_list {
                    let v = groups.get(*idx).and_then(|g| g.clone())
                        .map(|s| Value::String(Rc::new(s)))
                        .unwrap_or(Value::Undefined);
                    rt.object_set(g_obj, name.clone(), v);
                }
                rt.object_set(arr, "groups".into(), Value::Object(g_obj));
            } else {
                rt.object_set(arr, "groups".into(), Value::Undefined);
            }
            // RES-EXT 2: build .indices when /d flag is set per
            // ECMA-262 §22.2.7.7 MakeMatchIndicesArray. Each element is
            // [start, end] in UTF-16 code units (or undefined for non-
            // participating groups). Mirrors .groups by attaching a
            // null-prototype .groups Object with name → [start,end] when
            // there are named groups.
            if has_indices {
                // Re-acquire positions from the engine (we already had them
                // implicitly via captures_at strings; capturing positions
                // separately avoids changing the surrounding flow).
                let positions = {
                    let re = match &rt.obj(this_id).internal_kind {
                        InternalKind::RegExp(r) => r,
                        _ => unreachable!(),
                    };
                    let rx = re.compiled.as_ref().unwrap();
                    rx.captures_positions_at(input, start)
                };
                let indices_arr = rt.alloc_object(Object::new_array());
                if let Some((_, _, pos_caps)) = &positions {
                    for (i, p) in pos_caps.iter().enumerate() {
                        let v = match p {
                            Some((s, e)) => {
                                let pair = rt.alloc_object(Object::new_array());
                                rt.object_set(pair, "0".into(), Value::Number(byte_to_utf16(input, *s) as f64));
                                rt.object_set(pair, "1".into(), Value::Number(byte_to_utf16(input, *e) as f64));
                                rt.object_set(pair, "length".into(), Value::Number(2.0));
                                Value::Object(pair)
                            }
                            None => Value::Undefined,
                        };
                        rt.object_set(indices_arr, i.to_string(), v);
                    }
                    rt.object_set(indices_arr, "length".into(),
                        Value::Number(pos_caps.len() as f64));
                    // .indices.groups mirrors .groups but with [start,end] pairs.
                    if !named_list.is_empty() {
                        let ig_obj = rt.alloc_object_with_explicit_null_proto(Object::new_ordinary());
                        for (name, idx) in &named_list {
                            let v = match pos_caps.get(*idx).and_then(|c| c.as_ref()) {
                                Some((s, e)) => {
                                    let pair = rt.alloc_object(Object::new_array());
                                    rt.object_set(pair, "0".into(), Value::Number(byte_to_utf16(input, *s) as f64));
                                    rt.object_set(pair, "1".into(), Value::Number(byte_to_utf16(input, *e) as f64));
                                    rt.object_set(pair, "length".into(), Value::Number(2.0));
                                    Value::Object(pair)
                                }
                                None => Value::Undefined,
                            };
                            rt.object_set(ig_obj, name.clone(), v);
                        }
                        rt.object_set(indices_arr, "groups".into(), Value::Object(ig_obj));
                    } else {
                        rt.object_set(indices_arr, "groups".into(), Value::Undefined);
                    }
                }
                rt.object_set(arr, "indices".into(), Value::Object(indices_arr));
            }
            Ok(Value::Object(arr))
        }
    }
}

fn byte_to_char_index(s: &str, byte_off: usize) -> usize {
    s[..byte_off.min(s.len())].chars().count()
}

fn current_regexp_this(rt: &Runtime, label: &str) -> Result<ObjectRef, RuntimeError> {
    match rt.current_this() {
        Value::Object(id) if matches!(rt.obj(id).internal_kind, InternalKind::RegExp(_)) => Ok(id),
        _ => Err(RuntimeError::TypeError(format!("{}: this is not a RegExp", label))),
    }
}

// ──────────────── String.prototype regex-aware methods ────────────────
//
// These mount onto the existing string_prototype object after install_prototypes
// runs. They shadow/replace the existing .replace and .split, which handle
// only string arguments today.

fn install_string_regex_methods(rt: &mut Runtime) {
    let host = match rt.string_prototype {
        Some(id) => id,
        None => return,
    };

    // IR-EXT 71: Receiver coercion uses rt.to_string_strict instead of the
    // static abstract_ops::to_string. Static to_string yields '[object Object]'
    // for any Object receiver, including String wrappers — visible bug when
    // these methods are called on `new String("...")`. to_string_strict
    // properly dispatches @@toPrimitive / toString / valueOf.

    register_method(rt, host, "match", |rt, args| {
        let s = rt.to_string_strict(&rt.current_this())?;
        let re_id = coerce_regexp(rt, args.first().cloned().unwrap_or(Value::Undefined))?;
        let is_global = match &rt.obj(re_id).internal_kind {
            InternalKind::RegExp(r) => r.flags.contains('g'),
            _ => false,
        };
        if !is_global {
            return regexp_exec(rt, re_id, &s);
        }
        let rx = match &rt.obj(re_id).internal_kind {
            InternalKind::RegExp(r) => r.compiled.clone(),
            _ => None,
        };
        let rx = match rx {
            Some(r) => r,
            None => return Err(RuntimeError::TypeError(
                "String.prototype.match: regex pattern unsupported".into())),
        };
        let matches: Vec<String> = rx.find_iter_owned(&s).into_iter().map(|(_,_,s)| s).collect();
        if matches.is_empty() { return Ok(Value::Null); }
        let arr = rt.alloc_object(Object::new_array());
        for (i, m) in matches.iter().enumerate() {
            rt.object_set(arr, i.to_string(), Value::String(Rc::new(m.clone())));
        }
        rt.object_set(arr, "length".into(), Value::Number(matches.len() as f64));
        Ok(Value::Object(arr))
    });

    register_method(rt, host, "search", |rt, args| {
        let s = rt.to_string_strict(&rt.current_this())?;
        let re_id = coerce_regexp(rt, args.first().cloned().unwrap_or(Value::Undefined))?;
        let rx = match &rt.obj(re_id).internal_kind {
            InternalKind::RegExp(r) => r.compiled.clone(),
            _ => None,
        };
        let rx = match rx {
            Some(r) => r,
            None => return Err(RuntimeError::TypeError(
                "String.prototype.search: regex pattern unsupported".into())),
        };
        match rx.find_first(&s) {
            Some((start, _)) => Ok(Value::Number(byte_to_char_index(&s, start) as f64)),
            None => Ok(Value::Number(-1.0)),
        }
    });

    // SPML-EXT 1: ECMA-262 §22.1.3.{17,18,21} mandate length=2 for
    // String.prototype.{replace,replaceAll,split}. The regexp module
    // installs them last (overriding prototype.rs's arity-2 stubs),
    // and previously used register_method which forces length=0,
    // observable via __split.length test262 fixtures.
    crate::intrinsics::register_intrinsic_method(rt, host, "replace", 2, |rt, args| {
        let s = rt.to_string_strict(&rt.current_this())?;
        let pat_arg = args.first().cloned().unwrap_or(Value::Undefined);
        let repl = args.get(1).cloned().unwrap_or(Value::Undefined);
        string_replace_impl(rt, &s, pat_arg, repl, false)
    });

    crate::intrinsics::register_intrinsic_method(rt, host, "replaceAll", 2, |rt, args| {
        let s = rt.to_string_strict(&rt.current_this())?;
        let pat_arg = args.first().cloned().unwrap_or(Value::Undefined);
        if let Value::Object(id) = &pat_arg {
            if let InternalKind::RegExp(r) = &rt.obj(*id).internal_kind {
                if !r.flags.contains('g') {
                    return Err(RuntimeError::TypeError(
                        "String.prototype.replaceAll: non-global RegExp".into()));
                }
            }
        }
        let repl = args.get(1).cloned().unwrap_or(Value::Undefined);
        string_replace_impl(rt, &s, pat_arg, repl, true)
    });

    crate::intrinsics::register_intrinsic_method(rt, host, "split", 2, |rt, args| {
        let s = rt.to_string_strict(&rt.current_this())?;
        // SPML-EXT 2: ECMA-262 §22.1.3.21 step 6/7 — limit = (limit === undefined)
        // ? 2^32 - 1 : ToUint32(limit). NaN/non-finite collapse to 0 (NaN→+0→
        // ToUint32→0). Previously NaN went to None (treated as "no limit"),
        // so split("hello", /l/, "hi") returned ["he","","o"] instead of [].
        let limit: usize = match args.get(1) {
            None | Some(Value::Undefined) => u32::MAX as usize,
            Some(v) => {
                let n = abstract_ops::to_number(v);
                if !n.is_finite() { 0 } else {
                    let f = n.trunc();
                    f.rem_euclid(4294967296.0) as u32 as usize
                }
            }
        };
        if limit == 0 {
            let out = rt.alloc_object(Object::new_array());
            rt.object_set(out, "length".into(), Value::Number(0.0));
            return Ok(Value::Object(out));
        }
        let parts: Vec<String> = match args.first() {
            None | Some(Value::Undefined) => vec![s.clone()],
            Some(Value::Object(id)) if matches!(rt.obj(*id).internal_kind, InternalKind::RegExp(_)) => {
                let rx = match &rt.obj(*id).internal_kind {
                    InternalKind::RegExp(r) => r.compiled.clone(),
                    _ => None,
                };
                let rx = match rx {
                    Some(r) => r,
                    None => return Err(RuntimeError::TypeError(
                        "String.prototype.split: regex pattern unsupported".into())),
                };
                rx.split_str(&s)
            }
            Some(sep_v) => {
                let sep = rt.to_string_strict(sep_v)?;
                if sep.is_empty() {
                    s.chars().map(|c| c.to_string()).collect()
                } else {
                    s.split(&sep).map(|p| p.to_string()).collect()
                }
            }
        };
        let truncated: Vec<String> = parts.into_iter().take(limit).collect();
        let out = rt.alloc_object(Object::new_array());
        for (i, p) in truncated.iter().enumerate() {
            rt.object_set(out, i.to_string(), Value::String(Rc::new(p.clone())));
        }
        rt.object_set(out, "length".into(), Value::Number(truncated.len() as f64));
        Ok(Value::Object(out))
    });
}

/// Common backend for .replace and .replaceAll. `force_global` is true
/// for .replaceAll. Replacement may be a string (no $1 backref handling
/// in v1) or a function (called with (match) — extended args deferred).
/// ECMA-262 §22.1.3.18 step 11 GetSubstitution for regex replacement:
///   $$ → literal $
///   $& → matched substring
///   $` → portion before the match
///   $' → portion after the match
///   $N → Nth capture group (1-indexed; $0 is not special)
///   $NN → two-digit group form (if NN is a valid 1-99 group index)
///   ${name} → named group (best-effort; v1 returns empty for missing)
fn process_regex_substitution(
    repl: &str, matched: &str, before: &str, after: &str, groups: &[Option<&str>],
) -> String {
    let mut out = String::with_capacity(repl.len());
    let bytes = repl.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'$' => { out.push('$'); i += 2; continue; }
                b'&' => { out.push_str(matched); i += 2; continue; }
                b'`' => { out.push_str(before); i += 2; continue; }
                b'\'' => { out.push_str(after); i += 2; continue; }
                b'0'..=b'9' => {
                    // Try two-digit form first, then single-digit.
                    let n2 = if i + 2 < bytes.len() && (bytes[i + 2] as char).is_ascii_digit() {
                        let n = (bytes[i + 1] - b'0') as usize * 10 + (bytes[i + 2] - b'0') as usize;
                        if n >= 1 && n <= groups.len() { Some((n, 3usize)) } else { None }
                    } else { None };
                    let n1 = {
                        let n = (bytes[i + 1] - b'0') as usize;
                        if n >= 1 && n <= groups.len() { Some((n, 2usize)) } else { None }
                    };
                    let pick = n2.or(n1);
                    if let Some((n, adv)) = pick {
                        if let Some(g) = groups.get(n - 1).and_then(|g| g.as_deref()) {
                            out.push_str(g);
                        }
                        i += adv;
                        continue;
                    }
                }
                b'{' => {
                    // ${name} — find the closing brace; v1 returns empty
                    // string for missing names (RegExp named groups are
                    // not fully threaded through the captures tuple yet).
                    if let Some(end) = repl[i + 2..].find('}') {
                        let _name = &repl[i + 2..i + 2 + end];
                        i += 2 + end + 1;
                        continue;
                    }
                }
                _ => {}
            }
        }
        let ch_start = i;
        let mut ch_end = i + 1;
        while ch_end < bytes.len() && (bytes[ch_end] & 0xC0) == 0x80 { ch_end += 1; }
        out.push_str(&repl[ch_start..ch_end]);
        i = ch_end;
    }
    out
}

fn string_replace_impl(
    rt: &mut Runtime,
    s: &str,
    pat: Value,
    repl: Value,
    force_global: bool,
) -> Result<Value, RuntimeError> {
    // Pattern path.
    let (rx, is_global) = match &pat {
        Value::Object(id) if matches!(rt.obj(*id).internal_kind, InternalKind::RegExp(_)) => {
            let (rx, flags) = match &rt.obj(*id).internal_kind {
                InternalKind::RegExp(r) => (r.compiled.clone(), (*r.flags).clone()),
                _ => unreachable!(),
            };
            let rx = match rx {
                Some(r) => r,
                None => return Err(RuntimeError::TypeError(
                    "String.prototype.replace: regex pattern unsupported".into())),
            };
            (rx, force_global || flags.contains('g'))
        }
        _ => {
            // String needle — escape to a literal regex so we share the
            // same replacement plumbing. Cheaper than maintaining a
            // separate code path.
            let needle = rt.coerce_to_string(&pat)?;
            let escaped = regex::escape(&needle);
            let rx = regex::Regex::new(&escaped).map_err(|e| RuntimeError::TypeError(format!("{}", e)))?;
            (CompiledRegex::Rust(rx), force_global)
        }
    };

    // Replacement is either a function (callable) or coerced to string.
    let is_callable = matches!(&repl, Value::Object(id) if {
        matches!(rt.obj(*id).internal_kind,
            InternalKind::Function(_) | InternalKind::Closure(_) | InternalKind::BoundFunction(_))
    });

    if !is_callable {
        let repl_s = rt.coerce_to_string(&repl)?;
        // ECMA-262 §22.1.3.18 step 11 GetSubstitution — honor $$, $&,
        // $`, $', $N (capture groups), ${name} (named groups). Loop per
        // match so substitutions see the right before/after context.
        let mut out = String::new();
        let mut cursor = 0usize;
        let mut search_start = 0usize;
        let mut count = 0usize;
        let max_n = if is_global { usize::MAX } else { 1 };
        while count < max_n {
            let caps = match rx.captures_at(s, search_start) {
                Some(c) => c, None => break,
            };
            let (mstart, mend, groups) = caps;
            out.push_str(&s[cursor..mstart]);
            let matched = &s[mstart..mend];
            let before = &s[..mstart];
            let after = &s[mend..];
            let group_slices: Vec<Option<&str>> = groups.iter().skip(1)
                .map(|g| g.as_deref()).collect();
            let substituted = process_regex_substitution(
                &repl_s, matched, before, after, &group_slices);
            out.push_str(&substituted);
            cursor = mend;
            search_start = if mend == mstart { mend + 1 } else { mend };
            count += 1;
            if search_start > s.len() { break; }
        }
        out.push_str(&s[cursor..]);
        return Ok(Value::String(Rc::new(out)));
    }

    // Function replacer — collect match ranges, then invoke the function
    // for each match and stitch the output. We split the borrow so we
    // can call back into the runtime.
    // Tier-Ω.5.vvv: per ECMA-262 §22.1.3.18, the replace callback is
    // invoked with (match, p1, p2, ..., pN, offset, string). Earlier we
    // passed only `match` — pluralize's `function (match, index) {
    // ...word[index - 1]... }` callback expected `index` as the second
    // arg (first capture group OR offset when no captures); with only
    // `match` passed, index was undefined and `word[NaN]` cascaded into
    // a Cannot-read-of-undefined fault three layers downstream.
    //
    // The hand-rolled regex captures populate the captures field;
    // Rust's regex crate captures via captures_at. The dual-engine
    // wrapper's captures_at returns (start, end, captures_as_strings)
    // where captures[0] is the whole match and captures[1..] are the
    // groups.
    // RES-EXT 3: per ECMA-262 §22.1.3.18 step 11.a.iii, when the regex
    // has named groups, the replacer callback receives a final `groups`
    // argument after the input string: (match, ...captures, offset,
    // input, groups). Cache the name-map outside the loop.
    let named = rx.named_groups();
    let mut out = String::new();
    let mut cursor = 0usize;
    let mut search_start = 0usize;
    let mut count = 0usize;
    let max_n = if is_global { usize::MAX } else { 1 };
    while count < max_n {
        let caps = match rx.captures_at(s, search_start) {
            Some(c) => c,
            None => break,
        };
        let (mstart, mend, groups) = caps;
        out.push_str(&s[cursor..mstart]);
        let mut call_args: Vec<Value> = Vec::new();
        // groups[0] is the match itself; groups[1..] are capture groups.
        for g in groups.iter() {
            call_args.push(match g {
                Some(s) => Value::String(Rc::new(s.clone())),
                None => Value::Undefined,
            });
        }
        // offset
        call_args.push(Value::Number(mstart as f64));
        // full input string
        call_args.push(Value::String(Rc::new(s.to_string())));
        // RES-EXT 3: groups arg (only when named groups exist, per spec
        // — passing undefined when absent would change callback arity).
        if !named.is_empty() {
            let g_obj = rt.alloc_object_with_explicit_null_proto(Object::new_ordinary());
            for (name, idx) in &named {
                let v = groups.get(*idx).and_then(|g| g.clone())
                    .map(|s| Value::String(Rc::new(s)))
                    .unwrap_or(Value::Undefined);
                rt.object_set(g_obj, name.clone(), v);
            }
            call_args.push(Value::Object(g_obj));
        }
        let r = rt.call_function(repl.clone(), Value::Undefined, call_args)?;
        // RPTC.7 bug pattern: was static abstract_ops::to_string; replacer
        // may return an Object with toString() / @@toPrimitive.
        let r_s = rt.coerce_to_string(&r)?;
        out.push_str(&r_s);
        cursor = mend;
        // Advance search_start. Avoid zero-width infinite loop.
        search_start = if mend == mstart { mend + 1 } else { mend };
        count += 1;
        if search_start > s.len() { break; }
    }
    out.push_str(&s[cursor..]);
    Ok(Value::String(Rc::new(out)))
}

/// If the value is already a RegExp, return its id. Otherwise treat it
/// as a string pattern (no flags) and construct a fresh RegExp.
fn coerce_regexp(rt: &mut Runtime, v: Value) -> Result<ObjectRef, RuntimeError> {
    if let Value::Object(id) = &v {
        if matches!(rt.obj(*id).internal_kind, InternalKind::RegExp(_)) {
            return Ok(*id);
        }
    }
    let pattern = rt.coerce_to_string(&v)?;
    new_regexp(rt, &pattern, "")
}

// ──────────────── local helpers ────────────────

fn register_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    // NACR-EXT 1: built-in prototype methods are non-constructors per
    // ECMA-262 §21.3. Mirrors the intrinsics.rs::register_method discipline.
    // Pre-fix RegExp.prototype.{replace, exec, test, ...} registered here
    // had is_constructor=true so `new (/x/).test()` did not throw and
    // `isConstructor(RegExp.prototype.test)` returned true.
    let fn_obj = crate::intrinsics::make_native_non_ctor(name, 0, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.obj_mut(host).dict_mut().insert(name.into(), PropertyDescriptor {
        value: Value::Object(fn_id),
        writable: true,
        enumerable: false,
        configurable: true, getter: None, setter: None,
    });
}

/// Install an accessor getter on RegExp.prototype that, when invoked,
/// brand-checks the receiver for [[RegExpData]] (InternalKind::RegExp)
/// and reads from the receiver's own data property of the same name.
/// Per spec §22.2.6.{1..14} all of source/flags/global/ignoreCase/
/// multiline/sticky/unicode/dotAll/hasIndices are accessor properties
/// on RegExp.prototype; test262 probes the descriptor shape via
/// Object.getOwnPropertyDescriptor(RegExp.prototype, key).get.
fn install_regexp_proto_accessor(rt: &mut Runtime, host: ObjectRef, name: &'static str) {
    let getter_obj = crate::intrinsics::make_native_non_ctor(
        &format!("get {}", name), 0, move |rt, _args| {
            let this = match rt.current_this() {
                Value::Object(id) => id,
                _ => return Err(RuntimeError::TypeError(format!(
                    "get {}: this is not an Object", name))),
            };
            // Brand-check via [[RegExpData]] internal slot.
            if !matches!(rt.obj(this).internal_kind, InternalKind::RegExp(_)) {
                // §22.2.6.{...}: when called on RegExp.prototype directly,
                // some accessors return a default and others throw. Spec
                // chose throw for source/flags; for the boolean flags it
                // returns undefined when called on %RegExp.prototype%
                // specifically. v1 throws uniformly — test262 mostly
                // probes the accessor's existence, not its prototype-only
                // invocation behavior.
                return Err(RuntimeError::TypeError(format!(
                    "RegExp.prototype.{}: this is not a RegExp", name)));
            }
            // Read the per-instance data property (stored at init time).
            Ok(rt.object_get(this, name))
        });
    let getter_id = rt.alloc_object(getter_obj);
    rt.obj_mut(host).dict_mut().insert(name.into(), PropertyDescriptor {
        value: Value::Undefined,
        writable: false, enumerable: false, configurable: true,
        getter: Some(Value::Object(getter_id)), setter: None,
    });
}

fn register_global_native<F>(rt: &mut Runtime, name: &str, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    let fn_obj = make_native(name, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.globals.insert(name.into(), Value::Object(fn_id));
}
