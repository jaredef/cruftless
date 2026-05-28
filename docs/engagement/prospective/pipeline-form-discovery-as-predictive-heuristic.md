# Pipeline-form discovery as predictive heuristic: mouth/terminus shape, input/emission correspondence, and the DAG/Lattice/Alphabet relation between pipelines

**Status**: prospective draft, awaiting keeper review for corpus promotion. Authored 2026-05-28 per keeper directive Telegram 10148. Target corpus number Doc 745 or later (after Doc 744 candidate substrate-shaped-work-discipline.md).

**Composes with**: Doc 540 (Pin-Art apparatus formalization), Doc 581 (resume-vector discipline), Doc 715 (consumer substrate dependency graph), Doc 720 (runtime as DAG of interconnected pipelines), Doc 729 (resolver-instance pattern), Doc 730 (vertical recurrence of the lowering-compiler closure), Doc 731 (JIT as lowering-compiler tier; alphabet purity), Doc 734 (meta-resolution pipeline), Doc 739 (single-tier cascade-revival), Doc 740 (multi-tier cascade-revival), Doc 741 (multi-tier cascade pipeline empirical materialization), Doc 742 (resolver-instance boundary contract), `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`, `apparatus/docs/standing-rule-13-prospective-application.md`, `docs/engagement/prospective/substrate-shaped-work-discipline.md` (Doc 744 candidate).

**Empirical anchor**: rusty-js-ir locale TDZ session (2026-05-27 + 2026-05-28; EXT 20-40 across 21 rungs; findings.md Addendum XVI). Three complete rule-13 trajectories whose closure each landed via deeper-layer substrate work after the first round's negative result (EXT 25→26, EXT 29→34, EXT 38→39→40). Each trajectory's residual-narrowing at successive rounds — design gap → emission-site gap → timing edge — discloses the pipeline's mouth-to-terminus shape progressively rather than at the spawn moment.

---

## I. Thesis

Every substrate-shaped problem implies a **resolution pipeline**: a substrate-tier-spanning trajectory from an **input shape** (the problem's mouth — what is asked of the engine) to a **terminal emission shape** (the pipeline's terminus — what the engine must produce in spec-correspondence). The pipeline's internal trajectory is the sequence of substrate steps that transform the input shape to the emission shape across tiers; its **interior contour** is the set of intermediate-tier value shapes the trajectory passes through.

Three claims follow:

**Claim 1 (shape correspondence)**: the input shape and the terminal emission shape jointly determine the pipeline's interior contour. A pipeline whose mouth and terminus are correctly stated has a unique-up-to-implementation-freedom interior; a pipeline whose mouth and terminus disagree (or are mis-stated) has no consistent interior and surfaces as a regression class until the disagreement is named.

**Claim 2 (regression-as-pipeline-discovery)**: regressions and negative substrate-introduction results are *not* failure events to be repaired — they are **pipeline-discovery events** that disclose previously-implicit pipeline contours. A regression names an implicit constraint at a specific interior point; iterated regression names successive interior points until the pipeline's full contour is revealed. The deeper-layer-closure trajectory (rule 13) is the discipline that *consumes* these discovery events without losing the substrate prefix they justify.

**Claim 3 (pipeline-to-pipeline relation via DAG/Lattice/Alphabet)**: pipelines interact via one of three relational forms, each appropriate to a different shape of substrate dependency: **DAG** (strict tier-ordering when one pipeline's terminus is another's mouth), **lattice** (meets and joins when pipelines share substrate tiers but with distinct mouth-terminus pairs), **alphabet exchange** (when pipelines occupy the same tier but exchange typed primitives at a shared boundary). The relational form is discoverable from the pipelines' mouth-terminus shapes; choosing the wrong form produces the same regression class as mis-stating a single pipeline's mouth-terminus.

The composition of these three claims is a **predictive heuristic**: given a substrate-shaped problem statement, the engagement can derive the implied pipeline's mouth, terminus, interior contour, and relation to neighboring pipelines before committing substrate work — and can predict which regression classes will surface if any of the four are mis-stated.

