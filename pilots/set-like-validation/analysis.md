# set-like-validation — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| map-set-ops | PASS | Set.prototype methods (union, intersection, etc.) share the collection surface |
| error-types | PASS | TypeError is the expected error when a non-Set-like argument is passed |
| collection-iteration-order | FAIL | Set operation iteration order depends on correct GetSetRecord validation |

This locale adds GetSetRecord validation (spec section 24.2.1.2) to the seven Set.prototype set-theory methods so they reject non-Set-like arguments (e.g., plain Arrays that lack size/has/keys) with TypeError instead of silently iterating via Symbol.iterator. The collection-iteration-order FAIL is relevant because Set operation ordering tests may exercise the has/keys dispatch path that validation gates. The locale's direct measurement is via test262's Set/prototype/{op}/array-throws.js fixtures (14 tests).
