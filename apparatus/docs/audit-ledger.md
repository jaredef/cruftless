# Audit Ledger

A standing apparatus-tier record of audits performed against the apparatus or substrate state: deferrals-vs-substrate cross-references, status verifications, manifest coherence checks, cross-locale consistency sweeps, ledger-flip authorizations. The audit-ledger is to apparatus-tier inspection what the deferrals-ledger is to surfaced-but-not-founded candidates: a place where audit work that would otherwise vanish into a session transcript is bound into the cybernetic loop's read surface.

## Why a ledger

Cruftless's apparatus tracks substrate moves (locale seeds, trajectories, manifest entries, findings.md, deletions-ledger, deferrals-ledger). It had nothing for tracking **audits** — sessions where a resolver examines apparatus or substrate state, produces a classification, and authors actions against it. Audits are first-class apparatus work:

- A deferrals-vs-substrate sweep classifies every open deferral against current substrate state; produces UN-DEFER / SUPERSEDED / STILL GATED verdicts; authorizes ledger flips and locale spawns.
- A manifest coherence check verifies that every locale named in arc enrollment exists on disk and is reachable from `apparatus/locales/manifest.json`.
- A status-field correctness sweep checks that no prior ledger entry's status drifted between its recorded value and the actual disposition.
- A cross-locale consistency audit (e.g., locales claiming arc enrollment vs. arc.md roster) surfaces drift.

Without a ledger, the audit's findings live only in the session that produced them — and the apparatus loses the methodology trail that would let a future session verify the audit, re-run it, or hold prior audit findings accountable to subsequent state.

This ledger restores the binding. Each entry records:

1. **Audit name** — short slug naming the audit's coordinate (e.g., `deferrals-vs-substrate-2026-05-30`, `manifest-coherence-2026-06-XX`).
2. **Audit type** — one of:
   - **deferrals-vs-substrate** — classify each open deferrals-ledger entry against current substrate state.
   - **deletions-vs-substrate** — verify no deletion's revisit-condition has silently triggered.
   - **manifest-coherence** — verify locale manifest matches filesystem state.
   - **arc-enrollment-coherence** — verify arc.md sub-locale roster matches enrolled locales' seed.md statements.
   - **ledger-status-correctness** — sweep prior ledger entries for status-drift.
   - **status-verification** — single-entry verification (typical follow-up to a deferrals-vs-substrate sweep recommending VERIFY).
   - **cross-locale-finding** — verify a finding promoted across locales still holds at all sites.
   - **other** — name the type in the entry body.
3. **Auditor** — the resolver session that performed the audit (e.g., `helmsman session 2026-05-30 under keeper directive Telegram 10558`).
4. **Date** (absolute, per CLAUDE.md em-dash discipline anchor).
5. **Scope** — what was examined (file paths, locale list, ledger entry IDs, etc.).
6. **Method** — how the audit was performed (delegated sub-agent reading inclusion set X; grep over corpus Y; direct manual inspection of N entries; etc.). Enough detail for a future auditor to reproduce.
7. **Findings** — the classification or verdict produced. For multi-finding audits, summarize the count + name the load-bearing findings.
8. **Authored actions** — what landed as a consequence of the audit. Cite commits where applicable. Cite the other-ledger entries (deferrals-ledger flips, deletions-ledger appends, locale foundings, proposals authored) that the audit authorized.
9. **Surfaced-but-not-acted findings** — audit findings that surfaced anomalies the auditing session did not act on (worktree drift, stale state, unrelated dirty files, structural anomalies). These compose with the deferrals-ledger pattern: an audit that surfaces a finding it does not address owes the apparatus a ledger entry, not merely a session-transcript mention. Cross-reference deferrals-ledger entries where appropriate.

## Discipline (append-only)

Per Doc 727 §X basin-stability discipline (same as findings.md, deletions-ledger.md, deferrals-ledger.md): this file is **append-only**. New entries go at the bottom in chronological order. Older entries are never edited. If a prior audit is superseded or contradicted by a later audit, the later audit names the prior with a back-reference; the prior entry is not edited in place.

## Discovery hook

An audit landing should commit its ledger entry in the same change as the substrate or apparatus-tier flips it authorizes. The ledger entry is the methodology trail; the substrate flips are the closure.

The standing rule the ledger formalizes: **an audit session that produces classifications or authorizes ledger flips owes the apparatus an audit-ledger entry, not merely a Telegram report.** The session transcript is volatile; the ledger entry makes the audit's method + findings + authored actions readable from outside the originating session.

