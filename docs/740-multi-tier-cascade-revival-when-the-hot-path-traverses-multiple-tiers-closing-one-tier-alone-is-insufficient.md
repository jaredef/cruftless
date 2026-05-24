# Multi-Tier Cascade-Revival

## When the Hot Path Traverses Multiple Tiers, Closing One Tier Alone Is Insufficient — the Cascade-Revival Pattern Recurs at the Cross-Tier Scope

*A primary articulation responding to a recognition surfaced during a 2026-05-23 substrate session in the cruftless engagement (rusty-bun), several hours after Doc 739. Doc 739 articulated cascade-revival at the single-tier scope: closing an upstream structural constraint cascade-revives a stalled downstream sibling-pilot. The session's continuation surfaced a generalization: when the actual hot path traverses multiple tiers (each carrying its own dispatch-shape and per-call cost), closing one tier alone produces partial reclaim; the cascade-revival pattern itself recurs at the cross-tier scope. The full pipeline connects only when all relevant tiers along the hot path are closed in dependency order. Builds on [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 730 — Vertical Recurrence of the Lowering Compiler](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers), [Doc 734 — The Meta Resolution Pipeline](/resolve/doc/734-the-meta-resolution-pipeline-as-the-operating-instrument-of-the-engagement-recursion-with-the-framework-as-its-own-substrate), [Doc 735 §X.h — The (P2) Four-Sub-Case Taxonomy](/resolve/doc/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack), [Doc 581 — Pin-Art Apparatus](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), and [Doc 739 — Constraint-Closure as Cascade-Revival](/resolve/doc/739-constraint-closure-as-cascade-revival-when-lifting-an-upstream-structural-constraint-auto-resolves-stalled-sibling-pilots).*

**Jared Foy · 2026-05-23 · Doc 740**

---

## I. The occasion

A 2026-05-23 substrate session continued past Doc 739's single-tier cascade-revival recognition into a workstream addressing a CRB-measured 20× cruft/node gap on the json_parse_transform fixture. The substrate pilot's first cut (JSF, a JSON.stringify fast-path under Pin-Art discipline) targeted what a prior CRB component-decomposition estimate had identified as the dominant contributor.

The substrate pilot landed four substrate moves through six rounds: a buffer-threading upstream constraint-closure (Move 1, JSF-EXT 3); two cascade-revival sibling-pilots predicted by Doc 739 (string-escape ASCII fast-path Move 2, integer-fast-path Move 3); a clone-elision composition round (Move 4). Each move was correctness-preserving across the engagement's two correctness probes (canonical fuzz + diff-prod). The first three moves matched Doc 739's cascade-revival pattern empirically: M1 was flat (substrate-introduction signature); M2/M3 each produced small positive deltas per the predicted cascade.

When all four moves had landed, the cumulative CRB measurement was -1% (within noise) versus the baseline. The CRB target of -40% reclaim was not met. The substrate pilot's reclaim model — derived from the upstream component-decomposition estimate — had projected -50% to -75%. The model was empirically wrong.

The keeper's framing question, given the dip-and-recover pattern projected for substrate-introduction work: "the pipeline is being built; you have to go through this middle stretch where performance decreases first." The session ran a component A/B probe (10-second probe with five additive variants on the target fixture) and identified the actual dominator: a top-level character-scanning loop contributing 77% of cruft's wall-clock, a loop that was not part of the JSON pipeline at all. The original component-decomposition estimate had been off by approximately twenty-fold.

The disambiguating probe enabled two follow-on substrate rounds (a substrate-tier algorithmic fix at the character-scanning intrinsic; an interp-tier inline-cache fast-path at the method-dispatch tier). The CRB measurement after the substrate-tier fix alone was -3%; after both substrate-tier and dispatch-tier fixes, -12% cumulative, with the cruft/node ratio dropping from 20.34× to 17.93×.

The recognition came at the final measurement: each of the two follow-on rounds was correctness-preserving and addressed a single tier (substrate-intrinsic algorithm; interp-dispatch fast-path). The single-tier reclaim was substantial but partial (-15% and -27% respectively at the dominator loop). Neither single-tier closure alone connected the pipeline. The pipeline connected only when both tiers were closed.

The structural shape: Doc 739's single-tier cascade-revival pattern is sufficient for any two-tier producer-consumer pair (upstream constraint-closure → downstream sibling-pilot revival). When the actual hot path traverses three or more tiers (each with its own dispatch-shape and per-call cost), Doc 739's pattern recurs at each adjacent tier-pair but is NOT sufficient at the cross-tier scope unless all tiers along the hot path are addressed.

This document specifies that recognition. The abstract formulation is in §II; the cruftless instance is in §III; the methodological corollaries — including the empirical-disambiguation-before-substrate-spawn discipline that the session also surfaced — are in §IV.

