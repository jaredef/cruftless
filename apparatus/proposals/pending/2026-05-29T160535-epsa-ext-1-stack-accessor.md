---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 9f1b35ecacdb916243c8457a1b1da897886c932e
  - 6c0995a9c0bca944534580de84fb4f4a038f6a58
target_branch: main
summary: EPSA-EXT 1 - install Error.prototype.stack accessor and close exact 22-cell cluster
risk_class: substrate
gates_pre:
  test262_full: 67.6 (not re-measured this rung)
  test262_sample: post-EPSUA matrix row 0 PASS / 22 FAIL
  diff_prod: not re-measured
  per_locale:
    EPSA: 0 PASS / 22 FAIL exact matrix cell
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  per_locale:
    EPSA_exact_cell: 22 PASS / 0 FAIL
    EPSA_stack_directory: 28 PASS / 7 FAIL
---

## Substrate moves

Two commits are covered because the Phase 0 spawn commit was local in this resolver clone when EPSA-EXT 1 began:

- `9f1b35ec` spawns `pilots/error-prototype-stack-accessor/` and refreshes the locale manifest.
- `6c0995a9` installs the `%Error.prototype%.stack` accessor and records EPSA-EXT 1 measurement.

M-T-I-R:

- **M** = `Error.prototype.stack` accessor parity for the post-EPSUA `feat:error-stack-accessor` row.
- **T** = install a real non-enumerable configurable accessor on `%Error.prototype%`, mark Error instances with `InternalKind::Error`, and stop eagerly installing own `stack` on fresh Error instances.
- **I** = getter/setter native functions in `install_error_globals`, Error constructor/internal-error `InternalKind::Error` marking, and EPSA trajectory findings.
- **R** = closes the exact 22-row matrix cell while keeping trace formatting and Proxy/cross-realm edge routing deferred to later rungs.

## Verification

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS, 61 passed, 1 ignored.
- Targeted harness over `/home/jaredef/test262/test/built-ins/Error/prototype/stack/*.js`: 28 PASS / 7 FAIL.
- Exact 2026-05-29 matrix cell `Error.prototype.stack` / `feat:error-stack-accessor`: 22 PASS / 0 FAIL.

## Risk assessment

The main semantic risk is removing the fresh own `stack` data property that EIPD previously made non-enumerable. The current test262 accessor surface requires that removal: `instance-no-own-stack.js` and `instance-not-enumerable.js` both expect fresh Error instances to reach stack through the inherited accessor. User code that reads `e.stack` still gets a string via the getter, while assignment, direct setter calls, and `Error.captureStackTrace` can still create own `stack` data properties.

The accessor is installed only on `%Error.prototype%`, not each NativeError prototype. NativeError prototypes already chain through `%Error.prototype%`; giving them own stack descriptors would violate the NativeError immediate-prototype checks.

Out-of-scope residuals are explicitly retained: cross-realm constructor/prototype cloning and Proxy `[[GetOwnProperty]]` / `[[DefineOwnProperty]]` / `[[Set]]` edge routing.

## Composes-with

- `pilots/error-prototype-stack-accessor/` EPSA locale.
- `pilots/error-instance-property-descriptors/` EIPD descriptor work.
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_error_globals`.
- Post-EPSUA test262 sample matrix from 2026-05-29.
