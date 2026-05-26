---
name: plain-time-arithmetic
description: Sixth sub-rung of temporal-plain-time. Implements add / subtract / since / until — sub-day Duration arithmetic with 24h wrap, returns PlainTime or signed-magnitude Duration.
type: project
---

# plain-time-arithmetic — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-time/`.

Sibling shape to IA. Wall-clock semantics: add/subtract wrap modulo 24h; since/until return signed-magnitude Duration normalized to ±12h.

## Telos

- `add(durationLike)`: coerce arg; reject year/month/week (date units beyond day); compose total ns including days*24h; add to this ns-of-day; wrap via `rem_euclid(NS_PER_DAY)`; decompose; new PlainTime.
- `subtract(durationLike)`: same with negated.
- `since(other, options)`: coerce other; compute (this - other) in ns; normalize to (-12h, +12h]; return Duration with hours+min+sec+sub-second fields.
- `until(other, options)`: same with (other - this).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — 4 methods + helpers (duration_to_subday_ns_pt, pt_ns_of_day, pt_from_ns_of_day, diff_to_pt_duration, coerce_pt_to_ns) (~250 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist for `/Temporal/PlainTime/prototype/{add,subtract,since,until}/`.
- **Exemplar suite**: 214 fixtures.

## Status

PTA-EXT 1 LANDED 2026-05-26. 82/214 PASS (38%).
