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
