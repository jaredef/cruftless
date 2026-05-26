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
