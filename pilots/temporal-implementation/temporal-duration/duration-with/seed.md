---
name: duration-with
description: Fourth sub-rung of temporal-duration. Implements Duration.prototype.with — partial-update returning new Duration with overridden unit fields.
type: project
---

# duration-with — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-duration/`.

Per parent's sub-rung topology. ~50 LOC budget.

## Telos

`Duration.prototype.with(durationLike)`:
- Argument must be an Object; primitives (undefined / null / string / number / boolean / Symbol / bigint) → TypeError.
- Argument must have at least one recognized unit-name property; arg with only unrecognized keys (e.g., `{nonsense: 1}`, `{sign: 1}`) → TypeError.
- For each unit name present in the argument, override the corresponding field; missing units inherit from `this`.
- Return new Duration via shared `make_duration` helper.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — `with` added to dur_proto.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO allowlist extended with `/Temporal/Duration/prototype/with/`.
- **Exemplar suite**: 22 fixtures.

## Status

DWith-EXT 1 LANDED 2026-05-26. 17/22 PASS (77%). 5 residuals: 3 are uniform-sign validation (deferred to duration-sign-validation follow-on); 2 are option-coercion order edge cases.
