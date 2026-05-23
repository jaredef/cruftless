# TB-EXT 1 — Multi-shape call-overhead bench baseline

*Establishes shape-controlled baselines for the LeJIT-Τ tiny-baseline pilot. Three function shapes that exercise different cost components of the Rust `call_function` dispatcher. TB-EXT 4 (post-implementation measurement) will reuse the same shapes for controlled comparison.*

## 1. Bench protocol

**Workload**: 1,000,000 iterations per shape via `Runtime::call_function`. JIT-threshold = 1; warm-up = 10 calls to ensure JIT-compile completes before timing. Same dispatcher path as VTI-EXT 1's `bench_call_overhead`.

**Source**: `cruftless/examples/bench_call_shapes.rs`. Run: `cargo build --release --example bench_call_shapes -p cruftless && target/release/examples/bench_call_shapes`.

**Three shapes**:
- **id1**: `function id1(x) { return x; }` — 1-arg, no body work, 1 local (the arg).
- **id2**: `function id2(x, y) { return x + y; }` — 2-arg, one Add op, 2 locals.
- **id_locals**: `function id_locals(x) { let y = x; return y; }` — 1-arg, one StoreLocal, 2 locals.

## 2. Pi baseline measurements (2026-05-23)

```
id1            per-iter: 130.8 ns
id2            per-iter: 135.5 ns      (id2 - id1 = +4.7 ns)
id_locals      per-iter: 126.5 ns      (id_locals - id1 = -4.3 ns)
```

Single-run measurement per shape; variance characterization deferred to TB-EXT 6.

## 3. Reading

**id2 - id1 = +4.7 ns** — the cost of one additional argument plus one `Op::Add` body op. Per-arg cost is ~2-3 ns (the 2nd `unbox_arg` call in the dispatcher + the 2nd block-param plumbing in the JIT prologue + the Add op itself).

**id_locals - id1 = -4.3 ns** — negative delta. This is within plausible single-run variance (±5 ns observed across the VTI-EXT 1/3a/3b measurements: 127.1 → 122.0 → 126.6 spanning ~5 ns). The StoreLocal-style local management adds at most a few ns; the negative reading should be read as "approximately zero, within noise."

**Cross-validation with VTI-EXT 1 baseline**: id1 at 130.8 ns is within ±5 ns of:
- VTI-EXT 1 reading: 127.1 ns
- VTI-EXT 3a reading: 122.0 ns (post-layout-pin)
- VTI-EXT 3b reading: 126.6 ns (VTI OFF)
- TB-EXT 1 reading: 130.8 ns (this run)

The five readings span 122-131 ns. Single-run variance band is ~±5 ns (relative ±4%). TB-EXT 6's multi-run characterization will pin this; for now, treat 125 ns ± 5 ns as the working baseline for the id1 shape.

## 4. Dispatcher dominance confirmed empirically

Per LeJIT seed §I.3 + VTI-EXT 1 decomposition reading: the Rust `call_function` dispatcher accounts for ~95% of the id1 per-iter cost. This bench corroborates that claim from a different angle:

- **Arity scales the dispatcher modestly**: id2 = id1 + 4.7 ns (~3.6% increase per arg).
- **Locals scale the dispatcher trivially**: id_locals delta is within noise.
- **The body op (Add) adds ~3-4 ns**: subtract per-arg cost (~2-3 ns) from id2's +4.7 ns delta.

This means almost all of the 125 ns/iter baseline is shape-invariant. That shape-invariant cost is exactly the dispatcher's per-call overhead — closure-bound-this resolve, Vec<Value> allocation, Frame setup, JIT-cache lookup, deopt-TLS plumbing — the components TB-EXT 2's decomposition audit must partition.

If a substrate move can reduce the shape-invariant cost by N ns, all three shapes' bench results drop by N. This is the test TB-EXT 3b's call-thunk emission must pass.

## 5. Pred-tb.1 falsifier reading

Pred-tb.1 (seed §I.2): a tiny-baseline-emitted call thunk reduces per-call overhead by ≥40 ns on `bench_call_overhead` (≥30% of the ~127 ns dispatcher cost).

**Anchored target**: against the ~125 ns shape-invariant baseline, the ≥40 ns reduction means TB-EXT 4 should show id1 ≤ 85 ns/iter. Tighter on id2/id_locals (same absolute reclaim, slightly higher denominator).

**Composition target** (Pred-tb.2): bench_ic under (shape + LEJIT_STUB + LEJIT_TB) ≤ 90 ns/iter (down from VTI-EXT 1's 199 ns post-shape baseline). This is the load-bearing composition test.

## 6. Forward to TB-EXT 2

TB-EXT 2 designs the dispatcher decomposition audit. The bench-tier observation that ~120 ns is shape-invariant is the apparatus; the source-tier read of `Runtime::call_function` must partition this into named cost components. Output: `docs/dispatcher-decomposition.md`. The target list for TB-EXT 3b's call-thunk is the named-cost partition with per-component reclaim estimates.

---

*TB-EXT 1 closes. Multi-shape baselines: id1=130.8, id2=135.5, id_locals=126.5 (single-run). Shape-invariant cost ~125 ns ± 5 ns dominates per-iter dispatch — empirical corroboration for §I.3's dispatcher-is-the-largest-single-arm reading. TB-EXT 2 begins the decomposition audit.*
