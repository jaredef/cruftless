# Findings-disposition protocol

Standing apparatus-tier protocol for the periodic exercise of fitting findings (per-locale `findings.md` entries + trajectory-embedded findings, aggregated in `apparatus/docs/findings-ledger.md`) to a disposition: integrate-existing-arc, integrate-scaffolded-arc, lift-to-new-arc, promote-to-standing-rule, relocate-to-apparatus-pilot, lattice-meet-annotation, defer-with-cross-reference, close-as-locale-singleton.

This protocol is the apparatus self-applying its own methodology to its findings. The 5-phase substrate-shaped-work pipeline (`CLAUDE.md` §Substrate-shaped-work discipline) governs substrate rungs against the engine; this protocol applies the same pipeline at the finding tier with findings as the cells and arcs / standing rules as the lifted substrates. Where the orphan-disposition protocol (`apparatus/docs/orphan-disposition-protocol.md`) operates at the locale-to-arc subsumption boundary, this protocol operates at the finding-to-arc-or-standing-rule subsumption boundary.

The two protocols compose: orphan-disposition resolves locales-without-arcs; findings-disposition resolves findings-without-arcs (or findings-without-standing-rules). Both are engagement-tier instances of Phase 5 chapter-close-inspect (Rule 15).

---

## I. When to run the protocol

Trigger conditions:

1. **After a findings audit** (e.g., the audit-ledger Entry 002 sweep run 2026-05-30 per keeper Telegram 10568) that has just landed a `findings-ledger.md` update or a per-locale `findings.md` cluster.
2. **At engagement-wide chapter-close-inspect rungs** where the engagement reviews its finding-to-arc / finding-to-standing-rule subsumption coherence.
3. **Whenever a new finding is authored with cross-locale recurrence ≥2** — promotion-readiness is the immediate signal to run the protocol on that finding.
4. **At findings-ledger commit time**: any commit that introduces a new entry (or a new cross-locale-recurrence pattern) MAY trigger the protocol if the keeper or arbiter directs.

The protocol is not run per-finding-authoring; it is a periodic engagement-discipline artifact, mirroring the orphan-disposition cadence.

---

## II. The protocol — self-applied 5-phase pipeline

For each finding, six operational steps run in order. The steps map to the substrate-shaped-work 5-phase pipeline at the finding tier:

### Step 1 — Spawn (Rule 11 5-axis at finding tier)

Read the finding's `findings-ledger.md` entry. Recover (M, T, I, R) per Doc 744 §V.1 at the finding's tier:

- **M (Mouth)** at finding tier: the substrate pattern the finding describes (e.g., "IC cache lifetime", "JIT boundary encoding", "narrow Result dispatcher over wide signature lift").
- **T (Terminus)** at finding tier: the design discipline the finding obligates (e.g., "lift caches to Runtime", "5-axis pre-spawn coverage", "dispatcher-in-Result-frame heuristic").
- **I (Interior)** at finding tier: the named abstract ops / engine pillars / measurement instruments the finding operates over.
- **R (Relations)** at finding tier: cross-locale recurrence count + lattice-meets + DAG relations to other findings + relations to existing arcs + relations to predictive-ruleset rules.

Apply the 5-axis pre-spawn coverage check at the finding tier:
- **A1 component-A/B**: is the finding's pattern repeatable in independent locales?
- **A2 op-set**: what abstract-op or substrate-tier does the finding constrain?
- **A3 value-domain**: what value-domain coverage does the finding require (if substrate-pattern class)?
- **A4 locals-marshaling / scope**: what cross-substrate boundaries does the finding bind?
- **A5 emission-shape**: what verified emission shape does the finding mandate (Pin-Art validation trace)?

### Step 2 — Baseline-inspect (Rule 23 at finding tier)

Before declaring the disposition move-shape, inspect the finding's current state against the apparatus:

