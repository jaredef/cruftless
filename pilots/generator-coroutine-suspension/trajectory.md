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

## GCS-EXT 2b - yield-boundary opcode (2026-05-29)

**Directive**: introduce the real yield boundary that can capture an active interpreter frame through `FrameSnapshot::from_frame`, while leaving generator lifecycle wiring and the existing eager generator call path deferred to EXT 2c.

### Substrate

Added `Op::Yield` as a zero-operand bytecode instruction and changed plain `yield` lowering from `LoadGlobal "__yield_push__"; Call 1` to `compile argument; Yield`.

The opcode has two execution modes:

- **legacy eager mode**: when no active generator object is installed, `Yield` appends the yielded value to the existing `Runtime::gen_yields_stack` array and pushes `undefined` as the yield-expression result. This preserves the current eager-collect generator behavior.
- **suspension mode**: when `Runtime::active_generator_for_yield` names a generator object, `Yield` pushes `undefined` as the later resume value placeholder, captures the active frame with `FrameSnapshot::from_frame`, stores it in the generator object's continuation slot, marks the generator `SuspendedYield`, records the yielded value, and returns the yielded value through `run_frame`.

`GeneratorObject` now carries the dormant continuation slot plus the last yielded value. The snapshot object-reference trace is threaded through `InternalKind::Generator` so the saved frame remains visible to future GC tracing.

### Exemplar

Added `interp::gcs_tests::yield_opcode_captures_active_generator_frame_snapshot`, which constructs a minimal frame containing `PushI32 42; Yield; ReturnUndef`, installs an active generator object, runs the frame, and verifies:

- the suspension channel returns `42`
- the generator state becomes `SuspendedYield`
- the generator stores a continuation snapshot
- the captured pc points after `Yield`
- the operand stack contains the `undefined` yield-expression resume placeholder

### Verification

- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 54 passed, 1 ignored.
- Legacy eager generator smoke: `function* g(){ yield 1; yield 2; }` still returns `1 2 true` through the existing `.next()` array-cursor path.

### Finding

**Finding GCS.3**: `Yield` can be introduced as a dual-mode opcode. EXT 2c should not need another compiler rewrite for plain `yield`; it needs lifecycle routing that constructs a real `GeneratorObject`, installs it as `active_generator_for_yield` while running/resuming the saved frame, and replaces the eager `call_function` generator branch.
