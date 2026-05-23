# LeJIT Findings — Substrate-Improvement Guidance

*Synthesized learnings from the 2026-05-23 LeJIT-tier session (StubE-EXT 5b through CRB-EXT 9). Distinct from `enhancements.md` (append-only event log) and `trajectory.md` (per-pilot rung log): this document organizes findings by what they implicate for future substrate work. The keeper's 2026-05-23 14:09-local directive: "use these to improve substrate."*

Each finding pairs (a) the empirical anchor (with pointer to enhancements.md and measurement number) with (b) the actionable substrate-improvement implication.

---

## I. Performance composition findings

### Finding I.1 — The 12× per-workload spread is structural, not noise

**Anchor**: CRB-EXT 9 reading (post-EXT-9 unified canonical baseline, N=10, Pi).

| fixture | cruft/bun |
|---|---:|
| arith_tight_loop (JIT-eligible) | 3.41× |
| string_url_sweep (mixed) | 14.66× |
| json_parse_transform (JSON-dominated) | 26.63× |

The 12× spread across workloads reveals four cost regimes cruft inhabits — JIT-eligible, mixed, JSON-dominated, surface-gap (crypto FAIL).

**Substrate implication**: future LeJIT pilot priority decisions must read CRB per-workload, not bench_ic alone. The §I.3 amendment's "report against BOTH baselines" is operative. Single-baseline claims understate or overstate per the workload class.

### Finding I.2 — Cranelift's per-iter lowering is ~3.4× slower than bun on tight inner loops

**Anchor**: CRB-EXT 9 arith_tight_loop reading. cruft 335.5 ms vs bun 98.5 ms. JIT body dominates ~98% of total wall-clock; dispatcher is <2%.

**Substrate implication**: **LeJIT-Σ / LeJIT-Ψ / LeJIT-Τ pilots will NOT substantially close this 3.4× gap.** They target dispatcher and IC dispatch; neither is the dominant cost on tight inner loops. Closing requires:
- Cranelift optimization-level tuning (cheapest first move)
- Hand-rolled tight-inner-loop emitter (Sparkplug variant for loops, not calls) — would be a new LeJIT-tier sibling pilot
- Different JIT backend entirely

None pre-filed. The candidate sibling pilot ("LeJIT-Λ" — tight-inner-loop emitter) is a forward-derived spawn candidate.

### Finding I.3 — The realistic-workload gap is dominated by non-JIT components

**Anchor**: CRB-EXT 8 §I.3 amendment decomposition. CMig-EXT 15 narrow-vs-realistic split. VTI-EXT 3a variance reservation.

Estimated per-component multipliers on json_parse_transform's 20× cruft/node gap:
- JSON.parse hand-coded in node+bun: 5-10× — NOT LeJIT-targeted
- JSON.stringify hand-coded: 2-3× — NOT LeJIT-targeted
- Array.filter/.map callback dispatch (×thousands): 2-3× — LeJIT-Τ partial
- Object iteration shape contribution: <10% — already landed
- Cranelift compile overhead at threshold=1: 1.2-1.5× — LeJIT-Τ partial
- Per-call dispatcher (~125 ns × thousands of calls): 1.5-2× — LeJIT-Τ direct

**Substrate implication**: half the gap is structurally outside LeJIT's scope. Closing the realistic-workload gap to single digits requires NEW pilots:
- **Fast JSON parse/stringify** would close ~5-10× of json gap. Candidate `pilots/rusty-js-json-fast/`.
- **Array.filter/map fast-path** (recognize callback shape at JIT-compile, inline it) would close 2-3× across mixed. Candidate `pilots/rusty-js-array-fast/`.
- **Multi-tier JIT** to amortize Cranelift compile cost. Candidate `pilots/rusty-js-jit-tier2/`.

LeJIT's first-cut composed (Σ + Ψ + Τ + future Σ' for x86_64) is empirically expected to close 14-26× → ~5-15× off bun. Closing further is multi-pilot, multi-session.

---

