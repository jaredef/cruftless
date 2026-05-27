# eval-function-arguments-binding-semantics — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| eval-lexical-capture | FAIL | Mechanism gap #4: direct eval does not resolve outer const/let bindings |
| arguments-object | FAIL | Mechanism gap #8: arguments is a plain Array, not an exotic Arguments object |
| closures-scopes | PASS | Validates closure capture that eval lexical capture extends |
| hoisting-semantics | PASS | Related binding-instantiation semantics; already correct |

This locale targets two of the nine named mechanism gaps. The eval-lexical-capture and arguments-object FAIL fixtures are the direct empirical anchors for gaps #4 and #8 respectively. Closing this locale should flip both fixtures.
