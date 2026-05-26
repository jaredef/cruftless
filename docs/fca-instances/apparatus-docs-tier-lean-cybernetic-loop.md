# apparatus/ vs docs/ Tier Separation → Lean Cybernetic Loop Context

## Induced property

The agent's operating context across loop iterations stays **lean and stable**: the rung-1 substrate (`apparatus/`) is loaded uniformly every iteration; the rung-2 keeper supplement (`docs/`) is read only on explicit directive. The induced property is a **cybernetic loop whose context window does not drift toward keeper-tier exploration prose** as iterations accumulate, which would otherwise saturate the resolver's working memory and degrade substrate decision quality.

Anchor: `apparatus/docs/repository-apparatus.md` §0 + `CLAUDE.md` (root) "Canonical tier separation" section. Per [Doc 711](../corpus-ref/711-the-dyadic-ascent-fractal-spiral-recursive-self-similarity-across-tiers-of-the-rung-1-rung-2-dyad.md), the dyadic ascent requires rung-1 and rung-2 to stay distinct.

## The accumulation

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) one undifferentiated docs/ tree | — | no partition; every doc is loop-candidate |
| 1 | **apparatus/ as designated rung-1 surface** — every artifact under apparatus/ is required reading on every loop iteration | iteration-invariant context | property: "the agent's per-iteration operating context is bounded by apparatus/" |
| 2 | **apparatus/docs/ contains only discipline schemas + standing rules + apparatus enumeration** — no exploratory prose, no in-flight designs | discipline-vs-exploration partition | property: "rung-1 docs are stable across iterations; their content shape doesn't change in response to keeper thinking" |
| 3 | **apparatus/locales/ is the locale registry** — manifest.json + CANDIDATES.md + discover.sh; not workstream content | apparatus-tier-locale registry | property: "the locale graph is consulted as data, not as narrative" |
| 4 | **docs/ as designated rung-2 surface** — keeper sidecar, read only on explicit directive | bounded keeper supplement | property: "exploratory prose does not leak into the loop unless the keeper asks for it" |
| 5 | **docs/corpus-ref/ is a read-only mirror** — published corpus docs the keeper composes against; not authored locally | corpus boundary discipline | property: "rung-1 substrate work cannot accidentally edit corpus articulations" |
| 6 | **docs/engagement/ houses in-flight keeper designs** — analyses, prospective drafts, phase designs | keeper thinking surface | property: "keeper thinking has a dedicated locus separate from agent operations" |
| 7 | **Promotion path: docs/engagement/ → apparatus/docs/ (rule/schema crystallization) OR docs/engagement/prospective/ → docs/corpus-ref/NNN-*.md (corpus publication) OR docs/engagement/ → seed.md (locale founding)** — explicit promotion only | promotion-as-act | property: "rung-2 → rung-1 promotion is a deliberate keeper move, not a drift" |

The named composition is the **apparatus/-vs-docs/ tier separation**. The induced property is a lean cybernetic loop whose per-iteration context is dominated by apparatus/ content, with keeper-tier exploration available on-demand without saturating the loop.

Removing constraint 1 (no required rung-1 reading) means agents drift into whatever docs they encounter; the loop becomes context-window-dependent.
Removing constraint 2 (allow exploratory prose in apparatus/docs/) means rung-1 content shifts as keeper thinks; the loop becomes unstable across iterations.
Removing constraint 4 (no rung-2 partition) means every keeper note pollutes the loop; agents over-read and under-act.
Removing constraint 5 (allow local corpus edits) means promotions can happen accidentally; the published corpus drifts under agent work.
Removing constraint 7 (allow implicit promotion) means rung-2 content silently becomes rung-1 binding; the discipline degrades to convention.

## Tag on the DAG

This is an **engagement-tier discipline coordinate**, not a substrate coordinate. The pin:

```
apparatus/discipline ::
  E0/cybernetic-loop-context ::
  cut/dyadic-tier-separation ::
  property/lean-loop-context
```

Observable property: the agent's per-iteration loaded-files set is bounded by apparatus/ (and the specific locale's `pilots/<name>/`). docs/ reads happen only on directive — every directive is a logged interaction (keeper said "read Doc N"). The pattern's failure mode is observable as agent context-window saturation in long sessions; the discipline's effectiveness is observable as session longevity without compression.

## Composes-with

- `apparatus/docs/repository-apparatus.md` §0 — primary articulation.
- Doc 711 — dyadic-ascent fractal spiral; the rung-1/rung-2 distinction's theoretical anchor.
- [`docs/fca-instances/pin-art-resume-vector.md`](pin-art-resume-vector.md) — the resume vector is what rung-1 substrate-loci enable.
- [`docs/fca-instances/agent-feedback-cross-resolver-legibility.md`](agent-feedback-cross-resolver-legibility.md) — cross-resolver review is bounded by the rung-1 surface.

## Falsification

A session in which agent decision quality degrades AS context accumulates without prior keeper-directed rung-2 reads falsifies the leanness property — it would indicate apparatus/ content itself is drift-inducing. Empirically tested every long session; the failure mode would surface as agent decisions becoming more general / less substrate-coordinate-targeted as the conversation extends. The discipline's predictive coverage is the observation that long sessions in the cruftless engagement remain substrate-coordinate-targeted (today's session: 15+ commits, no observable drift, decision quality preserved through to BBND's findings.md analysis).
