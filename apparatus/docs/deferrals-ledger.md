# Deferrals Ledger

A standing apparatus-tier record of candidate locales (or candidate sub-substrate moves within an open locale) that a substrate rung surfaced but did not yet found. Per keeper directive (Telegram 10176): deferrals currently live scattered across `apparatus/locales/CANDIDATES.md` DEFERRED class + per-rung trajectory cross-locale notes, and the latter does not aggregate — once the trajectory tail rolls past, a flagged candidate is functionally invisible.

This ledger is the aggregator. Modeled on `apparatus/docs/deletions-ledger.md` (Telegram 9800 precedent: an asymmetry between "tracked substrate moves" and "untracked substrate moves" is corrosive when both classes carry methodological signal).

## Why a ledger

Cruftless's apparatus has rich machinery for tracking new substrate work (locale seeds, trajectories, manifest, standing-rule additions, findings.md, deletions-ledger). It had nothing for tracking **substrate work that was surfaced but deliberately not yet undertaken**.

The asymmetry is real and corrosive:

- A founded locale carries seed + trajectory + manifest entry; the cybernetic loop reads it.
- A deferral is named once at the surfacing rung and then — apart from CANDIDATES.md's DEFERRED class, which is keeper-curated and lossy — disappears from the apparatus's read surface.
- When the gating condition later closes (the upstream DAG terminus lands, the spawn threshold is reached, the consumer-app surfaces the divergence), no one is reading for the un-defer signal. The flagged candidate is forgotten until accidentally re-surfaced.

This ledger restores the binding. Each entry records:

1. **Candidate name** — the proposed locale (or sub-substrate move within an open locale) that was surfaced but deferred.
2. **Originating rung** — the locale + EXT # whose Phase-5 chapter-close-inspect (or equivalent surfacing) flagged the candidate. Often a `**Cross-locale note**` section in the rung's trajectory entry.
3. **Date flagged** (absolute, per CLAUDE.md em-dash discipline anchor).
4. **Class** — one of:
   - **mouth-gating** (Doc 744 §IV.1.a) — gated by an unclosed upstream-DAG terminus that another locale or rung will close.
   - **spawn-threshold** (Doc 737 §II) — the duplication count of the same substrate-shape across cells is below the locale-promotion threshold; defer until more cells exhibit the shape.
   - **cost-positive** (Doc 744 §V.3 C3) — the value/cost ratio is below 1 at the current consumer surface; defer until a consumer-app surfaces the need.
   - **consumer-app-driven** — speculative substrate work whose empirical anchor is absent; defer until a real-world divergence surfaces.
   - **probe-pending** — a baseline-inspect probe is needed before the locale can be founded with M-T-I-R confidence.
5. **Gating predicate** — the named upstream condition (specific DAG terminus / specific cell count / specific consumer surface / specific probe).
6. **Un-defer condition** — the observable signal that would promote the candidate from DEFERRED to FOUNDED. Future readers of this ledger should be able to detect un-defer events without re-deriving the original rung's reasoning.
7. **Status** — DEFERRED (open) / PROMOTED (founded; cite the founding rung) / SUPERSEDED (the gating condition resolved by a different route; cite the route) / RETRACTED (the candidate is no longer thought to carry yield; cite the rung that retracted).

## Discipline (append-only)

Per Doc 727 §X basin-stability discipline (same as findings.md, deletions-ledger.md): this file is **append-only**. New entries go at the bottom in chronological order. Older entries are never edited; if a candidate is PROMOTED / SUPERSEDED / RETRACTED, append a NEW entry citing the prior with a back-reference and update the prior entry's Status field in place (the single allowed in-place edit, per the deletions-ledger.md precedent for status-flips on prior entries).

## Discovery hook

Phase 5 chapter-close-inspect (Rule 15) and Phase 6 deferral-emission (proposed sibling, per the TAWR-EXT 3 cross-locale note pattern) are the two phases at which deferrals are surfaced. When a rung's Phase-5 inspection surfaces a candidate that does not meet the founding threshold of its proposed coordinate tier, the rung MUST emit a deferrals-ledger entry as part of the close, in addition to the trajectory's cross-locale note.

