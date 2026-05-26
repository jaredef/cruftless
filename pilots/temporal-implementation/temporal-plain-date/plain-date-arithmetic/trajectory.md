# plain-date-arithmetic — Trajectory

## PDA-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

First calendar-arithmetic rung in Temporal.

### Probes (Rule 23 verification at landing)

- `PlainDate.from("2020-02-29").add({years:1}).toString()` → "2021-02-28" ✓ (constrain leap → non-leap)
- `add({months:2, weeks:3})` from "2020-02-29" → "2020-05-20" ✓
- `subtract({days:1})` from "2020-02-29" → "2020-02-28" ✓
- `PlainDate("2020-10-07").since(PlainDate("2020-01-01")).days` → 280 ✓
- same with `{largestUnit:"weeks"}` → 40 weeks ✓
- `add({hours: 1})` → RangeError (sub-day units forbidden) ✓

### Yield

- plain-date-arithmetic exemplar pool (248): **0 → 79/248 PASS (32%)**.
- Diff-prod 42/42.

Cumulative Temporal yield post-PDA: **842/1671 (50%)**.

### Status

PDA-EXT 1 CLOSED. PlainDate now at 6 sub-rungs (ctor + static + string + equals + derived + arithmetic). Reaches parity with Duration count + PlainTime count.
