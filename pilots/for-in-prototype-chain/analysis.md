# for-in-prototype-chain — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| for-in-for-of-lowering | FAIL | Directly exercises for-in semantics including prototype chain enumeration |
| prototype-chain | PASS | Tests prototype chain traversal which for-in depends on |
| property-key-order | FAIL | OrdinaryOwnPropertyKeys ordering (gap #6) intersects with for-in key enumeration order |

The FAIL on for-in-for-of-lowering is directly explained by this locale: `__for_in_keys` delegates to `Object.keys` (own enumerable only, no proto walk), so for-in never yields inherited enumerable keys. The PASS on prototype-chain confirms the proto-chain lookup machinery itself works; the gap is specifically in the for-in key-collection helper. The property-key-order FAIL (mechanism gap #6) may also surface in for-in ordering across prototype levels.
