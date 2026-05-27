# for-of-async-lookahead — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| for-in-for-of-lowering | FAIL | Exercises for-of parsing paths including edge-case head forms |
| async-iteration | PASS | Async iteration passes, confirming for-await parsing is unaffected |

The FAIL on for-in-for-of-lowering may include the `async of` lookahead violation: `for (async of [1])` should be a SyntaxError per the grammar restriction at ECMA-262 14.7.5, but cruft's bare-ident fast-path accepts it. The PASS on async-iteration confirms the for-await parsing path is separate and correct, consistent with this locale's carve-out that for-await is untouched.
