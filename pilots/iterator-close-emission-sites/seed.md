# iterator-close-emission-sites — Seed

## Substrate-pilot — compiler-tier locale for IteratorClose emission at all consuming sites.

Per keeper directive 2026-05-27, spawned from the LPA-EXT 8/9 compiler gap analysis. Mechanism gap #2 from the diff-prod empirical cross-check. Scope-companion to `iterator-close-on-abrupt/` (which covers the runtime IteratorClose abstract operation).

## Telos

Emit `Op::IterClose` (opcode 0xD2) at every consuming site where the spec requires IteratorClose per §7.4.7: for-of `break`, for-of `throw`, for-of `return`, destructuring partial consumption, `yield*` delegation close, and `Array.from` mapFn throw. The opcode exists; the compiler does not generate it at the required sites.

## Current state

The compiler emits IterClose in exactly one context: destructuring array patterns via `emit_iter_close_if_not_done` (`compiler.rs:2628–2652`), which calls `__destr_iter_close(iter_slot)`. This covers partial destructuring consumption.

Missing emission sites:
- **for-of break**: `compile_for_of` emits `IterInit`/`IterNext` but does not emit `IterClose` on break exit (`compiler.rs:1680–1720`)
- **for-of throw**: no IterClose on exception during loop body
- **for-of return**: no IterClose on `return` inside for-of body
- **for-of normal exhaustion**: does not call `.return()` (correct per spec — normal completion does not close)
- **yield\* delegation**: `__yield_delegate__` does not forward `.return()` or `.throw()` to the inner iterator
- **spread in call/array**: `[...iter]` does not close on throw during iteration

## Constraints

- `compiler.rs:2628–2652` — existing `emit_iter_close_if_not_done` helper (destructuring only)
- `compiler.rs:1680–1720` — for-of compilation
- `compiler.rs:2982–3010` — yield/yield* compilation
- `op.rs:222` — `Op::IterClose = 0xD2` (exists, underused)
- Runtime: `Op::IterClose` handler must call `iter.return()` if it exists

## Methodology

1. **Rung 1**: for-of break. Track the iterator slot in the loop stack. When emitting `break` that exits a for-of loop, emit `IterClose` before the jump. Must compose with `finally-abrupt-completion-lowering/` (if the for-of is inside a try block, both IterClose and TryExit are needed).
2. **Rung 2**: for-of throw. Wrap the loop body in a synthetic try-catch that calls IterClose on exception before re-throwing.
3. **Rung 3**: for-of return. Same pattern as break: emit IterClose before the Return opcode.
4. **Rung 4**: yield* delegation close. Requires `generator-coroutine-suspension/` to land first; the delegating generator must forward `.return()` and `.throw()` to the inner iterator.
5. **Rung 5**: spread/Array.from close on throw. Lower-priority; covers edge cases in `[...iter]` and `Array.from(iter, mapFn)`.

## Composes-with

- `iterator-close-on-abrupt/` — runtime-side IteratorClose abstract operation (existing locale)
- `finally-abrupt-completion-lowering/` — break/return inside try-finally inside for-of needs both
- `generator-coroutine-suspension/` — yield* delegation close depends on suspension
- `iter-protocol-bytecode-rewrite/` — ForOfFastNext fast-path must bypass IterClose for exhausted arrays

## Carve-outs

- for-await-of close: deferred to `async-generator-and-for-await-lowering/`
- `.return()` throwing: error propagation from `.return()` itself is runtime-tier

## Resume protocol

Read this seed, then `trajectory.md` tail.

## Diff-prod anchors

| Fixture | Status | Connection |
|---|---|---|
| `iterator-close` | FAIL (exit 70) | for-of break/throw/return do not call .return(); crashes |
| `destructuring-iterators` | PASS | Destructuring partial close works (existing emit_iter_close_if_not_done) |
| `for-in-for-of-lowering` | FAIL | for-of with delete during iteration; adjacent surface |
| `iteration-protocol` | PASS | Basic iteration protocol works |
