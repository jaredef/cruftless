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
