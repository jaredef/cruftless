# Cruftless Predictive Ruleset

This document consolidates the engagement's standing rules into a single predictive reference. Each rule is presented as a falsifiable PREDICTION about what will happen when it is (or is not) followed, with a pointer to the empirical evidence supporting the prediction in the engagement record.

The ruleset compounds: rules higher-numbered build on prior rules + their predictions condition each other. The compounding is itself a load-bearing structural claim — see Doc 541 Appendix E (SIPE-T scale-invariance) and Doc 742 (resolver-instance boundary contract).

**Authoritative source**: `pilots/rusty-js-jit/findings.md` (23 rules across 14 addenda, append-only per Doc 727 §X). This doc is a derived consolidated view; on disagreement, findings.md is canonical.

**Status as of 2026-05-25**: **23 rules in findings.md; rules 1-15 + 23 articulated in this doc**. Rules 16-22 (Addenda XII + XIII) are in findings.md but not yet folded into this consolidated view; a full re-consolidation pass remains open work. Rule 23 (founding-baseline-inspection / locale-as-probe) was promoted at keeper directive 2026-05-25 from Finding NLC.1 (tokenization-above-IR arc).

---

## Rule 1 — Report per-workload

**Statement**: all LeJIT measurement claims must report against BOTH bench_ic-class AND CRB-class baselines.

**Predicts**: substrate moves whose reclaim is reported against only one baseline will mis-attribute the move's actual yield. The other baseline is the one with the consumer-relevant cost surface.

**Evidence**: CRB-EXT 8 §I.3 amendment; cumulative LeJIT pilot history shows bench-only claims systematically over-state CRB-side impact (Findings V.1-V.3).

---

## Rule 2 — Multi-run protocol

**Statement**: ≥5 runs and report median for any substrate claim. Single-run readings document; multi-run validates.

**Predicts**: single-run claims will be noise-dominated below a ~10% effect; reversed-sign measurements are observed routinely at N=1.

**Evidence**: findings.md §I.1; engagement-wide convention enforced via `pilots/apparatus/cross-runtime-bench/scripts/run-bench.sh --runs N`.

---

## Rule 3 — Detectability budget

**Statement**: at N=10, ≥10% changes detectable; at N=30, ≥7%; below 7% requires N=100+.

**Predicts**: substrate claims of <7% improvement at N=30 will not be reproducible across re-runs.

**Evidence**: empirical noise-floor measurement across the CRB fixture set.

---

## Rule 4 — Never split a substrate move

**Statement**: don't land a "stepping stone" that adds cost without removing equivalent cost. Either land the full move or don't land it.

**Predicts**: half-landed substrate moves accumulate as silent cost surface; the eventual full move will face higher LOC + correctness debt than landing it whole.

**Evidence**: VTI-EXT 3b lesson; Finding II.2 — TRIPLE-VALIDATED at TB-EXT 3b approach A, CMig-EXT 15, StubE-EXT 5c.

---

## Rule 5 — Three probes before default-on

**Statement**: bench + consumer-route + fuzz all required for any default-on flip of substrate behavior.

**Predicts**: any default-on flip lacking one of the three probe levels will surface a bug class the missing probe would have caught.

**Evidence**: CMig-EXT 15 wrong-result bug (caught out-of-band; fuzz absent); StubE-EXT 8 clean (rule applied prospectively); TB-EXT 7 fuzz caught a SEGFAULT pre-flip. Rule 5 EMPIRICALLY VALIDATED at 3 applications; **value compounds at each successive flip** (Addendum I).

---

## Rule 6 — Surface-completeness audit

**Statement**: any substrate-tier move that changes data-structure storage requires explicit enumeration + audit of all consumer sites.

**Predicts**: storage-shape changes whose audit is skipped will leave at least one consumer reading from the old shape, producing latent correctness bugs.

