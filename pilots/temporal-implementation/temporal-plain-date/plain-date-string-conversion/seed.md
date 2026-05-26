---
name: plain-date-string-conversion
description: Third sub-rung of temporal-plain-date. Implements toString/toJSON/toLocaleString — ISO date format YYYY-MM-DD.
type: project
---

# plain-date-string-conversion — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-date/`.

Sibling shape to PTSC/ISC/DSC. Simplest format yet.

## Telos

`toString/toJSON/toLocaleString` → `"YYYY-MM-DD"`. Year zero-padded to 4 digits for [0000, 9999]; ±YYYYYY expanded form for outside. Calendar annotation deferred (always default iso8601 in v1).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `pd_to_iso_string` helper + 3 methods (~60 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist for `/Temporal/PlainDate/prototype/{toString,toJSON,toLocaleString}/`.
- **Exemplar suite**: 33 fixtures.

## Status

PDSC-EXT 1 LANDED 2026-05-26. 27/33 PASS (82%).
