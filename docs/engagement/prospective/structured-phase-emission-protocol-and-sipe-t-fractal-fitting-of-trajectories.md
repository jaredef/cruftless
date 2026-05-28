# The structured per-Phase emission protocol for the resume vector: trajectory.md as the substrate-shaped pipeline's terminus, with a SIPE-T fractal heuristic for fitting trajectories to the larger arc

**Status**: prospective draft, awaiting keeper review for corpus promotion. Authored 2026-05-28 per keeper directive Telegram 10156. Target corpus number Doc 745 or later (after Doc 744).

**Composes with**: Doc 540 (Pin-Art apparatus formalization), Doc 541 Appendix E (SIPE-T scale-invariance), Doc 581 (resume-vector discipline), Doc 727 §X (basin-stability append-only protocol), Doc 733 (fractal seeds-and-trajectories across substrate depth), Doc 737 (locale as coordinate; nested seed/trajectory pairs), Doc 744 (pipeline-form discovery as predictive heuristic), `apparatus/docs/arc-as-coordinate.md` (arc as multi-locale operational unit), `apparatus/docs/repository-apparatus.md` (apparatus enumeration), `CLAUDE.md` §Substrate-shaped-work discipline.

**Empirical anchor**: same-day pickup of the Doc 744 amendment cycle (TTTC/EDIEE; cruftless commit 4bded1b6 2026-05-28) showed that the per-phase pipeline-form checks at spawn (§V.1 of Doc 744) and at chapter-close (§V.2) leave the *intermediate* phases (baseline-inspection, Pin-Art-probe-if-duplicated, revert-then-deeper-layer-closure) under-articulated in their trajectory-emission obligations. The keeper's conjecture (Telegram 10156): "the Phase 5 in your finding stands to be articulated as a structured protocol of all Phases for the theoretically perfect resume vector (trajectory.md) — also, using SIPE-T, we can create a fractal heuristic for fitting trajectories to the larger arc."

---

## I. Thesis

The trajectory.md artifact per Doc 581 is the **terminus** of the substrate-shaped-work discipline's per-rung pipeline. Each Phase (spawn, baseline-inspect, Pin-Art-probe-if-duplicated, revert-then-deeper-layer-closure, chapter-close-inspect; per CLAUDE.md §Substrate-shaped-work discipline + Doc 744 §V) contributes a structured emission to trajectory.md. The "theoretically perfect" trajectory is the one where every Phase's emission obligation is met explicitly — the trajectory.md reader can reconstruct (M, T, I, R, observability) for every rung and audit which Phase produced each artifact.

Three claims follow.

**Claim 1 (per-Phase emission shape)**: each Phase has a structured emission obligation into trajectory.md whose shape is invariant across rungs. The shapes compose: a complete trajectory.md entry has five sub-shapes corresponding to Phases 1-5, plus the substrate move itself. Phase 5's chapter-close-inspect protocol (Doc 744 §V.2) is the already-articulated case; Phases 1-4 admit symmetric articulation.

**Claim 2 (SIPE-T scale-invariance of the trajectory artifact)**: the per-rung trajectory emission shape recurs at the per-locale, per-arc, and per-engagement scales. Per Doc 541 Appendix E (SIPE-T scale-invariance), the substrate-shape that emerges at the rung tier also emerges at the locale, arc, and engagement tiers with the same five-Phase structure. A locale's trajectory.md is the join of its rungs' emissions; an arc's log.md is the join of its locales' trajectories; the engagement's findings.md addendum cycle is the join of its arcs.

**Claim 3 (fractal fitting heuristic)**: a candidate substrate move at any scale (rung, locale, arc) fits the larger arc iff its emission shape is recoverable from the larger arc's terminus by the same Phase-1-to-5 decomposition that the rung-tier discipline uses. Fitting is verifiable by reading the candidate's mouth-terminus pair against the arc's mouth-terminus pair (Doc 744 §II) and confirming the candidate's interior is a sub-interior of the arc's. A misfit candidate produces a class-three regression (timing edge) per Doc 744 §III.3 at the arc-tier boundary.

The three claims compose into a fractal heuristic: the trajectory.md emission protocol at the rung tier IS the same protocol that operates at the locale, arc, and engagement tiers, and a candidate at any tier fits the next-larger tier iff its (M, T, I, R, observability) tuple reconstructs to the larger tier's tuple via the SIPE-T scale-invariance correspondence.

