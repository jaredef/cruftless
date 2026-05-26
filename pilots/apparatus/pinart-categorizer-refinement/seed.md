# pinart-categorizer-refinement — Seed

## Apparatus-pilot spawned per LPA-EXT 3 Finding LPA.5 (Class A priority).

LPA-EXT 3's positioning-gap audit surfaced that **52% of the top-30 matrix gap fails (2,802 of 5,403) are in Class A: apparatus-refinement candidates**. These are coordinates that pin to `uncategorized/projection :: failure/other` or `uncategorized/resolver` — the apparatus's own measurement residue. Per heuristics §IX, the apparatus is doing its job when its own largest rows name the next refinement; this locale executes that refinement.

The audit's LPA.5 also articulated the priority order: **apparatus-tier refinement BEFORE substrate-tier spawns** for the apparatus-gap class. Substrate locales operating on blurred coordinates produce blurred work. Sharpening the categorizer converts apparatus-tier mass into substrate-tier coordinates with clear move shapes.

This is the engagement's first apparatus-on-apparatus locale: an apparatus-pilot operating on another apparatus-pilot's output (`pilots/apparatus/test262-categorize/`). Per the bilateral-pilot-tier housekeeping (commit 84798b0a §III discriminator), the consumer of this pilot's output is OTHER pilots' substrate work — it IS apparatus-shaped.

## Telos

Materialize the coordinate refinement at

```
apparatus/measurement :: E0/full-suite-projection ::
  cut/categorizer-rule-extension ::
  property/uncategorized-records-shift-to-specific-classes
```

The induced property is **reduction of the uncategorized record count** from the current 3,681 (of 23,520 FAIL records, 15.6%) to a substantially lower number, with the moved records landing in specific projection-class coordinates that downstream substrate-locales can target with clear move shapes.

## Apparatus

- `pilots/apparatus/test262-categorize/derived/src/bin/full_pinart.rs:340-372` — `projection_class` function (the `uncategorized/projection` fallback at line 371 is one extension site).
- `pilots/apparatus/test262-categorize/derived/src/bin/full_pinart.rs:220-258` — `resolver_for` function (the `uncategorized/resolver` fallback at line 256 is the other extension site).
- `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-25-165734-p2/interpreted.jsonl` — the existing apparatus output to re-process; no test262 re-run needed.
- Raw input (sidecar, machine-local): the test262 raw results that the categorizer ingests.

## Methodology

Three rungs.

### Rung 1 — Survey + rule extraction (PCR-EXT 1)

Survey the 3,681 uncategorized records by reason-text shape. Identify the dominant patterns; extract specific-coordinate rules. Initial survey (LPA-EXT 3 sampling):

| Pattern | Count | Specific coordinate |
|---|---:|---|
| `Expected SameValue(«X», «Y»)` | 361 | `value-semantics/wrong-result :: assertion/expected-mismatch` |
| `Expected a TYPE to be thrown` | 176 | `abrupt-completion/throw-missing :: TYPE` (parsed) |
| `should be an own property` | 114 | `descriptor-shape/missing-own-property` |
| `isConstructor invoked with non-function` | 66 | `availability/missing-method-or-intrinsic :: failure/other` |
| `unsupported by the v1 regex engine` | 5 | `partial/regex-features-missing` |
| `lex error: unterminated regex` / `RegExp lex` | 39 | `regexp-semantics/lex-error` |
| `$262 is not defined` / harness gaps | 136 | already `runner-deferred`; investigate why falling into uncategorized |
| `parse: unexpected token` / `parse: expected ...` | ~1500 | cruft parser failures on test source; `availability/missing-parser-feature` or `parser-form/early-error :: SyntaxError` |
| `Cannot read property of null/undefined` | ~200 | cruft runtime error; `availability/missing-internal-slot` or similar |
| Other unmatched | ~1100 | per-batch further analysis |

### Rung 2 — Implement + re-categorize (PCR-EXT 2)

Add the rules to `projection_class` + `resolver_for`. Rebuild the categorizer binary. Re-run on the existing raw results (no test262 re-run needed; the categorizer ingests sidecar JSONL).

Output: refreshed `pilots/apparatus/test262-categorize/full-suite/results/<new-run-id>/{interpreted.jsonl, matrix.md, summary.md}`.

### Rung 3 — Measure shift + update positioning-gaps (PCR-EXT 3)

Compute the categorizer's improvement:

- New uncategorized count vs prior 3,681
- New top-30 matrix coordinates vs prior (some uncategorized may shift INTO existing top-ranks, increasing covered counts; some may surface NEW coordinates)
- LPA-EXT 3 positioning-gaps.md re-run against the refreshed matrix

## Carve-outs

- **Cruft parser failures on test source** (`parse: ...` reasons) are themselves a substrate signal that cruft's parser lacks features. Routing them under `availability/missing-parser-feature` is correct as a categorization, but the *substrate work* on those coordinates is parser-tier (e.g., the keeper's broader substrate roadmap). The categorizer's job is to name the coordinate; substrate locales target it.
- **Harness $262 gaps** routing fix is small but distinct from projection refinement. Audit why `harness_surface(reason)` test isn't catching all 136 instances and tighten.
- **Substrate-tier behavior change is out of scope.** This locale only edits the categorizer; the engine substrate is unmodified. No diff-prod / random-300 risk.

## Composes-with

- `pilots/apparatus/test262-categorize/` — the parent apparatus-pilot this locale refines.
- `pilots/apparatus/locale-positioning-audit/findings/positioning-gaps.md` (LPA-EXT 3) — the immediate trigger; this locale's success will manifest as a re-rendered positioning-gaps document with smaller Class A.
- `apparatus/docs/ecma-conformance-...md` §IX — the apparatus-gap discipline this locale enacts.
- The bilateral-pilot-tier articulation (`repository-apparatus.md` §0) — this locale sits cleanly in `pilots/apparatus/` as an apparatus-pilot per the discriminator (its primary output is consumed by other pilots' substrate work).

## R13 prospective C1-C4 at founding

- **C1 (sibling closure pattern)**: HOLDS — the categorizer already has well-shaped rule chains for the buckets it handles correctly; extending the rules is a known shape.
- **C2 (shape-compat with substrate APIs)**: HOLDS — the categorizer is a stateless function over reason-text strings; rule addition is purely additive.
- **C3 (cost-positive when integrated)**: TBV at PCR-EXT 2; expected positive (per-rule cost is one regex/string-match per record; near-zero) with high yield (3,681 records to reclassify).
- **C4 (bail safety)**: HOLDS — apparatus-only edit; no engine substrate change.

All four hold. Expect ≤3-round closure per R13 thirteenth-corroboration discipline.

## Resume protocol

Read `trajectory.md` tail.