The standing rule the ledger formalizes: **a substrate rung that surfaces a candidate locale but does not found it owes the apparatus a ledger entry, not merely a trajectory note.** The trajectory note records the rung-local reasoning; the ledger entry makes the candidate readable from outside the originating locale.

---

## Entries

### Entry 001 — `bigint-arithmetic-wrongness` (2026-05-28)

- **Candidate name**: `bigint-arithmetic-wrongness` (proposed locale at `pilots/bigint-arithmetic-wrongness/`).
- **Originating rung**: `typed-array-wrong-result/trajectory.md` TAWR-EXT 3, "Cross-locale note" section.
- **Class**: spawn-threshold (Doc 737 §II) — currently one cell observed (asIntN/asUintN closed by EXT 3); the locale's locus repeats across at least three more cells in the BigInt namespace that share the same substrate shape (passthrough-stub-as-deferment + ordered-coercion-as-substrate-concern).
- **Gating predicate**: cell count of cells exhibiting the bigint-arithmetic-wrongness shape ≥ 3. Currently 1 (the EXT 3 close).
- **Un-defer condition**: a substrate rung in this engagement surfaces a second BigInt-namespace passthrough-stub / wrong-arithmetic / wrong-coercion-ordering failure on test262 or diff-prod that shares Finding TAWR.3's shape. Candidate cells to inspect: `BigInt(arg)` constructor, `BigInt.prototype.toLocaleString` (currently routes through `to_radix(10)` without locale handling), `BigInt.prototype.toString(radix)` (radix validation), bigint operator-overloaded arithmetic edge cases (modulo sign, divmod by zero error path), the Atomics.* BigInt overloads. If two of these surface failures matching the shape, found the locale.
- **Status**: DEFERRED.

### Entry 002 — `cruftscript-spec` (2026-05-24, back-fill from CANDIDATES.md (k))

- **Candidate name**: `cruftscript-spec` (proposed locale at `pilots/cruftscript-spec/`).
- **Originating rung**: TSR-EXT 5 (annotation-sidecar probe at IPBR consumer); citation in `apparatus/locales/CANDIDATES.md` Tier-A § (k).
- **Class**: probe-pending + cost-positive (Doc 744 §V.3 C3) — the C3 cost-positive condition failed at the 2026-05-24 probe; the probe returned NULL signal at the IPBR consumer for substrate-grounded rationale.
- **Gating predicate**: TSR-EXT 5's annotation-sidecar probe re-runs and returns either (a) positive signal (consumer-tier rationale for cruftscript-spec → found on grounded substrate claims) or (b) confirmed-null signal a second time (found on soundness-alone grounds with a smaller corpus claim per the candidate's note).
- **Un-defer condition**: second probe lands with either signal; the spec proceeds along the matching branch.
- **Status**: DEFERRED.

### Entry 003 — `ts-resolve-*` sub-locales (2026-05-XX, back-fill from CANDIDATES.md (m-s))

- **Candidate name**: `ts-resolve-{import-graph, declaration-files, paths-mapping, project-references, ...}` — sub-locales of the TypeScript-resolver arc.
- **Originating rung**: TCC-measurement instrument at `pilots/apparatus/ts-consumer-corpus/`; citation in `apparatus/locales/CANDIDATES.md` Tier-A § (m-s).
- **Class**: probe-pending — the TCC failure table is needed to gate per-sub-locale spawning.
- **Gating predicate**: TCC parse-parity measurement instrument runs against the curated TS-consumer corpus and emits a failure table partitioned by ts-resolve-sub-substrate.
- **Un-defer condition**: TCC failure table lands; sub-locales spawn against the empirically anchored failure-shape coordinates.
- **Status**: DEFERRED.

### Entry 004 — `gpi-override-safety` (back-fill from CANDIDATES.md (d))

