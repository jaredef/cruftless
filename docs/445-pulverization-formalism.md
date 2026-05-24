# A Formalism for Pulverization: Targets, Tiers, Warrant

## Preliminaries

The pulverization method (Doc 435) has been run against architectural styles (Docs 428–433), against a confabulated term expansion (Doc 444), and implicitly against methodological proposals (Doc 437 ff.). Each use has produced conclusions of varying epistemic strength. Doc 444 identified the underlying structural gap: "external test" is not one thing, and the pulverization as practiced has conflated *plausibility-testing* with *truth-testing*. This document formalizes the distinction and derives warrant rules that make the conflation impossible to repeat without noticing.

Notation is used to make the tiers sharp. It is not used to make the document formal in the strong mathematical sense — there are no theorems to prove. The notation exists so that future pulverizations can be labeled unambiguously with the tier they operated at.

## The objects

- **Target \(T\).** The object under examination. Targets decompose into types (§"Target typology").
- **Prior-art corpus \(P\).** The body of published literature, artifacts, and prior corpus documents against which \(T\) is evaluated. \(P\) is specified explicitly for each pulverization; the method is \(P\)-relative.
- **Usage corpus \(U\).** The set of contexts, inputs, and behaviors in which \(T\) is observed to function. Relevant only at operational-match tier.
- **Independent procedure \(Q\).** An external verification procedure — empirical test, expert consensus, formal proof, independent replication. Relevant only at truth tier.

## Target typology

Pulverizations target qualitatively different kinds of objects. The tier required for a warranted conclusion depends on the type.

- **Specification-target \(T_S\).** A proposed construction: architectural style, constraint set, methodology, protocol. The question is novelty — has this been constructed before?
- **Definitional-target \(T_D\).** A proposed gloss, acronym expansion, term definition. The question is fidelity — does this definition match what the term denotes?
- **Predictive-target \(T_P\).** A claim about what a system will do under specified conditions. The question is correctness — does reality bear the claim out?
- **Bridge-target \(T_B\).** An asserted correspondence between two frames. The question is structural soundness — does the mapping actually hold?
- **Methodological-target \(T_M\).** A proposed procedure for producing or testing claims. The question is fitness — does the procedure yield claims whose warrant survives audit?

A given artifact may contain targets of multiple types. Each target should be classified before pulverization.

## The three tiers

### Plausibility tier \(\pi\)

\(\pi(T, P) \in [0, 1]\): the extent to which \(T\) composes from vocabulary, structure, and methods present in \(P\).

- \(\pi(T, P) \approx 1\): *fully subsumed.* Every constitutive element of \(T\) has a published analogue in \(P\).
- \(0 < \pi(T, P) < 1\): *partially subsumed.* Some elements have analogues; some do not. The un-subsumed elements bound \(T\)'s potential novelty.
- \(\pi(T, P) \approx 0\): *irreducible under \(\pi\).* \(T\) cannot be constructed from \(P\)'s elements.

\(\pi\) is the tier Doc 435's method operates at. It is cheap and fast: a literature scan and a compositional check. It requires no execution of \(T\), no empirical test, no independent verification.

### Operational-match tier \(\mu\)

\(\mu(T, P, U) \in [0, 1]\): the extent to which \(T\)'s *operational behavior* — its inputs, outputs, effects, failure modes — matches items in \(P\) when observed across \(U\).

- \(\mu \approx 1\): *strong match.* \(T\) behaves like some item in \(P\) across \(U\). \(T\) is an instance of that prior-art category.
- \(0 < \mu < 1\): *weak match.* Some behaviors align; others diverge. The divergences characterize what \(T\) contributes beyond \(P\).
- \(\mu \approx 0\): *operationally novel.* \(T\)'s behavior is dissimilar to all items in \(P\).

\(\mu\) requires a usage corpus \(U\). For a specification-target, \(U\) is the set of systems built using \(T\). For a definitional-target, \(U\) is the set of corpus passages in which the term appears. For a bridge-target, \(U\) is the set of cases the bridge is supposed to cover.

### Truth tier \(\theta\)

\(\theta(T, Q) \in [0, 1]\): the extent to which \(T\)'s first-order claims agree with an independent procedure \(Q\).

