# ODP-EXT 2 Array exotic length/index boundaries

**Proposed by**: codex-substrate-resolver-4
**Date**: 2026-05-29
**Target branch**: `main`
**Risk class**: runtime Array exotic Object.defineProperty semantics

## Proposed commits

- `0497b2c6` - `runtime: handle array defineProperty indices`

## Scope

Land ODP-EXT 2 for Array exotic `[[DefineOwnProperty]]` length/index boundaries. The move tightens canonical array-index detection to `[0, 2^32 - 2]`, ignores non-array-index numeric strings for length derivation, rejects index definitions past a non-writable length, updates length after successful high-index definitions, and stops inherited getter lookup when an own data descriptor exists.

The rung also converts Object.defineProperty TypedArray integer-index false outcomes into TypeError for the shrink-during-key-coercion exemplar. Broader TypedArray and Reflect.defineProperty behavior remains out of scope.

## Gate report

- `cargo build --release --bin cruft -p cruftless` PASS.
- Targeted ODP-EXT 2 exemplars: 8/8 PASS (`4-184`, `4-185`, `4-186`, `4-188`, `4-189`, `4-193`, `4-275`, `coerced-P-shrink`).
- 43-row descriptor-shape/property-semantics bucket: 31 PASS / 11 FAIL / 1 no-output, up from ODP-EXT 1's 26 PASS / 16 FAIL / 1 no-output (+5 PASS).
- 54-row Object.defineProperty surface: 42 PASS / 11 FAIL / 1 no-output, up from ODP-EXT 1's 33 PASS / 20 FAIL / 1 no-output (+9 PASS).
- Adjacent first-80 `built-ins/Object/defineProperty/*.js` sample: 80 PASS / 0 FAIL.

## Residuals

Remaining failures are ODP-EXT 3 shaped: arguments mapped-parameter/index descriptor rows and prototype own-shadow rows, plus the single no-output `4-116` row.
