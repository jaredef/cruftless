# Measurement Determinism as an Apparatus Property

## A prospective articulation of the n=2 byte-identity short-circuit, its discipline implications, and proposed extensions of application across the apparatus and substrate

*A 2026-05-30 apparatus-tier prospective doc per keeper directive Telegram 10610, following the empirical observation that the canonical test262-sample measurement returned byte-identical results across two independent runs against the same substrate state (audit-ledger Entries 008 + 009). Builds on `apparatus/docs/predictive-ruleset.md` Rules 1-3 (per-workload + multi-run + detectability budget), Rule 9 (raw-pointer cache stability as an apparatus-tier address-stability condition), Rule 23 (founding baseline-inspection), and Rule 29 (Rule 2 amendment promoted in the same change as this doc).*

**Status**: prospective; companion to Rule 29 in the predictive-ruleset. The articulation is the prospective material; Rule 29 is the operational discipline edit. This doc is the prose home for the methodology + the exploratory extensions section.

---

## I. The occasion

The 2026-05-30 engagement session closed substantial substrate work (TABSC-EXT 0 byte-storage migration; TABSC-EXT 1 DataView coercion-faithfulness; TAECSF cascade-revival of integer-kind + Float32-NaN sub-substrates) and produced an engagement-rate movement of +4.4 percentage points on the canonical test262-sample (84.3% → 88.7%; +373 absolute PASS). At keeper direction (Telegram 10608), the canonical sample was re-run after findings were recorded.

The re-run produced **byte-identical results** to the first run: 6816 PASS / 865 FAIL / 16 SKIP / 7681 runnable / 88.7%. Zero variance at n=2.

The observation was first surfaced incidentally (the helmsman ran `bash pilots/typed-array-missing-method/exemplars/run-exemplars.sh` three times during the TAECSF-EXT 1 NEGATIVE diagnosis and reported 83/83/83 to verify the regression wasn't a flake). That triple-run was a sub-instance of the same pattern: cluster instruments at this engagement's maturity are reproducible across runs at zero variance when the substrate state is held constant.

The observation generalizes: every cluster-exemplar runner + the canonical sample produce byte-identical results across n=2 in the absence of substrate change. Rule 2's multi-run protocol (≥5 runs with median) is conservatively over-specified for these instruments. The discipline-cost saved by recognizing determinism at n=2 (~4× wall-clock per instrument; ~10 minutes saved per additional cluster-instrument run on this machine) is non-trivial across a session.

The keeper's directive composes three actions: (a) formalize the rule amendment (Rule 29 added to the predictive-ruleset, amending Rule 2 via back-reference per Doc 727 §X), (b) formalize the determinism property as an apparatus articulation (this doc), and (c) propose extensions of application across the apparatus and substrate (the doc's §V).

## II. The determinism property

A measurement instrument `I` is **deterministic at substrate maturity `M`** when, holding `M` constant, every invocation of `I(M)` produces the same recorded value. Determinism is a property of the instrument relative to a substrate state; the same instrument may be deterministic at maturity `M_k` and non-deterministic at maturity `M_{k+1}` (e.g., when substrate work introduces a race condition the instrument now exposes).

### II.1 The empirical signature

The n=2 byte-identity test: run `I(M)` twice; if both runs return byte-identical results, declare `I` deterministic at `M`. The test is cheap (one extra run; <½× the discipline cost of Rule 2's ≥5-runs protocol) and produces a falsifiable claim (if the third run produces different results, the n=2 test was a false positive; the apparatus updates the determinism judgment and re-enters Rule 2's full discipline).

### II.2 The classes of measurement instruments

This engagement's instruments fall into two classes at the current substrate maturity:

**Class A (deterministic)** — reproducible across invocations at zero variance:
- test262-sample (`scripts/test262-sample/run-sample.sh`) — empirically validated 2026-05-30.
- TAMM cluster exemplars (`pilots/typed-array-missing-method/exemplars/run-exemplars.sh`) — empirically validated across the TAECSF-EXT 1 diagnosis triple-run.
- TAWR cluster exemplars (`pilots/typed-array-wrong-result/exemplars/run-exemplars.sh`) — same.
- diff-prod 112-fixture suite (`scripts/diff-prod/run-all.sh`) — stdout byte-comparison; deterministic at the per-fixture level by construction.
- All per-fixture probe assertions executed via `cruft /tmp/probe-*.js` — single-process deterministic.