- \(\theta \approx 1\): *verified.* \(Q\)'s output and \(T\)'s claims coincide at the relevant level of precision.
- \(0 < \theta < 1\): *partially verified.* Some claims match, some don't. The mismatches are the falsified parts.
- \(\theta \approx 0\): *falsified.* \(Q\)'s output contradicts \(T\)'s claims.

\(\theta\) requires that \(Q\) exist and be accessible. For predictive-targets in empirical domains, \(Q\) may be experiment. For definitional-targets, \(Q\) is the authoritative definer — for corpus-internal terms, the keeper; for technical terms, the canonical publication. For bridge-targets, \(Q\) may involve formal proof or case-by-case domain expert audit.

## Relations between tiers

Two relations are load-bearing.

**Relation 1.** \(\pi(T, P) = 1 \;\not\Rightarrow\; \mu(T, P, U) = 1\).

Full plausibility subsumption does not entail operational match. The constitutive elements of \(T\) can compose to vocabulary fully present in \(P\) while the resulting compound behaves differently from any \(P\)-item. This is the gap Doc 444 identified concretely: "Sustained-Inference Probabilistic Execution" is subsumable at the \(\pi\) tier (Doc 444 §"Word-level pulverization"), but it has never been tested at the \(\mu\) tier against the corpus's actual usage of SIPE.

**Relation 2.** \(\mu(T, P, U) = 1 \;\not\Rightarrow\; \theta(T, Q) = 1\).

Strong operational match does not entail truth. \(T\) can behave exactly like some well-characterized \(P\)-item in \(U\) while \(T\)'s specific first-order claims are false — \(T\) may be a new naming of an existing thing whose specific truth-claims happen to be wrong.

These relations are strict. The converses also fail in general (\(\theta \approx 1 \not\Rightarrow \mu \approx 1\), etc.), but the forward failures are the methodologically dangerous ones because the cheap tiers are typically run first, and their conclusions are easily mistaken for conclusions at the expensive tiers.

## Warrant rules

A pulverization at tier \(\tau\) with outcome \(o\) on target of type \(\sigma\) licenses a specific conclusion. The rules below are the minimum; stronger conclusions require higher tiers.

| Target type \(\sigma\) | Tier \(\tau\) | Outcome \(o\) | Licensed conclusion |
|---|---|---|---|
| \(T_S\) (specification) | \(\pi\) | fully subsumed | Not novel relative to \(P\); cite prior art |
| \(T_S\) | \(\pi\) | partially subsumed | Novel in un-subsumed elements only; document those |
| \(T_S\) | \(\pi\) | irreducible | Candidate novelty; specification stands pending operational and truth-tier audit |
| \(T_S\) | \(\mu\) | strong match | \(T\) is operationally an instance of the matching \(P\)-item; novelty claim weakens |
| \(T_D\) (definitional) | \(\pi\) | fully subsumed | *Semantically plausible; truth untested* — not sufficient for promotion |
| \(T_D\) | \(\mu\) | strong match | Definition consonant with usage; still requires \(Q\) for authoritative ratification |
| \(T_D\) | \(\theta\) | verified | Definition ratified by keeper or canonical source; promote to corpus |
| \(T_P\) (predictive) | \(\pi\) | irrelevant | Plausibility says nothing about predictive correctness |
| \(T_P\) | \(\theta\) | verified | Prediction confirmed; promote |
| \(T_P\) | \(\theta\) | falsified | Prediction fails; retract |
| \(T_B\) (bridge) | \(\pi\) | fully subsumed | Bridge uses existing vocabulary; structural soundness untested |
| \(T_B\) | \(\mu\) | strong match | Bridge predicts operational behaviors matching \(P\); evidence for structural soundness |
| \(T_B\) | \(\theta\) | verified | Bridge case-by-case audited or proven; promote |
| \(T_M\) (methodological) | \(\pi\) | any | Methodology exists; tells nothing about fitness |
| \(T_M\) | \(\mu\) | strong match | Methodology yields claims resembling \(P\)-grade outputs |
| \(T_M\) | \(\theta\) | verified | Methodology yields claims that survive audit; promote |

