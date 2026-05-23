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

*Last updated 2026-05-23 (session close of TB-EXT 8 + JIT-EXT 29). Update protocol: append new findings as they emerge from subsequent LeJIT-tier rounds; never edit historical findings (per Doc 727 §X basin-stability discipline). Findings that turn out to be wrong become new entries referencing the prior, not edits.*

---

## Addendum — 2026-05-23 (post StubE-EXT 5c+7+8, TB-EXT 3b+4+7+8, JIT-EXT 29)

This addendum is append-only per the update protocol. Existing findings I-VI + standing rules above are preserved unchanged; this addendum records empirical validations + new findings that emerged from the substrate work after the original findings doc was seeded earlier in the same session.

### Promoted findings (empirical anchor strengthened)

**Finding V.1 (TB largest §I.3 arm BUT bounded CRB-side benefit)** — **CORROBORATED**. TB-EXT 3b delivered 62.7 ns reclaim on bench_call_overhead (Pred-tb.1 EXCEEDED by 50%); CRB-side reclaim was 2.2% on json (TB+STUB CRB cruft TB=1) — exactly the bounded-CRB-benefit pattern predicted. Per-pilot priority follows the bench reclaim, not the CRB reclaim.

**Finding V.2 (LeJIT-Σ bounded by shape cascade; needs composition)** — **PROMOTED**. StubE-EXT 5c flipped STUB's sign on bench_ic from +35.4 ns net-negative to −41.5 ns net-positive standalone. STUB no longer needs composition to be (P2.a). The "needs composition" qualifier is removable.

**Finding V.3 (LeJIT-Ψ (P2.d) at first cut)** — **UNCHANGED**, revival path empirically named: TB-EXT 7 + TB-EXT 4's TB+VTI bench_call_overhead reading (70.1 ≈ TB alone) prove that calling-convention restructure CAN pay when done with the precheck-removal (VTI-EXT 3c). VTI revival is queued, not deprecated.

**Finding II.2 (never split substrate moves)** — **TRIPLE-VALIDATED** at three applications in the same session: TB-EXT 3b approach A (low-LOC alternative to B/C, won); CMig-EXT 15 (out-of-band regression caught; spread fix removes the bypass without adding equivalent work); StubE-EXT 5c (Rust-extern A-level vs inline-IR B-level; A won bigger than B was predicted to).

**Standing rule 5 ("three probes before any default-on flip")** — **EMPIRICALLY VALIDATED at engagement scale** via three default-on flips: shape CMig-EXT 14 surfaced CMig-EXT 15 wrong-result bug (caught out-of-band, gap explicit); StubE-EXT 8 clean (three-probe-levels applied prospectively); **TB-EXT 8 clean POST-FIX** — TB-EXT 7 fuzz caught a SEGFAULT pre-flip. The rule's value compounds at each successive flip.

### New findings

**Finding II.4 (HashMap-value-slot raw-pointer caching is a load-bearing bug class)** *[new, 2026-05-23 via TB-EXT 7]*

**Anchor**: TB-EXT 7 enhancements log entry. TB-EXT 3b's design said "CompiledFn stable for process lifetime per leaked module" — wrong: the MODULE is stable (Box::leak'd) but the CompiledFn STRUCT in HashMap value slots moves on rehash. The TB closure-cell's cached `*const CompiledFn` dangled when subsequent JIT-compiles triggered HashMap rehash → segfault under fuzz workload. Fix: Box-wrap to `HashMap<usize, Option<Box<CompiledFn>>>`.

**Substrate implication**: any substrate move caching raw pointers into HashMap value-slot entries has the same dangling-pointer risk. Future raw-pointer-caching moves should audit upstream storage for by-value HashMap entries; if yes, the source needs Box-wrapping (or other stable-address discipline). The bench probes structurally CANNOT catch this — single-shape benches don't trigger rehash mid-fast-path. Only multi-pattern fuzz with sustained heap pressure during hot loop will reproduce.

