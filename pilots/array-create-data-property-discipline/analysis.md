# array-create-data-property-discipline — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| array-methods | PASS | Core Array.prototype methods (map, filter, slice, splice) pass for normal usage paths |
| array-from-of | PASS | Array.from/of pass; these also use CreateDataPropertyOrThrow but on fresh plain arrays |
| symbol-species | FAIL | Species-based constructor selection is the prerequisite for ACDPD's per-element write path to matter |

The diff-prod array-methods fixture passes because it tests normal Array usage where the output array is a plain Array with no pre-existing descriptors. The ACDPD gap only manifests when a custom species constructor pre-populates the output array with non-writable properties, which diff-prod does not probe. The FAIL on symbol-species confirms that the upstream species machinery (mechanism gap #1 ToPrimitive hint dispatch for Symbol.species lookup) is itself incomplete, meaning ACDPD's element-write fix will only become observable after ASCD wires ArraySpeciesCreate.
