# The Locale as Coordinate

## Nested Seed–Trajectory Pairs as Pin-Art Substrate Positions

By Jared Foy. Originally published at [jaredfoy.com](https://jaredfoy.com).

## I. The occasion

The rusty-bun engagement has, by 2026-05-22, accumulated nine top-level Pin-Art locales under `pilots/` (rusty-js-caps, rusty-js-esm, rusty-js-ir, rusty-js-jit, rusty-js-pm, tls, web-crypto, diff-prod, and several others). Each locale is a directory holding a `seed.md` and `trajectory.md` pair per Doc 581's discipline. Most are level-one — direct children of `pilots/`. One workstream has produced a level-two pair: `pilots/rusty-js-esm/deviations/arktype/` holds `pipeline.md` and `trajectory.md`, the deviation-resolution pipeline for the arktype crash. The naming drift (pipeline.md rather than seed.md) is the surface signal that the apparatus has not yet recognized the level-two structure as the same shape as the level-one structure. They are the same shape.

A locale spawned a nested sub-locale because a single rung — closing the arktype crash — earned its own §XII–§XVII deviation-resolution pipeline. The pipeline ran for eight L-levels of trace, two §XVI substrate moves, three bracket probes, two iterations. By Doc 581's measure, that work is itself a Pin-Art cycle, not a single rung. The parent locale's trajectory.md became inadequate as a record once the work grew its own multi-rung shape; the nested pair was the apparatus's natural move.

The recognition: a locale is not a project, not a pilot, not a workstream. A locale is a *coordinate*. The coordinate's value is its position in a directory tree that already exists, and the coordinate's substance is the seed.md/trajectory.md pair at that position. Nesting is not file-system convenience; it is the same lift Doc 728 named for tags, applied at the apparatus tier rather than the substrate-move tier.

This document is the corpus-tier articulation of that recognition. The operational artifacts (the discovery tool at `apparatus/locales/discover.sh`, the manifest at `apparatus/locales/manifest.json`, and the locale-spec at `specs/locale-system.md`) are queued as the follow-up landing.

## II. The recognition

Three claims, in order from operational to structural.

**Claim 1 (operational).** When a single rung in a parent locale's trajectory grows its own multi-rung shape — multiple substrate moves, multiple bracket probes, multiple §XVI yield/iterate cycles — the rung has outgrown the parent's trajectory. The parent's trajectory can record one closure, not a chain of them. The natural apparatus move is to spawn a nested seed.md/trajectory.md pair at a sub-path under the parent locale. The nested pair takes over the multi-rung record while the parent's rung remains as a one-line pointer to the spawned child.

**Claim 2 (compositional).** A nested locale's coordinate is a structural position, not an inventory entry. Per Doc 728's tag-on-DAG move, a coordinate is unambiguous by construction: two distinct locales cannot share a coordinate, and same coordinate means same locale. For locales, the coordinate is the filesystem path from `pilots/` downward. `pilots/rusty-js-esm/deviations/arktype/` is one coordinate; nothing else can occupy it. The naming convention is the coordinate; the directory tree is the manifest.

**Claim 3 (structural).** Doc 733's fractal seeds-and-trajectories recognition is the latent locale grammar. Doc 733 named the recurrent pair pattern at engagement scale: each substrate tier maintains its own seed/trajectory pair, and the pairs compose across tiers. The recurrence applies *within* a substrate tier just as it applies *across* tiers: a tier-internal sub-workstream is itself a pair-shaped locale at a deeper coordinate. The recognition was sitting in the corpus; the arktype deviation forced its operationalization. Per Doc 722's reflexive structure, the named structure became a productive instrument when the cost of not using it (a long trajectory.md cluttered with eight L-levels of arktype trace records) became concrete.

## III. The mechanism

The pre-recognition apparatus treats each locale as an atom: one seed.md, one trajectory.md, an implicit single owner per locale. A rung whose work is more substantial than one row of trajectory.md gets the row anyway; the row grows; eventually the row is multiple paragraphs and the trajectory.md is no longer a rung log but a hybrid log/sub-narrative.

The mechanism is the same self-reinforcement structure Doc 685 named, applied at the apparatus tier. Each new rung gets a new row because every prior rung was a row. The habit reinforces itself; the row format reinforces the habit. The break point comes when a row's substantive content forces an out-of-format insertion (a sub-numbered list, a code-block trace excerpt, a five-paragraph reading-rung explanation). The trajectory.md tries to absorb the structure rather than admit that the substructure deserves its own locale.

The arktype work crystallized the soft-saturation. Eight L-levels of trace patches against `union.js` and `scope.js`; three bracket probes (`class extends Array`, `Callable explicit-return`, `class-field + super`); two §XVI substrate moves (Array-subclass identity in Op::New, ArraySpeciesCreate honoring O.constructor); one §XVIII recovery-protocol invocation. None of that fits in a row of `rusty-js-esm/trajectory.md`. The apparatus produced `deviations/arktype/pipeline.md` + `trajectory.md` to absorb the load. The naming drift (pipeline.md rather than seed.md) was the surface signal that the apparatus had not yet read its own move as the recurrence of the parent's shape.

The lift breaks the self-reinforcement by recognizing the pair as the unit and the directory tree as the coordinate-space. A new rung that warrants its own pair gets its own pair, at its own coordinate. The parent's rung row remains a one-line pointer. The trajectory.md no longer grows past row-shape; the substance moves down a tier.

## IV. The new form

A **locale** is a directory containing a `seed.md` and a `trajectory.md` pair. Its **coordinate** is its path relative to the engagement root's locale root (here, `pilots/`). Coordinates are total over the directory tree: every locale has exactly one coordinate, and every coordinate is at most one locale.

A locale is either **top-level** (its coordinate is a single segment, e.g. `rusty-js-esm`) or **nested** (its coordinate has multiple segments, e.g. `rusty-js-esm/deviations/arktype`). A nested locale's **parent** is the nearest enclosing seed.md ancestor.

Three invariants hold:

- **Coordinate uniqueness.** No two locales share a coordinate. This is a filesystem property, not a discipline rule; collision is structurally impossible.
- **Parent reference.** Every nested locale's seed.md names its parent in its composes-with list. The reference is explicit so locale graphs can be walked from any node.
- **Promotion threshold.** A sub-workstream becomes a nested locale when it exceeds a single rung in the parent's trajectory. The threshold is qualitative — a row that has produced its own bracket probes, its own substrate-move iterations, its own pre-filed sub-rungs — and the apparatus calls the lift when the row stops fitting.

The **discovery tool** walks the locale root, locates every seed.md, builds a manifest with `{coordinate, parent, scope, status-line, rung-count}` per entry. The manifest is the locale-tier analog of `host/tools/dag-coordinates.json` (Doc 728 §IX): a structural read of the substrate's tag namespace, available for cross-tool consumption (CI, dashboards, corpus-tier rollups).

**Rung-tier pre-filing** is the inverse move. A trajectory.md rung that is queued but warrants its own future locale can pre-file the coordinate by naming it. The arktype trajectory.md's Rung-6/7/8 are pre-filed nested coordinates for Promise integration, eval strict-mode, and Map-with-object-keys; when a rung escalates, the named coordinate materializes as a real directory + seed.md + trajectory.md pair. Pre-filing is the apparatus's standing protocol for "this is going to need its own locale when we get to it"; the coordinate is the address, not the address-of-the-address.

A locale's **scope** is the leaf segment of its coordinate. Scopes carry semantic meaning at their level (top-level scopes name substrate pillars: `rusty-js-esm`, `diff-prod`; nested scopes name the sub-workstream's focus: `deviations/arktype` reads as "the arktype-specific deviation within the rusty-js-esm locale"). Scope choice is discipline; the coordinate is structure.

