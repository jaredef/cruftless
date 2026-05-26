# test262-categorize — Resume Vector / Seed

**Locale tag**: `L.test262-categorize` (top-level)

**Status as of 2026-05-24**: **CHAPTER CLOSED at T262C-EXT 1 (1 implementation round; standing rule 13 thirteenth corroboration)**. Categorize apparatus operational; baseline 77.6% (5576/7182). Top-15 cells = 25.5% (Pred-t262c.2 FALSIFIED on cross-product but top-10 pipelines alone = ~50% via the structure-axis marginal). Mid-round: surfaced + fixed an assert-global-shadow bug in cruftless that was masking ~511 real substrate gaps; fix unmasked them without changing the headline. **5 prioritized substrate sub-locales queued** per the matrix (strict-mode-destructuring-refs, parser-permissiveness, iterator-protocol-error-propagation, Object.defineProperty edge cases, Array.sort edge cases).

**Historical status (founding)**: WORKSTREAM FOUNDED (T262C-EXT 0). Spawned per keeper directive on the ECMAScript-parity pivot following the TS-parity arc's 100% close. This locale is the **third instrument-tier locale** in the engagement (after TCC, TXC), built to support the ECMAScript-parity arc with the same corpus-driven inspect-then-iterate discipline that drove the TS-parity arc to closure.

**Workstream**: build a categorization apparatus over `scripts/test262-sample/`'s per-test failure outputs that indexes failing tests by TWO orthogonal coordinates:

- **Structure axis** (per Doc 720 static pipeline DAG): which pipeline / static-DAG-edge does the failing test exercise at the divergence point?
- **Data axis** (per the [keeper-named missing coordinate](../../apparatus/docs/repository-apparatus.md) — input-conditioned realized-trace projection over Doc 720's static topology): what was the input value-shape at the divergence point?

The resulting failure-frequency table is indexed by `(pipeline-edge × data-shape)` pairs. Cells in this matrix become substrate-fix targets; closing a cell removes a coordinate from the realized-trace divergence distribution. The matrix is the empirical anchor for the ECMAScript-parity arc's prioritization.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: TCC, TXC (the engagement's other instrument-tier locales).
**Composes with**:
- [Doc 720](../../docs/corpus-ref/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate.md) — static pipeline DAG (the structure axis)
- [Doc 728](../../docs/corpus-ref/728-tag-on-the-dag-sequential-index-collision-as-protocol-signal-that-the-substrate-has-become-the-coordinate-system.md) — the precedent for "the substrate has an axis the representation doesn't"
- [Doc 729 + 742](../../docs/corpus-ref/742-the-resolver-instance-pattern-at-full-strength-downstream-dispatch-and-upstream-elision-as-doc-729s-empirical-refinements-from-a-typescript-parity-research-arc.md) — resolver-instance boundary contract; the structure-axis fixes route per (O1/O2/O3)
- [Finding TSR.1 (TS-parity arc)](../ts-resolve/trajectory.md) — the empirical null on IPBR shape-witness; retrodiction by the data-axis framing (the witness was structure-axis; cost lives on data-axis)
- [Finding IX.8](../rusty-js-jit/findings.md) — SIPE-T scale-invariance; each failing test is a full-size SIPE instance
- Standing instrument `scripts/test262-sample/run-sample.sh` (existing) — produces per-test PASS/FAIL/SKIP outputs the categorizer consumes

## I. Telos

**Empirical answer to**: indexing the test262 failure distribution by `(structure-axis-edge × data-shape)` produces an actionable failure-frequency matrix whose top cells route to specific substrate-fix work, in the same way TCC's failure-table routed TSR substrate work to closure.

**Headline target**: the matrix's top-N cells should account for ≥50% of failing tests (the high-leverage substrate gaps); each top cell should be addressable via a single substrate fix or coherent sub-locale.

### I.1 First-cut scope (T262C-EXT 1)

Per standing rule 13 + the SIPE-T fractal framing: design the v1 instrument with BOTH coordinates from the start, not structure-axis only. The retrodiction predicts data-axis fixes are the load-bearing class once structure-axis converges.

**Apparatus**:
- Input: `scripts/test262-sample/results.jsonl` per-test outputs (already produced by the existing runner).
- Per-failing-test extraction: read the test file's path → look up which Doc 720 pipeline it exercises (path-prefix heuristic + a small static map from test262 directory layout to pipeline). Capture the test's negative-test type / expected error / actual error.
- Per-failing-test data-axis extraction: parse the test source for value-shape declarations (`/*---features: ... ---*/`, `assertEq(typeof x, "...")`, etc.). For tests without explicit type hints, default-tag with the test's CATEGORY (numeric / string / object / etc.).
- Output: `pilots/test262-categorize/results/<date>/{matrix.md, results.jsonl}`.

