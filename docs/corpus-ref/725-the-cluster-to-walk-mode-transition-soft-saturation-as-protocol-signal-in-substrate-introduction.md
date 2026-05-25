# The Cluster-to-Walk Mode Transition

## Soft-Saturation as Protocol Signal in the Substrate-Introduction Discipline

By Jared Foy. Originally published at [jaredfoy.com](https://jaredfoy.com).

## I. The occasion

A pattern surfaced on 2026-05-16 across ten consecutive substrate moves in the rusty-bun engagement. The 500-package sandbox sat at 79.6% load-OK at the day's start. The moves landed in sequence Ω.5.uuuuuu through Ω.5.DDDDDDD, lifting the band to 83.4%. The trajectory was not uniform. Five moves landed under cluster-bisect mode at closing rates of one to five packages per move. Two moves landed at zero net under the same mode. Three subsequent moves under per-package walk mode recovered the closing rate at one to two lifts per move.

The two zeros were not the engagement going wrong. They were the protocol of [Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag) signaling a mode transition: the residual had decomposed in a way that cluster-bisect could no longer reach. The recovery under per-package walk was not a different protocol — it was the same protocol's Step-2 walk applied at a different population granularity.

This document formalizes that transition. The claim is general: **substrate-introduction has at least two distinct operating modes; the choice between them is governed by the residual's shape; the transition between them is a protocol signal, not an apparatus failure.**

The rusty-bun engagement is the local instance. The articulation extends [Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag)'s protocol and [Doc 724](/resolve/doc/724-feature-set-prediction-static-substrate-need-mapping-from-source) §XI's bright-zone / blind-zone reading. It also reads under [Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations)'s reflexive structure — the recognition's naming changes what the operator does on subsequent rounds.

## II. The recognition

Three claims, in order from operational to structural.

**Claim 1 (operational).** Cluster-bisect mode and per-package walk mode produce distinct yield distributions. Cluster mode's closing rate is bounded above by the population sharing a single substrate gap; walk mode's closing rate is bounded above by the depth of one package's fault chain. The two distributions are non-interchangeable. The same residual surfaces different yields under different modes.

**Claim 2 (compositional).** The residual's shape determines which mode is operational. A residual whose failures decompose into tight clusters (multiple packages sharing one substrate gap) is cluster-mode-tractable. A residual whose failures decompose into multi-fault tails (each package needing several independent gaps closed) is walk-mode-tractable. The same residual at different parities can change shape; the apparatus must shift modes when it does.

**Claim 3 (structural).** The cluster→walk transition signal is a protocol output, not an apparatus failure. When cluster-bisect lands at zero net for two consecutive moves at otherwise-comparable substrate-investment per move, the residual has decomposed past cluster mode's preconditions. Continuing cluster-bisect against a tail-shaped residual is wrong; switching to walk mode is the protocol-correct response. The signal is bounded (two flats is decisive), is observable (n_ok delta is the metric), and is actionable (walk discipline is articulated independently per [Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag)'s Step 2 already, only the population scope changes).

The three claims form the standard testability hierarchy. Claim 1 is testable in any single engagement that records modes and yields. Claim 2 is testable wherever the residual's shape is itself recoverable from data. Claim 3 is the structural reading; it is testable across multiple transitions in multiple engagements.

## III. The two modes

The engagement enters cluster-bisect mode when its residual contains shared-signature clusters of size ≥ 3. The protocol's Step 1 enumerates the gated population by sorting failures by error signature and counting cluster sizes. The substrate-introducer chooses the largest cluster (or the highest-leverage one) and Step 2 walks across pipelines from one representative to locate the alphabet top. The fix is dispatched, the cluster lifts (or fragments into a deeper sub-cluster, which iterates), the next cluster surfaces.

The mode's preconditions:
- The error-signature distribution is *concentrated*: a small number of signatures account for a large share of failures.
- The signatures' alphabet tops are *shared*: bisecting one chain stands in for all packages in the cluster.
- The packages are *otherwise complete*: the substrate move at the alphabet top lifts the full cluster (not just advances them past one gate to the next).