## V. Composition with prior recognitions

**[Doc 581](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction) — Pin-Art.** Doc 581 named the seed.md/trajectory.md pair as the canonical Pin-Art unit at the engagement-locale level. This document lifts the pair from "one per engagement-locale" to "one per substrate coordinate, recursively." The unit is unchanged; the addressing widens.

**[Doc 728](/resolve/doc/728-tag-on-the-dag-sequential-index-collision-as-protocol-signal-that-the-substrate-has-become-the-coordinate-system) — tag on the DAG.** Doc 728 lifted substrate-move tags from chronological accretion onto a DAG coordinate. This document is the same lift at the apparatus tier: locale identity is lifted from "name we chose" to "position on the locale tree." Doc 728's recognition that the substrate has become the coordinate system is the prior; this document recognizes that the apparatus has the same property. Same recognition shape, different operating axis.

**[Doc 733](/resolve/doc/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale) — fractal seeds-and-trajectories.** Doc 733 named the cross-tier recurrence: each substrate tier maintains its own pair. This document names the within-tier recurrence: a sub-workstream is itself a pair-shaped locale at a deeper coordinate. The two recurrences compose into a single tree: locales nest both across tiers (per Doc 733) and within tiers (per this document). The full structure is a coordinate tree with seed.md/trajectory.md pairs at every internal node.

