# Agent Feedback Schema → Cross-Resolver Review Legibility

## Induced property

A review by one LLM resolver (e.g., GPT-5.5) of substrate work landed by another (e.g., Claude Opus 4.7) is **read by the next resolver through the appropriate prior frame**. The induced property is cross-resolver legibility: subsequent agents weight a review by the reviewer's training cutoff + context-window class + tool inventory + prior session memory, not as anonymous documentation.

Anchor: `apparatus/docs/agent-feedback-schema.md`. First instance: `pilots/rusty-js-http-server/agent-feedback.md` (Claude reviewing GPT-5.5's HS-EXT 5 wireup).

## The accumulation

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) cross-resolver review as unstructured commit comments | — | anonymous critiques; next agent cannot weight by reviewer frame |
| 1 | **agent-feedback.md at `pilots/<locale>/`** — co-located with seed.md and trajectory.md | filesystem locus | property: "review has a locale-bound address" |
| 2 | **Block 1: Resolver metadata** — reviewer's model identifier + session description + target commit + target author + ISO date + authority frame | reviewer-as-known-frame | property: "the next resolver reads the review through the reviewer's training-and-context lens" |
| 3 | **Block 2: Working constraint set** — files read + standing artifacts loaded + recent engagement memory + gates re-verified + explicitly NOT loaded | trust signal | property: "the review's depth-of-context is declared, not assumed" |
| 4 | **Block 3: Feedback** — what-lands-well + concerns-ranked-by-leverage + recommended-next-rungs + standing-notes | structured assessment | property: "the review is actionable; each concern includes a fix shape" |
| 5 | **Running summary at file head** (3-5 bullets, distilled load-bearing claims) | compression | property: "the next agent reads the running summary first; full review on demand" |
| 6 | **Append-only entries** — older reviews never edited; running summary mutable in place | history-preserving | property: "the reviewer's read at that moment is the artifact; supersession is recorded as a new review" |
| 7 | **Locale-entry read protocol extension** — every locale entry must check for agent-feedback.md and read running-summary + most-recent review's Concerns + Recommended-next-rungs | discipline-bound consumption | property: "cross-resolver feedback is loop-input, not background" |

The named composition (1+2+3+4+5+6+7) is the **agent-feedback schema**. The induced property is cross-resolver review legibility: a fresh agent entering a locale reads prior resolvers' reads through their declared frame, weighted by the working-constraint-set transparency, and acts on the structured concerns + recommendations.

Removing constraint 2 (no reviewer metadata) means reviews are anonymous; concerns from a deep-context Claude review weight identically to concerns from a fresh-context GPT review; under-weighting and over-weighting both become routine.
Removing constraint 3 (no working constraint set) means review depth is unknown; agents either trust generic reviews and miss substrate-specific nuance, or distrust deep reviews and re-derive.
Removing constraint 4 structure means concerns become opinion blobs without fix-shape; backlog grows but substrate moves don't follow.
Removing constraint 5 (running summary) means each agent must read the full file; over time the file accumulates and the read is unbounded.
Removing constraint 6 (allow edits) means review history drifts; supersession is invisible.
Removing constraint 7 (skip per-entry read) means reviews exist but the loop doesn't consume them.

## Tag on the DAG

This is an **apparatus-tier discipline coordinate**:

```
apparatus/cross-resolver ::
  E0/cybernetic-loop-cross-author ::
  cut/schema-conformance ::
  property/review-legibility-through-prior-frame
```

The pattern's correctness is observable in the running summaries: when the next agent's substrate move addresses a prior reviewer's #1 concern, the summary bullet for that concern is updated or removed in the same trajectory entry. The HS-EXT 5a → 8a sequence is the first round-trip evidence: GPT-5.5's HS-EXT 5 wireup, Claude's review identifying 7 concerns, GPT-5.5's 5a-8a closures addressing 5 of them in parallel.

The schema is recently introduced (`apparatus/docs/agent-feedback-schema.md` landed earlier this session). The empirical anchor base is one round-trip; the predictive coverage is the discipline's prevention of cross-resolver context loss.

## Composes-with

- `apparatus/docs/agent-feedback-schema.md` — primary articulation.
- [`docs/fca-instances/pin-art-resume-vector.md`](pin-art-resume-vector.md) — agent-feedback is the cross-resolver extension of the resume vector.
- [`docs/fca-instances/apparatus-docs-tier-lean-cybernetic-loop.md`](apparatus-docs-tier-lean-cybernetic-loop.md) — agent-feedback is rung-1 (loop input), not rung-2 (keeper sidecar).
- `pilots/rusty-js-http-server/agent-feedback.md` — first empirical instance.

## Falsification

A cross-resolver round-trip in which the second resolver's substrate move ignores the first reviewer's concerns despite the agent-feedback file being present + read falsifies constraint 7's discipline-bound-consumption claim. The mode is observable but rare; per the schema's authoring discipline, concerns without fix-shape are explicitly disallowed (they escalate to keeper instead of landing as concerns the next agent will ignore).

A reviewer's metadata declaration that turns out to be misleading (claimed full Doc 736 read but missed an authority-composition concern visible in Doc 736) would falsify constraint 2's trust signal at that instance. Mitigation: the schema's Block 2 "Not loaded" field is the trust-signal anchor; explicit declaration of omissions makes the review's blind-spots legible.
