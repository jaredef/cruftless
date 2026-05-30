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
