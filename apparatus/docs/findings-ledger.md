# Findings Ledger

A standing apparatus-tier index of empirical findings surfaced at the substrate-rung tier of this engagement: substrate-pattern findings, abstract-op-discipline findings, measurement-discipline findings, error-propagation findings, apparatus-tier findings, source-identifier-convention findings. The findings-ledger is to substrate empirical discovery what the deferrals-ledger is to surfaced-but-not-founded candidates: a place where findings that otherwise live only in per-locale `findings.md` files (or, worse, only in trajectory entries) are bound into the cybernetic loop's readable surface and made cross-referenceable.

## Why a ledger

Per-locale `findings.md` files compound substrate insight within a locale's coordinate scope. `pilots/rusty-js-jit/findings.md` is the canonical 26-rule + 16-addendum ledger for the JIT-tier substrate (cited in CLAUDE.md §Substrate-shaped-work discipline as the "canonical 26-rule ledger"). `apparatus/docs/predictive-ruleset.md` is the consolidated 15-rule derived view downstream of findings. Between these two tiers, a gap:

- Per-locale findings live at `pilots/<coord>/findings.md` (and, in nested cases, `pilots/<coord>/<sub>/findings.md`). They are coordinate-local; a future reader who is not in that locale's scope rarely finds them.
- The JIT canonical ledger compounds JIT-tier substrate findings but does not host findings from other engine pillars (parser, runtime intrinsics, package manager, capability-passing) or from apparatus-pilots.
- The 15-rule predictive view is the most-distilled tier — load-bearing for substrate-shaped-work discipline — but it does not preserve per-finding traceability back to the originating locale + rung + substrate move.

This ledger restores the binding. Each entry records:

1. **Finding ID** — the locale-tag-prefixed identifier the original `findings.md` (or trajectory) used (e.g., TAWR.3, IHI.1, OSR.1, TL.1, TAECSF.1, BBND-yield).
2. **Locale** — the coordinate path where the finding was authored.
3. **Originating rung** — the trajectory entry that motivated the finding.
4. **Class** — one of:
   - **substrate-pattern** — a shape of substrate move that recurs across rungs or locales.
   - **abstract-op-discipline** — a rule about how an abstract op behaves spec-faithfully or how its callers must interact with it.
   - **measurement-discipline** — a rule about how an apparatus instrument must be operated (multi-run, detectability, three-probes, canonical-fuzz, etc.).
   - **error-propagation** — a rule about Result threading, error-kind selection, side-channels, or signature lifts.
   - **apparatus-tier** — a rule about apparatus design: locale-as-probe, locale coordinate sizing, ledger discipline, deferral semantics.
   - **source-identifier-convention** — a rule about the source-identifier coordinate system per Doc 738 (prefix / install helper / registration tier).
   - **other** — named in the entry body.
5. **Body summary** — one-sentence distillation of the finding's claim.
6. **Promotion status** — one of:
   - **standing-rule** — promoted to `apparatus/docs/predictive-ruleset.md` as one of the 15 + N consolidated rules. Cite the rule number.
   - **finding-addendum** — recorded in `pilots/rusty-js-jit/findings.md` as part of the canonical 26-rule + Addenda I-XVI structure. Cite the rule or addendum.
   - **cross-locale-recurrence** — observed in 2+ locales but not yet consolidated; promotion-ready or one-more-observation.
   - **trajectory-embedded** — surfaced in trajectory.md but not extracted to a `findings.md`. Tracked here for visibility; promotion-pending.
   - **none** — locale-specific only, no promotion expected.

## Discipline (append-only)

Per Doc 727 §X basin-stability discipline (same as findings.md, deletions-ledger.md, deferrals-ledger.md, audit-ledger.md): this file is **append-only**. New entries go at the bottom in chronological order. Older entries are never edited; if a finding is retracted (rare; see NLC.0 precedent in JIT findings.md) or promoted to a standing rule, append a new entry citing the prior with a back-reference and update the prior entry's Promotion status field in place (the single allowed in-place edit, per the deletions-ledger.md precedent for status-flips on prior entries).

## Discovery hook

A substrate rung that closes a chapter or surfaces a recurring pattern owes the apparatus a finding entry, not merely a trajectory note. The trajectory note records the rung-local insight; the findings-ledger entry makes the finding cross-referenceable from outside the originating locale. Promotion of a cross-locale-recurrence pattern to standing-rule status (`apparatus/docs/predictive-ruleset.md`) requires ≥2 locales corroborating the pattern + arbiter or keeper sign-off; the audit-ledger entry that authorizes the promotion is the discovery hook for the predictive-ruleset edit.

The standing rule the ledger formalizes: **a substrate rung that surfaces a generalizable finding owes the apparatus an aggregator entry, not merely a coordinate-local `findings.md` line.** The locale-local file records the rung-local reasoning; the apparatus ledger makes the finding readable from outside the originating coordinate.

---

## Per-locale findings.md inventory (audit 2026-05-30)