## II. The per-Phase emission protocol

Each Phase of the substrate-shaped-work discipline (CLAUDE.md + Doc 744 §V) emits a structured trajectory artifact. Together the five emissions form one trajectory rung entry.

### II.1 Phase 1 (Spawn) — six-element header emission

At rung-spawn, the trajectory entry's header records the six-element pipeline-form sketch per Doc 744 §V.1:

1. **M (mouth)** — the input shape the rung's substrate move targets. Cite spec section or apparatus event class.
2. **T (terminus)** — the emission shape the rung must produce. Cite spec-mandated artifact or discipline artifact.
3. **I (interior contour)** — the sketched sequence of intermediate-tier substrate shapes. Use Doc 730 + Doc 731 alphabet enumeration.
4. **R (relations)** — neighbor pipelines discriminated as DAG / lattice / alphabet-exchange per Doc 744 §IV.4.
5. **Mouth-gating prerequisites** — upstream DAG terminuses required to make M executable per Doc 744 §IV.1.a. List `(P_i, T_i)` for each.
6. **Observability classification** — whether the initial failure mode is observable (ordinary diagnostic) or unobservable (panic / abort / no-JSON / timeout) per Doc 744 §III.4. If unobservable, name the diagnostic scaffold and its sunset condition.

Any element marked "implicit at spawn" in the header is a predictive marker: per the Doc 744 §V.3 prediction, rounds-to-closure approximates the count of implicit elements plus one. The header thus self-reports the rung's expected complexity envelope.

### II.2 Phase 2 (Baseline-inspect at founding) — measurement-and-inspection emission

