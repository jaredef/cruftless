# JSF-EXT 2 — JSON.stringify fast-path design

*Enumerates per-component substrate-move plan derived from JSF-EXT 1 bench baseline. Each move scoped, falsifier-anchored, and ordered by dependency.*

## 1. Design constraints (C1-C8 enumeration)

Following the Pin-Art constraint-enumeration discipline (per Doc 581 + Φ seed §I.2 + Doc 739 cascade-revival apparatus):

```
C1. Byte-identical output to node on Pred-jsf.2 (canonical fuzz acc=-932188103).
C2. ECMA-262 §25.5.2 SerializeJSONProperty semantics preserved (Symbol skip;
    Undefined skip in object branch; @@-prefix skip; __primitive__ unwrap;
    NaN/Infinity → "null"; non-enumerable skip).
C3. Shape-aware iteration preserved per CMig-EXT 16.bis pattern (shape-iter
    chain then properties-iter).
C4. Cap-passing modes unaffected (JSON.stringify has no cap surface).
C5. Recursion semantics preserved (deep-nested still correct).
C6. PropertyDescriptor.clone() avoidance only where the recursive call's
    borrow allows (no breaking the existing borrow discipline).
C7. No new env flag; the fast-path is the default once landed (this is a
    correctness-preserving perf improvement, not a flag-gated experiment).
C8. The canonical fuzz (CMig-EXT 17) + diff-prod + bench all gate each
    JSF-EXT round per the standing three-probe-levels discipline.

The architecture induced by C1+C2+C5+C7: a chunked output buffer threaded
through the recursion, with per-type fast paths emitting directly into the
buffer (no intermediate String allocations beyond the buffer itself). The
fast paths preserve ECMA semantics by construction; the buffer threading
preserves recursion correctness by construction.
```

## 2. Component move plan

Five substrate moves, dependency-ordered:

### Move 1 (JSF-EXT 3, FOUNDATIONAL) — output buffer threading

**Mechanism**: change the JSON.stringify entry from `fn json_stringify(rt: &Runtime, v: &Value) -> String` to `fn json_stringify_into(rt: &Runtime, v: &Value, out: &mut String)`. Internal recursion uses the buffer; the entry-point wrapper allocates the buffer once with a sensible initial capacity (e.g., 256 bytes) and returns the final String.

**LOC estimate**: ~80 (signature change + per-branch buffer.push_str/.push calls + the entry wrapper).

**Per-shape expected reclaim**:
- A small-object: ~2× (eliminate the per-object format! + join + entries.collect()→Vec<String>)
- B deep-nested: ~3× (eliminates O(depth × siblings) intermediate allocations)
- C array-of-obj: ~2× (same as A; sort+collect still happen)
- D number-only: ~1.2× (eliminates the per-call String return; small gain since this path is allocation-bound)
- E string-only: ~1.5× (eliminates the per-call String::with_capacity; main win comes from Move 4 below)

**Pred-jsf.X gating**: Pred-jsf.2 (canonical fuzz byte-identical); Pred-jsf.3 (diff-prod 42/42); CRB json_parse_transform ≤25% regression-or-improvement (intermediate measurement; full Pred-jsf.1 measured after all 4 moves land).

**Falsifier**: any output divergence vs current implementation. Verified via differential test: run current `json_stringify` + new `json_stringify_into`, diff outputs across a corpus of seeded fixtures.

### Move 2 (JSF-EXT 4) — string-escape branchless ASCII fast path

**Mechanism**: replace `json_quote_string`'s char-by-char loop with a two-stage approach. Stage 1 scans the source as `&[u8]`; if all bytes are in the safe ASCII range `[0x20-0x7E]` minus `"` and `\`, `extend_from_slice` directly. Stage 2 fallback handles the escape cases (rare on most strings).

