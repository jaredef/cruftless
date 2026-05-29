---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - 7d4ed17a72bc583575a764f0ae6eb10648afb0d6
target_branch: main
summary: EPSA-EXT 2 - Error.prototype.stack cross-realm and Proxy receiver edges
risk_class: substrate
gates_pre:
  stack_directory_ext1: 28 PASS / 7 FAIL from 35 paths
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  stack_directory: 35 PASS / 0 FAIL from 35 paths
---

## Substrate Moves

This closes the EPSA residual rung requested by `helmsman/request/epsa-ext-2-cross-realm-proxy-r2`.

- **M** = cross-realm Error prototype stack accessor installation and Proxy/Reflect receiver handling.
- **T** = install realm-local Error stack accessors, expose a test262 realm global helper, and preserve receiver identity through proxy get fallback and `Reflect.get`.
- **I** = `install_error_stack_accessor`, `__cruftless_create_realm_global`, receiver-aware `reflect_get_via_receiver`, runner `$262.createRealm()` bridge, and Proxy-aware stack setter path.
- **R** = the whole current `built-ins/Error/prototype/stack/*.js` directory now passes.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 66 passed, 1 ignored.
- Full `built-ins/Error/prototype/stack/*.js` directory: 35 PASS / 0 FAIL / 0 SKIP from 35 paths.
- Artifact: `/home/jaredef/Developer/cruftless-r2-sidecar/results/epsa-ext2-final-20260529T164342Z/`.

## Risk Assessment

The change is concentrated in runtime property access, realm construction, generated `Reflect.get`, and the local test262 runner bridge. The broader risk is receiver propagation through proxy fallback paths; the final directory measurement covers the targeted stack getter/setter rows, including the previous cross-realm and Proxy residuals. Trace-format content remains out of scope for this EPSA rung.

## Composes-With

- Prior EPSA-EXT 1 accessor closure at `6c0995a9`.
- Follow-up trace-format content work, if separately authorized.

**APPROVED for push** per Helmsman EPSA-EXT 2 directive.