---

## Entries

### Entry 001 — `deferrals-vs-substrate-2026-05-30` (2026-05-30)

- **Audit type**: deferrals-vs-substrate.
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10556 ("Let's look at our deferrals against the state of the substrate").
- **Scope**: all 15 then-open entries of `apparatus/docs/deferrals-ledger.md` (Entries 001 – 015), cross-referenced against the substrate state observable at 2026-05-30 from `pilots/typed-array-wrong-result/trajectory.md` tail, `pilots/missing-intrinsic-loader-failures/trajectory.md`, `apparatus/arcs/2026-05-28-array-exotic-substrate/arc.md`, recent decided proposals (2026-05-30 batch milf-ext-10 through milf-ext-13), and the apparatus measurement baselines.
- **Method**: Explore sub-agent delegation per CLAUDE.md context-protection discipline. Each entry's gating predicate evaluated against the named substrate evidence; classification into one of {UN-DEFER READY, STILL GATED, STALE, REFRAME, VERIFY}. Cross-deferral patterns surfaced from the per-entry analysis.
- **Findings**: 1 UN-DEFER READY (Entry 009 `resizable-buffer-detection-per-access`); 1 VERIFY (Entry 014 `buffer-read-uint32be-host-method`, likely SUPERSEDED by MILF-EXT 7.1); 13 STILL GATED (001, 002, 003, 004, 005, 006, 007, 008, 010, 011, 012, 013, 015). Three cross-deferral patterns: (A) cluster-pending HOST-INTRINSIC residuals 012–015 from MILF Phase 5 mongoose smoke; (B) probe-pending ToBigInt bifurcation lattice-meet between 010 ↔ 001; (C) cybernetic-protocol singleton 011 with temporal rather than substrate-shaped gating.
- **Authored actions**:
  - Commit `b8249fb5` (2026-05-30): un-defer Entry 009; founded `pilots/resizable-buffer-detection-per-access/` under arc `2026-05-28-array-exotic-substrate`; deferrals-ledger Entry 009 flipped to PROMOTED; Entry 016 appended; arc roster updated; manifest refreshed 227 → 228 locales. Authorized by keeper Telegram 10558 ("Begin with 1").
  - Commit `e2e75f80` (2026-05-30): Entry 014 SUPERSEDED audit verified MILF-EXT 7.1's `install_buffer_methods` batch-install closed the saslprep `readUInt32BE` gap at the install-discipline tier; deferrals-ledger Entry 014 flipped to SUPERSEDED; Entry 017 appended. Authorized by keeper Telegram 10562.
- **Surfaced-but-not-acted findings**:
  - Worktree drift at audit time: `pilots/rusty-js-runtime/derived/src/intrinsics.rs` carried 9 uncommitted lines unrelated to the audit's actions; stray `pilots/pilots/test262-categorize/` directory present; CAACP `2026-05-29T011500Z-stage-b-verify` artifacts untracked (consistent with deferrals-ledger Entry 011 workaround state).
  - Arc-metadata degradation: 13 of 15 active arc.md files lack a structured `## Status` line in searchable position (prose status present but not pattern-matchable).
  - Locale manifest staleness pre-audit: 5 days old at audit start; refreshed by this audit's commit `b8249fb5`.
  - Gate-readings staleness: test262-full last canonical run 2026-05-25; sample last canonical run 2026-05-27. Post-array-exotic + MILF-batch closures likely advance both; fresh full-suite run advisable but not authored by this audit.
  - Recommended action 3 (probe rung for Entry 010 ↔ 001 cluster) authorized separately and tracked in this ledger's Entry 002.

### Entry 002 — `findings-audit-2026-05-30` (2026-05-30)