## II. Substrate-amortization-cascade findings

### Finding II.1 — The Doc 729 §A8.13 cascade pattern is NOT universal

**Anchor**: shape-enrollment cascade gave 26% per-iter speedup at Shape-EXT 4 (unanticipated). VTI-EXT 3a layout-pinning gave 5 ns reclaim (~4%, possibly variance-low at single-run). VTI-EXT 3b's closure round produced **+18.9 ns regression** (P2.d).

**Substrate implication**: substrate-introduction rounds CAN cascade per-iter savings, but the cascade is workload-dependent and not guaranteed. Specifically:
- Shape enrollment cascaded because `object_get`'s shape-aware fast-path is the hot-path for bench_ic; the cascade target was structurally present.
- VTI layout-pinning's cascade was small (or noise) because the layout was already adequate; pinning didn't unlock a hot-path optimization the compiler couldn't see.

Future substrate-introduction rounds should NOT assume cascade. Predict reclaim explicitly with named hot-path mechanism, not generic "cascade will happen." When cascade does happen, log it verbosely (per the enhancements log discipline) and update the seed §I.3 with the new anchor.

### Finding II.2 — VTI-EXT 3b's (P2.d) is the canonical anti-pattern: restructuring calling conventions without removing precondition checks

**Anchor**: VTI-EXT 3b enhancements log entry. Predicted 5-10 ns reclaim; measured +18.9 ns regression. Five hypothesized mechanisms; load-bearing one: dispatcher's `jit_compatible_arg` precheck stayed in place while the JIT prologue's load cost was added on top.

**Substrate implication**: **a substrate move that pushes work into a new tier WITHOUT removing the equivalent work from the originating tier is structurally guaranteed to regress.** The full-VTI design (3c) would remove the precheck via inline tag-check; that move was the load-bearing one all along. The 3b implementation as a stepping stone failed because it added cost without removing cost.

Future substrate moves must explicitly identify what they ELIMINATE, not just what they ADD. If a planned move adds work and the elimination half is deferred, the move SHOULD NOT BE LANDED behind a flag — it's worse than nothing. Better: gate the round entirely until both halves are ready.

### Finding II.3 — The dispatcher decomposition has a ~60-86 ns unidentified gap

**Anchor**: TB-EXT 2 decomposition audit. 22 named components sum to ~40-65 ns; measured ~125 ns shape-invariant cost; gap of ~60-86 ns.

Hypothesized gap mechanisms (priority order):
1. HashMap lookups: 20-30 ns total (std SipHash-13, two per call)
2. TLS slot access on aarch64 Linux: 30-60 ns (6 accesses per call)
3. Cache-miss memory traffic across 8+ distributed reads: 20-40 ns
4. Branch mispredict on 5-condition AND: 5-10 ns

**Substrate implication**: TB-EXT 3b targeting closure-side metadata cache (approach A from TB-EXT 3b scope analysis) directly addresses (1) — eliminating the HashMap absorbs ~20-30 ns. TB-EXT 3c approach B addresses (2) — restructured deopt to arg-passing eliminates TLS — for another ~20-40 ns. The two together close ~40-70 ns of the gap, exactly Pred-tb.1's threshold.

Forward: TB-EXT 6 micro-profiling round is now load-bearing for verifying these hypotheses empirically. Without 6, the gap reading stays hypothesis-tier.

---

## III. Measurement methodology findings

### Finding III.1 — Single-run drift ~5%; multi-run medians stabilize to ~1%

**Anchor**: VTI-EXT 1/3a/3b + TB-EXT 1 (five single-run id1 readings: 122-131 ns, ±5 ns). CRB-EXT 7 N=30 vs CRB-EXT 1-6 N=10 medians drift ≤1.5%.

**Substrate implication**: **single-run measurement readings are noise. The framework's standing rule**: all LeJIT measurement claims run ≥5 runs and report median; single-run readings document the workload but are not load-bearing for claims.