**Generalization**: the engagement should audit existing raw-pointer caches for the same class. Candidates: `runtime_ic_fast_get`'s receiver-Object access (TLS-deref + heap-Vec read); any other pointer captured from HashMap or Vec containers. Audit queued as `StubE-EXT 9 / TB-EXT 9` candidate.

**Finding V.5 (Default-on flip discipline compounds at engagement scale)** *[new, 2026-05-23 via three-flip pattern]*

**Anchor**: three default-on flips this session, each with different bug-class outcome:
- shape CMig-EXT 14: regression caught OUT-OF-BAND (parallel-Claude measurement)
- StubE-EXT 8: clean flip (three-probe-levels first prospective application)
- TB-EXT 8: clean POST-FIX (TB-EXT 7 fuzz caught segfault)

**Substrate implication**: the engagement's default-on flip discipline (three-probe-levels per standing rule 5) is empirically validated at three applications. The discipline's value MEASURABLY COMPOUNDS — each successive flip's bug surface is closed before the flip, not after. The fuzz probe class catches what bench + consumer-route structurally cannot.

**Generalization**: any future default-on flip in the engagement (across pilots) MUST run the three-probe-levels gate. The cost of NOT running it (CMig-EXT 14 → CMig-EXT 15 retrospective regression + ~half-day debugging) substantially exceeds the cost of running it (~85 LOC fuzz fixture + 15 minutes to run).

**Finding V.6 (LeJIT first-cut composition target empirically met at engagement default)** *[new, 2026-05-23 via JIT-EXT 29 synthesis]*

**Anchor**: post-StubE-EXT 8 + TB-EXT 8 default-on flips, bench_call_overhead = 71 ns and bench_ic = 81 ns at default-cruft (no env flags). Pre-session baselines were 122.9 / 197.9 ns. The bench_ic narrow-microloop result matches bun's typical per-op cost (~94 ns); LeJIT seed §I.3's prediction "matches Bun's per-op cost on the same workload" is corroborated and exceeded.

**Substrate implication**: the LeJIT pilot's first-cut chapter is closed at engagement-tier (P2.a) at scale. Subsequent LeJIT-tier work is forward optimization, not load-bearing for any standing Pred:
- VTI revival (VTI-EXT 3c)
- Skip STUB infra on no-property functions
- Inline Cranelift IR for IC fast-path (marginal vs current Rust-extern)
- Cross-pilot bug-class audits (StubE/TB-EXT 9)

**Engagement-tier consequence**: the CRB-class composition target (3-15× off bun on realistic workloads per CRB-EXT 9) is NOT met by LeJIT alone — closing it requires forward-derived non-LeJIT pilots per findings VI.1-VI.3 (fast JSON, tight-inner-loop emitter, Array.filter/map fast-path). These are multi-pilot, multi-session work.

**Finding III.4 (Composition synergy is constructive when flag mechanisms target different cost components)** *[new, 2026-05-23 via TB-EXT 4 → StubE-EXT 5c]*

**Anchor**: TB+STUB on bench_ic post-StubE-EXT 5c reads 80.8 ns; independent-delta prediction was 123.6 ns; **synergy −42.8 ns (constructive interference)**. TB removes dispatcher per-call overhead; STUB removes per-GetProp slow path. Together they remove both halves of bench_ic's per-iter cost almost completely.

**Substrate implication**: the §I.3 multiplicative composition claim holds at first cut with a SHARPENING — per-flag deltas don't just sum, they compose more tightly when each flag's reclaim mechanism is in a different cost component. VTI does NOT compose constructively yet because it overlaps TB's dispatcher target on the SAME cost component (calling-convention).

**Generalization for future LeJIT-tier pilots**: composition synergy is positive when reclaim mechanisms target ORTHOGONAL cost components; near-zero when targets overlap. Future pilot priority should consider not just per-pilot reclaim but cost-component orthogonality with existing pilots.

