# temporal-plain-time — Trajectory

## TPT-EXT 0+1 — FOUNDED + plain-time-ctor-fields LANDED (2026-05-26)

Third per-class parent. Validates the per-class-parent pattern transfers cleanly to wall-clock-time classes (no calendar, no TZ).

## PTCF-EXT 1 — plain-time-ctor-fields LANDED (2026-05-26)

See `plain-time-ctor-fields/trajectory.md` for substrate detail.

### Summary

Edit ~120 LOC: Temporal.PlainTime class with 6 unit fields (hour/minute/second/millisecond/microsecond/nanosecond) stored as __pt_<unit> sentinels. Range-validated per §11.7.2 (hour 0-23, minute/second 0-59, ms/μs/ns 0-999).

Yield: 0 → 32/34 PASS (94%) on plain-time-ctor-fields pool. Diff-prod 42/42. Earlier rungs stable.

Cumulative Temporal yield post-PTCF: 240/334 (72%).

Residuals (2): 1 @@toStringTag descriptor + 1 `Temporal.PlainTime.from` not implemented (next rung).

### Finding TPT.1 (third application of per-class-parent pattern confirms the template)

Duration (10-field tuple sentinel), Instant (1-field BigInt sentinel), and PlainTime (6-field range-validated tuple sentinel) all use the same skeleton: prototype + accessor-getter properties + valueOf-throws + ctor with NewTarget check + ctor.prototype frozen pointer. The variation is per-class: number of fields, type of each (Number/BigInt), range constraints. Standing recommendation: future per-class ctor-fields rungs (PlainDate, PlainDateTime, PlainMonthDay, PlainYearMonth, ZonedDateTime) can use the same template with field-count/type adjustment. Per-class LOC budget ~100-150 for ctor-fields alone.
