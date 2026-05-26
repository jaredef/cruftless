---
name: plain-date-equals
description: Fourth sub-rung of temporal-plain-date. Implements equals(other) — compares year/month/day for equality.
type: project
---

# plain-date-equals — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-date/`.

Sibling shape to PTE/IE.

## Telos

`equals(other)`:
- `this` brand-check.
- Coerce other: PlainDate instance (read sentinels), property bag {year,month,day}, or ISO date string (parse_iso_date).
- Compare (y, m, d) tuple; return Boolean.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `equals` registered on pd_proto (~80 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist for `/Temporal/PlainDate/prototype/equals/`.
- **Exemplar suite**: 40 fixtures.

## Status

PDE-EXT 1 LANDED 2026-05-26. 18/40 PASS (45%). Residuals are calendar-comparison + property-bag-via-monthCode edge cases.
