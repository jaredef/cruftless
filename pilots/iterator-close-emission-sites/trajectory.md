# iterator-close-emission-sites — Trajectory

## ICES-EXT 0 — founding (2026-05-27)

**Trigger**: LPA-EXT 8/9 compiler gap analysis identified IteratorClose emission as mechanism gap #2. The opcode `Op::IterClose` (0xD2) exists but the compiler emits it only for destructuring partial consumption, not for for-of break/throw/return, yield* delegation, or spread-on-throw.

**Status**: FOUNDED. ICES-EXT 1 (for-of break IterClose emission) is the first substantive rung.

## ICES-EXT 1 — LANDED (2026-05-31) — for-of break IterClose emission at the bytecode compiler

**Trigger**: Direct chapter-close-inspect carry-forward from IPTD-EXT 1 (cells 3 + 6 of the original probe surfaced as newly-visible gaps once the OOM-on-non-conforming-iterator regression closed). Keeper APPROVED via Telegram 10676.

**Substrate** (~30 LOC in `pilots/rusty-js-bytecode/derived/src/compiler.rs`):

1. `LoopFrame` struct gains `for_of_iter_slot: Option<u16>`. Set to `Some(iter_slot)` at the for-of frame push (line 2272); `None` at the 5 other push sites (while, do-while, C-style for, labelled block, switch).

2. New compiler helper `emit_iter_close_call(iter_slot)`: stack-neutral; emits `LoadGlobal __destr_iter_close; LoadLocal iter_slot; Call 1; Pop`. Reuses the IPTD-EXT 1 helper-tier `__destr_iter_close` (TypeError on non-callable non-null/undefined return, silent on null/undefined, invoke on callable).

3. Unlabelled `Stmt::Break`: if target frame's `for_of_iter_slot` is `Some(slot)`, emit `emit_iter_close_call(slot)` after finalizers and before the exit Jump.

4. Labelled `Stmt::Break`: walk frames from current top down to target frame inclusive, collect `for_of_iter_slot`s in innermost-first order, emit a close call per slot after finalizers and before the exit Jump. Matches §14.7.5.6 step 5 + §13.15.7 abrupt-completion propagation across multiple crossed for-of frames.

**Yield**:

```text
Original 7-cell IPTD probe:                    5/7 -> 7/7 PASS
  Cell 3 iter.return=42 (non-callable) on break:  FAIL -> PASS (throws TypeError)
  Cell 6 iter.return=callable on break:           FAIL -> PASS (.return() invoked)

Cross-consumer 7-cell probe:                   7/7 PASS preserved
Labelled-break probe (outer/inner for-of):     ["B","A"]  (innermost-first close order verified)

Residuals (out of EXT 1 scope, confirmed by direct probe):
  throw inside for-of body:    close NOT called  (ICES-EXT 3 carry-forward)
  return inside for-of body:   close NOT called  (ICES-EXT 2 carry-forward)
```

**Phase 2 (Baseline-inspect)** per Rule 23: confirmed existing `emit_iter_close_if_not_done` helper at compiler.rs:3447 (destructuring-only, guards on done_slot). The new `emit_iter_close_call` is the unconditional sibling at the same site, factored for re-use across ICES rungs.

**Phase 5 (Chapter-close-inspect)** per Rule 15: residual probe confirms two gaps remain — `return` and `throw` inside for-of body. Both named at ICES seed §Methodology rungs 2 + 3. Carry-forward to ICES-EXT 2 (return path) + ICES-EXT 3 (throw path); each requires a different lowering surface (Return opcode walk + synthetic try-catch instrumentation respectively).

**Finding ICES.1**: the LoopFrame as the cross-frame IteratorClose anchor. Tracking a single optional iter slot per loop frame is sufficient for break (and will extend to return + throw) regardless of nesting depth. The labelled-break frame-walk is a 4-line filter_map + per-slot emit — the abstraction lives entirely in the frame stack with no auxiliary state. Candidate for cross-tier reuse (e.g., yield* delegation tracking in `generator-coroutine-suspension/`).