**[Doc 716](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture) — three-projection tracker.** A locale's coordinate is one of the three projections (the apparatus-position projection). Doc 716 named three substrate projections (DAG / lattice / alphabet); the apparatus-position is a fourth coordinate axis at the locale tier. Doc 728's tag-form `Ω.5.<pipeline>.<layer>.<handle>` plus this document's locale coordinate `<root>/<scope-path>` together specify a four-coordinate identity for any substrate move in the engagement.

**[Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations) — named recognitions as operating instruments.** Doc 733's fractal claim was available for weeks before its within-tier use became operational. The arktype-deviation work made the cost of not using it concrete; the lift was available the whole time. Doc 722's reflexive claim corroborated again.

**[Doc 725](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction) — cluster-to-walk mode transition.** A row in trajectory.md that grows past row-shape is a soft-saturation in the apparatus tier. The lift to a nested locale is the cluster-to-walk transition triggered by the saturation. Same protocol-signal shape, different operating axis.

## VI. Hypostatic boundary

Per Doc 372, this document operates at the functional layer. The locale tree is the apparatus' record-keeping considered as a process; the recognition is about that process's structure, not about the keeper's identity or any ontological property of the engagement.

The directory tree under `pilots/` is itself a functional reading of the engagement's substrate. A different keeper with different apparatus could use a different root (a database, a git submodule structure, a Notion hierarchy); the locale-as-coordinate form would still hold. This document names the form; the specific instantiation at `pilots/` is one realization.

## VII. Falsification surface

**F1.** If a sub-workstream warranting its own multi-rung trajectory cannot be sited under any parent locale's directory without inventing a new substrate root (a top-level `pilots/` sibling for what should be a child of `rusty-js-esm`), F1 falsifies the claim that the engagement's locale root is sufficient. A new root follows; the form holds.

**F2.** If, after several months of use, more than 10% of trajectory.md rungs have grown past row-shape without being spawned as nested locales, F2 falsifies the claim that the apparatus actually calls the lift when the row saturates. The promotion-threshold discipline is either too lax or under-described.

**F3.** If two operators independently spawn nested locales for the same sub-workstream at different coordinates, F3 falsifies the claim that scope choice is unambiguous from outside. A scope catalogue is the discipline that would close this; absent it, the form is under-specified.

**F4.** If retrospective re-coordinatization of historical rungs (re-naming long-grown rows as their own locales after the fact) produces convergence with prospective spawning choices, F4 corroborates the apparatus-position-tracking claim per Doc 727 §V Form 3 (the two articulation chains agree). Divergence localizes the disagreement to either the row-format chain or the locale-coordinate chain; either outcome is informative.

