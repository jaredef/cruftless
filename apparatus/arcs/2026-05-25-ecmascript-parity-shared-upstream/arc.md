---
arc: 2026-05-25-ecmascript-parity-shared-upstream
trigger: Successor arc to T262C's first ECMAScript-parity round per T262C-EXT 2 prospective analysis (2026-05-25). Promoted from pilots/ecmascript-parity-shared-upstream-arc/ on 2026-05-28 per apparatus/docs/coverage-gap-orphan-disposition-2026-05-28.md (the seed was filed as a locale but is structurally an arc per Doc 744 + 745 candidate; canonical instance of pattern III.1 arc-tier-as-locale mis-categorization).
opened: 2026-05-25
closed: 2026-05-28 (CHAPTER CLOSED at EPSUA-EXT 4 per log; header reconciled 2026-05-31 per Telegram 10751)
close_condition: post-five-shared-substrates target ~85% runnable rate on test262-sample (+340 PASS cascade from the 80.6% baseline); zero PASS→FAIL regressions per round per Finding T262C.5 default discipline.
---

# EPSUA: ECMAScript Parity Shared-Upstream Arc

## Provenance

This arc was originally filed as a top-level locale at `pilots/ecmascript-parity-shared-upstream-arc/seed.md` (founded 2026-05-25). The 2026-05-28 coverage-gap orphan-disposition exercise (per keeper Telegram 10160; doc at `apparatus/docs/coverage-gap-orphan-disposition-2026-05-28.md` §II.1) identified it as the canonical instance of **pattern III.1 arc-tier-as-locale mis-categorization**: the seed's telos enumerated five sub-substrates at the same coordinate tier as the seed itself, which is the arc shape per Doc 744 + Doc 745 candidate. The promotion preserves the original 2026-05-25 founding date in the arc slug; original seed.md + trajectory.md content is migrated to this arc.md + log.md.

---

## I. Telos

**Empirical answer to**: when the five named shared-upstream constraints close, does the runnable-rate gain match the projection (~340 cascade across 5 substrates, ~+4.5 pp on the runnable rate), with zero PASS→FAIL regressions per round (per Finding T262C.5 default discipline)?

### I.1 Resume-vector projection (sub-locale ordering)

Per the prospective doc §V — ordering by Doc 740 leverage × Rule 14-mirror risk × blast radius:

| Order | Sub-locale | Constraint # | Projected cascade | Tier | Risk |
|---:|---|---|---:|---|---|
| 1 | `host-262-shim` | #3 | ~38 | host | minimal (pure additive) |
| 2 | `iterator-close-on-abrupt` | #4 | ~25 | bytecode+runtime | bounded (abrupt-completion paths only) |
| 3 | `parser-permissiveness-audit-extensions` | #5 | ~50 | parser | medium (4 distinct sites; each per-site risk bounded) |
| 4 | `strict-mode-parser-tracking` | #2 | ~80 | parser | medium (parser-state extension; per-script vs per-function tracking) |
| 5 | `host-method-prologue-discipline` | #1 | ~150 | runtime | high (touches every host-method registration; resolver-instance-style fix preferred) |

**Total projected: ~340 PASS across 5 sub-locales (~68 PASS/round amortized).**

### I.2 Constraints (engagement-level discipline carried in)

```
C1. Each sub-locale follows Doc 740 multi-tier closure (Finding T262C.5):
    identify R pre-implementation; land all of R as one commit.
C2. Each sub-locale exemplar-verifies before full-sweep authorization;
    full-sweep on keeper directive only per "No Auto Sweeps" feedback.
C3. Each sub-locale's exemplar verification check includes a
    regression probe on previously-passing tests in adjacent
    directories per Rule 14-mirror (adding restriction = false-positive
    risk).
C4. Per Finding T262C.6 refinement: before scoping each sub-locale, probe
    per-cluster failure-REASON heterogeneity. If reasons scatter
    (no reason >40%), pivot to the next shared-upstream candidate;
    do not commit to a heterogeneous cluster as a focused-fix target.
C5. Each sub-locale's chapter-close report includes the
    cumulative-vs-projected ratio. If <50% of projection, re-probe
    per C4 before scoping the next.
C6. Per CLAUDE.md: every commit user-authorized. Trajectory entries
    land with the commit they describe.
```

### I.3 Falsifiers

