# instant-arithmetic — Trajectory

## IA-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Fifth sub-rung of temporal-instant. First arithmetic rung in the Temporal program.

### Edit (~250 LOC in intrinsics.rs)

- `duration_to_sub_day_ns(rt, v) -> i64`: coerce arg to Duration (string via parse_iso_duration | object brand-or-bag); reject year/month/week/day; compose `h*3600e9 + min*60e9 + sec*1e9 + ms*1e6 + μs*1e3 + ns`.
- `add(durationLike)`: BigInt add total-ns to this.__ti_ns; new Instant via make_instant.
- `subtract(durationLike)`: BigInt sub.
- `diff_to_duration(rt, dur_proto, diff_ns)`: decompose f64 ns into seconds + ms + μs + ns; allocate Duration with seconds + sub-second fields.
- `since(other)`: coerce other (Instant or ISO datetime string); compute this_ns - other_ns; build Duration.
- `until(other)`: same with other_ns - this_ns.

### Probes (Rule 23 verification at landing)

- `later.since(earlier).seconds` → 1355167388 (43-year diff in seconds) ✓
- `later.since(earlier).seconds === earlier.until(later).seconds` → true ✓
- `earlier.add(diff).equals(later)` → true ✓
- `earlier.add(new Temporal.Duration(0,0,0,0,1,30)).epochNanoseconds - earlier.epochNanoseconds` → 5400000000000n ✓
- `earlier.add(new Temporal.Duration(1))` (year arg) → RangeError ✓

### Yield

- instant-arithmetic exemplar pool (196): **0 → 66/196 PASS (34%)**.
- Diff-prod: 42/42.
- Earlier rungs stable.

Cumulative Temporal yield post-IA: **495/884 (56%)**.

### Residual decomposition (130)

| Shape | Count | Destination |
|---|---:|---|
| RangeError edge cases (Duration limits, options validation) | ~27 | instant-options-handling + per-test |
| Fractional unit out of position (Duration with fractional H/M) | ~4 | iso-fractional-propagation |
| Options.largestUnit / smallestUnit / roundingMode | ~50 | instant-options-handling |
| BigInt precision (subtract recovers earlier from later test) | ~5 | instant-arithmetic-bigint-precision |
| TypeError on wrong option types | ~10 | spec-strict option-coercion |
| Sub-minute offset / extended year edges | ~10 | iso-edge-cases |
| Misc | ~24 | per-test |

### Findings

**Finding IA.1 (BigInt precision matters for since/until)**: Probe revealed `later.subtract(later.since(earlier)).equals(earlier)` returns false despite all components being individually correct. Root cause: `since` uses f64 to compute diff and to populate Duration seconds/sub-seconds; for a 1.35e18 ns diff (43 years), f64 has ~16 digits of precision but ns precision needs ~19 digits, so the last few ns get lost. Then `subtract` rebuilds total_ns and BigInt-subs; the lost low-order ns produce a slightly-off result. Standing recommendation: arithmetic that returns Duration from a BigInt diff must use BigInt throughout; downstream sentinel-storage as f64 forces precision loss only at the FINAL display step, not at intermediate decomposition.

**Finding IA.2 (arithmetic substrate is the leverage tier for per-class yield)**: Instant pool was 144 with 3 rungs (ctor + static + string + equals). IA-EXT 1 adds 66 in one rung — comparable to the entire prior Instant work. Standing recommendation: when a per-class has ctor / static / string-conversion / equals landed, arithmetic is the highest single-rung yield expectation; budget ~150-250 LOC for arithmetic rung per class.

### Status

IA-EXT 1 CLOSED. Instant now at 5 sub-rungs (ctor + static + string-conversion + equals + arithmetic). Cumulative Temporal yield 56%.