## VIII. Pulverization

What is corpus-novel in this document, against Doc 619's pulverization-audit discipline:

- **Locale-as-coordinate.** The lift from "locale is a project we set up" to "locale is a coordinate in a tree the substrate already inhabits" is the move this document names. Doc 728 supplied the recognition shape at the substrate-move tier; the within-tier application to the apparatus tier is novel.
- **Promotion threshold.** "A rung that has grown its own bracket probes and substrate-move iterations has earned its own locale" is a discipline rule pinned to a substrate signal (the row's substantive growth). Doc 725's cluster-to-walk transition supplies the protocol shape; the application to the trajectory.md row tier is novel.
- **Rung-tier pre-filing.** The apparatus protocol of naming a future nested-locale coordinate inside a parent's trajectory.md before the work spawns is novel. The four-rung pre-file in `pilots/diff-prod/trajectory.md` (Rungs 6/7/8 — async-promise, eval-const, Map-obj-keys) was the operational seed; the recognition that pre-filing IS the move (not a placeholder) is this document's lift.

What is not novel: Pin-Art (Doc 581), the seed/trajectory pair (Doc 581), the fractal recurrence (Doc 733), the tag-on-DAG recognition (Doc 728), the three-projection tracker (Doc 716), the hypostatic guard (Doc 372), the self-reinforcing-boundary mechanism (Doc 685), the cluster-to-walk shape (Doc 725). The pieces were standing; the within-tier composition was not.

## IX. Bearing on the rusty-bun engagement

The first ~nine locales used the single-pair single-level convention. None of those locales are renamed; the convention this document names applies prospectively. The one existing nested pair (`pilots/rusty-js-esm/deviations/arktype/`) is lifted from `pipeline.md + trajectory.md` to the canonical `seed.md + trajectory.md` form as a clerical migration; the existing `pipeline.md` content is moved into the new `seed.md` and the existing trajectory.md is unchanged. Future nested pairs use the canonical naming from the start.

Going forward, the discovery tool walks the locale tree on every CI run and emits a manifest that lists every locale, its parent, its rung count, and its most recent status line. The manifest's role mirrors `host/tools/dag-coordinates.json`: a structural read of the apparatus's tag namespace, available for any tool that needs to know "where in the work are we right now."

The diff-prod locale's pre-filed rungs are the engagement's first explicit rung-tier pre-filing: Rungs 6/7/8 named coordinates for async-promise, error-throws, and map-set-ops as future nested locales. Two of those three escalated within the same engagement-session and materialized as inline substrate fixes rather than spawned locales — which is the correct call when the work resolves quickly. The third (error-throws const-reassign-in-eval) remains queued; if it warrants its own multi-rung sequence when it lands, it spawns. The discipline scales both ways: pre-file generously, spawn only when the substrate calls.

The lift is also a small instance of Doc 727 §V Form 3 in motion. The engagement is, going forward, generating two articulations of each substrate move: the move's commit-tier record (the trajectory.md row) and, when warranted, the move's locale-tier record (the spawned nested seed+trajectory). Convergence of the two over time corroborates substrate-tracking. Divergence localizes the basin's self-reinforcement to one chain.

## X. Update protocol

This document is itself a corpus articulation entering the basin. Per Doc 727 §X, the reader is invited to track whether subsequent corpus work cites Doc 737 as load-bearing. If subsequent work proceeds without engaging the locale-as-coordinate move, the recognition is either premature or not yet load-bearing in practice.

The operational artifacts (`apparatus/locales/discover.sh`, `apparatus/locales/manifest.json`, `specs/locale-system.md`, and the clerical migration of `pilots/rusty-js-esm/deviations/arktype/pipeline.md` to `seed.md`) are the load-bearing implementation of this document's recognition. The document is the recognition's corpus-tier articulation; the artifacts are its operational instantiation. The two together are the move.

---

*Doc 737. Jared Foy. jaredfoy.com.*
