# ecmascript-parity-shared-upstream-arc — Trajectory

## EPSUA-EXT 0 — workstream founding (2026-05-25)

**Trigger**: keeper directive after T262C-EXT 2 chapter close: "Before doing so, close the chapter in the current locale and open this new one and project the resume vector in the trajectory.md". Succeeds T262C's first ECMAScript-parity-arc round (eight sub-locales, +300 PASS, 0 regressions, runnable rate 77.6% → 80.6%).

**Strategic framing**: T262C-EXT 2 produced two engagement-grade findings that scope this successor arc:
- **Finding T262C.4** identifies five shared-upstream constraints accounting for ~340 of the post-ASD 1416 fails (~24%; ~7.5× leverage vs mutually-exclusive long-tail). The arc operationalizes these.
- **Finding T262C.5** observes that the Doc 740 multi-tier closure became the engagement-default discipline mid-arc, producing 7 consecutive zero-regression cycles. This arc carries that discipline forward as C1.

**Pre-spawn Rule 11 5-axis check** (arc-tier; per-sub-locale checks deferred to each sub-locale's founding):
- (A1) component A/B — N/A; arc-tier coordinator
- (A2) op-set coverage — per sub-locale
- (A3) value-domain — per sub-locale
- (A4) locals-marshaling — per sub-locale
- (A5) emission-shape — per sub-locale
- (A6 — proposed engagement-tier extension per PPA seed): spec-section enumeration for the construct family. Each sub-locale's seed must name the spec sections it touches; failure to enumerate is the Rule 11 mirror for ECMAScript-parity sub-locales.

**Six Pred-epsua.* + discipline falsifier** (per seed §I.3).

**Founding artefacts**: seed.md + this trajectory.md + scaffolded dirs. **EPSUA-EXT 1** (`host-262-shim`, constraint #3, smallest-blast-radius first) is the next sub-locale founding per the §I.1 ordering and the keeper's authorization queue.

### Resume vector projection (carries forward through the arc)

The five queued sub-locales in dependency order:

| Order | Sub-locale dir | Constraint | Projected cascade | Status |
|---:|---|---|---:|---|
| 1 | `pilots/host-262-shim/` | #3 ($262 host hooks) | ~38 | queued |
| 2 | `pilots/iterator-close-on-abrupt/` | #4 (IteratorClose §7.4.9) | ~25 | queued |
| 3 | `pilots/parser-permissiveness-audit-extensions/` | #5 (escaped-of, dup-params, for-in-const, for-in-destr) | ~50 | queued |
| 4 | `pilots/strict-mode-parser-tracking/` | #2 (yield/let/static reserved per mode) | ~80 | queued |
| 5 | `pilots/host-method-prologue-discipline/` | #1 (RequireObjectCoercible + brand-check) | ~150 | queued |

Each sub-locale follows the EXT 0 (founding) → EXT 1 (multi-tier closure + exemplar verify + chapter close) pattern per C1+C2.

### Per-sub-locale founding template (carry-in for whoever picks up EPSUA-EXT 1)

When founding the next sub-locale:
1. `mkdir -p pilots/<name>/`
2. Write `seed.md` with: telos, apparatus, methodology, falsifiers (5 + 1 discipline), carve-outs, composes-with (must include EPSUA, T262C, this trajectory, the relevant ECMA-262 §, Doc 740, Doc 742, standing-rule-13-prospective).
3. Write `trajectory.md` with EXT 0 founding entry (rule 11 5-axis check + Pred-*).
4. `bash apparatus/locales/discover.sh` and commit the refreshed manifest with the seed+trajectory.
5. Implement per the multi-tier R identified pre-implementation (Doc 740 + Finding T262C.5 default discipline).
6. Exemplar-verify before full-sweep (C2). Include regression-probe on adjacent previously-passing tests (C3).
7. Update this trajectory with the sub-locale's chapter-close summary (cumulative-vs-projected ratio per C5).

### Status

EPSUA-EXT 0 CLOSED at founding. Next: keeper authorization to spawn EPSUA-EXT 1 (`host-262-shim`).

---

## EPSUA-EXT 0.5 — pre-scoping probe falsifies constraint #3 projection (2026-05-25)

**Trigger**: keeper "Continue" to spawn EPSUA-EXT 1 (`host-262-shim`). Per Finding T262C.6 carry-forward (C4): probe per-cluster failure-reason heterogeneity before scoping the fix.

**Probe result**: all 38 `$262` failures use a SINGLE API — `$262.createRealm`. Cluster is homogeneous (passes the discriminator's homogeneity test), BUT the substrate cost is high, not low as the prospective doc projected.

**Why projection was wrong**: the prospective doc estimated based on the plausible-typical $262 surface (detachArrayBuffer, evalScript, global, gc — any of which are stub-able). Empirical probe shows ALL fixtures in the sample exercise createRealm, which requires:
- A fresh Runtime instance (or fresh globalThis view), AND
- Distinct constructor identities (Array, Object, Function, etc.) for cross-realm tests where `array instanceof OtherRealm.Array` is the discriminator.

This is substantial substrate work, not the "smallest blast radius / pure additive" the doc named.

**Pred-epsua.3 falsified for constraint #3 specifically**: cumulative-vs-projected ratio for #3 alone is the wrong shape (substrate cost ≠ test-cascade ratio). The doc's aggregate prediction (~340 across 5) may still hold if other constraints compensate.

**Finding EPSUA.1**: pre-scoping probe per Finding T262C.6 catches projection drift cheaply. Without the probe, EPSUA-EXT 1 would have been founded against the wrong substrate cost; the round would have stalled mid-implementation when createRealm's true cost surfaced.

**Discipline preserved**: pivoting per EPSUA C5 to constraint #4 (`iterator-close-on-abrupt`, ~25 cascade, well-bounded substrate cost) rather than committing to a high-cost surface. Constraint #3 deferred to a later round where the realm-tier substrate cost can be properly budgeted.

**Re-ordered sub-locale queue** (post-probe):

| Order | Sub-locale dir | Constraint | Projected cascade | Status |
|---:|---|---|---:|---|
| 1 | `pilots/iterator-close-on-abrupt/` | #4 | ~25 | queued ← next |
| 2 | `pilots/parser-permissiveness-audit-extensions/` | #5 | ~50 | queued |
| 3 | `pilots/strict-mode-parser-tracking/` | #2 | ~80 | queued |
| 4 | `pilots/host-method-prologue-discipline/` | #1 | ~150 | queued |
| (deferred) | `pilots/host-262-shim/` (or `realm-substrate`) | #3 | ~38 | deferred pending realm-substrate budget decision |

---

## EPSUA-EXT 1 — iterator-close-on-abrupt (2026-05-25)

**Sub-locale**: `pilots/iterator-close-on-abrupt/` (ICOA-EXT 0+1; CLOSED).

**Cumulative-vs-projected**: +6 PASS vs ~25 projected (**24% of projection**); 0 regressions. Pred-epsua.3 falsified for constraint #4 specifically.

**Finding EPSUA.2** (cumulative across EXT 0.5 + EXT 1): TWO of the prospective doc's five constraints (#3 and #4) under-delivered vs projection in the first two probes. Aggregate doc projection (~340) requires upward revision; per-constraint amortization is lower than the prospective doc's matrix-cell heuristic suggested.

**Finding EPSUA.3** (methodology-level — strengthening Finding T262C.6 carry-forward C4): per-reason segmentation within a cluster is a HARD prerequisite before scoping each sub-locale, not optional. The matrix's cell label (pipeline × data-shape) over-aggregates when multiple distinct reason-shapes fall under one cell — the case for $262/createRealm (single API; cost mis-projected) AND for iter-close (cluster aggregated 6 close-pure + 14 deeper sub-causes).

**Implication for arc continuation**: EPSUA C4 strengthened — pre-scoping probe must enumerate the failure-REASON distribution within the candidate cluster AND identify the sub-cluster the candidate substrate fix targets. Projected cascade = the sub-cluster size, not the whole cluster size.

**Re-ordered sub-locale queue** (post-ICOA, sub-cluster-sized projections applied where probed):

| Order | Sub-locale dir | Constraint | Prior projection | Sub-cluster-segmented | Status |
|---:|---|---|---:|---:|---|
| 1 | `pilots/parser-permissiveness-audit-extensions/` | #5 | ~50 | unprobed | next |
| 2 | `pilots/strict-mode-parser-tracking/` | #2 | ~80 | unprobed | queued |
| 3 | `pilots/host-method-prologue-discipline/` | #1 | ~150 | unprobed | queued |
| (deferred) | `pilots/host-262-shim/` (realm-substrate) | #3 | ~38 | (createRealm = full realm subst.) | deferred |
| ✅ closed | `pilots/iterator-close-on-abrupt/` | #4 | ~25 | 6 close-pure | +6 |

---

## EPSUA-EXT 2 — parser-permissiveness-audit-extensions (2026-05-25)

**Sub-locale**: `pilots/parser-permissiveness-audit-extensions/` (PPAE-EXT 0+1; CLOSED).

**Pre-scoping probe** (per strengthened EPSUA C4): sub-cluster sizes for constraint #5 — escaped-of:1, params-duplicate:2, head-bound-names:14, for-in-destr-head:7 (deferred). In-scope: 17 tests vs prospective ~50.

**Edits**: ~70 LOC; 3 spec sites (§11.6.2 contextual unescaped, §15.2.1 arrow dup-params, §14.7.1.2 head-vs-body name conflict).

**Cumulative-vs-projected**: 7 PASS vs 17 in-scope (41%); 0 regressions across 838 adjacent previously-passing. Cluster-projection ratio: 7/50 = 14%.

**Finding EPSUA.4** (third constraint under-delivers): cumulative for EPSUA so far is 13 actual / ~113 projected = **12% of prospective amortization**. Pattern: matrix cell labels aggregate across distinct early-error sub-cases; the actionable scope per substrate fix is the per-filename sub-cluster, not the cell.

**Implication for arc continuation**: Pred-epsua.4 (≥2 of 5 within projection) requires #2 OR #1 to deliver ≥80% of projection. The methodology pattern suggests they too will under-deliver on whole-cluster sizing but be precise on sub-cluster sizing.

**Re-ordered queue** (post-PPAE):

| Order | Sub-locale | Constraint | Prior projection | Status |
|---:|---|---|---:|---|
| 1 | `pilots/strict-mode-parser-tracking/` | #2 | ~80 (probe first) | next |
| 2 | `pilots/host-method-prologue-discipline/` | #1 | ~150 (probe first) | queued |
| (deferred) | `pilots/host-262-shim/` (realm-substrate) | #3 | (createRealm) | deferred |
| (deferred) | `pilots/for-in-destr-head/` | #5-residual | ~7 | deferred |
| (deferred) | `pilots/head-bound-names-{tdz,let,dup}/` | #5-residual | ~10 | deferred |
| ✅ | `pilots/iterator-close-on-abrupt/` | #4 | ~25 | +6 |
| ✅ | `pilots/parser-permissiveness-audit-extensions/` | #5 | ~50 | +7 |

---

## EPSUA-EXT 3 — strict-mode-parser-tracking (2026-05-25)

**Sub-locale**: `pilots/strict-mode-parser-tracking/` (SMPT-EXT 0+1; CLOSED, partial).

**Pre-scoping**: prospective ~80 → in-scope ~12 (top-level yield-as-identifier; deferred function-body strict + generator tracking + onlyStrict yield-as-reserved).

**Edits**: ~20 LOC (Parser state + function_body_depth counter + yield-branch guard).

**Cumulative-vs-projected**: 8 PASS vs 12 in-scope (67%); 0 regressions. Cluster-projection ratio: 8/80 = 10%.

**Finding EPSUA.5**: cumulative EPSUA across 3 constraint sub-locales = 21 PASS / ~163 prospective = **13% of amortization**. The pattern is consistent: matrix cell labels uniformly aggregate across distinct sub-cases, and the prospective doc's projections were based on whole-cluster sizes. **Pred-epsua.4 (≥2 sub-locales within projection) is falsified across constraints #4, #5, #2.**

**Implication for Finding T262C.4 promotion**: the shared-upstream vs mutually-exclusive discrimination *is* correctly empirically demonstrated (all 3 closed constraints WERE single substrate fixes that cascaded across multiple clusters). What was wrong was the *scale* of the per-cascade prediction. The discriminator promotes to corpus per Pred-epsua.6 (≥2 corroborated), with a Finding-T262C.4-refinement: cascade SIZE must be projected per-sub-cluster, not per-matrix-cell.

**Updated arc state**:
| Constraint | Cluster Projection | In-Scope Sub-cluster | Actual | Status |
|---|---:|---:|---:|---|
| #4 (ICOA) | ~25 | ~25 | +6 | CLOSED 24% |
| #5 (PPAE) | ~50 | ~17 | +7 | CLOSED 41% (in-scope) |
| #2 (SMPT) | ~80 | ~12 | +8 | CLOSED 67% (in-scope) |
| #3 ($262/createRealm) | ~38 | — | — | deferred |
| #1 (host-method prologue) | ~150 | (probe needed) | — | next |

Note: in-scope-sub-cluster ratios are improving (#4 24%, #5 41%, #2 67%) as pre-scoping discipline tightens.

Next: constraint #1 with mandatory per-sub-cluster pre-scoping probe.

---

## EPSUA-EXT 4 — chapter close (2026-05-25)

**Trigger**: keeper directive "Do c and close this chapter, then spawn the sub locale". After pre-scoping probe on constraint #1 surfaced that the 226-test TypeError-not-thrown cluster fragments into ~6 distinct upstream causes (Symbol-coerce / ArraySpeciesCreate / revoked-proxy / non-extensible / heterogeneous tail), confirming that constraint #1 was misdiagnosed in the prospective doc as a single shared-upstream — same pattern observed across constraints #4, #5, #2 at smaller scale.

### Arc-tier results

| Constraint | Cluster Projection | In-Scope Sub-cluster | Actual Cascade | In-Scope Ratio |
|---|---:|---:|---:|---:|
| #4 (ICOA) | ~25 | ~25 | +6 | 24% |
| #5 (PPAE) | ~50 | ~17 | +7 | 41% |
| #2 (SMPT) | ~80 | ~12 | +8 | 67% |
| #3 ($262/createRealm) | ~38 | (full realm subst.) | — | deferred |
| #1 (host-method prologue) | ~150 | (probe surfaced fragmentation) | — | not entered |
| **Total** | ~343 | ~54 | **+21** | **39% of in-scope** |

**Cumulative**: +21 PASS / 0 PASS→FAIL regressions across 3 closed constraint sub-locales.

### Five Pred-epsua.* dispositions

| Predicate | Disposition |
|---|---|
| Pred-epsua.1 (≥84% runnable) | ⚪ NOT REACHED (current ~80.9% post-EPSUA); requires constraint #1 + heterogeneous-tail closures |
| Pred-epsua.2 (zero PASS→FAIL per round) | ✅ HELD (3/3 closed sub-locales) |
| Pred-epsua.3 (cumulative within ±30% of projections) | ❌ FALSIFIED (cumulative 21/163 = 13% of cluster-projections; 21/54 = 39% of in-scope sub-cluster projections) |
| Pred-epsua.4 (≥2 sub-locales within projection) | ❌ FALSIFIED across cluster-projections; ⚪ partial against in-scope projections (1 of 3 ≥50%) |
| Pred-epsua.5 (≤2 implementation rounds per sub-locale) | ✅ HELD (all 3 closed in 1 round) |
| Pred-epsua.6 (≥2 corroborate Finding T262C.4) | ✅ HELD with refinement (see below) |

### Findings

**Finding EPSUA.6 (Finding T262C.4 REFINEMENT)** — the shared-upstream vs mutually-exclusive discriminator was correctly applied at the matrix-cell level in T262C-EXT 2 but should be applied at the **reason-pattern level WITHIN the failure-reason histogram**:

> A failure-reason text shape (e.g., "Expected a TypeError, none thrown") is necessary but NOT SUFFICIENT evidence of shared-upstream. The reason TEXT may recur across pipelines while the *upstream causes* remain distinct. The discriminator must also verify the upstream cause is common — not just the reason wording.

The 226 TypeError-not-thrown cluster (constraint #1) demonstrates the failure mode: same reason text, ~6 different upstream causes. Each cause is its own shared-upstream sub-locale at a smaller cascade scale (~10-23 each).

**Finding EPSUA.7** — the discrimination methodology IS demonstrated empirically: all three closed constraints (#4, #5, #2) WERE single substrate fixes that cascaded across multiple pipelines (per-cluster ratio averaging 13% of cluster-projection, 39% of in-scope-sub-cluster projection). Cluster-projection over-counts; sub-cluster-projection (post per-reason segmentation) is the correct unit for Doc 740 leverage prediction.

**Finding EPSUA.8** — Doc 740 multi-tier closure default discipline (Finding T262C.5) continues to hold: 3 of 3 closed sub-locales with zero PASS→FAIL regressions; per-round substrate cost stayed under ~100 LOC; minimal-repro verification preceded every commit.

### Status

**CHAPTER CLOSED at EPSUA-EXT 4.**

The arc empirically validates Finding T262C.4's discriminator structure but refutes its projected magnitudes. The remaining work (constraint #1 sub-clusters + #3 createRealm + heterogeneous tail) is no longer a single coherent arc; each sub-cluster spawns as its own top-level locale.

Next per keeper directive: spawn `array-species-create-discipline` as a standalone top-level locale (the largest clean sub-cluster of the former constraint #1, ~20-23 tests, §7.3.21 single substrate site).
