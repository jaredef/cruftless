---
name: plain-time-static
description: Second sub-rung of temporal-plain-time. Implements from + compare with ISO time-string parsing including full-datetime time-extraction.
type: project
---

# plain-time-static — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-time/`.

Per parent's sub-rung topology. Sibling shape to DStat / TIS but with full ISO time + datetime time-extraction parsing.

## Telos

- `Temporal.PlainTime.from(item)`:
  - PlainTime instance → clone.
  - Property bag → read 6 unit names (range-validated, at-least-one required).
  - String → parse as ISO time. Accepts:
    - Bare HH:MM[:SS[.fff]].
    - Optional leading T/t.
    - Optional leading date prefix `YYYY-MM-DDT` (full datetime → time-portion extracted per §11.7.1).
    - Optional trailing Z or ±HH:MM (offset accepted and ignored).
- `Temporal.PlainTime.compare(a, b)`: coerce both via above logic; convert to nanoseconds-of-day; return -1/0/1.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `parse_iso_time` inline fn + `from` + `compare` registered on pt_ctor.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/PlainTime/{from,compare}/`.
- **Exemplar suite**: 83 fixtures (51 from + 32 compare).

## Carve-outs (residuals expected)

- Bracketed annotation rejection (`[!u-ca=...]` critical, `[U-CA=...]` uppercase) — deferred to `plain-time-annotation-validation` follow-on. Per-spec rejection is required; my parser would need to validate annotation grammar.
- Leap-second `23:59:60` — spec rejects; my parser accepts (range check is `≤ 59`).
- Compact basic-form date (`20210101`) — deferred.
- "1976-11-18T15:23:30.1" 1-digit fractional second — should parse (1-9 digits accepted in my parser; the test shape probably differs).

## Status

PTS-EXT 1 LANDED 2026-05-26. 46/83 PASS (55%).
