---
name: temporal-iso-string-parse
description: Parent locale for shared ISO 8601 parsing sub-substrates. Provides duration-string parsing (P[nY][nM][nW][nD][T[nH][nM][nS]]) and datetime-string parsing (YYYY-MM-DDTHH:MM:SS[.frac][Z|±HH:MM][TZ-annotation]).
type: project
---

# temporal-iso-string-parse — Seed

## Shared sub-substrate parent under `pilots/temporal-implementation/`.

Per parent program seed's "shared sub-substrate" plan. Not a per-class parent — its outputs are consumed by every per-class from(string) + Duration.from / Instant.from / future plain-X.from sub-rungs.

## Telos

Provide two ISO 8601 grammar parsers consumed by per-class entry points:

1. **iso-duration-parse** — `P[nY][nM][nW][nD][T[nH][nM][nS]]` per ECMA-262 §11.8.1. Used by Temporal.Duration.from + Temporal.Duration.compare(string).
2. **iso-datetime-parse** — `YYYY-MM-DDTHH:MM:SS[.frac][Z|±HH:MM][TZ-annotation]` per §11.8.2. Used by Temporal.Instant.from + Temporal.PlainDate.from + Temporal.PlainDateTime.from + every other per-class string parse.

## Sub-rung topology

| Rung | Sub-locale | Scope | Status |
|---|---|---|---|
| 1 | `iso-duration-parse/` | parse_iso_duration helper + Duration.from/compare string-arg wiring | LANDED |
| 2 | `iso-datetime-parse/` | parse_iso_datetime helper + Instant.from/compare wiring + foundation for plain-X classes | NOT SPAWNED |
| 3 | `iso-fractional-propagation/` | extend duration parser to propagate fractional H/M into smaller units per spec | NOT SPAWNED |
| 4 | `iso-tz-annotation-parse/` | bracketed IANA TZ annotation extraction (Now needs this) | NOT SPAWNED |

## Composes-with

- Parent `temporal-implementation/`.
- Sibling `temporal-duration/`, `temporal-instant/` (consumes outputs).

## Status

TISP-EXT 0 FOUNDED 2026-05-26. First sub-rung (`iso-duration-parse/`) LANDED 2026-05-26 with +5 sibling yield (DStat +4, DDP +1).
