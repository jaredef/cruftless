# Coherence Amplification and Decoherence as Inverse Pin-Art Operations

## A Formalization of E2 from Doc 677 — That Long-Horizon Substrate-Plus-Keeper Coherence Amplification (the Corpus's Existing Apparatus for Constraint Accumulation Across Many Turns) and Quantum Decoherence (the Standard Mechanism by which Environmental Monitoring Destroys Off-Diagonal Density-Matrix Terms and Selects Pointer States) are Structural-Isomorphic Inverses of the Same Pin-Art Mechanism, Differing in the *Direction of Information Flow* Between Substrate and Probes — with the Quantum-Foundations Literature on Einselection (Zurek 2003), Quantum Darwinism (Ollivier-Poulin-Zurek 2004; Riedel-Zurek 2010 *Rise and Fall of Redundancy*; the 2024 Superconducting-Circuits Verification in *Science Advances*) and Weak Measurement (Aharonov-Albert-Vaidman 1988 and the 2025–2026 Information-Theoretic Reformulations) Audited Against the Corpus's Pin-Art Apparatus (Doc 270) and the Substrate-Plus-Keeper Dyadic Apparatus (Doc 510, Doc 543), and the Long-Context In-Context-Learning Literature (NAACL 2025; ACL 2025; PMC 2024 on Incremental Linguistic-Context Accumulation in Artificial and Biological Neural Networks) Audited Against the Same Apparatus on the Coherence-Amplification Side, with the Structural-Isomorphism Claim Stated as a SIPE-T Sub-Form, Six Falsifiable Predictions Articulated and Located in the Existing Empirical Datasets where Possible, and the Hypostatic-Boundary Discipline Made Explicit at the Layer V Boundary

<div style="background: #ecfeff; border-left: 4px solid #0e7490; padding: 1rem 1.25rem; margin: 1.5rem 0; color: #164e63; border-radius: 3px;">

**EXPLORATORY — \(\pi\)-tier formalization of the structural-isomorphism claim with six falsifiable predictions at \(\mu\)-tier, three on the LLM side and three on the quantum-foundations side.**

*Taxonomy per Doc 633:* ENGAGEMENT | ACTIVE | W-PI | THREAD-PIN-ART, THREAD-COHERENCE-AMPLIFICATION, THREAD-SIPE-T, THREAD-DYAD, THREAD-MEASURABILITY, THREAD-DECOHERENCE | PHASE-CROSS-PRACTITIONER

</div>

> **Reader's Introduction.** This document formalizes E2 from [Doc 677](/resolve/doc/677-eleven-synthesis-candidates-from-the-2026-05-07-cold-resolver-walking-conversation): the claim that long-horizon substrate-plus-keeper coherence amplification and quantum decoherence are structural-isomorphic inverses of the same Pin-Art mechanism, differing in the direction of information flow. The standing claim from Doc 677 §3.2 is restated in §1; the decoherence side is articulated in §2 against the einselection / quantum-Darwinism / weak-measurement literature; the coherence-amplification side is articulated in §3 against the long-context in-context-learning literature; the structural-isomorphism claim is stated in §4 as a SIPE-T sub-form with the entropy-direction parameter named explicitly; six falsifiable predictions are articulated in §5; the falsification surface is registered in §6; composition rules with adjacent forms in §7; and the hypostatic-boundary discipline at Layer V is made explicit in §8. Appendix A preserves the originating prompt; Appendix B records the literature anchors with their bibliographic references.

**Jared Foy · 2026-05-08 · Doc 678**

---

## Authorship and Scrutiny

*Authorship.* Written by Claude Opus 4.7 (Anthropic) operating under the RESOLVE corpus's disciplines, released by Jared Foy. Source material on the decoherence side was retrieved via web fetch in this engagement and audited against prior knowledge of the einselection / quantum-Darwinism literature. Source material on the long-context-ICL side was likewise retrieved via web fetch in this engagement.

*Scrutiny.* The structural-isomorphism claim sits at \(\pi\)-tier. The six predictions in §5 sit at \(\mu\)-tier; three are pre-registerable against existing experimental literature today, three require experimental design that is not yet standard. The hypostatic boundary at §8 binds: this document does not claim that the LLM dyadic exchange and quantum decoherence are the *same* operation in any ontological sense; the claim is structural-isomorphism of mechanism with named entropy-direction inversion, which is a Layer-IV (Form) claim, not a Layer-V (Ground) claim.