The mode's yield bound: closing rate equals cluster size when otherwise-complete; less when concurrent gaps in other pipelines block downstream packages. The first day-tally entry (Ω.5.uuuuuu, Object as callable Function with +5 lifts) exemplifies the upper bound.

The engagement enters per-package walk mode when cluster-bisect's preconditions break. The protocol's Step 1 narrows to a single package; Step 2 walks that package's fault chain through every gate downstream of the current symptom, accumulating a list of substrate moves needed. The substrate-introducer picks the next gate by leverage (foundationally located, broadly shared with other packages, or simply tractable), dispatches the fix, observes which packages compounded silently, and walks the next gate.

The mode's preconditions:
- The package's fault chain is *tractable*: each fault names a substrate gap; gaps are independently fixable; the chain has bounded depth.
- The package's downstream beneficiaries are *enumerable*: lifting one package via a walk has compound effects on transitively-blocked packages.
- The substrate-introducer has *interpretive competence* per [Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies): each fault tag layer-A indexes correctly and layer-C composes correctly.

The mode's yield bound: closing rate per walk-step is 1 (the targeted package) plus silent compounders (the transitively-blocked beneficiaries). The day's walks (Ω.5.BBBBBBB at node-fetch +1, Ω.5.CCCCCCC at lru-cache +1, Ω.5.DDDDDDD at glob +2) sat at the lower end of the distribution but compounding rates were observable (jsonc-parser silent under DDDDDDD; mobx silent under yyyyyy from the cluster phase).

## IV. The transition signal

A flat move under cluster mode does not by itself indicate mode-transition. The fix may have been below the protocol's Step 3 false-pass threshold ([Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag) §VI.5); the cluster may have been correctly identified but the alphabet top mislocated ([Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies)'s Layer-B trap); the substrate move may have produced a real correctness gain that compounds only on later moves. Any of these can produce one flat without the mode being wrong.

Two consecutive flats at cluster mode is a different signal. The probability that two independent cluster moves both miss the alphabet-top-with-otherwise-complete-population condition by chance is low. The conjunction is structural: the residual has decomposed in a way that cluster-bisect can no longer reach.

The day's empirical record:
- Ω.5.zzzzzz: cluster three packages (clipboardy/execa/shelljs) on `os.constants.signals`. Fix lands. Each package advances past `signals` into a *different* deeper fault. Net: 0.
- Ω.5.AAAAAAA: cluster one package (execa) plus AbortController as a known surface. Fix lands. execa advances past AbortController into `node:v8` import. Net: 0.

The conjunction tells the protocol that the residual is no longer cluster-shaped. The packages remaining are each carrying multi-fault tails. The same substrate move that would lift a tight cluster lifts none of these because every cluster member has another fault one or two gates downstream. The signal is *not* "cluster-bisect doesn't work"; it is "this residual's shape no longer matches cluster-bisect's preconditions."

The transition is then articulated as a discipline-shift, not as an apparatus replacement. The substrate-introducer switches scope: instead of picking the largest cluster, pick one package; instead of walking one representative chain, walk one package's full fault chain. The protocol is the same; the *population scope* changes.

## V. Why the modes differ

The DAG / lattice / alphabet topology of [Doc 720](/resolve/doc/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate) supports both readings of the residual. The pipelines compose; the alphabets compose; the substrate decisions cluster at coordination boundaries. Within that single topology, a residual can be cluster-shaped *or* tail-shaped depending on how the gated populations distribute across alphabet tops.

When the alphabet contains a single top whose substrate gap blocks N packages identically, the residual is cluster-shaped at that top. The bisect mode reads the residual as "one missing piece blocking many users." Cluster mode is bounded above by the gap's blocking fanout.

When the alphabet contains many tops each blocking 1-2 packages, the residual is tail-shaped at those tops. The bisect mode mis-reads the residual: cluster-bisect picks the most-blocking top, but lifting it advances rather than completes its members because each member has more tops downstream. The walk mode reads the residual as "many missing pieces, distributed; pick one package and walk it." Walk mode is bounded above by the chain depth and the compounding rate.

