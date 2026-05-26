---
name: plain-time-equals
description: Fifth sub-rung of temporal-plain-time. Implements equals(other) — coerces other via from-like logic and compares all 6 units for equality.
type: project
---

# plain-time-equals — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-time/`.

Sibling shape to PTS (uses same coercion logic). Reuses `parse_iso_time` for string-arg.

## Telos

`Temporal.PlainTime.prototype.equals(other)` returns Boolean:
- `this` brand-check.
- Coerce `other`: PlainTime instance (clone-read), object property-bag (range-validated), or ISO time string.
- Compare all 6 unit values; return true iff every unit equals.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `equals` registered on pt_proto (~80 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/PlainTime/prototype/equals/`.
- **Exemplar suite**: 31 fixtures.

## Status

PTE-EXT 1 LANDED 2026-05-26. 21/31 PASS (68%). Residuals are annotation/critical-flag rejection edge cases.