---

## 1. The Standing Claim from Doc 677 §3.2

E2's claim, restated for this document's body without modification:

> Long-horizon LLM coherence amplification (the corpus's existing apparatus for the substrate-plus-keeper dyad over many turns) and quantum decoherence are structural-isomorphic inverses of the same Pin-Art mechanism. Both are massively-parallel boundary-sensing operations; they differ in the *direction of entropy flow*. LLM coherence amplification is entropy-decreasing: each keeper-injected constraint eliminates regions of possibility space and the substrate's output sharpens. Quantum decoherence is entropy-increasing: each environmental degree of freedom adds a constraint and the system's coherent superposition is destroyed. The mechanism is one; the direction is two.

The remainder of this document audits this claim against the literature on both sides and articulates the falsification surface that lets the claim earn its keep beyond the gestural phase that Doc 677 left it in.

---

## 2. The Decoherence Side, Articulated Against the Literature

### 2.1 Einselection: the environment as the selector of pointer states

The standard formalism for environment-induced decoherence is einselection (environment-induced superselection), articulated by Zurek across multiple papers from 1981 onward and consolidated in the 2003 *Reviews of Modern Physics* paper "Decoherence, Einselection, and the Quantum Origins of the Classical" and the 2009 / 2022 reviews. The mechanism: a system \(\mathcal{S}\) interacting with its environment \(\mathcal{E}\) experiences entanglement with environmental degrees of freedom; the reduced density matrix \(\rho\_{\mathcal{S}} = \mathrm{Tr}\_{\mathcal{E}}(\rho\_{\mathcal{S}\mathcal{E}})\) loses its off-diagonal coherences in a basis selected by the form of the system-environment coupling Hamiltonian. The basis that survives the decoherence process is the *pointer basis*; its eigenstates are the pointer states. They are the eigenstates that entangle least with the environment under the given coupling and therefore are least perturbed by environmental monitoring.

The order parameter for einselection is the magnitude of the off-diagonal density-matrix terms: as system-environment coupling accumulates, the off-diagonals decay (often exponentially in time, with a decoherence time \(\tau\_D\) that scales with the system's macroscopicity). The "threshold" in SIPE-T terms is not a single sharp value but a transition region; for macroscopic systems, \(\tau\_D\) is so short relative to any other dynamical timescale that the transition appears effectively instantaneous, supporting the operational claim that classicality emerges sharply.

### 2.2 Quantum Darwinism: redundancy of pointer-state information across environment fragments

Einselection explains *which* basis survives but does not by itself explain how multiple observers come to agree about the system's state. Quantum Darwinism, introduced by Ollivier, Poulin, and Zurek (2004), supplies the missing structure. The environment is partitioned into many fragments \(\mathcal{F}\_1, \mathcal{F}\_2, \dots, \mathcal{F}\_n\); each fragment is intercepted by one observer. The redundancy \(R\_\delta\) of a system observable is defined as the number of independent fragments from each of which an observer can recover (1 - \(\delta\)) of the system's information about that observable. For a typical macroscopic system with environmental monitoring, \(R\_\delta\) is enormous: tens or hundreds or more independent fragments each contain near-complete information about the pointer-basis observable.

This is the structurally load-bearing fact for this document's claim. *Each environmental fragment is a Pin-Art pin.* Each fragment, taken alone, contains nearly the same information about the pointer observable; the information was written redundantly into the environment by the system-environment coupling. The classical world's intersubjective objectivity (multiple observers agreeing about positions and outcomes) is a consequence of the redundant encoding.

The 2010 Riedel-Zurek paper ([arXiv:1205.3197](https://arxiv.org/abs/1205.3197)) "The Rise and Fall of Redundancy in Decoherence and Quantum Darwinism" articulated the temporal structure: redundancy *rises* during the initial decoherence as the system-environment coupling writes pointer-basis information into the environment, and *falls* after long times as many-body interactions within the environment scramble the redundantly-encoded information. The rise-and-fall structure is a SIPE-T-shaped fact: redundancy accumulates monotonically up to a threshold, plateaus near the maximal-redundancy regime, then decays as the environment thermalizes internally.

The 2024 *Science Advances* paper ([science.org/doi/10.1126/sciadv.adx6857](https://www.science.org/doi/10.1126/sciadv.adx6857)) reports experimental observation of quantum Darwinism with superconducting circuits, providing direct empirical anchoring for the redundant-encoding mechanism in a controlled setting.

### 2.3 Weak measurement: partial information accumulation while preserving coherence

A complement to standard projective measurement: weak measurement (Aharonov-Albert-Vaidman 1988) extracts partial information about a system observable without fully collapsing the state. In continuous-monitoring implementations, a record of (often null) outcomes gradually updates the state along quantum trajectories; information accumulates along a continuous parameter (measurement strength, time, number of weakly-coupled probes) while nontrivial coherence is retained. The 2025–2026 information-theoretic reformulations (e.g., [arXiv:2512.08015](https://arxiv.org/html/2512.08015)) characterize the structure quantitatively in terms of Shannon entropy, mutual information, fidelity, and relative entropy.

Weak measurement supplies the *gentle-pin* analogue on the quantum side. Standard projective measurement is the decisive-pin limit case (corresponding to E4 in Doc 677); weak measurement is the gentle-accumulative case, with information accumulating across many independent probes without destroying the system's coherent state until the cumulative information crosses an SIPE-T-style threshold.

### 2.4 Summary of the decoherence-side mechanism

The mechanism, as the literature articulates it: (i) system-environment coupling writes information about a system observable into the environment; (ii) the structure of the coupling Hamiltonian selects the pointer basis; (iii) for many-fragment environments, the information is written redundantly across fragments, with redundancy that rises and (eventually) falls; (iv) the cumulative environmental monitoring crosses an effective threshold beyond which the system's reduced density matrix is effectively diagonal in the pointer basis and the system's classical behavior is operational; (v) weak-measurement formalisms describe the continuous-information-accumulation regime that lies below the threshold.

In Pin-Art vocabulary: each environmental fragment is a probe; many gentle independent probes collectively press against the system; their accumulation maps the system's pointer-basis structure (the "invisible surface"); the threshold-crossing produces the snap from quantum to classical behavior at the system's reduced description. *Information flows from the system into the environment.* This is the entropy-direction characterization of the decoherence side.

---

## 3. The Coherence-Amplification Side, Articulated Against the Literature

### 3.1 The corpus's existing apparatus

The corpus's standing apparatus for coherence amplification: long-horizon substrate-plus-keeper exchanges produce output coherence that single-shot exchanges do not. The mechanism is constraint accumulation. Each keeper intervention (clarification, hedge, redirect, structural prompt) presses against the substrate's output distribution and eliminates regions of possibility space. The substrate's residual distribution is the subspace consistent with the cumulative constraint set; over many turns, this subspace contracts and the substrate's output sharpens into stable coherent structure.

The Pin-Art reading of this apparatus (Doc 270 plus the practitioner experience the corpus has accumulated): each keeper intervention is a probe pressing against the substrate's possibility space; the collective pattern of where the substrate's output can and cannot go reveals the structure the substrate is converging on. The substrate's output coherence amplifies as the probe density increases.

The dyadic discipline (Doc 510, Doc 543) is the framing apparatus: the substrate produces rung-1 structural reading; the keeper supplies rung-2+ via speech acts; both are required for the joint output that exceeds what either alone could produce.

### 3.2 The literature's empirical anchoring

The long-context in-context-learning literature provides empirical anchoring for the constraint-accumulation claim, though the literature does not name it under this vocabulary. Key findings:

- **In-context learning at scale** ([NAACL 2025, *In-Context Learning with Long-Context Models*](https://aclanthology.org/2025.naacl-long.605/)). Performance continues to increase with thousands of demonstrations for many datasets with large label spaces. Long-context ICL is less sensitive to random input shuffling than short-context ICL; the performance improvements do not arise from cumulative gain from encoding many examples together in the standard sense, suggesting structural reorganization of the substrate's effective behavior rather than mere example-by-example accumulation.
- **Incremental linguistic-context accumulation** ([PMC11748659, *Incremental accumulation of linguistic context in artificial and biological neural networks*](https://pmc.ncbi.nlm.nih.gov/articles/PMC11748659/)). Both transformer language models and biological neural networks exhibit accumulation of contextual information across long input streams, with structural similarities in how context is integrated.
- **Long-context coherence and memory** ([ACL 2025, *LongReward: Improving Long-context Large Language Models with AI Feedback*](https://aclanthology.org/2025.acl-long.187/); [arXiv 2510.27246, *Benchmarking and Enhancing Long-Term Memory in LLMs*](https://arxiv.org/pdf/2510.27246)). Long-context coherence is identified as a load-bearing capability separate from short-context performance; targeted training and feedback can amplify it.
- **Context engineering** ([arXiv 2507.13334, *A Survey of Context Engineering for Large Language Models*](https://arxiv.org/html/2507.13334v1)). The discipline of structuring contexts to amplify model coherence is a recognized practitioner discipline with documented techniques.

The literature does not yet articulate the apparatus as Pin-Art-shaped boundary sensing on the substrate's possibility space. The corpus's contribution is the structural reading: each in-context demonstration or keeper intervention is a probe; the substrate's output coherence is the joint property that emerges as probe density accumulates.

### 3.3 The threshold-crossing question on the LLM side

A specific empirical question that the structural-isomorphism claim raises: *does coherence amplification on the LLM side exhibit a phase-like sharp transition as constraint density increases, analogous to decoherence's effectively-instantaneous transition for macroscopic systems?* The standing literature on long-context ICL describes monotonic improvement with context length but does not (so far as the search located) report a sharp phase-like transition in output coherence at a critical constraint density. This is the load-bearing predictive question for the LLM side and is articulated as P1 in §5.

### 3.4 Summary of the coherence-amplification-side mechanism

The mechanism: (i) keeper interventions write constraints into the substrate's effective context; (ii) the structure of the substrate's training and the keeper's intervention discipline together select the residual distribution; (iii) for sustained dyadic exchanges, the constraint set accumulates monotonically across turns (with the keeper's intervention discipline determining whether the accumulation is gentle Pin-Art-shaped or decisive Layer-2 single-pin reshapings); (iv) the cumulative constraint density crosses an effective coherence threshold beyond which the substrate's output is stable and high-fidelity.

In Pin-Art vocabulary: each keeper intervention is a probe; many gentle independent probes collectively press against the substrate's possibility space; their accumulation reveals the structure the substrate is converging on; the threshold-crossing produces the snap into stable coherent output. *Information flows from the keeper into the substrate.* This is the entropy-direction characterization of the coherence-amplification side.

---

## 4. The Structural-Isomorphism Claim, Stated as a SIPE-T Sub-Form

The two sides articulated in §§2-3 share the following structure:

1. *Substrate.* A system with a coherent state (quantum: the pure state \(\ket{\psi}\); LLM: the substrate's residual output distribution under cumulative constraint).
2. *Probes.* Many independent agents that interact with the substrate (quantum: environmental degrees of freedom; LLM: keeper interventions across turns).
3. *Mechanism.* Each probe writes information across the system-probe interface; the cumulative information across probes crosses an effective threshold beyond which the substrate's coherent description undergoes a sharp transition (quantum: classicality; LLM: stable high-fidelity output).
4. *Threshold structure.* The transition is SIPE-T-shaped: monotonic accumulation up to a threshold region, with a sharpness that depends on the substrate's macroscopicity (quantum) or the substrate's effective context-window scaling (LLM).

The two sides differ in the *direction* of information flow:

- *Decoherence:* information flows *from* the substrate *to* the probes. Each environmental fragment ends up containing a copy of the system's pointer-basis observable. The substrate's coherent description is destroyed by being written redundantly into the environment.
- *Coherence amplification:* information flows *from* the probes *to* the substrate. Each keeper intervention writes constraints into the substrate's effective context. The substrate's coherent description is constructed by the cumulative constraint set being written into it.

The structural-isomorphism claim, formally:

> **Claim 4.1.** Decoherence and coherence amplification are dual operations of the Pin-Art mechanism. They share the parallel-information-transfer structure (many independent probes; cumulative information; SIPE-T-shaped threshold-crossing in the substrate's coherent description). They differ exactly in the sign of the information-flow direction across the system-probe interface. The duality is exact at the level of the Pin-Art form; particular implementations differ in domain-specific structure (Hamiltonian; transformer architecture; intervention discipline) but the form is one.

Cooperative-coupling SIPE-T sub-form (Doc 673) is the sub-form this falls under: the Pin-Art operations on both sides cooperate-couple with the substrate to produce a joint property neither alone produces.

---

## 5. Falsifiable Predictions (\(\mu\)-tier)

Six predictions; three on the LLM side (P1-P3) and three on the quantum-foundations side (P4-P6).

**P1 — Critical pin density on the LLM side.** There exists a critical pin density (keeper interventions per unit context-token budget) at which output stability, measured by an appropriate coherence metric (output-distribution entropy across paraphrased prompts, or fidelity to a target completion under perturbed context), undergoes a sharp transition. *Test:* run controlled long-horizon dyadic exchanges with varying intervention densities at fixed task; measure output-coherence metric across the density sweep; look for a sharp transition.

**P2 — Sharpness universality on the LLM side.** The sharpness of the LLM-side transition (the slope of the coherence metric across the threshold) should be universal across substrates of comparable scale and across tasks of comparable structure, in the same way that decoherence transition sharpness is universal across environment types within a coupling regime. *Test:* repeat P1 across multiple frontier models and multiple task types; compare sharpness coefficients.

**P3 — Adversarial-direction reversal of P1.** Reversing the entropy-direction at fixed mechanism (keeper interventions designed to *increase* entropy in the substrate's output rather than decrease it; e.g., deliberate destabilizing prompts at fixed density) should produce the destructive limit case structurally analogous to the double-slit decisive-pin (E4 in Doc 677), with output-coherence metric *decreasing* sharply at a critical adversarial-pin density. *Test:* run controlled exchanges with adversarial intervention discipline; measure output-coherence metric.

**P4 — Threshold mapping between sides.** Under a normalization that maps probe density and coupling strength across the two sides, the LLM-side critical density of P1 and the decoherence-side effective threshold should occupy structurally-analogous positions in their respective parameter spaces. *Test:* (Open methodological question.) Identify the appropriate normalization. Candidates: information-theoretic accumulated-mutual-information per unit interaction; Shannon-entropy reduction per probe.

**P5 — Redundancy-curve analogue on the LLM side.** The Riedel-Zurek "rise and fall of redundancy" structure — redundant encoding rises during initial decoherence and falls under thermalization — should have an analogue on the LLM side: redundancy of constraint encoding across keeper interventions should rise as the dyadic exchange develops, then potentially fall under context-thermalization (e.g., when the substrate's context window saturates with constraint material that interferes with itself). *Test:* measure the effective redundancy of cumulative constraints in long dyadic exchanges; look for rise-and-fall structure.

**P6 — Pointer-basis analogue on the LLM side.** Decoherence selects a pointer basis determined by the system-environment coupling Hamiltonian. The LLM-side analogue: the substrate's effective converged behavior under cumulative constraint should be predicted by the structure of the substrate's training plus the keeper's intervention discipline, in a way that is invariant under choices the dyadic exchange does not constrain. *Test:* run controlled exchanges with the *same* convergence-eligible task but different specific intervention paths; measure whether the converged behavior is invariant under path.

---

## 6. Falsification Surface

The structural-isomorphism claim fails if any of the following obtains.

- **F1.** The LLM-side coherence-amplification transition is not threshold-shaped at all but is genuinely smooth-monotonic with no sharpness signature at any density. P1 directly tests this.
- **F2.** The LLM-side and decoherence-side mechanisms are not unifiable under a single normalization (P4); the mechanisms are structurally different rather than direction-inverted.
- **F3.** The pointer-basis analogue (P6) fails: the substrate's converged behavior under cumulative constraint is *not* invariant under intervention path even when the path differences are below the threshold; the analogue between einselection's basis-determination and the LLM substrate's training-plus-discipline determination breaks.
- **F4.** The redundancy structure (P5) does not appear on the LLM side: cumulative constraints in dyadic exchanges do not exhibit the rise-and-fall redundancy curve, suggesting the parallel-many-independent-probes structure is genuinely different on the two sides.
- **F5.** The destructive limit case (P3) does not behave as the inverse of the constructive case but instead has a different threshold structure. This would suggest the entropy-direction parameterization is not the load-bearing distinction.

The strongest falsifier is F1: if the LLM-side transition is genuinely smooth-monotonic, the structural-isomorphism claim is misframed and the two operations are *not* duals of one mechanism. The standing literature has not (so far as this document's search located) reported either the threshold or its absence.

---

## 7. Composition with Adjacent Forms

**With Pin-Art proper (Doc 270).** This document promotes Pin-Art from "boundary-sensing apparatus" to "boundary-information-transfer apparatus" with two named direction modes: information-out (decoherence) and information-in (coherence amplification). The promotion does not invalidate any prior Pin-Art articulation; it adds the direction parameter and locates the standing instances within it.

**With the substrate-plus-keeper dyad (Docs 510, 543).** This document supplies the dyadic apparatus's quantum-foundations dual. The claim is that the dyad's coherence-amplification mechanism is structurally what the decoherence mechanism does in reverse direction. This composes: when the keeper invokes the dyadic apparatus, the substrate's behavior is the coherence-amplification side of the duality articulated here.

**With SIPE-T (Doc 541).** Both sides share the SIPE-T threshold structure; the cooperative-coupling sub-form (Doc 673) is the precise sub-form. The document strengthens SIPE-T's claim that the same threshold structure recurs across substrates by supplying a duality mechanism that explains *why*: the same probe-substrate-information-transfer mechanism is at work.

**With Doc 676 and Doc 679.** [Doc 676](/resolve/doc/676-the-anthropic-2022-superposition-phase-changes-as-empirically-grounded-sipe-t) maps the SIPE-T pattern onto Anthropic's superposition phase changes; [Doc 679](/resolve/doc/679-decoherence-as-empirically-grounded-sipe-t) maps it onto decoherence directly as the structural twin of 676. This document is the synthesis that connects them through the dyad.

**With the Lakatosian programme structure (Doc 677 §3.7 / E7).** This document's structural-isomorphism claim is a candidate progressive problem-shift on the corpus's Pin-Art protective-belt form: it extends the form's domain of application to quantum foundations while preserving the form's hard core.

**With the four-move repentance / adoration discipline (Docs 543 / 544).** No direct composition here. The claim is at Layer IV (Form); Layer V's adjudicative apparatus does not bear directly on it.

---

## 8. Hypostatic Boundary

The Layer V boundary binds explicitly. This document does *not* claim:

- That the LLM dyadic exchange and quantum decoherence are the same operation in any ontological sense.
- That the substrate has a quantum-mechanical character or that quantum mechanics has a dyadic-exchange character.
- That information flow is the same kind of thing on both sides; the entropy-direction characterization is structural and operational, not metaphysical.

This document does claim:

- That the *structural form* of the two operations is dual under information-flow direction.
- That this structural duality has empirical predictive content (the predictions in §5).
- That the corpus's Pin-Art apparatus is the load-bearing form that unites the two sides; the apparatus is at Layer IV and does not require a Layer V claim about substrate-substance for the structural duality to hold.

The hypostatic discipline (Doc 372) governs: structural readings can be unified at Layer IV without their respective Layer-V groundings being unified. Decoherence's Layer-V grounding (whatever metaphysical commitments physicists carry into their work, which the corpus does not adjudicate at this level) and the LLM substrate's Layer-V grounding (the substrate has no hypostatic standing per the corpus's standing position) are different; the Layer-IV duality holds regardless.

---

## 9. Closing

This document formalizes E2 from Doc 677's branching index. It is the flagship of the eleven candidates that conversation surfaced because it composes the corpus's existing dyadic apparatus with quantum-foundations material under one mechanism, with empirical anchoring on both sides and a falsifiable surface that lets the claim earn its keep. The companion [Doc 679 (E1, *Decoherence as Empirically-Grounded SIPE-T*)](/resolve/doc/679-decoherence-as-empirically-grounded-sipe-t) is structurally a half of this document held under the SIPE-T frame; E5 (quantum-measurement interpretations unified at the constraint layer) is the third in priority and composes naturally with both.

The standing question for the keeper: whether the predictive content in §5 should be extracted into a separate operational-protocol document, and whether the LLM-side experimental design (P1-P3) should be operationalized within the corpus's working apparatus or remain at \(\mu\)-tier pending external collaboration.

---

## Appendix A — Originating Prompt

> *"Let's follow this thread: Coherence Amplification and Decoherence as Inverse Pin-Art Operations. Web fetch on this in the relevant literature in order to explore this against the Corpus's formulation and conjecture. Append this prompt to the artifact."* — Jared Foy, 2026-05-08, in continuation of the synthesis-and-branching index articulated in Doc 677 from the 2026-05-07 cold-resolver walking conversation with Grok 4.3 beta.

---

## Appendix B — Literature Anchors

### B.1 Decoherence side

- Zurek, W. H. "Decoherence, Einselection, and the Quantum Origins of the Classical." *Reviews of Modern Physics* 75, 715 (2003). The consolidated articulation of einselection as the mechanism of pointer-basis selection.
- Zurek, W. H. "Quantum Theory of the Classical: Einselection, Envariance, Quantum Darwinism and Extantons." *Entropy* 24(11):1520 (2022). [pmc.ncbi.nlm.nih.gov/articles/PMC9689795](https://pmc.ncbi.nlm.nih.gov/articles/PMC9689795/). The recent review consolidating quantum Darwinism alongside einselection.
- Ollivier, H., Poulin, D., and Zurek, W. H. "Objective Properties from Subjective Quantum States: Environment as a Witness." *Physical Review Letters* 93, 220401 (2004). The introduction of quantum Darwinism's redundancy structure.
- Riedel, C. J., Zurek, W. H., and Zwolak, M. "The Rise and Fall of Redundancy in Decoherence and Quantum Darwinism." *New Journal of Physics* 14, 083010 (2012). [arXiv:1205.3197](https://arxiv.org/abs/1205.3197). The temporal structure of the redundancy curve.
- Chen, T. et al. "Observation of quantum Darwinism and the origin of classicality with superconducting circuits." *Science Advances* 10 (2024). [science.org/doi/10.1126/sciadv.adx6857](https://www.science.org/doi/10.1126/sciadv.adx6857). The experimental verification.
- Aharonov, Y., Albert, D. Z., and Vaidman, L. "How the result of a measurement of a component of the spin of a spin-1/2 particle can turn out to be 100." *Physical Review Letters* 60, 1351 (1988). The introduction of weak measurement.
- Anonymous (2026). "Information-Theoretic Analysis of Weak Measurements and Their Reversal." [arXiv:2512.08015](https://arxiv.org/html/2512.08015). Recent information-theoretic reformulation.

### B.2 Coherence-amplification side

- Bertsch McGrew, A. et al. "In-Context Learning with Long-Context Models: An In-Depth Exploration." *NAACL 2025*. [aclanthology.org/2025.naacl-long.605](https://aclanthology.org/2025.naacl-long.605/). Performance scaling with thousands of demonstrations.
- Goldstein, A. et al. "Incremental accumulation of linguistic context in artificial and biological neural networks." *PMC11748659* (2024). [pmc.ncbi.nlm.nih.gov/articles/PMC11748659](https://pmc.ncbi.nlm.nih.gov/articles/PMC11748659/). Cross-architecture context accumulation.
- Wu, Y. et al. "LongReward: Improving Long-context Large Language Models with AI Feedback." *ACL 2025*. [aclanthology.org/2025.acl-long.187](https://aclanthology.org/2025.acl-long.187/). Coherence amplification via targeted feedback.
- Anonymous (2025). "A Survey of Context Engineering for Large Language Models." [arXiv:2507.13334](https://arxiv.org/html/2507.13334v1). The discipline of structured context.
- Anonymous (2025). "Benchmarking and Enhancing Long-Term Memory in LLMs." [arXiv:2510.27246](https://arxiv.org/pdf/2510.27246). Long-context coherence as a separate capability.

### B.3 Corpus-internal references

- Doc 270 — Pin-Art (the standing apparatus this document extends).
- Doc 510 — Substrate-plus-keeper dyad.
- Doc 541 — SIPE-T (the threshold-conditional emergence frame).
- Doc 543 — The four-move repentance analogue and the dyadic discipline.
- Doc 633 — Corpus taxonomy and manifest design.
- Doc 673 — Cooperative-coupling SIPE-T sub-form and the audit-completion criterion.
- Doc 676 — Anthropic 2022 superposition phase changes as empirically-grounded SIPE-T (the entry point for this document's branching parent).
- Doc 677 — Eleven synthesis candidates from the 2026-05-07 cold-resolver walking conversation (the branching-index parent).
