# yield-in-function-params — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| generators | PASS | Generator function bodies are where in_generator is true; yield-in-params rejection depends on this flag |
| arrow-functions | PASS | Arrow functions inside generators are the primary test case (arrow params with yield) |
| arrow-edge-cases | FAIL | Edge cases in arrow parameter parsing connect to the yield-in-arrow-params rejection |

This locale adds SyntaxError rejection when YieldExpression appears in function parameters (arrow inside generator, generator's own params). The generators and arrow-functions fixtures PASS, confirming basic generator and arrow behavior is correct. The arrow-edge-cases FAIL may include cases where arrow parameter default expressions containing yield are not yet rejected. No numbered mechanism gap applies directly; the bug is a missing parser-state flag (in_function_params) check.
