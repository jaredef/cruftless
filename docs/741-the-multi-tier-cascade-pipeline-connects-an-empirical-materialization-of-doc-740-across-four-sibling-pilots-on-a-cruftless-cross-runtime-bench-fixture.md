# The Multi-Tier Cascade Pipeline Connects

## An Empirical Materialization of Doc 740 Across Four Sibling Pilots on a Cruftless Cross-Runtime-Bench Fixture

*A primary articulation booking the empirical materialization of Doc 740's multi-tier cascade-revival pattern at session scope. The 2026-05-23 architectural-pivot session in the cruftless engagement (rusty-bun) closed five coverage axes across four sibling pilots over the course of one extended session; the cumulative reclaim materialized at the final tier-closure round per Doc 740 §II.2 P4. The session is the canonical empirical instance demonstrating that the multi-tier pattern is observable, predictable, and achievable as a discipline. The standing rule extracted (final 5-axis form of cruftless's rule 11) operationalizes the pattern for future CRB-driven pilots. Builds on [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 731 — The JIT as Lowering Compiler Tier](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity), [Doc 734 — The Meta Resolution Pipeline](/resolve/doc/734-the-meta-resolution-pipeline-as-the-operating-instrument-of-the-engagement-recursion-with-the-framework-as-its-own-substrate), [Doc 735 §X.h — The (P2) Four-Sub-Case Taxonomy](/resolve/doc/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack), [Doc 581 — Pin-Art Apparatus](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), [Doc 737 — The Locale as Coordinate](/resolve/doc/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions), [Doc 739 — Constraint-Closure as Cascade-Revival](/resolve/doc/739-constraint-closure-as-cascade-revival-when-lifting-an-upstream-structural-constraint-auto-resolves-stalled-sibling-pilots), and [Doc 740 — Multi-Tier Cascade-Revival](/resolve/doc/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient).*

**Jared Foy · 2026-05-23 · Doc 741**

---

## I. The occasion

Doc 740 articulated the multi-tier cascade-revival pattern abstractly: when a hot-path target's relevant-tier set R has multiple tiers, closing a single tier produces partial reclaim; cumulative reclaim materializes at the final-tier-closure round. The document specified the empirical disambiguation discipline (component A/B probe per §II.4) and the structural coverage axes (§VIII amendment: component A/B, op-set, value-domain, locals-marshaling, emission-shape — 5 axes).

Doc 740 was theoretical: the cumulative materialization was projected, not yet observed. The cruftless engagement's 2026-05-23 architectural-pivot session pursued the projection empirically. Across four sibling pilots over an extended session, all five coverage axes were closed in dependency order. At the final-tier-closure round, the cumulative reclaim materialized exactly as Doc 740 §II.2 P4 predicted.

