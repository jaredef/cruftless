# CITPT-EXT 2 TDZ Object-Rest Init-Site Summary

Directive: `helmsman/request/citpt-ext-2-tdz-lexical-module-r4`.

## Landed

- `pilots/rusty-js-bytecode/derived/src/compiler.rs`: object-rest binding initialization in `emit_destructure()` now emits `Op::InitLocal` instead of `Op::StoreLocal`.
- This matches existing destructure leaf writes and allows declaration initialization to overwrite a TDZ sentinel.

## Closed

- `cargo test --release -p rusty-js-runtime --test destructure t11_object_rest -- --nocapture` now passes.
- The prior full runtime package-test blocker `ReferenceError("Cannot access 'rest' before initialization")` is closed.

## Verification

- `cargo build` — PASS.
- `cargo test --release -p rusty-js-runtime --lib` — PASS (`68 passed`, `1 ignored`).
- 9-cell TDZ package smoke via `legacy/host-rquickjs/tools/parity-measure.sh` — `0/9` PASS; all nine still report the original lexical/module TDZ failures.

## Residual

The package cluster remains open. The object-rest fix closes the destructuring/rest init-site sub-shape only; the remaining cells require a direct lexical/module evaluation-order rung.