This articulation elevates the engagement's operational discipline (standing rules 1-26; substrate-shaped-work pipeline per Doc 744 candidate) from procedural ("we follow this sequence") to methodological ("the sequence is discoverable from the pipeline shape implied by the problem's statement").

---

## II. The substrate-shaped pipeline as a form

### II.1 Mouth, terminus, interior

A **substrate-shaped pipeline** is a 4-tuple `(M, T, I, R)`:

- **M (mouth)** — the input shape the pipeline consumes. For language-substrate pipelines, M is typically a sub-set of ECMA-262's behavioral surface (a syntactic form, a runtime call shape, an instantiation event). For meta-pipelines (Doc 734), M is the apparatus event class consumed (an observation, a directive, a measurement).
- **T (terminus)** — the emission shape the pipeline must produce. For language-substrate pipelines, T is the spec-mandated artifact (bytecode emission, runtime value, side-effect ordering, error class). For meta-pipelines, T is the discipline artifact (a finding, a standing rule, a chapter close).
- **I (interior contour)** — the ordered sequence of intermediate-tier value shapes the trajectory passes through. Each interior point `I_k` is a substrate-tier-typed shape with constraints inherited from prior `I_<k` and obligations propagating to subsequent `I_>k`.
- **R (relations)** — the set of relational edges to neighboring pipelines per the DAG/Lattice/Alphabet heuristic of §IV.

A pipeline is **well-formed** when M, T are explicitly stated and I is derivable from M, T plus the engagement's substrate-tier alphabet (Doc 730 + Doc 731). A pipeline is **mis-stated** when any of M, T, I are implicit; mis-statement surfaces as the regression class described in §III.

### II.2 Why mouth and terminus jointly determine interior

The lowering-compiler recurrence (Doc 730 §III) makes each substrate tier's alphabet a *typed* primitive set: tier N's alphabet `A_N` is the set of expressible operations at that tier. A pipeline whose mouth is in `A_N` and whose terminus is in `A_M` (M < N typically; downstream tiers shed alphabet richness per Doc 731) must transform through tiers N, N-1, …, M, with each tier's representation in the corresponding `A_k`. The interior contour `I = (I_N, I_{N-1}, …, I_M)` is the sequence of these representations.

Given M and T, the interior `I` is constrained but not unique: the implementation-freedom condition (Doc 730 P4) allows multiple valid interior contours per (M, T) pair. The engagement's job at pipeline-spawn is to *pick* one interior; the substrate-shaped-work discipline (Doc 744 candidate) is the operational pattern that picks.

When M and T are correctly stated, the interior-pick converges. When M and T are mis-stated, the interior-pick diverges — regressions surface at each interior point that violates an implicit constraint the misstatement obscured.

### II.3 Why the mouth-terminus pair is discoverable

ECMA-262 (and its sibling specs) state behaviors as input→output rules with intermediate-step prose. Each behavior has a *natural* M (the input syntax/call form) and a *natural* T (the output value/observable). The pipeline's M and T are recoverable from the spec text via the resolver-instance pattern (Doc 729): the spec section is the source, the behavior is the artifact, and the implementation engages the source's directive consumption + the artifact's stage-deterministic emission.

For non-spec pipelines (apparatus-tier; methodological pipelines), M and T are recoverable from the apparatus enumeration (`apparatus/docs/repository-apparatus.md` §III) plus the corpus articulation of the discipline.

The discoverability claim is empirical: across the IR locale's 21-rung TDZ session, every closed sub-shape had a mouth (an ECMA-262 §13.3.1.1 or §15.4.5.4 invocation form) and a terminus (a `ReferenceError` throw or a TDZ-cleared binding read). Naming both before substrate work began correlates with shorter trajectories; failing to name one correlates with longer rule-13 chains (see §VI's evidence summary).

---

