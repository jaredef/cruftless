---
name: plain-time-with
description: Third sub-rung of temporal-plain-time. Implements PlainTime.prototype.with — partial-update method.
type: project
---

# plain-time-with — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-plain-time/`.

Sibling shape to DWith (Duration.prototype.with). Per parent sub-rung topology.

## Telos

`Temporal.PlainTime.prototype.with(timeLike)`:
- `this` brand-check via `__pt_hour` sentinel.
- Argument must be Object (primitives → TypeError).
- Argument must NOT be a Temporal class instance (rejects sibling instances via __pt_/__td_/__ti_ sentinel presence).
- Argument must have ≥1 recognized unit-name property.
- For each present unit: ToNumber + finite + integer + range-validate.
- Return new PlainTime with overridden units + retained units.

## Carve-outs (residuals expected)

- **options.overflow** = "constrain" (default) — should clamp out-of-range values rather than reject. My impl always rejects out-of-range. Deferred to `plain-time-options-handling` rung. Closes ~4 records.
- Spec-coercion-order tests (get hour.valueOf trace) — deferred.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `with` registered on pt_proto (~80 LOC).
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/PlainTime/prototype/with/`.
- **Exemplar suite**: 22 fixtures.

## Status

PTW-EXT 1 LANDED 2026-05-26. 12/22 PASS (55%).
