---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "ICES-EXT 1: emit IteratorClose call at for-of break exit per ECMA-262 §14.7.5.6 step 5. Closes cells 3 + 6 of the IPTD-EXT 1 chapter-close residual (plain-for-of IteratorClose-on-break) at the bytecode compiler's break-patch site. Tracks the for-of iter slot on the LoopFrame; on (un)labelled break, walks crossed for-of frames innermost-first and emits __destr_iter_close per frame before the exit jump."
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: 88.7% (unchanged by IPTD-EXT 1 net-zero on sample paths)
  diff_prod: 65 / 47 (post-ASTA-EXT 0; not re-measured after IPTD-EXT 1)
gates_post:
  build: "cargo build --release --bin cruft -p cruftless"
  probe_cells:
    - "for...break with iter.return non-callable -> TypeError (was FAIL post-IPTD-EXT 1)"
    - "for...break with iter.return callable -> .return() invoked (was FAIL post-IPTD-EXT 1)"
    - "for...break with iter.return null -> close silent (preserve)"
    - "for...break with iter.return undefined -> close silent (preserve)"
    - "Labelled break crossing two for-of frames: close inner first then outer"
    - "Cross-consumer 7-cell probe: 7/7 PASS preserved"
---

## Substrate

Carry-forward from IPTD-EXT 1 chapter-close (Rule 15). The IPTD-EXT 1 trajectory entry surfaced cells 3 + 6 of the original probe as newly-visible gaps: plain `for (x of iter) { break; }` did not invoke IteratorClose on the iterator. The bytecode for-of slow path had no break-patch hook for close emission.

### LoopFrame extension

Add `for_of_iter_slot: Option<u16>` to the `LoopFrame` struct (`pilots/rusty-js-bytecode/derived/src/compiler.rs:453`). Set to `Some(iter_slot)` at the for-of push (line 2271); `None` at the 5 other push sites (while / do-while / C-for / labelled-block / switch).

### Break emission

In the unlabelled `Stmt::Break` arm: if the target frame (top of `loop_stack`) carries `for_of_iter_slot = Some(slot)`, emit `emit_iter_close_call(slot)` after finalizers and before the exit jump.

In the labelled `Stmt::Break` arm: walk frames from current top down to the target frame inclusive, collect `for_of_iter_slot`s in innermost-first order, emit a close call per slot after finalizers and before the exit jump. Matches ECMA-262 §14.7.5.6 step 5 + §13.15.7 abrupt-completion propagation: each for-of frame the break crosses gets one IteratorClose call, innermost first.

### Helper

New compiler helper `emit_iter_close_call(iter_slot)` adjacent to the existing `emit_iter_close_if_not_done` (which guards on done_slot for destructuring). Stack-neutral: emits `LoadGlobal __destr_iter_close; LoadLocal iter_slot; Call 1; Pop`. Reuses the IPTD-EXT 1 helper-tier discipline at `__destr_iter_close` (TypeError on non-callable non-null/undefined return, invoke on callable, silent on null/undefined).

## Carry-forward (out of scope for EXT 1)

- **ICES-EXT 2** for-of `return` inside body: requires routing through the Return opcode emission to walk loop_stack and emit close for each for-of frame between current position and frame_idx 0 (or the enclosing function frame). Direct probe confirms `return` inside `for-of` does not currently invoke close.
- **ICES-EXT 3** for-of `throw` inside body: requires wrapping the loop body in a synthetic try-catch that calls IteratorClose on exception before re-throwing. Larger blast radius (TryEnter/TryExit instrumentation). Direct probe confirms.
- **for-await** close: parallel emission for async iteration; not in ICES base scope.
- **yield* delegation close**: separate adjacent locale `iterator-close-on-abrupt/`.

## Verification

1. `cargo build --release --bin cruft -p cruftless` PASS.
2. Direct probe `/tmp/probe-iptd-0.js` (7 cells): 7/7 PASS (was 5/7 post-IPTD-EXT 1; cells 3 + 6 now close).
3. Direct probe `/tmp/probe-iptd-1-destr.js` (7 cells, cross-consumer): 7/7 PASS preserved.
4. Direct probe `/tmp/probe-ices-labelled.js` (labelled break crossing two for-of frames): emits `["B","A"]` (innermost-first close order verified).
5. Residual probe `/tmp/probe-ices-residuals.js`: confirms ICES-EXT 2 + EXT 3 carry-forward (throw + return paths do not close).
6. All probes run under `ulimit -v 2097152` per the IPTD-locale standing forensic gate.

## Composes-With

- `pilots/iterator-close-emission-sites/` (this locale; ICES seed names ICES-EXT 1 as "for-of break IterClose emission")
- `pilots/iterator-protocol-throw-discipline/` IPTD-EXT 1 chapter-close (this rung consumes that carry-forward)
- `apparatus/docs/predictive-ruleset.md` Rule 15 (chapter-close-inspect surfaces the gap), Rule 17 (segmentation: break-only scope, return + throw deferred)
- `apparatus/proposals/decided/2026-05-31T035300Z-iptd-ext-1-foroffast-and-emit-site-audit/decision.md` (the direct precursor)

## Authorization

Per keeper Telegram 10676 ("Continue with 1") authorizing the IPTD-EXT 1 chapter-close carry-forward into ICES-EXT 1.