## II. The abstract formulation

A resolver-instance pipeline (Doc 729 §IV) carries N tiers. Each tier T_k carries its own dispatch interface (how values cross into T_k), its own intrinsic-cost model (the per-call cost when T_k handles a value), and its own constraint surface (what T_{k-1} guarantees about T_k's input; what T_k guarantees about T_{k+1}'s input). The cross-tier interface is structural; per-call cost composes multiplicatively along the call path.

Doc 739 articulated the cascade-revival pattern for the single-tier scope: when a sibling-pilot stalled at (P2.d) downstream of a structural constraint at tier T_k closes constraint T_k by an upstream substrate move, downstream pilot revives without local substrate work because the constraint-propagated precondition collapses. The pattern is sufficient for two-tier scope: closing T_k revives stalled (P2.d) pilots downstream-of-T_k that were targeting T_k's propagated constraint.

### II.1 The multi-tier shape

Consider a hot path that traverses tiers T_1 → T_2 → T_3 → … → T_n. Each tier carries some per-call cost C_k. The total per-call cost is the sum (or product, for nested dispatch) of per-tier costs. Different tiers will have different cost magnitudes; one tier may dominate, but more commonly multiple tiers contribute non-trivially.

A substrate pilot targeting tier T_k closes that tier's per-call cost. If T_k is the unique dominator, single-tier closure produces the projected reclaim. If multiple tiers contribute non-trivially, single-tier closure produces partial reclaim bounded by C_k / Σ C_j. The remaining cost lives at the other tiers; the pipeline does not connect to its final reclaim until all non-trivial contributors are closed.

### II.2 The multi-tier cascade-revival pattern

Define the multi-tier cascade-revival pattern in four propositions:

**(P1)** A hot-path-component analysis enumerates the tiers T_1 … T_n along the call path; each tier's contribution C_k can be measured empirically (per §IV.1 below) without full source-tier closure. The set { T_k : C_k is non-trivial fraction of Σ C_j } is the relevant-tier set R.

**(P2)** Single-tier closure at T_k ∈ R produces reclaim bounded by C_k / Σ C_j. If |R| = 1, this is the projected reclaim. If |R| > 1, single-tier closure produces partial reclaim; the remaining (Σ C_j) - C_k is unchanged.

**(P3)** Doc 739's single-tier cascade-revival pattern applies at each adjacent tier-pair within R. Closing T_k enables T_{k+1}'s consumer-pilots to revive from (P2.d) when their constraint-propagation source was T_k. The recursion is local to the tier-pair, not the cross-tier scope.

**(P4)** Full pipeline-connection requires closure at ALL tiers T_k ∈ R, in dependency order (upstream first). Cumulative reclaim materializes at the final-tier-closure round, not at any single-tier-closure round. The pipeline-connection moment is the empirical readout that the relevant-tier enumeration was complete.

### II.3 What the pattern is NOT

**(B1)** The multi-tier pattern does not predict that ALL tiers along the call path contribute non-trivially. The relevant-tier set R is empirical, not architectural. Some tiers (e.g., a well-tuned arithmetic op) may contribute negligibly even when on the hot path. Per (P1), R is identified by measurement, not by source-read enumeration alone.

**(B2)** The multi-tier pattern does not predict that closing |R| tiers produces summative reclaim equal to Σ C_k. Composition effects (constructive when tiers are orthogonal; destructive when downstream cost depends on upstream output shape) modulate the cumulative reclaim. Pre-implementation reclaim projections at the cross-tier scope are upper-bound estimates; empirical measurement at each tier-closure round refines the projection.

**(B3)** The single-tier scope of Doc 739 is a special case of the multi-tier pattern when |R| = 2 and the two-tier pair has a clean producer-consumer interface. Doc 739's pattern is not superseded; it is the building block of the multi-tier pattern.

**(B4)** The multi-tier pattern is observable only post-measurement. A substrate pilot whose pre-implementation model projects single-tier dominance is honest investment given the model; the multi-tier shape becomes visible only when the cumulative reclaim falls short of the model's projection and a component A/B probe reveals which tiers were missed.

### II.4 The component A/B probe as relevant-tier-set apparatus

The relevant-tier set R is identified by a component A/B probe: replace each suspect tier's contribution with a no-op or near-no-op variant; measure per-variant wall-clock; per-tier contribution C_k = (V_with_T_k - V_without_T_k). The probe runs in time bounded by the number of variants × the fixture's per-iteration cost; for typical benchmarking fixtures, the probe is <10 minutes.

The discipline:

