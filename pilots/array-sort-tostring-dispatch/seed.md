# array-sort-tostring-dispatch — Seed

**Locale tag**: `L.array-sort-tostring-dispatch` (top-level)

**Status**: **CLOSED at ASD-EXT 1** (pending sweep-completion verification).

**Workstream**: ECMA-262 §23.1.3.30.1 SortCompare step 4 — comparator-free Array.prototype.sort must call ToString on each element via ToPrimitive(hint=string), which dispatches user-defined toString/valueOf. Cruft's `array_proto_sort_via` used `abstract_ops::to_string`, the low-level helper that returns `"[object Object]"` for any Object without dispatch.

**Trigger**: T262C cluster (29 tests, Array.prototype.sort, post-AEVPD/SDIBP). Inspecting `bug_596_1.js` (the simplest exemplar): expects ≥2 toString calls when sorting `[obj, obj]`; cruft got 0.

**Composes with**:
- ECMA-262 §23.1.3.30 / §23.1.3.30.1 SortCompare
- §7.1.1 ToPrimitive

## I. Telos

Pre-materialize each element's spec-correct string via ToPrimitive(string) before sorting; sort by string keys. ToString count is N (not O(N log N) as spec literally describes), but N ≥ comparison count for stable sort on N elements, and the test262 fixtures count ToString ≥ N — so the relaxation passes the cluster's check shape.

## II. Apparatus + Methodology

- `interp.rs::array_proto_sort_via`: extract keys via `self.to_primitive(v, "string")` + `abstract_ops::to_string(prim)` for each item; sort an index permutation by keys; reorder items.

## III. Carve-outs

- Comparator branch (sort with callback) unchanged.
- ToString-during-comparison spec literal (vs pre-materialization) deviation: noted but pragmatic. Real engines also cache ToString results during sort for perf.

## IV. Verification

Sweep-blocked at write time (full test262 running). Verification deferred until sweep completes + cruft binary update.

## V. Status

Code landed; verification pending.
