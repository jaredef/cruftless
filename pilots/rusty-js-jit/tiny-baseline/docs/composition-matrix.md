# TB-EXT 4 — composition matrix

*N=5 per (bench × config); median ns/iter. Generated 2026-05-23T07:31:39-07:00. Reproduce via `pilots/rusty-js-jit/tiny-baseline/scripts/composition-matrix.sh`.*

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| none | 123.2 | 196.4 |
| TB | 71.1 | 152.8 |
| STUB | 125.2 | 231.8 |
| VTI | 122.2 | 758.5 |
| TB+STUB | 70.8 | 187.2 |
| TB+VTI | 70.1 | 725.7 |
| STUB+VTI | 122.1 | 743.3 |
| TB+STUB+VTI | 71.4 | 743.7 |

## Per-flag contribution (delta from `none`)

| flag | Δ bench_call_overhead | Δ bench_ic | reading |
|---|---:|---:|---|
| **TB** | **−52.1 ns (−42%)** | **−43.6 ns (−22%)** | clean (P2.a) on both benches |
| **STUB** | +2.0 ns (+1.6%) | +35.4 ns (+18%) | observer overhead; awaits StubE-EXT 5c inline |
| **VTI** | −1.0 ns (~noise) | +562.1 ns (+286%) | (P2.d) regression compounds on IC-heavy workload |

## Composition synergy reading

TB+STUB on bench_ic:
- TB alone: 152.8 ns (Δ −43.6)
- STUB alone: 231.8 ns (Δ +35.4)
- Independent-delta prediction: 196.4 + (−43.6) + (+35.4) = 188.2 ns
- Actual: 187.2 ns
- **Synergy: +1.0 ns (additive within noise)**

The §I.3 multiplicative composition reading holds at first cut at the level of per-flag combinations: no interaction surprises. The flags compose roughly additively (in the linear-delta sense — multiplicatively in the ratio sense at low-percentage changes).

TB+VTI on bench_call_overhead = 70.1 ns ≈ TB alone (71.1 ns). VTI's small bench_call_overhead overhead disappears when TB fast-paths around the standard dispatcher's unbox_arg entirely. **The closure-side metadata cache absorbs VTI's first-cut regression on this bench**: TB's fast path never reaches the dispatcher's `match params` arm where VTI's pointer-pass lives. Useful structural insight.

## Pred-tb.2 disposition

Pred-tb.2 (seed §I.2): bench_ic under (shape + LEJIT_STUB + LEJIT_TB) ≤ 90 ns/iter.

**FALSIFIED at first cut composition.** TB+STUB = 187.2 ns vs 90 ns target. Gap = 97 ns.

**Decomposition of the gap**:
- STUB's observer overhead (+35.4 ns of the gap): awaits StubE-EXT 5c inline emission. Per StubE-EXT 5b's reading, the +38 ns observer is replaced by ~5 ns inline check; net expected reclaim ~33 ns.
- Remaining ~62 ns of gap: requires StubE-EXT 5c's actual IC fast-path inline emission to absorb ~50-60 ns of the per-GetProp extern call.

**Forward path to Pred-tb.2 holding**:
- TB+STUB (with StubE-EXT 5c inline): predicted ~120 ns
- TB+STUB+VTI (with VTI-EXT 3c precheck-removal + StubE-EXT 5c inline): predicted ~95-110 ns
- Approaches Pred-tb.2's 90 ns target; **reachable with the remaining first-cut substrate work** (StubE-EXT 5c + VTI-EXT 3c).

Pred-tb.2 is **reachable in principle** but requires both StubE-EXT 5c and VTI-EXT 3c. Current TB+STUB+VTI = 743.7 ns demonstrates that VTI's (P2.d) state DOMINATES the composition until VTI-EXT 3c lands.

## VTI's bench_ic regression mechanism

VTI on bench_ic at +562 ns is much worse than on bench_call_overhead (-1 ns). Mechanism: bench_ic's inner loop calls a function that does GetPropOnObject, which goes through the JIT-emitted extern call. The extern path passes args as VTI-style `*const Value` under VTI=1. The runtime helper (`runtime_getprop_on_object` or `jit_getprop_with_ic`) re-loads from the pointer, paying the load cost on each access. Compounded over 1M iterations of multiple GetProp dispatches per iter, the extra-load cost dominates.

