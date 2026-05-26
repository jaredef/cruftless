# instant-static — Trajectory

## TIS-EXT 0+1 — FOUNDED + LANDED (2026-05-26)

Sibling-shape to DStat. Second sub-rung in temporal-instant.

### Edit (~95 LOC in intrinsics.rs)

- Inline helper `make_instant(rt, proto, ns_value) -> Result<Value, RuntimeError>` — BigInt range-check + alloc + sentinel.
- `from(item)`: string → defer; Instant clone via __ti_ns brand-check.
- `fromEpochMilliseconds(ms)`: ToNumber + finite + trunc; concat "000000" + BigInt::from_decimal; construct.
- `fromEpochNanoseconds(ns)`: must be BigInt (Number → TypeError per spec).
- `compare(a, b)`: extract_ns inline helper (Instant brand or string-defer); f64 compare.

### Edit (~4 LOC in runner.mjs)

RFSDO allowlist extended with `/Temporal/Instant/{from,fromEpochMilliseconds,fromEpochNanoseconds,compare}/`.

### Probes (Rule 23 verification at landing)

- `Temporal.Instant.from(instance)` → clone with same epochNanoseconds ✓
- `Temporal.Instant.fromEpochMilliseconds(217175010123)` → 217175010123000000n ✓
- `Temporal.Instant.fromEpochNanoseconds(123n)` → 123n ✓
- `Temporal.Instant.fromEpochNanoseconds(123)` → TypeError ✓
- `compare(new Instant(1n), new Instant(2n))` → -1 ✓
- `compare(new Instant(5n), new Instant(5n))` → 0 ✓

### Yield

- instant-static exemplar pool (81): **0 → 28/81 PASS (34.6%)**.
- Diff-prod: 42/42 maintained.
- Earlier rungs (instant-ctor-fields 21/25, Duration sub-rungs) stable.

Cumulative Temporal yield post-TIS: **178 PASS / 300 exemplars (59%)**.

### Residual decomposition (53 fails)

| Shape | Count | Destination |
|---|---:|---|
| ISO 8601 datetime string parsing (from-string) | 15 | temporal-iso-string-parse |
| ISO 8601 datetime string parsing (compare-string) | 9 | temporal-iso-string-parse |
| Calendar annotations in ISO strings | 2 | temporal-iso-string-parse |
| ISO uppercase annotations rejected | 2 | temporal-iso-string-parse |
| Variant minus sign edge | 2 | temporal-iso-string-parse |
| ZonedDateTime callee | 2 | temporal-zoned-date-time (out of scope) |
| Empty string ISO | 1 | temporal-iso-string-parse |
| misc edge cases | ~20 | per-test inspection |

### Findings

**Finding TIS.1 (the ISO-string-parse dependency is the dominant unblocker for static methods)**: Both DStat (DStat.2 standing-rec) and TIS show the same shape — residuals dominated by string-arg variants of from() and compare(). A `temporal-iso-string-parse` shared substrate would unblock ~24 records here + ~12 in DStat + future per-class string handling. Standing recommendation: prioritize temporal-iso-string-parse as the next rung; estimated impact ~50+ records across all current Temporal sub-rungs.

**Finding TIS.2 (Number vs BigInt coercion is asymmetric across Temporal entry points)**: `fromEpochMilliseconds` takes Number; `fromEpochNanoseconds` takes BigInt only (Number → TypeError per spec). The `Temporal.Instant` ctor itself takes BigInt-via-ToBigInt (Number → TypeError). This asymmetry is spec-correct but easy to get wrong; documenting it via the rejection error messages (which include "must be BigInt") helps consumer debugging.

### Status

TIS-EXT 1 CLOSED. Cumulative Temporal yield at 59%. Next ripe: temporal-iso-string-parse (would unlock ~50+ records across DStat + TIS + future string-conversion rungs).
