---
helmsman_session: helmsman-2026-05-29-principal
proposed_commits:
  - f00214efde7b46235e27802784811ace5326a300
target_branch: main
summary: GCS-EXT 2c - wire plain sync generators to GeneratorObject lifecycle
risk_class: substrate
gates_pre:
  test262_full: 67.6 (post-EPSUA interpreted matrix used for slice selection)
  test262_sample: post-EPSUA matrix external to this rung
  pass_gain: pending for-of/generator slice measurement
gates_post:
  build: cargo build --release --bin cruft -p cruftless PASS
  runtime_lib_tests: cargo test --release -p rusty-js-runtime --lib PASS
  forof_generator_slice: 34 PASS / 35 FAIL / 0 SKIP from 69 baseline FAIL rows
---

## Substrate Moves

GCS-EXT 2c replaces the eager collect-then-iterate path for plain sync generators with the dormant `GeneratorObject` and `FrameSnapshot` substrate from EXT 1, EXT 2a, and EXT 2b.

- **M** = plain sync generator lifecycle wiring.
- **T** = generator construction returns a suspended generator object; `.next()` resumes saved interpreter state until `Op::Yield` or completion.
- **I** = `new_generator_with_continuation`, object-local lifecycle methods, generator result objects, and the `call_function_inner` plain-generator routing split.
- **R** = construction is lazy, infinite generators no longer hang before first `.next()`, and ordinary `.next()` observes successive `yield` values.

## Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 56 passed, 1 ignored.
- CLI smoke PASS: `function* g(){ yield 1; yield 2; }` returns `1 false`, `2 false`, then `undefined true`; `function* inf(){ let i = 0; while (true) yield i++; }` returns `0`, then `1`.
- Post-EPSUA full-suite interpreted for-of/generator slice: 34 PASS / 35 FAIL / 0 SKIP from 69 baseline FAIL rows. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext2c-forof-generators-20260529T150231Z/summary.json`.

## Risk Assessment

The eager `gen_yields_stack` path remains for async/legacy generator paths, while plain sync generators return before eager body execution. `next(value)` still ignores the sent value because the suspended yield expression retains the EXT 2b `undefined` resume placeholder; `throw`, `return`, and `yield*` are conservative follow-on scaffolds. Remaining slice failures are expected around those deferred semantics plus iterator-close/destructuring behavior.

## Composes-With

- `generator-coroutine-suspension` locale.
- `async-generator-and-for-await-lowering`, once async generators can consume the same continuation substrate.
- Iterator-close and yield-delegation follow-ons that need abrupt completion injection into the suspended frame.

**APPROVED for push** per Helmsman GCS-EXT 2c same-turn directive.
