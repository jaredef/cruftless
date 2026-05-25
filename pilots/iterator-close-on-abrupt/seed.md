# iterator-close-on-abrupt — Seed

**Locale tag**: `L.iterator-close-on-abrupt` (top-level; EPSUA sub-locale #1 per re-ordered queue)

**Status**: **CLOSED at ICOA-EXT 1**.

**Workstream**: ECMA-262 §7.4.9 IteratorClose + §13.15.5.3 step 5: array-destructure must call `iterator.return()` after destructure if the iterator was not exhausted. Cruft's IPEP-EXT 1 left this for IPEP-EXT 2 candidate; this locale closes it under the EPSUA arc.

**Trigger**: EPSUA C5 pivot after EPSUA-EXT 0.5 falsified the $262 projection. Constraint #4 (~25 cascade, bounded substrate cost) is the next-leverage candidate.

**Composes with**:
- ECMA-262 §7.4.9 IteratorClose; §13.15.5.3 step 5
- [EPSUA seed](../ecmascript-parity-shared-upstream-arc/seed.md) — parent arc
- [IPEP trajectory](../iterator-protocol-error-propagation/trajectory.md) — predecessor that opened iter-protocol but deferred close
- [Finding T262C.5](../test262-categorize/trajectory.md) — Doc 740 multi-tier closure default discipline

## I. Telos

After array-destructure, if the opened iterator was not exhausted (any step returned done=true ≥ 1), call `iter.return()` per §7.4.9 IteratorClose. Symmetric across `emit_destructure` (binding) and `emit_destructure_assign` (assignment) paths. Skip when rest pattern is present (rest exhausts iter).

## II. Apparatus + Methodology

R = {iter_close_helper, emit_iter_step_value_done_tracking, emit_iter_close_if_not_done_helper, emit_destructure_close_call, emit_destructure_assign_close_call}. All landed combined per Doc 740 default.

Edits (~50 LOC):
1. `intrinsics.rs`: new `__destr_iter_close(iter)` helper per §7.4.9.
2. `compiler.rs::emit_iter_step_value`: optional `done_slot: Option<u16>` parameter; when present, writes done state to slot for caller use.
3. `compiler.rs::emit_iter_close_if_not_done(iter_slot, done_slot)` helper — emits `if (!done_slot) __destr_iter_close(iter_slot)`.
4. `compiler.rs::emit_destructure` Array path: alloc done_slot, init false, pass to step calls, emit close if not exhausted (no rest).
5. `compiler.rs::emit_destructure_assign` Array path: same.

## III. Carve-outs

- Object-pattern destructure unchanged (no iterator).
- For-of body break/throw paths: cruft's existing for-of code may handle some; this locale only touches the destructure-end close path per spec §13.15.5.3 step 5.
- IteratorClose-on-throw inside destructure (per §7.4.9 step 6 if call to return throws): not propagated specially in v1; outer throw wins per cruft's default.

## IV. Verification

Minimal repros (GREEN):
- `for (var [a] of [iterable]) { break; }` with iter.return() side-effect → returnCount=1 ✓
- `[a] = iterable` with iter.return() side-effect → returnCount=1 ✓

Exemplar (previously-failing iter-(nrml|rtrn|thrw)-close + expected-throw dstr, 20 tests):
- PASS: 0 → 6 (+6; projected 25; under-delivered by ~76%)
- Remaining 14 fail with shapes outside ICOA scope: TypeError-not-thrown when iter.next returns non-object (separate substrate), Test262Error-not-thrown, count mismatches from spec-step ordering.

Regression check:
- for-of/dstr previously-passing 387 → 387 (0 regressed)
- Random 50 from Array/Set/destructuring tests: 50/50 PASS (0 regressed)

Full sweep deferred per keeper directive.

## V. Status

CLOSED at ICOA-EXT 1.