**Evidence**: CMig-EXT 15 generalization; the audit-tool apparatus landed at CMig-EXT 16.

---

## Rule 7 — Cascade not assumed

**Statement**: substrate-introduction rounds may or may not cascade per-iter savings. Predict reclaim explicitly with named mechanism; don't bank on generic cascade.

**Predicts**: substrate-introduction work that asserts cascade without naming the mechanism will produce zero (or negative) measured reclaim.

**Evidence**: VTI-EXT 3a (no cascade; net-negative) vs Shape-EXT 4 (mechanism named upfront; net-positive) contrast.

---

## Rule 8 — Pilot priority follows the spread, not the seed §I.3 number

**Statement**: LeJIT pilots have bounded CRB-side benefit; the composition contribution to bench_ic-class is the seed §I.3 number, NOT the CRB cruft/bun reduction.

**Predicts**: pilots prioritized by CRB-reduction projection will systematically under-deliver vs pilots prioritized by their bench-class spread.

**Evidence**: Findings V.1-V.3 across the engagement's pilot prioritization history.

---

## Rule 9 — Raw-pointer cache stability

**Statement**: any raw-pointer cache capturing a pointer to a struct living in a HashMap or Vec value slot must verify the underlying storage uses Box-wrapping or equivalent stable-address discipline. Default audit: any `*const T` capture where `T` lives in `HashMap<_, T>` or `Vec<T>` is suspect.

**Predicts**: raw-pointer caches captured without address-stability verification will dangle on container resize/rehash, producing SEGFAULTs that fuzz probes will eventually catch.

**Evidence**: TB-EXT 7 SEGFAULT (Addendum I); Finding II.4 — caught by fuzz probe, pre-default-on per rule 5.

---

## Rule 10 — Canonical fuzz as standing default-on instrument

**Statement**: any future default-on flip's three-probe-levels gate must include a run of `pilots/rusty-js-shapes/consumer-migration/fixtures/fuzz-canonical.mjs` under the flag's default-on state. Output must be byte-identical to the node baseline (`acc=-932188103` at cmig-ext-17 version).

**Predicts**: default-on flips lacking the canonical fuzz run will surface byte-divergence bugs the per-flip fuzz fixture would have missed.

**Evidence**: canonical-fuzz-as-standing-instrument (Addendum III, Finding IV.4). Compounds with rule 5.

---

## Rule 11 — Pre-spawn 5-axis coverage check (multi-axis)

**Statement**: before spawning any pilot whose telos is "close a CRB-measured gap," run a 5-axis coverage check:

- **A1 (component A/B)** — empirically identify the actual hot-path component dominator via additive variants (runs in <10 minutes).
- **A2 (op-set coverage)** — confirm the planned substrate work's op-set covers the dominator's op-set.
- **A3 (value-domain coverage)** — confirm the value-domain matches (e.g., NaN-boxing covers the receiver tag set).
- **A4 (locals-marshaling coverage)** — confirm the entry-mechanism marshals the relevant locals.
- **A5 (emission-shape coverage)** — confirm the JIT/lowering emission shape matches the dominator's structure.

**Predicts**: pilots spawned without rule 11 will mis-target — the substrate work will be correct on its own terms but irrelevant to the bench's actual cost surface.

**Evidence**: JSF pilot (Addendum IV; Finding VII.1 — JSON.stringify projected at 50-70% of cost; component A/B revealed the actual dominator was character-scanning at 77% of wall-clock; JSF reclaim was -1% within noise). Standing instrument: `pilots/rusty-js-json-fast/fixtures/component-ab-probe.mjs`. Rule extended along A2-A5 axes across Addenda V/VII/VIII as additional pilots surfaced each blocker.

---

## Rule 12 — Adversarial IEEE-754 tests for bit-pattern schemes