### New standing rule

**Standing rule 9 (added 2026-05-23)**: any raw-pointer cache capturing a pointer to a struct living in a HashMap or Vec value slot must verify the underlying storage uses Box-wrapping or equivalent stable-address discipline. The structure must be address-stable for the cache's intended lifetime. Default audit: any `*const T` capture where `T` lives in `HashMap<_, T>` or `Vec<T>` is suspect.

*(Per Finding II.4. The TB-EXT 7 segfault would have been prevented by this rule applied prospectively at TB-EXT 3b design time.)*

---

## Addendum II — 2026-05-23 (post Φ-EXT 2+3 merged round)

### New finding

**Finding II.5 (gap-closure at the structural-constraint tier cascades sibling-pilot revival)** *[new, 2026-05-23 via Φ-EXT 2+3 → VTI revival]*

**Anchor**: Φ-EXT 2+3 merged round. Pre-Φ TB+STUB+VTI on bench_ic was 743.8 ns ((P2.d) at composition scale). Post-Φ (no other substrate moves to VTI) it is 85.5 ns. **VTI revived without any VTI-specific substrate work.** The Φ pilot's intended scope was the calling-convention shift; VTI's revival was an unanticipated cascade.

**Structural mechanism**: pre-Φ the JIT's value-domain interface (the P2 stage of an implicit resolver-instance pipeline per Doc 729 §IV) had an i64-only constraint. The constraint propagated downstream: P3 dispatcher had to integer-validate every arg (the `jit_compatible_arg` precheck); P4 externs had to encode Numbers as i64. **VTI's structural (P2.d) was a consequence of working around the P2 constraint** (its inline tag-check couldn't beat the precheck's integer-validity work). When Φ closed the P2 gap by lifting i64-only to f64-default, the constraint propagation collapsed: P3's precheck collapsed to tag-only; VTI's inline tag-check became an equivalent replacement; VTI's existing payload-extract-only code (per VTI-EXT 3b) became correct as-is because the JIT body now operates on loaded f64 directly.

**The cascade pattern**: substrate-amortization-cascade per Doc 729 §A8.13 has been engaged at two prior tiers in this engagement:
1. Per-iter cost reduction (Shape-EXT 4 enrollment, TB-EXT 3b reclaim)
2. Composition synergy (Φ III.4: constructive interference when targets are orthogonal)

This round names a third tier:
3. **Sibling-pilot revival**: gap-closure at the structural-constraint tier cascades downstream sub-pilot (P2.d) → (P2.a) without substrate work.

**Substrate implication**: when a sub-pilot stalls at (P2.d), the engagement should investigate whether the stall is downstream of a STRUCTURAL constraint at an upstream tier. If yes, the right substrate move may be to lift the upstream constraint (which addresses the stalled pilot AND may cascade-revive other downstream stalls), not to attempt yet another sub-pilot at the same constrained tier. The constraint-enumeration discipline (C1-C10 in Φ seed §I.2 + Pin-Art apparatus per Doc 581) is the framework for naming the upstream constraint.

**Generalization** (corpus-articulation candidate, formalized as Doc 739): the pattern likely recurs at any engagement that runs a resolver-instance pipeline per Doc 729. The engagement's standing reading should include "constraint-closure as cascade-revival" as a diagnostic instrument: when a sub-pilot stalls, the first question becomes "what upstream constraint propagates the stall?" before "what new sub-pilot do we spawn?"

**Forward implication for future LeJIT-tier work**:
- VTI is no longer (P2.d). Pred-vti.5 effectively met. VTI's "revival path" framing from earlier in the session is resolved.
- Forward-derived pilots from Findings VI (fast JSON, tight-inner-loop emitter, Array.filter/map fast-path) should be evaluated under the same lens: are they addressing constraint-propagated stalls, or local perf optimizations?
- Move 2 (typed-i64 promoted fast path) reads differently post-Φ: it's not a competitor to f64-default, it's a SPECIALIZATION ON TOP of f64-default. The architectural shift makes Move 2 simpler.