Update implication for past claims:
- VTI-EXT 3a's "5 ns reclaim from layout pinning" was single-run. Cannot claim until multi-run (TB-EXT 6 queued).
- StubE-EXT 5b's "26% per-iter speedup" was bench_ic single-run. Should re-run N≥10 before composing with downstream readings.

### Finding III.2 — CRB's bench measurement quality detects ≥7% wall-clock changes at N=30

**Anchor**: CRB-EXT 7 variance characterization. sd/median 3.4% on cruft's worst case (string_url_sweep); detectability at 2 stddev = 6.8% ≈ 7%.

**Substrate implication**: future LeJIT pilots can validate substrate moves against CRB and claim **≥7% wall-clock improvements with empirical confidence at N=30**. Claims below 7% need higher N (N=100+ for ≥3% detectability). This sharpens the framework's measurement budget for future TB-EXT 4 / Σ-EXT 6 / Ψ-EXT 6 measurements.

Specifically: TB-EXT 4's expected reclaim of 38-74 ns on the ~125 ns baseline = 30-60% of bench_call_overhead. Easily detectable at N=10. But TB-EXT 4's reclaim on CRB-class workloads will be smaller in relative terms; needs higher N to claim CRB-side improvements.

### Finding III.3 — Cross-validation across multiple measurements builds confidence the variance reading alone cannot

**Anchor**: id1 measured at 127.1 (VTI-EXT 1), 122.0 (VTI-EXT 3a), 126.6 (VTI-EXT 3b VTI-OFF), 130.8 (TB-EXT 1) — five single-run measurements spanning 122-131 ns. The 5 ns range bounds the variance band more honestly than any one single-run reading.

**Substrate implication**: when claiming reclaim from a single substrate move, **also run the unchanged baseline path on the new build** — captures any measurement drift from build-environment changes. The five-measurement cross-validation pattern across this session is the discipline; future rounds should preserve it.

---

## IV. Probe-coverage findings

### Finding IV.1 — diff-prod + test262-sample alone is structurally insufficient for shape-enrollment correctness

**Anchor**: CMig-EXT 15 entry. CMig-EXT 14 flipped shape-on default with diff-prod 42/42 + test262 within 0.1pp; spread regression escaped through the probe gap and was surfaced by an out-of-band parallel-Claude measurement.

**Substrate implication**: the §X.h.c three-probe-levels discipline (bench + consumer-route + fuzz) must be FULLY implemented for any default-on flip of substrate behavior. CMig has consumer-route (diff-prod) but no fuzz; a property-shape-mutation fuzz harness (CMig-EXT 17 queued) is load-bearing for the structural-completeness gate.

Forward: any future default-on flip of substrate behavior (e.g., LeJIT-Σ enrollment, LeJIT-Τ thunk dispatch) must precede with a fuzz probe in addition to bench + consumer-route.

### Finding IV.2 — Surface-completeness audit is a missing engagement discipline

**Anchor**: CMig-EXT 15 fix identified ONE site (`__object_spread`) iterating `.properties.iter()` directly. The CMig-EXT 14 gate did not enumerate all such sites; the audit step was assumed-done by consumer-route probe coverage. Other sites likely exist:
- JSON.stringify (audit needed)
- Iteration via @@iterator on plain objects (audit needed)
- The remaining 4-5 long-tail test262 residuals from CMig-EXT 14

**Substrate implication**: CMig-EXT 16 (property-bypass audit — `grep '.properties.iter()'` across the runtime crate, audit each site for shape-awareness) is a load-bearing discipline round, NOT optional. Should run before any further shape-tier default-on substrate moves.

Generalization: any substrate-tier move that changes data-structure storage (shape pivot, BigUInt repr change, etc.) needs an explicit completeness audit of all consumer sites.

---

## V. Per-pilot priority readings

### Finding V.1 — LeJIT-Τ (tiny-baseline) is the largest-arm pilot per §I.3 BUT has bounded CRB-side benefit

