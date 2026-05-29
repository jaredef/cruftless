---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - f1b83a9990030cc5b542674ac02e0eeff4a13214
target_branch: main
summary: APS-EXT 2 primitive receiver ToObject for Array.prototype.sort
risk_class: substrate
gates_pre:
  test262_sort_target: 25 PASS / 1 FAIL after APS-EXT 1
  build: cargo build --release --bin cruft -p cruftless PASS before proposal
  per_locale:
    array-prototype-sort: call-with-primitive residual open
gates_post:
  test262_sort_target: 26 PASS / 0 FAIL
  build: cargo build --release --bin cruft -p cruftless PASS
  per_locale:
    array-prototype-sort: APS target cluster closed
---

## Substrate Moves

Commit `f1b83a9990030cc5b542674ac02e0eeff4a13214` lands APS-EXT 2.

- **M** = Array.prototype.sort calls canonical `Runtime::to_object(current_this)` instead of the older `prototype::to_array_this` helper.
- **T** = `built-ins/Array/prototype/sort/call-with-primitive.js`.
- **I** = `pilots/rusty-js-runtime/derived/src/interp.rs::array_proto_sort_via`, `Runtime::to_object`, and APS trajectory.
- **R** = Sort now wraps primitive receivers and returns the wrapper object. `Runtime::to_object` also boxes Symbol primitives with `%Symbol.prototype%`, matching the existing Boolean/Number/String/BigInt wrapper path.

## Risk Assessment

The Array.prototype method duplication is broader than sort: most methods still use `prototype::to_array_this`, whose BigInt/Symbol path is stale. This rung deliberately scopes the behavior change to sort, while making the minimal shared correction needed for Symbol `ToObject`.

Verification:

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `call-with-primitive.js`: PASS.
- Post-EPSUA 26-row sort target: 26 PASS / 0 FAIL.
- Full `built-ins/Array/prototype/sort/*.js` mirror: 50 PASS / 4 FAIL.

## Residuals

The APS target cluster is closed. Remaining sort-directory failures are resizable ArrayBuffer / typed-array-adjacent rows: `comparefn-grow.js`, `comparefn-shrink.js`, `comparefn-resizable-buffer.js`, and `resizable-buffer-default-comparator.js`.
