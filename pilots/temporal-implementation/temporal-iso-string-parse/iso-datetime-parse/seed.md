---
name: iso-datetime-parse
description: Second sub-rung of temporal-iso-string-parse. Hand-written parser for ECMA-262 §11.8.2 ISO 8601 datetime strings; computes epochNanoseconds.
type: project
---

# iso-datetime-parse — Seed

## Leaf shared-substrate sub-locale under `pilots/temporal-implementation/temporal-iso-string-parse/`.

Consumed by Temporal.Instant.from(string), Temporal.Instant.compare(string), and future PlainDate.from / PlainDateTime.from / ZonedDateTime.from string entry points.

## Telos

`parse_iso_datetime(s: &str) -> Option<(i64, i64)>` returns `(epoch_sec_int_part, fractional_ns)` on success, None on parse failure.

Grammar accepted:
```
YYYY-MM-DD ['T'|'t'|' '] HH:MM[:SS[.fff]] (Z|z|±HH:MM[:SS]) ['[' annotation ']' ...]
```

- Date in extended ISO form (with `-` separators).
- Time separator: T, t, or space.
- Time: HH:MM minimum; HH:MM:SS optional; fractional (.fff or ,fff up to 9 digits) optional.
- Offset: `Z` (UTC) or `±HH:MM` (with optional `±HHMM` compact form or `±HH:MM:SS`).
- Bracketed annotations (IANA TZ, [u-ca=cal], etc.) accepted and IGNORED for Instant per spec (timezone-custom.js test).

## Constraints DEFERRED

- **Date without time** (e.g. `2026-01-01` alone) — Instant requires full datetime + offset per spec; PlainDate may accept just date (separate rung).
- **Compact basic-form dates** (`20260101T000000Z`) — extended form only for v1.
- **Calendar conversion** in annotations — `[u-ca=hebrew]` ignored.
- **Real IANA TZ resolution** — annotation accepted but not used; the explicit offset dominates.
- **Negative-zero offset detection** — spec rejects `-00:00` per RFC 3339; accepted here.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `parse_iso_datetime` inline fn + correct `days_from_civil` helper (cruft's existing `ymd_to_ms` has a latent month-convention bug that skips February for month >= 2; using inline correct algorithm avoids it).
- Caller sites: Instant.from(string), Instant.compare(string).
- **Exemplar suite**: none (measured via sibling Instant rung deltas).

## Status

IDTP-EXT 1 LANDED 2026-05-26. +25 sibling yield (instant-static 28→53). Latent ymd_to_ms bug avoided via inline Howard Hinnant chrono algorithm.