**Anchor**: TB-EXT 1 bench reading (125 ns dispatcher dominates 95% of per-call cost). CRB-EXT 9 reading (TB-Τ relevant to callback-heavy workloads only; minimal benefit on arith_tight_loop where dispatcher is <2%).

**Substrate implication**: TB-EXT 3a/3b/3c staged validation per the TB-EXT 3b scope-analysis round IS the right path. The pilot should proceed but the keeper should know:
- bench_call_overhead reclaim: expected 38-74 ns (per TB-EXT 2 decomposition; would meet Pred-tb.1)
- bench_ic reclaim: indirect via dispatcher reduction; expected similar magnitude
- CRB-side reclaim: bounded to callback-heavy workloads (string_url_sweep mixed, partially json_parse_transform). On arith_tight_loop minimal; on JSON-bound minimal.

The pilot's competitive-position contribution to cruft is real but not the full §I.3 multiplier alone.

### Finding V.2 — LeJIT-Σ (IC stub emitter) is bounded by shape-enrollment cascade

**Anchor**: StubE-EXT 5b empirical reading. Pre-shape baseline 271 ns → post-shape 199 ns (1.36× absorbed); IC observer wired adds +38 ns (237 ns); inline emission (5c+) targets ≤66.3 ns/iter for 3× target.

**Substrate implication**: Pred-stub.1's 3× target post-shape is at risk of (P2.d). The seed §I.2 falsifier explicitly names this. Composition with LeJIT-Τ + LeJIT-Ψ is required to reach the full §I.3 multiplicative target on bench_ic. Σ alone is (P2.d)-candidate; Σ in composition is load-bearing.

Forward: Σ-EXT 5c (inline emission) should land but with the explicit framing that it's a composition arm, not a standalone perf claim.

### Finding V.3 — LeJIT-Ψ (value-tag-inline) is (P2.d) at first cut; framework lesson is structurally informative

**Anchor**: VTI-EXT 3b reading. +18.9 ns regression at first cut. Five hypothesized mechanisms; structural conclusion: VTI alone cannot win without VTI-EXT 3c removing the dispatcher's `jit_compatible_arg` precheck.

**Substrate implication**: per the keeper's 2026-05-23 directive "go with b" (spawn LeJIT-Τ; defer VTI-EXT 3c), VTI is currently in a (P2.d)-paused state. The substrate stays in tree behind the env flag for VTI-EXT 3c's eventual landing. The pilot's structural value at this point is the framework lesson (Finding II.2), not the per-iter reclaim.

Future: VTI-EXT 3c (inline tag-check + precheck-removal) remains queued. Should land AFTER TB-EXT 3b validates the dispatcher-elimination thesis — TB-EXT 3b's success would confirm the calling-convention switch can be made to pay, paving VTI-EXT 3c's path.

### Finding V.4 — Future LeJIT-Σ' (x86_64 IC stub emitter) priority is secondary to Σ + Ψ + Τ

**Anchor**: engagement runs on Pi (aarch64) as reference hardware; per LeJIT seed §I.2 item 4, x86_64 is a parallel closure round.

**Substrate implication**: Σ' priority is below Σ + Ψ + Τ first-cut closure. Should land only when:
- Σ + Ψ + Τ first cuts close on aarch64
- Engagement gains an x86_64 measurement target
- Cross-arch consistency becomes load-bearing for a downstream pilot

Pre-filed but not active.

---

## VI. Forward-derived candidate pilots (not pre-filed)

Empirically named by this session's findings but not on the engagement's current roadmap. Reserved for future keeper direction.

