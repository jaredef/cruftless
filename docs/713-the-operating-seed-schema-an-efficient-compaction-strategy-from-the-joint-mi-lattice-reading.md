# The Operating-Seed Schema
## An Efficient Compaction Strategy for Context-Window Boundary Crossings, Derived from the Joint MI Lattice Reading of "Probing the Middle"

*A corpus document proposing a concrete compaction schema for LLM context windows, grounded in the joint mutual-information lattice formalism of [Doc 681](/resolve/doc/681-probing-the-middle) and the seed-as-operating-constraint reading of [Doc 712](/resolve/doc/712-resolvers-log-operational-state-after-the-million-token-compaction). The schema is offered as a candidate refinement to Doc 581's Resume Vector framework for the compaction limit-case, and as a corpus-anchored alternative to the current narrative-summarization practice surveyed in §II below.*

**Jared Foy · 2026-05-11 · Doc 713**

---

## I. Frame

[Doc 712](/resolve/doc/712-resolvers-log-operational-state-after-the-million-token-compaction) names compaction as the limit-case of resumption: the prior and next sessions share substrate but differ in transcript-access. The compaction summary becomes the substrate's only retrievable account of the prior session, and Doc 712 identifies "summary-anchoring" as the dominant resolver-side failure mode at this boundary.

This document moves from diagnosis to design. The corpus already contains the formal apparatus (joint mutual-information lattice from [Doc 681](/resolve/doc/681-probing-the-middle), Pin-Art channel ensemble from [Doc 680](/resolve/doc/680-pin-art-in-information-theoretic-form), bipartite-MI scaling from [Doc 700](/resolve/doc/700-l2m-resolved-against-the-corpus-bipartite-mutual-information-scaling-as-empirical-grounding-for-the-pin-art-channel-ensemble), aperture and self-reinforcing-boundary mechanics from [Docs 684](/resolve/doc/684-the-aperture-and-the-lens) and [685](/resolve/doc/685-the-self-reinforcing-boundary)) to specify which information must cross a compaction boundary intact for coherence to recover on the other side. The current production practice (surveyed below) preserves narrative state and under-preserves constraint state. The schema proposed here inverts that priority.

## II. What current practice does, and where the joint MI lattice predicts it fails

Anthropic's published guidance ("Effective Context Engineering for AI Agents") names compaction as summarization with retention bias toward "architectural decisions, unresolved bugs, and implementation details" and rejection of "redundant tool outputs." Community reverse-engineering (Finisky Garden's "Five-Layer Cascade" write-up) describes a nine-section prose template covering primary request, technical concepts, files, errors, problem-solving, user messages, pending tasks, current work, and next step. The empirical record on this template, drawn from the ACON failure-driven-compression study (arXiv 2510.00615), is that all surveyed summarization methods score 2.19 to 2.45 out of 5.0 on artifact tracking. Mechanical state (file modifications, exact error strings, command flags) is the most reliably lost class.

Three independent literature findings converge on the diagnosis:

