# let-as-bound-name-in-lexical — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| for-in-for-of-lowering | FAIL | ForBinding in for-in/for-of is a site where `let` as bound name must be rejected |
| hoisting-semantics | PASS | Variable declaration semantics work; gap is parser rejection of `let let` |

The for-in-for-of-lowering FAIL may partially reflect this locale's scope: `for (let let in ...)` and `for (const let of ...)` should be SyntaxErrors per ECMA-262 13.3.1.1 but are silently accepted. However, the fixture failure likely has broader causes (see sibling for-head locales). This is fundamentally a parser early-error gap: diff-prod fixtures test runtime behavior of accepted programs, so they would only expose this if they contain `let let` declarations that produce wrong runtime results rather than the expected SyntaxError.