| Path | Findings | Notes |
|---|---|---|
| `pilots/rusty-js-jit/findings.md` | 26 standing rules + Addenda I-XVI | Engagement-wide canonical ledger. Compounds across JIT + IHI + OSR + TL + JSF + CharCode-EXT + TB and other JIT-touching locales. Rules 1-15 + 17-22 + 23-26 consolidated in `apparatus/docs/predictive-ruleset.md` (rule 16 does not exist in canonical source; preserved per Doc 727 §X). Rules 17-22 consolidation landed 2026-05-30 per Entry 011. |
| `pilots/interp-hot-intrinsics/findings.md` | IHI.1 | IC cache lifetime; promoted as Finding VIII.4 in JIT ledger. |
| `pilots/rusty-js-jit/osr/findings.md` | OSR.1, OSR.2 | OSR invoke calling-convention + loop-boundary; promoted as Findings VIII.2 + VIII.3. |
| `pilots/rusty-js-jit/top-level/findings.md` | TL.1, TL.2 | Whole-body bail bounds + Φ value-domain; promoted as Findings VII.2 + VII.3. |
| `pilots/parser-early-error-residual/block-bound-names-dup/findings.md` | BBND yield-analysis | Five-condition multiplier; non-standard structure (yield-analysis rather than discrete findings). Standing-rule-16 candidate pending corpus Doc 743 corroboration. |

Trajectory-embedded findings discovered but not yet extracted to a `findings.md`:

- **TAECSF.1** at `pilots/ta-element-coercion-spec-faithful/trajectory.md` TAECSF-EXT 0 (2026-05-30) — narrow dispatcher beats wide signature lift for Result-threading. Promoted-readiness: one-more-observation (see Entry 005 below).

---

## Entries

### Entry 001 — JIT-tier 26 rules + Addenda I-XVI (canonical ledger reference)

- **Finding IDs**: JIT Rules 1–26 + Addenda I–XVI (full enumeration at `pilots/rusty-js-jit/findings.md`).
- **Locale**: `pilots/rusty-js-jit/`.
- **Originating rungs**: distributed across the JIT locale's trajectory; cross-locale compounding from `pilots/interp-hot-intrinsics/`, `pilots/rusty-js-jit/osr/`, `pilots/rusty-js-jit/top-level/`, and JIT-touching siblings.
- **Class**: aggregator (mixed substrate-pattern + abstract-op-discipline + measurement-discipline + apparatus-tier classes per rule).
- **Body summary**: the canonical engagement-wide rule ledger. Rules 1–15 and 23–26 are consolidated in `apparatus/docs/predictive-ruleset.md` as the 15-rule + 4-addendum derived view. Rules 16–22 are present in addenda but not yet consolidated to the predictive view. The 16 addenda (I–XVI) layer the rules with deeper context, multi-tier cascade analyses, and Pin-Art validation traces.
- **Promotion status**: standing-rule (multiple). Rules 4, 5, 6, 11, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26 consolidated to `apparatus/docs/predictive-ruleset.md`. Rule 16 does not exist in findings.md (canonical numbering jumps 15 → 17 in Addendum XII); preserved as-is per Doc 727 §X append-only discipline. Rules 17–22 consolidated 2026-05-30 per helmsman session under keeper directive Telegram 10574; see Entry 011 below for the consolidation back-reference.

### Entry 002 — IHI.1 (Per-Frame IC caches don't amortize)

- **Finding ID**: IHI.1.
- **Locale**: `pilots/interp-hot-intrinsics/`.
- **Originating rung**: IHI trajectory (cross-reference per Entry 001 promotion to JIT Finding VIII.4).
- **Class**: substrate-pattern.
- **Body summary**: per-frame IC caches do not amortize across invocations; substrate moves that introduce caching must lift the cache to Runtime-lifetime or accept that the cache is a single-frame instrument.
- **Promotion status**: standing-rule (rules 8 + 9 of the 15-rule consolidated view) + finding-addendum (JIT Finding VIII.4).

### Entry 003 — OSR.1 + OSR.2 (JIT calling-convention + loop-boundary)

- **Finding IDs**: OSR.1, OSR.2.
- **Locale**: `pilots/rusty-js-jit/osr/`.
- **Originating rungs**: OSR-EXT trajectory entries (see locale findings.md for rung tags).
- **Class**: abstract-op-discipline (OSR.1) + substrate-pattern (OSR.2).
- **Body summary**: OSR.1 — the JIT calling convention's params-only-as-args closure blocks OSR-invoke; three alternatives named (marshal / extern / frame-pointer). OSR.2 — for/while loop forward-exit jumps lower out-of-bounds; only do-while extracts cleanly. Three boundary-handling options named.
- **Promotion status**: finding-addendum (JIT Findings VIII.2 + VIII.3). Both compose into Standing Rule 11 (5-axis pre-spawn coverage: A4 locals-marshaling + A5 emission-shape).

### Entry 004 — TL.1 + TL.2 (whole-body bail bounds + Φ value-domain)

- **Finding IDs**: TL.1, TL.2.
- **Locale**: `pilots/rusty-js-jit/top-level/`.
- **Originating rungs**: TL trajectory.
- **Class**: abstract-op-discipline.
- **Body summary**: TL.1 — whole-body JIT bail bounds (b-narrow) reclaim to 0% yield on mixed-alphabet fixtures; the bail must be inner-loop-localized. TL.2 — JIT calling-convention encodes non-Number / non-Object Values to 0.0; Φ-tier value-domain encoding breaks IC fast-paths downstream.
- **Promotion status**: finding-addendum (JIT Findings VII.2 + VII.3) + standing-rule (Rule 11 5-axis A2 op-set + A3 value-domain).