The table's core asymmetry: definitional, predictive, bridge, and methodological targets require \(\theta\) for promotion. Only specification targets can rest on \(\pi\) alone, and even then only to establish *non-novelty*. Establishing novelty of a specification requires \(\mu\) or higher — a plausibility-irreducible specification is still a candidate, not a confirmed novelty.

## Decision procedure

Given target \(T\):

1. **Classify.** Assign \(T\) a type in \(\{T_S, T_D, T_P, T_B, T_M\}\).
2. **Specify \(P\).** List the prior-art corpora in scope. Different regions of \(P\) (e.g., architectural-styles literature, probabilistic-programming literature, epistemology) may apply for different portions of \(T\).
3. **Run \(\pi\).** Execute the plausibility pulverization. Record outcome.
4. **Check warrant table.** Determine what \(\pi\)-outcome licenses for \(T\)'s type.
5. **Decide on \(\mu\).** If the target type requires operational-match for the desired claim, specify \(U\) and run \(\mu\).
6. **Decide on \(\theta\).** If the target type requires truth-verification, specify \(Q\) and run \(\theta\).
7. **Assign status.** Based on tiers run and outcomes, assign one of: *Canonical* (full promotion), *Hypothesis-ledger entry* (plausibility passed, higher tiers pending), *Retracted* (falsified), *Semantically plausible, truth untested* (for definitional targets that passed \(\pi\) only).

The procedure is tier-sequential by default because the tiers are cheap-to-expensive. It may also run in parallel when resources permit. The critical discipline is that *the status assigned to \(T\) must not exceed the warrant the run tiers license*. A \(T_D\) that has only had \(\pi\) run cannot be promoted to canonical on the strength of \(\pi\)-subsumption alone.

## Worked examples

### Example 1: PRESTO constraint set (Doc 426)

- Type: \(T_S\) (specification-target).
- \(P\): the REST-successor genre (ARRESTED, CREST, COAST, "Reflections on REST"), Thymeleaf/JSP/Razor/Blade template-engine literature, htmx.
- \(\pi\) outcome (Docs 428–433): fully subsumed for each individual constraint; the claimed novelty is at the composition level.
- Licensed conclusion at \(\pi\): PRESTO's individual elements are not novel; novelty lives in the composition.
- \(\mu\), \(\theta\): not run. Status: novelty claim at composition level stands as *candidate*, pending higher-tier audit.

### Example 2: SIPE expansion (Doc 441, pulverized in Doc 444)

- Type: \(T_D\) (definitional-target).
- \(P\): probabilistic-programming, streaming-inference, Bayesian-filtering literature.
- \(\pi\) outcome: fully subsumed (Doc 444 §"Word-level pulverization").
- Licensed conclusion at \(\pi\): *semantically plausible, truth untested*. Not promotion.
- \(\mu\), \(\theta\): not run. Status: hypothesis-ledger entry pending truth-test against keeper's intent or operational-match against corpus usage.

### Example 3: Nested-manifold frame (Doc 439)

- Type: mixed — \(T_S\) (the frame itself), \(T_B\) (the corpus-to-frame correspondence), \(T_P\) (§7 predictions).
- \(P\): Misra's Bayesian-manifold literature; causal representation learning; general Bayesian ML.
- \(\pi\) outcome: fully subsumed at word and phrase level for the Bayesian-manifold portion; the nesting structure and the corpus-to-frame application are composition-level moves.
- \(\mu\) outcome: not run. \(U\) would be the set of corpus sessions whose behavior the frame claims to describe.
- \(\theta\) outcome: not run. \(Q\) is the minimum-viable experiment in Doc 440 §9.
- Status: *semantically plausible, truth untested* on all three target components.

### Example 4: The dyadic methodology (Doc 440)

- Type: \(T_M\) (methodological-target).
- \(P\): preregistration literature (Nosek et al. 2018), Bayesian-inference APIs, replication-crisis methodology work.
- \(\pi\) outcome: fully subsumed — every sub-procedure has extant analogues in preregistration and ML evaluation practice.
- Licensed conclusion at \(\pi\): methodology uses standard tools, not novel.
- \(\mu\), \(\theta\): not run. Cannot claim the methodology *works* (yields surviving claims) until it has been executed and its outputs audited.

