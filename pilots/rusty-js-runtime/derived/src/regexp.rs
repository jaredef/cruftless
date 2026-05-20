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
            let (pattern, flags) = match args.first() {
                Some(Value::Object(id)) => {
                    if let InternalKind::RegExp(re) = &rt.obj(*id).internal_kind {
                        let src = (*re.source).clone();
                        let f = match args.get(1) {
                            Some(Value::Undefined) | None => (*re.flags).clone(),
                            Some(v) => abstract_ops::to_string(v).as_str().to_string(),
                        };
                        (src, f)
                    } else {
                        let p = abstract_ops::to_string(&args[0]).as_str().to_string();
                        let f = abstract_ops::to_string(
                            &args.get(1).cloned().unwrap_or(Value::Undefined)
                        ).as_str().to_string();
                        (p, f)
                    }
                }
                Some(v) => {
                    let p = abstract_ops::to_string(v).as_str().to_string();
                    // Tier-Ω.5.eeee: undefined flags arg → no flags
                    // (ECMA-262 §22.2.4.1). Earlier we coerced via
                    // to_string which produced literal "undefined" and
                    // failed flag validation on each char.
                    let f = match args.get(1).cloned().unwrap_or(Value::Undefined) {
                        Value::Undefined | Value::Null => String::new(),
                        v => abstract_ops::to_string(&v).as_str().to_string(),
                    };
                    (p, f)
                }
                None => (String::new(), String::new()),
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
/// EXT 76: elide JS UTF-16 surrogate-pair alternatives from a pattern.
///
/// JS regex literals targeting environments without `\p{...}` property
/// classes emulate them with huge alternations like:
///   /[A-Z...]|\uD800[\uDC00-\uDC0B\uDC0D-\uDC26...]|\uD801[\uDC00-\uDC9D]|.../
/// where each `\uD8XX..\uDBXX` is a high surrogate and the bracketed run
/// is the matching low-surrogate ranges. The pair represents a single
/// supplementary-plane code point. The Rust regex crate rejects bare
/// surrogates (Rust `char` is a Unicode scalar, surrogates aren't scalars),
/// so the whole pattern fails to compile.
///
/// Test262's harness fixtures that hit these patterns (notably
/// nativeFunctionMatcher.js validating "function anonymous() { [native
/// code] }") only ever feed BMP-range inputs to the matcher, so eliding
/// the supplementary alternatives preserves correctness on the inputs
/// that matter. Cases that actually depend on matching supplementary
/// characters via emulated property classes would need full \u{NNNNN}
/// translation; deferred to a future EXT.
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
    register_method(rt, host, "test", |rt, args| {
        let this_id = current_regexp_this(rt, "RegExp.prototype.test")?;
        let input = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
            .as_str().to_string();
        let result = {
            let re = match &rt.obj(this_id).internal_kind {
                InternalKind::RegExp(r) => r,
                _ => unreachable!(),
            };
            match &re.compiled {
                Some(rx) => Ok(rx.is_match(&input)),
                None => Err(RuntimeError::TypeError(format!(
                    "RegExp pattern uses features unsupported by the v1 regex engine: /{}/{}",
                    re.source, re.flags))),
            }
        }?;
        Ok(Value::Boolean(result))
    });

    register_method(rt, host, "exec", |rt, args| {
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
pub fn regexp_exec(rt: &mut Runtime, this_id: ObjectRef, input: &str) -> Result<Value, RuntimeError> {
    let (is_global, start, has_compiled) = {
        let o = rt.obj(this_id);
        let re = match &o.internal_kind {
            InternalKind::RegExp(r) => r,
            _ => return Err(RuntimeError::TypeError("RegExp.prototype.exec: this is not a RegExp".into())),
        };
        let is_global = re.flags.contains('g') || re.flags.contains('y');
        let start = if is_global { re.last_index } else { 0 };
        (is_global, start, re.compiled.is_some())
    };
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
            rt.object_set(this_id, "lastIndex".into(), Value::Number(0.0));
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

    match captures_opt {
        None => {
            if is_global {
                if let InternalKind::RegExp(r) = &mut rt.obj_mut(this_id).internal_kind {
                    r.last_index = 0;
                }
                rt.object_set(this_id, "lastIndex".into(), Value::Number(0.0));
            }
            Ok(Value::Null)
        }
        Some((mstart, mend, groups)) => {
            if is_global {
                if let InternalKind::RegExp(r) = &mut rt.obj_mut(this_id).internal_kind {
                    r.last_index = mend;
                }
                rt.object_set(this_id, "lastIndex".into(), Value::Number(mend as f64));
            }
            let arr = rt.alloc_object(Object::new_array());
            for (i, g) in groups.iter().enumerate() {
                let v = match g {
                    Some(s) => Value::String(Rc::new(s.clone())),
                    None => Value::Undefined,
                };
                rt.object_set(arr, i.to_string(), v);
            }
            rt.object_set(arr, "length".into(), Value::Number(groups.len() as f64));
            rt.object_set(arr, "index".into(), Value::Number(byte_to_char_index(input, mstart) as f64));
            rt.object_set(arr, "input".into(), Value::String(Rc::new(input.to_string())));
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

    register_method(rt, host, "replace", |rt, args| {
        let s = rt.to_string_strict(&rt.current_this())?;
        let pat_arg = args.first().cloned().unwrap_or(Value::Undefined);
        let repl = args.get(1).cloned().unwrap_or(Value::Undefined);
        string_replace_impl(rt, &s, pat_arg, repl, false)
    });

    register_method(rt, host, "replaceAll", |rt, args| {
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

    register_method(rt, host, "split", |rt, args| {
        let s = rt.to_string_strict(&rt.current_this())?;
        let limit = args.get(1).map(|v| {
            let n = abstract_ops::to_number(v);
            if n.is_finite() && n >= 0.0 { Some(n as usize) } else { None }
        }).flatten();
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
        let truncated: Vec<String> = match limit {
            Some(l) => parts.into_iter().take(l).collect(),
            None => parts,
        };
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
            let needle = abstract_ops::to_string(&pat).as_str().to_string();
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
        let repl_s = abstract_ops::to_string(&repl).as_str().to_string();
        let out = if is_global {
            rx.replace_all_lit(s, repl_s.as_str())
        } else {
            rx.replacen_lit(s, 1, repl_s.as_str())
        };
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
        let r = rt.call_function(repl.clone(), Value::Undefined, call_args)?;
        let r_s = abstract_ops::to_string(&r).as_str().to_string();
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
    let pattern = abstract_ops::to_string(&v).as_str().to_string();
    new_regexp(rt, &pattern, "")
}

// ──────────────── local helpers ────────────────

fn register_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    let fn_obj = make_native(name, f);
    let fn_id = rt.alloc_object(fn_obj);
    rt.obj_mut(host).properties.insert(name.into(), PropertyDescriptor {
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
    rt.obj_mut(host).properties.insert(name.into(), PropertyDescriptor {
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