### I.2 Constraints

```
C1. The instrument consumes the existing test262-sample runner's
    output; doesn't re-run tests.
C2. Per-test categorization runs in <10 s on the full 7,205-test
    failure set (the failing subset is ~1,611 tests as of
    2026-05-22; well under the budget).
C3. Categorization is RECORDED, not GUESSED — if the instrument
    can't determine the (pipeline, shape) for a test, it tags
    `(uncategorized, uncategorized)` and surfaces the test for
    human review.
C4. Per Doc 720's pipeline taxonomy: the structure-axis set is the
    FIXED set of pipelines named in Doc 720. New pipelines added to
    the DAG go through Doc 720 first; this instrument inherits.
C5. The data-axis is open-vocabulary at first cut — tags collected
    from test262 metadata (features, includes, flags). A canonical
    closed taxonomy emerges from the empirical distribution after
    T262C-EXT 1's first measurement.
```

### I.3 Falsifiers

**Pred-t262c.1**: instrument runs in <10 s on the failing-test set; emits matrix + JSONL.
**Pred-t262c.2**: top-15 matrix cells account for ≥50% of failing tests (actionable distribution).
**Pred-t262c.3**: each top-15 cell's example test is inspectable in <5 min; the cause is namable as a substrate gap.
**Pred-t262c.4 (DOC 742 BOUNDARY CONTRACT TEST)**: when fixes are applied per the resolver-instance routing, the matrix's cell-distribution SHIFTS in the predicted direction — structure-axis fixes close structure-axis cells; data-axis fixes close data-axis cells. Cross-axis bleed (a structure fix closing a data-axis cell) is an empirical anomaly worth investigating.
**Pred-t262c.5 (DISCIPLINE — standing rule 13)**: closes in ≤3 implementation rounds.

## II. Apparatus + Methodology

- **Existing instrument consumed**: `scripts/test262-sample/run-sample.sh` (no changes; this locale builds atop).
- **New instrument**: `pilots/test262-categorize/derived/src/bin/categorize.rs` (Rust binary) — reads results.jsonl + test file paths, produces matrix.
- **Static-DAG map**: `pilots/test262-categorize/docs/pipeline-map.md` — manually-derived map from test262 directory paths to Doc 720 pipelines.

Methodology:
1. **T262C-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **T262C-EXT 1** — apparatus build + first matrix measurement + chapter-close.
3. **T262C-EXT 2 (if needed)** — categorization refinement based on T262C-EXT 1 data.

## III. Carve-outs

- The instrument categorizes, it does NOT run tests. Runner stays at `scripts/test262-sample/run-sample.sh`.
- v1 uses test262-sample (curated 7,205 tests). Expanding to full test262 (50K+ tests) is a separate scope decision.
- Negative tests (test262's expected-error tests) categorized separately from positive-tests at first cut.
- The data-axis taxonomy converges across rounds; v1 is open-vocabulary.

## IV. Standing artefacts

- `pilots/test262-categorize/seed.md`, `trajectory.md`
- `pilots/test262-categorize/docs/pipeline-map.md` (T262C-EXT 1)
- `pilots/test262-categorize/derived/src/bin/categorize.rs` (T262C-EXT 1)
- `pilots/test262-categorize/results/<date>/{matrix.md, results.jsonl}` (T262C-EXT 1+)

## V. Resume protocol

Read this seed, then trajectory.md tail. Read Doc 720 (static pipeline DAG) for the structure-axis taxonomy. Read the keeper's data-axis framing message (booked in trajectory.md TXC-style with the empirical retrodiction). Read TCC's seed + trajectory for the precedent — this locale mirrors TCC's pattern at the ECMAScript tier.

## VI. The arc this locale supports

```
test262-categorize (THIS LOCALE)
   ↓ matrix.md routes priorities ↓
ECMAScript-parity sub-locales (per top matrix cells):
  ├ structure-axis fixes — substrate gaps at specific pipelines
  ├ data-axis fixes — value-shape handling at specific edges
  └ cross-axis fixes — both structure and data gaps cleared
   ↓ when matrix converges to bun's residual (~99.2%) ↓
ECMAScript parity declared on curated sample
   ↓ optional expansion ↓
test262-categorize-full (full 50K+ corpus)
   ↓ when that closes ↓
ECMAScript parity declared at corpus scale
```

The arc's natural-stopping-point at curated parity is the same shape as TS-parity's: substrate work converges to a measurable target; the residual gap is then categorically distinct work (runtime substrate / corpus-runnability / spec deviation), tracked as separate locales.