**Class B (non-deterministic)** — variance present at the instrument level:
- Cross-runtime bench (CRB) — timer-noise + scheduler interactions; per-iteration cost variance is the instrument's signal, not noise.
- Any future measurement involving wall-clock timings, system-call latencies, or scheduler interactions.
- Any future measurement involving randomized inputs (per Rule 10 canonical fuzz) where the seed is not pinned.

Rule 29 applies to Class A; Rule 2's ≥5-runs discipline remains canonical for Class B.

### II.3 Why Class A is large at this engagement

Most of the engagement's regression-gate instruments work by enumerating a fixed list of test262 cells, running each under a single `cruft` invocation, parsing the runner's JSON output for `PASS`/`FAIL`, and aggregating. The test262 corpus is fixed; the runner is single-threaded per cell; the engine's behavior is deterministic at the substrate maturity (no scheduler-dependent codepaths in the language semantics at this engagement's current state). The product of these properties is byte-identical results across n=2.

The discipline implication: most of the engagement's regression-gate measurements are Class A. The ≥5-runs cost was being paid uniformly across the discipline; Rule 29 reallocates that cost.

## III. The discipline implication: Rule 29

Rule 29 (promoted in the same change as this doc; cross-referenced with Rule 2 via the §Composes-with discipline back-reference per Doc 727 §X append-only):

> When a measurement instrument's first two invocations against an identical substrate state return byte-identical results (zero variance at n=2), the instrument is declared deterministic at the current substrate maturity; the recorded measurement is the value, not a noisy estimate. Rule 2's multi-run protocol collapses to single-value reporting. The full ≥5-runs discipline reactivates the moment any subsequent run surfaces variance > 0.

The rule preserves Rule 2's protective intent — multi-run protocol exists to detect flake in noisy instruments — while sharpening its application to the actually-noisy class. The non-deterministic case (Class B) sees Rule 2's full discipline; the deterministic case (Class A) sees the n=2 short-circuit.

### III.1 Composition with Rule 3 (detectability budget)

Rule 3 specifies per-N detectability thresholds (N=10 ≥10%; N=30 ≥7%; <7% requires N=100+). Rule 29 does not relax Rule 3; for deterministic instruments the variance estimate is zero by construction, and Rule 3's detectability budget is satisfied trivially for any non-zero claim. For non-deterministic instruments Rule 3 continues to govern claim-strength.

### III.2 Composition with Rule 23 (founding baseline-inspection)

Rule 23 specifies the at-spawn measurement + inspection discipline. Rule 29's n=2 short-circuit naturally extends Rule 23 to instruments: when a new measurement instrument is added to the engagement, its first two production runs should be n=2 stability checks before being declared canonical. If byte-identical, the instrument enters Class A and Rule 29 governs subsequent measurements. The Rule 23 baseline-inspection is for substrate at locale founding; Rule 29's n=2 is the analog at instrument founding.

### III.3 Composition with Rule 9 (raw-pointer cache stability)

Rule 9 names the substrate-tier address-stability condition (cache hot-paths must not assume raw-pointer stability across re-allocations). Rule 29's determinism condition is the apparatus-tier analog: measurement-result stability must not be assumed across non-deterministic substrate behaviors. The two rules together name the engagement's "stability invariant" at substrate and apparatus tiers respectively.

## IV. The formalized findings

This section formalizes the findings surfaced by the 2026-05-30 observation, in the structured form the findings-ledger §Discipline expects.

### Finding DET.1 — n=2 byte-identity is sufficient evidence of instrument determinism at the current substrate maturity

**Source**: test262-sample re-run 2026-05-30 (audit-ledger Entries 008 + 009); TAMM cluster triple-run during TAECSF-EXT 1 diagnosis.

**Class**: measurement-discipline.

**Statement**: a measurement instrument that produces byte-identical results across two consecutive invocations against the same substrate state has, at zero apparatus cost beyond the two runs, demonstrated determinism sufficient to declare the recorded value the measurement (not an estimate). The third+ run adds no information about variance; it adds information only if substrate state changes between runs (which is a separate audit event, not a measurement protocol).

**Predicts**: instruments declared deterministic via Rule 29 will continue to return byte-identical results across arbitrary subsequent runs at the same substrate maturity. The falsifier is a single subsequent run that surfaces variance — that event triggers Rule 2's ≥5-runs reactivation + a discovery investigation into what introduced the variance (typically a new race condition, a non-deterministic codepath added by recent substrate work, or a measurement-instrument bug).