**Provenance**:
- Round: `pilots/rusty-js-jit/f64-calling-convention/trajectory.md` Φ-EXT 2+3
- Composition matrix: `pilots/rusty-js-jit/tiny-baseline/docs/composition-matrix.md`
- Cross-reference: this file's prior addendum Finding V.3 (LeJIT-Ψ (P2.d) at first cut) — NOW RESOLVED via cascade
- Cross-reference: Φ seed §I.2 constraint enumeration C1-C10 (the apparatus that named the constraint to be closed)
- Corpus articulation: jaredfoy.com Doc 739 (formalizes the abstract pattern + the LeJIT-Φ instance)

---

## Addendum III — 2026-05-23 (post CMig-EXT 16 + 16.bis + 17 substrate-correctness chapter close)

This addendum captures three lessons from the engagement's shape-enrollment substrate-correctness work (CMig-EXT 15 + 16 + 16.bis + 17). The lessons are operational rather than per-pilot performance findings; they refine the engagement's standing apparatus discipline. Per the append-only protocol, addenda land sequentially without editing prior sections.

### New finding

**Finding IV.3 (Audit quick-scans produce HYPOTHESIS lists, not fix lists)** *[new, 2026-05-23 via CMig-EXT 16 → 16.bis precision delta]*

**Anchor**: CMig-EXT 16's quick-scan audit identified 5 NEEDS-FIX call sites by grep + brief context inspection. The CMig-EXT 16.bis fix round's per-site reads moved 2 of the 5 to SAFE (intrinsics.rs:2682 + 5507 — both already had the shape-iter-chain pattern that the quick-scan missed in surrounding context). Plus 1 more was moved to SAFE during the audit's own verification pass (value.rs:508 has_own_str — shape-aware-via-fallback). Net actual NEEDS-FIX after full per-site read: 2 of original 5 = 40% precision.

**Substrate implication**: any audit that enumerates call sites without reading the surrounding scope is necessarily over-conservative. The discipline:

1. **Quick-scan produces a hypothesis list.** Per-call-site grep + minimal context to flag candidates.
2. **Per-site read in fix round refines the hypothesis.** Read 20+ lines around each site; verify whether the shape-aware pattern is already present in the surrounding scope.
3. **Categorization is finalized at fix-round close, not audit close.** The audit's NEEDS-FIX count is a ceiling; the actual fix count is bounded below by the audit's precision.

This generalizes beyond shape-enrollment: any future audit of call-site classes (security review, deprecation cleanup, semantic migration) should follow the same hypothesis-list → verify-in-fix-round workflow.

### New finding

**Finding IV.4 (Canonical fuzz harness as standing engagement instrument)** *[new, 2026-05-23 via CMig-EXT 17]*

**Anchor**: CMig-EXT 17 landed an 8-pattern × 2000-fixture × 8-configuration fuzz harness producing 128,000 effective fixture-runs, all byte-identical across cruft-default / cruft-explicit-flags / cruft-shape-off / node. The harness is engagement-wide, not pilot-specific.

**Substrate implication**: future default-on flips (any pilot, any session) can run this harness as their fuzz-probe-level instrument under the three-probe-levels discipline (rule 5). They do NOT need to spawn a new per-flip fuzz fixture; the canonical fuzz IS the standing instrument.

