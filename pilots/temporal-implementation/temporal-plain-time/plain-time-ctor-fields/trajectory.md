# plain-time-ctor-fields — Trajectory

## PTCF-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Per the per-class-parent template (Finding TInst.1 + DCF.1). Third application of the pattern; transferred cleanly.

### Edit (~120 LOC in intrinsics.rs)

- Removed `"PlainTime"` from the foundation stub-classes loop.
- `PT_UNITS` const: 6 tuples `(name, min, max)`: hour/0-23, minute+second/0-59, millisecond+microsecond+nanosecond/0-999.
- pt_proto allocated; 6 accessor-getter properties installed (one per unit) using `make_native_non_ctor` + `PropertyDescriptor` with `getter: Some(...)`.
- `valueOf` method throws TypeError.
- @@toStringTag set to "Temporal.PlainTime".
- Ctor via `make_native_with_length("PlainTime", 0, ...)`:
  - NewTarget check.
  - For each of 6 args: default-to-zero on undefined, ToNumber, finite+integer+range check.
  - Allocate instance with proto + 6 sentinels.
- ctor.prototype = pt_proto (frozen).
- Install on Temporal.PlainTime slot.

### Probes (Rule 23 verification at landing)

- `new Temporal.PlainTime(15, 23, 30, 123, 456, 789)` → reads back all 6 fields ✓
- `new Temporal.PlainTime()` → all zeros ✓
- `Temporal.PlainTime(12)` (no new) → TypeError ✓
- `new Temporal.PlainTime(24)` → RangeError (hour > 23) ✓
- `new Temporal.PlainTime(0, 60)` → RangeError ✓
- `new Temporal.PlainTime(1.5)` → RangeError (non-integer) ✓
- `new Temporal.PlainTime(1).valueOf()` → TypeError ✓
- `instanceof Temporal.PlainTime` → true ✓
- `Temporal.PlainTime.name === "PlainTime"`, `.length === 0` ✓

### Yield

- plain-time-ctor-fields exemplar pool (34): **0 → 32/34 PASS (94%)**.
- Diff-prod: 42/42 maintained.
- Earlier rungs stable.

Cumulative Temporal yield post-PTCF: **240/334 (72%)**.

### Residuals (2)

| Shape | Cause |
|---|---|
| `@@toStringTag should be an own property` | propertyHelper.verifyProperty descriptor-shape check (same residual seen in Duration + Instant ctor-fields) |
| `Temporal.PlainTime.from not callable` | static method, belongs to plain-time-static rung |

### Findings

**Finding PTCF.1 (per-class template now validated across three classes)**: Duration / Instant / PlainTime have used the same skeleton: prototype + accessors + valueOf-throws + NewTarget-checked ctor + frozen ctor.prototype. Each per-class ctor-fields rung lands at ~100-150 LOC. Standing recommendation: PlainDate (10 fields: year/month/day + calendar machinery) will be the first deviation — calendar interaction adds substrate beyond the template. PlainDateTime (year/month/day + 6 time fields + calendar) similarly. The pure-fields classes (PlainTime, Duration, Instant) are the template-applicable ones.

**Finding PTCF.2 (the @@toStringTag descriptor residual is a recurring cross-class issue)**: Same residual in Duration ctor-fields (1) + Instant ctor-fields (1) + PlainTime ctor-fields (1) = 3 across the per-class rungs. The shape: propertyHelper.verifyProperty checks `Object.getOwnPropertyDescriptor(proto, "@@toStringTag")` returns the expected descriptor exactly; cruft's `set_own_frozen` may set fields differently. Standing recommendation: spawn a cross-cutting `temporal-tostringtag-descriptor` rung to fix the descriptor shape at the foundation, closing 3+ residuals. Similar to DSV-EXT 1 pattern.

### Status

PTCF-EXT 1 CLOSED. Cumulative Temporal yield at 72%. Next ripe: any of (a) plain-time-static (~50 records), (b) temporal-tostringtag-descriptor (cross-cutting), (c) iso-fractional-propagation (7 Duration deferrals).
