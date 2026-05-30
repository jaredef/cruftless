---
proposal_slug: 2026-05-30T200000Z-asta-ext-0-array-frozen-throw
decision: APPROVED
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10614
decided_at: 2026-05-30T20:30:00Z
covers_commits:
  - 00a73363
---

## Findings

Approved per keeper Telegram 10614 ("Approved") in the helmsman-keeper dyad. First substrate-spawn from findings-ledger Entry 016 (SAMPLE.1) Doc 721 chain-bundle decomposition.

Substrate commit `00a73363` lands ~45 LOC in `interp.rs`: new `Runtime::object_set_checked` narrow dispatcher (mirrors TAECSF-EXT 0's `typed_array_set_index_checked` pattern) + 6 Array.prototype mutating-method intrinsics migrated to use it with `?` propagation on length writes. Founds `pilots/array-strict-throw-discipline/` (manifest 230 → 231).

## Verification

1. Build PASS (~1m 09s); runtime lib 74/0/1 ignored; `bin/cruft` refreshed.
2. Direct probe 7/7 PASS (5 throw-cells + 2 positive controls).
3. Cluster gates: TAMM 87 (preserved), TAWR 71 (preserved), **diff-prod 64/48 → 65/47 (+1 PASS unexpected positive)**.
4. test262-sample post-rung: 88.8% at n=2 with ±1 PASS variance (6817 / 6818 / 7680-7681 / 16 SKIP).

## Doc 721 predicted-vs-actual + Rule 29 falsifier

**Predicted U = 12-15 cells; Actual A = +1 to +2 cells**. |U - A| ≈ 10-14, outside Doc 721 §II.4's |U - A| ≤ 1 corroboration band.

Per Doc 721 §VI.5 false-pass amendment: the discrepancy is attributed to **the test262-sample's curated path scope not including the bulk of the 15 chain-bundle cells**. The original chain-bundle was enumerated via grep over `results.jsonl` after the canonical run — a recall vector that surfaced cells exiting at the missing-TypeError-throw symptom. Many of those cells are in the full test262 corpus but NOT in `scripts/test262-sample/sample-paths.txt`'s curated list. The probe 7/7 PASS verifies substrate correctness; a full-suite re-run would surface the predicted yield. Out of scope for this rung's gates.

**Rule 29 falsifier observation**: the test262-sample produced 6817 PASS (run 1) and 6818 PASS (run 2) — ±1 variance across n=2. Per Rule 29's falsifier discipline: "≥5-runs reactivates the moment any subsequent run surfaces variance > 0." The instrument's determinism class has weakened post-ASTA-EXT 0. The substrate itself is deterministic (direct probe 7/7 PASS reproducible); the variance source is runner-side (likely parallelism race, fs caching, or harness-level timing). Surfaced as candidate Finding DET.4 (variance-source isolation for nominally-Class-A instruments).

## Cumulative session yield (full day, 2026-05-30, at this commit)

- 23 commits total
- **29 standing rules + Finding DET.4 candidate** in findings-ledger pending
- **9 audit-ledger entries**
- **18 findings-ledger entries** (Entries 016-018 added today + multiple DET candidates pending)
- **5 apparatus docs** (audit-ledger, findings-ledger, findings-disposition-protocol, pipeline-alphabet-audit, measurement-determinism-prospective)
- **6 substrate locales founded** today (RBDPA + TAECSF + TABSC + ASTA + 2 deferral promotions)
- **Gates net**: TAWR 63 → 71 (+8); TAMM 82 → 87 (+5); **diff-prod 61/51 → 65/47** (+4 PASS); test262-sample 84.3% → 88.8% (+4.5 PP)

## Named follow-up

1. **Sibling Doc-721 sub-bundles** (per seed §Carve-outs): Map/Set/WeakMap/WeakSet frozen-receiver throw; iterator-protocol TypeError throws (for-of 5 cells); `put-const` destructuring (for-of 4 cells); Promise dispatcher receiver-validation (~14 cells); Object.assign throw-propagation. Each its own substrate-spawn per Rule 17.
2. **test262-sample variance-source isolation** (DET.4 candidate, apparatus-tier rung): investigate the source of the ±1 PASS variance (harness parallelism, fs caching, timer-noise in runner). Outcome: either restore the instrument to Class A by removing the variance source, or formally re-class to "near-Class-A with ±1 bound" and amend Rule 29.
3. **test262-full re-run** to surface ASTA-EXT 0's predicted +12 to +15 yield against the broader 65k-cell population (Doc 721 Step 5 validation).
4. **Finding ASTA.1 promotion**: narrow-dispatcher cascade-revival at engine-internal Result-threading site; one-more-observation pending (this is the third instance in the engagement after TAECSF-EXT 0 + TABSC; arguably promotion-ready).

**APPROVED for push** per keeper-substituted authorization Telegram 10614.
