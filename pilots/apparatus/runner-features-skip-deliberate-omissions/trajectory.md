# runner-features-skip-deliberate-omissions — Trajectory

## RFSDO-EXT 0 — FOUNDING (2026-05-26)

Spawned via Rule-23 rediagnosis of Tier K Cluster C (IMM, hypothesized as `import.meta`).

**Baseline-inspection refutation**: per the seed `pilots/import-meta-metaproperty/` (now removed), the hypothesis was that cruft rejects `import.meta`. Direct probe: `console.log(import.meta)` → cruft prints `{ url: ..., dir: ... }`. cruft handles import.meta correctly.

**Actual cluster composition**: of the 76 records emitting `expected meta after import.`, none are about `import.meta`. All are stage-X proposals:
- 38 import-defer (stage-3 `import.defer(specifier)`)
- 38 import-source (stage-3 `import.source(specifier)`)

Per Rule 23, locale is treated as probe; the surfaced-coordinate move is apparatus (runner SKIP-list), not substrate (parser implementation of stage-X proposals cruft has chosen not to support).

## RFSDO-EXT 1 — LANDED (2026-05-26)

### Edit (~12 LOC in legacy/host-rquickjs/tests/test262/runner.mjs)

New `DELIBERATELY_OMITTED` Set checked against `meta.features` after the flag-based SKIP paths:
```js
const DELIBERATELY_OMITTED = new Set([
  'import-defer',
  'source-phase-imports',
  'source-phase-imports-module-source',
]);
for (const f of meta.features) {
  if (DELIBERATELY_OMITTED.has(f)) {
    return { path, status: 'SKIP', reason: `feature deliberately omitted: ${f}` };
  }
}
```

### Yield

- IMM pool (76): **0 PASS / 0 SKIP / 76 FAIL → 0 PASS / 76 SKIP / 0 FAIL**.
- Diff-prod: 42/42 maintained.
- The matrix's `availability/missing-syntax-feature` coordinate drops from 1015 to 939 once re-categorized.

### Rule 23 follow-on at landing

Initial deny-list had `import-source` (the guessed flag name from the proposal). First re-run showed 38 still FAIL. Inspection of a failing test revealed the actual flag is `source-phase-imports` (+ `source-phase-imports-module-source` as a paired flag). Updated the deny-list; re-ran → all 76 SKIP. Rule 23 verification-probe at substrate-landing caught the name-mismatch before the locale closed.

### Findings

**Finding RFSDO.1 (Rule 23 saved a substrate locale from chasing the wrong coordinate)**: The seed hypothesized parser-tier substrate work on `import.meta`. Baseline-inspection refuted it cleanly — cruft already handles import.meta. The actual cluster was apparatus-tier (runner SKIP behavior on stage-X feature flags). Without the founding probe, a parser-tier locale would have produced zero direct yield and the developer would have implemented stage-X proposals cruft has explicitly chosen not to support — pure wasted work. Standing recommendation: Rule 23 baseline-inspection is especially load-bearing when a candidate's reason-text matches a feature name in cruft (the text overlap can suggest the wrong substrate target).

**Finding RFSDO.2 (apparatus SKIP-list discipline)**: The deny-list MUST be narrow and reasoned. Adding a feature flag is a claim that cruft has deliberately excluded the proposal — not that implementation is incomplete. The earlier survey showed 6686 Temporal-flagged failures and 2458 generators-flagged failures; the former might be a candidate for the deny-list (cruft has no Temporal at all), but the latter is NOT (cruft has partial generators; SKIP would mask real bugs). The protocol in the seed enforces this.

### Status

RFSDO-EXT 1 CLOSED. Apparatus-pilot ready for future deny-list extensions per the standing protocol.