### Entry 005 — TAECSF.1 (narrow dispatcher beats wide signature lift)

- **Finding ID**: TAECSF.1.
- **Locale**: `pilots/ta-element-coercion-spec-faithful/`.
- **Originating rung**: TAECSF-EXT 0 (2026-05-30, founding rung; trajectory-embedded, no `findings.md` yet).
- **Class**: error-propagation.
- **Body summary**: when a coercion abstract op needs to propagate its error from a deep storage path through a non-Result-returning intermediate function (>~10 callers), prefer a narrow new dispatcher in the Result-returning caller's frame over lifting the intermediate's signature. The narrow dispatcher's blast radius is the named call-site; the wide lift's blast radius is every caller of the intermediate.
- **Promotion status**: standing-rule (`apparatus/docs/predictive-ruleset.md` Rule 30 — Narrow-dispatcher cascade-revival as preferred error-propagation discipline; promoted 2026-05-30 at the post-ASTA-EXT 0 follow-up cycle per keeper directive Telegram 10616). Third-instance corroboration completed: TAECSF.1 (TAECSF-EXT 0) + TABSC.2 (TABSC-EXT 0 + 1 substrate-prefix amortization) + ASTA.1 (ASTA-EXT 0 `object_set_checked`). See Entry 019 below for the promotion back-reference.

### Entry 006 — BBND yield-analysis (five-condition multiplier)

- **Finding ID**: BBND-yield (non-standard naming; the file documents a yield-analysis rather than discrete `Finding <TAG>.<N>` entries).
- **Locale**: `pilots/parser-early-error-residual/block-bound-names-dup/`.
- **Originating rung**: BBND trajectory.
- **Class**: apparatus-tier.
- **Body summary**: five-condition multiplier for substrate-tier yield — spec-rule bijection + cross-product generation + one-rule-one-site + static-semantics tier + matrix-driven targeting. The conditions compose multiplicatively rather than additively; absence of any single condition collapses the yield. Fielding-REST-correspondence-shaped constraint stacking.
- **Promotion status**: cross-locale-recurrence pending; one-more-observation. Proposed for Standing Rule 16 + corpus Doc 743. Awaiting ≥2 additional locales corroborating the constraint-stacking pattern.

### Entry 007 — Cross-locale recurrence: IC cache lifetime dependency

- **Pattern name**: IC-cache-lifetime-amortization-requires-Runtime-tier-storage.
- **Locales corroborating**: `pilots/interp-hot-intrinsics/` (IHI.1) + `pilots/rusty-js-jit/` (Findings VIII.4 + VIII.5 + VIII.6).
- **Class**: substrate-pattern.
- **Body summary**: per-frame IC stores do not amortize; storage must live at Runtime tier or above to compound across invocations. The fixture-dependency and bytecode-rewrite tiers (VIII.5 + VIII.6) are sub-patterns of this rule.
- **Promotion status**: standing-rule (Rule 8 + Rule 9 of the 15-rule consolidated view). Cross-locale recurrence count: 2 locales (IHI + JIT subtree).

### Entry 008 — Cross-locale recurrence: JIT calling-convention boundary constraints

- **Pattern name**: JIT-boundary-encoding-shapes-IC-fast-path-eligibility.
- **Locales corroborating**: `pilots/rusty-js-jit/osr/` (OSR.1) + `pilots/rusty-js-jit/top-level/` (TL.2) + `pilots/rusty-js-jit/` (Findings VII.2 + VII.3 + VIII.2 + VIII.3).
- **Class**: abstract-op-discipline.
- **Body summary**: JIT boundary encoding (locals marshaling + value-domain + emission shape) gates whether downstream IC fast-paths can engage. Substrate moves at the JIT must explicitly consider all three axes; omission collapses yield to 0%.
- **Promotion status**: standing-rule (Rule 11 5-axis pre-spawn coverage check: A1–A5). Cross-locale recurrence count: 3 locales (OSR + TL + JIT root).

### Entry 009 — Cross-locale recurrence: multi-tier cascade-revival

- **Pattern name**: deeper-layer-closure-required-when-tier-N-revert.
- **Locales corroborating**: `pilots/rusty-js-jit/` (Finding II.3 / Rule 13) + `pilots/rusty-js-jit/top-level/` (TL.1) + `pilots/typed-array-wrong-result/` (TAWR-EXT 6 NEGATIVE Rule-13 REVERT closing Phase-5 inflection of the array-exotic arc) + `pilots/rusty-js-ir/` (EXT 25→26 and EXT 29→34 per CLAUDE.md §Substrate-shaped-work discipline Addendum XVI).
- **Class**: abstract-op-discipline.
- **Body summary**: a negative-result rung at tier N is not the substrate's terminal state; it points to a deeper-layer closure at tier N-1 or N+1. Revert the negative round's code, retain the trajectory entry + diagnosis, identify the deeper-layer closure that the negative round's design pointed toward, and implement the deeper-layer closure as the next round. The substrate prefix left on disk often becomes the cheap enabler of the deeper-layer closure (per Finding IR.33 cumulative substrate amortization).
- **Promotion status**: standing-rule (Rule 13 + standing-rule-13-prospective-application.md). Cross-locale recurrence count: 4+ locales.

