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

## GCS-EXT 2c - plain generator lifecycle wiring (2026-05-29)

**Directive**: replace the eager collect-then-iterate path for plain sync generators with the `GeneratorObject` + `FrameSnapshot` lifecycle introduced by EXT 1, EXT 2a, and EXT 2b. `next(value)`, `throw(value)`, `return(value)`, async generators, and `yield*` remain follow-on scope.

### Substrate

Plain generator calls now construct a `GeneratorObject` in `SuspendedStart` with an initial `FrameSnapshot` instead of executing the body at construction time. The old `gen_yields_stack` path remains for async/legacy generator paths that still execute eagerly, and this rung conservatively leaves `yield*` bodies on that legacy path because delegation semantics are explicitly EXT 6 scope.

`Generator.prototype.next` resumes the saved snapshot under `active_generator_for_yield`, transitions `Executing -> SuspendedYield` when `Op::Yield` captures the frame, and returns ordinary `{ value, done }` result objects. Completion clears the continuation and moves the generator to `Completed`.

The lifecycle methods are installed directly on the generated object for this rung: `next`, `return`, `throw`, and `@@iterator`. `return` and `throw` are conservative terminal scaffolds pending the later rungs that feed values/exceptions through the suspended frame.

### Exemplar

Added runtime-library tests for:

- lazy execution across `g().next()` calls, including a side-effect trace proving construction does not execute the body
- infinite generator construction, proving `function* inf(){ while(true) yield i++; }` no longer hangs before the first `next()`

### Verification

- Focused GCS tests: `cargo test --release -p rusty-js-runtime --lib gcs_tests -- --nocapture` PASS.
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 56 passed, 1 ignored.
- CLI smoke: `function* g(){ yield 1; yield 2; }` returns `1 false`, `2 false`, then `undefined true`; `function* inf(){ let i = 0; while (true) yield i++; }` returns `0`, then `1`, without hanging at construction.
- Post-EPSUA for-of/generator slice measurement: 34 PASS / 35 FAIL / 0 SKIP from 69 baseline FAIL rows. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext2c-forof-generators-20260529T150231Z/summary.json`. The gain is partial because `next(value)`, `throw`, `return`, `yield*`, and destructuring iterator-close semantics remain deferred.

### Finding

**Finding GCS.4**: the initial generator suspension boundary is enough to make plain `.next()` lazy and finite/infinite generators resumable, but the next semantic wall is sent-value/abrupt-completion injection. EXT 3 should wire `next(value)` into the suspended `yield` expression before `throw`, `return`, or `yield*` delegation.

## GCS-EXT 3 - next(value) sent-value injection (2026-05-29)

**Directive**: deliver `Generator.prototype.next(value)`'s argument into the suspended `yield` expression. `throw(err)`, `return(value)`, and `yield*` delegation remain follow-on scope.

### Substrate

`Generator.prototype.next` now distinguishes `SuspendedStart` from `SuspendedYield` before transitioning the generator to `Executing`.

On `SuspendedYield` resume, the runtime mutates the saved `FrameSnapshot` before reconstructing the active `Frame`: the top operand-stack slot, which `Op::Yield` installed as the yield-expression resume placeholder, is overwritten with the `next(value)` argument. If the saved stack is unexpectedly empty, the value is pushed as a conservative fallback.

First `.next(value)` from `SuspendedStart` still ignores its argument, matching the generator-start semantics and preserving EXT 2c's construction behavior.

### Exemplar

Added `interp::gcs_tests::generator_next_value_resumes_yield_expression`:

```js
function* g() {
  const x = yield 1;
  return x + 1;
}
const it = g();
it.next(7);   // first argument ignored
it.next(42);  // terminal result value is 43
```

### Verification

- Focused GCS tests: `cargo test --release -p rusty-js-runtime --lib interp::gcs_tests -- --nocapture` PASS: 4 passed.
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 57 passed, 1 ignored.
- Post-EXT 2c for-of/generator slice measurement: 46 PASS / 23 FAIL / 0 SKIP from the same 69-row slice, +12 PASS over EXT 2c. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext3-forof-generators-20260529T152014Z/summary.json`.

### Finding

**Finding GCS.5**: `Op::Yield`'s placeholder stack slot is the correct injection point for `next(value)`. No opcode rewrite is needed for sent-value delivery; the saved continuation boundary can be patched before frame restore. The remaining generator wall is abrupt-completion injection (`throw`, `return`) plus `yield*` delegation and iterator-close forwarding.

## GCS-EXT 4 - throw resume-with-exception (2026-05-29)

**Directive**: implement `Generator.prototype.throw(err)` for SuspendedYield generators by raising the value at the suspended yield-expression site. `return(value)` and `yield*` delegation remain follow-on scope.

### Substrate

`Generator.prototype.throw` now resumes only `SuspendedYield` generators. It takes the saved `FrameSnapshot`, restores a `Frame`, and injects the thrown value through the same catch-entry shape used by `Runtime::run_frame`: pop the top `TryFrame`, truncate the operand stack to `sp_at_entry`, push the thrown value, and set `pc` to the catch handler offset.