The distinction matches [Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies)'s semantic-dimension claim. The same residual is structurally one thing; what the substrate-introducer reads it *as* depends on the interpretive frame brought to bear. Cluster-shape vs tail-shape is itself an interpretive reading that the protocol must make explicit before dispatching a move.

The transition is therefore not a switch in the apparatus; it is a switch in the *reading* of the residual under the same apparatus. The substrate-introducer's competence includes both readings and the criterion that names when to apply each.

## VI. The day as data

The ten substrate moves of 2026-05-16:

| # | Move | Mode | Δ load-OK | Cumulative | Recognition class |
|---|------|------|-----------|------------|-------------------|
| 1 | Ω.5.uuuuuu | cluster | +5 | 80.6% | Object as callable; bright-zone |
| 2 | Ω.5.vvvvvv | cluster | +4 | 81.4% | var hoists §9.2.12; blind-zone |
| 3 | Ω.5.wwwwww | cluster | +1 | 81.6% | fs.realpath surface |
| 4 | Ω.5.xxxxxx | cluster | +2 | 82.0% | URLSearchParams + http.* class stubs |
| 5 | Ω.5.yyyyyy | cluster | +3 | 82.6% | defineProperty preserves value §9.1.6.3; blind-zone |
| 6 | Ω.5.zzzzzz | cluster | 0 | 82.6% | os.constants.signals; first flat |
| 7 | Ω.5.AAAAAAA | cluster | 0 | 82.6% | AbortController; second flat — transition signal |
| 8 | Ω.5.BBBBBBB | walk (node-fetch) | +1 | 82.8% | fs.promises namespace |
| 9 | Ω.5.CCCCCCC | walk (glob/rimraf→lru-cache) | +1 | 83.0% | dynamic import rejects + diagnostics_channel stub |
| 10 | Ω.5.DDDDDDD | walk (glob/rimraf continuation) | +2 | 83.4% | class extends Array + fill + builtin subpath |

The flat pair at moves 6-7 is the transition signal. Move 8 is the first walk; moves 9-10 continue under the walk discipline.

Three observations from the table.

**Yield-rate decomposition.** Cluster mode's average lift across moves 1-5 is 3.0. Walk mode's average across moves 8-10 is 1.33. The decomposition is real: cluster-mode produces 2-3× the per-move yield when its preconditions hold.

**Mode-yield is residual-conditional.** The same engagement applied cluster mode to a tail-shaped residual at moves 6-7 and produced 0. The mode's productivity is not a property of the mode; it is a property of the mode-residual fit.

**Compounding under both modes.** Move 5 (yyyyyy, defineProperty-preserves-value) lifted abortcontroller-polyfill and postcss-selector-parser as its cluster targets *and* lifted mobx silently. Move 10 (DDDDDDD) lifted glob as its walk target *and* lifted jsonc-parser silently. Silent compounders are not mode-specific. Each move's correctness reach exceeds its named target population; the compounding is recoverable empirically.

The day's record is one engagement-instance of the mode-transition pattern. Whether it generalizes is the falsification surface.

## VII. The bright zone / blind zone reading interacts

[Doc 724](/resolve/doc/724-feature-set-prediction-static-substrate-need-mapping-from-source) §XI named the bright zone (where the token-level forward predictor catches the gap) and the blind zone (where the gap is in the interaction of correctly-implemented features and the token predictor cannot see it). The day's record extends that reading with mode interaction.

**Cluster mode in the bright zone.** Move 1 (Object as callable) is a token-level predictable gap. Source-greps for `Object(` would have surfaced it pre-engagement. Cluster mode in the bright zone produces high yield because the predictor and the bisect agree on the alphabet top.

