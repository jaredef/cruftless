---
name: duration-string-conversion
description: Sixth sub-rung of temporal-duration. Implements toString/toJSON/toLocaleString — ISO 8601 duration format P[nY]...[nS] with sub-second roll-up and uniform-sign prefix.
type: project
---

# duration-string-conversion — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-duration/`.

Inverse of IDP parser. Reaches feature parity with PTSC / ISC.

## Telos

`Temporal.Duration.prototype.{toString, toJSON, toLocaleString}`:
- Format `[-]P[nY][nM][nW][nD][T[nH][nM][nS]]`.
- Sign prefix on the whole string when any non-zero unit is negative (uniform-sign invariant already enforced by DSV).
- Each date / time-non-seconds unit omitted if zero.
- Sub-second roll-up: combine `ms*1e6 + μs*1e3 + ns` → total nanoseconds; carry into seconds when ≥ 1e9.
- Seconds emitted when: non-zero, OR has fractional, OR neither H nor M is present in the time part.
- Fractional seconds: 9-digit zero-padded then trailing-zero trim.
- All-zero Duration → `"PT0S"`.

## Carve-outs (residuals expected)

- **options.smallestUnit / fractionalSecondDigits / roundingMode**: ~10 records deferred to `duration-options-handling`.
- **Large-value precision** (`PT5188146770.73...`): when seconds × 1e9 + sub-seconds exceeds i64, need BigInt arithmetic. Currently uses i64. ~1 record.
- **Fractional unit out of position** during `Temporal.Duration.from(string)` round-trip: 1 record; depends on iso-fractional-propagation.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `duration_to_iso_string` helper + 3 methods on dur_proto (~100 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/Duration/prototype/{toString,toJSON,toLocaleString}/`.
- **Exemplar suite**: 63 fixtures.

## Status

DSC-EXT 1 LANDED 2026-05-26. 33/63 PASS (52%). Basic forms (incl. negative + sub-second roll-up) correct.
