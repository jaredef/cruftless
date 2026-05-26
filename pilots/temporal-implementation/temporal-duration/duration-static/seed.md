---
name: duration-static
description: Third sub-rung of temporal-duration. Implements Temporal.Duration.from (Duration instance + property-bag forms) and Temporal.Duration.compare (sub-week-units only, throws for date-units without relativeTo).
type: project
---

# duration-static — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-duration/`.

Third per-class sub-rung. ~150 LOC budget.

## Telos

### `Temporal.Duration.from(item)`
- **Duration instance** → clone all 10 units into new instance.
- **Property bag** (plain object with unit-name properties) → read 10 units (unknown keys silently ignored per spec); throw TypeError if no unit property present; integer-validate each.
- **String** → DEFER (throws "ISO 8601 duration string parsing not yet implemented (Tier-L stub)"). Real implementation in shared sub-substrate `temporal-iso-string-parse`.

### `Temporal.Duration.compare(d1, d2, options)`
- Coerce both args via from-style logic.
- If either has year/month/week units → require `options.relativeTo`:
  - If absent → RangeError "starting point required".
  - If present → DEFER ("Temporal.Duration.compare with relativeTo not yet implemented").
- Else: convert to approximate nanoseconds (1 day = 86400e9 ns) and return -1/0/1.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `from` + `compare` registered on dur_ctor.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/Duration/{from,compare}/`.
- **Exemplar suite**: 81 fixtures (31 from + 50 compare).

## Carve-outs (residuals expected)

- ISO 8601 duration string parsing (8+4 records) — deferred to temporal-iso-string-parse.
- relativeTo-using compare (3+2 records) — deferred to duration-relative-to.
- PlainDate.from used to construct relativeTo (5 records) — depends on temporal-plain-date.

## Status

DStat-EXT 1 LANDED 2026-05-26. 22/81 PASS (27.2%) — low because residuals are mostly explicit deferrals. Combined Temporal yield across DCF+DDP+DStat: 109/172 (63%).