**Pred-epsua.1**: closing all 5 named constraints lifts test262-sample runnable rate to ≥84% (currently 80.6%; projected 85%).
**Pred-epsua.2**: zero PASS→FAIL regressions per round on full-sweep verification (T262C.5 default discipline holds).
**Pred-epsua.3**: cumulative cascade is within ±30% of the per-constraint projections in aggregate. (Per-constraint variance is allowed; aggregate prediction is the bound.)
**Pred-epsua.4**: at least 2 of 5 constraint sub-locales materialize the projected cascade within 1 implementation round per Pred-epsua.3 (otherwise the substrate-introduction-prefix shape would dominate and the projection was wrong).
**Pred-epsua.5 (DISCIPLINE — Rule 13 prospective + Finding T262C.5)**: each sub-locale closes in ≤2 implementation rounds.
**Pred-epsua.6 (METHODOLOGY)**: Finding T262C.4 (shared-upstream vs mutually-exclusive discriminator) is empirically corroborated by ≥2 of the 5 sub-locales materializing within the projected cascade range. If <2 corroborate, the discriminator's per-cluster reason-spread metric needs refinement.

## II. Apparatus + Methodology

- **Instrument**: T262C (existing) + per-sub-locale exemplar suites per substrate (host-method receivers, iterator-close fixtures, parser test fixtures, strict-mode fixtures, host-method prologue fixtures).
- **Gates**: canonical fuzz (acc=−932188103), diff-prod 42/42, TCC parse-parity 100%, TXC execute-parity ≥70.9%.
- **Verification cadence**: exemplar suite per sub-locale; full-sweep on keeper authorization.

Methodology per sub-locale:
1. **EXT 0** — workstream founding (seed + trajectory + manifest refresh).
2. **EXT 1** — implementation + exemplar verification + chapter close.
3. **EXT 2+** — only if Pred-epsua.5 (≤2 rounds) requires further work.

## III. Carve-outs

- Mutually-exclusive long-tail clusters (Object.defineProperty heterogeneity, Array.prototype.{reduce,indexOf,map,filter} edge cases, String.prototype.{trim,split}, Promise/JSON/RegExp/Number edge cases) are OUT OF SCOPE for this arc. They are residual work for a future arc or accepted-residual at the cluster scope.
- TCC and TXC instrument-tier work continues independently per their own seeds.
- The five named constraints are the FIXED scope for this arc; emergent shared-upstream candidates surfaced mid-arc require a new arc-tier scope decision per Doc 737 §II.

## IV. Standing artefacts

- `pilots/ecmascript-parity-shared-upstream-arc/seed.md`, `trajectory.md`
- Per-sub-locale `pilots/<name>/seed.md` + `trajectory.md` (5 sub-locales)
- Per-sub-locale exemplar runner scripts (when needed beyond inline bash)

## V. Resume protocol

Read this seed + trajectory tail. The arc has 5 sub-locales queued in dependency order per §I.1. Each sub-locale's resume is in its own `pilots/<name>/seed.md` once spawned. To pick up the arc:
1. Read this seed (this file).
2. Read trajectory.md tail (most recent EPSUA-EXT entry).
3. Read the next-queued sub-locale's seed (per §I.1 ordering, starting with `host-262-shim` for EPSUA-EXT 1).
4. Run T262C `cargo run --release -p test262-categorize --bin t262c` on the latest results.jsonl to verify the current matrix matches the arc's expected state.
5. Begin the next sub-locale's EXT 0 founding.

## VI. The arc this locale operationalizes

```
ecmascript-parity-shared-upstream-arc (THIS LOCALE)
   ↓ five sub-locales in dependency order ↓
  ├ host-262-shim (constraint #3, ~38 cascade)
  ├ iterator-close-on-abrupt (constraint #4, ~25 cascade)
  ├ parser-permissiveness-audit-extensions (constraint #5, ~50 cascade)
  ├ strict-mode-parser-tracking (constraint #2, ~80 cascade)
  └ host-method-prologue-discipline (constraint #1, ~150 cascade)
   ↓ when ≥4 of 5 close per Pred-epsua.1 ↓
test262-sample runnable rate ≥85% (projected)
   ↓ if Pred-epsua.6 corroborated ↓
Finding T262C.4 promotes to corpus
   ↓ residual = mutually-exclusive long-tail ↓
Future arc(s) for per-cluster heterogeneous-bug closures, or accepted-residual
```

The arc's natural-stopping-point is empirical-vs-projected divergence per C5 + Pred-epsua.3. If the arc closes ≥4 of 5 within projection, runnable rate ~85% is achievable; if not, the discriminator's refinement candidates are themselves the deliverable.
