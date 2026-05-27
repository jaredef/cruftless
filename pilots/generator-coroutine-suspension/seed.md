# generator-coroutine-suspension — Seed

## Substrate-pilot — compiler-tier locale for lazy generator suspension.

Per keeper directive 2026-05-27, spawned from the LPA-EXT 8/9 compiler gap analysis. Mechanism gap #3 from the diff-prod empirical cross-check. Highest-leverage single compiler candidate: unblocks 1,492 test262 rows (LPA-EXT 5 Arc B).

## Telos

Replace the eager-collect generator implementation with proper ECMA-262 §27.5 coroutine-style suspend/resume. The induced property: `yield` is a suspension point, `next()` resumes from that point, `next(val)` delivers a value back to the yield expression, and `throw(err)` resumes with an exception at the yield site.

## Current state

The compiler emits `is_generator: true` on `FunctionProto` (`compiler.rs:81`). `yield` lowers to a call into `__yield_push__` which appends to a thread-local vector (`compiler.rs:2982–2999`). `yield*` lowers to `__yield_delegate__` (`compiler.rs:3001–3010`). The runtime eagerly executes the entire generator body at construction time, collecting all yielded values, then returns an iterator over that array.

Observable deviations from spec:
- `next(val)`: sent value is ignored (yield expression always returns `undefined`)
- `throw(err)`: re-throws to caller instead of landing at the yield's try/catch
- Return value on terminal `{done:true}` step: discarded (body_result not captured)
- Side effects: all run at construction, not lazily on `.next()` calls
- Infinite generators: hang at construction (never terminate eager-collect)

## Constraints

- `compiler.rs:2982–3010` — yield/yield* lowering site
- `compiler.rs:74–81` — `is_generator` flag on FunctionProto
- Runtime's `call_function` path for generator-flagged closures
- Op set: may need new opcodes (e.g., `Yield`, `YieldDelegate`) or continuation-passing via existing frame stack

## Methodology

1. **Rung 1**: Design the suspension mechanism. Options: (a) Rust async/stackful coroutines, (b) CPS transform at bytecode level (save/restore frame state), (c) continuation-frame-per-yield (clone frame at each yield point). Choose based on composition with async generators.
2. **Rung 2**: Implement `GeneratorObject` with `[[GeneratorState]]` (suspendedStart, suspendedYield, executing, completed) per §27.5.3.
3. **Rung 3**: Wire `next(val)` to resume from yield, delivering the value.
4. **Rung 4**: Wire `throw(err)` to resume with exception at yield site.
5. **Rung 5**: Wire `return(val)` to complete the generator.
6. **Rung 6**: `yield*` delegation with inner generator close/throw forwarding.

## Composes-with

- `async-generator-and-for-await-lowering/` — async generators depend on core generator suspension
- `iterator-close-emission-sites/` — yield* delegation requires IteratorClose on break
- `iter-protocol-bytecode-rewrite/` — ForOfFastNext optimization must detect generator iterators

## Carve-outs

- Async generators: deferred to `async-generator-and-for-await-lowering/`
- Performance optimization: deferred to a successor locale after correctness lands

## Resume protocol

Read this seed (telos + constraints), then `trajectory.md` tail.

## Diff-prod anchors

| Fixture | Status | Connection |
|---|---|---|
| `generator-suspension` | FAIL (exit 70) | Crashes on bidirectional send / throw-into |
| `generators` | PASS | Existing fixture carves around eager-collect limitations |
| `iterator-close` | FAIL (exit 70) | yield* delegation close depends on suspension |
| `destructuring-iterators` | PASS | Iterator protocol works; generator suspension is the gap |