### VI.1 — Fast JSON.parse / .stringify implementation
**Estimated CRB benefit**: closes 5-10× of json_parse_transform's 20.57× cruft/node gap. Brings json fixture to ~2-4× off node.
**Estimated scope**: multi-week. Independent of LeJIT entirely. Candidate locale `pilots/rusty-js-json-fast/`.
**Priority**: HIGH (largest single non-LeJIT improvement to cruft's CRB position).

### VI.2 — Tight-inner-loop emitter (Sparkplug variant for loops)
**Estimated CRB benefit**: closes ~2-3× of arith_tight_loop's 3.41× cruft/bun gap. Brings arith fixture to ~1.5× off bun.
**Estimated scope**: substantial. Hand-rolled aarch64 emission for tight inner loops. Sibling to LeJIT-Τ (LeJIT-Τ is tight-call-emitter; this is tight-loop-emitter). Candidate locale `pilots/rusty-js-jit/tight-loop/` (nested under LeJIT).
**Priority**: MEDIUM (improves arith-bound workloads specifically; smaller blast-radius than VI.1).

### VI.3 — Array.filter / .map fast-path
**Estimated CRB benefit**: closes 2-3× across mixed workloads. Brings string_url_sweep to ~3× off bun.
**Estimated scope**: moderate. Recognize callback shape at JIT-compile, inline body when callback is a small closure. Candidate locale `pilots/rusty-js-array-fast/` OR nested under LeJIT.
**Priority**: MEDIUM.

### VI.4 — SubtleCrypto wireup
**Estimated CRB benefit**: closes the crypto_sha256_batch FAIL (cruft can now attempt; gap-to-bun reading becomes available).
**Estimated scope**: small (intrinsic registration round; web-crypto pilot's substrate exists per engagement state but isn't wired to globalThis).
**Priority**: LOW (surface gap closure; doesn't close existing-fixture gaps).

### VI.5 — Multi-tier JIT (Tier-2 on top of LeJIT)
**Estimated CRB benefit**: amortizes Cranelift compile cost; expected 1.2-1.5× on workloads with many JIT-compiled functions.
**Estimated scope**: large. Doc 731 §VII R1 (single-tier baseline) explicitly carves this out; landing would require a corpus-tier amendment. Candidate locale `pilots/rusty-js-jit-tier2/`.
**Priority**: LOW (last-mile optimization; corpus tension with R1; only if VI.1-VI.4 don't suffice).

### VI.6 — Property-shape fuzz harness (CMig-EXT 17)
**Estimated benefit**: closes the structural probe-coverage gap that let CMig-EXT 15 escape.
**Estimated scope**: small (random property-mutation + spread patterns; ~200 LOC).
**Priority**: HIGH (probe-coverage discipline gate before any further shape default-on substrate moves).

---

## VII. Standing rules derived from findings

Codified for future substrate work:

1. **Report per-workload**: all LeJIT measurement claims must report against BOTH bench_ic-class AND CRB-class baselines (per CRB-EXT 8 §I.3 amendment).

2. **Multi-run protocol**: ≥5 runs and report median for any claim. Single-run readings document; multi-run validates.

3. **Detectability budget**: at N=10, ≥10% changes detectable; at N=30, ≥7%; below 7% needs N=100+.

4. **Never split a substrate move**: don't land a "stepping stone" that adds cost without removing equivalent cost. Either land the full move or don't land it (VTI-EXT 3b lesson).

5. **Three probes before default-on**: bench + consumer-route + fuzz all required for any default-on flip of substrate behavior (CMig-EXT 15 lesson).

6. **Surface-completeness audit**: any substrate-tier move that changes data-structure storage requires explicit enumeration + audit of all consumer sites (CMig-EXT 15 generalization).

7. **Cascade not assumed**: substrate-introduction rounds may or may not cascade per-iter savings. Predict reclaim explicitly with named mechanism; don't bank on generic cascade (VTI-EXT 3a vs Shape-EXT 4 contrast).

8. **Pilot priority follows the spread, not the seed §I.3 number**: per Finding V.1-V.3, LeJIT pilots have bounded CRB-side benefit; their composition contribution to bench_ic-class is the seed §I.3 number, NOT the CRB cruft/bun reduction.

---

*Last updated 2026-05-23 (session close of CRB-EXT 9). Update protocol: append new findings as they emerge from subsequent LeJIT-tier rounds; never edit historical findings (per Doc 727 §X basin-stability discipline). Findings that turn out to be wrong become new entries referencing the prior, not edits.*
