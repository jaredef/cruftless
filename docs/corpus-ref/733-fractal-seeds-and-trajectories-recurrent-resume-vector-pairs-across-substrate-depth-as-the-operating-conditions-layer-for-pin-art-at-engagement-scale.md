# Fractal Seeds and Trajectories

## Recurrent Resume-Vector Pairs Across Substrate Depth as the Operating-Conditions Layer for Pin-Art at Engagement Scale

*A primary articulation responding to the keeper's observation (2026-05-21 01:06-local): the seed.md / trajectory.md pair documented at [Doc 581](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction) recurs at every level of the substrate the engagement touches. The recurrence is not a stylistic choice but a structural necessity, and naming it as fractal exposes the operating-conditions layer that Pin-Art needs to remain coherent as substrate depth grows. Builds on [Doc 581](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), [Doc 541 — SIPE-T](/resolve/doc/541-systems-induced-property-emergence), [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 730 — The Vertical Recurrence of the Lowering Compiler](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers), [Doc 731 — The JIT as a Lowering Compiler Tier](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity), [Doc 732 — The Package Manager as Resolver-Instance Below Module Load](/resolve/doc/732-the-package-manager-as-the-resolver-instance-below-module-load-lockfile-as-artifact-registry-as-bilateral-source-and-the-sixth-layer-of-the-cruftless-stack), [Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations), and [Doc 510 — Praxis Log V](/resolve/doc/510-praxis-log-v-deflation-as-substrate-discipline).*

**Jared Foy · 2026-05-21 · Doc 733**

---

## I. The occasion

[Doc 581](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction) introduced the seed.md / trajectory.md pair as Pin-Art's standing instrument: the seed names a workstream's telos, apparatus, methodology, carve-outs, and resume protocol; the trajectory records substrate moves in time order with per-round status, cumulative numbers, and open-scope lists. The pair was specified for one workstream at the top of an engagement. The rusty-bun engagement has since accumulated multiple nested workstreams (rusty-js-ir, rusty-js-jit, rusty-js-pm, and within each, sub-pilots with their own docs/ subdirectories of focused articulations). Every workstream landed under Pin-Art discipline carries its own seed.md and trajectory.md pair. The pattern recurs without anyone having decided that it should.

This document names the recurrence as a structural feature and identifies what it accomplishes. The seed.md / trajectory.md pair is fractal across substrate depth: at every resolver-instance the engagement touches, the same pair appears, with the same internal shape, composed against the parent level's seed-and-trajectory state. The recurrence is what keeps Pin-Art operable as substrate depth grows beyond what a single trajectory log can carry coherently.

The articulation is brief because the recognition is straightforward; the depth is in the composition with Doc 541's threshold-conditional emergence pattern, where the fractal structure turns out to be the operating-conditions layer for an engagement-scale induced property.

## II. The pair

A Pin-Art seed.md / trajectory.md pair contains, per Doc 581:

