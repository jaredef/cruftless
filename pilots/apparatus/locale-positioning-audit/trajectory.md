# locale-positioning-audit — Trajectory

## LPA-EXT 0 — founding (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9802), opened in response to the apparatus §XI.1.b amendment necessitated by PPIF-EXT 2's deletion. That amendment was the first instance where a sibling locale's close dissolved a prior locale's claimed-irreducible carrier without the apparatus auto-noticing; this locale exists to systematize the audit of such drift.

**Apparatus established**:

- `seed.md` — telos, three-rung methodology, trigger conditions, carve-outs, composes-with, R13 prospective check.
- No findings docs yet; the `findings/` subdirectory will house per-rung output (`stale-claims.md`, `spinoff-chains.md`, `positioning-gaps.md`).

**Initial inventory** (snapshot at founding):
- Locale count: 109 (per `apparatus/locales/manifest.json`).
- Deletions ledger entries: 2 (PPIF-EXT 2; LGSS-EXT 2).
- Visible spinoff chains: 1 confirmed (LGSS → PPIF → FHNB).
- Apparatus-doc amendments triggered by sibling-locale drift this session: 1 (apparatus §XI.1.b, this commit + the prior).

**R13 prospective C1-C4 all hold (per seed §Methodology)**:

- C1: Doc 727 basin-stability + Doc 415 retraction-ledger are corpus-tier siblings.
- C2: manifest is JSON, locales are markdown — both grep-able and walkable.
- C3: TBV at LPA-EXT 1; bounded by locale count.
- C4: append-only; never edits prior trajectories.

**Status**: LPA-EXT 0 FOUNDED. LPA-EXT 1 (stale-claim survey) is the first substantive rung; runs on-trigger rather than on a schedule, so the first run waits for the next deletion-ledger entry or keeper directive.

**Findings**

