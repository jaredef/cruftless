//! Spec-XML parser — resolver-instance #0b per IR-DESIGN.md §0.
//!
//! Parses ECMA-262's `<emu-alg>` blocks (the Bikeshed source format used
//! by tc39/ecma262) into `SpecStepRecord` lists, the canonical input to
//! the IR-vs-spec linter.
//!
//! Tier 2 scope:
//!   - Numbered-step extraction (the spec uses `1.` for every line; Bikeshed
//!     auto-numbers via nesting indent).
//!   - Abstract-op recognition (ToObject, IsCallable, Get, etc.).
//!   - Throw-class recognition (TypeError / RangeError / ReferenceError /
//!     SyntaxError).
//!   - Step-ID synthesis matching the conventional "1.2.3.a.i" form.
//!
//! Tier 2 deliberately ignores:
//!   - Markup beyond what's in the algorithm body (emu-xref, emu-grammar,
//!     etc.). The parser takes the unwrapped emu-alg text as input.
//!   - Note steps + editor's notes.
//!   - Steps wrapped in `<emu-note>` or conditional `<ins>` tags.
//!
//! Composition: this module produces `Vec<SpecStepRecord>` that the linter
//! at `lint.rs` consumes directly. No coupling beyond the SpecStepRecord
//! shape.

use crate::lint::SpecStepRecord;

/// Parse an emu-alg body (the text between `<emu-alg>` and `</emu-alg>`)
/// into a list of SpecStepRecords. The input is the line-oriented spec
/// prose, with indentation indicating nesting depth.
pub fn parse_emu_alg(body: &str) -> Vec<SpecStepRecord> {
    let mut out: Vec<SpecStepRecord> = Vec::new();
    // Stack of (indent-spaces, counter-at-that-level).
    let mut stack: Vec<(usize, u32)> = Vec::new();

    for raw_line in body.lines() {
        let trimmed = raw_line.trim_start();
        if !trimmed.starts_with("1.") {
            continue;
        }
        let indent = raw_line.len() - trimmed.len();
        let body_text = trimmed.strip_prefix("1.").unwrap_or(trimmed).trim();

        // Pop stack until we find the matching or shallower level.
        while let Some(&(top_indent, _)) = stack.last() {
            if indent < top_indent {
                stack.pop();
            } else {
                break;
            }
        }

        // Determine the counter at this indent level.
        let counter = if let Some(&(top_indent, top_counter)) = stack.last() {
            if top_indent == indent {
                let new_counter = top_counter + 1;
                let last = stack.last_mut().unwrap();
                last.1 = new_counter;
                new_counter
            } else {
                // Deeper level — push a fresh counter at 1.
                stack.push((indent, 1));
                1
            }
        } else {
            stack.push((indent, 1));
            1
        };

        // Build the step ID from the stack: depth-1 items use bare numbers;
        // depth-2 items use alphabetic suffix; depth-3 uses roman numerals
        // (per ECMA-262 / Bikeshed convention).
        let step_id = synthesize_step_id(&stack, counter);

        let (abstract_ops, throws) = extract_calls(body_text);
        out.push(SpecStepRecord {
            step_id,
            abstract_ops,
            throws,
            prose: Box::leak(body_text.to_string().into_boxed_str()),
        });

        // If this step ends with ", then" or ",", the next deeper level
        // belongs under it — keep the stack as-is so deeper-indent lines
        // attach.
        let _ = counter; // already used
    }

    out
}

/// Construct a step-ID from the stack of (indent, counter) pairs plus the
/// current counter. Depth 1 → "1", "2", "3"; depth 2 → "1.a", "1.b";
/// depth 3 → "1.a.i", "1.a.ii"; depth 4 → "1.a.i.1", "1.a.i.2".
fn synthesize_step_id(stack: &[(usize, u32)], _current: u32) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (depth_idx, (_, counter)) in stack.iter().enumerate() {
        let part = match depth_idx {
            0 => counter.to_string(),
            1 => alpha_suffix(*counter),
            2 => roman_lower(*counter),
            _ => counter.to_string(),
        };
        parts.push(part);
    }
    parts.join(".")
}

fn alpha_suffix(n: u32) -> String {
    if n == 0 {
        return String::new();
    }
    let n = n - 1;
    if n < 26 {
        ((b'a' + n as u8) as char).to_string()
    } else {
        // Spec rarely needs > 26 sub-steps; fall back to "aa", "ab", ...
        format!(
            "{}{}",
            ((b'a' + (n / 26 - 1) as u8) as char),
            ((b'a' + (n % 26) as u8) as char)
        )
    }
}

fn roman_lower(n: u32) -> String {
    let mut s = String::new();
    let mut n = n;
    let pairs = [
        (1000, "m"),
        (900, "cm"),
        (500, "d"),
        (400, "cd"),
        (100, "c"),
        (90, "xc"),
        (50, "l"),
        (40, "xl"),
        (10, "x"),
        (9, "ix"),
        (5, "v"),
        (4, "iv"),
        (1, "i"),
    ];
    for &(v, sym) in &pairs {
        while n >= v {
            s.push_str(sym);
            n -= v;
        }
    }
    s
}