### Entry 010 — Cross-locale recurrence: baseline-inspection-as-locale-probe

- **Pattern name**: locale-as-probe-surfaces-correct-coordinate.
- **Locales corroborating**: `pilots/tokenization-arc-NLC/` (NLC.0 retraction → real-coordinate parser-permissiveness; NLC.1 lock-in) + `pilots/json-stringify-fundamental/` (JSF) Finding VII.1.
- **Class**: apparatus-tier.
- **Body summary**: at locale founding, baseline-inspect against current substrate state + inspect a sample of failures. If inspection reveals the substrate move-shape is at a different coordinate than the seed declared, treat the locale as a probe that surfaced the real target; land the surfaced-coordinate move first. NLC.0 was retracted in exemplary form (eval-error-class was a probe-surface; the real coordinate was parser permissiveness).
- **Promotion status**: standing-rule (Rule 23, formalized 2026-05-25). Cross-locale recurrence count: 2 locales.

---

## Anomalies surfaced at audit 2026-05-30

1. **Rules 16–22 unconsolidated**: `pilots/rusty-js-jit/findings.md` Addenda XII–XIII name rules 16–22, but the predictive-ruleset.md derived view only consolidates 1–15 + 23–26. A consolidation pass would close the gap. Tracked for future audit-ledger entry; not addressed by this findings-ledger introduction.

2. **TAECSF.1 trajectory-embedded only**: locale `pilots/ta-element-coercion-spec-faithful/` (founded 2026-05-30) records TAECSF.1 in `trajectory.md` but has no `findings.md`. The per-locale convention (per JIT + IHI + OSR + TL precedent) is to extract single-rung findings to `findings.md` once a second rung in the locale lands; current single-rung state suggests deferring extraction until the second rung. Recorded here as Entry 005 with trajectory-embedded promotion-status.

3. **BBND yield-analysis structure**: `pilots/parser-early-error-residual/block-bound-names-dup/findings.md` documents a yield-analysis rather than discrete `Finding <TAG>.<N>` entries. This is acceptable per the §Discipline append-only convention but is non-standard relative to other findings.md files in the repo. The structure is appropriate to what BBND surfaced (a multiplier rather than a discrete pattern); promotion as Standing Rule 16 will adopt the discrete form.

4. **No apparatus-pilot findings.md files**: the apparatus-pilots at `pilots/apparatus/*/` do not carry `findings.md`. Their findings live in `apparatus/docs/` documents (e.g., `apparatus/docs/repository-apparatus.md`, `apparatus/docs/predictive-ruleset.md`, `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`). This is consistent with the bilateral pilot tier distinction (substrate-pilots author trajectory + findings inside the pilot directory; apparatus-pilots author articulation in `apparatus/docs/`). No anomaly; documented here for reader orientation.

5. **CHARMS-EXT 9 self-checking discipline**: per source-identifier convention findings, source-identifier coordinates (Doc 738) are described in CLAUDE.md but no `findings.md` aggregates the convention's self-check pattern (a name whose prefix and install helper disagree is a bug shape). Surfaced as future-extract candidate; promotion-status: none until a sibling source-identifier-convention finding lands at a different naming axis.

### Entry 011 — Rules 17–22 consolidation (2026-05-30)

- **Pattern name**: predictive-ruleset-canonical-source-consolidation.
- **Locales corroborating**: `pilots/rusty-js-jit/findings.md` (canonical source) Addenda XII + XIII (rules 17, 18, 19, 20, 21, 22).
- **Class**: apparatus-tier.
- **Body summary**: rules 17–22 articulated at JIT findings.md Addenda XII + XIII (Standing rule 17 EPSUA.6/EPSUA.7; Standing rule 18 SPBC.2; Standing rule 19 SPTW.2; Standing rule 20 NACR.1; Standing rule 21 RS.1+RS.2+CP.4; Standing rule 22 SMPT.4+ALST.2) but never consolidated to the derived-view `apparatus/docs/predictive-ruleset.md`. Consolidation closes the gap surfaced by audit-ledger Entries 001 / 002 / 003 / 004. Note: rule 16 does not exist in the canonical source (findings.md jumps 15 → 17 in Addendum XII); the gap is preserved per Doc 727 §X append-only discipline.
- **Promotion status**: standing-rule (rules 17–22 in `apparatus/docs/predictive-ruleset.md`). Cross-locale recurrence count: rules 17–22 originate in the JIT canonical ledger; cross-locale instantiations include rule 21 re-instantiated 2026-05-30 by TAECSF-EXT 0 probe (deferrals-ledger Entry 010 un-defer, ~60 LOC narrow-dispatcher probe answered the option-(i)-vs-option-(ii) bifurcation per the probe-first rule).
- **Authored actions**: commit (this commit) appends rules 17–22 to `apparatus/docs/predictive-ruleset.md` between rule 15's separator and rule 23's heading; updates the §Status note to record the consolidation date + cite keeper Telegram 10574 + flag the rule-16-does-not-exist preservation. Entry 001 of this ledger has its Promotion status field flipped in-place to enumerate the now-consolidated rules.