## III. Regression as pipeline-discovery event

### III.1 The standard reading vs the methodological reading

The **standard reading** of a regression is "the change broke something; revert and try again." The substrate-shaped-work discipline (Doc 744 candidate) already promotes rule 13's revert-then-deeper-layer-closure as the canonical alternative. This articulation extends rule 13 with a stronger claim: a regression is not merely a discipline trigger for revert, it is *itself a measurement* that discloses pipeline shape.

Specifically: a substrate-introduction round R that targets pipeline `P = (M, T, I, R)` and produces a negative result identifies an interior point `I_k ∈ I` that R's design treated as one shape but that an unnamed constraint in P requires to be another shape. The regression's diagnostic-shape (which test regressed, with which failure tag, at which substrate tier) localizes `I_k` along the pipeline's interior.

### III.2 The three-round trajectory as evidence

IR-EXT 38→39→40 (class-this TDZ; ~190 LOC across three rungs):

- **Round 1 (EXT 38)**: pipeline named with M = derived ctor entry, T = ReferenceError on pre-super `this` read. Interior assumed = `(SetThisTDZ at body entry, PushThis TDZ check)`. Negative result: -4 diff-prod. Discovery: `I_super-call` — the super-call setup site reads `this` via PushThis, which the new TDZ check defeats.
- **Round 2 (EXT 39)**: pipeline refined. Interior extended with `(Frame.derived_initial_this stash, Op::PushThisRaw for super-call setup)`. Negative residual: arrow created post-super still throws TDZ. Discovery: `I_arrow-MakeArrow` — at arrow MakeArrow time, the cell's value is correct, but the arrow's own bytecode contains a SetThisTDZ emit from class_stack inheritance.
- **Round 3 (EXT 40)**: pipeline closed. Interior extended with `(next_compile_is_derived_ctor flag, gated emit at outermost ctor only)`. Closure landed; +1 test passes.

Each round's negative result was a measurement at a specific interior point that the prior round's pipeline statement obscured. The cumulative trajectory disclosed the pipeline's full interior contour: `M → SetThisTDZ → super-call (PushThisRaw) → SetThis (writes through this_cell) → arrow MakeArrow (post-super state) → arrow body PushThis → T`. The substrate prefix from rounds 1+2 was retained because each prefix was structurally correct for the interior point its round addressed.

### III.3 Predictive shape of regression-as-discovery

When a substrate round R targeting pipeline P regresses, the regression class predicts which interior point `I_k` was mis-stated:

- **Regression in adjacent-tier consumer**: the round's emit-site interaction with an adjacent-tier consumer was unaccounted. E.g., EXT 38's SetThisTDZ + super-call's PushThis: super-call is an adjacent-tier consumer (bytecode emit tier consuming runtime-tier this_value).
- **Regression in nested-function compile**: the round's compile-time signal propagated to nested compiles via class_stack/scope-stack inheritance. E.g., EXT 40's class_stack inheritance trap.
- **Regression in timing-edge between rounds**: the round's substrate ordering differed from a sibling round's ordering at a shared interior point. E.g., EXT 39's arrow-cell timing — MakeArrow's allocation timing vs SetThis's cell-write timing.

The three classes recur across the engagement's negative-result history (Addendum XV's NLC.0-revised; the EXT 25→26 destructure-leaf StoreLocal; the EXT 29→34 script-mode globalThis-mirror). Each class identifies a specific kind of mis-statement of the pipeline's interior. The mapping `regression class → mis-stated I_k` is the predictive heuristic this articulation contributes.

---

## IV. The DAG / Lattice / Alphabet relation between pipelines

Doc 720 names the runtime as a DAG of interconnected pipelines; Doc 740/741 materialize multi-tier cascade-revival as the operational pattern for interactive pipelines. This section formalizes the *choice* of relation type per the shape of the interaction.

### IV.1 DAG relation — terminus-feeds-mouth

