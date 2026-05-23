# VTI-EXT 1 — Pre-emission call-overhead bench baseline

*Establishes the baseline measurement of the per-call dispatcher + arg-coercion + JIT-call + result-rebox overhead for the LeJIT-Ψ pilot. Pred-vti.1 reads against this number.*

## 1. Bench protocol

**Workload**: 1,000,000 iterations of `id(Number(42))` where `function id(x) { return x; }`. The function is the minimal possible — body is `LoadLocal 0; Return`. Per-iter cost is dispatcher + arg-coerce + JIT call + result rebox; no body work.

**Source**: `cruftless/examples/bench_call_overhead.rs`. Run: `cargo build --release --example bench_call_overhead -p cruftless && target/release/examples/bench_call_overhead`.

## 2. Pi baseline measurement (2026-05-23)

```
elapsed:     127.077 ms
per-iter:    127.1 ns
```

Single run; variance characterization deferred to VTI-EXT 6.

## 3. Composition reading vs LeJIT-Σ's bench_ic

The two benches decompose the per-iter cost across the substrate axes:

| bench | per-iter | components |
|---|---:|---|
| `bench_call_overhead` (post-shape, no VTI) | 127 ns | dispatcher + arg-coerce + JIT id + rebox |
| `bench_ic` (post-shape, no LeJIT-Σ) | 199 ns | dispatcher + arg-coerce + JIT call + **IC GetPropOnObject** + rebox |
| `bench_ic` pre-shape baseline (StubE-EXT 1) | 271 ns | as above plus IndexMap probe in object_get |

Inferred component costs:
- **IC GetPropOnObject dispatch** (extern call + runtime helper + return): 199 − 127 = **72 ns**. This is what LeJIT-Σ targets.
- **Shape fast path absorption** (object_get shape-aware): 271 − 199 = **72 ns saved** vs pre-enrollment. (Coincidence that IC dispatch cost ≈ shape savings; the IC cost on a Dictionary receiver would itself be larger than 72 because it'd include the IndexMap probe.)
- **Dispatcher + arg-coerce + JIT-id-body + rebox**: ~127 ns. The bulk is the Rust dispatcher (`call_function` closure + arg copy + Frame setup, estimated ~120 ns per StubE-EXT 2 §3 decomposition).
- **JIT preamble (arg-coerce)** specifically: ~5-15 ns (the LeJIT-Ψ target). The remainder of the 127 ns is dispatcher + JIT return + rebox, which are outside LeJIT-Ψ's scope.

## 4. Pred-vti.1 falsifier reading

Pred-vti.1 (seed §VIII): inline Number-tag check reduces per-call cost by ≥20 ns on a typed-i64 hot loop.

**Honest pre-implementation budget**: the arg-coerce-specific cost is ~5-15 ns; saving 20 ns from inlining the tag-check alone may be hard. The 20-ns threshold may need re-reading post-VTI-EXT 4 — Pred-vti.1 was set per the LeJIT seed §I.3 estimate of "~1.2-1.4× contribution" from VTI; on the 127 ns baseline that's 25-50 ns saved. Achievable iff VTI's emission also collapses some of the JIT preamble's other costs (not just the tag-check).

**Better falsifier target** for VTI-EXT 6 (proposed): per-iter on `bench_call_overhead` reduces by ≥10 ns. The composition falsifier (Pred-vti.4 from the seed) is the load-bearing test: bench_ic under (shape + LEJIT_STUB + LEJIT_VTI) ≤ 120 ns/iter.

## 5. The dispatcher is the largest single cost

The Rust dispatcher (`call_function`) accounts for ~95% of the 127 ns bench. This is what the **dispatcher refactor** (sibling pilot pre-filed per LeJIT seed §I.2 item 5: tiny-fn fast-baseline) targets. Per LeJIT seed §I.3 composition table, the dispatcher refactor contribution is 1.5-2×, the largest of the four arms.

The substrate-amortization-cascade reading sharpens at this round: **LeJIT-Ψ alone cannot meet the 3× target either; the dispatcher refactor is the remaining required arm.** VTI-EXT 4 measurement will quantify VTI's actual contribution; if below 20 ns, the seed §I.3 composition table needs re-reading.

## 6. Forward to VTI-EXT 2

VTI-EXT 2 designs:
- Discriminant layout reading: how to extract Value's tag from a packed i64 (NaN-box scheme? tagged-pointer? something else cruft uses).
- Inline IR shape: `cmp tag, NUMBER_TAG; b.ne slow_path; extract bits; jump fast_path`. Plus the slow_path fallthrough to the existing extern arg-coerce helper.
- Wire site: the JIT call-prologue's arg-coerce sequence in translator.rs. Identify exactly which CLIF instructions to replace.

The design must determine cruft's Value encoding first — if it's NaN-boxed (f64-based discriminant), the inline check is one comparison; if it's a fat Value enum (tag + payload), the inline check loads the tag byte at a fixed offset.

---

*VTI-EXT 1 closes. Per-call overhead baseline: 127 ns/iter on Pi. Pred-vti.1's 20-ns threshold tight against an arg-coerce-specific cost estimated at 5-15 ns. VTI-EXT 2 designs the inline emission against this constraint.*
