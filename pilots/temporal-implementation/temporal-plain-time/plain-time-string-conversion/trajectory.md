# plain-time-string-conversion — Trajectory

## PTSC-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Fourth sub-rung of temporal-plain-time. Sibling rung; reverse direction of PTS parser.

### Edit (~80 LOC in intrinsics.rs)

- `pt_to_iso_string(rt, this_id) -> Result<String, RuntimeError>` shared helper: brand-check; read 6 unit sentinels; compose nanos = ms*1e6 + μs*1e3 + ns; format "HH:MM:SS" + optionally ".fff…" with trailing-zero trim.
- `toString`, `toJSON`, `toLocaleString` methods all call `pt_to_iso_string` (v1: locale + options ignored per spec deferral).

### Probes (Rule 23 verification at landing)

- `new Temporal.PlainTime(15, 23).toString()` → `"15:23:00"` ✓
- `new Temporal.PlainTime(15, 23, 30).toString()` → `"15:23:30"` ✓
- `new Temporal.PlainTime(15, 23, 30, 123).toString()` → `"15:23:30.123"` ✓
- `new Temporal.PlainTime(15, 23, 30, 123, 456, 789).toString()` → `"15:23:30.123456789"` ✓
- `new Temporal.PlainTime(15, 23, 30, 123, 400).toString()` → `"15:23:30.1234"` ✓ (trailing-zero trim)
- `new Temporal.PlainTime(15, 23, 30).toJSON()` → `"15:23:30"` ✓

### Yield

- plain-time-string-conversion exemplar pool (54): **0 → 26/54 PASS (48%)**.
- Diff-prod: 42/42.

Cumulative Temporal yield post-PTSC: **324/493 (66%)**.

### Residual decomposition (28 fails)

| Shape | Count | Destination |
|---|---:|---|
| options.smallestUnit + options.fractionalSecondDigits | ~20 | plain-time-options-handling |
| options.roundingMode (truncation) | ~2 | same |
| TypeError on wrong options type | ~3 | spec-strict option-coercion |
| misc edge cases | ~3 | per-test |

### Status

PTSC-EXT 1 CLOSED. PlainTime now has 4 sub-rungs landed (ctor + static + with + string-conversion). Cumulative Temporal yield 66%.