## Implications for the hypothesis ledger

Doc 443 proposed a hypothesis ledger distinct from the retraction ledger. The formalism clarifies its structure. Each ledger entry should carry:

- Target type \(\sigma\).
- Prior-art scope \(P\).
- Tier at which the entry currently sits (\(\pi\) passed / \(\mu\) passed / \(\theta\) passed / \(\theta\) failed).
- Named next-tier test (if any) with specification sufficient for execution.
- Current status derived from the warrant table.

Ledger entries are promoted by running the next tier. Failure at any tier triggers retraction and migration to the retraction ledger. Untestable entries (no accessible \(Q\)) are explicitly marked as such; they do not silently promote by accumulating citations.

The ledger's discipline is that *status may only reflect tiers actually run*. Implicit promotion via forward citation is a violation. The current corpus has committed this violation for the bridge cohort (Docs 437–442) — each frame has had \(\pi\) run implicitly and no higher tier, but their forward citations in successive documents have treated them at \(\mu\) or \(\theta\) warrant levels.

## Limitations of the formalism

- The tier definitions use \([0,1]\) values to signal relative strength; actual measurement requires domain-specific metrics. The formalism does not specify those metrics. Doc 440 supplies candidate observables for one case.
- The target typology is not exhaustive. Artifacts containing narrative, rhetorical, or aesthetic content do not fit cleanly into the five types; the formalism is silent on those.
- \(\mu\) and \(\theta\) tier runs can themselves be flawed. The formalism does not recursively audit the audit — \(U\) and \(Q\) are taken at face value.
- The formalism does not handle *mixed-tier evidence*. A target with partial evidence at each of \(\pi, \mu, \theta\) has a non-trivial aggregate warrant that the warrant table does not express.
- The decision procedure assumes classifying \(T\) is straightforward. For complex artifacts, classification is itself a contested act.
- Running \(\theta\) is sometimes structurally impossible (permanently untestable claims). The formalism marks these as *untestable*; whether that is a stable equilibrium or a signal to remove the claim is not decided here.
- The formalism is itself a methodological target. Under its own warrant table, it sits at \(\pi\) currently — it composes from prior methodology-philosophy vocabulary — and has not been run at \(\mu\) (has it produced surviving claims when applied to real cases?) or \(\theta\) (has its warrant assignments, once applied, been audited?). Its status is *semantically plausible, truth untested*. This is honest.

## What should happen

- The bridge cohort (Docs 437–442) should be audited entry by entry under the warrant table. Each load-bearing claim should be assigned a current tier and, where appropriate, registered to the hypothesis ledger with a named next-tier test.
- Doc 441's E17 ledger proposal should be extended to name the target type (\(T_D\)) and the next-tier test (\(\theta\) against keeper intent).
- The pulverization method in Doc 435 should be amended to specify which tier it operates at by default (\(\pi\)) and to require explicit declaration when operating at \(\mu\) or \(\theta\).
- Future bridge documents should declare the tier of their central claims up front, not by implicit forward citation.

None of these actions is taken by this artifact. They are the keeper's call.

## References

