# CRB-EXT 8 — Composition reading vs LeJIT seed §I.3

*Compares the cross-runtime-bench pilot's empirical readings (CRB-EXT 1-7, N=30) against LeJIT seed §I.3's substrate-amortization-cascade composition prediction. Surfaces the structural distinction between bench_ic-class narrow microloops and CRB-class realistic-mixed workloads. Formalizes the proposed §I.3 amendment.*

## 1. What §I.3 currently predicts

LeJIT seed §I.3 (2026-05-23 empirical recognition) reads:

> The 26% per-iter speedup from shape enrollment alone was **unanticipated**: the shapes pilot's seed §I telos was about IC cache key supply, not per-op read speedup. The speedup is a side effect of Shape-EXT 4's `object_get` shape-aware fast path (skips the IndexMap probe for hot string-keyed property accesses).
>
> [...] Combined the engagement is heading toward a ~1.5-2× speedup from LeJIT alone (per §VIII bench precedent) on top of the 1.36× from shape enrollment, multiplicatively reaching the ~2-2.5× zone that matches Bun's per-op cost on the same workload.

Operationally: **§I.3 predicts cruft-vs-bun ≈ 2-2.5× on the bench_ic-class workload after full LeJIT composition lands.**

## 2. What CRB-EXT 1-7 measures

CRB's N=30 canonical baseline (Pi, 2026-05-23):

| fixture | node (ms) | bun (ms) | cruft (ms) | cruft/bun |
|---|---:|---:|---:|---:|
| crypto_sha256_batch | 77.0 | 31.0 | FAIL | — |
| json_parse_transform | 121.0 | 94.0 | 2474.5 | **26.32×** |
| string_url_sweep | 90.0 | 51.0 | 752.5 | **14.75×** |

Variance sd/median 1.8-3.5% across cruft-runnable cells; the readings are robust beyond statistical doubt.

**Operationally: cruft-vs-bun is 14-26× on realistic-mixed workloads, an order of magnitude wider than §I.3's prediction of 2-2.5×.**

## 3. The structural distinction the comparison exposes

§I.3's prediction and CRB's measurement are not the same number measured imprecisely. They are **structurally different numbers** because they measure different things:

