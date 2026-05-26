# plain-time-with — Trajectory

## PTW-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Sibling shape to DWith. Third sub-rung of temporal-plain-time.

### Edit (~80 LOC in intrinsics.rs)

`Temporal.PlainTime.prototype.with(timeLike)`:
- Brand-check via `__pt_hour` sentinel.
- Argument primitivity check.
- Reject if argument is a Temporal-class instance (via __pt_/__td_/__ti_ sentinel presence).
- At-least-one-unit + ToNumber + finite + integer + range-validate per unit.
- Allocate new PlainTime via direct sentinel install.

### Probes (Rule 23 verification at landing)

- `t.with({hour: 8, second: 0})` → fields override; others retained ✓
- `t.with(undefined)` → TypeError ✓
- `t.with("10:00")` → TypeError ✓
- `t.with({nonsense: 1})` → TypeError ✓

### Yield

- plain-time-with exemplar pool (22): **0 → 12/22 PASS (55%)**.
- Diff-prod: 42/42.
- Earlier rungs stable.

Cumulative Temporal yield post-PTW: **298/439 (68%)**.

### Residual decomposition (10 fails)

| Shape | Count | Destination |
|---|---:|---|
| options.overflow="constrain" not implemented | ~4 | plain-time-options-handling |
| TypeError shape mismatch | 2 | spec-strict error class |
| Spy-based valueOf trace | 1 | brand-check observer (sibling DCF residual) |
| RangeError not thrown | 1 | edge case |
| from() on other Temporal classes | 1 | per-class from |
| misc | 1 | per-test |

### Status

PTW-EXT 1 CLOSED. Three of Duration's six sub-rungs (ctor/derived/static/with/sign + signValidation = 6 actually — Duration has 5 sub-rungs landed) now have PlainTime parallels. PlainTime has 3 (ctor-fields + static + with).