### Entry 012 — TAECSF.3 (engine-architectural Value-cell-aliasing vs spec coercion) (2026-05-30)

- **Finding ID**: TAECSF.3.
- **Locale**: `pilots/ta-element-coercion-spec-faithful/`.
- **Originating rung**: TAECSF-EXT 1 NEGATIVE convergent diagnosis (2026-05-30, keeper directive Telegram 10580).
- **Class**: substrate-pattern (engine-architectural).
- **Body summary**: the engine's `ArrayBufferData.data: Vec<Value>` stores Values at byte indices rather than actual bytes; views aliased to the same buffer share Value cells. Spec-faithful coercion at the SetIndex dispatcher breaks the view-aliasing pass-through invariant the test262 harness exploits for resizable-buffer setup. Integer-kind coercion at the SetIndex dispatcher CANNOT land correctly without a precursor architectural rung that migrates buffer storage to `Vec<u8>` with NumberToRawBytes encoding per ECMA-262 §6.1.6.1.
- **Promotion status**: standing-rule (`apparatus/docs/predictive-ruleset.md` Rule 27 — Substrate-spec-correctness vs engine-architecture conflict; promoted 2026-05-30 at findings-disposition cycle 2 per keeper directive Telegram 10600). Second observation received via TABSC-EXT 0's empirical validation of the byte-storage architectural rectification (commit f2107bb6). See Entry 014 below for the promotion back-reference.
- **Implication for arc / new-locale spawning**: candidate new sibling locale `typed-array-byte-storage-conformance` (precursor architectural rung) to be founded when keeper authorizes the scope — multi-rung substrate move; massively wider than this locale's telos; would unblock sub-substrates (a) integer-kind + (b) Float32 canonical-NaN within `pilots/ta-element-coercion-spec-faithful/`. Currently DEFERRED pending keeper direction.

### Entry 013 — APP.PIPELINE-1 (dynamic-typing pipeline starts the type-specific alphabet at runtime) (2026-05-30)

