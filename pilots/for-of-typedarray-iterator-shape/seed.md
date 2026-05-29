# for-of-typedarray-iterator-shape - Seed

**Locale tag**: `L.for-of-typedarray-iterator-shape` (FOTIS).

**Status**: FOUNDED at FOTIS-EXT 0. Phase 0 spawn plus Phase 2 baseline probe only; no runtime substrate lands in this founding round.

**Parent arc**: Iterator protocol substrate, adjacent to TypedArray prototype-shape work. This locale is intentionally separate from TAPD: TAPD closed TypedArray method prologue/argument validation, while FOTIS targets `for-of` discovery of `%TypedArray%.prototype[@@iterator]`.

**Workstream**: `for (x of typedArray)` iterator-shape failures in the post-EPSUA test262 sample matrix, specifically `language.statements.for-of / feat:TypedArray;not-callable`.

## I. Telos

Close the 18-row `for-of` TypedArray iterator shape cluster without conflating it with general TypedArray method validation or destructuring iterator-close work.

The suspected substrate is the mismatch between TypedArray iterator method installation and the current concrete TypedArray prototype chain: the iterator triplet is present on one prototype object, but `for-of` reaches instances through a different prototype path.

## II. Apparatus

- Post-EPSUA matrix: `pilots/apparatus/test262-categorize/results/2026-05-29/{matrix.md,categorized.jsonl}`.
- Runtime for-of bytecode path: `pilots/rusty-js-runtime/derived/src/interp.rs::Op::ForOfFastNext` plus surrounding slow-path bytecode.
- Iterator acquisition helper examples: `Runtime::promise_collect_iterable` and generated/destructure helpers using `@@iterator`.
- TypedArray installation: `pilots/rusty-js-runtime/derived/src/intrinsics.rs`, especially `%TypedArray%` prototype creation, per-type prototype wiring, and `values`/`keys`/`entries`/`@@iterator` registration.

## III. Methodology

1. Phase 0: create this locale and refresh `apparatus/locales/manifest.json`.
2. Phase 2: inspect the matrix row and all matching `categorized.jsonl` entries.
3. Sample at least eight failures across mutate and non-mutate fixtures and across integer/float/clamped typed arrays.
4. Categorize failures into TypedArray `@@iterator` missing/non-callable, for-of iterator-not-callable handling, detached/out-of-bounds iteration, and other buckets.
5. Apply C4: proceed to Phase 3 only if one mechanism bucket accounts for at least 40% of the narrowed cluster.
6. Cross-reference the runtime implementation before proposing substrate; APS-style runtime edits are explicitly out of scope for FOTIS-EXT 0.

## IV. Carve-Outs

- TypedArray method prologues and argument validation remain TAPD/TAMM territory.
- Destructuring iterator-close and generator for-of clusters remain separate iterator-protocol locales.
- Detached/resizable ArrayBuffer semantics are out of scope unless Phase 2 shows they are the dominant cause.
- No substrate code change is authorized in FOTIS-EXT 0.

## V. Resume Protocol

Read this seed, then `trajectory.md`. Resume by rechecking the latest matrix for `language.statements.for-of / feat:TypedArray;not-callable`, then inspect TypedArray prototype wiring before proposing a runtime change.
