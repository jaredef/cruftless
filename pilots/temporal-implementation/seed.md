---
name: temporal-implementation
description: Parent substrate-program for implementing the ECMA-262 Temporal API (Temporal.Now + 8 plain/zoned/instant/duration classes). Multi-session Pin-Art track with nested sub-locales per class.
type: project
---

# temporal-implementation — Seed

## Parent substrate-program — Tier L (multi-rung, multi-session).

Spawned per keeper directive (Telegram 9873) to add a track for implementing Temporal. The RFSDO-EXT 2 deny-list currently SKIPs all Temporal tests (6,694 of them); this program incrementally implements the surface so the deny-list entry can be progressively removed.

## Telos

Implement the ECMA-262 Temporal API substrate sufficient to pass meaningful subsets of test262's `intl402/Temporal/*` suite (2,028 tests across 9 classes). Final state: cruft has Temporal.Now + Temporal.{PlainDate, PlainTime, PlainDateTime, PlainMonthDay, PlainYearMonth, Duration, Instant, ZonedDateTime} with ISO calendar + UTC time-zone baseline support, and the RFSDO-EXT 2 Temporal entry is removed.

## Surface inventory (2,028 tests)

| Class | Tests | Notes |
|---|---:|---|
| Temporal.Now | 3 | Smallest viable — host clock + plainDateTimeISO/zonedDateTimeISO factories |
| Temporal.PlainTime | 12 | Wall-clock time without date — small, no calendar |
| Temporal.Instant | 17 | Absolute timestamp (epochNanoseconds) — no calendar |
| Temporal.Duration | 21 | Difference between times — pure arithmetic |
| Temporal.PlainMonthDay | 90 | Calendar-dependent (month + day, no year) |
| Temporal.PlainYearMonth | 327 | Calendar-dependent (year + month) |
| Temporal.PlainDateTime | 483 | Date + wall-clock time |
| Temporal.PlainDate | 493 | Calendar date with arithmetic |
| Temporal.ZonedDateTime | 582 | Date + time + IANA time zone |

## Methodology — incremental Pin-Art per class

Each Temporal class is a nested sub-locale under `pilots/temporal-implementation/<class>/`. Each sub-locale follows the standard locale shape (seed + trajectory) and lands one substrate move per rung.

### Rung sequence (recommended)

1. **temporal-foundation** (pending spawn) — global Temporal namespace, ISO calendar registry, internal slot machinery. No user-visible classes; substrate for what follows.
2. **temporal-now** — Temporal.Now.{plainDateTimeISO, zonedDateTimeISO, instant}. 3 tests; smallest viable; validates the namespace + global registration + host-clock plumbing.
3. **temporal-instant** — Temporal.Instant ctor + arithmetic + comparison. 17 tests; pure epochNanoseconds.
4. **temporal-plain-time** — wall-clock time. 12 tests; no calendar.
5. **temporal-duration** — pure arithmetic. 21 tests.
6. **temporal-plain-date** — ISO calendar arithmetic. 493 tests; first big surface.
7. **temporal-plain-date-time** — date + time composition. 483 tests.
8. **temporal-plain-month-day** + **temporal-plain-year-month** — calendar-partial. 90 + 327 tests.
9. **temporal-zoned-date-time** — IANA time-zone integration. 582 tests; requires TZ database. LAST.

Each rung's exit criterion: ≥80% of its sub-suite passes, OR keeper accepts a deferred-residual list.

### RFSDO synchronization

When a sub-class lands meaningfully (≥50% of its sub-suite passes), remove its Temporal-related flag entries from the RFSDO deny-list so the matrix shows true engine state. Stale deny-list entries silently mask passing tests.

Specifically: once `temporal-now` lands, the `Temporal` flag stays (still blocks the 9 classes' surface tests). But once a critical mass of class-coverage exists, split the flag into per-class entries OR remove entirely and let real failures surface.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` — namespace + class registration.
- New: `pilots/rusty-js-runtime/derived/src/temporal/` (module) — Temporal-specific runtime types (Instant, PlainDate, PlainTime, etc.).
- **Exemplar suite**: `pilots/temporal-implementation/exemplars/exemplars.txt` — 2,028 fixtures.
- Each nested sub-locale carries its own exemplar subset.

## Baseline (FOUNDING)

All 2,028 tests currently SKIP via RFSDO-EXT 2's Temporal deny-list. Pre-RFSDO baseline: all 2,028 FAIL with `Temporal is not defined`.

## R13 prospective C1-C4

- C1 (sibling): WEAK — cruft has Intl partial implementation as a precedent for ECMA-402 surfaces, but Temporal's depth (calendar arithmetic, TZ database, BigInt-backed nanoseconds) has no direct sibling in cruft. Expect substrate-introduction cost per Rule 13.
- C2 (shape-compat): HOLDS — Temporal is a leaf namespace with no entanglement to existing JS semantics.
- C3 (cost-positive): TBV per rung. Foundation + Now will validate.
- C4 (bail-safe): HOLDS — incremental class-by-class; each rung gated by its own exemplar suite.

Rule 13's prospective application requires C1; this program will need substrate-introduction rungs (Foundation + first class) before prospective application becomes useful for later rungs.

## Carve-outs

- **No Intl integration in v1.** Temporal interacts with Intl.DateTimeFormat (e.g., `Temporal.PlainDate.prototype.toLocaleString`). v1 deliberately omits the Intl integration; toLocaleString returns the ISO string until Intl-coordinated work happens.
- **No IANA TZ database in v1.** Only UTC and named-offset zones ("Z", "+05:30") supported until temporal-zoned-date-time lands the TZ database.
- **No custom calendars in v1.** Only ISO 8601 calendar (`"iso8601"`) supported until temporal-calendar-extensions lands. Tests with `features: [Intl.Era-monthcode]` will continue to SKIP via the existing exclusion.

## Composes-with

- `pilots/apparatus/runner-features-skip-deliberate-omissions/` — Temporal flag in RFSDO-EXT 2's deny-list synchronizes with this program's progress.
- Future: per-class sub-locales nested at `pilots/temporal-implementation/<class>/`.

## Resume protocol

Read trajectory tail. Pick the next rung from the sequence above. Each rung is its own multi-session substrate sub-program.

## Status

TI-EXT 0 FOUNDED 2026-05-26. Parent program articulated. First sub-locale `temporal-now/` spawned (smallest viable). Implementation deferred — keeper directive was "add a track," not "begin work." Next move: temporal-foundation (namespace + class scaffolding) followed by temporal-now.
