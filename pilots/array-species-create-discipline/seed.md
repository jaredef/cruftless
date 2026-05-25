# array-species-create-discipline — Resume Vector / Seed

**Locale tag**: `L.array-species-create-discipline` (top-level)

**Status as of 2026-05-25**: **WORKSTREAM FOUNDED (ASCD-EXT 0)**. Spawned per keeper directive after EPSUA arc closed; carries the largest clean sub-cluster of the former constraint #1 (~23 tests).

**Workstream**: ECMA-262 §7.3.21 ArraySpeciesCreate. Array prototype methods that return a new array (concat, filter, map, slice, splice, copyWithin, fill, ...) must allocate the output via ArraySpeciesCreate, which (a) reads `this.constructor`, (b) if that is non-undefined reads `[Symbol.species]`, (c) if that is non-undefined+non-null IsConstructor-checks it (throw TypeError if not constructor), and (d) uses it as the constructor. Cruft's Array methods bypass this entirely — they just allocate a plain Array — so tests that assert "concat with non-constructor species throws TypeError" all fail with the "Expected a TypeError, none thrown" shape.

**Pre-scoping probe** (per Finding T262C.6 + Finding EPSUA.6 refinement): species-related TypeError tests = 23 (in-scope sub-cluster). Cluster-projection would have been 226 (whole reason-text), but per-reason-pattern segmentation correctly narrows to species-specific upstream cause.

**Author**: 2026-05-25 session.
**Parent**: none (top-level); successor to EPSUA's constraint #1 surface.
**Composes with**:
- ECMA-262 §7.3.21 ArraySpeciesCreate; §7.3.23 ArrayCreate; §10.4.2 Array exotic
- [EPSUA-EXT 4 chapter close](../ecmascript-parity-shared-upstream-arc/trajectory.md) — Finding EPSUA.6 refinement of T262C.4 discriminator
- [docs/engagement/prospective/test262-long-tail-shared-vs-mutually-exclusive.md](../../docs/engagement/prospective/test262-long-tail-shared-vs-mutually-exclusive.md) — constraint #1 surface
- [Doc 740](../../docs/corpus-ref/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) — multi-tier closure default

## I. Telos

Implement ArraySpeciesCreate per §7.3.21; wire it into the Array prototype methods that return new arrays. Closes the ~23-test species sub-cluster without regressing the ~700+ Array-method tests currently passing (Rule 14 mirror — adding restriction needs care, since most existing tests don't go through species path and shouldn't observe the new code).

### I.1 First-cut scope (ASCD-EXT 1)

Single substrate algorithm + call-site replacements:
- New helper `array_species_create(rt, this_arr, length)` per §7.3.21.
- Call sites: concat, filter, map, slice, splice, copyWithin (probably already in cruft as `array_proto_*`).
- Per-method change: replace direct `alloc_object(Object::new_array())` with `array_species_create(rt, this, length)`.

### I.2 Constraints

```
C1. cargo build + diff-prod 42/42 + canonical fuzz acc=-932188103 hold.
C2. Existing Array-method tests (~700+ passing) preserve.
C3. The new helper handles the spec's edge cases: undefined this.constructor,
    null Symbol.species, non-object Symbol.species, etc.
C4. Per Rule 14 mirror: the helper falls back to plain ArrayCreate on
    any path that doesn't trigger the species-check (typical case).
```

### I.3 Falsifiers

**Pred-ascd.1**: ~20 species tests pass post-fix (within ±10% of in-scope sub-cluster size 23).
**Pred-ascd.2**: zero PASS→FAIL on previously-passing Array.* tests (Rule 14 mirror).
**Pred-ascd.3**: single implementation round per Doc 740 (helper + 5-7 call-site changes landed together).
**Pred-ascd.4 (DISCIPLINE — Finding EPSUA.6 corroboration)**: in-scope sub-cluster ratio ≥50% (further validates per-reason-pattern segmentation as the correct projection unit).

## II. Apparatus + Methodology

- Edit at `pilots/rusty-js-runtime/derived/src/interp.rs` or `intrinsics.rs` (find ArrayCreate call sites in Array.prototype methods).
- Verify via species-test exemplar suite (built-ins/Array/prototype/{concat,filter,map,slice,splice}/create-species*.js).
- Regression check on adjacent Array-method tests.

## III. Carve-outs

- TypedArray species (separate spec section) — not covered.
- Other built-in classes with @@species (Promise, RegExp) — not covered.
- Symbol-coerce sub-cluster (~29) — separate sub-locale per Finding EPSUA.6.
- Revoked-proxy sub-cluster (~10) — separate sub-locale.
- Non-extensible sub-cluster (~10) — separate sub-locale.

## IV. Standing artefacts

- `pilots/array-species-create-discipline/seed.md`, `trajectory.md`
- Edit at runtime substrate (~50-80 LOC across helper + call sites)

## V. Resume protocol

Read seed + trajectory tail. The fix is one spec algorithm + N call-site replacements. Verify via species exemplar suite + adjacent Array regression check.
