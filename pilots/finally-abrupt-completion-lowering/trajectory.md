# finally-abrupt-completion-lowering — Trajectory

## FACL-EXT 0 — founding (2026-05-27)

**Trigger**: LPA-EXT 8/9 compiler gap analysis identified finally-on-abrupt-loop-exit as mechanism gap #5. The compiler's TryEnter/TryExit does not account for break/continue/return crossing a try boundary.

**Status**: FOUNDED. FACL-EXT 1 (try-block stack + break/continue TryExit emission) is the first substantive rung.

---

## FACL-EXT 1 — Finally blocks execute on break, continue, and return (2026-05-27)

**Trigger**: First substantive rung per the seed's methodology.

**Changes** (`pilots/rusty-js-bytecode/derived/src/compiler.rs`):

1. Added `try_depth: u32` and `pending_finalizers: Vec<Stmt>` to `LoopFrame`. Tracks active try-finally blocks between loop entry and current position.
2. Added `fn_finalizer_stack: Vec<Stmt>` and `in_finalizer_emission: bool` to the Compiler struct. Tracks function-level finally blocks for return statements.
3. At TryEnter inside a loop body: increment try_depth, push finalizer to both loop-frame and function-level stacks.
4. At TryExit (normal path): pop from both stacks after the catch handler and normal-path finalizer.
5. Break/continue: clone pending finalizers, emit TryExit + inline-compiled finalizer body for each, then emit the jump. Guarded by `in_finalizer_emission` to prevent infinite recursion when a finally body contains break/continue.
6. Return: same pattern — clone fn_finalizer_stack, emit TryExit + inline finally bodies before the Return opcode. Same recursion guard.

**Diff-prod results**:

11 of 12 finally-return-override test cases now match Bun:
- `break_finally`: PASS (finally runs on break in loop)
- `continue_finally`: PASS (finally runs on continue in loop)
- `override_try`: PASS (return in finally overrides return in try)
- `override_catch`: PASS (return in finally overrides return in catch)
- `override_throw`: PASS (return in finally overrides throw in try)
- `throw_override`: PASS (throw in finally overrides throw in try)
- `throw_overrides_return`: PASS (throw in finally overrides return in try)
- `nested`: PASS (inner and outer finally both run on return)
- `mutate_in_finally`: PASS (object mutation in finally visible after return)
- `passthrough`: PASS (try's return propagates through no-op finally)
- `throw_passthrough`: **FAIL** (throw through no-op finally returns null instead of propagating the error — runtime exception-handler issue, not compiler emission)

No regressions on adjacent fixtures (template-literals, generators, labeled-control-flow, switch-fallthrough, closures-scopes, hoisting-semantics, expression-precedence all PASS).

**Finding FACL.1 (inline finally duplication is correct but not size-optimal)**: the v1 approach duplicates the finally body at every break/continue/return site that crosses a try boundary. For deeply nested try-finally with multiple exit points this produces bytecode bloat. A successor approach could use a shared finally subroutine (a synthetic closure or a jump-to-shared-finally pattern). The duplication is correct and the bytecode size increase is negligible for typical code.

**Status**: FACL-EXT 1 CLOSED for break/continue/return. The `throw_passthrough` residual is a runtime exception-propagation issue (the caught exception value is lost when the exception handler runs the finally body on the throw path) — deferred to FACL-EXT 2.