- Is the finding present in `apparatus/docs/predictive-ruleset.md`? If yes, the disposition is at-most lattice-meet-annotation or close-as-promoted.
- Is the finding present in `pilots/rusty-js-jit/findings.md` Addenda? If yes, the disposition is at-most cross-reference + standing-rule-promotion-candidate.
- Is the finding trajectory-embedded only? Per-locale convention defers `findings.md` extraction until the second rung in the locale; the disposition recognizes this state.
- Is the finding's cross-locale recurrence count ≥2? If yes, promotion-readiness is achieved; if no, defer-with-cross-reference is the floor.

If inspection reveals the finding's true shape is at a different tier than the ledger entry declared (e.g., an apparatus-tier finding filed as substrate-pattern), treat the entry as a probe that surfaced the real coordinate; correct the classification first.

### Step 3 — Test the eight disposition candidates

For each finding, test the eight candidates in order; the first that fits is the disposition.

1. **Integrate into existing arc**: the finding's (M, T, I, R) at finding tier fits sub-shape correspondence (Doc 745 §IV.3 four conditions) under an existing arc's substrate scope. The finding becomes an arc-tier cross-locale finding (the arc.md §Cross-locale findings section gains a citation; the finding's ledger entry adds the arc back-reference).
2. **Integrate into scaffolded but not-yet-rostered arc**: same as 1 but for an arc scaffolded in the same audit cycle (e.g., an arc whose arc.md exists but whose findings list is still empty).
3. **Lift to new arc**: ≥3 findings share a multi-substrate program shape that is not captured by any existing or proposed arc → scaffold new arc per `apparatus/docs/arc-as-coordinate.md`, with the findings as the arc's founding cross-locale findings.
4. **Promote to standing rule**: cross-locale recurrence ≥2 + apparatus-tier or abstract-op-discipline class + keeper-or-arbiter sign-off → append to `apparatus/docs/predictive-ruleset.md` (numbered next-rule); update the finding's `findings-ledger.md` Promotion status in-place per the §Discipline single-allowed-edit.
5. **Relocate to apparatus-pilot tier**: the finding's primary output is a measurement instrument, an audit discipline, or a meta-apparatus articulation (mirror of orphan-disposition pattern III.3) → relocate to `pilots/apparatus/<name>/` and adjust the finding's coordinate.
6. **Lattice-meet annotation**: the finding shares mouth-or-terminus shape with another finding AND substrate tier (mirror of orphan-disposition pattern III.2) → group into a pair/triplet with cross-reference annotation in the ledger; pair-promotion when scaffolded.
7. **Defer with cross-reference**: the finding's natural arc has not been scaffolded and is not on the recommended-creation-order list → annotate the finding-ledger entry with the future arc enrollment and the relational form; defer.
8. **Close as locale-singleton**: the finding is locale-specific only with no cross-locale signal and no promotion expected → mark Promotion status `none` and close.

### Step 4 — Surface cross-finding patterns (Pin-Art probe at finding tier per Rule 24)

After all findings are dispositioned, group by disposition. Patterns recurring across ≥2 findings become candidate apparatus-tier observations for promotion via the Doc 727 §X basin-stability append-only protocol. Three patterns are predicted to recur based on the structural correspondence with orphan-disposition:

- **Pattern F.1** — finding-as-standing-rule-already-promoted-but-not-cross-referenced: the finding is already in predictive-ruleset.md but the ledger entry was authored without bidirectional traceability. Disposition is lattice-meet-annotation at the apparatus tier.
- **Pattern F.2** — finding-cluster-without-arc-host: 3+ findings share substrate tier + emit shape but no arc has been scaffolded that hosts them. Disposition is lift-to-new-arc.
- **Pattern F.3** — finding-promoted-to-rule-without-canonical-source-update: the finding is in predictive-ruleset but `pilots/rusty-js-jit/findings.md` Addenda do not carry the bidirectional back-reference. Disposition is lattice-meet-annotation with corrective edit at the canonical source.

A fourth or fifth pattern that recurs at 2+ subsequent findings-disposition exercises is promoted to a standing rule (Rule 30+ per the predictive-ruleset).

### Step 5 — Execute the disposition action sequence (Rule 13 — revert if negative)

Per the dispositions assigned in Step 3, execute in order:
- (a) promote-to-standing-rule first (highest-confidence + highest-apparatus-leverage);
- (b) lift-to-new-arc next (creates the receiver for subsequent integrations);
- (c) integrate-into-existing-or-scaffolded arc;
- (d) annotate cross-references for lattice-meets + deferred enrollments;
- (e) verify already-promoted findings have bidirectional back-references in predictive-ruleset.md and the canonical source `pilots/rusty-js-jit/findings.md`;
- (f) refresh `apparatus/locales/manifest.json` if any locale ownership shifted.

If a disposition's downstream verification fails (e.g., a promoted rule contradicts an existing rule; a lifted arc fails Doc 744's (M, T, I, R) coherence at scaffolding), apply Rule 13 prospective revert: revert the disposition, retain the trajectory + diagnosis, identify the deeper-layer closure (often a sub-step in the protocol that needs refinement), implement at the next pass.

