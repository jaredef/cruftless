# A Candidate Formalization of SIPE, Built From Its Pulverized Pieces

> **Canonical SIPE reference:** The operative formalization of Systems-Induced Property Emergence is [Doc 474](/resolve/doc/474-sipe-standalone-formalization). Read it first for the current definition, the three structural commitments, and the per-stack testability protocol. This document contributes specific material to that formalization; its place in the development arc is recorded in Doc 474's Appendix C.

## The move and its status

Doc 444 pulverized the confabulated SIPE expansion and found every component subsumed: *sustained* under continual / streaming inference, *inference* under Bayesian inference broadly, *probabilistic* under probabilistic programming and graphical models, *execution* under program-execution trace semantics. The composed phrase — even though it is not a named published technique — corresponds cleanly to a class of real methods (sequential Monte Carlo, streaming variational Bayes, probabilistic-programming trace-based inference, online Bayesian state-space filtering).

This document takes the deliberate next step: assemble those pieces into a formal construct and label it SIPE. Whether the result is what the corpus has always meant by SIPE, whether it is what the keeper intends, whether its claims about the corpus's operation are true — these remain open questions at the μ and θ tiers of Doc 445's formalism. What follows is π-tier work, executed on the debris of a confabulation. Its status, under the warrant table, is *semantically plausible, truth untested*. The artifact is offered as a candidate for keeper ratification, not as a fait accompli.

The exercise is a live test of Doc 443's coherentism risk. If the construct looks elegant, promises cleanly, and absorbs prior corpus vocabulary smoothly, those are exactly the features that would push the generator-keeper dyad toward accepting it without μ/θ audit. The honest response is to produce the construct, name its status, and refuse the promotion.

## Ingredients from the decomposition

The four decomposed pieces contribute distinct formal components:

- **Sustained** → a temporal/sequential structure. Computation proceeds over time steps, not as a single one-shot posterior. Formal home: online Bayesian updating, sequential Monte Carlo, streaming variational Bayes.
- **Inference** → a posterior over unobserved quantities, computed by Bayesian conditioning. Formal home: \(p(\theta \mid \text{data})\) as the object of interest.
- **Probabilistic** → the quantities being inferred live in a probability space; outputs are samples or distributions, not point estimates. Formal home: probabilistic graphical models, probabilistic programs.
- **Execution** → the computation is a *trace* through a program with stochastic choice points. Formal home: Wingate–Stuhlmüller–Goodman trace semantics for probabilistic programming.

The four together produce: **a stochastic program whose execution proceeds across time, maintaining a posterior at each choice point conditioned on the accumulated execution history.** This is a real class of objects. Particle filters are one instance; probabilistic-programming Markov-chain inference over trace rewrites is another.

## Formal definition (candidate)

Let \(\mathcal{P}\) be a probabilistic program — an abstract procedure containing an ordered sequence of stochastic *choice points* \(c_1, c_2, \ldots\) where execution samples a value from a distribution and continues. Let \(C\) be a *conditioning corpus*, \(D\) a *discipline set*, and \(Q\) a prompt. Let \(\mathcal{H}_t = (c_1, \ldots, c_{t-1})\) denote the *execution history* prior to step \(t\).

**SIPE** is the procedure:

1. At each choice point \(c_t\), maintain the posterior
   $$p(c_t \mid C, D, Q, \mathcal{H}_t)$$
   formed by conditioning on prior context \(C\), discipline set \(D\), prompt \(Q\), and the accumulated execution history \(\mathcal{H}_t\).
2. Sample \(c_t \sim p(c_t \mid C, D, Q, \mathcal{H}_t)\), or select \(c_t\) under an alternative decoding rule (see §"Decoding regimes").
3. Append \(c_t\) to \(\mathcal{H}_t\); continue execution with the sampled value.
4. Repeat until \(\mathcal{P}\) terminates.

The resulting sequence \(\tau = (c_1, c_2, \ldots, c_N)\) is the **derivation** produced by the SIPE run. Its joint probability under SIPE is

$$p(\tau \mid C, D, Q) \;=\; \prod_{t=1}^{N} p(c_t \mid C, D, Q, \mathcal{H}_t).$$

### The branching set \(B_t\)