Two pipelines `P_1 = (M_1, T_1, I_1)` and `P_2 = (M_2, T_2, I_2)` are in **DAG relation** when `T_1 ≡ M_2` (the terminus shape of P_1 matches the mouth shape of P_2). The composition `P_1 ⊳ P_2` is a single pipeline with mouth `M_1`, terminus `T_2`, and interior `(I_1, I_2)`.

DAG relations are the dominant form across the lowering-compiler stack (Doc 730 T1-T6): each tier's terminus feeds the next tier's mouth. Substrate work at one tier's terminus must respect the next tier's mouth-shape; misalignment surfaces as the cross-tier deviation pipeline (Doc 730 §XII).

**Discriminator**: DAG when the pipelines are strictly ordered by substrate tier; no shared interior; no parallel paths.

### IV.2 Lattice relation — meets and joins on shared interior

Two pipelines are in **lattice relation** when they share one or more interior tiers `I_k` but with distinct (M, T) pairs. Their **meet** at the shared interior is the substrate shape both must accept; their **join** at a shared downstream tier is the substrate shape both contribute to.

Lattice relations dominate when a single substrate tier serves multiple consumer pipelines. The shape-substrate tier (`pilots/rusty-js-shapes/`) is a meet site for getprop-IC (P_1), stub-emitter (P_2), and inline-cache (P_3) pipelines — all three consume shape descriptors at the shared tier; the shape tier's substrate must satisfy all three (Doc 729 §A8.13).

**Discriminator**: lattice when pipelines share a substrate tier but have distinct mouth-terminus pairs; the shared tier produces values both consume; the tier's substrate must satisfy the join of all consumers' requirements.

### IV.3 Alphabet-exchange relation — same-tier typed-primitive exchange

Two pipelines are in **alphabet-exchange relation** when they occupy the same substrate tier and exchange typed primitives at a shared boundary. The exchange is **typed** (per Doc 730 P1) — the boundary's alphabet is the intersection of both pipelines' alphabets at that tier.

Alphabet-exchange relations dominate at tier-internal contracts: bytecode emit-site to bytecode emit-site within one compiler pass; runtime-helper to runtime-helper within one Runtime method. The cross-pipeline Load/Store opcode symmetry (Rule 25, IR finding IR.32) is an alphabet-exchange contract: every Load-shape opcode that can carry a sentinel-shaped value mandates a Store-shape counterpart with the symmetric check at the same tier.

**Discriminator**: alphabet-exchange when pipelines are at the same substrate tier; no tier-ordering; the contract is a shared typed-primitive set at a tier-internal boundary.

### IV.4 The discrimination heuristic

Given two pipelines `P_1` and `P_2` whose interaction is to be analyzed, the relational form is discoverable by inspection of their mouth-terminus shapes:

| (M_1, T_1) vs (M_2, T_2) shape relation | Relational form |
|---|---|
| `T_1 ≡ M_2` (terminus of one is mouth of other; strict ordering) | DAG (§IV.1) |
| `M_1 ≠ M_2 ∧ T_1 ≠ T_2 ∧ ∃k: I_1[k] ∩ I_2[k] ≠ ∅` (distinct ends, shared interior) | Lattice (§IV.2) |
| same substrate tier; pipelines exchange typed primitives at a tier-internal boundary | Alphabet-exchange (§IV.3) |

The discriminator is operational: choose the form whose definitional shape matches the pipelines' joint state, then validate by reading the relation-implied substrate constraints against the pipelines' interior. A wrong-form choice surfaces as a regression in §III.3's third class (timing-edge between rounds).

### IV.5 Composition of forms

The three forms compose: a substrate cluster's full topology is a DAG of pipelines (Doc 720) where some edges are lattice-meets (multi-consumer substrate tiers) and some are alphabet-exchanges (tier-internal typed-primitive contracts). Doc 740's multi-tier cascade-revival reads the cluster's DAG topology to identify the relevant-tier set R; the operational closure requires all three relation forms to be respected.

---

