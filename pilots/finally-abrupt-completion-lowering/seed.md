# finally-abrupt-completion-lowering — Seed

## Substrate-pilot — compiler-tier locale for finally block execution on abrupt completion.

Per keeper directive 2026-05-27, spawned from the LPA-EXT 8/9 compiler gap analysis. Mechanism gap #5 from the diff-prod empirical cross-check.

## Telos

Ensure `finally` blocks execute when control leaves a `try` block via `break`, `continue`, `return`, or `throw` — not just on normal completion. Per ECMA-262 §14.15.3 (TryStatement runtime semantics), the finally block must run regardless of how the try/catch body completes.

## Current state

The compiler emits `TryEnter`/`TryExit` around the try block (`compiler.rs:1981–1986`). `TryExit` is emitted inline after the try body's normal completion. However, `break`, `continue`, and `return` statements that exit a try block jump directly to their target without first emitting `TryExit`, bypassing the finally block entirely.

Observable deviations from spec:
- `try { if (i===1) break; } finally { log.push("finally"); }` — finally block skipped on `break`
- `try { if (i===1) continue; } finally { log.push("finally"); }` — finally block skipped on `continue`
- `try { return "try"; } finally { return "finally"; }` — finally return does override try return (this works because the compiler emits the finally block after TryExit on normal path)
- Nested try-finally: inner finally runs on break but outer may not

## Constraints

- `compiler.rs:1975–2014` — try/catch/finally compilation
- `compiler.rs:2050–2070` — break/continue emission (jumps directly to loop target)
- `compiler.rs:72` — Return opcode emission
- The compiler's `loop_stack` tracks break/continue targets but does not track enclosing try blocks

## Methodology

1. **Rung 1**: Add a try-block stack to the compiler. Each entry records the finally-block bytecode range (or a pending-finally flag). When emitting `break`, `continue`, or `return`, walk the try-block stack and emit `TryExit` + inline-finally for each enclosing try block before the jump/return.
2. **Rung 2**: Handle nested try-finally (inner finally must complete before outer finally).
3. **Rung 3**: Handle `return` in finally overriding `return` in try (completion value replacement).
4. **Rung 4**: Handle `throw` in finally overriding `throw`/`return` in try.

## Composes-with

- `var-hoisting-through-try-block/` — already closed; try-block scope interactions
- `generator-coroutine-suspension/` — generator return/throw must also trigger finally
- `iterator-close-emission-sites/` — for-of break inside try-finally needs both finally and IterClose

## Carve-outs

- `for-await-of` inside try-finally: deferred to `async-generator-and-for-await-lowering/`

## Resume protocol

Read this seed (telos + constraints), then `trajectory.md` tail.

## Diff-prod anchors

| Fixture | Status | Connection |
|---|---|---|
| `finally-return-override` | FAIL | `break`/`continue` in loop with try-finally skip the finally block |
| `labeled-control-flow` | PASS | Labeled break/continue work (no try-finally interaction tested) |
| `switch-fallthrough` | PASS | Switch control flow works without try-finally |