The CMig-EXT 15 + 16 + 16.bis pattern (per-flip fixture spawn, e.g., fuzz-tb.mjs at TB-EXT 7) was appropriate when no canonical harness existed. Now that the canonical exists, per-flip fixtures are SUPPLEMENTARY (for pilot-specific patterns the canonical doesn't cover), not REPLACEMENT.

**Generalization for future engagement sessions**: when running a default-on flip's three-probe-levels gate, the procedure is:
1. Bench probe: pilot-specific bench (e.g., bench_ic for STUB; bench_call_overhead for TB)
2. Consumer-route probe: diff-prod under the flag's default-on state
3. Fuzz probe: run `pilots/rusty-js-shapes/consumer-migration/fixtures/fuzz-canonical.mjs` under the flag's default-on state; verify byte-identity vs node
4. Pilot-specific fuzz fixture is OPTIONAL supplementary coverage; only required if the pilot's substrate exercises patterns the canonical doesn't cover.

### New standing rule

**Standing rule 10 (added 2026-05-23)**: any future default-on flip's three-probe-levels gate must include a run of `pilots/rusty-js-shapes/consumer-migration/fixtures/fuzz-canonical.mjs` under the flag's default-on state. Output must be byte-identical to the node baseline (`acc=-932188103` at the CMig-EXT 17 version `cmig-ext-17-2026-05-23`; the version + reference acc update whenever the canonical fuzz is extended).

The rule's value compounds across flips: each future flip's correctness is gated on the canonical's coverage; the canonical's coverage grows over time as new patterns get added to it (when the engagement encounters a new substrate-correctness class). Per CMig-EXT 17's pattern set, the canonical currently covers JSON.stringify + spread + delete+re-add migration + Object.* enumeration + Map/Set iteration + hot property-access loops + nested composition + array-of-objects.

### Promotion of prior findings via the CMig-EXT 15-17 work

**Finding IV.1 (diff-prod + test262-sample alone insufficient for shape-enrollment correctness)** — **PROMOTED TO RESOLVED at canonical scope**. The third probe level (canonical fuzz per CMig-EXT 17) now exists as standing instrument. The "insufficient" qualifier was the gap CMig-EXT 17 closes.

**Finding IV.2 (Surface-completeness audit is a missing engagement discipline)** — **PROMOTED TO PARTIALLY RESOLVED**. The audit discipline now exists (per CMig-EXT 16). Its refinement (Finding IV.3 hypothesis-list workflow) is part of this addendum. Engagement-wide application of the audit to future data-structure-storage changes (per standing rule 6) is queued; the apparatus is in place.

### Findings-doc-as-standing-instrument cumulative status

After this addendum, the findings doc contains:
- 6 original finding sections (I-VI; per-category)
- 8 original standing rules
- Addendum: 5 findings (3 promoted; 2 new) + 1 new standing rule (#9)
- Addendum II: 1 new finding (II.5 gap-closure-as-cascade-revival)
- Addendum III (this): 2 new findings (IV.3 audit-precision; IV.4 canonical-fuzz-as-standing-instrument) + 1 new standing rule (#10) + 2 promotions (IV.1 RESOLVED; IV.2 PARTIALLY RESOLVED)

Total: 9 findings (6 original + 3 new across addenda); 10 standing rules (8 original + 2 added); multiple promotions tracking empirical anchor strengthening.

The findings doc is now a working engagement instrument with 4+ self-applications validated in the session it was created. The "use these to improve substrate" directive that motivated the doc's creation has produced compounding returns: each new finding refines future substrate work; each new standing rule prevents a bug class from recurring.

---

## Addendum IV — 2026-05-23 (post JSF chain + CharCode-EXT 1-2 multi-tier pipeline-connection)

This addendum captures four lessons from the engagement's JSF (JSON.stringify fast-path) workstream plus the post-JSF charCodeAt chain. The lessons span Pin-Art apparatus refinement, substrate-spawn discipline, and a multi-tier cascade-revival shape that extends Doc 739.

### New finding

**Finding II.2-bis (Substrate-introduction (P2.d) as cascade-revival signature)** *[new, 2026-05-23 via JSF-EXT 3 first-cut + JSF-EXT 5 aggregate]*

**Anchor**: JSF-EXT 3 (Move 1 buffer threading) landed correctly across both correctness probes but produced essentially flat per-shape bench. Per Finding II.2 (never split substrate moves), the round would naively classify as a partial failure. Per Doc 739 §II.2, the flat bench was instead the SIGNATURE of an upstream-constraint-closure round whose reclaim materializes downstream at cascade-revival pilots (M2/M3/M4 each closing a leaf emitter tier).

**Substrate implication**: when classifying a substrate-introduction round's (P2.d) outcome, the question is not "did this round deliver reclaim" but "did this round close an upstream constraint whose downstream consumers can now cascade-revive?" If yes, the (P2.d) is the round's correct categorization and is the SIGNATURE that the round IS substrate-introduction, not a falsification.

**How to apply** at each substrate-introduction round:
1. Name the upstream constraint being closed by the round.
2. Name the downstream consumer pilots that become cascade-revival candidates per the closure.
3. If both are nameable, accept (P2.d) bench at the introduction round and proceed to the consumer rounds.
4. If neither is nameable, the (P2.d) is a genuine pilot-failure signal.

Cross-reference: Doc 729 §A8.13 substrate-amortization-cascade (the per-iter cost axis); Doc 739 cascade-revival (the categorization axis).

### New finding

**Finding VII.1 (Component-decomposition estimates require empirical anchoring before pilot spawn)** *[new, 2026-05-23 via JSF-EXT 8 component-A/B probe]*

**Anchor**: CRB-EXT 9's component-decomposition estimate for json_parse_transform placed JSON.stringify at ~5-10× of the 20× cruft/node gap (so ~50-70% contributor). The JSF pilot was spawned on that anchor. The A/B probe (JSF-EXT 8) measured the actual decomposition empirically: JSON.stringify is 3% of cruft's total wall-clock; the actual dominator is a top-level `for (i; i<out.length; i++) cs += out.charCodeAt(i)` checksum loop at 77% — not part of the "JSON pipeline" at all. CRB-EXT 9's estimate was off by ~20×.

The A/B probe ran in <10 seconds. Had it been run before JSF spawn, the entire JSF pilot would have targeted a different component (charCodeAt + interp dispatch) at ~25× higher leverage per LOC.

**Substrate implication**: before spawning a substrate pilot whose telos is "close a CRB-measured gap," run a component A/B probe on the target fixture. Replace each suspect component with a no-op or near-no-op variant; measure per-variant wall-clock; the per-variant Δ isolates each component's contribution.

**How to apply** before pilot spawn:
1. Identify the suspect components in the target fixture (5-8 typically).
2. Author a probe fixture with N additive variants (V0 = baseline minus all suspects; V1 = +1 suspect; ... Vn = full fixture).
3. Run the probe on cruft + node (or whatever oracle); per-component Δ = (Vk - Vk-1) per runtime.
4. Per-component cruft/node ratio + absolute contribution ranks the actual targets.
5. Spawn the pilot at the actual dominator. Carry the probe forward as a standing fixture for subsequent measurement.

### New standing rule

**Standing rule 11 (added 2026-05-23)**: before spawning any pilot whose telos is "close a CRB-measured gap," run a component A/B probe to identify the actual dominator empirically. The probe should run in <10 minutes; its cost is amortized across the prevented mis-targeting of substrate work.

The rule's value compounds across pilot spawns: each future CRB-driven pilot spawns at the actual bottleneck, not a theoretically suspected one. CRB-EXT 9's mis-attribution is the cautionary anchor; the JSF chain's reclaim curve (12% CRB cumulative after 6 rounds + 2 follow-on chain rounds) is the empirical cost-of-not-applying-the-rule.

### New finding

**Finding II.3 (Multi-tier cascade-revival: closing one tier alone is insufficient when the hot path traverses multiple tiers)** *[new, 2026-05-23 via JSF + CharCode-EXT 1+2 chain]*

**Anchor**: Doc 739's cascade-revival pattern (single-tier: upstream constraint-closure → downstream sibling-pilot revival) materialized partially at the JSON-stringify pipeline (JSF M1 → M2/M3 revival at +5-7% each). The full CRB gap remained because the actual hot path traversed multiple tiers:
- substrate-tier (chars().nth() O(n²) bug in charCodeAt + length)
- dispatch-tier (call_function frame setup + this-binding + descriptor walk per method call)

Closing the substrate-tier alone (CharCode-EXT 1: ASCII fast-path) produced -15% on the dominator-loop; closing the dispatch-tier alone (a hypothetical interp-IC without substrate fix) would similarly produce partial reclaim. Closing BOTH (CharCode-EXT 1 + 2) produced -27% on the dominator-loop (-12% CRB cumulative); the cascade-revival pattern recurs at the multi-tier scope.

**Substrate implication**: when the hot path traverses N tiers, the cascade-revival pattern is sufficient at each tier-pair but NOT at the cross-tier scope unless all N tiers are addressed. A pilot whose telos is "close a CRB gap" should enumerate the tiers along the hot path; close each in dependency order; gate reclaim measurement only after all relevant tiers are closed.

**How to apply**:
1. Per Finding VII.1 + rule 11, identify the actual hot-path component empirically.
2. For that component, source-read the tier stack (substrate intrinsic → dispatch path → call frame → bytecode dispatch → JIT eligibility).
3. Name each tier's contribution to the per-call cost.
4. Spawn substrate work per-tier in dependency order (upstream → downstream).
5. Gate reclaim measurement after each tier; the cumulative is the pipeline-connection point.

### Promotion of prior findings via the JSF chain

**Finding II.5 (Cascade-revival at structural constraint tier)** — **PROMOTED TO GENERALIZED: multi-tier scope**. The JSF chain demonstrated cascade-revival across non-adjacent tiers (substrate → dispatch); Doc 740 (forthcoming) articulates the generalization. Doc 739's single-tier formulation remains a special case of the multi-tier pattern.

### Engagement instruments delta

**New standing instrument**: `pilots/rusty-js-json-fast/fixtures/component-ab-probe.mjs` — the JSF A/B probe is the engagement's first standing component-decomposition instrument. Future CRB-driven pilots adapt this fixture's pattern (5 additive variants × 50-iter warmup × 500-iter measurement) for their target fixture per rule 11.

**Hot-intrinsic IC pattern (engagement-tier substrate-work candidate)**: CharCode-EXT 2's interp-tier IC fast-path for String.prototype.charCodeAt validated the dispatcher-bypass pattern at ~100 ns/call reclaim. Generalization candidate: hot-intrinsic table for charAt, codePointAt, indexOf, slice, push, pop, shift, splice, and other dispatch-bound intrinsic calls. The pattern is small per-intrinsic; broadly applicable across the engine.

### Findings-doc cumulative status

After this addendum, the findings doc contains:
- 6 original finding sections (I-VI; per-category)
- 8 original standing rules
- Addendum I: 5 findings (3 promoted; 2 new) + 1 new standing rule (#9)
- Addendum II: 1 new finding (II.5 cascade-revival)
- Addendum III: 2 new findings (IV.3 audit-precision; IV.4 canonical-fuzz) + 1 new standing rule (#10) + 2 promotions
- Addendum IV (this): 3 new findings (II.2-bis substrate-introduction-signature; VII.1 component-A/B; II.3 multi-tier-cascade) + 1 new standing rule (#11) + 1 promotion + 2 new standing instruments

Total: 12 findings (6 original + 6 new across addenda); 11 standing rules (8 original + 3 added); 2 new engagement instruments (canonical fuzz + component A/B probe).

The findings doc continues compounding: each session's work refines future substrate work; each standing rule prevents a bug class or mis-targeting class from recurring. The JSF chain's empirical demonstration of multi-tier cascade-revival is corpus-articulation candidate (Doc 740, forthcoming).