**This is informative**: VTI's (P2.d) costliness scales with how often the function calls through the JIT-emitted extern boundary. bench_call_overhead has 0 GetProps; bench_ic has many. Real workloads with high property access (json_parse_transform, string_url_sweep) would see proportional VTI regressions — exactly why VTI is default-OFF and exactly why VTI-EXT 3c (which removes the precheck so the calling-convention switch pays) is load-bearing.

## Implication for the seed §I.3 amendment

This round adds a fourth empirical anchor to CRB-EXT 8's amendment (3 prior: VTI-EXT 3a variance reservation, CMig-EXT 15 narrow-vs-realistic split, CRB-EXT 1-7 realistic baseline).

The amendment's prediction "LeJIT first-cut composed expected to close 14-26× to ~5-15× off bun" reads against this composition matrix:
- **bench_ic class**: TB alone gives 152.8 ns (vs ~131 ns bun on identical workload — that's ~1.17× off bun, well within the ≤2× zone). Bench_ic class composition target IS within reach.
- **CRB class**: TB contribution there is ~2% (per the TB-EXT 3b CRB cruft TB=1 reading); doesn't close the realistic-workload gap. Spectrum reading from CRB-EXT 9 (3-15× off bun) stands.

The amendment text holds without modification.

## Implication for the VTI pilot revival path

VTI's (P2.d) state at first cut is dominantly responsible for the composition's failure to reach Pred-tb.2. The seed §I.2 falsifier for VTI named the (P2.d) risk; today's composition matrix EMPIRICALLY DEMONSTRATES it across all bench_ic + ic-included configurations. The forward path to revival:

- **VTI-EXT 3c** (inline tag-check + dispatcher precheck-skip) must land for VTI to move out of (P2.d). Per the TB-EXT 3b enhancements log entry: "VTI-EXT 3c viability improves" — TB-EXT 3b's success demonstrates that calling-convention restructuring CAN pay when done right.
- The TB+VTI bench_call_overhead reading (70.1 ns ≈ TB alone) shows that when TB's fast path bypasses the standard dispatcher entirely, VTI's first-cut overhead doesn't appear. This is structural evidence that VTI's regression is path-dependent, not intrinsic.
- After VTI-EXT 3c: re-measure composition matrix with all 8 configs. Expected: VTI becomes net-positive (small) on bench_ic; STUB+VTI combo also stops compounding.

VTI is not deprecated by this round; its forward work is empirically named.

## Findings doc validation (third application)

Findings doc V.2 (LeJIT-Σ is bounded by shape-enrollment cascade; needs composition): confirmed. STUB alone (+18% bench_ic) shows the observer cost cannot be offset by STUB alone; the inline emission (StubE-EXT 5c) is the closure round.

Findings doc V.3 (LeJIT-Ψ is (P2.d) at first cut; structural lesson is current value): empirically anchored. VTI on bench_ic at +562 ns is the (P2.d) at scale; the lesson (Finding II.2: never split substrate moves) generated the staged-validation discipline that produced TB-EXT 3b's win.

## Forward to TB-EXT 5+

TB-EXT 5 (consumer-route probe) already implicitly satisfied via TB-EXT 3b's diff-prod 42/42 PASS under TB=1.

TB-EXT 6 (variance characterization) should re-run the composition matrix at N=20+ to bound the variance band on each cell, particularly for the high-VTI cells where outliers might shift the median.

TB-EXT 7 (fuzz probe) and TB-EXT 8 (default-on flip) gate on TB-EXT 6.

The pilot's first-cut composition reading is complete. The composition target Pred-tb.2 is **falsified but reachable**; the empirical anchor for the §I.3 amendment is solidified; VTI's revival path is empirically named.

---

*TB-EXT 4 closes. Composition matrix produced. TB alone delivers (P2.a) on both benches; STUB awaits 5c; VTI is (P2.d) in bench_ic until 3c lands. Pred-tb.2 falsified at first cut (187.2 ns vs 90 ns) but decomposed gap shows the path to holding it via StubE-EXT 5c + VTI-EXT 3c.*
