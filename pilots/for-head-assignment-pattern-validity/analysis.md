# for-head-assignment-pattern-validity — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| for-in-for-of-lowering | FAIL | Directly exercises for-in/for-of head parsing, including LHS pattern validity |
| destructuring | PASS | Destructuring assignment patterns overlap with AssignmentPattern validation |
| destructuring-iterators | PASS | Iterator-based destructuring shares the expr_to_binding_pattern path |

The FAIL on for-in-for-of-lowering is consistent with this locale's scope: the silent fallback to an empty BindingIdentifier instead of throwing on invalid AssignmentPattern LHS means certain malformed for-of/for-in heads are accepted rather than rejected. The passing destructuring fixtures confirm that the core expr_to_binding_pattern conversion works for valid patterns; the gap is specifically at the for-head reinterpretation site where None is silently swallowed.
