# for-of-rhs-is-assignment-expression — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| for-in-for-of-lowering | FAIL | Exercises for-of/for-in parsing including RHS expression grammar |
| comma-grouping-eval | FAIL | Comma operator semantics intersect: for-of RHS incorrectly allows comma |

The FAIL on for-in-for-of-lowering aligns with this locale: cruft uses `parse_expression` (which allows the comma/sequence operator) for the for-of RHS, but the spec restricts it to AssignmentExpression. `for (x of [], [])` is silently accepted when it should be a SyntaxError. The comma-grouping-eval FAIL further confirms that comma-operator edge cases in expression parsing are a broader gap area that intersects with this locale's scope.