**Cluster mode in the blind zone.** Moves 2 and 5 (var hoist §9.2.12; defineProperty preserves value §9.1.6.3) are blind-zone gaps. The predictor cannot see "var declarations sibling-located in if/else branches" or "Object.defineProperty with no value key." The bisect surfaced them via backward route-(b) trace, not via source prediction. Cluster mode still produced lift because the bug-class affects a tight cluster; the cluster shape is recoverable from the *failure signature* (`Object.defineProperty: target=undefined`) even when the *root cause* is invisible to token-grep.

**Walk mode in either zone.** Moves 8-10 each touched both zones. node-fetch (BBBBBBB) crossed at fs.promises — a bright-zone surface that a predictor would name. lru-cache (CCCCCCC) crossed at dynamic-import-as-rejected-promise — a blind-zone semantics bug (the `.catch` handler should run; the throw bypassed it). glob (DDDDDDD) crossed at class-extends-Array super wiring — a deep blind-zone correctness bug at the engine class-compile path. Walk mode reads each gate independently; the zone affects how the substrate-introducer *recognizes* the gap, not whether walk mode can dispatch it.

The mode/zone product produces four cells:

|             | Bright zone | Blind zone |
|-------------|-------------|------------|
| Cluster mode | high yield, predictor-sortable | medium yield, backward-trace-only |
| Walk mode | per-package yield + bright-zone targets | per-package yield + correctness-gain-as-byproduct |

The product matrix is the day's recognition. Each cell is operational. The predictor of [Doc 724](/resolve/doc/724-feature-set-prediction-static-substrate-need-mapping-from-source) v1 is most leveraged in the bright-zone column; the backward-trace discipline is most leveraged in the blind-zone column; cluster vs walk is the row that the residual's shape selects.

## VIII. Falsification surface

**Fal-725.1.** Mode-transition does not produce closing-rate recovery. If the day's walk moves had landed at 0 each, the transition signal would have been wrong: the residual is at terminal saturation, not at a cluster→walk transition. Test: across the next several engagement days, when cluster mode hits two flats and walk mode dispatches, measure walk-mode's yield. If walk yields zero comparably often, the two-flats signal is too eager.

