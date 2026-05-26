# Pin-Art Locale → Resumable Substrate Work (Resume-Vector Property)

## Induced property

A fresh resolver (LLM agent, new keeper session, future contributor) becomes **operational on any cruftless workstream in one read**: reading `seed.md` (telos + apparatus + methodology) and the `trajectory.md` tail (most recent rungs) suffices to propose the next substrate move without re-deriving the workstream's prior context. The induced property is the **resume vector**: substrate work survives session boundaries, resolver swaps, and context-window resets without state loss.

Anchor: [Doc 581](../corpus-ref/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction.md). Recurrent at every substrate depth per [Doc 733](../corpus-ref/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale.md). Coordinate articulation: [Doc 737](../corpus-ref/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md).

## The accumulation

The Pin-Art discipline stacks constraints on what counts as a valid locale:

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) ad-hoc workstream notes scattered across commits / chat / personal-knowledge | — | no resume; each session re-derives context |
| 1 | **seed.md present** at `pilots/<name>/seed.md` | filesystem locus | property: "the workstream has an address" |
| 2 | **Telos named** explicitly in seed.md (what closure of this workstream looks like) | end-condition contract | property: "next-move proposals can be evaluated against a stated end" |
| 3 | **Apparatus enumerated** — files/instruments/probes the workstream operates on | scope boundary | property: "an agent knows what to read to engage the substrate" |
| 4 | **Methodology stated** — the specific approach (e.g. "extend the existing X helper by adding Y bucket") | concrete protocol | property: "next-move proposals are pre-discriminated by approach" |
| 5 | **Falsifiers stated** — Pred-N predictions whose failure refutes the locale's premise | falsifiable progress | property: "the locale closes only when measurements held" |
| 6 | **Carve-outs explicit** — what is OUT of scope; what mechanisms the methodology does NOT cover | scope precision | property: "downstream sibling locales are pre-identified by carve-outs" |
| 7 | **Composes-with cited** — other locales / corpus docs the work load-bears against | cross-locale linking | property: "the locale is a node in the locale graph, not an island" |
| 8 | **Resume protocol explicit** — "read seed first, then trajectory tail" stated | reader discipline | property: "the resume sequence is invariant across resolver swaps" |
| 9 | **trajectory.md append-only** — each substrate move appends a round; no edits to prior rounds | history-preserving | property: "the work record cannot drift; later rungs read prior rungs as fixed evidence" |
| 10 | **agent-feedback.md when cross-resolver review occurs** — per `apparatus/docs/agent-feedback-schema.md` | cross-resolver legibility | property: "reviews are read through the reviewer's prior frame, not anonymously" |

The named composition (1+2+3+4+5+6+7+8+9, + 10 when applicable) is the **Pin-Art locale**. The induced property is the resume vector: any agent reading the locale's standing-document set becomes operational on the workstream in one read.

Removing constraint 1 (no fixed locus) means the work has no address.
Removing constraint 2 (no telos) means closure cannot be tested.
Removing constraints 3+4+5+6 (apparatus, methodology, falsifiers, carve-outs) reduces seed to documentation, not a contract.
Removing constraint 7 (no composes-with) makes each locale a silo.
Removing constraint 8 (no resume protocol stated) lets each agent invent its own; multiplied across agents, this fractures the discipline.
Removing constraint 9 (allow trajectory edits) lets later rungs rewrite earlier rungs; the work record drifts.
Removing constraint 10 (no agent-feedback) means cross-resolver reviews are anonymous and unweighted.

The composition is multiplicative: each constraint closes a distinct failure mode of substrate-work-across-sessions. The resume-vector property is the conjunction of these closures.

## Tag on the DAG

The Pin-Art locale is an **apparatus-tier discipline coordinate**:

```
apparatus/locale-discipline ::
  E0/cybernetic-loop-constraint ::
  cut/resume-vector-property ::
  property/one-read-operational
```

The pattern does not project onto test262 fixtures. It is observable as the manifest's growth — `apparatus/locales/manifest.json` had 36 locales at 2026-05-24, ~104 after today's top-10 batch + nested spawns + BBND. Each locale's existence-and-validity is itself the empirical claim: a fresh agent reading any of them can become operational.

The pattern's correctness is observable in the agent-feedback.md instances where a second resolver entered a locale and produced a substantive review without re-derivation (pilots/rusty-js-http-server/agent-feedback.md, cross-Claude+GPT-5.5 surface). Failure mode: a locale whose seed.md is stale (telos drifted, apparatus list out of date) breaks the resume vector for new readers — flagged by an agent-feedback "missing block 2 working constraint set" notation per the schema.

## Composes-with

- Doc 581 — primary articulation (Pin-Art apparatus).
- Doc 733 — fractal recurrence at every substrate depth.
- Doc 737 — the locale as coordinate.
- `apparatus/docs/agent-feedback-schema.md` — constraint 10's contract.
- `apparatus/locales/manifest.json` — the registry the locales are enumerated in.

## Falsification

A locale whose seed+trajectory pair does NOT enable a fresh resolver to become operational in one read falsifies the resume-vector property for that instance. Empirically tested every time a new conversation enters a locale; the failure mode (re-reading commits, re-deriving telos, asking the keeper for context) is the negative signal. The discipline's predictive coverage is the manifest entries with `status=CLOSED` reading as "this locale's substrate moves landed via the resume vector" — currently the vast majority of closed locales.
