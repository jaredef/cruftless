---
name: instant-equals
description: Fourth sub-rung of temporal-instant. Implements equals(other) — compares epochNanoseconds (BigInt) for equality.
type: project
---

# instant-equals — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-instant/`.

Sibling shape to PTE. Reuses `parse_iso_datetime` for string-arg.

## Telos

`Temporal.Instant.prototype.equals(other)`:
- Brand-check `this` via `__ti_ns`.
- Coerce `other`: Instant instance (read `__ti_ns`) or ISO datetime string (parse via parse_iso_datetime).
- Compare BigInt epochNanoseconds; return Boolean.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `equals` registered on inst_proto (~40 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/Instant/prototype/equals/`.
- **Exemplar suite**: 30 fixtures.

## Status

IE-EXT 1 LANDED 2026-05-26. 18/30 PASS (60%). Residuals are IDTP-parser edge cases (extended-year format, HH-only time, critical annotations).