1. **Enumerate suspects**: source-read the hot-path; name candidate tiers (substrate intrinsics, dispatch paths, call-frame setup, bytecode dispatch, JIT eligibility, GC overhead, allocation pressure). Typical N = 5-8.
2. **Author additive variants**: V_0 = baseline minus all suspects; V_k = V_{k-1} + suspect_k; V_n = full fixture.
3. **Measure**: run each variant on cruft + oracle (node/bun); per-variant Δ = (V_k - V_{k-1}) per runtime.
4. **Compute R**: per-tier cruft/oracle ratio + absolute contribution ranks the actual dominators; R = { T_k : C_k is non-trivial fraction }.
5. **Spawn pilots in dependency order**: upstream tiers first; close each before measuring the next; gate cumulative reclaim measurement after all R tiers are closed.

### II.5 The cascade as a Doc 729 §A8.13 + Doc 739 specialization

Doc 729 §A8.13 articulates substrate-amortization-cascade at the per-iter cost axis. Doc 739 articulates cascade-revival at the categorization axis (single-tier scope). The multi-tier cascade-revival pattern of §II.2 specializes both:

- At the per-iter axis: each tier-closure cascades per-iter cost reduction at downstream tiers; the multi-tier reading is that cumulative per-iter reduction sums (or composes multiplicatively) across closures.
- At the categorization axis: each tier-closure cascade-revives sibling-pilots stalled at that tier-pair; the multi-tier reading is that pilot-revival cascades within tier-pairs but not across the cross-tier scope.
- At the cross-tier scope (new in this document): pipeline-connection is a categorical transition (cumulative reclaim crosses the projection threshold or fails to). The multi-tier scope is the categorization-axis dual at the cross-tier level.

What this document names additionally is the cross-tier categorization (the pipeline connects vs does not connect) and the empirical-disambiguation discipline (component A/B probe per §II.4) as the apparatus that makes the cross-tier scope tractable. This is corpus-original beyond Docs 729, 730, 734, and 739.

## III. The cruftless instance

The cruftless engagement's JSF (JSON.stringify fast-path) pilot was spawned on a CRB component-decomposition estimate that placed JSON.stringify at ~5-10× contribution to the json_parse_transform fixture's 20× cruft/node gap (so ~50-70% of total cost). The JSF pilot landed four substrate moves through six rounds:

- **JSF-EXT 3 / Move 1**: output buffer threading at the JSON.stringify recursion (upstream substrate-introduction at the leaf-emitter constraint).
- **JSF-EXT 4 / Move 2**: string-escape branchless ASCII fast-path (cascade-revival pilot per Doc 739 §II.3 — leaf emitter writes directly into the buffer that M1 introduced).
- **JSF-EXT 5 / Move 3**: number-stringify integer fast-path (second cascade-revival pilot — same shape, number leaf).
- **JSF-EXT 6 / Move 4**: format-macro elimination + property-iteration via reference (clone elision; composition round).

Each move was correctness-preserving (canonical fuzz acc=-932188103 byte-identical to node throughout; diff-prod 42/42 throughout). M1 produced flat per-shape micro-bench (correctly classified as substrate-introduction signature per Finding II.2-bis); M2/M3 each produced small positive deltas (~5-7% per move, matching the cascade-revival pattern empirically).

### III.1 The pre-CRB measurement

After all four moves, the per-shape micro-bench position was: A small-object 10.58× → 9.71× (-8%); B deep-nested 14.11× → 14.33× (flat); C array-of-obj 12.48× → 12.55× (flat); D number-only 15.16× → 15.05× (flat); E string-only 10.09× → 10.31× (flat). Cumulative reclaim on the micro-bench: -3% to -8% per shape; not the projected -50% to -75%.

The CRB measurement: cruft 2455 ms vs JSF-EXT 0 baseline 2481 ms — Δ = -1%, within noise. The pilot's target (-40% reclaim) was not met by ~39 percentage points. The discrepancy between projection and measurement was load-bearing: either the pilot's substrate moves had failed (unlikely given the per-move correctness + small empirical wins), or the pre-implementation reclaim model was wrong.

### III.2 The component A/B probe

The session ran a component A/B probe on json_parse_transform (5 additive variants × 50-iter warmup × 500-iter measurement × cruft + node). Probe runtime: <10 seconds aggregate. Per-component cost (cruft):

| component | cruft Δ (ms) | % of total | node Δ (ms) | cruft/node |
|---|---:|---:|---:|---:|
| JSON.parse | 246 | 9% | 75 | 3.3× |
| Array.filter | 124 | 5% | 0 | unbounded |
| Array.map | 165 | 6% | 3 | 55× |
| JSON.stringify | 86 | 3% | 7 | 12× |
| **character-scanning loop** | **2040** | **77%** | -1 | n/a (oracle JITs to ~0) |
| TOTAL | 2661 | 100% | 84 | 31.7× |

