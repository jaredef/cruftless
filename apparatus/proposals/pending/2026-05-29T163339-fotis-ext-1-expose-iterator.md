---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 965c3008acbf416ef2df58eca0111c4f8bc61247
target_branch: main
summary: FOTIS-EXT 1 expose TypedArray @@iterator at reached prototype
risk_class: substrate
gates_pre:
  test262_fotis_target: 0 PASS / 18 FAIL in language.statements.for-of / feat:TypedArray;not-callable
  build: null
  per_locale:
    for-of-typedarray-iterator-shape: C4-positive @@iterator exposure bucket open
gates_post:
  test262_fotis_target: 18 PASS / 0 FAIL
  build: cargo build --release --bin cruft -p cruftless PASS
  lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  per_locale:
    for-of-typedarray-iterator-shape: chapter closed
---

## Substrate Moves

Commit `965c3008acbf416ef2df58eca0111c4f8bc61247` lands FOTIS-EXT 1.

- **M** = Alias TypedArray `@@iterator` to the existing `values` function and mirror it to `%TypedArray%.prototype`.
- **T** = 18-row `language.statements.for-of / feat:TypedArray;not-callable` cluster.
- **I** = `pilots/rusty-js-runtime/derived/src/intrinsics.rs`; FOTIS seed, trajectory, manifest.
- **R** = Concrete TypedArray instances now see `@@iterator` through the TAWR/TAMM prototype chain, while `TypedArray.prototype.values === TypedArray.prototype[@@iterator]` remains true.

## Risk Assessment

The change is narrow to TypedArray prototype method exposure. It reuses the existing `values` implementation and does not alter for-of bytecode or iterator-next semantics.

Verification:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS, 63 passed / 1 ignored.
- FOTIS 18-row target: 18 PASS / 0 FAIL.
- Adjacent probe: `values()`, direct `[Symbol.iterator]()`, `values === [Symbol.iterator]`, and for-of over `Uint8Array` all PASS.

## Residuals

FOTIS chapter is closed. Full `cargo test --release -p rusty-js-runtime` still has unrelated `tests/destructure.rs::t11_object_rest` TDZ failure, surfaced per §V.8; the requested lib-test target passes.