The **branching set at step \(t\)** is the effective support of \(p(c_t \mid C, D, Q, \mathcal{H}_t)\). Its cardinality is measured operationally by the Shannon-entropy proxy from Doc 440 §4.1:

$$\widehat{|B_t|} = \exp\!\big(H\!\big(p(\cdot \mid C, D, Q, \mathcal{H}_t)\big)\big).$$

\(\widehat{|B_t|}\) is small when the conditioning has nearly collapsed the posterior to a point mass (deterministic step); large when the step is genuinely under-determined by the conditioning.

### The nested-manifold correspondence

The posteriors at successive steps are progressively conditioned in the manner of Doc 439's nested-manifold frame:

$$M_0 \supseteq M_1 = M_0 \mid C \supseteq M_2 = M_1 \mid D \supseteq M_3 = M_2 \mid Q.$$

Each SIPE step further conditions \(M_3\) on the execution history, producing a sub-manifold \(M_3 \mid \mathcal{H}_t\) at step \(t\). The derivation \(\tau\) is a walk through the sequence of these per-step sub-manifolds.

### Decoding regimes

Different procedures for producing \(c_t\) correspond to named inference strategies in the probabilistic-programming / decoding literature:

- \(c_t = \arg\max_c p(c \mid \ldots)\) — **argmax SIPE**, a greedy low-temperature trace; the derivation is the most-probable path under the conditioning.
- \(c_t \sim p(c \mid \ldots)\) — **sampled SIPE**, the standard stochastic regime.
- \(k\)-parallel candidates with pruning — **beam SIPE**, analogous to beam search but specified as a SIPE variant.
- \(N\)-particle maintenance with resampling — **particle SIPE**, isomorphic to sequential Monte Carlo applied to the program trace.
- MCMC over whole traces — **Metropolis-Hastings SIPE**, isomorphic to the Wingate–Stuhlmüller–Goodman LMH algorithm.

Each regime has different implications for \(\widehat{|B_t|}\), convergence behavior, and failure modes. The choice of regime is part of the methodology under Doc 440.

## What falls out

Several corpus-internal concepts acquire formal homes once SIPE is defined this way.

### The derivation is the trace

The corpus's *derivation* becomes a well-defined object: the sequence \(\tau\) of sampled choice-point values produced by a SIPE run. Two derivations from the same \((C, D, Q)\) may differ; the distribution over derivations is \(p(\tau \mid C, D, Q)\). Comparing derivations across sessions becomes a task of comparing samples from this distribution, with machinery from the sampling literature directly applicable.

### Branching-set semantics sharpen

\(|B_t|\) is no longer only a metaphor; it is the effective support size of a specific posterior. Its relation to temperature, to conditioning depth, and to discipline set \(D\) is all machinable. The observables in Doc 440 §4 re-express as measurements on this posterior.

### Forced determinism has a formal signature

Forced-determinism sycophancy (corpus term) becomes, under the formalization: *\(\widehat{|B_t|} \to 1\) at choice points where the task is underdetermined by the conditioning.* The prompt \(Q\)'s pressure collapses the posterior even where \(C\) and \(D\) would have supported branching. The corpus term names a specific pathology; the formalization makes the pathology measurable.

### Coherence curves become posterior-concentration trajectories

The "coherence curve" — a qualitative observable the corpus uses — maps to the trajectory of some concentration measure (e.g., \(\widehat{|B_t|}\), or mean semantic similarity of samples) across \(t\). If the curve is the derivation's concentration history, its axes are now well-defined.

### SIPE is an *instance* of a larger category

Critically: SIPE under this definition is not novel. It is a specific instance of **trace-based online probabilistic inference** under conditioning that happens to be corpus-generated rather than data-stream-generated. Sequential Monte Carlo with \(C \cup D \cup Q\) as conditioning would compute essentially the same object. The formalization places SIPE cleanly inside a pre-existing taxonomy. Doc 445's warrant table marks this: as a specification-target, SIPE is π-subsumed, not novel. As a definitional-target (*does SIPE in the corpus mean this?*), status is "semantically plausible, truth untested."

### Dyadic discipline becomes a family of operators

The practitioner's discipline set \(D\) can be formalized as a family of conditioning operators \(\{d_1, \ldots, d_k\}\) each of which shrinks the support of the posterior at every choice point:

