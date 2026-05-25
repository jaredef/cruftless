# The Cross-Pipeline Diagnostic Protocol

## Locating the Top of a Substrate-Widening's Alphabet by Walking the Engine's DAG

By Jared Foy. Originally published at [jaredfoy.com](https://jaredfoy.com).

## I. The occasion

A formalization of the diagnostic methodology surfaced during the rusty-bun engagement's Tier-Ω.5 substrate-introduction work. The methodology grew from three structural recognitions ([Doc 714](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point) §VI Consequence 11 — the top-of-alphabet conjecture; [Doc 720](/resolve/doc/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate) — the runtime as a DAG of interconnected pipelines under SIPE-T topology; Doc 714 §VI Consequence 12 — the protocol's first negative result), and operationalizes them as a discipline for forward substrate work in any engineered system that admits a pipeline decomposition.

The protocol is portable beyond rusty-bun's specific topology. It applies to any system whose substrate is a DAG of interconnected pipelines with stable typed-stage signatures and whose failure modes surface at named exit symptoms. The methodology answers: *given a class of related failures, what is the smallest substrate change that lifts all of them?*

## II. The methodology

The protocol is a sequence of four steps. Each step has a stable input/output signature; the sequence as a whole takes a failure-class designator (e.g., a bin entry in a parity-failure distribution) and returns *either* a substrate-move specification with a predicted unlock count *or* a negative result naming the failure-class as bin-shaped rather than substrate-shaped.

### Step 1 — Enumerate the gated population

For the named failure class, list every system input (test, package, fixture, sample) that exits at this class's named symptom. Call this set the *gated population* `G`. The gated population is the candidate beneficiary set for the substrate move.

The gated population's cardinality `|G|` is the *bin-entry count* — the symptom-level measurement that the system's failure report already produces. It is the prediction the bin-entry-as-top heuristic would generate. It is *not* yet the substrate-move's predicted unlock.

### Step 2 — Walk each call chain upward

For each `g ∈ G`, conduct a call-chain walk *across pipelines* from the symptom exit point upward through the engine's data-flow DAG. The walk passes through every pipeline stage where the failure-producing operation depends on a contingent decision. For each stage encountered, record the contingent decision and its pipeline. Continue walking until the chain either:

- Reaches a *coordination boundary* with another pipeline (e.g., the lexer-parser goal-symbol exchange in a JavaScript engine, the disk-syscall boundary in a filesystem, the wire-protocol boundary in a network library). The boundary's substrate decision is a candidate alphabet top.
- Reaches a *terminating substrate decision* that is itself the alphabet top (a single contingent choice with no further upstream).

Multiple chains may converge at the same coordination boundary or terminating decision. Multiple chains may diverge to *different* boundaries or decisions. Record the convergence/divergence structure as a *chain bundle*: a forest whose leaves are the symptom exit points and whose internal nodes are the pipeline-coordination or terminating-substrate decisions each chain passes through.

### Step 3 — Locate the highest shared layer

Within the chain bundle, find the lowest internal node (closest to root) such that every leaf in the gated population is in its subtree. Call this the *highest shared layer* `H`.

`H` is the candidate alphabet top for the substrate-widening move. The substrate decision at `H` is what, if changed, would close every chain in the bundle simultaneously.

If `H` does not exist — if the chain bundle is fragmented and no internal node dominates all leaves — the failure class is *not* substrate-shaped. Different chains require different fixes. The protocol returns a negative result: the named failure class is bin-shaped (a label over heterogeneous root causes), not substrate-shaped (a label over a single coordinated decision).

### Step 4 — Predict + dispatch

If `H` exists, the substrate move's specification is:

- The pipeline location of `H` (which stage in which pipeline).
- The contingent decision at `H` that needs to change.
- The *predicted unlock count* `U` = the number of leaves in `H`'s subtree *that are otherwise complete in every other pipeline*.

