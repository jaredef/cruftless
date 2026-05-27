# ecmascript-parity-shared-upstream-arc — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| iterator-helpers | PASS | Iterator helpers pass; iterator-close-on-abrupt (constraint #4) is the deeper protocol gap |
| iterator-close | FAIL | IteratorClose protocol on abrupt completion is constraint #4 in EPSUA's five-substrate plan (mechanism gap #2) |
| finally-return-override | FAIL | Finally-on-abrupt-loop-exit (mechanism gap #5) is a related abrupt-completion path in the same arc |
| coercion-pipeline | FAIL | ToPrimitive coercion pipeline gaps span multiple EPSUA constraints |
| property-key-order | FAIL | OrdinaryOwnPropertyKeys ordering (mechanism gap #6) is one of the five shared-upstream constraints |

The diff-prod results confirm that multiple EPSUA constraints remain open. The FAILs on iterator-close (mechanism gap #2), finally-return-override (mechanism gap #5), coercion-pipeline (mechanism gap #1), and property-key-order (mechanism gap #6) each correspond to one of the five shared-upstream substrates this arc targets. The PASS on iterator-helpers shows the happy-path iterator protocol works; the gaps are in the abrupt-completion and edge-case paths. The projected ~340-test cascade across all five sub-locales should flip several of these diff-prod fixtures when complete.