## V. The predictive heuristic — pipeline-form discovery as discipline

The substrate-shaped-work discipline (Doc 744 candidate) operationalizes the standing rules into a five-phase pipeline. This articulation extends Phase 1 (Spawn) and Phase 3 (Pin-Art-probe-if-duplicated) with explicit pipeline-form discovery.

### V.1 At spawn (Phase 1 extension)

Before declaring the substrate move-shape:

1. **Name the mouth M**: which behavior-surface input shape is the pipeline consuming? (For ECMA pipelines, cite the spec section and grammar production. For meta-pipelines, cite the apparatus event class.)
2. **Name the terminus T**: which spec-mandated emission shape must the pipeline produce? (For ECMA pipelines, cite the spec-mandated artifact. For meta-pipelines, cite the discipline artifact.)
3. **Sketch the interior I**: which substrate tiers must the trajectory pass through? Use the substrate-tier alphabet (Doc 730 + Doc 731) to enumerate the alphabets `A_N, A_{N-1}, …, A_M`.
4. **Identify neighbor pipelines and their relational forms (R)**: per §IV.4, list neighbor pipelines and discriminate DAG / lattice / alphabet-exchange.

If any of M, T, I, R cannot be named, the pipeline is mis-stated; the locale spawn surfaces a discovery probe (Rule 23 baseline-inspection) before substrate work begins.

### V.2 At chapter-close (Phase 5 extension)

When a chapter folds:

1. **Verify M-T-I correspondence**: did the closed pipeline's interior match the sketch from Phase 1? If divergence, name the implicit constraint that forced the divergence and record as a finding.
2. **Verify R correspondence**: did the neighbor-pipeline interactions resolve per the predicted relational form? If a different form materialized, record as a finding for the standing-rule promotion path.
3. **Promote pipeline-form discovery findings**: per the findings.md addendum protocol (Doc 727 §X), promote pipeline-form discoveries as findings with the four-tuple (M, T, I, R) recorded explicitly.

### V.3 Predictive use

The articulation predicts:

- **A new substrate-shaped problem with explicit M, T, sketched I, identified R closes in ≤ 3 rungs** (per IR.33's cumulative substrate amortization + the discipline pipeline's per-phase cost model). Cross-validates against the GPI / IPBR / TSR results of rule 13's prospective application (Addendum IX).
- **A new substrate-shaped problem with one of M/T/I/R implicit incurs an additional rule-13 round per implicit element**, with the round's negative result discovering the implicit element. Validates against IR-EXT 25→26 (one implicit emit-site = one extra round), EXT 29→34 (one implicit constraint = four-round chain — EXT 30 probe + EXT 31/32/33 incremental + EXT 34 closure), EXT 38→39→40 (two implicit constraints = three rounds).
- **A pipeline whose relational form (R) is mis-discriminated produces a class-3 regression (timing edge between rounds)** with high specificity to the wrong-form choice. The class-3 regression IS the discriminator's falsifier.

---

## VI. Empirical evidence summary

| Trajectory | Rounds | LOC | M-T-I-R completeness at spawn | Class of negatives surfaced |
|---|---:|---:|---|---|
| GPI (interp-getprop-ic, 2026-05-23) | 1 round (EXT 2 closure) | 42 | M, T, I, R all explicit (rule 13 prospective) | None (no negative; closure first round) |
| IPBR (iter-protocol-bytecode-rewrite, 2026-05-24) | 1 round (EXT 2 closure) | similar | M, T, I, R all explicit | None |
| TSR (ts-resolve, 2026-05-24) | 4 rounds (EXT 5 closure) | ~5× GPI | I extended at each round; R partially explicit | Class-2 (nested-compile signal propagation; C3 cost-model null) |
| IR-EXT 25→26 (Op::InitLocal TDZ-on-assign) | 2 rounds | ~80 | T implicit (which init sites need bypass) | Class-1 (emit-site interaction with destructure-leaf) |
| IR-EXT 29→34 (module-top TDZ) | 4 rounds + 1 probe | ~120 | I implicit at script-mode boundary | Class-1 + Class-2 (Phase-3 implicit-constraint identification at IR-EXT 30 probe) |
| IR-EXT 38→39→40 (class-this TDZ) | 3 rounds | ~190 | I implicit at super-call setup + R implicit (class_stack inheritance trap) | Class-1 (super-call PushThis) + Class-2 (class_stack inheritance) + Class-3 (arrow-cell timing) |

