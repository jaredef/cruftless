---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - c9119619e07f1ecff011fc74b95ac08f094056f6
target_branch: main
summary: H262S-EXT 1 - guarded `$262.detachArrayBuffer` shim plus detached ArrayBuffer/DataView substrate
risk_class: substrate
gates_pre:
  test262_full: 67.6 (not re-measured this rung)
  test262_sample: 84.8 (not re-measured this rung)
  targets: 3/6 failing-or-partial in pre-land focused probe
gates_post:
  test262_full: 67.6 (not re-measured this rung)
  test262_sample: 84.8 (not re-measured this rung)
  targets: 6/6 approved focused probes PASS
  adjacent: 7/7 ArrayBuffer/DataView regression probes PASS
  build: cargo build --release --bin cruft -p cruftless PASS
---

## Substrate Moves

Single H262S-EXT 1 rung in `pilots/host-262-shim/`.

- **M** = test262 `$262.detachArrayBuffer` hook and the ArrayBuffer/DataView observer behavior required after host detachment.
- **T** = install `$262` only for test262 runner execution (`T262_TEST_PATH` present), expose only `detachArrayBuffer`, and terminate the hook in runtime-level detached-buffer state.
- **I** = `cruftless/src/test262_host.rs`; `cruftless/src/lib.rs`; `Runtime::detach_array_buffer`; ArrayBuffer detached/maxByteLength/resizable/byteLength observers; DataView constructor and byteLength detached guards; H262S seed/trajectory updates.
- **R** = extends H262S-EXT 0's partition of the runner-harness `$262` projection without taking up `createRealm`, IsHTMLDDA, agent hooks, or GC hooks.

## Verification

Approved focused probes all PASS under `target/release/cruft` + legacy test262 runner:

- `built-ins/ArrayBuffer/prototype/byteLength/detached-buffer.js`
- `built-ins/ArrayBuffer/prototype/detached/detached-buffer.js`
- `built-ins/DataView/detached-buffer.js`
- `built-ins/DataView/prototype/byteLength/detached-buffer.js`
- `harness/detachArrayBuffer.js`
- `harness/detachArrayBuffer-host-detachArrayBuffer.js`

Adjacent regression probes all PASS:

- `built-ins/ArrayBuffer/prototype/byteLength/return-bytelength.js`
- `built-ins/ArrayBuffer/prototype/byteLength/this-is-not-object.js`
- `built-ins/ArrayBuffer/prototype/detached/invoked-as-accessor.js`
- `built-ins/ArrayBuffer/prototype/detached/this-is-not-object.js`
- `built-ins/DataView/prototype/byteLength/return-bytelength.js`
- `built-ins/DataView/prototype/byteLength/this-is-not-object.js`
- `built-ins/DataView/prototype/byteLength/instance-has-detached-buffer.js`

Build gates:

- `cargo check -p cruftless -p rusty-js-runtime` PASS.
- `cargo build --release --bin cruft -p cruftless` PASS.

## Risk Assessment

Primary risk is leaking test262-only host surface into normal runtime execution. The shim is guarded behind `T262_TEST_PATH`, matching the legacy runner environment, and installs only the approved `detachArrayBuffer` method.

Secondary risk is over-broad detached semantics for resizable ArrayBuffer surfaces. The change only sets a runtime detached bit, clears buffer storage, reports detached observers, treats typed-array views as out-of-bounds, blocks resize, and applies DataView detached TypeErrors at the probed coordinates.

## Composes-With

- EPSUA parent arc.
- H262S-EXT 0 host-hook partition.
- Helmsman approval `h262s-ext-1-approval-resend-to-r1`.

**APPROVED for push** per Helmsman same-turn approval.
