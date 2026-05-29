# array-prototype-sort - Seed

**Locale tag**: `L.array-prototype-sort` (APS).

**Status**: FOUNDED at APS-EXT 0. Phase 0 spawn plus Phase 2 baseline probe only; no runtime substrate lands in this founding round.

**Parent arc**: ECMAScript parity / Array exotic substrate. Related prior locale: [`pilots/array-sort-tostring-dispatch/`](../array-sort-tostring-dispatch/seed.md), which closed the earlier object-ToString-dispatch slice but left the broader precise sort semantics cluster.

**Workstream**: ECMA-262 Array.prototype.sort semantics for sparse arrays, accessor/prototype side effects, comparator error timing, ToLength preservation, and default/comparator string coercion. The post-EPSUA sample matrix surfaced `Array.prototype.sort` as the top ranked single pipeline x data cell.

## I. Telos

Close the post-EPSUA Array.prototype.sort failure cluster without collapsing it into unrelated Array method work. The likely substrate is the `Runtime::array_proto_sort_via` implementation in `pilots/rusty-js-runtime/derived/src/interp.rs`, especially the current eager `object_get` snapshot of every index followed by dense writeback.

The Phase 2 probe must discriminate whether the cluster is one coherent precise-sort snapshot/writeback defect or several mutually exclusive defects that should split into narrower locales.

## II. Apparatus

- Post-EPSUA matrix: `pilots/apparatus/test262-categorize/results/2026-05-29/{matrix.md,categorized.jsonl}`.
- Raw sample result source: `/home/jaredef/Developer/cruftless-r3-sidecar/results/test262-sample-2026-05-29/results.jsonl`.
- Runtime implementation: `pilots/rusty-js-runtime/derived/src/interp.rs::array_proto_sort_via`.
- Generated shim: `pilots/rusty-js-runtime/derived/src/generated.rs::array_prototype_sort`.
- Prior related work: `pilots/array-sort-tostring-dispatch/{seed.md,trajectory.md,analysis.md}`.

## III. Methodology

1. Phase 0: create this locale and refresh `apparatus/locales/manifest.json`.
2. Phase 2: inspect the `Array.prototype.sort` matrix row and every matching `categorized.jsonl` entry.
3. Sample at least eight failures spanning sparse arrays, default ToString, comparator branch, accessor/prototype side effects, primitive receivers, and array-like length behavior.
4. Apply C4: proceed only if one mechanism bucket accounts for at least 40% of the narrowed cluster.
5. Propose a Phase 3 move shape but do not edit runtime code in APS-EXT 0.

## IV. Carve-Outs

- TypedArray.prototype.sort remains out of scope; typed-array method discipline lives under TAPD/TAMM coordinates.
- General Array exotic storage and property descriptor mechanics are out of scope except where Array.prototype.sort needs them for collection/writeback.
- The already closed object ToString dispatch slice in `array-sort-tostring-dispatch` should not be reopened unless fresh measurement shows regression.
- Stability beyond the sampled sort cluster is measured, not assumed; do not flip broader Array sorting behavior default-on without targeted sort and adjacent Array regression probes.

## V. Resume Protocol

Read this seed, then `trajectory.md`, then the prior `array-sort-tostring-dispatch` locale. Resume by rechecking the latest test262 sample/full-suite matrix for `Array.prototype.sort`, then inspect `array_proto_sort_via` before proposing substrate.
