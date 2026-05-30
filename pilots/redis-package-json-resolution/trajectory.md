# redis-package-json-resolution - Trajectory

## 2026-05-30 - R2 dot-directory package resolution

### Trigger

CAACP directive `fea20f83-53ae-48a1-824a-5a87cbad7a6f` from Helmsman targeted
the redis singleton residual from R4 Round 13b (`bfcee498`).

### Baseline

Recorded baseline from the sidecar top500 sample:

`dynamic import('redis') load failed: TypeError("package.json read failed at '/home/jaredef/Developer/cruftless-sidecar/results/citpt-ext-3-parity-sandbox/redis/node_modules/./package.json': No such file or directory ...")`

The failing stack points through:

`node_modules/@redis/client/dist/lib/client/pool.js:9:36`

That file contains `require(".")`. The package-local expectation is that `.`
resolves relative to the requiring file's directory, landing on
`@redis/client/dist/lib/client/index.js`, not a top-level `node_modules/.`
package.

### Substrate Move

`Runtime::resolve_module` and `Runtime::resolve_module_full` now classify `.`
and `..` as relative specifiers alongside `./` and `../`.

This keeps the fix at the resolver boundary:

- no package-specific redis shim;
- no package.json field-policy change;
- no change to conditional exports or main/module priority.

### Verification

- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime module::tests::resolve_module_treats_dot_as_relative_directory`: PASS.
- `cargo test --release -p rusty-js-runtime --lib`: PASS, 72 passed, 1 ignored.
- Redis sidecar smoke from
  `/home/jaredef/Developer/cruftless-sidecar/results/citpt-ext-3-parity-sandbox/redis`:
  `import("redis")` reaches populated namespace, `keyCount=58`.

### Residual

The redis smoke prints an additional unhandled promise rejection after the
namespace result:

`TypeError("callee is not callable: undefined [argc=1]")`

That is post-load asynchronous behavior and is not the package-json resolution
failure. The package namespace is populated before that residual appears.
