---
proposal_slug: 2026-05-30T235300Z-iptd-ext-0-iterator-protocol-throws
decision: NEGATIVE
arbiter_session: keeper-substituted-per-no-arbiter-appointed-Telegram-10650
decided_at: 2026-05-31T03:50:00Z
covers_commits:
  - 7a1435d4  # landed (helper-tier substrate at __destr_iter_step + __destr_iter_close)
  - af5326b7  # revert per Rule 13 after Pi-OOM regression
---

## Findings

NEGATIVE per Rule 13 (revert-then-deeper-layer-closure on negative empirical result).

Substrate commit `7a1435d4` added the ECMA-262 §7.4.5 IteratorNext + §7.4.9 IteratorClose throw-discipline at the destructuring helper site only (`__destr_iter_step` + `__destr_iter_close` in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`). The proposal's Rule 23 founding-time tier-mismatch observation (predicted parser/IR; actual Runtime) was partially correct but incomplete: it identified the destructuring helper as one Runtime-tier consumer of `iter.next()` but missed the OTHER Runtime-tier consumer — the bytecode `Op::ForOfFastNext` slow-path emission at `pilots/rusty-js-bytecode/derived/src/compiler.rs:2292` that handles plain `for (x of iter)`.

Plain for-of never invokes `__destr_iter_step`. With `iter.next()` returning a non-Object value (e.g., `42`), the runtime read `.done` on the number → `Undefined` → falsy → unbounded loop allocating per-iteration through `Heap<T>::alloc::grow_one` until OOM.

## Pi-OOM regression

Probe `/tmp/probe-iptd-0.js` cell 1 (plain for-of with non-Object `.next()` result) was the first execution of the substrate. The Pi hard-reset within seconds with no kernel trace (no persistent journald). Session `a0ac435a` jsonl truncates mid-tool-call at 2026-05-30T17:02:12 PDT; system rebooted at 17:04:24 PDT.

Post-revert reproduction under `ulimit -v 2097152`:

```text
memory allocation of 1778384896 bytes failed
stack backtrace:
  rusty_js_gc::Heap<T>::alloc
  rusty_js_runtime::interp::Runtime::alloc_object
  rusty_js_runtime::interp::Runtime::call_function_inner
  ... (deep, allocation-bound)
```

## Rule 13 application

- Verify negative ✓
- Diagnose structurally ✓ (tier-mismatch within Runtime: helper covers destructuring, opcode covers plain for-of)
- Revert (commit `af5326b7`) ✓
- Keep trajectory + diagnosis ✓ (`pilots/iterator-protocol-throw-discipline/trajectory.md` IPTD-EXT 0 entry)
- Identify deeper-layer closure ✓ (IPTD-EXT 1: helper + ForOfFastNext slow-path + Rule 20 cross-consumer audit)

Substrate prefix retained: the helper-tier discipline drafted at EXT 0 (validated against spec, just wrong site coverage) becomes the cheap enabler of EXT 1's parallel-emit-site closure.

## Forensic note

Persistent journald enabled on the engagement host post-incident (`/var/log/journal/`). Future probes touching iteration-protocol throw paths run under `ulimit -v 2097152` until the locale closes.

## Composes-With

- `apparatus/proposals/decided/2026-05-31T035300Z-iptd-ext-1-foroffast-and-emit-site-audit/decision.md` (the deeper-layer closure).
- `apparatus/docs/predictive-ruleset.md` Rule 13, Rule 23.
- `pilots/iterator-protocol-throw-discipline/trajectory.md` IPTD-EXT 0 entry.
