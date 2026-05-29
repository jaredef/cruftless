# ODP-EXT 1 ValidateAndApply property-key closure

**Proposed by**: codex-substrate-resolver-4
**Date**: 2026-05-29
**Target branch**: `main`
**Risk class**: runtime Object.defineProperty semantic helper

## Proposed commits

- `079a4e78` - `runtime: validate object defineProperty descriptors`

## Scope

Land the first substrate rung for `pilots/object-defineProperty-edge-cases`: property-key-aware ordinary own descriptor lookup/storage and a shared `ValidateAndApplyPropertyDescriptor` helper for `Object.defineProperty`.

The move fixes SameValue checks, non-configurable data/accessor transitions, explicit `undefined` accessor field presence, Symbol-key descriptor reflection through `Object.getOwnPropertyDescriptor`, and strict computed writes to non-writable Symbol-key data descriptors.

Out of scope: array exotic length/index behavior, arguments mapped-parameter defineProperty behavior, prototype-chain own shadow creation, typed-array/resizable rows, and Reflect.defineProperty boolean-return alignment.

## Gate report

- `cargo build --release --bin cruft -p cruftless` PASS.
- Targeted exemplars: 8/8 PASS (`4-217`, `4-218`, `4-254`, `4-257`, and four `symbol-data-property-*` rows).
- 43-row descriptor-shape/property-semantics bucket: 26 PASS / 16 FAIL / 1 no-output. The bucket was matrix-classified failing at ODP-EXT 0, so the measured pass gain is +26.
- 54-row Object.defineProperty surface: 33 PASS / 20 FAIL / 1 no-output, up from ODP-EXT 0 current rerun baseline 4 PASS / 37 FAIL / 13 no-output.

## Residuals

Remaining failures match the planned ODP-EXT 2/3/4 roadmap: array length/index, arguments mapped parameters, prototype own shadowing, and typed-array/resizable behavior.
