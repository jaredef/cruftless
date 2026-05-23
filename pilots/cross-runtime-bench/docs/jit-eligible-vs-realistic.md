# CRB-EXT 9 — JIT-eligible workload reading

*Tests Pred-crb.5 (cruft's relative position improves under JIT-eligible workloads). Adds the `arith_tight_loop` fixture — pure-integer-arithmetic hot loop with no property access, no callback dispatch, no allocation in the inner loop. Cruft's relative position improves by an order of magnitude vs realistic-mixed fixtures.*

## 1. The arith_tight_loop fixture

```js
function sum(n) {
  let s = 0;
  let i = 0;
  while (i < n) {
    s = s + i;
    i = i + 1;
  }
  return s;
}
```

Inner: 1000 iterations per call → triangular number 499500.
Outer: 100000 calls.
Total inner iters: 100M.

**Op set used in `sum`** — all within cruft's JIT translator's supported set:
- `PushI32` (literals 0, 1)
- `LoadLocal` / `StoreLocal` (locals s, i, n)
- `Add` (s + i; i + 1)
- `Lt` (i < n)
- `JumpIfFalse` (loop exit)
- `Return`

After threshold=1, cruft's JIT compiles `sum` and all subsequent calls run JIT-emitted code. No deopt path (no property access, no Object args). The per-call dispatcher cost (~125 ns/call × 100k = ~12.5 ms) amortizes across the 100M-iter inner total; **JIT body cost dominates at ~98% of total wall-clock**.

## 2. Post-EXT-9 unified canonical baseline (N=10, Pi)

| fixture | equality | node (ms) | bun (ms) | cruft (ms) | cruft/node | cruft/bun |
|---|---|---:|---:|---:|---:|---:|
| **arith_tight_loop** (JIT-eligible) | EQUAL | 201.000 | 98.500 | 335.500 | **1.67×** | **3.41×** |
| crypto_sha256_batch | DIFFER | 79.000 | 31.500 | FAIL | — | — |
| string_url_sweep (mixed) | EQUAL | 90.000 | 51.000 | 747.500 | 8.31× | 14.66× |
| json_parse_transform (JSON-dominated) | EQUAL | 121.000 | 93.500 | 2489.500 | 20.57× | 26.63× |

## 3. Pred-crb.5 disposition

**STRONGLY CORROBORATED.**

Cruft's relative position to node spans an order of magnitude across the four fixtures:
- **1.67×** on JIT-eligible pure arithmetic (arith_tight_loop)
- **8.31×** on mixed regex+URL+string (string_url_sweep)
- **20.57×** on JSON-dominated (json_parse_transform)
- **FAIL** on crypto (substrate gap)

The 12× spread (1.67× → 20.57×) is not noise; it is direct empirical evidence that:

1. **LeJIT's JIT produces real per-iter speedup** when it covers the workload. Cruft at 1.67× off node on arith_tight_loop is competitive — within striking distance of the 1× target that LeJIT seed §I.3 names for bench_ic-class workloads.

2. **The realistic-workload gap (8-26×) is dominated by non-JIT components.** This corroborates the §I.3 amendment from CRB-EXT 8 + the cost-component decomposition: JSON.parse hand-coded primitives, Array.filter/map callback dispatch, Object iteration, regex handling — none of which the current JIT pilot accelerates.

## 4. Cruft/bun JIT-eligible reading vs the §I.3 prediction

LeJIT seed §I.3 predicts (bench_ic-class scoped): "matches Bun's per-op cost." The closest CRB analog is arith_tight_loop's cruft/bun = 3.41×.

The 3.41× gap on arith_tight_loop reads relative to §I.3's prediction:

- **§I.3 is bench_ic-anchored**, single-op-repeated workload. The JIT body there is ~5 ops; the dispatcher dominates.
- **arith_tight_loop is multi-op inner loop**, 100M iters total. The JIT body dominates 98%; dispatcher amortizes.

On arith_tight_loop the dispatcher contribution is structurally smaller than bench_ic — meaning the JIT body's quality (Cranelift's lowering of the Add+Lt+Jump loop) IS most of the cost. Cruft at 3.41× off bun on this means **Cranelift's per-iter lowering is ~3.4× slower than bun's** for a tight integer loop. This is real, structural, and the gap LeJIT-Σ/Ψ/Τ targets won't close substantially (dispatcher is only ~2% of the workload).