```rust
// New shape (depends on Move 1's buffer threading):
fn json_quote_string_into(s: &str, out: &mut String) {
    out.push('"');
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        // Stage 1: scan ahead for the next byte requiring escape.
        let start = i;
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'"' || b == b'\\' || b < 0x20 { break; }
            // ASCII non-special: continue. UTF-8 multibyte continuation
            // bytes (≥ 0x80) are non-special too.
            i += 1;
        }
        // Stage 1 emit: bulk copy [start..i] as UTF-8.
        if i > start {
            // SAFETY: slice is a valid UTF-8 prefix of the original String.
            out.push_str(unsafe { std::str::from_utf8_unchecked(&bytes[start..i]) });
        }
        // Stage 2: handle the escape case.
        if i < bytes.len() {
            match bytes[i] {
                b'"'  => out.push_str("\\\""),
                b'\\' => out.push_str("\\\\"),
                b'\n' => out.push_str("\\n"),
                b'\r' => out.push_str("\\r"),
                b'\t' => out.push_str("\\t"),
                b'\x08' => out.push_str("\\b"),
                b'\x0c' => out.push_str("\\f"),
                c => {
                    // Control char: emit \u00XX via a small inline buffer.
                    let hi = (c >> 4) & 0xF;
                    let lo = c & 0xF;
                    out.push_str("\\u00");
                    out.push(if hi < 10 { (b'0' + hi) as char } else { (b'a' + hi - 10) as char });
                    out.push(if lo < 10 { (b'0' + lo) as char } else { (b'a' + lo - 10) as char });
                }
            }
            i += 1;
        }
    }
    out.push('"');
}
```

**LOC estimate**: ~50 (the new fn + call-site migration; old `json_quote_string` retained for any non-fast-path consumer or as the test-oracle).

**Per-shape expected reclaim**:
- A small-object: ~1.3× (some strings present, mostly the keys)
- B deep-nested: ~1.2× (few strings; mostly object nesting)
- C array-of-obj: ~1.5× (more strings per element)
- D number-only: ~1.0× (no strings)
- E string-only: **~3-5×** (this is the path) — replaces N char-pushes with 1 bulk copy + N escape branches

**Pred-jsf.X gating**: same probes; specifically Pred-jsf.4 covers edge cases (control chars; high-byte UTF-8; \u escape correctness).

**Falsifier**: per-byte differential test on a corpus of strings (ASCII + UTF-8 multibyte + control chars + surrogates).

### Move 3 (JSF-EXT 5) — number-stringify fast path + chunked write