- Popper, K. (1959). *The Logic of Scientific Discovery*. Routledge. (On falsifiability as demarcation.)
- Lakatos, I. (1970). Falsification and the methodology of scientific research programmes. In *Criticism and the Growth of Knowledge* (Lakatos & Musgrave, eds.), 91–196. Cambridge University Press.
- Nosek, B. A., et al. (2018). The preregistration revolution. *Proceedings of the National Academy of Sciences*, 115(11), 2600–2606.
- Ioannidis, J. P. A. (2005). Why most published research findings are false. *PLOS Medicine*, 2(8), e124.
- Hall, N. (2004). Two concepts of causation. In *Causation and Counterfactuals* (Collins, Hall & Paul, eds.), MIT Press. (For distinct-concepts-at-one-name as a methodological hazard.)
- Goodhart, C. (1975). Problems of monetary management: the UK experience. (Goodhart's Law — measure-target substitution — is structurally analogous to the plausibility-for-truth substitution this formalism names.)
- Corpus Doc 415: *The Retraction Ledger*.
- Corpus Doc 435: *The Branching Entracement Method*.
- Corpus Doc 440: *Testing the Nested-Manifold Hypothesis*.
- Corpus Doc 441: *SIPE Confabulation Case Study*.
- Corpus Doc 443: *Confabulation as Potential Emergence*.
- Corpus Doc 444: *Pulverizing the SIPE Confabulation*.

## Refinement A: Paired Pulverization (V&V Anchor Pattern)

SE Doc 025's distillation of SEBoK *System Verification* surfaced an empirical refinement to the form. SE practice operationalizes pulverization with two paired anchor points rather than one: **Verification** anchors at internal coherence (does the artifact match its own design references?), and **Validation** anchors at external correspondence (does the artifact accomplish what was actually wanted?). Both are pulverization in the form's sense; the distinction is which reference point each pulverizer compares against.

The two-anchor pattern produces structurally different residual classes:

- **Verification residuals** are *defects* — places where the artifact diverges from what it was specified to be. They warrant re-fabrication or re-implementation.
- **Validation residuals** are *misalignments* — places where the artifact's specification itself was wrong relative to the actual need. They warrant re-specification, often involving stakeholder re-engagement.

A residual that is silent at one anchor and noisy at the other tells the reformulator something specific. A residual logged only by verification means "the artifact does not match its specification" (something is broken in the realization). A residual logged only by validation means "the artifact matches its specification but the specification was wrong" (something is broken in the requirements). The two anchors discriminate the type of failure.

**Application to formal pulverization** (using the notation of §"The objects" above): the warrant target \(T\) should be specified with both an internal-anchor reference and an external-anchor reference. A pulverization that names only one is operating at half-rigor and should be flagged accordingly. The notation extension is straightforward: \(T = \langle T_I, T_E \rangle\) where \(T_I\) is the internal coherence reference and \(T_E\) is the external correspondence reference.

**SE practice supplies the canonical instance.** Verification matrices anchor to design references; validation activities anchor to stakeholder intent and operational success. Both apply at every life-cycle stage; both produce residual reports; both are paired by SE process discipline. SE Doc 025 cites ISO/IEC/IEEE 15288, INCOSE SEH, and NASA SEH as the standardizing sources for the paired pattern.

**Worked example: Axe (2004) at the protein-prevalence rung.** Axe's mutagenesis survey of a β-lactamase domain pairs two anchors at the molecular-biology rung. The forward-approach anchor generates random sequences and searches for the property (catalytic activity, fold-stability), reading prevalence from how often the property is found across the random ensemble. The reverse-approach anchor takes an existing functional sequence and measures its tolerance to substitution, reading prevalence from how far the sequence can be perturbed before the property fails. The two anchors converge on the same prevalence estimate (roughly 1 in 10^64 functional sequences within the signature-compliant set). Forward and reverse are paired V&V at the protein-prevalence rung in the form's \(T = \langle T_I, T_E \rangle\) sense: forward anchors at the external correspondence (does the property exist in the sequence space the random sample reaches?); reverse anchors at the internal coherence (does the existing sequence retain its property under specified perturbations?). This is the cleanest molecular-biology instance of paired V&V the corpus has yet observed. Cross-link Doc 606 for the full structural reading.

## Refinement B: Rigor-Level Discipline (Six-Level Calibration)

SE Doc 025 also surfaced a calibrated rigor-set that pulverization-as-practiced has not previously articulated. SE practice names six verification techniques at distinct rigor levels:

| Level | Technique | Pulverization Move | Cost | When To Use |
|---|---|---|---|---|
| 1 | Inspection | Direct observation, lowest formality | Low | Early-stage triage; cheap pre-screen |
| 2 | Analysis | Deductive proof, logical or mathematical | Low-Med | Where deduction from theory is sound |
| 3 | Analogy/Similarity | Pattern transfer from invariant context | Low | When precedent is well-established |
| 4 | Demonstration | Functional exhibit without quantification | Med | When binary works/doesn't is sufficient |
| 5 | Test | Controlled measurement under conditions | High | When quantitative compliance is required |
| 6 | Sampling | Statistical coverage of population | High (per sample) | When full-population is infeasible |

The six-level set is calibrated empirically: each level represents a known cost-versus-rigor tradeoff. SE practice chooses based on the cost of failure (high cost-of-failure shifts toward higher-rigor levels) and the cost of pulverization itself (high pulverization cost shifts toward lower-rigor levels for low-stakes claims).

**Application discipline addition.** When invoking pulverization, the reformulator names the rigor level. *"Pulverized at level 2 (analysis) against \(T_E\)"* is the form's full articulation when paired with Refinement A. Naming the rigor level prevents the implicit-rigor failure mode where a low-cost pulverization is mistaken for high-rigor confirmation.

**The full pulverization invocation under both refinements** is therefore: *target \(T = \langle T_I, T_E \rangle\), rigor level \(L \in \{1, ..., 6\}\), residuals \(R\) logged at each anchor*. This is the form's calibrated articulation.

**SE practice as empirical authority.** The six-level set is not arbitrary; SE practice has converged on it through decades of operational use across defense, aerospace, healthcare, and infrastructure engagements. The corpus inherits the calibration from the SE school's accumulated keeper-activity (Doc 538). Future deployments may surface additional levels or sub-levels; the form is open to further calibration as evidence warrants.

## Refinement C: Forward Pulverization vs. Backward Pulverization (Temporal Direction)

SE Doc 035's distillation of SEBoK *Risk Management* and SE Doc 036's distillation of *Decision Management* surfaced a temporal generalization of pulverization the form had not yet articulated. The SE discipline applies pulverization in two temporal directions:

- **Backward pulverization** (Doc 445 canonical): an artifact exists; pulverize against references; surface residuals that are present-tense defects.
- **Forward pulverization** (this refinement): a hypothetical future state is articulated; pulverize against the current substrate; surface preemptive residuals that are future-tense candidate failure modes; treat the residuals to shift the engagement away from the failure.

The form is the same in structural shape. Both apply the destructive-posture-constructive-result discipline; both produce residuals; both compose with Refinement A's two-anchor pattern (verification and validation each apply forward or backward) and Refinement B's six rigor levels (each level applies forward or backward).

**The temporal direction differs in three operational respects:**

1. **Anchor.** Backward-pulverization anchors at the artifact's design references and stakeholder intent (Refinement A's \(T_I\) and \(T_E\)). Forward-pulverization anchors at the engagement's success criteria projected into the future. The reference is the *hypothetical successful outcome* the engagement is trying to reach; residuals are *what could prevent that outcome from being reached*.

