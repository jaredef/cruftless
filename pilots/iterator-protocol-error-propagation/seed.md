# iterator-protocol-error-propagation — Resume Vector / Seed

**Locale tag**: `L.iterator-protocol-error-propagation` (top-level)

**Status as of 2026-05-25**: **CLOSED at IPEP-EXT 1** (1 implementation round; Doc 740 multi-tier closure).

**Workstream**: ECMA-262 §13.15.5.3 RS:DestructuringAssignmentEvaluation + §14.4.2.4 IteratorBindingInitialization — Array-pattern destructure must open an IteratorRecord from the source via `GetIterator(value)` and read each element through `iterator.next()`, not via index access. Cruft's emit_destructure + emit_destructure_assign Array paths shortcut to `GetIndex`, bypassing @@iterator entirely. Iterables whose @@iterator getter throws, or whose next() throws, never propagate the throw.

**Author**: 2026-05-25 session.
**Parent**: none (top-level).
**Siblings**: TCC, TXC, T262C, FODAS, PPA, REOU, VHTB.
**Composes with**:
- ECMA-262 §13.15.5.3 (DAE), §14.4.2.4 (IBI), §7.4.1 (GetIterator)
- [Doc 740](../../docs/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) (P4) multi-tier closure
- [T262C matrix](../test262-categorize/results/2026-05-25/matrix.md) cluster #1 post-VHTB

## I. Telos

**Empirical answer to**: does routing both Array-destructure paths (binding + assignment) through new `__destr_iter_open/_step/_rest` engine helpers close the 40-test iterator-protocol cluster without regressing destructure-on-plain-arrays correctness?

## II. Apparatus + Methodology

R = {iter-helpers, emit_destructure-array, emit_destructure_assign-array, elision-advance, rest-collect}. All landed together per FODAS/PPA/REOU lessons.

Edits:
1. `intrinsics.rs`: add three engine helpers
   - `__destr_iter_open(value)` → calls @@iterator, returns iterator (throws on null/undef or non-iterable)
   - `__destr_iter_step(iter)` → calls iter.next(), returns IteratorResult
   - `__destr_iter_rest(iter)` → loops .next() until done, returns Array
2. `compiler.rs`: new `emit_iter_step_value(iter_slot)` helper that inlines the step+done-check+value/undef pattern.
3. `compiler.rs`: emit_destructure Array path opens iter once + per-element step (incl elision advances + discards) + rest via __destr_iter_rest.
4. `compiler.rs`: emit_destructure_assign Array path mirrors the same shape, preserving the FODAS NamedEvaluation hint at default-init sites.

## III. Carve-outs

- IteratorClose on abrupt completion (§7.4.9): NOT implemented in this round. Tests that observe close-on-throw cleanup will continue to fail; tracked as IPEP-EXT 2 candidate.
- Object-pattern destructure UNCHANGED — uses GetProp per key (spec-correct; no iterator involved).
- Existing __destr_array_rest helper retained for any callers not yet converted.

## IV. Verification

Minimal repros (all GREEN):
- `for ([_] of [{[Symbol.iterator]() { throw E }}]) {}` → throws E ✓
- `var [a] = { [Symbol.iterator]() { return { next() { throw E } } } }` → throws E ✓
- `var [a,b,c] = [1,2,3]` → 1,2,3 ✓
- `var [,,x] = [10,20,30]` → 30 ✓
- `var [p, ...rest] = [1,2,3,4]; Array.isArray(rest)` → true; rest = [2,3,4] ✓

test262-sample: results booked at chapter close.

## V. Resume protocol

Read seed. The fix is three helpers + two compiler-site rewrites + one inline-emit helper. Spec sections 13.15.5.3 / 14.4.2.4 / 7.4.1 define the protocol; spec section 7.4.9 (IteratorClose) is the next-round candidate.
