# array-pattern-rest-trailing-comma — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| destructuring | PASS | Destructuring patterns work broadly; the rest-trailing-comma edge case is a narrow grammar distinction |
| destructuring-iterators | PASS | Iterator-based destructuring passes; `[...x,]` as a pattern (vs expression) is a parser early-error concern |

No diff-prod fixture exercises the `[...x,]` pattern-vs-expression grammar distinction. The locale targets a parser early-error where `[...x,]` is valid as an ArrayLiteral expression but invalid as an ArrayAssignmentPattern. The PASS on destructuring and destructuring-iterators confirms that normal rest patterns work correctly; this locale addresses only the trailing-comma-after-spread edge case that test262's `for-of/dstr/array-rest-before-elision.js` probes.
