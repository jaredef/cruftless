---
name: date-month-convention-fix
description: Fix a latent month-indexing bug in cruft's ymd_to_ms helper that causes new Date(Y, 0..1, D) and Date(Y, 2..11, D) to land on the wrong month.
type: project
---

# date-month-convention-fix — Seed

## Substrate-pilot — discovered as a side-finding during IDTP-EXT 1.

Spawned per keeper directive (Telegram 9899) following IDTP-EXT 1's discovery that `cruft`'s `ymd_to_ms` has a month-convention bug. The bug surfaced when iso-datetime-parse needed correct epoch arithmetic; before that, all Date callers compensated through relative semantics (Date+offset, Date.getTime() differences) so the off-by-N-day error was invisible.

## The bug

`pilots/rusty-js-runtime/derived/src/intrinsics.rs::ymd_to_ms` at line 9932:

```rust
pub(crate) fn ymd_to_ms(year: i64, month: i64, day: i64) -> i64 {
    let y = if month < 2 { year - 1 } else { year };
    let m = if month < 2 {
        (month + 9) as i64
    } else {
        (month - 2) as i64
    };
    // ... Howard Hinnant chrono algorithm
}
```

The conditional `month < 2` + branch `month - 2` mis-encodes the standard Howard Hinnant chrono shift, which should be `m <= 2` + `m - 3` for a 1-indexed input or `m < 2` + `m - 2` for a 0-indexed input.

**Empirical**: `new Date(1970, 0, 1).getTime()` returns `-2678400000` (31 days before epoch, i.e., Dec 1 1969) instead of `0` (the spec'd UTC midnight Jan 1 1970, modulo local TZ which cruft treats as UTC throughout).

Worse, `ymd_to_ms` skips February when month ≥ 2:
- `new Date(1970, 2, 1).getTime()` returns Mar 1 1970, not Feb 1.
- `new Date(1970, 1, 1).getTime()` returns Jan 1 1970 (treats month=1 as Jan).

So the function effectively defines: month=0 → Dec-prior-year, month=1 → Jan, month=2 → Mar, month=3 → Apr, ..., skipping February for inputs ≥ 2.

## Telos

Replace `ymd_to_ms`'s month-handling with the correct Howard Hinnant convention. Decide between two equivalent fixes:

1. **Keep 0-indexed input** (spec-aligned JS Date convention) — change `month < 2` to `month < 2` (already) AND change `month - 2` to `month - 1` (which collapses to `m = month - 2` shifted... actually rework the algorithm).
2. **Switch to 1-indexed input + update all callers** — touching every `ymd_to_ms` call site in the Date constructor and Date setters.

Per spec ECMA-262 §21.4.1.5 MakeDay, month is 0-indexed (Jan=0). v1 should preserve this and fix the algorithm in place.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::ymd_to_ms` (line 9932) — fix the algorithm.
- Caller sites: Date ctor (line ~5147), Date set* methods (line ~5155 + setUTC* etc).
- **Exemplar suite**: `pilots/date-month-convention-fix/exemplars/exemplars.txt` — Date-ctor-correctness tests from `built-ins/Date/*` (~594 Date tests total; subset that exercises specific year-month combinations).

## Discovery context

IDTP-EXT 1 needed correct epoch math for `Temporal.Instant.from(ISO datetime)`. First cut used `ymd_to_ms(year, month - 1, day)` (treating ISO month as 1-indexed input, helper as 0-indexed). Result: `1970-01-01T00:00:00Z` → -31 days; `1975-02-02` → Mar 2 1975. IDTP wrote an inline correct `days_from_civil` to bypass the bug.

The IDTP workaround is fine for Temporal's needs but the Date side remains buggy. The Date ctor will still return wrong values for any month-input where the bug manifests; consumer code that does relative arithmetic compensates, but absolute-epoch consumers (anything that compares Date.getTime() to a known fixed epoch value) gets wrong results.

## R13 prospective C1-C4

- C1 (sibling): HOLDS — IDTP's `days_from_civil` is the known-good algorithm to import.
- C2 (shape-compat): HOLDS — `ymd_to_ms` signature stays the same; only the body changes.
- C3 (cost-positive): HOLDS — one-function fix, expected widespread Date-test yield improvement.
- C4 (bail-safe): TBV — some existing code may have compensated for the bug; need cross-locale sweep before declaring done.

## Methodology

### DMCF-EXT 1 — fix ymd_to_ms (pending)

Replace the buggy `month < 2` + `month - 2` with the correct convention for 0-indexed JS Date month:

```rust
pub(crate) fn ymd_to_ms(year: i64, month: i64, day: i64) -> i64 {
    // month is 0-indexed (Jan=0). Convert to Howard Hinnant frame
    // where the shifted month m is in [0, 11] with March=0.
    let y = if month < 2 { year - 1 } else { year };
    let m = if month < 2 { month + 10 } else { month - 2 };  // <-- +10, not +9
    // (rest unchanged)
}
```

(Verify the constants by testing against known epoch values; the +10 vs +9 distinction is the load-bearing fix.)

### DMCF-EXT 2 — cross-locale regression sweep (pending)

After DMCF-EXT 1, run:
- diff-prod (some Date fixtures live there)
- test262 sample for built-ins/Date
- All tokenization locales (none should depend on Date semantics, but verify)

Any test that was passing via bug-compensation will newly fail; document and decide per-test whether to fix the test or revert+rework the algorithm.

## Composes-with

- `pilots/temporal-implementation/temporal-iso-string-parse/iso-datetime-parse/` (IDTP) — discovered the bug; uses its own inline `days_from_civil` to avoid.
- Future Temporal sub-rungs (PlainDate, etc.) — also need correct epoch math; will benefit from the fix.

## Status

DMCF-EXT 1 LANDED 2026-05-26. One-character fix: `month + 9` → `month + 10` in the `month < 2` branch. Zero regression across tokenization locales + Temporal sub-rungs; instant-arithmetic +4 sibling yield. Date 100-sample at 61% PASS. DMCF-EXT 2 (full Date test262 baseline + targeted regression sweep) deferred.

---

## Cross-arc disposition (2026-05-28)

Per coverage-gap-orphan-disposition-2026-05-28.md §II.8: closed singleton (LANDED at IDTP-EXT 1; +4 sibling yield). Lattice-meet between the `2026-05-26-temporal-implementation` arc (Date math shared with Temporal.Instant) and the future `2026-05-28-annex-b-language-partition` arc (Date legacy methods per Annex B §B.2.3). Retroactive enrollment in the Annex B arc when scaffolded.
