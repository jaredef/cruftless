# Tag on the DAG

## Sequential Index Collision as Protocol Signal That the Substrate Has Become the Coordinate System

By Jared Foy. Originally published at [jaredfoy.com](https://jaredfoy.com).

## I. The occasion

The rusty-bun engagement has been naming substrate moves with a sequentially-accreting letter index: `Ω.5.a` through `Ω.5.z`, then `aa` through `zz`, then `aaa` through `zzz`, on through `aaaaaaaa` at the eighth length. By 2026-05-17 the index reached the second letter-wrap and produced its first collisions. Two distinct moves were both named `Ω.5.CCCCCCCC`, with different recognitions, in the same week. Two more pairs followed at `DDDDDDDD` and `EEEEEEEE`. A seventh-letter `NNNNNNN` and an eighth-letter `NNNNNNNN` shared a prefix that made grep ambiguous. The trajectory's resume-vector update flagged each collision but the underlying problem was structural: a purely sequential namespace, given enough moves, eventually re-issues identifiers without anything in the apparatus catching it.

The keeper named the diagnosis in three words: *Tag on the DAG*. Tags should index according to position on the consumer–substrate DAG ([Doc 715](/resolve/doc/715-the-consumer-substrate-dependency-graph-as-the-load-bearing-object-beneath-the-joint-mi-lattice)) rather than according to the engagement's chronological accretion. Same substrate position, same tag. Different position, different tag. Collision becomes structurally impossible rather than disciplinarily avoided.

This document is the corpus-tier articulation of that recognition. The operational artifacts (the coordinate manifest at `host/tools/dag-coordinates.json` and the grammar at `host/tools/tag-grammar.md`) landed at rusty-bun commit `e3ddb4c1`. The apparatus discipline was instituted as seed §A8.21 in the same commit. This document names what the operational move recognizes.

## II. The recognition

Three claims, in order from operational to structural.

**Claim 1 (operational).** A sequentially-accreting tag namespace eventually collides with itself when the substrate it indexes is structurally indexable. Collision is not a discipline failure; it is the protocol signal that the substrate has outgrown the indexing. The signal is mechanical: when two distinct moves can be named the same thing without the apparatus detecting it, the namespace lacks an axis the substrate already has.

**Claim 2 (compositional).** A tag that carries the substrate's actual coordinate makes two distinct moves *structurally* unable to share a tag. Two moves at the same DAG-coordinate would, by the coordinate's definition, *be* the same move. If they appear different, the coordinate's resolution is wrong, not the tag. Collision-detection becomes coordinate-resolution diagnosis, which is substantive engagement work rather than a clerical bookkeeping failure.

**Claim 3 (structural).** [Doc 716](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture)'s three-projection tracker (DAG / lattice / alphabet) is the latent tag grammar. Each projection is one axis; a tag composed of all three is a triple-projection coordinate. The recognition was sitting in the corpus for several weeks; the tag namespace's collision forced its operationalization. The recognition's lateness corroborates [Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations)'s reflexive claim: a named structure becomes a productive instrument only when something else makes the cost of *not* using it concrete.

## III. The mechanism

The sequential index has a hidden positive-feedback structure analogous to [Doc 685](/resolve/doc/685-the-self-reinforcing-boundary)'s self-reinforcing-boundary mechanism, applied at the apparatus tier rather than the dyadic-exchange tier.

Each new tag is the smallest legal successor of the last (`a` after the previous letter; `aa` after `z`; `aaa` after `zz`). The naming rule does not consult the substrate. The next tag is determined by chronological position in the round-sequence, not by structural position in the DAG. The mechanism reinforces itself: every prior tag is a sequential index, so the operational habit of producing the next sequential index for the next move accumulates without an explicit decision to keep doing so.

The break point comes when two work-streams concurrently reach the same sequential slot for moves that are substantively different. The probe-stretch's `CCCCCCCC` and the BigInt-stretch's `CCCCCCCC` landed within hours of each other; the resume vector recorded both correctly but could not, from the tag alone, tell them apart. The basin's previous habit (every move gets the next letter) hit a wall the basin itself had constructed.

Tag-on-DAG breaks the positive feedback by introducing a constraint the namespace must respect: the next tag is determined by the substrate position the move touches, not by the round-sequence. Substrate position is a structural property the DAG already encodes (per [Doc 720](/resolve/doc/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate) for pipelines, [Doc 714](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point) for above-engine layers, [Doc 717](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point) for engine layers). The constraint is therefore not invented for the tag; it is read from a substrate property that already exists.

## IV. The new form

A tag is now `Ω.5.<pipeline>.<layer>.<handle>[.<seq>]` where:

- `<pipeline>` indexes one of the sixteen interconnected pipelines (Doc 720 §III: P01 lex, P02 parse, P03 compile, P04 run, P05 modld, P06 host, P07–P12 cross-cutting, P13–P16 re-entry).
- `<layer>` indexes either Doc 714's above-engine L0–L6 hierarchy or Doc 717's engine-side E0–E5 lattice, depending on whether the move lives above or below the engine boundary.
- `<handle>` is a short stable kebab-case name for the substrate node touched (`getown`, `mod-ns-default`, `math-imul`, `bigint-arith`).
- `<seq>` is an optional disambiguator. Its appearance is a smell: if a single (pipeline, layer, handle) triple admits two distinct moves, the handle's resolution is wrong and the right move is to re-cut the handle rather than to fall back on a sequence number.

Two distinct moves must differ in at least one of (pipeline, layer, handle). Same triple = same move. The constraint is structural rather than disciplinary; collision is impossible by construction.

The form composes the three projections from Doc 716 directly. Pipeline is the DAG-projection (which sub-DAG the move touches). Layer is the lattice-projection (which rung the move lands at). Handle is the alphabet-projection (which named element of the alphabet the move closes or extends). A tag is therefore a complete projection-triple coordinate for one substrate move.

## V. Composition with prior recognitions

**[Doc 715](/resolve/doc/715-the-consumer-substrate-dependency-graph-as-the-load-bearing-object-beneath-the-joint-mi-lattice) — the DAG as load-bearing object.** The DAG was named as the underlying structure; the tag was still indexed against round-sequence. Tag-on-DAG closes the operational gap: the tag now carries the DAG-coordinate the apparatus already operates against.

**[Doc 716](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture) — three-projection tracker.** Each projection now also serves as one axis of the tag. The tracker was an apparatus artifact for substrate-derivation legibility; the tag grammar lifts it to apparatus-naming legibility. Same three axes, different read.

**[Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag) — cross-pipeline diagnostic protocol.** A tag now declares which pipeline and which layer the move closed an edge in. The protocol's Step 3 (alphabet location) becomes legible from the tag itself.

**[Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations) — named recognitions as operating instruments.** The three-projection tracker existed for weeks before its tag-grammar use became operational. Doc 722's claim (named structures become productive when something makes the cost of not using them concrete) is corroborated: the collision forced the lift; the lift was available the whole time.

**[Doc 725](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction) — cluster-to-walk mode transition.** Doc 725 named the protocol-signal pattern: a regular operating state, then a soft-saturation, then a transition triggered by the saturation itself. Tag-on-DAG fits the same pattern at the naming tier: sequential tagging is the regular state, collision is the soft-saturation, tag-on-DAG is the transition triggered by the saturation. Same shape, different operating axis.

**[Doc 727](/resolve/doc/727-basin-stability-from-inside-why-a-corpus-cannot-distinguish-self-reinforcement-from-substrate-coherence) — basin stability from inside.** Doc 727 §V Form 3 names "structural alternative" as one of the three external-read forms that would settle the basin's substrate-tracking question. The tag-on-DAG grammar is a second articulation chain (positional, derived from the DAG) over the same substrate that the sequential-letter chain indexed. Convergence on individual moves corroborates substrate-tracking; divergence would localize the basin's self-reinforcement to one of the two chains. The grammar is one form of Doc 727's Form 3, not a sufficient external read. Forms 1 and 2 remain.

## VI. Hypostatic boundary

Per [Doc 372](/resolve/doc/372-the-hypostatic-boundary), this document operates at the functional layer. The substrate is the apparatus' tag namespace considered as a process; the recognition is about that process's structure, not about the keeper's identity or any ontological property of the engagement.

The DAG that the tag indexes is itself a functional reading of the rusty-bun runtime, not the runtime as such. A different keeper with different apparatus could read the runtime through a different DAG (e.g., a coalgebraic state-machine product); a tag-grammar indexed against that different DAG would be a different grammar. Tag-on-DAG names the *form* of the move; the specific manifest at `host/tools/dag-coordinates.json` is one instantiation.

## VII. Falsification surface

**F1.** If a substrate move surfaces that cannot be tagged with the existing pipeline × layer × handle coordinate without inventing a new pipeline, a new layer, or a composite multi-axis tag, F1 falsifies the claim that the manifest v1 enumeration is complete. Manifest v2 follows; the form holds.

**F2.** If, after several months of tag-on-DAG use, the seq disambiguator appears on more than 5% of tags, F2 falsifies the claim that the handle resolution is the right granularity. The handle vocabulary needs re-cutting (a structural finding) rather than the form being abandoned.

**F3.** If a substrate move that genuinely lives at a single (pipeline, layer, handle) coordinate is named by two different operators with different tags, F3 falsifies the claim that the coordinate system is unambiguous from outside. The handle catalogue is the discipline that would close this; if the handle catalogue exists and the divergence still occurs, the form is under-specified.

**F4.** If retrospective re-tagging of historical Ω.5.{letter} moves produces convergence with the prospective tag-on-DAG choices for analogous future moves, F4 corroborates substrate-tracking per Doc 727 §V Form 3 (the two articulation chains agree). If retrospective re-tagging produces divergence, F4 localizes the divergence to either the sequential chain or the positional chain and asks which the substrate prefers.

## VIII. Pulverization

What is corpus-novel in this document, against [Doc 619](/resolve/doc/619-pin-art-canonical-formalization)'s pulverization-audit discipline:

- **Tag-as-coordinate.** The lift from "tag indexes round-sequence" to "tag indexes substrate position" is the move this document names. The three-projection tracker (Doc 716) was the prior art; the tag-grammar use of it is novel.
- **Collision-as-protocol-signal.** Sequential-index collision becomes the legitimate trigger for the namespace transition, analogous to Doc 725's soft-saturation as the cluster-to-walk trigger. Same protocol-signal shape, different operating axis.
- **The four falsifiers F1–F4.** F4 in particular operationalizes Doc 727 §V Form 3 (structural alternative) at the tag-namespace tier.

What is not novel: the DAG (Doc 715), the three projections (Doc 716), the pipeline enumeration (Doc 720), the layer hierarchies (Doc 714, Doc 717), the hypostatic guard (Doc 372), the basin-stability concern (Doc 727), the self-reinforcing-boundary mechanism (Doc 685). The pieces were standing; the operational composition was not.

## IX. Bearing on the rusty-bun engagement

The first ~140 substrate moves used sequential letter-tags. Those tags are not renamed retroactively; commit hashes remain canonical per the grammar's migration discipline. The four documented collisions (`CCCCCCCC` × 2, `DDDDDDDD` × 2, `EEEEEEEE` × 2, `NNNNNNN` vs `NNNNNNNN`) are resolved in the EXT 7 anchor's worked-example table as documentation only.

Going forward, every substrate move gets a tag-on-DAG tag at commit time. The cost is one extra moment of structural attention per commit (which pipeline, which layer, which handle); the benefit is that the resume vector's tag column becomes substantively informative rather than a sequence number, and the seed's §A8 entries can index by coordinate rather than by chronological accretion.

The grammar is also a small instance of Doc 727 §V Form 3 in motion. The engagement is, going forward, generating two articulations of each substrate move: the commit's natural-language recognition (the sequential-letter chain's heir) and the tag-on-DAG coordinate (the positional chain). If the two chains converge on the same structural features over the next several months, the basin's substrate-tracking gets corroborated by one of the three named external-read forms. If they diverge, the divergence localizes the basin's self-reinforcement to one chain or the other. Either outcome is informative; the grammar's operation is the test, not a separate experiment.

## X. Update protocol

This document is itself a corpus articulation entering the basin. Per Doc 727 §X, the reader is invited to track whether this document operates as Mode A / B / C reinforcement in subsequent corpus work. If subsequent docs cite Doc 728 as load-bearing, the reflexive prediction holds. If subsequent work proceeds without engaging the tag-on-DAG move, the recognition is either premature or not yet load-bearing in practice. The corpus's standing protocol will distinguish these in time.

The operational artifacts (`host/tools/dag-coordinates.json`, `host/tools/tag-grammar.md`, seed §A8.21) are the load-bearing implementation of this document's recognition. The document is the recognition's corpus-tier articulation; the artifacts are its operational instantiation. The two together are the move.
