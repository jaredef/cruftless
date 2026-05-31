---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "ICES-EXT 2: emit IteratorClose at for-of return per ECMA-262 §14.7.5.6 step 5 + §13.10.1 return-completion. Walks self.loop_stack innermost-first at the Stmt::Return emission site, emits stack-neutral close per for-of frame between the return site and the function frame. Substrate prefix from ICES-EXT 1 (LoopFrame.for_of_iter_slot + emit_iter_close_call) consumed without modification — Finding ICES.1 cross-frame anchor extends from break to return without auxiliary state."
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: 88.7%
  diff_prod: 65 / 47
gates_post:
  build: "cargo build --release --bin cruft -p cruftless"
  probe_cells:
    - "return inside for-of calls iter.return()"
    - "return crossing nested for-of closes innermost first (IN before OUT)"
    - "return value preserved through stack-neutral close calls"
    - "return with non-callable iter.return -> TypeError (overrides normal return)"
    - "plain return unaffected (positive control)"
    - "return from while-loop has no iter to close (positive control)"
    - "All prior probes preserved: original 7-cell IPTD 7/7; cross-consumer 7/7; labelled-break order [\"B\",\"A\"]"
---

## Substrate

Carry-forward from ICES-EXT 1 chapter-close. The residual probe confirmed `return` inside a for-of body did not invoke IteratorClose.

### Stmt::Return extension (~10 LOC)

In the existing Stmt::Return arm at compiler.rs:1631-1647, after the finalizer-emission block and before `Op::Return`, walk `self.loop_stack` in reverse, collect each frame's `for_of_iter_slot` where Some, and emit a stack-neutral `emit_iter_close_call(slot)` per slot. The return value pushed earlier stays at the operand-stack top through the close sequence (each close call is `LoadGlobal; LoadLocal; Call 1; Pop` — stack-neutral); Op::Return then consumes it as before.

The loop_stack is per-function (fresh `Vec::new()` at every `compile_function_proto` per compiler.rs:5081), so all frames visible at the return site are within this function frame and must be closed before the function returns. Matches §14.7.5.6 step 5 + §13.15.7 abrupt-completion propagation.

### Substrate prefix consumed unchanged

- `LoopFrame.for_of_iter_slot: Option<u16>` (ICES-EXT 1)
- `emit_iter_close_call(iter_slot)` helper (ICES-EXT 1)
- Helper-tier `__destr_iter_close` discipline (IPTD-EXT 1)

EXT 2 is one new code block at the return arm; no helper changes, no new opcodes, no LoopFrame changes. Finding ICES.1 (LoopFrame as cross-frame IteratorClose anchor) is empirically validated: the same primitive serves break (target-frame slot for unlabelled, target-to-top range for labelled) and return (top-to-bottom range), with no auxiliary state.

## Carry-forward (out of scope)

- **ICES-EXT 3** for-of `throw`: requires synthetic try-catch wrapping the loop body to catch exceptions and emit IteratorClose before re-throw. Larger blast radius (TryEnter/TryExit instrumentation, abrupt-completion threading). Direct probe confirms throw inside for-of still does not close.
- **for-await** close: parallel emission for async iteration; sibling locale.
- **yield* delegation close**: adjacent locale `iterator-close-on-abrupt/`.

## Verification

1. `cargo build --release --bin cruft -p cruftless` PASS (~1m 13s).
2. Direct probe `/tmp/probe-ices-ext-2.js` (6 cells): 6/6 PASS.
3. Regression sweep (3 prior probes): all preserved — original IPTD 7/7; cross-consumer 7/7; labelled-break order ["B","A"].
4. All runs under `ulimit -v 2097152`. Pi survived.

## Composes-With

- `apparatus/proposals/decided/2026-05-31T041500Z-ices-ext-1-for-of-break-iterclose/decision.md` (substrate prefix source)
- `pilots/iterator-close-emission-sites/trajectory.md` ICES-EXT 1 → EXT 2 chain
- `apparatus/docs/predictive-ruleset.md` Rule 15 (chapter-close-inspect carry-forward), Rule 17 (segmentation: return-only scope, throw deferred)

## Authorization

Per keeper Telegram 10678 ("Ext 2") authorizing continuation of the ICES carry-forward chain.