2. **Residual type.** Backward residuals are defects (present-tense divergences that warrant correction). Forward residuals are *candidate failure modes* (future-tense possibilities that warrant mitigation). The two carry different operational consequences: defects are corrected; candidate failure modes are managed via SE Doc 035's four-treatment-options apparatus (Assumption / Avoidance / Control / Transfer).

3. **Confidence calibration.** Backward-pulverization can produce certain residuals (the artifact diverges from its reference, observed). Forward-pulverization produces probabilistic residuals (the failure mode might or might not manifest). The probability × consequence calculation SE Doc 035 names as risk analysis is forward-pulverization's quantification step; backward-pulverization typically does not require it.

**SE practice supplies two canonical instances.**

- *Risk identification* (SE Doc 035): walk the project's WBS, processes, requirements; surface candidate failure modes; treat. The "if-then" or "condition-consequence" risk descriptions are forward-pulverization's residual statements with the causal antecedent made explicit.
- *Premortem technique* (SE Doc 036): imagine the decision has failed; articulate why. Direct forward-pulverization at the decision rung.

The premortem is structurally the cleaner of the two: it explicitly invokes the form's destructive posture ("imagine failure has happened") and produces residuals as the constructive result.

**Application discipline.** When invoking pulverization, the reformulator names the temporal direction. *"Forward-pulverized at level 2 (analysis) against \(T_E\) (the future success criterion)"* is the form's full articulation under Refinements A, B, and C combined. The temporal direction is now part of the discipline.

