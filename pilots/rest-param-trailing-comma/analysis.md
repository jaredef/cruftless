# rest-param-trailing-comma — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| arrow-functions | PASS | Arrow params with rest elements must reject trailing comma |
| destructuring | PASS | Destructured rest params (...{a,b}) must also reject trailing comma |

This locale enforces the ECMA-262 section 15.1.1 early error that a rest parameter must not be followed by a trailing comma. No diff-prod fixture directly tests the negative syntax `(a, ...rest,)` since diff-prod exercises valid programs. The arrow-functions and destructuring PASSes confirm that valid rest-param usage (without trailing comma) is unaffected. The locale's yield is measured via test262 negative-syntax fixtures, not diff-prod.
