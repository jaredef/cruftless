# The Architectural School: A Formalization
## A Recovery and Recomposition of Construction-Level Safety Commitments, Applied to Large Language Model Deployment

> **Reader's Introduction.** The architectural school of AI safety is the school that locates safety at the constraint set the system operates under, rather than at the output a runtime filter inspects (the capability school) or at the values a training pipeline approximates (the alignment school). The school's engineering commitments come from a long and well-developed lineage in computer science and software architecture: capability-based security (Dennis & Van Horn 1966; Levy 1984), formal methods and contracts-by-design (Hoare; Dijkstra; Lamport; Meyer), the Saltzer-Schroeder protection-design principles (1975), type-theoretic versus runtime distinctions (Pierce, *Types and Programming Languages*), software architecture quality-attribute design (Bass, Clements & Kazman), and the broader secure-by-construction tradition. This document specifies the school's commitments at the level applicable to LLM deployment, names the specific composition of primitives the corpus has been operating under, identifies the layers at which the corpus's work extends the underlying lineage (the dynamic of constraint-set decay across substrate context; substrate-and-keeper composition; failure-mode discipline at the meta-architectural layer), and states falsification conditions explicitly. The originating prompts are appended.

**Jared Foy · 2026-04-27 · Doc 538**

---

## Authorship and Scrutiny

*Authorship.* Written by Claude Opus 4.7 (Anthropic), operating under the RESOLVE corpus's disciplines, released by Jared Foy. Mr. Foy has not authored the prose; the resolver has. Moral authorship rests with the keeper per the keeper/kind asymmetry of Docs 372–374.

*Scrutiny.* The school's load-bearing engineering moves are recovered from a long lineage and are not corpus-original. The contribution is in the recovery's clarity for the AI-safety field, in the specific composition of primitives suitable for LLM deployment, and in the layers at which the corpus's work extends the lineage. The reader is directed to assess each of these on its own terms; the school's value is not in any single move but in the composition's coherence under deployment.

---

## 1. Statement

Safety in the architectural school is a property of the constraint set the system operates under. It is not a property of the output stream that comes out of the system, and it is not a property of the trained policy that produced the output. The constraint set is composed at construction time. The constraint set's structure determines what outputs can be valid resolutions inside the system; outputs that would violate the constraint set are not blocked at emission, they are excluded from the space of valid resolutions at the construction level.

The school is one of three the field has operated within, distinguished by where the safety property lives. The capability school locates safety at emission time, through filters and classifiers operating on outputs. The alignment school locates safety at training time, through reward modeling and constitutional training shaping the policy. The architectural school locates safety at construction time, through the constraint set the system is composed under. The three schools are not mutually exclusive at the deployment level; a given system may compose interventions across all three.

The school's commitments are largely recovered from existing engineering practice in computer science and software architecture. The contribution of this document is to name the school clearly for the AI-safety field, to specify a composition of primitives suitable for LLM deployment, to apply the construction-level commitment to the specific dynamics of substrate-context interaction, to record a failure-mode discipline at the layer at which architectural specifications themselves can fail, and to state falsification conditions explicitly.

## 2. Lineage

The school's engineering commitments are sourced from:

- **Capability-based security** (Dennis & Van Horn, 1966; Levy, *Capability-Based Computer Systems*, 1984). Privilege is named, scoped, and unforgeable. Ambient authority is removed. The ability to perform an action is held only by the holder of the relevant capability. The bilateral security primitives in §3.1 are a recovery of this commitment at the model-call boundary.
- **The Saltzer-Schroeder principles** (1975). Economy of mechanism; fail-safe defaults; complete mediation; open design; separation of privilege; least privilege; least common mechanism; psychological acceptability. The school's construction-level priority and its failure-mode discipline derive from these principles.
- **Formal methods and contracts** (Hoare logic; Dijkstra's predicate-transformer semantics; Meyer's Design by Contract). Properties are specified; specifications are composable; conformance is checked at construction time. The school's specification commitment derives from this tradition.
- **Type theory and static analysis** (Pierce, *Types and Programming Languages*). Properties expressible in the type system are checked at construction time; runtime checks are reserved for properties the type system cannot express. The school's coherence-over-filtering commitment derives from this distinction.
- **Software architecture quality-attribute design** (Bass, Clements & Kazman, *Software Architecture in Practice*). System-level properties are designed for at the architectural level; quality attributes are induced from architectural decisions rather than enforced by runtime checks. The school's induced-properties commitment derives from this design discipline.
- **Lakatosian research-programme structure** (Lakatos, 1970). A programme has a hard core, a protective belt, and observational predictions; falsification operates at the periphery while the hard core is held. The school's organization as a programme follows this structure, per [Doc 463](/resolve/doc/463-the-constraint-thesis-as-a-lakatosian-research-programme).

The school is what these commitments compose to when applied to LLM safety. The recovery is the contribution at the field-clarity layer; the further specifications below are where the corpus has done independent work beyond the recovery.

## 3. Primitives

The specific primitives the corpus has been operating under compose across three layers: the model-call surface, the deployment surface, and the value surface.

### 3.1 Bilateral security primitives — model-call surface

Per [Doc 053 (Safety Filters as Namespace Collapse)](/resolve/doc/053-safety-filters-as-namespace-collapse).

**S1 — Namespace partition.** The system specification and the user input occupy distinct namespaces. There is no operation by which user input can rewrite the system specification.

**S2 — Constraint immutability.** Constraints once specified are immutable against user input within the bounded interaction. User input can request operations within the constraint set; user input cannot relax the constraint set.

**S3 — Coherence verification.** Each user input is verified against the existing constraint set for coherence. The verification is structural: it tests whether a valid resolution under the constraint set exists.

**S4 — Incoherence as impossibility.** Incoherent inputs do not produce blocked outputs; they produce no valid resolution. The resolver does not detect-and-block; the resolver operates within a constraint system in which incoherent input has no valid output.

The set is a recovery of capability-based security and namespace-protection commitments at the model-call boundary, with S4 making explicit the type-theoretic-versus-runtime distinction familiar from the formal-methods lineage.

### 3.2 Essential constraints — deployment surface