If the suspended yield has no active try frame, the thrown value propagates out of `gen.throw()` and the generator is completed. If the catch handler yields, the generator transitions back to `SuspendedYield` through the existing `Op::Yield` path and `throw()` returns `{ value, done:false }`.

### Exemplar

Added runtime-library coverage for:

```js
function* g() {
  try {
    yield 1;
  } catch (e) {
    yield e + "!";
  }
}
const it = g();
it.next();
it.throw("oops"); // { value:"oops!", done:false }
```

Also added an uncaught throw exemplar proving `gen.throw("boom")` propagates to the caller and completes the generator.

### Verification

- Focused GCS tests: `cargo test --release -p rusty-js-runtime --lib interp::gcs_tests -- --nocapture` PASS: 6 passed.
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 59 passed, 1 ignored.
- Post-EXT 3 for-of/generator slice measurement: 46 PASS / 23 FAIL / 0 SKIP from the same 69-row slice, +0 PASS over EXT 3. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext4-forof-generators-20260529T153213Z/summary.json`.

### Finding

**Finding GCS.6**: throw-resume can reuse the interpreter's existing try-stack substrate by entering the catch handler before the restored frame is run. The measured for-of/generator residual is not primarily blocked on catch-resume; the next expected walls are `return(value)`, `yield*`, IteratorClose forwarding, and destructuring interactions.

## GCS-EXT 5 - return early termination (2026-05-29)

**Directive**: implement `Generator.prototype.return(value)` for SuspendedYield generators, including the `try/finally` cleanup-yield shape where `return()` first yields the finally value and the following `next()` completes with the requested return value.

### Substrate

`GeneratorObject` now carries an optional `pending_return` value. `Generator.prototype.return` stores that value when resuming a SuspendedYield continuation through an active try/finally handler, and the normal `.next()` resume path preserves it across a cleanup `yield`.

The return-resume path restores the saved `FrameSnapshot`, enters the active try handler without pushing a thrown value, and runs the frame under `active_generator_for_yield`. If the handler yields, `return()` returns `{ value, done:false }`. When execution later completes without another yield, the pending return value supersedes the fallthrough completion and the generator is marked completed. If no active try frame is present, `return(value)` completes immediately with `{ value, done:true }`.

`pending_return` is traced when it contains an object so GC keeps object-valued return payloads live while the finally block is suspended.

### Exemplar

Added runtime-library coverage for:

```js
function* g() {
  try {
    yield 1;
  } finally {
    yield "cleanup";
  }
}
const it = g();
it.next();          // { value:1, done:false }
it.return("done"); // { value:"cleanup", done:false }
it.next();          // { value:"done", done:true }
```

Also added the no-finally terminal path proving a suspended generator without a handler completes immediately and remains completed on the following `.next()`.

### Verification

- Focused GCS tests: `cargo test --release -p rusty-js-runtime --lib interp::gcs_tests -- --nocapture` PASS: 8 passed.
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 61 passed, 1 ignored.
- `built-ins/GeneratorPrototype/return/*` slice measurement: 13 PASS / 10 FAIL from 23 tests. Baseline from `/home/jaredef/Developer/cruftless-sidecar/results/test262-full-2026-05-28-123833-p2/results.jsonl` was 4 PASS / 19 FAIL on the same paths, for +9 PASS. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext5-generator-return-20260529T154741Z/summary.json`.

### Finding

**Finding GCS.7**: early `return(value)` needs a distinct pending-completion slot on the generator object because cleanup `yield`s split the return completion across two external calls. The residual return slice is now concentrated in descriptor metadata, nested try/catch/finally discrimination, and finally-return override semantics; those require a richer abrupt-completion record than the current try-stack-only handler entry.

## GCS-EXT 6 - yield-star delegation (2026-05-29)

**Directive**: replace the legacy `__yield_delegate__` eager-drain path with a real generator-lifecycle delegation mechanism for `yield*`.

### Substrate

The bytecode alphabet now has `Op::YieldDelegate`. The compiler lowers `yield* expr` by evaluating the delegate expression and emitting that opcode, rather than calling the old `__yield_delegate__` helper. Plain sync generator calls no longer exclude function protos containing `yield*`, so delegated generators enter the same SuspendedStart/SuspendedYield lifecycle as the EXT 2c generator substrate.

`GeneratorObject` now carries an optional `GeneratorDelegate` record containing the active delegate iterator and its `next` method. `Op::YieldDelegate` creates this record on first entry, calls delegate `next()`, and either pushes the delegate's completion value into the outer frame or captures a continuation at the opcode pc and yields the delegate value to the outer caller. On resume, `Generator.prototype.next(value)` overwrites the saved placeholder slot; `Op::YieldDelegate` reads that value and forwards it to the delegate `next(value)` call.

The delegate record is traced by GC so the iterator and next method remain live while the outer generator is suspended inside `yield*`.

### Exemplar

Added runtime-library coverage for:

```js
function* g() {
  yield* [1, 2, 3];
}
```

which now lazily returns `1`, `2`, `3`, then `undefined done:true`.

Also added nested-generator return propagation:

```js
function* inner() { yield "a"; return "b"; }
function* outer() { return yield* inner(); }
```

where the second `outer().next()` completes with value `"b"`.

### Verification

- Focused GCS tests: `cargo test --release -p rusty-js-runtime --lib interp::gcs_tests -- --nocapture` PASS: 10 passed.
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 63 passed, 1 ignored.
- For-of/generator slice measurement: 50 PASS / 19 FAIL from the same 69-row slice. Baseline from `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext4-forof-generators-20260529T153213Z/results.jsonl` was 46 PASS / 23 FAIL, for +4 PASS. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-ext6-forof-generators-20260529T160327Z/summary.json`.

### Finding

**Finding GCS.8**: `yield*` needs an opcode-owned delegation loop because the frame must resume at the delegation site, not after it, until the inner iterator completes. The saved-frame pc rewrite to the opcode site plus a generator-owned delegate record is enough for lazy array delegation, sent-value forwarding via `next(value)`, and inner generator return-value propagation. The remaining slice residuals are now outside the basic delegate pump: abrupt forwarding through delegate `throw`/`return`, IteratorClose, and destructuring iterator-close interactions.

## GCS residuals - delegate abrupt forwarding (2026-05-29)

**Directive**: close the consolidated GCS residuals if tractable, with explicit scope-down permission to land the delegate-abrupt path first if the four residual sub-items balloon.

### Substrate

This rung takes the scope-down path and closes delegate-abrupt forwarding through `yield*`.

When `Generator.prototype.throw(value)` reaches a suspended `yield*` site, the outer generator now checks the active `GeneratorDelegate` before injecting into the outer frame. If the inner iterator has a callable `throw`, the runtime calls it with the thrown value and interprets the returned iterator result. A non-done result becomes the outer `throw()` result and keeps the outer generator suspended at the same delegate site. A done result clears the delegate and resumes the outer frame after `Op::YieldDelegate` with the inner completion value. If the inner iterator has no `throw`, the runtime performs IteratorClose through the delegate `return` method when present, then rethrows the original value and completes the outer generator.

When `Generator.prototype.return(value)` reaches a suspended `yield*` site, the outer generator now forwards to the inner iterator's `return(value)` when present. A non-done result is yielded by the outer `return()` call while preserving the active delegate. A done result completes the outer generator with the inner return result's value. If the inner iterator has no `return`, the outer generator completes immediately with the requested return value.

The attempted compiler-side expansion for generic for-of IteratorClose on abrupt break/continue/return/throw was reverted before commit: it regressed the 69-row for-of/generator slice by one. That residual needs a separate compiler-control-flow rung rather than being bundled into the delegate-abrupt runtime path.

### Exemplar

Added runtime-library coverage for:

```js
function* inner() {
  try { yield "a"; } catch (e) { yield "caught:" + e; }
}
function* outer() { yield* inner(); }
const it = outer();
it.next();          // { value:"a", done:false }
it.throw("boom");  // { value:"caught:boom", done:false }
```

and for:

```js
function* inner() {
  try { yield "a"; } finally { yield "cleanup"; }
}
function* outer() { yield* inner(); }
const it = outer();
it.next();          // { value:"a", done:false }
it.return("done"); // { value:"cleanup", done:false }
it.next();          // { value:"done", done:true }
```

### Verification

- Focused GCS tests: `cargo test --release -p rusty-js-runtime --lib interp::gcs_tests -- --nocapture` PASS: 12 passed.
- `cargo build --release --bin cruft -p cruftless` PASS.
- `cargo test --release -p rusty-js-runtime --lib` PASS: 65 passed, 1 ignored.
- For-of/generator slice: 50 PASS / 19 FAIL from the 69-row slice, +0 over GCS-EXT 6. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/gcs-residuals-delegate-abrupt-20260529T163839Z/summary.json`.
- `built-ins/GeneratorPrototype/return/*`: 13 PASS / 10 FAIL from 23 tests, +0 over GCS-EXT 5.
- Delegate-related sync `yield-star` test262 slice: 2 PASS / 2 FAIL from 4 tests; remaining failures are ASI syntax rows, not delegate abrupt forwarding.

### Finding

**Finding GCS.9**: delegate abrupt forwarding belongs at the generator lifecycle method boundary, before frame-level throw/return injection. The active delegate record is the discriminant: if present, `throw`/`return` target the inner iterator protocol first; only after the delegate reports done does control return to the outer frame. Generic for-of IteratorClose and destructuring-close residuals are compiler-control-flow closures and should land as a follow-up, because naive loop-exit close emission can regress the current for-of/generator slice.
