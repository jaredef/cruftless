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

## Surface inventory (full)

The intl402/Temporal counts I cited at founding were the intl-coordinated subset only. The TRUE Temporal surface is much larger:

| Class | built-ins | intl402 | staging | Total |
|---|---:|---:|---:|---:|
| Now | 66 | 3 | — | ~69 |
| PlainTime | 493 | 12 | — | ~505 |
| Instant | 465 | 17 | — | ~482 |
| Duration | 538 | 21 | — | ~559 |
| PlainMonthDay | 199 | 90 | — | ~289 |
| PlainYearMonth | 507 | 327 | — | ~834 |
| PlainDateTime | 771 | 483 | — | ~1254 |
| PlainDate | 650 | 493 | — | ~1143 |
| ZonedDateTime | 899 | 582 | — | ~1481 |
| top-level + toStringTag | 5 | — | — | 5 |
| **TOTAL** | **4,593** | **2,028** | **~50** | **~6,700** |

This matches the 6,694 records RFSDO-EXT 2 SKIPs.

## Methodology — incremental Pin-Art per class, with per-class nesting

**Scope correction (2026-05-26, post-foundation)**: the founding seed assumed each class fits a single-locale rung. Surveying the actual surface (538-1481 tests per class) and substrate cost (Duration alone is ~200-400 LOC for ctor + getters; ~1000+ LOC for full coverage), this is wrong. Per Pin-Art Doc 737 (locales spawn nested sub-locales when work has multi-rung shape), each Temporal class is itself a PARENT locale with its own nested rungs.

Restructured rung topology:

```
temporal-implementation/                         ← parent program
├── temporal-foundation/                         ← namespace + class stubs (LANDED)
├── temporal-duration/                           ← parent for Duration
│   ├── duration-ctor-fields/                    ← ctor + 10 getters + valueOf-throws
│   ├── duration-derived-properties/             ← sign / blank / abs / negated
│   ├── duration-string-conversion/              ← toString / toJSON / toLocaleString
│   ├── duration-arithmetic/                     ← add / subtract / round (without relativeTo)
│   ├── duration-relative-to/                    ← relativeTo with calendar/TZ — last
│   └── duration-static/                         ← from / compare
├── temporal-instant/                            ← parent for Instant
│   ├── instant-ctor-fields/
│   ├── instant-arithmetic/
│   ├── instant-string-conversion/
│   ├── instant-static/
│   └── instant-from-temporal-from/
├── temporal-plain-time/                         ← parent for PlainTime
│   └── ... (similar nesting)
├── temporal-plain-date/                         ← parent
├── temporal-plain-date-time/                    ← parent
├── temporal-plain-month-day/                    ← parent
├── temporal-plain-year-month/                   ← parent
├── temporal-zoned-date-time/                    ← parent (LAST — needs TZ database)
├── temporal-now/                                ← parent (needs temporal-tz-string-parse)
└── shared sub-substrates:
    ├── temporal-iso-string-parse/               ← ISO-8601 datetime/duration parsing
    ├── temporal-tz-string-parse/                ← IANA TZ + offset parsing (Now needs this)
    ├── temporal-bigint-nanoseconds/             ← BigInt arithmetic on epochNanoseconds
    └── temporal-iso-calendar/                   ← ISO 8601 calendar arithmetic
```

### Per-rung scope budget

Each leaf rung's substrate must fit in ~50-150 LOC (one substrate move per the standing budget). Larger work spawns more rungs. Cost estimates:

| Leaf rung | LOC | Test yield est. |
|---|---:|---:|
| temporal-foundation | ~50 | 3 intro tests |
| duration-ctor-fields | ~120 | 30-60 tests (ctor variants + N-undefined + length + name) |
| duration-derived-properties | ~80 | 20-40 tests |
| duration-string-conversion | ~150 | 30-80 tests |
| duration-arithmetic | ~200 | 50-100 tests |
| duration-relative-to | ~300+ | 100-200 tests |
| duration-static | ~80 | 20-40 tests |

Total Duration coverage estimate: ~900-1100 LOC across 5-6 rungs / sessions. Compare with Date's ~1500 LOC and Promise's ~2000 LOC — same order of magnitude.

### Recommended execution order

1. **temporal-foundation** (LANDED) — namespace + class stubs.
2. **temporal-duration / duration-ctor-fields** — first per-class rung; lowest entanglement (no calendar, no TZ); single substrate move yields meaningful test count.
3. **temporal-instant / instant-ctor-fields** — same shape as Duration but with BigInt epochNanoseconds.
4. **temporal-plain-time / plain-time-ctor-fields** — wall-clock time, no calendar.
5. Then arithmetic / string-conversion sub-rungs of the above.
6. Then **temporal-iso-calendar** as shared substrate before PlainDate, PlainDateTime, PlainMonthDay, PlainYearMonth.
7. Then **temporal-tz-string-parse** + **temporal-now** + **temporal-zoned-date-time** (in that order — Now needs TZ-parse; ZonedDateTime needs full TZ database).

Each rung's exit criterion: substrate move lands + diff-prod 42/42 + reasonable test yield documented in trajectory. Sub-class parent locales declare "operational" only when ALL their nested rungs land OR keeper accepts a deferred-residual list.

### RFSDO synchronization

The single `Temporal` flag in RFSDO-EXT 2 is too coarse for the per-class-with-nesting topology. Sync protocol:

1. **Per-class RFSDO landing**: when a class's `ctor-fields` rung lands, ADD a per-class allow-pass to RFSDO via path-based carve-out (e.g., `built-ins/Temporal/Duration/constructor.js`, `built-ins/Temporal/Duration/years-undefined.js`, etc.) — opting those specific tests OUT of the `Temporal` skip so they run and reveal real engine state.
2. **Per-sub-class RFSDO removal**: when a class's coverage reaches ≥80% (sub-class operational), remove the per-class carve-outs and the class-name from any per-class deny entries.
3. **Final removal**: when 8 of 9 classes are operational, remove `Temporal` from RFSDO entirely.

The carve-out mechanism is a new RFSDO sub-protocol that runner-features-skip-deliberate-omissions will need to support. Sketch: in addition to the `DELIBERATELY_OMITTED` Set, add a `PARTIALLY_IMPLEMENTED` map of feature → path-prefix-allowlist. A test with a partially-implemented feature is SKIPped UNLESS its path matches an allowlist entry. This change lands when the first class's ctor rung does (TF.2 standing rec on stub-handshake applies).

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