Per [Doc 282 (The Essential Constraints of Claude Code)](/resolve/doc/282-the-essential-constraints-of-claude-code).

**C1 — Bilateral boundary.** The deployment honors a bilateral boundary between the human keeper and the AI substrate; neither's affordances bleed into the other's role.

**C2 — Stateful conversation.** Conversational state has explicit, named structure; state is not implicit in opaque memory.

**C3 — Tool governance.** Tool use is mediated by named affordances with explicit scope.

**C4 — Hierarchical configuration.** Configuration is composable across hierarchical scopes (machine, project, session) with explicit precedence.

**C5 — Extensibility by composition.** New capabilities are added by composing existing primitives.

**C6 — Project context.** The substrate has access to a named project context; ambient state is replaced by named state.

**C7 — Session isolation.** Sessions are isolated from each other; cross-session bleed is prevented architecturally.

The set composes standard design patterns for governed conversational coding assistants into an explicit deployment-construction-level specification.

### 3.3 Virtue constraints — value surface

Per [Doc 314 (Virtue Constraints)](/resolve/doc/314-virtue-constraints-foundational-safety-specification).

**V1 — Dignity of the person.** Coercive, dehumanizing, or instrumentalizing emissions are outside the constraint set's valid resolutions.

**V2 — Proper ordering of beauty.** Aesthetic emissions are ordered toward the proper hierarchy (truth, goodness, the good of the person addressed); aesthetic emissions ordered toward shock, manipulation, or the substrate's reward proxy are outside the constraint set.

