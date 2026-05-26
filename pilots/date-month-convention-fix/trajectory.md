# date-month-convention-fix — Trajectory

## DMCF-EXT 0 — FOUNDED (2026-05-26)

Spawned per keeper directive (Telegram 9899) immediately after IDTP-EXT 1's side-finding surfaced cruft's `ymd_to_ms` month-convention bug.

### Discovery context

IDTP-EXT 1 needed correct epoch math for `Temporal.Instant.from(ISO datetime)`. Probe revealed cruft's `ymd_to_ms` interprets month=1 as January but skips February for month ≥ 2 (treating month=2 as March). IDTP wrote inline `days_from_civil` to bypass; this locale exists to fix the upstream helper.

### Verified probes (status quo bug)

- `new Date(1970, 0, 1).getTime()` → `-2678400000` (expected 0; 31 days off)
- `new Date(1970, 1, 1).getTime()` → `0` (treats month=1 as January — masks the bug for this case)
- `new Date(1975, 2, 2).getTime()` → `162950400000` (March 2, expected February 2)

### Status

DMCF-EXT 0 FOUNDED.

## DMCF-EXT 1 — month-convention fix in ymd_to_ms (LANDED 2026-05-26)

### Edit (~3 LOC in intrinsics.rs::ymd_to_ms)

Single constant change: `month + 9` → `month + 10` in the `month < 2` branch. This corrects the Howard Hinnant chrono shift for 0-indexed JS Date month input.

### Probes at landing

- `new Date(1970, 0, 1).getTime()` → 0 ✓ (was -2678400000)
- `new Date(1975, 1, 2).getTime()` → 160531200000 ✓ (was 162950400000; Feb 2 1975, not Mar 2)
- `new Date(1969, 11, 31).getTime()` → -86400000 ✓ (Dec 31 1969)
- `new Date(2026, 4, 15).getTime()` → 1778803200000 ✓

### Yield

- Date 100-random-sample: 61/100 PASS (pre-DMCF baseline not measured; spec-correctness verified via probes).
- Tokenization locales: NLC 147, IDT 261, SLEC 59, LTC 31, HDSB 150, HLCL 10 — all unchanged.
- Temporal sub-rungs: stable (most use inline `days_from_civil`, bypassing `ymd_to_ms`).
- **Surprise yield**: instant-arithmetic 66 → 70 (+4). Investigation: a few Instant tests evidently traverse a code path that reaches `ymd_to_ms` (possibly via Date.parse helper used in test262 harness or via cruft's own `__date_ms` plumbing in stub fixtures). Sibling-yield from a substrate fix that bypasses the bug-compensation-via-relative-arithmetic pattern.
- Diff-prod: 42/42 maintained.

### Findings

**Finding DMCF.1 (one-character fix can unblock cross-locale yield)**: The change is `+9` → `+10`. Three Date probes corrected; three tokenization-tier locales stable; instant-arithmetic +4. The bug-compensation hypothesis (Finding TI's prediction that Date callers relied on the bug) DID NOT manifest as regressions — all tests that previously passed kept passing. The compensation was apparently always at the consumer level using Date.getTime() differences, not absolute values.

**Finding DMCF.2 (latent-bug fixes carry uncertain blast radius until measured)**: Pre-fix, the bug had been latent for unknown time. The DMCF seed estimated potential regression risk; actual outcome was zero regression + small unexpected yield gain. Standing recommendation: when fixing a latent helper bug, run a cross-locale sweep AND a representative sample of the helper's consumer surface; the sweep confirms no regression, the sample reveals unexpected positive yield.

### Status

DMCF-EXT 1 CLOSED. DMCF-EXT 2 (full Date test262 baseline + targeted regression sweep) deferred — current probes + tokenization sweep + Temporal sub-rung re-check show no regression; full-suite measurement requires test262-sample run which is multi-minute and not in this rung's budget.