**Implication**: closing the arith_tight_loop gap to ≤2× off bun requires either:
- A better Cranelift configuration (optimization level, register allocator settings)
- A hand-rolled emitter for tight inner loops (Sparkplug-style — exactly LeJIT-Τ's tiny-baseline scope, but for inner loops not just calls)
- A different JIT backend entirely

None of these are pre-filed pilots; they would be CRB-EXT 9's forward-derived candidate locale spawns if the keeper directs.

## 5. The 12× span as framework property

The four-fixture per-workload spread (1.67× → 20.57× cruft/node) is itself a framework finding. The engagement's standing performance vocabulary now needs to express:

- **Per-workload spread** is normal, not noise — different workloads exercise different substrate cost components
- **JIT-eligible vs JIT-ineligible** is a coarse binary; finer classifications (dispatch-bound vs allocation-bound vs primitive-bound) are useful refinements
- **A single "cruft is N× slower than node" claim is structurally incomplete** — must be qualified per workload class

This is a sharpening of CRB-EXT 8's §I.3 amendment from binary (bench_ic vs CRB) to a spectrum (JIT-eligible vs mixed vs JSON-dominated vs surface-gap). The amendment text holds; the spectrum is a finer-grained reading.

## 6. Forward implications

**For LeJIT pilots in flight**:
- **LeJIT-Σ** (stub-emitter) targets IC dispatch — relevant to mixed workloads (string_url_sweep partial benefit; json_parse_transform some) but not arith_tight_loop (no property access).
- **LeJIT-Ψ** (value-tag-inline) targets arg-coerce — relevant to dispatcher-dominated workloads (bench_ic, bench_call_overhead) but minimal benefit at arith_tight_loop where dispatch is <2% of cost.
- **LeJIT-Τ** (tiny-baseline) targets dispatcher — relevant to bench_ic + bench_call_overhead, minimal benefit at arith_tight_loop. Its strongest CRB benefit is on mixed-call workloads (Array.filter/map callbacks).

**For new pilots not yet spawned**:
- A **"fast JSON" pilot** would close ~5-10× of the json_parse_transform gap. ~Reservation-class size; bigger than any current LeJIT pilot.
- A **"tight inner loop emitter"** (Cranelift replacement for hot loops) would close ~2-3× of the arith_tight_loop gap. Substantial; Sparkplug-style hand-rolled.
- An **Array.filter/map fast-path** (recognize the callback shape at JIT-compile time, inline it) would close 2-3× across mixed workloads.

These three together would plausibly move cruft from 14-26× off bun on realistic workloads to ~3-5× off bun — competitive territory. Multi-month scope; not on the engagement's current roadmap but the empirical reading names the work.

## 7. Note on the §I.3 amendment

The CRB-EXT 8 amendment text holds without modification. The arith_tight_loop reading adds empirical specificity to the amendment's "5-15× off bun" forward expectation: that range was based on LeJIT first-cut composed; with arith_tight_loop now showing 3.41× off bun on the JIT-eligible end, the range can be refined as **"3-15× off bun spectrum depending on workload class, with arith-bound at the lower end and JSON-bound at the upper end."** Not a corpus amendment; a per-locale reading refinement.

---

*CRB-EXT 9 closes. Pred-crb.5 strongly corroborated. Cruft's per-workload spread is 12× (1.67-20.57× off node), proving LeJIT's JIT contribution is real on its eligible workloads and the realistic-mixed gap is dominated by non-JIT substrate components. The §I.3 amendment from CRB-EXT 8 holds; the spectrum reading sharpens it.*
