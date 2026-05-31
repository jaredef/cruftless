# iterator-protocol-throw-discipline — Trajectory

## IPTD-EXT 0 — NEGATIVE (2026-05-30 / reverted 2026-05-30) — wrong emit site; revert per Rule 13

**Trigger**: Proposal `apparatus/proposals/pending/2026-05-30T235300Z-iptd-ext-0-iterator-protocol-throws/proposal.md` drafted by helmsman during drift-recovery session. Substrate diff landed as commit `7a1435d4` under keeper bypass; reverted as commit `af5326b7` after negative empirical result.

**Substrate (as landed in 7a1435d4)**:

- `__destr_iter_next` (intrinsics.rs:7586): added ECMA-262 §7.4.5 step 3 type check — throw TypeError when `iter.next()` returns a non-Object.
- `__destr_iter_close` (intrinsics.rs:7604): narrowed silent-undefined branch — throw TypeError when `iter.return` is non-null, non-undefined, non-callable.

**Negative empirical result (Pi hard reset during initial probe; reproduced post-revert under `ulimit -v 2GiB`)**:

```text
memory allocation of 1778384896 bytes failed
stack backtrace:
  rusty_js_gc::Heap<T>::alloc
  rusty_js_runtime::interp::Runtime::alloc_object
  rusty_js_runtime::interp::Runtime::call_function_inner
  rusty_js_runtime::interp::Runtime::run_frame_inner
  ... (deep, allocation-bound)
```

Initial run (2026-05-30 17:02 PDT) executed without `ulimit`; Pi OOM-killer / kernel hard reset, jsonl truncates mid-tool-call, system back ~17:04.

**Diagnosis (Rule 23 baseline-inspect deferred to post-rung)**:

The locale seed §Carve-outs originally predicted a parser/IR-tier locale; the proposal corrected the tier prediction to Runtime (helper site `__destr_iter_next`) per a Rule 23 founding-time observation. The correction was incomplete: it identified ONE Runtime-tier consumer of iter.next() (the destructuring helper) but missed the OTHER Runtime-tier consumer (the bytecode for-of opcode dispatch).

Plain `for (x of iter)` lowers through `Op::ForOfFastNext` (`pilots/rusty-js-bytecode/derived/src/op.rs:361`; emitted at `pilots/rusty-js-bytecode/derived/src/compiler.rs:2292`), NOT through `__destr_iter_next`. The helper site only fires for destructuring lowerings (`let [a,b,c] = iter`, function-param destructure, etc.). Probe cell 1:

```js
for (const x of { [Symbol.iterator]() { return { next() { return 42; } }; } }) {}
```

never invoked the new throw path. With `next()` returning `42` and the runtime treating `42.done` as `undefined`→falsy, the for-of loop iterated unboundedly, allocating per-iteration scratch through `Heap<T>::alloc::grow_one` until OOM. On a Pi without persistent journald this manifested as a hard reset with no kernel trace.

**Rule 13 application**:

- Verify negative ✓ (reproduced under `ulimit -v 2097152` with Rust backtrace).
- Diagnose structurally ✓ (helper site vs opcode site tier-mismatch within Runtime; helper covers destructuring, opcode covers plain for-of; both consume iter.next() at distinct dispatch surfaces).
- Revert ✓ (commit `af5326b7` reverts `7a1435d4`).
- Keep trajectory + diagnosis ✓ (this entry).
- Identify deeper-layer closure ✓ (next entry: IPTD-EXT 1).

The substrate prefix the negative leaves on disk (the helper-tier discipline at `__destr_iter_next`/`__destr_iter_close`) becomes the cheap enabler of the deeper-layer closure: IPTD-EXT 1 re-instates both helper-tier checks AND adds the parallel checks at the `ForOfFastNext` opcode dispatch + any sibling iteration-protocol consumers surfaced by audit (for-await, spread-call, array-spread, Array.from, etc.) per Rule 20 cross-emission-site coherence.

**Forensic note**: this is the first locale entry in the engagement-wide trajectory ledger where a substrate-tier bug took out the host machine. Persistent journald has been enabled (`/var/log/journal/`) on the Pi for future-incident traceability; future probes touching iteration-protocol throw-paths should run under `ulimit -v` until the locale closes.

**Status**: IPTD-EXT 0 NEGATIVE; reverted; deeper-layer closure scoped as IPTD-EXT 1 (parallel emit-site coverage: helper + opcode + cross-consumer audit).

## IPTD-EXT 1 — LANDED (2026-05-31) — deeper-layer closure: helper reinstate + for-of slow-path + cross-consumer parallel-emit-site coherence

**Trigger**: Keeper APPROVED of proposal `apparatus/proposals/decided/2026-05-31T035300Z-iptd-ext-1-foroffast-and-emit-site-audit/decision.md` via Telegram 10669.

**Substrate** (~30 LOC across two crates):