**Evidence**: test262-sample byte-identical at n=2 post-TABSC-EXT 1 (commit 944a22dd → 6816/865/16/7681 both runs); TAMM cluster 83/83/83 across the TAECSF-EXT 1 diagnosis triple-run.

**Promotion status**: standing-rule (Rule 29). Promoted 2026-05-30 in the same change as this doc.

### Finding DET.2 — Most engagement regression-gate instruments are deterministic at the current substrate maturity

**Source**: enumeration of regression-gate instruments in §II.2 above; empirical reproducibility observed across the session's many gate-runs.

**Class**: apparatus-tier.

**Statement**: at this engagement's current substrate maturity, the cluster-exemplar runners (TAMM, TAWR, RBDPA, diff-prod) and the canonical test262-sample are all Class A (deterministic). CRB is Class B (non-deterministic by design — timer-noise is the signal). Future instruments involving wall-clock timings, system-call latencies, or randomized inputs without pinned seed will be Class B.

**Predicts**: as substrate work proceeds, an instrument may transition from Class A to Class B if the substrate acquires non-determinism the instrument exposes (e.g., a future async/coroutine substrate move that introduces scheduler-dependent codepaths in the language semantics). When a Class A instrument transitions to Class B, Rule 29's "variance event triggers Rule 2 reactivation" carries the discipline. Conversely, an instrument is unlikely to transition from Class B to Class A (timer-noise doesn't go away).

**Evidence**: enumerated in §II.2 above with cited validations.