**Mechanism**: replace `abstract_ops::number_to_string` allocate-per-call with direct buffer-write. For integers (cruft's typed-i64 alphabet path): write via `itoa`-style algorithm directly into the buffer. For f64 fractional: use the existing number_to_string but write its result into the buffer rather than allocating a new String.

Integer fast path (no allocation):

```rust
fn write_i64_into(mut n: i64, out: &mut String) {
    if n == 0 { out.push('0'); return; }
    if n < 0 { out.push('-'); n = -n; }  // careful with i64::MIN
    // Reverse-digit emit then reverse the appended slice.
    let start = out.len();
    while n > 0 {
        out.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
    }
    // SAFETY: digits are all ASCII; in-place reverse is byte-level safe.
    unsafe { out.as_bytes_mut()[start..].reverse(); }
}
```

f64 path: keep calling `abstract_ops::number_to_string` for now (Ryu-style algorithm is its own pilot if perf needs more), but write its String result into the output buffer rather than allocating a new String per call.

**LOC estimate**: ~70 (integer fast path + the f64 path adapter).

**Per-shape expected reclaim**:
- A small-object: ~1.2× (a few numbers per object)
- B deep-nested: ~1.1× (few numbers; nesting dominates)
- C array-of-obj: ~1.5× (number per element score field)
- D number-only: **~5-8×** (this is the path)
- E string-only: ~1.0× (no numbers)

**Falsifier**: integer round-trip correctness on canonical fuzz; f64 NaN/Infinity → "null" preserved; i64::MIN edge case handled.

### Move 4 (JSF-EXT 6) — format-macro elimination + property iteration via reference

**Mechanism**: replace `format!("[{}]", body.join(","))` and `format!("{{{}}}", entries.join(","))` with explicit push/push_str sequences. Iterate properties via `&Object` references (not the snapshot-clone pattern) where the recursive call's borrow doesn't conflict — for the shape-iter chain this works because we have the shape-aware shape_values reference; for the dict-iter chain we still need the clone (Op::SetProp could mutate during recursion).

**Subtlety**: recursive `json_stringify_into(rt, ..., out)` borrows `rt` immutably. The property iteration also borrows `rt.obj(*id)`. Both can coexist via NLL. But Op::SetProp mutation during nested recursion (e.g., via toJSON dispatch) would require &mut — currently the code skips toJSON dispatch for that reason (see comments at intrinsics.rs:5721-5726). So clone-avoidance is safe at first cut.

**LOC estimate**: ~40 (per-call format! → push sequences; the snapshot now references shape_values directly).

**Per-shape expected reclaim**:
- A small-object: ~1.2× (per-object format! + clone eliminated)
- B deep-nested: ~1.2× (compounds with depth)
- C array-of-obj: ~1.3× (per-element format! + clone eliminated)
- D number-only: ~1.0× (no objects)
- E string-only: ~1.0× (no objects)

**Falsifier**: same canonical fuzz; no semantic change.

### Move 5 (JSF-EXT 7, optional) — JSON.parse fast-path

Out of first-cut scope per seed §I.1. Reserved for follow-on if Pred-jsf.1 isn't met by Moves 1-4.

## 3. Composition reading

Multiplicative expected reclaim (per-shape, vs baseline):

| shape | M1 | M2 | M3 | M4 | total |
|---|---:|---:|---:|---:|---:|
| A small-object | 2× | 1.3× | 1.2× | 1.2× | ~3.7× |
| B deep-nested | 3× | 1.2× | 1.1× | 1.2× | ~4.7× |
| C array-of-obj | 2× | 1.5× | 1.5× | 1.3× | ~5.9× |
| D number-only | 1.2× | 1.0× | 5-8× | 1.0× | **6-10×** |
| E string-only | 1.5× | 3-5× | 1.0× | 1.0× | **4.5-7.5×** |

Post-Move-4 expected cruft/node ratios:
- A: 10.58× → ~2.9×
- B: 14.11× → ~3.0×
- C: 12.48× → ~2.1×
- D: 15.16× → ~1.5-2.5×
- E: 10.09× → ~1.3-2.2×

If reclaim multipliers are conservative (cumulative composition often beats sum-of-parts when targets are orthogonal per Finding III.4): cruft/node ratios could reach 1-2× across all shapes.

For Pred-jsf.1 (CRB json_parse_transform ≥40% reclaim target = ≤1500 ms):
- Baseline 2481 ms with JSON.stringify estimated at ~5-10× contributor (~50-70% of total cost)
- Post-Moves 1-4 with ~5× JSON.stringify reclaim → ~40-60% reclaim on JSON.stringify component
- Net CRB cruft-time: ~1500-1800 ms estimated; **Pred-jsf.1 likely MET** but with margin uncertainty

## 4. Doc 738 §II conventions checklist

- Module path: `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (existing); fast-path helpers as `pub(crate) fn` (engine-internal); no `_via` suffix (pure-primitive helpers).
- Function naming: `json_stringify_into`, `json_quote_string_into`, `write_i64_into`. The `_into` suffix names the buffer-threading convention (Doc 738 §II.b style, new convention class).
- The old `json_stringify` and `json_quote_string` remain as wrappers (allocate buffer + call `_into` + return String). Backward-compat preserved.

## 5. Staged-validation per Findings II.2 + Finding V.5

Each JSF-EXT round (3-6) is a self-contained substrate move that:
1. Adds the new function/path (additive)
2. Migrates the call sites (substitutive)
3. Verifies via canonical fuzz + diff-prod + bench (three probes per Finding rule 5/10)
4. Commits when GREEN
5. Next round builds on landed substrate

No splits into intermediate-state-worse states. Each move REMOVES per-shape allocation overhead AND ADDS the new fast path in the same round.

## 6. Risks

**R1 — String::push_str micro-cost**: many small push_str calls may be slower than one bulk format! due to Rust's per-call inlining overhead. Mitigation: bench at each round; if a move's per-shape reclaim is below estimate, profile + adjust.

**R2 — Output buffer initial capacity**: too small → reallocs; too large → wasted memory. Mitigation: start with 256-byte default; tune at JSF-EXT 5 measurement.

**R3 — i64::MIN edge case** in integer fast path: `-i64::MIN` overflows. Mitigation: special-case `n == i64::MIN` before the negation.

**R4 — UTF-8 multibyte slicing**: stage-1 scan must NOT split a multibyte char. The byte loop handles this naturally (UTF-8 continuation bytes are ≥ 0x80 which is in the "non-special" range); validated via fuzz.

**R5 — Borrow conflict in Move 4 property iteration**: if recursive `json_stringify_into` mutates `rt` somehow, the iter-via-reference breaks. Mitigation: keep clone in dict-iter chain; only shape-iter chain (which reads shape_values via reference) uses no-clone.

## 7. Forward to JSF-EXT 3

JSF-EXT 3 lands Move 1 (output buffer threading). Substrate-introduction per Doc 729 §A8.13; subsequent rounds (4/5/6) are closure rounds consuming the buffer.

---

*JSF-EXT 2 closes. Four substrate moves enumerated; dependency-ordered; per-shape reclaim estimates name expected post-implementation cruft/node ratios at ~1.5-3× (vs current 10-15×). Pred-jsf.1's ≥40% CRB reclaim target empirically reachable per the composition reading. JSF-EXT 3 begins with Move 1 buffer threading.*