The trajectory entry records the baseline measurement (PASS/FAIL counts on the locale's exemplar suite + a sample of failure-table top rows per Rule 23). If baseline-inspection surfaces a move-shape at a coordinate different from the seed-declared coordinate, the trajectory entry names the surfaced coordinate and treats the spawn as a probe per Rule 23.

The emission shape is `(baseline_measurement, top-row inspection digest, coordinate-confirmation OR coordinate-relocation)`. A trajectory rung that skipped Phase 2 emission is auditable as a rung-23 violation.

### II.3 Phase 3 (Pin-Art-probe-if-duplicated) — duplication-signal emission

If the rung's planned substrate emit shape is duplicated across 3+ sites with the same shape per Rule 24, the trajectory entry records the duplication enumeration AND a Pin-Art probe finding that names the implicit constraint the duplication is approximating per Doc 744 finding IR.29 Constraint δ. If no duplication is detected, the trajectory entry records the negative ("Phase 3: no duplication signal; proceeding to Phase 4").

The emission shape is `(duplication_site_enumeration | "none"), (implicit_constraint_named | "n/a")`. The IR-EXT 30 rung is the canonical instance.

### II.4 Phase 4 (Revert-then-deeper-layer-closure if negative) — trajectory-of-rounds emission

If a substrate-introduction round produces a negative empirical result, the trajectory entry for THIS rung records the negative + the deeper-layer-closure plan + the substrate prefix retained. Subsequent rungs in the rule-13 chain record their incremental contribution, citing the chain's prior rungs explicitly. The chain's terminus rung explicitly closes the trajectory with the cumulative substrate amortization summary per Doc 744 finding IR.33.

The emission shape per chain rung is `(round_index, negative_class | "positive", substrate_added, substrate_prefix_retained, residual_blocker_named | "closed")`. A trajectory rung that records "revert and try again" without a deeper-layer-closure plan is a rule-13 violation.

### II.5 Phase 5 (Chapter-close-inspect) — verification-and-promotion emission

Per Doc 744 §V.2 (already articulated): verify M-T-I correspondence, verify R correspondence, verify scaffold disposition, promote pipeline-form discovery findings. The trajectory rung that closes a chapter (rung-37 in the IR locale's TDZ chapter; rung-19 in the prior IR session's chapter-fold) emits the chapter-close summary table per the IR-EXT 37 template.

The emission shape is `(M-T-I correspondence: match | divergence-name, R correspondence: match | form-shift-name, scaffold_disposition: removed | bypassed | retained-with-rationale, promoted_findings: [F1, F2, ...])`.

### II.6 The complete rung entry

A theoretically-perfect rung entry has six sections: §Phase 1 header, §Phase 2 baseline, §Phase 3 duplication-or-not, §Phase 4 trajectory-of-rounds, §Phase 5 close-verification, plus the substrate change itself (the diff + LOC count + gate measurements). The rung entry is auditable: a fresh reader can reconstruct (M, T, I, R, observability) without re-deriving from the substrate diff.

Current trajectory.md entries are partial — most record Phases 4 and 5 emissions explicitly but treat Phases 1, 2, 3 as implicit. The protocol's value is making the implicit explicit; the cost is per-rung emission overhead. Per the Doc 744 §V.3 prediction, the per-rung overhead is paid back in rounds-to-closure reduction.

## III. The SIPE-T fractal: scale-invariance across rung, locale, arc, engagement

Doc 541 Appendix E names SIPE-T as the substrate-induced-property emergence pattern's scale-invariance: the same substrate-shape that emerges at the rung tier also emerges at the locale, arc, and engagement tiers. This articulation applies SIPE-T to the trajectory artifact itself.

### III.1 The four scales

**Rung tier**: a single trajectory.md entry. Emission: six-section rung entry per §II.6. Spans one substrate move + its discipline-phase artifacts.

**Locale tier**: trajectory.md as a whole. Emission: ordered sequence of rung entries + a seed.md naming the locale's (M, T, I, R, observability) at the locale scale. Spans the locale's full rung sequence from EXT 0 to closure.

**Arc tier**: arc.md + log.md per `apparatus/docs/arc-as-coordinate.md`. Emission: ordered sequence of locale entries + the arc's (M, T, I, R, observability) at the arc scale. Spans the locales spawned for one coherent multi-locale program.

**Engagement tier**: findings.md addendum cycle per Doc 727 §X. Emission: ordered sequence of arc entries + the engagement's standing-rule promotion path. Spans all arcs the engagement has executed.

### III.2 The scale-invariance correspondence

At each scale, the discipline's five Phases operate with the same shape:

| Phase | Rung tier | Locale tier | Arc tier | Engagement tier |
|---|---|---|---|---|
| 1 (Spawn) | rung header sketch (M, T, I, R) | seed.md telos + carve-outs | arc.md trigger + telos + sub-locale roster | locale spawn via CANDIDATES.md |
| 2 (Baseline-inspect) | EXT 0 measurement + top-row inspection | locale baseline + analysis.md cross-ref | arc-tier baseline across sub-locales | engagement-wide instrument re-measurement |
| 3 (Pin-Art probe if duplicated) | per-rung emit-site enumeration | cross-rung pattern-detection | cross-locale pattern-detection (cf. IR-EXT 30) | cross-arc pattern-detection → standing-rule promotion |
| 4 (Revert-then-deeper-layer-closure) | rung-level rule-13 trajectory | locale-level rule-13 chain | arc-level cascade-revival (Doc 739/740/741) | engagement-level deferral + future-rung carry |
| 5 (Chapter-close-inspect) | rung close-verification | locale fold (cf. IR-EXT 37 chapter-fold) | arc close per arc.md status | engagement fold + corpus articulation |

The correspondence is operational: the same questions asked at the rung tier (what is M? what is T? what is the interior? what is the relation R?) are asked at the locale tier (what is the locale's telos? what are its carve-outs? what locales does it neighbor?), at the arc tier (what triggers the arc? what is the close-condition? what locales compose it?), at the engagement tier (what corpus articulation does this engagement produce? what standing rules emerge?).

The shape recurs because the substrate-shaped-work discipline IS itself a substrate-shaped pipeline per Doc 744 §VIII: its mouth is an apparatus event class, its terminus is a discipline artifact, and its interior is the five-Phase sequence. The recurrence is SIPE-T per Doc 541 App E: the substrate-induced property (the five-Phase structured emission) emerges at every scale where the apparatus operates.

### III.3 The trajectory-as-fractal-image

A locale's trajectory.md is not merely a list of rungs; it is the rung-tier emission shape replicated and scaled to the locale tier. An arc's arc.md is not merely a list of locales; it is the locale-tier emission shape replicated and scaled to the arc tier. The engagement's findings.md is not merely a list of arcs; it is the arc-tier emission shape replicated and scaled to the engagement tier.

The fractal image is auditable: at each scale, a fresh reader sees the same shape (header / baseline / probe / chain / close), and the relation between scales is the SIPE-T scale-invariance correspondence.

## IV. The fractal heuristic for fitting trajectories to the larger arc

A candidate substrate move fits its enclosing arc iff its emission shape is a sub-shape of the arc's emission shape per the SIPE-T correspondence. The operational test runs in three steps.

### IV.1 Step 1 — Recover the larger arc's (M, T, I, R, observability)

Read the arc's arc.md trigger + telos + sub-locale roster + close-condition. Derive the arc-tier four-tuple: the arc's mouth (the triggering directive's apparatus event class), the arc's terminus (the close-condition's discipline artifact), the arc's interior (the sub-locale sequence), the arc's relations (which other arcs the arc is in DAG / lattice / alphabet-exchange relation with).

If the arc's arc.md is absent or under-specified, the candidate cannot be fitted — the arc must be retroactively specified per `apparatus/docs/arc-as-coordinate.md` before fitting.

### IV.2 Step 2 — Project the candidate's emission shape

Read the candidate's seed.md (if locale-tier) or its rung header (if rung-tier). Derive the candidate's four-tuple at the candidate's scale.

### IV.3 Step 3 — Test sub-shape correspondence

The candidate fits the arc iff:
- The candidate's M is in `A_N` where the arc's M is in `A_N` or upstream (the candidate consumes input the arc's mouth produces or is at the arc's mouth).
- The candidate's T is in `A_M` where the arc's T is in `A_M` or downstream (the candidate produces output the arc's terminus consumes or is at the arc's terminus).
- The candidate's interior I is a contiguous sub-sequence of the arc's interior at the candidate's scale (no gap-jumping).
- The candidate's relations R are consistent with the arc's relations at the candidate's scale (the candidate inherits the arc's neighbor-relations through the SIPE-T correspondence).

If any condition fails, the candidate is misfit. Per Doc 744 §III.3 class-three regression, mis-fit candidates produce timing-edge regressions at the arc-tier boundary when their trajectories interleave with sibling candidates' trajectories under the arc's joint emission.

### IV.4 Operational use

The fractal heuristic predicts which candidates should land first within a multi-locale arc: candidates whose (M, T, I, R) sub-shape fits cleanly land before candidates whose fit requires substrate work at the arc tier. The IR-EXT 31-34 chain is the canonical example: rung-31 (block-scope TDZ) fit cleanly because its interior was already a sub-interior of the IR locale's TDZ-enforcement arc; rung-29 (module-top TDZ) misfit on first attempt because its R was implicit at the arc scale and only became fit-able after the script-mode mirror audit landed.

The heuristic also predicts when an arc itself needs decomposition: if multiple candidates require sub-shapes that share an interior tier but with distinct mouth-terminus pairs, the arc is a lattice of pipelines per Doc 744 §IV.2, and the right move is to retroactively decompose the arc into sub-arcs before continuing.

## V. Operational integration with the existing apparatus

The protocol composes with existing apparatus artifacts.

`apparatus/docs/repository-apparatus.md` §III lists trajectory.md per locale; this articulation specifies the per-rung emission shape that fills the trajectory.md. Update at §III to point to this Doc 745 candidate for the per-Phase emission protocol; §IV.Locale-spawn-protocol step 5 (rule 23 baseline-inspection) corresponds to Phase 2 emission per §II.2 here.

`apparatus/docs/arc-as-coordinate.md` §C, §D, §E specify arc-tier emission shapes (sub-locale roster, cross-locale findings, cumulative yield); this articulation specifies the arc-tier Phase emissions per §III.1's arc-tier column. The arc.md format gains explicit Phase 1-5 sections per §II's rung-tier shape, scaled to arc scope.

`pilots/rusty-js-jit/findings.md` Addendum protocol (Doc 727 §X) corresponds to engagement-tier Phase 5 (chapter-close-inspect + corpus articulation). This articulation's Doc 745 candidate is itself an instance of engagement-tier Phase 5 emission: the engagement's experience with the per-Phase protocol becomes a finding promoted to corpus.

`CLAUDE.md` §Substrate-shaped-work discipline (the operational pipeline) is extended by this articulation with per-Phase trajectory emission obligations. The discipline's procedural sequence (Phases 1-5) gains a structured trajectory artifact protocol (this doc §II) and a fractal correspondence (this doc §III + §IV).

## VI. Predictive use + falsifier

### VI.1 Predictive use

A locale spawned with the full per-Phase emission protocol enabled produces trajectory.md entries with explicit (M, T, I, R, observability) for every rung. The trajectory is auditable; the locale's resume-vector reader (per Doc 581) can reconstruct the substrate state without re-deriving from source code. Predicted benefit: fresh-resolver onboarding cost decreases from "read the full trajectory + the diff" to "read the trajectory's structured-emission summary table" — typically a 5-to-10-times reduction in onboarding time per the Doc 744 amendment's TTTC/EDIEE empirical observation that mouth-gating prerequisites are discoverable from structured-emission alone.

A candidate substrate move fitted to its enclosing arc via §IV's three-step test closes in rounds bounded by Doc 744 §V.3's rounds-to-closure formula at the candidate's scale. Misfit candidates produce class-three regressions; the regression IS the fitness test's falsifier.

### VI.2 Falsifier

The articulation is falsifiable in two ways.

First, if a locale spawned with the per-Phase emission protocol enabled produces trajectory.md entries that, when read by a fresh resolver, do NOT reconstruct (M, T, I, R, observability) without re-deriving from source, the protocol is partially falsified for that emission shape. The falsifier surfaces either an under-specified Phase emission obligation (a Phase's emission needs more structure) or a category of substrate move whose state is not capturable by the five-Phase decomposition (a sixth Phase needed).

Second, if a candidate substrate move fitted to its arc via §IV's three-step test nonetheless produces a class-three regression at the arc boundary, the fractal heuristic is partially falsified for that scale-pair. The falsifier surfaces either a SIPE-T correspondence gap (the scale-invariance does not hold at that boundary) or a missing relational form at the arc tier (a fourth relational form beyond DAG / lattice / alphabet-exchange per Doc 744 §IV).

Forward-derived prediction: applying the per-Phase emission protocol to the IR locale's currently-active rungs (the EDIEE / TTTC / IHI-array-entries / JSON-parse-reviver locales spawned post-Doc-744-amendment) should produce trajectory.md entries that fit the IR locale's arc cleanly + reduce subsequent rung-spawn cost. If the application produces neither, this articulation's protocol is partially falsified for those substrate classes.

## VII. Cross-corpus references

- **Doc 540** — Pin-Art apparatus formalization; the substrate methodology this protocol operationalizes.
- **Doc 541 Appendix E** — SIPE-T scale-invariance; the property that justifies the fractal correspondence of §III.
- **Doc 581** — resume-vector discipline; trajectory.md as the operational artifact this protocol structures.
- **Doc 727 §X** — basin-stability append-only protocol; the engagement-tier Phase 5 mechanism.
- **Doc 729** — resolver-instance pattern; the architectural target whose recurrence makes the SIPE-T correspondence operational.
- **Doc 733** — fractal seeds-and-trajectories across substrate depth; the prior corpus statement this articulation extends with per-Phase emission shape.
- **Doc 734** — meta-resolution pipeline; the engagement's own operating pipeline as a substrate-shaped pipeline.
- **Doc 737** — locale as coordinate; the locale-tier in the fractal hierarchy.
- **Doc 740 / 741** — multi-tier cascade-revival; the operational expression of cross-tier fitting per §IV.
- **Doc 742** — resolver-instance boundary contract; the alphabet-exchange-relation primary articulation.
- **Doc 744** — pipeline-form discovery as predictive heuristic; the per-Phase pipeline-form heuristic this articulation structures into trajectory emission.
- **`apparatus/docs/arc-as-coordinate.md`** — arc-tier emission protocol; the apparatus formalization this articulation scales to all four tiers.

## VIII. Status

Working draft. Candidate for corpus promotion after keeper review. Located at `docs/engagement/prospective/structured-phase-emission-protocol-and-sipe-t-fractal-fitting-of-trajectories.md` per `apparatus/docs/repository-apparatus.md` §0 promotion path. On promotion: target corpus number Doc 745 (after Doc 744); mirror to `docs/corpus-ref/`; extend `apparatus/docs/arc-as-coordinate.md` per §V; extend `CLAUDE.md` §Substrate-shaped-work discipline with per-Phase emission obligations; introduce Rule 28 candidate "Structured per-Phase trajectory emission" derived from §II.6 (awaiting keeper review per the standing-rule promotion path).

---

> "Do number 6 and then it's my conjecture that the Phase 5 in your no. 5 finding stands to be articulated as a structured protocol of all Phases for the theoretically perfect resume vector (trajectory.md) — it's also my thinking that using SIPE-T, we can create a fractal heuristic for fitting trajectories to the larger arc."
