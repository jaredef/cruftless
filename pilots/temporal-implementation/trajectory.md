# temporal-implementation — Trajectory

## TI-EXT 0 — FOUNDING (2026-05-26)

Spawned per keeper directive (Telegram 9873) immediately after RFSDO-EXT 2 landed the Temporal SKIP entry. Parent multi-rung substrate program for implementing ECMA-262 Temporal API.

### State at founding

- 2,028 Temporal tests in `intl402/Temporal/*` — all SKIP via RFSDO-EXT 2.
- 9 classes: Now (3), PlainTime (12), Instant (17), Duration (21), PlainMonthDay (90), PlainYearMonth (327), PlainDateTime (483), PlainDate (493), ZonedDateTime (582).
- Total surface inventory: ~50 classes + protocol-implementations + 200+ methods (per spec); test262 covers a comprehensive but not exhaustive subset.

### Rung sequence committed at founding

Foundation → Now → Instant → PlainTime → Duration → PlainDate → PlainDateTime → PlainMonthDay/YearMonth → ZonedDateTime.

Smallest-first ordering serves two purposes:
1. **Apparatus validation**: Temporal.Now (3 tests) tests the namespace registration + host-clock plumbing before any class is built.
2. **Calendar-free first**: Instant, PlainTime, Duration have no calendar dependence; PlainDate adds ISO calendar arithmetic; ZonedDateTime adds TZ database. Each rung introduces one new substrate concern.

### Spawned sub-locales

- `pilots/temporal-implementation/temporal-now/` — first nested sub-locale, 3-test surface.

### Findings

**Finding TI.1 (program-scale locales require parent + nested sub-locale structure)**: A 2,028-test, 9-class substrate program does not fit a single locale's seed+trajectory shape — the work is multi-session and needs intra-program coordination (which class is next, what's deferred, when to drop RFSDO entries). Pin-Art's nested-locale pattern (Doc 737) handles this: parent locale articulates the program, sub-locales are the unit of substrate work. Standing recommendation: when test262 surface concentration is >500 in a single namespace AND the namespace is a coherent ECMA-262 chapter, spawn parent-with-nested-subs rather than a flat locale.

**Finding TI.2 (RFSDO entries are reversible apparatus claims, not permanent decisions)**: RFSDO-EXT 2's Temporal entry was added 2026-05-26 morning as a deliberate-deferral claim. By afternoon, the keeper committed to implementing it. RFSDO entries should be removed (or split into per-class entries) as substrate progress invalidates the deferral. Standing recommendation: add a synchronization protocol to RFSDO's seed — when a deny-listed feature becomes the subject of a substrate program, mark it in CANDIDATES and gate removal on substrate-rung-yield.

### Status

TI-EXT 0 FOUNDED. Parent program articulated. First sub-locale spawned. Implementation pending — keeper directive was scaffolding-only ("add a track"). Resume point: spawn temporal-foundation (or skip directly to temporal-now if keeper wants to validate apparatus first).

## TI-EXT 1 — scope correction + per-class nesting (2026-05-26)

### Trigger

Keeper directive (Telegram 9879) after foundation landed and TDur-EXT 1 was scoped against the actual Duration surface (538 tests in built-ins/Temporal/Duration alone). The founding seed cited 21 tests for Duration based on intl402 count only — the true surface is ~27× larger per class.

### Correction

Founding rung-sequence assumed each class = one locale = one substrate move. Survey of substrate cost (Duration ctor + 10 getters + valueOf-throws is ~200 LOC by itself; full Duration is ~900-1100 LOC across 5-6 rungs) refutes that assumption.

Per Doc 737 (locales spawn nested sub-locales when work has multi-rung shape), each Temporal class is itself a parent locale with its own nested rungs. Restructured topology in seed.md.

### Sub-substrates surfaced

The restructure exposed four shared sub-substrates that multiple per-class parents need:
1. `temporal-iso-string-parse` — ISO 8601 datetime/duration string parsing.
2. `temporal-tz-string-parse` — IANA TZ + offset parsing (Now needs this).
3. `temporal-bigint-nanoseconds` — BigInt arithmetic on epochNanoseconds.
4. `temporal-iso-calendar` — ISO 8601 calendar arithmetic.

These spawn as siblings to the per-class parents and are required by multiple per-class rungs.

### RFSDO synchronization protocol formalized

Single `Temporal` flag is too coarse for per-class progressive landings. New sub-protocol: `PARTIALLY_IMPLEMENTED` map of feature → path-prefix-allowlist. A test with a partially-implemented feature is SKIPped UNLESS its path matches an allowlist entry. This change lands when the first class's ctor rung does.

### Findings

**Finding TI.3 (per-class Temporal work is itself multi-rung and warrants its own parent-locale)**: The Pin-Art nested-locale pattern (Doc 737) recurses. A namespace-level parent (temporal-implementation) is right; a per-class parent (temporal-duration) is also right because each class has 5-7 substrate sub-moves. Standing recommendation: when a planned rung's substrate exceeds ~150 LOC OR has 3+ distinct sub-mechanisms, spawn a nested parent locale rather than expanding the parent's rung sequence.

**Finding TI.4 (RFSDO needs path-level granularity for progressive substrate programs)**: Single feature flags work for binary deliberate-omission decisions (RFSDO-EXT 1 stage-X proposals). They don't scale to progressive-landing substrate programs where SOME tests pass and others don't yet. Standing recommendation: add `PARTIALLY_IMPLEMENTED` map to RFSDO (path-prefix allowlist per partially-implemented feature) when first per-class Temporal rung lands.

### Status

TI-EXT 1 LANDED 2026-05-26. Per-class nesting topology + RFSDO sync protocol articulated. Sub-substrate plan recorded. Next: spawn first per-class parent (`temporal-duration`) with first rung (`duration-ctor-fields`) — substrate execution deferred to next session per scope-budget.
