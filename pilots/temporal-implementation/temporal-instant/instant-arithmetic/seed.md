---
name: instant-arithmetic
description: Fifth sub-rung of temporal-instant. Implements add / subtract / since / until — sub-day Duration arithmetic via BigInt, returns Instant or Duration.
type: project
---

# instant-arithmetic — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-instant/`.

First arithmetic rung in the Temporal program. Spec §11.6.4.x.

## Telos

- `add(durationLike)`:
  - Coerce arg to Duration (Duration instance, property bag, or ISO string).
  - Reject year/month/week/day units (Instant has no calendar context).
  - Convert sub-day units (h/min/sec/ms/μs/ns) to total ns.
  - BigInt add to this.epochNs; range-check; return new Instant.
- `subtract(durationLike)`: same with negated total.
- `since(other, options)`: coerce other; compute (this - other) in ns; return Duration with seconds + sub-second fields (default options.largestUnit = "second").
- `until(other, options)`: same with (other - this).

## Carve-outs

- **options.largestUnit / smallestUnit / roundingMode / roundingIncrement**: ~50 records. Default behavior (second + sub-seconds) only.
- **BigInt precision for since/until**: uses f64 intermediate (precision loss at ~2^53 ns ≈ 100 days). Needs full BigInt for spec-conformant since/until of large diffs.
- **Sub-minute offset / extended-year** edge cases shared with IDTP residuals.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — 4 methods + duration_to_sub_day_ns + diff_to_duration helpers (~250 LOC total).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist for `/Temporal/Instant/prototype/{add,subtract,since,until}/`.
- **Exemplar suite**: 196 fixtures.

## Status

IA-EXT 1 LANDED 2026-05-26. 66/196 PASS (34%). Largest single-rung yield today aside from DCF.