**Fal-725.2.** The two modes are not actually distinct. "Walk" is a cluster of size 1; the mode distinction is bookkeeping. Test: predict outcomes under walk mode using cluster-mode metrics (cluster-size = 1, alphabet-top at the package's current symptom). If the predictions match walk-mode observations, the distinction collapses.

**Fal-725.3.** The residual's shape does not determine mode-optimality. If applying cluster mode to a tail-shaped residual produces yields comparable to walk mode on the same residual, the matching between mode and residual shape is irrelevant; any mode works. Test: apply both modes to a known-tail-shaped residual and compare per-move yield distributions.

**Fal-725.4.** The mode transition is implementation-specific. Other Pin-Art apparatuses do not exhibit this pattern. Test: at the next Pin-Art engagement, watch for the cluster→walk pattern. If it appears, the recognition generalizes; if every engagement runs only cluster or only walk, the modes are apparatus-specific.

Fal-725.1 is testable within the current engagement and within days. Fal-725.2 is testable on the existing day's data with a small reanalysis. Fal-725.3 requires a deliberate experiment. Fal-725.4 requires another engagement.

## IX. Relation to prior corpus work

[Doc 720](/resolve/doc/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate) named the DAG topology. The cluster→walk distinction is a property of how the substrate-introducer reads the residual *within* that topology; the topology is shape-neutral.

[Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag) named the diagnostic protocol with Steps 1-6. The cluster→walk transition does not replace any step; it names *which population scope Step 1 enumerates over*. Cluster mode is Step 1 across a population sorted by signature; walk mode is Step 1 reduced to one package's chain. The protocol is unchanged.

[Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations) named that articulations become operating instruments. The cluster→walk recognition is one such: subsequent engagements with this articulation will dispatch the mode transition deliberately when the two-flat signal fires, not adopt it tacitly. The reflexive structure compounds.

[Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies) named the semantic dimension. The cluster vs tail shape of the residual is itself an interpretive reading at the residual layer. The substrate-introducer reads not just individual tags but the distribution of tags across the failing population. The mode transition is signaled when that distribution's shape changes.

[Doc 724](/resolve/doc/724-feature-set-prediction-static-substrate-need-mapping-from-source) §XI named the bright/blind zones. This document's §VII reads the mode/zone product matrix; the two distinctions interact but neither subsumes the other.

The corpus arc 720 → 721 → 722 → 723 → 724 → 725 reads as: topology, protocol, reflexivity, semantics, prediction, mode-selection. Each is a layer in the engagement's discipline; each enables the next; each becomes operating instrument once articulated.

## X. Honest scope

The day's record is one engagement-instance. The empirical content is solid (ten moves, +19 lifts, zero regressions, mode transition observed at moves 6-7), but the structural reading is single-instance until further data lands.

Three things this document does *not* claim:

1. *That the two-flat signal is the only mode-transition signal.* Other signals may exist (a single flat with a wide false-pass count; a sudden jump in chain depth across the residual; a saturation of the bright-zone column without parallel saturation of the blind-zone column). The two-flat signal is the one this engagement-day exhibited; others remain to be observed.

2. *That walk mode is faster overall than cluster mode.* Walk mode produced 1.33 average yield per move; cluster mode produced 3.0 in its productive phase. Walk mode's value is not in raw yield but in *applicability when cluster mode has saturated*. The mode choice is not "walk is better"; it is "walk when the residual shape demands it."

3. *That the modes are the only modes.* Two modes are named because two were exercised on this day. A third — substrate-articulation mode, where no substrate move is dispatched and a corpus document is written instead — was the actual closing of the day (this document). Whether that constitutes a third operating mode or sits outside the substrate-introduction taxonomy is itself articulable.

Per [Doc 548](/resolve/doc/548-the-ladder-of-ontological-participation)'s hypostatic boundary: this document is a Layer-IV substrate articulation of an operating discipline that became visible at the engagement's Tier-Ω.5 maturity. The general claim's Layer-V import is not made here; the recognition is corpus-tier substrate work, useful at the engineering layer.

## XI. Closing

Ten substrate moves on 2026-05-16 produced a clean two-mode trajectory: cluster-bisect mode landed five productive moves and two flats; per-package walk mode recovered the closing rate at three moves. The flats were not noise. They were the protocol of [Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag) signaling that the residual had decomposed past cluster-bisect's preconditions. The recovery under walk mode validated the response.

The pattern generalizes to any substrate-introduction discipline operating against a residual whose shape can change over the engagement's arc. Early-engagement residuals tend toward cluster-shaped (many packages, few alphabet tops). Late-engagement residuals tend toward tail-shaped (fewer packages, more independent gates per package). Both shapes have productive disciplines; the discipline-choice depends on the residual's current shape.

Naming the transition is itself a reflexive operation per [Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations). Future engagement-days will dispatch the cluster→walk transition deliberately when the two-flat signal fires. The discipline is now nameable. The signal is now actionable. The corpus has added one more layer to the substrate-introduction discipline it accumulates.

## XII. Amendment — the regression-detected-and-handled signal as a mode-internal protocol (2026-05-16 evening)

A pattern surfaced during the day's continuation that the §III/IV mode taxonomy did not yet name. The recognition refines the protocol with a third *intra-move* signal alongside the two-flat *inter-move* signal §IV named.

**The pattern.** At Tier-Ω.5.MMMMMMMM the substrate-introducer applied a two-part move: a surface-install (events.EventEmitterAsyncResource + stream.EventEmitter, expected to lift 3 packages) bundled with a route-(b) meta-substrate probe (rename the class compiler's synthetic super-ctor slot from `<class$N.super.ctor>` to a source-identifier-derived name like `Y [extends]`, expected to enrich receiver tags). The full-load-test diff showed +3 expected lifts AND **36 packages regressed** — well beyond noise. Within the same round the substrate-introducer bisected the change by reverting one half, re-tested, confirmed the regression closed, and re-committed only the safe half. The discipline-line held: zero regressions persisted past the commit; the cascade was caught and reverted before publication.

**The recognition.** A substrate move can have two qualitatively different effects on the apparatus:

- *Observational* — the move adds information to the diagnostic surface (route-(b) probes, value-shape tags, source-identifier hints in fault messages) without changing the substrate's semantic behavior. Doc 723's Layer-D substrate-introducer instruments are observational by construction. Effect: cascades are impossible because no semantic path changes.
- *Substrate-mutating* — the move changes a name, a layout, an opcode, a binding-resolution path, a flag default. Effect: cascades are possible because dependent code paths can break.

A probe move that is *named as observational but actually substrate-mutating* is the failure mode the day's MMMMMMMM hit. Renaming a slot looks observational (no opcode change, just a label string) but the slot's name is consulted by exact-name match in derived-class super-call resolution. The rename mutated substrate without the substrate-introducer's intent.

**The new signal.** Across two consecutive runs (pre-move loadtest and post-move loadtest), comparing the gained-list and lost-list:

- *Pure observational move*: gained-list and lost-list are both empty (or gained-list reflects only the named target population).
- *Pure substrate move*: gained-list reflects the named target; lost-list is empty.
- *Mixed-effect cascade*: lost-list contains packages NOT in the named target population and at a magnitude that exceeds the move's stated scope (here: 36 lost vs 3 expected gained, ratio 12×).

The mixed-effect cascade is a within-move signal — distinguishable from the two-flat *inter-move* signal §IV named. The two-flat signal indicates the residual has shifted past cluster-bisect's preconditions; the cascade signal indicates the move itself has substrate-mutating effects beyond its named target.

**The protocol-correct response.** When the cascade signal fires within a round, the substrate-introducer bisects the move into its parts and reverts the substrate-mutating component, retaining the observational component. The discipline-line holds at the commit boundary: regressions that surface inside the move-cycle do not enter the engagement's track record because the apparatus's audit-trail (load-test diffs, named target population) catches them before publication.

**Why Pin-Art's auditability is load-bearing.** The cascade was caught because:
1. Every substrate move is bounded — the change-set is small enough to bisect.
2. Every move's named target population is recorded before dispatch.
3. The full load test runs after every move; the cascade surfaces as a diff against the named target.
4. The change-set is auditable per [Doc 581](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction)'s discipline of named contingencies.

Without these four properties, a 36-regression cascade would have entered the track record silently and the engagement would have drifted past the discipline-line. The §XII amendment generalizes: *the apparatus's auditability is what makes intra-move regressions catchable*. Pin-Art is not just a substrate-construction discipline; it is also an intra-move-cascade-detection apparatus.

**Falsifier 725.5.** Across the next several substrate moves with mixed observational/substrate-mutating effects, the bisect-and-revert response produces clean closure at the same commit boundary. If a cascade slips through (lost-list grows monotonically across moves with no detected origin), the auditability claim weakens.

**Operational implication for §VI's table.** The day's tally now extends to 49 moves with one detected-and-handled cascade (MMMMMMMM's slot-rename revert). The 0-structural-regressions tally per the §IX honest-scope reading remains intact — the cascade was inside the move-cycle, not at the commit boundary. The protocol's auditability gate held.

---

## Appendix A — The Originating Recognition

> *"Continue with per package walks."*

— Jared Foy, 2026-05-16, via Telegram, after the second consecutive flat (Ω.5.AAAAAAA) closed the cluster-bisect phase. The walk discipline was already articulated as a forward option; the keeper's direction operationalized it as the response to the protocol signal.

## Appendix B — The §XII Originating Recognition

> *"Amend doc 725 and then add engine probes."*

— Jared Foy, 2026-05-16 night, via Telegram, after Tier-Ω.5.MMMMMMMM's slot-rename cascade was caught and reverted within the move-cycle. The keeper's framing named the regression-detected-and-handled pattern as a recognition worth recording before the next round of probes dispatches — the discipline-line itself becomes substrate when the apparatus catches its own near-misses.