/// Scan a spec step's prose for abstract-operation invocations and throw
/// statements. Returns (abstract_ops, throws_class).
fn extract_calls(prose: &str) -> (Vec<&'static str>, Option<&'static str>) {
    let mut ops: Vec<&'static str> = Vec::new();
    let mut throws: Option<&'static str> = None;

    // Spec abstract-op names that we recognize. Each is identified by the
    // pattern `OpName(...)` in the prose. The list is curated to match the
    // IR alphabet plus the runtime-builtin helpers cruftless exposes.
    let known_ops: &[&'static str] = &[
        "ToObject",
        "ToPrimitive",
        "ToString",
        "ToNumber",
        "ToInteger",
        "ToLength",
        "ToUint32",
        "ToBoolean",
        "ToPropertyKey",
        "IsCallable",
        "IsConstructor",
        "IsArray",
        "IsRegExp",
        "RequireObjectCoercible",
        "SameValue",
        "SameValueZero",
        "IsStrictlyEqual",
        "Get",
        "GetV",
        "Set",
        "HasProperty",
        "HasOwnProperty",
        "Call",
        "Construct",
        "Invoke",
        "LengthOfArrayLike",
        "ArraySpeciesCreate",
        "SpeciesConstructor",
        "OrdinaryObjectCreate",
        "OrdinaryDefineOwnProperty",
        "CreateDataPropertyOrThrow",
        "DeletePropertyOrThrow",
        "EnumerableOwnPropertyNames",
        "NewPromiseCapability",
    ];

    for op in known_ops {
        // Recognize `OpName(...)` — preceded by space / `?` / `!` / `(`
        // and followed by `(`. The simplest robust check: search for the
        // op-name followed by `(`, where it's not part of a longer
        // identifier.
        let needle = format!("{}(", op);
        if let Some(pos) = prose.find(needle.as_str()) {
            // Ensure preceded by non-identifier char.
            let preceded_ok = if pos == 0 {
                true
            } else {
                let prev = prose.as_bytes()[pos - 1];
                !(prev.is_ascii_alphanumeric() || prev == b'_')
            };
            if preceded_ok {
                ops.push(op);
            }
        }
    }

    // Throw detection: "throw a *TypeError* exception", "throw a *RangeError*
    // exception", etc. The spec wraps the class in `*Name*` markdown.
    for class in &["TypeError", "RangeError", "ReferenceError", "SyntaxError"] {
        let needle = format!("*{}*", class);
        if prose.contains(needle.as_str()) && prose.contains("throw") {
            throws = Some(class);
            ops.push("Throw");
            break;
        }
    }

    (ops, throws)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The §23.1.3.20 Array.prototype.map algorithm as published in
    /// ECMA-262 / tc39/ecma262 (Bikeshed source). Embedded as a test
    /// fixture; the production version of the parser would load this
    /// from a checkout of the spec repo.
    const MAP_EMU_ALG: &str = r#"
            1. Let _O_ be ? ToObject(*this* value).
            1. Let _len_ be ? LengthOfArrayLike(_O_).
            1. If IsCallable(_callbackfn_) is *false*, throw a *TypeError* exception.
            1. Let _A_ be ? ArraySpeciesCreate(_O_, _len_).
            1. Let _k_ be 0.
            1. Repeat, while _k_ < _len_,
              1. Let _Pk_ be ! ToString(F(_k_)).
              1. Let _kPresent_ be ? HasProperty(_O_, _Pk_).
              1. If _kPresent_ is *true*, then
                1. Let _kValue_ be ? Get(_O_, _Pk_).
                1. Let _mappedValue_ be ? Call(_callbackfn_, _thisArg_, K kValue, F(_k_), _O_ L).
                1. Perform ? CreateDataPropertyOrThrow(_A_, _Pk_, _mappedValue_).
              1. Set _k_ to _k_ + 1.
            1. Return _A_.
        "#;

    #[test]
    fn parses_map_section() {
        let steps = parse_emu_alg(MAP_EMU_ALG);
        // Top-level step count should be 7.
        let top_level: Vec<_> = steps.iter().filter(|s| !s.step_id.contains('.')).collect();
        assert_eq!(
            top_level.len(),
            7,
            "expected 7 top-level steps, got {}: {:?}",
            top_level.len(),
            top_level.iter().map(|s| &s.step_id).collect::<Vec<_>>()
        );

        // Step 1: ToObject
        let s1 = steps.iter().find(|s| s.step_id == "1").expect("step 1");
        assert!(
            s1.abstract_ops.contains(&"ToObject"),
            "step 1 must invoke ToObject"
        );

        // Step 3: IsCallable + Throw + TypeError
        let s3 = steps.iter().find(|s| s.step_id == "3").expect("step 3");
        assert!(s3.abstract_ops.contains(&"IsCallable"));
        assert!(s3.abstract_ops.contains(&"Throw"));
        assert_eq!(s3.throws, Some("TypeError"));

        // Step 6.c.i: Get
        let s_6_c_i = steps
            .iter()
            .find(|s| s.step_id == "6.c.i")
            .expect("step 6.c.i");
        assert!(s_6_c_i.abstract_ops.contains(&"Get"));

        // Step 6.c.ii: Call
        let s_6_c_ii = steps
            .iter()
            .find(|s| s.step_id == "6.c.ii")
            .expect("step 6.c.ii");
        assert!(s_6_c_ii.abstract_ops.contains(&"Call"));

        // Step 6.c.iii: CreateDataPropertyOrThrow
        let s_6_c_iii = steps
            .iter()
            .find(|s| s.step_id == "6.c.iii")
            .expect("step 6.c.iii");
        assert!(s_6_c_iii
            .abstract_ops
            .contains(&"CreateDataPropertyOrThrow"));
    }
}