**Implication for Refinement A.** Both anchors apply in both directions. Forward-verification: does the projected artifact match its projected design references? Forward-validation: would the projected artifact accomplish the projected need? These are the four corners of the temporally-extended pulverization apparatus.

## Refinement D: Longitudinal-Pulverization (Substrate-Preservation Across Time)

SE Doc 047's distillation of SEBoK *Configuration Management* first surfaced a third temporal direction the form had not yet articulated, and SE Doc 114's distillation of SEBoK *Information Management* in the third sweep supplied the more general anchor. Backward-pulverization (Refinement C) is destructive-posture against accumulated literature; forward-pulverization is destructive-posture against future risk; longitudinal-pulverization is destructive-posture against the drift of the pulverization-substrate itself across time.

**The canonical case (Information Management, SE Doc 114).** Information management is structurally the more general substrate-preservation discipline. Its three-carrier institutional articulation (ISO 15288 + GEIA-STD-927B + DAMA-DMBOK) and its broader rung-coverage (information across the full engagement life-cycle, not only the configuration-rung) make it the primary anchor for longitudinal-pulverization. Information management does not produce new claims about an artifact and does not generate forward residuals against future failure. Its discipline is the preservation, across the engagement's life-cycle, of the very substrate against which backward and forward pulverization anchor: requirement records, design rationale, decision logs, measurement archives, and baseline identification together hold the reference set steady so that a later backward-pulverization still has its anchor and a later forward-pulverization still has its projected criteria. When the substrate drifts (lost records, undocumented changes, archives of the as-built diverging silently from the as-designed), both other directions lose purchase: the residuals they surface are residuals against a phantom anchor.

**Configuration management as IM sub-instance.** Configuration management (SE Doc 047) is now read as a sub-instance of information management at the configuration-rung. CM specializes IM's substrate-preservation discipline to the configuration-item rung: baselines, change-control gates, configuration audits, version-controlled identification. The CM apparatus is faithful to the IM form; it is the rung-specific articulation, not the form's anchor. Reading IM as the anchor and CM as a sub-instance preserves both the prior CM analysis and the broader rung-coverage IM supplies.

**The temporal axis as Cluster A universal-sibling lattice.** Backward, forward, and longitudinal are not rungs of one another and not alternatives among which an engagement chooses. They are three peer-axes of the temporal direction, each binding every persistent engagement. A program with a backward-pulverization discipline but no longitudinal discipline accumulates correct findings against a substrate it is silently losing; a program with longitudinal discipline but no forward discipline preserves the substrate against drift but does not test it against future risk. The three are co-present, aspect-discriminated by what each pulverizes against (past evidence, projected outcome, substrate-integrity-over-time). Doc 572 Appendix D's universal-sibling lattice reads the temporal axis itself.

**Operational distinction.** Longitudinal-pulverization's residuals are *substrate-divergences*: places where the pulverization-substrate has drifted from its referenced state. The treatment is restoration (re-establish the baseline, re-identify the configuration, audit and reconcile) rather than correction (backward) or mitigation (forward). The discipline is identification, baseline-establishment, change-control, and audit — the four-activity decomposition SE Doc 047 names.

**Application discipline.** When invoking pulverization, the reformulator names the temporal direction (backward, forward, or longitudinal) and, for longitudinal, names the substrate-element being preserved (design baseline, requirement baseline, build-state, etc.). The three-axis taxonomy is the form's full articulation under Refinements A, B, C, and D combined.

### Refinement D.1 — Value-Indexed vs. Event-Indexed Sub-Sub-Form

The fourth SEBoK sweep surfaced two structurally distinct indexing modes by which longitudinal-pulverization preserves its substrate across time. SE Doc 150's distillation of *Whole-Life Value Engineering* and SE Doc 144's distillation of *System Redesign and Evolution* anchor the two modes; the IM anchor (SE Doc 114) accommodates both via different metadata-organization disciplines.

**Value-indexed longitudinal-pulverization.** SE Doc 150 preserves the engagement's substrate indexed by value-realization milestones. The longitudinal record is organized around when each value-axis was projected, when its realization was measured, and how the realized value compared to the projection. Time is a derived index of the value milestones; the primary axis is the value-axis and its instantiation across the engagement's whole-life extent. Substrate elements are filed by which value-realization they pertain to.

