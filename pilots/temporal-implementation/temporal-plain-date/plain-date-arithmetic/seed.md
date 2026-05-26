---
name: plain-date-arithmetic
description: Sixth sub-rung of temporal-plain-date. Implements add / subtract / since / until — ISO calendar arithmetic with overflow=constrain.
type: project
---

# plain-date-arithmetic — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-date/`.

First Temporal arithmetic rung with calendar semantics.

## Telos

- `add(durationLike)`: years+months adjustment (carry from months); day clamp to max-day-for(new_year, new_month) per overflow="constrain" default; then weeks*7+days as day offset via civil_from_days/days_from_civil roundtrip.
- `subtract`: same with negated.
- `since/until(other, {largestUnit?})`: days diff; output Duration with days (or weeks+days when largestUnit="weeks"/"week"). years/months largestUnit → RangeError (deferred to plain-date-calendar-balancing).

## Carve-outs

- options.overflow="reject" not handled (always constrain).
- options.largestUnit "months"/"years" not implemented (needs calendar-aware year/month subtraction with month-end handling).
- Sub-day Duration units rejected per spec.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — pda_civil_from_days + pda_days_from_civil + pda_is_leap + pda_days_in_month + pda_duration_units + pda_extract_ymd + pda_make_duration_days helpers + 4 methods (~280 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist for `/Temporal/PlainDate/prototype/{add,subtract,since,until}/`.
- **Exemplar suite**: 248 fixtures.

## Status

PDA-EXT 1 LANDED 2026-05-26. 79/248 PASS (32%).
