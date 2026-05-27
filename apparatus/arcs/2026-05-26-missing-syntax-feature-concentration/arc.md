---
arc: 2026-05-26-missing-syntax-feature-concentration
trigger: Telegram message 9851 ("Let's hone in on missing syntax features") chained from 9847 (TECR-EXT 2 lift) → 9853 ("Add these all to the CANDIDATES.md doc and then begin with A")
opened: 2026-05-26
closed: 2026-05-26
close_condition: All 5 candidate clusters (HDSB / WBMS / IMM / DIA / CAR) triaged — spawned + landed OR redirected to follow-on substrate.
---

# Tier-K Missing-Syntax-Feature Concentration Arc

## Trigger

After TECR-EXT 2 lifted the missing-syntax-feature class to its true count (1017 records), the keeper directed "hone in on missing syntax features." The arc surveyed the 17 distinct reason-shapes, found top-5 = 937 records (92%), and triaged them.

## Telos

Triage the 5 missing-syntax-feature clusters (HDSB 475, WBMS 264, IMM 76, DIA 41, CAR 44) — each candidate either lands as substrate, redirects to apparatus, or surfaces as a sibling locale's residual.

## Sub-locale roster

| Locale | Cluster | Role | Status | Yield |
|---|---|---|---|---|
| `hoistable-declaration-as-statement-body/` (HDSB) | A: 475 records | parser-tier substrate | LANDED | 150/475 (31.6%) |
| `with-body-multi-statement-parse/` (WBMS) | B: 264 records | parser-tier substrate | LANDED | 37/264 (14%) |
| `apparatus/runner-features-skip-deliberate-omissions/` (RFSDO) | C: 76 records (IMM rediagnosed) | apparatus-tier | LANDED | 76/76 SKIP |
| `dynamic-import-attributes/` (DIA) | D: 41 records | parser-tier substrate | LANDED | 40/41 (97.6%) |
| (CAR → WBMS-EXT 2 redirect) | E: 44 records | sibling-residual redirect | ★REDIRECTED | absorbed |

Follow-on candidates surfaced during the arc:
- `line-terminator-conformance/` (LTC, spawned same-day)
- `html-like-comment-lexing/` (HLCL, spawned same-day)
- `for-in-initializer-annex-b/` (FII, spawned same-day)
- `eval-scope-binding-chain/` (HDSB.2 standing-rec, registered Tier M)
- `with-runtime-semantics/` (WBMS-EXT 2, registered Tier M)

## Cumulative yield

| Checkpoint | Closed PASS | Closed SKIP | Notes |
|---|---:|---:|---|
| Before arc | 0 | 0 | — |
| + HDSB | 150 | 0 | parser carve-out |
| + WBMS | 187 | 0 | — |
| + IMM→RFSDO | 187 | 76 | apparatus redirect |
| + DIA | 227 | 76 | — |
| + CAR redirect | 227 | 76 | absorbed into WBMS-EXT 2 |
| Arc close | **227 PASS + 76 SKIP** | | — |

Plus same-day adjacent landings (LTC + HLCL + FII): another ~50 PASS in adjacent locales.

## Cross-locale findings

**Finding ARC-K.1 (cluster-coherence multiplier holds for 5/5 candidates)**: All 5 candidates were SINGLE-MECHANISM clusters (one parser rule, one runtime semantic, one apparatus protocol). Triage yielded clean closures for 3 (HDSB, RFSDO, DIA), partial closure for 1 (WBMS — parser only; runtime semantic deferred), and absorption-redirect for 1 (CAR → WBMS-EXT 2). Standing rec: concentrate-shaped clusters identified via top-N reason-shape analysis tend to be 1-mechanism per cluster.

**Finding ARC-K.2 (Rule 23 baseline-inspection saves substrate locales from chasing wrong coordinates)**: IMM cluster was hypothesized as "import.meta missing." Rule 23 probe showed cruft already handles import.meta correctly; the 76 records were entirely stage-X proposals (import.defer + source-phase-imports). Redirected to apparatus (RFSDO). Without the founding probe, a parser-tier substrate locale would have been spawned, executed, produced zero direct yield, and the developer would have implemented stage-X proposals cruft has explicitly chosen not to support.

**Finding ARC-K.3 (parser-only carve-outs close 30-50% of cluster; deeper-tier residuals warrant separate locales)**: HDSB parser carve-out closed 31.6%; WBMS parser fix closed 14%. The remainder in each case was lowering/runtime work warranting separate locales (HDSB-EXT 2 binding semantics; WBMS-EXT 2 with-runtime-semantics). Standing rec: when scoping a parser-tier locale, estimate direct-yield by the fraction of pool tests that probe parser-only vs runtime-semantic behavior.

## Status

CLOSED 2026-05-26. All 5 candidates triaged. Subsequent disambiguation work (post-Tier-K survey) ran as a separate operational unit.
