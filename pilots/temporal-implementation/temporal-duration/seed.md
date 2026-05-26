---
name: temporal-duration
description: Parent locale for Temporal.Duration substrate work. Nested sub-rungs implement ctor + getters → derived properties → string conversion → arithmetic → relativeTo → static methods.
type: project
---

# temporal-duration — Seed

## Parent locale under `pilots/temporal-implementation/`.

First per-class parent in the Temporal program. Surface: ~559 tests (538 built-ins + 21 intl402).

## Sub-rung topology

| Rung | Sub-locale | Scope | Status |
|---|---|---|---|
| 1 | `duration-ctor-fields/` | ctor + 10 unit getters + valueOf-throws + ctor.prototype | LANDED |
| 2 | `duration-derived-properties/` | sign / blank / abs / negated | NOT SPAWNED |
| 3 | `duration-string-conversion/` | toString / toJSON / toLocaleString | NOT SPAWNED |
| 4 | `duration-static/` | Temporal.Duration.from / compare | NOT SPAWNED |
| 5 | `duration-with/` | Duration.prototype.with (immutable update) | NOT SPAWNED |
| 6 | `duration-arithmetic/` | add / subtract (without relativeTo) | NOT SPAWNED |
| 7 | `duration-rounding/` | round / total (without relativeTo) | NOT SPAWNED |
| 8 | `duration-relative-to/` | relativeTo with calendar/TZ | NOT SPAWNED |

## Composes-with

- Parent `pilots/temporal-implementation/` — articulates the program.
- Sibling `temporal-foundation/` — established the namespace + stub Duration was overwritten by duration-ctor-fields.
- Future shared sub-substrates: temporal-iso-string-parse (duration-string-conversion needs it), temporal-iso-calendar + temporal-tz-string-parse (duration-relative-to needs them).

## Status

TDur-EXT 0 FOUNDED 2026-05-26. First sub-rung (`duration-ctor-fields/`) LANDED 2026-05-26 with 64/67 PASS (95.5%) on its exemplar surface. RFSDO PARTIALLY_IMPLEMENTED carve-out (RFSDO-EXT 3) added 30+ Duration test paths to the Temporal allowlist.
