# generator-coroutine-suspension — Trajectory

## GCS-EXT 0 — founding (2026-05-27)

**Trigger**: LPA-EXT 8/9 compiler gap analysis identified generator eager-collect as mechanism gap #3, the highest-leverage single compiler candidate (unblocks 1,492 test262 rows in LPA-EXT 5 Arc B).

**Status**: FOUNDED. GCS-EXT 1 (suspension mechanism design) is the first substantive rung.

## GCS-EXT 1 - suspension design + scaffold (2026-05-29)

**Directive**: helmsman assigned R1 to choose the generator suspension design path, land a minimal state scaffold, and keep observable eager-collect behavior unchanged until later rungs wire resume semantics.

### Design Choice

Chosen path: **(b) bytecode-level CPS / save-restore frame state**.

Rationale:

- Async generators compose with a resumable interpreter frame plus queue/promise mediation. The same saved continuation can be resumed by sync `next` or by the async-generator request queue, while Rust async/stackful coroutines would bifurcate the runtime execution model.
- The current interpreter already owns explicit `Frame` state (`pc`, locals, operand stack, try stack, with-env stack, this/new.target, and upvalues). Capturing that shape is a direct extension of the existing resolver instance.
- Continuation-frame-per-yield remains useful as a probing fallback, but cloning at each yield point risks incorrect aliasing for mutable locals/upvalues and try-stack state. Save/restore of the active frame is the cleaner substrate for `next(value)`, `throw(value)`, `return(value)`, and `yield*` close/throw forwarding.
- Rust async/stackful coroutines would force the JS VM to express every interpreter suspension through host-language control flow, complicating GC tracing, JIT/deopt boundaries, and async generator integration.

### Scaffold Landed

- Added `GeneratorState` with `SuspendedStart`, `SuspendedYield`, `Executing`, and `Completed`.
- Added `GeneratorObject` carrying the state cell.
- Added an `InternalKind::Generator` variant and kind-name reporting.
- Added runtime scaffold entry points: `new_generator_scaffold`, `generator_next_scaffold`, `generator_throw_scaffold`, and `generator_return_scaffold`.

The scaffold is intentionally dormant. Existing generator instances still use the eager-collected `__gen_*` sentinel path, so no PASS gain is expected in this rung and existing generator exemplars should remain stable.

### Next Rungs

- GCS-EXT 2: introduce the saved-frame representation and route generator construction to `GeneratorObject` without changing resume behavior.
- GCS-EXT 3: wire `next(value)` resume and sent-value delivery to the suspended `yield` expression.
- GCS-EXT 4: wire `throw(value)` into the suspended frame and try/catch machinery.
- GCS-EXT 5: wire `return(value)` completion.
- GCS-EXT 6: `yield*` delegation and IteratorClose forwarding.

### Finding

**Finding GCS.1**: the existing interpreter `Frame` already contains the continuation boundary GCS needs. The next substrate should preserve and resume `Frame` state rather than model generator suspension as host Rust coroutine control flow.

## GCS-EXT 2a - owned FrameSnapshot substrate (2026-05-29)

**Directive**: after the EXT 2 lifecycle wiring blocker, keeper/helmsman rescoped the next rung to the prerequisite owned continuation type only. No yield opcode, `__yield_push__` rewrite, lifecycle flip, or eager-generator behavior change is in scope.

### Substrate

Added `FrameSnapshot`, an owned continuation payload that can capture the parts of `Frame<'a>` needed by future generator suspension:

- function identity (`Option<Rc<FunctionProto>>`)
- bytecode and constant pool
- source maps, line starts, source URL, construct tags
- local/upvalue descriptor metadata
- locals, cell-backed locals, operand stack
- program counter, try stack, with-environment stack
- `this`, `this_cell`, derived initial this, upvalues
- method-call diagnostics (`last_property_lookup`, `pending_method_name`, `pending_method_getprop_pc`)
- private home, import.meta, new.target
- arrow/strict/param/eval-var-env metadata

`FrameSnapshot::from_frame` captures a borrowed frame into owned storage. `impl From<&FrameSnapshot> for Frame<'_>` restores a borrowed execution frame view over the snapshot's owned metadata while cloning the mutable execution vectors back into the active frame.

### Clone Notes

Straightforward clone fields: bytecode, constants, source metadata, locals, operand stack, try stack, with-env stack, diagnostics, private/import/new-target metadata.

Careful clone fields: `local_cells`, `this_cell`, and `upvalues` intentionally clone the `Rc<RefCell<Value>>` handles, not the pointed values. This preserves binding identity across a suspended frame and already-created closures, which is required for generator resumption to observe shared mutable bindings correctly.

Not captured yet: per-frame JIT/OSR caches and IC dispatch caches. They are optimization state, not semantic continuation state, and can be cold on resume in the first correctness implementation.

### Finding

**Finding GCS.2**: the owned continuation boundary can be introduced without touching generator behavior. The only semantic-sensitive clone boundary is cell-backed binding identity; cache state can be discarded at suspend/resume without changing observable JS behavior.
