# Orphan-disposition protocol

Standing apparatus-tier protocol for the periodic exercise of fitting locales that do not cluster cleanly under any single arc to a disposition: enroll-in-arc, scaffold-new-arc, lift-to-arc-tier, relocate-to-apparatus-pilot, defer-with-cross-reference, or close-as-singleton.

The protocol is a load-bearing instance of Doc 744 + Doc 745 candidate heuristics operating at the apparatus-discipline tier. It is run periodically (recommended cadence: after every Pin-Art matrix re-categorization, or whenever an arc-back-fit exercise surfaces orphans).

---

## I. When to run the protocol

Trigger conditions:

1. **After a back-fit cluster analysis** (e.g., the Plan-agent analysis run 2026-05-28 per keeper Telegram 10158) flags ≥3 locales that do not cluster under any single arc.
2. **At engagement-wide chapter-close-inspect rungs** (Phase 5 per `CLAUDE.md` §Substrate-shaped-work discipline) where the engagement reviews its locale-to-arc subsumption coherence.
3. **Whenever a locale's seed.md is judged to mis-categorize its tier** (locale-vs-arc or substrate-pilot-vs-apparatus-pilot per the bilateral pilot tier).

The protocol is not run per-locale-spawn; it is a periodic engagement-discipline artifact.

---

## II. The protocol

For each orphan, six operational steps run in order.

### Step 1 — Recover (M, T, I, R, observability) at the locale tier

Read the orphan's seed.md. Derive the four-tuple plus observability per Doc 744 §II.1: what is the locale's mouth (input shape), terminus (emission shape), interior contour (intermediate substrate tiers), relations to neighbor pipelines (DAG / lattice / alphabet-exchange per Doc 744 §IV), observability (ordinary / panic / abort / no-JSON / timeout per Doc 744 §III.4).

### Step 2 — Discriminate the relational form

Apply the Doc 744 §IV.4 discrimination heuristic table. The orphan's relation to its candidate arc(s) is one of: DAG (terminus-feeds-mouth or mouth-gating prerequisite per §IV.1+§IV.1.a), Lattice (shared interior with distinct mouth-terminus pairs per §IV.2), Alphabet-exchange (same-tier typed-primitive contract per §IV.3).

### Step 3 — Test the eight disposition candidates

For each orphan, test eight disposition candidates in order; the first that fits is the disposition.

1. **Enroll in existing arc**: orphan's (M, T, I, R) fits sub-shape correspondence (Doc 745 candidate §IV.3 four conditions) under an existing arc's tuple → enroll in arc's roster.
2. **Enroll in scaffolded but not-yet-rostered arc**: same as 1 but for an arc scaffolded in the same back-fit cycle.
3. **Lift to arc-tier**: orphan's seed.md telos enumerates sub-substrates at the same coordinate tier as the seed itself (pattern III.1) → promote to `apparatus/arcs/<date>-<slug>/`.
4. **Relocate to apparatus-pilot tier**: orphan's primary output is per-method classification / audit findings / measurement instruments (pattern III.3) → relocate to `pilots/apparatus/<name>/`.
5. **Lattice-meet annotation**: orphan shares emit-site enumeration shape AND substrate tier with another orphan (pattern III.2) → group into a triplet/n-tuple with cross-reference annotation; pair-enrollment in the future arc when scaffolded.
6. **Scaffold new arc**: orphan plus ≥3 sibling orphans share a coherent multi-substrate program shape that is not captured by any existing or proposed arc → scaffold new arc per `apparatus/docs/arc-as-coordinate.md`.
7. **Defer with cross-reference**: orphan's natural arc has not been scaffolded and is not in the back-fit's recommended-creation-order → annotate seed.md with the future arc enrollment and the relational form; defer.
8. **Close as singleton**: orphan is a single-rung substrate fix already LANDED with no arc-class roster density and no recurrence prediction → mark as closed singleton with lattice-meet annotations to relevant arcs; no relocation needed.

### Step 4 — Surface cross-orphan patterns

After all orphans are dispositioned, group by disposition. Patterns recurring across ≥2 orphans become candidate findings for promotion via the Doc 727 §X basin-stability append-only protocol. Three patterns have been observed to recur (per `coverage-gap-orphan-disposition-2026-05-28.md` §III):