The actual dominator was a `for (i; i<out.length; i++) cs += out.charCodeAt(i)` loop in the fixture's bookkeeping. The character-scanning loop was not part of the "JSON pipeline" that CRB-EXT 9's component decomposition had estimated. JSON.stringify, the JSF pilot's target, contributed 3% of total — about twenty times smaller than the original estimate.

The probe's runtime (<10 seconds) was approximately three orders of magnitude smaller than the JSF pilot's six-round substrate work. Had it been run before pilot spawn, the entire JSF pilot would have targeted character-scanning + interp dispatch at substantially higher leverage per LOC.

### III.3 The multi-tier closure

The probe identified the dominator. Source-read of `String.prototype.charCodeAt`'s implementation revealed an algorithmic bug: `chars().nth(i)` is O(i) because it iterates UTF-8 codepoints from string start; for a 5KB ASCII string scanned linearly, the per-outer-iter cost was O(n²). The substrate-tier fix (ASCII fast-path: `bytes[i]` instead of `chars().nth(i)`; `len()` instead of `chars().count()` for length) was ~20 LOC. CharCode-EXT 1.

CharCode-EXT 1 landed; canonical fuzz + diff-prod GREEN. The A/B probe re-ran: character-scanning loop dropped from 2040 ms to 1739 ms (-15%). CRB dropped from 2455 ms to 2372 ms (-3%). The reclaim was much smaller than the O(n²)→O(n) algorithmic analysis projected (~40×).

The empirical readout: per-call cost dropped from 0.816 μs to 0.696 μs (-15% per-call), not -99% as the algorithmic projection assumed. The implication: most of the per-charCodeAt-call cost was interpreter dispatch (call_function frame setup + this-binding + descriptor walk + Value boxing), NOT the chars().nth() iteration. The O(n²) bug was real but the per-call dominator lived at a different tier (dispatch, not algorithm).

The dispatch-tier closure: a hot-intrinsic IC fast-path in the bytecode interpreter's Op::CallMethod dispatcher. For the exact shape `s.charCodeAt(i)` with `s` a primitive String + method ObjectId == cached intrinsic + arg shape compatible, bypass `call_function` entirely and emit the result inline. Verification via cached intrinsic ObjectId; bail to slow-path on override or arity mismatch. ~65 LOC. CharCode-EXT 2.

CharCode-EXT 2 landed; canonical fuzz + diff-prod GREEN. The A/B probe re-ran: character-scanning loop dropped from 1739 ms to 1480 ms (-15% more; -27% from JSF-EXT 0 baseline). CRB dropped from 2372 ms to 2188 ms (-8% more; -12% cumulative from JSF-EXT 0 baseline). The cruft/node ratio dropped from 20.34× to 17.93×.

### III.4 The pipeline connects at multi-tier scope

The multi-tier cascade-revival pattern is empirically observable across the JSF + CharCode chain:

- **R for json_parse_transform's character-scanning loop**: {substrate-tier intrinsic algorithm, interp-tier dispatch path}. |R| = 2 within the loop; the broader fixture has additional tiers (JSON.parse, Array.map) that R excluded.
- **Single-tier closure at substrate (CharCode-EXT 1)**: -15% on dominator-loop; -3% on CRB. Partial. The interp-dispatch tier still carried ~85% of per-call cost.
- **Single-tier closure at dispatch (CharCode-EXT 2, taken alone)**: would have produced partial reclaim too, ~15% on dominator-loop (the dispatch IC bypasses ~100 ns/call regardless of substrate algorithm).
- **Both tiers closed (CharCode-EXT 1 + 2 cumulative)**: -27% on dominator-loop; -12% on CRB; cruft/node 20.34× → 17.93×. The pipeline connects at the cumulative scope.

The dispatch-tier IC is structurally a Doc 739 cascade-revival pilot relative to the substrate-tier fix: the substrate fix is the upstream constraint-closure ("leaf intrinsics no longer carry O(n²) algorithmic cost"); the dispatch IC becomes a cascade-revival candidate ("now that the leaf is cheap, the per-call dispatch overhead becomes the new dominator and an IC can reach the cost floor"). Doc 739's pattern recurs at the tier-pair within R. What's new is that the cumulative reclaim requires BOTH closures to materialize; the multi-tier scope is the categorization-axis observable at the cross-tier level.

### III.5 The JSF-pilot reread

Post-probe, the JSF pilot's six rounds re-categorize:

