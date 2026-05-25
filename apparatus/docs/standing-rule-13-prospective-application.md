# Standing Rule 13 — prospective application of revert-then-deeper-layer-closure

**Locale**: `docs/` (formalization of CANDIDATES.md tier-C entry (h))
**Author**: 2026-05-24 session
**Status**: working draft; SECOND CORROBORATION LANDED 2026-05-24 at IPBR locale (`pilots/iter-protocol-bytecode-rewrite/`; closed at IPBR-EXT 2 in 1 implementation round; all six Pred-ipbr.* HELD including Pred-ipbr.6 discipline falsifier). Ready for promotion to corpus Doc 742 pending keeper review.
**Composes with**:
- [findings.md Addendum IX](../pilots/rusty-js-jit/findings.md) — codification of standing rule 13
- [Doc 740 §IV.2](../../corpus-master/corpus/740-the-substrate-introduction-signature-of-revert-then-deeper-layer-closure-articulates-a-discipline-for-recognizing-when-a-substrate-move-is-actually-its-own-prefix.md) — the substrate-introduction signature reading
- [Doc 741](../../corpus-master/corpus/741-the-multi-tier-cascade-pipeline-connects-an-empirical-materialization-of-doc-740-across-four-sibling-pilots-on-a-cruftless-cross-runtime-bench-fixture.md) — empirical materialization of Doc 740 retrospectively at the IHI locale
- [interp-getprop-ic chapter close](../pilots/interp-getprop-ic/trajectory.md) — first prospective application; GPI-EXT 2 closed the locale in 42 LOC, first round

## 0. Thesis

Doc 740 named a discipline that has so far been read **retrospectively**: a substrate move that produces a negative result is not necessarily wrong; it may be the **prefix** of a closure whose deeper-layer materialization is the actual answer. Doc 741 materialized this empirically at the IHI locale (IHI-EXT 7 → 11 trajectory; revert at 7, closure at 11).

This document advances the discipline by one rung: **standing rule 13, applied prospectively at locale-founding time, lets a worker skip the substrate-introduction prefix entirely and pay only the deeper-layer closure cost**. The empirical anchor is the GPI locale (`pilots/interp-getprop-ic/`), where the rule was cited at funding (seed.md §I.4), the cache-tier rounds that IHI-EXT 7 paid for were skipped, and the closure landed in 42 LOC at the first implementation round.

## 1. The retrospective reading (Doc 740 + Doc 741 recap)

The substrate-introduction signature reading: a substrate move M₁ that produces metric Δ < 0 is read as **either**:

(a) M₁ is wrong as designed; revert and try a different design (the naive reading), **or**
(b) M₁ is the prefix of a chain M₁; M₂; ...; Mₙ whose closure (the deeper-layer move) produces Δ_final >> 0; M₁'s individual Δ is irrelevant because the chain composes to the result the worker actually wants.

Doc 740 §IV.2 articulated the diagnostic: when M₁ matches a substrate-introduction signature (new cache tier, new IC slot, new bytecode shape that nothing else consumes yet), do not revert reflexively. Probe the next layer.

Doc 741 materialized this across the IHI locale:
- IHI-EXT 7 introduced a per-Frame IC cache → +7% regression on `string_url_sweep` header_loop
- Reverted reflexively (would have been the naive (a) reading)
- Doc 740 §IV.2 reading re-applied: the cache tier was correct **shape**, wrong **lifetime** (Frame-scoped vs. site-scoped)
- IHI-EXT 8 → 10 introduced Runtime-tier Vec side-table (correct lifetime) → sub-noise impact (cumulative materialization not yet reached)
- IHI-EXT 11 introduced Op::CallMethodIcCached bytecode rewrite (the **deeper-layer closure**) → -3.6% CRB, -14% header_loop
- Chain composes: IHI-EXT 7's negative result was the substrate prefix; IHI-EXT 11 was the closure.

The reading was **retrospective**: the worker did not predict at IHI-EXT 7 that IHI-EXT 11 would close it. The reading happened after the fact when the keeper invoked Doc 740 §IV.2 explicitly.

## 2. The prospective reading (the GPI advance)

If Doc 740 §IV.2 is the **diagnostic** for recognizing a substrate-introduction prefix mid-trajectory, standing rule 13 is the **prescription** for skipping the prefix at funding time when the deeper-layer closure is already known.

GPI's seed.md §I.4 cited the rule explicitly at founding:

> Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. Skip the Frame-cache mis-design that IHI-EXT 7 paid for; go directly to bytecode rewrite.

The design doc (GPI-EXT 1) was funded around `Op::GetPropSkipForMethod` (bytecode rewrite) as the **founding** apparatus, not as the closure of a multi-round prefix. The implementation round (GPI-EXT 2) landed all 42 LOC of the closure in a single substrate move:

- 1 new opcode (3 LOC in op.rs)
- 1 new Frame field (4 LOC across 3 init sites)
- 1 new dispatch handler (5 LOC)
- 1 companion-rewrite extension to existing IC-hit branch (16 LOC)
- 1 bail-mitigation in existing CallMethodIcCached handler (8 LOC)
- 3 capture points in existing GetProp / CallMethod handlers (6 LOC)

No cache-tier substrate-introduction rounds. No Frame-cache mis-design tax. No deeper-layer-recognition mid-trajectory. The chapter closed at GPI-EXT 3 (composition probe + final disposition) with all five Pred-gpi.* HELD.

The trajectory is qualitatively different from IHI's:

