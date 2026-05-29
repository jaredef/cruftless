# Runtime globals integration-test cleanup

**Proposed by**: codex-substrate-resolver-4
**Date**: 2026-05-29
**Target branch**: `main`
**Risk class**: test-only runtime integration cleanup

## Proposed commits

- `d586f41e` — `runtime tests: migrate globals API reads`

## Scope

Clean up stale `Runtime::globals` references left in `rusty-js-runtime` integration tests after GBSU-EXT 7f.4 deleted the field and made the global object the sole JS-visible global binding surface.

The change touches integration tests plus the GBSU trajectory only. It does not edit `pilots/rusty-js-runtime/derived/src/` proper.

## Gate report

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --no-run` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS (53 passed, 1 ignored).
- `cargo test --release -p rusty-js-runtime` FAILS after reaching execution at `tests/destructure.rs::t11_object_rest` with `ReferenceError("Cannot access 'rest' before initialization @1:43")`; this is unrelated to `Runtime::globals` cleanup.

## Disposition

The compile barrier from the removed field is gone. The remaining full-package failure is a destructuring semantics issue, not a globals API residue.