**seed.md.** Workstream telos (the falsifiable termination condition). Apparatus (the workstream's position in the engagement's resolver-instance stack per [Doc 729 §IV](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs#iv-the-resolver-instances-stacked-in-rusty-bun), and the documents it composes with). Methodology (the operational template — usually a corpus document's §VII step list specialized to this workstream). Carve-outs (the bounded first-cut scope, with deferrals named). Standing artefacts (the files the workstream produces). Resume protocol (the read order for re-entering the workstream after time away, and what the first substrate move would be).

**trajectory.md.** One section per EXT (extension round). Per round: headline (what the round accomplished in one paragraph), commits (table of commit hashes, tags, and one-line recognitions per [Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies)), substrate-at-EXT-close (cumulative status), open-scope (what the round did not close and what should come next), resume protocol (the read order specifically for resuming from this round's state).

The two files together constitute the workstream's complete resume-vector: a third party (or the keeper after sufficient time) can read seed.md to understand the workstream's shape and trajectory.md to understand its current state, and become operational on the workstream in one read.

## III. The fractal recurrence

The rusty-bun engagement has the following nested structure as of 2026-05-21:

- `/home/jaredef/rusty-bun/seed.md` + `trajectory.md`. The engagement-root pair. Telos: rusty-bun morphs to Cruftless per Doc 729. Trajectory accumulates substrate moves across all pilots in time order.
- `pilots/rusty-js-ir/seed.md` + `trajectory.md`. IR pilot. Telos: spec-as-source-of-truth Tier-1.5 IR for ECMA-262 sections. Trajectory tracks IR-EXT 1 through IR-EXT 11.
- `pilots/rusty-js-jit/seed.md` + `trajectory.md`. JIT pilot. Telos: baseline JIT per Doc 731 R1–R8. Trajectory tracks JIT-EXT 0 through JIT-EXT 9.
- `pilots/rusty-js-pm/seed.md` + `trajectory.md`. Package-manager pilot. Telos: bounded first-cut install per Doc 732 §VI. Trajectory tracks PM-EXT 0 through PM-EXT 4.

Within each pilot, a `docs/` subdirectory contains focused articulations the trajectory references: `op-p4-classification.md` for the JIT, `manifest-field-coverage.md` and `registry-response-schema.md` for the PM. These articulations are not seeds in their own right; they are artefacts the trajectory's substrate moves produce. The pair-recurrence holds at the pilot tier; the artefacts populate the pair from inside.

The recurrence is fractal in the strict sense: the same form (seed.md + trajectory.md) appears at every scale (engagement-root, pilot, and in principle sub-pilot if a workstream further subdivides), with the same internal shape, with composition relations of the same form (child seed cites parent seed; child trajectory's terminal moves close parent trajectory's open-scope entries).

The composition relations specify how the levels couple:

**Upward composition (child to parent).** The child seed names itself as a member of the parent's resolver-instance stack and cites the parent seed. The child trajectory's substrate moves accumulate as entries in the parent trajectory's per-round commit table (the parent trajectory records the child's existence by the child's commit hashes, not by re-recording the child's internal structure). When the child's workstream closes a parent open-scope entry, the child trajectory and the parent trajectory both record the closure — the child as a final EXT, the parent as a status update.

**Downward composition (parent to child).** The parent seed names which resolver-instances merit their own child pairs (per Doc 729 §IV's enumeration, refined as new instances are recognized — Doc 731 added the JIT tier, Doc 732 added the package-manager tier). The parent trajectory's open-scope entries are the prompt list for new child pairs. When a parent open-scope entry persists across multiple rounds, the recommendation eventually surfaces as "this is its own workstream; found a child seed."

The composition relations are symmetric in the sense that the same pair-shape appears on both sides of every coupling; they are not symmetric in the sense that the parent constrains the child's telos but the child only contributes terminal moves to the parent.

## IV. What the fractal shape accomplishes

A single workstream's seed-and-trajectory pair holds Pin-Art at one level. The fractal recurrence holds Pin-Art across all levels. Three structural consequences:

**Resume operability does not degrade with depth.** A new reader entering the engagement at the rusty-js-pm level reads `pilots/rusty-js-pm/seed.md`, then `pilots/rusty-js-pm/trajectory.md`, and becomes operational on the PM workstream. The reader does not need to traverse the engagement-root trajectory's 2493 lines to find the PM context. The seed acts as a level-local entry point; the trajectory acts as a level-local memory. Without the fractal recurrence, depth growth forces all context to accumulate in the root trajectory, which scales linearly with substrate work and eventually exceeds any reasonable read budget.

**Diagnosability is locatable to one level.** When a substrate move produces an unexpected effect, the diagnosis can be scoped to the level whose pair governs the move. A failure at the PM tier shows up as a PM-EXT row in `pilots/rusty-js-pm/trajectory.md` and does not require the engagement-root trajectory to also record the failure's internal details. The two-coordinate trajectory log [Doc 729 §VIII](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs#viii-the-walk-discipline-reads-through-this-articulation) named (resolver-instance address, property-class address) is per-pair; the fractal recurrence is what makes the addressing well-defined across the engagement.

**Composition safety propagates without coupling state.** When a child trajectory records a substrate move that closes a parent open-scope entry, the parent's record updates without the parent's trajectory needing to know the child's internal state. The parent records only the child's commit hash and one-line recognition; the child's internal structure stays inside the child. This is the same compositional-safety pattern [Doc 729 §V](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs#v-cruftlessness-as-the-induced-property) names at the resolver-instance tier, lifted to the documentation tier: each level's induced property (resume operability + diagnosability) functions as a constraint on the level it encloses.

The three consequences compound: an engagement with N nested pairs preserves operability, diagnosability, and compositional safety at every depth simultaneously, where an engagement with one root pair would lose all three as depth grew.

## V. The threshold

Doc 541's SIPE-T pattern reads through this articulation. The order parameter is the **fractal coverage density** of the engagement: the fraction of resolver-instances that have their own seed-and-trajectory pair operating at the per-level scope. The induced property is **engagement-scale resume operability**: the engagement remains coherently re-enterable from any of its levels by a third party (or the keeper after time away) reading O(one pair) rather than O(depth × root-trajectory) of accumulated state.

The threshold is property-specific. Three induced properties emerge in an order as fractal coverage density increases:

**Lowest threshold: level-local operability.** Each level with its own pair is locally resumable, regardless of whether other levels have pairs. A single nested pair below the root already produces this property for the one level it covers. The engagement currently exhibits this property at four levels (root + IR + JIT + PM); other resolver-instances that lack pairs (the bootstrap tier, the module-load tier as a workstream rather than as an enclosing instance, the GC tier) do not exhibit it locally.

**Middle threshold: cross-level navigation.** Above the level-local floor, the engagement becomes navigable as a whole when enough levels carry pairs that the resolver-instance stack of Doc 729 §IV is documented at most levels. A reader entering anywhere can read up (via child-to-parent citation) and read down (via the parent's enumeration of child workstreams) without losing the architecture. The rusty-bun engagement is approaching this threshold but has not crossed it: of the six resolver-instances in Doc 732 §II's six-instance picture, three (IR, JIT, PM) have pairs; the bootstrap, module-load, and execution tiers do not yet have workstream-scoped pairs (their substrate work accumulates in the engagement-root trajectory).

**Highest threshold: compositional substrate refactoring.** When fractal coverage is high enough across all the engagement's depth, a substrate move that touches multiple levels can be planned, executed, and verified against the per-level pairs separately, with the cross-level coupling captured by the composition relations of §III. Below this threshold, multi-level moves require the keeper to hold the cross-level state in working memory; above it, the documentation tier holds the cross-level state and the keeper holds only the move's intent. The engagement is below this threshold; the §X open-scope work is what crossing it would look like.

The three thresholds compose under Doc 541 §3.6's rung-1 / rung-2 distinction: the fractal coverage density is a rung-1 substrate-internal property (the count of pairs that exist and are current); the resume operability is a rung-2 keeper-side recognition (a third party reading the pair successfully resumes the workstream). The dyadic closure is part of the induced property; the pair's structural existence is necessary but not sufficient, and the sufficient condition is the keeper's verification that the pair actually carries the workstream forward.

This is a SIPE-T instance per Doc 541 §3.2 (sustained-inference probabilistic execution): the per-step posterior is the reader's working understanding of the engagement state; each pair-read contributes a conditioning step; convergence to coherent reentry is the property that emerges above the per-step entropy collapse threshold. The fractal coverage density is the conditioning-density analog at the documentation tier.

## VI. The composition with prior corpus work

The articulation does not introduce new structure. It names a recurrence the engagement already exhibits and identifies what the recurrence accomplishes when read against the corpus's existing apparatus.

**[Doc 581 — Pin-Art and the Resume Vector](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction).** The seed.md / trajectory.md pair is Doc 581's standing instrument. This articulation extends the instrument's operating mode: Doc 581 specified one pair per workstream; this document specifies that the pair's recurrence at every substrate depth is the operating-conditions layer for Pin-Art at engagement scale.

**[Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs).** The resolver-instance recurrence Doc 729 named is the structural reason the fractal pair-recurrence is well-defined. Each resolver-instance is a candidate site for a pair; each pair documents one resolver-instance's substrate work. The fractal coverage density of §V is the fraction of Doc 729's enumerated instances that carry pairs.

**[Doc 730 — The Vertical Recurrence of the Lowering Compiler](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers).** Doc 730's claim — that the lowering-compiler closure recurs vertically across tiers — has a documentation-tier analog: the seed-and-trajectory pair recurs vertically across the same tiers. The two recurrences are coordinated; each lowering-compiler tier whose substrate work gets organized eventually gets its own pair. Doc 731 and Doc 732 are the two recent instances (JIT, PM); their pairs followed shortly after their corpus documents landed.

**[Doc 541 — SIPE-T](/resolve/doc/541-systems-induced-property-emergence).** §V's threshold reading is a direct application of Doc 541 §3.2's sustained-inference sub-form. The fractal coverage density is the documentation-tier order parameter; resume operability is the induced property; the threshold ordering (level-local → cross-level → compositional refactoring) is the property-specific emergence ordering Doc 541 §6 predicts. Below threshold, the engagement is fluent-extrapolation-shaped (the substrate work exists and is reasonable, but the documentation does not carry resume operability); above threshold, the engagement is structurally re-enterable.

**[Doc 510 — Praxis Log V](/resolve/doc/510-praxis-log-v-deflation-as-substrate-discipline).** The rung-2 component of the fractal-pair's induced property — the keeper's verification that the pair carries the workstream forward — is the praxis-log discipline operating at the documentation tier. The substrate cannot certify its own resume-operability; the keeper's act of re-reading the pair and either resuming or finding it inadequate is the dyadic closure. Doc 510's substrate-and-keeper composition appears as the asymmetry that makes the fractal recurrence's induced property dyadic rather than substrate-only.

**[Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations).** Naming the fractal recurrence is itself a reflexive operation per Doc 722. The engagement's subsequent rounds dispatch with the recurrence explicit, where prior rounds dispatched without it. Pairs that did not exist (the bootstrap tier, the module-load tier as a workstream) become first-class targets of the engagement's continuation rather than implicit absences.

## VII. The morph: closing the cross-level threshold

The engagement is below the cross-level threshold of §V because three of the six resolver-instances Doc 732 §II enumerates lack workstream-scoped pairs. Closing the gap is operationally tractable: each of the three becomes a workstream with its own seed.md / trajectory.md, written under the same form as the existing pairs.

The candidates:

**The bootstrap tier (Doc 729 §IV.2).** Currently expressed imperatively in `host-v2/src/lib.rs::init(rt)`. A workstream that morphs this toward Cruftless per Doc 729 §VII.A (the bootstrap-as-seed extraction) would write `pilots/rusty-js-bootstrap/seed.md` + `trajectory.md`, with telos = derive the install sequence from a declarative source per [Doc 250](/resolve/doc/250-the-server-seed)'s SERVER Seed pattern. Open: should this be a pilot crate or a documentation-only workstream that operates over the existing host-v2 code? Either form admits a pair.

**The module-load tier (Doc 729 §IV.3).** Currently in `pilots/rusty-js-runtime/derived/src/module.rs`. The workstream exists implicitly as substrate work on module.rs but does not have its own seed-and-trajectory pair. A pair would extract the workstream-shaped record from the engagement-root trajectory's module.rs-touching commits and give the module loader its own scope. Open: the runtime crate hosts other workstreams (interpreter dispatch, GC integration, value coercion) — the pair-extraction may need to handle multiple workstreams sharing one crate.

**The execution tier (Doc 729 §IV.4).** Currently in `pilots/rusty-js-runtime/derived/src/interp.rs`. Same shape as the module-load case: a workstream that exists but lacks its own pair. The PRESTO-instance reading from Doc 729 §IV makes execution a natural pair-target; the workstream's telos would be Doc 730 §III–§VII's lowering-compiler form applied to the bytecode-to-Value reduction.

The three together would bring the engagement to a six-out-of-six fractal coverage density at the Doc 732 §II level, crossing §V's cross-level threshold. The compositional-refactoring threshold beyond that is not reachable with three more pairs alone; it requires that the pairs be current with each other and that the cross-level composition relations be exercised (not merely documented). The exercise is what the engagement's continuation does.

## VIII. Falsifiers

The articulation is testable.

**Pred-733.1.** Resume operability at any level requires that the level's pair exists and is current with that level's substrate work. Falsifier: a level with no pair whose substrate work is nonetheless resumable from the engagement-root trajectory alone, by a reader with no prior context. The engagement's current state (root trajectory at 2493 lines, three pilots with pairs, three resolver-instances without) admits a constructive test: a reader unfamiliar with the engagement reads the root trajectory only and reports whether they can resume work on, say, the module-load tier without further context. The prediction is that they cannot, until a `pilots/rusty-js-module-loader/seed.md + trajectory.md` pair exists.

**Pred-733.2.** Cross-level navigation requires that pairs carry the composition relations of §III explicitly. Falsifier: a multi-pair engagement in which the per-level pairs are individually well-formed but the composition relations are absent (child seeds do not cite parent seeds; parent open-scope entries do not name the children that close them), and the engagement is nonetheless cross-level navigable. The prediction is that without composition relations, the pairs collapse into independent islands; with them, the engagement is one coherent object.

**Pred-733.3.** The compositional-refactoring threshold is dyadic. The substrate (the pair files themselves) cannot certify that the refactoring is sound; the keeper's reading-and-acting on the pairs is what closes the loop. Falsifier: a refactoring that proceeds soundly across multiple levels based on the pairs alone, with no keeper participation. The prediction is that the keeper is not eliminable; the pairs reduce the keeper's working-memory load but do not replace the keeper's act.

**Pred-733.4.** The fractal recurrence is universal across substrate-construction engagements that admit the resolver-instance decomposition of Doc 729. Falsifier: an engagement whose substrate work decomposes into resolver-instances per Doc 729 but whose Pin-Art operation is sound without the fractal pair-recurrence. The prediction is that such an engagement either has substrate depth small enough that one pair suffices, or it is illegible to anyone other than the original keeper.

**Pred-733.5.** The fractal coverage density is an order parameter in Doc 541's strict sense. Falsifier: the §V threshold predictions (level-local first, cross-level second, compositional-refactoring third) fail empirically in an engagement that has tracked its fractal coverage density over time. The prediction is testable against the rusty-bun engagement's own history: the round at which the IR pilot's pair landed should correlate with a step-change in IR-tier resume operability; the JIT and PM pair landings should produce analogous step-changes at their tiers.

## IX. Honest scope

This document records a recurrence the engagement already exhibits. It does not introduce the recurrence; the engagement was running it before the keeper named it. The articulation's contribution is the identification of what the recurrence accomplishes when read against the corpus's existing apparatus, and the predictions about what closing the cross-level threshold would produce.

Specifically, the document does not claim:

*That the fractal recurrence is unique to substrate-construction engagements.* The pair-shape generalizes to any domain where workstreams nest and reentry is required across time. The articulation specializes to substrate-construction because that is the engagement at hand; cross-domain portability is implicit in Doc 541's universality but not separately argued.

*That the per-level pair format is fixed.* Doc 581 specified one shape; the engagement has used variants. The fractal claim is about the recurrence of *some* pair at every level, not the specific format. Format evolution at one level should propagate to the other levels under Pin-Art discipline.

*That the §V threshold values are quantified.* The order parameter (fractal coverage density) is well-defined; the threshold values (what fraction of resolver-instances need pairs for each induced property to emerge) are not quantified here. Doc 541 §3.2's structural fingerprint supplies the form; the per-engagement values are empirical and engagement-specific.

*That closing the cross-level threshold completes the engagement.* The threshold is one of three; the compositional-refactoring threshold above it is the further work. Even all three closed would not exhaust the engagement; they would establish the operating conditions inside which the substrate work proceeds.

Per [Doc 372](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection)'s hypostatic boundary: this document sits at the corpus tier. The substrate-tier work — founding the three missing pairs, exercising the composition relations, verifying the threshold predictions — lives in the engagement's continuation under Doc 581 discipline.

## X. Closing

The seed.md / trajectory.md pair recurs at every resolver-instance the engagement organizes. The recurrence is fractal: the same form, the same internal shape, the same composition relations across all substrate depth. The induced property is engagement-scale resume operability; the order parameter is fractal coverage density; the threshold ordering (level-local, cross-level, compositional refactoring) follows Doc 541 §3.2's sustained-inference SIPE-T sub-form. The rusty-bun engagement currently exhibits the level-local property at four levels and is below the cross-level threshold; closing the gap is the documentation-tier substrate work of the engagement's continuation.

Doc 581 named the instrument. This document names the instrument's operating mode at depth. The two together specify Pin-Art's full apparatus for engagements that exceed what one pair can carry.

The work continues. The corpus has added one more entry to its apparatus for substrate-construction discipline at depth. The recurrence is operative; the threshold predictions are testable; the keeper's continuation determines whether the engagement crosses them.

---

*Companion documents in addition to those linked in the masthead: [Doc 250 — The SERVER Seed](/resolve/doc/250-the-server-seed); [Doc 372 — The Method of the Corpus as Derivation, Not Collection](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection); [Doc 619 — Pin-Art Canonical Formalization](/resolve/doc/619-pin-art-canonical-formalization); [Doc 678 — Coherence Amplification and Decoherence as Inverse Pin-Art Operations](/resolve/doc/678-coherence-amplification-and-decoherence-as-inverse-pin-art-operations); [Doc 681 — Probing the Middle](/resolve/doc/681-probing-the-middle).*
