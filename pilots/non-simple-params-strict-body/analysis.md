# non-simple-params-strict-body — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| destructuring | PASS | Non-simple param lists include destructured params ({a}, [a]) |
| arrow-functions | PASS | Arrow functions with non-simple params + "use strict" body must reject |
| hoisting-semantics | PASS | Strict-mode directive in function body affects parameter validation |

This locale enforces the ECMA-262 early error where "use strict" in a function body is illegal when the parameter list is non-simple (destructured, default-valued, or rest params). The relevant fixtures pass, indicating the parser correctly handles these forms in diff-prod scenarios. No fixture directly tests the rejection of `function(a=1){"use strict"}`, but the destructuring and arrow-functions fixtures exercise the param-list shapes involved.