**Promotion status**: trajectory-and-findings-embedded; one-more-observation pending cross-locale instance (specifically: a new measurement instrument added to the engagement that is declared Class A via Rule 29's n=2 test and remains stable across ≥10 subsequent invocations).

### Finding DET.3 — Apparatus-tier discipline-cost reallocation from Rule 29 short-circuit

**Source**: comparison of pre-Rule-29 vs post-Rule-29 measurement-cost arithmetic.

**Class**: apparatus-tier.

**Statement**: Rule 29 reallocates ~4× the measurement-protocol wall-clock from the ≥5-runs protocol to the n=2 short-circuit. For the test262-sample at ~10 min per run, the saving is ~30 min per canonical measurement; for the cluster-exemplar runners at ~30 sec per run, the saving is ~90 sec per cluster-gate. The aggregate session-tier saving (assuming ~10 cluster-gate measurements per session) is ~15 min of substrate-uninvolved wall-clock; for a multi-week engagement at one canonical sample per week, the saving is ~30 min × 4 weeks = ~2 hours of measurement-protocol overhead. The saved time is reallocable to additional substrate work, additional measurement (e.g., re-running CRB to detect a regression), or additional apparatus authoring.

**Predicts**: engagements operating under Rule 29 will run measurement instruments more frequently than engagements under Rule 2's ≥5-runs default, because the per-measurement cost is lower. More frequent measurement compounds the engagement's diagnostic resolution (Rule 22 axis-discrimination + Doc 721 chain-bundle walks both benefit from fresher measurements).

**Evidence**: arithmetic above; empirical session-tier saving will be observable across the next several sessions that adopt Rule 29.

**Promotion status**: trajectory-and-findings-embedded; one-more-observation pending a second session-tier instance.

## V. Exploratory: extensions of application

The determinism property formalized in §II + Rule 29's discipline are immediately applicable to the engagement's regression-gate instruments. The same property + discipline may extend to a wider class of apparatus and substrate sites. This section proposes those extensions.

### V.1 Apparatus-tier extensions

**V.1.a — Audit-ledger Entry stability re-verification.** When an audit-ledger entry records an "Authored actions" field citing measurement-driven flips (e.g., deferrals-ledger status flips contingent on a cluster-gate-PASS condition), the contingency is currently underspecified. Extension: amend the audit-ledger discipline to record the determinism class of the gate-instrument the flip was contingent on (Class A: byte-identical result is the contingency; Class B: median-of-≥5 with stated variance threshold). The findings-ledger Entry 016's SAMPLE.1 prediction (60-80 cells unlocked by a single substrate move) is a Class A measurement claim — explicitly so, since the test262-sample is Class A; a future predicted-vs-actual check against this claim should reference Rule 29's determinism property rather than ambiguously invoking Rule 2.

**V.1.b — Deferrals-ledger un-defer condition specificity.** Deferrals-ledger entries' "Un-defer condition" fields currently mix measurement-driven conditions (e.g., "second BigInt-namespace failure surfaces in test262") with discovery-driven conditions (e.g., "keeper directs the apparatus-pilot sweep"). Extension: classify each un-defer condition as Class A measurement, Class B measurement, or non-measurement (discovery / directive). The classification informs how often the un-defer is auto-checkable (Class A: cheap, can be a CI-style sweep; Class B: expensive, helmsman-initiated; non-measurement: keeper-initiated or event-triggered). Deferrals at Class A un-defer conditions could be auto-flipped by a periodic sweep without helmsman intervention; Class B requires the helmsman's variance-protocol; non-measurement requires the keeper's directive.

**V.1.c — Findings-ledger promotion-readiness as a Class A predicate.** Findings-ledger entries' Promotion status currently transitions on "second observation" criteria that are typically Class A (a cell flips, a probe passes, a sample number moves). Extension: explicitly type the promotion criterion as Class A measurement and adopt Rule 29's n=2 reproducibility check before promoting. The cycle-2 promotions (TAECSF.3 + APP.PIPELINE-1 → Rules 27 + 28) implicitly relied on Class A reproducibility but did not state it; future promotions should reference Rule 29 as the discipline that licenses the n=1 second-observation event as sufficient evidence.

**V.1.d — Audit re-runnability protocol.** An audit's reproducibility is currently implicit; only the most carefully-authored audit entries explicitly cite the inputs an independent auditor would need to reproduce. Extension: every audit-ledger entry of type `cross-locale-finding` or `findings-disposition` includes a "Re-runnability class" field (Class A / Class B / non-measurement) + the inputs required to reproduce. Class A audits become auto-reproducible by a future helmsman (or by an arbiter); Class B audits require the multi-run protocol; non-measurement audits depend on context that may have changed.

**V.1.e — Predictive-ruleset rule-evidence section determinism class tagging.** Each rule's `**Evidence**` section currently cites empirical anchors without explicitly noting whether the anchors are Class A reproducible or Class B variance-bounded. Extension: tag each evidence citation with `[A]` / `[B]` / `[N]` (non-measurement). For Class A evidence, a future reader can independently re-verify the rule by running the cited measurement; for Class B evidence, the re-verification requires the variance protocol; for non-measurement evidence (e.g., a corpus doc citation, a code-review observation), re-verification requires reading the cited material.

### V.2 Substrate-tier extensions

**V.2.a — Spec-conformance tests as Class A measurements at production-tier.** Bun, V8, JavaScriptCore, and SpiderMonkey all run the test262 suite at production-tier as part of CI. Each engine's measurement is Class A by construction (the test cells are pinned). Extension: cruft's CI (if and when it lands) can adopt Rule 29's n=2 protocol natively rather than running ≥5 times per CI job. The CI cost reduction at the build-engineering tier is the engagement-tier analog of the helmsman's measurement-protocol cost reduction.

**V.2.b — Deterministic substrate replay.** Class A measurement determinism + the engagement's existing test262 cell enumeration together permit a "substrate replay" capability: given a substrate change `S` and a previous canonical measurement `M_prev`, a future helmsman can predict the post-`S` measurement `M_post` for any cell unchanged in scope by running only `S`-touched cells, not the full sample. The unchanged cells will be byte-identical PASS/FAIL by Rule 29. Extension: a `scripts/test262-sample/replay.sh` that takes a substrate-diff and runs only the test262 cells in the substrate-diff's touch set + an N-cell stratified random sample for variance-check; reports the delta against the previous canonical at much lower wall-clock cost.

**V.2.c — Determinism as a substrate invariant.** The engagement's substrate work to date has been substantially deterministic by construction (no async / coroutine substrate races; no scheduler-dependent codepaths in the implemented language semantics). Extension: explicitly name "determinism preservation" as a substrate-discipline invariant — a substrate move that would introduce non-determinism (e.g., a future Promise scheduling pivot, an async/await race-fix, a Worker-tier introduction) must explicitly enumerate the determinism-class transition of every affected instrument as part of its proposal's Risk Assessment. This makes the Class A → Class B transition a first-class apparatus-visible event rather than an incidental side-effect.

**V.2.d — Substrate-change diff-prod auto-regression.** Diff-prod's 112-fixture suite is Class A. Extension: every commit landed via the helmsman discipline auto-runs diff-prod and records the byte-identity check (or the variance event) against the prior commit's recorded result. Per Rule 29, a single byte-identical result is sufficient evidence of no-regression at the substrate maturity of the prior commit. The discipline can be cheap (one diff-prod run; ~5 minutes); the auto-record provides a per-commit regression-gate without the manual helmsman gate-check.

**V.2.e — Hash-pinning the Class A measurement.** Each Class A measurement can be hashed (e.g., SHA-256 of the canonical `results.jsonl` after sorting) to produce a stable per-measurement identifier. Extension: every canonical Class A measurement records the hash in the audit-ledger entry; a future re-run that produces the same hash confirms byte-identity at the binary level (catching e.g., a partial result file truncation that produces the same per-line counts but different content). The hash is the apparatus-tier analog of git's commit-hash discipline at the measurement tier.

**V.2.f — Falsifier audit.** The session's measurement-cost saving from Rule 29 is empirical; Rule 29 itself predicts that Class A instruments stay deterministic at this substrate maturity. The natural falsifier audit: run each Class A instrument additionally N=5 times at a substrate-pinned state, record byte-identity across N runs, publish the per-instrument byte-identity rate. If Class A instruments are 100% byte-identical at N=5, Rule 29's n=2 short-circuit is corroborated. If <100%, the n=2 test was a false positive at some N>2 surface — investigate which substrate property surfaces the variance and amend Rule 29 with the boundary condition.

### V.3 Composition with cascade-revival pattern

The session's substantive substrate yield (Doc 739 + 740 + 741) was driven by the cascade-revival pattern. Rule 29's determinism property composes with the cascade-revival framing: the cascade is observable only post-landing per Doc 739 (B3); the post-landing measurement is the apparatus event that confirms the cascade. If the post-landing measurement is Class A (deterministic), one re-run at n=2 is sufficient evidence of the cascade's structural realization; if Class B, the variance-protocol governs. Rule 29 thus sharpens Doc 739's (B3) into a measurement protocol: the cascade is empirically validated when the n=2 measurement at the post-landing substrate state demonstrates byte-identical results at the predicted cluster movement.

The 2026-05-30 cascade-revival of TAECSF sub-substrates via TABSC-EXT 0 (Finding TABSC.1) implicitly relied on Class A reproducibility: the +4 TAWR + +1 TAMM cluster shift was a Class A measurement; its re-run at audit-ledger Entry 009 was byte-identical. Future cascade-revival empirical validations should explicitly reference Rule 29 + Finding DET.1 as the discipline that licenses the n=2 evidence.

## VI. Falsifier surface

The articulation's falsifier is a Class A instrument (one declared deterministic via n=2 byte-identity) that surfaces variance > 0 on a subsequent run at the same substrate maturity. If observed, Rule 29's n=2 short-circuit is partially falsified for that instrument; the instrument re-enters Rule 2's ≥5-runs discipline + an investigation into what produced the variance.

A weaker falsifier: a Class A declaration where the n=2 test was performed but the substrate maturity drifted between the two runs (e.g., a substrate-change landed in the interval). The drift would invalidate the n=2 test; corrective protocol: re-run the n=2 test against the post-drift substrate. The falsifier is real but procedural rather than structural.

A stronger corroborator: a multi-week engagement running Rule 29 with Class A instruments across many sessions, with no variance event surfacing. The aggregate session-count without variance corroborates the rule at high confidence.

## VII. Status

This articulation operates at the apparatus-prospective tier. Rule 29 is the operational discipline; this doc is the prose home. The findings-ledger §V.1 + V.2 extensions are proposals, not yet promoted. Promotion of any §V proposal follows the discipline: ≥2 cross-locale instances + arbiter or keeper sign-off; the audit-ledger entry that authorizes the promotion carries the empirical record.

Per `apparatus/docs/findings-disposition-protocol.md` §III validation, this doc's own articulation is subject to the same protocol: at second authoring (a future revision), it must pass through the protocol's Step 3 candidate test. The articulation's first live use is the test262-sample re-run that motivated it; subsequent live uses (proposed in §V) will refine or falsify the §II-IV claims.

## VIII. Closing

The 2026-05-30 session's incidental observation of byte-identical test262-sample results across two consecutive runs surfaced a previously-implicit apparatus property: measurement-instrument determinism at the current substrate maturity. Naming the property + formalizing Rule 29 + proposing extensions transforms the property from an incidental measurement-cost optimization into a first-class apparatus invariant the engagement can operate against. The exploratory extensions in §V are the predictive surface; each extension is a candidate substrate or apparatus move whose value compounds with Rule 29's determinism discipline.

The articulation closes the 2026-05-30 session's apparatus-tier work. The substrate-tier work continues at the SAMPLE.1 substrate-spawn signal (the cross-family missing-TypeError-throw pattern; findings-ledger Entry 016) and at the other Class A measurement signals this articulation now licenses the engagement to read.