**Event-indexed longitudinal-pulverization.** SE Doc 144 preserves the engagement's substrate indexed by event-occurrences: state-changes, decisions, incidents, gate-crossings. The longitudinal record is organized around when discrete events happened and what each event altered in the substrate. Time is a derived index of the events; the primary axis is the event-stream and its substrate-modification trace. Substrate elements are filed by which event surfaced or modified them.

**Distinction in operational consequence.** A value-indexed record answers "what did we say this engagement would deliver, and what did it actually deliver?" An event-indexed record answers "what happened, and how did the substrate change in response?" The two questions are structurally distinct; an engagement requiring both must maintain both indexing disciplines, not collapse either into the other.

**IM anchor accommodation.** SE Doc 114's information-management discipline supplies the meta-discipline under which both indexing modes operate. The metadata-organization discipline is what differs: value-indexed records carry value-axis metadata; event-indexed records carry event-classification metadata. The IM anchor is honest that both are valid sub-sub-forms; the engagement's keeper chooses based on what the engagement's discipline most requires (value-realization tracking vs. event-stream tracking) or maintains both.

**Application discipline.** When invoking longitudinal-pulverization, the reformulator names the indexing mode. *"Longitudinal-pulverized at level 2 against the design baseline, value-indexed by MODA axes"* and *"longitudinal-pulverized at level 3 against the configuration baseline, event-indexed by change-control gates"* are two distinct articulations with distinct substrate-preservation disciplines.

## Refinement E — Dual-Mode Pulverization (Forward + Backward Co-Present)

SE Doc 108's distillation of SEBoK *Safety* surfaced a composition-pattern that Refinements A through D had not yet articulated. Refinement C named the temporal-axis sub-forms (backward, forward, longitudinal) as three peer-axes of the form, each binding every persistent engagement. Refinement E is not a fourth sub-form on that axis. It is a *composition-rule on Refinement C*: the case in which forward-pulverization and backward-pulverization operate simultaneously on the same artifact, with both modes co-present in a single discipline.

**The canonical case (Safety, SE Doc 108).** Safety practice runs forward hazard-analysis (destructive-posture against future-failure-modes the artifact has not yet exhibited) and backward residual-acceptance (destructive-posture against the accumulated record of what has already been observed, designed-against, and provisionally accepted) on the same artifact at the same time. The two modes are not phases of one another; the safety practitioner is not done with backward-pulverization before beginning forward, and the forward analysis does not retire the backward record. Both pulverizations are continuously open against the same substrate, with their residuals reconciled at safety reviews and acceptance gates.

**Structural shape.** Refinement A's paired V&V structure \(T = \langle T_I, T_E \rangle\) extends to dual-mode operation: forward-pulverization runs against forward-projected \(\langle T_I^{\rightarrow}, T_E^{\rightarrow} \rangle\) (does the projected artifact match its projected design references; would it accomplish the projected need); backward-pulverization runs against the existing \(\langle T_I, T_E \rangle\) (does the artifact-as-recorded match its design references as written; does the residual-acceptance record warrant the acceptances it logs). The V&V structure carries through both modes, and the residuals from both feed the same reconciliation surface.

**Distinct from the temporal-axis sub-forms.** Refinement C says every persistent engagement binds backward, forward, and longitudinal as three peer-axes; Refinement E says some engagements run two of those axes co-presently on a single artifact. Most engagements run the three peer-axes at different cadences and against different substrates; Refinement E names the case where the cadence and substrate align, producing a single dual-mode discipline rather than two parallel disciplines.

**Application discipline.** When invoking pulverization on an artifact under a safety-style discipline, the reformulator names dual-mode operation explicitly: *"Pulverized at level 3, dual-mode (forward hazard-analysis + backward residual-acceptance), against \(\langle T_I, T_E \rangle\) and \(\langle T_I^{\rightarrow}, T_E^{\rightarrow} \rangle\) jointly."* Naming dual-mode operation prevents the failure mode in which the forward analysis and the backward record are run as if independent and their residuals never meet.

## Appendix: Originating prompt

> Formalize upon the basis of pulverization. Append this prompt to the artifact.