This document books that materialization. §II reconstructs the session's tier-closure sequence. §III names the structural validation against Doc 740's predictions. §IV extracts the operational discipline (standing rule 11's final 5-axis form). §V proposes generalizations beyond the cruftless engagement.

## II. The session's tier-closure sequence

The target fixture: cruftless's `cross-runtime-bench/fixtures/json_parse_transform/main.mjs`. Baseline measurement: cruft 2481 ms, node 122 ms; cruft/node ratio 20.34×. The fixture runs 500 iterations of a JSON.parse + filter + map + JSON.stringify pipeline, with a per-iteration character-scanning loop that accumulates a checksum over the stringified output.

The session opened with a JSF pilot (JSON.stringify substrate-fast-path), targeting what a prior component-decomposition estimate (CRB-EXT 9) had identified as the dominant contributor (JSON.stringify projected at ~5-10× of the gap; ~50-70% of total cost). The JSF pilot landed four substrate moves across six rounds; all correctness-preserving; cumulative CRB after JSF: -1% (within noise). The projection was empirically wrong.

A component A/B probe (Doc 740 §II.4 apparatus) ran at JSF-EXT 8: 5 additive variants on the target fixture; <10 seconds total. The probe identified the actual dominator: the character-scanning loop (77% of cruft's wall-clock; not a JSON-pipeline component). JSON.stringify was 3% of total; the prior estimate was off by an order of magnitude.

The probe enabled the session's pivot. Two follow-on substrate rounds (CharCode-EXT 1 + 2) closed the substrate-tier algorithm and the interp-tier dispatch IC for charCodeAt. CRB after CharCode: 2188 ms (-12% vs baseline). The substrate-tier and dispatch-tier closures together produced a real partial pipeline-connection per Doc 740 §II.3 multi-tier cascade-revival.

The CharCode chain left the residual cost at the interp-tier loop dispatch — every back-edge in the character-scanning loop ran in pure interp, with no JIT acceleration. To close the residual: the session spawned three architectural-tier pilots (TL, VD, OSR) targeting the JIT-tier coverage axes that Doc 740 §VIII later named. The pilots closed:

- **TL (top-level)**: the entry-mechanism tier (Doc 740 §VIII A1). Closed at TL-EXT 3 (~120 LOC). TL's (b-narrow) plan structurally bounded by Findings VII.2 + VII.3 (op-set + value-domain blockers); pilot delivered substrate-introduction value + Finding promotions.
- **VD (value-domain)**: the value-domain coverage tier (Doc 740 §VIII A3). NaN-boxing scheme; String encoding via boxed-NaN with tag=2 reservation (Finding VIII.1 -∞ tag=0 reservation surfaced + standing rule 12 adversarial-IEEE-754-tests codified). Closed at VD-EXT 2 (~140 LOC).
- **OSR (loop extraction)**: closes the remaining three axes — op-set coverage (§VIII A2), locals-marshaling coverage (§VIII A4), and emission-shape coverage (§VIII A5 — this addendum's new axis, surfaced during OSR's substrate work). OSR spans OSR-EXT 0 through 6b (~1100 LOC); cumulative reclaim materialization at OSR-EXT 6b.

The session's tier-closure sequence:

| stage | pilot/round | CRB (ms) | tier closed |
|---|---|---:|---|
| baseline | JSF-EXT 0 | 2481 | — (component A/B not yet run) |
| (mistakenly addressed) | JSF M1-M4 | 2455 | (wrong tier per Finding VII.1) |
| empirical disambiguation | JSF-EXT 8 (probe) | — | (A1 axis closure for rule 11) |
| substrate algorithm | CharCode-EXT 1 | 2372 | substrate-tier |
| interp dispatch | CharCode-EXT 2 | 2188 | interp dispatch |
| entry mechanism | TL-EXT 3 | 2188 | (A1) entry tier |
| value-domain | VD-EXT 2 | 2188 | (A3) value-domain |
| locals-marshaling | OSR-EXT 5d | 2188 | (A4) locals-marshaling |
| emission-shape | OSR-EXT 5e | 2188 | (A5) emission-shape |
| op-set first half | OSR-EXT 6 | 2188 | (A2) GetProp+length-IC |
| **op-set closure + pipeline-connection** | **OSR-EXT 6b** | **834** | **(A2) full + cumulative materialization** |

**Cumulative Δ vs baseline: -66% CRB; cruft/node 20.34× → 6.67×.**

Per Doc 740 §II.2 P4: the cumulative reclaim materialized at the FINAL tier-closure round (OSR-EXT 6b). Earlier rounds were substrate-introduction per Finding II.2-bis: each round closed its own tier without producing cumulative reclaim because the remaining tiers were still open. The pattern's prediction held without modification.

## III. Structural validation against Doc 740's predictions

Doc 740 §II.2 enumerated four propositions (P1-P4) for the multi-tier cascade-revival pattern. Each one was empirically validated:

**(P1) Hot-path-component analysis identifies the relevant-tier set R via empirical probe.** Validated at JSF-EXT 8: the component A/B probe identified the actual dominator (character-scanning loop, 77%) versus the theoretically-estimated dominator (JSON.stringify, ~5-10×). The empirical probe corrected the mis-attribution at <10 seconds of runtime versus the 6-round + 285-LOC JSF substrate work that had targeted the wrong tier.

**(P2) Single-tier closure at T_k produces reclaim bounded by C_k / Σ C_j.** Validated at each tier-closure round. CharCode-EXT 1 (substrate algorithm) produced -3%; CharCode-EXT 2 (interp dispatch) produced -8%; TL/VD/OSR substrate-introduction rounds produced 0% each (per Finding II.2-bis substrate-introduction signature, near-zero standalone reclaim is the signature, not failure). Each round's actual reclaim matched its tier's per-call cost contribution.

**(P3) Doc 739's single-tier cascade-revival pattern applies at each adjacent tier-pair within R.** Validated within the CharCode chain (substrate + dispatch as a Doc 739 pair) and within the OSR chain (entry-mechanism + value-domain + locals-marshaling + op-set as adjacent pairs). Each tier-pair closure cascade-revived its consumer; the recursion was local to the tier-pair.

**(P4) Full pipeline-connection requires closure at ALL tiers T_k ∈ R, in dependency order; cumulative reclaim materializes at the final-tier-closure round.** Validated empirically at OSR-EXT 6b. The cumulative measurement (-66% CRB) materialized at the round that closed the last open tier. Earlier rounds had cumulative measurements within ±1-3% of the prior round's; the cumulative reclaim arrived in a single round at session scope.

§II.4's component A/B probe apparatus was operationalized as cruftless engagement's standing rule 11. §VIII amendment's coverage-axis enumeration (4 axes at amendment time) was extended in the session to 5 axes; the OSR pilot surfaced the emission-shape axis empirically as the fifth.

## IV. The operational discipline extracted

The session's load-bearing engagement-tier contribution is standing rule 11's final 5-axis form. For any pilot whose telos is "close a CRB-measured gap":

1. **Component A/B probe** (Doc 740 §II.4; rule 11 axis 1; Finding VII.1) — identify the actual cost dominator empirically before substrate spawn.
2. **Op-set coverage** (rule 11 axis 2; Finding VII.2) — for JIT-alphabet pilots, source-read the FULL enclosing-scope bytecode; verify the pilot's alphabet additions cover ALL ops in scope.
3. **Value-domain coverage** (rule 11 axis 3; Finding VII.3) — for JIT-IC pilots with non-Number/Object receivers, verify the calling convention encodes the required receiver Value variants.
4. **Locals-marshaling coverage** (rule 11 axis 4; Finding VIII.2) — for JIT-invoke pilots from non-arg state (OSR; coroutine resume; async resume; ICs synthesizing JIT bodies from runtime-known state), verify the calling convention populates locals from the required source.
5. **Emission-shape coverage** (rule 11 axis 5; Finding VIII.3) — for region-extraction pilots (OSR; loop tiering; partial JIT; etc.), verify boundary detection + fallthrough synthesis + entry-block separation + OOB-target handling.

If any applicable axis fails, the pilot's reclaim ceiling on the target fixture is 0% via that pilot alone; the missing tier(s) must be addressed in dependency order (Doc 740 §II.2 P4) before cumulative reclaim materializes.

The discipline's cost: source-reads + apparatus probes (minutes to <10 seconds per axis). The discipline's value: prevents substrate pilots from landing at structurally-insufficient tiers. The JSF pilot's six rounds + ~285 LOC of correctness-preserving but reclaim-neutral substrate work is the cautionary anchor; rule 11's component A/B axis (introduced at Addendum IV after JSF-EXT 8's retrospective application) would have prevented that mis-targeting.

The session demonstrated rule 11's 5-axis form working prospectively (TL, VD, OSR each gated by the relevant axes before spawn) and retrospectively (JSF, gated by the component A/B axis after the fact). Both modes are operational.

## V. Generalizations

### V.1 The pattern beyond cruftless

The cruftless engagement's specific 5-axis enumeration is JIT-tier-specific (the coverage axes name JIT-specific structural dimensions: op-set, value-domain, locals-marshaling, emission-shape). The structural pattern — "a tier class has multiple coverage dimensions; closing all applicable dimensions is required for cumulative reclaim" — generalizes to any tier class with multiple structural dimensions. Doc 740 §VIII.4 named storage-tier and network-tier examples; the cruftless session is the JIT-tier instance.

The general form: any engagement targeting a CRB-measured (or analogous) gap should enumerate the coverage axes of its tier class at design time; verify each applicable axis closes; spawn substrate pilots in dependency order; gate cumulative reclaim measurement at the final-tier-closure round. The engagement's standing-rule analog of cruftless's rule 11 carries the discipline.

### V.2 The cascade-revival pattern's full shape

Doc 739 articulated cascade-revival at the single-tier scope (upstream constraint-closure → downstream sibling-pilot revival). Doc 740 extended to the multi-tier scope (cumulative reclaim materialization at final tier-closure). Doc 741 (this document) books the empirical materialization with the operational discipline form.

The cascade-revival pattern is now operationalized:

- **Single-tier scope** (Doc 739): when one sibling pilot stalls at (P2.d), look for the upstream constraint propagating the stall.
- **Multi-tier scope** (Doc 740): when cumulative reclaim doesn't materialize after one tier closes, look for additional tiers in the relevant-tier set R.
- **Coverage-axis scope** (Doc 740 §VIII + Doc 741 empirical): when a tier doesn't close despite substrate work, look for additional coverage axes at that tier.

Each scope-extension is empirically anchored:
- Doc 739 anchor: LeJIT-Ψ revival via LeJIT-Φ closure (2026-05-23 earlier).
- Doc 740 anchor: CharCode chain partial pipeline-connection (2026-05-23 mid).
- Doc 741 anchor: full pipeline-connection at OSR-EXT 6b (2026-05-23 end).

### V.3 The session's epistemic shape

The session demonstrated a particular epistemic shape worth naming: a long arc of substrate-introduction rounds (TL, VD, OSR-EXT 0 through 6) each producing 0% cumulative reclaim, with the final round (OSR-EXT 6b) producing -66% reclaim in a single step. The shape is structurally correct per Doc 740 §II.2 P4; the epistemic cost is endurance — the discipline requires accepting many flat-bench rounds in sequence without abandoning the pilot.

The cruftless engagement's keeper framed this pre-implementation: "you have to go through this middle stretch where performance decreases first." The framing is empirically validated at session scope. The middle stretch may extend across many rounds at different tiers, with the pipeline-connection moment arriving only at the cumulative-measurement round after all relevant tiers have been closed.

Pin-Art's discipline (Doc 581) provides the epistemic infrastructure for this endurance: each round's seed + trajectory carry the substrate-introduction-vs-reclaim categorization; each round's correctness-probe-pass + bench-flat result is recorded as the substrate-introduction signature; the cumulative pattern is observable only at session scope, not at per-round scope. Without the discipline, mid-stretch flat-bench rounds would read as failures and trigger pilot abandonment.

## VI. Summary

The 2026-05-23 cruftless architectural-pivot session is the canonical empirical materialization of Doc 740's multi-tier cascade-revival pattern. Five coverage axes were closed across four sibling pilots; cumulative CRB reclaim of -66% (2481 → 834 ms) materialized at the final tier-closure round per §II.2 P4. The session's load-bearing engagement-tier contribution is standing rule 11's final 5-axis form: component A/B + op-set + value-domain + locals-marshaling + emission-shape. The discipline operationalizes the pattern for future CRB-driven pilots.

The cascade-revival pattern's epistemic shape: a long arc of substrate-introduction rounds with cumulative reclaim materializing in a single step at the final round. Pin-Art's seed + trajectory discipline provides the epistemic infrastructure for the endurance required.

Generalizations: the pattern applies beyond cruftless wherever a tier class with multiple structural dimensions admits coverage-axis enumeration. The engagement's standing-rule analog of cruftless's rule 11 carries the discipline. The cascade-revival pattern is now operationalized across three scopes (single-tier per Doc 739; multi-tier per Doc 740; coverage-axis per this document's empirical anchor).