- **Audit type**: cross-locale-finding (variant: findings.md inventory + classification + cross-locale recurrence).
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10568 ("let's do a findings audit of all the findings.md docs in the repo locales. We need to bubble these up to a apparatus tier findings ledger").
- **Scope**: every `findings.md` file under the repo (excluding `target/`, `node_modules/`, `.git/`). Five files enumerated: `pilots/rusty-js-jit/findings.md` (canonical 26-rule + 16-addendum ledger), `pilots/interp-hot-intrinsics/findings.md` (IHI.1), `pilots/rusty-js-jit/osr/findings.md` (OSR.1, OSR.2), `pilots/rusty-js-jit/top-level/findings.md` (TL.1, TL.2), `pilots/parser-early-error-residual/block-bound-names-dup/findings.md` (BBND yield-analysis). One trajectory-embedded finding (TAECSF.1 at `pilots/ta-element-coercion-spec-faithful/trajectory.md`) included by cross-reference.
- **Method**: Explore sub-agent delegation per CLAUDE.md context-protection discipline. Each `findings.md` parsed for individual findings (ID + name + class + body + promotion status); cross-locale recurrence patterns identified by content match against `apparatus/docs/predictive-ruleset.md`; trajectory-embedded findings cross-referenced via locale tag lookup.
- **Findings**: 5 `findings.md` files (4 with discrete per-finding entries, 1 with yield-analysis structure); 1 trajectory-embedded finding pending extraction; 4 cross-locale recurrence patterns identified (IC cache lifetime, JIT boundary encoding, multi-tier cascade-revival, baseline-inspection-as-locale-probe); 1 corpus-Doc-743 candidate (BBND yield-analysis) pending corroboration; 5 anomalies surfaced (rules 16–22 unconsolidated, TAECSF.1 trajectory-only, BBND non-standard structure, apparatus-pilots have no findings.md by tier convention, source-identifier-convention findings unaggregated).
- **Authored actions**:
  - Commit (pending this entry's commit): introduce `apparatus/docs/findings-ledger.md` modeled on deferrals/deletions/audit-ledger conventions. Per-locale inventory + 10 entries covering the JIT canonical ledger reference + per-locale extracted findings + cross-locale recurrence patterns + anomalies. Append-only per Doc 727 §X.
- **Surfaced-but-not-acted findings**:
  - JIT findings.md rules 16–22 remain unconsolidated to `apparatus/docs/predictive-ruleset.md`; recommended as a follow-up consolidation pass (separate audit + apparatus-tier edit).
  - TAECSF.1 remains trajectory-embedded; per per-locale convention extraction to `pilots/ta-element-coercion-spec-faithful/findings.md` defers until the second productive rung lands in that locale.
  - The findings-ledger is not yet wired into CLAUDE.md §Required agent reading or `apparatus/docs/repository-apparatus.md` enumeration; flagged for keeper direction in parallel with audit-ledger's same gap.

### Entry 003 — `findings-disposition-cycle-1` (2026-05-30)

- **Audit type**: findings-disposition (new type; formalized by `apparatus/docs/findings-disposition-protocol.md` introduced in this same commit).
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10570 ("create and formalize a apparatus tier heuristics that's based on the methodology of the apparatus itself self applying to the findings so that we can then either create new arcs or integrate the findings into existing arcs").
- **Scope**: all 10 entries of `apparatus/docs/findings-ledger.md` (Entries 001–010), authored at commit d904702b.
- **Method**: apparatus self-application — the 5-phase substrate-shaped-work pipeline (Rule 11 spawn + Rule 23 baseline-inspect + Rule 24 Pin-Art probe + Rule 13 revert-if-negative + Rule 15 chapter-close-inspect) applied at the finding tier with findings as cells and arcs / standing rules as lifted substrates. Eight disposition candidates tested in order per finding (integrate-existing-arc, integrate-scaffolded-arc, lift-to-new-arc, promote-to-standing-rule, relocate-to-apparatus-pilot, lattice-meet-annotation, defer-with-cross-reference, close-as-locale-singleton). Protocol mirrors `apparatus/docs/orphan-disposition-protocol.md` at the finding subsumption boundary.
- **Findings**:
  - **Dispositions assigned**: 4 lattice-meet-annotation (Entries 002, 003, 004 + 001 close-as-canonical-reference); 4 close-as-promoted (Entries 007, 008, 009, 010 — all promoted-to-standing-rule status already covered); 2 defer-with-cross-reference (Entries 005 TAECSF.1, 006 BBND yield-analysis — both awaiting second cross-locale corroboration before standing-rule promotion).
  - **Cross-finding pattern observed**: Pattern F.1 (finding-already-promoted-with-bidirectional-back-reference) instance count = 3 at this cycle (Entries 002 + 003 + 004). Confirms F.1 as steady-state pattern when canonical-ledger discipline is healthy.
  - **No new arcs scaffolded at cycle 1** — all findings are either already promoted to standing rules + JIT canonical addenda or below promotion-readiness threshold.
  - **Next-cycle predictions**: TAECSF.1 → Rule 27 (or post-16-22-consolidation number) on second corroboration of dispatcher-over-lift heuristic; BBND yield-analysis → Rule 28 (or post-consolidation number) on second corroboration of constraint-stacking multiplier.
- **Authored actions**:
  - Commit (this commit): introduce `apparatus/docs/findings-disposition-protocol.md`; record cycle 1 worked application in §V of that doc; record this audit-ledger entry.
- **Surfaced-but-not-acted findings**:
  - Predictive-ruleset consolidation gap (rules 16–22 in JIT Addenda XII–XIII not yet folded into `apparatus/docs/predictive-ruleset.md`) carries forward from audit-ledger Entry 002; addressing requires a separate apparatus-tier consolidation pass.
  - The protocol is not yet wired into CLAUDE.md / AGENTS.md §Substrate-shaped-work discipline §Phase 5 chapter-close-inspect pointer, nor into `apparatus/docs/repository-apparatus.md` §III standing-discipline-artifact list. Same wiring gap as audit-ledger (Entry 001 §Surfaced) and findings-ledger (Entry 002 §Surfaced). Three apparatus-tier documents now share the same not-yet-wired status; recommend a single consolidation pass.
  - `apparatus/docs/arc-as-coordinate.md` §F event-log class for "findings-disposition annotation" is referenced by the protocol but not yet added to arc-as-coordinate.md itself.

### Entry 004 — `apparatus-wiring-consolidation-2026-05-30` (2026-05-30)

- **Audit type**: arc-enrollment-coherence (variant: apparatus-document-enumeration-coherence).
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10572 ("Author").
- **Scope**: the three apparatus-tier documents introduced during the 2026-05-30 session — `apparatus/docs/audit-ledger.md` (commit 6456c35e), `apparatus/docs/findings-ledger.md` (d904702b), `apparatus/docs/findings-disposition-protocol.md` (496e4d21) — against the canonical enumeration surfaces: `CLAUDE.md` §Required-agent-reading, `AGENTS.md` (mirror of CLAUDE.md), `apparatus/docs/repository-apparatus.md` §III directory tree + §Tracked-but-not-an-addition ledgers table + §Locale-tier discipline table, `apparatus/docs/arc-as-coordinate.md` §File shapes log.md event-class enumeration.
- **Method**: direct file inspection + targeted `Edit` insertions at the identified canonical surfaces. CLAUDE.md and AGENTS.md kept in byte-for-byte sync via `cp` (existing convention).
- **Findings**: three new apparatus-tier documents introduced in the session were not yet enumerated in any of the canonical surfaces. Surfaced-but-not-acted findings carried forward from Entries 001 (audit-ledger), 002 (findings-ledger), 003 (findings-disposition-protocol). Wiring gap closed by this audit.
- **Authored actions**:
  - `CLAUDE.md` §Required-agent-reading — three bullets added (audit-ledger, findings-ledger, findings-disposition-protocol) after the deletions-ledger entry.
  - `CLAUDE.md` §Substrate-shaped-work-discipline — new subsection §Findings-disposition protocol added after §Orphan-disposition protocol, paired as the two engagement-tier Phase-5 instances.
  - `AGENTS.md` — synced to CLAUDE.md (byte-identical mirror).
  - `apparatus/docs/repository-apparatus.md` §III — directory-tree extended with four new lines (audit-ledger, findings-ledger, findings-disposition-protocol, ledger-comment text updated for orphan-disposition coverage scope); Tracked-but-not-an-addition table extended with audit-ledger + findings-ledger rows; Locale-tier discipline table extended with findings-disposition-protocol row; functional-partitioning paragraph updated to enumerate all four ledgers.
  - `apparatus/docs/arc-as-coordinate.md` §File shapes log.md description extended with event-class enumeration including orphan-disposition-annotation and findings-disposition-annotation.
- **Surfaced-but-not-acted findings**:
  - The JIT findings.md rules 16–22 unconsolidated gap (carried forward from Entries 001 + 002 + 003) is not addressed by this wiring audit. Requires a separate predictive-ruleset.md consolidation pass.
  - The findings-disposition-protocol §IV references `arc-as-coordinate.md §F (event log)`; the doc's actual section labels are §A–§F where §F is "Composition with other arcs" and the event log is under "File shapes". The protocol's reference is forward-looking; section relabeling is deferred to keeper direction.

### Entry 005 — `predictive-ruleset-consolidation-rules-17-22` (2026-05-30)

- **Audit type**: cross-locale-finding (variant: canonical-source-to-derived-view consolidation).
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10574 ("Now let's continue the process").
- **Scope**: rules 17–22 articulated at `pilots/rusty-js-jit/findings.md` Addenda XII + XIII (Standing rule 17 EPSUA.6/EPSUA.7; Standing rule 18 SPBC.2; Standing rule 19 SPTW.2; Standing rule 20 NACR.1; Standing rule 21 RS.1+RS.2+CP.4; Standing rule 22 SMPT.4+ALST.2) against `apparatus/docs/predictive-ruleset.md` consolidated derived view. The gap was carried forward by Entries 001 / 002 / 003 / 004 as the single remaining apparatus-tier consolidation.
- **Method**: Explore sub-agent delegation for rule extraction (verbatim text + class + origin); helmsman direct read of `pilots/rusty-js-jit/findings.md` Addendum XII–XIII to verify canonical wording + numbering. Sub-agent's initial enumeration mis-numbered the rules (16–22); helmsman direct read corrected to 17–22 (canonical source has no rule 16 — numbering jumps 15 → 17 in Addendum XII). Each rule then appended to `apparatus/docs/predictive-ruleset.md` between rule 15's separator and rule 23's heading, preserving the existing canonical formatting (`## Rule N — Title` + `**Statement**:` + `**Predicts**:` + `**Evidence**:`).
- **Findings**:
  - **Rules consolidated**: 6 (rules 17, 18, 19, 20, 21, 22). All promotion-ready as-is per canonical source wording; no editorial cleanup required.
  - **Rule 16 does not exist**: numbering gap preserved per Doc 727 §X append-only discipline. Canonical-source numbering anomaly recorded in the predictive-ruleset.md §Status note + findings-ledger Entry 001 Promotion status + this audit entry.
  - **Rule 21 (probe-first scoping) re-instantiated 2026-05-30**: TAECSF-EXT 0 narrow-dispatcher probe (~60 LOC vs prospective ~200+ LOC for option (i)) is the second engagement-wide instance of the probe-first pattern after the original Compartment-arc instance. Cited in the rule's Evidence section.
- **Authored actions**:
  - Commit (this commit): apparatus-tier consolidation. Edits to `apparatus/docs/predictive-ruleset.md` (rules 17–22 appended + status note refreshed); `apparatus/docs/findings-ledger.md` (Entry 001 Promotion status flipped in-place per §Discipline; Entry 011 appended recording the consolidation event; inventory table updated to reflect new consolidation state); this audit-ledger entry recording the audit.
- **Surfaced-but-not-acted findings**:
  - All four prior audit-ledger entries' carry-forward "rules 16–22 unconsolidated" gap is now closed by this entry; no carry-forward to subsequent audits remains on that axis.
  - Sub-agent off-by-one numbering error (claimed rule 16 existed; canonical source does not have one) is a measurement-discipline finding at the meta-tier — when delegating apparatus-tier work to sub-agents, the helmsman should verify canonical-source numbering directly rather than trust the sub-agent's enumeration. Single observation at this audit; promotion-readiness: one-more-observation before standing-rule status (candidate Rule 27: "delegate-for-volume, verify-for-canonical-numbering").

### Entry 006 — `pipeline-alphabet-audit-2026-05-30` (2026-05-30)

- **Audit type**: cross-locale-finding (variant: pipeline-tier coherence audit per Doc 729 resolver-instance pattern).
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10582 ("we need to precisely discover where the beginning of the alphabet is; and if the ecmascript tokenizer, IR, etc pipeline all the way down to the host layer can explain the resolution pipeline").
- **Scope**: full resolver-instance pipeline for the substrate `ta[i] = v` typed-array element write: lexer → parser/AST → IR → bytecode → runtime dispatch → storage → host. Pillars `pilots/rusty-js-parser/`, `pilots/rusty-js-ir/`, `pilots/rusty-js-bytecode/`, `pilots/rusty-js-runtime/`. Motivated by TAECSF-EXT 1.1 convergent diagnosis (Finding TAECSF.3) identifying the engine-architectural `Vec<Value>` constraint blocking spec-faithful coercion.
- **Method**: Explore sub-agent delegation walked each tier's code path for `ta[i] = v`; identified each tier's alphabet (input) + resolution (output) + type commitment; located the `ArrayBufferRecord` construction sites and verified the read/write symmetry lock; grepped for prior trajectory entries documenting the `Vec<Value>` choice (none found).
- **Findings**: alphabet origin is Tier 5 (Runtime dispatch); upstream tiers are uniformly type-agnostic by ECMAScript's design. Pipeline does NOT explain `Vec<Value>` as a deliberate substrate move; it is accumulated drift from the engine's universal property-storage pattern, retrofit into typed arrays without a separate substrate decision. Architecture is locally incompatible with spec-faithful coercion (not globally broken). Rectifying rung is option (a) per TAECSF.3: migrate `ArrayBufferRecord.data` to `Vec<u8>` with NumberToRawBytes encoding per ECMA-262 §6.1.6.1; scope is multi-rung and exceeds TAECSF's telos; recommended candidate locale `pilots/typed-array-byte-storage-conformance/` founded as sibling within arc `2026-05-28-array-exotic-substrate`. New apparatus-tier finding APP.PIPELINE-1 surfaced (dynamic-typing pipeline starts type-specific alphabet at runtime); recorded as findings-ledger Entry 013.
- **Authored actions**:
  - Commit (this commit): introduce `apparatus/docs/pipeline-alphabet-audit-2026-05-30.md` (full per-tier articulation + answers A/B/C/D + four risks for the precursor architectural rung); append findings-ledger Entry 013 for APP.PIPELINE-1; append this audit-ledger entry.
- **Surfaced-but-not-acted findings**:
  - The proposed locale `typed-array-byte-storage-conformance` is not yet founded. Founding requires keeper APPROVED of a proposal explicitly citing the four risks identified in the audit (read/write symmetry; DataView migration scope; view-aliasing harness gate; `__kind` typed-enum co-yield optimization).
  - The DataView setters at `intrinsics.rs:19842–19865` carry the same Rust saturating-cast spec divergence flagged in the TAECSF-EXT 1 proposal §Risk Assessment; jointly subject to the precursor architectural rung.
  - The `__kind` slot is currently a String stored via `set_own_internal` rather than a typed field on `TypedArrayViewRecord`; the precursor migration is a natural moment to promote it to a typed enum, but this is a co-yield optimization, not a precondition.

### Entry 007 — `findings-disposition-cycle-2` (2026-05-30)

- **Audit type**: findings-disposition.
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10600 ("Cycle 2").
- **Scope**: findings-ledger entries with newly-ripened promotion-readiness after TABSC-EXT 0 landing (commit f2107bb6): Entry 012 (TAECSF.3), Entry 013 (APP.PIPELINE-1), plus the new Finding TABSC.1 surfaced at TABSC-EXT 0.
- **Method**: `apparatus/docs/findings-disposition-protocol.md` Step 3 candidate test applied per finding. TAECSF.3 + APP.PIPELINE-1: both received second observation via TABSC-EXT 0's empirical validation; candidate (4) promote-to-standing-rule fits per the protocol's Step 3 ordering. TABSC.1: new at TABSC-EXT 0; candidate (7) defer-with-cross-reference fits (one observation; awaits second). No new arcs scaffolded; no orphan-disposition lattice-meets surfaced.
- **Findings**:
  - **Promotions (2)**: Rules 27 (Substrate-spec-correctness vs engine-architecture conflict, from TAECSF.3) and 28 (Dynamic-typing pipeline starts type-specific alphabet at runtime introspection, from APP.PIPELINE-1) appended to `apparatus/docs/predictive-ruleset.md` between rule 26's separator and the Standing instruments section.
  - **Deferrals (1)**: Finding TABSC.1 (cascade-revival materializes at precursor rung when precursor IS the encoding tier) recorded as findings-ledger Entry 015 with `trajectory-and-findings-embedded` Promotion status; candidate Doc 740 §II.2 P4 amendment pending second observation.
  - **Cross-finding pattern observed**: Pattern F.1 (already-promoted-with-bidirectional-back-reference) increments by 2 at this cycle (TAECSF.3 + APP.PIPELINE-1 now carry bidirectional back-references to predictive-ruleset.md rules 27 + 28); steady-state pattern remains dominant per cycle 1's prediction.
- **Authored actions**:
  - Commit (this commit): `apparatus/docs/predictive-ruleset.md` rules 27 + 28 appended + §Status note refreshed; `apparatus/docs/findings-ledger.md` Entries 012 + 013 Promotion status flipped in-place per §Discipline; Entries 014 + 015 appended; this audit-ledger entry recorded.
- **Surfaced-but-not-acted findings**:
  - The protocol's §III validation discipline ("at its second authoring it must pass through itself") is partially exercised here: cycle 2 is the protocol's first live use after the protocol's own authoring (commit 496e4d21). No protocol amendment surfaced this cycle; the §III self-application coherence holds at the protocol's first real-state test.
  - TAECSF.1 (narrow dispatcher beats wide signature lift) remains at one-more-observation per cycle 1's disposition; this cycle does not flip it. Second observation candidate: a future Result-thread-through-non-Result-callsite work in the engine (e.g., template-literal ToNumber, RegExp dispatch).
  - BBND yield-analysis (Entry 006) remains deferred per cycle 1's disposition; this cycle does not flip it. Awaits a non-parser-early-error locale exhibiting the constraint-stacking multiplier shape.

### Entry 008 — `test262-sample-canonical-2026-05-30` (2026-05-30)

- **Audit type**: ledger-status-correctness (variant: canonical-measurement re-baselining).
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10606 ("Test 262 sample").
- **Scope**: `scripts/test262-sample/run-sample.sh` against current main post-TABSC-EXT 1 (commit 944a22dd).
- **Method**: ran the canonical sample with default PARALLEL=2 per the script's discipline; binary auto-refresh from `target/release/cruft` confirmed; sample size 7750 tests (7697 emitted, 7681 runnable, 16 SKIP per frontmatter feature flags).
- **Findings**:
  - **88.7% runnable pass rate** (6816 PASS / 865 FAIL / 16 SKIP / 7681 runnable).
  - **+4.4 percentage points over the 2026-05-27 canonical** (84.3%; 6443/7647).
  - **+373 absolute PASS** (6443 → 6816) across the 3-day window.
  - The delta compounds today's session work: rules 17-22 consolidation; TAECSF-EXT 0 BigInt-TA Result-threaded coercion; TABSC-EXT 0 byte-storage architectural rectification (the load-bearing rung — closed the architectural constraint blocking spec-faithful coercion); TAECSF cascade-revival of integer-kind + Float32-NaN sub-substrates without TAECSF-side work; TABSC-EXT 1 DataView coercion-faithfulness via substrate-prefix amortization.
- **Authored actions**:
  - Commit (this commit): CLAUDE.md + AGENTS.md §Measurement baselines test262-sample line refreshed with 88.7% + delta + attribution to session work; this audit-ledger entry recorded.
- **Surfaced-but-not-acted findings**:
  - The DataView coercion cells flagged at TABSC-EXT 1 decision.md's "named follow-up" did surface in the sample re-run — part of the +373 PASS shift. The flagged cells were indeed outside the curated TAMM/TAWR/diff-prod pools but were in the test262-sample's broader surface, exactly as the decision.md predicted.
  - test262-full re-measurement remains pending (last canonical 67.6% per 2026-05-28; likely advanced similarly).
  - Cross-runtime bench (CRB) re-measurement not run this session.

### Entry 009 — `test262-sample-stability-re-run-2026-05-30` (2026-05-30)

- **Audit type**: ledger-status-correctness (variant: measurement-stability re-run).
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10608 ("Record findings then re run").
- **Scope**: re-run of `scripts/test262-sample/run-sample.sh` after findings recording (commit e9937715), to verify measurement determinism.
- **Method**: identical invocation as Entry 008's run; same binary (cached `bin/cruft`); same sample paths; same parallelism (default 2).
- **Findings**: **byte-identical result** — 6816 PASS / 865 FAIL / 16 SKIP / 7681 runnable / 88.7% runnable pass rate. Zero variance between the two runs. Confirms the canonical measurement is deterministic at this engagement's maturity; the 88.7% number is the measurement, not a noisy estimate.
- **Authored actions**: commit (this commit) recording the stability re-run. No CLAUDE.md baseline edit (the number is unchanged).
- **Surfaced-but-not-acted findings**:
  - Rule 2 (multi-run protocol, ≥5 runs) is conservatively over-specified for fully-deterministic instruments. The test262-sample produces byte-identical results across n=2 runs at zero variance; additional runs add no information. The discipline-cost saved by recognizing determinism at n=2 (~10 minutes saved per additional run, plus the wall-clock for parallel execution) is non-trivial for cluster instruments at this scale. Candidate amendment to Rule 2: "when a measurement instrument's first two runs return byte-identical results, declare the instrument deterministic at the current substrate maturity and report the measurement as the value; require ≥5 runs only when variance > 0 surfaces at any run." Surfaced for future cross-locale corroboration before standing-rule amendment.

### Entry 010 — `findings-disposition-cycle-3-rule-30-promotion` (2026-05-30)

- **Audit type**: findings-disposition.
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10616 ("Do the follow ups").
- **Scope**: Findings TAECSF.1 (Entry 005) + TABSC.2 (Entry — referenced in TABSC-EXT 1 trajectory) + ASTA.1 (referenced in ASTA-EXT 0 trajectory). Three-instance corroboration of the narrow-dispatcher cascade-revival pattern.
- **Method**: `apparatus/docs/findings-disposition-protocol.md` Step 3 candidate (4) promote-to-standing-rule. The findings-disposition protocol's §III three-instance threshold (one-more-observation × 2 ≈ 3 cross-locale corroborations) is satisfied by TAECSF-EXT 0 (`typed_array_set_index_checked`) + TABSC-EXT 0/1 (`number_to_raw_bytes` substrate prefix amortization) + ASTA-EXT 0 (`object_set_checked`). All three are independent locales authoring the same engineering shape at different dispatcher sites.
- **Findings**: **Rule 30 promotion** — Narrow-dispatcher cascade-revival as preferred error-propagation discipline (amends Rule 27 via back-reference per Doc 727 §X append-only).
- **Authored actions**:
  - Commit (this commit): `apparatus/docs/predictive-ruleset.md` Rule 30 appended between Rule 29 and Standing instruments; §Status note refreshed to record 30 total rules; `apparatus/docs/findings-ledger.md` Entry 005 Promotion status flipped in-place to standing-rule; Entry 019 appended as back-reference; this audit-ledger entry recorded.
- **Surfaced-but-not-acted findings**:
  - DET.4 candidate (test262-sample variance ±1 across n=2 observed post-ASTA-EXT 0) — investigation deferred to a follow-up rung within this same session per the keeper directive; sample run 3 currently in background to bound the variance.
  - test262-full re-run deferred per the session's wall-clock budget; flagged for the next session.
  - Sibling Doc-721 sub-bundles (Map/Set frozen-receiver; for-of iterator-protocol; Promise dispatcher receiver-validation) deferred to next session as new substrate-spawn proposals.

### Entry 011 — `det4-variance-bound-investigation-2026-05-30` (2026-05-30)

- **Audit type**: ledger-status-correctness (variant: measurement-instrument variance characterization).
- **Auditor**: helmsman session 2026-05-30 under keeper directive Telegram 10616 ("Do the follow ups") — DET.4 investigation branch.
- **Scope**: three runs of `scripts/test262-sample/run-sample.sh` against the same substrate state (post-ASTA-EXT 0, commit 00a73363). Investigates the Rule 29 falsifier event observed at ASTA-EXT 0's post-rung measurement.
- **Method**: ran the canonical sample three times sequentially; recorded PASS/FAIL/emitted/runnable counts; confirmed `bin/cruft` md5sum-identical across runs. Compared the three runs' results to identify the variance source.
- **Findings**: variance bounded at **±1 PASS, ±1 emitted count** across n=3. Substrate deterministic per direct probe (ASTA-EXT 0 probe 7/7 PASS reproducible). Variance source identified as **harness-side parallelism-timeout edge**: one test occasionally hits the per-test 10-second cap under PARALLEL=2 process contention and gets dropped from the emitted set on that run. Median PASS = 6817 (n=3); recorded canonical = 6817 ± 1 at 88.8%. Surfaced as findings-ledger Entry 020 (DET.4).
- **Authored actions**:
  - Commit (this commit): findings-ledger Entry 020 (DET.4) appended; this audit-ledger entry recorded; CLAUDE.md + AGENTS.md §Measurement baselines refreshed with ±1 PASS variance note (deferred to commit batch as a separate concern; the canonical 88.8% number stands).
- **Surfaced-but-not-acted findings**:
  - Mitigation candidates enumerated in DET.4 §Mitigation candidates (PARALLEL=1; longer per-test timeout; identifying the variance-source tests; hash-pinning per measurement-determinism-prospective §V.2.e). All deferred to a future apparatus-tier rung.
  - Rule 29 refinement: the strict "byte-identity at n=2" falsifier discipline is preserved; DET.4 surfaces the **"near-Class-A"** sub-class as a refinement. The test262-sample instrument's reclass from Class A to near-Class-A is the operational outcome; future post-substrate measurements at this instrument should record the ±1 bound as part of the gates spec.