### Step 6 — Document the exercise (Rule 15 chapter-close-inspect)

Write the disposition outcome into the audit-ledger as a new entry (audit type: `findings-disposition` or `findings-disposition-cycle-N`). Cite per-finding disposition + cross-finding patterns + actions authored. The audit-ledger entry composes with the engagement's findings-history and arc-creation-history records.

If new arcs were scaffolded, each arc.md gains a §Cross-locale findings section with the source findings cited; if standing rules were promoted, `apparatus/docs/predictive-ruleset.md` is appended with bidirectional ledger back-references.

---

## III. Validation

The protocol's predictive use: each subsequent findings-disposition exercise produces faster-disposition cadence as the finding-to-arc and finding-to-standing-rule subsumption topology stabilizes. The findings-ledger's cross-locale-recurrence section monotonically grows toward zero un-promoted findings + zero arc-orphan findings (steady state).

The protocol's falsifier: a disposition that subsequently regresses (a promoted standing rule that turns out to contradict empirical practice, or a new arc whose roster never accumulates the predicted sibling findings). Per Doc 744 §III.3 class-three regressions, the timing-edge between the disposition and the subsequent regression is the falsifier signal; the regression triggers Rule 13 revert at the next cycle.

The protocol's self-application coherence check: at the meta-tier, this protocol IS a finding (about how findings should be dispositioned). At its second authoring (a future revision of this doc that adjusts the 8 candidates or 6 steps), it must pass through itself — the revision goes through Step 3 candidate-test before landing. If the protocol's revision cannot self-apply, the meta-coherence is broken; surface as a Rule-13 negative result and design the deeper-layer closure.

---

## IV. Operational integration

`apparatus/docs/repository-apparatus.md` §III standing-discipline-artifact list gains this protocol. `apparatus/docs/audit-ledger.md` recognizes a new audit type `findings-disposition`; the protocol's first worked instance is the audit-ledger Entry 003 record (2026-05-30; same change as this doc).

`CLAUDE.md` + `AGENTS.md` §Substrate-shaped-work discipline §Phase 5 chapter-close-inspect gains a pointer to this protocol as the engagement-tier instance of Phase 5's "verify finding-promotion disposition" obligation, paired with the existing orphan-disposition pointer for locale-promotion.

`apparatus/docs/arc-as-coordinate.md` §F (event log) gains an entry-class for "findings-disposition annotation" so arc.md updates from this protocol are recorded in the arc's log.md.

The composition with orphan-disposition is load-bearing: a locale dispositioned to "scaffold-new-arc" by orphan-disposition becomes a candidate receiver for "integrate-into-scaffolded-arc" by findings-disposition at the next cycle. Running orphan-disposition first, findings-disposition second, is the recommended cadence at engagement-wide chapter-close.

---