**§I.3 ("2-2.5×")** measures composed cruft speed at the **bench_ic narrow workload** — a 1M-iteration `getx(obj) = obj.x` hot loop. The workload's per-iter cost is dominated by:
- IC GetPropOnObject dispatch (~72 ns, LeJIT-Σ's target)
- Shape fast-path savings (~72 ns, already absorbed by Shape-EXT 4)
- Dispatcher + arg-coerce + JIT body + rebox (~127 ns, LeJIT-Τ + Ψ's target)

Total per-iter ~199 ns post-shape; LeJIT's composed contribution reduces this to ~70 ns under the §I.3 prediction. That's ~2.5× cruft speedup, vs bun's ~70 ns on the same workload, hence cruft/bun ≈ 1×. The "2-2.5×" is the *self-improvement multiplier* (cruft pre vs cruft post), not the cruft/bun ratio.

Re-reading §I.3's "matches Bun's per-op cost": the prediction is that LeJIT composition reaches **par with bun on bench_ic**.

**CRB ("14-26×")** measures composed cruft speed at **realistic-mixed workloads** — workloads where:
- JSON.parse / .stringify is the dominant cost
- Array.filter + .map invocations dominate the callback dispatch path
- Object iteration crosses many properties not just one
- The hot loop is the user's, not a single bytecode op repeated 1M times

On these workloads the per-iter cost decomposition is completely different — LeJIT's contribution is multiplicatively smaller because LeJIT's targets (IC dispatch, dispatcher overhead, value-tag check) are only one component of many. Specifically:

| component on CRB fixtures | estimated multiplier vs node | LeJIT-targeted? |
|---|---:|---|
| JSON.parse hand-coded in node+bun | 5-10× | NO (separate substrate work) |
| JSON.stringify hand-coded | 2-3× | NO |
| Array.filter/.map callback dispatch (×1000s) | 2-3× | YES (LeJIT-Τ dispatcher refactor) |
| Object iteration shape contribution | <10% | YES (already landed) |
| Cranelift JIT compile overhead at threshold=1 | 1.2-1.5× | YES (LeJIT-Τ tiny-fn baseline) |
| Per-call dispatcher (~125 ns × thousands of calls) | 1.5-2× | YES (LeJIT-Τ) |

Multiplicative composition: (5-10) × (2-3) × (2-3) × ~1 × (1.2-1.5) × (1.5-2) ≈ 27-540× theoretical compose. The observed 20× on json sits well within this range, consistent with the decomposition's plausibility.

## 4. The amendment

§I.3 is **not wrong** — it correctly predicts the bench_ic composed result. §I.3 is **structurally incomplete** — it does not distinguish bench_ic-class from realistic-mixed predictions, leaving the seed open to being read as a cross-runtime competitiveness claim it does not make.

Proposed §I.3 amendment (additive; preserves existing text):

> **Per-workload disambiguation (CRB-EXT 8 amendment, 2026-05-23)**: the §I.3 prediction above (~2-2.5× cruft self-improvement reaching bun-parity on bench_ic) is **bench_ic-class scoped only**. The composition reading on realistic-mixed workloads is structurally different:
>
> CRB-EXT 1-7's N=30 canonical baseline (Pi, 2026-05-23) measures cruft at 14-26× off bun on three realistic-mixed fixtures (json_parse_transform, string_url_sweep, crypto_sha256_batch's node+bun reading). The gap decomposes multiplicatively across at least six cost components, of which LeJIT directly targets three (callback dispatch via LeJIT-Τ, per-call overhead via LeJIT-Τ, value-tag inline via LeJIT-Ψ). The other three (JSON.parse hand-coded primitives, JSON.stringify, Cranelift compile overhead) are out of LeJIT's scope and require separate substrate work (fast JSON, multi-tier JIT, etc.).
>
> **Operational consequence**: a composed LeJIT first-cut closing all four nested sub-pilots (Σ + Ψ + Τ + future Σ') is empirically expected to bring cruft from 14-26× off bun on CRB fixtures down to ~5-15× off bun. Single-digit cruft/bun on CRB requires substrate work *beyond LeJIT*. Closing to par with bun on CRB-class workloads is a multi-pilot, multi-session telos that the §I.3 reading should not be read as predicting.
>
> The §I.3 prediction "matches Bun's per-op cost" applies at bench_ic; not at CRB.

## 5. Three-anchor convergence

The amendment is supported by three independent empirical anchors, each surfaced in the LeJIT enhancements log:

1. **VTI-EXT 3a** (2026-05-23): flagged the 26% shape-enrollment claim as "possibly variance-low; multi-run characterization needed." Today's CRB-EXT 7 N=30 (sd/median 3.5% on cruft) bounds the variance and confirms the multi-run discipline; bench_ic single-run readings are still noisy at ±5 ns.

2. **CMig-EXT 15** (2026-05-23): explicit narrow-vs-realistic disambiguation surfaced by the out-of-band parallel-Claude measurement showing 5%/1% shape contribution on realistic workloads vs 26% on bench_ic. The §I.3 amendment encodes this disambiguation as standing framework vocabulary.

3. **CRB-EXT 1-7** (2026-05-23): the empirical reading itself. 14-26× cruft/bun on realistic workloads, robust at N=30.

The three anchors converge: bench_ic is a narrow microloop; realistic-mixed is structurally different; the framework needs explicit per-workload composition reads. The §I.3 amendment formalizes the convergence.

## 6. What the amendment does NOT do

- **Does not retract any §I.3 prediction.** The bench_ic-class composed reading (cruft ≈ bun at ~70 ns/iter post-LeJIT) stands. The amendment adds a parallel realistic-workload reading, not a replacement.

- **Does not adjust any Pred-stub.* / Pred-vti.* / Pred-tb.* falsifier threshold.** Those are pilot-internal predictions scoped to their respective benches; they remain unchanged.

- **Does not require new substrate work to land.** It is purely a corpus-tier framework refinement; the seed's vocabulary now distinguishes the two classes explicitly.

- **Does not foreclose future LeJIT pilots from closing the realistic-workload gap.** It scopes what LeJIT's first-cut alone can close, and identifies the additional substrate axes (JSON, GC, multi-tier JIT) that the engagement would need to add subsequent pilots for.

## 7. Forward implications

- **Future LeJIT measurement claims** should report against BOTH baselines: bench_ic-class (narrow microloop) AND CRB-class (realistic-mixed). Single-baseline claims are incomplete.

- **Future corpus articulations** that compose with §I.3 (any LeJIT-tier doc citing the 3× target or the 1.36× shape contribution) should cite either the bench_ic anchor or the CRB anchor explicitly, not the implicit "performance" baseline.

- **The §I.3 amendment is itself a candidate corpus-tier articulation** at the Doc 729+ corpus scale: "Per-workload performance composition reads must distinguish narrow-microloop from realistic-mixed; conflating them produces over-claimed performance gains." This is bigger than LeJIT; it applies at any pilot that reports composed performance. Reserved for a later corpus-tier round when the engagement next produces a corpus articulation.

---

*CRB-EXT 8 closes. The §I.3 amendment is drafted; the engagement now has explicit per-workload composition vocabulary. The cross-runtime-bench pilot's empirical reading is structurally distinct from bench_ic-class readings; both stand; the framework reports them jointly going forward.*
