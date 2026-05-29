# promise-iterator-not-callable-discipline - Seed

**Locale tag**: `L.promise-iterator-not-callable-discipline` (PIND).

**Status**: FOUNDED at PIND-EXT 0. Phase 0 spawn plus Phase 2 baseline probe only; no substrate landing is authorized in the founding round.

**Parent arc**: [`apparatus/arcs/2026-05-28-iterator-protocol-substrate/`](../../apparatus/arcs/2026-05-28-iterator-protocol-substrate/arc.md).

**Workstream**: Promise combinator iterator-protocol error discipline for `Promise.all`, `Promise.allSettled`, and `Promise.race`. The post-EPSUA sample matrix surfaced a compact Promise.* `Symbol.iterator` + `not-callable` cluster in the top-10 failure cells.

## I. Telos

Close the Promise.* iterator-not-callable cluster by restoring spec-shaped abrupt-completion handling in the Promise combinator host paths. The first candidate substrate is the missing IsCallable/abrupt-completion discipline around iterable acquisition and iterator method calls in the `Promise.all`, `Promise.allSettled`, and `Promise.race` runtime paths.

The founding probe must determine whether the top-10 matrix rows are one coherent iterator-method defect or a two-part Promise combinator mouth: (1) Promise static method callability at the test harness callsite and (2) iterable `@@iterator` method callability that should reject through the combinator promise rather than throw synchronously.

## II. Apparatus

- Post-EPSUA sample matrix: `pilots/apparatus/test262-categorize/results/2026-05-29/{summary.md,matrix.md,categorized.jsonl}`.
- Raw sample result source: `/home/jaredef/Developer/cruftless-r3-sidecar/results/test262-sample-2026-05-29/results.jsonl`.
- Runtime Promise combinator implementation: `pilots/rusty-js-runtime/derived/src/interp.rs`, especially `promise_all_via`, `promise_all_settled_via`, and `promise_race_via`.
- Generated ECMA entry shims: `pilots/rusty-js-runtime/derived/src/generated.rs` Promise.all/allSettled/race wrappers.
- Parent iterator-protocol arc for GetIterator / IteratorStep / IteratorClose positioning.

## III. Methodology

1. Phase 0: create this locale and refresh `apparatus/locales/manifest.json`.
2. Phase 2: inspect the matrix cells for `Promise.race`, `Promise.allSettled`, and `Promise.all` with `feat:Symbol.iterator;flag:async;not-callable`.
3. Sample at least eight failures across all three Promise combinators and segment reasons by missing check/propagation shape.
4. Apply EPSUA C4: proceed only if a reason bucket reaches at least 40% of the narrowed cluster.
5. Propose a Phase 3 substrate move but do not edit runtime code in this founding round.

## IV. Carve-Outs

- Promise.any remains out of scope unless Helmsman widens this locale after Phase 2. It shares nearby machinery but was not one of the three top-10 rows named for this locale.
- General Promise constructor/new-capability correctness is out of scope except where it affects the three combinator paths.
- Async scheduling/microtask semantics are out of scope for PIND-EXT 0.
- IteratorClose-on-abrupt is parent-arc scope; PIND may surface it but should not absorb unrelated for-of/destructure/yield* closures.
- No runtime substrate lands in PIND-EXT 0.

## V. Resume Protocol

Read this seed, then `trajectory.md`, then the parent iterator-protocol arc. Resume by rechecking the latest sample/full-suite matrix for Promise.* not-callable rows, then inspect `promise_all_via`, `promise_all_settled_via`, and `promise_race_via` before proposing or landing substrate. If the top bucket shifts away from Promise combinator iterator callability, report the pivot before editing.
