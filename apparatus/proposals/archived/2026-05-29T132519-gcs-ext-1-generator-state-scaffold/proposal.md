---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - e012bc0e7b586b0b9fb238783170fc33f9ebbcef
target_branch: main
summary: GCS-EXT 1 - choose bytecode CPS/save-restore design and scaffold generator state
risk_class: substrate
gates_pre:
  test262_full: 67.6 (not re-measured this rung)
  test262_sample: post-EPSUA measurement external to this rung
  pass_gain: none expected
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  runtime_integration_tests: blocked by pre-existing Runtime.globals test references
  diff_prod_generators: blocked by missing bun oracle in r1 worktree environment
---

## Substrate Moves

Single GCS-EXT 1 scaffold rung in `pilots/generator-coroutine-suspension/`.

- **M** = generator coroutine suspension state surface, before observable resume behavior changes.
- **T** = choose bytecode-level CPS / save-restore of interpreter `Frame` state over Rust async/stackful coroutines or continuation-clone-per-yield.
- **I** = `GeneratorState`, `GeneratorObject`, `InternalKind::Generator`, dormant runtime scaffold constructors and `next`/`throw`/`return` entry points, plus GCS trajectory finding.
- **R** = prepares the later generator suspension rungs while leaving the current eager-collect generator path intact.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 53 passed, 1 ignored.
- `cargo test --release -p rusty-js-runtime` is blocked before test execution by existing integration tests referencing the removed `Runtime.globals` field.
- `scripts/diff-prod/run.sh generators` is blocked by missing `bun` in this worktree environment; the Cruft side executed, but the Bun oracle exited 127.

## Risk Assessment

The scaffold is dormant and does not replace the existing `__gen_*` eager-collect generator path. Primary risk is type-surface churn in `InternalKind`; build and library tests confirm the enum addition compiles through the runtime. The next implementation rung must not wire `GeneratorObject` until it can preserve enough `Frame` state to avoid regressing current finite generator behavior.

## Composes-With

- `generator-coroutine-suspension` locale.
- `async-generator-and-for-await-lowering`, which depends on a resumable generator continuation.
- `iterator-close-emission-sites`, because `yield*` eventually needs close/throw forwarding.

**APPROVED for push** per Helmsman R1 same-turn directive.
