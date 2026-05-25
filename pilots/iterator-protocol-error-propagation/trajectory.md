# iterator-protocol-error-propagation — Trajectory

## IPEP-EXT 0 — workstream founding (2026-05-25)

**Trigger**: keeper directive "A" (full iter-protocol scope) after recon on T262C cluster #1 post-VHTB (40 tests, for-of dstr iterator-protocol). Substrate gap: emit_destructure + emit_destructure_assign Array paths shortcut to GetIndex, bypassing the spec-required IteratorRecord protocol per §13.15.5.3 / §14.4.2.4.

**Doc 740 R-identification**: {iter-helpers, emit_destructure-array, emit_destructure_assign-array, elision-advance, rest-collect}. All landed together per FODAS/PPA/REOU lessons.

## IPEP-EXT 1 — multi-tier closure (2026-05-25)

**Edits** (~120 LOC total):
1. `intrinsics.rs`: three engine helpers `__destr_iter_open/_step/_rest` matching the existing engine-helper pattern.
2. `compiler.rs`: new `emit_iter_step_value(iter_slot)` helper inlines step+done-check+value/undef.
3. `compiler.rs`: emit_destructure Array path rewritten — open iter once, per-element step (incl elision advance-and-discard), rest via helper.
4. `compiler.rs`: emit_destructure_assign Array path mirrors, preserving FODAS NamedEvaluation hint.

**Verification**: all 5 minimal repros GREEN (see seed §IV).

**test262-sample**: results booked post-measurement.

**Status**: results pending.

**test262-sample** (vs pre-IPEP baseline 5792):
- PASS: 5792 → **5838** (+46)
- FAIL: 1501 → 1450 (−51)
- PASS→FAIL regressions: **0**
- FAIL→PASS transitions: 46
- Runnable pass rate: 79.4% → **80.1%** (crossed 80% threshold)

### Findings

**Finding IPEP.1**: Doc 740 (P4) pipeline-connection at full magnitude — R = {iter-helpers, emit_destructure-array, emit_destructure_assign-array, elision-advance, rest-collect} landed combined, +46 PASS / 0 regressions. The combined-multi-tier-closure pattern is now the engagement's default discipline for sub-locales with cascade-revival shape.

**Finding IPEP.2**: Plain-array destructure (the common case) still functionally correct via the iter-protocol path. `Array.prototype[Symbol.iterator]()` returns a real ArrayIterator; `next()` yields each element. Perf surface: per-element function call instead of direct GetIndex. Diff-prod / CRB regression check deferred (Pred-ipep.6 fallback).

**Status**: IPEP-EXT 1 CLOSED.