**Statement**: any pilot that introduces a bit-pattern-tagging scheme over a floating-point or other special-value-bearing type MUST include an adversarial unit-test pass covering ALL special values of the underlying type before the design closes. For IEEE 754 doubles: ±0, ±∞, qNaN, sNaN, denormals at the boundary, signed-zero distinction.

**Predicts**: bit-pattern schemes shipped without adversarial-special-value testing will collide with at least one IEEE 754 special at a frequency proportional to the special's usage in the consumer corpus.

**Evidence**: VD pilot (Addendum VI; Finding VIII.1 — -∞ tag=0 reservation collision surfaced during VD-EXT 2 implementation).

---

## Rule 13 — Revert-then-deeper-layer-closure discipline

**Statement**: when a substrate-introduction round produces a NEGATIVE empirical result:

1. **Verify the negative** — re-measure; confirm not noise.
2. **Diagnose structurally** — name WHY the round added cost without benefit. Is it design (wrong-lifetime cache; wrong-receiver-shape detection; wrong-cost-axis target)? Or implementation?
3. **Revert** the negative round's code via git (keep the trajectory entry + diagnosis).
4. **Identify the deeper-layer closure** that the negative round's design pointed toward but didn't reach. Often the negative is the substrate-introduction at the wrong layer; the deeper layer is the actual closure tier.
5. **Implement the deeper-layer closure** as the next round. Per Doc 740 §IV.2 + §II.2 P4: cumulative reclaim materializes at the deeper-layer closure.

**Predicts**: substrate-class abandonment after a single negative result misses the cumulative-materialization opportunity at the deeper layer. The opposite — applying rule 13 prospectively (designing from the deeper layer first when conditions hold) — produces ≤3-round closures per the prospective-application thesis.

**Evidence**: IHI-EXT 7→11 trajectory (Addendum IX); subsequent prospective corroborations at GPI, IPBR, TRGC, TRMLE, TROI (cf. `apparatus/docs/standing-rule-13-prospective-application.md`). **12 prospective-application corroborations as of 2026-05-24**.

---

## Rule 14 — Conservative-strip discipline (cost asymmetry)

**Statement**: when adding a substrate strip / classification heuristic, prefer false-negatives over false-positives by design. Bail conditions should be conservative; require all positive evidence (multiple gating predicates) before firing.

**Predicts**: false-positive over-stripping silently regresses previously-OK files; the instrument may not surface the regression without targeted regression testing. False-negative under-stripping surfaces at the next re-measurement (failure category remains visible).

**Evidence**: TRGC-EXT 2 mid-round regression (70.1% → 67.4% → corrected to 70.9% by tightening gating); TRMLE first-cut over-match recovered after tightening. **The cost asymmetry favors conservative strips**.

---

## Rule 15 — Chapter-close-inspect

**Statement**: at every chapter close, inspect the post-fix failure table's top rows before declaring the locale fully closed. If the top tag's actual cause (per example inspection) differs from the planned scope, the round is not done.

**Predicts**: chapter-close declarations made without failure-table top-row inspection will leave higher-impact substrate gaps undiscovered. Conversely, applying the rule routinely surfaces the next round's load-bearing gap as a mid-round discovery (the inspect-then-iterate compound-discovery pattern).

**Evidence**: Reproduced 9 times across the TS-parity arc (TRSLS, TRCAPS, TRGC×6 follow-ons, TRE, TROI, long-tail singletons). Standing observation: each reproduction delivered a higher-impact mid-round-discovery fix than the planned-scope fix. Cumulative empirical efficiency: ~2× vs spec-driven planning.

---

## Rule 23 — Founding baseline-inspection rung (locale-as-probe discipline)

**Statement**: at EXT 0 founding, before declaring the substrate move-shape, MEASURE the locale's failure-shape against current cruft + INSPECT a sample of failures to verify the substrate move actually lives at the seed-declared coordinate. If baseline-inspection reveals the move-shape is at a DIFFERENT coordinate, treat the locale as a probe that surfaced the real target — land the surfaced-coordinate move first; treat the spawned locale's pool as the validating test surface.