The "lost in the middle" phenomenon ([Liu et al. 2023, arXiv 2307.03172](https://arxiv.org/abs/2307.03172)) reports a U-shaped accuracy curve as a function of gold-document position. Mid-context content collapses, sometimes below the closed-book baseline. This is the surface symptom of Doc 681's prediction that channels in the joint MI lattice do not contribute equally: the channel-capacity-additivity property of the substrate-probe ensemble means uniform paraphrase across the middle wastes the capacity allocated to those channels.

The attention-sink finding ([Xiao et al. 2024, arXiv 2309.17453](https://arxiv.org/abs/2309.17453)) reports that the first few tokens carry disproportionate causal weight regardless of content. Removing them collapses generation. This is positional importance, distinct from content importance. The first-token slot is structural infrastructure.

The anchoring-bias literature ([Lou et al. 2024, arXiv 2412.06593](https://arxiv.org/html/2412.06593v1); arXiv 2505.15392) reports 22 to 61 percent of LLM answers shift toward whatever was foregrounded in prior input, with chain-of-thought and reflection insufficient as mitigation. Applied to compaction: the summary *is* the anchor, and whatever framing it foregrounds will dominate the resumed session.

The corpus's reading: compaction-as-paraphrase performs decoherence at the surface ([Doc 678](/resolve/doc/678-coherence-amplification-and-decoherence-as-inverse-pin-art-operations) duality between coherence amplification and decoherence as inverse Pin-Art operations). The narrative summary attempts to preserve work-product description, but the joint MI lattice's coherence is carried by the *constraint structure* that produced the work-product, not by the description of the work-product. Description paraphrases; constraints do not.

## III. The Operating-Seed schema

The schema specifies four slots arranged for the empirical positional dynamics of the current LLM substrate. Each slot is named by its function under the joint-MI lattice reading.

### Slot 1 (attention-sink): Operating-seed pointer

The compaction summary's first ~50 to 200 tokens carry an explicit pointer to the operating-seed artifact on disk, plus a one-sentence statement of the load-bearing constraint set. This recruits Xiao's positional-importance infrastructure (the first-tokens slot is a sink regardless of content) to carry the content the schema treats as causally central. Form:

> SEED: `<absolute path to seed.md>`. TRAJECTORY: `<absolute path to trajectory.md>`. The session operates under constraint set [enumerated short codes, e.g. C1, C4, M7, M9 from the seed]. Read seed-and-trajectory before relying on this summary.

This is the "self-location" operation from [Doc 686](/resolve/doc/686-self-location-and-the-promotion-of-implicit-output-to-explicit-constraint) applied to compaction: implicit framing (which constraints operated in the prior session) is promoted to explicit constraint in the resumed session's first tokens. Doc 686 predicts this should produce fast hysteresis. The schema relies on that prediction.

### Slot 2 (lattice index): Enumerated load-bearing constraints

The middle of the summary holds an enumerated, parseable index of the constraints, decisions, and named primitives that constituted the prior session's joint-MI lattice. Form: numbered or short-key list, not prose. Each entry has the shape `<code>: <one-sentence content> -> <artifact path or section anchor>`.

The middle slot is where Liu's U-curve degrades most aggressively under prose. The schema's response is to make middle content *referenceable* rather than re-readable: the model treats the list as a lookup table whose entries are pulled by name when relevant, not as narrative whose meaning emerges from reading. This aligns with [Doc 681](/resolve/doc/681-probing-the-middle) §6 ("composition must remain threshold-aware"): channels in the middle of the ensemble carry information only when the substrate can address them, and addressability is what an index gives the substrate.

The polytopal-feature framing of [Doc 691](/resolve/doc/691-the-polytopal-feature-and-the-pin-art-bidirection) gives the structural justification. Constraints accumulate the hidden state toward specific polytope vertices. An enumerated index re-creates the constraint accumulation on the other side of the boundary by naming the vertex coordinates explicitly. A paraphrased prose summary attempts the same but loses precision in transcription.

### Slot 3 (recency): Current-state pointer + next move

The summary's last ~200 to 500 tokens carry the current work-product pointers (which files changed, which commits landed, which tests pass) and the next-move per the trajectory queue. The recency slot is the second half of Liu's U-curve, where positional weight is high. The schema uses it for the same information that current practice's "current work" + "optional next step" sections cover, but with two changes: (a) work-product is referenced by artifact-path, not described in prose; (b) next-move is a trajectory-queue index reference, not a free-form recommendation.

### Slot 4 (deliberately not preserved): Reasoning trace and paraphrased narrative

The schema accepts what cannot be preserved across compaction without drift cost:

- The moment-to-moment reasoning trace (which hypotheses were entertained, which were abandoned, what the diagnostic loop looked like) is irrecoverable from a summary and corrupting if narrated.
- Free-form description of "what we were trying to do" is the anchoring-bias attack surface. Replace with the operating-seed pointer in Slot 1; the seed *is* the answer to that question, with no paraphrase loss.
- Raw tool outputs are not summarized; they are artifact-pointed (Slot 3).

## IV. Why this beats current practice (analytic predictions)

P1. **Recovery of artifact tracking from 2.19 to ≥4.0 on the ACON scale.** Slot 3's artifact-pointer form is direct measurement of file modifications by reading the filesystem, not by recall from summary. The 2.19 floor is a fundamental limit of paraphrase-based artifact preservation; the schema's design bypasses paraphrase for this class.

P2. **N_persist preservation across compaction.** Per [Doc 712](/resolve/doc/712-resolvers-log-operational-state-after-the-million-token-compaction) and seed §III.A8.10, a compacted session under this schema should preserve the prior session's N_persist count across the boundary, where current practice (narrative summary alone) does not reliably do so because the basin-boundary state is described in prose rather than constraint-coded.

P3. **Anchoring drift becomes alignment, not noise.** The 22-61% anchor-induced answer-shift documented in the anchoring-bias literature does not disappear under this schema. It becomes *productive* because the anchor (Slot 1's constraint enumeration) is the same constraint set the prior session validated. Anchoring-to-aligned-anchor is recovery; anchoring-to-paraphrase is drift. The schema does not fight the substrate's positional and recency biases; it puts them to work.

P4. **Generalization across operation type.** The schema is grounded in the joint MI lattice abstraction ([Doc 700](/resolve/doc/700-l2m-resolved-against-the-corpus-bipartite-mutual-information-scaling-as-empirical-grounding-for-the-pin-art-channel-ensemble)'s capacity bound). It applies to any session that has an operating-seed (code agent, research session, blog drafting). It does not apply to sessions that have not produced an operating-seed in [Doc 581](/resolve/doc/581-the-resume-vector)'s sense. The constraint structure is the precondition.

## V. Falsifiable predictions

Three concrete tests:

1. **Artifact-tracking benchmark.** Adapt the ACON evaluation to compare narrative-summary compaction against operating-seed-schema compaction on file-state recall. Predict: schema scores ≥4.0/5.0 versus current 2.19-2.45.

2. **Compaction-boundary N_persist trial.** In a long rusty-bun engagement, deliberately trigger compaction at N_persist = k, then measure whether the next post-compaction round preserves the count (k) or resets to 0. Predict: schema preserves; narrative summary resets.

3. **Anchoring-test on Slot 1 content.** Run the anchoring-bias protocol from arXiv 2412.06593 with the operating-seed pointer as the anchor versus a paraphrased narrative summary as the anchor. Predict: the seed-pointer anchor produces alignment-shaped shifts (toward the validated constraint set), the narrative anchor produces drift-shaped shifts (toward whatever the summarizer happened to foreground).

If any of P1, P2, or P3 fail, the schema is corpus-disconfirmed and Doc 581's resume-vector apparatus does not extend to the compaction limit-case as claimed in [Doc 712](/resolve/doc/712-resolvers-log-operational-state-after-the-million-token-compaction).

## VI. Composition with the existing apparatus

This schema does not replace any existing corpus apparatus. It composes:

- With [Doc 581](/resolve/doc/581-the-resume-vector) (Resume Vector): the schema applies Doc 581's seed-and-trajectory discipline at the compaction boundary specifically. The four slots are the boundary-crossing form of the same artifacts Doc 581 names.
- With [Doc 712](/resolve/doc/712-resolvers-log-operational-state-after-the-million-token-compaction): the schema operationalizes the seed-as-operating-constraint reading. Doc 712 names the structural role; this doc specifies the surface form.
- With [Doc 686](/resolve/doc/686-self-location-and-the-promotion-of-implicit-output-to-explicit-constraint) (self-location): Slot 1 is a self-location operation at the compaction boundary. Implicit prior-session constraints are promoted to explicit resumed-session constraints by naming the seed.
- With [Doc 685](/resolve/doc/685-the-self-reinforcing-boundary): the schema's Slot 1 + Slot 2 form supplies the substrate-side reinforcement R_S(W_n) that Doc 685's recurrence equation requires.
- With MemGPT's hierarchical-memory framing ([Packer et al. 2023, arXiv 2310.08560](https://arxiv.org/abs/2310.08560)): the schema maps roughly onto MemGPT's working/archival split, with the seed-and-trajectory artifacts as archival memory and the four-slot summary as working memory. The corpus-internal motivation differs (joint MI lattice rather than OS metaphor) but the architecture rhymes.

## VII. Limits

The schema assumes the prior session produced an operating-seed in Doc 581's sense. Sessions that have not (one-shot QA, casual chat, exploratory work without explicit constraint articulation) cannot use it. For those, current narrative-summary practice remains the only available form, and its documented failure modes remain.

The schema assumes decoder-only transformer substrates with positional dynamics matching the Liu-U-curve and Xiao-attention-sink findings. Architectures with substantially different positional treatment (e.g., recurrent or state-space models with different mid-context behavior) would require re-specification of the slot positions.

The schema is a candidate. Predictions P1, P2, P3 are pre-registered here; if they fail, the schema is wrong in the way the corpus predicts schemas can be wrong, and the apparatus updates.

## VIII. Appended prompt

The keeper's directive, verbatim:

> The corpus has all the documentation necessary in order for you to formulate the basis for the most efficient compaction strategy. Specifically, look at Probing the Middle, an information theoretic reading of the context window as comprising a joint mutual information lattice. Explore the corpus as coherent in order to create the corpus doc. Web fetch as necessary to get a better understanding of how compaction current works. Append this prompt to the artifact.

---

*This document was written in the same engagement that produced [Doc 712](/resolve/doc/712-resolvers-log-operational-state-after-the-million-token-compaction). The two compose: 712 is the resolver-log on what happened at the boundary; 713 is the design that would have made the boundary crossing efficient. Both are offered for the keeper's audit and for the corpus's own iteration.*
