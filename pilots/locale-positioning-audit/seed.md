# locale-positioning-audit — Seed

## Meta-apparatus locale opened per keeper directive (Telegram 9802).

Per Finding PPIF.4 — surfaced when PPIF-EXT 2 deleted `rewind_lexer_to` and reduced LGSS-EXT 3's "irreducible carrier" count from 2 to 1 — the apparatus has a coherence-maintenance gap: **when a sibling locale closes successfully and dissolves a constraint another locale named as orthogonal, the prior locale's claims do not automatically update**. The apparatus doc §XI.1.b had to be re-amended this session in response. This is a recurring drift class; this locale exists to systematize the audit.

The audit's posture is **meta-apparatus**: it operates on the locale graph itself (`apparatus/locales/manifest.json` + every `pilots/<name>/seed.md` + `trajectory.md`) rather than on the language substrate. It treats locales as DAG nodes and their cross-citations / irreducibility-claims / scope-borders as DAG edges. Per Doc 711's dyadic-ascent recursive self-similarity, the apparatus auditing itself at the engagement tier is the same shape as the substrate auditing itself at the conformance tier.

## Telos

Materialize the coordinate

```
apparatus/locale-graph ::
  E0/cybernetic-loop-meta-discipline ::
  cut/cross-locale-coherence ::
  property/locale-claims-remain-truthful-as-graph-evolves
```

The induced property is **claim coherence across the locale graph**: when locale A says "X is irreducible within A's scope" and a sibling locale B later names the orthogonal constraint that would reduce X, the audit surfaces the stale claim before a reader trusts it. When locale A's seed cites "spinoff candidate Y" and Y is later spawned (or absorbed into a different locale), the audit traces the chain. When locale A's status reads "CLOSED" but a sibling C in the same coordinate cluster opens new questions about A's surface, the audit flags the re-read.

This is not a code-substrate locale. It produces no test262 yield, no diff-prod movement. Its output is **apparatus-tier hygiene**: refreshed cross-citations, updated irreducibility claims, surfaced spinoff-chain gaps. The apparatus' legibility depends on the locale graph's coherence; this locale maintains it.

## Apparatus

- `apparatus/locales/manifest.json` — the authoritative locale inventory. Currently 109 locales.
- `apparatus/locales/discover.sh` — manifest refresher; run after any locale spawn / rename / delete.
- `apparatus/docs/deletions-ledger.md` — records cross-locale deletion chains; the audit reads this to find "sibling X dissolved Y's claimed-irreducible carrier" instances.
- Every `pilots/<name>/seed.md` + `trajectory.md` — the per-locale claims that may have drifted.
- `apparatus/docs/ecma-conformance-...md` §XI.1.b — the recent example of a §-level apparatus doc that needed amendment when a sibling locale's work invalidated a prior claim.

## Methodology

Three rungs.

### Rung 1 — Stale-claim survey (LPA-EXT 1)

Walk every closed locale's trajectory tail. For each claim of the shape "X is irreducible within scope" / "Y is outside this locale's scope" / "Z is the spinoff candidate that would address W":

- If a sibling locale exists today that names the orthogonal constraint X depended on, flag the claim as POTENTIALLY-STALE.
- If a spinoff candidate Y was named and now exists as a locale, surface the chain explicitly.
- If a spinoff candidate Y was named and does NOT yet exist, surface as candidate-pending.

Produce `pilots/locale-positioning-audit/findings/stale-claims.md` listing each flagged case with: prior claim, sibling locale that dissolved it, recommended amendment.

### Rung 2 — Spinoff-chain mapping (LPA-EXT 2)

Walk the locale graph and identify spinoff chains: locales that exist because another locale's Finding surfaced them. Build `pilots/locale-positioning-audit/findings/spinoff-chains.md` with each chain rendered as:

```
ParentLocale-FindingN → ChildLocale-EXT-0 founding
ChildLocale-FindingN → GrandchildLocale-EXT-0 founding
...
```

Each chain captures the FCA-amortization stack: each tier's named-constraint surfaced the next tier's named-constraint candidate. Today's chain `LGSS → PPIF → FHNB` is the first observed; the audit makes future chains legible.

### Rung 3 — Positioning-gap detection (LPA-EXT 3)

Cross-reference the current full-suite Pin-Art matrix (`pilots/test262-categorize/full-suite/...../matrix.md`) against the locale-coordinate space. For each top-N matrix coordinate:

- Is there a locale assigned to it? If yes, what's the locale's status?
- If no locale exists, is the coordinate in CANDIDATES.md? If yes, what's blocking spawn? If no, surface as candidate-missing.
- Is there a locale whose declared coordinate has SHIFTED (e.g., the cluster's mechanism turned out to be different than seed claimed)? Surface as coordinate-drift.

Produce `pilots/locale-positioning-audit/findings/positioning-gaps.md` enumerating each gap.

## Triggers (when the audit runs)

- **After any deletion** lands per `apparatus/docs/deletions-ledger.md`: re-check whether the deleted carrier was claimed-irreducible elsewhere.
- **After any locale CLOSES**: re-check whether the close invalidates sibling locales' claimed-orthogonal constraints.
- **After any full-suite categorize re-run**: re-check positioning gaps per Rung 3.
- **On keeper directive**: explicit re-audit.

The audit is **opportunistically run, not scheduled**. Its purpose is to maintain coherence at boundaries where coherence has been observed to drift; running it without a trigger event is wasted work.

## Carve-outs

- **Open locales' claims** (status != CLOSED): not audited; their seed/trajectory is still moving and claims-stability is expected to lag substrate work.
- **Per-rung Pred-N falsifiers**: not in scope; those are intrinsic to the locale's own discipline (R15 chapter-close-inspect handles them).
- **Code-substrate coordinates**: not in scope; this is meta-apparatus. The audit produces no test yield, no LoC changes to substrate crates.
- **Corpus-tier claims**: not in scope; the corpus has its own retraction ledger (Doc 415) for handling stale articulations.

## Composes-with

- `apparatus/docs/deletions-ledger.md` (the deletion record that this audit cross-references).
- `apparatus/docs/repository-apparatus.md` §III + §IV (the locale-tier discipline this audit supplements).
- Finding LGSS.5 + LGSS.5-refined-by-PPIF.4 (the empirical instance that surfaced the need for this locale).
- Doc 727 §X basin-stability (the audit operates UNDER basin-stability discipline: it doesn't edit prior trajectories, it appends new findings about them).
- Doc 711 dyadic-ascent (the apparatus auditing itself is the rung-2-supplements-rung-1 instance applied at the engagement-discipline tier).

## R13 prospective check at founding

- **C1 (sibling closure pattern)**: HOLDS — Doc 727's basin-stability discipline + Doc 415's retraction-ledger pattern are siblings at the corpus tier; the audit applies the same shape at the locale-graph tier.
- **C2 (shape-compat with substrate APIs)**: HOLDS — the manifest's JSON format and every locale's seed/trajectory.md text are machine-readable + grep-able.
- **C3 (cost-positive when integrated)**: TBV at LPA-EXT 1 — the audit's per-trigger cost is bounded by the locale count (109 today); each audit run produces a markdown finding doc, not substrate edits.
- **C4 (bail safety)**: HOLDS — the audit appends findings, never edits prior locales' trajectories or seeds. Drift is documented, not corrected unilaterally.

All four conditions hold.

## Resume protocol

Read `trajectory.md` tail.
