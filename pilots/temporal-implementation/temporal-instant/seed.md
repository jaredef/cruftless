---
name: temporal-instant
description: Parent locale for Temporal.Instant substrate work. Implements the absolute-timestamp class — epochNanoseconds-keyed BigInt + derived epochMilliseconds.
type: project
---

# temporal-instant — Seed

## Parent locale under `pilots/temporal-implementation/`.

Second per-class parent in the Temporal program. Surface: ~482 tests (465 built-ins + 17 intl402).

## Sub-rung topology

| Rung | Sub-locale | Scope | Status |
|---|---|---|---|
| 1 | `instant-ctor-fields/` | ctor + epochNanoseconds + epochMilliseconds + valueOf-throws | LANDED |
| 2 | `instant-static/` | from / fromEpochMilliseconds / fromEpochNanoseconds / compare | NOT SPAWNED |
| 3 | `instant-arithmetic/` | add / subtract / since / until / equals / round | NOT SPAWNED |
| 4 | `instant-string-conversion/` | toString / toJSON / toLocaleString | NOT SPAWNED |
| 5 | `instant-zoned-conversion/` | toZonedDateTimeISO (needs temporal-tz-string-parse) | NOT SPAWNED |

## Composes-with

- Parent `temporal-implementation/`.
- Future: `temporal-bigint-nanoseconds` shared sub-substrate (will land before instant-arithmetic).
- Future: `temporal-iso-string-parse` (needed by instant-string-conversion + from(string)).

## Status

TInst-EXT 0 FOUNDED 2026-05-26. First sub-rung (`instant-ctor-fields/`) LANDED 2026-05-26 with 21/25 PASS (84%) on its exemplar surface.
