# JSF-EXT 1 — JSON.stringify bench baseline

*Establishes per-shape cruft baselines for the JSON.stringify fast-path pilot. Five shape classes cover the hot-path components; cruft/node ratios identify the highest-impact targets for JSF-EXT 2's design.*

## 1. Bench protocol

Harness: `cruftless/examples/bench_json_stringify.rs`. Runs 5 fixtures × 3 iterations × {cruft, node} via process invocation; reports median wall-clock.

Each fixture exercises a different stringify shape class:
- **A** — small-object stringify 100k times (10 mixed-type keys)
- **B** — deep-nested stringify 100k times (5 levels)
- **C** — array-of-objects stringify 5k times (50 elements × 5 keys)
- **D** — number-only stringify 1M times (drives `number_to_string`)
- **E** — string-only stringify 1M times (drives `json_quote_string`)

## 2. Baseline measurements (Pi, 2026-05-23)

| shape | cruft (μs) | node (μs) | **cruft/node** | cruft per-op |
|---|---:|---:|---:|---:|
| A: small-object 100k | 1,193,587 | 112,765 | **10.58×** | ~11.4 μs/op |
| B: deep-nested 100k | 1,327,405 | 94,052 | **14.11×** | ~12.8 μs/op |
| C: array-of-obj 5k | 1,394,824 | 111,800 | **12.48×** | ~268 μs/op (~1 μs/property) |
| D: number-only 1M | 2,238,954 | 147,679 | **15.16×** | ~2.2 μs/op |
| E: string-only 1M | 2,331,291 | 231,040 | **10.09×** | ~2.3 μs/op |

**Per-shape spread**: 10-15×. **Worst gap: D (number-only) at 15.16×**. **Best gap: E (string-only) at 10.09×** (still 10×).

## 3. Hot-path component decomposition (source-read)

Read of current `json_stringify` (intrinsics.rs:5693-5797) identifies these per-call cost components:

**Per-call (any value)**:
- `match v` dispatch: cheap (1-2 cycles)
- For Object/Array: `rt.obj(*id)` heap-vec index, internal_kind match
- For Object/Array: shape-aware property snapshot (per CMig-EXT 16.bis fix) — clones PropertyDescriptor per property

**Per-Number** (drives D's 15× gap):
- `n.is_finite()` check
- `abstract_ops::number_to_string(*n)` allocates a String for each Number
- Return that String

**Per-String** (drives E's 10× gap):
- `json_quote_string(s)` allocates `String::with_capacity(s.len() + 2)`
- char-by-char iteration via `for c in s.chars()`
- per-char `match` then `out.push(c)` or `out.push_str(escape)` 
- For control chars: `format!("\\u{:04x}", c as u32)` — extra allocation per control char

**Per-Object** (drives A's 10.58× + B's 14.11× gaps):
- `props.iter().filter(...).map(|(k, d)| format!("{}:{}", json_quote_string(k), json_stringify(rt, &d.value)))` — `format!` allocates per property
- `.collect()` into `Vec<String>` — each entry is a fresh String allocation
- `entries.join(",")` — re-allocates the join'd String
- `format!("{{{}}}", joined)` — another String allocation
- For nested objects: recursive call returns fresh String → multi-allocation per depth level

**Per-Array** (drives C's 12.48× gap):
- Same as Object but with extra sort: `entries.sort_by_key(|(i, _)| *i)` per array
- `parse::<usize>().ok()` per array key

## 4. The highest-impact targets

Ranked by (cruft/node ratio × frequency-in-realistic-workloads):

1. **Number-stringify** (15.16× gap; D bench): replace `abstract_ops::number_to_string`'s allocate-per-call with a chunked-buffer-write into the caller's output buffer. For integers: lookup-table-based int-to-decimal (no allocation). Expected reclaim: 4-5× on the number path.

2. **String-escape branchless ASCII fast-path** (10.09× gap; E bench): scan the source string for special chars; if all ASCII-non-special, `extend_from_slice(bytes)` directly without char-by-char. Expected reclaim: 3-5× on ASCII strings (the common case).

3. **Chunked output buffer** (affects all shapes): replace per-call `String::with_capacity` + `format!` + `join(",")` with a single output buffer passed down the recursion. Eliminates O(depth × siblings) intermediate allocations. Expected reclaim: 2-3× across all shapes.

4. **Property snapshot clone-reduction** (smaller; affects A/B/C): the current `props.iter().map(|(k, d)| (k.to_string_content(), d.clone()))` clones each PropertyDescriptor. For the iteration-only path, we don't need owned descriptors; could iterate via reference if the recursive call's borrow doesn't conflict. Expected reclaim: 1.1-1.3× per property.

## 5. Composition with prior corpus / engagement work

- **CRB-EXT 9 reading**: json_parse_transform was 20.34× cruft/node. JSON.stringify estimated at 5-10× of that gap. The micro-bench here at 10-15× per shape EMPIRICALLY ANCHORS the upper end of that estimate.
- **CMig-EXT 16.bis**: the shape-aware property snapshot pattern is the current implementation. The fast-path design preserves shape awareness without the per-property clone overhead.
- **LeJIT pilots**: JSON.stringify runs in interp, not JIT. The LeJIT pilots' wins (Σ, Τ, Ψ-cascaded, Φ) don't directly improve this path. JSON.stringify perf is purely interp-tier substrate work.
- **Findings VI.1 HIGH priority**: this bench empirically validates the priority. Per-shape cruft/node 10-15× across the 5 shape classes confirms JSON.stringify is structurally slow.

## 6. JSF-EXT 2 design queue

JSF-EXT 2 (design doc) should propose:

1. **Output buffer threading**: change `json_stringify(rt, v) -> String` to `json_stringify_into(rt, v, &mut out: String)` — single buffer down the recursion.
2. **Number stringify lookup table** for integer fast path: pre-computed decimal strings for 0-99 (or 0-9999); subtract+shift for larger.
3. **String-escape branchless ASCII scan**: SIMD-or-byte-loop to detect special chars; bulk `extend_from_slice` if none.
4. **Format-macro elimination**: replace `format!("[{}]", body.join(","))` with explicit `out.push('['); for ... { out.push(','); }; out.push(']');`.
5. **Property iteration via reference** (not clone) where the recursive call's borrow allows.

Expected cumulative reclaim: 5-10× cruft-side wall-clock improvement on the bench fixtures. Per Pred-jsf.1's ≥40% reclaim target on json_parse_transform CRB fixture (2481 → ≤1500 ms): the bench → CRB extrapolation is conservative since CRB's workload is JSON.stringify + JSON.parse + Array.filter/map; closing JSON.stringify alone gets us part of the way.

## 7. Forward to JSF-EXT 2

Design doc enumerating per-component substrate-move plan. Output: `docs/design.md`. Each substrate move scoped + falsifier-anchored.

---

*JSF-EXT 1 closes. Baseline established: per-shape cruft/node 10-15×; worst is number-stringify at 15.16×. Four highest-impact targets named (number lookup; ASCII fast-path; output buffer threading; format-macro elimination). JSF-EXT 2 designs the substrate moves.*
