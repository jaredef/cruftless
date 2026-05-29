# promise-iterator-not-callable-discipline - Seed

**Locale tag**: `L.promise-iterator-not-callable-discipline` (PIND).

**Status**: CHAPTER CLOSED at PIND-EXT 4. The named 40-row Promise.all/allSettled/race `Symbol.iterator` + not-callable cluster measures 40/40 PASS after Promise-local iterator acquisition and `C.resolve` rejection routing.

**Parent arc**: [`apparatus/arcs/2026-05-28-iterator-protocol-substrate/`](../../apparatus/arcs/2026-05-28-iterator-protocol-substrate/arc.md).

**Workstream**: Promise combinator iterator-protocol error discipline for `Promise.all`, `Promise.allSettled`, and `Promise.race`. The post-EPSUA sample matrix surfaced a compact Promise.* `Symbol.iterator` + `not-callable` cluster in the top-10 failure cells.

## I. Telos

Close the Promise.* iterator-not-callable cluster by restoring spec-shaped abrupt-completion handling in the Promise combinator host paths. Closure landed through a Promise-local iterable acquisition helper plus combinator-local `C.resolve` abrupt-completion routing in the `Promise.all`, `Promise.allSettled`, and `Promise.race` runtime paths.

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
5. Close with staged narrow rungs: iterator-acquisition rejection first, `C.resolve` rejection second, and Promise-local accessor-aware `@@iterator` lookup last.

## IV. Carve-Outs

- Promise.any remains out of scope unless Helmsman widens this locale. It shares nearby machinery but was not one of the three top-10 rows named for this locale.
- General Promise constructor/new-capability correctness is out of scope except where it affects the three combinator paths.
- Async scheduling/microtask semantics remain out of scope.
- IteratorClose-on-abrupt is parent-arc scope; PIND may surface it but should not absorb unrelated for-of/destructure/yield* closures.
- Global `crate::intrinsics::collect_iterable` accessor-awareness remains out of scope for this chapter; PIND closed by Promise-local lookup to preserve narrow blast radius.

## V. Resume Protocol

Read this seed, then `trajectory.md`, then the parent iterator-protocol arc. This chapter is closed unless a future matrix reopens Promise.all/allSettled/race `Symbol.iterator` + not-callable rows. If reopened, first inspect `promise_collect_iterable_or_reject`, `promise_collect_iterable`, `promise_all_via`, `promise_all_settled_via`, and `promise_race_via` before proposing substrate.
