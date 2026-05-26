---
name: runner-features-skip-deliberate-omissions
description: Test262 runner extension — SKIP tests whose required `features:` includes a stage-X / non-standard proposal that cruft has deliberately omitted.
type: project
---

# runner-features-skip-deliberate-omissions — Seed

## Apparatus-pilot — narrow SKIP-list for deliberately-omitted features.

Spawned per keeper directive (Telegram 9857) "continue to next." Rule 23 baseline-inspection of Tier K Cluster C (IMM, hypothesized as `import.meta`) refuted the seed: cruft already handles `import.meta` correctly. The 76 records are entirely stage-X proposals (`import.defer`, `import.source`) that cruft has chosen not to implement.

The right move is apparatus, not substrate: extend the runner to SKIP tests whose required features cruft has deliberately excluded — so the matrix reflects intent (these are not failures of the engine, they are inapplicable tests).

## Telos

Test262 runner emits SKIP instead of FAIL for tests whose `meta.features` includes any feature in a narrow, intentional deny-list. Sibling to the existing `flags.module / async / raw` SKIP paths.

**Critical scope discipline**: the deny-list lives ONLY in this locale's record and the runner code. Features are added ONLY when cruft has DELIBERATELY excluded the proposal (not when implementation is incomplete or partial). Otherwise the apparatus would mask real engine bugs as SKIP.

## Apparatus

- `legacy/host-rquickjs/tests/test262/runner.mjs` — new `DELIBERATELY_OMITTED` Set, checked against `meta.features` after the existing flag-based SKIP paths.
- **Exemplar suite**: `pilots/apparatus/runner-features-skip-deliberate-omissions/exemplars/exemplars.txt` — 76 fixtures (the original IMM matrix coordinate).

## Deny-list (RFSDO-EXT 1, LANDED 2026-05-26)

| Feature flag | Proposal | Rationale |
|---|---|---|
| `import-defer` | Stage-3 deferred dynamic import (`import.defer(...)`) | cruft does not implement |
| `source-phase-imports` | Stage-3 source-phase import (`import.source(...)`) | cruft does not implement |
| `source-phase-imports-module-source` | Sibling flag for source-phase | same proposal |

## Methodology

### RFSDO-EXT 1 — initial deny-list (LANDED)

Added the three flags above. ~12 LOC in runner.mjs.

**Yield**: IMM pool 0 → 76 SKIP (was 76 FAIL). The matrix surface for "missing-syntax-feature" drops by 76 records (1015 → 939) — cleaner signal.

Diff-prod: 42/42 maintained (apparatus-only edit, no engine change).

### Standing protocol

Before adding a feature flag to the deny-list:
1. Verify cruft has DELIBERATELY excluded the proposal (e.g., it's stage-2/3 and not on the implementation roadmap).
2. Verify the only failure-shape from tests gated by that feature is parser-level rejection of the syntax (not partial-runtime support).
3. Add the flag with a one-line rationale in the table above.
4. Re-categorize and confirm the records move from FAIL to SKIP without affecting any unrelated test.

If cruft later decides to implement the proposal, remove the flag from the deny-list as part of the implementation change.

## Composes-with

- `pilots/apparatus/tokenizer-error-classification-refinement/` — TECR's missing-syntax-feature coordinate gets cleaner once these inapplicable records SKIP.
- `apparatus/locales/CANDIDATES.md` Tier K — Cluster C (IMM) was redirected here after Rule 23 baseline-inspection.

## R13 prospective C1-C4

- C1 (sibling): HOLDS — existing `flags.module / async / raw` SKIP paths.
- C2 (shape-compat): HOLDS — additive Set + loop.
- C3 (cost-positive): HOLDS — O(features) per test.
- C4 (bail-safe): HOLDS — runner-only edit; engine unchanged.

## Status

RFSDO-EXT 1 LANDED. 76 records moved from FAIL to SKIP. Standing deny-list maintained here; extensions require the protocol above.