**Status**: ICES-EXT 1 LANDED. Plain-for-of break path now spec-correct at the IteratorClose surface across all helper-tier dispatch shapes. Return + throw paths carry forward to EXT 2 + EXT 3.

## ICES-EXT 2 — LANDED (2026-05-31) — for-of return IterClose emission

**Trigger**: Direct carry-forward from ICES-EXT 1 chapter-close (return-inside-for-of residual). Keeper APPROVED via Telegram 10678.

**Substrate** (~10 LOC at one site in `pilots/rusty-js-bytecode/derived/src/compiler.rs:1631-1647`): in the `Stmt::Return` arm, after the finalizer-emission block and before `Op::Return`, walk `self.loop_stack` in reverse, collect each frame's `for_of_iter_slot` where `Some`, emit a stack-neutral `emit_iter_close_call(slot)` per slot. Return value preservation: each close call is `LoadGlobal; LoadLocal; Call 1; Pop` — stack-neutral — so the return value pushed earlier stays at the operand-stack top through the close sequence and is consumed by `Op::Return` as before.

Per-function isolation is structural: `loop_stack` is fresh `Vec::new()` at every `compile_function_proto` (compiler.rs:5081), so all frames visible at the return site are within this function frame. Matches §14.7.5.6 step 5 + §13.15.7.

**Substrate prefix consumed unchanged**:
- `LoopFrame.for_of_iter_slot` (ICES-EXT 1)
- `emit_iter_close_call` helper (ICES-EXT 1)
- Helper-tier `__destr_iter_close` (IPTD-EXT 1)

Zero new helpers, zero new opcodes, zero LoopFrame changes.

**Yield**:

```text
ICES-EXT 2 probe (/tmp/probe-ices-ext-2.js): 6/6 PASS
  return inside for-of -> iter.return() called          ✓
  return crossing nested for-of -> close inner first    ✓ (["IN","OUT"])
  return value preserved through stack-neutral close    ✓ (got 42)
  return with non-callable iter.return -> TypeError     ✓
  plain return unaffected (positive control)            ✓
  return from while-loop -> no close (positive control) ✓

Regression sweep:
  Original 7-cell IPTD probe:                7/7 preserved
  Cross-consumer 7-cell probe:               7/7 preserved
  Labelled-break probe (innermost-first):    ["B","A"] preserved
```

**Phase 2 (Baseline-inspect)** per Rule 23: confirmed `loop_stack` is per-function (fresh at each `compile_function_proto`). No need for explicit function-frame boundary tracking; the stack walk naturally bounds at this function's first-pushed loop.

**Phase 3 (Pin-Art if duplicated)** per Rule 24: the EXT 2 emission pattern at Stmt::Return is structurally identical to EXT 1's labelled-break pattern — both walk a range of loop_stack frames and emit close per for-of frame innermost-first. Could be factored into a shared `emit_iter_closes_for_range` helper at EXT 3 if the throw path adds a third instance (Rule 24 threshold: 3+ duplicated sites with the same shape).

**Phase 4 (Revert-then-deeper-layer if negative)** per Rule 13: not invoked — positive on first probe.

**Phase 5 (Chapter-close-inspect)** per Rule 15: one residual remains — `throw` inside for-of body. Larger blast radius than break/return (needs synthetic try-catch wrapping the loop body to catch and emit close before re-throw). Domain of ICES-EXT 3.

**Finding ICES.1 empirical validation**: Finding ICES.1 (LoopFrame as cross-frame IteratorClose anchor) called for a single optional iter slot per frame to suffice across break, return, and throw paths regardless of nesting depth. EXT 2 lands as the predicted-and-confirmed first cross-path application. The break (target-frame range) and return (top-to-bottom range) emission patterns both consume the same primitive without auxiliary state. ICES-EXT 3 (throw) will be the third instance, completing the predicted abstraction. Two of three applications validated; recurrence threshold for findings-disposition lift-to-arc or standing-rule promotion approaches.

**Status**: ICES-EXT 2 LANDED. Return path now spec-correct. ICES-EXT 3 (throw inside for-of body) is the final residual within this locale's primary scope.