- **Finding ID**: APP.PIPELINE-1.
- **Locale**: apparatus-tier (not locale-scoped; cross-pillar).
- **Originating rung**: pipeline alphabet audit (`apparatus/docs/pipeline-alphabet-audit-2026-05-30.md`), under keeper directive Telegram 10582.
- **Class**: apparatus-tier (architectural-design discipline).
- **Body summary**: in a dynamically-typed language pipeline (lexer → parser → IR → bytecode → runtime → storage), type-specific element semantics cannot be encoded in any tier upstream of the runtime introspection that distinguishes the type. The "beginning of the alphabet" for a typed-array element write is the first introspection site that distinguishes typed-array receivers from generic objects — for this engine, the canonical-numeric-index branch of the `Op::SetIndex` handler at `interp.rs:14137`. Substrate moves that attempt to encode TA-specific semantics in upstream tiers require parse-time type proof that ECMAScript does not provide. Sound rectification of architectural-coercion conflicts must land at Tier 5 (dispatch) or Tier 6 (storage); never upstream.
- **Promotion status**: standing-rule (`apparatus/docs/predictive-ruleset.md` Rule 28 — Dynamic-typing pipeline starts type-specific alphabet at runtime introspection; promoted 2026-05-30 at findings-disposition cycle 2 per keeper directive Telegram 10600). Second observation received via TABSC-EXT 0's empirical instantiation of the rectification-at-Tier-6 prediction (commit f2107bb6). See Entry 014 below for the promotion back-reference.
- **Composes with**: Doc 729 (resolver-instance pattern; this finding is a worked instance of the per-tier alphabet articulation at the engine's runtime tier); Doc 738 (source-identifier coordinate; the `__kind` slot is the runtime-tier source-identifier that the dispatch reads). Lattice with TAECSF.3 (engine-architectural Value-cell-aliasing): APP.PIPELINE-1 names WHY the architectural rectification must land at the storage tier rather than upstream; TAECSF.3 names WHAT must change at that tier.

### Entry 014 — Findings-disposition cycle 2 promotion (TAECSF.3 + APP.PIPELINE-1 → Rules 27 + 28) (2026-05-30)

- **Pattern name**: findings-disposition-cycle-2-standing-rule-promotion.
- **Locales corroborating**: `pilots/ta-element-coercion-spec-faithful/` (TAECSF.3 second observation via the post-TABSC byte-storage cascade); `pilots/typed-array-byte-storage-conformance/` (APP.PIPELINE-1 second observation via the rectification-at-Tier-6 empirical instantiation).
- **Class**: apparatus-tier.
- **Body summary**: TABSC-EXT 0 (commit f2107bb6) landed the byte-storage architectural rectification predicted by both findings; the cascade-revival of TAECSF sub-substrates + the +5 cluster yield + the 10/10 cascade probe constituted the second observation for both. At findings-disposition cycle 2 (audit-ledger Entry 007) per keeper directive Telegram 10600, both findings were dispositioned to candidate (4) promote-to-standing-rule per the protocol's Step 3 sequence. Rules 27 + 28 appended to `apparatus/docs/predictive-ruleset.md` between rule 26's separator and the Standing instruments section. Entries 012 + 013 Promotion status fields flipped in-place to standing-rule per the §Discipline single-allowed-edit.
- **Promotion status**: standing-rule (Rules 27 + 28 in `apparatus/docs/predictive-ruleset.md`). Cross-locale recurrence count: TAECSF.3 in 2 locales (TAECSF + TABSC); APP.PIPELINE-1 in 2 sites (pipeline-alphabet-audit + TABSC empirical instantiation).
- **Authored actions**: commit (this commit) — Rules 27 + 28 appended; predictive-ruleset §Status note refreshed; Entries 012 + 013 Promotion status flipped in-place; this entry appended.

### Entry 015 — TABSC.1 (cascade-revival can materialize cluster yield at the precursor rung when precursor IS the encoding tier) (2026-05-30)

- **Finding ID**: TABSC.1.
- **Locale**: `pilots/typed-array-byte-storage-conformance/`.
- **Originating rung**: TABSC-EXT 0 (LANDED 2026-05-30, commit f2107bb6).
- **Class**: apparatus-tier (architectural-design discipline + cascade-revival pattern amendment).
- **Body summary**: when the upstream constraint-closure substrate move ALSO produces the downstream tier's required encoding format (here, byte storage IS both the upstream `Vec<Value>`-vs-`Vec<u8>` constraint-closure AND the byte-encoding tier per ECMA-262 §6.1.6.1), the cascade-revival materializes immediately at the precursor rung rather than at a separate subsequent tier-closure rung. Amends Doc 740 §II.2 P4's "cumulative reclaim materializes at the final-tier-closure round" with: "cumulative reclaim may materialize at the precursor rung itself when the precursor's substrate is structurally complete for the downstream tier's requirements." TABSC-EXT 0 produced +5 cluster cells + 10/10 cascade probe at the precursor rung where Doc 740 + Doc 741 predicted ≈0% substrate-introduction signature.
- **Promotion status**: trajectory-and-findings-embedded; one-more-observation. Candidate Doc 740 §II.2 P4 amendment; promotion at next cross-locale recurrence (a future precursor rung in a different engagement pillar that ALSO serves as the downstream tier's encoding/required-format producer, exhibiting the same surplus-at-precursor signature).
- **Composes with**: Doc 739 (single-tier cascade-revival; TABSC.1 narrows the WHEN of cascade materialization), Doc 740 (multi-tier cascade-revival; TABSC.1 amends §II.2 P4's epistemic shape), Doc 741 (rule-11 5-axis; TABSC.1 noted that the 5-axis check at TABSC-EXT 0 produced higher confidence than Doc 740's substrate-introduction prediction would suggest, indicating the 5-axis check may already incorporate the precursor-as-encoding-tier signal).

### Entry 016 — SAMPLE.1 (cross-family missing-TypeError-throw pattern dominates the residual after byte-storage closure) (2026-05-30)

- **Finding ID**: SAMPLE.1.
- **Locale**: cross-pillar / apparatus-tier (test262-sample residual surfaced at the canonical 2026-05-30 measurement).
- **Originating rung**: audit-ledger Entry 008 (test262 sample canonical 2026-05-30; 88.7% / 6816 PASS / 865 FAIL). Residual failure-class clustering surfaced via jq summarization.
- **Class**: abstract-op-discipline (cross-locale spec-correctness pattern).
- **Body summary**: of the 865 failing cells in the 2026-05-30 canonical test262-sample, **86 cells (~10% of the residual)** share the exact reason shape `"Expected a TypeError to be thrown but no exception was thrown at all"`. The pattern recurs across 11+ surface families: built-ins/Promise (20), built-ins/Array (15), built-ins/Object (12), language/statements (11), built-ins/Map (8), built-ins/WeakMap (7), built-ins/Set (4), built-ins/WeakSet (3), built-ins/String (3), language/expressions (2), built-ins/Error (1). The pattern's shape is consistent: spec mandates a TypeError on the operation (typically a non-callable callback, an invalid receiver, a frozen-property write); cruft accepts the operation as a no-op silently. Per Rule 22 (partial-exemplar-closure as substrate-axis discriminator), this is a single axis sharing a large exemplar across families — a candidate for one substrate move (a `validate_callable_or_throw` discipline added to method-dispatch wrappers + receiver-brand-check throw-discipline added to the relevant abstract ops) that flips a substantial share of the 86 cells at once. Per Rule 17 (pre-scoping per-reason-pattern segmentation), sub-locales scoped against the per-family count (e.g., "fix Promise's TypeError throws") will under-deliver vs sub-cluster scope ("fix the missing-TypeError-throw discipline across all method-dispatch wrappers"); the cross-family count is the predictive unlock figure.
- **Promotion status**: cross-locale-recurrence (11+ surface families); promotion-ready for a substrate-spawn at the dispatch-wrapper tier. Doc 721 Step 3 highest-shared-layer analysis: the wrapper sites where method invocation validates `isCallable(cb)` or `isObject(receiver)` are the candidate alphabet top; closure there cascades the 86 cells to PASS subject to per-cell completeness in other pipelines (Doc 721 Step 4 U vs |G| delta). Predicted unlock U: 60-80 cells (subset of 86 that are otherwise complete in other pipelines); the remaining 6-26 will migrate to next-exit-symptom per Step 5 iteration.
- **Composes with**: Rule 17 (pre-scoping segmentation), Rule 18 (brand-check at registration wrapper, not in shared impl — confirms the substrate-move site is the registration wrapper for receiver-brand-check throws; not the shared impl), Rule 20 (substrate-discipline coherence drift across parallel helpers — the 11+ families exhibiting the same reason-shape is exactly the cross-module reason-shape coherence rule 20 predicts).

### Entry 017 — SESSION.1 (apparatus-driven compound-yield within a single session through cascade-revival + substrate-prefix amortization) (2026-05-30)

- **Finding ID**: SESSION.1.
- **Locale**: methodology-tier (engagement-scope; not coordinate-local).
- **Originating rung**: full 2026-05-30 session — 18 commits; 4 apparatus docs introduced; 5 substrate locales founded; 28 standing rules in predictive-ruleset (up from 26); 3 deferrals dispositioned; gates net positive: TAWR 63 → 71 (+8), TAMM 82 → 87 (+5), diff-prod 61/51 → 64/48 (+3 PASS), **test262-sample 84.3% → 88.7% (+4.4 PP / +373 PASS)**.
- **Class**: apparatus-tier (methodology-discipline).
- **Body summary**: a single session that compounds apparatus discipline (audit-ledger + findings-ledger + findings-disposition-protocol + pipeline-alphabet-audit) with substrate work driven by the apparatus's predictions can produce engagement-rate movement (4.4 percentage points in 3 days; 2-3 percentage points attributable to this single session) by realizing Doc 739 cascade-revival + Doc 740 multi-tier compound-yield + Doc 741 substrate-prefix amortization in close temporal sequence. The compounding shape: each apparatus document authored becomes the predictive scaffolding for the next substrate move; each substrate prefix becomes the cheap enabler of subsequent rungs; each finding's promotion-readiness flips on the next rung that empirically tests it. The session-tier rate-shift is the integral of the cascade across many tiers, not a single landmark commit.
- **Promotion status**: trajectory-and-findings-embedded; one-more-observation. Candidate apparatus standing-rule on session-tier compound-yield discipline (when apparatus + substrate work compound in close temporal sequence with explicit cascade-revival framing, the engagement-rate movement is substantially larger than the sum of individual rungs would predict). Awaiting a second session-tier instance for promotion. The day's sequence — corpus reading → apparatus authoring → substrate proposal → substrate landing → cascade-revival empirical validation → re-measurement — composes a reproducible session-tier discipline pattern.
- **Composes with**: Doc 739 + Doc 740 + Doc 741 + Doc 744 (the cascade-revival + pipeline-form discovery foundation this session exercised); Finding TABSC.1 (cascade-revival at precursor rung); Finding TABSC.2 (substrate-prefix amortization across cascade-revival rungs); Finding IR.33 (cumulative substrate amortization).

### Entry 018 — DET.1 / DET.2 / DET.3 (measurement-determinism finding triplet) (2026-05-30)

- **Pattern name**: measurement-determinism formalization.
- **Locales corroborating**: apparatus-tier (engagement-wide); test262-sample re-run (audit-ledger Entries 008 + 009); TAMM cluster triple-run during TAECSF-EXT 1 diagnosis.
- **Class**: apparatus-tier (measurement-discipline).
- **Body summary**: three findings recorded in `apparatus/docs/measurement-determinism-2026-05-30.md`:
  - **DET.1** — n=2 byte-identity is sufficient evidence of instrument determinism at the current substrate maturity. **Promoted to Rule 29.**
  - **DET.2** — Most engagement regression-gate instruments are Class A (deterministic) at the current substrate maturity (test262-sample, TAMM, TAWR, RBDPA, diff-prod). CRB is Class B. Trajectory-and-findings-embedded; one-more-observation pending Rule 29 cross-locale instance.
  - **DET.3** — Apparatus-tier discipline-cost reallocation from Rule 29 short-circuit (~4× wall-clock saved per measurement-protocol invocation). Trajectory-and-findings-embedded; one-more-observation pending session-tier instance.
- **Promotion status**: DET.1 → standing-rule (Rule 29 promoted 2026-05-30 same change). DET.2 + DET.3 → trajectory-and-findings-embedded; one-more-observation each.
- **Composes with**: Rule 2 (multi-run protocol — Rule 29 amends via back-reference per Doc 727 §X), Rule 3 (detectability budget), Rule 9 (raw-pointer cache stability — apparatus-tier analog), Rule 23 (founding baseline-inspection), audit-ledger Entries 008 + 009 (the originating empirical observations).
- **Authored actions**: commit (this commit) — Rule 29 appended to predictive-ruleset.md between Rule 28 and Standing instruments; predictive-ruleset §Status note refreshed; `apparatus/docs/measurement-determinism-2026-05-30.md` introduced with formalized findings (DET.1/2/3) + exploratory extensions §V (apparatus-tier V.1.a-e + substrate-tier V.2.a-f) + cascade-revival composition §V.3 + falsifier surface §VI; this entry appended.

### Entry 019 — Narrow-dispatcher cascade-revival pattern → Rule 30 (3-instance promotion 2026-05-30)

- **Pattern name**: narrow-dispatcher-cascade-revival-promotion.
- **Locales corroborating**: `pilots/ta-element-coercion-spec-faithful/` (TAECSF.1, TAECSF-EXT 0 `typed_array_set_index_checked`); `pilots/typed-array-byte-storage-conformance/` (TABSC.2, TABSC-EXT 0 + 1 substrate-prefix amortization with `number_to_raw_bytes`); `pilots/array-strict-throw-discipline/` (ASTA.1, ASTA-EXT 0 `object_set_checked`).
- **Class**: error-propagation + apparatus-tier (engineering-shape discipline).
- **Body summary**: three locale-instance corroborations of the narrow-Result-returning-dispatcher pattern for spec-error propagation through non-Result intermediates. Each instance: a substrate prefix (the spec-correct abstract op) already exists; the dispatcher introduces a Result-returning narrow consumer above the unchanged non-Result helper; user-visible call paths route through the checked dispatcher; internal infrastructure paths continue using the unchecked helper. Blast radius is bounded by the consumer set, not by the helper's callers. Promoted at the post-ASTA-EXT 0 follow-up cycle per the findings-disposition protocol §III three-instance threshold (TAECSF.1 + TABSC.2 + ASTA.1 = 3 cross-locale corroborations).
- **Promotion status**: standing-rule (Rule 30 in `apparatus/docs/predictive-ruleset.md`, amending Rule 27 via back-reference per Doc 727 §X). Entry 005 (TAECSF.1) Promotion status flipped in-place to standing-rule per §Discipline.
- **Composes with**: Rule 27 (Rule 30 amends; Rule 27 names the conflict, Rule 30 names the rectification shape), Rule 4 (never split a substrate move), Rule 21 (probe-first scoping), Doc 739 (P3) cascade-revival; this entry is the discovery hook for the rule's empirical record.
- **Authored actions**: commit (this commit) — Rule 30 appended to predictive-ruleset.md between Rule 29 and Standing instruments; §Status note refreshed; Entry 005 Promotion status flipped in-place; this entry appended.

### Entry 020 — DET.4 (test262-sample near-Class-A with ±1 bound under parallelism-timeout edge) (2026-05-30)

- **Finding ID**: DET.4.
- **Locale**: apparatus-tier / measurement-discipline.
- **Originating rung**: post-ASTA-EXT 0 follow-up cycle (audit-ledger Entry 010 +1 cycle); three test262-sample runs at the same substrate maturity (commit 00a73363).
- **Class**: measurement-discipline.
- **Body summary**: three independent runs of `scripts/test262-sample/run-sample.sh` against the same substrate state (post-ASTA-EXT 0; `bin/cruft` md5sum-identical across runs) produced: run 1 = 6817 PASS / 7696 emitted; run 2 = 6818 PASS / 7697 emitted; run 3 = 6817 PASS / 7696 emitted. **Variance bounded at ±1 PASS and ±1 emitted count**; median = 6817 (n=3); recorded value = 6817 ± 1 PASS at 88.8% runnable. The substrate is deterministic per direct probe (ASTA-EXT 0 probe 7/7 PASS reproducible across arbitrary invocations); the variance source is the harness-side parallelism-timeout edge. One test occasionally hits the per-test 10-second cap under parallel-process contention (PARALLEL=2 means two cruft processes contend for CPU on this hardware) and gets dropped from the emitted set on that run. The dropped test's prior result is the PASS-count variance.
- **Promotion status**: trajectory-and-findings-embedded; refines Rule 29 + Finding DET.2 with a "near-Class-A with bounded variance" sub-class. Specifically: the test262-sample instrument should be reclassed from "Class A (deterministic)" to **"near-Class-A (deterministic substrate; harness-bounded variance ±1)"** at the current substrate maturity. Rule 29's strict "byte-identity at n=2" falsifier is partially satisfied: substrate output IS byte-identical for emitted cells; the variance is in which cells the harness manages to emit under timeout pressure.
- **Composes with**: Rule 2 (multi-run protocol — at near-Class-A instruments, ≥3 runs with median is sufficient; ≥5 runs is conservative), Rule 3 (detectability budget — ±1 variance at 7681-runnable population is 0.013% noise, well within the strictest detectability threshold), Rule 29 (the falsifier event surfaces the near-Class-A class as a refinement, not a refutation; Rule 29's claim that "Class A instruments at this substrate maturity exhibit byte-identical results" needs the "modulo harness-side timeout-edge variance" caveat), Finding DET.2 (the engagement-tier enumeration of Class A instruments needs DET.4's harness-variance addendum at the test262-sample line).
- **Mitigation candidates** (out-of-scope; flagged for future apparatus-tier rung):
  - Lower PARALLEL (from 2 to 1) to eliminate process contention; trades wall-clock (~2× slower) for byte-identity.
  - Increase per-test timeout from 10s to 15s or 20s; reduces the edge for tests near the cap; minor wall-clock cost.
  - Identify the specific test(s) that hit the timeout edge; pin them via a fixed test-order or pre-execute them serially before the parallel sweep.
  - Hash-pinning per `measurement-determinism-prospective` doc §V.2.e — a hash mismatch flag at byte-equivalent counts would distinguish "harness dropped a test" from "engine produced different output."