$$p(c_t \mid C, D, Q, \mathcal{H}_t) \;=\; \frac{p(c_t \mid C, Q, \mathcal{H}_t) \cdot \prod_{d_i \in D} \mathbb{1}[d_i(c_t, \mathcal{H}_t)]}{Z_t}$$

where \(\mathbb{1}[d_i(\cdot)]\) is the indicator of \(d_i\)'s satisfaction at the current step and \(Z_t\) is the normalizing constant. Disciplines formally *prune* the posterior's support. "Non-coercion" becomes a specific pruning rule. Hypostatic-boundary preservation becomes another. The ENTRACE stack becomes a composite operator with specific prunings across its components. Each named discipline is in principle executable — or at least characterizable — as a rule on \((c_t, \mathcal{H}_t)\).

### The SIPE/retraction-ledger interaction

Under the formalization, a retraction event corresponds to discovering that an emitted \(c_t\) — or a downstream consequence of it — contradicts \(Q\)'s external ground. The retraction ledger (Doc 415) is a record of such contradictions. The hypothesis ledger (Doc 443) is a record of \(c_t\)s whose external grounding has not yet been attempted. The pulverization formalism (Doc 445) classifies new \(c_t\)s by target type and specifies which tier of test applies.

## What falls out the wrong way

The same exercise produces exactly the appearance of productivity that Doc 443 warned about. The construct is elegant, accommodates prior corpus vocabulary, and generates plausible connections to published literature. Every feature that makes it attractive is also a feature that makes it dangerous as a candidate for uncritical promotion.

Specifically:

- The construct's elegance is a poor predictor of its fidelity to the keeper's actual intent for SIPE. A different choice of decomposed pieces — e.g., *Serial Inferential Path Elaboration*, *Structured Iterative Posterior Extraction*, or any of several other plausible expansions — would produce a different but equally elegant construct. The space of elegant constructs compatible with the four letters is not small.
- The construct recovers prior corpus concepts (branching set, derivation, coherence curve, forced determinism) cleanly. This is load-bearing against promotion, not for it: if the corpus's own concepts could be recovered from *any* of several plausible SIPE expansions, then the recovery is a property of the corpus's conceptual neighborhood, not of this specific construct's fidelity.
- The construct is consonant with Docs 437, 439, 440, 444. Under the cohort dynamics Doc 442 diagnosed, consonance with recent cohort material is the attractor-shape the feedback loop is already pulling toward. Another way to say this: the generator that confabulated the expansion and the generator that formalizes the pulverized pieces share a conditioning context that virtually guarantees they will produce mutually coherent outputs.

The honest position: this artifact has produced a formalization that coheres with itself, with its predecessor documents, and with the subsumed literature. The keeper should trust none of these coherences as evidence of truth.

## What this construct licenses, under Doc 445's warrant table

- **As a \(T_S\) (specification):** Not novel. The construct is a specific parameterization of trace-based online probabilistic inference. No promotion on novelty warrants.
- **As a \(T_D\) (definition of corpus SIPE):** \(\pi\)-subsumed. Status: *semantically plausible, truth untested*. Hypothesis-ledger entry required. Truth test is: keeper confirms this is the intended expansion; or operational-match test against corpus usage of SIPE confirms the construct describes what SIPE has done in practice; or the corpus declares SIPE is *not* this construct.
- **As a \(T_B\) (bridge from corpus to probabilistic-programming literature):** \(\pi\)-subsumed at element level; bridge claim at \(\pi\) tier. Requires \(\mu\)-tier evidence (observations of corpus sessions behaving like SIPE runs) for structural-soundness warrant.
- **As a \(T_M\) (methodology for future corpus derivation):** Cannot claim fitness until executed and audited.

In all four readings, the artifact's status is *candidate, pending higher-tier test*. Doc 445's discipline applies: status may only reflect tiers actually run. The elegance of the construct does not substitute for the tests it has not undergone.

## Open questions

