# Repartition Audit Algorithm — LPA-EXT 6 output

Reusable apparatus-tier algorithm for repartitioning a broad matrix bucket into coherent locale candidates, scope extensions, redirects, or apparatus refinements.

This algorithm exists because raw matrix count is not a spawn decision. A high-count bucket can contain already-absorbed work, stale rows after recent substrate movement, multiple resolver-layer mechanisms, runner/policy residue, and genuine fresh locale candidates. Repartitioning is the audit layer that prevents a worker from founding a broad locale whose first useful act would have been to split itself.

---

## I. Inputs

Required:

1. A full-suite `interpreted.jsonl` file from `pilots/apparatus/test262-categorize/full-suite/results/<run-id>/`.
2. The matching `matrix.md`.
3. `apparatus/locales/manifest.json`.
4. `apparatus/locales/CANDIDATES.md`.
5. Existing locale seeds and trajectory tails for any potentially overlapping locales.

Optional but recommended:

- Focused exemplar suite results from related locales.
- Recent commits touching the surface.
- Relevant runner skip / policy artifacts if the bucket contains `runner-harness`, stage-X, or host-hook signatures.

---

## II. Output Contract

Each repartition audit produces one Markdown artifact under:

```
pilots/apparatus/locale-positioning-audit/findings/<bucket>-partition.md
```

The artifact must include:

1. Baseline inputs and run-id.
2. Bucket selector and total record count.
3. Surface marginal.
4. Projection marginal.
5. Surface × projection top cells with examples.
6. Existing-locale absorption map.
7. Proposed arcs with one of five dispositions:
   - `spawn-ready`
   - `baseline-first`
   - `scope-extension`
   - `audit-first`
   - `redirect/defer`
8. Candidate queue edits, if any.
9. Findings that explain non-obvious ordering decisions.

---

## III. Algorithm

### Step 1 — Select The Bucket

Define the selector explicitly. Examples:

- `resolver == "ast-to-bytecode/language-lowering"`
- `surface starts with "language.statements.class"`
- `projection == "availability/missing-lowering-feature"`
- `pin == "<full coordinate>"`

The selector must be reproducible from `interpreted.jsonl`. Do not use prose labels from a prior discussion as the selector.

### Step 2 — Aggregate Three Marginals

Compute:

1. Surface marginal.
2. Projection marginal.
3. Surface × projection marginal, with one example path per cell.

Minimum useful cut:

- top 20 surfaces,
- top 15 projections,
- top 50 surface×projection cells,
- and all cells above 1% of the selected bucket.

### Step 3 — Classify Each Top Cell By Mechanism

For each high-count surface×projection cell, assign a mechanism class:

| Class | Meaning | Default disposition |
|---|---|---|
| Single-mechanism substrate | One coherent parser/lowering/runtime fix likely dominates. | `baseline-first` or `spawn-ready` |
| Existing-locale absorption | A current locale already owns the mechanism. | `scope-extension` or `redirect/defer` |
| Stale-after-recent-work | Recent commits probably changed the residual. | `audit-first` |
| Apparatus blur | Projection is uncategorized or reason text is too broad. | `audit-first` |
| Runner/policy residue | Harness, stage-X, host-hook, or deliberate omission. | `redirect/defer` |
| Cross-layer dependency | Bucket label points to one layer but root mechanism is elsewhere. | `baseline-first` with redirect condition |

### Step 4 — Read Existing Locale Coverage

Before proposing a new locale, inspect:

- manifest coord names,
- `CANDIDATES.md`,
- related `seed.md` telos/carve-outs,
- related trajectory tails.

The audit must answer:

1. Is this already founded?
2. Is this a child/nested rung of an existing parent?
3. Did an existing locale explicitly exclude this scope?
4. Did recent work probably move the row distribution?
5. Would a new locale duplicate an active one?

### Step 5 — Emit Arcs, Not Just Rows

Group cells into arcs only when they share a plausible substrate mechanism. A valid arc needs:

- surface family,
- projection family,
- root mechanism hypothesis,
- existing-locale relationship,
- first baseline probe shape,
- redirect condition.

Example arc record:

```
### Arc B — Async Iteration And Async Generators
Coverage: 1,492 rows
Mechanism hypothesis: async-generator object protocol + for-await AsyncFromSync lowering
Existing anchors: for-of-async-lookahead, private-field-runtime-slots narrow async bridge
Disposition: baseline-first
Redirect: if Promise reaction/job ordering dominates, route to runtime/job-queue-promise
```

### Step 6 — Assign Disposition

Use these meanings consistently:

- `spawn-ready`: enough mechanism clarity exists to found a locale immediately.
- `baseline-first`: likely candidate, but Rule-23 baseline must prove the mechanism before founding scope hardens.
- `scope-extension`: extend an existing locale rather than creating a sibling.
- `audit-first`: rerun/repartition/sample before substrate work.
- `redirect/defer`: not a substrate locale now, usually apparatus, runner, policy, or cross-layer dependency.

### Step 7 — Update Candidate Queue

If the audit yields stable arcs, update `apparatus/locales/CANDIDATES.md`.

Rules:

- Add candidate entries only for arcs that have a name, pool, mechanism hypothesis, methodology, and disposition.
- Do not add every subcell as a candidate.
- Mark audit-first and apparatus-first arcs explicitly.
- Preserve existing spawned entries; add updates rather than silently rewriting history.

### Step 8 — Record In LPA Trajectory

Append an `LPA-EXT N` entry with:

- trigger,
- selector,
- generated artifact,
- method,
- top-level counts,
- candidate queue changes,
- findings,
- status.

---

## IV. Baseline Probe Requirements For Spawn-Ready Arcs

Before converting `baseline-first` to a founded substrate locale:

1. Build a 50-100 path exemplar list stratified by the partition's internal family axis.
2. Run the exemplar suite against current `cruft`.
3. Inspect at least 10 failures manually, covering top cells.
4. Classify each inspected failure by actual root layer.
5. State redirect conditions in the seed.

The baseline should prove that the candidate is not:

- already absorbed by a sibling,
- actually runner/policy residue,
- an apparatus categorizer blur,
- a downstream symptom of a runtime chapter,
- or a mixed bucket requiring a parent locale.

---

## V. Falsifiers

The repartition audit is wrong if:

1. A spawned locale immediately discovers that its top exemplar failures belong to a different resolver layer.
2. More than half of the candidate's exemplar failures are absorbed by an existing locale's current focused suite.
3. The first substrate move produces no movement and baseline inspection would have revealed the mismatch.
4. A new candidate duplicates an active locale's declared telos/carve-outs.

When falsified, do not patch the candidate silently. Append a new LPA finding that records the bad partition and the corrected mechanism.

---

## VI. Minimal Reproduction Command

The core aggregation can be reproduced with a small JSONL pass over `interpreted.jsonl`:

```
filter by selector
count by surface
count by projection
count by surface + projection
record first example path per surface + projection
```

This does not need a permanent tool yet. If the same algorithm is run three more times, promote it into `pilots/apparatus/locale-positioning-audit/scripts/` and have future partition docs cite the script invocation.

