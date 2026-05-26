---
name: duration-ctor-fields
description: First sub-rung of temporal-duration. Implements the Temporal.Duration constructor, 10 accessor-property unit getters, valueOf-throws-TypeError, ctor.prototype, and @@toStringTag.
type: project
---

# duration-ctor-fields — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-duration/`.

First per-class substrate rung in the Temporal program. ~120 LOC budget (per parent's per-rung cost estimate).

## Telos

After landing:
- `Temporal.Duration` is a constructor with `length === 0` and `name === "Duration"`.
- `new Temporal.Duration(y, mo, w, d, h, mi, s, ms, μs, ns)` returns an instance with internal-slot sentinels `__td_{unit}` for each unit.
- All 10 args are optional; `undefined` → 0; non-integer or non-finite → RangeError; -0 normalized to 0.
- Called as a function (no `new`) → TypeError per §11.1.1 step 1.
- `Temporal.Duration.prototype` is a frozen object with `@@toStringTag === "Temporal.Duration"` + 10 accessor-property getters (one per unit) + a `valueOf` method that throws TypeError + a `constructor` back-pointer.
- Each getter brand-checks via sentinel presence; `Temporal.Duration.prototype.years.call(plainObject)` throws TypeError.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — extended with the Duration ctor + prototype + 10 accessor-property getters. ~110 LOC added.
- `legacy/host-rquickjs/tests/test262/runner.mjs` — RFSDO-EXT 3 (PARTIALLY_IMPLEMENTED map) added.
- **Exemplar suite**: 67 fixtures (ctor + 10 unit-undefined + basic + builtin + call-builtin + prop-desc + name + length + 3 range-error + valueOf + 10 getters).

## R13 prospective C1-C4

- C1 (sibling): HOLDS — Date / RegExp use the same prototype + ctor + sentinel pattern.
- C2 (shape-compat): HOLDS — additive class install on existing namespace.
- C3 (cost-positive): HOLDS — 64 tests close from one rung.
- C4 (bail-safe): HOLDS — runtime-tier only; engine unchanged for non-Duration code.

## Status

DCF-EXT 1 LANDED 2026-05-26. 64/67 PASS (95.5%). 3 residuals belong to follow-on rungs (2 getter-trace, 1 `from` method).