- Does the corpus's existing usage of SIPE, across its ~199 occurrences, operationally match this construct? An \(\mu\)-tier audit would read each occurrence and check fit. The audit has not been performed.
- Is the decomposition in Doc 444 the right decomposition? A different seeding of the confabulation — different priors on the four letters — would produce a different formalization. Whether "Sustained-Inference Probabilistic Execution" is the *best* expansion, or merely *a* plausible expansion, is underdetermined.
- Can the discipline operators be actually specified as executable rules? The paragraph in §"Dyadic discipline becomes a family of operators" gestures at indicator functions \(\mathbb{1}[d_i(\cdot)]\); writing out a real operator for "non-coercion" requires specifying what non-coercion means at the level of \((c_t, \mathcal{H}_t)\) — a non-trivial specification task.
- What happens at choice points where the prompt requires behavior outside the conditioned manifold's support? Formally: \(p(c_t \mid \ldots) = 0\) over all \(c_t\) that would satisfy \(Q\). The operator analogy breaks; in practice, the forward model produces *something*. Characterizing that something is the forced-determinism-meets-confabulation intersection.
- Does a keeper-ratified SIPE need to be this construct, or a closely-related one that diverges from it in specific ways (e.g., the choice points are not stochastic in the usual sense but are deterministic conditional on a different structure)? The construct is a starting proposal for the keeper's response, not a closed case.

## Limitations

The construct relies on probabilistic-programming trace semantics, which is a well-developed but not universal framing of probabilistic computation. Alternative framings (e.g., differentiable probabilistic programming, categorical probability via Markov categories) would produce different formalizations.

The reduction of discipline operators to indicator functions is a strong simplification. Real disciplines may have graded, continuous effects on the posterior rather than hard support-pruning. The construct can accommodate soft operators (multiplicative re-weighting instead of indicators) at the cost of losing the clean support-shrinkage reading.

The treatment of \(\mathcal{H}_t\) as conditioning implicitly assumes independence of past stochastic choices conditional on the present conditioning — which is how autoregressive LLMs work at the token level, but not necessarily how higher-level corpus derivation works.

The construct is a formalization, not an implementation. Instantiating it against a real LLM API would require specifying what counts as a *choice point* in the underlying inference procedure. Token-level, sentence-level, and argument-level choice points give different SIPE semantics.

This document has not modified Doc 439, Doc 441, Doc 415, Doc 435, the hypothesis ledger, or any prior corpus entry. It is a proposal standing in the hypothesis-ledger position §"What this construct licenses" describes.

## References

- Wingate, D., Stuhlmüller, A., & Goodman, N. (2011). Lightweight implementations of probabilistic programming languages via transformational compilation. *AISTATS*.
- van de Meent, J.-W., Paige, B., Yang, H., & Wood, F. (2018). An introduction to probabilistic programming. arXiv:1809.10756.
- Wood, F., van de Meent, J.-W., & Mansinghka, V. (2014). A new approach to probabilistic programming inference. *AISTATS*.
- Doucet, A., de Freitas, N., & Gordon, N. (Eds.). (2001). *Sequential Monte Carlo Methods in Practice*. Springer.
- Andrieu, C., Doucet, A., & Holenstein, R. (2010). Particle Markov chain Monte Carlo methods. *JRSS-B*, 72(3), 269–342.
- Broderick, T., Boyd, N., Wibisono, A., Wilson, A. C., & Jordan, M. I. (2013). Streaming variational Bayes. *NeurIPS*.
- Särkkä, S. (2013). *Bayesian Filtering and Smoothing*. Cambridge University Press.
- Fritz, T. (2020). A synthetic approach to Markov kernels, conditional independence and theorems on sufficient statistics. *Advances in Mathematics*, 370. (For Markov-category alternative framings.)
- Corpus Doc 415: *The Retraction Ledger*.
- Corpus Doc 439: *Recursively Nested Bayesian Manifolds*.
- Corpus Doc 440: *Testing the Nested-Manifold Hypothesis*.
- Corpus Doc 441: *SIPE Confabulation Case Study*.
- Corpus Doc 443: *Confabulation as Potential Emergence*.
- Corpus Doc 444: *Pulverizing the SIPE Confabulation*.
- Corpus Doc 445: *A Formalism for Pulverization*.

## Appendix: Originating prompt

> Great now formalize the new SIPE construct that was confabulated, based upon its decomposed state after pulverization. We will see what falls out. Append the prompt to the artifact.
