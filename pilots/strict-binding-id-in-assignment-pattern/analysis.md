# strict-binding-id-in-assignment-pattern — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| destructuring | PASS | AssignmentPattern destructure is the direct parse surface this locale validates |
| for-in-for-of-lowering | FAIL | for-of destructuring head is where the 5 target test262 residuals live |
| arguments-object | FAIL | `arguments` in assignment pattern strict-mode rejection is a direct target |

This locale closes the AssignmentPattern path for strict-mode eval/arguments/yield rejection in for-of destructuring heads. The for-in-for-of-lowering FAIL is directly relevant: all 5 target fixtures are for-of destructuring patterns. The arguments-object FAIL connects via mechanism gap #8 (Arguments object shape), since `[arguments]` as assignment target in strict mode is one of the 5 residuals. Destructuring PASSes for the general case, consistent with this locale targeting only the narrow for-head expression-conversion path.
