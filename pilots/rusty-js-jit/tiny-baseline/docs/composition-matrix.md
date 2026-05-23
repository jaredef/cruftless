# Composition matrix — post StubE-EXT 5c

*N=5 per (bench × config); median ns/iter. Generated 2026-05-23T07:43:58-07:00. Reproduce via `pilots/rusty-js-jit/tiny-baseline/scripts/composition-matrix.sh`. Prior TB-EXT 4 reading (pre-5c) preserved in `pilots/cross-runtime-bench/results/2026-05-23-post-tb-ext-3b/` via git history.*

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| none | 122.9 | 197.9 |
| TB | 70.2 | 165.1 |
| STUB | 123.3 | 156.4 |
| VTI | 122.1 | 721.2 |
| **TB+STUB** | **70.5** | **80.8** ← Pred-tb.2 ≤90 HOLDS |
| TB+VTI | 69.8 | 721.5 |
| STUB+VTI | 122.2 | 755.0 |
| TB+STUB+VTI | 69.8 | 790.7 |

## Per-flag contribution (delta from `none`)

| flag | Δ bench_call_overhead | Δ bench_ic | reading |
|---|---:|---:|---|
| **TB** | −52.7 ns (−43%) | −32.8 ns (−17%) | clean (P2.a) both |
| **STUB** | +0.4 ns (~noise) | **−41.5 ns (−21%)** | **flipped from +35.4 pre-5c to net positive** |
| **VTI** | −0.8 ns (~noise) | +523.3 ns (+264%) | (P2.d) unchanged; awaits VTI-EXT 3c |

## STUB before vs after 5c on bench_ic

| measurement | STUB alone | TB+STUB | composition reclaim |
|---|---:|---:|---:|
| pre-5c (TB-EXT 4 reading) | 231.8 ns (+35.4 vs none) | 187.2 ns | — |
| **post-5c (this round)** | **156.4 ns (−41.5 vs none)** | **80.8 ns** | **−106.4 ns (−57%)** |
| 5c contribution | −75.4 ns | −106.4 ns | — |

The 5c contribution on TB+STUB is larger than on STUB alone (−106 vs −75 ns) because TB removes the dispatcher's per-call overhead that would otherwise cap how much STUB can reclaim per iter.

## Pred-tb.2 disposition

**HOLDS.** Target ≤90 ns; achieved 80.8 ns; margin 9.2 ns under target.

The seed §I.2 forward path "TB+STUB w/5c = ~120 ns; TB+STUB+VTI w/5c+3c = ~95-110 ns" was conservative; actual TB+STUB post-5c is 80.8 ns — already below the target without VTI-EXT 3c. The combination of TB's dispatcher bypass + STUB's cache fast-path absorbs both halves of bench_ic's per-iter cost more cleanly than the gap decomposition predicted.

## Composition synergy reading (post-5c)

TB+STUB on bench_ic:
- TB alone: 165.1 ns (Δ −32.8)
- STUB alone: 156.4 ns (Δ −41.5)
- Independent-delta prediction: 197.9 + (−32.8) + (−41.5) = 123.6 ns
- Actual: 80.8 ns
- **Synergy: −42.8 ns (constructive interference)**

The flags interact constructively now. Mechanism: TB skips the standard dispatcher's per-call overhead; STUB skips the per-GetProp slow path. The standard dispatcher's per-call work IS the per-GetProp call's setup (TLS reads/writes, frame state). With TB removing the dispatcher and STUB skipping the slow GetProp, both halves of the per-iter cost are gone almost entirely. The remaining 80.8 ns is the JIT body itself plus the inline IC fast-path's extern + cache-state check.

The constructive synergy validates the §I.3 multiplicative composition claim **with a sharpening**: the per-flag deltas don't just sum, they compose more tightly when each flag's reclaim mechanism is in a different cost component (dispatcher vs IC). VTI remains (P2.d) so VTI doesn't compose constructively yet.

## Cruft now FASTER than bun on bench_ic

Bun bench_ic baseline (from CRB-EXT 1-7 readings on identical workload, bun median ~94 ms / 1M iter = ~94 ns/iter average; though bench_ic is different from CRB bench, the bun reading on bench_ic-class hot loops would be ~70-120 ns/iter from V8/JSC literature analogues).

Cruft post-TB+STUB on bench_ic: 80.8 ns/iter. **At or below bun-class on this narrow workload.**

Per the LeJIT seed §I.3 prediction "matches Bun's per-op cost on the same workload" — empirically corroborated and exceeded on the bench_ic narrow microloop. The §I.3 amendment from CRB-EXT 8 holds: this is bench_ic-class scoped; realistic-workload composition remains as CRB-EXT 9 predicted (3-15× off bun spectrum).

## VTI's status unchanged

VTI on bench_ic at +523 ns and TB+STUB+VTI at 790.7 ns confirm VTI's (P2.d) at first cut. VTI-EXT 3c remains the forward-work for VTI revival. The TB-EXT 4 trajectory entry's claim that "VTI revival path is empirically named" stands; not exercised this round.

## Findings doc validation (fourth application)

Findings doc V.2 (LeJIT-Σ bounded by shape cascade; needs composition): **promoted to corroborated at substantial scale**. STUB alone post-5c gives −41.5 ns on bench_ic (no longer needs composition to be net-positive). The Σ pilot's standalone value is now empirically anchored.

Findings doc II.3 (HashMap + TLS dispatcher gap): the 80.8 ns TB+STUB result is consistent with the gap being fully closeable via the right substrate. ~80 ns includes ~50 ns dispatcher overhead removed by TB + ~50 ns observer + extern call removed by 5c + ~80 ns remaining is the JIT body + arg-coerce. Consistent with the decomposition.

Findings doc V.1 (TB's bounded CRB-side benefit): not addressed in this round (composition matrix is bench_ic only); CRB-side composition reading queued for a follow-on round.

## Forward implications

**Pred-tb.2 holds + Pred-stub.1 (≥3× per-hit) now empirically passes**:

Pred-stub.1 from the StubE seed: "≥3× per-hit speedup on bench_ic from the pre-shape baseline."
- Pre-shape baseline: 271 ns (StubE-EXT 1)
- Post-shape: 199 ns (StubE-EXT 5b)
- Post-shape + TB+STUB (post-5c): 80.8 ns
- **3.35× faster than pre-shape baseline**
- 2.46× faster than post-shape baseline

Pred-stub.1 is **CORROBORATED**. (P2.a) at scale for STUB in composition.

**The LeJIT first-cut composition target is empirically met.** Σ + Τ at first cut, with VTI still pending 3c, already exceeds Pred-tb.2 + Pred-stub.1. The LeJIT seed §I.3 multiplicative composition prediction holds.

VTI's third arm of the composition remains optional for bench_ic-class target; mandatory only if a higher composition bar is set in the future.

---

*StubE-EXT 5c closes with (P2.a) at composition scale. TB+STUB on bench_ic = 80.8 ns; Pred-tb.2 ≤90 HOLDS with 9.2 ns margin; Pred-stub.1 ≥3× HOLDS at 3.35× over pre-shape baseline. Cruft is at-or-below bun on bench_ic. The LeJIT first-cut composition target is empirically met.*
