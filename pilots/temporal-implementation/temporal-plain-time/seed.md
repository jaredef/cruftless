---
name: temporal-plain-time
description: Parent locale for Temporal.PlainTime substrate work. Wall-clock time (hour/minute/second/ms/μs/ns) with no calendar/TZ entanglement.
type: project
---

# temporal-plain-time — Seed

## Parent locale under `pilots/temporal-implementation/`.

Third per-class parent in the Temporal program. Surface: ~505 tests (493 built-ins + 12 intl402).

## Sub-rung topology

| Rung | Sub-locale | Scope | Status |
|---|---|---|---|
| 1 | `plain-time-ctor-fields/` | ctor + 6 getters + valueOf-throws + range validation | LANDED |
| 2 | `plain-time-static/` | from / compare | NOT SPAWNED |
| 3 | `plain-time-arithmetic/` | add / subtract / round / since / until / equals | NOT SPAWNED |
| 4 | `plain-time-string-conversion/` | toString / toJSON / toLocaleString | NOT SPAWNED |
| 5 | `plain-time-with/` | with method | NOT SPAWNED |

## Status

TPT-EXT 0 FOUNDED 2026-05-26. First sub-rung (`plain-time-ctor-fields/`) LANDED 2026-05-26 with 32/34 PASS (94%).
