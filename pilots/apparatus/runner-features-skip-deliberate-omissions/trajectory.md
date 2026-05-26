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

## RFSDO-EXT 2 — Temporal + Atomics + DisposableStack family + ShadowRealm (2026-05-26)

### Trigger

Keeper directive (Telegram 9871) after the post-Tier-K landscape survey. The 7,532 "X is not defined @file://<eval:" records engagement-wide concentrated on a few large standard-but-deliberately-deferred subsystems:
- 6,162 Temporal
- 161 Atomics
- 93 + 71 DisposableStack / AsyncDisposableStack
- 57 ShadowRealm
- 18 SuppressedError

These are ratified ECMA-262 / ECMA-402 features, but cruft v1 deliberately defers them — keeper judgment per the standing RFSDO protocol ("a feature flag is added only when cruft has DELIBERATELY excluded the proposal").

### Mapping identifiers to feature flags

Survey across test262 source: each identifier maps unambiguously to one feature flag:
- `Temporal` → `Temporal`
- `Atomics` → `Atomics` (+ `Atomics.waitAsync` for the async waiter subset; + `SharedArrayBuffer` for SAB-dependent tests)
- `DisposableStack` / `AsyncDisposableStack` / `SuppressedError` / `using` → `explicit-resource-management`
- `ShadowRealm` → `ShadowRealm`

### Edit (~9 LOC added to DELIBERATELY_OMITTED Set)

Added: `Temporal`, `Atomics`, `Atomics.waitAsync`, `SharedArrayBuffer`, `explicit-resource-management`, `ShadowRealm`.

### Yield (against the 2026-05-25-full matrix)

| Feature flag | Records moved FAIL → SKIP |
|---|---:|
| Temporal | 6,694 |
| Atomics | 321 |
| explicit-resource-management | 302 |
| SharedArrayBuffer | 201 |
| ShadowRealm | 60 |
| **TOTAL NEW SKIPs** | **7,578** |

Matrix compression: ~14% of all engagement-wide FAIL records (7,578 of ~53,000) now correctly SKIP rather than masquerading as engine bugs. The dominant Temporal coordinate (6,694) was previously absorbing measurement attention across multiple downstream categorizer rules; sharpening it drains availability/missing-global-or-binding (was 7,033 → projected ~340 post-rerun).

Diff-prod: 42/42 maintained (apparatus-only edit, engine unchanged).

### Findings

**Finding RFSDO.3 (matrix sharpening from a single apparatus edit can drain multiple downstream coordinates)**: The 6,694 Temporal records were the dominant pool of `availability/missing-global-or-binding` (7,033 → ~340 projected). The categorizer's downstream coordinates inherit the apparatus's discrimination power; sharpening a feature-skip decision UPSTREAM compresses every downstream coordinate that was inheriting the noise. Standing recommendation: when surveying for next-substrate-work, sharpen apparatus deny-lists BEFORE picking from the matrix — otherwise prioritization decisions absorb the noise.

**Finding RFSDO.4 (keeper-judgment apparatus moves require explicit invocation)**: RFSDO-EXT 2's additions are STANDARD features (Temporal is in ECMA-262 since 2025). The standing protocol's "deliberately excluded" gate requires keeper judgment — these aren't stage-X proposals like RFSDO-EXT 1's import-defer. The substrate worker should NOT add ratified-standard features to the deny-list without explicit keeper authorization. Standing recommendation: distinguish ext-1-shape (stage-X auto-deny, no judgment needed) from ext-2-shape (ratified-standard deferral, keeper judgment required).

### Status

RFSDO-EXT 2 CLOSED. 7,578 records moved FAIL → SKIP. Apparatus matrix sharpened substantially; subsequent substrate prioritization works against the cleaner residual surface.