**Finding LPA.0 (the apparatus's drift class is bounded by its own discipline)**: cruftless's discipline is heavy on append-only artifacts (findings.md, trajectories, deletions ledger, this locale's findings/). The drift class the audit catches is specifically the kind that append-only doesn't catch: claims in prior artifacts that become STALE because sibling work elsewhere dissolved their basis. The audit's role is to surface staleness, not to correct it (correction is the original locale's amendment-by-new-trajectory-entry move; the audit just makes the case for amendment legible). Standing recommendation: any apparatus discipline whose primary mechanism is append-only is structurally vulnerable to claim-staleness; a co-running audit locale is the dyadic-ascent counterpart that restores coherence.

---

## LPA-EXT 1 — Phase 2 path-staleness sweep after bilateral-pilot-tier housekeeping (2026-05-25)

**Trigger**: Keeper directive (Telegram 9806) "Move to phase 2." Phase 2 refers to the bilateral pilot tier landing (commit 84798b0a) which deferred cross-citation sweeps to lazy resolution. LPA is the natural home for this sweep (a stale-claim survey of one specific class: path-staleness across the 6-locale bilateral move).

**Scope**:

- **In scope**: all `.md` files under `pilots/` (except `rusty-js-jit/findings.md` — protected as the canonical findings ledger under Doc 727 §X basin-stability), `apparatus/docs/`, `apparatus/locales/`, `docs/engagement/`, `docs/fca-instances/`.
- **Out of scope**: `docs/corpus-ref/` (read-only mirror of the published corpus per the apparatus tier-separation §0). `pilots/rusty-js-jit/findings.md` (canonical findings ledger).

**Carve-out preserved**: `docs/corpus-ref/737-the-locale-as-coordinate-...md` retains its pre-move path reference. The corpus doc's text is a historical record from publication time; updating it would constitute editing-the-corpus-from-substrate, a tier-separation violation. Path-staleness in corpus-ref is the keeper's to resolve at next corpus-publish if material.

**Execution**: mechanical `sed -i` over 20 files with six 1:1 pattern replacements:

```
pilots/test262-categorize/       → pilots/apparatus/test262-categorize/
pilots/diff-prod/                → pilots/apparatus/diff-prod/
pilots/cross-runtime-bench/      → pilots/apparatus/cross-runtime-bench/
pilots/ts-consumer-corpus/       → pilots/apparatus/ts-consumer-corpus/
pilots/ts-execute-corpus/        → pilots/apparatus/ts-execute-corpus/
pilots/locale-positioning-audit/ → pilots/apparatus/locale-positioning-audit/
```

**Verification**: post-sweep grep for stale refs (excluding the two carve-outs) returns zero matches.

**Yield**:

- **20 files updated** (63 insertions / 63 deletions; pure 1:1 path rewrites, no semantic content edited)
- **0 files in the protected carve-outs** touched
- **64 stale references** resolved
- **1 reference** intentionally left stale in corpus-ref/737 (documented)

**Findings**

**Finding LPA.1 (path-staleness is the most-mechanical staleness class)**: of the staleness classes the audit can surface (stale irreducibility-claims, stale orphan-claims, stale spinoff-pending claims, path-staleness, coordinate-drift), path-staleness is fully mechanical to detect (grep for old paths) and fully mechanical to resolve (sed). It is the easiest first instance of the audit's value proposition. The harder classes (stale irreducibility-claims) require semantic comparison across locales' Findings and remain LPA-EXT 2+ work. This first execution closes the easy case as a working-discipline demonstration; subsequent rungs require richer reasoning.

**Finding LPA.2 (the 2-tier carve-out is principled, not lazy)**: the sweep preserved two carve-outs that are NOT laziness but apparatus-discipline: `docs/corpus-ref/` is the published-corpus mirror (editing it crosses the apparatus/docs tier-separation that §0 of repository-apparatus.md makes load-bearing); `rusty-js-jit/findings.md` is the canonical append-only ledger (editing prior entries violates Doc 727 §X basin-stability). The audit RECORDS the carve-outs rather than working around them; future readers chasing pre-move paths in those files now have this trajectory as the navigation breadcrumb. Standing recommendation: every audit sweep should produce a per-file disposition (updated / carve-out / protected); silent skipping is incompatible with the audit's claim-coherence telos.

**Status**: LPA-EXT 1 CLOSED. The bilateral-pilot-tier housekeeping is now fully landed (Phase 1 structural move + Phase 2 reference sweep). LPA-EXT 2 (spinoff-chain mapping) and LPA-EXT 3 (positioning-gap detection) remain on the methodology and run on next trigger.

---

## LPA-EXT 2 — spinoff-chain mapping (2026-05-25)

**Trigger**: Keeper directive (Telegram 9808) "continue with ext 2."

**Produced**: `pilots/apparatus/locale-positioning-audit/findings/spinoff-chains.md` — the first chain-map snapshot.

**Survey method**:

1. Grep `pilots/` for explicit markers: "spinoff", "spawned from", "surfaced from", "surfaced by", "opened in response", "nested under", "nested locale".
2. For each hit, walk the cited parent locale's trajectory tail to confirm the spawn-causation arrow.
3. Group findings into chain types (3-tier substrate cascade, multi-sibling spawn, parent→nested rung, apparatus-pilot cascade, matrix fan-out, self-reflexive).

**Snapshot at 2026-05-25**: **7 confirmed chains + 1 self-reflexive locale (LPA)**.

| # | Chain | Type | Tier-span |
|---|---|---|---|
| 1 | LGSS → PPIF → FHNB | 3-tier substrate cascade | lexer → climber → bytecode/runtime |
| 2 | TSR → 11 ts-resolve-* siblings | multi-sibling spawn | TSR-tier (single-tier broad fan-out) |
| 3 | IHI → GPI → IPBR | 3-tier substrate cascade (LeJIT) | interp → bytecode-rewrite (Doc 740/741 instance) |
| 4 | Shape → CMig (nested) | parent→nested rung | shapes-tier sub-workstream |
| 5 | TCC → TXC | apparatus-pilot cascade | parse-parity → execute-parity instruments |
| 6 | PEER → BBND (nested) → corpus-candidate Doc 743 | parent→nested rung→corpus | parser-tier; nested BBND surfaced apparatus-tier corpus draft |
| 7 | full-suite matrix → top-10 batch | matrix fan-out | one matrix read → 10 coordinate-shaped sibling locales |
| 8 | LPA → (self-reflexive) | meta-apparatus | no children yet |

**Findings**

**Finding LPA.3 (chain types are not all alike — each predicts different yield shape)**: the audit surfaced six distinct chain shapes across the 8 instances. 3-tier substrate cascades (Chains 1 + 3) produce the strongest amortization-conjecture corroboration — each tier's named constraint enables the next. Multi-sibling spawns (Chain 2) produce the strongest yield-per-spawn-event but no sequential depth. Parent→nested rungs (Chains 4, 6) are R4-disciplined (the parent's scope was correct, the sub-shape just needed its own seed). The audit's value scales with the engagement's chain count; today's 7 confirmed chains suggest the engagement has crossed from "spawn-per-need" into "spawn-via-chain-causation" as the dominant locale-spawn mode. Standing recommendation: tag new locales at spawn time with the chain-type they belong to (or are likely to anchor); this gives future audits a coordinate to read against, rather than re-deriving chain causation from grep.