**Predicts**: locales that skip baseline-inspection at founding and proceed directly to substrate work at the seed-declared coordinate will produce wasted cycles in proportion to how often the substrate move actually lives at an adjacent or upstream coordinate. Rule 23 catches such cases at founding (when the inspection cost is ~5-10 minutes) instead of post-hoc (when the wasted-substrate-work cost is hours-to-days).

**Evidence**: NLC-EXT 0 founding 2026-05-25 — baseline 104/157 + ~10-minute inspection of 20 fails surfaced an engagement-wide eval-error-class wrapping root cause; all 5 cluster-coherence-multiplier conditions held at the locale's seed coordinate, but the load-bearing substrate move was at a different (runtime) coordinate. Rule 23 codifies the discipline that caught this; future locales' founding follows the same protocol.

**Composes with**: Rule 11 (5-axis pre-spawn — BEFORE-spawn; rule 23 is AT-spawn); Rule 13 prospective application (when rule 23 surfaces a different coordinate, R13's C1-C4 check runs against the surfaced coordinate); heuristics §V row-coherence (baseline-inspection IS row-coherence applied at founding); Doc 711 dyadic-ascent (locale-as-probe is the rung-2/rung-1 check applied to a single locale at founding).

**Operational integration**: `apparatus/docs/repository-apparatus.md` §IV.Locale-spawn-protocol step 5 (added 2026-05-25 same commit) names this rung as the inspection step inserted between seed-creation and EXT 1 substrate work.

---

## Standing instruments (rule-supporting apparatus)

The ruleset is supported by standing instruments that rules 5, 10, 11, 14, 15 explicitly invoke:

| Instrument | Path | Supports |
|---|---|---|
| Multi-run bench harness | `pilots/apparatus/cross-runtime-bench/scripts/run-bench.sh` | Rules 1, 2, 3 |
| Canonical fuzz | `pilots/rusty-js-shapes/consumer-migration/fixtures/fuzz-canonical.mjs` | Rules 5, 10 |
| JSF component-A/B probe | `pilots/rusty-js-json-fast/fixtures/component-ab-probe.mjs` | Rule 11 (A1) |
| `string_url_sweep` component-A/B probe | `pilots/apparatus/cross-runtime-bench/fixtures/string_url_sweep/component-ab-probe.mjs` | Rule 11 (A1) |
| TCC parse-parity corpus | `pilots/apparatus/ts-consumer-corpus/` | Rules 14, 15 (TS-tier) |
| TXC execute-parity corpus | `pilots/apparatus/ts-execute-corpus/` | Rules 14, 15 (TS-tier) |
| diff-prod | `scripts/diff-prod/` | Cross-cutting correctness gate (all rules) |
| test262 sample | `scripts/test262-sample/` | Cross-cutting correctness gate |
| Locale manifest | `apparatus/locales/manifest.json` | Rule 11 spawn-discipline anchor |
| Locale candidates queue | `apparatus/locales/CANDIDATES.md` | Rule 15 prioritization anchor |

---

## Rule composition + dependencies

```
Rules 1-3     measurement discipline
   ↓
Rule 4        landing discipline
   ↓
Rules 5-10    correctness gates at default-on / data-storage changes
   ↓
Rule 11       pilot-spawn discipline (5-axis)
   ↓
Rule 12       bit-pattern scheme discipline
   ↓
Rule 13       revert-then-deeper-layer (negative-result discipline)
   ↓
Rules 14-15   substrate-heuristic discipline (TS-parity arc surfaced)
   ↓
Rule 23       founding-baseline-inspection (locale-as-probe; tokenization-above-IR arc surfaced)
```

Rules at higher levels compose with prior rules:
- Rule 5 + Rule 10 — three-probe-levels at default-on; canonical fuzz is the standing instrument for the fuzz probe level.
- Rule 11 + Rule 13 — A1 (component A/B) identifies the dominator; rule 13's prospective application designs the closure from the deeper layer first when C1-C4 hold (see `apparatus/docs/standing-rule-13-prospective-application.md`).
- Rule 14 + Rule 15 — conservative-strip + chapter-close-inspect together produce the inspect-then-iterate compound-discovery pattern that Doc 541 Appendix E identifies as a SIPE-T instance.
- Rule 11 + Rule 23 — rule 11 is BEFORE-spawn (where to spawn); rule 23 is AT-spawn (verify the spawned coordinate is the substrate target). Together they bracket the spawn moment with a pre- and post-check, catching mis-spawns at both ends of the founding boundary.
- Rule 23 + Rule 15 — both are inspection-based; rule 23 inspects at founding, rule 15 at chapter-close. Together they constitute the "inspect-twice" discipline (founding + closing) that bookends the locale's lifecycle.

---

## Predictive coverage map

What the ruleset, taken together, claims to PREVENT:

| Bug class | Rules that prevent it |
|---|---|
| Noise-dominated measurement claims | 1, 2, 3 |
| Half-landed substrate moves | 4 |
| Default-on regressions | 5, 10 |
| Consumer-tier storage bugs | 6 |
| Cascade over-projection | 7, 11 (A2-A5) |
| CRB-vs-bench mis-prioritization | 8 |
| Raw-pointer SEGFAULTs | 9 |
| IEEE 754 special-value collisions | 12 |
| Substrate-class abandonment after negative | 13 |
| Silent false-positive substrate regressions | 14 |
| Premature chapter close | 15 |
| Wrong-coordinate substrate work (locale at X, move-target at Y) | 23 |

What the ruleset DOES NOT yet cover (open territory for future rules):
- Cross-substrate-tier dispatch contracts (Doc 742's O1/O2/O3 — corpus-tier articulation, not yet a rule)
- ESM live-binding cycle handling (the TROI fix removed the symptom but not the underlying capability)
- Runtime-bearing TS construct lowering (enums, decorators, ctor-param shorthand)
- Test262-sample regression detection at sub-1% granularity (rule 3's detectability-budget floor)

---

## Update protocol

Per Doc 727 §X basin-stability discipline: **append only**. New rules become rule 16, 17, … with their own evidence pointer. Existing rules are never edited; if a rule turns out to be wrong, a new rule documents the correction with a back-reference. The findings.md addendum cycle is the canonical mechanism for adding rules; this doc is updated as a consolidated derived view.

When adding a rule:
1. Land the rule + evidence in `pilots/rusty-js-jit/findings.md` as the next Addendum.
2. Update this doc's rule list, instrument table, composition diagram, and predictive-coverage map.
3. If the rule's evidence is corpus-published, add the Doc-N pointer.

---

## Cross-corpus references

- **Doc 540** — Pin-Art apparatus formalization (the substrate methodology this ruleset operationalizes).
- **Doc 541 Appendix E** — SIPE-T scale-invariance; identifies the inspect-then-iterate compound-discovery pattern (rules 14+15) as a SIPE-T instance.
- **Doc 727** — basin-stability discipline; the append-only update protocol.
- **Doc 729** — resolver-instance pattern; the architectural target this ruleset's substrate work serves.
- **Doc 740 / 741** — multi-tier cascade-revival; rule 13's theoretical anchor + empirical materialization.
- **Doc 742** — resolver-instance boundary contract; the corpus-tier consolidation of rules 14+15's TS-parity arc outcomes.

---

*This doc is a consolidated view; `pilots/rusty-js-jit/findings.md` is canonical. Last full consolidation: 2026-05-24 post-Addendum X (15 rules). Last partial consolidation: 2026-05-25 post-Addendum XIV adds Rule 23 only; Rules 16-22 (Addenda XII + XIII) await consolidation pass.*