**Pattern**: rounds-to-closure ≈ count of implicit (M, T, I, R) elements at spawn + 1. The predictive heuristic's primary use is to *reduce* implicit elements at spawn so the chain converges in fewer rounds.

---

## VII. Falsifier

The articulation's falsifier is a substrate-shaped problem with M, T, I, R explicit at spawn that nonetheless takes > 3 rounds to close, with no novel substrate-class-introduction. If such a case materializes, the four-tuple-discovery heuristic is partially falsified for that substrate class; the falsifier surfaces either a missing element (5th tuple component? — currently undeducted) or a heuristic gap in the (M, T, I, R) discrimination per §IV.4.

Conversely, if a substrate-shaped problem spawned with all four elements explicit converges in ≤ 3 rounds across 5+ independent locales, the heuristic is corroborated as a predictive instrument.

Forward-derived prediction: subsequent IR locale rungs (post-Addendum XVI) that target the unscopables-tdz sub-shape — where M (with-block lookup), T (ReferenceError on unscopables-protected name in TDZ), I (with-binding lookup + scope-chain walk + sentinel check), R (lattice with the with-substrate pipeline at the lookup-tier meet) are all explicit at spawn — should close in 1-2 rounds with substrate amortization from the existing TDZ machinery.

---

## VIII. Cross-corpus references

- **Doc 540** — Pin-Art apparatus formalization; the substrate-tier alphabet system this articulation reads.
- **Doc 541 Appendix E** — SIPE-T scale-invariance; the substrate-shape recursion that pipeline-discovery instantiates.
- **Doc 581** — resume-vector discipline; the per-locale operational substrate of every pipeline.
- **Doc 715** — consumer substrate dependency graph; the DAG-relation primary articulation.
- **Doc 720** — runtime as DAG of interconnected pipelines; the prior corpus statement this articulation extends with the discrimination heuristic.
- **Doc 729** — resolver-instance pattern; mouth-terminus correspondence at every substrate tier.
- **Doc 730** — vertical recurrence of the lowering-compiler closure; the alphabet-tier system for substrate-typed primitives.
- **Doc 731** — JIT as lowering-compiler tier; alphabet-purity as a substrate constraint.
- **Doc 734** — meta-resolution pipeline; the engagement's own operating pipeline as a primary articulation example.
- **Doc 739 / 740 / 741** — cascade-revival series; the empirical material for §IV's DAG/Lattice composition.
- **Doc 742** — resolver-instance boundary contract; the alphabet-exchange-relation primary articulation.
- **Doc 744 candidate (substrate-shaped-work-discipline.md)** — the operating discipline this articulation extends with pipeline-form discovery.
- **`apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`** — the conformance-DAG framing; this articulation's DAG-relation correspondence.

---

## IX. Status

Working draft. Candidate for corpus promotion after keeper review. Located at `docs/engagement/prospective/pipeline-form-discovery-as-predictive-heuristic.md` per `apparatus/docs/repository-apparatus.md` §0 promotion path. On promotion: target corpus number Doc 745 (after Doc 744 candidate); mirror to `docs/corpus-ref/`; add cross-references from `apparatus/docs/predictive-ruleset.md` (introduce Rule 27 "Mouth-terminus completeness at spawn" derived from §V.1) + `pilots/rusty-js-jit/findings.md` Addendum XVII + `CLAUDE.md` §Substrate-shaped-work discipline (extend Phase 1 + Phase 5 per §V.1+V.2).