1. `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (~25 LOC):
   - `__destr_iter_step` (line 7582): re-instate ECMA-262 §7.4.5 step 3 — throw TypeError when `iter.next()` returns non-Object.
   - `__destr_iter_close` (line 7613): re-instate §7.4.9 step 2 / §7.3.10 GetMethod throw — TypeError when `iter.return` is non-null, non-undefined, non-callable.
   - **new** `__iter_result_check` engine helper: post-call IteratorNext result-type check invoked from the bytecode for-of slow path. Throws or returns the result.

2. `pilots/rusty-js-bytecode/derived/src/compiler.rs` (~12 LOC at line 2306+):
   - After the slow-path `CallMethod 0` (`iter.next()`), emit `LoadGlobal __iter_result_check; Swap; Call 1` to thread the result through the check. Stack discipline preserved: `[result]` in, `[result]` out (or throw). Skipped on for-await heads (async-iter routes through `__await` first; the type-check post-await is deferred to a sibling rung at the `__async_from_sync_value` site).

**Phase 2 (Baseline-inspect)** per Rule 23: confirmed at `Op::ForOfFastNext` (interp.rs:14960) that the fast-path is shape-bounded to ArrayIterator instances — non-ArrayIterator falls through to the slow path emitted immediately after by the compiler. The slow path is therefore the correct closure point for plain for-of; no fast-path change needed (the fast-path's source is an internal ArrayIterator, which always returns {value, done} objects by construction).

**Phase 3 (Pin-Art if duplicated)** per Rule 24: invoked via the Closure D cross-consumer audit. Probed 5 sibling consumers (array-spread, Array.from, spread-call, yield*, destructuring-rest). Discovery: **all five share the helper-tier dispatch surface** at `__destr_iter_step` (or near-siblings). The single helper-tier reinstate covers every audited consumer without additional emit-site changes. This is the parallel-emit-site coherence win predicted by Finding ASTA.2: a single mouth-gating dispatcher serves multiple emit sites; closing it once propagates to all downstream emissions.

**Yield**:

```text
Direct probe /tmp/probe-iptd-0.js (originally drafted for EXT 0, 7 cells):
  Cell 1 next() returns 42  -> for-of throws TypeError    ✓ PASS  (OOM fixed)
  Cell 2 next() returns undefined -> throws TypeError      ✓ PASS
  Cell 3 iter.return = 42 (non-callable, on break)         ✗ FAIL (carry-forward)
  Cell 4 iter.return = null on break                       ✓ PASS
  Cell 5 iter.return missing on break                      ✓ PASS
  Cell 6 iter.return callable, on break                    ✗ FAIL (carry-forward)
  Cell 7 normal iterator, 3 iterations                     ✓ PASS

Direct probe /tmp/probe-iptd-1-destr.js (cross-consumer surface, 7 cells):
  destructure   let [a]=it with next()=42                  ✓ PASS
  destructure   let [a]=it with next()=undefined           ✓ PASS
  array-spread  [...it]                                    ✓ PASS
  Array.from    Array.from(it)                             ✓ PASS
  yield*        function*g(){yield* it}                    ✓ PASS
  spread-call   f(...it)                                   ✓ PASS
  destr-rest    let [...r]=it                              ✓ PASS

Aggregate: 12/14 PASS. The 2 failures (cells 3 + 6 of the original probe) are NOT regressions — pre-EXT 1 baseline OOMed on cell 1 before reaching cells 3 / 6, so those cells were never observable at baseline. They surface as newly-visible gaps in plain for-of's IteratorClose discipline on `break` (the slow-path emission does not invoke `__destr_iter_close` on abrupt completion). Domain of the adjacent locale `pilots/iterator-close-emission-sites/` (named at this seed §Composes-with). Carry-forward to IPTD-EXT 2.
```

**Forensic gate**: all dev probes run under `ulimit -v 2097152` per the proposal's mandatory constraint. No allocation exceeded ~50 MiB on any cell (well below the 100 MiB Phase-5 forensic gate). Pi survived.

**Phase 4 (Revert-then-deeper-layer if negative)**: not invoked — closure positive.

**Phase 5 (Chapter-close-inspect)** per Rule 15: surfaced two newly-visible gaps at the for-of IteratorClose-on-break path (cells 3 + 6). Both are in scope of the adjacent locale `iterator-close-emission-sites`, named at seed §Composes-with as a carry-forward boundary at locale founding. The unmasking is itself the Rule-15 finding: closing the OOM-on-non-conforming-iterator regression makes the IteratorClose-for-plain-for-of gap empirically observable for the first time.

**Cross-rung findings**:

- **Finding IPTD.1** (parallel-emit-site coherence): for the iterator-protocol throw discipline, the helper-tier dispatcher at `__destr_iter_step` serves destructuring, array-spread, Array.from, spread-call, yield*, and destructuring-rest — six surfaces, one closure. Plain for-of is the exception (its slow-path opcode emission predates the helper) and required its own closure. The 6:1:1 (helper:slow-path:fast-path) split is the load-bearing topology of iterator consumption in this engine; future iterator-protocol discipline changes (e.g., async-iter §7.4.6) should baseline-inspect against this map before scoping. Surfaces as candidate Finding for `apparatus/docs/findings-ledger.md` under cross-locale recurrence with ASTA.2 (parallel-emit-site discipline general pattern).

- **Finding IPTD.2** (forensic-gate as substrate-rung component): for substrate-rung work that touches loop-allocation paths, the dev probe MUST run under `ulimit -v` even when the substrate looks safe by inspection. The IPTD-EXT 0 incident demonstrated that a tier-mismatch in scope coverage (helper closed; opcode open) produces an unbounded-allocation regression undetectable by source-reading but trivially-detectable by a probe with a memory cap. Candidate standing-rule promotion at the next findings-disposition cycle.

**Status**: IPTD-EXT 1 LANDED. OOM-on-non-conforming-iterator closed at all surveyed emit sites (helper + for-of slow-path + 5 sibling cross-consumer surfaces). Plain-for-of IteratorClose-on-break gap surfaced as carry-forward to adjacent locale `iterator-close-emission-sites`. Forensic discipline (persistent journald + dev-probe `ulimit -v`) standing.