## V. First worked application — 2026-05-30 findings-disposition cycle 1

Applied to the 10 entries of `apparatus/docs/findings-ledger.md` at audit-ledger Entry 002 commit (d904702b). Disposition outcomes:

| Ledger Entry | Finding | Disposition | Action |
|---|---|---|---|
| 001 | JIT 26-rule + Addenda I–XVI (canonical) | close-as-locale-singleton (already canonical) | None; ledger entry is the canonical reference. |
| 002 | IHI.1 (IC cache lifetime) | lattice-meet-annotation (Pattern F.1) | Already promoted as Rule 8+9 + JIT VIII.4; annotation in ledger entry suffices. |
| 003 | OSR.1 + OSR.2 (JIT calling-convention + loop-boundary) | lattice-meet-annotation (Pattern F.1) | Already promoted as JIT VIII.2+3 + Rule 11 A4+A5; bidirectional back-ref present in ledger. |
| 004 | TL.1 + TL.2 (whole-body bail bounds + Φ value-domain) | lattice-meet-annotation (Pattern F.1) | Already promoted as JIT VII.2+3 + Rule 11 A2+A3; bidirectional back-ref present. |
| 005 | TAECSF.1 (narrow dispatcher beats wide signature lift) | defer-with-cross-reference (one-more-observation pending) | Lattice candidates named in ledger Entry 005 (integer-kind ConvertNumberToTypedArrayElement; template-literal ToNumber; future Result-thread sites). Defer until second corroboration. |
| 006 | BBND yield-analysis (five-condition multiplier) | defer-with-cross-reference (corpus Doc 743 promotion pending) | Cross-locale corroboration count = 1; defer until second locale corroborates the constraint-stacking pattern. |
| 007 | Cross-locale: IC-cache-lifetime | close-as-promoted (Rule 8+9 already covers) | None; ledger pattern entry is the cross-reference. |
| 008 | Cross-locale: JIT boundary encoding | close-as-promoted (Rule 11 5-axis already covers) | None. |
| 009 | Cross-locale: multi-tier cascade-revival | close-as-promoted (Rule 13 + Doc 740 already covers) | None. |
| 010 | Cross-locale: baseline-inspection-as-locale-probe | close-as-promoted (Rule 23 already covers) | None. |

**No new arcs scaffolded at cycle 1** — all current findings are either already promoted to standing rules + JIT canonical addenda (Entries 001–004, 007–010) or below promotion-readiness threshold (Entries 005–006, awaiting second corroboration). The cycle's output is the protocol itself + the bidirectional traceability state that the protocol formalizes.

**Cross-finding pattern observed at cycle 1 (Pattern F.1 instance count = 3)**: Entries 002, 003, 004 are all promoted findings with bidirectional back-references already present; the cycle confirms Pattern F.1 is a steady-state pattern when the canonical-ledger discipline is healthy. The pattern is predictive: subsequent cycles will continue to surface promoted findings; Pattern F.1 will dominate until a wave of new locale findings lands without the bidirectional back-reference discipline.

**Next-cycle prediction**: when TAECSF.1 receives its second corroboration (likely from the integer-kind ConvertNumberToTypedArrayElement sub-substrate within `pilots/ta-element-coercion-spec-faithful/` or from a sibling template-literal ToNumber close), apply the protocol's Step 3 candidate (4) promote-to-standing-rule. Expected outcome: Rule 27 (numbered after the unconsolidated rules 16–22 are formalized) — "narrow-dispatcher-over-wide-signature-lift for Result threading through non-Result intermediates with ≥10 callers".

When BBND yield-analysis receives its second corroboration (a non-parser-early-error locale exhibits a five-condition multiplier shape), apply candidate (4) promote-to-standing-rule. Expected outcome: Rule 28 (or earlier if 16–22 are consolidated first) — "constraint-stacking yield multiplier".

The 2026-05-30 cycle does not author either rule yet; both await their second corroboration per the protocol's promotion-readiness gate.