**Finding LPA.4 (the 7 confirmed chains corroborate the keeper's amortization conjecture engagement-wide, not just at LGSS→PPIF→FHNB)**: the keeper's amortization conjecture (Telegram 9794) was framed in response to LGSS's specific case. The audit shows the pattern was operating ENGAGEMENT-WIDE before LGSS — TSR's 11-sibling cascade, IHI/GPI/IPBR's 3-tier cascade, Shape's parent→nested. The conjecture's empirical track is older + broader than the LGSS→PPIF→FHNB chain implied; LGSS made the pattern legible at the engagement-discipline tier (via Findings + the cluster-coherence-multiplier prospective doc), but the substrate-engineering tier has been operating this way for the entire session arc. Standing recommendation: the prospective Doc 743 (cluster-coherence multiplier) should cite the 7-chain engagement-wide track, not just LGSS, as its empirical anchor. Update pending keeper review.

**Status**: LPA-EXT 2 CLOSED. The spinoff-chains.md is the first standing output of this rung; refreshed opportunistically per the triggers (new spawn, locale close, full-suite re-categorize). LPA-EXT 3 (positioning-gap detection) remains.

---

## LPA-EXT 3 — positioning-gap detection (2026-05-25)

**Trigger**: Keeper directive (Telegram 9810) "continue with 3."

**Produced**: `pilots/apparatus/locale-positioning-audit/findings/positioning-gaps.md` — the first positioning-gap snapshot against the current full-suite matrix.

**Method**:

1. Walk every `pilots/**/seed.md`; extract any 4-tuple `pin` string the seed cites (regex on backtick-wrapped strings matching the `X :: Y :: Z :: W` shape).
2. Build a pin→locale map; for each top-30 matrix coordinate, look up whether a locale claims it.
3. Partition gaps by heuristics §III + §IV class (apparatus-refinement / subsystem-absent-chapter / sibling-of-covered / temporal-downstream / regexp / net-new substrate).
4. Detect drift: locales whose declared coordinate may have shifted since spawn.

**Snapshot 2026-05-25** (against `test262-full-2026-05-25-165734-p2` matrix):

| Rank range | Count of covered | Count of gaps | Aggregate fails in gaps |
|---|---:|---:|---:|
| 1–10 | 10 | 0 | 0 |
| 11–30 | 1 | 19 | 5,403 |
| **Total top-30** | **11** | **19** | **5,403** |

**Class breakdown of gaps** (heuristics §III + §IV):

| Class | Ranks | Count | Move shape |
|---|---|---:|---|
| A: apparatus-refinement (uncategorized/projection or uncategorized/resolver) | 8, 11, 12, 16, 17, 18, 24 | 2,802 | spawn `pinart-categorizer-refinement` apparatus-pilot FIRST; substrate locales for these unblock after |
| B: subsystem-absent-chapter (intl402 cluster) | 2, 14, 25 | 2,613 | spawn `intl402-availability/` mirroring temporal-availability pattern |
| C: regexp coordinate cluster | 19, 23 | 491 | spawn single `regexp-conformance/` locale (both sibling rungs) |
| D: siblings of covered ranks | 15, 21, 28, 29 | 947 | extend existing top-10 batch locales' scope at next chapter close (no new locales) |
| E: temporal-downstream (auto-resolves with TA-EXT 1+) | 20, 27 | 438 | no new spawns; will absorb when temporal-availability lands MVP |
| F: net-new substrate coords not yet locale'd | 26, 30 | 363 | small; consider scope-extension of existing locales rather than spawn |

**Cumulative addressable via recommendations**: ~6,853 fails (~29% of total FAIL records) through 3 new spawns + 4 scope-extensions.

**Drift cases surfaced**:

- `ast-bytecode-uncategorized-projection/` — declared coordinate is apparatus-gap; correctly positioned but successor (categorizer-refinement) not yet spawned; substrate work blocked until then.
- `ts-resolve-*/` family — coordinates may be superseded by TXC execute-parity residual; per-locale verification deferred to LPA-EXT 1-style sweep.
- `parser-early-error-residual/` — pool size in seed.md stale (809 cited; post-BBND ~714); coordinate still correct, just the residual count changed.

**Locales without matrix coordinates** (substrate-only work): identified as a separate partition — substrate-architecture work-class (rusty-js-jit, rusty-js-caps, rusty-js-shapes, the FCA-amortization spinoff chain LGSS/PPIF/FHNB) vs matrix-coverage work-class (top-10 batch + temporal-availability + this audit's recommended spawns). Both legitimate; the audit notes the partition.

**Findings**

**Finding LPA.5 (the locale-coverage gap top-30 is dominated by apparatus-refinement, not substrate work)**: 2,802 of the 5,403 top-30 gap fails (52%) are in Class A (apparatus-refinement). A single `pinart-categorizer-refinement` apparatus-pilot would convert these from blurred-coordinate work-shape into specific-coordinate work-shapes that downstream substrate locales could address. The locale-spawn priority order is therefore: **apparatus-tier refinement BEFORE substrate-tier spawns** for the apparatus-gap class; substrate-tier spawns AFTER coordinates are sharpened. This inverts the naive "spawn locales by raw count" reading and matches the heuristics §III partition-before-rank discipline at the engagement-graph scope.

**Finding LPA.6 (recommended spawns + scope-extensions are well-shaped per the cluster-coherence multiplier)**: of the 3 recommended new spawns (`pinart-categorizer-refinement`, `intl402-availability`, `regexp-conformance`), the latter two satisfy multiple conditions of the cluster-coherence multiplier (per `docs/engagement/prospective/cluster-coherence-multiplier-as-sipe-t-instance.md`): subsystem-availability gates (Conditions 1+3+4 of the five-condition multiplier) producing high yield-per-locale. The 4 scope-extensions (Class D) are R4-disciplined extensions of existing locales' surface, not new spawns; the audit explicitly recommends extension over new-spawn for siblings to avoid apparatus-tax non-amortization (per BBND's findings §IV recommendation).

**Status**: LPA-EXT 3 CLOSED. The three audit methodology rungs (stale-claim sweep / spinoff-chain mapping / positioning-gap detection) all have first-execution outputs. The locale itself is now **operationally complete** — future runs re-render the same documents per the triggers without needing additional methodology rungs.

---

## LPA-EXT 4 — resolution-layer composition snapshot (2026-05-26)

**Trigger**: Keeper question after ECMA-262 gap survey: whether the current gaps/locales are recorded in a doc or manifest, with the proposed need to stratify current locales across resolution layers and capture a rough state-of-composition snapshot.

**Produced**: `pilots/apparatus/locale-positioning-audit/findings/resolution-layer-snapshot.md`.

**Read of existing apparatus**:

- `apparatus/locales/manifest.json` is the authoritative generated inventory, but it is not the right place for hand-authored layer commentary.
- `positioning-gaps.md` maps matrix coordinates to locales, but its latest rendered snapshot predates several current spawns and does not stratify the whole locale graph by resolver layer.
- `spinoff-chains.md` maps composition edges, but not layer occupancy or current matrix pressure by layer.

**Move**:

- Added a new LPA finding document that treats the manifest as input and records the current layer-stratified composition view.
- Stratified layers include apparatus, lexer/parser, AST-to-bytecode/language lowering, runtime ECMA-262 built-ins, typed-array/ArrayBuffer, RegExp, Atomics/agent-memory, object internals, array exotic, Promise/jobs, ECMA-402, Node/host APIs, and JIT/shapes/performance substrate.
- Recorded the latest available full-suite matrix baseline and noted that it predates recent `intl402-availability` work.
- Marked immediate apparatus gaps: nullable free-text manifest state, implicit layer taxonomy, stale LPA positioning gaps, over-broad AST-to-bytecode pressure, and under-localed Atomics/Promise-jobs pressure.

**Finding LPA.7 (manifest is inventory, not composition-state)**: the generated manifest answers "what locales exist?" and stores free-text status where available. It does not answer "which resolution layer owns this coordinate?", "what is the current composition edge?", or "what is the next layer-level action?" Those are audit findings, not manifest facts. The right apparatus shape is therefore a derived snapshot under LPA, with a future generated `state`/`primary_layer` extension if the snapshot becomes recurring.

**Finding LPA.8 (AST-to-bytecode is now the largest unstratified layer)**: after apparatus refinement and many tokenization/runtime spawns, the largest ECMA-262 pressure is not a missing subsystem but the 10,839-row language-lowering resolver bucket. Treating it as one candidate would violate the partition-before-rank discipline. The next useful apparatus move is a layer-internal partition table by syntactic family and projection class.

**Status**: LPA-EXT 4 CLOSED. Snapshot established. Future LPA re-render should either refresh this document or promote its tabular core into a generated layer manifest.

---

## LPA-EXT 5 — language-lowering partition (2026-05-26)

**Trigger**: Continuation of the resolution-layer snapshot arc. LPA-EXT 4 identified `ast-to-bytecode/language-lowering` as the largest unresolved ECMA-262 layer and explicitly called for partitioning before any new broad spawn.

**Produced**: `pilots/apparatus/locale-positioning-audit/findings/language-lowering-partition.md`.

**Method**:

- Parsed the latest available full-suite `interpreted.jsonl`.
- Filtered records where `resolver == "ast-to-bytecode/language-lowering"`.
- Aggregated by surface, projection, and surface+projection.
- Read existing locale names and CANDIDATES entries to map the aggregate rows against current locale coverage.

**Result**:

- Bucket size: **10,839** records.
- Dominant surfaces:
  - `language.statements.class` 2,420
  - `language.expressions.class` 2,257
  - `annexB.language` 734
  - `language.statements.for-await-of` 646
  - `language.expressions.async-generator` 568
  - `language.expressions.object` 487
  - `language.expressions.dynamic-import` 296
- Dominant projections:
  - `availability/missing-method-or-intrinsic` 4,338
  - `value-semantics/wrong-result` 1,622
  - `parser-form/early-error` 1,324
  - `abrupt-completion/throw-missing` 1,165
  - `availability/missing-syntax-feature` 919

**Partitioned arcs**:

1. Class elements and class lowering.
2. Async iteration and async generators.
3. Annex B language semantics.
4. Object literal, computed property, and super.
5. Dynamic import and module-like lowering.
6. Direct eval, function declarations, and arguments object.
7. Assignment, compound assignment, and for-head targets.
8. With / try / switch / completion records.
9. Literal and identifier residual routing.

**Finding LPA.9 (class pressure is largest, but not necessarily the next spawn)**: class statement+expression rows account for **4,677** records, but existing class/private-name/private-field locales have recently moved this surface. The right next action is a class residual re-partition against the current focused probes, not a generic class locale.

**Finding LPA.10 (async generator / for-await is the cleanest fresh language-lowering candidate)**: the combined async iteration + async generator surface is **1,492** records, and unlike the class cluster it is not already covered by a focused active conformance locale. A candidate such as `async-generator-and-for-await-lowering` is likely the cleanest fresh substrate arc, provided baseline inspection separates async harness behavior, async generator object protocol, AsyncFromSync wrapping, and abrupt completion propagation.

**Finding LPA.11 (Annex B language must stay separate from Annex B runtime)**: `annexB-runtime-quirks/` is scoped to Date/String/RegExp/global runtime built-ins and explicitly avoids Annex B grammar/lowering. The 734-row `annexB.language` surface is therefore a distinct candidate family (`annexB-language-semantics`), not an extension of the runtime quirks locale.

**CANDIDATES.md update**:

Added Tier M, "language-lowering partition outputs":

- `async-generator-and-for-await-lowering` — ripe for baseline.
- `annexB-language-semantics` — ripe for baseline.
- `class-lowering-residual-repartition` — audit-first.
- `object-literal-computed-property-semantics` — sample-needed.
- `eval-function-arguments-binding-semantics` — overlap-check-needed.
- `dynamic-import-residual-audit` — apparatus-first.

**Finding LPA.12 (candidate queue now distinguishes spawn-ready from audit-first arcs)**: the language-lowering bucket contains both large counts and stale/blurred sub-surfaces. Recording all six arcs with status distinctions prevents the next worker from reading raw count as spawn priority. In particular, class rows are largest but audit-first, while async-generator/for-await is smaller but cleaner for a fresh baseline.

**Status**: LPA-EXT 5 CLOSED. The language-lowering layer now has a first apparatus partition and candidate queue entries; no substrate locale spawned in this round.

---

## LPA-EXT 6 — repartition audit algorithm (2026-05-26)

**Trigger**: Keeper directive after choosing an AST-to-bytecode / language-lowering candidate partition: the apparatus needs a tier-level audit algorithm for repartition, not just an ad hoc partition doc.

**Produced**: `pilots/apparatus/locale-positioning-audit/findings/repartition-audit-algorithm.md`.

**Seed update**:

- Promoted LPA methodology from three rungs to four.
- Added Rung 4: repartition audit algorithm.
- Added trigger: when a top matrix bucket is broad or mixed, run repartition audit before spawning new substrate locales from that bucket.

**Algorithm summary**:

1. Select a reproducible bucket from `interpreted.jsonl`.
2. Aggregate surface, projection, and surface×projection marginals.
3. Classify top cells by mechanism class.
4. Read existing locale coverage for absorption, exclusion, staleness, and duplication.
5. Emit arcs, not isolated rows.
6. Assign disposition: `spawn-ready`, `baseline-first`, `scope-extension`, `audit-first`, or `redirect/defer`.
7. Update `CANDIDATES.md` only for stable arcs.
8. Record the move in LPA trajectory.

**Finding LPA.13 (repartition is a reusable apparatus algorithm, not a one-off analysis)**: LPA-EXT 5's language-lowering partition exposed a recurring method. The method is independent of the specific bucket: any broad matrix coordinate can be treated as a selector over `interpreted.jsonl`, then reduced through the same surface/projection/existing-locale/ disposition pipeline. This moves "partition-before-rank" from judgment into an apparatus-tier algorithm.

**Finding LPA.14 (candidate disposition is the load-bearing output)**: the important result of repartition is not just counts; it is the disposition attached to each arc. Counts alone would select class lowering first. Disposition-aware repartition selected async-generator/for-await as cleaner for baseline, class as audit-first, and dynamic import as apparatus-first. The algorithm therefore prevents raw-count priority inversions.

**Status**: LPA-EXT 6 CLOSED. Repartition audit is now part of LPA methodology.

---

## LPA-EXT 7 — resolver-axis gap partition (2026-05-27)

**Trigger**: Keeper supplied the recent heuristics-commit read after the cited commit object (`600afe6`) was not locally reachable. The read partitioned a 49-gap snapshot by resolver instance and axis, with explicit (`█`) versus implicit (`░`) Pin-Art markers.

**Produced**: `pilots/apparatus/locale-positioning-audit/findings/resolver-axis-gap-partition.md`.

**Provenance note**:

- Source commit cited by keeper: `600afe6`.
- Local lookup after `git fetch --all --prune`: object not reachable from `origin/main`, local refs, or reflog.
- This rung records the keeper-provided partition as apparatus input. A future rerun should replace the provenance with exact commit/run ids once available.

**Partition**:

1. Instance 2 x Axis H: 24 bootstrap host-install gaps, totality-of-consumption violations at the bootstrap resolver.
2. Instance 4 x Axis R: 13 AST-to-bytecode boundary-integrity gaps, boundary leaks and unconsumed directives.
3. Instance 4 x Axis O: 7 operator-directive gaps, narrower single-rung substrate fixes.

**CANDIDATES.md update**:

Added Tier N, "resolver-axis heuristics partition":

- `bootstrap-host-install-totality` — baseline-first.
- `ast-bytecode-boundary-integrity-audit` — audit-first.
- `operator-directive-completion` — baseline-first.

**Finding LPA.15 (resolver-axis cells expose mechanism coherence beyond raw surface rows)**: the 49-gap snapshot concentrates into two hot cells and one narrow operator queue. Instance 2 x Axis H is bootstrap install totality; Instance 4 x Axis R is boundary integrity; Instance 4 x Axis O is operator directive completion. The resolver-axis view is therefore a useful repartition lens: it distinguishes install-sequence gaps, boundary-spec gaps, and single-rung operator gaps that raw surface labels would blur.

**Finding LPA.16 (implicit probe-collision constraints are first-class apparatus output)**: 25 of the 49 gaps were implicit constraints discovered by fixture collision. These rows are not merely failures; they are newly surfaced decision-basis edges. Future repartition artifacts should preserve the explicit/implicit marker instead of collapsing both into the same count.

**Status**: LPA-EXT 7 CLOSED. Resolver-axis partition recorded; no substrate locale spawned.

---

## LPA-EXT 8 — spec boundary integrity audit (2026-05-27)

**Trigger**: Keeper directive to continue with the spec boundary audit after LPA-EXT 7 queued `ast-bytecode-boundary-integrity-audit` as `audit-first`.

**Produced**: `pilots/apparatus/locale-positioning-audit/findings/spec-boundary-integrity-audit.md`.

**Method**:

1. Read the Instance 4 x Axis R examples from the resolver-axis partition.
2. Search existing locale seeds and trajectory tails for ownership of private fields, tagged templates, eval, destructuring, generator suspension, and RegExp named captures.
3. Classify each family as absorbed, scope-extension, or fresh baseline-first candidate.

**Result**:

- Private fields reflection leak: absorbed by `private-field-runtime-slots/` and `class-elements-static-semantics/`.
- Generator suspension deferred: scope-extension under `async-generator-and-for-await-lowering/` or a future generator-specific child.
- RegExp named captures: scope-extension under `regexp-conformance/`.
- Tagged-template `strings.raw`: fresh baseline-first candidate.
- Direct eval outer lexical capture: fresh baseline-first candidate.
- Destructuring iterator protocol: fresh baseline-first candidate, with async rows redirecting to AGFA if dominant.

**CANDIDATES.md update**:

- Marked `ast-bytecode-boundary-integrity-audit` as audited/split.
- Added `tagged-template-object-boundary`, `direct-eval-lexical-capture`, and `destructuring-iterator-protocol-boundary` as baseline-first children.

**Finding LPA.17 (boundary-integrity is a parent signal, not a substrate locale by itself)**: Instance 4 x Axis R correctly identified a design-level boundary-integrity smell, but its named examples map to distinct resolver boundaries. The right apparatus move is ownership partition, not a broad substrate locale.

**Finding LPA.18 (more than half of the boundary examples are already owned)**: private fields are absorbed by PFRS/CESS, generator suspension is an AGFA scope-extension, and RegExp named captures belong under RC. The `audit-first` disposition prevented duplicate locale founding.

**Finding LPA.19 (three fresh boundary candidates remain after absorption)**: tagged-template object construction, direct eval lexical capture, and destructuring iterator protocol each have a plausible single boundary mechanism and a concrete baseline probe. They should proceed independently, not as one AST-boundary locale.

**Status**: LPA-EXT 8 CLOSED. Spec boundary audit recorded; no substrate locale spawned.

---

## LPA-EXT 9 — tagged-template object boundary baseline (2026-05-27)

**Trigger**: Keeper directive to proceed with fresh baseline after LPA-EXT 8 split `tagged-template-object-boundary` as a baseline-first child candidate.

**Baseline target**:

- Candidate: `tagged-template-object-boundary`.
- Path set: all 27 fixtures under `language/expressions/tagged-template/`.
- Results artifact: `/Users/jaredfoy/Developer/cruftless-sidecar/results/spec-boundary-baseline-20260526-224210/`.

**Build**:

- `cargo build --release --bin cruft -p cruftless` passed with existing warnings.

**Result**:

- PASS: 12.
- FAIL: 13.
- ABORT/no-json: 2 (`tco-call.js`, `tco-member.js`).

**Failure clusters**:

1. Template cache identity by source site/function/top-level: 4.
2. Direct eval / realm context around tag binding: 4.
3. Template-object allocation / map / constructor shape: 3.
4. Frozen template object / strict write behavior: 2.
5. Illegal escape cooked value should be `undefined`: 1.
6. TCO tagged-template call/member aborts: 2.

**CANDIDATES.md update**:

- Promoted `tagged-template-object-boundary` from baseline-first to spawn-ready.
- Recorded baseline counts and the TCO carve-out.

**Finding LPA.20 (tagged-template baseline confirms a coherent boundary locale)**: the 27-row focused baseline produced 13 ordinary failures all centered on TemplateStringsArray construction, caching, freezing, raw/cooked shape, or eval/realm context. The two no-JSON rows are TCO tagged-template tests and can be carved out initially. This is no longer merely `strings.raw`; it is a coherent template-object boundary locale.

**Status**: LPA-EXT 9 CLOSED. Candidate is spawn-ready; no substrate locale spawned in this audit rung.

---

## LPA-EXT 10 — diff-prod empirical cross-check of language-lowering partition (2026-05-27)

**Trigger**: 70 new diff-prod fixtures (18 gap-partition probes + 8 deep-engine probes + 24 spec-lowering probes + 20 resolution-layer probes) were authored and run against both `cruft` and `bun`, producing byte-for-byte stdout comparisons across the 9 arcs identified in LPA-EXT 5's language-lowering partition and all 13 resolution layers from LPA-EXT 4. The run triangulates the matrix-derived partition against direct empirical engine behavior.

**Full suite**: 112 fixtures (42 original + 70 new). Run result: **58 PASS / 54 FAIL**.

**Produced**: Updated `pilots/apparatus/locale-positioning-audit/findings/language-lowering-partition.md` §IV with empirical arc-level pass rates, per-fixture divergence details, and 9 cross-cutting mechanism gaps. Added 90 per-locale `analysis.md` files alongside existing seed.md + trajectory.md pairs.

**Cross-cutting mechanism gaps identified**:

1. ToPrimitive hint dispatch (affects Arcs A, D, abstract ops)
2. IteratorClose protocol (affects Arcs B, G)
3. Lazy generator suspension (affects Arc B)
4. Direct eval lexical capture (affects Arc F)
5. Finally on abrupt loop exit (affects Arc H)
6. OrdinaryOwnPropertyKeys ordering (cross-cutting MOP)
7. Proxy trap invariant enforcement (cross-cutting MOP)
8. Arguments object shape (affects Arc F)
9. strings.raw on tagged templates (affects Arcs D, I)

**Finding LPA.21 (diff-prod empirical pass rates invert the matrix-count priority for 2 arcs)**: Arcs F (eval/function/arguments, 582 rows, 0% pass rate) and G (assignment/for-head, 551 rows, 0% pass rate) are more acutely broken than the matrix-count leaders A and B.

**Finding LPA.22 (9 cross-cutting mechanism gaps compress 54 fixture failures into a smaller fix surface)**: the 54 failing fixtures compress into 9 mechanism gaps spanning multiple arcs.

**Finding LPA.23 (the lexer's core token emission pipeline is sound)**: 7 lexer-exercising fixtures all PASS. 6 lexer gaps identified (L.1 through L.6), all with existing locale coverage.

**Finding LPA.24 (the highest-leverage lexer gap is actually in the compiler)**: L.2 (strings.raw) — the lexer produces raw data correctly, but the compiler doesn't thread it to the runtime.

**Status**: LPA-EXT 10 CLOSED.

---

## LPA-EXT 11 — Compiler-tier locale spawning (2026-05-27)

**Trigger**: LPA-EXT 10's mechanism gap analysis identified 5 compiler-tier gaps without adequate locale coverage.

**Spawned**:

1. `generator-coroutine-suspension/` (GCS) — mechanism gap #3, 1,492 test262 rows
2. `finally-abrupt-completion-lowering/` (FACL) — mechanism gap #5
3. `tagged-template-object-construction/` (TTOC) — lexer gap L.2
4. `eval-function-arguments-binding-semantics/` (EFABS) — mechanism gaps #4 + #8, promoted from CANDIDATES.md (abe)
5. `iterator-close-emission-sites/` (ICES) — mechanism gap #2

**Finding LPA.25 (compiler tier has the widest gap between available data and locale coverage)**: the lexer (7 locales, 6 gaps all covered) and parser (12+ locales) have dense locale coverage. The runtime (20+ locales) has broad coverage. The compiler had 0 dedicated mechanism-gap locales despite being the layer where 5 of the 9 mechanism gaps originate.

**Status**: LPA-EXT 11 CLOSED. 5 compiler-tier locales founded; manifest refreshed to 135 locales.
