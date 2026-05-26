---
name: plain-date-static
description: Second sub-rung of temporal-plain-date. Implements from + compare with ISO date parser (YYYY-MM-DD + optional time tail).
type: project
---

# plain-date-static — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-date/`.

Sibling shape to DStat/TIS/PTS. Per-class static methods.

## Telos

- `Temporal.PlainDate.from(item)`:
  - PlainDate instance → clone via __pd_* sentinels.
  - Property bag {year, month, day, [calendar]} → range-validated; calendar must be "iso8601" or undefined.
  - ISO date string "YYYY-MM-DD" with optional T/t/space-prefixed tail (time + offset + annotations all ignored).
- `Temporal.PlainDate.compare(a, b)`: tuple compare on (year, month, day).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `parse_iso_date` + `make_plain_date` helpers + 2 methods on pd_ctor (~150 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist for `/Temporal/PlainDate/{from,compare}/`.
- **Exemplar suite**: 113 fixtures.

## Carve-outs

- Extended year format ±YYYYYY deferred.
- Property-bag with monthCode instead of month deferred.
- Calendar annotation validation (`[u-ca=cal]`) deferred.

## Status

PDS-EXT 1 LANDED 2026-05-26. 49/113 PASS (43%) + PDCF sibling yield (28 → 33).
