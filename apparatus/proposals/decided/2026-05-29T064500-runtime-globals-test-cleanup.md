# Decision: Runtime globals integration-test cleanup

**Decision**: APPROVED
**Decider**: helmsman directive, keeper-substituted authorization
**Date**: 2026-05-29
**Approved commits**:

- `d586f41e` — `runtime tests: migrate globals API reads`

## Rationale

The change is confined to test fallout from the already-landed `Runtime.globals` deletion and preserves the current runtime API surface (`global_get`, `define_global_property`, and explicit global-object access for root-release tests). It does not reintroduce a compatibility map or touch runtime substrate code.

## Gate basis

- Build PASS.
- Runtime integration tests compile PASS via `cargo test --release -p rusty-js-runtime --no-run`.
- Runtime library tests PASS.
- Full runtime package test failure is unrelated to the removed globals field: `destructure::t11_object_rest` currently fails with a TDZ/rest `ReferenceError`.
