# array-exotic-virtual-property-discipline — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| array-length-exotic | FAIL | Directly exercises Array exotic length semantics: delete, hasOwnProperty, propertyIsEnumerable on length |
| object-define-property | FAIL | Object.defineProperty on array length exercises the descriptor path AEVPD's fixes compose with |
| property-key-order | FAIL | OwnPropertyKeys ordering interacts with array-exotic virtual length visibility |

The FAIL on array-length-exotic is a direct signal: the fixture probes exactly the three sibling code paths this locale targets (delete on non-configurable length, hasOwnProperty on virtual length, propertyIsEnumerable returning false for non-enumerable length). The locale's seed reports it as CLOSED at AEVPD-EXT 1, so these fixes should be landed; the continued FAIL in diff-prod suggests either the fixture tests additional array-exotic behaviors beyond the three fixed paths, or the binary used for the diff-prod run predates the closure. Mechanism gap #6 (OrdinaryOwnPropertyKeys ordering) may also contribute to the array-length-exotic fixture's failure.