- **Pattern III.1** — arc-tier-as-locale mis-categorization (a multi-substrate program filed as a single locale)
- **Pattern III.2** — lattice-meet repetition (locales sharing emit-shape + tier should pair-enroll)
- **Pattern III.3** — apparatus-vs-substrate mis-categorization (audit-discipline artifacts filed as substrate-pilots)

A fourth or fifth pattern that recurs at 2+ subsequent orphan-disposition exercises is promoted to a standing rule (Rule 29+ per the predictive-ruleset).

### Step 5 — Execute the disposition action sequence

Per the dispositions assigned in Step 3, execute in order: (a) promote/relocate first (highest-confidence misfiles); (b) annotate cross-references for lattice-meets + deferred enrollments; (c) verify already-enrolled orphans; (d) refresh `apparatus/locales/manifest.json` via `apparatus/locales/discover.sh`.

### Step 6 — Document the exercise

Write a dated apparatus-tier doc (`apparatus/docs/coverage-gap-orphan-disposition-<date>.md`) recording per-orphan (M, T, I, R) + disposition + cross-orphan patterns. The doc is itself a worked instance of the Doc 745 candidate SIPE-T fractal heuristic operating at the apparatus-discipline tier; it composes into the engagement's coverage-gap-history record.

---

## III. Validation

The protocol's predictive use: each subsequent orphan-disposition exercise produces fewer orphans + faster-disposition cadence as the back-fit cycle stabilizes the locale-to-arc subsumption topology. If a subsequent exercise produces MORE orphans than the prior one (without an engagement-wide substrate-class expansion), the protocol's Step 4 pattern-promotion is partially falsified — a missing pattern class is causing repeat-orphan recurrence.

The protocol's falsifier: an orphan disposition that subsequently regresses (the disposition assignment proves wrong; e.g., a "lift-to-arc-tier" promotion that turns out to be a substrate-pilot, or a "lattice-meet" annotation that turns out to be a DAG-feed). Per Doc 744 §III.3 class-three regressions, the timing-edge between the disposition and the subsequent regression is the falsifier signal.

---

## IV. Operational integration

`apparatus/docs/repository-apparatus.md` §III lists discipline artifacts; this protocol is added to the standing-discipline set there. The 2026-05-28 worked example at `coverage-gap-orphan-disposition-2026-05-28.md` is the canonical first instance.

`CLAUDE.md` + `AGENTS.md` §Substrate-shaped-work discipline §Phase 5 chapter-close-inspect gains a pointer to this protocol as the engagement-tier instance of Phase 5's "verify scaffold disposition" obligation.

`apparatus/docs/arc-as-coordinate.md` §F (event log) gains an entry-class for "orphan-disposition annotation" so arc.md updates from orphan-disposition exercises are recorded in the arc's log.md.

---

## V. Cross-corpus references

- **Doc 727 §X** — basin-stability append-only protocol; the pattern-promotion mechanism for Step 4.
- **Doc 729** — resolver-instance pattern; the architectural target the orphan dispositions converge toward.
- **Doc 737** — locale as coordinate; the substrate-tier the orphans inhabit.
- **Doc 744** — pipeline-form discovery as predictive heuristic; the four-tuple (M, T, I, R) recovery in Step 1 + the relational discrimination in Step 2.
- **Doc 745 candidate** — structured per-Phase emission + SIPE-T fractal fitting; the test in Step 3 disposition candidates 1, 2, 6.
- **`apparatus/docs/arc-as-coordinate.md`** — arc-tier artifact format; the target of Step 3 candidates 3, 4, 6.
- **`apparatus/docs/repository-apparatus.md` §0** — bilateral pilot tier (apparatus vs substrate); the target of Step 3 candidate 4.

---

## VI. Status

Formalized 2026-05-28 per keeper directive Telegram 10164 (after the 2026-05-28 worked example exercise per Telegram 10160 + 10162). Canonical first run at `coverage-gap-orphan-disposition-2026-05-28.md`. Subsequent runs append dated docs in the same naming convention; this protocol doc is the standing reference.