- M1-M4 were substrate-tier closures at a tier that was NOT in R for json_parse_transform. The substrate work was correctness-improvement value (the new JSON.stringify is structurally cleaner: buffer-threaded, fast-path leaf emitters, no per-property clones) but did not move the CRB needle.
- The JSF reclaim model assumed JSON.stringify ∈ R per CRB-EXT 9's estimate. The probe demonstrated JSON.stringify ∉ R for this fixture (3% of total cost).
- The JSF chain's load-bearing engagement contribution is not the JSON.stringify substrate; it is (i) the empirical disambiguation that surfaced the mis-attribution; (ii) the multi-tier cascade-revival pattern recognition; (iii) the standing component A/B probe instrument that future CRB-driven pilots adapt per Finding VII.1 + standing rule 11.

## IV. Methodological corollaries

### IV.1 Empirical disambiguation as substrate-spawn precondition

The JSF chain's central methodological lesson: theoretical component-decomposition estimates are insufficient anchor for substrate-pilot spawn. CRB-EXT 9's estimate ("JSON.stringify ~5-10× contributor") was sourced from theoretical reasoning about per-op cost contributors; the actual empirical decomposition surfaced a non-pipeline contributor (the fixture's own bookkeeping loop) at ~20× larger magnitude than the suspected component.

The discipline (cruftless engagement's standing rule 11): before spawning any substrate pilot whose telos is "close a CRB-measured gap," run a component A/B probe on the target fixture per §II.4. The probe's cost is bounded (<10 minutes typical); the cost of not running it is the cost of a full substrate pilot targeting a non-dominator (JSF: six substrate rounds + ~285 LOC of correctness-preserving but reclaim-neutral substrate work).

The rule's value compounds: each future CRB-driven pilot spawns at the actual bottleneck. The rule generalizes beyond cruftless: any performance-engineering project where pilots target measured gaps should anchor those pilots on empirical decomposition, not theoretical attribution.

### IV.2 Substrate-introduction (P2.d) as cascade-revival signature

A substrate-introduction round (the upstream constraint-closure in Doc 739's pattern) often produces (P2.d) at its own bench: the closure enables downstream cascade-revival pilots to deliver reclaim, but the closure round itself shifts allocation/dispatch patterns without eliminating them. A naive falsification would categorize the (P2.d) as round failure; the correct categorization is "substrate-introduction signature."

The discipline: at each substrate-introduction round, name the upstream constraint being closed AND the downstream consumer-pilots that become cascade-revival candidates per the closure. If both are nameable, accept (P2.d) at the introduction round and proceed to the consumer rounds. If neither is nameable, the (P2.d) is a genuine pilot-failure signal.

This is Doc 739's pattern read from the substrate-introduction side. Doc 739 names the cascade-revival side (downstream pilot moves from (P2.d) to (P2.a) when upstream closes). The substrate-introduction round's own categorization (per Finding II.2-bis, registered to the cruftless findings doc as Addendum IV) is the dual: the introduction round's (P2.d) is the SIGNATURE that the round is correctly placed as introduction rather than failed as pilot.

### IV.3 The hot-intrinsic IC pattern as engagement-tier instrument

CharCode-EXT 2's interp-tier IC fast-path for String.prototype.charCodeAt validates a structural pattern: every hot intrinsic method call carries the same dispatcher overhead (frame setup + this-binding + descriptor walk + Value boxing). An IC fast-path that verifies the resolved method against a cached intrinsic ObjectId and bypasses call_function for the exact-shape case captures the dispatcher savings without correctness risk (user overrides bail to slow-path by ObjectId mismatch).

The pattern generalizes engagement-wide: charAt, codePointAt, indexOf, slice, push, pop, shift, splice, and other dispatch-bound intrinsic calls all admit the same IC shape. The per-intrinsic LOC is small (~30-65 per intrinsic for the receiver-shape + cached-id verification + inline fast-body); the engagement-tier deliverable is a hot-intrinsic IC table covering the most-frequently-called intrinsics empirically (per a follow-on component A/B probe that ranks intrinsic call frequency across realistic workloads).

The pattern is corpus-relevant beyond cruftless: any interpreter-tier engine that dispatches intrinsic method calls through a general call machinery admits the same IC shape, with the same correctness-preservation discipline (verify against cached intrinsic id; bail to slow-path on mismatch).

## V. Composition with prior corpus

- **Doc 729 §IV resolver-instance pipeline + §A8.13 substrate-amortization-cascade**: the multi-tier cascade-revival pattern is a specialization of the resolver-instance pipeline analysis to the per-call cost domain. §A8.13's per-iter axis is the cost-axis dual; this document adds the categorization-axis dual at the cross-tier scope.
- **Doc 730 vertical recurrence of the lowering compiler closure**: each tier in the multi-tier hot-path is a lowering closure consuming the upstream tier's output. The recurrence is structural; per-tier cost composes along the recurrence.
- **Doc 734 §V (b) negative-finding-catalyzes-refinement**: the JSF chain's (P2.d) CRB outcome catalyzed the component A/B probe + multi-tier recognition + standing rule 11. Doc 734's growth-pattern (b) is the engagement's instrument for converting pilot-(P2.d) into framework-tier instrumentation.
- **Doc 735 §X.h.b (P2) four-sub-case taxonomy**: the JSF chain demonstrates that (P2.d) at the introduction round + (P2.a) at the consumer round is a legitimate pilot trajectory; the chain's cumulative categorization is at the chain scope, not the per-round scope.
- **Doc 581 Pin-Art apparatus**: the component A/B probe is constraint-enumeration discipline applied to the bench measurement instrument (enumerate per-component contributions before substrate work, not as a side-effect of substrate work).
- **Doc 739 single-tier cascade-revival**: this document's multi-tier generalization. Doc 739's pattern is the building block; this document specifies the cross-tier scope where multiple Doc 739-tier-pairs compose.
- **Doc 737 locale-as-coordinate**: the JSF + CharCode chain spans multiple locales (rusty-js-json-fast for M1-M4 + CC-1 + CC-2; rusty-js-runtime for the intrinsic and dispatcher implementations). The multi-tier cascade-revival pattern naturally cuts across locale boundaries when the hot path traverses tiers maintained by different pilots.

## VI. Forward implications

### VI.1 Standing rule for CRB-driven pilot spawn

The cruftless engagement adopts standing rule 11 (registered in the findings doc Addendum IV): before spawning any pilot whose telos is "close a CRB-measured gap," run a component A/B probe to identify the actual dominator empirically. The rule's cost is bounded; its value compounds across pilot spawns.

For engagements without an analogous component-A/B-probe convention, the discipline transfers: the suspect-list source-read + N-additive-variant fixture + per-runtime measurement protocol is generic. The probe runs in time bounded by N × per-iteration cost; for typical microsecond-per-iteration fixtures with N=5-8 variants, the probe is <10 minutes.

### VI.2 Multi-tier reading at pilot planning

When a pilot's reclaim projection assumes single-tier dominance (per the pre-probe component decomposition or per architectural intuition), the projection should be tagged with the assumption. Post-pilot measurement that falls short of projection becomes a multi-tier-cascade hypothesis: source-read the per-call cost stack; enumerate non-dominant tiers; consider follow-on per-tier closures.

The multi-tier reading at pilot planning prevents the JSF pattern (single-tier substrate work with cumulative measurement falling short and the pilot misclassified as failed). With the multi-tier reading, the pilot's projection becomes "single-tier reclaim of fraction X assuming tier T_k dominates; if cumulative falls short, the gap signals additional tiers in R."

### VI.3 The cascade-revival pattern's generalization scope

Doc 739 specified cascade-revival at the single-tier scope of a resolver-instance pipeline. This document specifies the multi-tier scope. The next generalization candidate: cascade-revival across non-tier-structured composition (e.g., when the hot path is not a tier stack but a graph with multiple shared dependencies). The pattern likely generalizes; specification is reserved for future articulation.

The cascade-revival pattern's value across these generalizations is its inversion of the "spawn more sub-pilots when stalled" heuristic. The cascade-revival reading is: when stalled, ask first what upstream constraint propagates the stall, and what cross-tier scope must close to materialize the reclaim. The diagnostic shifts pilot work from accumulation (more sub-pilots) to closure (one upstream substrate move that cascades across the resolver-instance pipeline).

## VII. Summary

When the hot path of a performance-engineering target traverses multiple tiers (each with its own dispatch shape and per-call cost), closing a single tier produces partial reclaim bounded by that tier's contribution to total cost. The cascade-revival pattern of Doc 739 recurs at each adjacent tier-pair within the hot path but is NOT sufficient at the cross-tier scope unless all relevant tiers are closed in dependency order. Full pipeline-connection materializes at the final-tier-closure round.

The relevant-tier set R is identified by a component A/B probe per §II.4: N additive variants × per-runtime measurement × <10 minute runtime. The probe is empirical; theoretical attribution is necessary but not sufficient apparatus. The discipline (standing rule for CRB-driven pilot spawn): run the probe before spawning any substrate pilot whose telos is "close a measured gap."

The cruftless instance: the JSF chain landed correctness-preserving substrate work at a tier outside R for its target fixture; -1% CRB after six rounds. The component A/B probe surfaced the actual dominator (character-scanning loop, 77% of cost). Two follow-on rounds closed substrate-tier algorithm + interp-tier dispatch; -12% CRB cumulative; cruft/node 20.34× → 17.93×. The pipeline connects at the cumulative scope.

The keeper's pre-implementation framing — "you have to go through this middle stretch where performance decreases first" — was correct: the middle stretch is the substrate-introduction round whose (P2.d) is the cascade-revival signature, not pilot failure. What the multi-tier reading adds: the middle stretch may extend across multiple rounds at different tiers, and the pipeline-connection moment is the cumulative-measurement round after all relevant tiers have been closed.

---

## VIII. Amendment — Coverage-axis enumeration for tier-class pilots

*Added 2026-05-23, several hours after the document's first publication, following the architectural-pivot session's TL → VD → OSR sequence. The amendment specifies the coverage-axis enumeration that the §II relevant-tier-set R apparatus exposes when the pilot tier-class is "JIT closure of a measured-CRB-gap." The amendment is a refinement of §II.4's empirical-disambiguation discipline, not a revision: §II.4's component A/B probe identifies the COST dominator; §VIII enumerates the COVERAGE axes that must close at the dominator tier for cumulative reclaim to materialize.*

### VIII.1 The coverage-axis enumeration

The §II multi-tier reading treats R as a set of tiers along a single dimension (per-call cost contribution). For a tier class with multiple structural dimensions (e.g., "the JIT tier" composes a code-emission dimension, a value-encoding dimension, and a calling-convention dimension), the relevant-tier set R has additional structure: for the tier to close, ALL of its structural dimensions must support the pilot's required closure shape.

A coverage axis is a structural dimension of a tier class that gates whether a pilot at that tier can deliver its intended closure. For the cruftless engagement's JIT-closure pilots, four coverage axes surfaced empirically during the 2026-05-23 architectural-pivot session:

**(A1) Component A/B coverage**: the pilot targets the actual cost dominator (identified by the §II.4 probe), not a suspected component. Without (A1), the pilot's substrate work lands at a tier outside R and produces 0% cumulative reclaim regardless of the pilot's correctness. Apparatus per §II.4 + Pin-Art component-decomposition probe.

**(A2) Op-set coverage**: for JIT-alphabet pilots (closing a JIT-eligibility gap by adding alphabet variants), the pilot's alphabet additions cover ALL ops in the hot-path enclosing scope, not just the inner-loop sub-region. The JIT's whole-body bail discipline (a structural property of single-entry JIT compilation) means any op outside the alphabet causes the whole body to fall through to interp regardless of how completely the inner-loop sub-region's alphabet is closed. Apparatus: source-read enumeration of the full enclosing-scope bytecode before pilot spawn.

**(A3) Value-domain coverage**: for JIT-IC pilots that require non-Number / non-Object receivers (e.g., String-receiver method ICs), the calling convention encodes the required receiver Value variants. Without (A3), the JIT body receives a structurally-incomplete representation of the receiver (e.g., 0.0 instead of an Rc<String> pointer) and cannot correctly emit IC fast-path bodies regardless of the alphabet's coverage. Apparatus: source-read of the calling-convention's unboxing helpers before pilot spawn.

**(A4) Locals-marshaling coverage**: for JIT-invoke pilots that invoke JIT bodies from non-arg state (OSR loop extraction; coroutine / async resume; mid-function deopt resume; ICs synthesizing JIT bodies from runtime-known state), the calling convention populates locals from the required source. The args-only initialization shape (locals 0..params from f64 args; locals params..N = 0.0) is sufficient for function-call entry and module-body entry; it is INSUFFICIENT for state-injection pilots whose JIT body reads enclosing-frame locals. Apparatus: source-read of the locals-init path before pilot spawn.

### VIII.2 The 5-tier lower bound for JIT-invoke pilots

For a JIT-invoke pilot whose target is a hot-path closure on a fixture with mixed-Value receivers + non-arg state (the cruftless json_parse_transform fixture is the canonical case), the relevant-tier set R has a structural lower bound of five tiers:

1. **Entry mechanism**: the JIT body's entry point is reachable from the dispatcher / interp loop. Closes the "JIT never fires" gap.
2. **Op-set coverage (per A2)**: the loop body's bytecode ops are all in the JIT alphabet. Closes the "whole-body bail" gap.
3. **Value-domain coverage (per A3)**: the calling convention encodes the Value variants the loop body's receivers / operands require. Closes the "0.0-degradation at boundary" gap.
4. **Locals-marshaling coverage (per A4)**: the calling convention populates locals from the enclosing frame's state at JIT body entry. Closes the "stale-locals at invoke" gap.
5. **IC fast-path body**: for hot intrinsic calls within the loop body (e.g., String.prototype.charCodeAt), the JIT emits inline fast-path IR that reads the receiver via (A3)'s encoding and produces the result without dispatcher round-trip. Closes the "per-call dispatch overhead" gap.

A pilot addressing only a subset of these five tiers delivers substrate-introduction value at the addressed tier(s) but not cumulative reclaim. Per §II.2 (P4): the cumulative reclaim materialization point is the round that closes the LAST of the relevant tiers.

The cruftless engagement's 2026-05-23 session closed tiers 1 (TL pilot's entry-mechanism), 3 (VD pilot's value-domain), and the cross-tier substrate + dispatch closures (CharCode-EXT 1+2 at non-JIT tiers). Tiers 2, 4, and 5 remain for the OSR pilot's subsequent rounds. The current 12% CRB cumulative reclaim on json_parse_transform is the partial-closure measurement; full closure projects to 40-60% reclaim per the OSR-EXT 1 design's reclaim model.

### VIII.3 Apparatus extension: standing rule with multi-axis coverage check

The cruftless engagement's standing rule 11 (the component A/B probe rule, originally introduced as the (A1) coverage check) extends to multi-axis: before spawning any pilot whose telos is "close a CRB-measured gap," run the (A1) probe AND verify (A2) op-set coverage if the pilot is JIT-alphabet AND verify (A3) value-domain coverage if the pilot is JIT-IC with non-Number/Object receivers AND verify (A4) locals-marshaling coverage if the pilot invokes JIT bodies from non-arg state.

If any of the applicable coverage checks fails, the pilot's reclaim ceiling on the target fixture is 0% via that pilot alone; the missing tier(s) must be addressed in dependency order (per §II.2 P4) before cumulative reclaim materializes.

The compounding value: each future JIT-tier pilot's spawn decision is gated on the multi-axis check; mis-attribution at any axis is caught BEFORE substrate work begins. The cost of the multi-axis check is bounded (each axis check is a source-read + brief enumeration; minutes per axis); the cost of NOT running the check is the cost of a substrate pilot landing at a structurally-insufficient tier (one example from the session: the TL (b-narrow) plan, six rounds + ~390 LOC, closed structurally at TL-EXT 3 with the remaining rounds re-scoped after Finding VII.2 surfaced).

### VIII.4 Generalization beyond JIT

The four coverage axes (A1-A4) named here are JIT-specific. The structural pattern — "a tier class has multiple coverage dimensions; the pilot's closure requires ALL applicable dimensions to be covered" — generalizes to any tier class with multiple structural dimensions. For example:

- A storage-tier pilot (closing a measured query-latency gap) may have coverage axes: query-shape coverage (the query plan handles the actual hot-path shape), index-coverage (the relevant index exists), partition-coverage (data layout aligns with the access pattern), serialization-coverage (the wire format supports the required types).
- A network-tier pilot may have coverage axes: protocol-coverage (the wire protocol supports the operation), buffer-coverage (buffer sizes accommodate the workload), connection-coverage (the connection pool supports the concurrency), encoding-coverage (the payload encoding round-trips the required types).

The §II.4 component A/B probe identifies the cost dominator at the per-call axis; the §VIII coverage-axis enumeration identifies the structural dimensions that must close at the dominator tier. Together, they form the engagement's standing pre-spawn discipline for tier-class pilots.

### VIII.5 Composition with prior sections

- §II.2 (P4): the cumulative-reclaim materialization point holds; this amendment adds that "all relevant tiers closed" includes "all relevant coverage axes at each tier."
- §II.4 component A/B probe: extended; the probe remains the apparatus for (A1) coverage check; this amendment adds (A2-A4) as additional pre-spawn checks for JIT-tier pilots.
- §III cruftless instance: the JSF chain closed (A1) at JSF-EXT 8 + the substrate/dispatch tiers at CharCode-EXT 1+2; the TL pilot closed entry-mechanism (tier 1); the VD pilot closed value-domain (tier 3 + (A3)); the OSR pilot's remaining scope is to close (A2) at the loop scope + (A4) locals-marshaling + tier 5 IC bodies. Each closure round adds the corresponding coverage; cumulative reclaim materialization is queued for the OSR pilot's final round.
- §IV.1 empirical disambiguation: extended along the coverage axes; (A1) is empirical (probe); (A2)-(A4) are source-read enumerations (each takes minutes; each prevents structural mis-scoping).
- §IV.2 substrate-introduction signature: holds; a substrate-intro round that closes one coverage axis at the tier still expects (P2.d) bench because the remaining coverage axes at the tier still gate cumulative reclaim.

### VIII.6 Summary of the amendment

The §II multi-tier reading enumerates tiers along the per-call cost dimension. §VIII enumerates coverage axes along the structural dimensions of each tier class. For a tier class with multiple structural dimensions, the pilot's closure at the tier requires ALL applicable coverage axes to be covered; partial coverage delivers substrate-introduction value but not cumulative reclaim.

For JIT-closure pilots specifically, four coverage axes are identified: component A/B; op-set coverage; value-domain coverage; locals-marshaling coverage. The cruftless engagement's standing rule 11 multi-axis check applies all four pre-spawn for any JIT-tier CRB-driven pilot. The structural pattern generalizes to any tier class with multiple structural dimensions.