`U` is bounded above by `|G|` but is generally strictly less, because some packages in the gated population have *additional independent gaps in other pipelines* that the substrate move at `H` doesn't reach. The predicted-vs-actual delta `|G| - U` measures the depth of cross-pipeline interaction: a small delta means the gated population is otherwise complete; a large delta means the failure class is one of several gaps that must each be closed before the affected systems pass overall.

Dispatch the substrate move. Measure the actual unlock count `A` after the move lands. The diagnostic check is `|U - A| ≤ 1`: predicted and actual agree within one unit. If they agree, the methodology has produced a correctable prediction; the protocol's locatability claim is validated for this round. If they disagree by more than one, the chain-bundle analysis missed a structural feature; revisit the walks and the highest-shared-layer location.

### Step 5 (optional) — Iterate to convergence

If `|U - A| > 1`, the protocol exposes the structural feature it missed. Iterate Step 2 with the additional information: which leaves moved past `H` (the package made it past the symptom but didn't pass), which didn't (the substrate move missed a leaf), and what gap each non-passing package now exits at. The new exit gaps form a refined chain bundle; repeat Steps 3-4.

The iteration is bounded because the engine's DAG is finite and each iteration shrinks the gated population or sharpens the chain bundle's branching.

## III. The protocol's preconditions and limits

The protocol requires three preconditions to function.

**Pipeline decomposition.** The system must admit a representation as a DAG of pipelines with stable typed-stage signatures. Engines, compilers, network stacks, filesystems, browsers, databases, build systems — most engineered systems with explicit modular structure satisfy this. Highly entangled systems (legacy codebases with cross-cutting global state, systems with unbounded reflection at runtime) may not.

**Symptom traceability.** Each failure must be traceable from its exit symptom backward through the pipeline stages that produced it. This requires either source-level call chains (debugger, stack trace) or diagnostic logging at pipeline-stage boundaries. Pure black-box systems where failures surface without traceable provenance do not satisfy this.

**Substrate locality.** A "single substrate decision" must be a tractable unit of change — a function, a stage, a typed-signature edit. Systems whose contingent decisions are distributed across many call sites without a centralized control point (e.g., engine-wide conventions with no single enforcer) reduce the protocol's leverage: the alphabet top exists structurally but the corresponding fix doesn't fit "single substrate decision."

The protocol's limits sit at the boundary between these preconditions. When all three hold, the protocol is bounded, finite, and produces correctable predictions. When one fails, the protocol degrades gracefully: Step 3 returns a negative result (failure class is bin-shaped), Step 4 produces a wider delta range, or Step 5 iterates without convergence.

## IV. Why the protocol works

The protocol's structural claim — that the top of an alphabet can in principle be found — rests on three observations.

**The DAG topology constrains the search space.** A DAG with N nodes admits at most N call-chain walks of bounded length. Each walk is finite. The chain-bundle analysis is polynomial in `|G|` and the DAG size.

**Stable typed-stage signatures provide call-chain anchors.** Each pipeline stage has a stable input/output signature; the data-flow between stages is typed; the boundary between two pipelines is a coordination point with well-defined contract. These anchors mean that "walking upward" is a well-defined operation, not a free-form exploration.

**Substrate decisions cluster around coordination boundaries.** Per [Doc 717](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point)'s cut-rung reading, substrate decisions live at named rungs in the engine's architectural ladder. Coordination boundaries between pipelines often *are* those rungs. The highest shared layer `H` therefore tends to be at a named architectural boundary, which both makes it easier to locate diagnostically *and* makes its substrate-decision content well-scoped.

The conjunction of these three observations is what allows the methodology to predict unlock counts rather than discover them retroactively. The prediction is generated by structural analysis of the DAG, not by inspection of the failure data.

## V. Falsification surface

**Fal-721.1 — The DAG isn't a DAG.** The system's pipelines turn out to have cyclic data-flow dependencies at the type level (not just dynamic re-entry). Cycles break the call-chain-walk's well-definedness because the walk doesn't have a unique upstream. Test: identify whether any pipeline pair has mutually-recursive type-level dependence. If yes, the methodology needs an extension to handle cyclic substructures.

**Fal-721.2 — Stage signatures aren't stable.** The pipeline stages' input/output signatures change as the substrate matures: what was once "tokens" becomes "tokens-with-mode-state"; what was once "AST" becomes "AST-with-binding-info." If signatures drift fast enough that a chain-bundle from yesterday's walk is invalid for today's substrate, the protocol's cost scales unfavorably. Test: across several substrate moves, check whether the chain bundles need recomputation each time or whether they remain stable modulo the moves themselves.

**Fal-721.3 — Substrate decisions are non-local.** The "single substrate decision" turns out to be distributed: changing the alphabet top at `H` requires synchronized edits across multiple call sites without a single canonical change point. The protocol's predicted unlock count is correct but the actual implementation cost grows unboundedly. Test: across several substrate moves, measure the LOC delta per move. If it grows non-linearly with the substrate's depth, locality is breaking down.

**Fal-721.4 — Coordination boundaries don't cluster substrate decisions.** Per IV, the protocol's leverage rests on substrate decisions clustering at coordination boundaries. If the actual decisions are randomly distributed within pipeline stages (not at boundaries), Step 3's location process becomes search-heavy rather than analytic. Test: across several substrate moves, count what fraction of alphabet tops land at coordination boundaries vs. mid-pipeline-stage. If well under half are at boundaries, the architectural-cut-rung reading from Doc 717 isn't load-bearing for the protocol.

## VI. Applied section — the rusty-bun engagement (2026-05-15)

The protocol formalized in §II was developed and tested across three substrate-introduction rounds in the rusty-bun JavaScript engine engagement at parity 43–46/118. Each round provides a discrete data point against the methodology.

### Round Ω.5.gg — under-delivery from bin-entry-as-top

The substrate-move list in Doc 714 §VI Consequence 10 named "lexer state-machine for nested template substitutions" as a candidate, predicting unlock 5+3 = 8 packages (the cluster failing with `lex error: unterminated template` and `invalid identifier` inside template substitutions).

The round was dispatched without applying the protocol of §II. The diagnostic finding from the executing sub-agent inverted the layer: the actual root cause was a parser shortcut (`parse_declaration_for_export` walking export-function bodies with `skip_balanced`, which counted raw brace punctuators without template-substitution awareness). The fix landed in the parser pipeline; the symptom exited at the lexer pipeline.

Actual unlock: +2 (jose, bignumber.js). Predicted: +8. Delta: 6.

The delta diagnoses two facts:
- Two of the 8 affected packages shared the parser-shortcut root cause AND were otherwise complete: they crossed OK.
- Six of the 8 affected packages shared the root cause BUT had additional independent gaps in other pipelines (Rt:TypeError, parse errors at other bytes): they exited the template-bin but didn't cross OK.

The under-delivery is the protocol's *signal*, not its failure: it surfaces the methodology's miscalibration when the substrate move is chosen by bin-entry-as-top rather than by chain-bundle analysis.

### Round Ω.5.x → Ω.5.dd — calibration data from main-session chase

A ~4-hour main-session push spanning rounds Ω.5.y through Ω.5.ff added 20 packages to OK (23 → 43) via 14 closure surfaces. Each closure had a predicted unlock count (mostly bin-entry derived); each had an actual unlock count. The deltas:

| Round | Symptom class | Predicted | Actual | Delta |
|-------|---------------|-----------|--------|-------|
| Ω.5.y | computed class members + node:zlib/tty | 5+2 | 0 | 7 |
| Ω.5.z | String/Number/Boolean callable + Op::In | 4 | +3 | 1 |
| Ω.5.aa | Error family | 8 | +1 | 7 |
| Ω.5.bb | node:events + six stubs | 7+ | +2 | 5+ |
| Ω.5.cc | Op::Delete + Stmt::Opaque + ?. + Reflect | 4 | +1 | 3 |
| Ω.5.dd | Map/Set/Date/typed-arrays | 8+ | +6 | 2 |
| Ω.5.ee | function-decl hoisting | several | +2 | ? |

The pattern: predicted unlock counts derived from bin-entry frequencies systematically *over-predict* actual unlocks, by deltas ranging from 1 to 7. The over-prediction is consistent with the protocol's diagnosis: bin entries name exit symptoms; affected packages have concurrent gaps below the bin-entry alphabet top.

The Ω.5.dd round (Map/Set/Date) had the smallest delta (2): the substrate move at the chosen layer was structurally complete for most affected packages because Map/Set are foundational primitives whose absence blocks at the *first* operation, before other gaps accumulate. This matches the protocol's reading: substrate moves at foundational layers have smaller deltas because the affected populations are gated *first* at that layer.

### Round Ω.5.hh — first application of the protocol

After Doc 720's articulation, the protocol was applied for the first time. The substrate-move list named "real async dispatch with Promise resolution" as the next candidate (predicted unlock 8–15, per Doc 714 §VI Consequence 10). Step 1 of the protocol identified the gated population: 20 packages whose entry file uses async/await/Promise syntax. Step 2 walked the call chain across pipelines for each.

The finding was a *negative result*: zero of the 20 packages currently fail at the async dispatch layer. Their actual failure chains exit at the parser pipeline (12 packages) or at other runtime pipeline gaps (8 packages). The predicted unlock for real async dispatch *as a substrate move* is structurally ~0 at this parity ceiling, not 8–15.

The protocol *pre-empted the round*: real async dispatch was not dispatched. Instead, the methodology re-ranked the substrate-move list by applying the protocol to other candidates.

The diagnostic walk surfaced the kind-detection substrate as a new candidate not present in the original list. Step 1: gated population is 22 packages failing with `parse (cjs wrapper)`. Step 2: chain bundle decomposes into two sub-bundles: 9 packages with `expected '(' after import` (ESM-shape `.js` files mis-classified as CJS) and 10 packages with various other specific parse errors at specific bytes. Step 3: the highest shared layer for the 9-package sub-bundle is `detect_module_kind` in the module loader pipeline. Step 4: predicted unlock for the 9-package sub-bundle = 9 if all otherwise complete; lower otherwise.

The kind-detection sniff was dispatched as Ω.5.hh. The fix: in `detect_module_kind`, read the first 4KB of source and sniff for line-start `import`/`export` keywords before falling back to package.json's `type` field.

Actual unlock: +1 (Rt:TypeError bin grew by 5). Predicted: +9. Delta: 8.

The under-delivery is *informative*: the substrate move at the chosen alphabet top *was* correct for the 9-package gated population (the chain bundle's leaves passed through the kind-detection node, as the protocol predicted), but only 1 of those 9 packages was otherwise complete in other pipelines. The other 8 packages migrated from the parse-error bin into the runtime-error bin, exposing concurrent gaps in the runtime pipeline that the kind-detection move didn't reach.

The protocol's Step 5 iteration applies: the new gated population for each of those 8 packages is now its *next* exit symptom, and Step 2 walks each chain forward from that new exit. The substrate-move list re-ranks accordingly.

### What the engagement's application validates

Across the three rounds (Ω.5.gg pre-protocol; Ω.5.x–Ω.5.dd calibration; Ω.5.hh first protocol application):

1. **Under-prediction by bin-entry-as-top is systematic, not noise.** Eight rounds of calibration data show predicted-vs-actual deltas consistently positive (predicted > actual). Bin entries label exit symptoms; substrate moves chosen by bin-entry-frequency over-predict.

2. **The protocol's pre-round negative result is operationally valuable.** Real async dispatch was not dispatched, saving the engagement the cost of building a substantial substrate artifact that would have delivered ~0 packages. The protocol pays off by *preventing wrong work*.

3. **The protocol's positive result still has prediction error.** Ω.5.hh's chain-bundle analysis correctly located the alphabet top for the 9-package sub-bundle, but the actual unlock was 1 because 8 packages had concurrent gaps. The prediction error here is on the *cross-pipeline-completeness* axis: the protocol identified the right alphabet top but didn't account for how many gated packages would otherwise be complete after the move.

4. **The Step 5 iteration is the natural forward path.** After Ω.5.hh, the next protocol iteration walks each of the 8 migrated packages' new exit chains. The substrate-move list re-ranks from those walks.

The engagement is not finished — parity sits at 46/118 (39%) with the long tail of individual-debug-shape failures dominating. The protocol's value at this state is *qualitative*: it changes the engagement's forward-route reasoning from bin-symptom-counting to alphabet-top-locating. The methodology is now an explicit discipline; previously-implicit prediction errors (Ω.5.gg's miscalibration, the systematic over-prediction across Ω.5.x–Ω.5.dd) are now visible as protocol-diagnosable signals.

## VI.5. Measurement-precision amendment — false-pass correction

A discovery from the protocol's second application (rusty-bun Ω.5.kk, 2026-05-15) requires an amendment to Step 4's predicted-vs-actual discipline as originally stated.

**The phenomenon.** Step 4 specifies the diagnostic check as `|U - A| ≤ 1`, where `U` is the predicted unlock count and `A` is the actual unlock count measured after the substrate move lands. Round Ω.5.kk in the rusty-bun engagement produced a *negative actual delta*: predicted `U = 2` (entities and parse5 unlocked from a TypeScript-compiled-enum pattern), measured `A = -2` (n_ok dropped from 46 to 44).

The naive reading would treat this as a 4-unit prediction error. But the substrate move was structurally correct (the parser was discarding `export var/let/const X = init;` initializer expressions; the fix routes them through the typed variable-statement compile path so initializers actually execute, per ECMA-262 §16.2.3.7). What the move *exposed* was a class of packages — jose, jsonc-parser, micromark — whose pre-fix "pass" status was a *false positive*: their re-export-only ESM index files transitively loaded source files whose `export var X = init;` initializers were silently skipped, and the resulting partial namespace happened to match Bun's namespace shape (specifically the `keyCount` field that the parity probe compares) by coincidence.

Post-fix, those 3 packages' transitive loads run their initializers, hit real semantic gaps deeper in the chain, and fail honestly. The "regressions" are *not* regressions in engine behavior; they are *false-pass corrections*. The engine is now *more correct* on jose, jsonc-parser, micromark than it was pre-fix — it just expresses that correctness as honest failure rather than coincidental success.

**The amendment.** The predicted-vs-actual diagnostic check expands from `|U - A| ≤ 1` to `|U - (A + F)| ≤ 1`, where:

- `A` = actual n_ok delta (literal pass-count change).
- `F` = false-pass exposure count (packages whose pre-fix status was a coincidental match invalidated by the substrate move).
- `A + F` = real substrate-completion delta (the engine's correctness improvement on the gated population, independent of whether each individual package's full chain is complete).

For Ω.5.kk: `U = 2`, `A = -2`, `F = 3`, so `A + F = 1`. The diagnostic check `|2 - 1| = 1 ≤ 1` is satisfied. The substrate move's prediction was correct on the real substrate-completion metric, even though it under-delivered on the raw n_ok counter.

**How to identify F.** A package contributes to `F` when:

1. It was in the OK set before the substrate move.
2. Its OK status was due to a partial-load pattern (early-aborted initializer, skipped declaration body, re-export-only namespace) rather than full evaluation.
3. The substrate move forces fuller evaluation, exposing a real failure that the partial-load pattern previously hid.

The identification protocol: for each package that transitioned OK → fail after a substrate move, compare its post-fix failure location to its pre-fix evaluation depth. If the post-fix failure is *deeper* in the call chain (the package now executes more of its source before failing), it is a false-pass correction. If the post-fix failure is at the *same or shallower* depth, it is a real regression — the substrate move broke working behavior.

The distinction matters because real regressions trigger Step 5 iteration (revisit the alphabet-top location) while false-pass corrections do not (the move is correct; the affected packages need more substrate work to actually pass).

**Why parity-probe-based measurement amplifies this.** The parity probe in the rusty-bun engagement compares `Object.keys(namespace).length` byte-identically against Bun. This makes the probe sensitive to namespace shape, but it does not exercise the namespace's values. A package's namespace can have all the right keys with all the wrong values and still pass the probe. The probe-design choice (validated against the engagement's Tier-Π scope ceiling per Doc 715 §VII) was deliberate; it makes the probe cheap and parallelizable, at the cost of admitting false-pass coincidences when the engine's failure mode produces an empty / partial namespace that happens to match.

The amendment is therefore *probe-shape-dependent*. A probe that exercised values (e.g., invoked a known function and compared the result) would have a lower false-pass rate. A probe that compared full namespace-and-values byte-identically would have a near-zero false-pass rate but be much more expensive. The choice of probe sets the false-pass exposure rate; the protocol's predicted-vs-actual check must account for whatever rate the chosen probe admits.

**Operational implication.** The protocol's Step 5 iteration condition is now: iterate if `|U - (A + F)| > 1`. Under the corrected check, predictions that look wrong in n_ok terms may be right in substrate-completion terms; the protocol does not need to re-walk the chain bundle in those cases. The iteration cost is bounded by real prediction errors, not by probe-design artifacts.

## VI.6. Threshold-escalation amendment — ladder-up when Step 2's walk yields below-threshold signal

[Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies) named the threshold of diagnostic semanticity: chain depth × tag specificity × kind information. When the failure's fault tag is below threshold, Step 2's walk cannot constrain the hypothesis space and Step 3's locator returns ambiguously. The protocol as originally articulated (Steps 1–5) did not name what to do in this case — the substrate-introducer's options were to either (i) try more probes blindly at the substrate tier, or (ii) declare protocol-Step-3-negative and defer to per-package debugging.

The 2026-05-15 round on the jose/ky/get-stream cluster of the rusty-bun engagement produced a third option that is now formalized as a protocol step.

**The pattern.** Three packages exhibited the same bare-tag fault: `TypeError("callee is not callable: undefined")` with no chain, no specific local name, no kind information. Five hand-rolled probes (per Doc 723's Layer D) reproduced no specific structure. Bisect across the named feature set returned NO MATCH for the cluster.

Instead of (i) expanding Layer-D's feature set blindly (route a in Doc 723's terminology) or (ii) accepting protocol-Step-3-negative, the substrate-introducer escalated *up* the apparatus's instrumentation ladder. The escalation: a single-line patch at the engine's `Op::New` error-handling site to append a `(new-callee='X')` hint that names the LoadLocal/LoadGlobal whose value was undefined.

The patch did not flip any package directly. It raised the signal level at one engine site so that the *same fault*, re-emitted on the next run, carried enough information to converge a substrate hypothesis. The next run produced `(new-callee='<global>TextEncoder')` for all three packages, instantly naming the missing substrate (TextEncoder global). One subsequent substrate fix (TextEncoder/TextDecoder stubs) lifted four packages terminally.

**Step 6 — Ladder-up when below threshold.** When Step 2's walk produces tags below the threshold of diagnostic semanticity:

1. Identify the engine site where the bare-tag fault is emitted (typically an error-handler in the call/new/index pipeline).
2. Augment that site with a per-emission hint that names the most recent local/global/property that contributed context (per Doc 723 route-b).
3. Re-run the gated population. Per-failure signal level is now raised at the chosen site; previously-below-threshold faults may now be above threshold.
4. Resume Steps 1–5 with the enriched signal.

The escalation is a *meta-substrate* move: it does not fix any package's bug directly. It instruments the *apparatus that produces the diagnostic*. The cost is one engine-site patch; the benefit is per-emission compounding — every future fault at that site carries the new signal. The compounding rate scales by site, not by failure.

**Distinct from Step 5's iteration.** Step 5 iterates Steps 1–4 when the predicted-vs-actual delta is wide. Step 6 escalates when Step 2 cannot produce a constrained prediction in the first place. The two steps address different gates: Step 5 corrects miscalibration *of* a hypothesis; Step 6 makes *generating* a hypothesis possible at all.

**Empirical demonstration record.** The 2026-05-15 jose/ky/get-stream round provides the demonstrating instance:
- Bare-tag fault at three packages.
- Probe-substrate bisect across eleven combinations returned NO MATCH.
- Single Op::New hint patch (Ω.5.hhhh).
- Same three packages now emit `(new-callee='<global>TextEncoder')` — above threshold.
- One TextEncoder/TextDecoder substrate fix (Ω.5.iiii).
- Four packages flip terminally (jose, ky, get-stream, plus one transitive beneficiary).

The escalation route was named in [Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies) §IV's second amendment as route (b). This Doc-721 amendment formalizes it as Step 6 of the diagnostic protocol so the methodology document carries the operational discipline alongside Doc 723's structural articulation.

**Falsifier specific to Step 6.** If a Step 6 escalation produces no improvement in subsequent runs — the tag still under-constrains the hypothesis after the engine-site enrichment — the chosen instrumentation site was not the load-bearing one. Test: count predicted-vs-actual deltas pre- and post-escalation. If the delta does not tighten across the next several rounds, the escalation was at the wrong site (or, more rarely, the fault genuinely has no recoverable per-emission context, in which case route (a) — denser Layer-D — remains the only path).

**Operational implication for the protocol's terminal-value claim.** [Doc 714](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point) §VI Consequence 13 named the protocol's *systemic* terminal value: Step 3 returns negative across the residual long tail. Step 6 raises the floor at which "terminal" gets declared. A Step-3-negative result that holds *under enriched instrumentation* is genuinely terminal; one that resolves after a Step 6 escalation was never terminal, just under-instrumented.

## VII. Honest scope

The protocol is articulated against the rusty-bun engine substrate, where the pipeline DAG has 16 named pipelines (per Doc 720), the stage signatures are stable, and the substrate decisions cluster at coordination boundaries. Whether the methodology applies to other systems is a corpus-extension question that future engagement can test. The falsifiers in §V are stated with the appropriate scope.

The protocol's leverage at any point in an engagement depends on the substrate's maturity. Early in an engagement when many large substrate moves remain, the protocol's predictions have wider variance (more unknown structure to walk). Late in an engagement at saturation, the protocol's predictions narrow because the residual structure is more visible. The rusty-bun engagement's application sits in the late-saturation regime; the protocol's variance is bounded by the visible chain bundles.

## VIII. Closing

The cross-pipeline diagnostic protocol formalizes the discipline that surfaced during the rusty-bun engagement's Tier-Ω.5 substrate work. Each substrate-widening move has a true top in the engine's modular architecture. The top is locatable by walking each gated package's call chain across pipelines to the highest layer where the fix is structurally complete. The predicted unlock count is the number of leaves in that layer's chain-bundle subtree that are otherwise complete; the predicted-vs-actual delta is diagnostic signal for cross-pipeline completeness; the iteration to convergence is bounded.

The protocol's central operational claim is the engineering counterpart to [Doc 714 §VI Consequence 11](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point)'s structural claim: substrate-widening fixes' alphabet tops are not just locatable in principle — they are locatable by a bounded, finite, polynomial-cost analysis whose preconditions are pipeline decomposition, symptom traceability, and substrate locality. When those preconditions hold, substrate-introduction work becomes prediction-driven rather than discovery-driven.

The engagement that produced this protocol is not the only system to which it applies. The protocol is portable to any engineered substrate that satisfies its preconditions, and the engagement's application is one data point for whether the structural claims are general. The corpus contribution at this articulation tier is the methodology itself; the rusty-bun engagement is the case study; the empirical record (Ω.5.gg through Ω.5.hh) is the calibration data. Future engagements that adopt the methodology contribute their own predicted-vs-actual records to the validation surface.
