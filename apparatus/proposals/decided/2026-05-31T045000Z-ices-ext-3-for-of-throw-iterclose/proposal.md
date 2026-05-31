---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "ICES-EXT 3: emit IteratorClose at for-of body-throw via synthetic Op::TryEnter/Op::TryExit wrapping the user body, with a per-loop catch stub that spills the thrown value, calls __destr_iter_close, reloads, and re-throws. Closes the last residual within the ICES primary scope. Adds LoopFrame.for_of_body_try_open tracking so break / continue / return paths emit matching Op::TryExit before their control-flow jumps; per-frame TryExit + IteratorClose unwind semantics now uniform across all three abrupt-completion shapes."
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: 88.7%
  diff_prod: 65 / 47
gates_post:
  build: "cargo build --release --bin cruft -p cruftless"
  cargo_test_runtime_lib: "74 / 0 / 1 (preserved)"
  probe_cells:
    - "throw in body -> iter.return() called"
    - "nested for-of throw -> close inner first then outer"
    - "thrown value preserved through close"
    - "non-callable iter.return -> close-throw (TypeError) replaces body-throw"
    - "outer try-catch sees close happen before catch handler runs"
    - "break-close + return-close regression preserved"
    - "continue keeps iterating across n iterations (synthetic try/exit-entry balanced)"
---

## Substrate

Carry-forward from ICES-EXT 2 chapter-close — the last residual within the ICES primary scope (for-of body throw not invoking IteratorClose).

### LoopFrame extension

Add `for_of_body_try_open: bool` to `LoopFrame`. Set `true` while compiling the for-of body inside the synthetic try wrap; reset to `false` after `compile_stmt(body)`.

### For-of body wrap (~30 LOC at one site)

After all bind setup and before `compile_stmt(body)`:
1. Emit `Op::TryEnter <catch_off_patch>`.
2. Set `for_of_body_try_open = true`.
3. `compile_stmt(body)?`.
4. Reset `for_of_body_try_open = false`.
5. Emit `Op::TryExit` + `emit_back_jump(loop_start)`.

After the back-jump and before the `j_done` patch, emit the catch stub:
6. Spill thrown value to a fresh temp (`<for-of.body.throw>`).
7. `emit_iter_close_call(iter_slot)` — reuses the IPTD-EXT 1 helper-tier `__destr_iter_close`. If close itself throws (non-callable iter.return), that throw replaces the body's per §14.7.5.6 step 5 + §13.15.7 `IfAbruptCloseIterator`.
8. Reload thrown value, emit `Op::Throw`.

The catch stub is unreachable from fall-through (the back-jump above branches unconditionally to loop_start); control reaches it only via the runtime's catch-dispatch.

### Per-frame TryExit unwind in break / continue / return

Break (unlabelled / labelled), continue (unlabelled / labelled), and return all now emit `Op::TryExit` before `emit_iter_close_call` per crossed for-of frame whose `for_of_body_try_open` is `true`. This keeps the runtime's per-call `frame.try_stack` balanced across all abrupt-completion shapes, so subsequent throws in the same function frame don't accidentally land in an orphaned synthetic catch.

Per-frame semantics:
- **break** (target excluded for unlabelled = top = target; included for labelled): unwind = `TryExit + close` for every crossed for-of frame including target (all are EXITED).
- **continue** (target re-entered): unwind = `TryExit + close` for frames ABOVE target (crossed and exited); `TryExit` only for target (its try-frame is popped, will be re-pushed at loop_start; its iter is NOT closed because the loop continues consuming it).
- **return**: unwind = `TryExit + close` for every for-of frame in the function (all EXITED). Return value pushed before unwind stays at the operand-stack top through the stack-neutral close calls and is consumed by `Op::Return`.

### Pin-Art (Rule 24) emit-site coherence

The unwind pattern is now structurally identical at three sites (break, continue-crossed-frames, return). Each emits the same `(try_open, close_slot)` walk against `loop_stack`. Per the Rule 24 threshold (3+ duplicated sites with the same shape), the pattern is a candidate for promotion to a single helper `emit_frame_unwind` in a subsequent rung. Not promoted at this rung to keep the surface tight; promotion deferred to a Rule-24 follow-up if a fourth emit site appears (yield* delegation close, async-iter unwind).

## Verification

1. `cargo build --release --bin cruft -p cruftless` PASS (~1m 11s).
2. `cargo test --release -p rusty-js-runtime --lib`: 74 / 0 / 1 (preserved).
3. Direct probe `/tmp/probe-ices-ext-3.js` (9 cells): 9/9 PASS.
4. Regression sweep (4 prior probes): all preserved — IPTD 7/7, cross-consumer 7/7, labelled-break ["B","A"], ICES-EXT 2 6/6.
5. All runs under `ulimit -v 2097152`. Pi survived.

## Carry-forward (out of ICES primary scope)

- **for-await body throw**: parallel synthetic try wrap at the for-await emission site (uses `forawait_tmp` path); sibling rung.
- **yield* delegation close**: separate adjacent locale `iterator-close-on-abrupt/`.
- **Rule 24 helper promotion**: `emit_frame_unwind(frame_indices, mode: Exits|ReEntersTarget)` if a fourth emit site appears.

## Composes-With

- `apparatus/proposals/decided/2026-05-31T044000Z-ices-ext-2-for-of-return-iterclose/decision.md` (substrate prefix source: LoopFrame anchor + close helper)
- `pilots/iterator-close-emission-sites/trajectory.md` ICES-EXT 1 → 2 → 3 chain
- `apparatus/docs/predictive-ruleset.md` Rule 15 (chapter-close-inspect surfaced the gap), Rule 17 (segmentation: throw-only scope), Rule 24 (Pin-Art threshold for emit-site coherence)

## Authorization

Per keeper Telegram 10680 ("Continue") authorizing final ICES rung.
