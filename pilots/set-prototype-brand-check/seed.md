# set-prototype-brand-check — Seed

**Locale tag**: `L.set-prototype-brand-check` (top-level)

**Status**: **CLOSED at SPBC-EXT 1**.

**Workstream**: ECMA-262 §24.2.4.X (Set.prototype set-methods — union, intersection, difference, symmetricDifference, isSubsetOf, isSupersetOf, isDisjointFrom): each method must brand-check `this` is a Set (has [[SetData]] internal slot) per RequireInternalSlot. Cruft's set-methods only checked `this` is an Object; any Object with no `__set_data` was silently treated as an empty Set.

**Trigger**: keeper directive "New top level locale as coherent" after RPTP-EXT 2 close. Probe identified 28 brand-check Set/prototype tests (7 set-methods × ~3-4 receiver-variants each + 7 basic methods × 1 each — basic ones already pass).

**Composes with**:
- ECMA-262 §24.2.4 Set.prototype (set-methods proposal, now ratified)
- §10.1.X RequireInternalSlot

## I. Telos

Add brand-check at the 7 Set.prototype set-method entries. Closes the receiver-not-set / does-not-have-setdata-internal-slot-weakset sub-cluster.

## II. Apparatus + Methodology

R = {require_set_brand helper, 7 method entry points}.

Edits (~25 LOC):
1. `interp.rs::Runtime::require_set_brand`: helper returning Err(TypeError) if this isn't an Object or doesn't have `__set_data`.
2. Replace `let this = match self.current_this() { ... }` boilerplate in the 7 set-method `_via` impls with `let this = self.require_set_brand(self.current_this(), "name")?;`.

## III. Carve-outs

- Set basic methods (add, has, delete, clear, forEach, entries, values) already brand-check correctly.
- Map.prototype / WeakMap / WeakSet brand checks: separate sub-locales.
- "Set-like class" inherited-SetData edge cases: out-of-scope (inheritance gives valid SetData by construction).

## IV. Verification

Minimal repro: `Set.prototype.union.call({}, new Set())` → TypeError ✓.

Exemplar (28 in-scope tests): PASS 0 → **14**. 14 remaining are Set-like-class / Set-like-object variants that need additional brand-check refinement.

Regression check on Set/prototype (256 previously-passing): 0 regressed.

## V. Status

CLOSED at SPBC-EXT 1.
