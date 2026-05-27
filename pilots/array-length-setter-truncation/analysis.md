# array-length-setter-truncation — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| array-length-exotic | FAIL | Directly exercises arr.length assignment truncation semantics |
| array-methods | PASS | Array methods that mutate length internally (splice, push, pop) work via different code paths |
| object-define-property | FAIL | Object.defineProperty(arr, "length", {value:N}) already routes through array_set_length; assignment path is the gap |

The FAIL on array-length-exotic directly confirms this locale's gap: `arr.length = N` for N < current length should truncate the backing storage but does not. The PASS on array-methods shows that methods like splice/push/pop that internally adjust length use a different (working) path, isolating the bug to the OrdinarySet assignment entry point. The object-define-property FAIL is a sibling concern where defineProperty on array length has additional descriptor-enforcement gaps beyond the truncation path this locale targets.