- **Candidate name**: `gpi-override-safety` (proposed locale).
- **Originating rung**: GPI-EXT 3 cost analysis; citation in `apparatus/locales/CANDIDATES.md` Tier-A § (d).
- **Class**: consumer-app-driven — a synthetic correctness fixture surface; no empirical anchor in the current consumer-app surface.
- **Gating predicate**: a real-world consumer-app (npm-package-tier surface) surfaces the divergence where user-installed own-property override of an intrinsic key is observed under GPI cache.
- **Un-defer condition**: any test262 failure or consumer-app divergence whose root-cause is the GPI override-safety hole.
- **Status**: DEFERRED.

### Entry 005 — `module-loader-eager-cache` (back-fill from CANDIDATES.md (f))

- **Candidate name**: `module-loader-eager-cache` (proposed locale).
- **Originating rung**: cold-start perf observation; citation in `apparatus/locales/CANDIDATES.md` Tier-B § (f).
- **Class**: consumer-app-driven — no current empirical anchor; would need a "module-load-perf" arc with ≥3 sibling locales to justify the arc-tier coordinate.
- **Gating predicate**: a consumer-app surfaces a cold-start surface where module-loader is the dominator; AND ≥2 sibling module-load-perf candidates surface.
- **Un-defer condition**: CRB or consumer-app probe surfaces a cold-start fixture whose component A/B identifies module-loader as the dominator.
- **Status**: DEFERRED.

### Entry 006 — `regex-jit-precompile` (back-fill from CANDIDATES.md (g))

- **Candidate name**: `regex-jit-precompile` (proposed locale).
- **Originating rung**: regex-perf observation; citation in `apparatus/locales/CANDIDATES.md` Tier-B § (g).
- **Class**: consumer-app-driven — no current empirical anchor.
- **Gating predicate**: CRB or consumer-app probe surfaces a regex-heavy fixture whose component A/B identifies regex-compile/match as the dominator.
- **Un-defer condition**: empirical anchor lands.
- **Status**: DEFERRED.

### Entry 007 — `crypto-sha256-batch-investigation` (back-fill from CANDIDATES.md (i))

- **Candidate name**: `crypto-sha256-batch-investigation` (proposed locale within host-runtime-api umbrella).
- **Originating rung**: host-runtime-api surface speculation; citation in `apparatus/locales/CANDIDATES.md` Tier-C § (i).
- **Class**: consumer-app-driven — speculative.
- **Gating predicate**: a consumer-app surfaces a sha256-batched workload where the per-batch dispatch overhead is observable.
- **Un-defer condition**: empirical anchor lands.
- **Status**: DEFERRED.

### Entry 008 — `prototype-constructor-reverse-edge-audit` (2026-05-28)

- **Candidate name**: `prototype-constructor-reverse-edge-audit` (proposed apparatus-pilot at `pilots/apparatus/prototype-constructor-reverse-edge-audit/` — audit-tier, NOT a substrate locale per orphan-disposition Pattern III.3).
- **Originating rung**: `typed-array-wrong-result/trajectory.md` TAWR-EXT 4, Phase 6 deferral emission.
- **Class**: spawn-threshold (Doc 737 §II) — three offenders surfaced at one rung (ArrayBuffer, DataView, BigInt). Below the ≥5 threshold for an audit-pilot LIFT scan. Defer until a fourth offender surfaces OR until the keeper directs a sweep.
- **Gating predicate**: any future rung surfaces a fourth built-in whose `prototype.constructor` reverse-edge is missing (the failure shape: `Object.getPrototypeOf(instance).constructor === Object` rather than the expected ctor).
- **Un-defer condition**: fourth offender lands, OR keeper directs an apparatus-pilot sweep over the constructor-registration helpers in `intrinsics.rs` to enumerate every `set_own_frozen("prototype", proto)` site and verify a paired `set_own_internal("constructor", ctor)` exists.
- **Status**: DEFERRED.
