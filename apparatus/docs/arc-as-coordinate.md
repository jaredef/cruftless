# arc-as-coordinate — the discipline-tier above locale

## Why this document exists

Per keeper observation (Telegram 9969): "It seems like we need to formalize the arc as indicating many locales, each locale having a seed and a trajectory. The arc needs formal characteristics itself."

Empirically, several discipline-tier units of work have emerged above the locale level — Tier-K (missing-syntax-feature concentration), Tier-L (Temporal implementation), and the post-Tier-K disambiguation. Each functioned coherently across many spawned locales, accumulated cross-locale findings, reached a recognizable close-condition, and produced a yield signature visible in the matrix. None had a first-class apparatus artifact.

This document promotes the arc to a named coordinate alongside the locale, with formal characteristics analogous to seed+trajectory.

## Position in the coordinate hierarchy

```
coordinate level    unit                  artifact pair               registry
----------------    ----                  -------------               --------
0 (deepest)         rung                  (inline in trajectory)      —
1                   leaf locale           seed.md + trajectory.md     manifest.json
2                   parent locale         seed.md + trajectory.md     manifest.json (parent flag)
3                   arc                   arc.md + log.md             apparatus/arcs/
4 (widest)          tier                  CANDIDATES.md section       CANDIDATES.md
```

The arc sits above parent-locale and below tier. Tiers are the apparatus's logical partition (recorded statically in `apparatus/locales/CANDIDATES.md`). Arcs are the **operational unit of work** — a coherent multi-locale program driven by a keeper directive (or chain of related directives) that opens, advances through many sub-locales, and closes.

## What an arc is

An **arc** is a multi-locale execution unit with the following formal characteristics:

### A. Trigger (mandatory)

The arc begins with a single identifiable directive (typically a Telegram message ID) that initiates a multi-locale program. The trigger is recorded with: directive ID, keeper-stated goal, and the matrix-coordinate or scope-statement the goal targets.

### B. Sub-locale roster (accumulating)

Every locale spawned during the arc is listed in the arc's `arc.md` with its name, role in the arc (parent vs leaf, shared-substrate vs per-class, apparatus vs substrate), and landing status. The roster is append-only; locales removed from the program are marked as ★REDIRECTED, never deleted.

### C. Cross-locale findings (load-bearing)

Findings that emerge ACROSS multiple locales within the arc — patterns that don't fit any single locale's trajectory because they describe the program's shape — are recorded in the arc's `log.md` with letter-numbered IDs (e.g., ARC.1, ARC.2). Examples from the Tier-L arc:
- "Per-class template transfers cleanly across N classes" (would have been arc-level finding TI.4 had it existed)
- "Inverse-parser pattern is repeatable" (PTSC + ISC + DSC)
- "BigInt precision matters for since/until" (IA.1, generalizes across per-class)

Per-locale findings stay in the locale's trajectory. Arc findings cite the locales that surfaced them.

### D. Yield trajectory (cumulative)

The arc tracks cumulative yield across all sub-locales (e.g., "Cumulative Temporal yield post-PDDP: 1038/2002 (52%)" lines that appeared in 30+ per-rung trajectories). The arc's `log.md` consolidates these into a single yield-over-time table.

### E. Close-condition (declared at arc-spawn)

Every arc declares its close-condition at spawn time. Examples:
- "Tier-K: all 5 missing-syntax-feature concentration candidates triaged (spawned + landed OR redirected OR marked-deferred)."
- "Tier-L Temporal: at least one sub-rung landed for every Temporal class."

When the close-condition is met, the arc is marked CLOSED in its `arc.md`. Further work on the surface spawns a NEW arc, not a continuation.

### F. Composition with other arcs

Arcs can reference other arcs ("Per-class template established in Tier-L applies here") but don't nest. Each arc is an autonomous unit of work; the relationship between arcs is sequential or branching, not hierarchical.

## File shapes

### `apparatus/arcs/YYYY-MM-DD-<slug>/arc.md`

```markdown
---
arc: YYYY-MM-DD-<slug>
trigger: <Telegram message ID or commit hash>
opened: YYYY-MM-DD
closed: YYYY-MM-DD (or 'IN PROGRESS')
close_condition: <one-sentence>
---

# <Arc title>

## Trigger
<keeper directive verbatim or paraphrased + context>

## Telos
<one-paragraph statement of what the arc targets>

## Sub-locale roster
| Locale | Role | Status | LOC | Direct yield |
| ... | ... | ... | ... | ... |

## Cumulative yield
<table over time as rungs land>

## Cross-locale findings
**Finding ARC.1**: ...
**Finding ARC.2**: ...

## Status
CLOSED (or IN PROGRESS). Close-condition met / pending: ...
```

### `apparatus/arcs/YYYY-MM-DD-<slug>/log.md`

Append-only event log. Each entry is a single rung landing or directive received, timestamped. Mirrors the trajectory format at the locale level but spans the arc. Event classes include: rung-landing, directive-received, sub-locale-enrollment, orphan-disposition-annotation (per `apparatus/docs/orphan-disposition-protocol.md`), and findings-disposition-annotation (per `apparatus/docs/findings-disposition-protocol.md`; entries record when a finding was integrated into the arc as a cross-locale finding, or when an arc was scaffolded by a findings-disposition cycle's lift-to-new-arc disposition).

## What an arc is NOT

- Not a directive log (those live in conversation history and per-locale trajectory entries).
- Not a substitute for tier registration in CANDIDATES.md (tier is the static partition; arc is the operational unit).
- Not nested (no sub-arcs; one arc = one coherent program).
- Not retroactively renamed (an arc's name is its directive-time slug; even if the work pivots, the arc keeps its name).

## Backfill policy

Existing arcs that ran before this formalization may be backfilled. Priority order:
1. Tier-L Temporal arc (largest; most cross-locale findings to register)
2. Tier-K missing-syntax-feature concentration arc (next-largest)
3. Earlier arcs (Tier-K-disambiguation, post-Tier-K landscape survey)

Backfill is optional; the value is forward (future arcs follow the protocol); historical arcs are reconstructed only where the cross-locale findings would otherwise be lost.

## Composition with CLAUDE.md

This document joins the required-reading set per `CLAUDE.md` agent orientation. Future agent sessions should:
1. On a keeper directive that spans multiple locales: spawn an arc at `apparatus/arcs/YYYY-MM-DD-<slug>/`.
2. Each rung landing: append to the arc's `log.md`.
3. Each cross-locale finding: enter under "Cross-locale findings" in `arc.md`.
4. Close the arc when its close-condition is met; pivot decisions spawn new arcs.

## Status

Formalization landed 2026-05-27 per keeper directive (Telegram 9969). Backfill of Tier-L Temporal arc tracked separately.
