# The substrate-shaped-work discipline: Pin-Art probing + revert-then-deeper-layer-closure + duplication-as-signal as the apparatus's induced methodology

**Status**: prospective draft, awaiting keeper review for corpus promotion. Authored 2026-05-28 per keeper directive Telegram 10134.

**Composes with**: Doc 540 (Pin-Art apparatus formalization), Doc 581 (resume-vector discipline), Doc 727 §X (basin-stability append-only protocol), Doc 729 (resolver-instance pattern), Doc 740 (substrate-introduction signature), Doc 741 (multi-tier cascade pipeline empirical materialization), `apparatus/docs/standing-rule-13-prospective-application.md` (rule 13's prospective-application thesis).

**Empirical anchor**: rusty-js-ir locale TDZ enforcement session (2026-05-27, EXT 20-34 across 15 rungs; findings.md Addendum XVI). Two complete rule-13 revert-then-deeper-layer-closure trajectories executed inside one session (EXT 25→26, EXT 29→34). Three new standing rules promoted from the session (rules 24, 25, 26).

---

## I. The thesis

The cruftless engagement's standing rules are not independently-applied checks. They compose into a five-phase discipline pipeline that every substrate move follows. The pipeline is what makes the rules collectively load-bearing rather than per-rung-applied. Once named explicitly, the pipeline becomes the engagement's induced methodology — substrate moves that follow it produce compounding progress; substrate moves that skip phases produce per-rung firefighting + regressions that surface the missed phase post-hoc.

The pipeline is induced by the engagement's structure: spawning a locale (Doc 737), measuring its baseline (Doc 581), implementing substrate work against it, and closing the chapter. The standing rules are the per-phase invariants that the apparatus has accumulated from prior negative-result rungs. The pipeline articulation here makes the rules' joint contract observable.

## II. The five phases

### Phase 1 — Spawn

Choose the locale coordinate. Apply Rule 11 (5-axis pre-spawn coverage check):
- A1 (component A/B): identify the actual hot-path component dominator
- A2 (op-set coverage): planned substrate's op-set covers the dominator's
- A3 (value-domain coverage): value-domain matches (e.g., NaN-boxing covers receiver tag set)
- A4 (locals-marshaling coverage): entry-mechanism marshals the relevant locals
- A5 (emission-shape coverage): JIT/lowering emission shape matches the dominator's structure

For matrix-derived coordinate picks, also consult `apparatus/locales/CANDIDATES.md` and confirm non-overlap with parallel agents per directive history. Phase 1 ends with a `seed.md` written + `apparatus/locales/manifest.json` refreshed.

### Phase 2 — Baseline-inspect at founding

Apply Rule 23 (founding-baseline-inspection / locale-as-probe). Before declaring the substrate move-shape:
- Measure the locale's failure-shape against current cruft.
- Inspect a sample of failures (10-20 rows).
- If inspection reveals the move-shape is at a different coordinate than the seed declared, treat the locale as a probe that surfaced the real target. Land the surfaced-coordinate move first; treat the spawned locale's pool as the validating test surface.

Phase 2 is what catches mis-spawns at founding (~5-10 minute inspection cost) instead of post-hoc (~hours-to-days of wasted substrate work). Cumulative engagement evidence shows ~50% of locales surface a different actual target than their seed declared on first measurement.

### Phase 3 — Pin-Art probe if duplicated

Apply Rule 24 (duplication-as-Pin-Art-signal). If the substrate work would emit a pattern duplicated across 3+ sites with the same shape and divergent failure modes:
- Pause the per-site work.
- Run a Pin-Art probe: enumerate the duplicated emit sites + cross-reference with any prior negative-result rungs at the surface.
- Surface the implicit constraint(s) the duplication is approximating.
- Design from the tier-above coordinate downward (the LIFT) rather than paying the per-site enumeration tax.

The LIFT need not be a monolithic refactor. The IR locale's EXT 31-34 chain demonstrated that the LIFT can land incrementally — each per-surface piece (block, switch, etc.) absorbs the duplication for that surface; the first piece pays the abstraction cost, subsequent pieces cost ~1/3 (per finding IR.30).

### Phase 4 — Revert-then-deeper-layer-closure if negative

Apply Rule 13. When a substrate-introduction round produces a negative empirical result (regression, parity loss, broken probe):
1. **Verify the negative** — re-measure; confirm not noise.
2. **Diagnose structurally** — name WHY the round added cost without benefit. Is it design (wrong-lifetime cache; wrong-receiver-shape detection; wrong-cost-axis target)? Or implementation?
3. **Revert** the negative round's code via git (keep the trajectory entry + diagnosis).
4. **Identify the deeper-layer closure** that the negative round's design pointed toward but didn't reach. Often the negative is the substrate-introduction at the wrong layer; the deeper layer is the actual closure tier.
5. **Implement the deeper-layer closure** as the next round.

The substrate prefix the negative leaves on disk frequently becomes the cheap enabler of the deeper-layer closure. Two complete trajectories at rusty-js-ir EXT 25→26 + EXT 29→34 both produced positive yield via the deeper-layer closure that the substrate prefix enabled. Finding IR.33 codifies the cumulative substrate amortization: an N-rung chain of rule-13 trajectories has comparable total LOC to a single naive monolithic rewrite but spreads cost across rungs that each have measurable yield.

### Phase 5 — Chapter-close-inspect

Apply Rule 15. At every chapter close:
- Inspect the post-fix failure table's top rows.
- If the top tag's actual cause (per example inspection) differs from the planned scope, the round is NOT done.
- The inspect-then-iterate compound-discovery pattern routinely surfaces higher-impact mid-round gaps than the planned-scope fix.

Phase 5 is what makes the discipline self-improving rather than drift-prone. Each chapter-close inspection that finds a higher-impact mid-round-discovery feeds back into Phase 1 of the next locale (CANDIDATES.md updated with the newly-surfaced coordinate).

## III. Cross-pipeline standing rules

Several standing rules apply at every phase, not at a specific phase:
- **Rules 1-3** (multi-run + detectability budget) for any measurement-bearing claim.
- **Rule 4** (never split a substrate move) on the implementation side of each rung.
- **Rules 5+10** (three-probe-levels + canonical fuzz) before any default-on flip.
- **Rule 6** (surface-completeness audit) when a rung changes data-structure storage.
- **Rule 14** (conservative-strip) when a rung adds a heuristic classifier.
- **Rule 25** (Load/Store opcode symmetric checks) when a rung adds a value-flow opcode that may carry a sentinel-shaped value.
- **Rule 26** (captured-slot TDZ uses compile-time guard) when a rung probes TDZ on a slot captured by inner-closure upvalues.

These cross-pipeline rules are the substrate-shaped invariants that any per-phase work must respect. They are not phase-conditional; missing them at any phase surfaces as a regression at a later phase.

## IV. The induced methodology

The five-phase pipeline + cross-pipeline rules form the substrate-shaped-work discipline. The discipline is *induced* by the engagement's structure, not externally imposed. Each rule was promoted from a finding that surfaced from a negative-result rung; each phase emerged from the operational rhythm of locale spawning + closing.

Three induced properties of the discipline:

1. **Substrate amortization across rungs**. Per-rung cost decreases as the engagement matures: each Pin-Art probe adds a coordinate the next probe can reuse; each LIFT establishes an abstraction subsequent rungs ride. Cumulative cost is comparable to monolithic rewrites but distributed across measurable-yield rungs (finding IR.33).

2. **Compounding standing-rule coverage**. As rules are promoted, the predictive-coverage map (`apparatus/docs/predictive-ruleset.md` §Predictive coverage map) expands. Each new bug class that surfaces becomes either preventable by existing rules or a candidate for a new rule. The rule count converges (currently 26) as the engagement saturates its substrate-shape space.

3. **Cross-locale convergence**. Locales that follow the discipline produce findings with consistent shape: per-Phase-1 spawn rationale, per-Phase-2 baseline-inspection, per-Phase-3-or-Phase-4 trajectory cadence, per-Phase-5 chapter-close-inspection. The shape consistency makes findings.md addenda mergeable across locales without per-author convention drift.

## V. Discipline-tier anchor

Rule 13 + Pin-Art probing form the load-bearing methodology pair. Rule 13 is the trajectory primitive (each negative result either reverts cleanly or seeds a deeper-layer closure); Pin-Art probing is the diagnostic primitive (each substrate move's failure shape is read against the apparatus's coordinate space to surface implicit constraints). Together they make the pipeline self-correcting: a substrate move that fails one phase reveals which phase was missed or which rule was violated.

The two-trajectory validation in the rusty-js-ir locale TDZ session (2026-05-27) demonstrated reproducibility:
- EXT 25→26: Op::InitLocal substrate prep + StoreLocal TDZ check. The EXT 25 negative result (diff-prod 60/52 → 56/56 with 4 fixture regressions on destructure paths) reverted cleanly; EXT 26 added 2 emit-site conversions + 1 runtime re-flip to close the deeper layer.
- EXT 29→34: module-top TDZ. The EXT 29 negative result (broader cluster 106→104 on generator destructure + closure-get tests) reverted cleanly; EXT 30 ran the Pin-Art probe surfacing four implicit constraints (α/β/γ/δ); EXT 31-33 implemented Constraint β incrementally per scope surface + closed Constraint γ via compile-time guards (per Rule 26); EXT 34 re-enabled EXT 29's substrate prefix cheaply.

The chain delivered 10 closed TDZ enforcement sub-shapes across 15 rungs with comparable total LOC to a single naive monolithic TDZ rewrite (~500 LOC) but at distributed, measurable-yield granularity. Three new standing rules promoted (24, 25, 26). One Pin-Art-probe rung (IR-EXT 30) explicitly identified the implicit constraints — that rung is itself a methodological artifact, not a substrate move, and produced no per-rung yield but unlocked the subsequent four substrate-bearing rungs.

## VI. Falsifier

The discipline's falsifier is the next locale's chapter-close measurement. If a locale follows all five phases + applies cross-pipeline rules and produces sub-noise or negative cumulative yield (cumulative test262 movement < 1pp across 10+ rungs), the discipline is partially falsified at that locale's substrate-class — either the rules don't compose as claimed, or a missing phase / missing rule failed to surface in time.

Conversely, if a locale follows the discipline and produces positive cumulative yield comparable to monolithic-rewrite alternatives, the discipline is corroborated for that substrate-class.

Forward-derived prediction: the discipline composes with rule 13's prospective application thesis (cf. `apparatus/docs/standing-rule-13-prospective-application.md`). When the deeper-layer closure can be designed from first when C1-C4 conditions hold, the prefix-revert-deeper-layer trajectory shortens to one round. Prospective application + the five-phase pipeline together should produce ≤3-rung closures for substrate-classes whose constraints are already named in the apparatus.

## VII. Cross-corpus references

- **Doc 540** — Pin-Art apparatus formalization; the substrate methodology this discipline operationalizes.
- **Doc 541 Appendix E** — SIPE-T scale-invariance; identifies the inspect-then-iterate compound-discovery pattern (Phase 5) as a SIPE-T instance.
- **Doc 581** — resume-vector discipline; seed/trajectory pair as the operational substrate of every phase.
- **Doc 727 §X** — basin-stability discipline; append-only protocol for findings.md addenda + rule promotion.
- **Doc 729** — resolver-instance pattern; the architectural target the discipline's substrate work serves.
- **Doc 737** — locale as coordinate; the spawn-phase target.
- **Doc 740 / 741** — multi-tier cascade-revival; rule 13's theoretical anchor + empirical materialization, the closest existing primary articulation to this doc's thesis.
- **Doc 742** — resolver-instance boundary contract; the corpus-tier consolidation of rules 14+15's TS-parity arc outcomes.

## VIII. Status

Working draft. Candidate for corpus promotion after keeper review. Located at `docs/engagement/prospective/substrate-shaped-work-discipline.md` per `apparatus/docs/repository-apparatus.md` §0 promotion path. On promotion: target corpus number Doc 744 or later; mirror to `docs/corpus-ref/`; add cross-references from `apparatus/docs/predictive-ruleset.md` + `pilots/rusty-js-jit/findings.md` + `CLAUDE.md` § Substrate-shaped-work discipline.