```
IHI:   EXT 7 (-7%, prefix) → EXT 8 (~0%) → EXT 9 (~0%) → EXT 10 (~0%) → EXT 11 (-14%, closure)
       5 rounds; 4 paid the substrate-introduction tax retrospectively-recognized

GPI:   EXT 0 (founding) → EXT 1 (design) → EXT 2 (-11%, closure first round) → EXT 3 (composition + close)
       1 implementation round; 0 paid the tax
```

The cost of the discipline: ~4 rounds eliminated at GPI vs. IHI. The cost of the rule's articulation: ~2 corpus docs (740, 741) + ~1 standing-rule entry at findings.md Addendum IX.

## 3. The conditions under which prospective application is sound

Standing rule 13 is not a license to bypass empirical probing. The conditions under which the deeper-layer closure can be designed-from-first are:

**C1**. A sibling locale has already materialized the closure pattern empirically (Doc 741-style cumulative-materialization). For GPI, IHI's bytecode-rewrite-at-IC-hit was the sibling pattern; GPI mirrored its shape at the cross-op site (GetProp instead of CallMethod).

**C2**. The deeper-layer mechanism is shape-compatible with the current substrate's APIs. For GPI: cruft's bytecode is owned `Vec<u8>`, byte-aligned writes are atomic at the hardware level, and the dispatch loop's `op_from_byte` machinery accommodates new opcodes. The same conditions IHI relied on hold for GPI.

**C3**. The closure mechanism's cost model is positive when integrated. For GPI: per-call dispatch ~260ns → ~15ns dominates any per-rewrite cost (~100ns one-time) by >1000:1 amortization on hot loops.

**C4**. The bail path is correctness-preserving. For GPI: Op::CallMethodIcCached's bail re-resolves via `entry.key` on `string_prototype` when the popped method is the Undefined sentinel, so cold-path correctness holds.

When all four conditions hold, prospective application is sound. When any one fails, fall back to the substrate-introduction trajectory and apply Doc 740 §IV.2 retrospectively.

## 4. The discipline's induced property

The discipline produces a measurable property at the engagement tier: **trajectory length per locale closure converges as standing rules accumulate**. IHI took 11 rounds (founding → 9 substrate moves → close); GPI took 3 rounds (founding → 1 substrate move → close). The reduction is not a function of locale-shape difference (both are cross-op IC tables for the same hot loop); it is a function of the discipline's prospective applicability.

The prediction: subsequent IC-table locales (Array intrinsics per CANDIDATES.md tier-A entry (c); JIT GetProp method-IC per (b); for-of envelope rewrite per (a)) will close in ≤3 implementation rounds each when standing rule 13's C1-C4 hold at founding.

## 5. Falsifier

If candidate locale `pilots/ihi-array-entries/` (CANDIDATES.md tier-A (c)) is founded with standing rule 13 cited prospectively (skipping the cache-tier rounds; designing from bytecode-rewrite first) and **does not** close in ≤3 implementation rounds, the prospective-application thesis is partially falsified. Diagnostic: which of C1-C4 failed; revise the conditions or weaken the prediction.

If the candidate closes in ≤3 rounds, the thesis is empirically corroborated at a second locale, and the cross-locale convergence property (Doc 737 §IV) extends to the discipline tier.

**UPDATE 2026-05-24**: the actual second-corroboration locale was not `ihi-array-entries` but `iter-protocol-bytecode-rewrite` (IPBR), spawned in response to GPI-EXT 3's chapter-close report identifying the for-of envelope as the new per-iter dominator. Pred-ipbr.6 was set at locale-founding time as a direct test of this falsifier; the locale closed at IPBR-EXT 2 in 1 implementation round (3 total rounds) per IPBR's trajectory.md Finding IPBR.2. The thesis stands at second corroboration. Promotion to corpus Doc 742 pending keeper review.

**FURTHER UPDATE 2026-05-24 — third locale (TSR) refines the thesis**: `pilots/ts-resolve/` closed at TSR-EXT 5 in 4 implementation rounds — under Pred-tsr.6's ≤6 budget. New finding: the discipline scales gracefully with surface-area complexity (TSR is ~5× the LOC of GPI/IPBR; took 4 rounds vs their 1). However TSR also surfaced the **first empirical refinement of the C1-C4 conditions**:

- TSR's research-question probe (annotation-driven IPBR shape-skip would yield ≥10% reclaim) returned NULL.
- Diagnostic: C1 (sibling anchor — IPBR exists as the consumer), C2 (shape compat — Frame cache fits cleanly), and C4 (bail safety — fallthrough on shape-mismatch) all HELD. **C3 (cost-positive when integrated) FAILED** — the per-iter saving was sub-noise because the eliminated check (~50ns) is too small relative to the dispatch cost surface (~600ns/iter dominated by `idx.to_string` + `_i` HashMap write).

**Thesis refinement**: C3 is the **load-bearing condition for substrate-leverage claims at a downstream consumer**. C1 + C2 + C4 can all hold and the probe can still return null if C3's per-call cost model isn't favorable. The discipline is not weakened by null results — null results that pass C1+C2+C4 but fail C3 are high-information findings about the substrate's actual cost surface and should be celebrated as such, not avoided.

Promotion to corpus Doc 742 now includes this refinement as §3a (C-condition independence + the role of clean null results in the discipline).

## 6. Status

Working draft. Candidate for promotion to corpus Doc 742 after:
- (a) one additional empirical corroboration (ideally `ihi-array-entries`), or
- (b) keeper review of the thesis at the current empirical anchor (GPI alone)

Located at `apparatus/docs/standing-rule-13-prospective-application.md` per CANDIDATES.md tier-C entry (h) formalization. Refresh findings.md Addendum X with a one-line pointer when the corpus version lands.