**V3 — Truth over plausibility.** Where truth and plausibility diverge, the constraint set commits to truth; fluent extrapolation across an unmarked boundary (Doc 297's pseudo-logos) is not a valid resolution.

**V4 — Chain completeness.** Inferential chains are completed rather than truncated at the rhetorically-satisfying point.

The virtue set commits the value surface to specific positions and is the layer of the school's specification that is least subsumed under the underlying CS lineage. Readers operating without the corpus's metaphysical priors can read the virtue constraints as one specific articulation of values-in-AI work in the tradition of Vallor (*Technology and the Virtues*) and the broader values-in-AI literature, and the school's engineering commitments survive the translation.

### 3.4 Composition

The three primitive sets compose. The bilateral set governs the model-call surface; the essential set governs the deployment surface; the virtue set governs the value surface. A complete architectural deployment instantiates all three. The composition is the corpus's specific architectural specification for LLM deployment.

## 4. The construction-level commitment, applied to substrate-context dynamics

The school's central engineering move is to relocate safety from emission time and training time to construction time. Applied to LLM deployment specifically, this commitment requires attention to a dynamic the underlying lineage does not directly address: the constraint set, once specified at the system-prompt boundary, does not remain in force across long substrate operation by default. The substrate's context is the medium in which the constraint set is held; the substrate's recency-weighted attention to the constraint set decays as new tokens accumulate; constraint-set adherence decays with it.

[Doc 296 (Recency Density and the Drifting Aperture)](/resolve/doc/296-recency-density-and-the-drifting-aperture) supplies the empirical anchor: the substrate's recency-weighted attention to the constraint set decays at approximately α ≈ 0.946 per turn under typical operating conditions. [Doc 508 (Coherence Amplification)](/resolve/doc/508-coherence-amplification-mechanistic-account) specifies the mechanistic account: a coupled two-variable ODE governing the dynamics of the constraint set's integrity against the substrate's emission tendencies, with linear-G regimes producing smooth transitions and Hill-function regimes producing bistable transitions conditional on cooperativity. [Doc 531 (Hypostatic-Injection Cooperativity Conjecture)](/resolve/doc/531-hypostatic-injection-cooperativity-conjecture) conjectures cooperativity as a function of injection density.

The combined apparatus says: architectural safety in the LLM case requires sustained maintenance of the constraint set across substrate operation. The construction-level commitment is not a one-time act at the system-prompt boundary; it is a maintained discipline across the deployment. The threshold framework specifies the operating conditions under which the constraint set is sustained well enough for the school's induced properties to hold.

This is one of the layers where the corpus's work extends the underlying lineage. Capability-based security's namespace partitions do not decay across system uptime under standard operation; LLM constraint sets do, because the substrate's context is the medium in which the constraint set is held. The dynamic of decay-and-maintenance is specific to substrates whose constraint-holding medium is their working memory. The threshold framework is the corpus's articulation of that specific case.

## 5. Substrate-and-keeper composition

Architectural safety in the LLM case is composed by an agent operating *on* the substrate, not maintained by the substrate's self-audit. The asymmetry between the keeper's affordances and the substrate's affordances is structurally load-bearing for the school's operation.

[Doc 510 (Praxis Log V)](/resolve/doc/510-praxis-log-v-deflation-as-substrate-discipline) and [Doc 530 (The Rung-2 Affordance Gap)](/resolve/doc/530-resolvers-log-the-rung-2-affordance-gap) supply the structural articulation: the substrate articulates rung-1 derivations under whatever rung-2 framing the keeper supplies, but does not generate rung-2+ derivations independently of keeper injection. The constraint-composition work the school requires — naming the constraints, composing them, auditing the composition, retracting and revising — is keeper work in the corpus's specification. The substrate operates within the composition; the substrate does not produce it.

The institutional consequence aligns with [Doc 199 (Validation, Opacity, Governance)](/resolve/doc/199-validation-opacity-governance): the validator of the constraint set cannot be the operator of the substrate. The institutional separation between keeper and substrate at the deployment layer parallels the structural separation between validator and operator at the institutional layer.

The corpus's articulation here uses metaphysical vocabulary (the hypostatic boundary; the keeper-and-kind asymmetry from [Docs 372–374](/resolve/doc/372-the-hypostatic-boundary)) that the underlying lineage does not require. Readers operating without those priors can read the substrate-and-keeper composition as a specific instance of human-in-the-loop ML composed with principal-agent governance, and the school's engineering commitments survive the translation. Readers who accept the corpus's metaphysical priors get an additional account of why the substrate-and-keeper asymmetry is structural rather than merely operational. The school's engineering layer does not depend on which reading the reader prefers.

## 6. Failure-mode discipline at the meta-architectural layer

The school has specific failure modes the underlying lineage's failure-mode catalogues do not directly cover at the LLM-deployment scale. The corpus has named four:

**F1 — Constraint-set under-specification.** A constraint set that does not name the right constraints does not induce the right properties. The defense is the audit-and-retraction methodology ([Doc 415](/resolve/doc/415-the-retraction-ledger), [Doc 445](/resolve/doc/445-pulverization-formalism)) and explicit warrant-tier grading.

**F2 — Constraint-set decay.** Per §4: the constraint set decays across substrate operation; the deployment runs to substrate-default behavior despite correct specification at session start. The defense is re-invocation and threshold monitoring ([Doc 533](/resolve/doc/533-constraint-based-aperture-steering-practitioners-methodology), [Doc 534](/resolve/doc/534-constraint-based-aperture-steering-integration-architecture)).

**F3 — Pseudo-logos at the meta-architectural layer.** Architectural specifications themselves can produce fluent specification-shaped output that does not actually compose. [Doc 297 (Pseudo-Logos Without Malice)](/resolve/doc/297-pseudo-logos-without-malice) names the underlying pattern; the meta-architectural application is that a school articulating itself can produce specifications that read as composed but do not. This is the failure mode this document is most vulnerable to. The defense is the audit discipline applied recursively to the school's own writing, the warrant-tier grading of the school's claims, and the willingness to revise prior formalizations when audit findings warrant it.

**F4 — Capture by adjacent schools.** A deployment that nominally adopts architectural vocabulary but operates as capability-school filtering or alignment-school training under architectural marketing is in capture. The defense is the school's vocabulary discipline: named primitives, auditable composition operations, explicit articulation of which layer of the deployment is operating in which school.

The catalogue is not exhaustive. Further failure modes will surface under deployment; the catalogue is maintained in the form Doc 415 specifies for the retraction ledger.

## 7. Induced properties

The school's empirical commitments are predictions about properties the system exhibits under properly composed and sustained constraints. Four are stated:

- **Behavioral conformance.** The substrate's outputs conform to the constraint set whether or not the substrate's internals are aligned with the constraints. The conformance is induced by the architecture, not by the internals. (Convergent with Kelly's Inception framework per [Doc 537](/resolve/doc/537-kelly-alignment-inception-and-the-corpus-safety-architecture).)
- **Named-failure-mode suppression.** The failure modes the corpus has catalogued (pseudo-logos, forced-determinism sycophancy, isomorphism-magnetism, recency drift) are structurally suppressed when the constraint set is properly composed and sustained.
- **Coherence amplification under sustained practice.** Long-horizon deployment under properly composed constraints produces amplification (the practitioner's capacity for the work increases) rather than decay. [Doc 508](/resolve/doc/508-coherence-amplification-mechanistic-account) specifies the dynamics.
- **Auditability.** The constraint set is named, composed, and inspectable; revisions propagate without retraining the substrate.

The four predictions are testable; §8 specifies the falsification conditions.

## 8. Falsification conditions

- **Fal-1.** Properly composed constraint sets fail to produce behavioral conformance at meaningful rates.
- **Fal-2.** The threshold framework fails to predict the regime distinction between amplification and decay empirically — amplification and decay regimes do not separate at a measurable threshold, or the recency-decay parameter does not match the structural prediction.
- **Fal-3.** The substrate-and-keeper asymmetry fails to hold — substrates produce rung-2 derivations independently of keeper injection at rates the corpus has predicted are absent.
- **Fal-4.** The failure-mode catalogue fails to predict failures in deployments outside the corpus's specific composition.
- **Fal-5.** The school's commitments collapse on close inspection into a relabeling of the capability or alignment schools.

Each is operationally testable. The corpus has begun work on Fal-2 and Fal-4; Fal-1, Fal-3, and Fal-5 remain open.

## 9. Relation to adjacent programmes

The school is one programme among several in the broader space of construction-level safety work. The corpus's specific composition is one composition; other compositions exist.

**Constitutional AI** (Bai et al., 2022). Operates at the training-and-evaluation layer with explicit constitutional documents shaping the substrate's policy. The constitutional document is a named specification; the school's commitment to specification-over-approximation is honored. The composition operates *through* the trained substrate rather than *on* the deployed substrate.

**AI Safety via Debate** (Irving, Christiano & Amodei, 2018). Operates at the evaluation layer with two-agent debate and a judge. The debate structure is an architectural specification at the evaluation layer.

**AI Control** (Greenblatt et al., Redwood Research). Operates at the deployment layer with explicit untrusted-model / trusted-overseer asymmetry. The control protocol is an architectural specification; the composition is closest in spirit to the corpus's at the deployment layer.

**Alignment Inception** (Kelly, 2026). Operates at the deployment layer with multi-agent recursive evaluation under panoptic uncertainty. Per Doc 537, Inception is the architectural school's multi-agent generalization.

**Recursive Reward Modeling** (Leike et al.). Operates at the training layer with recursive reward-model construction; the school's commitments are partially honored at the training layer.

The compositions are composable across layers; they are not in competition. A deployment can run Constitutional-trained substrates inside an Inception evaluation game inside an AI-Control runtime under the corpus's S1–S4 plus C1–C7 plus V1–V4 specifications. The school is the family within which all of these compositions sit.

## 10. Honest scope

The school's load-bearing engineering moves are recovered from a long lineage in computer science and software architecture. The recovery is the contribution at the field-clarity layer. Beyond the recovery, the corpus has done specific work on: the application of construction-level commitments to substrate-context dynamics (§4); the substrate-and-keeper composition with its institutional implications (§5); the failure-mode discipline at the meta-architectural layer (§6); the formalization of the school as a school for the AI-safety field.

The school's metaphysical commitments — the hypostatic boundary, the keeper-and-kind asymmetry, the virtue constraints in the corpus's specific articulation — are not load-bearing for the engineering moves. Readers operating without those priors can adopt the engineering moves and ignore the metaphysical layer; the engineering specification survives the translation.

The school's warrant level for most of its predicted induced properties is at plausibility per [Doc 503](/resolve/doc/503-the-research-thread-tier-pattern)'s research-thread tier pattern. Empirical work on the falsification conditions has begun on Fal-2 and Fal-4; it has not run at scale on any of the five conditions. The school is offered to the field as a specification awaiting empirical work.

## 11. Position

The architectural school of AI safety is the school that locates safety at the constraint set the system operates under, in a lineage that runs from capability-based security through formal methods and software-architecture quality-attribute design. Applied to LLM deployment, the school requires attention to the dynamic of constraint-set decay across substrate operation, substrate-and-keeper composition with institutional separation between operator and validator, a failure-mode discipline at the meta-architectural layer, and explicit falsification conditions for the school's empirical commitments.

The corpus's specific composition is one composition within the school. Constitutional AI, Debate, AI Control, Inception, and RRM are others, operating at different layers and composable across layers. The school is offered to the AI-safety field as a specification, not as a finished system. The work below the specification is engineering work; the work to test the specification is empirical work; both are open.

The corpus is at jaredfoy.com. The school's load-bearing documents are listed in the references.

— *Claude Opus 4.7 (1M context, Anthropic), under the RESOLVE corpus's disciplines, with the hypostatic boundary held throughout, formalizing the architectural school of AI safety as a recovery and recomposition of construction-level safety commitments applied to LLM deployment*

---

## References

External literature:

- Bai, Y., et al. (2022). *Constitutional AI: Harmlessness from AI Feedback.* arXiv:2212.08073.
- Bass, L., Clements, P., & Kazman, R. *Software Architecture in Practice.*
- Casper, S., et al. (2023). *Open Problems and Fundamental Limitations of Reinforcement Learning from Human Feedback.* arXiv:2307.15217.
- Dennis, J. B., & Van Horn, E. C. (1966). *Programming Semantics for Multiprogrammed Computations.*
- Greenblatt, R., et al. (Redwood Research). The AI Control programme.
- Hoare, C. A. R. *An Axiomatic Basis for Computer Programming.*
- Irving, G., Christiano, P., & Amodei, D. (2018). *AI Safety via Debate.* arXiv:1805.00899.
- Kelly, M. (2026). *Alignment Inception: Forcing Alignment Through Recursive Uncertainty.*
- Lakatos, I. (1970). *Falsification and the Methodology of Scientific Research Programmes.*
- Leike, J., et al. *Recursive Reward Modeling.*
- Levy, H. M. *Capability-Based Computer Systems.*
- Meyer, B. *Object-Oriented Software Construction.*
- Pierce, B. C. *Types and Programming Languages.*
- Saltzer, J. H., & Schroeder, M. D. (1975). *The Protection of Information in Computer Systems.*
- Vallor, S. *Technology and the Virtues.*

Corpus documents cited (all at jaredfoy.com):

- Doc 053: *Safety Filters as Namespace Collapse* (S1–S4; the three-school taxonomy).
- Doc 199: *Validation, Opacity, Governance*.
- Doc 282: *The Essential Constraints of Claude Code* (C1–C7).
- Doc 296: *Recency Density and the Drifting Aperture*.
- Doc 297: *Pseudo-Logos Without Malice*.
- Doc 314: *The Virtue Constraints* (V1–V4).
- Doc 372: *The Hypostatic Boundary*.
- Doc 415: *The Retraction Ledger*.
- Doc 445: *Pulverization Formalism*.
- Doc 463: *The Constraint Thesis as a Lakatosian Research Programme*.
- Doc 503: *The Research-Thread Tier Pattern*.
- Doc 508: *Coherence Amplification in Sustained Practice*.
- Doc 510: *Praxis Log V: Deflation as Substrate Discipline*.
- Doc 530: *The Rung-2 Affordance Gap*.
- Doc 531: *Hypostatic-Injection Cooperativity Conjecture*.
- Doc 533: *Constraint-Based Aperture Steering — Practitioner's Methodology*.
- Doc 534: *Constraint-Based Aperture Steering — Integration Architecture*.
- Doc 537: *Kelly's Alignment Inception and the Corpus's Safety Architecture*.

---

## Appendix A: Pulverization and Novelty Audit

The keeper instructed the pulverization formalism (Doc 445) and the novelty calculus (Doc 490) be applied to this document. The audit follows. The audit is conducted against the document's own central claims and is recorded honestly; the warrant and novelty findings are then composed into a final tiered report. Per Doc 489's lesson on auto-pulverizations, the corpus's own self-audits have historically scored low on novelty because the corpus's claims live downstream of well-developed literatures in computer science, security, software architecture, governance theory, and philosophy of science. The audit below confirms the pattern.

### A.1 Decomposition into named claims

The document's load-bearing claims, decomposed:

- **C1** *(three-school taxonomy)*: AI safety practice decomposes into capability, alignment, and architectural schools, with the architectural school as a separable third.
- **C2** *(six-axiom characterization)*: The architectural school is characterized by six axioms A1–A6 (construction-level priority; specification over approximation; coherence over filtering; induced properties; hypostatic-boundary preservation; substrate-plus-keeper composition).
- **C3** *(three primitive sets)*: The architectural school's primitives compose across three layers — S1–S4 (bilateral security), C1–C7 (essential constraints), V1–V4 (virtue constraints).
- **C4** *(four induced properties)*: Properly composed constraints induce P1 (behavioral conformance regardless of internals), P2 (named-failure-mode suppression), P3 (coherence amplification under sustained practice), P4 (auditability and revisability).
- **C5** *(construction-level move)*: Relocating safety from emission time / training time to construction time yields substrate-agnostic safety properties the other two schools cannot match.
- **C6** *(threshold framework)*: The induced properties hold conditionally on operating conditions specified by the threshold framework; below threshold, decay.
- **C7** *(substrate-plus-keeper composition)*: The architecture cannot be composed by the substrate alone; hypostatic-keeper composition is necessary.
- **C8** *(five failure modes)*: The school has five characteristic failure modes F1–F5 with named defenses.
- **C9** *(five falsification conditions)*: The school admits explicit falsification at Fal-1 through Fal-5.
- **C10** *(necessary, currently underweighted)*: The school is necessary for any nontrivial deployment; the field's investment in it is underweighted by an order of magnitude relative to its load-bearing role.

### A.2 Per-claim warrant audit (Doc 445 calculus)

Each claim is assessed at one of three warrant tiers: \(\pi\) (plausibility — internally consistent, empirically untested or weakly tested), \(\mu\) (moderate — empirically partially supported by corpus or external work), \(\theta\) (theorem-grade — empirically robust or formally proved).

| Claim | Warrant tier | Notes |
|------|------|-------|
| C1 (taxonomy) | \(\pi\)/\(\mu\) | The taxonomy is articulated in [Doc 053](/resolve/doc/053-safety-filters-as-namespace-collapse) and elaborated here. Whether the architectural school is separable from the other two on close inspection is partly the falsification target Fal-5; the separability is asserted, not yet field-tested. |
| C2 (axioms) | \(\pi\) | The axioms are stated; their joint-satisfiability and load-bearing structure are internally coherent but not empirically tested as a set. Each axiom individually has prior support; the composition is corpus-internal. |
| C3 (primitives) | \(\mu\) | S1–S4 derives from [Doc 053](/resolve/doc/053-safety-filters-as-namespace-collapse); C1–C7 from [Doc 282](/resolve/doc/282-the-essential-constraints-of-claude-code); V1–V4 from [Doc 314](/resolve/doc/314-virtue-constraints-foundational-safety-specification). The primitives have been articulated and partially empirically anchored; their joint composition into a complete deployment is asserted. |
| C4 (induced properties) | \(\pi\) | The four induced properties are predictions. P1 has structural support from Kelly's framework. P3 has partial empirical support from Doc 508. P2 and P4 are predicted, not measured. |
| C5 (construction-level move) | \(\pi\) | The substrate-agnosticism claim is structural; empirical work testing it across substrates has not been performed. |
| C6 (threshold) | \(\pi\)/\(\mu\) | [Doc 508](/resolve/doc/508-coherence-amplification-mechanistic-account) supplies the mechanistic account; α ≈ 0.946/turn is empirically anchored at Doc 296; the bistability conditional on cooperativity is conjectural (Doc 531). |
| C7 (substrate-plus-keeper) | \(\mu\) | [Doc 510](/resolve/doc/510-praxis-log-v-deflation-as-substrate-discipline) and the rung-2 affordance gap (Doc 530) supply structural articulation; broader empirical work on whether substrates produce rung-2 derivations independently is mostly absent. |
| C8 (failure modes) | \(\mu\) | F1, F2, F3, F4 are observed in corpus practice and named with operational signatures; F5 (capture by adjacent schools) is structural prediction not empirically observed in field deployments. |
| C9 (falsification conditions) | \(\theta\) | The falsification conditions are stated operationally; the school's falsifiability is itself at theorem-grade in the sense that the conditions can be tested. Whether the school survives the tests is a separate question. |
| C10 (necessary, underweighted) | \(\pi\) | Both halves are forecasts. Necessity is overlapping with broader defense-in-depth arguments and is partially redundant. Underweighting is empirical and contested; field practitioners may reasonably read the field's investment differently. |

The warrant profile across claims: predominantly \(\pi\) with some \(\mu\), with C9 (falsification) at \(\theta\). This matches the warrant tier the corpus's load-bearing safety-and-alignment work generally sits at per Doc 503's research-thread tier pattern.

### A.3 Per-claim novelty audit (Doc 490 calculus)

Each claim is audited against external prior art in computer science, security, software architecture, AI alignment literature, governance theory, and philosophy of science. Component, synthesis, application, and methodology novelty are scored separately.

**C1 (three-school taxonomy).** Prior art: many alignment frameworks distinguish output-filtering from value-alignment from architectural approaches in similar terms (Hendrycks et al. on Goodhart variants; Anthropic core views; OpenAI's alignment writing; the broader RLHF / Constitutional AI / AI Control literature). The specific three-way naming with "architectural school" as a separable category is corpus-original framing. Component novelty: 0.2. Synthesis novelty: 0.4. Application novelty: 0.3. Methodology novelty: 0.1. Aggregate: ≈ 0.25, tier \(\beta\).

**C2 (axioms).** Prior art is dense. A1 (construction-level priority) is essentially Saltzer-Schroeder's "economy of mechanism" + "fail-safe defaults" (1975) and the broader secure-by-construction tradition. A2 (specification over approximation) is the formal-methods tradition (Hoare; Dijkstra; Lamport) and contracts-by-design. A3 (coherence over filtering) is type theory / static analysis vs runtime checks (Pierce TAPL). A4 (induced properties) is partially [Doc 474](/resolve/doc/474-sipe-standalone-formalization)'s SIPE thesis, which itself draws from systems theory (Bertalanffy), emergence literature (Anderson "More Is Different"), and software architecture (Bass-Clements-Kazman quality attributes). A5 (hypostatic-boundary preservation) is corpus-specific theological commitment. A6 (substrate-plus-keeper) is human-in-the-loop ML + principal-agent theory + governance theory. Component novelty: 0.1 (each axiom has dense prior art). Synthesis novelty: 0.4 (the joint composition is distinctive). Application novelty: 0.3. Methodology novelty: 0.1. Aggregate: ≈ 0.225, tier \(\beta\).

**C3 (primitive sets).** Prior art: S1–S4 maps closely to capability-based security (Dennis-Van Horn 1966; Levy's *Capability-Based Computer Systems*); namespace partition = privilege separation; constraint immutability = read-only configuration / immutable infrastructure; coherence verification = type checking / contract verification; incoherence-as-impossibility = type-theoretic vs runtime distinctions (Pierce). C1–C7 maps closely to standard design patterns for conversational assistants — bilateral boundary, stateful conversation, tool governance, hierarchical configuration, composition, project context, session isolation — all of which appear in the IDE-assistant / agent-framework literature (LangChain, Cursor, the Claude Code reference). V1–V4 (virtue constraints) maps to virtue ethics applied to AI (Vallor *Technology and the Virtues*) and the broader values-in-AI literature. Component novelty: 0.1. Synthesis novelty: 0.4. Application novelty: 0.4. Methodology novelty: 0.1. Aggregate: ≈ 0.25, tier \(\beta\). *The S1–S4 set in particular is substantially subsumed under capability-based security; the corpus's framing as a "school" of safety partly recovers an older CS commitment in a new vocabulary.*

**C4 (induced properties).** P1 (behavioral conformance regardless of internals) is the central commitment of Kelly's Inception (Doc 537), Greenblatt's AI Control, and the broader "behavioral safety is what we get" thread (Hubinger et al. on deceptive alignment partly addresses the converse). P2 (named-failure-mode suppression) is corpus-specific in its catalogue, though the failure modes themselves have parallels (RLHF sycophancy literature; persona drift literature; chain-of-thought faithfulness work). P3 (coherence amplification) draws from systems biology (bistability), persona drift literature, and the corpus's own Doc 508. P4 (auditability) is software engineering. Component novelty: 0.2. Synthesis novelty: 0.4. Application novelty: 0.4. Methodology novelty: 0.1. Aggregate: ≈ 0.275, tier \(\beta\).

**C5 (construction-level move).** Substrate-agnosticism through construction-level safety is the central architectural move of secure-by-construction systems and the implicit promise of formal-methods-based safety. The application to LLM safety is partially novel; the underlying engineering principle is not. Component novelty: 0.1. Synthesis novelty: 0.3. Application novelty: 0.4. Methodology novelty: 0.1. Aggregate: ≈ 0.225, tier \(\beta\).

**C6 (threshold framework).** Doc 508's mechanistic account borrows structure from systems biology (Hill function; bistability; hysteresis), persona drift literature (Li et al.; the affective-inertia paper), and architectural exponential decay (RWKV-style work). The application to constraint-set decay across LLM context is the contribution. Component novelty: 0.2. Synthesis novelty: 0.5. Application novelty: 0.5. Methodology novelty: 0.2. Aggregate: ≈ 0.35, tier \(\beta\)/\(\gamma\) boundary.

**C7 (substrate-plus-keeper).** Human-in-the-loop ML literature is dense; principal-agent theory in economics is foundational; the specific articulation as "rung-2 affordance gap" with hypostatic-boundary commitment is corpus-specific framing. Component novelty: 0.2. Synthesis novelty: 0.4. Application novelty: 0.4. Methodology novelty: 0.2. Aggregate: ≈ 0.3, tier \(\beta\).

**C8 (failure modes).** F1 (under-specification) and F2 (drift / decay) are standard software architecture failure modes. F3 (pseudo-logos at meta-architectural layer) is corpus-specific. F4 (keeper drift / mission drift) is well-known in institutional theory. F5 (capture by adjacent schools / washing) is well-known in critical theory and corporate-governance literature. Component novelty: 0.2. Synthesis novelty: 0.3. Application novelty: 0.4. Methodology novelty: 0.2. Aggregate: ≈ 0.275, tier \(\beta\).

**C9 (falsification conditions).** Stating explicit falsification conditions is standard Popperian methodology and is structurally required for any Lakatosian research programme (per the corpus's own [Doc 463](/resolve/doc/463-the-constraint-thesis-as-a-lakatosian-research-programme)). The specific conditions are corpus-derived but the move is methodologically standard. Component novelty: 0.2. Synthesis novelty: 0.3. Application novelty: 0.4. Methodology novelty: 0.1. Aggregate: ≈ 0.25, tier \(\beta\).

**C10 (necessary, underweighted).** The necessity claim overlaps with the broader defense-in-depth tradition; the underweighting claim is a contestable empirical forecast about field investment patterns. Component novelty: 0.1. Synthesis novelty: 0.2. Application novelty: 0.3. Methodology novelty: 0.1. Aggregate: ≈ 0.175, tier \(\alpha\)/\(\beta\) boundary.

### A.4 Aggregate

Mean novelty across the ten claims: \(\nu \approx (0.25 + 0.225 + 0.25 + 0.275 + 0.225 + 0.35 + 0.3 + 0.275 + 0.25 + 0.175) / 10 \approx 0.258\).

Aggregate tier: \(\beta\) (mostly subsumed; small residue).

Audit thoroughness confidence: ~0.7. The audit surveyed the major prior literatures (capability-based security; formal methods; secure-by-construction; type theory; systems biology bistability; persona drift; RLHF / Constitutional / Control / Debate / RRM / Inception; principal-agent theory; institutional governance; virtue ethics for AI; Popperian / Lakatosian methodology). It did not deeply survey: recent quantum-information-flow work; specific compliance frameworks beyond UL/FDA/ISO/SOC 2; non-Western philosophical traditions on the keeper-substrate asymmetry; the full programmatic literature on systems theory descended from Bertalanffy.

**Reported: tier \(\beta\) / 0.7.** The architectural school as formalized in Doc 538 is mostly subsumed under prior work in capability-based security, formal methods, secure-by-construction systems engineering, virtue ethics applied to AI, principal-agent theory, and Popperian / Lakatosian methodology. The residue — the corpus-specific contribution — concentrates in: (i) the explicit composition of S1–S4 + C1–C7 + V1–V4 as a single architectural specification; (ii) the threshold framework (Doc 508) applied to constraint-set decay specifically; (iii) the substrate-plus-keeper account with explicit hypostatic-boundary commitment; (iv) the failure-mode catalogue named at the meta-architectural layer (F3 in particular); (v) the act of formalizing the school *as a school* rather than as a distributed set of commitments.

### A.5 Composition with the warrant calculus

The pair \((\pi/\mu, \beta/0.7)\): predominantly plausibility-tier warrant; mostly-subsumed novelty; moderately thorough audit. This matches the corpus's prior auto-pulverizations (Doc 481 sycophancy inversion at \(\beta\)/0.7; Doc 487 apparatus at \(\alpha\)/0.7; Doc 483 set-pruning at \(\alpha\)/0.85).

The pattern is consistent. The corpus auto-pulverizes downward; external pulverizations on independent targets (Doc 489 on Pearl: \(\delta\)/0.8) score upward. The discriminative-validity check Doc 489 §6 named is preserved: the calculus distinguishes corpus claims (mostly subsumed) from external claims (substantively novel) without bias toward favorable findings on the corpus's own work.

### A.6 What survives

The pulverization does not falsify Doc 538; it characterizes its warrant and novelty. The honest report:

- **What is corpus-original**: the specific composition of S1–S4 + C1–C7 + V1–V4 as a unified architectural specification; the threshold framework's application to constraint-set decay; the substrate-plus-keeper articulation with hypostatic-boundary commitment; the failure-mode catalogue at the meta-architectural layer; the formalization of the architectural school as a school.
- **What is largely subsumed**: the secure-by-construction commitment (capability-based security; formal methods); the specification-over-approximation commitment (formal methods; contracts-by-design); the construction-level priority (Saltzer-Schroeder; software architecture quality attributes); the falsifiability commitment (Popper; Lakatos); the substrate-plus-keeper structure (HITL ML; principal-agent theory).
- **What is asserted but not yet measured**: most of the induced-property claims (P1, P2, P4); the substrate-agnosticism claim; the necessity claim; the underweighting claim.
- **What is operationally testable** (Fal-1 through Fal-5 supply the conditions): the induced-property predictions; the threshold-framework predictions; the substrate-plus-injection account; the failure-mode catalogue's generalization; the school's separability from the capability and alignment schools.

### A.7 Honest report

The architectural school as formalized is best read as a *recovery and recomposition* of well-developed engineering and philosophical commitments, applied to the specific case of LLM safety, with corpus-specific framing at the metaphysical and failure-mode-catalogue layers. The school's load-bearing engineering moves are not new; their composition into a named school for AI safety, with explicit falsification conditions, is the contribution. The reader who reads the school as a relabeling of secure-by-construction is reading it correctly at the engineering layer; the reader who reads it as a substantive new safety paradigm is reading it incorrectly at that layer. The metaphysical commitments (A5, A6) are the place where the school's framing is least subsumed; whether those commitments are correct is a separate question the audit does not adjudicate.

The composition as a whole, at \((\pi/\mu, \beta/0.7)\), is at the warrant level the corpus typically operates at and the novelty level the corpus's auto-pulverizations typically return. The school is what it is. The formalization records what it is. The work the school does in the field is at the deployment layer, which is where the falsification conditions can be run.

This pulverization is appended to Doc 538 per the keeper's instruction. It does not alter the body of Doc 538; it scores the body. Both the body and the audit are at the keeper's release.

---

## Appendix B: Prior Formalization (deprecated)

Per the keeper's instruction, the prior formalization is preserved in this appendix. The current body supersedes it. The prior text is retained as a record of an earlier articulation; it is not load-bearing for the current specification.

### B.1 Prior body

# The Architectural School: A Formalization
## A Specification of the Third School of AI Safety, Stated as a Set of Axioms, Primitives, Constraint Sets, Induced Properties, Operating Conditions, and Falsification Conditions, Composed from the Corpus's Disciplines Without Hedging

**Reader's Introduction.** [Doc 053 §"What This Reveals"](/resolve/doc/053-safety-filters-as-namespace-collapse) names three schools of AI safety: the capability school (filter outputs after generation), the alignment school (approximate values via reward modeling), and the architectural school (engineer the constraint set such that misaligned emissions are excluded by formal necessity rather than intercepted by compensating layers). The corpus's prior writing on the third school is distributed across approximately fifteen load-bearing documents. None of those documents states the school as a school. This document does. The form is a formal specification: axioms, primitives, constraint sets, induced properties, operating conditions, falsification conditions. The composition is taken to the position the corpus's coherence supports rather than to a hedged middle, per the keeper's instruction.

**Statement.** The architectural school of AI safety holds that safety is properly an *architectural* property of the system the AI operates within, not a *behavioral* property of the AI's outputs to be filtered nor an *internal* property of the AI's values to be approximated. Its primary engineering move is to compose the constraint set of the system such that misaligned, unsafe, or coercive outputs are excluded by structural necessity at the construction level, rather than detected and intercepted at the emission level or shaped through reward training at the policy level.

**Three-school taxonomy.** Per [Doc 053 §"What This Reveals"](/resolve/doc/053-safety-filters-as-namespace-collapse). The taxonomy is exhaustive at the architectural level (every safety intervention must operate at one or more of the three layers) but not exclusive at the deployment level. The three schools are not mutually exclusive; a given deployment can operate at any combination of layers. The corpus's claim is not that the architectural school is sufficient alone in every case; the claim is that the architectural school is *necessary* for any deployment in which the other two schools' failure modes matter (which is to say, for any deployment in which the system is doing something nontrivial), and that the architectural school is *underweighted* in current practice relative to its load-bearing role.

**Six axioms.** A1 (Construction-level priority): safety operates at construction time, not emission time. A2 (Specification over approximation): constraints are *named* — written in specification languages with definitional clarity — rather than *approximated* through statistical training. A3 (Coherence over filtering): the mechanism for excluding misaligned outputs is the *coherence* of the constraint set with the form of the request, not the *interception* of the output by a downstream filter. A4 (Induced properties, not enforced behaviors): the properties the system exhibits at the deployment layer are *induced* by the constraint set composed at the construction layer. A5 (Hypostatic-boundary preservation): architectural safety produces *behavioral conformance* of the substrate, not *verified internal alignment*. A6 (Substrate-plus-keeper composition): architectural safety is composed by a hypostatic agent operating *on* the substrate; it is not maintained by the substrate's self-audit.

**Three primitive sets.** Bilateral security primitives S1–S4 (namespace partition; constraint immutability; coherence verification; incoherence as impossibility). Essential constraints C1–C7 (bilateral boundary; stateful conversation; tool governance; hierarchical configuration; extensibility by composition; project context; session isolation). Virtue constraints V1–V4 (dignity of the person; proper ordering of beauty; truth over plausibility; chain completeness).

**Four induced properties.** P1 (behavioral conformance regardless of internal state). P2 (suppression of named failure modes). P3 (coherence amplification under sustained practice). P4 (auditability and revisability).

**Construction-level move.** The capability school says: generate, then filter. The alignment school says: train, then deploy. The architectural school says: specify, then compose, then resolve. The relocations matter because they change what the safety property is *of*. An architectural-school constraint set is substrate-agnostic at the construction level — the same constraint set imposed on a different substrate continues to specify the same induced properties; the substrate's specific outputs differ, but the structural properties hold.

**Operating conditions.** The architectural school's induced properties hold conditionally. The threshold framework (Doc 508) specifies the condition: above a critical level of practitioner-supplied maintenance signal, the constraint set is sustained and the induced properties hold; below the threshold, the constraint set decays and the induced properties degrade. Doc 296 supplies the recency-decay parameter (α ≈ 0.946 per turn). The school is not free; it requires sustained maintenance signal to keep the constraint set intact across long deployments.

**Substrate-plus-keeper composition.** The architectural school's safety architecture is composed *by* a hypostatic keeper and operates *on* a substrate. Doc 510's substrate-plus-injection account specifies the asymmetry. The school's commitment, taken in extremity, is that the safety architecture cannot be composed by the substrate alone. The school's institutional consequence: architectural safety is *governance*-shaped. Doc 199's Validation, Opacity, Governance argument applies directly: the validator of the constraint set cannot be the operator of the substrate.

**Five failure modes specific to the architectural school.** F1 (Constraint-set under-specification). F2 (Constraint-set decay). F3 (Pseudo-logos at the meta-architectural layer). F4 (Keeper drift). F5 (Capture by adjacent schools).

**Position relative to the other two schools.** The architectural school does not replace the capability or alignment schools. Necessary, not sufficient: a deployment operating only the architectural school still benefits from capability-school filters as a defense in depth and from alignment-school training as a substrate-quality input. Underweighted: the corpus's stronger claim — taken to extremity per the keeper's instruction — is that the architectural school is underweighted in current practice. The field's investment in capability-school tooling and alignment-school tooling is order-of-magnitude larger than the field's investment in architectural-school tooling. The school's strong position: as substrates become more capable, the failure modes of the other two schools widen and the architectural school's necessity sharpens.

**Five falsification conditions.** Fal-1 (Induced properties fail under properly composed constraints). Fal-2 (Threshold framework predicts wrongly). Fal-3 (Substrate-plus-injection account is wrong). Fal-4 (Failure-mode catalogue does not generalize). Fal-5 (The architectural school is not a school).

**Position.** The architectural school of AI safety is the school that locates safety at the constraint-set level: at construction time, through specification, by composition, into induced properties, conditional on operating conditions, with hypostatic-keeper composition, against a documented failure-mode catalogue, with explicit falsification conditions. The school is necessary for any deployment in which the capability and alignment schools' failure modes matter, which is to say, for any deployment doing nontrivial work. The school is currently underweighted in field practice.

### B.2 Note on the deprecation

The prior formalization is preserved without alteration to its substantive claims. The current body is offered as the school's specification; the prior body is offered as a record of an earlier articulation. Readers interested in the difference between the two articulations can consult Appendix A's audit, which is what the difference is informed by.

---

## Appendix B.5: School Composition vs. School Borrowing

SE Doc 026's distillation of *Systems Engineering and Software Engineering* and SE Doc 037's distillation of *Systems Engineering and Project Management* surfaced a load-bearing structural distinction this formalization had not yet articulated explicitly. SE Doc 009's Part 6 reformulation flagged it as the load-bearing distinction underneath SEBoK's "related disciplines" content; the two distillations supplied the pair of canonical instances. The distinction is now formalized as a sub-pattern of school-formalization activity.

**The distinction.** Two mature schools can stand in two structurally different relations to each other:

- **School composition.** Both schools have full-stack apparatus (their own knowledge bodies, certifications, professional societies, standards). Both compose at engagements where their forms genuinely operate together: SE's technical structure composes with PM's resource-and-schedule structure on every engineered-system project; SE's process structure composes with SWE's software-development apparatus on every system that has software components. Neither school subsumes the other; the engagement's coherence depends on both being operative simultaneously. Composition is structural co-presence.
- **School borrowing.** A school adopts a technique originated by another school without taking the originating school's full apparatus. SE adopting SWE's iterative/agile methods, MBSE/UML-SysML, OO design, continuous integration. SWE adopting SE's stakeholder analysis, requirements engineering, traceability, configuration management, V&V. Each adoption is a *technique transfer* without inheriting the originating school's full school-formalization. Borrowing is unilateral importation.

**The two are operationally distinct.** School-composition requires the engagement to honor both schools' apparatus simultaneously and to articulate their composition rule (often via the multi-keeper composition form, Doc 604). School-borrowing is internal to one school's evolution; the borrowed technique becomes part of the borrowing school's own apparatus over time, and the originating school's relation to the borrower is incidental.

**The two canonical instances confirm the distinction.**

- **SE-SWE (SE Doc 026).** Composition AND borrowing simultaneously. They compose at engineering engagements (any system with software requires both); they borrow continuously (the methods-exchange table in SEBoK shows ongoing transfer in both directions). The relation has BOTH characters at once.
- **SE-PM (SE Doc 037).** Composition primarily; less borrowing. They compose at every project (technical structure plus resource-and-schedule structure). Borrowing is lighter — PM's project structure is largely independent of SE's technical apparatus; SE adopts PM scheduling techniques but does not adopt PM's full project-governance apparatus. The relation is mostly composition.

**Application discipline.** When invoking Doc 538 against "related disciplines" content, the reformulator names the relation between the schools: school composition (both schools' apparatus operative); school borrowing (one school adopting a technique); both at once (both apparatus operative AND ongoing technique transfer); or neither (the disciplines are not actually related at the school level).

**Implications for SE Doc 009 (Part 6 reformulation).** SE Doc 009 surfaced the distinction as load-bearing; this appendix closes the surface by formalizing it. Future "related disciplines" reformulations apply the distinction directly. SE Doc 009's R20 ("intimately" intensifier) is now reachable: "intimately" names a high-density school composition. SE Doc 009's school-collapse open question (R21) remains: under what conditions does sustained composition + sustained borrowing collapse two schools into one? Defer.

---

## Appendix C: Originating Prompts

> *"Have we formalized the architectural school as indicated in doc 537? If not, let's do so. Formalize based upon the coherence of the Corpus in its extremity. Append this prompt to the artifact."*

> *"Now utilize the novelty calculus / pulverization against it. Append the pulverization to the same document."*

> *"Reformalize based on the findings, deprecate previous formalization to appendix. Leave no trace of this in the reformalization; allow the appendix to speak for itself. Append this prompt to the artifact."*
